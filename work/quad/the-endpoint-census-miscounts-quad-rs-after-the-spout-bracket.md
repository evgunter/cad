---
id: the-endpoint-census-miscounts-quad-rs-after-the-spout-bracket
kind: issue
title: certified_endpoint_census counts props/quad.rs at 10/4 against its pinned 7/2 on main after PR 5588; the slow-set row is red
status: closed
opened: 2026-10-22
priority: P1
cost: E
closed: 2026-10-26
pr: 5809
---


## What

`crates/geom-core/tests/certified_endpoint_census.rs:722`
`every_production_endpoint_read_is_counted_and_dispositioned` is red on
main. Its census of `crates/geom-brep/src/props/quad.rs` reads 10/4
against the pinned 7/2. That file on main is as PR #5588 (the teapot
spout bracket) left it. The row is in the slow set, so the per-PR gate
never ran it.

Found by INTENT's PR #5587 lane, in a full local run of `geom-core` with
the slow set. #5587 touches no file the census counts.

## Owed

Disposition the new endpoint reads in `props/quad.rs`, then re-pin the
census with their stated cause. Do not raise the pin blindly.

**Nightly (INTENT triage, 2026-10-23).** The row is red at every ε row
in the full suite from the 2026-10-23 nightly (run 39906156937, main
`be9b907694`), its first after #5588; the only file that moved is
`crates/geom-brep/src/props/quad.rs`, measured `(10, 4)` against the
pinned `(7, 2)`.

## 2026-10-24 — the count grew again (INTENT nightly triage)

Nightly run 40066431545 (main `3dfb96b91c`) reads `props/quad.rs` at
`(17, 7)` against the pinned `(7, 2)`. On 10-23 it read `(10, 4)`.
The census reads source, so one test binary built at `3dfb96b91c` was
run over each merge's tree:

| tree | `props/quad.rs` |
|---|---|
| `be9b9076` (10-23 nightly) | (10, 4) |
| `2300efe8` (#5589) | (10, 4) |
| `c8b398d2` (#5611, `quad/patch-doors-one-net`) | (14, 6) |
| `0ca666f4` (#5634, `iso/trimmed-composite-rounds`) | (17, 7) |
| `9180a67d`, `3dfb96b91c`, `7bfb66fc` | (17, 7) |

So #5611 (QUAD) added 4 reads (2 dispositioned) and #5634 (ISO) added
3 (1 dispositioned). Each new read still owes the census's question:
does it ask `is_certified()` by name, or is it argued safe at the site?

## Closed

PR 5809. The census was red on six rows, not one. `props/quad.rs` reads 17/7: #5582 added three reads (not #5588), #5611 added four, #5634 three, and #5635 rewrote one. Every read is dispositioned in the roster: each asks by name, or is safe by construction (point-bracket weights, a frame choice, the composite rule's cell location, a certified hull's width). No `quad.rs` code changed. FLUX's #5696, #5699 and #5734 moved `spiric.rs`, `rational_rim.rs`, `taylor.rs`, `quad_lane.rs` and `topo/src/props.rs`, which are re-pinned with their causes and seamed on FLUX's log. One read was unsafe: `deciding_walk` (#5628) confirmed a settlement off a re-derivation it never asked to certify. It now refines on an uncertified one.
