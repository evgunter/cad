---
id: crossing-mate-id-is-provenance-that-nothing-reads
kind: issue
title: An interface crossing's mate id is provenance nothing reads: checked at insertion today, deletable tomorrow
status: closed
closed: 2026-09-20
pr: 2906
branch: edit/crossing-drops-mate-id
opened: 2026-09-19
---


## The finding

`InterfaceCrossing::Mate { mate, class, outer, inner }`
(`crates/editor-core/src/node.rs`) carries the id of the mate that
crossed the cut. Since PR #2872's fix pass it is a PROVENANCE reference
the insert door checks live (`Node::payload_read_sites` lists it, so a
dangling id refuses `ReadSiteMissingNode`), and a later delete of the
mate is not reported. The id is read only by PROSE, and keyed
(memo-only, never persisted): `eval/wire.rs` carries it into
`NodeErrorKind::CrossingUnverified`'s message (`mate.0` is printed),
`crates/pncad-py/src/py/refactor.rs` republishes it as a `NodeId`, and
the instantiate arm of the content key feeds it (`h.write_u64(mate.0)`
in `eval/mod.rs`) — so two documents differing in the id alone key
apart, in a memo that no file holds. No door looks the node up, no
re-verification resolves against it, and the two references that do
the work are `outer` and `inner`.

## Why it may matter

A field that is checked but never read is a check with no reader to
protect. The more harmonious shape may be to DELETE the field: the
crossing then carries exactly the two references the seam needs, the
read-site arm and its fixtures go, and the refusal prose names the
crossing by its `outer` instead of by a mate id. What keeps it today is
the wire: `InterfaceCrossing` is persisted (`serde`, `deny_unknown_fields`),
so removing `mate` is a format change — a decision of its own, and the
persist schema is contended ground per `work/paths/program.md`'s
keep-out convention. Put to Ev on `[ev]` #2869's thread (2026-09-19)
as the one item where a nicer shape may exist.

## What closing it would decide

Whether a crossing's provenance is worth a persisted field; if not,
the wire change (a file that carries `mate` refuses or ignores it —
say which), the deletion of the read-site arm and its rows, and the
refusal prose re-worded to name the crossing by `outer`. Ground:
`crates/editor-core/src/{node.rs, edit.rs, eval/wire.rs, refactor.rs}`
(EDIT, FIX), `crates/pncad-py/src/py/refactor.rs` and `pncad.pyi`
(LIB, by announcement).

## Ruled and spec'd (2026-09-20, EDIT orchestrator) — middle tier, wave 15, branch `edit/crossing-drops-mate-id`

**Ruling: the field goes.** A crossing carries exactly the references
the seam needs — `class`, `outer`, `inner` — and no provenance. Ev's
answer on `[ev]` #2869 ("if there isn't a nicer way, sounds good")
left this the one item where a nicer shape exists, and it does: a
checked-but-unread field is a check with no reader to protect, and
the delete of the mate it names is already unreported (DM7's read-site
rule), so nothing observable is lost. No design page names the field
(`ASSEMBLY.md`'s A4 paragraph and AQ8 name the RECORD and the mate
EDGE, not its id) — nothing binding moves.

The spec, as premises (verify each against the tree before building):

1. `InterfaceCrossing::Mate { mate, .. }` (`crates/editor-core/src/node.rs`)
   loses `mate`. The one writer is `refactor::split`'s crossing walk
   (`refactor.rs` ~1595); the readers are `Node::payload_read_sites`'s
   `InstantiatePart` arm (`node.rs` ~3536 — the arm goes, and with it
   the two doc paragraphs above it that argue a provenance check),
   `eval/wire.rs`'s `CrossingUnverified` construction (~439, which
   prints `mate.0`), `doc.rs`'s fixture (~1471), and
   `crates/pncad-py/src/py/refactor.rs`'s republish (~101) with its
   `pncad.pyi` paragraph. `inline`'s dissolve (`refactor.rs` ~1959)
   reads `inner` only and does not move.
