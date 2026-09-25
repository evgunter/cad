---
id: the-quad-sheet-helper-is-written-three-times-across-two-chart-region-files
kind: issue
title: The xy-plane quad-sheet helper is written three times across chart_region.rs and chart_region_r2_probes.rs
status: closed
opened: 2026-09-19
priority: P4
cost: E
closed: 2026-09-24
branch: dup/topo-fixture-batch
pr: 3152
---


## Finding

- **Where**: `crates/topo/src/chart_region.rs` (two copies, in two
  separate `mod` blocks: `xy_plane`/`sheet` at `:4151`/`:4162`, and
  `xy_plane`/`sheet` again at `:4858`/`:4879`) against
  `crates/topo/src/chart_region_r2_probes.rs`
  (`face_of` `:20`, `xy_plane` `:36`, `sheet` `:51`). `face_of` is a
  fourth spelling, at `chart_region.rs:3695` and
  `chart_region_r2_probes.rs:20`.
- **Importance**: low-medium
- **Confidence**: sure by reading; not dumped
- **Raised by**: the PR 2843 fix pass, 2026-09-19, out of the
  `find_half_edge(seed.face` sweep run for
  `the-cube-sequence-is-written-five-times-and-twice-inside-src`. Three
  of that sweep's 26 hits are these, and they are **not** cubes, so they
  belong to a different class and get their own row rather than riding
  that one.

## The shape

`sheet` is one `mvfs` at a rectangle corner, three `mev_line`s round
the rim, a `find_half_edge(seed.face, e_cd.vertex, e_bc.vertex)` and one
`mef` closing the quad onto a plane — a one-face sheet, not a solid, and
so neither a `prism_ops` body nor a member of the cube family. The two
`chart_region.rs` copies differ from each other in exactly one place:
the first takes its `FaceSurface` as an argument, the second inlines
`FaceSurface::New(xy_plane())`. `chart_region_r2_probes.rs`'s is the
first of those with `tol` threaded as a parameter instead of
`Tol::witness()` inline.

`xy_plane` is a bare `newell_plane` over the unit square's four corners,
written out three times.

## What this row is NOT

It is not "hoist them into `test_support_fixtures`". A single-face sheet
is a tier-1/2 chart fixture, and the file it would move to is the
**Euler-op fixture family**, whose doors all build closed bodies with
certified chords. The candidate homes are the two `chart_region.rs`
`mod` blocks merging their pair, and `chart_region_r2_probes.rs`
naming it from `chart_region`'s test module (or the reverse) — which is
a visibility question this row has not measured.

## What is unmeasured

- Whether `chart_region_r2_probes.rs` can name a `#[cfg(test)] mod`
  item of `chart_region.rs` at all, and at what cost.
- Whether the two `chart_region.rs` copies are in the same `cfg` arm.
- `face_of`'s two copies were read, not diffed.

## Re-census (2026-09-24, `dup/topo-fixture-batch`; taken at `6db5b87f2`, re-taken at `1d5922f1b`)

The definition census below was re-run at `1d5922f1b`: the same 24
helper definitions in the two files.

**Instrument**: every `fn` definition in the two files, by name, in
every `mod` block, read against each other; then the bodies built,
`{:?}` compared. **Scope**: `chart_region.rs` and
`chart_region_r2_probes.rs`, which is the row's scope.

| helper | copies | where |
| --- | --- | --- |
| `sheet` | 3 | `tests`, `inf_arms`, `r2_probes` |
| `xy_plane` | 3 | the same three |
| `face_of` | 2 | `tests`, `r2_probes` |
| `rect` | 4 | `tests`, `inf_arms`, `r2_mate8_probes`, `r2_probes` |
| `band` | 4 | `tests`, `inf_arms`, `inf_arms_interval`, `r2_probes` |
| `pt` | 3 | `tests`, `r2_mate8_probes`, `r2_probes` |

**Measured, not read**: `tests::sheet` at the unit square and
`inf_arms::sheet` build `{:?}`-identical bodies, as do `tests::sheet`
and `r2_probes::sheet` at the unit square and at the two-sheet shared
key configuration `r2_probes` uses; the three `xy_plane`s are
`{:?}`-identical. The row's *"`xy_plane` is a bare `newell_plane`"*
is not so: all three are a literal `Surface::Plane`. `r2_probes`'s
`tol` parameter was `Tol::witness()` at all nine call sites.

**The visibility question the row left open is free**:
`chart_region_r2_probes.rs` is `chart_region`'s child module (a
`#[path]` `mod r2_probes`), and every block involved is
`#[cfg(test)]`, so a `pub(super)` item of `tests` resolves from every
sibling block and from `r2_probes` with no gate change.

**Second pass, outside the row's scope**: `fn band`/`rect`/`sheet`/
`face_of`/`xy_plane` definitions over `crates/topo/src` return 18
further files, one or two each. Those are one- and two-line locals in
unrelated suites, not the quad sheet; not this row's class.

## Closed (2026-09-24, PR #3152)

All six helpers live once, in `chart_region`'s `tests` module, as
`pub(super)`; `inf_arms`, `inf_arms_interval`, `r2_mate8_probes` and
`r2_probes` import them. `inf_arms`' one-argument `sheet` became the
shared `sheet` at the unit square, with its reason ("the pipeline
needs a body only for the loop keys its refusals name") moved to the
call site. The PR body carries the plant table.

**Members the first pass left in the same files, folded in the fix
pass**:

| helper | copies | now |
| --- | --- | --- |
| `xy_plane_rotated` | `r2_probes`' fn and an inline `Surface::Plane` literal in `tests::r1_probes` | one `pub(super)` fn in `tests` |
| the `FaceUv` builder | `tests::uv_of`, `r2_mate8_probes::uv`, and closures in `tests`, `inf_arms` and `r2_probes` | one `pub(super) fn uv(outer, rings)` in `tests` |
| `flat_chart` | `inf_arms` at `f64`, `inf_arms_interval` at `Interval` | one generic `inf_arms::flat_chart::<T>`, corners lifted by `T::from_f64` as the `Interval` copy did |

**What the sheet's callers measure (plants V5/V6, re-run at
`1d5922f1b`)**: `sheet` is reached by 9 rows (a `panic!` in it reds
exactly those), and moving its corners — lifted 1e-3 off the plane,
or one corner moved 0.5 in plane — reds none of them. Those rows hand
the pipeline their own `uv` polygons and take only a face key and a
surface from the body. That is now stated at `sheet`'s rustdoc.
