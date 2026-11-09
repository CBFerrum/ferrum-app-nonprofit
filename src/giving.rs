//! Donations, their refunds, and what is spent from funds (FEO-051).
//!
//! Submitting a donation posts it to the books as a journal entry: the deposit account debited, its
//! fund's income account (the settings' when the fund names none) credited, both under the fund's
//! cost center, so a restricted fund is kept apart in the ledger (ERPNext 13's Donation made a
//! Payment Entry against a donor customer; a gift is income, not an advance, so this posts it as
//! such). A donation may be refunded in part or whole, as far as its restricted fund still holds:
//! each refund posts the reverse, is kept on the donation, and the donor's receipt is for what is
//! left. Fund spending posts the expense against the deposit account under the fund's cost center;
//! a restricted fund is spent only on the accounts it may be spent on, and only as far as it holds.
//! The donor's, fund's and grant's totals follow; cancelling reverses the entries.

use crate::{date, obj};
use async_trait::async_trait;
use ferrum_domain::util::{b, d, elevated, id, money, rows, s};
use ferrum_kernel::{Doc, DocHook, Error, HookCtx, Result, Tx};
use rust_decimal::Decimal;
use serde_json::{Value, json};
use uuid::Uuid;

pub struct DonationHook;
pub struct SpendHook;
pub struct FundHook;

/// The settings' company and accounts.
async fn books(tx: &mut Tx, tenant: Uuid) -> Result<(Uuid, Uuid, Uuid)> {
    let row: Option<(Option<Uuid>, Option<Uuid>, Option<Uuid>)> =
        sqlx::query_as("select company, deposit_account, donation_income_account from e_nonprofit_settings where tenant_id = $1 limit 1")
            .bind(tenant)
            .fetch_optional(&mut **tx)
            .await?;
    match row {
        Some((Some(c), Some(dep), Some(inc))) => Ok((c, dep, inc)),
        _ => Err(Error::invalid("set the company and the donation accounts in Nonprofit Settings first")),
    }
}

/// What a fund says of the books: its cost center, its income account, and whether it is restricted.
async fn fund_books(tx: &mut Tx, tenant: Uuid, fund: Option<Uuid>) -> Result<(Option<Uuid>, Option<Uuid>, bool)> {
    let Some(fund) = fund else { return Ok((None, None, false)) };
    let row: Option<(Option<Uuid>, Option<Uuid>, Option<bool>)> =
        sqlx::query_as("select cost_center, income_account, restricted from e_fund where tenant_id = $1 and id = $2")
            .bind(tenant)
            .bind(fund)
            .fetch_optional(&mut **tx)
            .await?;
    let (cc, income, restricted) = row.ok_or_else(|| Error::field("fund", "is not a fund here"))?;
    Ok((cc, income, restricted.unwrap_or(false)))
}

/// What a fund holds: received, less refunded, less spent (but for the spending `except`).
async fn fund_balance(tx: &mut Tx, tenant: Uuid, fund: Uuid, except: Option<Uuid>) -> Result<Decimal> {
    let (received, refunded): (Decimal, Decimal) =
        sqlx::query_as("select coalesce(sum(amount), 0), coalesce(sum(refunded), 0) from e_donation where tenant_id = $1 and fund = $2 and docstatus = 1")
            .bind(tenant)
            .bind(fund)
            .fetch_one(&mut **tx)
            .await?;
    let spent: Decimal =
        sqlx::query_scalar("select coalesce(sum(amount), 0) from e_fund_spend where tenant_id = $1 and fund = $2 and docstatus = 1 and id <> $3")
            .bind(tenant)
            .bind(fund)
            .bind(except.unwrap_or_else(Uuid::nil))
            .fetch_one(&mut **tx)
            .await?;
    Ok(received - refunded - spent)
}

