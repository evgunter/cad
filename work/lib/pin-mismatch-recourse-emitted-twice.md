---
id: pin-mismatch-recourse-emitted-twice
kind: issue
title: Refusal text - the pin-mismatch recourse is emitted twice; Contradictory and NoAtRestRecord emit no recourse sentence
status: open
opened: 2026-08-23
github: 947
refs: [938]
---

## From GitHub issue 947

opened 2026-08-23, 0 comments.

Two library-TEXT findings from the ASM-DEMO exit walk (#938), which reads every refusal it provokes out loud. Filed together because they are the same class — what a refusal actually says to the author — and both are cheap.

## 1. The pin-mismatch recourse arrives twice

`WorkspaceError::PinMismatch`'s `Display` already ends on `PIN_MISMATCH_RECOURSE`:

```rust
"… but the document hashes to {found} — {PIN_MISMATCH_RECOURSE}"
```

and `impl PartResolver for Workspace` appends it again when it classifies the failure for the kernel:

```rust
WorkspaceError::PinMismatch { .. } => format!("{e}; {PIN_MISMATCH_RECOURSE}"),
```

So every pin-mismatch message that reaches an evaluation carries the whole paragraph twice. Observed in the demo's update walk.

**The fix is at the `PartResolver` impl** (the store's own `Display` is the one that should carry it), but note the coupling: the impl's arm exists precisely so a caller who sees only the kernel-side `ResolveFailure::message` still gets the recourse, so deleting it needs the `Display` side to be the guaranteed source, which it is.

**⚠ The demo has an armed assertion on this.** `demos/tour/src/assembly.rs::update_door` asserts

```rust
assert_eq!(refused.kind.to_string().matches(PIN_MISMATCH_RECOURSE).count(), 2, …);
```

so it goes RED the moment this is fixed, by design — the count must be flipped to 1 in the same change. The assertion's message says so at the site.

## 2. Two refusals carry no recourse sentence at all

The demo's refusal walk prints four typed refusals. Two of them tell the author what to DO:

- `MateFault::Under` ends on `UNDER_RECOURSE` ("add the complementary mate, or delete the mate if free relative motion was intended").
- `WorkspaceError::PinMismatch` ends on `PIN_MISMATCH_RECOURSE`.

Two do not:

- **`MateFault::Contradictory`** — "mates 3 and 5 cannot both hold: predicate `mate_member_translation_zero` measured a clash of 0.010000000000000009 m where their cosets would have had to meet". Names both mates and the measured clash, which is the diagnosis; says nothing about the repair (delete one of the two, or re-author the one whose datum is wrong).
- **`AssemblyError::NoAtRestRecord`** — quotes the class table's reason (a tangency's record is a `CurveContact` keyed by a witness edge, and an assembly at rest has none) and ends "the record is not minted with an invented witness". That explains the refusal; it does not tell the author that the way to declare this contact today is a `Rest`, or that curved contact verification at rest is R3/M9 work.

Both are honest and both name their subject, so this is polish rather than a defect — but the ASM ladder's own exit criterion is "everything outside v1 refuses typed **with recourse text naming its rung**", and for these two the rung is not named.

— Claude (ASM-DEMO lane)

## Home

S-MATE's `keep_out` assigns this issue and the refusal-display prose to LIB, whose charter carries the library's user-facing surface.
