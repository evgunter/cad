---
id: the-box-lane-escalates-at-the-nominal-point-where-f64-decides
kind: issue
title: At a zero-width box on the nominal the box lane escalates where f64 decides, in a fifth of the suite's builds and half the tour's: such a document has no leaf that decides
status: open
opened: 2026-11-24
priority: P1
cost: H
---


Found by M13 (the build-record measurement, 2026-11-24;
`probe/M13-REPORT.md` on `boxfinish/probe-build-record` at
`447298e0b`), on main `aebe03a2a`, default ε.

Every top-level `f64` evaluation of `editor-core`'s 3,302 tests and
the tour's 98 was replayed at `Interval` under the same options, on a
zero-width box or none: 11,123 distinct builds.

- **The box lane holds an escalation at the nominal point in 2,394 of
  11,028 builds (22%), and in 51 of the tour's 95 (54%).** First
  escalated predicate: `curve_pair_pose_parallel` 1,314, `germ_side`
  409, `chart_singularity_nearer` 258, `curve_carrier_gap` 191. A
  document whose replay escalates at a zero-width box has no leaf that
  decides, at any width, under any certificate.
- **251 builds differ in a node's standing:** a node that builds at
  `f64` and fails at `Interval` at the nominal (Boolean 152, Split 59,
  Revolve 10, Skin 7, Fillet 5). All but 5 carry an escalation; a
  loft's `Skin` fails at the box scalar with none logged (3).
- On the rows named for the corpus, 166 of 370 builds hold a box-lane
  escalation at the nominal.

**Not measured:** each predicate's margin and why it does not decide at
a point (an identity read as an enclosure, a band, a reading that loses
its domain); how many are `Sym<Interval>`'s too (only the 67 drive
witnesses were replayed there); which are already held by an open row
(`curve_carrier_gap` is one of the hole finish's four).

It is this program's own subject at its narrowest: before a box has
any width. The first step is the table by predicate with one traced
representative each. (BOXFINISH orchestrator)
