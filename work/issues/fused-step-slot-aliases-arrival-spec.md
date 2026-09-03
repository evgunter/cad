---
id: fused-step-slot-aliases-arrival-spec
kind: issue
title: Fused-step slot addressing aliases the arrival spec's argument for Sweep/ArcLen/Bulge
status: open
opened: 2026-08-20
github: 829
---

## From GitHub issue 829

opened 2026-08-20, 0 comments.

`spec_slots` enumerates a role twice, and `step_expr` gives both slots the same expression, whenever a fused step's two arc specs are the same `Sweep`, `ArcLen` or `Bulge` mode.

**Where.** `crates/editor-core/src/program.rs` — `spec_slots` (the `second` axis) and the `spec_arg_access!` table.

**The shape.** The spec-2 role twins exist for the positional roles (`Center2X/Y`, `Via2X/Y`, `Target2X/Y`, `CarrierRadius2`) but *not* for `SweepVal`, `ArcLenVal` or `Bulge`. `spec_slots(spec, second = true, …)` therefore pushes the unsuffixed role:

```rust
(S::Sweep { .. }, true) => out.extend([A::CarrierRadius2, A::SweepVal]),
(S::ArcLen { .. }, true) => out.extend([A::CarrierRadius2, A::ArcLenVal]),
```

and `step_arg_access!`'s `ArcFilletArc` arm resolves an argument by trying the incoming spec first and only falling back to the arrival spec on `None`. So for `ProgramStep::ArcFilletArc { spec: Sweep{..}, radius, spec2: Sweep{..} }`:

* `slots()` lists `SweepVal` **twice** at that step;
* both entries address the **incoming** spec's `angle`;
* the arrival spec's `angle` is **not addressable at all** — `SetParam` / `SetExpression` / `Doc::expr_at` cannot reach it, and `slots()` claims otherwise.

Same for two `ArcLen` specs (`ArcLenVal`) and two `Bulge` specs (`Bulge`).

**Reachability.** Not from any recording surface: `family::ArrivalSpec` is implemented for `Center`, `Via` and `Radius` only, so `profile`'s typed algebra and the Python surface cannot put a `Sweep`, `ArcLen` or `Bulge` in second position, and `LoopProgram::from_recorded` therefore never builds one. But `ProgramStep`'s fields are public data by design (the node-slot pattern), so a hand-built or programmatically-generated program can represent it, and the document layer already treats hand-built programs as a supported construction (`RecordedProgramError`'s two "unreachable from the algebra" arms exist for exactly that reason).

**Reproduction.** Add the step to the corpus in `crates/editor-core/tests/switch_program_vocabulary.rs` and `every_enumerated_slot_addresses_a_distinct_expression` fails with

```
Profile { loop_: 0, step: 16, arg: SweepVal } addresses an expression another slot already addresses
```

**Why this is not fixed in the lane that found it.** The obvious fix adds `SweepVal2` / `ArcLenVal2` / `Bulge2` to `StepArg`, and `StepArg` is `serde::Serialize`/`Deserialize` inside `SlotId` — a persisted vocabulary. Adding variants is backward-compatible (no existing file names them) but not forward-compatible, and choosing that is a persistence decision rather than a style fix. The alternative — refusing the shape at the edit door instead of addressing it — is a different decision with the same owner.

Found by smell-scan Track G, lane G-f (row G7 / S106).

## Home

`crates/editor-core/src/program.rs` — the recipe/slot vocabulary — sits in no open program's `paths:` territory (SEAT owns `editor-core/src/verbs/*`, M10 the analysis lane, S-MATE `mate.rs`/`assembly.rs`), so it lands unowned under `work/issues/`.
