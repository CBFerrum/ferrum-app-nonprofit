//! Nonprofit (FERRUM-INDUSTRY-011): members and their memberships invoiced through ERP, donors and
//! donations posted to the books, funds, grants from funders, and volunteers with their hours.
//!
//! * `members`: a membership's term and amount, its invoice, and the member's standing, which upkeep
//!   keeps: a paid membership makes a member active until it ends, and an ended one lapses them.
//! * `giving`: a submitted donation posts a journal entry (the deposit account against donation
//!   income) and adds to its donor's, fund's and grant's totals; cancelling it reverses both.
//! * `volunteers`: a volunteer's approved hours, kept with their shifts.
//!
//! Adapted from ERPNext 13's Non Profit module (GPL-3.0; docs/PROVENANCE.md). Hooks are registered
//! per entity, so a workspace without the `nonprofit` package never runs them.

mod giving;
mod members;
mod reports;
mod volunteers;

use chrono::{NaiveDate, Utc};
use ferrum_domain::reports::ReportSource;
use ferrum_domain::util::s;
use ferrum_kernel::{Doc, Error, HookRegistry, Kernel, Result};
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

include!(concat!(env!("OUT_DIR"), "/packages.rs"));
include!(concat!(env!("OUT_DIR"), "/ui.rs"));

/// This app's manifest (`app.yaml`).
pub const MANIFEST: &str = include_str!("../app.yaml");

/// This app's metadata packages, embedded at build time.
pub fn packages() -> std::result::Result<Vec<ferrum_meta::Package>, ferrum_meta::MetaError> {
    ferrum_meta::catalog::load_embedded(EMBEDDED)
}

/// Registers this app's document hooks.
pub fn register(hooks: &mut HookRegistry) {
    hooks
        .register("Membership", Arc::new(members::MembershipHook))
        .register("Donation", Arc::new(giving::DonationHook))
        .register("Fund", Arc::new(giving::FundHook))
        .register("FundSpend", Arc::new(giving::SpendHook))
        .register("VolunteerShift", Arc::new(volunteers::ShiftHook));
}

/// This app's reports: the fund ledger, reconciled with the books (FEO-051).
pub fn reports() -> Vec<Arc<dyn ReportSource>> {
    vec![Arc::new(reports::FundLedger)]
}

/// Upkeep: memberships whose invoices were paid count, and members whose memberships ended lapse.
pub async fn maintenance(k: &Kernel) -> Result<()> {
    maintenance_on(k, Utc::now().date_naive()).await
}

/// [`maintenance`] as of `today`.
pub async fn maintenance_on(k: &Kernel, today: NaiveDate) -> Result<()> {
    let mut tx = k.db.system_tx().await?;
    let tenants: Vec<Uuid> = sqlx::query_scalar("select id from tenant where status = 'active' order by id").fetch_all(&mut *tx).await?;
    tx.commit().await?;
    let mut first_error: Option<Error> = None;
    for tenant in tenants {
        match k.registry.schema(tenant).await {
            Ok(s) if s.packages.iter().any(|p| p == "nonprofit") => {
                if let Err(e) = members::upkeep(k, tenant, s, today).await {
                    first_error.get_or_insert(e);
                }
            }
            Ok(_) => {}
            Err(e) => {
                first_error.get_or_insert(e);
            }
        }
    }
    first_error.map_or(Ok(()), Err)
}

/// A date field.
pub(crate) fn date(doc: &Doc, k: &str) -> Option<NaiveDate> {
    s(doc, k).and_then(|v| NaiveDate::parse_from_str(v.get(..10).unwrap_or(v), "%Y-%m-%d").ok())
}

/// A document from JSON.
pub(crate) fn obj(v: Value) -> Doc {
    v.as_object().cloned().unwrap_or_default()
}
