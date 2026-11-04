//! Memberships: a term of a member's membership, its amount from its type, and its invoice through
//! ERP. A member's standing follows their paid memberships, as ERPNext 13's Membership does: active
//! while one covers today, lapsed once the last has ended.

use crate::{date, obj};
use async_trait::async_trait;
use chrono::{Datelike, Duration, Months, NaiveDate};
use ferrum_domain::flow::{create_for_user, created};
use ferrum_domain::util::{d, elevated, id, money, s, set, sys};
use ferrum_kernel::{Ctx, Doc, DocHook, Error, HookCtx, Kernel, Result};
use ferrum_meta::effective::Schema;
use serde_json::{Value, json};
use std::sync::Arc;
use uuid::Uuid;

pub struct MembershipHook;

/// A term's last day: the day before the same date `months` later, or the end of that month when
/// it has no such date (a month from 31 January ends on the last day of February).
pub(crate) fn term_end(from: NaiveDate, months: u32) -> Option<NaiveDate> {
    let same = from.checked_add_months(Months::new(months))?;
    Some(if same.day() == from.day() { same - Duration::days(1) } else { same })
}

#[async_trait]
impl DocHook for MembershipHook {
    fn name(&self) -> &'static str {
        "nonprofit.membership"
    }

    fn fills(&self) -> &'static [&'static str] {
        &["to_date", "amount"]
    }

    async fn before_validate(&self, hc: &mut HookCtx<'_>, doc: &mut Doc) -> Result<()> {
        let (Some(ty), Some(from)) = (id(doc, "membership_type"), date(doc, "from_date")) else { return Ok(()) };
        let row: Option<(i64, rust_decimal::Decimal)> = sqlx::query_as("select months, amount from e_membership_type where tenant_id = $1 and id = $2")
            .bind(hc.ctx.tenant)
            .bind(ty)
            .fetch_optional(&mut **hc.tx)
            .await?;
        let (months, amount) = row.ok_or_else(|| Error::field("membership_type", "is not a membership type here"))?;
        // An invoiced term keeps the amount it was invoiced at.
        if s(doc, "status").is_none_or(|st| st == "Draft") {
            set(doc, "amount", money(amount));
        }
        let end = term_end(from, months.clamp(1, 120) as u32).ok_or_else(|| Error::field("from_date", "is too far ahead"))?;
        set(doc, "to_date", json!(end.to_string()));
        Ok(())
    }

    async fn validate_change(&self, hc: &mut HookCtx<'_>, old: Option<&Doc>, doc: &Doc) -> Result<()> {
        let Some(old) = old else { return Ok(()) };
        if hc.ctx.is_system {
            return Ok(());
        }
        let was = s(old, "status").unwrap_or("Draft");
        if was != "Draft" && ["member", "membership_type", "from_date"].iter().any(|k| old.get(*k) != doc.get(*k)) {
            return Err(Error::invalid(format!("a membership that is {} does not change: cancel it and make another", was.to_lowercase())));
        }
        Ok(())
    }

    async fn action(&self, hc: &mut HookCtx<'_>, name: &str, doc: &Doc, params: &Value) -> Result<Option<Value>> {
        match name {
            "make_invoice" => Ok(Some(invoice(hc, doc).await?)),
            "cancel_membership" => {
                let ms = id(doc, "id").ok_or_else(|| Error::internal("a membership without an id"))?;
                let _ = params;
                hc.kernel.set_values_tx(&elevated(hc.ctx), hc.tx, "Membership", ms, obj(json!({"status": "Cancelled"}))).await?;
                Ok(Some(json!({"status": "Cancelled"})))
            }
            _ => Ok(None),
        }
    }
}

/// The member's customer: theirs, or one made from them (and kept on the member).
async fn customer_for(hc: &mut HookCtx<'_>, member: &Doc) -> Result<Uuid> {
    if let Some(c) = id(member, "customer") {
        return Ok(c);
    }
    let mid = id(member, "id").ok_or_else(|| Error::internal("a member without an id"))?;
    let customer =
        create_for_user(hc, "Customer", obj(json!({"customer_name": s(member, "member_name"), "customer_type": "Individual", "email": s(member, "email")})))
            .await?;
    let cid = id(&customer, "id").ok_or_else(|| Error::internal("a customer without an id"))?;
    hc.kernel.set_values_tx(&elevated(hc.ctx), hc.tx, "Member", mid, obj(json!({"customer": cid.to_string()}))).await?;
    Ok(cid)
}

