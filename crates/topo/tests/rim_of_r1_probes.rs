//! FILLET-RIM review probes (r1), `topo` half — what the rim door's
//! TOPOLOGICAL chain test admits, on bodies hand-assembled through the
//! Euler doors so every stored carrier is stated exactly.
//!
//! 1. The chain test is vertex-key equality, so two arcs of one circle
//!    that both cover the SAME half of it — a double cover leaving the
//!    other half bare — close the walk and come back as a rim.
//! 2. On a three-arc rim where one arc is stored on the opposite
//!    winding, every seed's answer is a rotation of every other's: the
//!    order follows the lower surface key's half-edges, not any arc's
//!    carrier.
//! 3. An opposite axis minted fresh (other signed zeros) closes the rim
//!    too: no carrier value is compared.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Point3, Tol, Vec3};
use topo::query::rim_of;
use topo::{Body, EdgeKey, FaceSurface, MefSite, MevSite, VertexKey};

const RIM_Z: f64 = 0.5;

fn rim_r() -> f64 {
    (1.0 - RIM_Z * RIM_Z).sqrt()
}

fn unit_sphere() -> Surface<f64> {
    Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn rim_plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, RIM_Z),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// The rim circle, wound `+z`.
fn rim_circle() -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, RIM_Z),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: rim_r(),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// The same circle wound the other way, with `axis` spelled as the
/// NEGATED VALUE of the seed's (`-(0, 0, 1)` = `(-0, -0, -1)`), which
/// is the spelling the door's match admits. Its parameter `t` is at
/// world azimuth `-t`.
fn rim_circle_negated_value() -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, RIM_Z),
        axis: -Vec3::new(0.0, 0.0, 1.0),
        radius: rim_r(),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// The same circle wound the other way, with `axis` MINTED FRESH as
/// `(0, 0, -1)` — positive zeros — the residue the door's own comment
/// names.
fn rim_circle_negated_fresh() -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, RIM_Z),
        axis: Vec3::new(0.0, 0.0, -1.0),
        radius: rim_r(),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// The world point at azimuth `theta` on the rim.
fn at(theta: f64) -> Point3<f64> {
    Point3::new(rim_r() * theta.cos(), rim_r() * theta.sin(), RIM_Z)
}

/// The sphere face alone, with ONE arc of the rim (world azimuth
/// `0 → π`, wound `+z`) out and back inside it — `topo/tests/rim_of.rs`'s
/// `half_built`, restated.
fn half_built() -> (Body<f64>, EdgeKey) {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(at(0.0), true).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: unit_sphere(),
            sense: true,
        },
    )
    .unwrap();
    let made = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            at(PI),
            EdgeCurveSpec::arc_of_circle(rim_circle(), 0.0, PI).unwrap(),
            tol,
        )
        .unwrap();
    (body, made.edge)
}

fn ends(body: &Body<f64>, k: EdgeKey) -> (VertexKey, VertexKey) {
    let e = body.get_edge(k).unwrap();
    (
        body.get_half_edge(e.he_plus).unwrap().start,
        body.half_edge_end(e.he_plus).unwrap(),
    )
}

/// **A double cover of half the circle passes the chain test.** Both
/// arcs cover azimuth `0 → π` wound `+z` on ONE stored circle — the
/// same `rim_circle()` value, so nothing here depends on how the match
/// treats an opposite axis — and both run `V0 → V1`. The walk closes
/// on the second step having consumed both, and the door answers a rim
/// whose two arcs lie on top of each other and leave the lower half
/// bare.
///
/// The door's contract now says this in as many words: the test is a
/// closed chain on shared vertices, not a covering test. The durable
/// home for the gap is issue `rim-door-admits-a-double-cover`; what
/// refuses such a body is tier 3, printed rather than asserted below.
#[test]
fn a_double_cover_of_half_the_circle_is_answered_as_a_rim() {
    let tol = Tol::witness();
    let (mut body, first) = half_built();
    let e = body.get_edge(first).unwrap();
    // `he_plus` of the new edge runs start(he1) → start(he2) = V0 → V1.
    let made = body
        .mef(
            MefSite::Chords {
                he1: e.he_plus,
                he2: e.he_minus,
            },
            EdgeCurveSpec::arc_of_circle(rim_circle(), 0.0, PI).unwrap(),
            FaceSurface::New {
                surface: rim_plane(),
                sense: true,
            },
            tol,
        )
        .expect("the second arc certifies against V0 → V1");
    let second = made.edge;
    // The two arcs cover the same half: their midpoints coincide.
    let mid = |k: EdgeKey| {
        let g = body
            .get_curve_geom(body.get_edge(k).unwrap().curve)
            .unwrap()
            .certified()
            .unwrap();
        let (t0, t1) = g.params();
        g.carrier().eval((t0 + t1) / 2.0)
    };
    let (m0, m1) = (mid(first), mid(second));
    assert!(
        (m0 - m1).norm() < 1e-12,
        "both arcs cover the upper half: midpoints {m0:?} and {m1:?}"
    );
    let answer = rim_of(&body, first);
    assert_eq!(
        answer,
        Ok(vec![first, second]),
        "the topological tiling test admits the double cover as a rim"
    );
    assert_eq!(rim_of(&body, second), Ok(vec![second, first]));
    // Whether a producer could ever hand this body out: the tier-3
    // verdict, recorded rather than asserted.
    println!(
        "DOUBLE_COVER tier-3: {:?}",
        topo::validate_geometric(&body, tol).map(|_| "valid")
    );
}

