---
id: a-census-contact-is-a-row-and-a-record
kind: ruling
title: A contact the at-rest census finds between groups no op joined is a row and a record citing it; what the quieting assertion reads
status: closed
opened: 2026-10-11
priority: P1
closed: 2026-10-26
---


From INTENT stage 4 PR J (`an-unattributed-contact-at-rest-is-a-finding`).
Weighed by two designers (fork-log row 113), who converged on the
substance.

## The question

J turns A5's hard error on an unattributed contact between copies into
an `unproven-coincidence` finding. If that contact becomes only a
finding (a row on the product) and no contact record on the body, any
later census over the body finds it again. There it sits inside one
copy, which D1 (iii) refuses as that op's defect. Two concrete cases:
- a product placed as a part in a parent document;
- a touching STEP assembly, once `ImportOptions::declared_contacts`
  retires.

## What both designers recommend

The census is the deciding operation for groups no op joined: the
copies of a product, or an imported file's instances.
- Each contact it finds between two such groups is a `CensusAtRest`
  row, the finding the lint reports.
- It is also a contact record on the result, citing that row.
- A later census confirms the record and never decides the touch
  again.

One kernel door does this for the gather, for import, and for an op's
own gate. Inside one op's result, an unrecorded touch stays that op's
defect.

## What waits on Ev

D10's quieting assertion (FORK-S5Q, ratified on PR 4320) reads a `Gap`
"over an opposed pair of faces of the two copies". Two kinds of contact
cannot be quieted under that wording:
- a vertex–vertex or vertex-on-face contact, which is import's shipped
  corpus class;
- a contact between two solids inside one copy at the asserting level,
  as with an imported file or a nested part.

The diff re-words it so a contact is quiet when the assertion's `Gap`
is taken between the finding's two cells, at any dimension and
wherever they lie. Interference keeps the face-pair reading. D1 (iii)
is re-stated to say the census records what it decides; that text was
agent-written and ratified with PR 3990.

The build proceeds on the recommendation meanwhile. The kernel door
is unit `the-census-records-what-it-decides`, then J.

**Ev's answers (PR 4666, 2026-10-19).** The cell-reading quieting wording: "1. this sounds good". A census per space, after Ev asked how this interacts with PR 5312's spaces: "the second seems solidly better, yes". The exact rule is in D10 `[assertions]` (fork-log row 164, pair `a-space-decides-its-own-contacts`).

**Units that follow** (filed as their own items):
- `each-space-runs-its-census-between-arrivals`: the graft returns a space's aggregate with its arrivals; `record_at_rest` takes that one value (its `groups` closure retires); a census per space with two or more arrivals, memoised with the space's inputs; findings reported once as the deciding space's; spaces outside the product checked too.
- `a-placement-carries-its-sources-at-rest-result`: generalises `a-part-drops-its-census-findings-at-the-seam` from the instance seam to every placement.

**Sequencing (Ev, PR 4666, 2026-10-19: the confirmation inside an arrival "might be ok as a transitional state if explicitly scheduled to be removed when boolean etc are fixed").** D10 states the final rule: inside an arrival the census decides nothing and reads nothing. Until every door certifies its own result, `each-space-runs-its-census-between-arrivals` keeps a transitional check inside each arrival. It confirms the arrival's carried records, matches carried findings, and reports a touch matching neither as that arrival's door's defect, typed and never a panic. That check is removed by `the-in-arrival-census-check-retires`, which is blocked on the doors it stands in for. `an-escalated-census-finding-names-no-pair` is a prerequisite of matching an escalated carried finding while the check stands.

**Ev (PR 4666, 2026-10-19).** Spaces that never reach the world are checked: "1. this is good". The in-arrival confirmation: transitional only, scheduled for removal (above).

## Closed

2026-10-26 (ATREST orchestrator). Every question on this ruling is answered: Ev approved the cell-reading quieting wording and the census per space on PR 4666 (2026-10-19, quoted above), and D10 `[assertions]` carries both. The units it names are filed and owned by INTENT (`each-space-runs-its-census-between-arrivals`, `a-placement-carries-its-sources-at-rest-result`, `the-in-arrival-census-check-retires`).
