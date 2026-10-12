//! **A prism edge lying on a tube's wall.** The prism's
//! vertical edge is a ruling of the wall, and the two planar faces
//! through it are decided distinct from the wall, so the edge is an ON
//! event: the reduction places its ends against the wall face and splits
//! it where it crosses the face's boundary.
//!
//! - **Inside one wall face, and across the top rim**: every op in both
//!   member orders builds at its closed form, at tiers 3 and 3′.
//! - **On a seam ruling**: two rulings of one cylinder are parallel, so
//!   a ruling cannot cross a seam ruling mid-span: it shares a stretch of
//!   it or misses it. The prism turned onto either seam ruling of the
//!   two-face tube is that shared stretch, and builds the same.
//! - **In band of the wall but not on it**: the edge's ends in the
//!   sliver band of the wall escalate, a lean in the sliver band of the
//!   axis escalates, and a lean decided off the axis keeps the crossing
//!   layer's door. None is taken as ON.
//! - **Across an ellipse**: on a tube whose bottom is cut obliquely, the
//!   ruling meets the ellipse at its plane, and every op builds. A
//!   coaxial rod's rim lying on that tube's walls meets the ellipse the
//!   same way, and every op builds there too.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::test_support::{brick, corners, finished, prism_at};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{AtRestBody, BooleanError, BooleanResult};

/// The tube's radius and height.
const R: f64 = 1.0;
const H: f64 = 2.0;

/// The square's area: its diagonals are both `0.8`.
const AREA: f64 = 0.32;

/// A z-axis cylinder of radius [`R`] from `z0`, length `len`, through
/// the extrude door: two wall faces, their seam rulings at `±x`.
fn rod_z(z0: f64, len: f64) -> AtRestBody<f64> {
    rod_of(R, z0, len)
}

/// [`rod_z`] at radius `r`.
fn rod_of(r: f64, z0: f64, len: f64) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(0.0, 0.0), r, tol).unwrap();
    let plane = profile::SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let p = profile::Profile::new(plane, vec![lp.into()])
        .validate(tol)
        .unwrap();
    let rod = extrude(
        &p,
        Extrusion::Distance {
            depth: len,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body;
    finished("the rod", rod, tol)
}

/// `rod` less the half-space under the plane `z = c + slope·x`.
fn cut_below(rod: &AtRestBody<f64>, slope: f64, c: f64) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let tilt = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), -slope.atan());
    let under = topo::transform_rigid(
        &brick((-50.0, 50.0), (-50.0, 50.0), (-100.0, 0.0), tol),
        &tilt,
        tol,
    )
    .unwrap();
    let under = finished(
        "the slab under the plane",
        topo::transform_rigid(&under, &Affine3::translation(Vec3::new(0.0, 0.0, c)), tol).unwrap(),
        tol,
    );
    match topo::subtract(rod, &under, tol) {
        Ok(BooleanResult::Body(b)) => b.body,
        other => panic!("the cut rod builds: {other:?}"),
    }
}

/// The square `(1, 0)`, `(0.6, 0.4)`, `(0.2, 0)`, `(0.6, −0.4)` turned
/// `turn` rad about `z` and extruded from `z0` by `h`: its edge at
/// `(cos turn, sin turn)` lies on the tube's wall.
fn square(turn: f64, z0: f64, h: f64) -> AtRestBody<f64> {
    let (s, c) = turn.sin_cos();
    let pts: Vec<(f64, f64)> = [(R, 0.0), (0.6, 0.4), (0.2, 0.0), (0.6, -0.4)]
        .iter()
        .map(|&(x, y)| (c * x - s * y, s * x + c * y))
        .collect();
    let tol = Tol::witness();
    finished("the prism", prism_at(corners(&pts), z0, h, tol), tol)
}

/// [`rod_z`]'s tube from `z = −1` to [`H`], its bottom cut by the plane
/// `z = 0.5 + 0.2·x`: each wall face is bounded below by an ellipse arc.
fn slanted_tube() -> AtRestBody<f64> {
    cut_below(&rod_z(-1.0, H + 1.0), 0.2, 0.5)
}

