---
id: fit-subject-ungathered-has-no-producer
kind: issue
title: FitSubject::Ungathered is public with no producer since the at-rest gate retired, and no row schedules its retirement
status: open
opened: 2026-10-27
priority: P3
cost: E
---

Filed from stage 5 C's review (PR 5810).

`crates/viewer/src/evalseam.rs:591` `FitSubject::Ungathered` is a fit
subject for a document whose product nobody holds: the fit gathers one
on the worker. Its producer was the landing whose at-rest gate consumed
the gathered body. Since stage 5 C the landing always keeps the body
(`Gathered::body: Arc<Body>`), so no landing produces `Ungathered`; the
arm is public and tested at the seam only
(`crates/viewer/tests/eval_seam.rs:755`).

Done when the arm retires with its seam row, or a producer it is for is
named and wired.