/// Posts and submits a journal entry: each line an account, its debit, its credit, and the cost
/// center it carries.
async fn post(hc: &mut HookCtx<'_>, day: chrono::NaiveDate, remark: &str, lines: &[(Uuid, Decimal, Decimal, Option<Uuid>)]) -> Result<Uuid> {
    let (company, _, _) = books(hc.tx, hc.ctx.tenant).await?;
    let accounts: Vec<Value> = lines
        .iter()
        .map(|(account, debit, credit, cc)| json!({"account": account.to_string(), "debit": money(*debit), "credit": money(*credit), "cost_center": cc.map(|c| c.to_string())}))
        .collect();
    let system = elevated(hc.ctx);
    let entry = hc
        .kernel
        .insert_tx(
            &system,
            hc.tx,
            "JournalEntry",
            obj(json!({"company": company.to_string(), "posting_date": day.to_string(), "remark": remark, "accounts": accounts})),
        )
        .await?;
    let entry_id = id(&entry, "id").ok_or_else(|| Error::internal("a journal entry without an id"))?;
    hc.kernel.submit_tx(&system, hc.tx, "JournalEntry", entry_id, None).await?;
    Ok(entry_id)
}

/// Cancels a submitted journal entry.
async fn reverse(hc: &mut HookCtx<'_>, entry: Option<Uuid>) -> Result<()> {
    let Some(entry) = entry else { return Ok(()) };
    let status: Option<i16> = sqlx::query_scalar("select docstatus from e_journal_entry where tenant_id = $1 and id = $2")
        .bind(hc.ctx.tenant)
        .bind(entry)
        .fetch_optional(&mut **hc.tx)
        .await?;
    if status == Some(1) {
        hc.kernel.cancel_tx(&elevated(hc.ctx), hc.tx, "JournalEntry", entry, None).await?;
    }
    Ok(())
}

/// The totals of the donor, fund and grant `doc` names, as the books have them: given net of
/// refunds; a fund's received, refunded, spent and what it holds; a grant's received and spent.
async fn totals(hc: &mut HookCtx<'_>, doc: &Doc) -> Result<()> {
    let tenant = hc.ctx.tenant;
    let system = elevated(hc.ctx);
    if let Some(donor) = id(doc, "donor") {
        let given: Decimal =
            sqlx::query_scalar("select coalesce(sum(amount - coalesce(refunded, 0)), 0) from e_donation where tenant_id = $1 and donor = $2 and docstatus = 1")
                .bind(tenant)
                .bind(donor)
                .fetch_one(&mut **hc.tx)
                .await?;
        hc.kernel.set_values_tx(&system, hc.tx, "Donor", donor, obj(json!({"total_given": money(given)}))).await?;
    }
    if let Some(fund) = id(doc, "fund") {
        let (received, refunded): (Decimal, Decimal) =
            sqlx::query_as("select coalesce(sum(amount), 0), coalesce(sum(refunded), 0) from e_donation where tenant_id = $1 and fund = $2 and docstatus = 1")
                .bind(tenant)
                .bind(fund)
                .fetch_one(&mut **hc.tx)
                .await?;
        let spent: Decimal = sqlx::query_scalar("select coalesce(sum(amount), 0) from e_fund_spend where tenant_id = $1 and fund = $2 and docstatus = 1")
            .bind(tenant)
            .bind(fund)
            .fetch_one(&mut **hc.tx)
            .await?;
        let values = json!({"received": money(received), "refunded": money(refunded), "spent": money(spent), "balance": money(received - refunded - spent)});
        hc.kernel.set_values_tx(&system, hc.tx, "Fund", fund, obj(values)).await?;
    }
    if let Some(grant) = id(doc, "grant") {
        let received: Decimal = sqlx::query_scalar(
            "select coalesce(sum(amount - coalesce(refunded, 0)), 0) from e_donation where tenant_id = $1 and \"grant\" = $2 and docstatus = 1",
        )
        .bind(tenant)
        .bind(grant)
        .fetch_one(&mut **hc.tx)
        .await?;
        let spent: Decimal = sqlx::query_scalar("select coalesce(sum(amount), 0) from e_fund_spend where tenant_id = $1 and \"grant\" = $2 and docstatus = 1")
            .bind(tenant)
            .bind(grant)
            .fetch_one(&mut **hc.tx)
            .await?;
        hc.kernel.set_values_tx(&system, hc.tx, "Grant", grant, obj(json!({"received": money(received), "spent": money(spent)}))).await?;
    }
    Ok(())
}