/// `make_invoice`: the membership's invoice, a draft Sales Invoice to the member's customer.
async fn invoice(hc: &mut HookCtx<'_>, doc: &Doc) -> Result<Value> {
    let tenant = hc.ctx.tenant;
    let ms = id(doc, "id").ok_or_else(|| Error::internal("a membership without an id"))?;
    let system = sys(hc.ctx);
    let settings: Option<(Option<Uuid>,)> =
        sqlx::query_as("select company from e_nonprofit_settings where tenant_id = $1 limit 1").bind(tenant).fetch_optional(&mut **hc.tx).await?;
    let company = settings.and_then(|s| s.0).ok_or_else(|| Error::invalid("set the company in Nonprofit Settings first"))?;
    let member_id = id(doc, "member").ok_or_else(|| Error::field("member", "is required"))?;
    let member = hc.kernel.get_tx(&system, hc.tx, "Member", member_id, false).await?;
    let ty = hc
        .kernel
        .get_tx(&system, hc.tx, "MembershipType", id(doc, "membership_type").ok_or_else(|| Error::field("membership_type", "is required"))?, false)
        .await?;
    let item = id(&ty, "item").ok_or_else(|| Error::invalid("the membership type has no item to invoice it as"))?;
    let currency: Option<Uuid> = sqlx::query_scalar("select default_currency from e_company where tenant_id = $1 and id = $2")
        .bind(tenant)
        .bind(company)
        .fetch_optional(&mut **hc.tx)
        .await?
        .flatten();
    let customer = customer_for(hc, &member).await?;
    let inv = create_for_user(
        hc,
        "SalesInvoice",
        obj(json!({
            "company": company.to_string(),
            "customer": customer.to_string(),
            "currency": currency.map(|c| c.to_string()),
            "items": [{"item": item.to_string(), "qty": "1", "price_list_rate": money(d(doc, "amount")), "discount_percent": "0",
                       "description": format!("{} membership, {} to {}", s(&ty, "membership_type_name").unwrap_or("A"), s(doc, "from_date").unwrap_or(""), s(doc, "to_date").unwrap_or(""))}],
            "remarks": format!("Membership {}", s(doc, "number").unwrap_or_default()),
        })),
    )
    .await?;
    hc.kernel
        .set_values_tx(
            &elevated(hc.ctx),
            hc.tx,
            "Membership",
            ms,
            obj(json!({"status": "Invoiced", "sales_invoice": inv.get("id").cloned().unwrap_or(Value::Null)})),
        )
        .await?;
    Ok(json!({"status": "Invoiced", "sales_invoice": created("SalesInvoice", &inv)}))
}

/// One workspace's upkeep as of `today`: invoiced memberships whose invoices are submitted and paid
/// count, and each member's standing follows their paid terms.
pub(crate) async fn upkeep(k: &Kernel, tenant: Uuid, schema: Arc<Schema>, today: NaiveDate) -> Result<()> {
    let ctx = Ctx::system(tenant, schema);
    let mut tx = k.db.tenant_tx(tenant).await?;
    let paid: Vec<Uuid> = sqlx::query_scalar(
        "select m.id from e_membership m join e_sales_invoice i on i.tenant_id = m.tenant_id and i.id = m.sales_invoice \
         where m.tenant_id = $1 and m.status = 'Invoiced' and i.docstatus = 1 and coalesce(i.outstanding_amount, 0) <= 0",
    )
    .bind(tenant)
    .fetch_all(&mut *tx)
    .await?;
    for ms in &paid {
        k.set_values_tx(&ctx, &mut tx, "Membership", *ms, obj(json!({"status": "Paid", "paid_on": today.to_string()}))).await?;
    }
    // Each member's standing from their paid terms: active while one covers today.
    let standing: Vec<(Uuid, Option<NaiveDate>, Option<NaiveDate>, String)> = sqlx::query_as(
        "select mb.id, min(ms.from_date), max(ms.to_date), mb.status from e_member mb \
         left join e_membership ms on ms.tenant_id = mb.tenant_id and ms.member = mb.id and ms.status = 'Paid' \
         where mb.tenant_id = $1 group by mb.id, mb.status",
    )
    .bind(tenant)
    .fetch_all(&mut *tx)
    .await?;
    for (member, since, until, was) in standing {
        let status = match until {
            Some(end) if end >= today && since.is_some_and(|s| s <= today) => "Active",
            Some(end) if end < today => "Lapsed",
            _ => was.as_str(),
        };
        if status != was || since.is_some() {
            k.set_values_tx(
                &ctx,
                &mut tx,
                "Member",
                member,
                obj(json!({"status": status, "member_since": since.map(|d| d.to_string()), "expires_on": until.map(|d| d.to_string())})),
            )
            .await?;
        }
    }
    tx.commit().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::term_end;
    use chrono::NaiveDate;

    #[test]
    fn a_term_ends_the_day_before_its_anniversary() {
        let day = |s: &str| NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap();
        assert_eq!(term_end(day("2026-01-15"), 12), Some(day("2027-01-14")));
        assert_eq!(term_end(day("2026-01-31"), 1), Some(day("2026-02-28")), "a month from 31 January ends with February");
        assert_eq!(term_end(day("2024-02-29"), 12), Some(day("2025-02-28")));
        assert_eq!(term_end(day("2026-03-01"), 1), Some(day("2026-03-31")));
    }
}
