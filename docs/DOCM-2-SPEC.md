# DOCM-2 — `Node::Part`: a split's half or a pattern's instance as ONE body (spec)

**Program:** DOCM (`work/docm/plan.md`), unit `DOCM-2`
(`work/docm/DOCM-2.md`). **Ratified design:**
`docs/DOCM-REFERENCES-DESIGN.md` DM3 — read it first; this spec binds
the build and does not re-open it. The finding it answers is
`work/docm/split-side-and-pattern-instance-as-operand.md` (closed,
pointing here); read it for the seat's current words.
**Track:** kernel change — the standard v6 unit (binding spec, drawn
implementer arm, cross-model dual review, union fix pass,
record-at-merge; §Review below).
**Pre-draw fields, logged before the draw:** difficulty **M**, task-class
**STRUCTURAL**.

- **M** — one new node kind reaches every exhaustive `Node` match (the
  `Node::Union` sweep's shape: `node.rs`, `eval/`, `refactor.rs`, the
  corpus census, the viewer's `tree.rs` and `combine.rs`; the Python
  mirror is LIB's and is filed, not built), one new structural slot,
  one table projection, no new edit, no new naming segment, no
  corpus rewrite beyond one added document.
- **STRUCTURAL** — the node moves data: it selects one body out of a
  value and hands its rows through. No numeric decision anywhere; a
  number on the path is a finding.

## What the unit builds

**1. `Node::Part { of: RecipeNodeId, select: PartSelect }`** (`node.rs`,
beside `Split` and `Pattern`) with

```rust
pub enum PartSelect {
    /// The named half of a `Split` value.
    SplitHalf(SplitHalf),
    /// The `i`-th instance of a pattern's `Instances` value — a
    /// Count-typed STRUCTURAL slot (`SlotId::Instance`).
    Instance(Expr),
}
```

`of` is the one input (`Node::inputs`). `Instance`'s expression is a
new slot **`SlotId::Instance`**, `Dimension::Count`, structural — so it
is edited by `SetStructuralParam` like `Pattern`'s count, refused by
`SetParam` through the existing `StructuralSlotNeedsStructuralEdit`,
and enters the content key through the resolved-slot stream as every
slot does. It is its own slot, not a reuse of `SlotId::Count`: a panel
that spelled an index "count" would be lying about it. `SplitHalf`
is recipe payload outside the slots, so the structural-payload half of
the content key (`eval/mod.rs`, the exhaustive match under the tag
match) feeds it by hand — the half as a tag — or two Parts of the two
halves would key identically, and a memo hit would serve one half's
body for the other. The node is name-free (`name_free_node!` gains the
arm: it carries no `StableName` payload).

**2. Evaluation** (`eval/wire.rs`, `wire_part`, beside `wire_split`).
Read `of`'s value; the selector and the payload must agree in kind:

- `SplitHalf(h)` against `ValuePayload::Split { above, below }`: the
  named side. `SplitSide::Body(b)` ⇒ `ValuePayload::Body(b)` — the
  `Arc` itself, no clone, no re-stamp, no transform. `SplitSide::Empty`
  ⇒ a NEW typed refusal `NodeErrorKind::EmptyHalf { input, half }`
  ("the split's above half holds no material"), not `EmptyOperand`,
  whose prose is a boolean's.
- `Instance(i)` against `ValuePayload::Instances(v)`: `v[i]` when
  `0 ≤ i < v.len()`, else a NEW typed refusal
  `NodeErrorKind::InstanceOutOfRange { input, index, count }` — a
  negative index is the same refusal, not a panic, not a wrap.
- Any other pairing (`SplitHalf` on a pattern, `Instance` on a split,
  either on a single body or anything else) refuses `WrongOperand
  { input, expected: "split" | "instances", found }` through the same
  door `body_operand` uses. A single body is NOT admitted as its own
  instance 0: "wire, don't invent" (D3) — nothing is several bodies
  until a node says so.

The value is `ValuePayload::Body`, so `body_operand`, `sources_of`,
`interrogate`, the stackup digest and every other consumer of a body
are unchanged and `eval::wire::body_operand` is not touched.

**3. The half ↔ output-body index, stated once.** `Above` is output
body 0 and `Below` is 1; today that mapping is written as literals in
`names/emit_topo.rs::name_split` (`ix: 0/1`), `product.rs::sources_of`
and `names/interrogate.rs`. Give it ONE definition —
`SplitHalf::output_body(self) -> u32` on `names/role.rs` — and make
`wire_part` and those three sites its callers; the reviewer greps for
a literal on that path.

**4. Names pass through** (`names/table.rs`, a `NameTable::project(body:
u32) -> Result<NameTable, NamingError>`): the Part's table is `of`'s
table restricted to the rows whose `EntityRef::body` is the selected
output-body index, each re-keyed to body 0 (a `Body` payload is body 0
to every reader), names VERBATIM. A `Tied` entry keeps the refs in
the selected body and is dropped if none remain. The Part contributes
no `RoleSeg` and mints nothing — `Transform`'s rule (`eval/wire.rs`,
"identity-preserving pass-through"): the name still points at the
split's or the pattern's minting node with its `SplitBody(half)` or
`Instance { i, of }` path, so every selector a user has already spelled
against that half or that instance resolves on the Part unchanged.
`check_total` runs against the projected body and stays the totality
tripwire. Consequence, stated at the projection and pinned (A3): a
selector spelled for instance `j ≠ i` finds NO row in Part(i)'s table
and refuses through the N5 ladder as absent — it never re-anchors to
instance `i`'s entity, however congruent.