/// The six ops of `t` and `b`, labelled.
fn six(
    t: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
) -> [(&'static str, Result<BooleanResult<f64>, BooleanError>); 6] {
    let tol = Tol::witness();
    [
        ("t ∪ b", topo::union(t, b, tol)),
        ("b ∪ t", topo::union(b, t, tol)),
        ("t ∖ b", topo::subtract(t, b, tol)),
        ("b ∖ t", topo::subtract(b, t, tol)),
        ("t ∩ b", topo::intersect(t, b, tol)),
        ("b ∩ t", topo::intersect(b, t, tol)),
    ]
}

/// `(faces, edges, vertices, shells)`.
type Census = (usize, usize, usize, usize);

/// What one op yields: nothing, or a body's census and its
/// `[v-v, v-f, curve, patch]` contact record counts.
type Want = Option<(Census, [usize; 6])>;

/// **The prism's edge on the wall builds every op undeclared.** Two
/// poses off the seam rulings (`0.3` and `0.5` rad) and the two seam
/// rulings themselves (`0` and `π`), each with the edge inside one wall
/// face (`z ∈ [0.5, 1.5]`) and across the top rim (`z ∈ [1, 3]`).
///
/// Inside, the union is the tube, the intersection the prism, `b ∖ t`
/// empty, and `t ∖ b` the tube with a prism-shaped void that touches
/// the wall along the edge: its two shells are recorded touching at
/// the edge's ends, v-f against the wall face off a seam ruling and,
/// on one, v-e against the ruling the join made whole again, with the
/// edge-edge record of the prism's edge lying along that ruling. Across the rim, the prism's
/// part above `z = H` is `b ∖ t`, and `t ∖ b` is a notch whose edge
/// runs down the wall from the rim, its lower end recorded the same
/// way and, on a seam ruling, the edge along it.
#[test]
fn a_prism_edge_on_the_tubes_wall_builds_every_op_undeclared() {
    let tol = Tol::witness();
    let t = rod_z(0.0, H);
    let tube = PI * R * R * H;
    let nothing = [0; 6];
    let prism: Want = Some(((6, 12, 8, 1), nothing));
    for turn in [0.3, 0.5, 0.0, PI] {
        let on_seam = turn == 0.0 || turn == PI;
        // `[∪, t ∖ b, b ∖ t, ∩]` inside one face, then across the rim.
        let inside: [Want; 4] = if on_seam {
            [
                Some(((4, 6, 4, 1), nothing)),
                Some(((10, 18, 12, 2), [0, 0, 2, 1, 0, 0])),
                None,
                prism,
            ]
        } else {
            [
                Some(((4, 6, 4, 1), nothing)),
                Some(((10, 18, 12, 2), [0, 2, 0, 0, 0, 0])),
                None,
                prism,
            ]
        };
        let notch = if on_seam {
            [0, 0, 1, 1, 0, 0]
        } else {
            [0, 1, 0, 0, 0, 0]
        };
        // On a seam ruling the prism's edge meets the rim at the tube's
        // own rim vertex, one vertex fewer.
        let rim = if on_seam {
            (9, 18, 11, 1)
        } else {
            (9, 19, 12, 1)
        };
        let across: [Want; 4] = [Some((rim, nothing)), Some((rim, notch)), prism, prism];
        for (extent, z0, h, census, [union, t_less_b, b_less_t, common]) in [
            ("inside", 0.5, 1.0, inside, [tube, tube - AREA, 0.0, AREA]),
            (
                "across",
                1.0,
                2.0,
                across,
                [tube + AREA, tube - AREA, AREA, AREA],
            ),
        ] {
            let b = square(turn, z0, h);
            let wants = [
                (census[0], union),
                (census[0], union),
                (census[1], t_less_b),
                (census[2], b_less_t),
                (census[3], common),
                (census[3], common),
            ];
            for ((op, r), (want, volume)) in six(&t, &b).into_iter().zip(wants) {
                let label = format!("turn {turn}, {extent}: {op}");
                let bb = match (r, want) {
                    (Ok(BooleanResult::Empty), None) => continue,
                    (Ok(BooleanResult::Body(bb)), Some(_)) => bb,
                    (r, _) => panic!("{label}: {want:?}: {r:?}"),
                };
                let (census, contacts) = want.unwrap();
                let body = &bb.body;
                topo::validate_geometric(body, tol)
                    .unwrap_or_else(|e| panic!("{label}: tier 3: {e:?}"));
                topo::validate_pseudomanifold(body, &bb.contacts, tol)
                    .unwrap_or_else(|e| panic!("{label}: tier 3′: {e:?}"));
                let v = topo::mass_properties(body, tol).unwrap().volume;
                assert!(
                    (v - volume).abs() <= 1e-12 * volume,
                    "{label}: the closed form: {v} vs {volume}"
                );
                assert_eq!(
                    (
                        body.faces().count(),
                        body.edges().count(),
                        body.vertices().count(),
                        body.shells().count()
                    ),
                    census,
                    "{label}: F, E, V, shells"
                );
                let c = &bb.contacts;
                assert_eq!(
                    [
                        c.vertex_vertex().count(),
                        c.vertex_face().count(),
                        c.vertex_edge().count(),
                        c.edge_edge().count(),
                        c.edge_face().count(),
                        c.face_face().count()
                    ],
                    contacts,
                    "{label}: [v-v, v-f, v-e, e-e, e-f, f-f] records"
                );
                assert_eq!(
                    topo::joinable_vertices(body, geom_core::Band::linear(tol).unwrap()).unwrap(),
                    vec![],
                    "{label}: maximal edges"
                );
            }
        }
    }
}

/// **A ruling in band of the wall but not on it is never taken as ON.**
/// The `0.3` rad prism, both extents, moved off the wall three ways, by
/// multiples of the band's zero threshold `z` (the sliver band runs from
/// `z` to `10·z`):
///
/// - **shifted** `±3·z` along the wall's normal: its ends sit in the
///   sliver band of the wall, and the vertex placement escalates;
/// - **leaned** about the wall's normal through its lower end, so the
///   upper end slides `±3·z` along the wall: both ends are on it, and
///   whether the edge runs along the axis escalates;
/// - **leaned** `±40·z`: the edge is decided off the axis, over its
///   whole span and over either half the rim splits it into: a chord
///   whose two crossings the root door cannot separate, which keeps the
///   door.
#[test]
fn a_ruling_in_band_of_the_wall_but_not_on_it_is_not_on() {
    let tol = Tol::witness();
    let z = geom_core::Band::linear(tol).unwrap().zero();
    let t = rod_z(0.0, H);
    let turn: f64 = 0.3;
    let (s, c) = turn.sin_cos();
    let normal = Vec3::new(c, s, 0.0);
    for (z0, h) in [(0.5, 1.0), (1.0, 2.0)] {
        let foot = Point3::new(c, s, z0);
        let lean = |slide: f64| Affine3::rotation_about_axis(foot, normal, slide / h);
        for d in [3.0 * z, -3.0 * z] {
            for (how, xf) in [
                ("shifted", Affine3::translation(normal * d)),
                ("leaned", lean(d)),
            ] {
                let b = finished(
                    "the moved prism",
                    topo::transform_rigid(&square(turn, z0, h), &xf, tol).unwrap(),
                    tol,
                );
                for (op, r) in six(&t, &b) {
                    let label = format!("z0 = {z0}, {how} {d}: {op}");
                    let Err(BooleanError::Escalated { decision, .. }) = r else {
                        panic!("{label}: escalates: {r:?}");
                    };
                    if how == "leaned" {
                        assert_eq!(
                            decision,
                            topo::BooleanDecision::WallRoots(topo::WallRung::AxisParallel),
                            "{label}: on whether the edge runs along the axis"
                        );
                    }
                }
            }
        }
        for d in [40.0 * z, -40.0 * z] {
            let b = finished(
                "the leaned prism",
                topo::transform_rigid(&square(turn, z0, h), &lean(d), tol).unwrap(),
                tol,
            );
            for (op, r) in six(&t, &b) {
                let label = format!("z0 = {z0}, leaned {d}: {op}");
                assert!(
                    matches!(r, Err(BooleanError::CurvedPierceUnsupported { .. })),
                    "{label}: the crossing layer's door: {r:?}"
                );
            }
        }
    }
}

/// **A ruling across an ellipse builds every op undeclared.** The tube
/// from `z = −1` with its bottom cut by the plane `z = 0.5 + 0.2·x`, so
/// each wall face is bounded below by an ellipse arc, and the prism
/// from `z = 0` to `1`, its edge on the wall running out through that
/// plane. The ruling meets the ellipse where it meets the ellipse's
/// plane and is split there. The tube holds `π·R²·1.5`; the prism above
/// the plane holds `A·(0.5 − 0.2·x̄)`, `x̄ = 0.6·cos turn` its
/// centroid's `x`. `t ∖ b` is a notch whose edge runs up the wall from
/// the ellipse, its upper end recorded on the wall face. The wall's
/// ellipse trim is measured by certified quadrature, so each volume is
/// read to the enclosure's own half-width.
#[test]
fn a_ruling_across_an_ellipse_builds_every_op_undeclared() {
    let tol = Tol::witness();
    let t = slanted_tube();
    let tube = PI * R * R * 1.5;
    let (whole, prism) = ((9, 19, 12, 1), (6, 12, 8, 1));
    for turn in [0.3, 0.5_f64] {
        let common = AREA * (0.5 - 0.2 * 0.6 * turn.cos());
        let wants = [
            (tube + AREA - common, whole, [0; 4]),
            (tube + AREA - common, whole, [0; 4]),
            (tube - common, whole, [0, 1, 0, 0]),
            (AREA - common, prism, [0; 4]),
            (common, prism, [0; 4]),
            (common, prism, [0; 4]),
        ];
        for ((op, r), (volume, census, contacts)) in
            six(&t, &square(turn, 0.0, 1.0)).into_iter().zip(wants)
        {
            let label = format!("turn {turn}: {op}");
            let Ok(BooleanResult::Body(bb)) = r else {
                panic!("{label}: builds: {r:?}");
            };
            let body = &bb.body;
            topo::validate_geometric(body, tol)
                .unwrap_or_else(|e| panic!("{label}: tier 3: {e:?}"));
            topo::validate_pseudomanifold(body, &bb.contacts, tol)
                .unwrap_or_else(|e| panic!("{label}: tier 3′: {e:?}"));
            let m = topo::mass_properties(body, tol).unwrap();
            assert!(
                (m.volume - volume).abs() <= m.volume_pad + 1e-12,
                "{label}: the closed form: {} ± {} vs {volume}",
                m.volume,
                m.volume_pad
            );
            assert_eq!(
                (
                    body.faces().count(),
                    body.edges().count(),
                    body.vertices().count(),
                    body.shells().count()
                ),
                census,
                "{label}: F, E, V, shells"
            );
            let c = &bb.contacts;
            assert_eq!(
                [
                    c.vertex_vertex().count(),
                    c.vertex_face().count(),
                    c.edge_face().count(),
                    c.face_face().count()
                ],
                contacts,
                "{label}: [v-v, v-f, e-f, f-f] records"
            );
        }
    }
}

/// **A continuation on a wall an ellipse bounds builds every op.** The
/// slanted tube and a coaxial rod of the same radius (`z ∈ [1, 3]`,
/// turned `0.4` rad), every pair of their wall faces one carrier: the
/// rod's lower rim lies on the tube's wall faces, which an ellipse
/// bounds, and the interior question reads that ellipse through its
/// plane. With the tube `1.5π` and the rod `2π`, overlapping in the
/// unit-height cylinder `z ∈ [1, 2]`: the union is `2.5π`, `t ∖ r` the
/// tube below `z = 1` (`0.5π`), `r ∖ t` the rod above `z = 2` (`π`),
/// and the intersection `π`.
#[test]
fn a_continuation_on_a_wall_bounded_by_an_ellipse_builds_every_op() {
    let tol = Tol::witness();
    let t = slanted_tube();
    let spin = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), 0.4);
    let r = finished(
        "the rod",
        topo::transform_rigid(&rod_z(1.0, H), &spin, tol).unwrap(),
        tol,
    );
    // The union's two wall pairs meet on a four-arc circle; every other
    // result is a two-faced wall between two caps.
    let (whole, part): (Census, Census) = ((6, 12, 8, 1), (4, 6, 4, 1));
    for (op, res, volume, census) in [
        ("t ∪ r", topo::union(&t, &r, tol), 2.5 * PI, whole),
        ("r ∪ t", topo::union(&r, &t, tol), 2.5 * PI, whole),
        ("t ∖ r", topo::subtract(&t, &r, tol), 0.5 * PI, part),
        ("r ∖ t", topo::subtract(&r, &t, tol), PI, part),
        ("t ∩ r", topo::intersect(&t, &r, tol), PI, part),
        ("r ∩ t", topo::intersect(&r, &t, tol), PI, part),
    ] {
        let Ok(BooleanResult::Body(bb)) = res else {
            panic!("{op}: builds: {res:?}");
        };
        let body = &bb.body;
        topo::validate_geometric(body, tol).unwrap_or_else(|e| panic!("{op}: tier 3: {e:?}"));
        topo::validate_pseudomanifold(body, &bb.contacts, tol)
            .unwrap_or_else(|e| panic!("{op}: tier 3′: {e:?}"));
        let m = topo::mass_properties(body, tol).unwrap();
        assert!(
            (m.volume - volume).abs() <= m.volume_pad + 1e-12,
            "{op}: the closed form: {} ± {} vs {volume}",
            m.volume,
            m.volume_pad
        );
        assert_eq!(
            (
                body.faces().count(),
                body.edges().count(),
                body.vertices().count(),
                body.shells().count()
            ),
            census,
            "{op}: F, E, V, shells"
        );
    }
}

