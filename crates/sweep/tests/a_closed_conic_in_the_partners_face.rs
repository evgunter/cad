//! **A closed conic lying in the partner's face.** A one-segment circle
//! is one edge with one vertex, both its ends. Where such an edge of one
//! operand lies in a face of the other, the section loop is that edge,
//! and its one site is the edge's vertex: one pair record carries both
//! of the loop's germs, one each way round it. The join takes it as one
//! segment (`boolean::join`'s `one_site_segments`): the edge's own side
//! copies it whole, and the partner's face is pierced at the site and
//! joined there as a ring of its own. Where the partner holds the circle
//! along a closed edge of its own through the same vertex (two coaxial
//! one-segment circles phased alike), both sides copy their edges.
//!
//! The rows, each a one-segment circle built through the raw loop door:
//!
//! - a boss standing on a plate's top face, its vertex at four azimuths,
//!   and one hanging under its bottom face off the axis, with the
//!   census; the same boss at `Interval`;
//! - a second boss on a plate whose top face already holds a boss's
//!   ring, and a boss over a hole whose ring lies inside its rim;
//! - a plug in the bore of an annulus standing on a plate;
//! - two equal coaxial cylinders stacked end to end;
//! - a tube whose rim lies inside one face of a ball;
//! - a puck and the quarter-disc ring revolved round it.
//!
//! Every row runs ∪, ∩ and both differences in both operand orders, and
//! each run is read through `differential::outcome` (tiers 2 and 3′, the
//! certificate, the legal operand, the closed-form volume) and meshed.
//! The ∩ of two solids meeting only in a face is empty. Two rows pin
//! what is not built yet: a boss over a hole tangent inside its rim at
//! both vertices, whose union is wrong today, and a closed edge in a
//! cylinder or a cone face, which refuses typed. The coaxial
//! stack and the puck run their unions alone: their ∩ and differences
//! refuse `FallbackExtentUnsupported` at the coincident walls, as the
//! two-arc forms do
//! (`work/reachhold/rounded-stack-subtract-and-intersect-refuse-fallback-extent.md`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI, SQRT_2, TAU};

use geom_core::{Affine3, Arc2, Bounds, Decide, Interval, Point2, Point3, Real, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane};
use sweep::test_support::{ball_poled_z, brick, revolved_about_y};
use sweep::{ExtrudeSide, Extrusion, Revolution, extrude};
use topo::{AtRestBody, AtRestPolicy, Body, BooleanError, BooleanResult};

use crate::common::differential::{every_op_both_orders, outcome};
use crate::common::oracles::{ball_volume, cap_volume};

fn tol() -> Tol {
    Tol::witness()
}

/// A one-segment circle of radius `r` about `(cx, cy)`, its one vertex
/// at azimuth `az`.
fn circle<T: Real>(cx: f64, cy: f64, r: f64, az: f64) -> ProfileLoop<T> {
    let f = T::from_f64;
    RawLoop::new([(
        Point2::new(f(cx + r * az.cos()), f(cy + r * az.sin())),
        Segment::Arc(Arc2 {
            centre: Point2::new(f(cx), f(cy)),
            radius: f(r),
            sweep: f(TAU),
        }),
    )])
}

/// `loops` extruded from `z = z0` up by `h`.
fn extruded<T: Decide + AtRestPolicy>(loops: Vec<ProfileLoop<T>>, z0: f64, h: f64) -> Body<T> {
    let f = T::from_f64;
    let profile = Profile::new(SketchPlane::<T>::xy(), loops)
        .validate(tol())
        .unwrap();
    let depth = Extrusion::Distance {
        depth: f(h),
        side: ExtrudeSide::Along,
    };
    let body = extrude(&profile, depth, tol()).unwrap().body;
    let up = Affine3::translation(Vec3::new(f(0.0), f(0.0), f(z0)));
    topo::transform_rigid(&body, &geom_core::test_support::rigid(&up, tol()), tol()).unwrap()
}

fn fin<T: AtRestPolicy>(what: &str, body: Body<T>) -> AtRestBody<T> {
    topo::test_support::finished(what, body, tol())
}

/// The plate `[−1, 1]² × [−0.5, 0]`: 2.
fn plate<T: Decide + AtRestPolicy>() -> AtRestBody<T> {
    fin(
        "the plate",
        brick((-1.0, 1.0), (-1.0, 1.0), (-0.5, 0.0), tol()),
    )
}

