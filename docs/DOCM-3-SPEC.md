# DOCM-3 — `Node::Union` (n-ary), `DocEdit::SetMembers`, and pairwise-distinct inputs (spec)

**Program:** DOCM (`work/docm/plan.md`), unit `DOCM-3`
(`work/docm/DOCM-3.md`). **Ratified design:**
`docs/DOCM-REFERENCES-DESIGN.md` DM4, DM5, DM6 — read them first; this
spec binds the build and does not re-open them.
**Track:** kernel change — the standard v6 unit (binding spec, drawn
implementer arm, cross-model dual review, union fix pass, record-at-merge;
§Review below).
**Pre-draw fields, logged before the draw:** difficulty **L**, task-class
**STRUCTURAL**.

- **L** — one new node kind reaches every exhaustive `Node` match (the
  v13 `Node::Mate` sweep's shape: `node.rs`, `eval/`, `persist/wire.rs`,
  `persist/check.rs`, `refactor.rs`, `resolve/`, the viewer's `tree.rs`
  and `combine.rs`, the Python mirror is LIB's and is filed, not built),
  plus a new edit, a new naming segment with its own emitter, and the
  die rewritten onto it with an acceptance row that only the naming
  design can pass.
- **STRUCTURAL** — the fold reuses the pair verb unchanged; no new
  numeric decision anywhere. Naming is data movement.

## What the unit builds

**1. `Node::Union { members: Vec<RecipeNodeId> }`** (`node.rs`, beside
`Boolean`). Two or more members, each a single-body operand under
`eval::wire::body_operand`'s existing rule (`eval/wire.rs:457`), ONE body
out as `ValuePayload::Boolean(BooleanValue::Body { .. })` or
`BooleanValue::Empty` — the same value shape a pair union yields, so every
consumer of a union is unchanged. `Node::inputs` returns the members in
list order; `Node::payload_names` is unchanged (`name_free_node!` gains
the arm). No `declare` field: a declared-contact union is spelled with
`Boolean` today and stays so; say this at the variant.

Evaluation (`eval/wire.rs`, beside `wire_boolean` at `:1962`): fold the
pair verb over the members in list order — `((m0 ∪ m1) ∪ m2) ∪ …` — with
`BooleanOp::Union` and no declarations, through the same
`verbs::run_pair` door and the same `refusal_menu`. D9: the order is the
list's. Content key: a new op tag (take the next free tag under the
injectivity census at `eval/mod.rs`, `verb_tags_are_injective`'s
neighbourhood; do not reuse a number) hashing the member list in order.
An empty intermediate result short-circuits to `Empty` only if that is
what the pair verb would do next; do not invent ∅-absorbing semantics —
"wire, don't invent" (D3) — refuse `EmptyOperand` naming the member
exactly as `body_operand` does for a pair.

**2. Naming keyed by member** (`names/role.rs`, `names/emit_topo.rs`).
A new segment **`RoleSeg::FromMember(Box<StableName>)`**, in the boolean
group beside `FromA`/`FromB`, wrapping a member's own name; seam and
merged rows keep `Seam`/`Merged`/`Fragment` as the pair emitter mints
them. The requirement, and the reason the unit exists: **a member's names
under the union do not depend on the member's POSITION in the list or on
how many members precede it.** The pairwise fold inside the node produces
nested `FromA`/`FromB` descents; the union's emitter collapses each
surviving entity's descent chain to the member it came from and mints
`FromMember(inner)` with `inner` the member's name in that member's own
table — one wrapper, whatever the depth. Seam entities between two
members name both members' contributing entities through the existing
`Seam { a, b }` shape with each side's inner name un-nested the same way.
`check_total` stays the totality tripwire. `names/select.rs` gets the
selector tag for the new segment; `SegTag` and the `.pyi` mirror are the
`C6`/`D366` rows' business and are filed, not built.

**3. `DocEdit::SetMembers { node, members: Vec<RecipeNodeId> }`**
(`edit.rs`). Replaces the member list of a `Union` whole. Refuses typed:
`UnknownNode`; `SetMembersOnNonList { node }` when the node holds no
list; `UnresolvedInput` for a member that is not live; `WouldCycle`
through the existing `check_acyclic`; `TooFewMembers { node, found }`
under two; and DM5's `DuplicateInput` (below). `structural: true`. It is
persisted and replayed like every edit; the wire mirror grows the arm.
`Loft`'s `profiles` list is the same shape: take it in this unit if the
edit generalizes cleanly over a "list input" accessor on `Node`, else
leave `Loft` untouched and say so in the PR — either is in spec.

