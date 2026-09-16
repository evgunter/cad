---
id: persist-check-renders-enum-variants-through-debug-into-user-prose
kind: issue
title: persist/check.rs writes {arg:?} and {slot:?} into user-facing sentences, so a variant identifier reaches the reader
status: closed
opened: 2026-09-15
branch: edit/error-prose
pr: 2719
closed: 2026-09-16
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

## Built (2026-09-16)

**The measurement the row asked for first.** `prose_census.rs` does not
silently permit either arm, and the two are permitted differently:

- `{slot:?}` was **flagged and allowlisted** — `KNOWN_BRACED` carried
  `("crates/editor-core/src/persist/check.rs", "ProgramFault", "slot",
  1)` because `SlotId::Profile` is a struct variant, so the census's own
  question (can this payload's `Debug` carry `" { "`?) answers yes. The
  roster is compared in both directions, so the entry was a live
  allowlist, not a silence.
- `{arg:?}` was **named undecided** — `UNDECIDED` carried it with the
  reason that the binding is introduced by a pattern nested inside the
  field pattern the census reads (`slot: SlotId::Profile { .., arg }`),
  which is `work/fix/census-cannot-type-a-nested-pattern-binding`.

So the row's "if it permits them, that is a second and bigger finding"
does not fire as written. **The bigger finding underneath is real and is
already on a slate**: had the binding typed, `StepArg` is a FIELDLESS
enum, so `declaration_verdict` answers `Verdict::Prose` and the site
would have passed silently — the census asks about braces, never about
variant identifiers, which is
`work/census/prose-census-cannot-see-a-bypassed-prose-renderer`'s Gap 2.
Evidence added there rather than opened as a second row, including the
interaction worth knowing before either is taken: repairing the
nested-pattern row alone would move this site from a named blind spot to
a silent pass.

**The fix.** Both arms render through the prose doors that already
existed — `SlotId::label` and `StepArg::label`, which `range.rs` was
already using and which `edit.rs`'s own header named as the spelling it
was not using. No new door was minted. A user reads
`loop 1 step 3's centre x argument needs a length expression, got an
angle` and `slot origin x needs a length expression, got a scalar`.

**The guard.** `crates/editor-core/tests/display_contract.rs` gains
`a_program_fault_addresses_its_slot_in_the_slot_vocabulary`, a
`f6_variants!` census over `ProgramFault`'s two variants with four
cases, whose `also_banned` list is the `SlotId` and `StepArg`
identifiers read off the very values the cases carry
(`test_utils::f6::variant_identifier`), so it cannot fall behind a
rename. Verified red: reverting both arms to `{slot:?}`/`{arg:?}` fails
the test. That suite is S-TINT's claim on the cost/integrity dimension;
a guard for EDIT's own `Display` impl is EDIT's to add, and that is what
this is.

**What the guard deliberately does not ban.** The `Lattice` arm's
`{verb:?}` and `{state:?}`. That pair is the transition table's
coordinate, which `profile`'s `ReplayError` and this arm's own comment
both say, and `work/fix/verb-and-dimension-render-through-debug`'s
closing record explicitly holds the rule intact for
`ProgramFault::Lattice` while moving the viewer's sentence to `Verb`'s
new `Display`. Banning those identifiers here would be this suite
deciding a question settled the other way beside the code.

Landed with the other four `SlotId` sites in `editor-core` —
`EditError` (six arms), `ProgramRefusal`, `resolve::Diagnosis` — under
`debug-in-prose-residue-after-finding-sink`.

## Closed (2026-09-16, EDIT orchestrator)

Merged as PR #2719 on green CI (run 35047419997, full matrix) and the
orchestrator's read. Residue is in its own files, named in the Built
section above.
