# JETFRAME: a rational net's derivative over a cell does not depend on where the body sits

Unit: `work/clear/rect-jet-forms-the-rational-derivative-in-the-absolute-frame.md`
(P0/M). Program: CLEAR. Branch: `clear/jetframe`. Review: a single FULL
review with an exact oracle and mutation. This is an enclosure on the
certificate's soundness path.

Read the row in full. It holds three designers' measurements and two
prototypes.

## What is true when this lands

1. **One home.** The derivative of a rational net over a cell has one
   home, and its width cannot depend on the body's position: it is
   invariant under translation, up to rounding at the new origin.
   - Weigh two shapes in the PR body:
     - recentre each homogeneous control point, `A_i − c·w_i` with `c` a
       point in the cell, before the hull;
     - the convex-combination form already in `geom_core::spline::derivative`
       (the numerator over the weight squared, bounded by control ratios).
   - Pick one and argue that it encloses the true derivative over the
     whole cell: rational, every multiplicity, and SLIVER's span-own-knots
     derivative (`SpanRow::restrict_derivative`) preserved.
   - A recentring point `c` must not need certifying for soundness. Say
     why.
   - `rect_jet`/`restricted_jet` read through the home.
   - **Sweep the class.** The three rows the unit row cites (NURBS `ders`,
     QUAD's patch cross, ENCL's offset recentring) should read through
     the same home. Fix any that is in reach and a few lines; file or
     cite the rest, naming the home.
2. **Rows. Each can go red.**
   - **Translation.** A rational quarter-circle wall at the origin and at
     (3, 3), and the lily wall moved by (10, 10, 10). Each jet's width
     matches its untranslated twin within a stated factor, and the
     certified-cell counts match.
   - **The lily and Klein walls against themselves** hold at the default
     budget at f64 and Interval. Give receipts.
   - **Whole bodies.** Report `lily_leaf_a` and the Klein loop, with
     ZEROSIDE (PR #6234) merged in if it has landed: verdict, pairs, time.
   - **Soundness.** An exact or high-precision oracle on random rational
     nets (weights 1e-3 to 1e3, all multiplicities, cells of 2⁻³ to 2⁻⁴⁰,
     translated by up to 1e3) finds no miss. Extend the `rect_enclosure`
     fuzzer with translation.
   - **Every self-intersection, pipe and cavity row keeps its verdict
     class** at both scalars. Receipts will move, since more cells now
     certify. List the moved receipts, and confirm that no verdict
     changed class.
3. **Mutation.** Each of these must red a row:
   - the fix reverted;
   - recentring the hull after taking it (shown to change nothing);
   - `c` taken outside the cell;
   - the weight-slope term dropped.

## Ground

- `crates/geom/src/surfaces/nurbs.rs` (`rect_jet`, `restricted_jet`,
  `rect_net`);
- `crates/geom-core/src/spline/derivative.rs` and `blossom.rs`
  (`SpanRow::restrict_derivative`);
- `crates/topo/src/injective.rs` (`monotone`);
- the sweep and tour tests.
- ZEROSIDE (#6234) also edits `nurbs.rs` (`rect_net`, `rect_enclosure`).
  Merge main as it lands.
- The designers' prototypes are in their sessions' scratch; take the
  shape from the row, not their code.
- Read `docs/prompts/implementer-discipline.md` in full.
- Never arm a waiter or a background chain for a build or a test.
  Anything that outlives 600 s runs `setsid`-detached and is polled in
  the foreground. Watch the disk.
- CI is the verification of record. Allow 25 min.

## Report

≤100 lines, covering:
- the shape chosen, the alternative, and the soundness argument;
- the translation rows;
- the receipts and whole-body numbers;
- the oracle;
- the class sweep;
- the mutants;
- the PR number and its CI result.