/// **A rim across a wall's ellipse builds every op in both orders.**
/// The slanted tube and a coaxial rod of the same radius from
/// `z = 0.5`, turned about `z` through a family of angles, their walls
/// one carrier: the rod's lower rim crosses the tube's ellipse where
/// `x = 0`, so each is split at the other's crossing, read where the
/// circle's plane meets the ellipse's. The overlap is the tube less the
/// part under `z = 0.5`, `1.5π − 2/15`; `t ∖ r` is that part, `2/15`,
/// `r ∖ t` the rod above `z = 2` and the part under the slant,
/// `0.5π + 2/15`, and the union `2π + 2/15`. Whichever member's wall a
/// union keeps, the piece of the other's rim lying across it (the
/// ellipse's arc over `x > 0` in `t ∪ r`, the circle's over `x < 0` in
/// `r ∪ t`) is described in the kept wall's chart.
#[test]
fn a_rim_across_a_walls_ellipse_builds_every_op_in_both_orders() {
    let tol = Tol::witness();
    let t = slanted_tube();
    let common = 1.5 * PI - 2.0 / 15.0;
    for turn in TURNS {
        let spin = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), turn);
        let r = finished(
            "the rod",
            topo::transform_rigid(&rod_z(0.5, H), &spin, tol).unwrap(),
            tol,
        );
        let label = |op: &str| format!("turn {turn}: {op}");
        let union = 2.0 * PI + 2.0 / 15.0;
        for (op, res, volume) in [
            ("t ∪ r", topo::union(&t, &r, tol), union),
            ("r ∪ t", topo::union(&r, &t, tol), union),
            ("t ∖ r", topo::subtract(&t, &r, tol), 2.0 / 15.0),
            ("r ∖ t", topo::subtract(&r, &t, tol), 0.5 * PI + 2.0 / 15.0),
            ("t ∩ r", topo::intersect(&t, &r, tol), common),
            ("r ∩ t", topo::intersect(&r, &t, tol), common),
        ] {
            let Ok(BooleanResult::Body(bb)) = res else {
                panic!("{}: builds: {res:?}", label(op));
            };
            topo::validate_geometric(&bb.body, tol)
                .unwrap_or_else(|e| panic!("{}: tier 3: {e:?}", label(op)));
            topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol)
                .unwrap_or_else(|e| panic!("{}: tier 3′: {e:?}", label(op)));
            let m = topo::mass_properties(&bb.body, tol).unwrap();
            assert!(
                (m.volume - volume).abs() <= m.volume_pad + 1e-9 * volume,
                "{}: the closed form: {} ± {} vs {volume}",
                label(op),
                m.volume,
                m.volume_pad
            );
        }
    }
}

