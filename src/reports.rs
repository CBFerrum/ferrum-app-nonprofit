//! Fund Ledger (FEO-051): each fund's donations received, refunded and spent as Nonprofit records
//! them, beside what the books carry under the fund's cost center: income less its reversals,
//! expenses, and what is held in the deposit account. A fund kept apart reconciles when the two
//! agree. For those who keep funds: Nonprofit Managers, Fundraisers and administrators.

use async_trait::async_trait;
use ferrum_domain::reports::{ReportSource, col, read_tx};
use ferrum_domain::util::money;
use ferrum_kernel::{Ctx, Error, Kernel, Result};
use rust_decimal::Decimal;
use serde_json::{Value, json};
use std::collections::HashMap;
use uuid::Uuid;

pub struct FundLedger;

#[async_trait]
impl ReportSource for FundLedger {
    fn catalog(&self) -> Vec<Value> {
        vec![json!({"name": "fund_ledger", "label": "Fund Ledger", "module": "nonprofit", "entity": "Fund", "params": []})]
    }

    async fn run(&self, k: &Kernel, ctx: &Ctx, name: &str, _params: &HashMap<String, String>) -> Result<Value> {
        if name != "fund_ledger" {
            return Err(Error::NotFound(format!("report {name}")));
        }
        if !ctx.is_admin && !ctx.roles.iter().any(|r| r == "Nonprofit Manager" || r == "Fundraiser") {
            return Err(Error::Forbidden("the fund ledger is for those who keep funds".into()));
        }
        let mut tx = read_tx(k, ctx).await?;
        #[allow(clippy::type_complexity)]
        let funds: Vec<(Uuid, Option<String>, Option<bool>, Option<Uuid>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>)> = sqlx::query_as(
            "select id, fund_name, restricted, cost_center, received, refunded, spent, balance from e_fund where tenant_id = $1 order by fund_name limit 1000",
        )
        .bind(ctx.tenant)
        .fetch_all(&mut *tx)
        .await?;
        // What the books carry under each cost center, by the kind of account.
        let books: HashMap<Uuid, (Decimal, Decimal, Decimal)> = sqlx::query_as::<_, (Uuid, Decimal, Decimal, Decimal)>(
            "select l.cost_center_id, \
             coalesce(sum(l.credit - l.debit) filter (where a.root_type = 'Income'), 0), \
             coalesce(sum(l.debit - l.credit) filter (where a.root_type = 'Expense'), 0), \
             coalesce(sum(l.debit - l.credit) filter (where a.root_type = 'Asset'), 0) \
             from journal_line l join e_account a on a.tenant_id = l.tenant_id and a.id = l.account_id \
             where l.tenant_id = $1 and l.cost_center_id is not null group by l.cost_center_id",
        )
        .bind(ctx.tenant)
        .fetch_all(&mut *tx)
        .await?
        .into_iter()
        .map(|(cc, income, spent, held)| (cc, (income, spent, held)))
        .collect();
        tx.commit().await?;
        let rows: Vec<Value> = funds
            .into_iter()
            .map(|(_, name, restricted, cc, received, refunded, spent, balance)| {
                let (received, refunded, spent, balance) =
                    (received.unwrap_or_default(), refunded.unwrap_or_default(), spent.unwrap_or_default(), balance.unwrap_or_default());
                let carried = cc.and_then(|c| books.get(&c).copied());
                let reconciles = carried.map(|(income, out, held)| income == received - refunded && out == spent && held == balance);
                json!({"fund": name, "restricted": restricted.unwrap_or(false), "received": money(received), "refunded": money(refunded), "spent": money(spent),
                       "balance": money(balance), "books_income": carried.map(|c| money(c.0)), "books_spent": carried.map(|c| money(c.1)),
                       "books_held": carried.map(|c| money(c.2)), "reconciles": reconciles})
            })
            .collect();
        Ok(json!({
            "columns": [col("fund", "Fund", "data"), col("restricted", "Restricted", "check"), col("received", "Received", "currency"),
                col("refunded", "Refunded", "currency"), col("spent", "Spent", "currency"), col("balance", "Holds", "currency"),
                col("books_income", "Books: Income", "currency"), col("books_spent", "Books: Spent", "currency"), col("books_held", "Books: Held", "currency"),
                col("reconciles", "Reconciles", "check")],
            "rows": rows,
        }))
    }
}