**5. Sweep of the exhaustive matches.** Every site the compiler names
plus the ones it cannot: `node.rs` (`inputs`, `slots`, `expr`/
`expr_mut`, the classification predicates, `placement_rule_fault`,
`name_free_node!`), `eval/mod.rs` (tag match — take the NEXT FREE tags
under `node_tag_space_is_injective` AT DISPATCH: DOCM-1's `FaceFrame`
holds 32 on main by then, so expect 33 for `Part{SplitHalf}` and 34
for `Part{Instance}`, two tags as `Pattern`'s rule kinds are two; a
published tag is never taken back — and the structural-payload match),
`refactor.rs`, `persist/check.rs` if any per-kind validation applies
(the `of` liveness is `inputs`' and needs no arm), the corpus census
(`tests/corpus/mod.rs`: `sub_kinds` gains `Part::SplitHalf` and
`Part::Instance`, `node_kind` gains `Part`), `m5_pr5_corpus.rs`, the
viewer's `tree.rs` (row label `Part`) and `combine.rs::denotes_body`
(`true`, with the sentence the function's doc now says is waiting on
"a vocabulary the recipe does not yet have" rewritten to the present).
No wildcard arm anywhere.

**6. The corpus.** One document under `tests/corpus/` authored from its
function carrying a split with BOTH halves selected by two Parts and a
three-instance pattern with its middle instance selected, each Part
consumed downstream (a boolean of the two halves back together; a
transform of the instance) so the corpus census counts both sub-kinds
and the wire round-trip carries both. The committed corpus regenerates
from its authoring function — a format change is a corpus regeneration
and nothing else (`persist/mod.rs` module doc).

## Acceptance

- **A1 — the half IS the half.** Split a box; `Part(Above)` fed to a
  `Transform`, a `Boolean` and a `Fillet` yields, for each, a body
  `bit_eq` to the same consumer fed the half read straight off the
  split's value through `interrogate`. Same for `Part(Below)`. The
  memo reuses the split across the three (`reused`/`recomputed` read).
- **A2 — the instance IS the instance.** A three-instance linear
  pattern; `Part(Instance(1))` unioned with `Part(Instance(2))` is
  `bit_eq` to the pair union of `v[1]` and `v[2]` read off the value;
  `Part(Instance(0))` is the input body's `Arc` (pointer-equal:
  instance 0 is the input itself).
- **A3 — names pass through, and only the selected body's.** A fillet
  whose selection was spelled against the split's `SplitBody(Above)`
  face rows resolves on `Part(Above)` with the same edge count; a
  selector spelled `Instance { i: 2, .. }` against `Part(Instance(1))`
  refuses typed through the N5 ladder (name the arm) and does not
  re-anchor; `Part(Instance(1))`'s table has exactly the master's row
  count and every name carries `Instance { i: 1, .. }`.
- **A4 — refusals, one row each, typed:** `EmptyHalf` (a plane that
  misses the box; the empty side's Part refuses, the other's
  evaluates); `InstanceOutOfRange` at `i == count`, at a negative
  index, and after `SetStructuralParam` lowers the pattern's count
  below a live index (memo recomputes the Part; the pattern node is
  `Ok`); `WrongOperand` for `SplitHalf` on a pattern, `Instance` on a
  split, and either on a plain body; `SetParam` on `SlotId::Instance`
  refuses `StructuralSlotNeedsStructuralEdit`.
