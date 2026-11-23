---
id: the-memo-hits-log-prefix-check-is-a-plain-assert-in-shipped-code
kind: issue
title: The memo hit's log-prefix check is a plain assert! in shipped code: a re-derivation check, which the failure taxonomy spells debug_assert!
status: closed
opened: 2026-11-23
closed: 2026-11-23
pr: 7512
---


Found by the member-read-key lane while re-reading `eval::eval_node`
against its keys (`a-family-members-read-of-another-member-is-served-from-a-prior-that-read-its-old-body`).
By reading; nothing executed.

`eval_node`'s memo hit (`crates/editor-core/src/eval/mod.rs`, the
`if let Some(v) = prior && v.content_key == content_key && …` arm)
finishes the node's fresh verdict frame and checks, with a plain
`assert!`, that the reused value's log opens with it ("a memo hit's
pre-key decisions differ from the reused log's prefix"). Its comment
says the identity is "asserted in every profile".

DESIGN `[failure.taxonomy]` has two mechanisms for a kernel bug: row 4,
`unreachable!`, where it is observable in a branch, and row 5,
`debug_assert`, where it is detectable only by re-derivation. This
check is a re-derivation (it recomputes a profile's pre-key decisions to
compare them with the stored ones), in every profile, by a mechanism
the table does not list.

What it guards is the claim this unit is about: that the key fixes
every input the pre-key decisions read. So the check has a use while
the key is under audit, and the choice is between `debug_assert!` (the
taxonomy's row) and a typed outcome where the two disagree (recompute
the node rather than serve it, and say so).

**Owed:** the taxonomy's mechanism for this check, or the reason it is
an exception written where the taxonomy is.

**Closed in PR 7512.** The check is a kernel bug's re-derivation: the
keys fix every input the pre-key decisions read, so no document
reaches a mismatch, and the taxonomy's row 5 spells it `debug_assert!`.
