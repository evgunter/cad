---
id: quad-de-boor-through-the-blossom-breaks-the-import-bit-identity
kind: issue
title: props/quad's de_boor through the blossom: the arc-loft round trip's one ulp was the face-sum order, and the routing ships
status: closed
opened: 2026-10-10
closed: 2026-10-12
branch: nurbs/quad-de-boor-through-the-blossom
pr: 4807
---


Found by PR 4551's fix pass, whose CI slow set ran step-import's row for
the first time on that branch.

## What

`crates/geom-brep/src/props/quad.rs`'s `de_boor` evaluates a spline
channel at a quadrature node. Routed through `CoeffWindow::blossom`
(every level `Certification::between`) for a node inside the span, and
kept on the affine `(1 − α)·x + α·y` past it, it makes
`crates/step-import/tests/nurbs_import.rs`'s
`arc_loft_natively_computes_its_rational_volume` red at ε 1e-9: the
imported body's certified volume reads `12.49350995916491` against the
native `12.493509959164909`, one ulp apart, with identical pads. The
row pins BIT identity ("the reader reconstructs the same chart and the
same trim, so the same quadrature runs on the same bits").

Measured on a bisection worktree at opt-level 1:

- routing reverted in `de_boor` and `de_boor_on` (both back to the
  affine step): green, volume `12.493509959164905` both ways;
- `bezier_bisect` reverted alone: still red;
- `between`'s two fast paths removed alone: not measured to completion.

So PR 4551 ships `de_boor` unrouted, as its per-site table says.

## Corrected (2026-10-11): the one ulp is the face-sum order

PR 4684's measurement refutes the "inputs differ" premise below. The
native and imported walks hand the quadrature identical bits on every
face and get the same flux back. The whole-body fold (`topo::props`'
`fold_runs`, face-arena order) sums the same six terms in two orders,
and the reader allocates faces in its own order. The blossom route
moved the rational wall's flux bits, and with them whether the two
orders happen to round alike. It is FLUX's design row
`the-whole-body-property-fold-depends-on-face-order`, closed by PR 5402
(filed on PR 4684's branch). Main's row now bounds native against
imported at `faces − 1` ulps with the pad bit-identical (ENCL's PR
4682), so it no longer reddens on this.

## Next

Route `de_boor` through the blossom for nodes inside the span, with
the affine step kept past the domain (Q10's vertex pad puts nodes
there), and re-baseline what moves with that cause: on the fix pass's
tree, the step-export sidecars (`loft_prism`, `nonuniform_loft`,
`swept_elbow`) and the sweep thread-count digest moved under the route.

## Superseded: what was not known

Why the two sides differ. Equal inputs give equal outputs in either
form, so the native and imported walks hand `de_boor` inputs that
differ, and the affine form happens to agree bitwise on them where the
monotone one does not. Candidates, none checked: a reversed trim (the
affine step and the monotone step are not symmetric under
`(x, y, λ) → (y, x, 1 − λ)` in the same way), a knot vector translated
or scaled by the reader, a signed zero. A trace of every call (about
11 million on this fixture, 1.4 GB) never reached the import phase
inside a 10-minute run, so the next attempt should aggregate in
process: hash each call's inputs per phase and diff the two multisets.

The original next step, now moot:

Find the differing inputs; then either make the reader reproduce the
native chart bit for bit (the row's premise), or show that the
divergence is the monotone step's asymmetry and decide which side of
the identity row gives.

## Closed

`props::quad`'s `de_boor` evaluates a node inside its span as the
window's blossom (`CoeffWindow::blossom`, every level
`Certification::between`), and `de_boor_on` steps through `between`
inside the span's closure; past it (a vertex pad's node) the affine
extension stays.

With the routing, step-import's
`arc_loft_natively_computes_its_rational_volume` is green: PR 4684's
per-face bit identity (every face's flux term and half-width, native
against imported) holds, and so do the gate's identity with the
imported body's own measurement and ENCL's `faces − 1` ulp bound. The
old red was the whole-body fold's face order, as corrected above.

What the routing moves, every new enclosure inside the old one:
- the step-export sidecars: the volume pads of `loft_prism`,
  `nonuniform_loft` and `swept_elbow` fall 1.93x, 1.73x and 3.95x, and
  the midpoints of the last two move two ulps and one;
- the sweep thread-count digest at all three ε: `loft_prism`'s volume
  pad by 48%, `arc_loft_1e9eps` by a few ulps, and `quintic_prism` at
  ε = 1e-6 and 1e-9;
- the certified endpoint census: `quad.rs` 11/6 → 12/7, the two reads
  in `within`, behind its `is_certified()`.

Each re-cut names the cause beside its numbers.
