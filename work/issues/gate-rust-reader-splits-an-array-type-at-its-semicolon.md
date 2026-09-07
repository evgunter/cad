---
id: gate-rust-reader-splits-an-array-type-at-its-semicolon
kind: issue
title: gate_rust_code --statements cuts a Rust array type at its semicolon, so a statement-anchored matcher sees a const list as a declaration with nothing after the =
status: open
opened: 2026-09-07
---


Found by VIEW's `view/all-gate` unit while writing
`scripts/gates/viewer-vocab-declared-once.sh`, and reproduced
independently by the VIEW orchestrator before filing. The subject is
`scripts/gates/lib.sh`, which is GATES' territory, so this is a report
rather than a change.

## The defect

`gate_rust_code --statements` documents itself
(`scripts/gates/lib.sh:318-324`) as *"one record per STATEMENT, cut at
`{`, `}` and `;`"*, on the argument that `{}`/`;` *"is where a generic
list and its `where` clause end"*. A Rust **array type** carries a `;`
that ends nothing:

    pub const BOOLEAN_OPS: [(BooleanOp, &str); 3] = [
        (BooleanOp::Union, "union"),
    ];

is cut into two records —

    pub const BOOLEAN_OPS: [(BooleanOp, &str)
    3] = [ (BooleanOp::Union, ""), ]

The first is a `const` declaration with **nothing after the `=`**; the
second has no `const` in it. A matcher anchored on a statement and
looking for a `const` whose initialiser is an array literal matches
neither record, and so reports the item as a declaration with no
initialiser.

## Why it matters beyond the unit that hit it

Three of the four hand-written membership lists under
`crates/viewer/src` are written this way, so a statement-anchored scan
saw three of four as empty declarations. The gate that found this
carries its own bracket-depth item reader instead, which is a
workaround in one gate and not a fix.

The same `;` appears in every array type, every `[T; N]` field and
every fixed-size buffer in the tree, so any future gate that reasons
over a statement containing one inherits the fault. The failure mode is
the quiet direction — a scan that finds nothing and reports OK — which
is the class `scripts/gates/lib.sh:113-141` already exists to prevent
for `grep`'s exit codes.

## Confidence

`sure`. Reproduced directly against `scripts/gates/lib.sh` at
`79e68d8d0` with the three-line source above; the two records printed
are quoted verbatim.
