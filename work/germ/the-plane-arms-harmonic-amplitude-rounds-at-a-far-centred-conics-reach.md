---
id: the-plane-arms-harmonic-amplitude-rounds-at-a-far-centred-conics-reach
kind: issue
title: The plane arm reads wrong signs, uncharged, where the conic is stored far from its arc: its harmonic amplitude k.along(n) rounds at u·far and cancels against the now-exact offset
status: closed
closed: 2026-11-24
opened: 2026-11-22
priority: P1
cost: M
---


Filed by GERM's `germ/plane-pose-far-frame` lane (PR 7411) from its
FULL review (finding M1). This loss was there before PR 7411 and was not
introduced by it: the PR made the conic's offset `d₀` exact, and this is
the other term of the same gap.

## Finding

`curve_face::pose::plane_conic` (`crates/geom-brep/src/curve_face/pose.rs`)
reads a conic's height above a plane as `d₀ + R cos(θ − φ)` through
`curve_pair::pose::height` (`crates/geom-brep/src/curve_pair/pose.rs`,
`height`), and decides the belly `r − |d₀|` with
`r = √(a² + b²)`, `(a, b) = k.along(n)`. Since PR 7411, `d₀ = n·(c − o)` is
read to its own digits (`Real::difference_dot`). The amplitude is not.
For a conic whose semi-axes are of the order `far` (a section ellipse of
a shallow cut, centred far from the vertex the plane grazes),
`k.along(n)` multiplies `far`-sized semi-axis vectors by `n` and rounds
at `u·far`. The belly `r − |d₀|` then cancels two `far`-sized terms down
to a gap of the order of the band, so the gap is read at `u·far`, with
no charge (`[simple.conditioned]` forbids one).

## Probe (the reviewer's, restated)

- An ellipse with `a = far` and `b = far/3`, centred `far` from its
  vertex, grazed near the vertex by a plane whose stored origin is near
  the arc.
- ε 1e-12, gap set within ±2× the escalation and read exactly by an
  `Expansion` oracle on the stored terms. 2000 poses per far.

Reads definite against the exact sign:

| far | wrong of 2000 | of them beyond the escalation |
|---|---|---|
| 1e5 | 163 | 30 |
| 1e6 | 359 | 274 |

Example: exact gap −3.86e-11, read `Isolated`.

A sibling on the same ground: an ellipse held on its arc (`Ellipse::anchored`)
answers `frame()` at its derived centre, `anchor − u·a·cos θ₀ − v·b·sin θ₀`,
which rounds at `u·far` when the anchor is far from the centre. A reader
that takes that frame as a plane (`OnSurface::plane`,
`ring_path::LoopArc::plane`) places the plane's origin off the plane by
that much, along the normal. Placing the plane at the stored anchor, which
lies in it exactly, would avoid this, but no public door hands the anchored
placement out (`Ellipse::stored` is `pub(super)`).

## Settles when

The belly `r − |d₀|` is posed so its digits are the gap's own. That
means the gap is formed about the conic's point nearest the plane rather
than about its centre, or `r − |d₀|` is formed from the stored terms
cancellation-free. Measured by the probe above as a row at ε 1e-12,
reading 0 wrong at 1e5 and 1e6, with no charge.

## Progress

- 2026-11-23, `germ/harmonic-belly`.
  - **The probe, as a row:** `curve_face::pose_rows::a_graze_of_a_far_centred_conic_is_read_wrong_nowhere`, slow set.
    - It reads the whole turn, counts the escalation, and its oracle is an exact `Expansion` on the stored terms, cross-checked against Python `fractions` over 6000 poses.
    - On main it is red at 78 / 596 / 478 wrong of 2000 at far 1e4 / 1e5 / 1e6 (seed 0x1), of them 0 / 52 / 334 against the exact sign past the escalation.
    - With the re-pose it reads 0 at every far, on seeds 0x1, 0x2 and 0x3.
  - **The re-pose:** `Real::harmonic_belly` computes `(A² + B² − d₀²)/(√(A² + B²) + |d₀|)`, with the numerator exact and `A`, `B`, `d₀` each rounded once. It is within 9u of the belly, relative to the belly. `curve_pair::pose::belly` reads it.
  - **Its readers:** the plane arm (and through it the co-surface arm and the ring path), `line_conic`, `conic_conic`'s parallel screen and `transverse`, and `section_loops::line_clears_conic`.
  - **A second row:** `curve_pair::far_graze_rows::a_line_grazing_a_far_centred_conic_is_read_wrong_nowhere_past_the_touchs_placement` (renamed and narrowed later the same day, below).
  - **The sibling** is filed as `an-arc-held-ellipses-plane-is-read-at-its-derived-centre`.
