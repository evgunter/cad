# CHART log

## Opened at TRIM's cut (2026-09-20)

Opened by TRIM's orchestrator on Ev's in-chat direction (break the two
programs into smaller tracks; leave in TRIM only what closes this
session). Thirty-one items moved from `work/trim/` and three SSI
drive-bys from `work/curved/`. Band 6200–6299 recorded in the ledger's
banding entry in the same commit. No unit dispatched; the first sitting
picks from `work/chart/plan.md` §Order. Context the items carry:
`min-separation-tightening-crosses-the-drive` was REWRITTEN by TRIM-3
PR-2's fix pass to the measured mechanism (both reviewers refuted the
original: the diverging census rows are the loop walk's, recorded by
the interval leaf and never by the f64 lane, which returns `None`);
`revolved-bands-reach-no-clearance-row` REPLACES a refuted file (an
extrude CAN mint a negative cylinder band); the rational-gate class's
`mesh` copy was fixed by TRIM-2 PR-2 (constancy), the cap class remains.

## Rows arriving from FIX, 2026-09-20

Ev ruled in chat on 2026-09-20 that **FIX carries no design decisions**:
*"can you kick all the design decisions back to the track they actually
belong to, leaving fix design-free?"* FIX is the program for rows whose
fix is already written; three waves closed those and left a slate that
had drifted into decisions. Fourteen rows moved out by `git mv` to the
track owning the surface each decision is about, each carrying a
`## Re-homed` section stating the question, the routing basis, and what
was NOT decided for the receiver. Routing was taken from
`python3 scripts/work.py territory --files -` over every path the rows
cite, not from FIX's `keep_out` prose — two of that clause's fence
claims were stale and are corrected in the moved rows.

**One row: `normalize-without-the-length-question-two-more-sites`,
and it carries an AMENDMENT that contradicts its own first half.**

Site 1 is `crates/topo/src/chart_region.rs` (yours): the collinear lane
decides an offset whose norm is in the DENOMINATOR, and its `Sign::Zero`
arm then normalizes `r` — whose length no door asked to be finite.

**A reviewer EXECUTED it, and the mechanism the row argued is not the one
that fires.** What reproduces, through `proper_crossings` with two-point
loops: two collinear overlapping segments give `Err(TouchingBoundary)` at
finite scale, and the same configuration scaled by 1e199 gives **`Ok(0
crossings)`** — a touching boundary silently lost. Not the spurious `Zero`
the row predicted: the offset rung decides a CORRECT `Zero`, and then
`rhat = r.normalize()` is `(0, 0)`, which drives `s0 = s1 = 0`, so the
overlap decides `Zero` and the lane `continue`s past the crossing.
**The operative consequence: a fix at the offset rung would not close this
row.** The repair belongs where the collapsed `rhat` is USED. Start from
the reproduction, not from the argument above it.

**Site 2 travels with it and is not yours**:
`crates/geom-brep/src/enters.rs` is in no open program's `paths` —
territory returns no owner. `enters_material` normalizes a `dir` whose own
length is never decided or asked about, so a `dir` past the overflow band
normalizes to zero and the door answers `Tangent` — definite, and about
nothing. `geom_core::is_finite_length`'s docs call this *the declined half
of the direction family* and cite ONE instance; this is a second, so the
doc's count is wrong today whatever is decided about the half. **If you
would rather not carry an unowned geom-brep site, split it into its own
file rather than leaving it as a sentence** — `work/README.md` is explicit
that a residue disclosed in prose is invisible to the re-homing sweep.

Signed (FIX orchestrator).

## 2026-09-27 — seam note from S-DUP (#3304)

#3304 retyped topo's two ray-direction tables. `splitting::containment::SCHEDULE` is now `[Vec3<f64>; 16]`, and `chart_region::SCHEDULE_2D` is now `[Vec2<f64>; 16]`.

Their five lift sites now read `r.map(T::from_f64)`:
- `containment`, `order` and `solid_contain` (3-D table);
- `chart_region` and `chart_bound` (2-D table).

