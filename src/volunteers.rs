//! Volunteers' hours: a volunteer's approved hours are the sum of their approved shifts, kept as
//! shifts are added, approved, moved or removed.

use crate::obj;
use async_trait::async_trait;
use ferrum_domain::util::{d, elevated, id, money};
use ferrum_kernel::{Doc, DocHook, Error, HookCtx, Result};
use rust_decimal::Decimal;
use serde_json::json;
use uuid::Uuid;

pub struct ShiftHook;

/// A volunteer's approved hours, without `leaving` (a shift being deleted).
async fn recount(hc: &mut HookCtx<'_>, volunteer: Uuid, leaving: Option<Uuid>) -> Result<()> {
    let hours: Decimal = sqlx::query_scalar(
        "select coalesce(sum(hours), 0) from e_volunteer_shift where tenant_id = $1 and volunteer = $2 and approved and id <> coalesce($3, '00000000-0000-0000-0000-000000000000'::uuid)",
    )
    .bind(hc.ctx.tenant)
    .bind(volunteer)
    .bind(leaving)
    .fetch_one(&mut **hc.tx)
    .await?;
    hc.kernel.set_values_tx(&elevated(hc.ctx), hc.tx, "Volunteer", volunteer, obj(json!({"hours": money(hours)}))).await
}

#[async_trait]
impl DocHook for ShiftHook {
    fn name(&self) -> &'static str {
        "nonprofit.shift"
    }

    async fn validate(&self, _hc: &mut HookCtx<'_>, doc: &Doc) -> Result<()> {
        let hours = d(doc, "hours");
        if hours <= Decimal::ZERO || hours > Decimal::from(24) {
            return Err(Error::field("hours", "a shift is more than nothing and at most 24 hours"));
        }
        Ok(())
    }

    async fn after_save(&self, hc: &mut HookCtx<'_>, doc: &Doc, old: Option<&Doc>) -> Result<()> {
        if let Some(v) = id(doc, "volunteer") {
            recount(hc, v, None).await?;
        }
        // Moved to another volunteer: the first one's hours too.
        if let Some(before) = old.and_then(|o| id(o, "volunteer")).filter(|b| Some(*b) != id(doc, "volunteer")) {
            recount(hc, before, None).await?;
        }
        Ok(())
    }

    async fn before_delete(&self, hc: &mut HookCtx<'_>, doc: &Doc) -> Result<()> {
        if let Some(v) = id(doc, "volunteer") {
            recount(hc, v, id(doc, "id")).await?;
        }
        Ok(())
    }
}
