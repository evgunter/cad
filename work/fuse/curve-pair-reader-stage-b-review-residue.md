---
id: curve-pair-reader-stage-b-review-residue
kind: issue
title: The curve-pair reader's stage (b) re-review residue: an unfiled touch refusal window, a misnamed basin floor, a minted straddle copy, and three tangency-floor spellings
status: closed
closed: 2026-10-15
branch: fuse/curve-pair-reader-c3
opened: 2026-10-14
priority: P2
cost: M
refs: [the-curve-pair-reader, curve-pair-polish-meters-refuse-shallow-meetings]
---


## Found

These come from the re-review of the stage (b) fix pass, PRs 5118 and 5121, which merged with them open. Every refusal below is typed, and the re-review found no decided wrong answer: 0 in 2,100 built-to-meet draws plus 1,968 adversarial poses. What remains is needless refusals, prose that disagrees with the code, and copies.

1. **The touch refusal window is unfiled, and its row cannot go red on its loop.**
   - Every pose in `meet_rows.rs`'s `a_touch_of_the_other_plane_meets_where_the_curves_do` loop refuses `Meter`: tilt 1.2e-8..1e-6, crossings ±2°..±20°, 320 of 320 poses.
   - `meets_or_refuses` therefore passes against a reader that refuses everything. Only the final 1e-4 square case can go red.
   - The window reaches tilt ≈1e-6 at a 90° crossing and ≈1e-5 at 20°. These are square, decidable crossings.
   - `curve-pair-polish-meters-refuse-shallow-meetings` does not list this window.
2. **The basin floor is misnamed, and the refusals below it are not shown undecidable.**
   - The rows and the P2 call the floor "√(κe)". The meter implements about √(8κe): half the angle, region 2(ℓ+e), and κa+κb.
   - The sphere-circles pose refuses up to φ≈2.4e-4 and decides from 3.0e-4, while √(κe)≈9.1e-5.
   - The row asserts refusal at φ≈1.08e-4, which is above the floor its own doc names.
   - The least distance there is exactly 0, so the refusal is the meter's own, not undecidability at ε. Say so, or tighten the meter.
3. **The P2's "loses no answer" cites the wrong evidence.** It cites the walk. Stage (c) replaces `carrier_cross::meetings`, and on the sphere-circles pose that function:
   - escalates at tilt 1e-4, where its candidates lie 1.7e-9 off;
   - decides "no" wrongly at 1e-5, 8.8e-8 off.

   The conclusion likely holds, but re-state the reason.
4. **MAJOR B's sibling was never swept: `line_conic` still reads the tilt d̂·n̂, not the conic's swing.**
   - A line in a circle's plane beside a thin ellipse tilted 5e-8..1e-3 refuses `Meter`, at least distances of 1.04 m and 4e-2 m.
   - This is a needless refusal on a listed arm, and it is not on the P2.
5. **The basin docs disagree with the code.** The `pose.rs` module docs say "4(κ₁+κ₂)·2(ℓ+e)" and "a quarter of the angle". The code (~525-532) computes 2·κ·region, i.e. half.
6. **The travel cull decides "none" because the polish failed.**
   - `pose.rs` ~537-540 returns `Ok(None)` when the polish travels past 2× its region. That rests on contraction, which the basin meter does not prove; a `Meter` refusal reads more naturally.
   - No breaking pose was found.
   - One row name, `curve_pair_pose_clear`, records both the clearance cull and the travel cull.
7. **The oracle's `off_span` measures distance to the carrier, not the span, on a line or circle** (`meet_oracle.rs` ~382-390).
8. **Copies the fix pass minted:**
   - `pose.rs` `constant` (~820-834) re-implements `roots::spread_signs`, with a straddle policy (escalate) that differs from topo's `constant_residual_roots` (`Tangency`).
   - There are two first-harmonic root readers, `pose::harmonic_roots` and topo's `first_harmonic_roots_in_band`, and they disagree on an in-band extreme.
   - `Side::ders` plus `tangent` re-spell `Curve3::ders1`.
   - The tangency floor is spelled three ways: `bisect::tangency_floor` (√(ρe)/8), the basin's ≈√(8κe), and the prose's "√(κe)".
9. **`reduce`'s no-lane guard is checked at runtime, where a type could rule it out.** It checks a fact fixed at write time and ends in `ClassificationInvariant`.

## Owed

- Items 1–4: either decide those poses or file them on `curve-pair-polish-meters-refuse-shallow-meetings`, with the floor named as the code computes it. Make the touch row able to go red.
- Items 5–9: fix them with the stage (c) caller moves, which touch these files anyway.

## Closed — stage (c) (fuse/curve-pair-reader-c, -c3)

- **Items 1 and 4** are on `work/fuse/curve-pair-polish-meters-refuse-shallow-meetings.md`: the touch window, with its row that cannot go red, and `line_conic`'s tilt read where the swing is owed.
- **Item 2:** that row names the floor as the meter computes it, a crossing angle of about `√(8κe)` ("the basin floor"). The walk's piece floor keeps the name tangency floor.
- **Item 3:** the evidence is restated there. At ε = 1e-6, main's `common_line` placed crossings that `meet` refuses.
- **Item 5:** the basin docs read `2(κ₁ + κ₂)·2(ℓ + e)` and "half the angle" (PR 5154).
- **Item 6:** a polish that travels past twice its region refuses `Meter` on its own row, `curve_pair_pose_travel`, instead of reading "none".
- **Item 7:** the oracle's `off_span` is the distance to the span, exact on a segment and an arc.
- **Item 8:**
  - `pose::constant` reads `roots::spread_readings`, beside `spread_signs`.
  - `Side::ders` is `Curve3::ders1`, and `tangent` is gone.
  - The floor prose has one meaning per name.
  - The two first-harmonic readers are filed as `work/fuse/two-first-harmonic-root-readers.md`: unifying them moves every circle-root door row.
- **Item 9:** `reduce`'s no-lane arm takes the refusals' path to `carrier_touch::off_face`, for lines and conics alike, and the runtime guard is gone.
