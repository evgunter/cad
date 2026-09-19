---
id: recorded-notation-makes-a-rust-author-count-step-indices
kind: issue
title: a RecordedNotation entry is keyed by an index the path algebra never hands its caller
status: closed
closed: 2026-09-19
pr: 2876
branch: edit/notation-derived-index
opened: 2026-09-16
---


Found by the style review of PR #2779 (lane `notation-rv`), reviewing
`recorded-program-arguments-carry-no-notation` at `1c7b7340d`. Outside
that unit's fence — it is a door this unit could not have built without
touching `profile` — so it is filed rather than fixed.

## What happens

`RecordedNotation::set` takes an authored step index
(`crates/editor-core/src/program.rs:2020`), and the recording side —
`profile`'s `PartialPath`, whose `record` calls are inside the
transition table (`crates/profile/src/path/program.rs:945`) — hands
the author no index at all. A caller writing

```rust
let path = Open.at(p0).line_to(p1, t)?.line_to(Start, t)?;
```

must count `At` = 0, `LineTo` = 1 by reading the table's `record` calls
to write `n.set(1, StepArg::TargetX, MM.def())`. `crates/editor-core/tests/edit_recorded_notation.rs:45`
hard-codes that count as a `const LEG: u32 = 1;` with a comment, which
is the suite doing by hand what a caller would have to.

The refusal that catches a miscount —
`RecordedProgramError::NotationOffProgram` — only fires when the role
does not exist at the index named. A miscount that lands on a step
carrying the SAME role (the `TargetX` of leg 2 instead of leg 1) is
accepted and puts the unit on the wrong argument, silently. That is the
sharp edge, not the counting.

## Why it is not LIB's existing finding

`work/lib/path-legs-erase-the-authored-notation-one-layer-down` is
about VALUES — "Rust's own path API is `f64` throughout" — and its
`B-PATH-NOTATION` charter
(`crates/pncad-py/tests/test_binding_census.py:1329`) scopes the work to
the Python builder recording an entry per typed quantity it lowers. A
Python path builder threading the entries itself never exposes the
index, so closing that row leaves the RUST author with the count. The
gap is this side of the seam and has no row.

## What closing it would decide

Whether the recorder hands back what it recorded — a
`record_with_notation`-shaped door, or a `RecordedNotation` builder
that takes `(unit, StepArg)` and derives the index from the step it was
called after — and where it can live, given D6 ¶1 keeps `quantity` and
`UnitSym` out of `profile`. A wrapper in `editor-core` over
`PartialPath`'s program is the shape that stays inside the layering.

## Ruled and spec'd (2026-09-19, EDIT orchestrator) — middle tier, branch `edit/notation-derived-index`

**Ruling: the recorder hands back what it recorded, and the notation's
authoring door derives the index from it.** Two doors, one on each
side of the layering seam, neither moving a unit into `profile`:

- `profile`: `PartialPath::recorded(&self) -> &[Step<T>]` — the steps
  recorded so far, in program order, the prefix of the `program` a
  finished path publishes. Pure data the path already holds; no
  `quantity`, no `UnitSym` (D6 ¶1 stands). PATHS's crate, crossed by
  announcement for one accessor.
- `editor-core`: `RecordedNotation::set_after(&mut self, recorded:
  &[Step<f64>], arg: StepArg, unit: UnitDef) -> Result<…>` — writes
  the notation for the LAST recorded step's `arg`, the index being
  `recorded.len() − 1` and never the author's count. A caller writes
  `let path = path.line_to(p1, t)?; n.set_after(path.recorded(),
  StepArg::TargetX, MM.def())?;` at the leg itself. An empty
  recording refuses typed (there is no last step): the honest error
  type is the lane's to argue — a new `RecordedProgramError` arm
  (`NotationBeforeAnyStep { arg }`, which adds one tag word to
  `pncad-py`'s exhaustive map, LIB's by announcement) or the existing
  `NotationOffProgram { step: 0, arg }` if its sentence is true of the
  case; say which and why. `set` stays as the ADDRESSED door — the
  viewer's `Notation::over` derives its indices from
  `LoopProgram::step_args()` and is the other honest caller of it — and
  its doc now says that a hand-written index is the caller's second
  description of the recording, with `set_after` named as the door
  that cannot miscount.

**What closes and what does not.** The sharp edge — a miscount landing
on a step that carries the SAME role, accepted silently — is closed
for every author who writes through `set_after`, because there is no
count. A wrong ROLE on the right step stays the lift's
`NotationOffProgram` refusal, as today, now with the index certainly
the author's leg. `set` with a hand index is still expressible (the
viewer needs the addressed door), so the trap is not made
unrepresentable; the row's doc says so and why.

