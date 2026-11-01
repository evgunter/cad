---
id: pncad-py-doc-has-no-node-kind-read-door
kind: issue
title: pncad-py: Doc exposes no node-kind read door, so a Python row cannot say which node is the group
status: closed
opened: 2026-09-03
closed: 2026-09-06
---

Found while landing LIB-DIETOOL's Python re-authoring of the `die_tool`
corpus document.

## The gap

`Doc`'s read surface is `order()`, `node_count`/`__len__`,
`placement`/`placements`, `reference` and `interface`
(`crates/pncad-py/pncad.pyi:1290-1370`). None of them answers **what
kind of node** a `NodeId` holds. The evaluation side does not close it
either: `Evaluation.value(n).kind` is the VALUE's kind — a
`PlacedUnion` and a `Boolean(Union)` both answer `"body"`
(`crates/pncad-py/pncad.pyi:1944-1954`).

So the claim the group boolean exists to make cannot be asserted
directly from Python. The Rust row says it outright by matching on the
node:

    crates/editor-core/tests/lib_placedunion.rs:240
    fn the_die_tool_is_one_node_and_still_cuts()
      -> counts Node::PlacedUnion / Node::Boolean{Union} / Node::Transform
         over doc.order() and asserts (1, 0, 0)

Its Python mirror
(`crates/pncad-py/tests/test_placed_union.py::TestTheDieTool::
test_the_tool_is_one_node_and_still_cuts`) can only assert the node
COUNT — seven against the pairwise chain's eighteen — and then lean on
the saved-text byte pin, whose JSON happens to name every node's kind,
to settle which of the seven is the group. That works and is honest,
but it routes a structural question through the persistence door, and
`test_placed_union.py`'s own header bills the file as the mirror of the
Rust suite.

## Why it is worth closing

The audit's discipline is that a YES is EXECUTED by the Python suite.
Any future row about recipe SHAPE — the group replacing a chain, a
pattern node refused where a group is accepted, a fillet sitting where
a chamfer was — hits the same wall, and each will invent its own
workaround. It is also the read half of a write surface that is fully
bound: `Node.placed_union_at` authors the node, and nothing reads it
back.

## Shape of the fix

A `Doc.node_kind(node) -> str` (or a `Doc.node(node)` projection in the
shape `py/value.rs` already uses for `Value`), with the string drawn
from the same exhaustive `match` over `Node` that the wire uses, so a
kernel-side node kind added without a Python spelling is a compile
error. It lands with its `pncad.pyi` entry, its binding-census row and
its stub test, like every other door.

## Home

`work/lib/` — `crates/pncad-py`'s read surface and `pncad.pyi` are
LIB's territory glob and charter.

## Closed

Taken by LIB-MECH2 (branch `lib/mech2`).

**`Doc.node_kind(node) -> str`**, and not the `Doc.node(node)`
projection this file offered as the alternative. The projection would
have to mint a Python type per `Node` payload to be worth having, and
that is the hand-maintained-mirror shape rather than a read door; the
question this issue names — WHICH node is the group — is answered by
one word, and a projection can be added later on top of the same match
without moving the word.

**The word comes from one exhaustive `match`.**
`crates/pncad-py/src/node_kind.rs` holds a single function over the
kernel's `Node` with NO wildcard arm, so a node kind added kernel-side
and given no Python spelling does not compile. Verified by deleting
one arm: `error[E0004]: non-exhaustive patterns:
`&Node::Chamfer { .. }` not covered`. 24 words for 22 variants —
`Node::Boolean` answers a word per OPERATION (`boolean_union` /
`boolean_intersect` / `boolean_subtract`), because the three are three
kernel operations sharing one payload shape; the unprefixed `union` is
`Node::Union`, the n-ary one.

**The words are snake_case and are NOT the wire's words.** The saved
text spells a node by its serde variant identifier (`"PlacedUnion"`),
which belongs to the persistence format's compatibility contract; the
Python words follow the convention every other stable string this
crate publishes (`crate::tags`, `Value.kind`). Keeping them separate
is what stops a format decision dragging the Python API, or the
reverse.

**The whole vocabulary is pinned** by
`the_node_kind_vocabulary_matches_its_committed_roster`
(`crates/pncad-py/src/tests.rs`), on `TAG_INVENTORY`'s discipline: the
literal view of `src/node_kind.rs` is re-derived at test time through
the shared lexer and compared against `NODE_KIND_ROSTER` committed
beside it, so an added, renamed or deleted kind reds BY NAME. Verified
by renaming `"placed_union"` to `"group"`: *word(s) ADDED ["group"],
word(s) GONE ["placed_union"]*. The same test reads `pncad.pyi` and
requires every roster word to appear in backticks inside
`node_kind`'s own docstring, scoped to that docstring — a word a
caller cannot discover is a word that is not really bound.

**An unknown id REFUSES**, `EditError` carrying `unknown_node`. The
sibling reads on `Doc` (`reference`, `interface`) answer `None`
instead, and the difference is stated at the door: their `None` is a
real answer about a real node, while every live node HAS a kind, so a
`None` here could only mean "no such node".

**The workaround this issue cites is flipped.**
`tests/test_placed_union.py::TestTheDieTool::
test_the_tool_is_one_node_and_still_cuts` now counts kinds over
`doc.order()` and asserts `(1, 0, 0)` for
placed_union/boolean_union/transform — the mirror of
`crates/editor-core/tests/lib_placedunion.rs:240`. The saved-text byte
pin still holds; it is no longer what settles a structural claim.

**The census did not move, and that is a decision.**
`tests/test_binding_census.py` accounts for CURATED RUST names, and
`node_kind` adds none: the kernel answers this question by matching
the enum, not through a named export, so no `BOUND_AS` row and no
`NOT_BOUND` row changes. Its three floors are lower bounds, so a new
stub member cannot make it pass more easily.

**Also landed**: the `pncad.pyi` entry listing the whole vocabulary,
four Python rows in `tests/test_document.py`
(`TestNodeKindReadDoor`), a legal and two illegal `ty` fixture lines
(`NodeId` in, `str` out — `Node` and `NodeId` are one sentence
apart), and the `reader_census` disposition comment for
`crates/pncad-py/src/tests.rs`, which now reads a second file.

## What it does NOT cover

- **The pin proves the VOCABULARY, not the MAPPING.** Swap two arms'
  literals and the set is unchanged. Which node answers which word is
  executed by the Python suite, over real documents.
- **No projection door.** `Doc.node(node)` — the payload, not the
  word — is still unbound, and every field of every node still reads
  back only through the saved text.
- **The sweep found no second instance to fix.** Patterns: every
  `save()`, `canonical_bytes`, `read_text` and `FIXTURE` in
  `crates/pncad-py/tests/*.py`, and every `node kind` / `node-kind`
  phrase. The saved-text readers that remain are ABOUT the
  persistence door — the header line, the bit-exact round trip, and
  the two line-for-line byte pins in `test_north_star.py` and
  `test_placed_union.py` — not structural questions routed through
  it. The blind spot: a test that settles a node-shape question by
  loading a document and reading a field of the reloaded object,
  never touching the word "save"; nothing in the suite reads that
  way today, and no pattern here would catch it if it did.
