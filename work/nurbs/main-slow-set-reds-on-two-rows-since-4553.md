---
id: main-slow-set-reds-on-two-rows-since-4553
kind: issue
title: Main's slow set red since PR 4553: the sweep digest moved for 4553's ring refinement and the skin in T (re-cut), and the arc-loft round trip sums its faces in another order (row re-scoped)
status: closed
opened: 2026-10-11
priority: P0
branch: nurbs/main-slow-set-reds-since-4553
closed: 2026-10-11
pr: 4684
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

The serial-golden row's `Sym` lines also move under PR 4662
(nurbs-weights-and-knots-are-the-scalar u2): the `validate_geometric`
rows' `frozen` column is +2 (`sym_arc_loft`) and +1 (`sym_thin_strip`)
against main, because the carriers' knots and weights are held as nodes,
with decisions, discharges, shapes and verdicts unchanged and the `f64`
lines byte-identical to main's red output. Re-cut that golden on a tree
that carries PR 4662.

PR 4708 (`nurbs-weights-and-knots-are-the-scalar` u4) moves the
serial-golden row's `Sym` lines again, against its merge base
`fe323f24d0` (whose own lines had already moved under main's
`from_computed` loft lift, 9618d84fd8). `sym_arc_loft validate_geometric`
goes from `sz=28 num=624 frozen=3706` to `sz=40 num=612 frozen=3732`, and
`sym_thin_strip validate_geometric` from `sz=24 num=636 frozen=3430` to
`sz=36 num=624 frozen=2792`. Twelve more decisions per row discharge as
theorems, because topo now reads the carriers' held domain ends.
Decisions, shapes and verdicts are unchanged, and the `f64` lines are
byte-identical. Re-cut the golden on a tree that carries PR 4708.

PR 4726 (`the-skin-computes-its-values-in-t`) moves the serial-golden
row again. Measured against main's own output for that row
(`work/nurbs/the-arc-loft-thread-count-digest-is-red-on-main.md`'s
table at ε = 1e-9), not the committed digest:
- **`arc_loft_1e9eps`**: `v` and `a` are main's (`…d899`, `…3c57`), and
  only the pads move, `vpad` `3eb0783423f55555` → `3eb0783424400000`
  and `apad` `3fb5fb0bfc12a5e0` → `3fb5fb0bfc12a660`. Its verdicts are
  unchanged. The likely cause is that the skin's arc conversion now
  calls `Real`'s sines and cosines, the pure-Rust `libm` D9 prescribes,
  where it called `f64`'s inherent (platform libm) ones. That moves an
  arc's weight or tangent pole by an ulp on a few per cent of sweeps.
- **The `Sym` lines' `frozen` column** reads 1060 (`sym_arc_loft
  validate_geometric`) and 1177 (`sym_thin_strip`) at the PR's head,
  against the committed 1244 and 1268. Main's own counts were not
  re-measured here. The walls are computed in `T` instead of lifted as
  opaque unknowns, and the fixture placements are constants rather
  than `from_computed` unknowns. Decisions, `sz`, shapes and verdicts
  read as the committed ones.

The import row (`arc_loft_natively_computes_its_rational_volume`)
builds an arc loft too, so it may move for the same libm reason.
Re-cut on a tree that carries PR 4726.

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

**What moved the digest, in full** (re-cut on this branch at all three
ε, after merging main at `18555d6eca`):
- **PR 4553**, above: `arc_loft_1e9eps`'s `v`, `a` and pads. The
  enclosures nest inside the old ones.
- **PR 4662 and PR 4708** (u2, u4), per the notes above: the `Sym`
  rows. u4's movement (`sz=40 num=612 frozen=3732`,
  `sz=36 num=624 frozen=2792`) was never re-cut on main, and the skin
  in `T` undid it.
- **`8054ec236f`** (PR 4726, the skin in `T`), bisected at ε = 1e-9
  against its parent `cb58e63b8b`:
  - `arc_loft_1e9eps`'s pads move by at most 1.1e-9 of themselves, and
    `a` by an ulp at 1e-6. The body moves, by `libm`'s sines and
    cosines, so old and new enclosures nest one inside the other but
    not always new inside old.
  - The `Sym` `frozen` column falls to 1062 / 1060 / 1061
    (`sym_arc_loft`) and 1178 / 1177 / 1177 (`sym_thin_strip`) at
    ε = 1e-6 / 1e-9 / 1e-12, with `sz=48` as committed.
  - The full list is in `expected()`'s re-cut log.

**The import row.** It was resolved on main first by ENCL's PR 4682 (a
`faces − 1` ulp bound on the volumes, the pad bit-identical). This
branch keeps those assertions and adds two bit identities:
- every face's flux term and half-width, native against imported, read
  off the same walk as `mass_properties` through
  `topo::test_support::mass_properties_with_face_terms`;
- the import's gate enclosure against the imported body's own
  measurement.

Summing in the shells' stored face order was tried and backed out: it
reddened two sweep composition rows that agree in arena order and not
in shell order. Whether the fold should be order-independent is FLUX's
design row,
`the-whole-body-property-fold-depends-on-face-order`, closed by PR 5402. That
row also carries the note on PR 4551's
`quad-de-boor-through-the-blossom-breaks-the-import-bit-identity`: the
same fold, with its "inputs differ" premise refuted.
