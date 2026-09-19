---
id: recorded-notation-makes-a-rust-author-count-step-indices
kind: issue
title: a RecordedNotation entry is keyed by an index the path algebra never hands its caller
status: review
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
  binders included (54 `Core::record` call sites, one step each);
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
