//! FILLET-RIM review probes (r1), `topo` half — what the rim door's
//! TOPOLOGICAL tiling test and its bit-equal axis match admit, on
//! bodies hand-assembled through the Euler doors so every stored
//! carrier is stated exactly.
//!
//! Three rows, each a claim of `docs/FILLET-RIM-SPEC.md` or of the
//! door's own doc comment, executed rather than read:
//!
//! 1. The tiling test is vertex-key equality, so two arcs of one circle
//!    that both cover the SAME half of it — a double cover leaving the
//!    other half bare — close the walk and come back as a rim.
//! 2. The match admits an axis that is the bit-equal NEGATION of the
//!    seed's. On a three-arc rim where one arc is stored on the
//!    negated axis, every seed's answer runs its own carrier's
//!    positive direction — and the two windings are opposite, so the
//!    two answers are NOT rotations of each other.
//! 3. The stated residue: an opposite axis minted fresh with the other
//!    signed zero is not matched, and the door refuses `NotOneRim`
//!    naming the seam vertex — although the arcs do tile the circle.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Point3, Tol, Vec3};
use topo::query::rim_of;
use topo::{Body, EdgeKey, FaceSurface, MefSite, MevSite, RimError, VertexKey};

fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

fn v3(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

const RIM_Z: f64 = 0.5;

fn rim_r() -> f64 {
    (1.0 - RIM_Z * RIM_Z).sqrt()
}

fn unit_sphere() -> Surface<f64> {
    Surface::Sphere {
        center: p3(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: v3(0.0, 0.0, 1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

fn rim_plane() -> Surface<f64> {
    Surface::Plane {
        origin: p3(0.0, 0.0, RIM_Z),
        normal: v3(0.0, 0.0, 1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// The rim circle, wound `+z`.
fn rim_circle() -> Curve3<f64> {
    Curve3::Circle {
        center: p3(0.0, 0.0, RIM_Z),
        axis: v3(0.0, 0.0, 1.0),
        radius: rim_r(),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// The same circle wound the other way, with `axis` spelled as the
/// NEGATED VALUE of the seed's (`-(0, 0, 1)` = `(-0, -0, -1)`), which
/// is the spelling the door's match admits. Its parameter `t` is at
/// world azimuth `-t`.
fn rim_circle_negated_value() -> Curve3<f64> {
    Curve3::Circle {
        center: p3(0.0, 0.0, RIM_Z),
        axis: -v3(0.0, 0.0, 1.0),
        radius: rim_r(),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// The same circle wound the other way, with `axis` MINTED FRESH as
/// `(0, 0, -1)` — positive zeros — the residue the door's own comment
/// names.
fn rim_circle_negated_fresh() -> Curve3<f64> {
    Curve3::Circle {
        center: p3(0.0, 0.0, RIM_Z),
        axis: v3(0.0, 0.0, -1.0),
        radius: rim_r(),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// The world point at azimuth `theta` on the rim.
fn at(theta: f64) -> Point3<f64> {
    p3(rim_r() * theta.cos(), rim_r() * theta.sin(), RIM_Z)
}

/// The sphere face alone, with ONE arc of the rim (world azimuth
/// `0 → π`, wound `+z`) out and back inside it — `topo/tests/rim_of.rs`'s
/// `half_built`, restated.
fn half_built() -> (Body<f64>, EdgeKey) {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(at(0.0)).unwrap();
    body.set_face_surface(seed.face, FaceSurface::New(unit_sphere()))
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

fn is_rotation(a: &[EdgeKey], b: &[EdgeKey]) -> bool {
    a.len() == b.len() && (0..a.len()).any(|k| (0..a.len()).all(|i| a[(i + k) % a.len()] == b[i]))
}

/// **A double cover of half the circle passes the tiling test.** The
/// first arc covers azimuth `0 → π` wound `+z`; the second is stated on
/// the negated-value circle over `t ∈ (π, 2π)`, i.e. azimuth `-π → -2π`
/// — the SAME upper half, traversed back from `V1` to `V0`. Every
/// stored `center`, `radius` and `|axis|` bit matches, the walk closes
/// on the second step having consumed both arcs, and the door answers
/// a rim whose two arcs overlap and leave the lower half bare.
#[test]
fn a_double_cover_of_half_the_circle_is_answered_as_a_rim() {
    let tol = Tol::witness();
    let (mut body, first) = half_built();
    let e = body.get_edge(first).unwrap();
    // `he_plus` of the new edge runs start(he1) → start(he2) = V1 → V0.
    let made = body
        .mef(
            MefSite::Chords {
                he1: e.he_minus,
                he2: e.he_plus,
            },
            EdgeCurveSpec::arc_of_circle(rim_circle_negated_value(), PI, TAU).unwrap(),
            FaceSurface::New(rim_plane()),
            tol,
        )
        .expect("the second arc certifies against V1 → V0");
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
}

/// **With a negated-axis arc in the rim, the two answers are not
/// rotations of each other.** Three arcs tile the circle: `a` and `b`
/// wound `+z` (azimuth `0 → 2π/3 → 4π/3`), `c` stored on the
/// negated-value circle (`t ∈ (0, 2π/3)`, azimuth `0 → -2π/3`, i.e.
/// `V0 → V2` closing the last third). Each seed's answer runs its own
/// carrier's positive direction, as the door documents — and the two
/// directions are opposite, so `rim_of(c)` is the reversal of
/// `rim_of(a)`, not a rotation of it.
#[test]
fn a_negated_axis_arc_breaks_the_rotation_claim_on_a_three_arc_rim() {
    let tol = Tol::witness();
    let (v0, v1, v2) = (0.0, 2.0 * PI / 3.0, 4.0 * PI / 3.0);
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(at(v0)).unwrap();
    body.set_face_surface(seed.face, FaceSurface::New(unit_sphere()))
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
            FaceSurface::New(rim_plane()),
            tol,
        )
        .expect("the closing arc certifies against V0 → V2")
        .edge;
    let (a0, a1) = ends(&body, a);
    let (c0, c1) = ends(&body, c);
    assert_eq!(c0, a0, "c starts where a starts (V0)");
    assert_ne!(c1, a1, "and ends at V2, not V1");

    let from_a = rim_of(&body, a).expect("three arcs tile the circle");
    let from_b = rim_of(&body, b).expect("from b too");
    let from_c = rim_of(&body, c).expect("and from the negated-axis arc");
    assert_eq!(from_a, vec![a, b, c]);
    assert!(is_rotation(&from_a, &from_b), "a and b share a winding");
    assert_eq!(
        from_c,
        vec![c, b, a],
        "the negated-axis seed walks its own positive direction — the reverse"
    );
    assert!(
        !is_rotation(&from_a, &from_c),
        "the doc's unconditional rotation claim fails once the match's own \
         negated-axis admission is exercised"
    );
}

/// **The residue, executed.** The closing arc tiles the lower half
/// exactly, between the same two surfaces, but its axis is minted
/// fresh as `(0, 0, -1)`: `-(0, 0, -1)` is `(-0, -0, 1)`, whose zero
/// bits differ from the seed's `(0, 0, 1)`. The arc is not matched,
/// the walk dangles at `V1`, and the refusal says the arcs "do not
/// tile one closed circle" at parameter `π` — the diagnosis names the
/// tiling, while the cause is the identity comparison.
#[test]
fn a_fresh_opposite_axis_refuses_not_one_rim_although_the_arcs_tile() {
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
            FaceSurface::New(rim_plane()),
            tol,
        )
        .expect("the lower half certifies against V0 → V1")
        .edge;
    // Same point set: the second arc's midpoint is the antipode of the
    // first's on the rim.
    match rim_of(&body, first) {
        Err(RimError::NotOneRim { arcs, gap }) => {
            assert_eq!(arcs, vec![first], "the fresh-axis arc is not matched");
            assert!(
                (gap - PI).abs() < 1e-12,
                "the walk dangles at V1 (π), got {gap}"
            );
        }
        other => panic!("the residue refuses NotOneRim, got {other:?}"),
    }
    assert!(
        matches!(rim_of(&body, second), Err(RimError::NotOneRim { .. })),
        "and from the other seed too"
    );
}
