---
id: main-slow-set-reds-on-two-rows-since-4553
kind: issue
title: Main's slow set is red on two rows since PR 4553 (knot algebra off the ring): the sweep serial-golden digest and the arc-loft import volume bits
status: closed
opened: 2026-10-11
priority: P0
closed: 2026-10-11
branch: nurbs/main-slow-set-reds-since-4553
---


Two rows of the slow set (the nightly's; CI's per-PR `ci` profile
filters them out) fail on main since PR 4553's merge (98e620a195,
`nurbs/knot-algebra-off-the-ring`). INTENT's stage 3 A lane reported
that both pass on its parent. The INTENT orchestrator reproduced both
failures on main at 80fa908950 with
`cargo nextest run -p sweep -p step-import -E 'test(the_walk_matches_the_serial_golden_at_four_threads) | test(arc_loft_natively_computes_its_rational_volume)'`:
2 run, 2 failed.

- `sweep::all mass_props_are_thread_count_invariant::the_walk_matches_the_serial_golden_at_four_threads`:
  the serial-golden digest moved
  (`crates/sweep/tests/mass_props_are_thread_count_invariant.rs:471`,
  `check_against_golden`).
- `step-import::all nurbs_import::arc_loft_natively_computes_its_rational_volume`:
  the imported solid's certified volume is no longer bit-identical to
  the native one (`crates/step-import/tests/nurbs_import.rs:373`).
  The row pins bit identity on purpose (R1 MINOR-2), so the reader's
  reconstruction and the native build now run different quadrature
  bits. That is a finding, not a re-pin, until the divergence is
  explained.

Owed: root-cause both against 4553's change. If a digest moved for a
stated reason, re-baseline it and say what moved. If the bits now
differ because the two sides compute the knots differently, make them
compute them the same way.

## Root-caused (NURBS, branch `nurbs/main-slow-set-reds-since-4553`)

Both reds start at PR 4553's merge `98e620a195`: its parent
`afabd44061` reads the old digest line, and the merge reads the new
one.

**The digest moved for a stated reason, and is re-cut.** Only
`arc_loft_1e9eps` moves, at every ε: the one body in the roster whose
walls are rational patches. 4553 moved those patches' knot refinement
(`props::quad`'s `refine_dir`) from `ring_lerp`, which combined points
through the plan's rounded `f64` λ, into the certification ring's convex
step. Every new volume and area enclosure lies inside the old one. The
volume pads fall by 7e-6 of themselves and the area pads by 4e-10.

**The import bits differed in the face SUM's order, not in the
quadrature.** Fingerprinting the quadrature's inputs and outputs per
face, native against imported: every face hands the lane identical
bits (knots, net, weights, rectangle, perimeter, defect), and every
face's flux comes back identical. The body sum then added the same
six terms in different orders. `mass_properties` and check 7 summed in
face-ARENA order. The native body's arena is its shell's face list, but
the reader's Euler-operator assembly allocates faces in its own order,
and the `CLOSED_SHELL` list it rebuilds (`Shell::faces`) is the native
list. So: native `[9.14, 0, 6.2, 9.7389…, 6.2, 6.2]` against imported
arena `[9.14, 9.7389…, 0, 6.2, 6.2, 6.2]`, totals one rounding apart.
4553 changed the rational face's flux bits, and with them whether the
two orders happen to round alike. No enclosure was lost; this is not a
soundness finding.

Fix: the whole-body walks sum in WALK order (`Body::faces_in_walk_order`:
solids, each solid's shells as listed, each shell's faces as listed).
That order is the body's data, and a STEP round trip carries it. The
call sites are `mass_properties`, the closed-form door, check 7's
subjects, the assembled body certificate and the boolean volume
backstop. A body built with its shell lists in arena order, as every
fresh build in the slow sets is, keeps its bits.

PR 4551's `quad-de-boor-through-the-blossom-breaks-the-import-bit-identity`
(on its branch) is the same cause: a one-ulp change in the rational
face's flux, summed in two arena orders. This fix closes that too.