/// An amount given as a parameter, as text or a number.
fn amount_param(params: &Value) -> Option<Decimal> {
    match params.get("amount")? {
        Value::String(t) => t.trim().parse().ok(),
        Value::Number(n) => n.to_string().parse().ok(),
        _ => None,
    }
}

/// Refunds part or all of a submitted donation: posts the reverse under its fund's cost center, and
/// keeps the refund on it. A restricted fund refunds only what it still holds: what it spent is not
/// given back.
async fn refund(hc: &mut HookCtx<'_>, doc: &Doc, params: &Value) -> Result<Value> {
    let donation = id(doc, "id").ok_or_else(|| Error::internal("a donation without an id"))?;
    let amount = amount_param(params).filter(|a| *a > Decimal::ZERO).ok_or_else(|| Error::field("amount", "a refund is above zero"))?;
    let reason = params.get("reason").and_then(Value::as_str).map(str::trim).filter(|r| !r.is_empty());
    let reason = reason.ok_or_else(|| Error::field("reason", "a refund says why"))?.to_string();
    let left = d(doc, "amount") - d(doc, "refunded");
    if amount > left {
        return Err(Error::field("amount", format!("is more than the {} not yet refunded", left.normalize())));
    }
    let tenant = hc.ctx.tenant;
    let (_, deposit, settings_income) = books(hc.tx, tenant).await?;
    let fund = id(doc, "fund");
    let (cc, income, restricted) = fund_books(hc.tx, tenant, fund).await?;
    if let (true, Some(fund)) = (restricted, fund) {
        let holds = fund_balance(hc.tx, tenant, fund, None).await?;
        if amount > holds {
            return Err(Error::invalid(format!("its fund holds {}: what it spent is not refunded", holds.normalize())));
        }
    }
    let today = hc.ctx.today();
    let remark = format!("Refund of donation {}: {reason}", s(doc, "number").unwrap_or_default());
    let entry = post(hc, today, &remark, &[(income.unwrap_or(settings_income), amount, Decimal::ZERO, cc), (deposit, Decimal::ZERO, amount, cc)]).await?;
    let system = elevated(hc.ctx);
    let mut kept: Vec<Doc> = rows(doc, "refunds")
        .into_iter()
        .map(|r| r.into_iter().filter(|(k, _)| ["date", "amount", "reason", "journal_entry"].contains(&k.as_str())).collect())
        .collect();
    kept.push(obj(
        json!({"id": Uuid::now_v7().to_string(), "date": today.to_string(), "amount": money(amount), "reason": reason, "journal_entry": entry.to_string()}),
    ));
    hc.kernel.set_child_rows_tx(&system, hc.tx, "Donation", "refunds", donation, kept).await?;
    let refunded = d(doc, "refunded") + amount;
    let net = d(doc, "amount") - refunded;
    hc.kernel.set_values_tx(&system, hc.tx, "Donation", donation, obj(json!({"refunded": money(refunded), "net_amount": money(net)}))).await?;
    let mut now = doc.clone();
    now.insert("refunded".into(), money(refunded));
    totals(hc, &now).await?;
    Ok(json!({"refunded": money(refunded), "net_amount": money(net), "journal_entry": entry}))
}

