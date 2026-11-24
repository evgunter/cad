---
id: three-refusal-sites-of-the-embedding-ask-are-reached-by-no-built-body
kind: issue
title: Two refusal sites of the embedding ask (by-kind, declined witness) are reached by no built body and are pinned at the engine only
status: open
opened: 2026-11-21
priority: P2
---


## Finding

From `embed/first-finding` (2026-11-21), corrected by its review
(2026-11-22). The embedding ask ends at its first refusal through one
rule (`Bound::refuse_below`, `crates/topo/src/clearance.rs`), read at
six sites. A mutant that made one site keep its refusal and read on
was run per site. Of the three this row first named as reached by no
body:

- **a terminal sliver** IS reached by a built body: two bricks 30ε
  across grafted 3ε apart (`topo::instance::graft_disjoint`) refuse
  `Sliver` on the two facing faces. Pinned
  (`two_bricks_a_band_apart_refuse_as_a_sliver_and_end_the_ask`,
  sweep's `self_intersection_proves_injectivity`).
- **a face its kind does not make injective** (`Sweep::run`'s candidate
  loop) and **a witness the verifier declines** (`Bound::settle`) are
  reached by no built body. Tried: a horn torus (revolve refuses
  `ArcCrossesAxis`); a lofted prism (its walls read `Embedded` on main
  `4d3402ecb`); a declining verifier on the plate whose void touches
  its top and on two ε-scale bricks overlapping by 10ε and 30ε (a
  depth refusal or a sliver comes first on the same level). Both are
  pinned at the engine instead, on a brick's windows
  (`clearance::first_finding_rows`): a window given a torus that is no
  ring, and a pair classed violating offered to a verifier that
  declines.
- **a poison enclosure off a shared cell** (`Bound::terminal`'s other
  arm) is reached by nothing: `window_of` refuses a face whose lifted
  values do not certify before the walk.

`injective::by_kind`'s free-form, approximated and spindle arms are
dead behind `window_of`, which refuses those faces before the walk, so
only its torus arm can reach the site. The engine row is a pin of the
site and of no body.

## What is owed

A built body for the by-kind refusal and for the declined witness, or
the argument that none exists. Until then the engine rows pin the
stop and the receipt at each; they do not show a body can read them.
