---
id: main-slow-set-reds-on-two-rows-since-4553
kind: issue
title: Main's slow set red since PR 4553: the sweep digest moved for 4553's ring refinement (re-cut), and the arc-loft round trip sums its faces in another order (row re-scoped)
status: closed
opened: 2026-10-11
priority: P0
branch: nurbs/main-slow-set-reds-since-4553
closed: 2026-10-11
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
one. (A first bisection read "passes at the merge" off a worktree in
which cargo reused the parent's artifacts. Every build that compiled
agrees with this.)

**The digest moved for a stated reason, and is re-cut on this branch.**
Only `arc_loft_1e9eps` moves, at every ε: the one body in the roster
whose walls are rational patches. 4553 moved those patches' knot
refinement (`props::quad`'s `refine_dir`) from `ring_lerp`, which
combined points through the plan's rounded `f64` λ, into the
certification ring's convex step. Every new volume and area enclosure
lies inside the old one. The volume pads fall by 7e-6 of themselves and
the area pads by 4e-10.

**The import bits differ in the face SUM's order, not in the
quadrature.** Fingerprinting the quadrature's inputs and outputs per
face, native against imported:
- every face hands the lane identical bits (knots, net, weights,
  rectangle, perimeter, defect);
- every face's flux comes back identical (the rational wall
  `[9.738934194323258, 9.738940253487032]` both ways);
- the body sum (`props::fold_runs`, face-arena order) adds the same six
  terms in two orders, native `[9.14, 0, 6.2, 9.7389…, 6.2, 6.2]`
  against imported `[9.14, 9.7389…, 0, 6.2, 6.2, 6.2]`, totals
  `37.48052987749494` against `…493`.

The reader's Euler-operator assembly allocates faces in its own order.
The `CLOSED_SHELL` list it rebuilds (`Shell::faces`) IS the native
arena order. 4553 changed the rational face's flux bits, and with them
whether the two orders happen to round alike. No enclosure was lost:
this is not a soundness finding.

**Resolved on this branch by re-scoping the row**, as ENCL's repair (b)
proposed (`work/exch/arc-loft-round-trip-volume-differs-by-face-order.md`):
- bit identity is kept where it holds: the import's gate enclosure
  against `topo::mass_properties` of the imported body, the same body
  in the same order;
- native against imported is bounded by the recursive sum's reordering
  error, `2(n − 1)·u·Σ|xᵢ|`, which is `Σ|xᵢ| = 3V` here because every face
  term is non-negative, plus an ulp per side for the `/3`.

Summing in the shells' stored face order was tried first and backed
out: it reddened two sweep composition rows that agree in arena order
and not in shell order. An order-independent fold is FLUX's question,
`work/flux/whole-body-volume-fold-depends-on-face-order.md`.

PR 4551's `quad-de-boor-through-the-blossom-breaks-the-import-bit-identity`
(on its branch) reads as the same mechanism: one ulp on the same body,
pads identical. Its premise that "the two walks hand `de_boor` inputs
that differ" is what this measurement refutes: the inputs agree, and
the sum's order differs.
