//! **A cut through a wrap edge's vertex keeps one wall face exactly
//! where the piece goes round.** The body is a one-segment circle
//! (r = 1 about the local z axis, its vertex at azimuth `φ`) extruded
//! over `z ∈ [0, 2]`: one wall whose strut, on the meridian through the
//! vertex, is its wrap edge (D1). A plane through the strut's bottom or
//! top vertex leaves the whole strut on one side, inside that side's
//! piece of the wall.
//!
//! Over the wall the plane is a height `z = g(θ)`, and the piece above
//! it, `g(θ) < z ≤ 2`, is at each azimuth empty or one interval, so it
//! is one band of azimuths:
//!
//! - **round** (`max g < 2`): an annulus, and the strut is its wrap
//!   edge, ONE face whose loop uses the strut twice;
//! - **a disc**: an arc of azimuths (`max g > 2`, the plane crossing the
//!   far cap), or every azimuth but one (`max g = 2`, the plane tangent
//!   to the far rim, which pinches the band shut there). The strut runs
//!   across it from section to rim, and a face on both sides of a chord
//!   has an interior in two pieces and a shell whose `V − E + F` is odd.
//!   The strut is the cut the author wrote (D1), but the two faces on
//!   either side of it are on the one surface, so `split`, which ends
//!   each side with maximal faces, and a boolean's output stage both
//!   glue them into one maximal face and the strut dies (DESIGN,
//!   "Maximal faces and maximal edges": the sub-period re-merge).
//!
//! The piece below the plane, `0 ≤ z < g(θ)`, meets the strut only at
//! the vertex the plane passes through, and is one face. The pose
//! `one_segment_loop.rs` names "through the bottom seam vertex" is the
//! pinched disc: its plane, through the vertex and the axis's midpoint,
//! is tangent to the top rim at the vertex's antipode.
//!
//! Every result is read at all three tiers, against that census of wall
//! faces and strut, with every edge's wrap flag against its faces, and
//! against the closed-form volume `∫∫ clamp(g, 0, 2) dA` over the disc.
//! Two doors: `topo::split` by planes through either vertex, of the
//! cylinder and of a block the circle is a hole in, both senses of each
//! normal; the boolean against a half-space through the bottom vertex
//! of the cylinder tilted about x, every op in both operand senses.
//!
//! **Near the vertex** the plane is also moved ±0.6ε and ±0.9ε along its
//! normal, which leaves the vertex On. A pose whose plane only touches
//! the near rim at the vertex builds as through it. Every other pose
//! crosses the rim, or meets the section's carrier, between ε and Kε
//! from the vertex, and refuses typed with the margin it read in the
//! band (D4); the two-arc form refuses those poses alike, but for one
//! class that reads a sector's subdivision ray rather than the geometry
//! (`work/cleave/a-wide-sectors-subdivision-ray-along-the-planes-trace-reads-in-the-band.md`).
//! A near pose is held to the closed form wherever it builds.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI, TAU};
use core::fmt::Debug;

use geom_brep::EdgeDescription;
use geom_core::{Affine3, Arc2, Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, BooleanError, SplitJoinError};

fn tol() -> Tol {
    Tol::witness()
}

const HEIGHT: f64 = 2.0;

/// The one-segment circle of radius 1 about the origin, its vertex at
/// azimuth `phase`, turning `sweep`.
fn circle(phase: f64, sweep: f64) -> ProfileLoop<f64> {
    RawLoop::new([(
        Point2::new(phase.cos(), phase.sin()),
        Segment::Arc(Arc2 {
            centre: Point2::new(0.0, 0.0),
            radius: 1.0,
            sweep,
        }),
    )])
}

/// `loops` extruded over `z ∈ [0, 2]`.
fn extruded(loops: Vec<ProfileLoop<f64>>) -> Body<f64> {
    let profile = Profile::new(SketchPlane::<f64>::xy(), loops)
        .validate(tol())
        .unwrap();
    let depth = Extrusion::Distance {
        depth: HEIGHT,
        side: ExtrudeSide::Along,
    };
    extrude(&profile, depth, tol()).unwrap().body
}

/// The one-segment cylinder: [`circle`] extruded.
fn cylinder(phase: f64, sweep: f64) -> Body<f64> {
    extruded(vec![circle(phase, sweep)])
}

/// The square `[-2, 2]²` with a [`circle`] hole, extruded.
fn holed_block(phase: f64, sweep: f64) -> Body<f64> {
    let p = |x: f64, y: f64| Point2::new(x, y);
    let square = RawLoop::polygon([p(-2.0, -2.0), p(2.0, -2.0), p(2.0, 2.0), p(-2.0, 2.0)]);
    extruded(vec![square, circle(phase, sweep)])
}

/// A plane in the cylinder's frame, `n · p = d`, with `n.z ≠ 0`.
#[derive(Clone, Copy, Debug)]
struct Plane {
    n: [f64; 3],
    d: f64,
}

