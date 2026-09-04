---
id: prose-gate-has-no-mechanical-guard
kind: issue
title: the prose gate is enforced only where someone remembered: no row renders every Display-reachable refusal at its struct-shaped payload variants
status: open
opened: 2026-09-04
---


Cut by the FIX orchestrator from PR 1779's style review, which found
**two live instances** of a panic that unit had just closed once. The
point of this item is that a fourth, fifth and sixth instance are
cheaper to prevent than to find.

## The gate, and how it is enforced today

`crates/pncad-py/src/errors.rs`'s `reads_as_prose` rejects a message
containing the field-brace fingerprint `" { "`, and `py::typed_err`
asserts it on **every** raise — live under release, since the root
manifest keeps `debug_assert` on. So a kernel refusal whose `Display`
renders a struct-shaped payload through `Debug` does not degrade: **it
panics the binding**, where the arm meant to refuse gracefully.

Enforcement is a `debug_assert` at the raise. That catches an instance
**when someone runs the door that raises it**. Nothing enumerates the
doors.

## Three instances, one fix

- `ValidationError::UndeclaredContact` / `StaleContactDeclaration` —
  panicked on the first honest call of `validate_pseudomanifold`;
  closed by PR 1779.
- `BlendError::Escalated { site: BlendSite }` — `Link` and `Joint`
  carry braces; a fillet or chamfer escalation panics. **Live.**
- `StepImportError::Placement` / `Instance` — `{source:?}` on types
  that already have a `Display`. **Live.**

`errors.rs:376` already carries the general warning in prose: *"A
future STRUCT variant of that kernel enum would trip this assertion and
panic where that arm means to refuse gracefully."* The warning is
correct, it is written down, and it did not stop three instances.

## What this unit builds

A row that, for every refusal type reachable through `typed_err`,
constructs each **struct-shaped payload variant** and asserts
`reads_as_prose` on the rendered message.

The hard part is the enumeration, and it is the reason this is a unit
rather than a chore:

1. **A hand-written roster re-creates the defect.** `blend/mod.rs:1195`
   is the proof — a `seeds()` list that looks exhaustive over
   `BlendError` and samples `Escalated` with `BlendSite::Chain`, the one
   brace-free variant. A roster that picks its own samples excludes the
   failing mode by construction. Whatever this unit builds must not be
   another such list, or it will pass for the same reason.
2. **The resolver sweep is the method that works**, and PR 1779's lane
   ran one: every `impl fmt::Display` in `crates/`, resolving each
   `{ident:?}` to its declared field type and asking whether that type
   is brace-shaped. 370 sites, 32 brace-shaped. Its stated blind spots
   are the scope question here — chiefly the **51 positional `{:?}`
   sites it could not type at all**.
3. **Reachability, not just shape, decides severity.** A brace-shaped
   payload that never reaches `typed_err` is cosmetic; one that does is
   a panic. The two live instances were found by tracing the raise path,
   not by matching the rendering.

Whether the guard is a test, a lint, or a `#[test]` over a derived
roster is the unit's to decide. What it may not be is a list somebody
maintains by hand.

## Scope note

The two live point fixes are **not** this unit's: `blend` is FILLET's
ground and `step-import` is EXCH's, filed together at
`work/issues/debug-in-prose-at-blend-and-step-import.md` and routed. A
live panic on a public door should not wait on a test. This unit is what
stops the fourth one.
