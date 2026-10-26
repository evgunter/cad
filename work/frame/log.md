# FRAME log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/frame/plan.md`. A/B band 5900-5999
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Cut from PROPS (2026-09-20)

Ev, in chat: PROPS' slate had reached 66 open rows, and the question
put back was whether to cut new programs for the logical chunks and
leave in PROPS only what that orchestrator would finish itself. Ev:
*"that sounds like a good plan!"* This program is one of three cut
that day (QUAD, ENCL, FRAME). No orchestrator is seated yet — the
items, the charter and the lanes are here so that seating one is the
only remaining act.

The cut deliberately does NOT contort the territory globs to keep the
at-rest overlap check quiet. Ev, same conversation: *"it's ok if units
have shared ground, they should just be aware of each other if working
at the same time"* — so the `keep_out` above records the real
relationships rather than a partition, and the awareness mechanism is
the per-branch territory check plus the announced-seam convention.

## DECIDE-3 told FRAME what the tier now folds (2026-09-22)

`work/decide/the-decision-door-is-opaque-to-the-tier` is the fold this
program waits on, and DECIDE-3 (PR #3039) is where it landed: every
`sqrt` atom is keyed on its argument's VALUE CLASS at one mint door,
and a `Select` or a `min`/`max` whose comparison is certified over the
leaf's box takes that arm (counted `sign_gated`, never a theorem). So
a frame built through `Vec3::orthonormal_basis`'s sign hull is no
longer opaque to the tier: the tilted derived boss certifies at both
halves and both lifts, and the tilt-`u` one certifies outright.

What moved on this program's own row,
`a-widened-derived-placement-normalises-a-straddling-newell-sum`: on
the TILTED document at `half = 5e-2` the clause-1 `newell_plane_residual
… margin is invalid` this row names is no longer what refuses — the
tier settles the frame, the cap plane goes through, and what stops the
document is `interval_span_forward` over a box that wide. The HEIGHT
document's `5e-2` rung still refuses on this row's margin, so the
class is NOT answered; what moved is which document reaches it first.
The row carries the measurement.

**DECIDE-3 lands.** One measured trade goes with it: on R2's link,
rule G costs `carrier_on_surface_2` sixteen theorems while the
document's totals rise, and that predicate is re-baselined and PINNED
on both sides (`[98, 0, 0, 10]` with rule G shut against
`[82, 0, 6, 20]` shipped) in
`editor-core/tests/decide_3_split_rows_interval`, so a drift reds and
says which. The measurement and the remedy that was tried and rejected
are on `work/decide/rule-g-trades-sixteen-of-the-links-carrier-on-surface-2`,
which stays OPEN at P1 for SYM-9.

The unit briefly stopped on that trade against a "no decision LOST"
acceptance clause; Ev corrected the attribution on #3039 (02:03Z) —
that sentence was the orchestrator's spec text, not the ruling, and
the ruling is shape 1 plus "never skip out on a change that would make
the code better because it would require rebaselining". So the folds
above are what this program gets, and they are merged rather than
parked.
- 2026-10-13 — Seam note from ENCL: PR 5003 merged. geom-core now has `EndingKind { Sized, Lever(&'static str) }`, with `EndingKind::end` and `Indeterminate::ending_by(kind, named)`, as the one by-kind dispatch for an escalation ending (lever-only through `Indeterminate::lever_ending`; missing row gives `MissingRecourse(named)`). profile `validate` and editor-core `decision` call it. `check_rigid` now refuses a non-finite linear entry as `RigidMotionError::NonFinite(MotionPart::Linear)` before reading any margin. (ENCL orchestrator)

## 2026-10-21 — FRAME picked up; the slate read against the D10 hold

New FRAME orchestrator (session 014jgsWQiGQD6AZDRDeqc7uu); status `active`. The slate was last read at the cut (2026-09-20), and INTENT has since built most of what it stood on, so each row was read against the code on `31b59b01` and against the hold (Ev, `5f7a1c71e3`: hold off starting units that meaningfully use dependency, placement or intent).

- **Closed, done elsewhere:** `orthonormal-basis-poisons-vertical-planes` (the sign hull; pinned at `Interval`), `frame-from-unit-aim-takes-perp-and-its-norm-as-an-unverifiable-pair` (`RigidMotion::from_aim` decides `perp` itself), `rotation-about-re-divides-a-decided-axis` (`Mat3::rotation_about_unit`). Each row's `## Closed` cites its evidence.
- **Parked on INTENT:** `a-widened-derived-placement-normalises-a-straddling-newell-sum` on stage 3 E; `path-start-frame-is-the-only-frame-from-a-tangent-and-is-named-for-the-start` and `two-doors-answer-an-axis-off-a-tangent-with-no-rule-for-which` on D′. Each row says why. PR #5547 (merged today) and `[ev]` PR 5494 are where the "who mints a frame" question now lives.
- **Dispatchable:** `normalize-overflow-yields-zero-axis` (P0 E), `frame-degenerate-escalation-ends-a-poisoned-gauge-without-the-note` (priced P3 E), and the geom-core half of `exact-frame-mints-cover-three-of-the-world-frames` (re-priced M, `design: true`); moving the tour's `sketch_frame`/`authored_frame` onto new mints would cross INTENT's tour PR 5562 and D′'s helpers, so that half waits.

**Review posture (set at first dispatch):** the repo-wide tiers. `normalize-overflow` draws a single FULL review: the change is small, but it moves a total function's output at an extreme of its range and every rotation constructor reads it.

**Sequencing:** once `normalize-overflow` lands, FRAME has no dispatchable P0 or P1 row except the exact-mints design question; the sitting weighs that and hands the track back.

## 2026-10-21 — normalize-overflow lands (PR 5572)

Full single review (tier as logged at dispatch). Round 1: mergeable with fixes, no MAJOR. It falsified two of the PR's own claims: a fixed 2⁻⁶⁰⁰ scale-down dropped bits the plain spelling had right for a witness in [2⁵⁰⁰, 2⁵¹¹), and the documented low cut was really 2⁻⁴⁴⁹. It also found seven stale prose sites and the STEP and `unit_from_components` doors. Orchestrator rulings: the branch on `is_bounded` reads stands, because every arm denotes the same real, the Sym ‖v̂‖ = 1 theorem still fires and the branch-free spellings are worse. The new f64/interval disagreement on a `(1e200, 0, 0)` pattern rides the door row; DL6 points at the f64 door's refusal as the suspect side. The fix pass redesigned the scale into four sharp arms and shared one `geom_core::pow2` with `spline/weight_scale.rs`. The re-review (delta only) found nothing falsified: 800k cut probes, 28M mixed-binade vectors, and `pow2` agreeing with `powi` over every reached exponent. A class finding with no durable home before this: generic code over `T` cannot read an exponent, so an exact rescale is spelled as fixed arms cut by `is_bounded` reads. `Real::is_bounded`'s doc now states when that is sound.

Seam (INTENT, noted by the exact-frames lane): the render workflow's GUI montage is red on main. Three committed `demos/renders-gui/datum-extrude-place-profile-*.png` cells now render under a new name with no re-baseline. This matches the "gui montage" red in INTENT's handoff of nightly 65.

## 2026-10-21 — exact frames are written turns (designer pair, converged)

`exact-frame-mints-cover-three-of-the-world-frames` weighed by a designer pair (labels on `analysis/design-fork/frame-exact-mints`). Both, in round 1: no new frame vocabulary. Every axis-aligned frame is already reachable with no decision as an exact axis turned by a written fraction of a turn, the vocabulary D10 gives a placement's values; the receipts were written in a different one. Decided by the orchestrator (no ratified text moves; not Ev's fork): the row's decision section says what is owed. Dispatched as `frame/exact-frames-are-turns`, review tier **single STYLE**: a new constructor plus caller rewrites, believable by reading; the bits claim (a quarter turn about x̂ equals Gram–Schmidt's frame from `(x̂, ẑ)`) is pinned by the unit's own tests. Filed from the pair's off-question findings: `rigid-motion-radian-rotation-doors-twin-their-angle-doors` (P3 M) and `python-sketch-plane-from-frame-decides-exact-axes` (P3 M, parked on D′). `Angle::of_turns`'s fate is left to INTENT's expression evaluator (`expr.rs`), its only document caller.

