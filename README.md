# Nonprofit

Members and memberships, donors and donations posted to the books, funds, grants from funders, and
volunteers for [Ferrum](https://github.com/CBFerrum/Ferrum). Invoices and the ledger are ERP's.

| | |
|---|---|
| 🤝 App | `nonprofit`, native, in the server |
| 🧩 Package | `nonprofit` (needs ERP's selling and accounting) |
| 🖥️ Interface | its own, at `/nonprofit/`: the figures, and the records |
| 🔐 Roles | Nonprofit Manager, Fundraiser, Volunteer Coordinator |
| 📜 Licence | AGPL-3.0-only |

## Highlights

- **Memberships count once paid.** A membership term takes its amount and length from its type, and
  is invoiced to the member's customer, made from the member when they have none. Upkeep marks it
  paid once its invoice is submitted and settled. The member is active while a paid term covers today,
  and lapsed once the last has ended. An invoiced term does not change: cancel it and make another.
- **Donations are posted to the books.** Submitting one posts a journal entry: the account donations
  are received into is debited, and donation income credited. Cancelling it reverses the entry. Its
  donor's, fund's and grant's totals follow.
- **Grants** from funders go from draft through submitted to awarded, declined or closed, with the
  reports owed. A grant is paid only once awarded, and only by its funder.
- **Volunteers' hours** are the sum of their approved shifts, kept as shifts are added, approved,
  moved and removed.
- **Personal data** requests find members, donors and volunteers by email and erase their names and
  contacts. Amounts and dates stay, as the books need them.

## Develop

```bash
cargo test                       # needs FERRUM_NONPROFIT_TEST_DATABASE_URL's database
cd ui && npm run build && npm run check
```

`tests/nonprofit_it.rs` walks a year:
- a membership invoiced, paid through ERP, active, then lapsed;
- a grant awarded and paid, and donations posted and one reversed;
- volunteers' hours;
- who may do what.

## Not yet

- Recurring giving, gift aid and tax receipts as documents, online donation and membership pages, and
  payment gateways.
- Chapters, events and campaigns (Campaigns has campaigns).
- Fund accounting with restricted net assets in the ledger: funds total their donations, and the books
  carry one income account.

## Changelog

| Version | Highlights |
|---|---|
| **0.1.3** | Personal-data requests find member, volunteer records by the person's phone number as well as their email, and list the donor, member, volunteer records … |
| **0.1.2** | Rebuilt with Ferrum-Frontend daeadda: actions say what they did. |
| **0.1.1** | Rebuilt with Ferrum-Frontend 80ff179: labels name their fields, and linked titles show their titles. |
| **0.1.0** | Members and memberships, donors and donations posted to the books, funds, grants and volunteers (FERRUM-INDUSTRY-011). |

## License

AGPL-3.0-only. See [LICENSE](LICENSE).