/// `∫∫ max(0, f) dA` over the unit disc for the linear
/// `f(x, y) = f0 + px·x + py·y`.
pub(crate) fn hoof(f0: f64, px: f64, py: f64) -> f64 {
    let s = px.hypot(py);
    if s == 0.0 {
        return PI * f0.max(0.0);
    }
    let u0 = -f0 / s;
    if u0 <= -1.0 {
        PI * f0
    } else if u0 >= 1.0 {
        0.0
    } else {
        let w = (1.0 - u0 * u0).sqrt();
        s * (2.0 / 3.0 * w * w * w - u0 * (u0.acos() - u0 * w))
    }
}

/// `∫∫ max(0, f) dA` over the square `[-2, 2]²` for the linear
/// `f(x, y) = f0 + px·x + py·y`: the square clipped to `f ≥ 0`, whose
/// integral is its area times `f` at its centroid.
pub(crate) fn square_hoof(f0: f64, px: f64, py: f64) -> f64 {
    let f = |q: [f64; 2]| f0 + px * q[0] + py * q[1];
    let square = [[-2.0, -2.0], [2.0, -2.0], [2.0, 2.0], [-2.0, 2.0]];
    let mut kept: Vec<[f64; 2]> = Vec::new();
    for (i, &a) in square.iter().enumerate() {
        let b = square[(i + 1) % 4];
        if f(a) >= 0.0 {
            kept.push(a);
        }
        if (f(a) >= 0.0) != (f(b) >= 0.0) {
            let t = f(a) / (f(a) - f(b));
            kept.push([a[0] + t * (b[0] - a[0]), a[1] + t * (b[1] - a[1])]);
        }
    }
    let (mut area, mut cx, mut cy) = (0.0, 0.0, 0.0);
    for (i, &a) in kept.iter().enumerate() {
        let b = kept[(i + 1) % kept.len()];
        let cross = a[0] * b[1] - b[0] * a[1];
        area += cross / 2.0;
        cx += (a[0] + b[0]) * cross / 6.0;
        cy += (a[1] + b[1]) * cross / 6.0;
    }
    if area == 0.0 {
        0.0
    } else {
        area * f([cx / area, cy / area])
    }
}

/// What one side of a plane keeps of the wall and its strut.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Wall {
    /// No wall on this side.
    None,
    /// One face; `strut` says whether the strut is in it, as its wrap
    /// edge.
    One { strut: bool },
    /// Two faces on the one surface, the strut between them.
    Two,
}

/// [`Wall`] after the merge a split and a boolean's output stage end
/// with, which glues a disc's two faces into one and kills the strut.
fn merged(wall: Wall) -> Wall {
    match wall {
        Wall::Two => Wall::One { strut: false },
        other => other,
    }
}

/// What the side `n · p > d` (`positive`) or `< d` keeps of the
/// cylinder whose vertex is at azimuth `phase`, or of the block it is a
/// `hole` in, before any merge: its wall and its volume. `band` is
/// how near `max g` or `min g` may come to a cap before the plane
/// counts as tangent to that cap's rim.
fn closed_form(plane: Plane, phase: f64, positive: bool, hole: bool, band: f64) -> (Wall, f64) {
    let [a, b, c] = plane.n;
    // `z < g` is the side `n · p < d` when `c > 0`.
    let under = positive == (c < 0.0);
    let (f0, px, py) = (plane.d / c, -a / c, -b / c);
    let disc = hoof(f0, px, py) - hoof(f0 - HEIGHT, px, py);
    let (below, whole) = if hole {
        let square = square_hoof(f0, px, py) - square_hoof(f0 - HEIGHT, px, py);
        (square - disc, (16.0 - PI) * HEIGHT)
    } else {
        (disc, PI * HEIGHT)
    };
    let volume = if under { below } else { whole - below };
    let amp = px.hypot(py);
    let (lo, hi) = (f0 - amp, f0 + amp);
    let at_strut = f0 + px * phase.cos() + py * phase.sin();
    let wall = if under {
        let strut = at_strut > band;
        if hi <= band {
            Wall::None
        } else if lo > band || !strut {
            Wall::One { strut }
        } else {
            Wall::Two
        }
    } else {
        let strut = at_strut < HEIGHT - band;
        if lo >= HEIGHT - band {
            Wall::None
        } else if hi < HEIGHT - band || !strut {
            Wall::One { strut }
        } else {
            Wall::Two
        }
    };
    (wall, volume)
}

/// Whether `edge` is described as its chart's wrap edge.
fn is_wrap(body: &Body<f64>, edge: topo::EdgeKey) -> bool {
    let e = body.get_edge(edge).unwrap();
    let c = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
    matches!(c.description(), EdgeDescription::Chart(ch) if ch.wrap)
}