**4. DM5, stated once** (`edit.rs`): a structural validity check that a
node's inputs are pairwise distinct — `EditError::DuplicateInput { node,
input }` — called by `InsertNode` and by `SetMembers` on the rewritten
node. Today nothing refuses `Boolean { a: X, b: X }` (measured: the
`InsertNode` arm at `edit.rs:1300`–`1345` and `wire_boolean`); after this
unit the door does. Sweep every existing corpus and test document for a
node this rule newly refuses and report the hit list (§Sweeps in the
discipline doc); expect none.

**5. The die** (`demos/tour/src/diefillet.rs`, `pipped_node`): the 21
transforms become the members of ONE `Node::Union`; the "NAMED GAP
(2026-08-14)" comment goes (its content is now false); the tour corpus
regenerates (`corpus_text`) and the committed corpus files re-bless from
their authoring functions — a format change is a corpus regeneration and
nothing else (`persist/mod.rs` module doc). Both fillets' selections are
re-taken through the same `select_where` calls; the row counts
(`box_edges`, `rims`) are unchanged.

## Acceptance

- **A1 — remove one pip and both fillets still resolve.** On the rewritten
  die, for pip `i` in {first, middle, last}: apply `SetMembers` without
  member `i` and `DeleteNode` of that transform, re-evaluate with the
  previous evaluation as `prior`, and assert the box-edge fillet and the
  rim fillet both evaluate `Ok` with `rims − 1` rim edges selected and no
  `BlendSelectionResolve`. This is the row the naming design exists for;
  under position-dependent names it goes red for every earlier pip.
- **A2 — names are position-free.** For a three-member union of three
  distinct boxes, the name table of the union restricted to member `k`'s
  faces is identical (as a set of `StableName`s) whether `k` is listed
  first or last. A row per `k`.
- **A3 — the fold equals the chain.** The union of the die's 21 pips is
  bit-identical in geometry (`bit_eq` or the dump differential the tour
  already has) to the pairwise chain it replaces, evaluated at the same
  scalar; only the names differ, and the PR shows the two tables side by
  side for one pip.
- **A4 — DM5 refuses at both doors.** `InsertNode` of `Boolean { a: X, b:
  X }`, of `Union { members: [X, X] }`, of `Split { target: X, tool: X }`,
  and `SetMembers` producing a duplicate, each refuse
  `DuplicateInput`; the check is one function with two callers (the
  reviewer will grep for a second copy).
- **A5 — `SetMembers`' refusals**, one row each: non-list node, non-live
  member, cycle, under two members. And the wire round-trip: a document
  carrying a `Union` and a `SetMembers` in its edit log saves, loads and
  replays bit-identical (the existing replay-identity row extended).
- **A6 — the seat tracks the door.** `viewer::combine::denotes_body`
  answers `true` for `Union`;
  `the_body_seat_tracks_the_evaluators_operand_door` builds a minimal
  union and keeps passing; `several_bodies_are_not_one_body_at_a_seat`
  is untouched. The union SEAT (N picks) is CHROME's and is not built
  here — file it in the PR body.

## Constraints, binding

- `docs/prompts/implementer-discipline.md` in full, by path. Hosted CI is
  the verification of record; poll it in the foreground; never end a turn
  with background work active.
- **Blinding: NO `Co-Authored-By` trailer in lane commits** (the A/B
  experiment's rule overrides the harness convention; if one lands in a
  pushed commit, note it in the PR body and carry on — never rewrite
  history).
- Merge-only: no rebase, no force-push, no squash. Push early and often.
- Private `CARGO_TARGET_DIR` outside the worktree. Read `git status`
  before every `git add`; never `git add -A`.
- Comments state the invariant, not the history. The PR description
  carries the argument.
- Nothing here touches `resolve/vdiff.rs`, `crates/profile/*`, the
  analysis lane, or `crates/pncad-py` (LIB's; file the `.pyi` and
  `Node.union` consequences in the PR body, one line each).
- Do not add a `Node::Intersection`/`Subtract` n-ary sibling, a
  `declare` field, or a `SetMembers` for anything but list inputs. A
  need you meet for any of these is a finding for the PR body.

## Out of scope

The union seat in the viewer (CHROME); the Python surface (LIB); splice
(DM6, parked); `PlacedUnion` (a different sentence, unchanged).

## Review

v6 dual on the frozen head, claims to falsify (the reviewers get these
verbatim plus `docs/prompts/reviewer-style-lane.md` by path):

- **C1** A member's names under a `Union` are independent of its list
  position and of the count of members before it (A2), and A1 holds for
  the first, a middle and the last pip — the reviewer removes a pip the
  implementer did not choose.
- **C2** The fold is bit-identical to the pairwise chain at the same
  scalar (A3), and nothing ∅-absorbing was invented on an empty
  intermediate.
- **C3** The DM5 check has exactly one definition and two callers, and
  refuses every shape in A4; no corpus document was silently changed to
  pass it (the hit list in the PR is the receipt).
- **C4** `SetMembers` cannot reach a state `InsertNode` would have
  refused (cycle, dangling, duplicate, short list), and the wire
  round-trip replays it bit-identically.
- **C5** Every exhaustive `Node` match gained its arm with no wildcard,
  and the content-key tag is new and passes the injectivity census.
- **C6** The die renders through the tour's own door and the two fillets
  select the same counts; if a frame moved the PR says what and why.
