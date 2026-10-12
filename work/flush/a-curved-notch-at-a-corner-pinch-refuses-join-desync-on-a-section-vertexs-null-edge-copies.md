---
id: a-curved-notch-at-a-corner-pinch-refuses-join-desync-on-a-section-vertexs-null-edge-copies
kind: issue
title: A curved notch at a corner pinch refuses JoinDesync: a section vertex's null-edge copies have not exactly one kept end (15 lines, 5 poses)
status: dispatched
opened: 2026-10-11
priority: P2
cost: M
refs: [4249]
branch: flush/pinch-residue
---



## What

Found re-measuring PR 4249 review r1's curved-notch probe
(`a-curved-face-at-a-shared-pinch-vertex-builds-bodies-the-census-and-the-reuse-check-refuse`).
The probe is now `sweep` `curved_notch_pinch::curved_notch_pinch`
(ignored, release): `notch343` with its face `v → (2, 1.15)` bowed
into an arc (bulge tan 2.5°), against the two-cube corner pinch of
`pinch_runs_battery`, every op in both orders, plain booleans.

On main `6e3985cb7`, 15 of its 1 512 lines refuse
`JoinDesync { what: "a section vertex's null-edge copies have not
exactly one kept end" }`, raised in `crates/topo/src/boolean/finish.rs`
`discarded` (the several-copies arm: "a vertex several runs of one
pierce cut"):

| pose (i j k) | lines |
|---|---|
| 0 3 2 | ab U, ab S, ba U, ba S |
| 1 6 0 | ab U, ab S, ba U, ba S |
| 3 1 0 | ab U, ab S, ba U, ba S |
| 6 1 0 | ab I, ba U, ba S |

On main `875e049a` the same probe gave 11 of these; pose `3 1 0` was
`ClassificationInvariant` there, and PR 4249's hang moved it on to
this refusal. Replay one with
`CURVED_POSE="0 3 2 ab U" cargo test -p sweep --release --test all
curved_notch_pinch_one -- --ignored --nocapture`.

## Open

Not root-caused. Whether the twin edge joins no pair of the vertex's
copies, or more than one, was not read. The planar batteries'
pinches do not reach this arm; the curved wall at `v` does.

## Not the pinch weld's ground (2026-10-11, branch `flush/pinch-cluster`)

The pinch cluster priced this row against its fix and left it. That
fix mints the pierces of one face at one point as one ring
(`vtxfac::mint_ring`) and retires `finish::weld_pinches`. The
`curved_notch_pinch` probe is byte-identical across it (1 513 lines,
release; the base side is the branch built with a switch that restores
the per-pierce rings and the weld), and so are its 15 `JoinDesync`
lines. This probe's pinch is
the two cubes' vertex-vertex pinch at the notch's corner. It has no
coincident pierces, and `discarded`'s several-copies arm reads the
null-edge copies the vertex-vertex insertion leaves, which the ring
does not reach.