## 2026-10-21 — exact frames land (PR 5573); FRAME handed back

The style review found no correctness MAJOR. Its main finding: the fix minted the defect it closes. One turn helper existed three times (tour, sweep fixtures, a geom-core test), and the new `sketch_xz_at`/`xz_plane` named the same plane as `axes_zx` with the opposite normal under a letter-swapped name. The fix pass:
- put one infallible library door in their place, `RigidMotion::turned`;
- removed the xz names, and filed `SketchPlane`'s missing `yz_at`/`zx_at` on PROFILE rather than filling the gap in a consumer;
- added `Turns::quarters`, because a literal quarter turn carried an `expect` that could never fire, and `join2_r2_probes` denies `expect`;
- fixed the docs and tightened two tests.

Orchestrator's read on the fix pass: `turned` takes an origin, as `axes_*` do, and that deviation from the brief is accepted. No frame moved relative to the earlier render. If the nightly k-lint fires on `tour_frame_axis`'s smaller sample, re-derive its baseline.

**Handed back, status `ready`.** The sitting's P0 and P1 rows are done (`memories/orchestration-model.md`). What remains:
- dispatchable at P2 and P3: `direction-doors-refuse-finite-directions-normalize-now-answers` (P2, a fork per door, touches INTENT's `mate/coset.rs`), `project-and-reject-collapse-past-the-squares-range`, `frame-degenerate-escalation-ends-a-poisoned-gauge-without-the-note`, and the exact-frames residue rows;
- parked on INTENT's stage 3: the widened derived placement (on E), the two `path_start_frame` rows (on D′) and the Python `SketchPlane.from_frame` row (on D′).

A successor should run `work.py incoming --program frame` first: E and D′ are the triggers.

- 2026-10-26 — (tracker sweep) `authored-and-derived-directions-decide-a-unitless-norm-against-the-length-band` (P3 M) arrives from FLUXHOLD by `mv`, id kept. It is off the D10 hold, and it asks of the same direction doors what `direction-doors-refuse-finite-directions-normalize-now-answers` asks, so the plan's lanes now pair them. Its hit list is refreshed against main `bb25fa4f`. Every other live row was re-read against the tree and kept. Three rows carry no `priority` (`a-frame-turned-by-a-radian-angle-is-spelled-as-its-columns`, `exact-axis-and-reference-frames-still-decide`, `turns-has-two-answers-to-an-integer-past-the-mantissa`), and those three have no `cost` either; band them at the next sitting.
- 2026-10-26 — (easy-fixes lane) Closed `frame-degenerate-escalation-ends-a-poisoned-gauge-without-the-note`: the `Degenerate` escalation ends through `lever_ending`, pinned on a NaN margin.
