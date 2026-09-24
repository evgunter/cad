---
id: certification-doors-have-no-differential
kind: issue
title: no differential stands over the certification doors that are more than a delegate (hull, clamped_to, contains, width, mag)
status: open
opened: 2026-09-24
---


## Finding

Split out of `certification-value-hygiene-has-no-gate` (item 5, the
half RING-5 did not close). `crates/geom-core/tests/interval_backend_differential.rs`
compares `Interval`'s forwarded operators and the two delegate doors
(`Certification::sqr`, `Certification::powi`, both `Real::powi`)
against the backend, so its agreements hold by construction; its module
doc now says so (RING-5). The certification doors that add something of
their own — `Certification::hull` (NaI on any refused operand, where the
backend's hull absorbs the empty set), `clamped_to` (refusal first, the
enclosure's own decoration kept), `contains`, `width` (rounded up one
step), `mag` — are pinned by per-door rows
(`crates/geom-core/tests/certified_door.rs`, the `certification_door_tests`
module in `crates/geom-core/src/interval/certification.rs`), and nothing
compares them against an independent reading.

## Fix

A door-level differential: each of the five against a reference spelled
over the backend's endpoints and decoration (`DInterval` directly), over
the corner corpus the backend differential already has, asserting the
refusal verdict first and the endpoints bit for bit.
