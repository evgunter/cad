---
id: a-reflex-corner-on-a-cube-edge-or-corner-refuses-in-the-vertex-vertex-lane
kind: issue
title: The L-prism's reflex corner on a cube's edge or corner, undeclared, refuses in 526 of 4 032 sweep runs (PairingMismatch, B senses agree, SelfLoopEdge)
status: open
opened: 2026-10-04
priority: P0
cost: H
---


## What

Found by the sweep of `a-pierce-with-two-out-runs-at-one-vertex-refuses-every-op`:
`crates/sweep/tests/join_pierce_runs_sweep.rs`, `pierce_runs_battery`.
Run it with `cargo test -p sweep --release --test all pierce_runs_battery -- --ignored --nocapture`.

The L-prism's reflex top corner `v = (1, 1, 1)` sits on a cube of
side 4, which is turned over a grid of 84 directions. Three
placements:

- `face`: `v` inside the cube's near face. This is the vertex-on-face
  lane, and the pierce row covers it;
- `edge`: `v` on an edge of that face, at four turns about the face
  normal;
- `corner`: `v` at a corner of the cube, at four turns.

The `edge` and `corner` placements reach the corner through the
vertex-edge and vertex-vertex lanes (`boolean/insert.rs`). Each runs
every op in both orders against an oracle that clips the prism's two
boxes by the cube's half-spaces. Nothing is declared.

Of 4 032 runs, 526 refuse (`edge` 368, `corner` 158). None ships a
`BAD` body. Main `f8aabaae` refuses one run more: `edge i=8 j=1
psi=1` prism ∩ cube, which the pierce PR builds `SOUND`.

| refusal | runs |
|---|---|
| `PairingMismatch` | 243 |
| `JoinDesync { "B senses agree at a matched pair" }` | 111 |
| `Euler(FanStartMismatch)` | 45 |
| `JoinDesync { "conflicting seam vertex correspondence" }` | 40 |
| `JoinDesync { "pair B edge has not exactly one surviving end" }` | 33 |
| `Euler(SelfLoopEdge)`, from the zip | 30 |
| `Join(UnpairedLooseEnds / Euler(NotSameFace))` | 24 |

`m3_pr6_saddle.rs` calls the F12 pairing guard unwitnessed on its
tilted-cube corpus. This sweep witnesses it 243 times.

## Related rows, not yet known to be the same cause

- cleave's `a-corner-crossing-another-four-times-refuses-pairing-mismatch`:
  the B-adjacency guard orders two germs of one B sector by their A
  sector;
- `four-germ-vertex-pairs-run-b-in-a-order`: parked, on declared
  flush poses;
- `a-reflex-vertex-and-its-partner-read-the-same-b-sense-along-an-edge-through-the-corner`.

## The shape to give

Sort the refusals by cause on a few representative poses, using
`pierce_runs_battery`'s lines. Several of them may be the vertex-vertex
lane's copies of the pierce row's class: several runs at one vertex,
or a seam passing a pinch vertex twice. Fold each cause into the row
that owns it, or give it its own row.

## Built

Branch `join/reflex-corner-vertex-vertex`. All 526 refusing runs build
`SOUND` (`pierce_runs_battery`: every one of the edge and corner
placements' 4 032 runs is `SOUND` or rightly empty, against the
clipping oracle). The face placement's 17 refusals
are the pierce rows'. Four wrong states, all in
`insert::plan_null_pairs` and its mints, each the first wrong state on
some of the 526:

- **The guard, not the pairing (`PairingMismatch`, 243).** F12 guard 1
  ordered the survivors in one B sector by their A sector, and the
  pairing ordered those in one A sector by their B sector (cleave's
  `a-corner-crossing-another-four-times-refuses-pairing-mismatch`). Each
  solid now walks its own orbit (`walk_order`): by sector entry, and
  round the entry by `walks_after`; two germs along one direction in
  one entry refuse typed. Each link is a simple closed curve on the
  sphere, and at four crossings both non-crossing matchings fix one
  cyclic order on both, so the guard cannot fire there on distinct
  germs. At six a nested matching is legal and it does fire, as on
  main (both reviews' notch poses; filed as
  `a-six-crossing-vertex-pair-nests-its-pairing-and-refuses-pairing-mismatch`).
- **B ran forward in A's order** (`four-germ-vertex-pairs-run-b-in-a-order`):
  each solid now runs a null edge from the germ the other follows in
  its own walk order (`run_order`). That row's fix, built here; its
  REST-zip wrong bodies do not reappear (`join1_r1_reflex_battery`, 0
  refusal→BAD).
- **A strut anchored on a half a fan had moved.** A fan and a strut in
  the two entries of one physical sector (the cube's edge vertex, a
  bisected half-turn): the fan minted first and took the half the strut
  splices beside, so the strut hung at the fan's copy (`JoinDesync`
  "B senses agree", `NotSameFace`, `UnpairedLooseEnds`). The runs of a
  pair that crosses more than twice are now shared (`SideRun::shared`):
  struts mint before fans, anchored off the sectors, as
  `reconcile_shared`'s pairs do.
- **The pairing start ignored the op.** Either start pairs the corner;
  one leaves a kept vertex of A holding both null edges, which the
  finish's seam correspondence and the zip refuse ("conflicting seam
  vertex correspondence", `Euler(SelfLoopEdge)`). The pairing now starts
  where A's runs lie on the side the op keeps of A
  (`finish::kept_side`).

Rows: `join_pierce_runs_sweep.rs` `four_germ_vertex_pairs_build_every_op`
(red under each of the four mutants), its subset row now asks `SOUND`
of every edge and corner run, and
`a_six_crossing_notch_corner_refuses_pairing_mismatch_on_distinct_germs`
drives the planner into the guard (red when a non-adjacent pair falls
to the run-swallowing test instead).
