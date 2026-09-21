# TESS-1 (ordinal 5100) — dual review, adjudicated 2026-09-20

Frozen head `7a5fe831e`. R1 = OPUS, R2 = FABLE (byte 82). Both
APPROVE-WITH-FIXES; neither falsified any behavioural claim; both
reproduced the guard-removed mutant.

| | MAJ/MIN/NOTE | devs reported / silent | idiom | tests | docs | wall-clock | tokens |
|---|---|---|---|---|---|---|---|
| R1 | 2 / 6 / 7 | 4 / 3 | 4 | 4 | 3 | ~50 min (mostly slot wait) | 258k |
| R2 | 1 / 5 / 5 | 5 / 0 | 4 | 4 | 3 | ~2 h 20 m (over half slot wait) | 234k |

MAJORs, one line each:
- R1 MAJOR-1 (doc/claim): `walk.rs` module doc says the pole premise is
  "enforced, not assumed"; `require_a_meridian` enforces only the
  no-meridian half, the pole-at-a-rim-junction half is a debug-only
  guard. By inspection. R2 raised the same fact as NOTE-2 (measured:
  a short meridian spur is caught later, δ-dependently).
- R1 MAJOR-2 (test-gap): the only executed witness that
  `require_swept_rectangle` admits a zero-height polygon was deleted;
  the fact is now prose at four sites. By inspection. R2 raised it in
  Style (Q3, "executed → argued").
- R2 MAJOR-1 (contract/API): the variant files itself under D2 addendum
  row 2, whose naming rule is `Unsupported*`; the public python tag
  freezes the name. By inspection. R1 raised it as MINOR-3.

**Tally candidates: none** — every MAJOR was mentioned by the other
reviewer (item 3a fails), and none was demonstrated by execution (3d).
R1's "3 silent" are DESIGN/behaviour-adjacent rather than spec-letter by
its own account; R2 counts 0 silent against the spec. Recorded as
R1 3 (2 spec-adjacent, 1 understated disclosure) / R2 0.

Method asymmetry to record: R1 did NOT build the assertions-off profile
(disk fell to 13 G; the brief's floor was 8 G, so this was R1's own
economy, not a granted relaxation); R2 did. Both briefs were identical.
One isolation glimpse each, names only (R1: a directory listing showed
the other lane's directory and brief file names; R2: none of the other
lane). No findings were exposed; pair treated as FAIR.

Orchestrator's own error surfaced by both: the SPEC told the lane to
file the refusal under row 2 ("valid input, unbuilt lane"). Ruling (N)
landed mid-unit and makes the face row 1 (invalid input). The fix pass
re-files the doc under row 1 and keeps the name.

Fix pass: union of both reports, executor implementer-inherited.
