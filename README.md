<div align="center">

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/assets/banner-dark.svg">
  <img alt="Ferrum Nonprofit: members, donations, funds, grants and volunteers in the books" src="docs/assets/banner-light.svg" width="100%">
</picture>

<br>

[![Version](https://img.shields.io/badge/version-0.2.2-be185d?style=flat-square)](CHANGELOG.md)
[![Licence: AGPL-3.0](https://img.shields.io/badge/licence-AGPL--3.0-blue?style=flat-square)](LICENSE)
[![Ferrum platform](https://img.shields.io/badge/runs%20on-Ferrum-2f5bea?style=flat-square)](https://github.com/CBFerrum/Ferrum)
![Rust](https://img.shields.io/badge/Rust-server-b7410e?style=flat-square&logo=rust)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-data-336791?style=flat-square&logo=postgresql&logoColor=white)
![React](https://img.shields.io/badge/React%20%2B%20TypeScript-interface-087ea4?style=flat-square&logo=react&logoColor=white)
![Tests](https://img.shields.io/badge/tests-2%20Rust%20%2B%20UI%20suite-16a34a?style=flat-square)

### Members, donors and volunteers, with every gift posted to the books and every restricted fund kept apart.

Ferrum Nonprofit is for charities, associations and clubs that run on memberships, gifts and grants. Memberships are
invoiced through Ferrum ERP. Donations post journal entries. Restricted funds have a cost centre of their own and are
spent only on what they are for.

[💡 Why](#-why-nonprofit) · [📸 Screenshots](#-screenshots) · [✨ Features](#-features) · [📊 Compare](#-how-it-compares) · [📍 Roadmap](#-roadmap) · [🚀 Install](#-install) · [🔧 Develop](#-develop)

</div>

## 💡 Why Nonprofit

- 📚 **Gifts are in the ledger.** Submitting a donation posts a journal entry: the account donations are received into is
  debited and donation income credited. Cancelling reverses it. There is no second system to reconcile.
- 🧭 **Restricted funds stay restricted.** A restricted fund has its own cost centre, is spent only on the expense accounts
  it lists, and only as far as it holds. Refunds hand back only what the fund still has.
- 💳 **Memberships count once paid.** A membership term is invoiced through ERP and counts when its invoice is paid.
  A member is active while a paid term covers today and lapses when the last one ends.
- 🤲 **Grants and volunteers in the same place.** Grants from funders with the reports owed, volunteers with skills,
  shifts and approved hours.
- 🔏 **Personal data handled by rule.** Requests find members, donors and volunteers by email and erase names and contacts;
  amounts and dates stay for the books. A donor's tax ID sits in a field group.
- 🔓 **Open source and self-hosted.** AGPL-3.0, on your own PostgreSQL.
- 🌍 **Arabic and Hindi** words and names, with right-to-left layout.

## 🎯 At a glance

| | |
|---|---|
| **Version** | 0.2.2 |
| **Kind** | Its own interface at `/nonprofit/` (a home of figures), with every record in Ferrum's lists and forms |
| **What it adds** | 15 record types (10 documents, 4 child tables, 1 settings record), 3 roles, 1 report, 1 print format, 1 menu |
| **Depends on** | Ferrum platform `>=0.3.2, <0.4`; ERP `>=0.1` (installing Nonprofit installs ERP) |
| **Works with** | ERP (customers, items, companies, accounts, cost centres, sales invoices, journal entries) |
| **Languages** | English, Arabic, Hindi (words and record names); right-to-left layout |
| **Tests** | 2 Rust tests (one walks a year on PostgreSQL) and a UI suite |
| **Licence** | AGPL-3.0-only |

## 📸 Screenshots

<div align="center">
<picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/screens/nonprofit-donations-dark.webp"><img alt="Donations" src="docs/assets/screens/nonprofit-donations-light.webp" width="100%"></picture>
<p><b>💝 Donations</b><br><sub>Every donation with its donor, date, amount and fund.</sub></p>
</div>

<table>
<tr>
<td width="50%" valign="top">
<picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/screens/nonprofit-member-list-dark.webp"><img alt="Members" src="docs/assets/screens/nonprofit-member-list-light.webp"></picture>
<p><b>🪪 Members</b><br><sub>Each member with their membership type, standing and when it ends.</sub></p>
</td>
<td width="50%" valign="top">
<picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/screens/nonprofit-donor-list-dark.webp"><img alt="Donors" src="docs/assets/screens/nonprofit-donor-list-light.webp"></picture>
<p><b>🎁 Donors</b><br><sub>Donors with their type, email and what they have given net of refunds.</sub></p>
</td>
</tr>
<tr>
<td width="50%" valign="top">
<picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/screens/nonprofit-fund-list-dark.webp"><img alt="Funds" src="docs/assets/screens/nonprofit-fund-list-light.webp"></picture>
<p><b>🏦 Funds</b><br><sub>Each fund, whether it is restricted, and what it has received, spent and holds.</sub></p>
</td>
<td width="50%" valign="top">
<picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/screens/nonprofit-member-detail-dark.webp"><img alt="One member" src="docs/assets/screens/nonprofit-member-detail-light.webp"></picture>
<p><b>🧑‍🤝‍🧑 One member</b><br><sub>Read one member: contact details, membership type, standing and notes.</sub></p>
</td>
</tr>
<tr>
<td width="50%" valign="top">
<picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/screens/nonprofit-home-dark.webp"><img alt="The organisation at a glance" src="docs/assets/screens/nonprofit-home-light.webp"></picture>
<p><b>💗 The organisation at a glance</b><br><sub>Members by standing, giving for the year, grants awaiting or awarded, and volunteers&#x27; approved hours. Each figure opens its list.</sub></p>
</td>
<td width="50%" valign="top">
<picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/screens/nonprofit-fund-detail-dark.webp"><img alt="One fund" src="docs/assets/screens/nonprofit-fund-detail-light.webp"></picture>
<p><b>🧭 One fund</b><br><sub>A fund&#x27;s purpose, its cost centre, eligible accounts, and what it has received and spent.</sub></p>
</td>
</tr>
</table>

### 📱 On a phone

<table>
<tr>
<td align="center" valign="top">
<picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/screens/nonprofit-home-phone-dark.webp"><img alt="The home page on a phone" src="docs/assets/screens/nonprofit-home-phone-light.webp" width="260"></picture>
<p><b>💗 The home page on a phone</b><br><sub>The same figures on a narrow screen.</sub></p>
</td>
<td align="center" valign="top">
<picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/screens/nonprofit-member-list-phone-dark.webp"><img alt="Members on a phone" src="docs/assets/screens/nonprofit-member-list-phone-light.webp" width="260"></picture>
<p><b>🪪 Members on a phone</b><br><sub>Look up a member at an event or at the door.</sub></p>
</td>
</tr>
</table>


## ✨ Features

### 🪪 Members and memberships

- **Membership types.** A type has a name, an amount, how many months one membership lasts (1 to 120, 12 by default),
  and the item it is invoiced as.
- **Members.** Name, email, phone, membership type, notes, and the customer invoices go to (made from the member when
  empty). Standing is *Applicant*, *Active* or *Lapsed*, with a member-since date and an expiry date.
- **Terms.** A membership is one term: a member, a type, a start date (today by default) and an end date worked out for
  you, the day before the same date months later, or the end of the month when there is no such date.
- **Four states.** A term is *Draft*, *Invoiced*, *Paid* or *Cancelled*. Invoice It makes a draft Sales Invoice to the
  member's customer. Cancel It works on a draft or an invoiced term.
- **Counts once paid.** Upkeep marks a term paid once its invoice is submitted and settled. A member is active while a paid
  term covers today and lapsed once the last has ended.
- **An invoiced term does not change.** Cancel it and make another.
- **Numbered** `MEM-2026-00001` and `MSP-2026-00001`. Members can be imported and exported.

### 🎁 Donors and donations

- **Donors.** Name, type (*Individual, Organization, Foundation, Government*), email, tax ID (in a restricted field group
  so only granted roles see it) and the total given, net of refunds.
- **Donations.** A donation names a donor, a date (today by default), an amount above zero, an optional fund, an optional
  grant it pays, how it was given and a reference.
- **Posted to the books.** Submitting a donation posts a journal entry under the fund's cost centre: the deposit account
  debited, the fund's income account credited (the settings' account when the fund names none). Cancelling it reverses
  the entry.
- **Refunds.** A manager who may cancel donations refunds part or all of a submitted donation, saying why. The reverse is
  posted and kept on the donation. A refund cannot exceed what is not yet refunded. A restricted fund refunds only what it
  still holds: what it spent is not given back. A donation with refunds is not cancelled; refund the rest instead.
- **A donor's receipt.** The donation's print shows what was received, each refund, and what was given. It carries no
  country-specific tax wording (see the roadmap).
- **Totals follow.** A donor's total, a fund's received, refunded and spent, and a grant's received and spent follow
  the books.
- **Numbered** `DON-2026-00001`.

### 🏦 Funds

- **A purpose for the money.** A fund has a name, a purpose and a restricted flag.
- **Kept apart.** A restricted fund must have a cost centre, and every posting it makes carries it: donations, refunds and
  spending. Each fund has a cost centre of its own.
- **An income account of its own,** or the settings' account.
- **May be spent on.** A restricted fund lists the expense accounts it may be spent on.
- **Fund spending.** A *Fund Spending* document names a fund, a date, an expense account, an amount and a description, and
  may report against a grant. Submitting it posts the expense under the fund's cost centre; cancelling reverses it.
  A restricted fund is spent only on its eligible accounts and only as far as it holds.
- **What a fund holds.** Received, less refunded, less spent.
- **Numbered** `SPD-2026-00001`.

### 📊 Fund Ledger

- **A report that checks itself.** Each fund as Nonprofit records it (received, refunded, spent, holds) beside what the
  books carry under its cost centre (income, spending, held), and whether the two reconcile. It is for Nonprofit
  Managers, Fundraisers and administrators.

### 🤲 Grants

- **Money a funder is asked for.** A grant has a title, a funder (a donor), an optional fund, an amount requested, an
  amount awarded, and submitted and decided dates.
- **Five states.** *Draft, Submitted, Awarded, Declined, Closed.*
- **Reports owed.** A grant lists the reports it owes, each with a due date and the date it was sent.
- **Paid once awarded, and by its funder.** A donation naming a grant is refused unless the grant is awarded and the donor
  is its funder. Fund spending that reports against a grant must be from the fund the grant pays into.
- **Totals.** Received and spent follow the books.

### 🙋 Volunteers

- **Volunteers.** Name, email, phone, active flag, skills and availability (*Weekdays, Weekends, Evenings, Any Time*).
- **Shifts.** A shift has a volunteer, a date, hours (more than nothing and at most 24), an activity and an approved box.
- **Approved hours.** A volunteer's hours are the sum of their approved shifts, kept as shifts are added, approved, moved
  and removed.

### 🏠 The home page

- **Figures that open their lists.** Active members, members not yet paid, lapsed members, giving in the workspace's year,
  grants awaiting or awarded, and volunteers' approved hours.
- **What the giving figure counts.** Submitted donations dated in the workspace's current year; a draft donation does not
  count until it is submitted.
- **Counted over every record,** page after page, not one page of 500.
- **First run.** Before the books are named, the page says so and links to Nonprofit Settings.

### 🔐 Roles and permissions

| Who | Can |
|---|---|
| 🗝️ Nonprofit Manager | Everything: settings, membership types, members, memberships, donors (including tax ID), funds, donations (submit, cancel, refund), fund spending, grants, volunteers and shifts; imports and exports; reads accounts, cost centres and journal entries |
| 💸 Fundraiser | Reads members; creates and edits donors; submits donations; creates and edits grants; reads funds and fund spending |
| 🙌 Volunteer Coordinator | Keeps volunteers and their shifts and approves hours |

### 🔏 Personal data

- **Found by email.** A personal-data request finds members, donors and volunteers by email, and erases their names and
  contacts (members' notes; donors' tax ID). Amounts and dates stay, as the books need them.

### 🌍 Interface

- **Its own home,** and every record in the shared list and form pages with search, filters, import and export.
- **Languages.** English, Arabic and Hindi words, Arabic and Hindi names for every record and field. A workspace can add
  its own words in Admin.
- **Right to left.** Pages mirror as they read.
- **Assistant.** It knows the states and where figures come from, and suggests "How much did we raise this year, by fund?",
  "Which memberships lapse next month?", "Which grant reports are due?" and "Which volunteer shifts still need people?".

## 🧭 How it works

```mermaid
flowchart LR
  D["Donation (draft)"] -->|submit| P["Journal entry: deposit debited, income credited"]
  P --> F["Fund totals and fund cost centre"]
  P --> DT["Donor total"]
  P -->|grant payment| G["Grant received"]
  P -->|refund| R["Reverse entry, net of refunds"]
  P -->|cancel| X["Entry reversed"]
```

Donations are income, posted as journal entries under the fund's cost centre. Refunds and cancellations reverse the
entry, so the fund, the donor and the grant always agree with the ledger.

```mermaid
flowchart TD
  MT["Membership type (amount, months, item)"] --> M["Membership term (draft)"]
  M -->|invoice it| I["Sales invoice in ERP"]
  I -->|paid| PD["Term paid"]
  PD --> ACT["Member active until the term ends"]
  ACT --> LAP["Member lapsed"]
```

A term counts once its invoice is paid. Upkeep keeps each member's standing in step.

## 🧱 Data model

| Entity | What it is | Key fields or states |
|---|---|---|
| Membership Type | A kind of membership | Name, amount, months, item |
| Member | Someone who belongs | Name, email, phone, type, customer, member since, expires on; *Applicant, Active, Lapsed* |
| Membership | One term of a member's membership | Member, type, from, to, amount, invoice, paid on; *Draft, Invoiced, Paid, Cancelled* |
| Donor | A giver or funder | Name, type, email, tax ID, total given |
| Fund | Money given for a purpose | Name, restricted, cost centre, income account, eligible accounts, received, refunded, spent, balance |
| Donation | A gift received (submittable) | Donor, date, amount, fund, grant, how given, journal entry, refunds, net of refunds |
| Fund Spending | Money spent from a fund (submittable) | Fund, date, expense account, amount, description, grant, journal entry |
| Grant | Money asked of a funder | Funder, fund, requested, awarded, received, spent, reports owed; *Draft, Submitted, Awarded, Declined, Closed* |
| Volunteer, Shift | A volunteer and their hours | Skills, availability, approved hours; date, hours, activity, approved |
| Nonprofit Settings | Where donations are posted | Company, donations received into, donation income account |

**Roles:** Nonprofit Manager, Fundraiser, Volunteer Coordinator.

**Reports:** Fund Ledger.

**Print formats:** Donation receipt.

**Scheduled upkeep:** terms are marked paid and members' standing is updated.

## 🔌 Integrations

| With | How | Needs |
|---|---|---|
| **Ferrum ERP** | Memberships become draft Sales Invoices; donations and spending post journal entries under cost centres; customers, items and accounts are ERP's | ERP installed with the app; a company and two accounts in Nonprofit Settings |
| **Personal-data requests** | Members, donors and volunteers declared with what is erased | Nothing |
| **Ferrum assistant** | Knows the records' states | Nothing |

Not built in: payment gateways and online giving pages, recurring giving, country tax receipt rules, events and email
campaigns. Ferrum has a separate Campaigns app; this app does not link to it yet.

## 📊 How it compares

Nonprofit is compared with **Salesforce Nonprofit Cloud**, **Blackbaud Raiser's Edge NXT**, **Bloomerang**, **Bonterra** and
**CiviCRM** (the open-source one). The design started from the ERPNext Non Profit module. Those products are built around
the donor relationship; this one is built around the ledger and the fund.

| Capability | Ferrum Nonprofit | Salesforce Nonprofit Cloud | Raiser's Edge NXT | Bloomerang | Bonterra | CiviCRM |
|---|:-:|:-:|:-:|:-:|:-:|:-:|
| Donor records with giving totals | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Donations posted to the accounting ledger as journal entries | ✅ | 🟡 | 🟡 | 🟡 | 🟡 | 🟡 |
| Restricted funds: own cost centre, eligible accounts, spending capped at what is held | ✅ | 🟡 | 🟡 | 🟡 | 🟡 | 🟡 |
| Fund ledger reconciled with the books | ✅ | ❌ | 🟡 | ❌ | 🟡 | 🟡 |
| Partial refunds with reversed postings and net receipts | ✅ | 🟡 | ✅ | 🟡 | 🟡 | ✅ |
| Memberships invoiced, standing from paid terms | ✅ | 🟡 | ✅ | ✅ | ✅ | ✅ |
| Membership renewals, tiers, auto-renew and a member portal | ❌ | 🟡 | ✅ | ✅ | ✅ | ✅ |
| Grants with a lifecycle and reports owed | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ |
| Grant budgets by line, deliverables, reminders, funder portal | ❌ | ✅ | 🟡 | ❌ | ✅ | 🟡 |
| Volunteers with skills, shifts and approved hours | ✅ | ✅ | 🟡 | ✅ | ✅ | 🟡 |
| Volunteer opportunities with portal sign-up and background checks | ❌ | ✅ | 🟡 | 🟡 | ✅ | 🟡 |
| Online donation pages and payment gateways | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Pledges and recurring gifts with schedules and reminders | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Tax receipts by country and receipt series | 🟡 | 🟡 | ✅ | ✅ | ✅ | 🟡 |
| One constituent profile: donor, member, volunteer, household | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Appeals, campaigns and source on each gift | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Automatic thank-yous and lapsed-donor reminders | ❌ | ✅ | ✅ | ✅ | ✅ | 🟡 |
| Email campaigns to donors and members | 🟡 | ✅ | 🟡 | ✅ | ✅ | ✅ |
| Major gifts pipeline: prospects, moves, ratings | ❌ | ✅ | ✅ | 🟡 | ✅ | 🟡 |
| Donor reports: LYBUNT, SYBUNT, retention, campaign return | ❌ | ✅ | ✅ | ✅ | ✅ | 🟡 |
| Net-assets statements and Form 990-style reporting | ❌ | ❌ | 🟡 | ❌ | 🟡 | ❌ |
| Events, ticketing and auctions | ❌ | 🟡 | 🟡 | 🟡 | ✅ | ✅ |
| General ledger, invoices and payables in the same system | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ |
| Personal-data erasure declared per record, amounts kept for the books | ✅ | 🟡 | 🟡 | 🟡 | 🟡 | 🟡 |
| Self-hosted on your own servers | ✅ | ❌ | ❌ | ❌ | ❌ | ✅ |
| Open source | ✅ | ❌ | ❌ | ❌ | ❌ | ✅ |
| No licence fee per user | ✅ | ❌ | ❌ | ❌ | ❌ | ✅ |

Notes. **Ferrum 🟡:** the receipt shows what was given net of refunds, with no deduction wording for any country; email
campaigns exist as a separate Ferrum app and are not linked to donors yet. **Leaders:** 🟡 means a lighter feature, a
separate product (fund accounting and ledgers are usually a different system or product), or an integration or
extension. A refund here is recorded in the books; the money itself goes back outside Ferrum.

### 🏆 Where Nonprofit stands out

- 📚 **One ledger.** Gifts, refunds and fund spending are journal entries in the books, not exports.
- 🧭 **Restricted funds enforced.** Separate cost centre, eligible accounts and a cap at what is held.
- ✅ **A report that reconciles.** The Fund Ledger shows whether records and books agree, fund by fund.
- 💳 **Memberships through invoicing.** A term counts when its invoice is paid.
- 🔓 **Open and owned.** AGPL, self-hosted, no per-seat price.

### 🚧 Where the leaders are still ahead

- 🧑‍🤝‍🧑 **The donor relationship.** One profile across gifts, memberships and volunteering, households, interactions, major gifts.
- 💻 **Online giving.** Donation pages, gateways, pledges and recurring gifts.
- 🧾 **Receipts by country,** and automatic thank-yous.
- 📈 **Reporting.** Donor retention and campaign return.
- 🎟️ **Events,** and a member portal with renewals.

> [!NOTE]
> Leader columns are from public product information as of November 2026 and may change. Product names belong to
> their owners.

## 📍 Roadmap

### ✅ Recently shipped

- [x] **0.2.2** Tuned design system: better menu icons, lighter first page, cleaner print.
- [x] **0.2.1** The redesigned interface with shaped loading and told-apart states.
- [x] **0.2.0** Funds kept apart in the books, fund spending, refunds as far as a fund holds, and the Fund Ledger report.
- [x] **0.1.15** Names of records and fields in Arabic and Hindi.
- [x] **0.1.14** The assistant knows the records; the donor's tax ID in a field group.
- [x] **0.1.8** Home figures counted over every record, on the workspace's year, with members not yet paid.

### 🔜 Next up

**Giving**
- [ ] Online donation and membership pages with a payment gateway and emailed receipts.
- [ ] Pledges and recurring gifts with schedules, reminders and write-offs.
- [ ] Tax receipts by country (for example 80G and 12A, Gift Aid, US acknowledgements) with receipt series.
- [ ] Appeals, campaigns and source on each gift.
- [ ] Split gifts across funds, in-kind and stock gifts, tributes and anonymous gifts.

**People**
- [ ] One constituent profile across donor, member, volunteer and funder, with households and relationships.
- [ ] Automatic thank-yous and lapsed-donor and renewal reminders.
- [ ] Membership renewal, grace period, auto-renew, tiers, benefits and a member portal.

**Money**
- [ ] Payment reconciliation: gateway payouts to the bank and refunds through the gateway.
- [ ] Reports: LYBUNT and SYBUNT, retention, levels and campaign return.

### 🔭 Later

**Grants and volunteers**
- [ ] Grant pipeline, budget by line, deliverables, report reminders and a funder portal.
- [ ] Volunteer opportunities with portal sign-up, shifts against needs, background checks and hours certificates.

**Reporting**
- [ ] Net assets with and without restriction, releases and functional expenses, with country returns as add-ons.
- [ ] A board pack.

**Relationships and events**
- [ ] A major gifts pipeline with ratings and ask amounts.
- [ ] Events, ticketing, registrations and auctions.
- [ ] Link donors and members to the Campaigns app.

## 🚀 Install

Nonprofit is a native Ferrum app: the Ferrum server includes it (`runtime: native`) and a workspace turns it on from
**App Manager**. It needs platform `>=0.3.2, <0.4`. Installing it installs **ERP**, which holds the ledger and invoices.

After installing:

1. Give people the roles **Nonprofit Manager**, **Fundraiser** and **Volunteer Coordinator**.
2. Open **Nonprofit Settings** and set the **company**, the account donations are **received into**, and the **donation
   income** account.
3. Create **membership types** (amount, months, item) and, if you keep restricted money, **funds** with a cost centre and the
   expense accounts they may be spent on.
4. Add members, donors, grants and volunteers, or import members and donors.

## 🔧 Develop

```bash
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
FERRUM_NONPROFIT_TEST_DATABASE_URL=postgres://ferrum:ferrum@localhost:5432/ferrum_nonprofit_test cargo test
```

The database must exist, be owned by a role that is not a superuser (superusers bypass row-level security) and be reset
before the run. `tests/nonprofit_it.rs` walks a year: a membership invoiced, paid through ERP, active, then lapsed; a
grant awarded and paid, and donations posted and one reversed; volunteers' hours; and who may do what. Rust dependencies
come from the Ferrum platform and Ferrum ERP repositories on `main`.

The interface lives in `ui/` and is built with Ferrum-Frontend. Check the Frontend out beside the repository as
`../ferrum/web`, install its packages, then:

```bash
cd ui
npm run build       # builds into ui/dist, which the server embeds
npm run check       # interface lint
npm run typecheck
npm test
```

CI (`.github/workflows/ci.yml`) runs Rust on PostgreSQL 17 and the interface checks.

## 📂 Project layout

```text
app.yaml                    manifest: id, roles, shell entries, assistant notes
packages/nonprofit/         entities, roles, personal-data rules and the donation receipt
src/members.rs              membership terms, invoices, standing
src/giving.rs               donations, refunds, fund spending, totals
src/volunteers.rs           approved hours
src/reports.rs              Fund Ledger
ui/                         the home page; words in Arabic and Hindi
tests/nonprofit_it.rs       a year of giving on PostgreSQL
docs/PROVENANCE.md          where the design comes from
```

## 📜 Changelog

Every release is described in [CHANGELOG.md](CHANGELOG.md).

## 🤝 Contributing

Issues and pull requests are welcome at [github.com/CBFerrum/ferrum-app-nonprofit](https://github.com/CBFerrum/ferrum-app-nonprofit/issues).
Keep the checks above green. The design is adapted from ERPNext 13's Non Profit module; see
[docs/PROVENANCE.md](docs/PROVENANCE.md).

## 📄 License

AGPL-3.0-only. See [LICENSE](LICENSE).

<div align="center">

<sub>Part of <a href="https://github.com/CBFerrum/Ferrum">Ferrum</a>, the open business platform · Made with 🦀 Rust and 🐘 PostgreSQL</sub>

</div>
