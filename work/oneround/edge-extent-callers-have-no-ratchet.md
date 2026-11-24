---
id: edge-extent-callers-have-no-ratchet
kind: issue
title: no gate holds new callers of edge_extent_inf/sup, the doors a point-read datum keeps reaching for
status: open
opened: 2026-11-23
priority: P3
---


Found by `oneround/reach-from-the-read-point` while it moved point-read data off `geom_brep::edge_extent_inf` (`work/oneround/diameter-levers-of-a-datum-read-at-a-point.md`).

**What a gate could hold.** A ratchet in the ledger form `lever-doors.sh` and `cover-kind-match.sh` already share (`scripts/gates/pin_ledger.py`):

- It would count the production references of `edge_extent_inf`, `edges_extent_inf` and `edge_extent_sup`, pinned per file. A pin only falls.
- So a new caller reds and has to argue, at review, that its datum is one of the whole span (the doors' docs after the unit).

**Why it was not added there.** The count is exact for the references, but not for the class.

- Whether a datum is read at a point is a reading of the code, not a pattern: a whole-span length, a pair of directions, a tube.
- So the gate would hold "no new caller unread", not "no point-read datum at a diameter".
- It also has a cost: the left callers (the spline lanes', `conic_bend`'s, the import's `faces_run`) become pins that a later unit has to lower as it converts them.

That trade is a choice of what the ledger holds, so it is filed rather than made inside a conversion unit.