2. **The wire.** `InterfaceCrossing` is persisted under
   `deny_unknown_fields`, so a file carrying `mate` refuses at load
   with serde's unknown-field refusal — say so in the PR body, and
   whether that refusal is typed by the load door's census or is
   serde's own message (the EDIT-DECL precedent: an unversioned shape
   with nothing outside the tree persisting it moves with no
   migration; if any corpus or persisted fixture carries a crossing,
   re-author it and list it). The persist schema is PATHS's contended
   ground by keep-out: announce the exact lines.
3. **The refusal names the crossing by `outer`.** `NodeErrorKind::CrossingUnverified`
   carries and prints the remainder-side reference (the one the
   remainder's mate keeps) in place of a node id; its Display row in
   `display_contract.rs` moves with it; the Python tag map does not
   change (grep `crossing_unverified` in `tags.rs` and say).
4. **Rows.** Every row that measured the live mate-id check retires
   BY NAME with what replaces it (the record rows in
   `edit_one_predicate.rs`, `edit_instance_crossing_names.rs`'s
   mate-id rows, `asm_r2b_interface_wire.rs`'s wire rows): the
   replacement pins that a crossing round-trips the wire with three
   fields, that a file carrying a fourth refuses, and that
   `payload_read_sites` on an `InstantiatePart` yields exactly what it
   yields now minus the mate ids (the crossnames unit's exhaustive
   match keeps the arm's absence honest). The census in
   `crates/pncad-py/tests/test_binding_census.py` moves if it lists
   the field.
5. **Territory**: `node.rs`, `edit.rs`, `doc.rs` (EDIT); `eval/wire.rs`,
   `eval/mod.rs` (WIRE, by announcement); `refactor.rs` (FIX, by
   announcement); `persist/*` only if a codec names the field (PATHS,
   contended — announce); `crates/pncad-py` (LIB, by announcement);
   `crates/editor-core/tests/*` (TCOST/TINT). Middle tier: one opus
   style review with a correctness arm, then the fix pass.


## Built (2026-09-20)

**The field is gone.** `InterfaceCrossing::Mate` is `{ class, outer,
inner }`: a crossing carries what the seam needs and no provenance.

- `node.rs` — the variant loses `mate`; both doc rows (the
  `compile_fail` twin and its running twin) lose the line; a **No
  provenance** paragraph says why the field is absent.
  `Node::payload_read_sites` loses its `InstantiatePart` arm and the
  two paragraphs that argued a provenance check, and the variant
  joins the exhaustive empty group — an instance answers no read
  site.
- `refactor.rs` — `split`'s crossing walk writes three fields.
  `inline`'s dissolve reads `inner` and did not move.
- `eval/wire.rs`, `eval/mod.rs` — `NodeErrorKind::CrossingUnverified`
  carries `outer: Box<FaceName>` in place of `mate`, and the Display
  names the crossing by the reference the remainder keeps. The
  content key feeds class, `outer`, `inner` — the mate id is not
  data any more, so it is not keyed.
- `doc.rs`, `pncad/tests/all.rs` — fixtures re-authored.
- `crates/pncad-py` — the `mate` getter and its `pncad.pyi` paragraph
  go; `tags.rs` is unchanged (`crossing_unverified` and the crossing
  tag both match `{ .. }`). The binding census now lists
  `InterfaceCrossing::Mate` in `MEMBERS_BOUND_AS` as
  `InterfaceCrossing.variant`: the arm was accounted by the accident
  that the retired getter shared its snake-cased name.

**The wire.** `InterfaceCrossing` is `deny_unknown_fields`, so a file
carrying `mate` refuses at the load door as `PersistError::Unreadable`
— the door's own typed arm (serde_json classifies the failure `Data`;
`persist::parse_err` maps that class), with serde's sentence inside
it. No committed corpus or persisted fixture carries a crossing: the
four `.pncad` files in the tree hold no instantiate node at all.

**Rows.** Retired: `edit_instance_crossing_names`'
`the_insert_door_refuses_a_record_whose_mate_is_not_live` and
`deleting_a_crossings_mate_reports_nothing`, replaced by
`an_instances_record_answers_no_read_site` (empty read sites over a
two-crossing record, with a mate's two operands beside it as the
control). Added in `asm_r2b_interface_wire`:
`a_crossing_is_three_fields_on_the_wire` and
`a_file_whose_crossing_carries_a_fourth_field_refuses_at_the_load_door`.
`edit_one_predicate`'s literal wire pin and
`asm_r2b_assembly`'s `the_crossing_refusal_is_a_named_node_error`
moved with the shape; the latter now pins that the refusal names the
crossing by its `outer`.

**Premise corrections** — four, all reported in the PR body: the
Display row lives in `asm_r2b_assembly`, not `display_contract.rs`
(which has no `CrossingUnverified` row at all); `crates/pncad/tests/all.rs`'s
cross-process probe is a fifth construction site the spec did not
list; the binding census DOES move, though not for the reason premise
4 allowed (it never listed the field — it counted the ARM as spelled
because the retired getter shared the arm's snake-cased name); and
premise 4's other half, that no design page names the field, HOLDS as
written.

**AQ8.** `refactor::split`'s crossing walk is the field's only writer
and is unreachable today (`rev_fix_xsplit_unreachable`: a crossing
mate welds its ends into one cluster and `TornCluster` refuses cutting
through one), so the seam's write path stays hypothetical and the
round trip is pinned through the hand-built door,
`Node::instantiate_part_with`.

### Fix pass (2026-09-20)

The review's two probes are adopted authorship-preserving (merge of
`review/mateid-rv`, no cherry-picks): `asm_r2b_assembly`'s `row5_b`
now pins that the ONE construction site hands in `outer`, and
`a_crossing_record_keys_on_each_of_its_fields` pins that each
surviving field feeds the content key. Both mutants were re-applied
after the merge and each reds its row.

- **One home for the read-only wire walk.** `tests/wire/mod.rs` gains
  `wire_body` beside `doctored`, and both come off one `split_body`,
  so the read-only half and the surgery cut the file in the same
  place. `asm_r2b_interface_wire` imports it instead of re-inlining
  the split; the three suites the review named
  (`docm7_union_declare`, `asm_r2a_mate_solve`, `r2_m10_2_probes`)
  turned out to be copies of the WRITE walk, not the read-only one,
  and all three now call `doctored`. The suites that cut the same
  save by LINE rather than by brace are a second rule for one
  question and S-DUP's ground; filed as
  `work/dup/wire-surgery-header-split-is-spelled-nine-more-times-by-line.md`.
- **One home for the "no provenance" argument.** The variant's doc in
  `node.rs` argues it; `payload_read_sites`' group comment, the
  `pncad.pyi`/`py/refactor.rs` pair (one sentence, identically
  worded), the census comment and the two suite headers point to it.
- **The census's spelling rule is explicit.** A new roster,
  `ARMS_SPELLED_BY_A_PROPERTY`, lists every arm rule 1 accounts for by
  a same-named property, and a new row fails until a new one is
  written down. Five arms are accounted that way on the merge base —
  `InterfaceCrossing::Mate` among them, which is the accident that
  hid this field's arm — and four today. No binding moved.
- **The sentences.** The finding above says "read only by prose, and
  keyed"; the `Display` reads "…declaration crosses at the
  remainder's … and claims …"; the premise corrections are counted
  here and in the PR body alike.

## Closed (2026-09-20, EDIT orchestrator) — middle tier, one opus style review, merged on green CI

Built and merged as PR #2906. `InterfaceCrossing::Mate` is
`{ class, outer, inner }` — the references the seam needs and no
provenance; the refusal names the crossing by `outer`; a file carrying
the old field refuses typed at the load door under
`deny_unknown_fields` (no migration: the format is unversioned and no
committed corpus or fixture carries a crossing, measured); the
persist codecs were not touched. The review was MERGEABLE (0/2/3):
its two probes — the one refusal site handing in `outer`, and each
crossing field feeding the content key — are adopted
authorship-preserving; the fix pass gave the read-only wire walk one
home beside `doctored` (three suites the review took for read-only
copies were write-walk copies and now call `doctored`), the
"no provenance" argument one home, and the Python census a written
rule for a same-named property spelling an arm (an additive roster
and guard, four members listed, that would have named this accident
the day it landed — Ev's 2026-09-09 census rule untouched). The
"read by nobody" sentence was corrected: the merge base keyed the id,
memo-only. One row filed on S-DUP's slate for the by-line header
split spelled at sixteen more sites. The split walk that writes the
record stays unreachable (AQ8), said so on the row.

