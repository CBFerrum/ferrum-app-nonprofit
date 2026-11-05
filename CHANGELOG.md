# Changelog

## 0.1.3

Personal-data requests find member, volunteer records by the person's phone number as well as their email, and list the donor, member, volunteer records holding their name apart, for an administrator to judge (never erased for the name alone) (Ferrum's FERRUM-PRIVACY-002).

## 0.1.2

Rebuilt with Ferrum-Frontend daeadda: after an action runs, a record page shows what the action said it did rather than only that it is done.

## 0.1.1

Rebuilt with Ferrum-Frontend 80ff179: a field's label names its control for assistive technology, and a record whose title is a link shows that record's title, not its id.

## 0.1.0

Members, donations, grants and volunteers (FERRUM-INDUSTRY-011).

- Membership types, members and their terms, invoiced through ERP; upkeep counts paid terms, keeps
  each member's standing and lapses those whose terms ended.
- Donors, funds and donations; a submitted donation posts a journal entry and cancelling reverses it.
- Grants from funders with the reports owed, paid by their funder once awarded.
- Volunteers with their skills, and shifts whose approved hours they total.
- Its own interface at `/nonprofit/`: the figures, and the records.
