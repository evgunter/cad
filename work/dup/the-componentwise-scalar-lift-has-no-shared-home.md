---
id: the-componentwise-scalar-lift-has-no-shared-home
kind: issue
title: Point3/Point2 componentwise T::from_f64 lifts are spelled out in eighteen places with no shared home
status: open
opened: 2026-09-19
priority: P1
cost: E
---


## Finding

Lifting an `f64` point or vector into a generic scalar lane is written
out component by component, `T::from_f64` per field, at **23 sites in
18 files** — measured 2026-09-19 at merge base `5b4979ef2`, with
`git grep` over every tracked file and no path argument:

```
git grep -c 'T::from_f64(p\.x)\|T::from_f64(p\.pos()\.x)\|T::from_f64(v\.pos()\.x)\|T::from_f64(vx\.pos()\.x)\|T::from_f64(foot\.x)\|T::from_f64(f0\.x)' -- '*.rs'
```

crates/editor-core/tests/cert3r1_dump.rs 1 · m10_p_fence.rs 1 ·
crates/geom-brep/src/edge_nurbs.rs 1 · tests/interior_iso_column.rs 2 ·
tests/interior_iso_review.rs 2 · crates/geom/tests/span_bit_identity_ext.rs 2 ·
crates/profile/tests/bool9r1_probes.rs 1 · cert4r1_e2e.rs 1 ·
common/mod.rs 1 · generic_replay.rs 1 · guided_replay.rs 1 ·
r2_bool9_review_probes.rs 1 · crates/topo/src/pcurves.rs 2 ·
crates/topo/src/test_support_fixtures.rs 1 ·
crates/topo/tests/fixture/mod.rs 2 · m6_3_chart_completion.rs 1 ·
review_ssiflat_r1_probes.rs 1 · review_ssiflat_r2_probes.rs 1

**Blind spot**: the pattern keys on the receiver's spelling (`p`, `v`,
`vx`, `foot`, `f0`), so a lift whose receiver is named anything else
does not match, and a lift written over a tuple or by destructuring
does not either. The count is a floor.

The last of those was minted by this program (the
`the-cylindrical-patch-rim-builder-is-written-nine-times` unit's
`lift_point`/`lift_vec`), which is the observation that opened the
row: a unit that closes a duplication minted the next one, exactly as
method item 5 says.

## The shape of a home

`geom_core` owns `Point3`, `Point2` and `Vec3` and owns `Real`, so the
home is there — `Point3::<T>::from_f64(p: Point3<f64>)`, or a small
`Lift` trait — and it is a production path, not test vocabulary. That
makes this a `geom-core` unit rather than a fixture one, which is why
the count above is worth re-taking before anyone writes it: a
production door serves the `src` sites as well as the `tests` ones.

## Why this row is not on `props`' slate

`scripts/work.py territory` reads `crates/geom-core/src/linalg/vec.rs`
as `props`'. The finding is one thing spelled many times, which is this
program's charter; it is filed here so the class stays with its
evidence, and `props` owns the file whenever it wants the row.
