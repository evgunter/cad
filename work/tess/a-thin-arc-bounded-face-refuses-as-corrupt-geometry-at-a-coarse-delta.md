---
id: a-thin-arc-bounded-face-refuses-as-corrupt-geometry-at-a-coarse-delta
kind: issue
title: a valid thin arc-bounded planar face refuses tessellation as corrupt geometry at a coarse chordal delta
status: open
opened: 2026-10-06
priority: P3
cost: M
---



Filed by CARVE (PR 4187's sweep), outside its fence, and re-measured
at that PR's review.

## What

A C-shape (a counterclockwise region bounded by one convex 350° arc of
radius 1 and a clockwise polyline at radius `1 − w`, the `c_shape`
family of `crates/sweep/tests/m5_s10_face_sense.rs`), extruded by 1,
validates and certifies at tier 3. `mesh::tessellate(body, δ, tol)`
then refuses some `(w, δ)` with `TessellateError::Triangulation`. Its
sentence reads "the CDT rejected a point insertion on a face — a
non-finite or out-of-range chart coordinate, i.e. corrupt geometry",
and the body is not corrupt. Measured 2026-10-06 (`Tol::witness()`,
`check_mesh` on every mesh that built):

| w \ δ | 0.5 | 0.1 | 1e-2 | 1e-3 |
|---|---|---|---|---|
| 0.1 | Ok | Ok | Ok | Ok |
| 0.06 | Ok | Ok | Ok | Ok |
| 0.04 | Ok | **refused** | Ok | Ok |
| 0.02 | Ok | **refused** | Ok | Ok |
| 0.01 | **refused** | **refused** | Ok | Ok |

The PR 4187 reviewer measured the same refusals on base, so the cap
orientation change does not cause them. The result is not monotone in δ.
No mesh that built failed `check_mesh`, and in particular none failed
`MismatchedWinding`. The chord polygon's winding, which this row first
suspected (`chart_frame`'s Newell over the chord walk), did not fail
at any width tried: the π/4 cap on a chord's sweep (`chords.rs`) bounds
the sagitta, which keeps the chord polygon's area sign through w = 0.01.

## Suspected cause (not confirmed)

The outer arc's chords cut across the inner boundary wherever their
sagitta exceeds the region's width, so the CDT receives crossing
constraint edges. If so, the refusal is an honest "this δ cannot mesh
this face", worded as corrupt geometry, and the fix is either a δ
the face's own width bounds or a refusal that names the width and
offers the δ that meshes it.

## Also a thin annulus whose circles' vertices are opposed (JOIN, `join/annular-tube-roles`)

Found by both reviews of PR 4397 and measured on its fix pass. The
extruded annulus `r ∈ [1 − w, 1]`, `z ∈ [0.5, 2.5]`, is two one-segment
circles with their one vertices at azimuths 0 and `ai`. The tube
alone, before any boolean, gives `mesh::tessellate(body, δ,
Tol::witness())`:

| w, `ai` | δ 0.05 | δ 5e-3 | δ 1e-3 |
|---|---|---|---|
| 1e-2, 0 | refused | Ok | Ok |
| 1e-2, π | refused | Ok | Ok |
| 1e-3, 0 | Ok | Ok | Ok |
| 1e-3, π | refused | refused | Ok |
| 1e-4, 0 | Ok | Ok | Ok |
| 1e-4, π | refused | refused | Ok |

"refused" is `TessellateError::Triangulation { face }`; every `Ok`
passed `check_mesh`. As in the table above, the result is not monotone
in δ (`w = 1e-2` refuses at 0.05 and `w = 1e-3` with aligned vertices
does not). `crates/sweep/tests/an_annular_tube_through_a_plate.rs`
`thin_nested_rings_build` pinned the refusal at δ 5e-3 on all six ops of
three nested rings of width 1e-3, 9e-4 and 1e-4, until they meshed (re-measured below).

## 2026-10-11 — lens faces at a curved corner pinch (FLUSH)

`sweep` `curved_notch_pinch::curved_notch_pinch`
(ignored, release; a notch with one face bowed into an arc of bulge
tan 2.5°, against a corner pinch, plain booleans) on main
`6e3985cb7`: 37 of 1 497 bodies refuse `mesh::tessellate` at δ = 0.05.
All have the oracle's volume and pass tier 2 and the certificate. 36
are `Triangulation` on a planar face with no rings, and one is
`MissingEntity` "degenerate trimmed boundary". On the faces read, the
planar face is a sliver bounded by the arc and a straight cut. Two
are two-corner lenses with `v` as one corner (`2 6 0 ab I`, `0 1 0
ab S`). One is the bottom cap, cut by a straight edge near the arc
(`9 3 2 ab U`).

Over δ ∈ {0.05, 0.02, 5e-3, 1e-3}, on those 37 lines (replay:
`CURVED_POSE="i j k order op" cargo test -p sweep --release --test all
curved_notch_pinch_one -- --ignored --nocapture`):
- 3 lines (`2 6 0 ab I`, `ab S`, `ba I`) refuse `Triangulation` at
  every δ down to 1e-3, and 5 more down to 5e-3;
- most build at some finer δ, then refuse `CertificateExceeded` at
  1e-3;
- 2 build at 0.02 and fail `check_mesh` (`0 1 0 ab S`, `6 1 0 ab S`).

This is not monotone in δ, as in the C-shape table above. A mesh that
builds and then fails `check_mesh` is worse than a refusal.

## 2026-10-12 — four more lens-face refusals at the curved pinch (FLUSH)

FLUSH's pinch-residue lane (branch `flush/pinch-residue`) made 15
`curved_notch_pinch` lines build that refused `JoinDesync` on main
(the discard read a section face passing `v` twice). Four of the new
bodies refuse `mesh::tessellate` at δ 0.05 with this row's signature,
`Triangulation` on a planar face with no rings, with the oracle's
volume, tier 2 and the certificate: `0 3 2 ab S`, `0 3 2 ba S`,
`1 6 0 ba S`, `6 1 0 ba U`. Same replay as above.

## 2026-10-15 — a capped rod's cap lunes beside a crossing rod (TANG)

Two unit rods at right angles, A about `z` over `[dz, 2]` and B about
`y` over `[−2, 2]`. On main and on `tang/pinch-u2a-factored-crossing`
alike, A ∪ B and A ∖ B refuse `Triangulation` on A's cap face at
δ = 2e-3 for dz ∈ {3e-4, 1e-3, 1e-2}, and at δ ∈ {2e-4, 2e-5} where
those were tried (dz = 5e-4). They build at dz = 0.1, and ∩ and B ∖ A
build at every dz. B's strip `|x| ≤ √(1 − dz²)` leaves the cap disc
two lunes about `dz²/2` wide and `2·dz` long, each bounded by the rim
arc and one straight cut: this row's sliver. The certified volumes
match the closed forms. The row
`crates/sweep/tests/bisector_nodes.rs::an_edge_near_a_node_crosses_through_the_bisector_planes`
holds its cap poses to the certified bracket alone, citing this file.

## Re-measured (2026-10-28, main `ed45c3c8`, TESS thin-rings lane)

This re-measurement follows PR 5717 (`pred/mesh-sizing-rule`, merge `463956a6`),
which sizes each mesh lane at the coarsest step its own theorem grants. The
measurement uses a throwaway probe that was not committed: the C-shape at
inner radius `1 − w`, extruded by 1, then
`mesh::tessellate(body, δ, Tol::witness())` and `check_mesh` on every mesh
that built.

| w \ δ | 0.5 | 0.1 | 1e-2 | 1e-3 |
|---|---|---|---|---|
| 0.1 | Ok | Ok | Ok | Ok |
| 0.06 | Ok | Ok | Ok | Ok |
| 0.04 | Ok | Ok | Ok | Ok |
| 0.02 | Ok | Ok | Ok | Ok |
| 0.01 | **refused** | **refused** | Ok | Ok |

Against the 2026-10-06 table, `w = 0.04` and `w = 0.02` at δ 0.1 now
mesh. `w = 0.01` still refuses at δ 0.5 and 0.1, as
`Triangulation { face }`.

The JOIN tube alone, re-taken with the same probe:

| w, `ai` | δ 0.05 | δ 5e-3 | δ 1e-3 |
|---|---|---|---|
| 1e-2, 0 | Ok | Ok | Ok |
| 1e-2, π | Ok | Ok | Ok |
| 1e-3, 0 | Ok | Ok | Ok |
| 1e-3, π | Ok | Ok | Ok |
| 1e-4, 0 | Ok | Ok | Ok |
| 1e-4, π | Ok | Ok | **refused** |

Six cells moved from refused to Ok. One cell, `w = 1e-4` with opposed
vertices at δ 1e-3, moved from Ok to **refused**, so the result is still
not monotone in δ. No mesh that built in either table failed `check_mesh`.

`thin_nested_rings_build` now holds all six ops of the three nested thin
rings to `check_mesh` at δ 5e-3. Their mesh volumes match the closed
forms within 1.8e-5 at every width.

These sections were not re-measured: the FLUSH curved-pinch lines, the
TANG capped-rod lunes, and the `bisector_nodes.rs` comments that cite this
row ("does not triangulate at any chord tried").
