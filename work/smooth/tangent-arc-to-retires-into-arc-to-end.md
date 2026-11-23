---
id: tangent-arc-to-retires-into-arc-to-end
kind: unit
title: tangent_arc_to retires: every caller becomes arc_to(End), Step and ProgramStep TangentArcTo go, the corpus re-blesses
status: parked
opened: 2026-11-22
priority: P1
cost: M
parent: tangent-arc-to-needs-an-explicit-tangent-where-line-takes-the-directed-point
blocked_on: [arc-to-gains-the-end-mode]
---


Ruled on PR 7251 (Ev, 2026-11-21): `tangent_arc_to` is deleted in favour
of `arc_to(End{p})`. The text this implements is `docs/PATHS-DESIGN.md`
as PR 7251 leaves it (read at `7c8acfd225`): §3's table, where TIER 1 no
longer lists `tangent_arc_to`; §2 Legs; §2 Closure's stadium; §2a item
6's table, whose recourse is `.tangent().arc_to(End { p })`; and §4
items 1 and 4.

## What is true when it is done

- **No `tangent_arc_to` anywhere**:
  - `verb TangentArcTo` and `do_tangent_arc_to` are gone from
    `crates/profile/src/path/program.rs`;
  - `TangentArcTarget` and its impls are gone from `path.rs`, and the
    kernels stay under `End`'s name;
  - `Step::TangentArcTo`, `StepForm::TangentArcTo` (`path/shape.rs`)
    and the arm in `structure.rs` are gone;
  - `Verb::TangentArcTo` is gone, and its content-key tag `21` moves
    into `RETIRED_VERB_TAGS` (`crates/editor-core/src/eval/mod.rs`);
  - `ProgramStep::TangentArcTo` and its arms in `program.rs`,
    `step_handle.rs`, `trace.rs` and the viewer are gone.
- **Every caller is respelled**: `.tangent_arc_to(t)` →
  `.arc_to(End { p: t })`, and `Step::TangentArcTo(t)` →
  `Step::ArcTo(ArcData::End { target: t })` (and the `ProgramStep`
  twin). The sweep's hit list goes in the PR. On main at the cut,
  `tangent_arc_to(` was 136 occurrences in 44 files and `TangentArcTo`
  110 in 61 files:
  - `crates/profile` (src and tests) the bulk;
  - `crates/editor-core`, `crates/viewer`, `crates/sweep`;
  - `crates/step-import/tests/cusp_round_trip.rs`;
  - `demos/tour/src/klein.rs` and `curvedcut.rs`, and
    `demos/tour/tests/verbs_teapot*.rs`;
  - `docs/renders/band-2026-11/scenes.rs`;
  - the Python tests.
- **Tests whose subject is the tangent arc keep their assertions under
  `End`**, renamed if the name says `tangent_arc_to`:
  `crates/profile/tests/tangent_arc_near_a_full_turn.rs`,
  `tangent_arc_on_the_ray.rs`, `arc_onto.rs`, and `path.rs`'s in-crate
  rows that build `program::Verb::TangentArcTo` (five sites). A row
  asserting a message such as "step 1's leg (tangent_arc_to)" asserts
  the new verb name.
- **The lift**: `crates/profile/src/lift.rs` emits
  `Step::Tangent, Step::ArcTo(ArcData::End { target })` where it
  emitted `Tangent, TangentArcTo`, in the tangent-joint arm and in
  `repair_same_carrier`. The module doc says so. `lift_census.rs`
  passes unchanged, or its moved counts are explained.
- **The recourse texts** name `.tangent().arc_to(End { p })`:
  - `JUNCTION_TURN_LEVER` in `crates/profile/src/fillet_recourse.rs`
    has only its arc example re-spelled. Its `line(len)` clause stays
    true until the next unit.
  - `arc_onto`'s `JunctionTangent` recourse (the `ArcOnto` row doc,
    `path/onto.rs`).
  - The tests that assert either.
- **Python**: `PathDirected.tangent_arc_to` is gone from `pncad.pyi` and
  `py/path.rs`. `surface_census.rs` has no `Verb::TangentArcTo` row.
  `tests/test_paths.py`, `test_north_star.py`,
  `ty_fixtures/legal.py` and `test_binding_census.py` are respelled.
- **The schema break**:
  - A document naming `TangentArcTo` refuses `PersistError::Unreadable`
    with `REGENERATE_RECOURSE`, and a row asserts it.
  - `tests/golden/golden.cad` is re-blessed
    (`M4_PR6_BLESS_GOLDEN=1`). It holds one `Tangent, TangentArcTo`
    pair, which becomes `Tangent, ArcTo{End}`.
  - `tests/corpus/wire_rv_bytes.json` is re-blessed (`PNCAD_BLESS=1`).
  - The literal pins in `tests/switch_program_vocabulary.rs` drop the
    word.
- **No frame moves.** The kernel is the one `End` already runs, so the
  tour and the wild corpus render the same geometry. If the nightly
  shows a moved frame, that is a finding to report, not a re-baseline
  to take.
- `docs/GUIDE.md` and `docs/guide/north-star-audit.md` (PYDOOR's) are
  re-worded. So is the PROFILE row that spells the verb,
  `degenerate-arc-chord-names-two-cases-under-one-kind`.

## Left for later units

- `continue_to`, the bare leg-end rows and the final §4 item 1 recourse:
  `continue-to-and-the-bare-leg-end-rows-retire`.

## Check first

- **The historical goldens.** `crates/editor-core/tests/bool13_goldens/`
  bytes are older-build documents asserted to refuse Unreadable naming
  `unit` (`bool13_r1_probes.rs`
  `older_build_goldens_refuse_unreadable_and_name_the_break`,
  `bool13r2_probes.rs`). In v9 to v19 the first literal sits at line 40
  and the first `TangentArcTo` at line 237, so the refusal should still
  name `unit`. Confirm it. If a golden now names `TangentArcTo`, stop and
  report: those bytes are not this unit's to re-bless.
- **Content-key goldens may move**:
  `tests/golden/m10_6_certifying_keys.txt` and the perf census files.
  The recorded step changes from `TangentArcTo` to `ArcTo(End)`. Keys
  are process-internal and never persist (`verb_tag`'s doc), so a moved
  key is re-blessed and named in the PR, not treated as a format break.

## Rows

- Carries part of the parent.
- Retires the `Verb::TangentArcTo` row of BIND's census.
- Tell PYDOOR (Python spelling), the away channel (schema), TRACE
  (`trace.rs` arms) and CHROME, DOORS and VGEOM (the viewer's verb
  menu loses an entry).

## Ground crossed

`python3 scripts/work.py territory` on the branch will name:

- DOCTAIL, SELFREF, BORDERS and TALLY (editor-core src);
- CIW and GREEN (the goldens, the sweep tests);
- PYDOOR and BIND;
- CHROME, DOORS and VGEOM.

Every one of these changes is a respelling; the coupling is the
vocabulary itself.

## Review

Tier **single (STYLE)**: once the differential in `arc-to-gains-the-end-mode`
holds, a mechanical respelling, compile-found almost everywhere. The
reviewer reads the sweep's hit list and the schema re-bless.
Difficulty **M** (large, shallow).
