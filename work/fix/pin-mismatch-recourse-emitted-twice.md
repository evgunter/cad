---
id: pin-mismatch-recourse-emitted-twice
kind: issue
title: Refusal text - the pin-mismatch recourse is emitted twice; Contradictory and NoAtRestRecord emit no recourse sentence
status: closed
opened: 2026-08-23
github: 947
refs: [938]
closed: 2026-09-03
---

## From GitHub issue 947

Opened 2026-08-23; 0 comments.

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

## Closed

**Finding 1 only.** The `PartResolver for Workspace` arm that
re-appended `PIN_MISMATCH_RECOURSE` is gone
(`crates/pncad/src/workspace.rs`): the message is now the
`WorkspaceError`'s own `Display`, unaltered. That is safe because
`Display`'s `PinMismatch` arm ends on the recourse UNCONDITIONALLY —
verified at the arm before the deletion — so the arm was never a
fallback for a message that lacked the sentence, only a second copy of
it. The argument, and the coupling it rests on (a future edit that
drops the recourse from `Display` silently stops this door carrying
it, and nothing on this side would catch that), is left as a comment
where the arm was.

Both armed pins were flipped in the same change:

- `demos/tour/src/assembly.rs::update_door` — the count is 1, the
  `GAP (#947)` block is rewritten to say why 1 is the number with
  meaning on both sides, and the `println!` that announced the
  doubling to the demo's reader is replaced by one that reads the
  refusal out loud once. Per `memories/demo-purpose.md` a demo's
  awkwardness is never hidden — but this awkwardness is gone, so
  keeping the note would have printed a false statement to a user.
- `crates/pncad-py/tests/test_assembly_author.py` — the 2-versus-1
  contrast is rewritten as the claim that now holds: the recourse
  reaches the caller through BOTH doors (the evaluation's classified
  message and the store's own `WorkspaceError`) exactly once each. The
  test counts rather than `assertIn`s, because the failure it pins is
  a DUPLICATE, not an absence.

Every other `PIN_MISMATCH_RECOURSE` site was read and left alone:
`crates/pncad/tests/all.rs`, `crates/viewer/tests/instance_authoring.rs`,
`crates/pncad-py/tests/test_assembly_eval.py`,
`crates/pncad-py/tests/test_workspace.py`,
`crates/pncad-py/src/py/store.rs`, `crates/pncad-py/src/py/mod.rs`,
`crates/pncad-py/pncad.pyi`, `docs/guide/assembly.md` and
`docs/guide/north-star-audit.md` assert or describe only that the
message ENDS ON the recourse, which is still true and is now the whole
truth.

**Finding 2 is NOT closed here.** It is carried out of this file and
re-filed as its own issue by the orchestrator, because it is authoring
new diagnostic prose in kernel crates rather than a mechanical repair.
Its substance, quoted so nothing is lost if this file is swept:

> The demo's refusal walk prints four typed refusals. Two of them tell
> the author what to DO: `MateFault::Under` ends on `UNDER_RECOURSE`
> ("add the complementary mate, or delete the mate if free relative
> motion was intended"); `WorkspaceError::PinMismatch` ends on
> `PIN_MISMATCH_RECOURSE`. Two do not:
>
> - **`MateFault::Contradictory`** — "mates 3 and 5 cannot both hold:
>   predicate `mate_member_translation_zero` measured a clash of
>   0.010000000000000009 m where their cosets would have had to meet".
>   Names both mates and the measured clash, which is the diagnosis;
>   says nothing about the repair (delete one of the two, or re-author
>   the one whose datum is wrong).
> - **`AssemblyError::NoAtRestRecord`** — quotes the class table's
>   reason (a tangency's record is a `CurveContact` keyed by a witness
>   edge, and an assembly at rest has none) and ends "the record is not
>   minted with an invented witness". That explains the refusal; it
>   does not tell the author that the way to declare this contact today
>   is a `Rest`, or that curved contact verification at rest is R3/M9
>   work.
>
> Both are honest and both name their subject, so this is polish rather
> than a defect — but the ASM ladder's own exit criterion is
> "everything outside v1 refuses typed **with recourse text naming its
> rung**", and for these two the rung is not named.