#[async_trait]
impl DocHook for DonationHook {
    fn name(&self) -> &'static str {
        "nonprofit.donation"
    }

    async fn validate(&self, hc: &mut HookCtx<'_>, doc: &Doc) -> Result<()> {
        if d(doc, "amount") <= Decimal::ZERO {
            return Err(Error::field("amount", "a donation is above zero"));
        }
        // A grant's payment comes from its funder, once it is awarded.
        if let Some(grant) = id(doc, "grant") {
            let row: Option<(Option<Uuid>, String)> = sqlx::query_as("select funder, status from e_grant where tenant_id = $1 and id = $2")
                .bind(hc.ctx.tenant)
                .bind(grant)
                .fetch_optional(&mut **hc.tx)
                .await?;
            let (funder, status) = row.ok_or_else(|| Error::field("grant", "is not a grant here"))?;
            if funder != id(doc, "donor") {
                return Err(Error::field("grant", "is another funder's"));
            }
            if !matches!(status.as_str(), "Awarded" | "Closed") {
                return Err(Error::field("grant", format!("is {}: a grant is paid once awarded", status.to_lowercase())));
            }
        }
        Ok(())
    }

    async fn on_submit(&self, hc: &mut HookCtx<'_>, doc: &Doc) -> Result<()> {
        let tenant = hc.ctx.tenant;
        let donation = id(doc, "id").ok_or_else(|| Error::internal("a donation without an id"))?;
        let (_, deposit, settings_income) = books(hc.tx, tenant).await?;
        let (cc, income, _) = fund_books(hc.tx, tenant, id(doc, "fund")).await?;
        let amount = d(doc, "amount");
        let day = date(doc, "date").ok_or_else(|| Error::field("date", "is required"))?;
        let remark = format!("Donation {}", s(doc, "number").unwrap_or_default());
        let entry = post(hc, day, &remark, &[(deposit, amount, Decimal::ZERO, cc), (income.unwrap_or(settings_income), Decimal::ZERO, amount, cc)]).await?;
        let values = obj(json!({"journal_entry": entry.to_string(), "refunded": money(Decimal::ZERO), "net_amount": money(amount)}));
        hc.kernel.set_values_tx(&elevated(hc.ctx), hc.tx, "Donation", donation, values).await?;
        totals(hc, doc).await
    }

    async fn before_cancel(&self, _hc: &mut HookCtx<'_>, doc: &Doc) -> Result<()> {
        if d(doc, "refunded") > Decimal::ZERO {
            return Err(Error::invalid("it has refunds, which stand in the books: refund the rest instead"));
        }
        Ok(())
    }

    async fn on_cancel(&self, hc: &mut HookCtx<'_>, doc: &Doc) -> Result<()> {
        reverse(hc, id(doc, "journal_entry")).await?;
        totals(hc, doc).await
    }

    async fn action(&self, hc: &mut HookCtx<'_>, name: &str, doc: &Doc, params: &Value) -> Result<Option<Value>> {
        match name {
            "refund" => refund(hc, doc, params).await.map(Some),
            _ => Ok(None),
        }
    }
}

#[async_trait]
impl DocHook for FundHook {
    fn name(&self) -> &'static str {
        "nonprofit.fund"
    }

    async fn validate(&self, hc: &mut HookCtx<'_>, doc: &Doc) -> Result<()> {
        // Its cost center is its own: two funds under one could not be told apart in the ledger.
        if let Some(cc) = id(doc, "cost_center") {
            let other: Option<String> = sqlx::query_scalar("select fund_name from e_fund where tenant_id = $1 and cost_center = $2 and id <> $3 limit 1")
                .bind(hc.ctx.tenant)
                .bind(cc)
                .bind(id(doc, "id").unwrap_or_else(Uuid::nil))
                .fetch_optional(&mut **hc.tx)
                .await?;
            if let Some(other) = other {
                return Err(Error::field("cost_center", format!("is {other}'s: each fund has a cost center of its own")));
            }
        }
        Ok(())
    }
}

