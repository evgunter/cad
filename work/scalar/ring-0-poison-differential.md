---
id: ring-0-poison-differential
kind: unit
title: RING-0: the poison differential — ring poison ⇔ backend dec < Def per op, and the newtype dry run that names every dependent site
status: closed
opened: 2026-09-21
closed: 2026-09-21
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

## Closed (2026-09-21) — PR 2993

`crates/geom-core/tests/ring_interval_differential.rs` asserts the
VERDICT per op (`+ − × ÷ neg sqr powi`, exponents `-3…31`) against both
oracles before the endpoint skip, with a closed allowlist of four
input-predicated, direction-asserting classes (a zero times an
unbounded operand under `×`; unbounded over unbounded under `÷`; a
`powi` chain reaching zero and infinity, walked with the ring's own
`sqr`/`Mul` so `allowed == holds`; a negative exponent, where the
backend pads the base and refuses while the ring certifies — no
production ring value takes one) and an exhaustive corner sweep;
division agrees on every zero-touching divisor (~395k per lane). The
dry run on `scalar/ring-0-dry-run` (never merged): the newtype over
`DInterval`, surface kept, no caller edited — 22 red of 3,771 at
default (23 under the feature), sixteen tighter pins, zero looser,
zero division verdicts, one sign clamp, one overflow, three samples a
tighter bound stops dominating, one structural exactness; the
coefficient corpora 436/960 and 1,590/3,403 tighter, none looser
(ruling 2's number). The endpoint register in
`ring-nan-poison-is-load-bearing-at-unguarded-reads`: 31 hazards, 2
conditional on `from_certified` under cut (ii), 3 safe, with its
regenerating command; the red set conditional on the `hull` /
`clamped_to` guards; a real-endpointed `Trv` witness in
`certified_door.rs`. No `src` change merged. Reviews: dual, both
APPROVE WITH FIXES; one unilateral MAJOR by execution (the `powi`
hole) — the block's tally candidate; eleven dispositions, none
declined. Rows: `ring-nan-poison-is-load-bearing-at-unguarded-reads`,
`ring-2-red-rows-that-are-not-re-pins` (this slate).
