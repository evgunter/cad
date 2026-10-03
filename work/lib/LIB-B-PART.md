---
id: LIB-B-PART
kind: unit
title: binding census family B-PART
status: closed
branch: lib/b-part
opened: 2026-09-04
pr: 2163
closed: 2026-09-08
---


Queued mechanical census family (the B-RESOLVE shape): sweep the
family's bindings against the census contract, construct the
previously unconstructible pins where the surface now allows, and
re-cut the census rows honestly. Families share the census/tags/test
files, so at most two run concurrently, staggered.

## Derived scope (stated before any code changed)

### The census roster of this family is ONE entry

`crates/pncad-py/tests/test_binding_census.py` names `B-PART` in
exactly three places: the charter in `FAMILIES` (`:737`), the comment
above the entry (`:1645`), and the entry itself —
`"PartSelect": f"{GAP}: B-PART the projection node's surface"`
(`:1648`). That single row IS the roster. The charter names three
things, and the other two are NOT rows and could not become ones —
the B-FACE-FRAME lesson, one level in each direction:

- **`Node.part` is an ARM of a curated enum.** The census's alphabet
  is top-level stub names plus the `Class.member` spellings `BOUND_AS`
  points AT. `Node` is curated (`crates/pncad/src/document.rs:40`) and
  `pncad.pyi:1183` declares a top-level `Node`, so rule 1 accounts it
  WHOLE. Twenty-odd `Node::*` arms hide behind that one name, and
  which of them Python spells as a constructor is invisible here. That
  is the enum-VARIANT half of the blind spot `py/select.rs`'s growth
  tripwire records (issue #1309); `face_carrier_kind` was a row
  because it is a FREE FUNCTION, and `Node.part` is not one.
- **`SlotId.Instance` is a FIELD of a `different-shape` row.**
  `SlotId` is in `NOT_BOUND` as `SHAPE` (`:1350`) with its reason
  written out at `:867` — "`SlotId` is what `DocEdit.bind_count_param`
  names implicitly". So the slot vocabulary crosses as NAMED DOORS,
  never as the enum, and a slot with no door is invisible: the census
  cannot see that `bind_count_param` hardcodes `SlotId::Count` and
  that `SlotId::Instance` therefore has no door at all.

So the census delta is one entry and one charter. Under rule 1 the
`PartSelect` entry leaves `NOT_BOUND` ENTIRELY rather than moving to
`BOUND_AS`: the sibling selector vocabulary `PatternKind` is spelled
identically in the stub as a class of static constructors, and
`PartSelect` binds the same way, so a name-for-name match accounts it.

### What `pncad-py` can reach

`pncad-py` depends on `pncad` and `quantity` only. Every door is
inside that: `PartSelect` and `Node` are curated in
`crates/pncad/src/document.rs:40`, `SplitHalf` in `select.rs:63` and
`prelude.rs:325` (and already spelled identically at
`pncad.pyi:2026`, with `Above`/`Below`), `SlotId` in `document.rs:41`.

Ground truth verified in Rust: `PartSelect::{SplitHalf(SplitHalf),
Instance(Expr)}` (`crates/editor-core/src/node.rs:909`), `Node::Part {
of, select }` (`:1604`), `SlotId::Instance` (`:336`, `Dimension::Count`
at `:486`, so `is_structural()` at `:498`), the eval arm `wire_part`
(`crates/editor-core/src/eval/wire.rs:2442`) and the two refusals
`NodeErrorKind::{EmptyHalf, InstanceOutOfRange}`
(`crates/editor-core/src/eval/mod.rs:783`, `:793`), already tagged
`empty_half` / `instance_out_of_range`
(`crates/pncad-py/src/tags.rs:427-428`) and already inventoried in
`crates/pncad-py/src/tests.rs:1711`/`:1724`. `Doc.node_kind` already
answers `"part"` (`crates/pncad-py/src/node_kind.rs:71`, LIB-MECH2).

### The finding this sweep turned up BEFORE any code: the Instance arm has no input

`PartSelect::Instance` selects out of a value whose payload is
`ValuePayload::Instances`, and **exactly one node in the kernel emits
that payload**: `Node::Pattern` (`eval/wire.rs:3740`, the only
construction site outside the consumers). `Node::Pattern` is UNBOUND
in Python — the stub's module docstring lists "the pattern node" as
deliberately absent (`pncad.pyi:70`), the audit page's G8 row says
"binding it would still flip no row", and
`test_north_star.py:3238` asserts `assertFalse(hasattr(Node,
"pattern"))`.

So binding `Node.part` alone would ship a door whose Instance half is
UNREACHABLE from Python: no positive row, and one of the family's two
chartered refusal tags (`instance_out_of_range`) unconstructible.
The census cannot see that — `Node` is accounted whole — which is the
same blind spot as above, now with a bill.

**`Node.pattern` is therefore in scope, and G8's stated reason for
leaving it out is the thing this unit retires.** That reason is that a
pattern's plural payload feeds no downstream door; `Node::Part` IS
that door, and the kernel says so at `node.rs:1601` ("the only node a
pattern of a pattern can be built through"). The audit row's
CONCLUSION is untouched — no tour scene flips, `PlacedUnion` is still
what the heat sink is authored with, and a boolean still refuses a
plural payload — so what moves is one sentence of rationale, not a
YES/NO cell.

### The second finding: `SlotId::Instance` has no edit door

`DocEdit.bind_count_param` (`crates/pncad-py/src/py/doc.rs:2094`)
hardcodes `slot: d::SlotId::Count`, and its doc comment justifies that
with "the slot is `Count` — the only structural slot there is". That
is false: `SlotId::{Count, VDegree, Stations, Instance}` all answer
`Dimension::Count` (`node.rs:486`) and so all four are structural.
A Part's index slot therefore has no door, and the family charters
one. Bound here as a SIBLING door, `DocEdit.bind_instance_param`,
because the census's own reason for `SlotId` being `different-shape`
is that Python addresses a slot through its own spelling — a `slot=`
argument would be the enum crossing under another name. The stale
sentence is corrected in place; the general door stays out (it needs
the expression-authoring vocabulary that is still a named gap).

### Decisions honored, not relitigated

- `crates/pncad/tests/all.rs`'s `NOT_CARRIED` says nothing about this
  family: `PartSelect` is CARRIED (`document.rs:40`) and `SplitHalf`
  is carried twice over.
- `SplitHalf` already crosses INTO the kernel (`py/select.rs:307`, in
  `SideArg::to_kernel`), so this unit adds a second crossing of an
  existing mirror and no new vocabulary. Its kernel-growth tripwire is
  `growth_tripwire::side` and stays where it is: the tripwire matches
  the KERNEL enum, and the new crossing matches the PYTHON one, whose
  exhaustiveness the compiler already checks.
- `StableName` stays text, `SplitSide` stays the position in
  `Value.split`'s tuple, `Dimension` stays what the `DocParam`
  constructors choose between. None is re-opened.
- The refusal tags exist and are inventoried. This unit adds no tag
  function, no tag value and no inventory row; what it adds is the
  first CONSTRUCTION of both arms from Python, which is the half
  `TAG_INVENTORY` says outright it does not cover (`node_error_tag`
  has no construction pin).

### What the census still cannot see after this unit

That `Node.part`, `Node.pattern` and `DocEdit.bind_instance_param`
exist at all. All three are members behind names rule 1 already
accounted for, so the roster after this unit is one entry shorter and
reports nothing about the three doors that closed it. The positive
form is `crates/pncad-py/tests/test_part_select.py`.

## Outcome

The family closed, and it took FOUR doors where the charter named
three things:

- `PartSelect.split_half(half)` / `PartSelect.instance(index)` — the
  selector pair, as a class of static constructors, which is
  `PatternKind`'s shape and what makes the name-for-name accounting of
  rule 1 true rather than convenient.
- `Node.part(of, select)`.
- `Node.pattern(input, count, kind)` — NOT in the charter, and
  required by it: `PartSelect.instance` selects out of a plural
  `instances` payload, exactly one node emits one, and that node was
  unbound. Binding the selector alone would have shipped an
  unreachable half and one unconstructible refusal tag.
- `DocEdit.bind_instance_param(node, name)` — `SlotId::Instance`'s
  door, `bind_count_param`'s sibling.

`crates/pncad-py/tests/test_part_select.py` is the positive form: 20
tests whose numbers are all oracles against the split's or the
pattern's OWN value (a Part's mass is the side's mass, read off
`Value.split` / `Value.bodies` rather than transcribed), the two
halves rejoined into the box through the flush protocol, the names
shown passing through verbatim, all four `wrong_operand` crossings,
both chartered refusals, and the memo's statement that an index edit
recomputes one node.

Census delta, exactly as predicted: `PartSelect` leaves `NOT_BOUND`
entirely under rule 1, the `B-PART` charter leaves `FAMILIES`, and the
closure paragraph records why a three-name charter had a one-row
roster — twice over, plus the door the charter never mentioned.

Three claims outside the census moved with it, and each had a test or
a sentence asserting the old state: `test_north_star.py`'s
`assertFalse(hasattr(Node, "pattern"))`, the stub's "deliberately
absent" list, and the audit page's G8 row. G8's CONCLUSION is
untouched — no scene flips and `PlacedUnion` is still what the heat
sink is authored with; what moved is the sentence that said binding
the pattern node would buy nothing, which stopped being true when its
consumer bound.

One fix taken rather than banked: `bind_count_param`'s prose claimed
`Count` was "the only structural slot there is", which four
Count-dimensioned slots contradict. One finding banked:
`work/lib/structural-slots-without-a-binding-door.md` (`VDegree` and
`Stations`, the two structural slots still without a door).

## Home

LIB's, filed by DOCM at DOCM-2's review (the Python surface is outside
DOCM's fence). Same class, same shape, unscheduled alongside it:
B-FACE-FRAME (`LIB-B-FACE-FRAME`), B-DISTRIBUTIONS, B-MEASURES,
B-NOTATION.
