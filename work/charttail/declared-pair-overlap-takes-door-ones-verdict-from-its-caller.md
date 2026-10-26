---
id: declared-pair-overlap-takes-door-ones-verdict-from-its-caller
kind: issue
title: topo::declared_pair_overlap takes Door 1's ContactVerdict from its caller: a public input that is a decision the door cannot check
status: open
opened: 2026-10-26
priority: P3
cost: M
---


Found by FLUSH's `flush/nary-union-door` sweep for public kernel inputs
that carry a decision back in (the class of
`work/flush/a-glue-verdict-is-not-bound-to-the-bodies-it-judged.md`,
whose instance, `BooleanInputs::given`, that branch made crate-private).

## What

`topo::declared_pair_overlap` (`crates/topo/src/chart_region.rs`) is
public, re-exported from `crates/topo/src/lib.rs`, and takes
`door_one: ContactVerdict`, the verdict Door 1 (the carrier ladder)
reached on the pair. That verdict gates the door: the planar
interior-witness rung runs only on `ContactVerdict::Definite`, and the
cylinder arm's premise budget tightens under `Bridged` (the function's
own doc). Nothing ties the value to the two faces handed in, so a caller
can pass `Definite` for a pair Door 1 never read as definite and unlock
the rung, which turns a refusal into a positive overlap.

The production caller is crate-internal (`crates/topo/src/pcurves.rs`,
two sites, each passing the verdict it read). The public callers are
tests (`crates/topo/tests/census_g2_carrier.rs`), which hand it chosen
verdicts on purpose.

## Owed

Either the door reads Door 1 itself from the two faces (one home for
that read), or the verdict-taking door goes crate-private beside a
public one that reads it, and the tests move to crate-internal tests or
to the reading door. No measured harm: no public caller outside tests.
Since INTENT's PR 5605 the rescue the verdict gates also runs where the
region walk escalates and on the shared-chart path, so the reach of a
caller's `Definite` grew with it.