/// A body's shells, faces, rings, edges and vertices.
fn census<T: Real>(body: &Body<T>) -> [usize; 5] {
    [
        body.shells().count(),
        body.faces().count(),
        body.faces().map(|(_, f)| f.rings.len()).sum(),
        body.edges().count(),
        body.vertices().count(),
    ]
}

/// One run's verdict: `SOUND` at its closed form, or an empty result
/// where the closed form is empty; a body is meshed, unless it holds a
/// window in a curved face, which the mesher refuses typed
/// (`TessellateError::RingOnCurvedFace`). The body's [`census`], if
/// there is one.
fn holds(row: &str, r: Result<BooleanResult<f64>, BooleanError>, want: f64) -> Option<[usize; 5]> {
    let counted = match &r {
        Ok(BooleanResult::Body(b)) => {
            match mesh::tessellate(&b.body, 5e-3, tol()) {
                Ok(m) => {
                    mesh::validate::check_mesh(&m).unwrap_or_else(|e| panic!("{row}: mesh {e:?}"))
                }
                Err(mesh::TessellateError::RingOnCurvedFace { .. }) => {}
                Err(e) => panic!("{row}: {e:?}"),
            }
            Some(census(&b.body))
        }
        _ => None,
    };
    let line = outcome(r, want, tol());
    let ok = if want == 0.0 {
        line.starts_with("EMPTY ok")
    } else {
        line.starts_with("OK SOUND")
    };
    assert!(ok, "{row}: {line}");
    counted
}

/// Every op in both orders between `a` and `b` (volumes `va`, `vb`,
/// overlapping in `vab`), each held to its closed form ([`holds`]);
/// their censuses, by op.
fn every_op(
    what: &str,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    v: (f64, f64, f64),
) -> Vec<(&'static str, Option<[usize; 5]>)> {
    every_op_both_orders(a, b, v, tol())
        .into_iter()
        .map(|(op, r, want)| (op, holds(&format!("{what}: {op}"), r, want)))
        .collect()
}

/// A boss on the plate's census, by op, counted by hand: the union is
/// the plate's six faces, the boss's wall and top, and its rim a ring
/// of the plate's top (one shell, 8 faces, 1 ring, 15 edges, 10
/// vertices); the boss less the plate is the boss (3, 3, 2); the plate
/// less the boss is the plate (6, 12, 8).
const BOSS_ON_PLATE: [(&str, Option<[usize; 5]>); 6] = [
    ("A ∪ B", Some([1, 8, 1, 15, 10])),
    ("B ∪ A", Some([1, 8, 1, 15, 10])),
    ("A ∩ B", None),
    ("B ∩ A", None),
    ("A ∖ B", Some([1, 3, 0, 3, 2])),
    ("B ∖ A", Some([1, 6, 0, 12, 8])),
];

/// Both unions of `a` and `b`, which meet in no volume, held to `want`.
fn both_unions(what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>, want: f64) {
    for (op, r) in [
        ("A ∪ B", topo::union(a, b, tol())),
        ("B ∪ A", topo::union(b, a, tol())),
    ] {
        holds(&format!("{what}: {op}"), r, want);
    }
}

