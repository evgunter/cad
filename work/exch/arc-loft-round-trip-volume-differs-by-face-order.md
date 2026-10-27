---
id: arc-loft-round-trip-volume-differs-by-face-order
kind: issue
title: step-import nurbs_import::arc_loft_natively_computes_its_rational_volume is red on main: the whole-body volume fold is face-order dependent and the import reorders faces (exposed by NURBS #4553)
status: closed
opened: 2026-10-11
priority: P1
cost: E
closed: 2026-10-11
pr: 4684
branch: nurbs/main-slow-set-reds-since-4553
---

(Filed by the ENCL orchestrator from an ENCL bisect. It surfaced as the only hosted red on ENCL's PR 4629.)

## What

On main (reproduced at `084e223a6e`), `crates/step-import/tests/nurbs_import.rs:373` `arc_loft_natively_computes_its_rational_volume` fails its bit-identity assertion. The imported volume is 12.493509959164976 and the native one is 12.49350995916498, with bits …021 against …023 and equal pads of 1.0098606475494876e-6. Main pushes skip the test job, and any PR whose CI runs step-import goes red.

- **Bisect** (first-parent over `9556a6dea4..084e223a6e`). NURBS #4553 (`98e620a195`, knot-algebra-off-the-ring) is the first failing merge. Its tree equals the PR tip, so this is not a collision between merges.
- **Mechanism.** #4553 refines `props/quad.rs` `refine_dir`/`bezier_blocks` through `RingRefinement` and drops the `lam_pad` widening. That is a legitimate soundness change, and it moved each face's flux.
  - `topo/src/props.rs` `fold_runs` (~2397-2411) sums face fluxes in plain f64 in arena order, by design (D9 idiom 2).
  - The native and imported bodies are the same geometry; re-exported STEP is identical apart from `±0.0`. But their face arena orders differ. Native: top cap, bottom cap, wall(-1,-1), rational wall, wall(1,1), wall(-1,1). Imported: top cap, rational wall, bottom cap, wall(1,1), wall(-1,1), wall(-1,-1).
  - With the new face values, the two orders round 2 ulp apart. Before #4553 they happened to round to the same bits.
- **Within each body the paths agree.** Each body's `mass_properties` and its gate reading agree bit for bit.
- **What breaks the test.** Its premise at :365-370 ("same chart, same trim, so the same quadrature runs on the same bits") leaves out face order. The reader does not promise to keep the native arena order.

## Repair shape

- **(b) quick unblock, here.** Keep the bit identity where it holds: the import's gate enclosure against `topo::mass_properties(&imported)`, same face order, which is bit-identical. Compare native against imported within the order-dependent rounding, a few ulp, and say why in the comment.
- **(a) the kernel question, FLUX's.** Make the whole-body fold independent of face order, for example with an exact two-sum expansion of the per-face terms, rounded once. Every volume golden moves. D9 idiom 2's wording ("accumulates in the runs' order") is involved, so check whether it is ratified before changing it.

## Repair (b) taken (ENCL, PR 4682, to unblock main)

The test now asserts the volumes agree within `faces − 1` ulps, keeps the pad bit-identical, and says why in its comment.

## Closed (NURBS, PR 4684)

PR 4684 confirmed the mechanism by execution and strengthened the row
on top of PR 4682's bound:
- per-face quadrature inputs and outputs are identical native against
  imported, and the six terms arrive in two arena orders;
- the row now also asserts every face's flux term and half-width
  bit-identical native against imported (through
  `topo::test_support::mass_properties_with_face_terms`), and the
  import's gate enclosure bit-identical to the imported body's own
  measurement;
- PR 4682's `faces − 1` ulp bound and bit-identical pad stay as the
  body-level claim.

Repair (a), an order-independent fold, moves to FLUX's design row
`the-whole-body-property-fold-depends-on-face-order`, closed by PR 5402, so
this row closes.
