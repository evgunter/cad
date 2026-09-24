# AUTH-4 — the viewer authors a Part, and duplicates a body

**Rows**: `work/author/viewer-cannot-author-a-part-node` (P0, D) and
`work/author/viewer-cannot-author-a-duplicate-node` (P0, D — **Ev's
own request**). Read both in full; the second carries Ev's words and a
ruling he gave on 2026-09-21.

**Branch** `author/part-and-duplicate`. **Never merge; I merge.**

## Ev's ruling, and what it settles

The duplicate row said the design choice — a new node, a flag on
transform, or a gesture over existing nodes — goes to Ev. It went to
him on 2026-09-21 with the reading below, and **he ruled: a `Pattern`
of count 2, no new document node.** So there is no EDIT half and
nothing here waits on the document vocabulary.

He also raised a larger idea — unify placement across normal
placement, transform and pattern behind one edited `placement` arg —
and then ruled that it is a bigger change belonging to another
program. It is filed as
`work/edit/placement-is-spelled-three-ways-node-registry-and-rule`
(P0, H, `needs_ev`). **AUTH-4 does not touch it and must not wait on
it.**

## Why these two rows are one unit

`AddPart` is what makes a duplicate usable. Both halves live in
`session/op.rs` and the create pane, so splitting them would put two
lanes in the same files.

## Half 1 — `AddPart`

`crates/viewer/src/session/op.rs` has `AddSplit`, `AddTransform`,
`AddPattern`, `AddPlacedUnion`, `AddInstance` and eight more — and no
`AddPart`, while `crates/viewer/src/combine.rs` admits `Node::Part` at
every body seat and names the viewer as the door that should author
one. I verified the absence rather than trusting the row.

The node is `Node::Part { of: RecipeNodeId, select: PartSelect }`
(`editor-core/src/node.rs:2159`), and
`PartSelect::{ SplitHalf(SplitHalf), Instance(Expr) }` (`:1041`).

**The design call the row flags as the whole of the work — decide and
say.** What does the op take? There is a precedent worth weighing:
`AddPattern`'s `count` is an **`i64`** at the op door and "lands as an
exact Count literal: it is the node's STRUCTURAL slot (spec D3),
edited afterwards through `SetStructuralParam` and never through the
continuous door". `PartSelect::Instance` is likewise a Count-typed
structural slot (`SlotId::Instance`). Mirroring `AddPattern` argues
for an `i64` instance index at the op door; carrying `PartSelect`
whole argues for one shape for both selections. Weigh them, pick one,
write the reasoning in the PR — and if the precedent does not mean
what I think it does, **say so rather than implementing it**.

**The seat.** `of` must name a node whose value is multi-body — a
`Split` or a `Pattern`. Decide where that is checked and say why: the
op door's `require_kind` family, or the value-shaped door AUTH-1
learned to prefer. AUTH-1's review found a gate testing a node KIND
where the evaluator tests a VALUE, and the form admitted something
the edit then refused. Do not repeat it.

## Half 2 — duplicate

A gesture that emits a `Pattern` of count 2 over the picked body, so
the result is the original placed whole plus one moved copy. Decide
and say: the default offset (direction and spacing), where the gesture
lives in the UI, and whether it is one button or a small form.

## The thing I most want verified, because I may be wrong about it

I read `roots::on_insert` (`editor-core/src/roots.rs:174-181`) as
removing a new node's inputs from `Doc::roots` and inserting the new
node in the earliest consumed root's slot, with `doc.rs:730` stating
that the root SET is exactly the DAG's sink set, and
`viewer/src/display.rs:480` (`drawn_targets`) drawing roots.

**If that is right, then a naive duplicate-then-move-one is broken**,
and the unit has to know it:

- `AddPattern` consumes the original from `roots`; the pattern node
  becomes the root with two bodies. Fine.
- `AddPart { of: pattern, select: Instance(1) }` then consumes the
  PATTERN from `roots`, so the drawn set becomes the one selected
  body — **and the other copy stops being drawn at all.**
- Two `Part`s both naming the pattern would give two roots (the
  second's input is no longer in `roots`, so it is pushed), which
  would make the gesture `Pattern` + two `Part`s.

**Verify this by running, not by reading, and report what you find.**
Build the document, look at `doc.roots()` after each insert, and see
what the viewport draws. If I am right, say what the duplicate
gesture has to emit for the copy to be independently movable while
both stay visible, and whether that is still within this unit or is a
second row. If I am wrong, that is the more useful answer — five of
my premises have been falsified on this program already and every one
was cheaper caught here than downstream.

## Traps this program has hit repeatedly

- **Closing a duplication mints one.** Three units running; three
  times a reviewer found a fresh second spelling the lane had just
  introduced. Check your own diff before pushing.
- **A unit-tested helper is not a wired one.** AUTH-3 shipped a label
  composition held by NO test — deleting it reddened nothing in 674 —
  in the unit whose brief warned about exactly that. `pane::headless`
  now exists (`viewer/src/pane.rs`) with `painted` / `painted_text` /
  `painted_after_clicking`; use it and drive the panel, not just the
  helper.
- **A mutation table is a test result, not a plan.** Never restore a
  mutation with `git checkout -- crates/` — AUTH-3 lost uncommitted
  work that way and reported it as present. Restore from a byte copy.
- **Assert what you claim.** For every behaviour stated in the PR,
  have a row that reds if it stops being true, and name the mutation
  that proved the row can fail.

## Scope

In: `session/op.rs`, `session.rs`, `session/author.rs`, `combine.rs`,
`pane/create.rs`, `drafts.rs`, `forms.rs`, `tree.rs`. All are double
claims (`work.py territory` will say so); run it on your branch and
put the output in the PR. CHROME has three lanes live on viewer
ground and one row held out of its wave waiting on `session.rs` —
`work/chrome/at-rest-badge-reports-an-empty-document-as-a-refusal` —
so keep your `session.rs` edits to what the ops need.

Out: `crates/editor-core/` entirely — no document-vocabulary change is
needed and the placement row is another program's. The `AddBoolean`
declaration gap, the label rows, and `bounds.rs`. A finding outside
the fence gets a row under implementer-discipline §6.

## Verification

Hosted CI is the verification of record. §2 no longer states a job
count — it names the matrix, so confirm a code-tier run gates every
point of {default features, `interval`} x {default eps, 1e-6, 1e-12}
and every `k-lint (gate, <row>)` unification. **Count job names by
SUBSTRING, not prefix**: the interval lane now arrives as
`interval / test (interval, eps = …)`, and a prefix match reads six
rows on a fully green run. Confirm the run's head SHA equals your
branch head before reading anything.

Locally: `cargo fmt --all --check`, `cargo clippy --workspace
--all-targets -- -D warnings` and again `--features viewer/app`,
`scripts/doc-gate.sh`, the viewer suites, `python3 scripts/work.py
lint`, `scripts/gates/viewer-module-kinds.sh`.

`CARGO_TARGET_DIR` outside the worktree, **exported on every
invocation** including the excluded cargo roots (`demos/*`, `benches`,
`tools/*` are separate roots; two lanes have now filled this box's
disk by letting the export miss a subshell). Scratch in your own lane
directory, never the session scratchpad.

## Deliverable

A PR titled `AUTH-4: the viewer authors a Part, and duplicates a
body`, body carrying both design calls with their reasoning, the
roots/drawn-set finding, the territory output, the mutations proving
your rows can fail, and any §6 rows. Report to me; do not merge.
