---
id: mate-head-entity-kind-is-decided-only-at-assembly
kind: issue
title: A mate head's EntityKind is decided at assembly and never at the edit door
status: review
pr: 2799
branch: edit/mate-head-kind
opened: 2026-09-16
refs: [three-door-predicates-are-hand-copied-not-shared]
---


Raised by the style review of `edit/one-predicate-round-two` (PR 2772)
while reading that unit's shared-predicate moves. **It is not a
duplication**, which is why it is its own row rather than a hit on the
sweep: nothing spells this rule twice, because only one door spells it
at all.

**The finding.** A `Node::Mate`'s two heads are `SitedRef`s whose
`StableName` carries an `EntityKind`. A mate is a FACE-to-FACE contact:
`crates/editor-core/src/assembly.rs`'s `resolve_face` refuses
`RefusedRef::NotAFace { found }` for a head whose name is not a face,
and says at the site that kind precedes multiplicity.

That is the only place the kind is decided. The edit door that ADMITS
the mate — `InsertNode`, through `check_node_inputs` — checks the
heads' referenced nodes and the alignment's finiteness
(`Node::has_non_finite_alignment`, now shared), and says nothing about
what the heads denote. `EntityKind::Face` appears in `edit.rs` only for
the appearance and metadata doors. So a mate whose `b` head names an
EDGE inserts cleanly, saves cleanly, loads cleanly, and refuses at
evaluation.

**Which class this is.** It is the V1 class-2 shape — a document may
hold a node that refuses to evaluate — and that reading is defensible:
a head's name-level resolution needs a product, which the edit door
does not have (the ruled `Declare` carve-out applies the same way to
the second name-referencing edit). But the KIND is not name-level
resolution: `name.kind` is data on the `StableName`, readable with no
product at all, and `assembly.rs` reads it before it consults any
table.

**The question the row asks**: should the edit door refuse a mate head
whose `EntityKind` is not `Face`, in the vocabulary
`AppearanceWrongKind` already uses for the same shape of mistake? If
yes, the predicate has one home (beside `Node::Mate`, or on the
`SitedRef`) and `assembly.rs` names its answer — the round-two unit's
move, applied to a rule that currently has one door instead of two. If
no, both sites say why the kind waits for evaluation although it needs
no product.

Not built by the round-two unit: it is an addition to what the edit
door refuses, not a relocation of a predicate, so it changes which
documents exist.

## Ruled and spec'd (2026-09-17, EDIT orchestrator) — middle tier, branch `edit/mate-head-kind`

**Ruling: yes.** The edit door refuses a mate head whose
`StableName.kind` is not `EntityKind::Face`, in the vocabulary
`AppearanceWrongKind` uses for the same shape (`EditError::MateHeadWrongKind
{ name, found }` or the spelling the existing arm's convention gives).
The kind is data on the name and needs no product, so it belongs with
what the edit door already checks of a mate (`check_node_inputs`: the
heads' nodes, the alignment's finiteness). `assembly.rs`'s
`resolve_face` keeps its `NotAFace` refusal (a loaded document can
still carry the node — see the load half) and its doc names the edit
door as the first reader.

**The predicate has one home**: on `SitedRef` (`fn is_face(&self)` or
the name that says it) or beside `Node::Mate`; the edit door and
`assembly.rs` both read it; the load door (`persist/check.rs`) asks it
too, placed on the exhaustive `Walk` (a new arm, F6 case, `pncad-py`
tag — LIB's, mechanical, the round-three shape), so a saved mate whose
head names an edge refuses to load as it refuses to insert. Ev is told
of the ruling on the fourth `[ev]` PR (#2795) as an FYI.

**Rows.** Red first: a mate whose `b` head names an edge inserts today
and refuses at evaluation — write the row that says it refuses at
`InsertNode`, watch it red, then the door. Then: the load twin; a
round trip of a face-to-face mate; `assembly.rs`'s `NotAFace` still
reachable (a head whose kind is `Face` but resolves to no face — say
whether that is possible, and if not, what `NotAFace` now guards).

**Mutants:** drop the edit-door check (the insert row reds); check
only head `a` (the `b` row reds); the load walk dropped from `ORDER`
(the load row + the roster row red).

**Territory.** `crates/editor-core/src/{edit.rs, node.rs, assembly.rs,
persist/check.rs}` (EDIT); `crates/editor-core/tests/*` (TCOST/TINT);
`crates/pncad-py/src/{tags.rs, tests.rs}` (LIB, mechanical). Middle
tier.

## Built (2026-09-17, `edit/mate-head-kind`)

**Re-scoped before the build landed** (orchestrator, ratified by Ev on
the `[ev]` PR): a non-face mate head fails to TYPECHECK rather than
refusing at the edit door. The ruling's answer — the edit door decides
the kind, not the at-rest gate — stands; what changed is where the
decision is written down.

`crates/editor-core/src/names/role.rs` carries `FaceName`, a
`StableName` whose kind is `EntityKind::Face` by construction, with
one checked constructor (`FaceName::new -> Result<_, NotAFaceName>`),
`Deref`/`AsRef` to the inner name, and a `Deserialize` that goes
through the constructor. `crates/editor-core/src/node.rs` carries
`SitedFace` — `SitedRef`'s shape over a `FaceName` — and `Node::Mate`'s
two heads are `SitedFace`s. **A mate whose head names an edge is a
program that does not compile**, pinned by `SitedFace`'s own
`compile_fail,E0308` row.

A separate `SitedFace` rather than a generic `SitedRef<N>`: the two
payloads that hold a sited reference want different things of it — a
measure's `at` is a DAG edge and its name is any kind, a mate's `at` is
an A12 reading edge and its name is a face — and only three consumers
are mate-only (`assembly::resolve_face`/`operand_answer`,
`mate::member`'s walk, `refactor`'s crossing gate). A type parameter
would have put a bound on every signature that names the type for a
distinction two structs make with no bounds at all.

**The three boundaries that turn data into names call the
constructor**: the wire (`FaceName`'s `Deserialize`, so a file whose
head is retyped refuses as `PersistError::Unreadable` — the reader
accepted the bytes and this build's types rejected them), the Python
binding (`face_name_from_text`, raising `EditError.variant ==
"mate_head_not_a_face"` at the `Node.mate` call), and the viewer's
picked face (whose selection door already refused a non-face; filed as
`work/view/face-selection-carries-a-bare-stable-name`).

**No edit-door arm and no `Walk` arm** — the spec's two doors are what
the type replaced. `EditError` and `SnapshotError` are unchanged.

`assembly.rs`'s `RefusedRef::NotAFace` **stays, and now guards one
thing**: `NameTable::insert`'s rule that a row's kind is its name's.
Its two kind rungs are gone — `resolve_face`'s (the head is a face by
type) and `operand_answer`'s rung 2 (same) — leaving the release
answer for a table that handed a face name a non-face key, asserted in
debug at the site. The ladder is three rungs now and its doc says so.

`msolve5_read_below_a_root`'s three kind rows are DELETED: each built
a document that cannot be written, and a row measuring an unreachable
state measures nothing. What replaced them: the `compile_fail` row,
`edit_one_predicate`'s load-door row over both heads and all three
non-face kinds, `a_face_to_face_mate_round_trips`, and the Python
row. The suite's header says where the question went and what its
ladder still decides.

Residue in its own file:
`interface-crossing-heads-are-bare-stable-names` (EDIT's — the split's
crossing record did not follow the type) and
`work/view/face-selection-carries-a-bare-stable-name` (VIEW's).