/// What `body` keeps of the wall: its faces on a cylinder, and the
/// edges between two of them or both of whose halves bound one. An
/// edge whose wrap flag disagrees with its faces is an error.
fn wall_census(body: &Body<f64>) -> Result<Wall, String> {
    let on_wall = |face: topo::FaceKey| {
        matches!(
            body.get_surface(body.get_face(face).unwrap().surface),
            Some(geom::Surface::Cylinder { .. })
        )
    };
    let walls = body.faces().filter(|&(f, _)| on_wall(f)).count();
    let (mut wraps, mut parted) = (0, 0);
    for (edge, _) in body.edges() {
        let sides = topo::readback::edge_sides(body, edge).unwrap();
        let one = sides.plus.face == sides.minus.face;
        if is_wrap(body, edge) != one {
            return Err(format!("{edge:?}'s wrap flag against its faces"));
        }
        if on_wall(sides.plus.face) && on_wall(sides.minus.face) {
            if one {
                wraps += 1;
            } else {
                parted += 1;
            }
        }
    }
    match (walls, wraps, parted) {
        (0, 0, 0) => Ok(Wall::None),
        (1, w @ (0 | 1), 0) => Ok(Wall::One { strut: w == 1 }),
        (2, 0, 1) => Ok(Wall::Two),
        other => Err(format!("wall faces, wrap edges, parted struts {other:?}")),
    }
}

/// A refusal that read its margin in the band (D4).
fn in_band(refusal: &impl Debug) -> bool {
    format!("{refusal:?}").contains("Indeterminate {")
}

/// All three tiers, the wall and, with `volume` set, the volume of one
/// result. `near` accepts a volume the quadrature reads in the band.
fn check(
    body: Option<&Body<f64>>,
    want: (Wall, f64),
    volume: bool,
    near: bool,
) -> Result<(), String> {
    let (wall, closed) = want;
    let Some(body) = body else {
        return if closed <= 1e-9 {
            Ok(())
        } else {
            Err(format!("no body where {closed} is kept"))
        };
    };
    topo::validate(body).map_err(|e| format!("tier 1: {e:?}"))?;
    topo::validate_closed(body).map_err(|e| format!("tier 2: {e:?}"))?;
    topo::validate_geometric(body, tol()).map_err(|e| format!("tier 3: {e:?}"))?;
    let got = wall_census(body)?;
    if got != wall {
        return Err(format!("the wall: {got:?}, closed form {wall:?}"));
    }
    if !volume {
        return Ok(());
    }
    let m = match topo::mass_properties(
        &topo::test_support::finished("the measured body", topo::Body::clone(body), tol()),
        tol(),
        topo::test_support::pinned_resolution(tol()),
    ) {
        Ok(m) => m,
        Err(e) if near && in_band(&e) => return Ok(()),
        Err(e) => return Err(format!("the volume: {e:?}")),
    };
    if (m.enclosure.volume - closed).abs() > m.enclosure.volume_pad + 1e-9 * closed.max(1.0) {
        return Err(format!(
            "volume {} ± {}, closed form {closed}",
            m.enclosure.volume, m.enclosure.volume_pad
        ));
    }
    Ok(())
}

