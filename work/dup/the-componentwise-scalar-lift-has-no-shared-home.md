---
id: the-componentwise-scalar-lift-has-no-shared-home
kind: issue
title: Point3/Point2 componentwise T::from_f64 lifts are spelled out in eighteen places with no shared home
status: closed
opened: 2026-09-19
priority: P1
cost: E
closed: 2026-09-25
pr: 3242
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

## Closed (2026-09-25, PR 3242)

**The home already existed; no API was added.** `geom_core`'s four
leaf types each carry a structural `map` (`Point2::map`,
`Point3::map`, `Vec2::map`, `Vec3::map`, the last written as
`try_map`), `Point2::map`'s rustdoc already named `Real::from_f64` as
the exact case, and `crates/geom/src/scalar_lift.rs` states the
convention the tree follows ("`map_scalar` on every geometry type and
`map` on every leaf"). The tree already spelled lifts this way. At
`6db5b87f2`, `.map(<scalar>::from_f64)` or `.map(<scalar>::constant)`
appears 27 times in `.rs` files, on the leaves, `Affine3`,
`SketchPlane` and `ProfileVertex` (and some over iterators). A `Point3::from_f64` or `Lift` trait
would have been a second spelling of that one. So the door is
`p.map(T::from_f64)`. The unit added a rustdoc line naming it on
`Point2::map` and `Vec2::map`, and the unit rows it lacked:
`map_lifts_each_coordinate_into_its_own_slot` (points) and
`map_lifts_each_component_into_its_own_slot` (vectors), which pin each
slot's bits at `Interval` and `Dual64`.

**The census, re-taken.** Denominator first: every
`<path>::from_f64(`/`<path>::constant(` call whose argument (balanced
parentheses, whitespace dropped) ends in a component read (`.x/.y/.z`,
`[0..2]`, `.0..2` on a non-numeric receiver) or is a bare `x`/`y`/`z`,
over `git ls-files` with no path argument. Calls are grouped by file,
callee, receiver and read kind: distinct components within 200
characters. At `6db5b87f2` that gave 333 calls in 102 groups, 78 lone.
By kind: 55 field reads, 34 bare coordinates, 8 array rows, 3 tuples.
The row's receiver-named grep was a floor at 23. A second pass aimed at
the callee-name blind spot matched any callee `F` in `F(R.x), F(R.y)`,
which reaches aliases like `let iv = <Interval as Real>::from_f64`, a
`d` for `Dual64::constant`, and `Probe(..)`. It found 13 more member
groups.

**Folded: 50 groups at 68 call sites in 34 files**, in six crates: 37
field groups from the first pass and all 13 from the second. The
private helpers `lift_point`/`lift_vec` (`topo::test_support_fixtures`),
`lift_p`/`lift_v` (`geom/tests/curves/param_near_interval.rs`) and
`interval_plane` (`editor-core/tests/m10_p_lift.rs`, where the fold was
`SketchPlane::map`) are deleted. The shared helpers `ip`/`iv3`
(`geom-brep/tests/shared/interval.rs`, 20 callers) and the
`Step`-embed `pt`s keep their names, with the door as their body.

**Left, 18 field groups, each for a reason:** the target is not a
point: `geom-brep/src/edge_nurbs.rs` (two scalar `ders` arguments),
`geom-brep/src/ssi/jet.rs` ×3 (`Vec3Poly` of `Poly3`, not a `Real`).
The lift is deliberately non-uniform: `geom-brep/tests/cert_n2r2_class3_probes.rs`
×4 (`x` is poisoned on purpose). The site projects rather than lifts:
`curve3_r1_probes.rs` and `span_bit_identity_ext.rs` `lift2`,
`ders1_r2_probes.rs` `lift2`. The site is an ORACLE the door must not
be used for: `geom-core/src/linalg/affine.rs` `lifted_by_hand` ("calling
no walk"), `geom/tests/curves/n1r1_lift_probes.rs` ×3 and
`n1r2_lift_probes_interval.rs` (hand against composed `map_scalar`),
`profile/tests/{bool9r1_probes,r2_bool9_review_probes}.rs` (retired
walks kept verbatim). The 45 bare, array and tuple groups have no
point value to call `map` on. The 41 of them that build a point are
filed as `coordinates-lifted-into-a-point-are-spelled-per-component`.

**Re-taken after merging `main` at `4968e6862`**: the same 18 field
groups remain and no new member appeared. Three folded sites were
re-resolved onto `main`'s new vertex-and-bulge loop shape. One new
bare-coordinate group (`geom-core/src/real.rs`) went to the sibling
row.

**Reverse direction, not folded.** Lane to `f64` componentwise inside a
constructor gave 5 hits: 3 `Dual` channel projections in one
`unit_vec.rs` test, and 2 `SpanBox` reads that are not a point. These
are projections, not lifts, and below a row's worth.

**Proof.** The plant swapped `x`/`y` in all four leaf maps. It ran on
the fold head and on the pre-fold tree, each in 4866 rows (6
crates plus `topo`/`geom-core` libs, and `editor-core` scoped to the
three folded files). That gave 298 red and 212 red, with 86 red only
with the fold and 0 red only without it. The 86 are in `geom-brep` 4,
`geom` 16, `profile` 11, `sweep` 1, `topo` lib 16 and `topo` tests 38.
A `#[track_caller]` reach log found 65 of the 68 sites reached. Of the
other three, `r1_p2_probes` was proved reached by a reach-only run,
`m10_p_lift`'s was reached through `SketchPlane::map` (a `panic!` there
reds the row with the fold and not without it), and `n1r2_dump`'s is
dark behind an env var. A shift of `x` by one, run on both trees, reds
8 more rows only with the fold (`intersect_table`, `offset_mint`,
`m5_pr5_tilted_cut`). The lifted values at seven sites stayed green
under both answers and are filed with S-TINT as
`lifted-fixture-values-reached-and-asserted-by-no-row`. The table is in
the PR.