/// The rod's turns about `z`: its seam rulings on either side of the
/// crossings at `(0, ±1)`, near them and far.
const TURNS: [f64; 6] = [0.0, 0.4, 1.3, 1.5, 2.0, 2.9];

/// **Two sections the band reads coplanar are not taken for one
/// curve.** Two radius-`0.01` rods on one axis, each cut below a plane
/// through the origin, of slopes `100` and `100(1 + d)`: their ellipse
/// rims cross at `(0, ±0.01, 0)`. With `d = k·ε` the planes' tilt
/// carries about `k·ε` across the ellipses, so the band reads them
/// parallel and through one point, and their one-curve margin is about `3·k·ε`
/// at most. At `k = 0.5` and `0.9` every op refuses `Escalated` on
/// `bool_carrier_cross_ellipses_one_curve`, the two rims being neither
/// one curve nor apart within the band; at `k = 0.1` they are one curve
/// within it, and every op refuses further on. None reads the rims'
/// crossings away.
#[test]
fn two_rims_the_band_reads_coplanar_are_not_one_curve() {
    let tol = Tol::witness();
    let rod = rod_of(0.01, -3.0, 6.0);
    for k in [0.1, 0.5, 0.9] {
        let d = k * tol.eps();
        let (a, b) = (
            cut_below(&rod, 100.0, 0.0),
            cut_below(&rod, 100.0 * (1.0 + d), 0.0),
        );
        for (op, res) in six(&a, &b) {
            let label = format!("k = {k}: {op}");
            if k > 0.2 {
                let Err(BooleanError::Escalated { diag, .. }) = &res else {
                    panic!("{label}: escalates: {res:?}");
                };
                assert_eq!(
                    diag.predicate,
                    Some("bool_carrier_cross_ellipses_one_curve"),
                    "{label}: on the one-curve row"
                );
            } else {
                assert!(res.is_err(), "{label}: refuses: {res:?}");
            }
        }
    }
}
