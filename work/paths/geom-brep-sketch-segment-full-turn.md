---
id: geom-brep-sketch-segment-full-turn
kind: unit
title: geom-brep's SketchSegment takes the canonical arc form; certify, topo description readers and the symbolic tier re-keyed on Δθ
status: dispatched
opened: 2026-09-25
priority: P1
cost: H
parent: lower-profiles-to-carrier-and-interval-not-vertex-and-bulge
branch: claude/clever-bardeen-4itqb3
---


Unit 2 of the #3218 lowering. `SketchSegment::Arc{a,b,bulge}` becomes the profile's arc form; `restrict` becomes Δθ·(s1−s0). Certification, the topo description readers (`split.rs`, `offset_axial.rs`, `replace_face.rs`, `transform.rs`), and `geom-core` sym/trig re-key from 4·atan b to Δθ (DECIDE's `rule-d-reaches-the-unit-bulge-only` is reshaped; announce there). Byte-identical for partial arcs. Measure the `m10_*_interval` rows. Survey §1d, §3(g).

Unit 1 found that tan(Δθ/4) does not round-trip the bulge: b = 1.0
comes back 0.9999999999999999. So `swept::sketch_segment`, `arc_span`
(`param_end`, `axis_arc_span`), `arc_apex` and `axis.rs::radial_extent`
still read the kept bulge. This unit moves them to Δθ along with
`register_span_identity` and the sym rules, and re-baselines what moves.

`arc_span`'s equality to |sweep| has only been shown at f64. At
Interval, `|4·atan b|` and `4·atan|b|` are different enclosures on a
box that straddles zero, so the move to Δθ has to decide which one the
span is (and what `register_span_identity` is stated about). Found in
review of #3224.

## Spec (PATHS orchestrator, 2026-09-25)

**Goal.** geom-brep's `SketchSegment` (in `mapped.rs`) takes the
canonical arc form, so that no bulge crosses from `profile` into
geom-brep. The form is `Line { a, b }` or
`Arc { a, b, centre, radius, sweep: Δθ }`. Endpoints stay stored
verbatim, as in the profile.

**Evaluation and restriction read the carrier and Δθ:**
- `eval(s)` gives centre + radius·(cos, sin)(θ₀ + s·Δθ), where θ₀ is
  the start angle;
- `restrict(s0, s1)` gives Δθ·(s1 − s0).

Endpoints still come back verbatim at s = 0 and s = 1: that is the
he_plus / vertex-authority contract, so keep the exact-endpoint
special case.

**Readers that move off the kept bulge in the same unit:**
- sweep: `swept::sketch_segment`, `placed_segment_spec`, `arc_span`,
  `arc_apex`, `cap_points`;
- revolve: `axis.rs` `radial_extent`, `axis_arc_span`, `axis_arc_apex`;
- `skin::segment_curve`, one of the seven hand copies. It builds the
  loft's NURBS from the carrier;
- certification in `geom-brep/src/certify.rs`;
- the topo description readers: `split.rs`, `offset_axial.rs`,
  `replace_face.rs`, `transform.rs`;
- geom-core's symbolic tier (`sym.rs`, `sym/trig.rs`) and
  `register_span_identity`, re-keyed from `4·atan b` to Δθ.

**Decide and state the Interval span.** At Interval, `|4·atan b|` and
`4·atan|b|` differ on a box that straddles zero. With Δθ stored, the
span is `|Δθ|`. State what `register_span_identity` is about now, and
check that DECIDE's rule D still fires where it did. That rule is
`work/decide/rule-d-reaches-the-unit-bulge-only.md` or its successor.
Put an announced-seam note on DECIDE's log.

**Expect bits to move, and say exactly which.** This unit is NOT
byte-identical by construction. `tan(atan(b)·t)` against `Δθ·t`, and
chord-plus-bulge evaluation against carrier evaluation, differ in
ulps. The rules for re-baselining:
- **Vertices never move**, because endpoints are verbatim.
- **Goldens and K CSVs that move** are re-baselined in the PR, each
  with a one-line reason and the size of the move.
- **k-lint:** if the gate fires, follow the K-REPORT runbook. Never
  bend geometry to silence it.
- **A decision that flips** (a verdict, a refusal, a topology count)
  is NOT a re-baseline. Stop and report it.
- **The lily and the tour** (D9): report whether they moved, and
  re-baseline them if the new output is right.

**Units 5 and 3 are not this unit.** Do not store constructed carriers
here. The profile's stored carrier is still `seg::arc_carrier`'s
derivation, and this unit only carries it across the boundary. Full
turns stay refused.

**Review tier: dual.** This is the numeric representation every edge
built from a profile is minted through, and it is shared with
certification and the symbolic tier. It is broad and hard to reverse.

**Claims to falsify:**
- (C1) Every moved bit is disclosed, with its reason. No decision
  flips anywhere in the suite, including the corpora and the
  interval/Sym lanes.
- (C2) Endpoints are verbatim at s = 0 and s = 1 in every evaluation
  path, including restricted sub-segments.
- (C3) After this unit, nothing in geom-brep, topo or sweep reads a
  bulge.
- (C4) The Interval span and `register_span_identity` are stated
  correctly, and rule D still fires where it fired before, or its
  change is disclosed.
- (C5) Certification still certifies every edge it certified before.
