# Provenance

## ERPNext 13's Non Profit module

- Source: [frappe/erpnext](https://github.com/frappe/erpnext), branch `version-13`, commit 37e00a6
  (13.55.2), `erpnext/non_profit/doctype/`.
- Licence: GNU General Public License v3. This app is under the GNU Affero General Public License v3,
  which may be combined with GPL-3.0 code (section 13 of each licence).
- What was taken, as designs rather than code:
  - its doctypes Member, Membership and Membership Type (with its item), Donor and Donor Type, Donation,
    Volunteer (with skills and availability), and Grant Application, which this app's Grant turns
    around: a grant the organisation asks a funder for;
  - the rule that a member's standing follows their paid memberships.
- Not taken: Razorpay subscriptions, chapters, the website forms, and Donation's payment entry against a
  donor customer. A gift is income, so this app posts it as a journal entry to donation income.
- Its own: funds, a grant's reports and payments, volunteer shifts and hours, and all the code.

The copy read is in `ERP/referance/industry/erpnext-v13` on the machine this was written on (sparse:
`erpnext/non_profit` and `erpnext/education`).
