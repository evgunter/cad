---
id: separation-unavailable-carries-the-kernel-refusal-shared
kind: issue
title: CheckEvidence::SeparationUnavailable carries a class beside a rendered string where Refusal<E> shares the kernel refusal whole
status: closed
closed: 2026-10-14
opened: 2026-10-02
priority: P3
cost: E
---

Found by PR 3794's review (S1). That PR gave a shared refusal one home, `editor_core::Refusal<E>` (`crates/editor-core/src/refusal.rs`). It is an `Arc` with equality over the `Debug` structure, and `NodeRefusal` and `ProductRefusal` are now aliases of it. One spelling of the same need is left. It holds a document type (`CheckEvidence`, the separation resident's payload), so it is a payload change and was left out of 3794.

- `CheckEvidence::SeparationUnavailable { kind: topo::BooleanErrorKind, reason: String }` (`crates/editor-core/src/checks.rs`, built by `CheckEvidence::separation_unavailable`). It holds the kernel's `topo::BooleanError` as its class beside its rendered `Display`, because the error is neither `Clone` nor `PartialEq` and a report is both. `Refusal<topo::BooleanError>` would carry it whole: the class is `refusal.get().kind()`, and the sentence is the error's own.

Moving it changes a published report payload. Its readers are Python's `CheckFinding` projection (`crates/pncad-py/src/py/checks.rs`, the evidence's `kind`/`reason` attributes), the `checks.rs` unit test `the_separation_door_carries_the_class_of_the_error_it_saw`, and the payload-rung sweep's table. So the row is a unit of its own, with a machine-channel table in its PR.

The sweep that found this one matched `reason: String`/`message: String` fields in `crates/editor-core/src` and `crates/pncad/src`. Every other hit carries a resolver's or a store's own words (`PartFault::Unresolved`, `ResolveFailure`, persist and workspace refusals), not a refusal value this layer holds. It cannot see a class kept beside a string under another field name.

## Closed

`CheckEvidence::SeparationUnavailable` is `{ refusal:
Refusal<topo::BooleanError> }` (`crates/editor-core/src/checks.rs`), and
`CheckEvidence::separation_unavailable`, the builder that kept the two
fields off one error, is gone: with one value there is nothing to keep
in step. The finding's prose is unchanged, since it read the error's
`Display` before and reads the same `Display` now.

The machine channel:

| Surface | Field | Before | After |
|---|---|---|---|
| Rust `CheckEvidence::SeparationUnavailable` | class | `kind: topo::BooleanErrorKind` | `refusal.get().kind()` |
| Rust `CheckEvidence::SeparationUnavailable` | sentence | `reason: String` | `refusal`'s `Display` |
| Rust `CheckEvidence::SeparationUnavailable` | the error | not held | `refusal.get(): &topo::BooleanError` |
| Python `CheckFinding.reason` | sentence | `str`, the kernel's sentence | unchanged |
| Python `CheckFinding.boolean_variant` | class | `str`, `boolean_error_tag` of the class | unchanged |

The three readers:

- pncad-py's projection (`crates/pncad-py/src/check_payload.rs`,
  `check_payload`) reads the sentence and class off the refusal; its
  tests (`crates/pncad-py/src/tests.rs`,
  `every_check_evidence_arm_projects_the_payload_it_carries`) build the
  arm from a `BooleanError` and pin `reason` as the error's own sentence
  and `boolean_variant` as `classification_invariant`.
- The `checks.rs` unit test is now
  `the_separation_door_reads_the_kernel_refusal_in_its_own_sentence`:
  the class-beside-sentence invariant it held is now the type's, so it
  pins the finding's rendering of the refusal instead.
- The payload-rung sweep's table (`tests/refusal_concision_chains.rs`,
  `check_findings`) builds the arm from the refusal it rendered before;
  its rows' text is unchanged.

The sweep: `SeparationUnavailable`/`separation_unavailable` across
`crates/` and `demos/`/`tools/` (`.rs`, `.py`, `.pyi`): the readers
above, `tags.rs`' doc (reworded), the `.pyi` doc and the binding census
(attribute names, unchanged). It cannot see a reader that matches the
arm by `..` and reads nothing; such a reader is unaffected.

