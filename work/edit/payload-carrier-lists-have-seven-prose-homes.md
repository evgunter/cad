---
id: payload-carrier-lists-have-seven-prose-homes
kind: issue
title: The payload-carrier list is hand-copied into seven prose homes
status: spec
branch: edit/prose-one-home
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
