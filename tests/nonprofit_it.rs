//! A nonprofit's year (FERRUM-INDUSTRY-011) against a real PostgreSQL
//! (`FERRUM_NONPROFIT_TEST_DATABASE_URL`, default `postgres://ferrum:ferrum@localhost:5432/ferrum_nonprofit_test`;
//! reset first): a membership invoiced through ERP that counts once its invoice is paid and lapses
//! when it ends; donations posted to the books and reversed when cancelled, with their donor's, fund's
//! and grant's totals; a grant paid only once awarded and only by its funder; volunteers' approved
//! hours; and who may do what.

use chrono::NaiveDate;
use ferrum_kernel::secret::SecretKey;
use ferrum_kernel::tenant::NewTenant;
use ferrum_kernel::{Ctx, Db, DbConfig, Doc, Error, HookRegistry, Kernel, KernelSettings, ListQuery};
use ferrum_meta::Catalog;
use serde_json::{Value, json};
use std::sync::Arc;
use uuid::Uuid;

fn obj(v: Value) -> Doc {
    v.as_object().cloned().unwrap()
}

fn uid(v: &Value) -> Uuid {
    v.as_str().unwrap_or_else(|| panic!("not an id: {v}")).parse().unwrap()
}

fn dec(v: &Value) -> f64 {
    v.as_str().map(|s| s.parse().unwrap()).or_else(|| v.as_f64()).unwrap_or_else(|| panic!("not a number: {v}"))
}

fn day(s: &str) -> NaiveDate {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
}

struct T {
    k: Arc<Kernel>,
    tenant: Uuid,
    admin: Ctx,
}

impl T {
    async fn insert(&self, ctx: &Ctx, entity: &str, v: Value) -> Doc {
        self.k.insert(ctx, entity, obj(v)).await.unwrap_or_else(|e| panic!("insert {entity}: {e:?}"))
    }
    async fn get(&self, entity: &str, id: Uuid) -> Doc {
        self.k.get(&self.admin, entity, id, false).await.unwrap_or_else(|e| panic!("get {entity}: {e:?}"))
    }
    async fn find_one(&self, entity: &str, filters: Value) -> Doc {
        let q = ListQuery { filters: ferrum_kernel::query::parse_filters(&filters).unwrap(), limit: Some(10), ..Default::default() };
        let rows = self.k.list(&self.admin, entity, &q).await.unwrap().rows;
        assert_eq!(rows.len(), 1, "{entity} {filters}");
        rows[0].clone()
    }
    async fn user(&self, email: &str, roles: &[&str]) -> Ctx {
        let roles: Vec<String> = roles.iter().map(|r| r.to_string()).collect();
        let id = self.k.create_user(&self.admin, email, email, Some("Correct-Horse-9"), &roles, false).await.expect("user");
        let s = self.k.login("helpers", email, "Correct-Horse-9").await.expect("login");
        self.k.load_ctx(self.tenant, id, Some(s.token_version), "it".into()).await.expect("ctx")
    }
    async fn docstatus(&self, table: &'static str, id: Uuid) -> i16 {
        let mut tx = self.k.db.tenant_tx(self.tenant).await.unwrap();
        let sql = match table {
            "je" => "select docstatus from e_journal_entry where tenant_id = $1 and id = $2",
            _ => "select docstatus from e_sales_invoice where tenant_id = $1 and id = $2",
        };
        let v: i16 = sqlx::query_scalar(sql).bind(self.tenant).bind(id).fetch_one(&mut *tx).await.unwrap();
        tx.commit().await.unwrap();
        v
    }
}

fn refused<V: std::fmt::Debug>(r: Result<V, Error>, kind: &str, says: &str) {
    let msg = format!("{r:?}");
    let ok = matches!(
        (&r, kind),
        (Err(Error::Invalid(_) | Error::Validation(_)), "invalid") | (Err(Error::Forbidden(_)), "forbidden") | (Err(Error::NotFound(_)), "not found")
    );
    assert!(ok && msg.contains(says), "expected {kind} saying {says:?}, got {msg}");
}