/// Panics listing every failure in `failures`, out of `runs`.
fn report(failures: &[String], runs: usize) {
    assert!(
        failures.is_empty(),
        "{} of {runs} failed:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

/// The offsets of the plane from the vertex along its unit normal.
fn offsets() -> [f64; 5] {
    let eps = tol().eps();
    [0.0, 0.6 * eps, -0.6 * eps, 0.9 * eps, -0.9 * eps]
}

/// The poses of a plane through a vertex, as `(h, β)`: the normal's
/// part across the axis over its part along it, and its azimuth from
/// the vertex's. Through the bottom vertex the piece above is round
/// where `h(1 + cos β) < 2`, pinched at the antipode where it equals 2,
/// and a disc where it exceeds 2. `β = 0` touches the near rim at the
/// vertex.
const POSES: [(f64, f64, &str); 6] = [
    (0.4, 0.0, "round, leaning to the vertex"),
    (1.0, 0.0, "pinched at the far rim's antipode"),
    (1.6, 0.0, "a disc to the far cap"),
    (0.8, FRAC_PI_2, "round, leaning across"),
    (2.5, FRAC_PI_2, "a disc to the far cap, leaning across"),
    (1.5, 2.5, "round, leaning away"),
];

/// Splits the body `build` makes at each of `phases` (the cylinder, or
/// the block it is a `hole` in) through either vertex of the strut,
/// over `poses` and `offsets`, both senses of each normal (the volumes
/// read on one), and holds every result to [`closed_form`].
fn split_family(
    build: fn(f64, f64) -> Body<f64>,
    hole: bool,
    phases: [(f64, f64); 2],
    poses: &[(f64, f64, &str)],
    offsets: &[f64],
) {
    let band = 4.0 * tol().eps();
    let (mut runs, mut failures) = (0, Vec::new());
    for (phase, sweep) in phases {
        let body = topo::test_support::finished("the body", build(phase, sweep), tol());
        let (c, s) = (phase.cos(), phase.sin());
        for (z, up) in [(0.0, 1.0), (HEIGHT, -1.0)] {
            for &(h, beta, pose) in poses {
                for sense in [1.0, -1.0] {
                    let n = [
                        sense * h * (phase + beta).cos(),
                        sense * h * (phase + beta).sin(),
                        sense * up,
                    ];
                    let len = h.hypot(1.0);
                    for &offset in offsets {
                        let at = [
                            c + offset * n[0] / len,
                            s + offset * n[1] / len,
                            z + offset * n[2] / len,
                        ];
                        let plane = Plane {
                            n,
                            d: n[0] * at[0] + n[1] * at[1] + n[2] * at[2],
                        };
                        let what = format!(
                            "phase {phase}, sweep {sweep}, vertex z = {z}: {pose}, sense {sense}, \
                             offset {offset:e}"
                        );
                        let split = topo::test_support::split_plane(
                            Point3::new(at[0], at[1], at[2]),
                            Vec3::new(n[0], n[1], n[2]),
                            tol(),
                        );
                        let (near, volume) = (offset != 0.0, sense > 0.0);
                        let side = |positive| {
                            let (wall, volume) = closed_form(plane, phase, positive, hole, band);
                            (merged(wall), volume)
                        };
                        runs += 1;
                        let got = match topo::split(&body, &split, tol()) {
                            Ok(out) => {
                                check(out.above.body().map(|b| &**b), side(true), volume, near)
                                    .map_err(|e| format!("above: {e}"))
                                    .and_then(|()| {
                                        check(
                                            out.below.body().map(|b| &**b),
                                            side(false),
                                            volume,
                                            near,
                                        )
                                        .map_err(|e| format!("below: {e}"))
                                    })
                                    .and_then(|()| {
                                        // A plane touching the rim at the vertex: tier 3′.
                                        if beta != 0.0 {
                                            return Ok(());
                                        }
                                        [out.above.body(), out.below.body()]
                                            .into_iter()
                                            .flatten()
                                            .try_for_each(|b| {
                                                topo::validate_pseudomanifold(
                                                    b,
                                                    &topo::ContactRecords::default(),
                                                    tol(),
                                                )
                                                .map_err(|e| format!("tier 3′: {e:?}"))
                                            })
                                    })
                            }
                            Err(e) if near && in_band(&e) => Ok(()),
                            Err(e) => Err(format!("the split: {e:?}")),
                        };
                        if let Err(e) = got {
                            failures.push(format!("{what}: {e}"));
                        }
                    }
                }
            }
        }
    }
    report(&failures, runs);
}

/// **A split through either vertex of the strut**: two vertex phases
/// (one per winding), every pose of [`POSES`] and its mirror through the
/// top vertex, through the vertex and near it.
#[test]
fn a_split_through_a_strut_vertex_keeps_one_face_where_the_piece_goes_round() {
    split_family(
        cylinder,
        false,
        [(0.0, TAU), (FRAC_PI_2, -TAU)],
        &POSES,
        &offsets(),
    );
}

/// **A hole's wall reads the same**: the circle as a hole through the
/// block `[-2, 2]² × [0, 2]`, its wall concave to the material, split
/// through either vertex of its strut by every pose of [`POSES`]. The
/// volumes are the block's prism under the plane less the hole's.
///
/// A plane touching the hole's rim at the vertex (`β = 0`) has material
/// on both sides of its trace there: the trace on the cap and the wall's
/// section leave the vertex along one tangent and are ordered by how
/// they curve off it, and where the section also touches the far rim
/// (the pinched pose) the wall's crossings at each touch are paired
/// exits first.
#[test]
fn a_split_through_a_hole_walls_strut_vertex_keeps_one_face_where_the_piece_goes_round() {
    split_family(
        holed_block,
        true,
        [(0.0, -TAU), (FRAC_PI_2, TAU)],
        &POSES,
        &[0.0],
    );
}

/// The square `[-2, 2]²` with a two-arc circle hole, its vertices at
/// azimuth `phase` and its antipode, extruded.
fn two_arc_holed_block(phase: f64, sweep: f64) -> Body<f64> {
    let p = |x: f64, y: f64| Point2::new(x, y);
    let square = RawLoop::polygon([p(-2.0, -2.0), p(2.0, -2.0), p(2.0, 2.0), p(-2.0, 2.0)]);
    let arc = |at: f64| {
        (
            p(at.cos(), at.sin()),
            Segment::Arc(Arc2 {
                centre: p(0.0, 0.0),
                radius: 1.0,
                sweep: sweep / 2.0,
            }),
        )
    };
    extruded(vec![square, RawLoop::new([arc(phase), arc(phase + PI)])])
}

/// **The per-PR row of the tangent poses**: the two-arc hole split
/// through its vertex at `(1, 0, 0)` by the round pose leaning onto it
/// (`h = 0.4`), both senses, each side at all three tiers, tier 3′ and
/// its closed-form volume. The order of the two germs leaving the vertex
/// along one tangent decides this pose's corners; read the other way
/// round, it does not build.
#[test]
fn a_plane_tangent_to_a_two_arc_holes_rim_splits_sound() {
    let body = topo::test_support::finished("the body", two_arc_holed_block(0.0, TAU), tol());
    let mut failures = Vec::new();
    for sense in [1.0, -1.0] {
        let n = [sense * 0.4, 0.0, sense];
        let plane = Plane { n, d: n[0] };
        let split = topo::test_support::split_plane(
            Point3::new(1.0, 0.0, 0.0),
            Vec3::new(n[0], n[1], n[2]),
            tol(),
        );
        let what = format!("sense {sense}");
        match topo::split(&body, &split, tol()) {
            Ok(out) => {
                for (side, positive) in [(&out.above, true), (&out.below, false)] {
                    let (_, closed) = closed_form(plane, 0.0, positive, true, 0.0);
                    if let Err(e) = tangent_side(
                        side.body(),
                        closed,
                        (Point3::new(1.0, 0.0, 0.0), positive == (sense < 0.0)),
                        &what,
                    ) {
                        failures.push(format!("{what}, positive {positive}: {e}"));
                    }
                }
            }
            Err(e) => failures.push(format!("{what}: the split: {e:?}")),
        }
    }
    report(&failures, 2);
}

/// A body built from a vertex phase and a sweep.
type Build = fn(f64, f64) -> Body<f64>;

/// **A plane tangent to a hole's rim at a vertex**: the poses of
/// [`POSES`] that touch the rim there (`β = 0`), through the bottom
/// vertex and the top one, both senses of each normal, either winding,
/// of the one-segment hole and the two-arc one. Each side holds at all
/// three tiers and at tier 3′, and its volume is the closed form's: the
/// block's prism under the plane less the hole's.
#[test]
fn a_plane_tangent_to_a_holes_rim_at_a_vertex_splits_sound() {
    let builds: [(Build, &str); 2] = [
        (holed_block, "one segment"),
        (two_arc_holed_block, "two arcs"),
    ];
    let (mut runs, mut failures) = (0, Vec::new());
    for (build, form) in builds {
        for (phase, sweep) in [(0.0, TAU), (FRAC_PI_2, -TAU)] {
            let body = topo::test_support::finished("the body", build(phase, sweep), tol());
            let (c, s) = (phase.cos(), phase.sin());
            for (z, up) in [(0.0, 1.0), (HEIGHT, -1.0)] {
                for &(h, _, pose) in POSES.iter().filter(|&&(_, beta, _)| beta == 0.0) {
                    for sense in [1.0, -1.0] {
                        let n = [sense * h * c, sense * h * s, sense * up];
                        let plane = Plane {
                            n,
                            d: n[0] * c + n[1] * s + n[2] * z,
                        };
                        let split = topo::test_support::split_plane(
                            Point3::new(c, s, z),
                            Vec3::new(n[0], n[1], n[2]),
                            tol(),
                        );
                        let what = format!(
                            "{form}, phase {phase}, sweep {sweep}, vertex z = {z}: {pose}, sense \
                             {sense}"
                        );
                        runs += 1;
                        match topo::split(&body, &split, tol()) {
                            Ok(out) => {
                                for (side, positive) in [(&out.above, true), (&out.below, false)] {
                                    let (_, closed) =
                                        closed_form(plane, phase, positive, true, 0.0);
                                    if let Err(e) = tangent_side(
                                        side.body(),
                                        closed,
                                        (Point3::new(c, s, z), positive == (sense < 0.0)),
                                        &what,
                                    ) {
                                        failures.push(format!("{what}, positive {positive}: {e}"));
                                    }
                                }
                            }
                            Err(e) => failures.push(format!("{what}: the split: {e:?}")),
                        }
                    }
                }
            }
        }
    }
    report(&failures, runs);
}

/// One side of [`a_plane_tangent_to_a_holes_rim_at_a_vertex_splits_sound`]:
/// all three tiers, tier 3′, its volume against `closed`, and its
/// vertices on the touch point: two on the side that keeps the horns
/// between the trace and the rim (the hole's side of the trace), which
/// meet there with its row, else one.
fn tangent_side(
    body: Option<&topo::AtRestBody<f64>>,
    closed: f64,
    (touch, horns): (Point3<f64>, bool),
    what: &str,
) -> Result<(), String> {
    let Some(b) = body else {
        return Err(format!("no body where {closed} is kept"));
    };
    let at_touch = b
        .vertex_points()
        .filter(|(_, p)| p.distance(touch) <= tol().eps())
        .count();
    if at_touch != 1 + usize::from(horns) {
        return Err(format!("{at_touch} vertices on the touch point"));
    }
    sound(b, &topo::ContactRecords::default(), closed, what)
}

/// All three tiers of `b`, tier 3′ over its `contacts`, and its volume
/// against `closed`.
fn sound(
    b: &topo::AtRestBody<f64>,
    contacts: &topo::ContactRecords,
    closed: f64,
    what: &str,
) -> Result<(), String> {
    topo::validate(b).map_err(|e| format!("tier 1: {e:?}"))?;
    topo::validate_closed(b).map_err(|e| format!("tier 2: {e:?}"))?;
    topo::validate_geometric(b, tol()).map_err(|e| format!("tier 3: {e:?}"))?;
    topo::validate_pseudomanifold(b, contacts, tol()).map_err(|e| format!("tier 3′: {e:?}"))?;
    let m = topo::test_support::pinned_volume(what, b, tol());
    if (m.volume - closed).abs() > m.volume_pad + 1e-9 * closed {
        return Err(format!(
            "volume {} ± {}, closed form {closed}",
            m.volume, m.volume_pad
        ));
    }
    Ok(())
}

/// **A boolean against a half-space through the bottom vertex**: the
/// cylinder at three vertex phases, tilted about x by ±0.4, ±π/4, ±0.9
/// and ±1.3 rad, against the brick `z ≤ z₀` whose top face passes
/// through the vertex or near it; A − B and A ∪ B keep the piece above,
/// A ∩ B and B − A the piece below. At phase 0 the tilt leans the plane
/// across the vertex (round below `tan t = 2`); at phase π/2 onto it
/// (round below `tan t = 1`, pinched at π/4), where a pose leaning away
/// touches the cylinder at the vertex alone and is left out.
///
/// One refusal is held as it stands: the plane touching the near rim at
/// the vertex with the piece above round (phase π/2, tilt 0.4) refuses
/// `Join(SingleSiteSectionLoop { count: 1 })`, where `split` builds the
/// same pose
/// (`work/secttail/a-plane-tangent-to-a-rim-at-its-wrap-edges-vertex-has-no-join-arm.md`).
#[test]
fn a_boolean_through_a_strut_vertex_keeps_one_face_where_the_piece_goes_round() {
    let band = 4.0 * tol().eps();
    let half = 5.0;
    let (mut runs, mut failures) = (0, Vec::new());
    for (phase, sweep) in [(0.0, TAU), (FRAC_PI_2, -TAU), (2.0, TAU)] {
        let cyl = cylinder(phase, sweep);
        for t in [0.4, FRAC_PI_4, 0.9, 1.3, -0.4, -FRAC_PI_4, -0.9, -1.3] {
            let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), t);
            let lift = |p: [f64; 3]| turn.transform_point(Point3::new(p[0], p[1], p[2])).z;
            let origin = lift([0.0; 3]);
            let n = [
                lift([1.0, 0.0, 0.0]) - origin,
                lift([0.0, 1.0, 0.0]) - origin,
                lift([0.0, 0.0, 1.0]) - origin,
            ];
            let vertex = [phase.cos(), phase.sin(), 0.0];
            // `n` against the vertex's azimuth, as a cosine.
            let lean = (n[0] * vertex[0] + n[1] * vertex[1]) / n[0].hypot(n[1]);
            if lean < -0.99 {
                continue;
            }
            let a = topo::test_support::finished(
                "the tilted cylinder",
                topo::transform_rigid(&cyl, &geom_core::test_support::rigid(&turn, tol()), tol())
                    .unwrap(),
                tol(),
            );
            for offset in offsets() {
                let z0 = lift(vertex) + offset;
                let plane = Plane { n, d: z0 - origin };
                let what = format!("phase {phase}, sweep {sweep}, tilt {t}, offset {offset:e}");
                let brick =
                    sweep::test_support::brick((-half, half), (-half, half), (-half, z0), tol());
                let slab = 4.0 * half * half * (z0 + half);
                let b = topo::test_support::finished("the half-space", brick, tol());
                let (wall_above, above) = closed_form(plane, phase, true, false, band);
                let (wall_below, below) = closed_form(plane, phase, false, false, band);
                let (wall_above, wall_below) = (merged(wall_above), merged(wall_below));
                let single_site = lean > 0.99 && wall_above == (Wall::One { strut: true });
                let near = offset != 0.0;
                for (op, name, first, second, want) in [
                    (BooleanOp::Subtract, "A − B", &a, &b, (wall_above, above)),
                    (BooleanOp::Intersect, "A ∩ B", &a, &b, (wall_below, below)),
                    (
                        BooleanOp::Union,
                        "A ∪ B",
                        &a,
                        &b,
                        (wall_above, slab + above),
                    ),
                    (
                        BooleanOp::Subtract,
                        "B − A",
                        &b,
                        &a,
                        (wall_below, slab - below),
                    ),
                ] {
                    runs += 1;
                    let got = boolean_op_with(
                        op,
                        first,
                        second,
                        &topo::BooleanInputs::default(),
                        SweepStrategy::Realized,
                        tol(),
                    );
                    let got = match got {
                        Ok(_) if single_site => Err("builds: the single-site row's pin".into()),
                        Ok(out) => check(
                            out.body().map(|b| -> &Body<f64> { &b.body }),
                            want,
                            true,
                            near,
                        ),
                        Err(BooleanError::Join(j))
                            if single_site
                                && matches!(
                                    j.error(),
                                    SplitJoinError::SingleSiteSectionLoop { count: 1 }
                                ) =>
                        {
                            Ok(())
                        }
                        Err(e) if near && in_band(&e) => Ok(()),
                        Err(e) => Err(format!("the boolean: {e:?}")),
                    };
                    if let Err(e) = got {
                        failures.push(format!("{what}: {name}: {e}"));
                    }
                }
            }
        }
    }
    report(&failures, runs);
}

