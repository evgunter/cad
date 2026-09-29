CONTACT-11: the torus chart-box check compares areas

Closes `work/contact/torus-chart-box-check-passes-an-l-shaped-face`.

## The defect, measured

`torus_chart_windows` (`crates/topo/src/boolean/solid_contain.rs`)
decided `bool_torus_chart_box` on `variation − 2·span` per chart
channel. A rectilinear polygon has variation `2·span` in a channel
exactly when it is monotone in that channel, and an L is monotone in
both, so it passed. The face was then served its bounding box.

The row was arithmetic; it is now a measurement. `donut_sheet`
(`section_cert_rows.rs`) builds a face of the `R = 2, r = 1/2` donut
from Euler ops, bounded by parallels and meridians through given chart
corners, the way `cone_sheet` builds the cone's. At base
(`f4e9aa68b`), through the public face door `curved_face_containment`:

| face, point | base | ground truth | this branch |
|---|---|---|---|
| L, a point in the notch | `Some(In)` | `Out` (on the tube, off the face) | `None` |
| L, a point in its arm | `Some(In)` | `In` | `None` (the face refuses) |
| U, a point in the notch | `None` | `Out` | `None` |
| rectangle, inside | `Some(In)` | `In` | `Some(In)` |
| rectangle, past its window | `Some(Out)` | `Out` | `Some(Out)` |

The U was already refused: it is not monotone in `t`, so the variation
check did see it.

**No boolean reaches the L yet.** A half donut less a bar over a
quarter of its upper tube would leave both torus walls L-shaped (the
bar's floor cuts the outer and inner equators, its side a meridian).
`topo::subtract` refuses the pair first:
`CurvedPairUnsupported { site: RevertRoster, kind: Torus, other_kind:
Plane }`. `crates/sweep/tests/contact11_torus_chart_l.rs` pins that
refusal. If the pair is admitted, that row is where the notch
(`point_in_solid` answers `Out`) and the volume (`3π²/8`) will need
checking. So the wrong answer reaches a user only through the face door
on a face built by hand. `reduce`'s curved-face classification reads
the same door, and the solid door reads the same windows, so both
inherit the fix.

## The fix

The walk now collects each boundary edge's chart image as a
`chord_join::AzimuthImage`, with the major angle as the azimuth and the
minor angle as `v`. It then decides `bool_torus_chart_box` on
`chord_join::chart_box_defect`, the cone's check: the shoelace area
compared with the bounding box's area. The window returned is that
`ChartBox`'s `(u, v)`, so the hull is folded once, in one place. The
defect is in radian²; divided by the minor span it is the major angle
the notch removes, levered by `R + r` like the major window. That is
the cone's normalisation (`bool_cone_chart_box` divides by the slant
span). The closure check, `bool_torus_chart_closure`, is unchanged and
reads the same images.

`chord_join.rs` is not touched: `chart_box_defect`, `AzimuthImage` and
`ChartBox` were already `pub(crate)`, so no seam note is owed.

Rows (`section_cert_rows.rs`,
`an_l_shaped_torus_face_refuses_rather_than_trim_by_its_hull`): the L
refuses at the notch and in its arm, the U refuses, and the rectangle
answers `In` inside and `Out` past its window. The first commit carries
the row red: at base, the notch answer is `Some(In)`.

## The class sweep

Pattern 1 was the functions that fold a face's chart window or hull:
`fn *window*`, `fn *hull*` and `fn *_trim` across `topo`, `geom-brep`,
`sweep` and `verbs`. It was run together with `variation` and
`chart_box`.

| hit | disposition |
|---|---|
| `solid_contain::torus_chart_windows` | fixed here |
| `solid_contain::cone_trimmed_window` | already area (`bool_cone_chart_box`), the precedent |
| `solid_contain::cylinder_chart_trim` + `wall_outline` | correct: the rectangle class requires rims on exactly two levels, and a simple rectilinear polygon with two levels is a rectangle; the rest is read by parity |
| `contain.rs` cylinder arm | the same `wall_outline` class |
| `solid_contain::sphere_chart_trim` | the defect, already filed: `work/contact/sphere-chart-trim-folds-any-number-of-rim-levels`; this PR adds the shared area check to it as a second closing shape |
| `boxes::torus_chart_window`, `torus_window_extent`, `census::torus_chart_window` | not this class: bounding boxes that must contain the face, and over-covering is their contract |
| `chord_join::face_azimuth_window`, `run_azimuth_window` | not this class: a one-channel projection, which an L covers in full; the region claim built on it is `wall_outline`'s |
| `replace_face::group_cone_v_window`, `chart_region::v_window` | not this class: one coordinate's extent, which a boundary hull states exactly |
| `splitting::containment::arc_trim`, `validate::edge_trim`/`window`, `blend::surgery::mef_trim` | not chart-region windows |

What pattern 1 cannot match is a "the face is its rectangle" premise
held under another name. Pattern 2 targets that gap: every
`rectangle` in the doc comments of the same crates.

| hit | disposition |
|---|---|
| `geom-brep` `props/curved.rs` `require_iso_rectangle` (`props_rim_level`) | correct: every rim sits at one of two extreme levels, so the width is constant (#649 closed the span-sum version of this defect) |
| `topo` `props.rs` NURBS rectangle certificate | correct: shoelace against the rectangle's area, the same check as this PR |
| `chart_region::wrap_band` | correct: exactly four exact-point vertices |
| `reduce.rs` window reads | consumers of `curved_face_containment`, so they inherit the fix |

The sweep is as of `f4e9aa68b`.

## Verification (local; hosted CI is the verification of record)

The full battery ran on `b0fc86ab8`. The only later code commit is
`9ec03c027`, which rewrites one `.err().expect()` in the new sweep row
as `let … else` for clippy. Clippy, fmt and that row were re-run on it.

| check | commit | result |
|---|---|---|
| `topo`, default eps | `b0fc86ab8` | 1669 passed |
| `topo`, `CAD_TOLERANCE_EPS=1e-6` | `b0fc86ab8` | 1669 passed |
| `topo`, `CAD_TOLERANCE_EPS=1e-12` | `b0fc86ab8` | 1669 passed |
| `sweep`, default eps | `b0fc86ab8` | 1761 passed |
| `sweep`, 1e-6 | `b0fc86ab8` | 1761 passed |
| `sweep`, 1e-12 | `b0fc86ab8` | 1761 passed |
| `editor-core`, all (slow set included) | `b0fc86ab8` | 2274 passed |
| `test-utils` | `b0fc86ab8` | 79 passed |
| Python suite (maturin wheel, unittest) | `b0fc86ab8` | 857 OK |
| `cargo fmt --all --check` | `9ec03c027` | clean |
| clippy `--workspace --exclude viewer --all-targets --all-features` | `b0fc86ab8` red (`err_expect`), `9ec03c027` clean | clean |
| clippy `pncad-py --features python` | `b0fc86ab8` | clean |
| `scripts/gates/*.sh` + `payload-rung-sweep.py --check` | `b0fc86ab8` | pass |
| `work.py lint` | `b0fc86ab8` | ok |
| the new sweep row | `9ec03c027` | pass |

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01HkgsMyrV52i5fDxhA2ojxL
