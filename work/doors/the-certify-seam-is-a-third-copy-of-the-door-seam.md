---
id: the-certify-seam-is-a-third-copy-of-the-door-seam
kind: issue
title: CertifyService is a third copy of the door/stager seam: three traits, three inline seams, three thread wrappers
status: open
opened: 2026-11-02
priority: P3
cost: M
---

Found by the style review of PR 6357 (finding 8).

`crates/viewer/src/evalseam.rs` now holds three identical seams over
different request and answer types:

- the traits `DoorService`, `StagedService`, `CertifyService` — four
  methods each (`submit`, `cancel`, `poll`, `busy`), docs word for word;
- the inline seams `InlineProver`, `InlineStager`, `InlineCertifier`;
- the threaded wrappers `ThreadProver`, `ThreadStager`,
  `ThreadCertifier`, each forwarding to a `Cancelling<R, D>`;
- and `app.rs`'s three cfg-armed constructors (`prover`, `stager`,
  `certifier`).

`Cancelling` already gives the threaded half one home; the trait and the
inline half have none. `FitService` and `IndexService` are the same
shape without `cancel`.

## Repair shape

One generic seam trait over `(Request, Done)` with one inline
implementation, the three seams type aliases of it, and one constructor.

**FORMS, PR 6431 (2026-11-03).** The staged seam is deleted (`StagedService`, `InlineStager`, `ThreadStager`, `Worker::Staged`, `SessionOp::Stage`): its one asker was the split panel's verdict, which went with the split form's typed plane. The door and certify seams are the two that remain.