/// The held rows' refusal: the join's ring role order (`join.rs`
/// `ring_order`), its one site.
const RING_ROLE_DESYNC: &str = "derived ring role order separates a loose scaffolding pair";

/// One pose of [`a_boolean_tangent_to_a_holes_rim_at_a_vertex_builds_sound`]:
/// the block `build(π/2, sweep)` turned about x by `t`, against the
/// brick `z ≤ z₀` whose top face passes through the turned vertex. Each
/// of A − B, A ∩ B and A ∪ B builds sound at its closed form, or, where
/// `held`, refuses as it stands. Returns the failures.
fn tangent_boolean(build: Build, sweep: f64, t: f64, held: bool, what: &str) -> Vec<String> {
    let half = 5.0;
    let phase = FRAC_PI_2;
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), t);
    let lift = |p: [f64; 3]| turn.transform_point(Point3::new(p[0], p[1], p[2])).z;
    let origin = lift([0.0; 3]);
    let n = [
        lift([1.0, 0.0, 0.0]) - origin,
        lift([0.0, 1.0, 0.0]) - origin,
        lift([0.0, 0.0, 1.0]) - origin,
    ];
    let z0 = lift([phase.cos(), phase.sin(), 0.0]);
    let plane = Plane { n, d: z0 - origin };
    let a = topo::test_support::finished(
        "the turned block",
        topo::transform_rigid(
            &build(phase, sweep),
            &geom_core::test_support::rigid(&turn, tol()),
            tol(),
        )
        .unwrap(),
        tol(),
    );
    let b = topo::test_support::finished(
        "the half-space",
        sweep::test_support::brick((-half, half), (-half, half), (-half, z0), tol()),
        tol(),
    );
    let slab = 4.0 * half * half * (z0 + half);
    let (_, above) = closed_form(plane, phase, true, true, 0.0);
    let (_, below) = closed_form(plane, phase, false, true, 0.0);
    let mut failures = Vec::new();
    for (op, name, closed) in [
        (BooleanOp::Subtract, "A − B", above),
        (BooleanOp::Intersect, "A ∩ B", below),
        (BooleanOp::Union, "A ∪ B", slab + above),
    ] {
        let what = format!("{what}: {name}");
        let got = boolean_op_with(
            op,
            &a,
            &b,
            &topo::BooleanInputs::default(),
            SweepStrategy::Realized,
            tol(),
        );
        let got = match got {
            Ok(_) if held => Err("builds: the held row's pin".into()),
            Ok(out) => match out.body() {
                Some(r) => sound(&r.body, &r.contacts, closed, &what),
                None => Err(format!("no body where {closed} is kept")),
            },
            Err(BooleanError::JoinDesync { what }) if held && what == RING_ROLE_DESYNC => Ok(()),
            Err(e) => Err(format!("the boolean: {e:?}")),
        };
        if let Err(e) = got {
            failures.push(format!("{what}: {e}"));
        }
    }
    failures
}

