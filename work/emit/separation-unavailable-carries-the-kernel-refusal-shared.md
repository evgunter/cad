---
id: separation-unavailable-carries-the-kernel-refusal-shared
kind: issue
title: CheckEvidence::SeparationUnavailable carries a class beside a rendered string where Refusal<E> shares the kernel refusal whole
status: open
opened: 2026-10-02
---



Found by PR 3794's review (S1). That PR gave a shared refusal one home, `editor_core::Refusal<E>` (`crates/editor-core/src/refusal.rs`). It is an `Arc` with equality over the `Debug` structure, and `NodeRefusal` and `ProductRefusal` are now aliases of it. One spelling of the same need is left. It holds a document type (`CheckEvidence`, the separation resident's payload), so it is a payload change and was left out of 3794.

- `CheckEvidence::SeparationUnavailable { kind: topo::BooleanErrorKind, reason: String }` (`crates/editor-core/src/checks.rs`, built by `CheckEvidence::separation_unavailable`). It holds the kernel's `topo::BooleanError` as its class beside its rendered `Display`, because the error is neither `Clone` nor `PartialEq` and a report is both. `Refusal<topo::BooleanError>` would carry it whole: the class is `refusal.get().kind()`, and the sentence is the error's own.

Moving it changes a published report payload. Its readers are Python's `CheckFinding` projection (`crates/pncad-py/src/py/checks.rs`, the evidence's `kind`/`reason` attributes), the `checks.rs` unit test `the_separation_door_carries_the_class_of_the_error_it_saw`, and the payload-rung sweep's table. So the row is a unit of its own, with a machine-channel table in its PR.

The sweep that found this one matched `reason: String`/`message: String` fields in `crates/editor-core/src` and `crates/pncad/src`. Every other hit carries a resolver's or a store's own words (`PartFault::Unresolved`, `ResolveFailure`, persist and workspace refusals), not a refusal value this layer holds. It cannot see a class kept beside a string under another field name.
