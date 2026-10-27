# SPEC: QUAD — the measurement door refines until a predicate resolves

Unit: `work/quad/the-measurement-door-refines-until-a-predicate-resolves.md`.
Target text, ratified on `[ev]` PR 5574 (2026-10-22): DESIGN.md
`[tolerance.measure]` and `crates/geom-brep/README.md` C12(7). The weighing
is fork-log row 188.

Line numbers cite main at `9f7f131` and drift. Each unit's lane re-greps.

## What is ratified (and so not re-opened by a lane)

- **One door.** A measurement refines each face until the caller's predicate
  over the enclosure resolves, until refinement stops paying (a floor no
  round removes), or until one per-face work bound. That bound is a single
  bound in a single unit, the same for every lane. It returns the narrowest
  enclosure reached and why it stopped.
- **The predicate is closed data at the public doors:** `Sign`,
  `AtMost { bound }` and `AtLeast { bound }` (a relation, `≤` or `≥`),
  `Contains`, `Resolution(len)`.
  - A deciding predicate is a named funnel predicate: it settles when
    definite, refines while it straddles, stops in the band, and is
    recorded every round it is asked.
  - `Resolution` is a sizing: it decides nothing and is not recorded.
  - No resolution finer than ε is offered.
- **No default** (Ev). Every caller states its predicate, a resolution
  included.
- **Refusals.** A deciding predicate still undecided at the work bound is
  the one quadrature refusal, and it carries the enclosure. An unmet
  resolution is the bracket, with the resolution it reached.

## Orchestrator rulings for the build (implementation level)