#[test]
fn a_nonprofits_year() {
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().thread_stack_size(16 << 20).build().unwrap();
    rt.block_on(async { tokio::spawn(scenario()).await.unwrap() });
}

async fn scenario() {
    let url = std::env::var("FERRUM_NONPROFIT_TEST_DATABASE_URL").unwrap_or_else(|_| "postgres://ferrum:ferrum@localhost:5432/ferrum_nonprofit_test".into());
    let db = Db::connect(DbConfig { url, max_connections: 16, ..Default::default() }).await.expect("connect");
    sqlx::raw_sql("drop schema if exists public cascade; create schema public;").execute(&db.pool).await.expect("reset");
    let mut packages = ferrum_domain::packages().expect("platform packages");
    packages.extend(ferrum_app_erp::packages().expect("ERP's packages"));
    packages.extend(ferrum_app_nonprofit::packages().expect("app packages"));
    let catalog = Catalog::from_packages(packages).expect("catalog");
    let mut hooks = HookRegistry::default();
    ferrum_domain::register(&mut hooks);
    ferrum_app_erp::register(&mut hooks);
    ferrum_app_nonprofit::register(&mut hooks);
    let settings = KernelSettings { secret_key: Some(SecretKey::new([7u8; 32])), ..Default::default() };
    let k = Kernel::boot(db, catalog, hooks, settings, None).await.expect("boot");
    ferrum_domain::maintenance(&k).await.expect("partitions");
    ferrum_app_erp::maintenance(&k).await.expect("partitions");
    let tenant = k
        .create_tenant(NewTenant {
            slug: "helpers".into(),
            name: "Helping Hands".into(),
            admin_email: "admin@helpers.test".into(),
            admin_password: "Correct-Horse-9".into(),
            admin_name: None,
            packages: vec!["nonprofit".into()],
            settings: Some(json!({"fiscal_year_start_month": 4, "timezone": "Asia/Kolkata"})),
        })
        .await
        .expect("tenant");
    let session = k.login("helpers", "admin@helpers.test", "Correct-Horse-9").await.expect("login");
    let admin = k.load_ctx(tenant, session.user, Some(session.token_version), "it".into()).await.expect("ctx");
    let t = T { k: k.clone(), tenant, admin };
    let a = t.admin.clone();

    // ───────────── the books ─────────────
    let inr = t.find_one("Currency", json!({"code": "INR"})).await;
    let company = uid(&t.insert(&a, "Company", json!({"company_name": "Helping Hands", "abbr": "HH", "default_currency": inr["id"]})).await["id"]);
    let bank = uid(&t.find_one("Account", json!({"account_name": "Primary Bank", "company": company.to_string()})).await["id"]);
    let income = uid(&t.find_one("Account", json!({"account_name": "Other Income", "company": company.to_string()})).await["id"]);
    let services = t.find_one("ItemGroup", json!({"group_name": "Services"})).await;
    let nos = t.find_one("Uom", json!({"uom_name": "Nos"})).await;
    let item = uid(&t
        .insert(
            &a,
            "Item",
            json!({"item_code": "MEMBERSHIP", "item_name": "Membership", "item_group": services["id"], "stock_uom": nos["id"], "is_stock_item": false}),
        )
        .await["id"]);
    let manager = t.user("manager@helpers.test", &["Nonprofit Manager"]).await;
    let raiser = t.user("raise@helpers.test", &["Fundraiser"]).await;
    let coordinator = t.user("people@helpers.test", &["Volunteer Coordinator"]).await;

    // ───────────── a membership ─────────────
    let annual = uid(&t
        .insert(&manager, "MembershipType", json!({"membership_type_name": "Annual", "amount": "1200", "months": 12, "item": item.to_string()}))
        .await["id"]);
    let ana =
        uid(&t.insert(&manager, "Member", json!({"member_name": "Ana Rao", "email": "ana@people.test", "membership_type": annual.to_string()})).await["id"]);
    let ms = t.insert(&manager, "Membership", json!({"member": ana.to_string(), "membership_type": annual.to_string(), "from_date": "2026-01-15"})).await;
    assert_eq!((ms["to_date"].as_str(), dec(&ms["amount"]), ms["status"].as_str()), (Some("2027-01-14"), 1200.0, Some("Draft")));
    let msid = uid(&ms["id"]);
    refused(t.k.run_action(&manager, "Membership", msid, "make_invoice", Value::Null).await, "invalid", "Nonprofit Settings");
    t.insert(
        &manager,
        "NonprofitSettings",
        json!({"company": company.to_string(), "deposit_account": bank.to_string(), "donation_income_account": income.to_string()}),
    )
    .await;
    let made = t.k.run_action(&manager, "Membership", msid, "make_invoice", Value::Null).await.expect("invoiced");
    let invoice = uid(&made["sales_invoice"]["id"]);
    let member = t.get("Member", ana).await;
    let inv = t.get("SalesInvoice", invoice).await;
    assert_eq!(inv["customer"], member["customer"], "a customer made from the member");
    assert_eq!(dec(&inv["items"][0]["price_list_rate"]), 1200.0);
    refused(t.k.update(&manager, "Membership", msid, obj(json!({"from_date": "2026-02-01"})), None).await, "invalid", "does not change");

    // Not paid yet: nothing counts.
    ferrum_app_nonprofit::maintenance_on(&k, day("2026-03-01")).await.unwrap();
    assert_eq!((t.get("Membership", msid).await["status"].as_str(), t.get("Member", ana).await["status"].as_str()), (Some("Invoiced"), Some("Applicant")));
    // Paid through ERP: submitted, then a payment recorded and submitted.
    t.k.submit(&a, "SalesInvoice", invoice, None).await.expect("the invoice submitted");
    let payment = t.k.run_action(&a, "SalesInvoice", invoice, "make_payment", Value::Null).await.expect("a payment");
    t.k.submit(&a, "PaymentEntry", uid(&payment["id"]), None).await.expect("the payment submitted");
    ferrum_app_nonprofit::maintenance_on(&k, day("2026-03-01")).await.unwrap();
    let ms = t.get("Membership", msid).await;
    assert_eq!((ms["status"].as_str(), ms["paid_on"].as_str()), (Some("Paid"), Some("2026-03-01")));
    let member = t.get("Member", ana).await;
    assert_eq!(
        (member["status"].as_str(), member["member_since"].as_str(), member["expires_on"].as_str()),
        (Some("Active"), Some("2026-01-15"), Some("2027-01-14"))
    );
    ferrum_app_nonprofit::maintenance_on(&k, day("2027-02-01")).await.unwrap();
    assert_eq!(t.get("Member", ana).await["status"], "Lapsed", "the term ended");

    // ───────────── giving ─────────────
    let foundation =
        uid(&t.insert(&raiser, "Donor", json!({"donor_name": "Mehta Foundation", "donor_type": "Foundation", "email": "grants@mehta.test"})).await["id"]);
    let bo = uid(&t.insert(&raiser, "Donor", json!({"donor_name": "Bo Chen", "email": "bo@people.test"})).await["id"]);
    let meals = uid(&t.insert(&manager, "Fund", json!({"fund_name": "School meals", "restricted": true})).await["id"]);
    let grant = uid(&t
        .insert(&raiser, "Grant", json!({"title": "Meals for 200 children", "funder": foundation.to_string(), "fund": meals.to_string(), "requested": "50000"}))
        .await["id"]);
    let gift = |donor: Uuid, amount: &str, extra: Value| {
        let mut v = json!({"donor": donor.to_string(), "date": "2026-04-10", "amount": amount, "mode": "Bank transfer"});
        v.as_object_mut().unwrap().extend(extra.as_object().cloned().unwrap_or_default());
        v
    };
    refused(
        t.k.insert(&raiser, "Donation", obj(gift(foundation, "25000", json!({"grant": grant.to_string()})))).await,
        "invalid",
        "a grant is paid once awarded",
    );
    t.k.update(&raiser, "Grant", grant, obj(json!({"status": "Awarded", "awarded": "40000", "decided_on": "2026-04-01"})), None).await.unwrap();
    refused(t.k.insert(&raiser, "Donation", obj(gift(bo, "100", json!({"grant": grant.to_string()})))).await, "invalid", "another funder's");
    refused(t.k.insert(&raiser, "Donation", obj(gift(bo, "0", json!({})))).await, "invalid", "above zero");
    let big = uid(&t.insert(&raiser, "Donation", gift(foundation, "25000", json!({"grant": grant.to_string(), "fund": meals.to_string()}))).await["id"]);
    t.k.submit(&raiser, "Donation", big, None).await.expect("a fundraiser submits it");
    let posted = t.get("Donation", big).await;
    let entry = uid(&posted["journal_entry"]);
    assert_eq!(t.docstatus("je", entry).await, 1, "posted to the books");
    let je = t.get("JournalEntry", entry).await;
    let lines: Vec<(Uuid, f64, f64)> = je["accounts"].as_array().unwrap().iter().map(|l| (uid(&l["account"]), dec(&l["debit"]), dec(&l["credit"]))).collect();
    assert_eq!(lines, [(bank, 25000.0, 0.0), (income, 0.0, 25000.0)], "the deposit account against donation income");
    let small = uid(&t.insert(&raiser, "Donation", gift(bo, "500", json!({}))).await["id"]);
    t.k.submit(&raiser, "Donation", small, None).await.unwrap();
    assert_eq!(dec(&t.get("Donor", foundation).await["total_given"]), 25000.0);
    assert_eq!(dec(&t.get("Donor", bo).await["total_given"]), 500.0);
    assert_eq!(dec(&t.get("Fund", meals).await["received"]), 25000.0);
    assert_eq!(dec(&t.get("Grant", grant).await["received"]), 25000.0);
    refused(t.k.cancel(&raiser, "Donation", big, None).await, "forbidden", "");
    t.k.cancel(&manager, "Donation", big, None).await.expect("cancelled");
    assert_eq!(t.docstatus("je", entry).await, 2, "its entry reversed");
    assert_eq!(dec(&t.get("Donor", foundation).await["total_given"]), 0.0);
    assert_eq!(dec(&t.get("Grant", grant).await["received"]), 0.0);

    // ───────────── volunteers ─────────────
    let cy = uid(
        &t.insert(&coordinator, "Volunteer", json!({"volunteer_name": "Cy Diaz", "email": "cy@people.test", "skills": [{"skill": "Cooking"}]})).await["id"]
    );
    let dee = uid(&t.insert(&coordinator, "Volunteer", json!({"volunteer_name": "Dee Ito"})).await["id"]);
    let shift = |v: Uuid, hours: &str, approved: bool| json!({"volunteer": v.to_string(), "date": "2026-05-02", "hours": hours, "activity": "Kitchen", "approved": approved});
    let three = uid(&t.insert(&coordinator, "VolunteerShift", shift(cy, "3", true)).await["id"]);
    let two = uid(&t.insert(&coordinator, "VolunteerShift", shift(cy, "2", false)).await["id"]);
    refused(t.k.insert(&coordinator, "VolunteerShift", obj(shift(cy, "25", true))).await, "invalid", "at most 24 hours");
    assert_eq!(dec(&t.get("Volunteer", cy).await["hours"]), 3.0, "only approved hours");
    t.k.update(&coordinator, "VolunteerShift", two, obj(json!({"approved": true})), None).await.unwrap();
    assert_eq!(dec(&t.get("Volunteer", cy).await["hours"]), 5.0);
    t.k.delete(&coordinator, "VolunteerShift", three, None).await.unwrap();
    assert_eq!(dec(&t.get("Volunteer", cy).await["hours"]), 2.0, "a removed shift no longer counts");
    t.k.update(&coordinator, "VolunteerShift", two, obj(json!({"volunteer": dee.to_string()})), None).await.unwrap();
    assert_eq!((dec(&t.get("Volunteer", cy).await["hours"]), dec(&t.get("Volunteer", dee).await["hours"])), (0.0, 2.0), "moved with the shift");
    refused(t.k.insert(&coordinator, "Donation", obj(gift(bo, "10", json!({})))).await, "forbidden", "");
    let _ = t.docstatus("si", invoice).await;
}
