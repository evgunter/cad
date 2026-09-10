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

## What must not be inherited when this is fixed

Added by the correctness review of #2106, which audited the
bracket-depth item reader that exists to work around this defect.

`scripts/gates/viewer-vocab-declared-once.sh:139-153` says its own
reader should be deleted once this is fixed. When that happens, one
property of the workaround must travel with the fix rather than be
dropped: **the shared reader does not track `<>` either.** It cuts at
`{`, `}` and `;`, so a `const` generic parameter — `fn stack<const N:
usize>(…)` — presents an opening that a `const`-item matcher will take,
and the item then never closes at the right place.

The review reproduced both directions of that in the workaround
(a hand-written `const ALL` going green because a const-generic `fn`
sat three lines above it, and two false reds on the real tree with one
planted in `forms.rs`), and closed it there by anchoring the opening
pattern at an item position rather than allowing a bare `const`
anywhere in the line — verified population-preserving, since all 104
`const`-opening lines in the crate's code view are already anchored
that way.

So a `--items` mode added to `gate_rust_code` owes the same anchor. A
fix that removes the workaround without it re-opens a defect that was
found once, at the cost of finding it again.

## Citation correction, 2026-09-08

This file cited `scripts/gates/viewer-vocab-declared-once.sh:139-153`
for the workaround reader's header. That is the wrong passage — at
`origin/main` those lines are the roster-fidelity paragraph. **The
header is at `:212`**, re-derived by finding its subject (`THE ITEM
READER, AND IT IS A WORKAROUND`) rather than by shifting a number.

