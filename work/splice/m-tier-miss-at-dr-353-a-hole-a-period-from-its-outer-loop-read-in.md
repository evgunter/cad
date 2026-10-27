---
id: m-tier-miss-at-dr-353-a-hole-a-period-from-its-outer-loop-read-in
kind: ruling
title: Short readout: M-tier miss at DR-353 (PR 5805), a holdout pair's tallied MAJOR: on a closed chart, a hole lifted one period from its outer loop read In, and the other review did not raise it
status: closed
opened: 2026-10-27
closed: 2026-10-27
---

**An FYI with no options.** Dual-review protocol rule 9 requires a short
readout at every M-tier miss. Acknowledging it closes this item.

- **The unit.** PR 5805 builds point-in-face for a spline face
  (`topo::boolean::spline_region`). It is the NURBS/Approx arm of
  `contain::curved_interior_placement`, which until then answered
  `Trim(None)`. A point near a spline face is placed In or Out by the
  face's trim, read as ray parity over the face's lifted pcurve loops.
  It is an M unit, drawn into the HOLDOUT arm (byte 48), and recorded as
  DR-353.
- **The miss.** R1 raised a MAJOR and showed it with a red row.
  - On a closed chart, `Trim::of` lifted each loop on its own, starting
    each at its own member in the identity copy. So the outer loop and
    a hole could land one period apart.
  - Parity at each copy of the query then read a point inside the hole
    as **In**. Whether it did depended on which half-edge a loop's walk
    starts on.
  - The witness row is
    `r1_a_hole_lifted_a_period_from_its_outer_loop_is_still_a_hole`.
  - R2 raised no MAJOR, and its report says it did not exercise closed
    charts. It noted that two period sources exist, which is a
    different defect. Had R2 been the sequential arm's only review, a
    wrong In on any revolved or periodic spline face with a hole would
    have shipped.
  - The blinded coding (byte 16) tallied it.
- **What happened next.** The fix pass moves each ring by the one
  decided whole number of periods that places it in the outer loop's
  cover, and refuses where no single shift is decided. It also makes
  the two period sources agree or refuse. A square tube closed in u is
  built at rest as a witness: a face spans its seam, with a hole on
  either side, and both read correctly. Zeroing the shift turns both
  rows red. The unit merged as DR-353.
- **The count.** The foot of `docs/DUAL-REVIEW-LOG.md` gives the running
  tally and the M-tier misses since the last full readout. This is the
  fourth since readout 5, after DR-185, DR-208 and DR-321.

## Closed (2026-10-27, PR 5821)

Ev acknowledged the readout ("thanks for letting me know!"). Nothing further is owed.
