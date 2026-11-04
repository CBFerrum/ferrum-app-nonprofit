//! Donations: submitting one posts it to the books as a journal entry, the deposit account debited
//! and donation income credited (ERPNext 13's Donation made a Payment Entry against a donor
//! customer; a gift is income, not an advance, so this posts it as such). Its donor's, fund's and
//! grant's totals follow; cancelling it reverses the entry and the totals.

use crate::{date, obj};
use async_trait::async_trait;
use ferrum_domain::util::{d, elevated, id, money, s};
use ferrum_kernel::{Doc, DocHook, Error, HookCtx, Result, Tx};
use rust_decimal::Decimal;
use serde_json::json;
use uuid::Uuid;

pub struct DonationHook;

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

/// Totals of submitted donations for a donor, a fund and a grant, as the books have them.
async fn totals(hc: &mut HookCtx<'_>, doc: &Doc) -> Result<()> {
    let tenant = hc.ctx.tenant;
    let system = elevated(hc.ctx);
    for (entity, field, total_field) in [("Donor", "donor", "total_given"), ("Fund", "fund", "received"), ("Grant", "grant", "received")] {
        let Some(target) = id(doc, field) else { continue };
        let sql = match field {
            "donor" => "select coalesce(sum(amount), 0) from e_donation where tenant_id = $1 and donor = $2 and docstatus = 1",
            "fund" => "select coalesce(sum(amount), 0) from e_donation where tenant_id = $1 and fund = $2 and docstatus = 1",
            _ => "select coalesce(sum(amount), 0) from e_donation where tenant_id = $1 and \"grant\" = $2 and docstatus = 1",
        };
        let sum: Decimal = sqlx::query_scalar(sql).bind(tenant).bind(target).fetch_one(&mut **hc.tx).await?;
        hc.kernel.set_values_tx(&system, hc.tx, entity, target, obj(json!({total_field: money(sum)}))).await?;
    }
    Ok(())
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
        let (company, deposit, income) = books(hc.tx, tenant).await?;
        let amount = money(d(doc, "amount"));
        let day = date(doc, "date").ok_or_else(|| Error::field("date", "is required"))?;
        let remark = format!("Donation {}", s(doc, "number").unwrap_or_default());
        let system = elevated(hc.ctx);
        let entry = hc
            .kernel
            .insert_tx(
                &system,
                hc.tx,
                "JournalEntry",
                obj(json!({"company": company.to_string(), "posting_date": day.to_string(), "remark": remark,
                           "accounts": [{"account": deposit.to_string(), "debit": amount, "credit": "0", "remarks": "Donation received"},
                                        {"account": income.to_string(), "debit": "0", "credit": amount, "remarks": "Donation income"}]})),
            )
            .await?;
        let entry_id = id(&entry, "id").ok_or_else(|| Error::internal("a journal entry without an id"))?;
        hc.kernel.submit_tx(&system, hc.tx, "JournalEntry", entry_id, None).await?;
        hc.kernel.set_values_tx(&system, hc.tx, "Donation", donation, obj(json!({"journal_entry": entry_id.to_string()}))).await?;
        totals(hc, doc).await
    }

    async fn on_cancel(&self, hc: &mut HookCtx<'_>, doc: &Doc) -> Result<()> {
        if let Some(entry) = id(doc, "journal_entry") {
            let system = elevated(hc.ctx);
            let status: Option<i16> = sqlx::query_scalar("select docstatus from e_journal_entry where tenant_id = $1 and id = $2")
                .bind(hc.ctx.tenant)
                .bind(entry)
                .fetch_optional(&mut **hc.tx)
                .await?;
            if status == Some(1) {
                hc.kernel.cancel_tx(&system, hc.tx, "JournalEntry", entry, None).await?;
            }
        }
        totals(hc, doc).await
    }
}