/// **A boolean whose face is tangent to a hole's rim at a vertex**: the
/// block with the one-segment hole and the two-arc one, its vertex at
/// azimuth π/2, either winding, turned about x by ±0.4 and ±0.9 against
/// the brick whose top face passes through the turned vertex. The face's
/// trace on the block's bottom face is tangent to the rim there: leaning
/// onto the vertex (`t > 0`) the trace and the hole wall's section leave
/// it along one tangent in the face, with material on both sides, and
/// the ring orders them by how they curve off it. A − B, A ∩ B and
/// A ∪ B each hold at all three tiers and at tier 3′, at the closed
/// form.
///
/// Held as it stands: the one-segment hole at `t = 0.4`, one pose of a
/// class. Wherever the wall's section stays one site (round, meeting
/// the rim at the wrap edge's vertex alone), the one-segment form
/// refuses `JoinDesync` at the join's ring role order
/// ([`RING_ROLE_DESYNC`];
/// `work/secttail/a-plane-tangent-to-a-rim-at-its-wrap-edges-vertex-has-no-join-arm.md`).
#[test]
fn a_boolean_tangent_to_a_holes_rim_at_a_vertex_builds_sound() {
    let builds: [(Build, &str); 2] = [
        (holed_block, "one segment"),
        (two_arc_holed_block, "two arcs"),
    ];
    let (mut runs, mut failures) = (0, Vec::new());
    for (build, form) in builds {
        for sweep in [TAU, -TAU] {
            for t in [0.4, 0.9, -0.4, -0.9] {
                let held = form == "one segment" && t == 0.4;
                runs += 3;
                failures.extend(tangent_boolean(
                    build,
                    sweep,
                    t,
                    held,
                    &format!("{form}, sweep {sweep}, tilt {t}"),
                ));
            }
        }
    }
    report(&failures, runs);
}

