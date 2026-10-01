# AUTH-3 — the add-profile form mints its own frame, and says which frame it means

**Row**: `work/author/add-profile-mints-no-frame` (P0, cost D). Read it
in full first; it carries two corrections to its own earlier text and
you should know which sentences are already retracted.

**Branch** `author/profile-frame`. **Never merge; I merge.**

## Why this row, and why both halves in one unit

Placing a sketch is the gesture AUTH-1 just made possible from a
picked face. From an EMPTY document it is still a dead end: the form
says *"on frame — none in this document — add a frame datum first"*,
which is true and useless, because the commonest first act in a new
document is a sketch on world XY.

The two halves both live in `add_profile_ui`'s ComboBox — half 1 adds
a choice to it, half 2 changes what its entries say. **Splitting them
would put two lanes in the same widget**, so they go together. That is
the whole reason; it is not a claim that they are one problem.

## Half 1 — "on a new XY frame"

`crates/editor-core/REFERENCES.md`, DM1's chrome-consequence bullet,
already rules the shape and names this row by id:

> "on a new XY frame" is two inserts in one committed action
> (`commit_action`); "on this face" mints one `FaceFrame` and one
> profile the same way.

I verified the door exists rather than trusting that: `commit_action`
is at `crates/viewer/src/session.rs:2457` — private, taking
`Vec<DocEdit<ProfileProgram>>`, all-or-nothing, the whole run recorded
as ONE history state — and `delete_node` already uses it for its
cascade at `:2440`. So the invariant to keep is **one submit, one
UNDO**, and the door for it is shipped.

**The design call is yours: DECIDE and SAY, do not assume I have.**
How does the form express "a new XY frame"? A distinct
`SessionOp`? An enum on `AddProfile`'s plane field
(`Existing(RecipeNodeId)` / `NewXy`)? Something else? Weigh it against
how `SessionOp` variants are shaped today and against the op-door
checks each would need, write the reasoning in the PR body, and if the
tree makes my framing wrong, **say so instead of implementing it** —
four of my premises have been falsified on this program already and
that was the cheap outcome every time.

Whatever you choose: the frame a person draws on must be a frame they
can see and edit afterwards, which is why `AddProfile` names an
existing node in the first place. A hidden or implicit frame fails the
row.

## Half 2 — a frame's label says which frame it is

Three sites. **One of them has moved since the row was written** and
the row does not know it:

1. `crates/viewer/src/pane/create.rs:55` and `:60` —
   `format!("feature {}", id.0)`, in the closed combo text AND in the
   options, for every frame of either kind.
2. `crates/viewer/src/tree.rs:215-216` — `"Datum frame"` /
   `"Datum frame (on face)"`. Distinguishes the KINDS; never says
   WHICH frame.
3. The face pick in `add_datum_ui` — the row says it is a hand-rolled
   `format!("feature {} body {}", …)`. **It is not any more.** AUTH-1's
   fix pass routed it through `BlendTarget::of_face(face).to_string()`
   (`create.rs:495`), and `impl Display for BlendTarget`
   (`blend.rs:146-150`) writes `"feature {} body {body}"`. So the node
   number is still what a person reads, but it now has ONE home. Fix
   that home and this site and the blend sites move together.

**The hard part, and the second thing to decide and say.** "XY at
z = 0" or "on feature 5's top" is a statement about a frame's POSE,
and pose is not uniformly available: a `Datum::Frame`'s origin/u/v are
`Expr`s on the node, but they can be parameter-driven, and a
`Datum::FaceFrame`'s pose is only known after evaluation. So a label
that always tells the truth either reads the landed evaluation (and
must say something honest when there is none — the node is new, or
undone, or refused) or restricts itself to what the node alone can
say. **Pick one, say which, and say what the label reads when the
answer is not available.** A label that quietly prints a stale or
guessed pose is worse than `feature 3`, because `feature 3` at least
does not claim to be geometry.

## Traps this program has hit twice each — do not hit them a third time

- **Closing a duplication mints one.** AUTH-1 and AUTH-2 BOTH did it,
  and both had it caught in review rather than at the keyboard. You
  are about to give labels a home; check before you push whether you
  have left a second spelling behind, and whether the home you chose
  is the one the tree already has (`Display for BlendTarget` is a
  shape to copy, and possibly to extend rather than to sit beside).
  Naming this in a PR body has never once prevented it.
- **A unit-tested helper is not a wired one.** AUTH-2's guard was
  correct as a pure function and dead code at the panel, because a
  second emitter ran above it. Nothing in the suite covered the
  panel's op emission. If you add a labelling function, prove the
  PANEL calls it — drive the widget, not just the helper.
- **Assert what the tree says, by running it.** Where you state a
  behaviour in the PR, have a test that goes red if it stops being
  true, and say which mutation you used to check the test can fail.

## Scope

In: `pane/create.rs`, `tree.rs`, `blend.rs`, `session.rs`,
`session/op.rs`, `session/author.rs`, `drafts.rs`. All are double
claims with CHROME/VIEW/VSEAM/VNEWS (`work.py territory` says so and
that is fine) — what is owed is awareness, so run it on your branch
and say in the PR what it printed.

Out: the add-datum form's own kinds; the tree's non-frame rows; the
`AddBoolean` declaration gap (a separate open row); anything in
`bounds.rs`. A finding outside the fence gets a row under
implementer-discipline §6, not a fix.

## Verification

Hosted CI is the verification of record. Expect **twelve `test (…)`**
and **five `k-lint (gate, …)`** jobs; confirm the run's head SHA
equals your branch head before reading counts, and read the STEP for
rows whose job name does not name them. Locally: `cargo fmt --all
--check`, `cargo clippy --workspace --all-targets -- -D warnings` and
again `--features viewer/app`, `scripts/doc-gate.sh`, the viewer
suites, `python3 scripts/work.py lint`.

`CARGO_TARGET_DIR` outside the worktree, exported on EVERY invocation
including the excluded cargo roots (`demos/*`, `benches`, `tools/*`
are separate roots and a lane has already filled the disk with in-tree
`target/` dirs this way). Scratch in your own lane directory, never
the session scratchpad — it is shared between worktrees and has
already caused a lane to misread CI.

## Deliverable

A PR titled `AUTH-3: the add-profile form mints its own frame`, body
carrying both design calls and their reasoning, the territory output,
the mutations you used to prove your tests can fail, and any §6 rows.
Report to me; do not merge.
