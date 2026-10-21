# PATHS exit walk

**Program:** PATHS, the profile lattice (`work/paths/`), opened 2026-09-16 at S-BOOL's exit and closed 2026-10-21. The criteria are `work/paths/plan.md`'s "Exit shape", taken one clause at a time. **Narrative:** `work/paths/log.md`.

## The criteria

1. **One spelling of each carrier form: met.**
   - `geom_core::Arc2` is the one arc carrier for profile and geom-brep (#3504).
   - The stored bulge is retired (#3527), and constructions store the carriers they build (#3774).
   - The half-angle quotient has one home, `sugar::split_half_tan` (#5403).
   - The original row, `arc-carrier-has-three-spellings-under-a-comment-saying-one`, went to ROUND in the 2026-09-20 cut and is still open there. The three `arc_carrier` copies it cites no longer exist, so ROUND only has to close it.
2. **Every closer declares its arrival: met.** Ev ruled `arc_onto(target)` + one director (#5374). It is built in #5384, which admits `Start.arrives_tangent()` on the `Via`/`Center` legs and routes every closer's seam through `seam_joint` (`arc-closer-constructed-from-arrival-tangent`, `via-and-center-arc-closers-declared-arrival`). Two vocabulary gaps move to PROFILE: `no-fused-verb-takes-a-departure-tangent-arc-toward-a-point` (P2) and `radius-arrival-has-no-start-anchored-close` (P3).
3. **The comparator and `carriers_are_identical` agree with the sixth round: half met.**
   - The comparator does (#3780, `lift-comparator-misclasses-a-declared-joint-difference`): a lift may add a declaration and never drops one.
   - **`carriers_are_identical` does not.** It still decides `path_carrier_identity` (`crates/profile/src/path.rs`, called from `path/family.rs`). Its row, `carriers-are-identical-reads-carrier-identity-at-the-lattice` (P1), went to ROUND on 2026-09-20 and is open there, unruled.
4. **The sketch plane carries its witness to the read-back boundary with one lift rule: met.** This is #5068 (Ev ruled #3775 and ruling A).
   - `SketchPlane` holds one `RigidMotion`.
   - `ProfileLift` is retired, and every evaluation runs the guided second pass, except the `f64` lane over the nominal, which keeps pass 1's form, held bit for bit to the second.
   - It closed `sketch-plane-holds-the-affine-and-the-witness-dies-at-the-read-boundary` and ROUND's `sketch-plane-lift-choice-has-no-home`.
5. **The tangent legs refuse an over-full angle, typed: met.** `Sweep` and `ArcLen` refuse an angle that is not definitely short of 2π, as `PathError::ArcSweepNotShortOfFullTurn` (#3774). A stored arc past a full turn no longer builds as a line (#5115).
6. **Every unit merged on its own green hosted head: not met.**
   - #5068 merged with three `ε = 1e-12` sweep rows red. They were inherited from main and filed on BAND, GERM and FLUX.
   - #3774 and #4169 were green on their own heads, but their change filters skipped rows that then went red on main: the klein pin, which CLEAVE's #4083 fixed forward, and the seam row at `1e-6`, which #4205 fixed forward.

## Beyond the charter

- **The #3218 lowering**, ruled by Ev on #3218 and #3453.
  - Built: the canonical segment (#3224), the fixture door (#3231), geom-brep's full turn (#3254), and one-segment loops through the builders and onto one wall with a wrap edge (#4169, #4226).
  - Still parked: units 4 and 6, and the epic row itself. They move to PROFILE.
- **Validation's silent and false answers**, closed: the fillet radius at `1e-6` (#4259), arc departures (#4264), the tangent-pair miss (#4276), the shallow arc×arc corner (#4290), the secant arm at unequal radii (#5032), internal tangency (#5175), and the joint reading its own turn (#5377, Ev's #5354).
- **Shared homes taken across fences:** `geom_core::Circles`, with RESTFRONT's P0 (#5124), and SECT's in-face one-site arm (#5159).
- **`arc_onto`** (#5384), the chord director (#5370), and the **near-full-turn** tangent arcs (#5403).
- **The arc read-back tripwire** (#5449): every constructed arc is read back against what was authored, at the door. It also found and fixed the `Via` cross term.
- **The hairpin line fillet** (#5441), re-banded P0 at review.

**Re-homed:** 46 live rows are cut along their priority seam into four new tracks, none over budget: PROFILE (10 rows, 30/30), PROFILEREST (13, 29.5), PROFILETAIL (12, 24) and PROFILEPROSE (8, 12.5). The station-turn row goes to INLANE.
