---
id: payload-carrier-lists-have-seven-prose-homes
kind: issue
title: The payload-carrier list is hand-copied into seven prose homes
status: open
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
