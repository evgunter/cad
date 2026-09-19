---
id: instantiate-part-crossings-are-names-payload-names-does-not-list
kind: issue
title: An InstantiatePart's crossing references are names payload_names does not list
status: review
branch: edit/instance-crossing-names
pr: 2872
opened: 2026-09-17
refs: [interface-crossing-heads-are-bare-stable-names, 2814]
---

(Found by the style review of PR 2814, which made an interface
crossing's two references `FaceName`s and so made it plain that they
are names.)

## The finding

`Node::InstantiatePart` is listed in `name_free_node!`
(`crates/editor-core/src/node.rs`, the macro at the top of the file) —
the pattern for "the [`Node`] variants whose payload REFERENCES no
[`StableName`]". It is not name-free. Every `InstantiatePart` carries an
`InterfaceRecord`, and every `InterfaceCrossing::Mate` in it carries
two names: `outer`, a REMAINDER name, and `inner`, a name in the
PART's own id space. Both are `FaceName`s as of PR 2814, so the type
now says out loud what the list denies.

Three consequences follow, each from the one list:

1. **`payload_names`' "single answer" claim is false for this
   variant.** Its doc (`Node::payload_names`, `node.rs`) says it is
   *"The single answer to 'which payloads carry a name': every reader
   reads this rather than its own copy of the list"*. For an
   `InstantiatePart` it answers the empty vector, so a reader that
   trusts it sees a node with no names where there are two per
   crossing.