#[async_trait]
impl DocHook for SpendHook {
    fn name(&self) -> &'static str {
        "nonprofit.fund_spend"
    }

    async fn validate(&self, hc: &mut HookCtx<'_>, doc: &Doc) -> Result<()> {
        if d(doc, "amount") <= Decimal::ZERO {
            return Err(Error::field("amount", "spending is above zero"));
        }
        let fund = id(doc, "fund").ok_or_else(|| Error::field("fund", "is required"))?;
        let account = id(doc, "account").ok_or_else(|| Error::field("account", "is required"))?;
        let root: Option<String> = sqlx::query_scalar("select root_type from e_account where tenant_id = $1 and id = $2")
            .bind(hc.ctx.tenant)
            .bind(account)
            .fetch_optional(&mut **hc.tx)
            .await?
            .flatten();
        if root.as_deref() != Some("Expense") {
            return Err(Error::field("account", "is an expense account"));
        }
        let f = hc.kernel.get_tx(&elevated(hc.ctx), hc.tx, "Fund", fund, false).await?;
        if b(&f, "restricted") && !rows(&f, "eligible_accounts").iter().any(|r| id(r, "account") == Some(account)) {
            return Err(Error::field("account", format!("is not what {} may be spent on", s(&f, "fund_name").unwrap_or("the fund"))));
        }
        if let Some(grant) = id(doc, "grant") {
            let grant_fund: Option<Uuid> = sqlx::query_scalar("select fund from e_grant where tenant_id = $1 and id = $2")
                .bind(hc.ctx.tenant)
                .bind(grant)
                .fetch_optional(&mut **hc.tx)
                .await?
                .flatten();
            if grant_fund != Some(fund) {
                return Err(Error::field("grant", "is paid into another fund"));
            }
        }
        Ok(())
    }

    async fn before_submit(&self, hc: &mut HookCtx<'_>, doc: &mut Doc) -> Result<()> {
        let fund = id(doc, "fund").ok_or_else(|| Error::field("fund", "is required"))?;
        let (_, _, restricted) = fund_books(hc.tx, hc.ctx.tenant, Some(fund)).await?;
        if restricted {
            let holds = fund_balance(hc.tx, hc.ctx.tenant, fund, id(doc, "id")).await?;
            if d(doc, "amount") > holds {
                return Err(Error::invalid(format!("the fund holds {}: a restricted fund is not spent beyond what it holds", holds.normalize())));
            }
        }
        Ok(())
    }

    async fn on_submit(&self, hc: &mut HookCtx<'_>, doc: &Doc) -> Result<()> {
        let tenant = hc.ctx.tenant;
        let spend = id(doc, "id").ok_or_else(|| Error::internal("spending without an id"))?;
        let (_, deposit, _) = books(hc.tx, tenant).await?;
        let (cc, _, _) = fund_books(hc.tx, tenant, id(doc, "fund")).await?;
        let account = id(doc, "account").ok_or_else(|| Error::field("account", "is required"))?;
        let amount = d(doc, "amount");
        let day = date(doc, "date").ok_or_else(|| Error::field("date", "is required"))?;
        let remark = format!("Fund spending {}: {}", s(doc, "number").unwrap_or_default(), s(doc, "description").unwrap_or_default());
        let entry = post(hc, day, &remark, &[(account, amount, Decimal::ZERO, cc), (deposit, Decimal::ZERO, amount, cc)]).await?;
        hc.kernel.set_values_tx(&elevated(hc.ctx), hc.tx, "FundSpend", spend, obj(json!({"journal_entry": entry.to_string()}))).await?;
        totals(hc, doc).await
    }

    async fn on_cancel(&self, hc: &mut HookCtx<'_>, doc: &Doc) -> Result<()> {
        reverse(hc, id(doc, "journal_entry")).await?;
        totals(hc, doc).await
    }
}
