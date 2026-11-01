# SEAMDIAG: why departure refuses at depth on loft cap corners and twisted seams

Unit: `work/clear/injectivity-departure-refuses-at-depth-on-loft-cap-corners-and-twisted-seams.md`
(P0/H). Program: CLEAR. Branch: `clear/seamdiag`. **This is a
diagnosis unit**, modelled on CAVITYDIAG (#6105). It produces a cause
per locus and a decided fix shape. It does not change the certificate
unless the fix is a few lines at a producer.

Read the row in full.

## What is true when this lands

1. **Each locus is printed and its cause named.**
   - **Cap against wall.** Use `lily_leaf_a` and `lily_leaf_b`, the
     Klein loop's top (`klein::bottle().top`), and `m8_14`'s half-turn
     helix at 33 stations. Print the cells that reach depth 40: their
     chart rectangles on both faces, the presentation each side tried
     and why it failed, and the shared edge's classification there.
     Confirm or refute the row's guess, that this is the plane-edge
     case of
     `work/cleartail/departure-reads-no-curved-corner-off-its-shared-edge.md`.
   - **Twisted seam.** Use `skinned`'s `twisted_loft`, all four seams.
     Print the same, and say why the untwisted `s_duct` seams hold
     while these do not: dihedral variation, the shared `u` iso being
     curved in space, or a presentation that needs the dihedral bounded
     away from π.
2. **Each cause gets a fix shape, weighed.** For each locus:
   - give the smallest sound fix shape;
   - say where it lives (producer or reader, which arm);
   - say what it would need argued (a new arm, an extension of
     CORNER/CURVEDEP, or a producer fix);
   - estimate its size.

   If a locus shares its cause with the sliver-knot row (SLIVER, in
   flight on `clear/sliver`) or the price row, say so.
3. **Rows that pin today's refusal, typed, one per locus,** at both
   scalars, built by the tour's or sweep's own builders. Each row names
   the issue. A later fix flips them.
4. If a locus's fix is a few lines at a producer, with no new arm and
   no ε, you may make it here. Give it rows and mutants, and say so.
   Otherwise, make no change to `injective.rs`.

## Ground

- `crates/topo/src/injective.rs`, `crates/topo/src/clearance.rs`,
  `demos/tour` (the lily, Klein and `skinned` scenes) and
  `crates/sweep/tests`.
- CARVE's scratch is `demos/tour/tests/carve_embed_scratch.rs` and
  `crates/sweep/tests/carve_embed_scratch.rs` on
  `carve/embedding-certificate`. Read them, but do not merge that
  branch.
- Read `docs/prompts/implementer-discipline.md` in full.
- Never arm a waiter or a background chain for a build or a test.
  Anything that outlives 600 s runs `setsid`-detached and is polled in
  the foreground.
- `demos/tour` is an excluded root, so build it from its own directory.

## Report

≤120 lines, covering:
- per locus: the cells, the failed presentation and the cause;
- the fix shapes weighed, with a recommendation per locus;
- the pinned rows;
- the PR number and its CI result.
