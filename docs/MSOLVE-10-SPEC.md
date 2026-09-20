# MSOLVE-10 — A mate the coset table refuses on its own is refused at the edit door (spec)

Unit of the `msolve` program. Item `work/msolve/MSOLVE-10.md`. Answers
half (1) of `work/msolve/mate-clocking-has-no-gui-path.md` — read it
in full, including its three re-homing sections — and records half
(2)'s disposition (§Out of scope). **Track:** a kernel change to WHEN
a refusal the solve already makes is met — at the door instead of the
next evaluation — with no change to what the solve decides. One style
review plus a correctness arm (§Review). No A/B row. Sequenced after
MSOLVE-8 (the lever and the frame witness it reads are that unit's).

## What the tree says now

1. **The table's per-mate refusals are met one evaluation late.**
   `mate_coset` (`mate/solve.rs`) is where a mate's OWN datum meets
   the coset table: a clocking rider on a planar rest and a standalone
   `Clocking` primitive refuse `MateFault::TableLacks` with no number
   read; a rider on a frame coincidence is DECIDED — the roll is
   levered by the mate's arm and `mate_clocking_redundant` answers
   zero (redundant, admitted) or not (contradictory, refused,
   `Clash::Levered(Lever::Roll)`); a frame whose axis or reference
   has no definite direction refuses `MateFault::Frame`. Every one of
   these is a fact about the mate alone. The edit door
   (`edit.rs`, `InsertNode` of a `Node::Mate`) asks only
   `Node::has_non_finite_alignment` (`EditError::NonFiniteAlignment`)
   and the slot checks, so a mate the table refuses enters the
   history, the commit succeeds, and the user meets the refusal as a
   tree badge one step later. The item's measurement: `proposal(…,
   MateChoice { FrameCoincidence, clocking: Some(π/2) })` is `Ok`,
   `perform` commits, the next evaluation fails.

2. **The door already holds what the decision needs.** Since MSOLVE-6
   `apply(doc, edit, tol, &dyn MateReach)` takes the reach, and the
   maintenance asks it lazily when a gauge moves; `fold_pair` asks the
   pair's reaches lazily after the class and self-mate checks and
   forms the mate's arm as `parts + alignment.lever_arm()`. The
   per-mate prefix of `fold_pair` — class admission, the two walks,
   the self-mate check, the pair reach, `mate_coset` — is the
   admission the door should ask, and it is one function away from
   being callable on one mate.

3. **The viewer's tool has the precedent.** `matetool::proposal`
   refuses `ClassRefused` before any geometry is read
   (`class_admission` — "the class door FIRST"), so a static table
   fact refusing at the tool is the shape already there; the rider
   on a coincidence is not static (it is decided over the arm), so
   the tool cannot honestly refuse it without the reach.

## What the unit builds

**1. One per-mate admission, the solve's own.** `mate/solve.rs` gains
a crate-private `admit_mate(doc, mate: RecipeNodeId, reach: &dyn
MateReach, tol) -> Result<(), Box<MateFault>>` (name yours) that IS
the per-mate prefix of `fold_pair`, refactored so `fold_pair` calls it
rather than restating it: the class door, the two walks (`walk_of`),
`check_reference` on both, the self-mate check, the pair reach asked
lazily — only when `mate_coset` will lever a decision, i.e. a rider
on a coincidence — and `mate_coset` itself. It returns the fault the
solve would record against THAT mate for its own datum; it folds
nothing and reads no other mate. A refusal that needs the reach and
has none (`RefusingReach`) is `Unleverable`, exactly as the solve
answers it.

**2. The door asks it.** `apply`'s `InsertNode` arm for a `Node::Mate`
— and any other arm through which a mate's alignment can enter or
change (measure: `ReplaceNode`-shaped edits, `SetMembers` does not
touch a mate) — runs `admit_mate` on the document AFTER the edit,
after the non-finite check and before the maintenance, and refuses
`EditError::MateRefused { node, fault: Box<MateFault> }` carrying the
solve's own fault unaltered (the `MaintenanceRefused` precedent). The
`Display` forwards the fault's sentence after naming the node. What
refuses at the door is exactly what `mate_coset` refuses about the
mate alone: `TableLacks`, the decided contradictory rider, `Frame`,
plus the walk's own `DanglingHead` / `PartSelectsAnotherCopy` /
`SelfMate` / `ClassNotAdmitted` / `Unleverable` where those are not
already refused earlier in the door (measure which are, and say).
Relational verdicts — UNDER, a contradiction against ANOTHER mate,
`Indeterminate` on a fold — stay the solve's: the door decides the
mate, not the cluster. A rider the band decides redundant is admitted
as today (no verdict moves).

**3. Replay and old logs.** `apply_logged` runs the same door, so a log
that recorded a mate the table has always refused now refuses at
`persist::load` with `PersistError::EditReplay { index, MateRefused }`
naming the entry — the document was already broken at every
evaluation, and the load says so where it can be acted on. No
migration door: the recourse is the one the fault names (delete or
re-author the mate). Measure the four tracked `.pncad` (none carries
a mate; C5 holds) and every fixture and demo document that inserts a
mate through the refusing reach: a coincidence WITH a rider through
`RefusingReach` now refuses `Unleverable` at the door, so a test that
authored one must hand the store (the `step_with` road), and the PR
lists each such row. The tour's `assembly` demo: if its `refusals`
walk authors a table-refused mate to show the evaluation's refusal,
it now shows the door's — the demo is evidence and moves with the
kernel (discipline §3); say what it shows now.

**4. The tool refuses the static shapes first.** `matetool::proposal`
refuses `MateToolError::TableRefused { what: &'static str }` for a
rider on a planar rest and for the standalone `Clocking` primitive,
before any geometry is read (the class door's shape and position);
the decided rider on a coincidence is the DOOR's to refuse, and the
tool's `perform` surfaces `EditError::MateRefused`'s sentence as it
does every door refusal — no code, one row. The story suite's stage
(`viewer/tests/story_assembly.rs`) moves from "the next evaluation
fails with `mate_clocking_redundant`" to "`perform` refuses typed";
its recovery (turning the roll reference and committing through
`AddMate`) is unchanged and stays the record of half (2)'s cost.

**5. Consumers and docs.** `pncad-py`: `EditError.variant ==
"mate_refused"`, the inner `MateFault` projected through the existing
`edit_inner_variant_tag` → `mate_fault_tag` delegation and the
payload, the `.pyi`, the census; one Python row (`Doc.insert` of a
coincidence with a π/2 rider through `resolver=self.ws` refuses with
`inner_variant == "contradictory"` and the lever's roll). Docs:
`Doc::apply`'s admission list gains the sentence; `crates/editor-core/
README.md`'s edit-vocabulary line likewise; `ASSEMBLY.md` A11 rule 1
gains one clause — "the edit door asks the same per-mate admission,
so a mate the table refuses on its own never enters the document" —
argued in the PR body as an elaboration of the ratified policy (the
table is unchanged; where it is asked is what moves); if a reviewer
reads it as a design change it is split out and lands with Ev's
word. `docs/guide/assembly.md`'s fail-loud sentence on mates says the
door refuses.

**6. The rows.** `crates/editor-core/tests/msolve10_door_admission.rs`
(registered in `tests/all.rs`), through ordinary doors with a
`PartStore`: a coincidence with a rider beyond the band refuses at
insert `MateRefused { fault: Contradictory { predicate:
"mate_clocking_redundant", clash: Levered(Roll { radians, arm }) } }`
with `arm` equal to the solve's lever for that pair (re-derived); the
same rider inside the band is admitted and the solve places the pair
(no verdict moves — compare against the solve on main's road); a rider
on a planar rest and a standalone `Clocking` refuse `TableLacks` at
insert with no reach asked (a counting reach: zero asks); a degenerate
frame refuses `Frame` at insert; a coincidence with a rider through
`RefusingReach` refuses `Unleverable`; a coincidence without a rider
through `RefusingReach` is admitted (no ask); the history holds no
entry for a refused insert; `save` → `load` of a log whose entry was
admitted round-trips, and a hand-edited log entry carrying a refused
mate refuses at load naming the index; `admit_mate` and `fold_pair`
agree on every mate of the mate suites' fixtures (one row over the
corpus: for every document, every mate the solve faults for its own
datum is refused by `admit_mate` with the same fault, and every mate
the solve admits is admitted). One viewer row: the story stage above.
One Python row (§5).

## Acceptance

- **A1** The item's `proposal(… FrameCoincidence, clocking: Some(π/2))`
  → `perform` refuses typed at the door; no entry enters the history;
  the next evaluation has nothing to fail.
- **A2** `admit_mate` and the solve agree on every mate of the corpus
  (the row above); `fold_pair` calls `admit_mate` rather than
  restating it.
- **A3** No verdict moves: every mate row in the workspace passes
  unchanged but the rows that authored a table-refused mate to meet
  the refusal at evaluation, each named in the PR with where it meets
  it now.
- **A4** The static shapes refuse at the tool before geometry; the
  decided one at the door; the story stage moves as stated.
- **A5** Python, `.pyi`, census, the docs; the item closed with both
  halves recorded; `work.py lint` clean.

## Constraints, binding

- `docs/prompts/implementer-discipline.md` in full, by path. Hosted CI
  is the verification of record; poll it in the foreground; never end
  a turn with background work active. Four Cargo workspaces; the new
  `EditError` arm is public and is checked in all of them before the
  push; `pncad-py`'s clippy with `--features python --all-targets`.
- Merge-only; push early and often; the PR through the GitHub MCP
  tools. Private `CARGO_TARGET_DIR` outside the worktree; `git status`
  before every `git add`; never `git add -A`.
- Fence: `crates/editor-core/src/edit.rs` (the admission call and the
  arm), `mate/solve.rs` (`admit_mate` and `fold_pair`'s call), `mate.rs`
  (docs only), `persist/mod.rs` (docs only, if a sentence names what
  replay refuses), `crates/viewer/src/matetool.rs` (the static door)
  and its tests, `crates/pncad-py` (tag, payload, `.pyi`, tests),
  `demos/tour` (the assembly demo's evidence, if it moves), `docs/
  guide/assembly.md`, `ASSEMBLY.md` (the one clause), `README.md`,
  tests, the item. Nothing in the coset table, the walk, the
  maintenance or the evaluator.
- Comments state the invariant (discipline §4).
- **Stop clause.** If `admit_mate` cannot be the per-mate prefix of
  `fold_pair` without changing what the solve decides for any mate
  (the corpus row disagrees); if the door would need to fold a
  cluster to decide a mate alone; or if a tracked `.pncad` or a demo
  document refuses at load — STOP, write what you measured in the PR
  as a draft, and end your turn.

## Out of scope

Half (2) of the item — turning a mate's roll: the spelling that
exists is a coaxial mate with a clocking rider (the table's
coaxial+clocking row), which the tool's `MateChoice` already offers;
a roll-reference convention for authored frames is MSOLVE-9's
`reference` rule; a rotate-mate affordance is CHROME's viewer seam,
handed there when MSOLVE-9's convention is ratified. The item's
`## Closed` section records this disposition beside half (1)'s rows.
Refusing relational verdicts at the door (a mate that contradicts
ANOTHER mate needs the fold, which is the solve's).

## Review

One style review plus a correctness arm, claims verbatim:

- **C1** A1 through the viewer door and through `apply` directly: the
  refusal's fault is bit-identical to what the solve records for the
  same mate on main's road (same predicate, same clash, same lever).
- **C2** A2 over the corpus: `admit_mate` agrees with the solve on
  every mate's own-datum fault; `fold_pair` restates nothing.
- **C3** A3: enumerate the mate suites; every moved row is one that
  authored a table-refused mate, and it now meets the refusal at the
  door.
- **C4** The reach is asked at the door exactly when a rider on a
  coincidence is decided (a counting reach), and never for a mate the
  table refuses statically or admits without a lever.
