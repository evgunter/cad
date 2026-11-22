---
id: arc-to-gains-the-end-mode
kind: unit
title: arc_to gains the End{p} mode at every Directed tip, on tangent_arc_to's kernel, with ProgramArcData::End and the Python End spec
status: open
opened: 2026-11-22
priority: P1
cost: M
parent: tangent-arc-to-needs-an-explicit-tangent-where-line-takes-the-directed-point
---


Ruled on PR 7251 (Ev, 2026-11-21: "End as drafted is fine"). The text
this implements is `docs/PATHS-DESIGN.md` as PR 7251 leaves it (read at
`7c8acfd225`):

- §2 Legs: "`End{p}`, the unique arc from the tip to `p` leaving along
  the bound angle, its end direction DERIVED";
  `.tangent().arc_to(End { p })` and `.angle(θ).arc_to(End { p })`.
- §2c's `ArcData` family: `End{p}` is heading-taking, its endpoint
  authored, DOF-complete from a Directed start, and "admissible at every
  Directed tip alike, authored or inherited, and at no Point".
- §3's `arc_to(spec)` row: "Directed → Point (Sweep/ArcLen/End)".
- §2 Closure: the stadium closes as
  `.tangent().arc_to(End { p: Start.arrives_tangent() })`.
- §2a item 6: `arc_onto` is `End`'s mirror.

## What is true when it is done

- **The mode exists.** `arc_modes!` in `crates/profile/src/path/program.rs`
  declares `mode End { target: Target<T> }`, so `ArcData::End` and
  `ArcMode::End` exist and `ArcMode::ALL` holds seven modes.
  - Every `spec_dispatch!` names it. `ARC_TO_DIRECTED` admits
    `End @ Point`, `@ Start` and `@ StartArriving`.
  - `ARC_TO_POINT`, `ARRIVAL`, `FUSED_POINT` and `FUSED_LEG_END`
    refuse it, since it has no heading to take at a Point and is never
    an arrival.
  - `FUSED_DIRECTED` refuses it until
    `the-fused-verbs-take-end-and-radius-at-every-directed-tip`.
- **One kernel.** The typed `arc_to(End { p })` on a Directed tip runs
  `tangent_arc_to`'s kernel (`tangent_arc_to_point` and
  `tangent_arc_to_start` in `path.rs`; `tangent_arc_geom`, `tangent_arc`).
  `verb TangentArcTo` stays and calls the same code. A differential test
  asserts `tangent_arc_to(t)` ≡ `arc_to(End { p: t })` bit for bit, for
  each target and from both an authored and an inherited tip. The test
  is deleted by `tangent-arc-to-retires-into-arc-to-end`.
- **Its refusals are `tangent_arc_to`'s**, each with a row:
  - a target at the tip is `DegenerateArcChord`;
  - a target on the ray ahead is a straight segment;
  - a target behind it is the full turn;
  - a tangent departure onto plain `Start` is `SeamTangent`.
- **The typed surface.** `End` has a spec type beside `Sweep` and
  `ArcLen` (`crates/profile/src/path/family.rs`, `verbs.rs`), and the
  Directed `arc_to` row takes it. To `Start` it returns `ClosedLoop`,
  so that row's output becomes per-spec, as the Point row's `PointLeg`
  already is. `arc_to(End{..})` at a Point does not compile.
- **The document layer and the schema (additive).**
  - `ProgramArcData::End { target }` (`crates/editor-core/src/program.rs`)
    reaches every exhaustive site: `res_step`, `step_bit_eq`,
    `loop_roles`, `try_map_slots`, `from_recorded`, `eval::feed_step` and
    `feed_lane_step` (`eval/mod.rs`), `step_handle.rs` and `trace.rs`'s
    `step_targets`.
  - `End` takes a fresh `arc_mode_tag` number, checked by
    `arc_mode_tags_are_injective`.
  - An `End` with a trace target refuses `TraceStepFault::NotARun`,
    as every non-centre arc does.
  - A document written before this unit still loads.
    `tests/switch_program_vocabulary.rs` pins the spelling `End`.
  - `tests/corpus/wire_rv_bytes.json` is re-blessed (`PNCAD_BLESS=1`)
    because its fixture must hold every variant, and `End` is the only
    word that moves.
- **Python.** `pncad.pyi` and `py/path.rs` gain an `End` spec class,
  taken by `PathDirected.arc_to` with a point or a closing target.
  `surface_census.rs`'s arc-spec census binds it. pncad's
  `prelude.rs` and `profile.rs` re-export it beside `ArcLen`.
- **The viewer.** Its arc-mode code follows `arc_specs_at`, and the
  per-mode sites name `End`: `crates/viewer/src/sketch.rs`, `forms.rs`
  and `widgets.rs`.

## Left for later units

- The fused `End`: `the-fused-verbs-take-end-and-radius-at-every-directed-tip`.
- Deleting `tangent_arc_to`: `tangent-arc-to-retires-into-arc-to-end`.
- Whether `End` will reach a trace's crossing is TRACE's row
  `an-arc-runs-to-a-crossing-only-about-a-centre`, which names "a
  tangent arc" among its missing forms. This unit leaves that refusal
  as it is.

## Check first

- **The scalar bound.** `tangent_arc_to`'s row is `T: Decide`, and the
  Directed `arc_to` row is `T: ArcCarrierScalar` (Decide + Bounds,
  `path/arc_fillet.rs`). Every scalar this tree replays a
  `TangentArcTo` at has to satisfy `ArcCarrierScalar`: f64,
  `Interval`, `Sym<T>`, `Dual<T>` under its `Bounds` impl's where
  clauses, and k_stats's `Probe`. Find where each is replayed,
  including the lift's lane pass and the sensitivity rows. If one of
  them does not satisfy the bound, stop and report. Widening the row
  or narrowing the scalar is a decision, not this unit's.
- **The open fillet.** Read what `arc_to(Sweep)` does at a Directed tip
  whose fillet is still open (`.fillet(r).angle(θ).at(m)`), and give
  `End` the same answer: proceed, or `ArcLegOnOpenFillet`. The
  designers did not check this cell.

## Rows

- Carries part of the parent.
- Tell TRACE that its "tangent arc" case is now spelled
  `arc_to(End{..})`.

## Ground crossed

- The persist schema (additive): announce on the away channel. DOCTAIL
  and SELFREF hold `program.rs`.
- BORDERS and TALLY: `eval/mod.rs`.
- CIW and GREEN: `wire_rv_bytes.json`.
- PYDOOR and BIND: Python.
- CHROME, DOORS and VGEOM: the viewer.

## Review

Tier **single (FULL)**: a new mode at four doors and a schema growth.
The kernel is reused, so the risk is in the plumbing and the bound,
not in geometry. Difficulty **M**.
