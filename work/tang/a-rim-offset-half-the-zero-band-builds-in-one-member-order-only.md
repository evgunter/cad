---
id: a-rim-offset-half-the-zero-band-builds-in-one-member-order-only
kind: issue
title: A dome rim offset half the zero band off the tube builds in one member order and refuses in the other
status: closed
opened: 2026-10-02
priority: P1
cost: M
closed: 2026-10-08
---

## What

The tube of `crates/sweep/tests/pi_seam_and_kiss_through_the_boolean.rs`
with its radius set to `R + 0.5 · band.zero()`, unioned with
`dome_on_the_cap()`, the discs declared `Rest` (measured on branch
`tang/abutting-rim`, PR 3823, `Tol::witness()`):

- order 0 (tube ∪ dome) builds: `V = 6.971041473977074`, census
  `(5, 8, 5)` faces, edges, vertices, one shell;
- order 1 (dome ∪ tube) refuses `Merge(Pcurve { source: Certify {
  error: Escalated { check: Envelope, .. } } })`, on `pcurve_envelope`
  with margin `1.000000082740371e-9`, which is the band's zero.

The crossing layer decides the rim exactly on (`LiesOn`) in both orders,
since the offset is inside the zero band. What differs is the merge
stage's pcurve certification of the seam it then builds, which reads
the half-zero offset as an in-band envelope in one order only.

## Where

`crates/topo/src/merge_faces.rs` and the pcurve certification it calls,
outside PR 3823's diff. Filed on TANG because the rim lane is what
reaches it; the owner of the merge stage (FUSE) may take it.

A body that one member order builds and the other refuses breaks the
op's symmetry. Either both orders should build (the zero band says the
rim is on) or both should escalate.

## Released from the D10 hold (2026-10-08)

Nothing D10 changes gates this row, so it is open: an order asymmetry in merge-stage pcurve certification (geom-brep pcurve_cache.rs pcurve_envelope); the Zero-glue merge still runs it, so a fix now is kept. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Closed (2026-10-08, TANG)

Both orders build on main, since PR 3759 (bisected over first-parent
merges from PR 3823's head). The change that did it is PR 3812, merged
into 3759: it restated the periodic envelope as frame, incidence and
fidelity.

**Why the orders differed.** The REST zip keeps the first operand's
copy of the rim, and the merge's mint certifies the second operand's
row against that circle. In order 0 that is the dome's sphere row
against the tube's circle `R + δ`. In order 1 it is the tube's wall row
against the dome's circle `R`. The old envelope bounded the image's
`a`, `b` coefficients by `‖Δa‖ + ‖Δb‖`, which reads a radius offset
twice: `2δ` for a point deviation `δ`. At `δ = zero/2` both rows sat on
the flip, and rounding chose. The sphere row read `0.99999964·zero` and
built; the wall row read `1.00000008·zero` and escalated.

**Which reading is right (D4).** The rim's point deviation from the
partner's carrier is `δ`, so the envelope's verdict should flip at
`δ = zero`. Half a zero band is inside it, so both orders build. On main
each row's `Radius` term reads `δ(1 ± 4e-7)`. The `(1 ± 4e-7)` is the
two charts' rounding, and it leaves a tie window that narrow at
`δ = zero`. No such row reaches the merge today, because the crossing
layer escalates first (below).

`pi_seam_and_kiss_through_the_boolean.rs`,
`a_rim_offset_inside_the_zero_band_answers_alike_in_both_member_orders`
pins `k ∈ {0, ±¼, ±½, ±0.9, ±1.1, ±2}` zero bands, both orders, at every
ε row:
- `0, ¼, ½` build alike: (5, 8, 5, 1), tier 3, and the volume within
  `zero·A` of the closed form. The volumes differ between the orders by
  `1.05e-9` m³ at ½ (a mean boundary displacement of `5.4e-11` m),
  because the rim keeps a different operand's circle in each.
- `−¼, −½` refuse the tight union bound in both orders. That is
  `work/reachhold/a-settled-declared-coincidence-crosses-a-tight-volume-bound.md`,
  with this evidence added there.
- `±0.9` escalate `bool_wall_root_in_span` in both orders, on the
  meridian's root `√2·0.9·zero` along its arc. That is
  `work/cleave/boolean-in-span-readings-of-grazing-roots-are-levered-by-arc-length.md`,
  with this evidence added there.
- `±1.1, ±2` are in band and escalate in both orders.

The mutant that doubles the wall's `Radius` term (the old lever)
reproduces this item's refusal exactly, at `k = ½`, order 1.

The same coefficient-sum shape survives at other check-4 sites. It is
filed as
`work/pcert/pcurve-envelope-terms-sum-the-cos-and-sin-coefficients-of-a-deviation.md`.
