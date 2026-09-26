---
id: private-extruded-wrappers-over-test-support-extruded
kind: issue
title: Ten sweep suites keep a private extruded(loops, h) that is sweep::test_support::extruded on the xy plane
status: open
opened: 2026-09-26
priority: P4
cost: E
---


## Finding

- **Where**: `crates/sweep/tests` — `bool1_fix_pass.rs` (~:40),
  `bool1_r1_probes.rs` (~:45), `bool1_r2_probes.rs` (~:19),
  `p1b_r1_probes.rs` (~:110), `shellfix1_r1_probes.rs` (~:52),
  `verbs_shell_r2b.rs` (~:67), `review_fillet_h7_r1_probes.rs` (~:73,
  takes its plane), and two variants: `verbs_shell_r2_probes.rs`
  (~:88, returns `Option`, so it is a fallible door, not a copy) and
  `review_closed_chain_junctions_r1_probes.rs` (~:317, tangent joints
  on a single bulge loop); `m5_s11_concave_sense.rs` `extruded_twin`
  (~:542) is the same body again.
- **Confidence**: sure; each read.
- **Raised by**: the `dup/sweep-topo-drain` fix pass, 2026-09-26, while
  confirming at HEAD that `mate2_common::extruded` was gone
  (`git grep 'fn extruded'` over `crates/sweep/tests`).

Each is `Profile::new(plane, loops).validate(tol)` then
`extrude(.., Distance(h), tol).body` — the six lines
`sweep::test_support::extruded(plane, loops, h, tol)` is, with the
plane fixed to `SketchPlane::xy()` and the tolerance to
`Tol::witness()`; they differ only in their `expect` messages. PR #3284
folded `mate2_common`'s copy (which lifted the plane) onto
`extruded(sketch_at(z0), ..)`.

## What a taker owes

Fold the seven exact copies (and `extruded_twin`) onto
`test_support::extruded(SketchPlane::xy(), .., Tol::witness())` or
`sketch_at(0.0)` (measured `Debug`-equal to `SketchPlane::xy()` in
that PR), per call site or through one named xy wrapper in
`common` — about forty call sites. Keep `verbs_shell_r2_probes`'
fallible one unless a fallible door is added beside `extruded`; decide
whether `review_closed_chain_junctions`' tangent-joint loop is a loop
concern or an extrusion one.
