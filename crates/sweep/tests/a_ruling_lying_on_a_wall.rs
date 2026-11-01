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
//!   same way, and every op builds there too, as do two oblique bottoms
//!   whose rims cross on one wall. Two rims in planes the band reads
//!   coplanar are one curve only within the band.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::{CurveKind, Surface};
use geom_brep::EdgeDescription;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::test_support::{brick, corners, finished, prism_at};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{AtRestBody, BooleanDecision, BooleanError, BooleanResult, Coincide, DeclarationRead};

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
    let plane = profile::SketchPlane::xy_at(Point3::new(0.0, 0.0, z0));
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
        &geom_core::test_support::rigid(&tilt, tol),
        tol,
    )
    .unwrap();
    let under = finished(
        "the slab under the plane",
        topo::transform_rigid(
            &under,
            &geom_core::RigidMotion::translation(Vec3::new(0.0, 0.0, c)).unwrap(),
            tol,
        )
        .unwrap(),
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

/// **The prism's edge on the wall builds every op undeclared**, but
/// `t ∖ b` inside one face and on a seam ruling. Two
/// poses off the seam rulings (`0.3` and `0.5` rad) and the two seam
/// rulings themselves (`0` and `π`), each with the edge inside one wall
/// face (`z ∈ [0.5, 1.5]`) and across the top rim (`z ∈ [1, 3]`).
///
/// Inside, the union is the tube, the intersection the prism, `b ∖ t`
/// empty, and `t ∖ b` the tube with a prism-shaped void that touches
/// the wall along the edge, the void's two faces leaving it into the
/// wall's material. Off a seam ruling the edge is two, a ring of no area
/// in the wall, which refuses `EdgeInFace` (`CurvedFace`); the census
/// and the v-f rests it pinned are owed back with the ring
/// (`work/splitdoor/the-boolean-builds-the-slit-on-a-curved-face-once-it-is-meshed-and-measured.md`).
/// Across the rim, the prism's part above `z = H` is `b ∖ t`, and
/// `t ∖ b` is a notch whose edge runs down the wall from the rim: off a
/// seam ruling two edges, a slit of the curved wall, which refuses
/// `CurvedFace` as well. On a seam ruling `t ∖ b` refuses
/// `LineWedges`, inside and across: its result keeps the seam edge
/// (180°) and the void's edge along it (270°) on one line, overlapping,
/// which every tier passed; the material there is two 45° wedges, each
/// between a void face and the wall. It returns, with the v-e and
/// edge-edge records it pinned against the ruling, with the re-pairing
/// (`work/topo/revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet.md`).
#[test]
fn a_prism_edge_on_the_tubes_wall_builds_every_op_but_its_parting() {
    let tol = Tol::witness();
    let t = rod_z(0.0, H);
    let tube = PI * R * R * H;
    let nothing = [0; 6];
    let prism: Want = Some(((6, 12, 8, 1), nothing));
    for turn in [0.3, 0.5, 0.0, PI] {
        let on_seam = turn == 0.0 || turn == PI;
        // `[∪, t ∖ b, b ∖ t, ∩]` inside one face, then across the rim. A
        // `t ∖ b` that refuses (below) holds nothing here.
        let inside: [Want; 4] = [Some(((4, 6, 4, 1), nothing)), None, None, prism];
        // On a seam ruling the prism's edge meets the rim at the tube's
        // own rim vertex, one vertex fewer.
        let rim = if on_seam {
            (9, 18, 11, 1)
        } else {
            (9, 19, 12, 1)
        };
        // Off a seam ruling `t ∖ b` refuses (below) and holds nothing here.
        let across: [Want; 4] = [Some((rim, nothing)), None, prism, prism];
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
                if !on_seam && op == "t ∖ b" {
                    assert!(
                        matches!(
                            r,
                            Err(BooleanError::EdgeInFace {
                                gap: topo::EdgeInFaceGap::CurvedFace,
                                ..
                            })
                        ),
                        "{label}: {r:?}, not the curved face's refusal"
                    );
                    continue;
                }
                if on_seam && op == "t ∖ b" {
                    assert!(
                        matches!(
                            r,
                            Err(BooleanError::LineWedges {
                                fault: topo::LineWedgeFault::Overlap { .. },
                                ..
                            })
                        ),
                        "{label}: {r:?}, not the overlap's refusal"
                    );
                    continue;
                }
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
                let v = topo::test_support::pinned_volume("the measured body", body, tol).volume;
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
///   whether the edge runs along the axis escalates, at the edge and the
///   wall (`CurvedPierceEscalated`);
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
                    topo::transform_rigid(
                        &square(turn, z0, h),
                        &geom_core::test_support::rigid(&xf, tol),
                        tol,
                    )
                    .unwrap(),
                    tol,
                );
                for (op, r) in six(&t, &b) {
                    let label = format!("z0 = {z0}, {how} {d}: {op}");
                    // The span's door reads the lean in band, located at
                    // the edge and the wall; the shift, elsewhere.
                    let decision = match r {
                        Err(BooleanError::CurvedPierceEscalated { decision, .. })
                            if how == "leaned" =>
                        {
                            decision
                        }
                        Err(BooleanError::Escalated { decision, .. }) => decision,
                        r => panic!("{label}: escalates: {r:?}"),
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
                topo::transform_rigid(
                    &square(turn, z0, h),
                    &geom_core::test_support::rigid(&lean(d), tol),
                    tol,
                )
                .unwrap(),
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

/// **A ruling across an ellipse builds every op undeclared but `t ∖ b`.** The tube
/// from `z = −1` with its bottom cut by the plane `z = 0.5 + 0.2·x`, so
/// each wall face is bounded below by an ellipse arc, and the prism
/// from `z = 0` to `1`, its edge on the wall running out through that
/// plane. The ruling meets the ellipse where it meets the ellipse's
/// plane and is split there. The tube holds `π·R²·1.5`; the prism above
/// the plane holds `A·(0.5 − 0.2·x̄)`, `x̄ = 0.6·cos turn` its
/// centroid's `x`. `t ∖ b` is a notch whose edge runs up the wall from
/// the ellipse, both its faces going into the wall's material, so it is
/// two edges, a slit of the curved wall, which refuses `EdgeInFace`
/// (`CurvedFace`) rather than ship one edge
/// (`work/splitdoor/the-boolean-builds-the-slit-on-a-curved-face-once-it-is-meshed-and-measured.md`). The wall's
/// ellipse trim is measured by certified quadrature, so each volume is
/// read to the enclosure's own half-width.
#[test]
fn a_ruling_across_an_ellipse_builds_every_op_but_its_slit() {
    let tol = Tol::witness();
    let t = slanted_tube();
    let tube = PI * R * R * 1.5;
    let (whole, prism) = ((9, 19, 12, 1), (6, 12, 8, 1));
    for turn in [0.3, 0.5_f64] {
        let common = AREA * (0.5 - 0.2 * 0.6 * turn.cos());
        let wants = [
            (tube + AREA - common, whole, [0; 4]),
            (tube + AREA - common, whole, [0; 4]),
            (tube - common, (0, 0, 0, 0), [0; 4]),
            (AREA - common, prism, [0; 4]),
            (common, prism, [0; 4]),
            (common, prism, [0; 4]),
        ];
        for ((op, r), (volume, census, contacts)) in
            six(&t, &square(turn, 0.0, 1.0)).into_iter().zip(wants)
        {
            let label = format!("turn {turn}: {op}");
            if op == "t ∖ b" {
                assert!(
                    matches!(
                        r,
                        Err(BooleanError::EdgeInFace {
                            gap: topo::EdgeInFaceGap::CurvedFace,
                            ..
                        })
                    ),
                    "{label}: {r:?}, not the curved slit's refusal"
                );
                continue;
            }
            let Ok(BooleanResult::Body(bb)) = r else {
                panic!("{label}: builds: {r:?}");
            };
            let body = &bb.body;
            topo::validate_geometric(body, tol)
                .unwrap_or_else(|e| panic!("{label}: tier 3: {e:?}"));
            topo::validate_pseudomanifold(body, &bb.contacts, tol)
                .unwrap_or_else(|e| panic!("{label}: tier 3′: {e:?}"));
            let m = topo::mass_properties(body, tol, topo::test_support::pinned_resolution(tol))
                .unwrap();
            assert!(
                (m.enclosure.volume - volume).abs() <= m.enclosure.volume_pad + 1e-12,
                "{label}: the closed form: {} ± {} vs {volume}",
                m.enclosure.volume,
                m.enclosure.volume_pad
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
        topo::transform_rigid(
            &rod_z(1.0, H),
            &geom_core::test_support::rigid(&spin, tol),
            tol,
        )
        .unwrap(),
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
        let m =
            topo::mass_properties(body, tol, topo::test_support::pinned_resolution(tol)).unwrap();
        assert!(
            (m.enclosure.volume - volume).abs() <= m.enclosure.volume_pad + 1e-12,
            "{op}: the closed form: {} ± {} vs {volume}",
            m.enclosure.volume,
            m.enclosure.volume_pad
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
/// `r ∪ t`) rests in the first wall record's chart, derived
/// ([`across_a_wall`]).
#[test]
fn a_rim_across_a_walls_ellipse_builds_every_op_in_both_orders() {
    let tol = Tol::witness();
    let t = slanted_tube();
    let common = 1.5 * PI - 2.0 / 15.0;
    for turn in TURNS {
        let spin = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), turn);
        let r = finished(
            "the rod",
            topo::transform_rigid(
                &rod_z(0.5, H),
                &geom_core::test_support::rigid(&spin, tol),
                tol,
            )
            .unwrap(),
            tol,
        );
        let label = |op: &str| format!("turn {turn}: {op}");
        let union = 2.0 * PI + 2.0 / 15.0;
        for (op, res, volume, across) in [
            (
                "t ∪ r",
                topo::union(&t, &r, tol),
                union,
                Some(CurveKind::Ellipse),
            ),
            (
                "r ∪ t",
                topo::union(&r, &t, tol),
                union,
                Some(CurveKind::Circle),
            ),
            ("t ∖ r", topo::subtract(&t, &r, tol), 2.0 / 15.0, None),
            (
                "r ∖ t",
                topo::subtract(&r, &t, tol),
                0.5 * PI + 2.0 / 15.0,
                None,
            ),
            ("t ∩ r", topo::intersect(&t, &r, tol), common, None),
            ("r ∩ t", topo::intersect(&r, &t, tol), common, None),
        ] {
            let Ok(BooleanResult::Body(bb)) = res else {
                panic!("{}: builds: {res:?}", label(op));
            };
            topo::validate_geometric(&bb.body, tol)
                .unwrap_or_else(|e| panic!("{}: tier 3: {e:?}", label(op)));
            topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol)
                .unwrap_or_else(|e| panic!("{}: tier 3′: {e:?}", label(op)));
            let m =
                topo::mass_properties(&bb.body, tol, topo::test_support::pinned_resolution(tol))
                    .unwrap();
            assert!(
                (m.enclosure.volume - volume).abs() <= m.enclosure.volume_pad + 1e-9 * volume,
                "{}: the closed form: {} ± {} vs {volume}",
                label(op),
                m.enclosure.volume,
                m.enclosure.volume_pad
            );
            let kinds = across_a_wall(&bb.body, &label(op));
            if let Some(kind) = across {
                assert!(
                    kinds.contains(&kind),
                    "{}: a {kind:?} piece lies across the kept wall: {kinds:?}",
                    label(op)
                );
            }
        }
    }
}

/// The rod's turns about `z`: its seam rulings on the crossings at
/// `(0, ±1)` (`π/2`, `3π/2`), near them, and away from them.
const TURNS: [f64; 9] = [0.0, 0.4, 1.3, 1.5, PI / 2.0, 2.0, 2.9, PI, 1.5 * PI];

/// The carrier kind of each edge of `body` lying across a wall: both
/// its faces on a cylinder, the two faces on different members' walls
/// (a member's own seam ruling bounds two faces of one surface). Each
/// edge of both kinds is first checked to rest where it lies — a seam
/// ruling in its one surface's chart, an edge across a wall in the
/// chart of the first of its faces' two records (they are one carrier,
/// and incidence on both is incidence on one: `topo::rest_locus`) — and
/// each lying across a wall to be derived: nothing in these
/// scenes declared a rim or a ruling, so a describer that stamps a
/// declaration on one, or leaves it citing a surface it no longer
/// bounds, reds here.
fn across_a_wall(body: &topo::Body<f64>, label: &str) -> Vec<CurveKind> {
    let mut kinds = Vec::new();
    for (key, edge) in body.edges() {
        let surface = |he| {
            let face = body.face_of_half_edge(he).unwrap();
            body.get_face(face).unwrap().surface
        };
        let (sp, sm) = (surface(edge.he_plus), surface(edge.he_minus));
        let cylinder = |k| matches!(body.get_surface(k), Some(Surface::Cylinder { .. }));
        let c = body
            .get_curve_geom(edge.curve)
            .and_then(topo::CurveGeom::certified)
            .unwrap();
        if !(cylinder(sp) && cylinder(sm)) {
            continue;
        }
        match c.description() {
            EdgeDescription::Chart(ch) if sp == sm => assert_eq!(
                ch.surface, sp,
                "{label}: {key:?} rests in its one surface's chart"
            ),
            EdgeDescription::Chart(ch) => assert_eq!(
                ch.surface,
                geom_brep::SurfacePair::new(sp, sm).keys()[0],
                "{label}: {key:?} across a wall rests in its first record's chart"
            ),
            other => panic!("{label}: {key:?} rests where it lies, got {other:?}"),
        }
        if sp != sm {
            assert!(
                !c.authority().is_declared(),
                "{label}: {key:?} ({:?}) is derived, not declared: {:?}",
                c.carrier().kind(),
                c.authority()
            );
            kinds.push(c.carrier().kind());
        }
    }
    kinds
}

/// **A half rod's rim across a rod's wall moves with the half rod's
/// flat face.** The half rod `y ≥ 0` on `z ∈ [0, 2]` and a coaxial rod
/// of the same radius on `z ∈ [1, 3]`, turned `1.3` rad so its seam
/// rulings miss the flat face's corners, their walls one carrier. Its
/// volume is `2.5π` in both orders, and what lies across the kept wall
/// rests in the first wall record's chart ([`across_a_wall`]):
///
/// - **Rod first** keeps the rod's wall whole: the rod's lower rim lies
///   across it over `y > 0`. The flat face `y = 0` under the rod, moved
///   by `d` along its outward normal (`−y`), adds the strip of the disc
///   between it and `y = −d` over `z ∈ [0, 1]`:
///   `2.5π + d·√(1 − d²) + asin d`. The move carries the rim's ends, so
///   a declaration stamped on the rim, which nothing stated, would have
///   to follow a move that is no rigid shift of it, and refuse.
/// - **Half rod first** keeps the half rod's wall whole: its top rim and
///   its flat face's two rulings over `z ∈ [1, 2]` lie across it, the
///   rulings restated as derived lines. Its flat face does not move: a
///   ruling of the face continues across the wall above it, so the
///   corner between them has no one place to go, and the door refuses
///   that.
#[test]
fn a_half_rods_rim_across_a_rods_wall_moves_with_its_flat_face() {
    let tol = Tol::witness();
    let under = finished(
        "the half-space under y = 0",
        brick((-50.0, 50.0), (-50.0, 0.0), (-50.0, 50.0), tol),
        tol,
    );
    let half = match topo::subtract(&rod_z(0.0, 2.0), &under, tol) {
        Ok(BooleanResult::Body(b)) => b.body,
        other => panic!("the half rod builds: {other:?}"),
    };
    let spin = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), 1.3);
    let rod = finished(
        "the rod",
        topo::transform_rigid(
            &rod_z(1.0, 2.0),
            &geom_core::test_support::rigid(&spin, tol),
            tol,
        )
        .unwrap(),
        tol,
    );
    for (op, res, rulings, moves) in [
        ("rod ∪ half", topo::union(&rod, &half, tol), 0, true),
        ("half ∪ rod", topo::union(&half, &rod, tol), 2, false),
    ] {
        let Ok(BooleanResult::Body(bb)) = res else {
            panic!("{op}: builds: {res:?}");
        };
        topo::validate_geometric(&bb.body, tol).unwrap_or_else(|e| panic!("{op}: tier 3: {e:?}"));
        let m = topo::mass_properties(&bb.body, tol, topo::test_support::pinned_resolution(tol))
            .unwrap();
        assert!(
            (m.enclosure.volume - 2.5 * PI).abs() <= m.enclosure.volume_pad + 1e-9,
            "{op}: the closed form: {} ± {} vs 2.5π",
            m.enclosure.volume,
            m.enclosure.volume_pad
        );
        let kinds = across_a_wall(&bb.body, op);
        assert!(
            kinds.contains(&CurveKind::Circle),
            "{op}: a rim lies across the kept wall: {kinds:?}"
        );
        assert_eq!(
            kinds.iter().filter(|&&k| k == CurveKind::Line).count(),
            rulings,
            "{op}: the flat face's rulings lying across the kept wall: {kinds:?}"
        );
        if !moves {
            continue;
        }
        // The plane `y = 0`, read within a tolerance on its stored data.
        let on_flat = |k| match bb.body.get_surface(k) {
            Some(Surface::Plane { origin, normal, .. }) => {
                normal.dot(Vec3::unit_y()).abs() >= 1.0 - 1e-12 && origin.y.abs() <= 1e-12
            }
            _ => false,
        };
        let flat: Vec<_> = bb
            .body
            .faces()
            .filter(|(_, f)| on_flat(f.surface))
            .map(|(k, _)| k)
            .collect();
        let [flat] = flat[..] else {
            panic!("{op}: one face on the plane y = 0: {flat:?}");
        };
        for d in [0.05, -0.05] {
            let mut moved = bb.body.clone().into_body();
            topo::replace_face_offset(&mut moved, flat, d, tol)
                .unwrap_or_else(|e| panic!("{op}, d = {d}: the flat face moves: {e}"));
            let moved = AtRestBody::validate(moved, tol)
                .unwrap_or_else(|e| panic!("{op}, d = {d}: the moved body is finished: {e:?}"));
            topo::validate_geometric(&moved, tol)
                .unwrap_or_else(|e| panic!("{op}, d = {d}: tier 3: {e:?}"));
            // The rim across the kept wall is re-anchored with the move
            // and asked the rule again over its new extent: the two
            // walls are one cylinder, which under-determines it at any
            // length, so it keeps its image.
            let label = format!("{op}, d = {d}");
            assert!(
                across_a_wall(&moved, &label).contains(&CurveKind::Circle),
                "{label}: the rim still lies across the kept wall as an image"
            );
            let want = 2.5 * PI + d * (1.0 - d * d).sqrt() + d.asin();
            let m = topo::mass_properties(&moved, tol, topo::test_support::pinned_resolution(tol))
                .unwrap();
            assert!(
                (m.enclosure.volume - want).abs() <= m.enclosure.volume_pad + 1e-9,
                "{op}, d = {d}: the closed form: {} ± {} vs {want}",
                m.enclosure.volume,
                m.enclosure.volume_pad
            );
        }
    }
}

/// **Two sections the band reads coplanar are one curve only where
/// they lie within it of each other.** Two radius-`0.01` rods on one
/// axis, each cut below a plane through the origin, of slopes `100` and
/// `100(1 + d)`: their ellipse rims cross at `(0, ±0.01, 0)`. With
/// `d = k·ε` the planes' tilt carries `0.01·k·ε` across the ellipses, so
/// the band reads them parallel and through one point at every `k`
/// here, while the rims' far ends lie `k·ε` apart. At `k` = 0, 0.1, 0.5
/// and 0.9 the rims are one curve within the band, the two operands one
/// body, and every op builds it ([`a_cut_rod_and_its_band_copy_are_one_body`]):
/// `∪` and `∩` the cut rod, `3π·0.01²` within its pad, `∖` nothing. At 2
/// and 5 a seam ruling's crossing with the other rim's plane lies `k·ε`
/// from the ruling's end, and every op refuses `CurvedPierceEscalated`
/// on the meeting (`CurveMeeting`): the ruling's end, read against the
/// rim first, lies in the band's gap of its carrier (`curve_carrier_gap`). At 30 and 80 the band decides the two
/// cut planes one carrier and the rims two curves, which touch in the
/// planes' common plane at `(0, ±0.01, 0)` and stay within the band's
/// escalation of each other over `|cos θ| < 10/k` of their angle: the
/// level of one on the other has no definite sign across that stretch,
/// and the coplanar ellipse reading's walk can cut it nowhere definite.
/// At 30 that meeting is in band: the walk's side reading lands in the
/// gap (`curve_pair_coplanar_sub_side`), and every op refuses
/// `CurvedPierceEscalated` on the meeting, at the ruling and the face,
/// its text naming the meeting and its lever. At 80 the three with the
/// first rod first escalate on the sector reading's `conic_bend` first,
/// and the other three keep the `CurvedPierceUnsupported` frontier: the
/// walk's clear reading decides zero there, a decided reading the band
/// does not cut, which is the frontier's and not an escalation. At 200
/// the ops escalate in band too, the first rod first on a vertex's side
/// of a face (`bool_vertex_face_side`), the second on the meeting, the
/// transverse arm's swing (`curve_pair_pose_swing`). None reads the
/// rims' crossings away.
///
/// **At a coarse ε the one-curve poses refuse `CurvedPierceUnsupported`
/// first.** A seam ruling crosses each rim's plane at `|d̂·n̂| ≈ 0.01`,
/// so it stays within the band's escalation `e = 10ε` of the plane for
/// `e/0.01` metres either side. It meets the rim at the ellipse's tip,
/// whose radius of curvature is `b²/a = 10⁻⁴` m. The meeting reader's
/// basin meter needs the sine between the two (here one) to clear the
/// turn of the curvature over that stretch, `2·(a/b²)·2·(e/0.01 + e)`,
/// which fails once `ε` passes about `2.5·10⁻⁸`: at `k` below one the
/// reading refuses `Unsettled::Meter`
/// (`work/cpair/curve-pair-polish-meters-refuse-shallow-meetings.md`) and
/// the pierce is unread. There every op at 30 and past refuses
/// `CurvedPierceUnsupported` too, a precision limit and not a reading in
/// band, and the three with the first rod first at 80 do so before the
/// sector reading, on the same meter: the seam ruling lies on the other rod's
/// wall, the interior question reads its meetings with that wall's rim
/// (`boolean::carrier_cross`), and the line × ellipse pose's basin meter
/// (`curve_pair_pose_basin`) decides negative, so the crossing is unread
/// and the pierce keeps the frontier. Every one of the coarse ε's pierce
/// refusals here is that meter. The rest of the table holds at
/// every ε.
#[test]
fn two_rims_the_band_reads_coplanar_are_one_curve_only_within_it() {
    let tol = Tol::witness();
    let rod = rod_of(0.01, -3.0, 6.0);
    let e = geom_core::Band::linear(tol).unwrap().escalate();
    let meter_refuses = 2.0 * 1e4 * 2.0 * (e / 0.01 + e) >= 1.0;
    for k in [0.0, 0.1, 0.5, 0.9, 2.0, 5.0, 30.0, 80.0, 200.0] {
        let d = k * tol.eps();
        let (a, b) = (
            cut_below(&rod, 100.0, 0.0),
            cut_below(&rod, 100.0 * (1.0 + d), 0.0),
        );
        for (op, res) in six(&a, &b) {
            let label = format!("k = {k}: {op}");
            if meter_refuses && k < 1.0 {
                assert!(
                    matches!(res, Err(BooleanError::CurvedPierceUnsupported { .. })),
                    "{label}: the basin meter refuses at this ε: {res:?}"
                );
            } else if k < 1.0 {
                one_body(&label, op, res);
            } else if k < 10.0 {
                let Err(BooleanError::CurvedPierceEscalated { decision, diag, .. }) = &res else {
                    panic!("{label}: escalates, located: {res:?}");
                };
                assert_eq!(*decision, BooleanDecision::CurveMeeting, "{label}");
                assert_eq!(
                    diag.predicate,
                    Some("curve_carrier_gap"),
                    "{label}: on the end's reading"
                );
            } else if meter_refuses {
                assert!(
                    matches!(res, Err(BooleanError::CurvedPierceUnsupported { .. })),
                    "{label}: the basin meter refuses at this ε: {res:?}"
                );
            } else {
                let first = op.starts_with('t');
                let meeting = |res: &Result<_, BooleanError>, predicate: &str| {
                    let Err(e @ BooleanError::CurvedPierceEscalated { decision, diag, .. }) = res
                    else {
                        panic!("{label}: the meeting escalates, located: {res:?}");
                    };
                    assert_eq!(*decision, BooleanDecision::CurveMeeting, "{label}");
                    assert_eq!(diag.predicate, Some(predicate), "{label}");
                    let text = e.to_string();
                    assert!(
                        text.contains("meets a curve of that face's boundary is undecided")
                            && text
                                .contains("make the two cuts of the curved face one construction"),
                        "{label}: the text names the meeting and its lever: {text}"
                    );
                };
                let escalated = |res: &Result<_, BooleanError>, want, predicate| {
                    let Err(BooleanError::Escalated { decision, diag }) = res else {
                        panic!("{label}: escalates on {predicate}: {res:?}");
                    };
                    assert_eq!(*decision, want, "{label}");
                    assert_eq!(diag.predicate, Some(predicate), "{label}");
                };
                let coincidence = |c| BooleanDecision::Coincidence(c, DeclarationRead::Moot);
                match (k as u32, first) {
                    (80, true) => escalated(&res, coincidence(Coincide::SectorSide), "conic_bend"),
                    // The walk's clear reading decides zero there: main's frontier.
                    (80, false) => assert!(
                        matches!(res, Err(BooleanError::CurvedPierceUnsupported { .. })),
                        "{label}: a decided zero keeps the frontier: {res:?}"
                    ),
                    (200, true) => escalated(
                        &res,
                        coincidence(Coincide::VertexOnFace),
                        "bool_vertex_face_side",
                    ),
                    (200, false) => meeting(&res, "curve_pair_pose_swing"),
                    _ => meeting(&res, "curve_pair_coplanar_sub_side"),
                }
            }
        }
    }
}

/// What `op` on two operands that are one body yields: `∪` and `∩` that
/// body, the cut rod of [`two_rims_the_band_reads_coplanar_are_one_curve_only_within_it`],
/// at `3π·0.01²` within its pad and tier 3 and 3′; `∖` nothing.
fn one_body(label: &str, op: &str, res: Result<BooleanResult<f64>, BooleanError>) {
    let tol = Tol::witness();
    if op.contains('∖') {
        assert!(
            matches!(res, Ok(BooleanResult::Empty)),
            "{label}: one body less itself is nothing: {res:?}"
        );
        return;
    }
    let Ok(BooleanResult::Body(bb)) = res else {
        panic!("{label}: one body: {res:?}");
    };
    topo::validate_geometric(&bb.body, tol).unwrap_or_else(|e| panic!("{label}: tier 3: {e:?}"));
    topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol)
        .unwrap_or_else(|e| panic!("{label}: tier 3′: {e:?}"));
    let m = topo::test_support::pinned_volume(label, &bb.body, tol);
    let want = 3.0 * PI * 0.01 * 0.01;
    assert!(
        (m.volume - want).abs() <= m.volume_pad + 1e-9 * want,
        "{label}: the closed form: {} ± {} vs {want}",
        m.volume,
        m.volume_pad
    );
}

/// **A cut rod and its copy within the band are one body.** The rod of
/// [`two_rims_the_band_reads_coplanar_are_one_curve_only_within_it`] cut
/// below the plane of slope `100`, against its cut at `100(1 + ε/2)`,
/// whose rim lies `ε/2` from the first's: every face pair is one
/// carrier, nothing crosses, and each operand's shell lies on the
/// other's. `∪` and `∩` build that body and `∖` nothing, in both orders.
/// At a coarse ε the basin meter refuses first, as in that row. Red where
/// the coincident wall pair's section refuses the no-crossings path.
#[test]
fn a_cut_rod_and_its_band_copy_are_one_body() {
    let tol = Tol::witness();
    let e = geom_core::Band::linear(tol).unwrap().escalate();
    let meter_refuses = 2.0 * 1e4 * 2.0 * (e / 0.01 + e) >= 1.0;
    let rod = rod_of(0.01, -3.0, 6.0);
    let a = cut_below(&rod, 100.0, 0.0);
    let b = cut_below(&rod, 100.0 * (1.0 + 0.5 * tol.eps()), 0.0);
    for (op, res) in six(&a, &b) {
        let label = format!("against its band copy: {op}");
        if meter_refuses {
            assert!(
                matches!(res, Err(BooleanError::CurvedPierceUnsupported { .. })),
                "{label}: the basin meter refuses at this ε: {res:?}"
            );
        } else {
            one_body(&label, op, res);
        }
    }
}

/// **Two slanted bottoms crossing on one wall build every op.** The
/// tube from `z = −1` to [`H`] cut below `z = 0.5 + s·x` at two slopes:
/// each rim lies on the other's wall and crosses the other's rim where
/// `x = 0`, in planes decided apart, so each is split where it meets the
/// other's plane. The overlap is `1.5π − (2/3)·|s₁ − s₂|`, each
/// difference `(2/3)·|s₁ − s₂|` and the union `1.5π + (2/3)·|s₁ − s₂|`.
#[test]
fn two_slanted_bottoms_crossing_on_one_wall_build_every_op() {
    let tol = Tol::witness();
    let tube = rod_z(-1.0, H + 1.0);
    for (s1, s2) in [(0.2, -0.3), (0.2, 0.5), (-0.6, 0.1)] {
        let (t, b) = (cut_below(&tube, s1, 0.5), cut_below(&tube, s2, 0.5));
        let gap = 2.0 / 3.0 * (s1 - s2).abs();
        for (op, res) in six(&t, &b) {
            let label = format!("slopes {s1}, {s2}: {op}");
            let volume = if op.contains('∪') {
                1.5 * PI + gap
            } else if op.contains('∖') {
                gap
            } else {
                1.5 * PI - gap
            };
            let Ok(BooleanResult::Body(bb)) = res else {
                panic!("{label}: builds: {res:?}");
            };
            topo::validate_geometric(&bb.body, tol)
                .unwrap_or_else(|e| panic!("{label}: tier 3: {e:?}"));
            topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol)
                .unwrap_or_else(|e| panic!("{label}: tier 3′: {e:?}"));
            let m =
                topo::mass_properties(&bb.body, tol, topo::test_support::pinned_resolution(tol))
                    .unwrap();
            assert!(
                (m.enclosure.volume - volume).abs() <= m.enclosure.volume_pad + 1e-9 * volume,
                "{label}: the closed form: {} ± {} vs {volume}",
                m.enclosure.volume,
                m.enclosure.volume_pad
            );
        }
    }
}

/// Review R1 probe (PR 5993): the rendered text of the two rims' new
/// escalation, per op, at k = 30 and 200.
#[test]
#[ignore = "review probe"]
fn review_r1_two_rims_escalation_text() {
    let tol = Tol::witness();
    let rod = rod_of(0.01, -3.0, 6.0);
    for k in [30.0, 80.0, 200.0] {
        let d = k * tol.eps();
        let (a, b) = (
            cut_below(&rod, 100.0, 0.0),
            cut_below(&rod, 100.0 * (1.0 + d), 0.0),
        );
        for (op, res) in six(&a, &b) {
            match res {
                Err(e) => println!("k={k} {op}: {e:?}\n   TEXT: {e}"),
                Ok(_) => println!("k={k} {op}: built"),
            }
        }
    }
}

/// Review 3 probe (PR 5993): the two rims at `k = 80` and `200`, each
/// op's outcome, one op at a time (`RV_PROBE` prints the walk's exits).
#[test]
#[ignore = "review probe"]
fn review3_two_rims_per_op() {
    let tol = Tol::witness();
    let rod = rod_of(0.01, -3.0, 6.0);
    for k in [80.0, 200.0] {
        let d = k * tol.eps();
        let (a, b) = (
            cut_below(&rod, 100.0, 0.0),
            cut_below(&rod, 100.0 * (1.0 + d), 0.0),
        );
        for (op, res) in [
            ("b ∪ t", topo::union(&b, &a, tol)),
            ("b ∖ t", topo::subtract(&b, &a, tol)),
            ("b ∩ t", topo::intersect(&b, &a, tol)),
        ] {
            eprintln!("RV OP {op}");
            eprintln!("RV RESULT k = {k}: {op}: {res:?}");
            if let Err(e) = &res {
                crate::review_r1_roots_split::review3_locate(&format!("k {k} {op}"), &b, &a, e);
            }
        }
    }
}
