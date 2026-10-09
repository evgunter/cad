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

use crate::common::outcomes::outcome;
use geom_core::{Point2, Tol};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::ExtrudeSide;
use sweep::test_support::{brick, finished};
use sweep::{Extrusion, extrude};
use topo::{AtRestBody, BooleanResult};

fn tol() -> Tol {
    Tol::witness()
}

fn prism(pts: &[(f64, f64)], z: (f64, f64)) -> AtRestBody<f64> {
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
    let placed = topo::transform_rigid(
        &body,
        &geom_core::Affine3::translation(geom_core::Vec3::new(0.0, 0.0, z.0)),
        tol(),
    )
    .unwrap();
    finished("the prism", placed, tol())
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
    let b = finished(
        "the box",
        brick((-0.5, -0.25), (-0.5, -0.25), (-1.0, 3.0), tol()),
        tol(),
    );
    let (va, vb) = (0.75 * 2.0, 0.0625 * 4.0);
    let vi = 0.25 * 0.125 / 2.0 * 2.0;
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
    // Undeclared, the `y = −0.5` faces are one carrier by margin, so the
    // op glues them as the continuation they are: the declared body bit
    // for bit (D10).
    for (what, (x, y)) in [("hex ∪ box", (&hex, &b)), ("box ∪ hex", (&b, &hex))] {
        let d = declare_all(&find_flush_candidates(x, y, tol()).unwrap());
        assert_eq!(
            outcome(&topo::union(x, y, tol())),
            outcome(&topo::union_with(x, y, &d, tol())),
            "{what}: undeclared is the declared union"
        );
    }
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
    let a = finished(
        "brick A",
        brick((0.0, 2.0), (0.0, 2.0), (0.0, 1.0), tol()),
        tol(),
    );
    let b = finished(
        "brick B",
        brick((1.0, 3.0), (1.0, 3.0), (0.0, 1.0), tol()),
        tol(),
    );
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
/// `x = 0.25`, `π/12 − πh²(3r − h)/6` at `r = ½`, `h = ¼`. Each result,
/// whose sphere faces the tilted circle `x = 0.25` bounds, is the next
/// boolean's operand: its union with a far box, which only point
/// classification places, adds the box's volume. And each answers
/// points just inside and just outside the sphere, all round it, against
/// the ball's and the box's own tests, where the carved sphere faces are
/// what the solid door's rays meet first.
#[test]
fn a_pole_struts_halves_face_their_own_meridians() {
    use core::f64::consts::PI;
    let ball = sweep::test_support::ball_poled_y(0.5, geom_core::Vec3::new(0.0, 0.0, 0.0), tol());
    let ball = finished("the ball", ball, tol());
    let b = finished(
        "the box",
        brick((-1.0, 0.25), (-1.0, 1.0), (-1.0, 0.0), tol()),
        tol(),
    );
    let (r, h) = (0.5f64, 0.25f64);
    let shared = PI / 12.0 - PI * h * h * (3.0 * r - h) / 6.0;
    let (v_ball, v_box) = (4.0 / 3.0 * PI * r.powi(3), 1.25 * 2.0);
    // Each result's signed depth from the ball's and the box's own:
    // `max` for ∪, `min` for ∩, and a difference negates its subtrahend.
    let union: fn(f64, f64) -> f64 = |b, x| b.max(x);
    let common: fn(f64, f64) -> f64 = |b, x| b.min(x);
    let ball_less: fn(f64, f64) -> f64 = |b, x| b.min(-x);
    let box_less: fn(f64, f64) -> f64 = |b, x| x.min(-b);
    for (what, res, want, depth) in [
        (
            "ball ∪ box",
            topo::union(&ball, &b, tol()),
            v_ball + v_box - shared,
            union,
        ),
        (
            "box ∪ ball",
            topo::union(&b, &ball, tol()),
            v_ball + v_box - shared,
            union,
        ),
        (
            "ball ∖ box",
            topo::subtract(&ball, &b, tol()),
            v_ball - shared,
            ball_less,
        ),
        (
            "box ∖ ball",
            topo::subtract(&b, &ball, tol()),
            v_box - shared,
            box_less,
        ),
        (
            "ball ∩ box",
            topo::intersect(&ball, &b, tol()),
            shared,
            common,
        ),
        (
            "box ∩ ball",
            topo::intersect(&b, &ball, tol()),
            shared,
            common,
        ),
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
        let far = finished(
            "the far box",
            brick((5.0, 6.0), (5.0, 6.0), (5.0, 6.0), tol()),
            tol(),
        );
        assert_sound(
            &format!("{what}, then ∪ a far box"),
            topo::union(&bb.body, &far, tol()),
            want + 1.0,
        );
        // Points just inside and just outside the sphere, all round it,
        // where the carved sphere faces are what a ray from them meets
        // first: each against the ball's and the box's own tests.
        let band = geom_core::Band::linear(tol()).unwrap();
        let (mut ins, mut outs) = (0, 0);
        for k in 0..60 {
            let t = (f64::from(k) + 0.5) / 60.0;
            let (y, ring) = (1.0 - 2.0 * t, (1.0 - (1.0 - 2.0 * t).powi(2)).sqrt());
            let phi = f64::from(k) * PI * (3.0 - 5f64.sqrt());
            let dir = geom_core::Vec3::new(ring * phi.cos(), y, ring * phi.sin());
            for radius in [r - 2e-3, r + 2e-3] {
                let q = geom_core::Point3::origin() + dir * radius;
                let in_box = (q.x + 1.0)
                    .min(0.25 - q.x)
                    .min(q.y + 1.0)
                    .min(1.0 - q.y)
                    .min(q.z + 1.0)
                    .min(-q.z);
                let d = depth(r - radius, in_box);
                if d.abs() < 1e-3 {
                    continue;
                }
                let want = if d > 0.0 {
                    ins += 1;
                    topo::SolidContainment::In
                } else {
                    outs += 1;
                    topo::SolidContainment::Out
                };
                let got = topo::point_in_solid(&bb.body, q, band, tol())
                    .unwrap_or_else(|e| panic!("{what}: {q:?} refused: {e:?}"));
                assert_eq!(got, want, "{what}: {q:?}");
            }
        }
        assert!(ins > 10 && outs > 10, "{what}: {ins} in, {outs} out");
    }
}