The numerals are textually unchanged. A throwaway test asserted every lifted component bit-identical at `f64` and `Interval` (160 components), and a one-ulp plant reddened it. No behaviour changed.

Signed (S-DUP orchestrator).
- 2026-09-29 — Seam note from ENCL: PR 3398 (merged `3aac59af62`) moves `topo::validate`'s remaining 'lower the tolerance' endings onto their decisions (D4 ¶1). `geom_brep::recourse` gains `Unsized` and `defect_ending`, moved out of certify. Filed: `pcurve-mint-escalation-carries-no-decision` (validate's `M::Escalated` ends 'no way through yet' until the mint carries its decision). (ENCL orchestrator)
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)
- 2026-09-29 — Seam note from TOPO: PR 3467 (`topo/sense-reads-same-chart`, not yet merged) implements Ev's D1 ruling (PR 3480): `FaceSurface::New { surface, sense }` and `Shared { key, sense }` state the new face's bit; on the parent's chart `mef` derives the parent's bit and `mfkrh` its negation, and a contradicting stated bit is refused (`EulerOpError::SenseContradictsChart`); `set_face_surface` takes the same spec and `set_face_surface_and_sense` is gone; `Body::mvfs` and `Body::mfkrh_plug` take the seed's provisional bit. Paths: `topo/src/chart_region.rs`. In your files every `New`/`Shared` spec, `mvfs` and `mfkrh_plug` call states the bit it carried before; no expected value moved. (TOPO implementer)
- 2026-10-10 — Seam note from PIPE (PR 4486, `pipe/census-arms-and-witness-caps`): `ChartRegionError::WitnessBudgetExhausted { segments, cells }` is split into `WitnessSegmentCapExceeded { segments }` and `WitnessCellCapExceeded { segments, cells }`, and `topo::WITNESS_BUDGET` / `WitnessBudget` into `topo::WITNESS_SEGMENT_CAP` / `WITNESS_CELL_CAP`. Every census and validate mapping keeps one answer for both faces. A match in your files names both. (PIPE implementer, witness-cap lane)
- 2026-10-10 — Seam note from ENCL (PR 4497, merged at `4c7fb9b0fc`). A poisoned or contradicted margin now ends in a followable recourse:
  - at rest, the census coincidence menu (`validate::too_close` → `own_close`) gives the defect ending;
  - `ChartRegionError::Escalated`'s poisoned arm gives `defect_ending(Build)`;
  - `PointInSolidError::Escalated` gives the unnamed placement lever plus the reading's note, as `contfp`, `classify_point_in_solid` and the Boolean already did.

  `SliverSector` is unchanged, and its mints are on CLEAVE's split-escalations row. (ENCL orchestrator)

- **2026-10-13 — seam note from NURBS: PR 4928 merged (proven rational stretch and area floors).** `chart_stretch_inf`'s rational infs now read `NurbsSurface::{stretch_floor, area_floor}` (`geom_core::spline::{rational_stretch_floor, rational_area_floor}`, proofs in the derivative module docs). `weight_ratio_factor`, `skipping_surface_derivative_net` and `floor_weights_admissible` are deleted, and `net_direction` reads `derivative_net`.
  - Your ground is touched only in prose or tests: the citations of the retired factor, and the chart reach rows. — (NURBS orchestrator)

