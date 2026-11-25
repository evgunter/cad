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

## Traced, one representative each (M13's addendum A5 and M13b, 2026-11-24)

Margins at `Interval` on the nominal; band zero 1e-9 and escalate 1e-8
unless said. Causes are inferred from the read site. All but 9 of the
2,445 builds are plain evaluations, documents no suite row drives; no
driven document's zero-width replay escalates (M13b: 347 of 347
references fully decided).

| predicate | builds | the margin | why undecided at a point |
|---|---|---|---|
| `curve_pair_pose_parallel` | 1,314 | `Invalid` | a rate whose divisor encloses zero poisons the margin (`geom-brep`'s `curve_pair/pose.rs`) |
| `germ_side` | 409 | [-6.3e-13, 6.3e-13], a `Zero` refused by a gate that passes nonzero | a definite Zero meets a nonzero gate (`topo`'s `star/order.rs`) |
| `chart_singularity_nearer` | 258 | [-5.6e-17, 5.6e-17] against a band of 2.2e-308 | a tie: two equal distances differ by rounding, against a band no enclosure fits; decided at `Sym<Interval>` (3 builds) |
| `curve_carrier_gap` | 191 | [0, 0.114] | a distance to a foot whose enclosure widened, clamped at 0 (`geom`'s `curves/on_carrier.rs`); one of the hole finish's four |
| `section_site_foot_bend` | 74 | `Invalid` | not traced |
| `pcurve_span_covered` | 40 | [-8.9e-16, 8.9e-16] against a band of 5e-324 | a tie against a band no enclosure fits |
| `curve_pair_pose_clear` | 25 | [-1.00000056e-8, -9.9999944e-9] | straddles the band's edge |
| `seam_cover_crossing` | 20 | [-6.28, 18.85] | not traced |
| `split_join_order_u` | 14 | [-2.4e-15, 2.4e-15] against a band of 5e-324 | a tie against a band no enclosure fits |
| `plane_eq_offset`, `mate_chart_pole` | 13, 8 | wholly in band | terminal slivers |

Three shapes cover most of it: **a margin that is `Invalid`** because a
divisor encloses zero (1,388 builds; the two-parameter plate's junction
margin is the same shape:
`the-two-parameter-plates-volume-refuses-on-a-junction-margin-that-is-not-a-number`);
**a tie read against a band of the smallest positive number**, which no
enclosure wider than a point can sit inside (312); and **a `Zero` a
gate does not accept** (409). Each wants its own fix at its site; this
row is the table. Not measured: `germ_side` at `Sym<Interval>`, and
the `f64` margins (a decided verdict keeps none).

**A loft whose section holds an arc fails at every box scalar with
nothing logged** (3 builds, one document): `Skin` refuses
`CarrierWeightNotAPoint` because the arc's weight `cos(Δ/2)` is
computed at the run's scalar and an interval cosine of a point is not
a point; the test is a structure test, not a decision. INLANE's
`an-enclosure-carrier-holds-point-weights-and-knots` is that ground.
(BOXFINISH orchestrator)

