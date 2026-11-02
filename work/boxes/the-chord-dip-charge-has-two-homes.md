---
id: the-chord-dip-charge-has-two-homes
kind: issue
title: The chord-dip charge f2*step^2/8 has two homes and nine spellings: the one in topo/boolean/boxes.rs cannot be depended on from geom-brep
status: closed
opened: 2026-09-14
priority: P1
cost: C
rides_with: split-gate-sphere-zone-folds-into-face-box-rule
closed: 2026-11-02
pr: 6336
---


## What

The chord-dip charge — how far a C² function of second-derivative
bound `f2` leaves the chord of a sub-interval of width `h`, `f2·h²/8`
— is the certified widening behind every subdivision enclosure in the
kernel. It has **two homes and, before this filing, nine spellings**:

- `topo::boolean::boxes::subdivision_charge` (the named one), read by
  `arc_extent`, `edge_axial_span` and the box construction rows;
- bare `* 0.125` arithmetic in `geom-brep`'s `implicit.rs` and
  `tangent.rs`, and in the suites that pin either.

`topo` sits ABOVE `geom-brep`, so the named home cannot be depended on
from the crate that needs it most: the circle-residual enclosure
family. That is why the spellings multiplied rather than converged.

**Half-fixed at CURVED-TORUS PR-2** (#2535): the `geom-brep`-side
spellings collapsed to one `pub(crate) implicit::chord_dip_charge`,
which `tangent.rs`'s two `residual_sag` sites now read. What is left
is the cross-crate half — `boxes.rs::subdivision_charge` is the same
quantity under a second name, and `boxes.rs` is S-BOOL's file.

## The fix

Move the home DOWN — `geom-core` if a third crate ever wants it,
`geom-brep` otherwise — and re-aim `boxes.rs` at it, deleting
`subdivision_charge`. One name, one doc comment stating the proof
(a C² function's dip below the chord of a width-`h` interval), one
place a reader checks the constant.

Raised by both arms of CURVED-TORUS PR-2's v6 dual (R2 S-1),
adjudicated `Recorded, not fixed here`.

## Home

CURVED — `geom-brep/implicit.rs` is this program's file and the
`geom-brep` half landed here; the `boxes.rs` half needs S-BOOL's
announced handover or its exit.

## Closed (PR 6336, 2026-11-02)

`topo::boolean::boxes::subdivision_charge` is gone with the sampled
torus window it served, which the closed form `torus_rect_extent`
replaces; `geom_brep::implicit::chord_dip_charge` is the one home.
The sweep for bare `f2·h²/8` spellings: `pcurve_cache.rs`
`harmonic_span_box` repointed at the home; `props/quad/trimmed.rs`'s
`0.125·|ℓ′|·h²·width` is a different bound (an integral against the
oscillation of g) whose ⅛ is coincidence; `mesh` `cert.rs`
`torus_chord_bound` is a 2-D Hessian form with a mixed term; the
`mesh` `sizing.rs` hits are test oracles of the step inverses. No
other production spelling of the quantity remains. (BOXES orchestrator)
