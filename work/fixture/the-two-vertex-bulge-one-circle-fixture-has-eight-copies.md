---
id: the-two-vertex-bulge-one-circle-fixture-has-eight-copies
kind: issue
title: The two-vertex bulge-1 circle section is hand-written in eight sweep/profile test files, the newest added by a PR whose subject was retiring hand-spelling
status: open
opened: 2026-09-12
refs: [2409]
priority: P4
cost: E
---


## Finding

From the full review of WIRE's PR 2409 (S6, confidence `sure` on the
duplication, `unsure` on the disposition — which is why it is filed
rather than fixed). Filed here by the WIRE orchestrator because
`crates/*/tests/*` is S-TINT's glob and this is a question about what
the suite asserts and how, not about kernel behaviour.

`crates/sweep/tests/wire_loft_end_profile_lift.rs:29-34`'s `circle(r)`
is **byte-for-byte** `crates/sweep/tests/m7_skin_integral.rs:368`'s
`circle_section`. The same two-vertex bulge-1 circle recurs as a
`circle_loop`-shaped helper at:

- `crates/sweep/tests/extrude_acceptance.rs:52`
- `crates/sweep/tests/fillet_h6_cap_rim.rs:106`
- `crates/sweep/tests/p1b_r1_probes.rs:115`
- `crates/sweep/tests/review_m2_pr4.rs:46`
- `crates/sweep/tests/blend6_verb_vocab.rs:65`
- `crates/sweep/tests/review_blend6_r2_probes.rs:264`

Eight copies of one fixture.

## The part worth keeping

It is **pre-existing**, and the newest instance was added by a PR whose
entire subject was retiring hand-spelled constructions — the standing
trap, at the fixture layer this time rather than in `src/`. That is the
fifth time this shape has fired on WIRE's units, and the first at the
test layer, which is worth knowing because every previous instrument
pointed at `src/`.

## What a taker owes, and what it should NOT assume

**Not automatically a shared helper.** `memories/test-suite-cost.md`
asks which SHAPE a test is before giving it a seed, and reviewer-authored
probe suites have an independence-from-the-library argument that a
shared fixture would consume — `review_m2_pr4.rs`,
`p1b_r1_probes.rs` and `review_blend6_r2_probes.rs` are review lanes'
own rows and may be exactly the files that should keep their copy. The
same retain/retire criterion `D385` applies to hand-written lifts
applies here.

So: decide **per file**, and where a copy stays, say at the copy site
why — which is the thing none of the eight currently does, and the
reason a reader cannot tell an independent fixture from an unnoticed
duplicate.

## Re-derived (2026-09-15, lane D)

**VERDICT: PARTIAL** — 7 of the 8 copies live, 1 now delegates to a
homed builder, and the floor rises to **9** with two members the list
did not carry.

**Commands.**

```
grep -rn 'fn \(circle\|circle_section\|circle_loop\)\s*(' crates/sweep/tests/
grep -rn 'fn cylinder(r: f64, h: f64)' crates/*/tests/
```
plus a multiline shape sweep for a two-vertex loop whose both vertices
carry bulge `1.0`, over `crates/` — `ProfileVertex::new(…, 1.0),
ProfileVertex::new(…, 1.0), ]`.

**The eight, by name.**

- `wire_loft_end_profile_lift.rs::circle` — LIVE (now at `:36`).
- `m7_skin_integral.rs::circle_section` — LIVE (`:368`). Note the row's
  *"byte-for-byte"* is no longer literally true: this one is
  `vec![sweep::ProfileLoop::new(vec![sweep::ProfileVertex::new(…)])]`
  returning a `Section`, i.e. the same two vertices under `sweep::`
  path qualifiers. Same fixture, different bytes.
- `extrude_acceptance.rs::circle_loop` (`:53`),
  `fillet_h6_cap_rim.rs::circle_loop` (`:106`),
  `p1b_r1_probes.rs::circle_loop` (`:115`),
  `review_m2_pr4.rs::circle_loop` (`:47`) — LIVE, and the four bodies
  are byte-identical to each other.
- `review_blend6_r2_probes.rs::cylinder` (`:262`) — LIVE.
- `blend6_verb_vocab.rs::cylinder` (`:64`) — **FIXED.** It is now
  `disc_of_arcs(2, r, h, Tol::witness())`, and its doc says so: *"a
  circular prism: two half-arc profile segments extruded … — the homed
  `disc_of_arcs` at two arcs."* The home is
  `sweep::test_support::disc_of_arcs` (over `cylinder_of_arcs_at`), in
  `crates/sweep/src/test_support.rs`, whose own doc names `n = 2` as
  *"the two-semicircle rim the other suites build"* — so a home for this
  exact fixture now exists and one of the eight uses it.

**Two members the list did not carry, so the floor is 9 not 7.**

- `crates/sweep/tests/verbs_chamfer.rs::cylinder` (`:320`) — `diff` of
  the extracted function against `review_blend6_r2_probes.rs::cylinder`
  is EMPTY: byte-identical including the doc comment. Three suites
  therefore spell this one fixture, one of which (`blend6_verb_vocab`)
  has already been converted, so the conversion is already proven.
- `crates/sweep/tests/bitdump.rs::circle` (`:481`, a closure) — its two
  vertex lines are byte-identical to the four `circle_loop` bodies
  (`p2(cx - r, cy), 1.0` / `p2(cx + r, cy), 1.0`).

**Blind spot.** The bulge-1 two-vertex loop as a SHAPE is far more
widespread than this row's class (12 files under `crates/` match the
multiline pattern, and many more with a non-antipodal vertex pair);
most of those are revolve profiles and off-centre discs, not "a circle
of radius r as a fixture", and none of them was counted here. The
count above is the row's own class — a helper or closure returning the
circle — re-derived, not the shape's population.

**Recommend: keep open, correct the count to 9** and record that
`disc_of_arcs` is the home the retain/retire decision is made against.