- **R1. Lanes know no target.** A lane (`geom-brep` `props/quad/`,
  `props/spiric.rs`) refines a face and reports, per round, the bounds and
  why it would stop:
  - `Refinable`: the next round is affordable and can narrow;
  - `Floor`: what remains is the part no round removes, or two rounds that
    did not narrow (`crate::descent`'s stall, on the lowest bound);
  - `Wall`: the next round's weighted work passes the bound.

  The convergence test (`props_quad_converged`) and the last-round test
  (`props_quad_last_round`) leave the lanes: whether a width is enough is
  the predicate's question, asked in topo. `props_quad_face_extent` stays: it
  is a gate on the face, not a stop.
- **R2. "Stops paying" is `Floor`, defined per lane** as the removable part
  of the width (the rule remainder) falling to at most 2⁻¹⁰ of the width
  (`removable·1024 ≤ width`); the rest is the irremovable part (the pads,
  and arithmetic). Each lane computes the two apart. The stall rule is the
  backstop for a lane that cannot split them.
  - Evidence for 2⁻¹⁰ over "removable below irremovable": PR 5589's
    `verbs_germarms::a_thin_bar_turned_about_two_axes_gets_through_the_join`,
    a cylinder face stopped at round 4 with 23% of its width still
    removable, which main certifies one round later.
- **R3. The work unit.**
  - One constant, `FACE_WORK_WALL`, in weighted cell-evaluations,
    **cumulative** over a face's rounds (`crate::descent::Work`), not per
    round. A per-round wall would let the 1-D lanes spend up to 2W.
  - Each lane states its own fixed per-cell weight. The weights are
    calibrated once, measured on this box and recorded with the
    measurement, so that one unit costs roughly the same in every lane:
    designer B measured a v-exact cell at about 30× a composite cell.
  - The wall sits at the worst face in the corpus that converges under
    today's caps, measured: the tour's `lily_leaf_a`, a knot-cut
    composite wall at round 7, 14,243,680, rounded up to 14,250,000. It
    bounds work, not rounds. A face costlier per round than that one
    (more knots, chords, edges or spirics) gets fewer rounds than its
    cap gave it, and stops at `Wall` with a sound bracket. No finite
    work bound keeps every face the caps certified.
  - Witnesses (PR 5589's DUAL review): review_r1's quarter torus at 12×
    on 9×9 conic spans (main r7, here `Wall` after r6), and a flat chart
    trimmed by a regular hexagon of curved arcs (`Wall` at r4).
  - `QUAD_MAX_ROUNDS`, `QUAD2_MAX_ROUNDS`, `QUAD2_RATIONAL_MAX_ROUNDS`,
    `TRIM_MAX_ROUNDS` and `LAST_ROUND_EVERY_LANE_RUNS` go.
- **R4. Narrowest, never last.** Each face keeps the narrowest sound
  enclosure it has reached, by its flux width, and the door reads that one.
  Rounds are independent sound enclosures, so their intersection is sound
  too. This unit takes the narrowest; the intersection is QUADTAIL's
  `a-face-runs-rounds-it-never-intersects`, and this unit leaves the
  seam open for it.
- **R5. Rings.** A ring's winding is decided at the round the face stops.
  A reading that straddles refines while `Refinable`, under PR 5580's
  `refinable` rule (`Enclosure` margin, not `terminal_sliver`). One that
  lies wholly in the band escalates at once.
- **R6. The predicate type** lives in `topo::props` as
  `pub enum Settle { Sign, AtMost { bound: T }, AtLeast { bound: T },
  Contains { lo, hi }, Resolution(Length) }`, each variant spelled as
  Python's constructor (`sign`, `at_most`, `at_least`, `contains`,
  `resolution`). It is re-exported through `pncad`, and Python takes it as
  data.
  - A relation's bound is a value of the body's scalar, not a measured
    enclosure: a relation between two measured quantities is the
    multi-body door (R7).
  - Each deciding variant carries the funnel predicate names its call site
    gives (as `RoleNames` do today).
- **R7. One multi-body walk.** The boolean volume backstop decides a
  relation between enclosures of three bodies. `SignCertificate::assembled`
  and `PastTarget::refine` become one walk over a set of certificates,
  advancing every open face a round at a time, settled by a predicate over
  the set's enclosures. `PastTarget::refine`'s cap at
  `LAST_ROUND_EVERY_LANE_RUNS` goes with the constant: faces refine to
  their own `Wall`.
- **R8. Deciders route through the door.** Each passes its own predicate:
  - check 7 and shell roles: `Sign`;
  - `at_infinity_side`: `Sign`;
  - the backstop: the multi-body `Relation`.

  `classify_shells` stops running the reporting walk before reading a role.
  It returns each shell's role and the enclosure the role's read reached:
  the sign walk's where the role was read off the volume, round 0's
  otherwise. Through B that enclosure rides in `ShellClassification`'s
  four number fields (`volume`, `volume_pad`, `surface_area`,
  `area_pad`), documented as that enclosure and not a measured volume; C
  replaces the four fields with the enclosure itself.
  `editor_core::checks::connectedness` reads roles only, which closes
  COINCIDE's `connectedness-reads-shell-roles-through-the-reporting-walk`.
- **R9. The public number doors** are `topo::mass_properties`,
  `SignCertificate::measure`, `pncad`'s re-exports, `pncad-py`'s
  `Body.mass_properties()`, `validate_geometric_measured()` and
  `ImportReport.enclosure`.
  - Each takes a required `Settle`.
  - Each returns `MassMeasurement { enclosure, stop }`, where `stop` is one of
    `Settled(verdict)`, `Met`, `Floor`, `Wall` or `Band`.
  - A number is `Settle::Resolution(len)`'s result.
  - `step-import`'s `ImportReport` stores the certificate, not a measured
    enclosure, and the caller measures through it with its own predicate.
    The import has no purpose to state, so it takes none.
- **R10. What goes:**
  - `PropsError::QuadratureBudget`, `quadrature_budget_recourse` and its
    `validate.rs` mirror;
  - `TargetUnreached`;
  - `budget_exit`, `last_round_refuses` and `last_round_width_lo`;
  - `OpenEnd::{Unreachable, Exhausted}` and `RoundOutcome::into_target`;
  - `reporting_hook`, `measured_at_target`, `target_walk` and `Resume`;
  - `FaceRun`'s four-field state (R11).
- **R11. One face state.** `FaceRun` becomes one enum with one
  transition, which closes `face-run-is-a-state-machine-kept-by-convention`.

## Units

Each lands green. None lands half a representation change.

- **A (lanes; geom-brep).**
  - R1–R5 in all five lanes, through one shared `Run`/stop home.
  - An **interim adapter** in topo maps the new lane outcome back to
    today's target semantics. The target reaches the lane as a per-round
    closure from topo, and the adapter's closure mints `props_quad_converged`
    through today's funnel call, so bits and the k-stream are unchanged.
    With it, every public signature, refusal kind and test outcome stays
    put, except where the moved caps flip an outcome, which is named.
  - geom-brep's whole-schedule doors (`nurbs_patch_face`,
    `cylinder_cut_face`) take the target closure now, and their tests move.
    There is no second adapter and no second `QuadratureBudget` mint.
  - The narrowest round also survives across one-round windows:
    `FaceRun::advance` keeps the narrower enclosure.
  - `budget_exit` goes, so a face that cannot reach the target runs to
    the wall before it refuses. That cost is accepted: a target-free early
    exit would hand back round 0's bracket where the wall buys a far
    narrower one, which is the paradox the design rejected. Rows that cross
    0.5 s go to the slow set. Report if the PR gate passes its 15-minute
    bar.
  - Moved numbers are re-pinned, with their cause named. They should be
    narrower or equal; any wider number is a finding.
  - k-lint's `predicate_roster` and `EPS_COUPLED_*` lose
    `props_quad_last_round`, with a K-REPORT note.
    `props_quad_converged` stays until B, because A's adapter still mints
    it.
  - Review: **DUAL** (H): it changes when every certified walk stops.
