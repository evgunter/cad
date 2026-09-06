---
id: select-refusal-predicate-names-are-unpinned
kind: issue
title: four of the five SelectRefusal.predicate names reachable from Python are pinned nowhere
status: open
opened: 2026-09-03
---


Residue of `python-refusal-tag-values-pinned-nowhere`, closed at
LIB-MECH1 with the tag-VALUE half taken and this half argued out.
Filed so the argument is not lost with the closed file.

## The channel

`predicate: &'static str` rides `SelectRefusal::{InBand, PairInBand}`
(`crates/editor-core/src/names/geompred.rs:202,253`), and
`crates/pncad-py/src/py/select.rs:676,748` projects it as
`SelectRefusal.predicate`. So a K predicate name is a
Python-visible stable string on the same footing as a refusal tag —
and unlike the tags, nothing enumerates it.

## What is reachable, and what is pinned

Five names reachable, one pinned:

* `InBand` — `SEL_DATUM_DISTANCE` = `"sel_datum_distance"`, minted at
  `crates/editor-core/src/names/geompred.rs:486`. **Pinned**, at
  `crates/pncad-py/tests/test_selectors.py:124` — in the Python
  suite, not on the no-interpreter row.
* `PairInBand` — `source.predicate.unwrap_or("carrier_pair_relation")`
  (`crates/editor-core/src/names/flush.rs:270`). The funnel sites it
  can carry are the C4 ladder's own, per carrier kind:
  `"bool_plane_parallel"`, `"bool_plane_orient"`,
  `"bool_plane_offset"`
  (`crates/topo/src/boolean/plane_eq.rs:203,225,242,274,282,301,311`),
  and, since SEAT-FW pointed the detector at the whole ladder,
  `"carrier_sphere_*"`, `"carrier_cyl_*"` and `"carrier_torus_*"`
  (`crates/topo/src/boolean/carrier_eq.rs:254,289,324` and their
  neighbours) — plus the `"carrier_pair_relation"` fallback. **None
  pinned.**

## Why the cheap fix is the wrong one

A one-line `assert_eq!(SEL_DATUM_DISTANCE, "sel_datum_distance")` was
considered and rejected at LIB-MECH1: it re-pins the only name already
pinned, closes none of the other four, and would read as coverage of a
contract it does not cover.

## What closing it actually takes

Neither arm is constructible from `pncad-py` —
`crates/pncad-py/src/tests.rs::select_refusal_tags_are_stable` already
records that `InBand`/`PairInBand`/`BadValue` carry funnel internals
with no public constructor. So a pin needs either real geometry driven
through the datum-distance selector and the flush detector inside the
no-interpreter tests, or a sweep of `topo`'s predicate-constant
namespace three crates away. That is the K-name-space job, which is
why it is an issue rather than a rider.
