---
id: ellipse-to-draws-the-stretched-circle-from-a-point-tip
kind: unit
title: ellipse_to(Stretched { c, along, factor, winding, p }) draws the stretched circle from a Point tip, lowers to the ellipse segment and lifts back
status: parked
opened: 2026-11-22
priority: P1
cost: H
parent: an-ovoid-cannot-be-authored
blocked_on: [a-profile-segment-is-an-ellipse-arc]
---



Unit 3 of `an-ovoid-cannot-be-authored` (Ev, PR 7289), part 1's
authoring: "one new leg from a Point tip,
`ellipse_to(Stretched { c, along, factor, winding, p })`: the circle
about `c` through the tip, stretched by `factor` along `along`, ending
at `p`". The lattice it is written in is `docs/PATHS-DESIGN.md` §2
"The binding lattice" and "Legs" as PR 7251 ruled them (merged
2026-11-22): `ellipse_to` is a **self-aiming** leg (its data fix its
own departure), so it departs a `Point` of either flavour, is
junction-checked on a directed point (§4 item 1, `JunctionTurnWithinTolerance`
as PR 7260 states it), and ends at a directed point that `.tangent()`
chains on from. Checked against the ruled lattice on 2026-11-22: it
fits as written, beside `arc_to`'s `Center { c, winding, p }`.

## What is true when it is done

- `Stretched { c, along, factor, winding, p }` (beside `Center` in
  `crates/profile/src/path/verbs.rs`) and the verb `EllipseTo` in the
  vocabulary table (`crates/profile/src/path/program.rs`, the `verb`
  macro rows beside `ArcTo`; `Step::EllipseTo`, `Verb::EllipseTo`,
  `Step::map_scalar`, the replay's transition table).
- The lowering is `Center`'s, done in the un-stretched frame and mapped
  back (`crates/profile/src/path.rs`): the tip is pulled back by
  `1/factor` along `along` about `c`, the circle `Center` lowering runs
  there (its sweep is the stored eccentric-anomaly sweep, the same
  `4·atan`), and the carrier is pushed forward. The only redundant
  datum is `p`, checked on the stretched circle as `Center` checks
  equidistance (PATHS-DESIGN §3 "The arc about a centre": checked,
  never repaired), refusing typed with both readings named.
- A `factor` within the band of 1 refuses typed with the recourse
  "write `arc_to`" (the ruling). `factor ≤ 0` and a non-unit or zero
  `along` refuse typed. The refusals carry recourse sentences in the
  shape `crates/profile/README.md` "The refusal envelope" requires.
- `p` takes the targets `Center`'s `p` takes (a point, or `Start` to
  close). This is the analogue with `Center`, not a ruling; if the
  spec finds a reason it should not close, ask.
- No fused fillet takes an ellipse: `ellipse_fillet` and its kin are
  absent verbs (the ruling; PATHS-DESIGN "Fillet carriers are
  line/arc"). `.tangent().fillet(r)` after an ellipse leg is the
  uniform ray extension and needs nothing new.
- `profile::lift` (`crates/profile/src/lift.rs`) lifts an ellipse
  segment back to `ellipse_to`, and a lifted-then-replayed loop is
  bit-identical where the `Center` lift is
  (`the-lift-center-writer-loses-the-stored-radius-bits` is that
  writer's known gap and applies here too).
- The program form carries the step everywhere the vocabulary is
  exhaustive: editor-core's `program.rs`, `step_handle.rs`, `trace.rs`,
  `coincide.rs`, `eval/wire.rs`, `checks/at_rest.rs`; the viewer's
  `sketch.rs` (`fresh_step_at`, `not_well_typed`), `forms.rs`,
  `widgets.rs`, `drafts.rs`, `pane/profile.rs` (a field for each of the
  five arguments); pncad's `trace.rs`; pncad-py's `py/path.rs` and
  `pncad.pyi`. A saved document round-trips a profile with an
  `ellipse_to` step.
- `docs/PATHS-DESIGN.md`'s Legs section and §3 table gain the
  `ellipse_to` row, and its "Fillet carriers are line/arc" sentence
  names the ellipse; `crates/profile/README.md`'s VQ7 says the ellipse
  segment exists and the NURBS legs still wait. These implement the
  approved PR 7289 and land with the change.
- Every lane scalar replays the leg (`f64`, `Interval`, `Sym`): a
  `Sym` replay reads the authored data through the stored fields.

## Check first

That a self-aiming leg from a directed point whose turn is zero
refuses (`JunctionTurnWithinTolerance`) means **a tangent joint INTO an
ellipse arc cannot be authored**: the lattice's smooth joint is
`.tangent()` followed by a heading-taking leg, and `ellipse_to` has no
heading-taking mode. A tangent joint OUT of one can (`.tangent()` off
its end). Confirm on the bud's meridian and on a 2:1 vessel head (the
cylinder wall then the head, which joins tangent INTO the ellipse when
traversed from the wall) which way each must be traversed. If a
target family the designers named cannot be written at all, **stop and
report**: a heading-taking ellipse mode is a design question, not this
unit's.

## Leaves for later

- The pair rows a multi-segment loop needs:
  `profile-validation-reads-an-ellipse-arcs-pairs-on-the-planar-reader`.
  Until it lands, a loop drawn with `ellipse_to` and any other leg
  refuses typed at validation.
- A heading-taking ellipse mode (`.tangent().ellipse_to(..)`), if Ev
  wants one: not ruled.

## Rows and ground

- Changes no other row.
- Crosses SELFREF and DOCTAIL (`editor-core/src/program.rs`; SELFREF's
  U1 reshapes `ProgramStep`'s point arguments in the same file, so
  coordinate: `c` and `p` are point arguments and take whatever
  representation U1 gives `Center`'s), REPEAT (`program.rs`), CHROME,
  DOORS, FORMWORDS and VSEAM (the viewer), PYDOOR and BIND
  (`pncad-py`). The persist schema is contended: announce on the away
  channel before landing. The vocabulary is PROFILE's: announce on its
  log.
- Review: **single (FULL)**: a ratified page's leg table moves and the
  lowering's read-back is a correctness argument. Difficulty **M**.
