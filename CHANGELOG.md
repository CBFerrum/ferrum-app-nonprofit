# Changelog

## 0.1.13

Rebuilt with Ferrum-Frontend 5b22e7b: its own words carry built-in Arabic and Hindi (11 words each, ui/src/locales, translated by the model and to be reviewed by native speakers), fetched for the page's language only. The labels it keeps in objects, its dialogs, its page titles and the words it keeps in variables go through t() as well, and the names and sizes it shows as they are keep their order right to left. Its pages are drawn once their words are in, and its entry carries only the parts of the kit it uses.

## 0.1.12

Rebuilt with Ferrum-Frontend 6537d65: an action's link, code or token shown once is kept in a dialog to copy, and on a page that reads right to left, English left untranslated keeps its own order. Its own words go through the kit's t() (5 places), so a workspace's own words for its language reach them, and Admin, Interface words lists them under it (dist/words.json).

## 0.1.11

Rebuilt with Ferrum-Frontend d5d98f2: a workspace's own words for its interface (Admin, Interface words) reach its pages as they load; an attachment field that holds a picture shows it; and a value another record already has is said so, where the form said the record had been changed by someone else.

## 0.1.10

Rebuilt with Ferrum-Frontend 63280d3: right to left, its pages mirror as they read (icons that point along the line, margins and alignment by the line's start and end, arrow keys along it), and the records pages' words are in Arabic and Hindi.

## 0.1.9

Rebuilt with Ferrum-Frontend a64416a: a new record's box whose default is 1 shows ticked, as it is saved. An action's dialog says every refusal, the errors on what the action made (another record's rows) among them, and an action with parameters and a confirmation asks once.

## 0.1.8

What walking a year's giving through the pages found:
- The home page's figures were counted and added over one list of at most 500 records (it asked for 1,000; the server pages at 500). A larger organisation saw a short "Given in" and short member counts. Members are now counted, and donations and hours added over every page.
- "Given in" took the browser's year; it now takes the workspace's.
- A member invoiced but not yet paid was in no figure. "Members not yet paid" counts them.
- Before the books are named, the home page says to name them in Nonprofit Settings first, since donations post there.
- New memberships, donations and volunteer shifts are dated today unless another day is chosen.

## 0.1.7

Rebuilt with Ferrum-Frontend a76295e: its pages show times on a person's own clock when they chose one on their Account page, while today and due dates stay on the workspace's day.

## 0.1.6

Rebuilt with Ferrum-Frontend 24c3951: its pages show times on the workspace's clock, naming the zone when it is not the browser's, a draft numbered on submit is headed as a draft, and a record of an app that is not installed says so.

## 0.1.5

Rebuilt with Ferrum-Frontend bb60115: a record page reads an empty number or check as the server does, so an action whose condition needs one shows, and a workspace's settings open as their one record.

## 0.1.4

Rebuilt with Ferrum-Frontend 00dea07: in a workflow state that names what may change, the other fields show read-only, and the fields the state needs show required.

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