/// The union of `a` and `b`, which must build.
fn united(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> AtRestBody<f64> {
    topo::union(a, b, tol())
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone()
}

/// **A boss on a plate**: the cylinder `r = 0.3`, `z ∈ [0, 1]`, standing
/// on the top face, its rim's vertex at four azimuths; and the same
/// cylinder hung under the bottom face about `(0.2, 0.1)`. Boss `0.09π`,
/// plate 2, overlap 0.
#[test]
fn a_boss_on_a_plate() {
    let plate = plate();
    for az in [0.0, 1.0, PI, -2.0] {
        let boss = fin(
            "the boss",
            extruded(vec![circle(0.0, 0.0, 0.3, az)], 0.0, 1.0),
        );
        let what = format!("a boss, vertex at {az}");
        let counted = every_op(&what, &boss, &plate, (0.09 * PI, 2.0, 0.0));
        assert_eq!(counted, BOSS_ON_PLATE, "{what}: census");
    }
    let under = fin(
        "the boss",
        extruded(vec![circle(0.2, 0.1, 0.3, 0.5)], -1.5, 1.0),
    );
    let counted = every_op("a boss under", &under, &plate, (0.09 * PI, 2.0, 0.0));
    assert_eq!(counted, BOSS_ON_PLATE, "a boss under: census");
}

/// **The boss on the plate at `Interval`**, its vertex at two
/// azimuths: every op builds at tier 3 with the f64 run's census (∩
/// empty), and its volume enclosure brackets the closed form.
#[test]
fn a_boss_on_a_plate_at_interval() {
    let plate = plate::<Interval>();
    for az in [0.0, 1.0] {
        let boss = fin(
            "the boss",
            extruded::<Interval>(vec![circle(0.0, 0.0, 0.3, az)], 0.0, 1.0),
        );
        let (vb, vp) = (0.09 * PI, 2.0);
        for ((op, r), (_, counted)) in [
            ("A ∪ B", topo::union(&boss, &plate, tol())),
            ("B ∪ A", topo::union(&plate, &boss, tol())),
            ("A ∩ B", topo::intersect(&boss, &plate, tol())),
            ("B ∩ A", topo::intersect(&plate, &boss, tol())),
            ("A ∖ B", topo::subtract(&boss, &plate, tol())),
            ("B ∖ A", topo::subtract(&plate, &boss, tol())),
        ]
        .into_iter()
        .zip(BOSS_ON_PLATE)
        {
            let what = format!("vertex at {az}, {op}");
            let want = match op {
                "A ∪ B" | "B ∪ A" => vb + vp,
                "A ∖ B" => vb,
                "B ∖ A" => vp,
                _ => 0.0,
            };
            let r = r.unwrap_or_else(|e| panic!("{what}: {e:?}"));
            let Some(b) = r.body() else {
                assert_eq!(want, 0.0, "{what}: empty");
                assert_eq!(counted, None, "{what}: empty");
                continue;
            };
            topo::validate_geometric(&b.body, tol())
                .unwrap_or_else(|e| panic!("{what}: tier 3: {e:?}"));
            assert_eq!(Some(census(&b.body)), counted, "{what}: census");
            let v = topo::mass_properties(&b.body, tol()).unwrap().volume;
            assert!(
                v.lo() <= want && want <= v.hi() && v.hi() - v.lo() < 1e-9,
                "{what}: [{}, {}] against {want}",
                v.lo(),
                v.hi()
            );
        }
    }
}

/// **The pierced face's other rings**: a second boss on a plate whose
/// top face holds the first boss's ring, outside the new rim; and a boss
/// over a hole `r = 0.1`, whose ring lies inside the rim and goes to the
/// disc the rim cuts out.
#[test]
fn a_boss_beside_a_ring_and_over_a_hole() {
    let first = fin(
        "the first boss",
        extruded(vec![circle(-0.5, 0.0, 0.3, 0.0)], 0.0, 1.0),
    );
    let second = fin(
        "the second boss",
        extruded(vec![circle(0.5, 0.0, 0.3, 1.0)], 0.0, 1.0),
    );
    let one = topo::union(&plate(), &first, tol())
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone();
    every_op(
        "the second boss",
        &second,
        &one,
        (0.09 * PI, 2.0 + 0.09 * PI, 0.0),
    );
    let square = RawLoop::polygon([
        Point2::new(-1.0, -1.0),
        Point2::new(1.0, -1.0),
        Point2::new(1.0, 1.0),
        Point2::new(-1.0, 1.0),
    ]);
    let holed = fin(
        "the holed plate",
        extruded(vec![square, circle(0.0, 0.0, 0.1, 0.0)], -0.5, 0.5),
    );
    let boss = fin(
        "the boss",
        extruded(vec![circle(0.0, 0.0, 0.3, 0.0)], 0.0, 1.0),
    );
    every_op(
        "a boss over a hole",
        &boss,
        &holed,
        (0.09 * PI, 2.0 - 0.005 * PI, 0.0),
    );
}

/// **A plug in an annulus's bore**: the plate united with the tube
/// `r ∈ [0.3, 0.5]`, `z ∈ [0, 1]`, and the cylinder `r = 0.3` of the
/// same height filling its bore, every vertex at one azimuth, at two.
/// The plug's floor rim lies in the plate's top along the bore's floor
/// rim, a closed edge of each through one site, in a ring of the
/// plate's top: its join is a lone ring holding the null edge and the
/// closed edge (`chord_join`'s `LoneRing::Closed`), whose chord runs
/// against the edge's half beside it (`closed_chord_runs_forward`) and
/// is stated in a chart. Plug `0.09π`, the plate and tube `2 + 0.16π`,
/// overlap 0.
#[test]
fn a_plug_in_an_annulus_bore() {
    for az in [0.0, 1.0] {
        let tube = fin(
            "the tube",
            extruded(
                vec![circle(0.0, 0.0, 0.5, az), circle(0.0, 0.0, 0.3, az)],
                0.0,
                1.0,
            ),
        );
        let annulus = united(&plate(), &tube);
        let plug = fin(
            "the plug",
            extruded(vec![circle(0.0, 0.0, 0.3, az)], 0.0, 1.0),
        );
        every_op(
            &format!("a plug, vertices at {az}"),
            &plug,
            &annulus,
            (0.09 * PI, 2.0 + 0.16 * PI, 0.0),
        );
    }
}

/// **A boss over a hole tangent inside its rim**, both vertices at the
/// tangent point: the boss `r = 0.3` and the through hole `r = 0.1`
/// about `(0.2, 0)`. The union builds two shells, the boss and the holed
/// plate, which share the crescent of the boss's floor the hole does not
/// take. Its volume is right and tier 3′ fails on it; the true union is
/// one shell. This pins today's wrong body
/// (`work/cleave/a-boss-over-a-hole-tangent-inside-its-rim-at-its-vertex-unites-as-two-shells.md`):
/// the fix makes it one shell, and this row then holds the union to
/// `SOUND`. The differences are right already.
#[test]
fn a_boss_over_a_hole_tangent_inside_its_rim() {
    let square = RawLoop::polygon([
        Point2::new(-1.0, -1.0),
        Point2::new(1.0, -1.0),
        Point2::new(1.0, 1.0),
        Point2::new(-1.0, 1.0),
    ]);
    let holed = fin(
        "the holed plate",
        extruded(vec![square, circle(0.2, 0.0, 0.1, 0.0)], -0.5, 0.5),
    );
    let boss = fin(
        "the boss",
        extruded(vec![circle(0.0, 0.0, 0.3, 0.0)], 0.0, 1.0),
    );
    let (vb, vh) = (0.09 * PI, 2.0 - 0.005 * PI);
    for (op, r) in [
        ("A ∪ B", topo::union(&boss, &holed, tol())),
        ("B ∪ A", topo::union(&holed, &boss, tol())),
    ] {
        let r = r.unwrap_or_else(|e| panic!("{op}: {e:?}"));
        let b = r.body().unwrap_or_else(|| panic!("{op}: empty"));
        assert_eq!(census(&b.body), [2, 10, 2, 18, 12], "{op}: two shells");
        let line = outcome(Ok(r), vb + vh, tol());
        assert!(
            line.starts_with("OK BAD t2=true t3p=false cert=true operand=true"),
            "{op}: {line}"
        );
    }
    for (op, r, want) in [
        ("A ∩ B", topo::intersect(&boss, &holed, tol()), 0.0),
        ("A ∖ B", topo::subtract(&boss, &holed, tol()), vb),
        ("B ∖ A", topo::subtract(&holed, &boss, tol()), vh),
    ] {
        holds(&format!("a boss over a tangent hole: {op}"), r, want);
    }
}

/// **A closed edge lying in a cylinder or a cone face**: a triangle
/// revolved about the axis straddling the wall, its vertex on the wall
/// sweeping a circle in the wall's face. The one-site join has no arm
/// for such a face (`chord_join`'s `RimSide`): a plane can cut it in more
/// than the conic. Every op refuses before the join, at the crossing
/// layer (`work/sect/a-closed-edge-lying-in-a-cone-cylinder-or-torus-face-has-no-join-arm.md`).
#[test]
fn a_closed_edge_in_a_cylinder_or_cone_face_refuses_typed() {
    let revolved = |what: &str, verts: &[(f64, f64)]| {
        fin(
            what,
            revolved_about_y(
                verts
                    .iter()
                    .map(|&(x, y)| (Point2::new(x, y), 0.0))
                    .collect(),
                Revolution::Full,
                tol(),
            ),
        )
    };
    let straddle = revolved("the straddle", &[(1.0, 0.0), (1.5, 0.5), (0.5, 0.5)]);
    let cylinder = revolved(
        "the cylinder",
        &[(0.0, -1.0), (1.0, -1.0), (1.0, 1.0), (0.0, 1.0)],
    );
    let cone = revolved(
        "the cone",
        &[(0.0, -1.0), (1.5, -1.0), (0.5, 1.0), (0.0, 1.0)],
    );
    for (what, wall) in [("cylinder", &cylinder), ("cone", &cone)] {
        for (op, r) in [
            ("A ∪ B", topo::union(wall, &straddle, tol())),
            ("B ∪ A", topo::union(&straddle, wall, tol())),
            ("A ∩ B", topo::intersect(wall, &straddle, tol())),
            ("B ∩ A", topo::intersect(&straddle, wall, tol())),
            ("A ∖ B", topo::subtract(wall, &straddle, tol())),
            ("B ∖ A", topo::subtract(&straddle, wall, tol())),
        ] {
            assert!(
                matches!(r, Err(BooleanError::CurvedPierceUnsupported { .. })),
                "{what}, {op}: {r:?}"
            );
        }
    }
}

/// **Equal coaxial cylinders stacked end to end**: `r = 1` over
/// `z ∈ [0, 1]` and `[1, 2]`, both vertices at azimuth 0, so the rims
/// are one closed edge of each through one site; and with the upper
/// vertex turned 2 rad, where each vertex lies inside the other's rim
/// and the loop has two sites. `2π`.
#[test]
fn coaxial_equal_cylinders_unite() {
    let lo = fin(
        "the lower",
        extruded(vec![circle(0.0, 0.0, 1.0, 0.0)], 0.0, 1.0),
    );
    for az in [0.0, 2.0] {
        let hi = fin(
            "the upper",
            extruded(vec![circle(0.0, 0.0, 1.0, az)], 1.0, 1.0),
        );
        both_unions(&format!("the stack, upper vertex at {az}"), &lo, &hi, TAU);
    }
}

/// **A tube's rim inside one face of a ball**: the tube `r = 1` rising
/// two from where its rim lies on the ball `√2` about the origin, tilted
/// 1.2 rad about `x` so its rim crosses neither of the ball's seam
/// meridians, and turned besides. The ball keeps of the tube the cap
/// above the rim's plane. Read without a mesh: a sphere face bounded by
/// a circle off its chart's latitudes has no tessellation lane
/// (`work/tess/sphere-face-bounded-by-a-tilted-circle-has-no-tessellation-lane.md`).
#[test]
fn a_rim_inside_a_ball_face() {
    let z0 = 1.0f64;
    let ball = fin("the ball", ball_poled_z(SQRT_2, Vec3::zero(), tol()));
    let o = Point3::origin();
    for turn in [0.0, 0.4] {
        let place = Affine3::rotation_about_axis(o, Vec3::unit_z(), turn)
            * Affine3::rotation_about_axis(o, Vec3::unit_x(), 1.2);
        let tube = topo::transform_rigid(
            &extruded(vec![circle(0.0, 0.0, 1.0, 0.0)], z0, 2.0),
            &geom_core::test_support::rigid(&place, tol()),
            tol(),
        )
        .unwrap();
        let tube = fin("the tube", tube);
        let v = (
            2.0 * PI,
            ball_volume(SQRT_2),
            cap_volume(SQRT_2, SQRT_2 - z0),
        );
        for (op, r, want) in every_op_both_orders(&tube, &ball, v, tol()) {
            let line = outcome(r, want, tol());
            assert!(line.starts_with("OK SOUND"), "turned {turn}, {op}: {line}");
        }
    }
}

/// **A puck and its rounding ring unite**: the puck `r = 1`, `z ∈ [0,
/// 0.25]`, and the quarter disc of radius 0.25 revolved round it, whose
/// torus meets the puck's top along its top rim, whose inner wall is the
/// puck's and whose floor continues the puck's. Both of the puck's rims
/// are edges of the ring through the puck's vertices: the puck's
/// volume and the ring's, by Pappus.
#[test]
fn a_puck_and_its_rounding_ring_unite() {
    let (a, r) = (1.0_f64, 0.25_f64);
    let q = (FRAC_PI_2 / 4.0).tan();
    let ring = revolved_about_y(
        vec![
            (Point2::new(a, 0.0), 0.0),
            (Point2::new(a + r, 0.0), q),
            (Point2::new(a, r), 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), FRAC_PI_2);
    let mut ring =
        topo::transform_rigid(&ring, &geom_core::test_support::rigid(&turn, tol()), tol()).unwrap();
    ring.merge_coplanar_faces(tol()).unwrap();
    let ring = fin("the ring", ring);
    let puck = fin("the puck", extruded(vec![circle(0.0, 0.0, a, 0.0)], 0.0, r));
    let quarter = PI * r * r / 4.0 * TAU * (a + 4.0 * r / (3.0 * PI));
    both_unions("the rounded puck", &puck, &ring, PI * a * a * r + quarter);
}
