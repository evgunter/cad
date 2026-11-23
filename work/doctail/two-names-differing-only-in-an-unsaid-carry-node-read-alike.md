---
id: two-names-differing-only-in-an-unsaid-carry-node-read-alike
kind: issue
title: Two names that differ only in a carry node the words never say read alike, even in full
status: open
opened: 2026-10-06
priority: P3
blocked_on: []
---


Found by PR 3886's fifth review (m1, its probe `e_silent_carries_in_one_table`), left as the ruling has it in the fix pass.

A name's words say neither a primary carry's node (a Boolean's A, a fillet's target: silent by the ruling on #3906) nor the node of a split's half, a pattern's copy, a band cut or a part qualifier (the wrap's words leave it unsaid; `names/words.rs`, module docs and `walk`). So two distinct names that differ ONLY in such a node read alike in every form, the full form included. The review's witness, by tag:

- `Merged[FromA(end cap of node 1) @ node 8, wall 1] @ node 3` and `Merged[FromA(end cap of node 1) @ node 9, wall 1] @ node 3` both read "the merged face of the end cap of node 1 and the side wall … of node 3" — and in ONE table, so `table_details` (`words.rs`) ends its loop with them still alike. It asserts that in a debug build (and the release profile keeps debug assertions), so a table that holds such a pair now panics loudly instead of saying two names alike.
- `SplitFragment@N9` and `SplitFragment@N10` of one cap both read "the part below the split of the end cap of …".

**Reachability.** No evaluation is known to mint such a pair in one table: a Boolean's merged constituents share its node (`names/emit_topo.rs`, the merged-face emission), and the corpus gate (`name_words_corpus.rs`, every table read apart in full and scoped) has never seen one. The full-form fuzzer (`name_words_rows::the_full_form_says_no_two_names_alike`) counts such pairs apart from real collisions, and the table fuzzer keeps them out of its tables.

**What closing it would take.** Either say the unsaid node where it is the only difference (a qualifier the table detail could open, like a citation: "carried at Fillet 92b0"), or show no node mints such a pair and pin that. The first changes the ruled silence, so it goes to Ev as a design question.

## Re-homed at RECIPE's close (2026-10-11)

Moved to DOCTAIL and parked on INTENT's `a-name-is-scoped-by-the-variable-holding-its-body` (dispatched). That unit scopes a name by the variable holding its body and drops the split half from a split's roles, which changes both what holds a name and which nodes a name leaves unsaid. Re-read this row against its merge before building anything.

## Re-read against the unit's merge (PR 4674)

The trigger fired. A split's roles no longer spell the half: each half is its own table at its port, and a fragment of a face the plane cuts is ONE name that both halves' tables hold. So the `SplitFragment@N9` / `@N10` witness above is no longer two names of one cap; the halves are told apart by the read, which a name's words never said. The Boolean witness (`Merged[FromA(...) @ node 8 …]` vs `@ node 9`) is unchanged by the unit and still stands as written.