- **A5 — the key separates what the memo must separate.** `Part(Above)`
  and `Part(Below)` of one split have different content keys;
  `Part(Instance(1))` and `Part(Instance(2))` of one pattern do; an
  edit of the index recomputes the Part and nothing upstream. The tag
  census passes with the two new tags.
- **A6 — the seat tracks the door.** `denotes_body` answers `true` for
  `Part`; `the_body_seat_tracks_the_evaluators_operand_door` builds a
  Part of a split and a Part of a pattern and keeps passing;
  `several_bodies_are_not_one_body_at_a_seat` is untouched. The Part
  DOOR in the viewer (`SessionOp::AddPart`) is CHROME's and is not
  built here — file it in the PR body.
- **A7 — lanes and the wire.** The corpus document evaluates at
  `Interval` with a widened parameter the split's body reads, and each
  Part's body is `bit_eq` to the half/instance read off the value at
  the same lane (run at two ε); the document saves, loads and replays
  bit-identical (the existing replay-identity row extended); the
  product of a document whose only root is a `Part(Above)` is that one
  half — the unselected half is in no product, stated at
  `sources_of`'s doc.

## Constraints, binding

- `docs/prompts/implementer-discipline.md` in full, by path. Hosted CI
  is the verification of record; poll it in the foreground; never end
  a turn with background work active.
- **Blinding: NO `Co-Authored-By` trailer in lane commits** (the A/B
  experiment's rule overrides the harness convention; if one lands in
  a pushed commit, note it in the PR body and carry on — never rewrite
  history).
- Merge-only: no rebase, no force-push, no squash. Push early and
  often.
- Private `CARGO_TARGET_DIR` and private scratch directory, both
  outside the worktree. Read `git status` before every `git add`;
  never `git add -A`.
- Comments state the invariant, not the history. The PR description
  carries the argument.
- Nothing here touches `eval::wire::body_operand`, `resolve/vdiff.rs`,
  `crates/profile/*`, `crates/sweep/*`, `crates/topo/*`, the analysis
  lane, or `crates/pncad-py` beyond the rows its exhaustive mirrors
  force (file the `.pyi`/`Node.part` consequences in the PR body).
- Do not add a `DocEdit` that rewrites `select` (delete-and-insert is
  the edit today; a row that wants more is a finding for the PR body),
  a `Part` of a `Boolean`/`Union` value, a "whole pattern as one body"
  selector (that is `PlacedUnion`'s sentence), or a re-stamp of the
  selected body's descriptions.
- **Stop clause.** If the projected table fails `check_total` on any
  corpus body, or a consumer turns out to read a `Body` payload's
  rows by an index other than 0, STOP: write what you measured
  (file:line, the shape) in the PR as a draft and end your turn — the
  orchestrator rules.

## Out of scope

The viewer's Part door (CHROME); the Python surface (LIB); an edit
that changes a Part's selection in place; splice (DM6, parked).

## Review

v6 dual on the frozen head, claims to falsify (the reviewers get these
verbatim plus `docs/prompts/reviewer-style-lane.md` by path):

- **C1** The Part's body is the half's/instance's own `Arc` (no clone,
  no re-stamp) and every consumer sees it `bit_eq` to the value read
  off the split/pattern (A1, A2) — on a body and a plane the
  implementer did not choose.
- **C2** The table is a projection with names verbatim: no segment
  added, only the selected body's rows, re-keyed to body 0, and a
  selector for another instance refuses typed and never re-anchors
  (A3).
- **C3** Every refusal in A4 is typed and reached; no path panics,
  wraps or clamps an index; the index is a structural slot and cannot
  be set continuously.
- **C4** The half ↔ index mapping has one definition and four callers
  (grep for a literal); the two content-key tags are next-free and
  pass the census; the two halves and two instances key apart (A5).
- **C5** Every exhaustive `Node` match gained its arm with no wildcard;
  the corpus census counts both sub-kinds; the round-trip replays
  bit-identical; `denotes_body`'s doc no longer promises a vocabulary
  that now exists (A6, A7).
- **C6** The Interval-lane rows hold at two ε and the product row holds
  (A7); nothing in the diff compares a number to decide anything.