- **B (the door and the deciders; topo).**
  - R6–R8 and R11. The adapter's comparison becomes `Settle::Resolution`
    inside topo.
  - The public number doors keep today's signatures by passing
    `Resolution(QUAD_TARGET_LEN_FACTOR·ε)` internally: one marked interim
    call site per door, which C deletes.
  - `QuadratureBudget` is still minted at those interim sites only.
  - Review: **DUAL** (H).
- **C (no default; the public doors and every consumer).**
  - R9, R10, and the interim sites deleted.
  - `ShellClassification`'s four number fields give way to the role
    read's enclosure (R8).
  - The resolution-finer-than-ε panic at B's interim number door becomes
    a typed refusal of the caller's `Resolution`.
  - Every caller states its predicate:
    - the tour and the Python tests: `Contains` for closed-form checks,
      `Resolution` for a printed number, `Sign` for a sign;
    - the stl, viewer and examples rows;
    - `demos/wild` (it wants a number: `Resolution` at its own stated
      precision).
  - Python stubs and `pncad-py` tests follow. The thread-count digests are
    re-cut, with every moved line named.
  - Width asserts that test kernel precision for its own sake (projectbox
    `volume_pad <= want*1e-6`, skinned `PRISM_PAD_MAX`, `pad > 0`,
    `pad == 0`) pass the `Resolution` their assert needs, and say so.
  - Review: single FULL. It is broad but each site is mechanical; the
    reviewer samples call sites per crate.

## Acceptance (the exit walk reads these)

1. No round cap remains. No `QuadratureBudget`, `TargetUnreached` or
   `budget_exit`.
2. Every public number door takes a required `Settle`. Grep-proven: no
   `Resolution(` with an ε-derived literal outside tests that state why.
3. Check 7, shell roles, the backstop and connectedness pass a deciding
   predicate.
4. A face that converges within the wall today returns its bits
   unchanged under `Resolution(1024·ε)`.
5. The quarter torus at the default ε returns a bracket under
   `Resolution(1.024e-6)`, never a refusal, and its `stop` says `Wall` or
   `Met`.
6. A sign still open at the wall is the only quadrature refusal, and it
   carries the enclosure.
7. The thread-count digests re-cut at all three ε rows. k-lint's roster is
   green.
8. The rows this unit dissolves are closed into it:
   `quadrature-budget-conflates-its-lanes-and-budgets`,
   `quad-last-round-margin-has-no-sign-premise`,
   `a-sign-walk-escalates-on-the-last-rounds-in-band-reporting-reading`,
   QUADTAIL's `quad-round-capped-loops-do-not-read-the-shared-stop-rule`,
   COINCIDE's `connectedness-reads-shell-roles-through-the-reporting-walk`,
   `face-run-is-a-state-machine-kept-by-convention`,
   `a-quadrature-loop-returns-its-last-round-not-its-narrowest`, and
   `quadrature-budget-prints-its-two-lengths-alike` (moot once the variant
   goes). `C3`'s round-cap constant family lands here.

## Shared ground

The diffs reach several programs' paths, and each lane runs
`work.py territory` and posts on the owner's log:
- `topo/src/props.rs` and `validate.rs`: FLUX, TALLY, RESTFRONT;
- `boolean/ops.rs`: CLEAVE, TALLY;
- editor-core `checks.rs`: COINCIDE;
- `pncad-py`: LIB, BIND;
- step-import: EXCH;
- `demos/*`, and `tools/k-lint` and `docs/K-REPORT.md`: INSTR.
