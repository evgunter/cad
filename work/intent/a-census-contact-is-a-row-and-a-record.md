---
id: a-census-contact-is-a-row-and-a-record
kind: ruling
title: A contact the at-rest census finds between groups no op joined is a row and a record citing it; what the quieting assertion reads
status: open
opened: 2026-10-11
priority: P1
needs_ev: true
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
