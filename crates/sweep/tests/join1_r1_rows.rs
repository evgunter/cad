//! JOIN-1 review lane r1: rows for poses the locus matching newly
//! builds, each checked at tiers 2, 3′ and the at-rest certificate and
//! at its closed-form volume, and for a pose whose refusal is a typed
//! frontier only while a strut's halves are bound right.
//!
//! The first row is RED on the reviewed head (4ef105c30): a hexagonal
//! prism unioned with a box whose corner edge lies along one of the
//! prism's vertical edges, one box face coplanar with (and on the far
//! side of) a prism face. On main the union refused
//! `Join(UnpairedLooseEnds)`; on the head it returns a body of the right
//! volume that carries a scaffold edge at rest (tier 3′ and the
//! certificate refuse `ScaffoldAtRest`). Both ends of the segment along
//! the shared edge are edge-edge sites where no flanking record folds
//! the edge In on both operands, so the record is chosen by A's fold
//! alone and B's flank is whatever that record carries.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::ExtrudeSide;
use sweep::test_support::brick;
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanResult};

fn tol() -> Tol {
    Tol::witness()
}

fn prism(pts: &[(f64, f64)], z: (f64, f64)) -> Body<f64> {
    let lp = bulge_loop(pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect());
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    let body = extrude(
        &profile,
        Extrusion::Distance {
            depth: z.1 - z.0,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    topo::transform_rigid(
        &body,
        &geom_core::Affine3::translation(geom_core::Vec3::new(0.0, 0.0, z.0)),
        tol(),
    )
    .unwrap()
}

fn assert_sound(what: &str, r: Result<BooleanResult<f64>, topo::BooleanError>, want: f64) {
    let r = r.unwrap_or_else(|e| panic!("{what}: {e:?}"));
    let bb = r.body().unwrap_or_else(|| panic!("{what}: empty"));
    topo::validate_closed(&bb.body).unwrap_or_else(|e| panic!("{what}: tier 2: {e:?}"));
    topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol())
        .unwrap_or_else(|e| panic!("{what}: tier 3′: {e:?}"));
    topo::validate_geometric_certificate(&bb.body, tol())
        .unwrap_or_else(|e| panic!("{what}: certificate: {e:?}"));
    let v = topo::mass_properties(&bb.body, tol()).unwrap().volume;
    assert!((v - want).abs() < 1e-9, "{what}: volume {v} against {want}");
    sweep::test_support::assert_legal_operand(what, &bb.body, tol());
}

const HEX: [(f64, f64); 6] = [
    (0.5, 0.0),
    (0.25, 0.5),
    (-0.25, 0.5),
    (-0.5, 0.0),
    (-0.25, -0.5),
    (0.25, -0.5),
];

/// **RED on the reviewed head.** The box `[−0.5, −0.25]²` has its corner
/// edge `x = −0.25, y = −0.5` along the hexagon's vertical edge there;
/// its face `y = −0.5` is coplanar with the hexagon's bottom face and on
/// the other side of the shared edge. The union's material spans a
/// half-plane about that edge. Area: hexagon 0.75, box 0.0625, overlap
/// the triangle under the slanted face, `0.25 · 0.125 / 2`.
#[test]
fn a_hexagon_unions_a_box_on_its_corner_edge_soundly() {
    let hex = prism(&HEX, (0.0, 2.0));
    let b = brick((-0.5, -0.25), (-0.5, -0.25), (-1.0, 3.0), tol());
    let (va, vb) = (0.75 * 2.0, 0.0625 * 4.0);
    let vi = 0.25 * 0.125 / 2.0 * 2.0;
    // Undeclared, the op refuses: the hexagon's and the box's `y = −0.5`
    // faces are a continuation no declaration licenses (topo README C4,
    // the reduction's continuation scan).
    for (what, r) in [
        ("hex ∪ box", topo::union(&hex, &b, tol())),
        ("box ∪ hex", topo::union(&b, &hex, tol())),
    ] {
        assert!(
            matches!(
                r,
                Err(topo::BooleanError::UndeclaredCoincidence {
                    relation: topo::PlaneRelation::SameOriented,
                    ..
                })
            ),
            "{what}: {r:?}"
        );
    }
    // Declared (the flush detector finds the pair), it builds, and the
    // merge stage glues the two faces: a legal operand.
    use topo::flush::{declare_all, find_flush_candidates};
    let d = declare_all(&find_flush_candidates(&hex, &b, tol()).unwrap());
    assert_sound(
        "hex ∪ box, declared",
        topo::union_with(&hex, &b, &d, tol()),
        va + vb - vi,
    );
    let d = declare_all(&find_flush_candidates(&b, &hex, tol()).unwrap());
    assert_sound(
        "box ∪ hex, declared",
        topo::union_with(&b, &hex, &d, tol()),
        va + vb - vi,
    );
    // The ∖ and ∩ of the same pose, declared, build soundly too.
    let d = declare_all(&find_flush_candidates(&hex, &b, tol()).unwrap());
    assert_sound(
        "hex ∖ box, declared",
        topo::subtract_with(&hex, &b, &d, tol()),
        va - vi,
    );
    assert_sound(
        "hex ∩ box, declared",
        topo::intersect_with(&hex, &b, &d, tol()),
        vi,
    );
}

/// `review_m3_pr55`'s multi-spike corner pose, which the head now
/// builds, held to tier 3′ and the certificate as well as the volume
/// (that row checks tier 2 and the volume only).
#[test]
fn the_multi_spike_corner_meet_passes_tier_3() {
    use topo::flush::{declare_all, find_flush_candidates};
    let a = brick((0.0, 2.0), (0.0, 2.0), (0.0, 1.0), tol());
    let b = brick((1.0, 3.0), (1.0, 3.0), (0.0, 1.0), tol());
    let decl = declare_all(&find_flush_candidates(&a, &b, tol()).unwrap());
    let ab = match topo::intersect_with(&a, &b, &decl, tol()).unwrap() {
        BooleanResult::Body(bb) => bb.body,
        BooleanResult::Empty => panic!("nonempty"),
    };
    let c = prism(
        &[(1.0, 1.0), (2.0, 0.0), (3.0, 1.0), (2.0, 2.0)],
        (0.0, 1.0),
    );
    let decl = declare_all(&find_flush_candidates(&ab, &c, tol()).unwrap());
    assert_sound(
        "AB ∩ C at the shared corner",
        topo::intersect_with(&ab, &c, &decl, tol()),
        0.5,
    );
}

/// The pole-strut pose: `ball_poled_y(0.5)` against the box
/// `[−1, 0.25] × [−1, 1] × [−1, 0]`, whose face `z = 0` holds both
/// meridian edges of the sphere face through its poles. Each pole is a
/// vertex-on-face site minted as a strut whose germs run along those
/// meridians; its halves face the germ beside their own meridian
/// (`insert::strut_facing`). Bound the other way, the join matches a
/// meridian's segment with the half beside the other meridian and
/// refuses `JoinDesync` ("every chord arc separates a loose scaffolding
/// pair"). Bound right, every op in either order builds: the shared
/// volume is the half ball below `z = 0` less the half of its cap beyond
/// `x = 0.25`, `π/12 − πh²(3r − h)/6` at `r = ½`, `h = ¼`. A result
/// carrying a sphere face bounded by the tilted circle `x = 0.25` is no
/// operand for the next boolean yet: point classification cannot read
/// that face
/// (`work/reach/carved-sphere-body-cannot-be-classified-or-reused-as-an-operand.md`).
#[test]
fn a_pole_struts_halves_face_their_own_meridians() {
    use core::f64::consts::PI;
    let ball = sweep::test_support::ball_poled_y(0.5, geom_core::Vec3::new(0.0, 0.0, 0.0), tol());
    let b = brick((-1.0, 0.25), (-1.0, 1.0), (-1.0, 0.0), tol());
    let (r, h) = (0.5f64, 0.25f64);
    let shared = PI / 12.0 - PI * h * h * (3.0 * r - h) / 6.0;
    let (v_ball, v_box) = (4.0 / 3.0 * PI * r.powi(3), 1.25 * 2.0);
    for (what, res, want) in [
        (
            "ball ∪ box",
            topo::union(&ball, &b, tol()),
            v_ball + v_box - shared,
        ),
        (
            "box ∪ ball",
            topo::union(&b, &ball, tol()),
            v_ball + v_box - shared,
        ),
        (
            "ball ∖ box",
            topo::subtract(&ball, &b, tol()),
            v_ball - shared,
        ),
        (
            "box ∖ ball",
            topo::subtract(&b, &ball, tol()),
            v_box - shared,
        ),
        ("ball ∩ box", topo::intersect(&ball, &b, tol()), shared),
        ("box ∩ ball", topo::intersect(&b, &ball, tol()), shared),
    ] {
        let res = res.unwrap_or_else(|e| panic!("{what}: {e:?}"));
        let bb = res.body().unwrap_or_else(|| panic!("{what}: empty"));
        topo::validate_closed(&bb.body).unwrap_or_else(|e| panic!("{what}: tier 2: {e:?}"));
        topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol())
            .unwrap_or_else(|e| panic!("{what}: tier 3′: {e:?}"));
        topo::validate_geometric_certificate(&bb.body, tol())
            .unwrap_or_else(|e| panic!("{what}: certificate: {e:?}"));
        let v = topo::mass_properties(&bb.body, tol()).unwrap().volume;
        assert!((v - want).abs() < 1e-9, "{what}: volume {v} against {want}");
        let far = brick((50.0, 51.0), (50.0, 51.0), (50.0, 51.0), tol());
        match topo::union(&bb.body, &far, tol()) {
            Ok(_)
            | Err(topo::BooleanError::Containment(topo::PointInSolidError::PartialSphereFace {
                ..
            })) => {}
            Err(e) => panic!("{what}: as an operand, {e:?}"),
        }
    }
}
