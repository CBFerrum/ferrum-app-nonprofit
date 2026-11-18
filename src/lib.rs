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

use chrono::NaiveDate;
use ferrum_domain::reports::ReportSource;
use ferrum_domain::util::s;
use ferrum_kernel::{Doc, HookRegistry, Kernel, Result};
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

/// Upkeep, for one workspace: memberships whose invoices were paid count, and members whose memberships ended lapse, on
/// the workspace's own day.
pub async fn maintenance(k: &Kernel, tenant: Uuid) -> Result<()> {
    upkeep(k, tenant, None).await
}

/// [`maintenance`] as if the workspace's day were `today`.
pub async fn maintenance_on(k: &Kernel, tenant: Uuid, today: NaiveDate) -> Result<()> {
    upkeep(k, tenant, Some(today)).await
}

async fn upkeep(k: &Kernel, tenant: Uuid, on: Option<NaiveDate>) -> Result<()> {
    members::upkeep(k, tenant, k.registry.schema(tenant).await?, on).await
}

/// A date field.
pub(crate) fn date(doc: &Doc, k: &str) -> Option<NaiveDate> {
    s(doc, k).and_then(|v| NaiveDate::parse_from_str(v.get(..10).unwrap_or(v), "%Y-%m-%d").ok())
}

/// A document from JSON.
pub(crate) fn obj(v: Value) -> Doc {
    v.as_object().cloned().unwrap_or_default()
}