/// **The per-PR row of the boolean's tangent poses**: the two-arc hole
/// leaning onto its vertex (`t = 0.4`), every op sound at its closed
/// form ([`a_boolean_tangent_to_a_holes_rim_at_a_vertex_builds_sound`]).
/// Read at first order alone, the ring refuses the two germs along one
/// tangent.
#[test]
fn a_boolean_tangent_to_a_two_arc_holes_rim_builds_sound() {
    let failures = tangent_boolean(two_arc_holed_block, TAU, 0.4, false, "two arcs, tilt 0.4");
    report(&failures, 3);
}

/// **A curved face tangent to a hole's rim at a vertex**: the two-arc
/// holed block turned 0.4 about x, against the ball of radius `R` whose
/// sphere touches the world plane `z = z₀` from below at the turned
/// vertex, its poles along x, off the vertex. The vertex pierces the
/// sphere; the sphere's sections by the block's bottom face and by the
/// hole wall leave it along the rim's tangent, and the ring orders them
/// by how they curve in the sphere (`implicit_section_acceleration`'s
/// curved-reference term). A − B, A ∩ B and A ∪ B each hold at all three
/// tiers and at tier 3′, and their volumes agree with the operands':
/// `|A − B| + |A ∩ B| = |A|` and `|A ∪ B| + |A ∩ B| = |A| + |B|`.
///
/// Held as it stands: the bottom rim's arc is tangent to the sphere at
/// the vertex too, and the crossing layer refuses it
/// `CurvedPierceUnsupported` before the ring reads anything
/// (`work/contacthold/conic-ties-outside-the-vertex-on-face-pass-still-read-first-order.md`),
/// all three ops alike. Where it builds, every check above binds.
#[test]
fn a_ball_tangent_to_a_two_arc_holes_rim_is_held_or_builds_sound() {
    let radius = 3.0;
    let t = 0.4;
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), t);
    let v = turn.transform_point(Point3::new(0.0, 1.0, 0.0));
    let a = topo::test_support::finished(
        "the turned block",
        topo::transform_rigid(
            &two_arc_holed_block(FRAC_PI_2, TAU),
            &geom_core::test_support::rigid(&turn, tol()),
            tol(),
        )
        .unwrap(),
        tol(),
    );
    let b = topo::test_support::finished(
        "the ball",
        sweep::test_support::ball_poled(
            radius,
            Vec3::new(v.x, v.y, v.z - radius),
            Vec3::new(1.0, 0.0, 0.0),
            tol(),
        ),
        tol(),
    );
    let block = (16.0 - PI) * HEIGHT;
    let ball = 4.0 / 3.0 * PI * radius.powi(3);
    let (mut failures, mut measured, mut held) = (Vec::new(), Vec::new(), 0);
    for (op, name) in [
        (BooleanOp::Subtract, "A − B"),
        (BooleanOp::Intersect, "A ∩ B"),
        (BooleanOp::Union, "A ∪ B"),
    ] {
        let got = boolean_op_with(
            op,
            &a,
            &b,
            &topo::BooleanInputs::default(),
            SweepStrategy::Realized,
            tol(),
        );
        let out = match got {
            Ok(out) => out,
            Err(BooleanError::CurvedPierceUnsupported {
                operand: topo::boolean::Operand::A,
                edge,
                ..
            }) if matches!(
                topo::readback::edge_carrier_kind(&a, edge),
                Ok(geom::CurveKind::Circle)
            ) =>
            {
                held += 1;
                continue;
            }
            Err(e) => {
                failures.push(format!("{name}: the boolean: {e:?}"));
                continue;
            }
        };
        let Some(r) = out.body() else {
            failures.push(format!("{name}: no body"));
            continue;
        };
        let body = &r.body;
        let tiers = topo::validate(body)
            .map_err(|e| format!("tier 1: {e:?}"))
            .and_then(|()| topo::validate_closed(body).map_err(|e| format!("tier 2: {e:?}")))
            .and_then(|()| {
                topo::validate_geometric(body, tol()).map_err(|e| format!("tier 3: {e:?}"))
            })
            .and_then(|()| {
                topo::validate_pseudomanifold(body, &r.contacts, tol())
                    .map_err(|e| format!("tier 3′: {e:?}"))
            });
        if let Err(e) = tiers {
            failures.push(format!("{name}: {e}"));
        }
        let m = topo::test_support::pinned_volume(name, body, tol());
        measured.push((m.volume, m.volume_pad));
    }
    if held != 0 && held != 3 {
        failures.push(format!("{held} of 3 held: the row's pin is all or none"));
    }
    if let [(sub, sub_pad), (int, int_pad), (uni, uni_pad)] = measured[..] {
        let slack = |pads: f64, of: f64| pads + 1e-9 * of;
        if (sub + int - block).abs() > slack(sub_pad + int_pad, block) {
            failures.push(format!(
                "|A − B| + |A ∩ B| = {} against |A| = {block}",
                sub + int
            ));
        }
        if (uni + int - block - ball).abs() > slack(uni_pad + int_pad, block + ball) {
            failures.push(format!(
                "|A ∪ B| + |A ∩ B| = {} against |A| + |B| = {}",
                uni + int,
                block + ball
            ));
        }
    }
    report(&failures, 3);
}
