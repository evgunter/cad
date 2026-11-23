---
id: continue-to-and-the-bare-leg-end-rows-retire
kind: unit
title: continue_to and the bare leg-end rows of line, fillet, fillet_arc and the fused Radius retire: every caller writes .tangent(), the corpus re-blesses
status: parked
opened: 2026-11-22
priority: P1
cost: H
parent: tangent-arc-to-needs-an-explicit-tangent-where-line-takes-the-directed-point
blocked_on: [a-tangent-tip-is-its-own-state-and-line-to-is-checked-from-it, the-fused-verbs-take-end-and-radius-at-every-directed-tip, tangent-arc-to-retires-into-arc-to-end]
---


Ruled on PR 7251 (Ev, 2026-11-21):

- `continue_to` is deleted in favour of `.tangent().line_to(p)`.
- The bare leg-end rows of `line`, `fillet`, `fillet_arc` and
  `arc_fillet(Radius)` go.
- No verb is legal only after `.tangent()`.

The text this implements is `docs/PATHS-DESIGN.md` as PR 7251 leaves it
(read at `7c8acfd225`):

- §2 Legs: "a leg end continues only through a director … or through a
  self-aiming leg";
- §3's rows `line(len)` (Directed → Point only), `fillet(r)` and
  `fillet_arc` (Directed → …), and `arc_fillet` (`Radius` under
  `.tangent()`);
- §3's Refusals: heading-taking legs from non-Directed tips, "a leg end
  among them";
- §4 item 1's recourse, item 2 ("tangency enters only via a written
  declaration") and item 4;
- §6's straight-run seam.

## What is true when it is done

- **Rows deleted** (`crates/profile/src/path/program.rs`):
  - `Line`'s straight-continuation row at `DirectedPoint`, and with it
    `straight_continuation_kernel` (`path.rs`), whose doc said "nothing
    is declared" above a body that constructs the joint;
  - `Fillet`'s and `FilletArc`'s `HasPos<WithIncoming>, NoAng` rows;
  - `Radius` in `FUSED_LEG_END`, which keeps its three endpoint-full
    modes for `ArcFillet` and `ArcFilletArc`; `ray_extend` in
    `path/family.rs` goes if nothing else calls it;
  - `verb ContinueTo`, `ContinueTarget` and `do_continue_to`. Their
    kernels already serve the inherited `line_to` row.