2. **`Rebind` never reaches a crossing's `outer`.**
   `Node::rebind_payload_names` (`node.rs`) shares `name_free_node!`
   with `payload_names` — the read and the rewrite are one answer read
   two ways — so the `InstantiatePart` arm rewrites nothing. `outer` is
   a remainder name: it denotes a face in THIS document, on a node this
   document can delete or a name this document can rebind, and N5's one
   repair does not reach it. The mate's own heads are rebound (the
   `Node::Mate` arm), so after a rebind the record and the mate it
   records can disagree. (`inner` is a part-side name and correctly
   out of reach — it lives in the part's id space, not this one.)

3. **The insert door's liveness check never sees either reference.**
   `edit.rs`'s `InsertNode` arm loops `node.payload_names()` and
   refuses `DeclareNamesMissingNode` for a name whose node is not live
   — *"the ONLY door that checks, for every payload that carries a
   name"*. `Node::instantiate_part_with` is public, so a record naming
   a dead node inserts unrefused.

The same list is **spelled a second time** in `refactor.rs`'s crossing
walk (`split`, the `for &id in doc.order()` loop that collects
`crossings`), which knows perfectly well that a crossing carries two
names: it classifies each with `derivation_nodes(name)` against the cut
and remaps the part-side one. That is a second, independent answer to
"which names does this node hold" — exactly the shape
`document-stablename-carriers-have-no-enumeration` closed over for the
appearance store, one rung further in.

## Why this row and not the fix

PR 2814's fence was the record's TYPE. Listing `InstantiatePart` as a
name carrier is a behaviour change with three doors behind it — what
`Rebind` does to a crossing it can now reach, what the insert door
refuses, and whether `inner` must be excluded by name rather than by
the variant — and each wants a ruling before code. The record was
inert data until ASM-R2b D-4 inhabited it; it is document data now.

## Ruled and spec'd (2026-09-19, EDIT orchestrator) — middle tier, branch `edit/instance-crossing-names`

**Ruling: an instance's crossing `outer`s are payload names; its
`inner`s are not names in this document at all.** `Node::InstantiatePart`
leaves `name_free_node!` and gains its own arm in BOTH twins
(`payload_names` and `rebind_payload_names`, `crates/editor-core/src/node.rs`):
`payload_names` answers each crossing's `outer` in record order, and
the rewriting twin rewrites an `outer` exactly equal to `from`. An
`inner` is never listed, and the reason is stated on the arm rather
than left to the reader: it is spelled in the PART's id space (its
`node` is a part-side id, which this document may not hold or may
hold as an unrelated node), so the insert door's liveness check over
it would be a wrong check and `Rebind` a wrong repair; its life is the
pinned product's (`CrossingUnverified` at evaluation, A4). The
`payload_names` doc's "single answer" claim is thereby made TRUE by
scope: the list is of names in THIS document's name space, which is
the space every reader of it (the insert door, `Rebind`, DM7's strand
walk, `resolve`'s insert census) reasons in.

**Three doors follow from the one list, with no code of their own:**
the insert door refuses `EditError::DeclareNamesMissingNode` for a
record whose `outer` names a node that is not live (through
`Node::instantiate_part_with`, the public door); `DocEdit::Rebind` of
an `outer` rewrites the record together with the mate that carries the
same head, so the two cannot disagree after a rebind; and a delete of
an `outer`'s minting node reports `Maintenance::Strand { node:
instance, name: outer }` through `Doc::name_carriers` (DM7), the
instance being the surviving carrier. `content_key` already feeds the
record, so a rebound record re-keys the instance as any payload change
does — say so in the PR body, and pin nothing new for it.

**`refactor.rs`'s crossing walk is not a second list.** The walk in
`split` enumerates MATES (`Node::Mate { a, b, .. }` directly, because
it needs the sited structure to classify each side against the cut)
and mints the record; `inline`'s loop reads the record's `inner`s to
check the dissolve. Neither answers "which names does an instance
hold in this document", so neither is corrected; the PR body says
this in one paragraph so the row's third consequence is closed by
argument, not silently.

**Premises to verify before building.** (1) `name_free_node!`'s two
exhaustive matches: moving `InstantiatePart` out of the macro into an
explicit arm in both twins keeps both exhaustive with no overlap
(`-D warnings` reds an unreachable pattern — measure by leaving it in
the macro once). (2) `Doc::name_carriers` reads `payload_names`, so
DM7 needs no arm — confirm by the strand row below going green with
no change in `doc.rs` beyond its unit test. (3) The existing fixtures:
`edit_one_predicate.rs`'s record rows (authored through
`instantiate_part_with`), `asm_r2b_assembly.rs`'s three records,
`asm_r2b_interface_wire.rs`, and `fix_pattern_mate_crossing` (a split
that mints one) — each is the starting fixture for a row below; if any
`outer` in them names a node that is NOT live in its document, that
fixture was inserting an unchecked record and the row says which.
(4) `resolve/mod.rs`'s insert census (line ~1690) extends
`payload_names` — with the arm it now sees `outer`s; confirm that
census's rows stay green and what they measure.

**Rows** (each red on `origin/main` first, then green):
`payload_names` of an instance carrying `n` crossings lists exactly
the `n` `outer`s, in record order, and never an `inner` (`Vec::new()`
today); the insert door refuses `DeclareNamesMissingNode` naming the
dead `outer` for a record authored through `instantiate_part_with`
(inserts today); `Rebind` of an `outer` rewrites the record and the
carrying mate together (the record stays today) and a `Rebind` of an
unrelated name leaves the record byte-identical; deleting the
`outer`'s minting node reports one `Strand { node: instance, name:
outer }` in `Applied::maintenance` at the DM7 position (silent today);
the `doc.rs` unit test `name_carriers_reads_the_payloads_then_the_store`
gains the instance as a payload carrier; the persisted form of a
record is unchanged (bytes pinned against the existing wire row —
nothing here moves the wire). Existing rows: `fix_pattern_mate_crossing`
and `edit_one_predicate`'s record rows green unchanged.

**Mutants** (each named with the row that reds it): listing `inner`
too (the insert-door row over a record whose `inner.node` is not an
id in this document refuses for the wrong reason; the round-trip row
reds); the rewriting twin left in the macro (the rebind row); the
reading twin left in the macro (the first row, and the compiler if
the two twins disagree — say which); the strand row's instance
reported under the mate's id instead (the strand row asserts the
carrier is the instance).

**Sweep.** `name_free_node`, `payload_names`, `InstantiatePart`,
`name-free`, `crosses nothing` over `*.rs`/`*.md`/`*.py`/`*.pyi`,
every hit dispositioned; the prose lists of payload carriers in
`edit.rs`'s insert arm comment, `Node::payload_names`' doc, DM7's
clause text in `crates/editor-core/REFERENCES.md` ("`Node::payload_names`
stays the one list of NODE carriers" — unchanged and now true, cite it,
do not re-word it) and `ASSEMBLY.md`'s A4 paragraph (touch only if a
sentence is now false; say which).

**Territory.** `crates/editor-core/src/{node.rs, edit.rs, doc.rs}`
(EDIT); `crates/editor-core/tests/*` (TCOST/TINT); `refactor.rs` only
if a comment is now false (FIX, by announcement); `crates/pncad-py`
only if a binding enumerates payload names (LIB, by announcement).
Middle tier: one opus style review with a correctness arm, then the
fix pass.

## Built (2026-09-19)

Ruled as spec'd. `Node::InstantiatePart` left `name_free_node!` and
gained an arm in both twins (`crates/editor-core/src/node.rs`):
`payload_names` answers each crossing's `outer` in record order, and
`rebind_payload_names` rewrites an `outer` exactly equal to `from`
through `FaceName::map_derivation`. No `inner` is listed, and the arm
says why (the part's id space). No other code: the three doors follow
from the one list.

Six rows, five in the new suite
`crates/editor-core/tests/edit_instance_crossing_names.rs` (the list
in record order with no `inner` and `named_nodes` beside it; the
insert door's refusal over a dead `outer`, with the live control; the
rebind that moves the record and the carrying mate together; the
unrelated rebind that leaves the record alone; the DM7 strand carried
by the instance) and one in `doc.rs`'s
`name_carriers_reads_the_payloads_then_the_store`, which now holds an
instance as a payload carrier.

Premises: (1) confirmed — leaving `InstantiatePart` in the macro
alongside the new arms is `unreachable_patterns` at BOTH twins, an
error under `-D warnings`. (2) confirmed — `Doc::name_carriers` reads
`payload_names`, and the strand row went green with no change in
`doc.rs` beyond its unit test. (3) **fell**: four of the five existing
fixtures were inserting a record whose `outer` named a node that is
not live — `asm_r2b_assembly`'s `row5_b`/`row5_c`/`row6` reused the
part-side `inner` as the `outer`, and `asm_r2b_interface_wire`'s
`doc_with_a_crossing` named the instance being inserted. Each now
holds a plain instance whose remainder-side face the crossing keeps,
which is the shape a split mints; `edit_one_predicate`'s rows and
`fix_pattern_mate_crossing` were already correct and are unchanged.
(4) confirmed — `resolve`'s insert census extends `payload_names` and
now sees `outer`s; its rows measure resolution against an evaluation's
tables and stayed green (no row there authors a record).

Not built, filed instead: the crossing's third reference, its `mate`
id, is checked by no door —
`crossing-mate-back-pointer-is-checked-by-no-door`.
