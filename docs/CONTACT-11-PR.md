CONTACT-11: the torus and cone chart-box checks decide in metres, linearly

Closes `work/contact/torus-chart-box-check-passes-an-l-shaped-face`.

## The defect

The torus and cone containment trims serve a face its chart bounding box
only after deciding that the face IS that box. Both decisions were wrong
for a notched face:

- **Torus** (`torus_chart_windows`, `bool_torus_chart_box`). At base the
  check compared total variation to `2·span` in each channel. An L is
  monotone in both channels, so it passed.
- **Cone** (`cone_trimmed_window`, `bool_cone_chart_box`), and the torus
  as this PR's first pass had it. The check compared the polygon's
  shoelace area with its box's, as `defect / span · lever`. For a notch
  of side `s` metres that margin is about `s²·lever / (ρ·span)`, so it
  is quadratic in the notch. A small notch therefore read Zero ("the
  face is its box"), and the door answered `In` at a point `s/2` from
  the face. One decade larger, the same notch escalated in band where it
  should have refused. The area check was not a sound precedent for the
  torus to copy.

## The fix

`solid_contain::chart_polygon_box` is the one decision both trims now
call. Every side of the boundary polygon is an iso segment, so the
polygon is its bounding box exactly when every side lies on one of the
box's four sides. The boundary is then a closed curve inside the box's
boundary, which a face's outer loop, walked once, covers.

- **What is decided.** For each side, the distance to each box side is
  the larger of its two ends' distances, crossed to metres. The side is
  accepted when one of those distances decides Zero.
- **Why the margin is linear.** A notch's sides lie inside the box, each
  as far from the nearest box side as the notch is wide or deep, so the
  margin grows linearly with the notch.
- **The arms bound the metric separation from above.** Zero is the
  accepting verdict, so each arm has to overstate the separation, never
  understate it. The site says so, per verdict:
  - torus: `R + r` per radian of major angle (a parallel's radius
    `R + r·cos t` is at most that); `r` per radian of minor angle (the
    arc `r·Δt` is at least its chord);
  - cone: slant is metres along a generator, exactly; azimuth is levered
    by the largest parallel radius, `max|slant|·sin α`.
- **The cone's apex jump** is the polygon's closing side, at `v = 0`,
  so it lies on the box's apex side. The first-pass L rows in
  `an_l_shaped_cone_face_refuses_rather_than_trim_by_its_hull`, the
  narrow one and the wide apex-closed one, still refuse.

`chord_join::chart_box_defect` and `ChartBox` have no callers left and
are deleted. There are seam notes in `work/reach/log.md` and
`work/tang/log.md`. `AzimuthImage` stays, since the cone's walks still
produce it.

**The torus walk's continuity.** The walk steps over null-scaffolding
edges, taking each to be a zero-length coincident copy. With the
variation check gone, a non-degenerate edge skipped in the middle of the
walk was invisible: the closure check compares only the last exit with
the first entry. The walk now also decides that each entry is the
previous exit, under `bool_torus_chart_closure`, through one helper
(`torus_chart_meets`) shared with the closure check. A gap anywhere
refuses as `PartialTorusFace`. The header's null-scaffolding paragraph
now says this instead of claiming the gap is caught downstream.

## Rows

The builders take the torus's `(R, r)`. The torus builder and
`cone_sheet` share one chain builder (`chain_sheet`): `mvfs`, then
`mev` along the chain, then `mef` to close. The rows are in
`section_cert_rows.rs`:

- `a_small_notch_in_a_torus_face_refuses_at_every_size`:
  - rings `(R, r)` ∈ {(2, 0.5), (10, 0.1), (1, 0.9), (100, 1)};
  - notch side `s` ∈ {1, 10, 100} × `10·K·ε`, plus the review's
    1e-6 / 1e-5 / 1e-4 m wherever they clear that floor;
  - shapes: a corner L, a centred U, and two thin Ls (`s` by `π/8`
    each way);
  - each is asked at the notch's centre and must answer `Ok(None)`:
    never `In`, never an escalation.
- `a_small_notch_in_a_cone_face_refuses_at_every_size`:
  - frusta [1, 2] and [10, 20] on the unit cone, azimuth [0, π/4];
  - shapes: a corner L, a centred U, and an apex-sector L (the apex
    closure's walk);
  - each must answer `Ok(None)`. The same frusta also check the
    rectangle: `In` at its centre, `Out` past its window.
- `a_torus_rectangle_trims_on_every_ring`: each ring's rectangle
  answers `In` at its centre and `Out` past its window.
- `a_gap_in_the_torus_walk_refuses`: each of the rectangle's four edges
  in turn is made null scaffolding, and each must refuse as
  `PartialTorusFace`.
- `an_l_shaped_torus_face_refuses_rather_than_trim_by_its_hull`: the
  first pass's large L, U and rectangle, on the parametrised builder.

**Red at `59eeeebb2`.** The kernel code is `59eeeebb2`'s and the rows
are `fbc837baf`'s. Each cell answers `Some(In)`, or escalates in band.

| row | ε 1e-9 | ε 1e-6 | ε 1e-12 |
|---|---|---|---|
| torus small notch | 28 cells | 16 cells | 32 cells |
| cone small notch | 23 cells | 15 cells | 25 cells |
| torus walk gap | accepted with a middle edge skipped | same | same |

The first-pass rows (the large L, U and rectangle, and both rectangle
rows) were green there and are green now. At base (`f4e9aa68b`), the
variation check refused every square U; at `59eeeebb2` the small U
cells answer `Some(In)`. They refuse again now. **Green after**: all 66
torus/cone rows at each of the three ε.

**Mutants**:

| mutant | where | result |
|---|---|---|
| normalisation (`defect / u_span`, levered by `r`) | `59eeeebb2` | killed: torus small-notch row red at all three ε (37 / 25 / 41 cells) |
| closure check off | head | killed: `a_gap_in_the_torus_walk_refuses` (edges 0 and 3) |
| continuity check off | head | killed: the same row (edges 1 and 2) |
| box margin forced to Zero | head | killed: the torus and cone L rows and both small-notch rows |
| box margin squared (quadratic) | head | killed: both small-notch rows |

## What reaches a user

No boolean mints an L-shaped torus face today. A half donut less a bar
over a quarter of its upper tube is refused first by `topo::subtract`,
with `CurvedPairUnsupported { op: Subtract, site: RevertRoster, kind:
Torus, other_kind: Plane }`, and
`crates/sweep/tests/contact11_torus_chart_l.rs` pins exactly that.

**STEP import is a second door.** It adopts toroidal, conical and
spherical faces with arbitrary certified loops, and `point_in_solid` is
public, so an L face from a CAD file reaches these trims. Before this
fix, it would have been served its box. A STEP fixture needs a closed
shell authored by hand, which is more than this unit's fence. It is
filed with the argument:
`work/exch/step-import-l-shaped-curved-face-has-no-containment-row`.
The face door (`curved_face_containment`) and `reduce`'s curved-face
classification both read the fixed trims.

## The class sweep

Pattern 1 was functions that fold a face's chart window or hull
(`fn *window*`, `fn *hull*`, `fn *_trim`), plus `variation` and
`chart_box`, across `topo`, `geom-brep`, `sweep` and `verbs`:

| hit | disposition |
|---|---|
| `solid_contain::torus_chart_windows` | fixed |
| `solid_contain::cone_trimmed_window` | fixed (the same quadratic area margin) |
| `solid_contain::cylinder_chart_trim` + `wall_outline`, `contain.rs` cylinder arm | correct: the rectangle class needs rims on exactly two levels, each level decided as a position (linear), and a simple rectilinear polygon with two levels is a rectangle |
| `solid_contain::sphere_chart_trim` | filed already: `work/contact/sphere-chart-trim-folds-any-number-of-rim-levels`. It now points at the linear side test or its own level count |
| `boxes::torus_chart_window`, `torus_window_extent`, `census::torus_chart_window` | not this class: a bounding box has to contain the face, and over-covering is its contract |
| `chord_join::face_azimuth_window`, `run_azimuth_window` | not this class: one channel's projection |
| `replace_face::group_cone_v_window`, `chart_region::v_window` | not this class: one coordinate's extent |
| `splitting::containment::arc_trim`, `validate::edge_trim`/`window`, `blend::surgery::mef_trim` | not chart-region windows |

Pattern 1 cannot match a "the face is its rectangle" premise held under
another name. Pattern 2 targets that gap: every `rectangle` in the doc
comments of the same crates.

| hit | disposition |
|---|---|
| `geom-brep` `props/curved.rs` `require_iso_rectangle` (`props_rim_level`) | correct, linear: each rim's level is decided against the two extreme levels |
| `topo` `props.rs` NURBS rectangle certificate | correct: EXACT comparisons with no band, so it has no quadratic Zero. Every vertex must sit on a corner coordinate bit for bit, and the shoelace must equal `±` the rectangle's area bit for bit |
| `chart_region::wrap_band` | correct: four exact-point vertices |
| `reduce.rs` window reads | consumers of the fixed face door |

The sweep is as of `f4e9aa68b`.

## Work files

- `work/boxes/chart-window-walk-written-twice.md` gains a dated line on
  where the torus walk stands, with a seam note in `work/boxes/log.md`.
- There are seam notes for the `chord_join` deletion in
  `work/reach/log.md` and `work/tang/log.md`.
- The sphere row is rewritten to point at the linear side test, or at
  its own level count.
- The STEP issue is filed on EXCH's slate.

## Verification (local; hosted CI is the verification of record)

Every check ran on `989be02c1`. The commits after it change only this
file.

| check | result |
|---|---|
| `topo`, default ε / 1e-6 / 1e-12 | 1673 passed at each |
| `sweep`, default ε / 1e-6 / 1e-12 | 1761 passed at each |
| `editor-core`, all (slow set included) | 2274 passed |
| `test-utils` | 79 passed |
| Python suite (maturin wheel, unittest) | 857 OK |
| `cargo fmt --all --check` | clean |
| clippy `--workspace --exclude viewer --all-targets --all-features` | clean |
| clippy `pncad-py --features python` | clean |
| `scripts/gates/*.sh` + `payload-rung-sweep.py --check` | pass |
| `work.py lint` | ok |

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01HkgsMyrV52i5fDxhA2ojxL
