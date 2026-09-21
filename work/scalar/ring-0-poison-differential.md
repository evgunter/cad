---
id: ring-0-poison-differential
kind: unit
title: RING-0: the poison differential — ring poison ⇔ backend dec < Def per op, and the newtype dry run that names every dependent site
status: review
opened: 2026-09-21
branch: scalar/ring-0
pr: 2993
---

## What

`H5` ruling 1's acceptance for cut (ii) and RING-2's dry run: the
differential `crates/geom-core/tests/ring_interval_differential.rs`
gains a per-op VERDICT assertion (ring poison ⇔ backend `dec < Def`,
NaI or empty) with a closed, printed allowlist of characterised
disagreement classes and an adversarial corpus that reaches them; a
scratch newtype-over-`DInterval` branch (`scalar/ring-0-dry-run`, never
merged) whose red rows name every consumer that depends on a semantic
difference, classed (division verdict / clamp rules / overflow /
tighter / looser); the survey's 28 one-sided endpoint reads
dispositioned. Spec: `docs/RING-0-SPEC.md` (deleted at merge). Block
SCALAR-B5 slot 0. Ground: TCOST/TINT (`crates/geom-core/tests/*`);
announced.