- 2026-10-13 — **Seam note from NURBS: PR 4954 merged** (`092b30227f`). Every first-order rational derivative bound now reads one Bernstein quotient bound on exact dyadic Bézier pieces (`geom_core::spline::derivative::QuotientPiece`, with the pieces in `geom_core::spline::pieces`).
  - **Retired:**
    - `rational_speed_sup`, `rational_stretch_sup`, `rational_stretch_floor` and `rational_area_floor`;
    - `RationalPairTerm`, `pair_sup`, `Cell` and `over_cells`;
    - enclose's `pair_norm_sup`, `Centred`, `Affine` and `screen_bound`;
    - `pcurve_cache`'s `net_direction` and `floor_along`.
  - **API change:** `NurbsSurface::stretch_floor(along_u)` and `area_floor()` are norm floors and lost their `d` argument.
  - **Tighter readings:** `speed_upper_bound`, `stretch_sup`, `chart_stretch_inf` and enclose's `deriv_box` / `speed_sup` / `transverse_readings`. So iso slacks shrink and floors rise.
  - **Degree ceilings:** curves meter to degree 28, surface sups and per-axis floors to 28 per axis, and the area floor to 18. Above those the bound refuses.
  - **Filed:** `work/ssiedge/a-tighter-strip-box-refuses-a-side-the-roots-decided.md` and `work/nurbs/binomial-bernstein-product-and-blossom-have-two-homes-each.md`.
  - **Next:** NURBS unit 2 moves `patch_bound` `rational_cells` (first and second order), mesh chords and `offset_meters` onto the same form. — (NURBS orchestrator)

## 2026-10-24 — CHART picked up; the priority-seam cut

CHART was at 88 budget points against 30, so it is split along its priority
seam:
- CHART keeps its P0, its eight P1 rows and
  `chart-inversion-is-spelled-three-times`, banded P1 at the move as two
  implementations of one reading (D3). That is 28 points.
- The 26 P2–P4 rows and the unpriced rows go to CHARTTAIL, opened
  `ready` with nobody on it (43/45; its program.md gives the reason for
  the 45-point budget). Eleven unpriced rows were banded and costed at the move,
  and one duplicate was closed.
- `min-separation-tightening-crosses-the-drive` goes to CLEAR. Its whole
  fix shape is on the clearance engine's and the measure's ground, and its
  ruling is Ev's.
- The legacy `D` rows are re-priced `M`. `design: true` is set on the
  region-description family and the arms tables.

Sequencing (the orchestrator's call): one designer pair (Opus and Fable,
`docs/prompts/designer.md`) weighs the region description, with the arms.
The three rows that are not that question run as lanes in parallel. The
alternative was to build the round-hole lune first, which is cheaper. It
was declined because the region question would decide whether the lune
survives.

## 2026-10-24 — dispatched (05:07Z)

- **The region-description designer pair**, concurrent and independent, with the same problem statement and no candidate solutions: A `session_01G7XGGxV8D3yfuSM8RdL4R8` and B `session_01XG7Nq1pHzPyv3Eeweqzqzc`. It covers the pole, round-hole, exact-region and cone/sphere/torus-window rows, together with the three arms rows.
- **`normalize-without-the-length-question-two-more-sites`**: `session_01BEinSjRAhefi8FBtveZTva`, branch `chart/normalize-length`. Tier: single FULL. The fix is to a silent loss of a touching boundary, so correctness carries the review.
- **`chart-inversion-is-spelled-three-times`**: `session_01Y3MbonGyom32v6Vp4QGBzt`, branch `chart/chart-inversion-one-reading`. Tier: single FULL, because it unifies a reading across three crates. The lane stops if D3 leaves a fork.
- **`chart-bound-outer-span-decides-a-poisoned-margin`**: `session_013ALUU1XvFKmRJjbgmBe3B9`, branch `chart/outer-span-poison`. It measures first, and stops if the right reading changes what k-lint may ask. Tier: single FULL if built.

## 2026-10-24 — the region description is decided (the designer pair converged)

**Round 1.** Both designers corrected the same premises:
- W2 reads `chart_boundary(..).is_ok()` as a topological claim, so relaxing the pole refusal inside `chart_boundary` is a wrong-boolean hazard;
- the P0 band was wrong;
- `hi` needs an on-face witness, not exact cells;
- the round-hole lune buys nothing.

They split on the design:
- **A:** decide cells in model space, off the face's edges on their carriers plus the existing point-in-face doors, with no chart region and no arms.
- **B:** one exact chart region minted at rest, with cut edges, inside and outside certificates, and a window-scoped inf-arm door.

