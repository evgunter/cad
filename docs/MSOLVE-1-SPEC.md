# MSOLVE-1 — The mate reads at its operand: the transform-aware solve (spec)

**Program:** MSOLVE (`work/msolve/plan.md`), unit `MSOLVE-1`
(`work/msolve/MSOLVE-1.md`). **Ruling of record:** Ev, in chat,
2026-09-05 (recorded in `work/msolve/log.md`): a mate reference carries
the node it is read at, exactly as the measurement reference already
does; the reading edge A12 recomputes goes to that operand; N1 is
untouched (a transform still mints no segment); the solve composes the
map of every pose-bearing node between the operand and the minting
instance. The finding it answers is
`work/msolve/mate-solve-is-transform-blind.md` — read it in full, and
read `crates/editor-core/tests/fix_xblind_probe.rs`, whose header says
what this unit does to it (deletes it). The ruling on PR 1731
(`work/msolve/nested-pattern-mate-heads-refuse.md`) is the reason the
pattern-of-transform case lands HERE and the nested-pattern case does
not.
**Track:** kernel change — one style review plus a correctness arm
(§Review). No A/B row (`work/msolve/plan.md`, "Review posture").

## The defect, in one paragraph

`Node::Mate` carries two `StableName`s and nothing else about where
they are read. By N1 a `Transform` contributes no segment, so a mate on
a transformed instance is byte-identical to a mate on the instance
itself; `member_of` admits it on its first arm, the solve places the
instance in its own coordinates, and `wire_transform` composes the map
downstream. Nothing refuses. The pattern case is not blind only because
the pattern DOES mint a segment (`Instance(i)`), which is an accident
of naming, not a property of pose (the measurement lane's framing:
admission is decided on naming transparency, correctness needs pose
transparency).

## What the unit builds

**1. One reference type for "a name, and the node it is read at."**
`MeasureRef` (`node.rs:963`) is already exactly this, with the
argument for it in its doc. It becomes the mate's reference too. Give
it a name that is not the measure's — `EntityRef` unless the
implementer argues a better one in the PR — keep `at`/`name` as the
serialized field names and `at_mint` as the constructor. `Node::Mate`
becomes `{ a: EntityRef, b: EntityRef, class, alignment }`. No
`Option` on `at`: the honest spelling of "as authored" is
`at_mint(name)`, and every constructor says so.

**2. The walk** (`mate/solve.rs`). `member_of(doc, r: &EntityRef) ->
Option<Member>` walks from `r.at` DOWN the consuming edges to
`r.name.node`, which must be a live `InstantiatePart`:

- at a node that is NOT the name's head: it must be a `Node::Transform`
  (the one production node that moves a body and mints nothing);
  continue at its `input`;
- at the name's head: an `InstantiatePart` ends the walk; a `Pattern`
  requires `name.path.first()` to be `Instance { i, of }`, records the
  copy `(pattern, i)`, and continues at the pattern's `input` with
  `of` as the name;
- anything else — a boolean, a union, a split, a second pattern level,
  a head the chain never reaches — is outside the vocabulary and
  returns `None`.

Structural only: no expression is evaluated in admission (the cluster
partition never depends on a slot value — this is a standing rule, keep
it). **Nested patterns stay outside the vocabulary in this unit**
(`Member.copy` stays `Option<(RecipeNodeId, u32)>`; MSOLVE-2 carries
the chain). Pattern-of-transform and transform-of-pattern are both
inside it.

