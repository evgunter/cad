---
id: payload-carrier-lists-have-seven-prose-homes
kind: issue
title: The payload-carrier list is hand-copied into seven prose homes
status: closed
closed: 2026-09-19
branch: edit/prose-one-home
pr: 2879
opened: 2026-09-19
refs: [instantiate-part-crossings-are-names-payload-names-does-not-list, 2872]
---

(Measured by `edit/instance-crossing-names` (PR #2872), which added ONE
variant to `Node::payload_names` and had to hand-extend seven separate
prose lists to keep them true. The unit did not make the class; it
measured it.)

## The finding

`Node::payload_names` (`crates/editor-core/src/node.rs`) is the code's
single answer to "which payloads carry a name", and its `match` is
exhaustive, so the ANSWER cannot drift. Its PROSE can: seven separate
places spell the list of carriers out in English, each hand-maintained,
and none of them is derived from or checked against the match.

The seven, as the unit found them:

1. `Node::payload_names`' own doc comment — the carrier list in its
   first paragraph (`crates/editor-core/src/node.rs`).
2. `edit.rs`'s `InsertNode` arm comment, above the
   `for name in node.payload_names()` loop — the D3 carve-out's list
   ("Declare pairs, a BLEND's selection, a SHELL's ordered open list,
   …").
3. `edit.rs`'s `Rebind` arm site list, in the comment above the
   payload rewrite.
4. `resolve/mod.rs`' "Checked sites" list, above
   `apply_with_names`' insert census.
5. `doc.rs`'s `Carrier::Payloads` doc ("a `Fillet` selection, a
   `Declare` pair, a `Mate` head, a `Measure` ref, an instance's
   crossing `outer`").
6. `crates/editor-core/REFERENCES.md` §0's `Carriers:` list.
7. `crates/editor-core/tests/dm7_delete_strands.rs`' module-doc
   carrier list.

An eighth site is not prose but a second MATCH:
`dm7_delete_strands.rs`' hand-spelled "name-free kinds" arm, which
PR #2872 found stale and corrected. That one at least fails loudly
when it is wrong about a variant it names; the seven do not fail at
all.

## Why it may matter

A prose list that has gone stale reads exactly like one that has not.
The class is the one `document-stablename-carriers-have-no-enumeration`
closed over for the store: an answer with more than one home. Here the
authoritative home exists and is enforced — what is unenforced is every
restatement of it, and the cost is paid by whoever next adds a carrier
(the unit that filed this paid it seven times, in one PR, with no tool
to tell it whether it had found them all).

## What a fix might look like (not a ruling)

Candidates, in rough order of cost: (a) delete the restatements and
point at `Node::payload_names` from each site, keeping a list only
where a reader genuinely cannot follow a link (the `.md` pages); (b) a
`scripts/gates/` check that parses the match arms and greps the seven
homes for the variant names, failing on a variant present in one and
absent from the other; (c) generate the doc list from the match with a
macro, which buys the doc comment and nothing else.

Ruling wanted before code: which of the seven are load-bearing
documentation and which are decoration.

## Ruled and spec'd (2026-09-19, EDIT orchestrator) — E-class, branch `edit/prose-one-home` (shared with `doc-param-refusals-keep-two-conventions-inside-one-enum`)

**Ruling: two of the seven are load-bearing; the other five point.**
Load-bearing: (1) `Node::payload_names`' own doc — the code's home,
beside the exhaustive match that enforces it; (6) `REFERENCES.md`
§0's `Carriers:` list — the design page a reader without the code
reads. Every other prose restatement (2, 3, 4, 5, 7) is decoration
that can rot silently: each becomes one sentence that says what the
site DOES with the list and points at `Node::payload_names` for what
is in it ("every payload name — `Node::payload_names` is the list"),
with no variant names. The eighth site, `dm7_delete_strands.rs`'
hand-spelled "name-free kinds" arm, is a second MATCH: replace it
with the door's own answer (`node.payload_names().is_empty()`, or
whatever predicate the suite actually needs), so the suite cannot
disagree with the crate about a variant. No gate script and no macro
(candidates (b) and (c) buy a tool for a list that now has two homes).

**Rows.** None new; `dm7_delete_strands` green unchanged after its
arm is replaced (say which rows exercised the arm). **Sweep.** Every
site the row lists, plus a grep for two adjacent carrier names
(`Declare` pairs … `Mate` heads, `blend's selection`, `shell's open`)
across `crates/editor-core/src`, `tests`, `*.md`, for an eighth prose
home the unit did not count; disposition each hit.

**Territory.** `crates/editor-core/src/{node.rs, edit.rs, doc.rs,
resolve/mod.rs}` (EDIT), `REFERENCES.md` §0 untouched (already
load-bearing), `crates/editor-core/tests/dm7_delete_strands.rs`
(TCOST/TINT, by announcement). E-class: merges on green CI and the
orchestrator's read; no review lane.

## Built (2026-09-19, `edit/prose-one-home`)

The list has two homes and five pointers, as ruled.

- **Kept, with the two-homes rule stated on it**: `Node::payload_names`'
  doc (`node.rs`) and `REFERENCES.md` §0's `Carriers:` clause. Each now
  says the other is the only other home, so a new carrier is two edits
  and a third list is a deletion rather than a maintenance job.
- **Pointed** (variant names gone, one sentence each saying what the
  site does): `edit.rs`'s `InsertNode` liveness comment and its
  `Rebind` rewrite comment; `resolve/mod.rs`' "Checked sites";
  `doc.rs`'s `Carrier::Payloads`; `dm7_delete_strands.rs`' module doc.
  `edit.rs`'s `structural:` comment beside the `Rebind` record named
  two carriers to say why the edit is structural and now names none.
- **The eighth site, the second MATCH** in
  `dm7_delete_strands::every_payload_kind_that_carries_a_name_reports_its_strand`:
  the hand-spelled name-free arm is replaced by
  `other => assert!(other.payload_names().is_empty(), …)`. The suite
  no longer holds an opinion about which variants are name-free; it
  asks the crate. The carrying arms stay hand-written — deriving them
  from `payload_names` would make the row tautological.
- **Correction to the row's premise**: `REFERENCES.md` §0's list, one
  of the two ruled load-bearing homes, was itself STALE — it omitted a
  `Shell`'s open list and a `Datum::FaceFrame`'s face, two carriers
  `Node::payload_names` has had throughout. Both added. The clause is
  catching up with code already merged, not deciding anything.
- **Ninth prose home, found by the sweep and NOT taken**: the
  `every_payload_kind_that_carries_a_name_reports_its_strand` row's own
  doc comment, a second carrier list in the same file the row counted
  once. Rewritten to describe the fixture and point at the match.

**Rows exercised.** The replaced arm is reached only from
`every_payload_kind_that_carries_a_name_reports_its_strand`, whose
fixture's blocks are name-free nodes; all 19 `dm7` rows green
unchanged. No new row: the change adds no behaviour to pin.

**Not done.** No gate script and no macro, as ruled.

## Closed (2026-09-19, EDIT orchestrator)

Built and merged as PR #2879 (E-class: green CI and the orchestrator's
read, together with `doc-param-refusals-keep-two-conventions-inside-one-enum`).
The list has TWO homes — `Node::payload_names`' own doc beside its
exhaustive match, and `REFERENCES.md` §0's `Carriers:` clause, each
naming the other as the only other home — and every other site says
what it does with the list and points at the door: the five the row
counted, a sixth the lane found (`edit.rs`'s `structural:` comment)
and a NINTH prose home in `dm7_delete_strands.rs`'s own row doc. The
eighth site, the suite's hand-spelled name-free match arm, now asks
the crate (`other => assert!(other.payload_names().is_empty())`),
trading compile-forced exhaustiveness — which did not catch the
measured failure, a list that was exhaustive and wrong — for an arm
that cannot be wrong that way. One premise corrected: the
load-bearing `REFERENCES.md` list was itself stale (a `Shell`'s open
list and a `Datum::FaceFrame`'s face were missing) and was completed
as a clause catching up with merged code (last enumeration change an
implementer PR; no ratification found). LIB's `rebind` docstring keeps
a partial list as the ruling's own exception (a Python reader cannot
follow a link into the crate). No gate script, no macro. Territory
crossed by announcement: `dm7_delete_strands.rs` (TCOST/TINT).