**Round 2** (each was shown the other's report). A held, and argued that an exact region with `contains(p)` would be a second point-in-face reading beside the boolean's doors (`[one-way.readings]`). B withdrew its region and adopted A, with four amendments:
1. separation is carrier-aware, never box against box;
2. the cylinder and cone point doors read rings BEFORE clearance switches;
3. `hi` is a witness;
4. W2 gets its own predicate off the walk.

B also conceded that minting at rest does not close the min_separation lane split. It checked the torus `v`-winding concern and found it is not a hole.

This was not a crossover, because only one designer moved, so reconciliation ended there. No ratified text moves, and the design follows `[intrinsic]` and `[one-way.readings]`. It is therefore a high-confidence design decision taken here, not an `[ev]` fork (`memories/orchestration-model.md`). Ev reviews it retroactively. Where the two reports differed on sequencing, the amendment-2 prerequisite is taken: no holed-cylinder regression.

**The slate now:**
- `clearance-cells-are-decided-in-model-space` (P1 H), parked on `curved-face-containment-answers-none-on-a-ringed-face` (P1 H, claimed from CLEAVE). The pole, round-hole, cone/sphere/torus and exact-region rows are closed into it.
- `w2-reads-a-one-sheet-predicate-off-the-walk` (P1 M, new). It measures first whether a sphere-octant boolean refuses today. If one does, the row is P0.
- The arms rows shrink to typing work plus a rename.
- `Certified::LowerBoundOnly` still waits on CLEAR's lane-split row, under either design.
- The outer-span lane is narrowed to Phase 1: its check is deleted with `chart_bound.rs`, so it reports whether the poison reaches a surviving predicate.
- `w2-torus-v-winding-is-unverified` (CLEAVE) is closed, checked.

## 2026-10-24 — dispatched (05:25Z)

- `w2-reads-a-one-sheet-predicate-off-the-walk`, with its passenger: `session_01DScnVgqrsdRV3qkW1hn4Qc`, branch `chart/w2-one-sheet`. It measures the sphere-octant boolean first. Tier: single FULL, on W2's soundness.
- `curved-face-containment-answers-none-on-a-ringed-face`: `session_01ASbZAvowuj7eiwTjedp2hN`, branch `chart/ringed-curved-doors`. Tier: DUAL, because these doors decide booleans (the ray lane, the pierce arm) and a third point-in-face reading may need unifying.
- The three arms rows as one PR: `session_017S6REdqJJkQ1SwVnS3RsvZ`, branch `chart/arms-typing`. Tier: single STYLE.
- Designers A and B are done; their sessions stay unarchived until this decision has been read back.

- 2026-10-24 — W2 lane Phase 1: **not P0.** A sphere octant (with its pole as a vertex) against six boxes under ∖, ∪ and ∩ returns a body in all 18 cases, and `describes` is never called. No classify arm marks a sphere component essential, which is correct: S² is simply connected. Steer for Phase 2: no unreachable sphere clause. W2 is asked of no simply connected carrier, so on a sphere either the predicate is a typed kernel defect or its arm answers by the carrier's topology. The same test goes for every arm the classifier cannot reach.

- 2026-10-24 — **Outer-span lane (PR 5656, gate green).** The lane built the fix before my Phase-1-only redirect reached it. Its finding makes the fix worth landing as an interim. The span check guards the OuterSpansPeriod premise that W2 reads at f64 through `describes`, so reading 1 is right and the NaN was not a probe artefact. The fix: `ChartEdge::Envelope` gains a finite `reach` box (`chart_box`), and the span check reads its hull. All 41 rows now decide, and k-lint rule 1 goes from 43 to 2. It is in single FULL review (`session_012KfN3uK7rd77NmmhzR6yCJ`); claim 1 asks whether the NaN let `Ok` stand, which would have been a live soundness hole. The W2 lane has been told to keep the premise in its walk predicate. The red nightly the lane found is on INTENT's ground; it is noted on INTENT's log.
- 2026-10-24 — Normalize lane: PR 5654 is gate green, and its FULL review is `session_01SMmWH32k3EX7PLZ2uB46jH`.
