//! **A prism edge lying on a tube's wall, undeclared.** The prism's
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
//!   ruling meets the ellipse at its plane, and every op builds.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::outcomes::outcome;
use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::test_support::{brick, corners, finished, prism_at};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{AtRestBody, BooleanDeclarations, BooleanError, BooleanResult};

/// The tube's radius and height.
const R: f64 = 1.0;
const H: f64 = 2.0;

/// The square's area: its diagonals are both `0.8`.
const AREA: f64 = 0.32;

/// A z-axis cylinder of radius [`R`] from `z0`, length `len`, through
/// the extrude door: two wall faces, their seam rulings at `±x`.
fn rod_z(z0: f64, len: f64) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(0.0, 0.0), R, tol).unwrap();
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
    let tol = Tol::witness();
    let tilt = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), -(0.2_f64).atan());
    let under = topo::transform_rigid(
        &brick((-3.0, 3.0), (-3.0, 3.0), (-5.0, 0.0), tol),
        &tilt,
        tol,
    )
    .unwrap();
    let under = finished(
        "the slab under the plane",
        topo::transform_rigid(&under, &Affine3::translation(Vec3::new(0.0, 0.0, 0.5)), tol)
            .unwrap(),
        tol,
    );
    match topo::subtract(&rod_z(-1.0, H + 1.0), &under, tol) {
        Ok(BooleanResult::Body(b)) => b.body,
        other => panic!("the slanted tube builds: {other:?}"),
    }
}

/// The six ops of `t` and `b`, labelled.
fn six(
    t: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
) -> [(&'static str, Result<BooleanResult<f64>, BooleanError>); 6] {
    let (tol, none) = (Tol::witness(), BooleanDeclarations::none());
    [
        ("t ∪ b", topo::union_with(t, b, &none, tol)),
        ("b ∪ t", topo::union_with(b, t, &none, tol)),
        ("t ∖ b", topo::subtract_with(t, b, &none, tol)),
        ("b ∖ t", topo::subtract_with(b, t, &none, tol)),
        ("t ∩ b", topo::intersect_with(t, b, &none, tol)),
        ("b ∩ t", topo::intersect_with(b, t, &none, tol)),
    ]
}

/// The wall (cylinder) faces of `b`.
fn walls(b: &AtRestBody<f64>) -> Vec<topo::FaceKey> {
    b.faces()
        .filter(|(_, f)| {
            matches!(
                b.get_surface(f.surface),
                Some(geom::Surface::Cylinder { .. })
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// Every pair of `x`'s and `y`'s wall faces declared a continuation.
fn continued(x: &AtRestBody<f64>, y: &AtRestBody<f64>) -> BooleanDeclarations {
    let mut d = BooleanDeclarations::none();
    for fa in walls(x) {
        for fb in walls(y) {
            d.coincident_faces.push(topo::FacePairDeclaration::new(
                fa,
                fb,
                topo::BooleanCoincidence::Continuation,
            ));
        }
    }
    d
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
/// on one, v-e against the ruling the join made whole again. Across the rim, the prism's
/// part above `z = H` is `b ∖ t`, and `t ∖ b` is a notch whose edge
/// runs down the wall from the rim, its lower end recorded the same
/// way.
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
                Some(((10, 18, 12, 2), [0, 0, 2, 0, 0, 0])),
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
            [0, 0, 1, 0, 0, 0]
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
                        c.vv.len(),
                        c.a_on_b.len() + c.b_on_a.len(),
                        c.ve.len(),
                        c.ee.len(),
                        c.curves.len(),
                        c.patches.len()
                    ],
                    contacts,
                    "{label}: [v-v, v-f, v-e, e-e, curve, patch] records"
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
                    c.vv.len(),
                    c.a_on_b.len() + c.b_on_a.len(),
                    c.curves.len(),
                    c.patches.len()
                ],
                contacts,
                "{label}: [v-v, v-f, curve, patch] records"
            );
        }
    }
}

/// **A ruling whose own face shares the wall's carrier glues as the
/// continuation it is.** A long tube `t` (`z ∈ [0, 4]`) and a coaxial
/// tube `b` of the same radius (`z ∈ [1, 3]`, turned `0.4` rad so no
/// seam ruling of one lies on the other's), their caps apart. `t`'s
/// seam rulings lie on `b`'s wall, and so does each of its wall faces:
/// the ruling is not a curve where two carriers meet, and the lying-on
/// lane does not take it. The walls are one carrier by margin, so every
/// op, undeclared, is the op with every wall pair declared a
/// continuation (D10).
#[test]
fn a_ruling_whose_face_shares_the_walls_carrier_is_the_declared_continuation() {
    let tol = Tol::witness();
    let t = rod_z(0.0, 4.0);
    let spin = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), 0.4);
    let b = finished(
        "the short tube",
        topo::transform_rigid(&rod_z(1.0, H), &spin, tol).unwrap(),
        tol,
    );
    let (tb, bt) = (continued(&t, &b), continued(&b, &t));
    let declared = [
        topo::union_with(&t, &b, &tb, tol),
        topo::union_with(&b, &t, &bt, tol),
        topo::subtract_with(&t, &b, &tb, tol),
        topo::subtract_with(&b, &t, &bt, tol),
        topo::intersect_with(&t, &b, &tb, tol),
        topo::intersect_with(&b, &t, &bt, tol),
    ];
    for ((op, r), want) in six(&t, &b).into_iter().zip(declared) {
        assert_eq!(
            outcome(&r),
            outcome(&want),
            "{op}: the declared continuation"
        );
    }
}

/// **The declared one-carrier arms keep their reach.** The slanted tube
/// and a coaxial rod of the same radius (`z ∈ [1, 3]`, turned `0.4`
/// rad), every pair of their wall faces declared a continuation: the
/// rod's lower rim lies on the tube's wall faces, which an ellipse
/// bounds. The declared arms' interior question reads lines and circles
/// only, so every op refuses on that rim, against a wall face of the
/// tube, as it did before the lying-on lane read ellipses.
#[test]
fn a_declared_continuation_on_a_wall_bounded_by_an_ellipse_keeps_the_door() {
    let tol = Tol::witness();
    let t = slanted_tube();
    let spin = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), 0.4);
    let r = finished(
        "the rod",
        topo::transform_rigid(&rod_z(1.0, H), &spin, tol).unwrap(),
        tol,
    );
    let (tr, rt) = (continued(&t, &r), continued(&r, &t));
    let (a, b) = (topo::Operand::A, topo::Operand::B);
    for (op, res, rod_is) in [
        ("t ∪ r", topo::union_with(&t, &r, &tr, tol), b),
        ("r ∪ t", topo::union_with(&r, &t, &rt, tol), a),
        ("t ∖ r", topo::subtract_with(&t, &r, &tr, tol), b),
        ("r ∖ t", topo::subtract_with(&r, &t, &rt, tol), a),
        ("t ∩ r", topo::intersect_with(&t, &r, &tr, tol), b),
        ("r ∩ t", topo::intersect_with(&r, &t, &rt, tol), a),
    ] {
        let Err(BooleanError::CurvedPierceUnsupported { operand, face, .. }) = res else {
            panic!("{op}: the crossing layer's door: {res:?}");
        };
        assert_eq!(operand, rod_is, "{op}: on the rod's edge");
        assert!(
            walls(&t).contains(&face),
            "{op}: against a wall of the tube"
        );
    }
}
