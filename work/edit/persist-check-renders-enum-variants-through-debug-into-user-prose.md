---
id: persist-check-renders-enum-variants-through-debug-into-user-prose
kind: issue
title: persist/check.rs writes {arg:?} and {slot:?} into user-facing sentences, so a variant identifier reaches the reader
status: open
opened: 2026-09-15
---


(S-TINT orchestrator, 2026-09-15) Filed on EDIT's slate because
`crates/editor-core/src/persist/check.rs` is EDIT's territory
(`work.py territory`). Surfaced by the style review of S-TINT's TINT-1,
which was hardening the F6 display-contract guards in
`crates/editor-core/tests/`; this is the same defect one level away, in
`src/`, where that suite has no row on it.

## The finding

The project's F6 rule, as `crates/editor-core/tests/display_contract.rs`
states it in its own header: an error arm must *"state what happened in
prose — and must never read as the `Debug` struct dump"*, and *"the
variant identifier and the field-name punctuation are the dump's
fingerprints"*.

Two arms in `persist/check.rs`'s `Display` impl put a variant identifier
into the sentence:

```rust
} => write!(
    f,
    "loop {loop_} step {step}'s {arg:?} argument needs {} {expected} \
     expression, got {} {found}",
```

```rust
} => write!(
    f,
    "slot {slot:?} needs {} {expected} expression, got {} {found}",
```

`slot` is `SlotId` (`crates/editor-core/src/node.rs`), whose variants are
`Origin(Axis3)`, `Normal(Axis3)`, `Direction(Axis3)`, `U(Axis3)`,
`V(Axis3)`, `Profile { .. }` and the rest — so the rendered sentence
reads `slot Origin(X) needs a Length expression, got a Scalar`. The
identifier and its `Debug` payload are both in the user's line. `{arg:?}`
is the same shape one field down.

## Why it has survived

**No row covers either arm.** The F6 suite pins seven error enums in
`display_contract.rs` and `HitTestError` in `m4_pr4_hit.rs`; this
`Display` impl is in neither, so nothing asserts the absence of a variant
identifier here and nothing ever will by accident. TINT-1 has just made
the covered enums' guards exhaustive over their variants, which makes the
uncovered impls the remaining exposure rather than a hypothetical one.

It is also invisible to the sweep TINT-1 ran: that keyed on the ban-list
SHAPE (`dumps`, `for dump in`, `contains("<Capitalised>")`), and a
`Display` impl with no test at all matches none of those patterns. A
different instrument finds this class —
`crates/pncad-py/src/prose_census.rs` already reads every `{binding:?}`
inside every `impl Display` in the tree — and **whether that census
currently flags these two sites, or silently permits them, is the first
thing to check.** If it permits them, that is a second finding and a
bigger one.

## What is NOT claimed

That either rendering is wrong for the user. `slot Origin(X)` is
arguably readable, and EDIT may judge that a `SlotId` is a name the reader
should see. **The claim is only that nothing decides it**: the tree has
a stated rule about variant identifiers in error prose, two arms that
appear to break it, and no row that would notice either way. Deciding is
EDIT's; S-TINT's interest ends at the observation that the guard class
does not reach here.

No fix is proposed and nothing is scheduled on EDIT's behalf.