/// **An arc stored on the opposite winding is answered in the rim's
/// one order.** Three arcs tile the circle — `a` and `b` wound `+z`
/// (azimuth `0 → 2π/3 → 4π/3`), `c` stored on the negated-VALUE circle
/// (`t ∈ (0, 2π/3)`, azimuth `0 → -2π/3`, i.e. `V0 → V2` closing the
/// last third). Each seed's answer is a rotation of the others',
/// because the walk follows the half-edges on the lower surface key
/// rather than any seed's carrier direction.
#[test]
fn an_opposed_winding_arc_is_answered_as_a_rotation_on_a_three_arc_rim() {
    let tol = Tol::witness();
    let (v0, v1, v2) = (0.0, 2.0 * PI / 3.0, 4.0 * PI / 3.0);
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(at(v0), true).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: unit_sphere(),
            sense: true,
        },
    )
    .unwrap();
    let a = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            at(v1),
            EdgeCurveSpec::arc_of_circle(rim_circle(), v0, v1).unwrap(),
            tol,
        )
        .unwrap()
        .edge;
    let a_minus = body.get_edge(a).unwrap().he_minus; // starts at V1
    let b = body
        .mev(
            MevSite::Fan {
                he1: a_minus,
                he2: a_minus,
            },
            at(v2),
            EdgeCurveSpec::arc_of_circle(rim_circle(), v1, v2).unwrap(),
            tol,
        )
        .unwrap()
        .edge;
    let a_plus = body.get_edge(a).unwrap().he_plus; // starts at V0
    let b_minus = body.get_edge(b).unwrap().he_minus; // starts at V2
    // The closing arc runs V0 → V2 on the negated circle: azimuth
    // 0 → -2π/3 ≡ 4π/3, the third the first two arcs leave open.
    let c = body
        .mef(
            MefSite::Chords {
                he1: a_plus,
                he2: b_minus,
            },
            EdgeCurveSpec::arc_of_circle(rim_circle_negated_value(), 0.0, 2.0 * PI / 3.0).unwrap(),
            FaceSurface::New {
                surface: rim_plane(),
                sense: true,
            },
            tol,
        )
        .expect("the closing arc certifies against V0 → V2")
        .edge;
    let (a0, a1) = ends(&body, a);
    let (c0, c1) = ends(&body, c);
    assert_eq!(c0, a0, "c starts where a starts (V0)");
    assert_ne!(c1, a1, "and ends at V2, not V1");

    let from_a = rim_of(&body, a).expect("the three arcs close one rim");
    assert_eq!(from_a, vec![a, b, c], "the lower surface's winding from a");
    for seed in [b, c] {
        let from_seed = rim_of(&body, seed).expect("every arc names the rim");
        assert_eq!(from_seed[0], seed, "the seed comes first");
        assert!(
            is_rotation(&from_a, &from_seed),
            "from {seed:?}: {from_seed:?} is a rotation of {from_a:?}"
        );
    }
    // Not vacuous: `c` really is stored on the other winding.
    assert_ne!(stored_axis(&body, a), stored_axis(&body, c));
    assert_eq!(stored_axis(&body, a), stored_axis(&body, b));
}

fn is_rotation(a: &[EdgeKey], b: &[EdgeKey]) -> bool {
    a.len() == b.len() && (0..a.len()).any(|k| (0..a.len()).all(|i| a[(i + k) % a.len()] == b[i]))
}

/// An edge's stored carrier axis, as bits.
fn stored_axis(body: &Body<f64>, k: EdgeKey) -> [u64; 3] {
    let g = body
        .get_curve_geom(body.get_edge(k).unwrap().curve)
        .unwrap()
        .certified()
        .unwrap();
    match *g.carrier() {
        Curve3::Circle { axis, .. } => [axis.x.to_bits(), axis.y.to_bits(), axis.z.to_bits()],
        ref other => panic!("a rim arc is a circle, got {other:?}"),
    }
}

/// **A fresh opposite axis closes the rim.** The closing arc tiles the
/// lower half between the same two surfaces, with its axis minted fresh
/// as `(0, 0, -1)`, whose zero bits differ from the seed's negation. No
/// carrier is compared, so the two arcs are one rim from either seed.
#[test]
fn a_fresh_opposite_axis_arc_closes_the_rim() {
    let tol = Tol::witness();
    let (mut body, first) = half_built();
    let e = body.get_edge(first).unwrap();
    // V0 → V1 on the fresh-negated circle over t ∈ (0, π): azimuth
    // 0 → -π, the lower half.
    let second = body
        .mef(
            MefSite::Chords {
                he1: e.he_plus,
                he2: e.he_minus,
            },
            EdgeCurveSpec::arc_of_circle(rim_circle_negated_fresh(), 0.0, PI).unwrap(),
            FaceSurface::New {
                surface: rim_plane(),
                sense: true,
            },
            tol,
        )
        .expect("the lower half certifies against V0 → V1")
        .edge;
    assert_eq!(rim_of(&body, first), Ok(vec![first, second]));
    assert_eq!(rim_of(&body, second), Ok(vec![second, first]));
}
