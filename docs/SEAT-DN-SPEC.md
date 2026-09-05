# SEAT-DN — direction normalization: one door, two names (unit spec)

Executes Ev's ruling (B) on `[ev]` PR 1902 (2026-09-05) for
`work/seat/direction-normalization-two-doors-one-home.md` (SEAT-DV's
dual-review residue, issue 1570). Faithful elaboration of a ruling;
self-merges. Any shape that would move a funnel-site NAME or a K-REPORT
row is a deviation from the ruling and is Ev-gated — STOP and report.

## DN-1 — one body, in the kernel seat, site-parameterized

`topo::query` owns the direction-length decision: ONE function
(`unit_direction`-shaped) that gates finiteness through
`is_finite_length`, decides `Margin::norm3(v)` against the band under a
funnel-site name PASSED BY THE CALLER, and normalizes-or-refuses typed
— exactly the six lines that today live twice. `UnitVec3::new` becomes a
call to it under `DATUM_UNIT_NORM` (its public contract, refusal type
and `datum_unit_norm` emission unchanged — SEAT-DV's pins stay green
untouched). `editor-core::eval::wire::unit` becomes a call to it under
`eval_direction_norm`, mapping the kernel refusal onto the existing
`NodeErrorKind::{NonFiniteDirection, DegenerateDirection, Escalated}`
arms byte-identically (pin the mapping: every arm reachable, every
`role` string preserved). The site name is a `&'static str` parameter,
never a stored field and never an enum the kernel must extend.

## DN-2 — the two names, ratified where they are read

No K-REPORT row moves; no census count re-baselines; the funnel-site
name census (`m4_pr2_wire`, `m10_3_driver_interval`, K-REPORT §…) is
green untouched — that is the ruling's whole point and the unit's
prime directive. The prose that currently says "two doors, not one"
(`unit()`'s doc), "tolerated"/"issue 1570" (`mate/solve.rs:305-325`)
and the K-REPORT paragraph on one triple under both names are
rewritten to say RATIFIED, with the reason (which layer owns the value
decides the name) and a pointer at the ruling. `MATE-1`'s collapse
record is restated, not lost.

## DN-3 — `profile::path::Dir<T>::from_unit`

The 2-D director's unvalidated-ray constructor gets issue-1527's
treatment: either derive the ray from the angle door (the unit-by-
construction spelling — preferred if every caller has the angle) or
validate through the one kernel body at a site name of its own ONLY if
a caller genuinely holds a ray and not an angle — in which case that
new site is a K-REPORT carrier row and the unit measures and says so
(SEAT-DV's `datum_unit_norm` precedent). Measure the callers first;
report which case holds.

## Acceptance

- One body in the tree for the direction-length decision (grep: the
  decide/normalize/refuse six-liner appears once); both names emitted
  exactly where they were (the census green untouched); every existing
  pin green untouched; both feature graphs; red-first: deleting the
  finiteness gate in the one body reds SEAT-DV's overflow rows AND the
  FIX program's `unit()` rows (both families now guarded by one line).
- `Dir::from_unit` measured and settled per DN-3, pinned.
- The item closed in the PR with a `## Closed` pointer; lint green.

## Out of scope

Renaming either funnel site (ruled out — (A) was declined); migrating
the revolve axis / face normal / transform axis onto `UnitVec3` (each
declined per site in SEAT-DV; record per-site reasons only if this
unit touches them anyway); `datum_unit_norm`'s K-REPORT row.