**3. The member's identity gains its operand.** `Member { instance,
at: RecipeNodeId, copy }`. Two mates from one instance through two
different transforms are two members over one instance — the same
shape as two sibling pattern copies — so they key `by_pair` as
different pairs and the second is a loop-closing declaring edge, not a
fold-mate of the first. For every mate that exists today `at ==
name.node`, so no existing document's pairs, spanning tree or solve
changes (A7). `Member` stays `Copy`.

**4. The offset over the whole chain.** `derived_offset` composes the
map of EVERY pose-bearing node the walk passed, in evaluation order
(the map nearest the instance applied first, so for pattern `M(i)` over
transform `T` the offset is `M(i) ∘ T`):

- a pattern's map is `stepped_rule_map` at `i`, as today;
- a transform's map is what `wire_transform` builds — rotation about
  the unit axis through the origin by the angle, then the translation
  — evaluated at the document's parameter bindings, with the direction
  decided through the same normalization door the pattern rules use.
  **One home:** factor the map's construction out of `wire_transform`
  into a function both callers use; do not re-derive it beside the
  pattern's copy. (The reviewer greps for a second spelling.)

`pair_left_factor` and the rest of the fold are untouched by
construction: the offset is still a static document-coordinate map
composed outside the placement, `None` when the chain is empty, so a
document without transforms or patterns composes nothing and its solve
is bit-for-bit what it was. Use ONE walk for admission and for the
offset — a chain the walk yields and the offset folds over — not two
walks with two spellings of the vocabulary.

**5. The document doors.**

- `DocEdit::InsertNode`: `at` must name a live node at insert, refused
  typed the way a never-existed name node is refused today (N5's
  door); a LATER delete of `at` may strand it — a dangling operand is a
  dangling head, `MateFault::DanglingHead` at the solve, no edge until
  the mate is re-authored. `Node::inputs()` for a mate stays EMPTY:
  the operand edge is an A12 READING edge, never consuming, or the
  mated bodies leave A10's root set. Say this at the variant.
- `reading_edges` still yields `(mate, member.instance)`.
- The split remap (`refactor.rs`, the `Node::Mate` arm): `at` remaps
  through the node map the way `MeasureRef::at` does two arms below,
  missing loudly when the cut severed it.
- The content key (`eval/mod.rs`, the `Node::Mate` arm) feeds `at`:
  two mates differing only in their operand are different keys.
- Persistence: the derive carries the new shape; the format has no
  version by design (`persist/mod.rs` header) — regenerate any
  persisted fixture holding a mate and SAY which (expect none: grep
  first, state the pattern).
- The Python door (`pncad-py/src/py/doc.rs`, the `Mate` constructor,
  and its `.pyi`): each side is the node it is read at plus the name
  text, mirroring the kernel type; `at_mint` is not a default there
  either. One Python row mates a transformed instance through the
  public door.

**6. The viewer's mate tool** (`viewer/src/matetool.rs`).
`picked_member` authors `at` from `pick.node` — the node the ray met
— and admits through the new `member_of` instead of its own
`pick.name.node != pick.node` guard (the walk refuses a fused body's
node for the same reason that guard did: a boolean is not a
pass-through). The frame is still read at the MEMBER's instance and
divided by that instance's placement: the alignment is authored in
part coordinates and the offset is the solve's to apply, which now
covers transforms exactly as it covers pattern copies. The tool's
output carries `EntityRef`s.

**7. The measurements the finding could not take, taken first.**
Before the fix, on the finding's own documents: (a) a `Prismatic`
mate along the transform's direction — does the free direction absorb
the translation (class-dependent blindness), or not? (b) a transform
with a NON-ZERO angle. Report both in the PR; whichever way (a) goes,
a row pins the CORRECT behaviour after the fix.

**8. The rows.** DELETE `fix_xblind_probe.rs` — the header says so —
and add `msolve1_transform_aware.rs` carrying the acceptance rows
below through ordinary doors (`DocEdit::InsertNode`, `solve_document`,
`evaluate`, `product`), asserting on the product's own face frames,
never by eye.

**9. The docs.** `crates/editor-core/ASSEMBLY.md`: A12's "a mate is a
DAG leaf (`inputs()` is empty)" becomes the operand sentence — a mate's
two references are read at operands, its reading edges go to the
members those operands resolve to, `inputs()` stays empty because a
reading edge is not consuming; A11 (5)'s member sentence becomes the
walk (a reference head is a live instance reached from its operand
through transforms and at most one pattern level, the member's frame
the composed static offset on its instance's pose). Present tense, no
history. `node.rs`'s `Mate` doc likewise. `names/README.md` is NOT
touched: N1 is unchanged.

## Acceptance

- **A1 — the finding's documents, fixed.** The two-block document with
  a +z translation (angle 0): `fault = None`, `product = Ok`, the
  mated faces coincide in the PRODUCT (contact true, measured on the
  gathered bodies' face frames), and the solved relative pose differs
  from the control's by exactly the transform's map.
- **A2 — rotation.** A transform with a non-zero angle about z, and
  one about a non-axis direction: mated faces coincide in the product
  within `Tol::witness()`; the `Opposed` axis agreement holds.
- **A3 — pattern-of-transform** (`step4`'s document): solves; copy 1
  seats on the base in the product. **Transform-of-pattern** (a
  transform over a pattern over an instance, mate to copy `i` read at
  the transform): seats.
- **A4 — sides.** A transform over the GAUGE side, over both sides, and
  a chain of two transforms on one side: all seat.
- **A5 — two operands, one instance.** Two mates from `base` to two
  different transforms of `top`: two members, the second mate
  `Declaring`; a geometrically consistent pair passes the gate and an
  inconsistent one refuses typed (name the fault).
- **A6 — the Prismatic measurement** (item 7a) reported and pinned.
- **A7 — nothing else moves.** Every existing mate, assembly, split,
  viewer and Python suite passes unchanged; a document with no
  transform and no pattern solves bit-for-bit (the suites that assert
  bit-identity of solved frames are the witness — name them in the PR).
- **A8 — doors.** Insert with a never-existed `at` refuses typed;
  deleting the transform a mate reads at leaves `DanglingHead { mate,
  side, head }`; a split whose cut severs `at` from the mate refuses
  at the remap; content keys differ on `at` alone; a mate with a
  transform operand round-trips through persistence.
- **A9 — the tool.** A pick on a transformed instance is admitted and
  authors `at = pick.node`; a pick on a fused body is still
  `NotAnInstancePick`; a pick on a pattern copy over a transform is
  admitted.
- **A10 — the vocabulary's fence is stated.** A nested-pattern head
  still refuses `DanglingHead` and the doc at `member_of` says MSOLVE-2
  is where that ends.

## Constraints, binding

- `docs/prompts/implementer-discipline.md` in full, by path. Hosted CI
  is the verification of record; poll it in the foreground; never end
  a turn with background work active.
- Merge-only: no rebase, no force-push, no squash. Push early and
  often. Open the PR with the GitHub MCP tools (there is no `gh` here).
- Private `CARGO_TARGET_DIR` and private scratch directory, both
  outside the worktree. Read `git status` before every `git add`;
  never `git add -A`.
- Comments state the invariant, not the history. The PR description
  carries the argument, the measurements of item 7, the sweep for
  persisted mates, and the hit list for every `Node::Mate { a, b, .. }`
  constructor in the tree (28 files at the spec's writing).
- Fence: `crates/editor-core/src/{node.rs, edit.rs, refactor.rs,
  mate.rs, mate/*, eval/mod.rs (the key arm), eval/wire.rs (the
  transform-map factoring only), persist/* as the derive forces}`,
  `crates/viewer/src/matetool.rs` and what consumes its output,
  `crates/pncad-py` (the mate door and `.pyi`), tests, the two docs
  named in item 9, `demos/tour` only where a constructor forces it.
  Nothing in `names/*`, `assembly.rs`'s mint, `crates/topo/*`.
- Do not evaluate an expression inside `member_of`; do not give `at`
  an `Option` or a default; do not make the operand a consuming edge;
  do not extend `Member.copy` to a chain (MSOLVE-2); do not touch
  `derived_offset`'s catch-all arm beyond what the walk forces
  (MSOLVE-3).
- **Stop clause.** If admission and the offset cannot share one walk
  without evaluating in admission; if a persisted fixture with a mate
  exists that cannot be regenerated from its authoring function; or if
  `Member` gaining `at` changes the pair set of ANY existing suite —
  STOP, write what you measured (file:line, the shape) in the PR as a
  draft, and end your turn.

## Out of scope

Nested patterns and the member chain (MSOLVE-2); the `DanglingHead`
catch-all's typed replacement (MSOLVE-3); a `Rebind`-style edit that
re-targets a dangling operand (re-author the mate); the AQ8 SKIP half's
ratification (its own `[ev]` PR); the assembly mint's reading of
mate names (`assembly.rs`), which resolves in the product's table and
is unchanged by the operand.

## Review

One style review (`docs/prompts/reviewer-style-lane.md` by path) plus
a correctness arm on the frozen head; both get these claims verbatim:

- **C1** A1–A4 hold on documents the implementer did not choose:
  build one of your own with a rotation and a chain and measure the
  product's face frames.
- **C2** Admission is structural (no evaluation in `member_of`) and
  admission and the offset are ONE walk; the transform's map has ONE
  home shared with `wire_transform` (grep for a second rotation-then-
  translation construction).
- **C3** A5: two operands over one instance are two members; the
  second mate declares; the gate verifies it; an inconsistent pair
  refuses typed.
- **C4** A7: bit-for-bit on the no-transform, no-pattern solve; every
  existing suite unchanged; the mated bodies remain product roots
  (`inputs()` empty; A10's invariants see no new edge).
- **C5** A8–A9: every door and the tool author, refuse, remap and key
  `at` as the spec says; the persisted format round-trips; no
  persisted fixture silently changed meaning.
- **C6** The docs state the present design (A11 (5), A12, the `Mate`
  variant) and `names/README.md` is untouched; `fix_xblind_probe.rs`
  is gone and nothing re-pins its expectations as a baseline.