- 2026-11-23, fix pass on PR 7553 (its review: MAJOR-1, the across-seam term read from a rounded `v_ref`).
  - **The belly is now the conic's own door**, `Conic::belly_along(o, n)` in `conic.rs`. It reads the stored placement (stored point and phase, axis, seam, semi-axes) with `B = b·((axis × u_ref)·n)` crossed exactly (`exact::triple_product`). An arc-held ellipse is read from its anchor.
  - **Out of the exact range** (decided on the inputs' lowest bits) the `f64` reading is poison and escalates.
  - **Every row now grazes at any angle round the turn, both placements**, judged by an exact dyadic oracle on the stored data (`geom_core::test_support::stored_belly`), never through `Expansion` or `v_ref`.
  - **Rows added for each swept reader:** the parallel screen, the near extreme, and `line_clears_conic`.
  - **The probe:** 0 wrong at far 1e4 / 1e5 / 1e6 on seeds 0x1 and 0x2. Under the old belly (M1, seed 0x1) it reads 50 / 317 / 406 centre-held and 52 / 308 / 395 arc-held.
  - **The sibling** is closed as a duplicate of ONEROUND's unit, its plane readers folded in there.
- 2026-11-23, confirmation pass on PR 7553 (APPROVE-WITH-FIXES).
  - **The screen's call site has an end-to-end row:** `curve_pair::pose::far_rows::a_far_conic_grazing_a_circle_through_its_vertex_is_never_apart`. A far ellipse grazes a unit circle's plane at its vertex, and the circle passes through that point. It reads 0 `Apart` of 2000, and 0 / 12 / 1 with the call reverted.
  - **The arc-held probe** anchors at all four vertex phases.
  - **The Interval docs** are corrected: a production-lifted frame reads the plain enclosure. Filed as `a-lifted-frames-interval-belly-reads-the-plain-enclosure`.
- 2026-11-23, PR 7553's hosted CI red on the line row.
  - **The red:** seed `0x310ae3910d809302`, far 1e6, one wrong `Apart`, red on the merged head and on its pre-merge parent alike.
  - **The cause:** the belly reads the touch right. `place`'s carrier walk `clear_by` then reads the far conic clear of its own point at `u·far` (CONIC's `on_carrier.rs`).
  - **What was done:** filed on CONIC as `a-centre-held-far-conics-touch-is-dropped-by-a-carrier-walk-on-points-evaluated-about-its-centre`. The row is narrowed openly to `…_past_the_touchs_placement`: it counts `placed` apart, and M3 still reds it.
- 2026-11-23, PR 7553's fast-set time bar.
  - **The red:** `topo chord_join::torus_ring_rows::a_ring_on_a_torus_face_is_re_homed_by_a_path` read 1.116 s on CI.
  - **Measured locally**, `--profile ci`, default ε, interleaved, four runs each: main median 2.34 s, this head 2.69 s. Replacing the exact path with the plain formula reads 2.45 s, so the exact path is the whole added cost.
  - **The fix keeps exactness wherever it decides.** A double-double reading answers first, on its own error bound against the terms' magnitudes. The exact expansions answer only where the belly cancels below that bound, and their terms are compressed before they multiply. The new head reads a median of 2.30 s, main's time.
  - **The filter is witnessed:** a draw at the data's own rounding, pinned from the proptest's own failure under the always-accepting mutant.
- 2026-11-23, merge with main after PR 7436 (HELDRECORDS) replaced `line_clears_conic` with `edges_clear`, read through `curve_pair::meet`.
  - **The row is ported:** `splitting::section_loops::far_clear_rows::a_segment_clears_a_far_centred_conic_only_where_it_is_clear`, on a segment in the plane `z = 0` exactly beside a centre-held far ellipse.
  - **Why the plane is exact:** a segment built off the stored plane stands apart from it in 3D at `u·far`, which the in-plane belly cannot judge.
  - **Result:** it reads 0 wrong clears past the zero-band touch placement (CONIC's row), and M3 reds it.

## Closed

2026-11-24, by the GERM orchestrator. Closed by PR 7553 (merge `80a3bf53fe`). The belly is now the conic's own door, `Conic::belly_along` / `belly_across` in `conic.rs`. It is read from the stored placement, with B crossed exactly, through `Real::harmonic_belly`: an exact-expansion numerator behind a double-double filter, within 7u relative to the belly. Against an independent oracle it reads 0 wrong at far 1e4–1e6 in both placements, down from 50/317/406 on the old belly. The arc-held sibling is fixed by the same door. Residuals are filed: CONIC's carrier-walk dropped touch, a lifted frame's Interval belly, and the far-radius cylinder.