**Premises to verify before building.** (1) `PartialPath`'s `core`
holds the recording as a `Vec<Step<T>>` grown by `Core::record`
(`crates/profile/src/path.rs` ~2311) and every verb records exactly
one step per call (a fused verb records one step — confirm on
`arc_fillet_arc`); binders (`fillet`) record a step too, so "last
recorded" is the binder after a `fillet(r)` and the radius is that
step's `StepArg::Radius`. (2) `LoopProgram::from_recorded` numbers
steps by recording index (the suite's `LEG = 1` after `At`), so
`recorded.len() − 1` IS the lift's `step`. (3)
`edit_recorded_notation.rs`'s `const LEG` is the hand count the row
complains of; its rows re-author through `set_after` where they author
a leg, keeping ONE row that writes a hand index (the lift's
off-program refusal) and saying why it keeps it. (4) `pncad-py` binds
`RecordedNotation`? (grep says no — `pncad`'s prelude and `document.rs`
re-export it, the viewer's sketch builds one); if a Python door
exists, LIB's row `work/lib/path-legs-erase-the-authored-notation-one-layer-down`
is where it would use this, not this unit.

**Rows** (each red on `origin/main` first, then green): a three-leg
chain with the notation written by `set_after` after leg two lifts
with the unit on step two's target and nowhere else (`get(2, TargetX)`
answers, `get(1, …)` does not); the sharp edge pinned in the
direction that tells — the same chain with `set(1, …)` written by a
hand count that is off by one is ACCEPTED by the lift and lands on the
wrong leg (the trap, stated as what `set_after` closes); `set_after`
on an empty recording refuses typed; `set_after` naming a role the
last step does not carry refuses at the lift with `step ==
recorded.len() − 1`; `PartialPath::recorded` equals the finished
program's prefix after every verb of a mixed chain (line, arc, fillet,
close) — a `profile` unit row; the viewer's `Notation::over` and the
existing notation rows green unchanged; the `Display` sentence of any
new arm (`display_contract`).

**Mutants** (each named with the row that reds it): `set_after`
indexing `recorded.len()` (the three-leg row: off program); `recorded`
omitting binder steps (the prefix row on a chain with a `fillet`);
the empty-recording arm returning `Ok` (its row).

**Sweep.** `RecordedNotation`, `NotationOffProgram`, `count`/`counts
step` prose in `program.rs`'s notation docs and the suite header;
`crates/pncad-py/src/tags.rs`'s `RecordedProgramError` map (a new arm
is a compile break there and a new inventory word).

**Territory.** `crates/editor-core/src/program.rs` (EDIT);
`crates/profile/src/path.rs` (one accessor — PATHS's, by
announcement); `crates/editor-core/tests/*` and one `profile` unit row
(TCOST/TINT); `crates/pncad-py/src/tags.rs` only if an arm is added
(LIB, by announcement). Middle tier: one opus style review with a
correctness arm, then the fix pass.


## Built (2026-09-19, lane `notation`)

Both doors landed as ruled, and every premise the spec asked to be
verified held.

- `profile`: `PartialPath::recorded(&self) -> &[Step<T>]` — the steps
  recorded so far, the prefix of the program the chain publishes. One
  accessor over data the path already holds; no `quantity`, no
  `UnitSym`. PATHS's crate, crossed by announcement.
- `editor-core`: `RecordedNotation::set_after(&mut self, recorded,
  arg, unit) -> Result<(), RecordedProgramError>` — writes the last
  recorded step's `arg`, index `recorded.len() - 1`, never a count.
  `set` stays the ADDRESSED door and its doc now says a hand-written
  index is the caller's second description of the recording, naming
  `set_after` as the door that cannot miscount.
- The empty recording refuses as a NEW arm,
  `RecordedProgramError::NotationBeforeAnyStep { arg }`, argued rather
  than reusing `NotationOffProgram { step: 0, arg }`: that sentence
  attributes an index 0 the author never wrote, and step 0 of a
  recording whose entry verb lacks the role is a live, different
  mistake the suite already pins. One tag word
  (`notation_before_any_step`) in `crates/pncad-py/src/tags.rs` and its
  inventory in `src/tests.rs` — LIB's, by announcement, mechanical.
- Rows: the three-leg chain written through `set_after` after leg two
  (`get(2, TargetX)` answers, `get(1, …)` does not); the trap pinned as
  ACCEPTANCE — a hand `set(1, …)` off by one lands on the wrong leg and
  nobody is told; the empty recording refusing typed with its `Display`
  sentence; a role the last step does not carry still refusing at the
  lift with `step == recorded.len() - 1`; `PartialPath::recorded`
  equalling the published program's prefix after every verb of a mixed
  chain (arc, `fillet` binder, re-entry, legs, closer) in
  `crates/profile/tests/path_program.rs`; and a `display_contract`
  census over the whole of `RecordedProgramError`, which had none.
  Every existing row that authors a leg now authors it through
  `set_after`; `const LEG` survives as the suite's READ address, which
  is what makes the derived write checkable.
- Premises: every verb records exactly one step, fused verbs and
  binders included (51 `Core::record` call sites, one step each — of
  the 54 `.record(` calls in `crates/profile/src/`, three belong to
  other `record` fns: `structure.rs`'s fillet-decision recorder and
  the two `guide.record` sites in `validate.rs` and
  `path/arc_fillet.rs`);
  `from_recorded` numbers steps by recording index; no Python door
  binds `RecordedNotation` (`pncad`'s prelude and `document.rs`
  re-export it, the viewer builds one), so LIB's
  `path-legs-erase-the-authored-notation-one-layer-down` is still where
  a Python builder would use this.