- **Variants deleted.** `Step::ContinueTo`, `StepForm::ContinueTo`,
  `Verb::ContinueTo` (its tag `42` moves to `RETIRED_VERB_TAGS`; re-word
  `eval/mod.rs`'s note that `LANE_SCALAR` and `ContinueTo` share 42),
  and `ProgramStep::ContinueTo` with its arms in `program.rs`,
  `coincide.rs`, `step_handle.rs` (the `PenRun::Leg` arm), `trace.rs`
  and the viewer.
- **Assertions that hold on main:**
  - `Verb::states(TipState::DirectedPoint)` names no `Line`, `Fillet`,
    `FilletArc`, or `ArcFillet`/`ArcFilletArc` with `Radius`.
  - A replayed `[.., leg, Step::Line(len)]` refuses
    `ReplayErrorKind::Transition` at the `Line`, naming the state's
    verbs, and the typed spelling does not compile.
  - `.tangent().line(len)`, `.tangent().fillet(r)` and
    `.tangent().arc_fillet(Radius{..}, r)` build what the deleted rows
    built.
- **Every caller is respelled**, and the hit list goes in the PR:
  - `continue_to(t)` → `.tangent().line_to(t)`;
  - a bare leg-end `line(len)` → `.tangent().line(len)`, and the same
    for `fillet`, `fillet_arc` and `arc_fillet(Radius)`;
  - `Step::ContinueTo(t)` → `Step::Tangent, Step::LineTo(t)`, and the
    `ProgramStep` twin.

  On main at the cut, `continue_to(` was 104 occurrences in 25 files and
  `ContinueTo` 87 in 34. **The bare leg-end sites are NOT all found by
  the compiler.** Only typed Rust chains are. A shape scan at the cut
  found about 27 typed sites in 6 files of `crates/profile/tests`:
  `path_property.rs`, `bool8_r1_probes.rs`, `path_program.rs`,
  `r1_bool11_review_probes.rs`, `junction_turn_lever.rs` and
  `common/mod.rs`. It found about 40 `Step::`/`ProgramStep::` record
  sites in 13 files. Those, Python's `PathDirectedPoint.fillet` /
  `fillet_arc` / `arc_fillet(Radius)` callers, and stored documents
  refuse only at replay or load. Budget CI rounds for them, and sweep
  for the shape (a leg step followed by `Line`/`Fillet`/`FilletArc`/
  a fused `Radius`), not the symbol.
- **The fuzzers adapt by the table**: `chain_shape_random.rs` and
  `path_property.rs` draw from `Verb::states`. Any hand-written arm for
  the deleted rows goes.
- **The document layer keeps both readings of `LineTo`.**
  `ProgramStep::ContinueTo(OnTrace | AtX | AtY)` becomes
  `Tangent, LineTo(same)`, lowered to `Line(len)` along the inherited
  heading. `LineTo` with a trace target AFTER AN AUTHORED DIRECTOR
  stays legal: its target supplies extent only, and §2 Closure's
  "ill-typed" sentence is about point targets. Its rows in
  `crates/editor-core/tests/intent_s6_c_trace.rs` (`LineTo(AtX(..))`
  along a director) and `intent_s6_d_tangency.rs` must still pass
  unchanged. `trace.rs`'s `TraceStepFault` recourse sentences ("bind a
  heading before `line_to`, or `continue_to` off a piece's end"; "run
  to it with `line_to`, `continue_to`, …") name `.tangent()`.
- **`Chain`**: `continue_to` is gone from `crates/pncad/src/trace.rs`,
  from `pncad.pyi` and from `py/path.rs`. Callers write
  `.tangent().line_to(t)`, using the `tangent()` the first unit added.
- **The lift** (`lift.rs`): the tangent-joint straight close emits
  `Tangent, LineTo(target)` where it emitted `ContinueTo(target)`. The
  module doc's "is `continue_to` and not `.tangent().line(len)`"
  paragraph is re-worded.
- **The viewer**: `replay_provisionally_closed`
  (`crates/viewer/src/sketch.rs`) appends `Tangent, LineTo(start)` for
  the straight-on close, which is two steps where it was one. Check
  the step indices it reports refusals at.
  `crates/viewer/src/drafts.rs`'s and `sketch.rs`'s `ContinueTo`
  constructors are respelled.
- **The recourse texts match §4 item 1 as the merged page states it.**
  `JUNCTION_TURN_LEVER` in `crates/profile/src/fillet_recourse.rs` (PR
  7260's sentence, which still names bare `line(len)`) and every test
  asserting it (`crates/profile/tests/junction_turn_lever.rs` and
  others found by grepping the sentence) take §4 item 1's levers. At
  `7c8acfd225` those are: write `.tangent()` in place of the director;
  where a self-aiming leg computed the direction, `.tangent().line_to(p)`
  for a straight leg and `.tangent().arc_to(End { p })` for an arc.
  The sentence keeps 7260's levered-turn framing and its word budget.
- **Python**: `PathDirectedPoint.fillet`, `fillet_arc` and
  `arc_fillet`/`arc_fillet_arc` with `Radius` are gone. The
  `_LegEndIncoming` alias loses `Radius`. `surface_census.rs`'s
  `NotBound` row for `Verb::ContinueTo` (`would_be:
  PathDirectedPoint.continue_to`) goes with the verb.
- **The schema break**:
  - A document naming `ContinueTo` refuses Unreadable with the
    regenerate recourse.
  - A stored document with a bare leg-end `Line` refuses at load's
    lattice check as a Transition, and a row asserts each.
  - `tests/golden/golden.cad` and `tests/corpus/wire_rv_bytes.json` are
    re-blessed, and the `switch_program_vocabulary.rs` pins
    are updated.
- **No frame moves**, given the first unit's differential. A moved frame
  is a finding.
- Rows on other slates that spell the verb are re-worded:
  - PROFILE's `a-close-from-a-tip-on-its-start-is-refused-as-a-tangent-seam`
    (its `continue_to Start` column);
  - TRACE's `an-arc-runs-to-a-crossing-only-about-a-centre`;
  - DOORS' `a-last-leg-no-close-can-follow-is-dropped`;
  - `docs/GUIDE.md`.

## Check first

1. **§4 item 1 is merged.** PR 7251 must be merged, with its §4 item 1
   reconciled against PR 7260's levered wording. If it is not, stop:
   the recourse sentence has no ruled text to match.
2. **The bool13 historical goldens** still refuse naming `unit` (as in
   `tangent-arc-to-retires-into-arc-to-end`).
3. **The step-index shift.** Respelling a bare `line` as
   `.tangent().line` in a DOCUMENT inserts a step, which moves every
   later step's `SlotId::Profile { step, .. }` and the program's
   per-step `ids`. In tests this re-blesses `tests/golden/slot_tables.txt`
   and the naming goldens. If a stored id or pin outside the test
   corpus depends on a step's index, stop and report.

## Rows

- Carries the rest of the parent:
  `tangent-arc-to-needs-an-explicit-tangent-where-line-takes-the-directed-point`
  is parked on this unit and closes in its PR.
- Retires BIND's census row for `continue_to`.

## Ground crossed

Every program `tangent-arc-to-retires-into-arc-to-end` names, plus TRACE
(`trace.rs`, the trace lowering). Announce the schema break on the away
channel and the Python removals on PYDOOR's and BIND's logs before
landing.

## Review

Tier **single (FULL)**, and **dual** if the runtime-found sites turn out
to include stored documents outside the test corpus. The deletion is
mechanical. The risk is a site no compiler finds, and a lattice change
that every door reads. Difficulty **L**.
