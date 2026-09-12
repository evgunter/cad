---
id: LIB-MECH2
kind: unit
title: the second mechanical bundle — statement-based LB13 guards and the node-kind read door
status: closed
opened: 2026-09-06
branch: lib/mech2
pr: 2072
refs: [lb13-guards-are-line-local, pncad-py-doc-has-no-node-kind-read-door]
closed: 2026-09-08
---


Mechanical under the 08-29 ruling (no review lane, no A/B row;
brief-as-spec, the LIB-MECH1 precedent). Two members of the banked-findings
pile, taken by LIB-MECH1's selection rule: a finding whose fix is fully
specified by its own issue file, landing inside `crates/pncad*`, and
ending in a guard or a pin rather than in a design call. Both do.

## Delivered

**Both LB13 boundary guards read STATEMENTS.** `crates/pncad/tests/all.rs`
scanned line by line and required an arena key or `RawLoop` to share a
line with its `pub use`, while the façade's dominant idiom is the
multi-line brace list (33 of 75 statements, 17 of the 33 reaching
`editor_core::`). Both guards now match against a `pub use` accumulated
to its `;`, and the RawLoop guard's minting patterns against a
whitespace-squashed view of the whole file; both readers are
self-tested on the shape they exist for, and both report the line the
statement opens on. The issue's second instance — a doc sentence
claiming "twenty-six interior modules" — carries no count at all now,
and moved to the function it describes, which is not the one it was
attached to.

**`Doc.node_kind(node) -> str`.** `pncad-py`'s document surface answered
placement, reference and interface but never what kind of node an id
holds, so a Python row asking which node is the group had to read the
saved text. The word comes from one exhaustive match over the kernel's
`Node` with no wildcard arm — a kind added kernel-side without a Python
spelling is a compile error — and the whole 24-word vocabulary is pinned
against a roster committed beside it, on `TAG_INVENTORY`'s discipline,
with the stub read too. The die-tool row now counts kinds and asserts
`(1, 0, 0)`, the mirror of the Rust suite's row.

## Banked

`work/lib/facade-guard-file-keeps-two-line-local-readers.md` — three
readers in the same file still read a statement through a line, all
negative claims, none convertible mechanically. Carried out of the
sweep rather than swept.