Green on hosted CI run 35457612872 (head `33d36280d`): 35 jobs success,
4 the change filter's own skips, twelve `test (…)`, five
`k-lint (gate, …)`, the python suite, `gate ok`.

Not closed, and said in the doc: `set` with a hand index stays
expressible, because the viewer's `Notation::over` derives its indices
from `LoopProgram::step_args()` and needs the addressed door. A wrong
ROLE on the right step stays the lift's `NotationOffProgram` refusal.


## Built — fix pass (2026-09-19, lane `notation-fix`)

The review's findings, each as the invariant it asked for.

- **One home for narrowing a program address.**
  `program.rs`'s private `program_index(usize) -> u32` holds the
  `u32::try_from` and the D2-row-4 justification once, and the ten
  bare `as u32` narrowings in that file plus the derived door call
  it. The eight-line paragraph at the door is gone; its argument is
  the helper's doc.
- **The derived door is reachable wherever an author has just
  recorded.** `recorded()` now answers on all five arrival builders
  (`RadiusArrival`, `RadiusArrivalAt`, `RadiusArrivalDir`,
  `ViaArrival`, `ViaArrivalStart`) as well as on `PartialPath` — one
  accessor shape, the reason stated once on the arrival-builder
  banner. PATHS's crate, by announcement. After the closer the
  recording is `ClosedLoop::program`, and `set_after`'s doc names the
  two moments and the two spellings.
- **The dead vocabulary is said to be dead.** Neither notation tag
  can reach a Python caller (`py::path::loop_program` lifts through
  `from_recorded`), and `recorded_program_error_tag`'s doc now says
  so and points at
  `work/lib/path-legs-erase-the-authored-notation-one-layer-down`,
  which gained a `## Widened` section naming both doors.
- **The record.** 51 `Core::record` sites, not 54; the
  `display_contract` census doc counts five arms, three about the
  recording, and names the one raised at the writing door;
  `program.rs`'s enum and `tags.rs`'s map say that in their first
  sentence; three rows keep `set` and the suite header names each
  with its reason; the profile row got its own doc block and
  `circle_is_a_one_step_program_that_replays_to_its_two_poles` its
  paragraph back.
- **One predicate.** `RecordedNotation`'s "A unit measures what its
  role holds" names both doors and says `set_after` delegates to
  `set`, so `UnitSym::checked_for` is asked in one place.
- **The review's three probes** are the unit's rows now, each headed
  by the invariant it pins: the derived index over fused verbs,
  binders and the closer; the `fillet` binder as the last recorded
  step; and — turned from an asymmetry into the door's own row — an
  arrival state's author reaching `recorded()`, plus the finished
  loop's `program` as the second spelling.
- **The suite's helpers, checked against the fixture's doors:**
  `vertex_bits` was `fixture::run`'s body spelled again and now
  imports it; `square_authored`, `arg_bits` and `read_back` match no
  door and stay.

Green on hosted CI run 35462469877 (head `03d168452`): 39 jobs, 35
success and 4 the change filter's own skips; twelve `test (…)`, five
`k-lint (gate, …)`, the python suite, `gate ok`. Read at the step
level: 422 success, 102 skipped, nothing else.

## Closed (2026-09-19, EDIT orchestrator)

Built and merged as PR #2876 (middle tier: one opus style review with
a correctness arm, then the union fix pass). `PartialPath::recorded`
(and the same accessor on all five arrival builders — every state an
author has just recorded from) hands back the steps so far, and
`RecordedNotation::set_after(recorded, arg, unit)` writes the LAST
recorded step's argument with the index derived, never counted; an
empty recording refuses through a new arm, `NotationBeforeAnyStep`,
raised at the writing door (the reuse of `NotationOffProgram { step:
0 }` would attribute an index the author never wrote — the one row
that tells the arms apart pins it). `set` stays as the addressed door
the viewer's `Notation::over` derives its indices for, and the
miscount trap is pinned as acceptance through it, with the reason it
is kept. The review (0 MAJOR, 4 MINOR, all prose or counts) found the
door's index conversion a third spelling beside nine silent casts in
the same file, the derived door absent from the arrival states where
a hand count is hardest, and the new tag word unreachable from Python
with nothing saying so; the fix pass gave the conversion one home
(`program_index` — ten sites, four of them loop indices, so the name
the brief proposed would have been false at half of them: the lane's
spelling stands), put the accessor on every builder state, said the
dead vocabulary on the tag map's doc per its convention, corrected
the counts (51 `Core::record` sites, five census arms, three rows
keeping `set`), and appended the hand-off to LIB's
`path-legs-erase-the-authored-notation-one-layer-down`. Territory
crossed by announcement: `crates/profile/src/path.rs` (PATHS — the
accessors), `crates/pncad-py/src/{tags.rs, tests.rs}` (LIB — one tag
word), three suites (TCOST/TINT), one LIB row appended.
