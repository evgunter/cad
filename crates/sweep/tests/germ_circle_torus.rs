//! GERM: the circle × torus root lane at the boolean's crossing layer
//! (`topo::boolean::circle_torus`, routed from the circle rung of
//! `reduce::curved_face_arm` through `wall_crossing`).
//!
//! Before the lane, a circle edge against a torus face was decided only
//! by a certified clearance of the arc; anything else refused
//! `CurvedPierceUnsupported`.
//!
//! # The fixture family
//!
//! `B` is a quarter tube: ring centre at the origin, axis `−y`, azimuth
//! running from `+x` toward `+z`, window `[0°, 90°]`, `R = 1`, `r = 0.2`.
//! `A` is a short tube (`R_A = 0.3`, `r_A = 0.05`) whose ring lies in
//! the plane `y = 0.1`, half a turn facing (or facing away from) `B`'s
//! axis. Its equator seams are circles in that plane, and the plane cuts
//! `B`'s tube in the two circles `ρ = 1 ± √(r² − 0.1²)`, so every
//! crossing is transverse and lands OFF `B`'s own equator seams (which
//! lie in `y = 0`) — a face-interior pierce, not an edge contact.
//!
//! The rows read the reduction sweep only (`sweep_traces`): whether it
//! refuses, and which (edge, face) pairs the exact predicates accepted.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol, Vec3};
use sweep::test_support::tube_frame;
use sweep::{TubeWindow, tube_along_arc};
use topo::{Body, BooleanError, EdgeKey, FaceKey, SweepStrategy, sweep_traces};

use crate::mate7a_r1_probes::{arch, stem, weld_declarations};

const B_MAJOR: f64 = 1.0;
const B_MINOR: f64 = 0.2;
const A_MAJOR: f64 = 0.3;
const A_MINOR: f64 = 0.05;
const A_HEIGHT: f64 = 0.1;

/// `B`'s tube met by the plane `y = A_HEIGHT`: its outer contour radius.
fn b_outer_contour() -> f64 {
    B_MAJOR + (B_MINOR.powi(2) - A_HEIGHT.powi(2)).sqrt()
}

fn tube(
    center: Point3<f64>,
    axis: Vec3<f64>,
    radial: Vec3<f64>,
    major: f64,
    minor: f64,
    turn: f64,
) -> Body<f64> {
    tube_along_arc(
        tube_frame(center, axis, radial, Tol::witness()),
        major,
        TubeWindow::Arc { t0: 0.0, t1: turn },
        minor,
        Tol::witness(),
    )
    .expect("tube builds")
    .body
}

fn b_quarter() -> Body<f64> {
    tube(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        B_MAJOR,
        B_MINOR,
        core::f64::consts::FRAC_PI_2,
    )
}

/// `A`, its ring centre at azimuth `alpha` about `B`'s axis and distance
/// `d` from it, at height `A_HEIGHT`. `toward` puts the half turn facing
/// `B`'s axis; otherwise it faces away.
fn a_half(alpha: f64, d: f64, toward: bool) -> Body<f64> {
    let (s, c) = alpha.sin_cos();
    let center = Point3::new(d * c, A_HEIGHT, d * s);
    // Starting on the ring's tangent line: the half turn then sweeps
    // through the radial point nearest (or farthest from) `B`'s axis.
    let axis = if toward {
        Vec3::new(0.0, 1.0, 0.0)
    } else {
        Vec3::new(0.0, -1.0, 0.0)
    };
    tube(
        center,
        axis,
        Vec3::new(s, 0.0, -c),
        A_MAJOR,
        A_MINOR,
        core::f64::consts::PI,
    )
}

/// `body`'s edges riding a circle of this radius.
fn circle_edges(body: &Body<f64>, radius: f64) -> Vec<EdgeKey> {
    body.edges()
        .filter_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve)?.certified()?;
            match *c.carrier() {
                geom::Curve3::Circle { radius: r, .. } if (r - radius).abs() < 1e-12 => Some(k),
                _ => None,
            }
        })
        .collect()
}

fn torus_faces(body: &Body<f64>) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Torus { .. })
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// The A→B sweep's accepted (edge, torus face) pairs for `edges`.
fn accepted_against_torus(
    a: &Body<f64>,
    b: &Body<f64>,
    edges: &[EdgeKey],
) -> Vec<(EdgeKey, FaceKey)> {
    let (ab, _) = sweep_traces(a, b, SweepStrategy::Realized, None, Tol::witness())
        .unwrap_or_else(|e| panic!("the sweep refused: {e:?}"));
    let walls = torus_faces(b);
    ab.accepted
        .into_iter()
        .filter(|(e, f)| edges.contains(e) && walls.contains(f))
        .collect()
}

/// **The lily's stem glue is past the crossing layer.** Red on the base
/// as `CurvedPierceUnsupported` at the stem's inner equator seam
/// against the arch's tube wall, and — once that pair had a lane — at
/// the arch's outer equator seam against the stem's wall, whose one
/// crossing on the arc lies on the stem's carrier past the stem face's
/// 22° window (the certified negative: the root is placed `Out` in the
/// stem face's own chart, both ends are definitely off the carrier, so
/// the arc meets that face nowhere). Both pairs now answer, and the op
/// stops at the join, where the germ pair of the stem's weld cap
/// (plane) against the arch's wall (torus) has no section frame arm:
/// a typed refusal downstream, and no body to measure.
#[test]
fn the_lily_stem_glue_is_past_the_circle_torus_pairs() {
    let (s, a) = (stem(), arch());
    let (decls, _) = weld_declarations(&s, &a);
    let err = topo::union_with(&s, &a, &decls, Tol::witness())
        .expect_err("the stem glue still refuses, at the join");
    let BooleanError::GermFrameUnsupported {
        a_face,
        a_kind: geom_brep::SurfaceKind::Plane,
        b_face,
        b_kind: geom_brep::SurfaceKind::Torus,
    } = err
    else {
        panic!("the lily's next door is the plane × torus germ frame: {err:?}");
    };
    assert!(
        matches!(
            s.get_face(a_face).and_then(|f| s.get_surface(f.surface)),
            Some(geom::Surface::Plane { .. })
        ),
        "the stem's face is its weld cap: {a_face:?}"
    );
    assert!(
        torus_faces(&a).contains(&b_face),
        "the arch's face is its tube wall"
    );
}

/// **Two crossings on one arc, the FIRST outside the face's window and
/// the second inside.** `A` faces AWAY from `B`'s axis from azimuth 0°
/// at distance 1, so its inner seam (radius 0.25) starts inside `B`'s
/// tube at azimuth ≈ −14°, leaves it through the outer contour at a
/// NEGATIVE azimuth (past the window: `Out`), passes its apex clear of
/// the tube (and of `B`'s cap disc at azimuth 0), and re-enters at a
/// positive azimuth (inside). Both ends are inside the tube, so the
/// endpoint signs promise nothing: only the roots see the crossing, and
/// a lane that stopped at the first one would read the pair clear.
#[test]
fn an_arc_whose_first_crossing_is_outside_the_window_still_pierces_inside_it() {
    let (a, b) = (a_half(0.0, 1.0, false), b_quarter());
    let seam = circle_edges(&a, A_MAJOR - A_MINOR);
    assert_eq!(seam.len(), 1, "one inner seam");
    let hits = accepted_against_torus(&a, &b, &seam);
    assert!(!hits.is_empty(), "the in-window crossing is a pierce");
}

/// **The certified negative, on the lily's own pairs.** The arch's
/// outer equator seam crosses the stem's torus CARRIER on its arc, at a
/// point past the stem face's 22° window, and its two ends are
/// definitely off that carrier (one inside the tube at the weld, one
/// far outside). The root is placed `Out` in the stem face's chart, so
/// the pair is examined and is NO event — against both of the stem's
/// wall faces — and the sweep does not refuse. On the base this pair
/// refused `CurvedPierceUnsupported` (the straddle arm read a crossing
/// accounted for off the face as a contradiction). The stem's inner
/// seam against the arch's walls is the same certificate from the
/// other side: two roots on the arc, both before the arch face's
/// window.
#[test]
fn the_lily_seams_cross_each_others_carriers_only_outside_the_windows() {
    let (s, a) = (stem(), arch());
    let (ab, ba) = sweep_traces(&s, &a, SweepStrategy::Realized, None, Tol::witness())
        .unwrap_or_else(|e| panic!("the lily's reduction sweep refused: {e:?}"));
    for (label, trace, x, y, radius) in [
        ("stem inner seam × arch walls", &ab, &s, &a, 5.0 - 0.060),
        ("arch outer seam × stem walls", &ba, &a, &s, 1.1 + 0.052),
    ] {
        let seam = circle_edges(x, radius);
        let walls = torus_faces(y);
        assert_eq!(seam.len(), 1, "{label}: one seam");
        let pairs = |v: &[(EdgeKey, FaceKey)]| {
            v.iter()
                .filter(|(e, f)| seam.contains(e) && walls.contains(f))
                .count()
        };
        assert_eq!(
            pairs(&trace.examined),
            walls.len(),
            "{label}: examined against every wall face"
        );
        assert_eq!(pairs(&trace.accepted), 0, "{label}: and no event on any");
    }
}

/// **The inner-contour region**: `A` facing AWAY from `B`'s axis at
/// distance 0.62, so its outer seam (radius 0.35) runs from the hole
/// (`ρ ≈ 0.71`) into the tube (`ρ ≈ 0.97` at its apex) and back out —
/// two crossings of `B`'s INNER contour `ρ = 1 − √0.03`, both inside
/// the window.
#[test]
fn an_arc_through_the_inner_contour_pierces() {
    let (a, b) = (a_half(45_f64.to_radians(), 0.62, false), b_quarter());
    let seam = circle_edges(&a, A_MAJOR + A_MINOR);
    assert_eq!(seam.len(), 1, "one outer seam");
    assert!(
        !accepted_against_torus(&a, &b, &seam).is_empty(),
        "the inner-contour crossings are pierces"
    );
}

/// **A tangent arc refuses.** `A`'s inner seam touches `B`'s outer
/// contour at its apex (a double root of the quartic), inside the
/// window: the ladder has no certified count, and the pair keeps the
/// typed frontier door.
#[test]
fn a_tangent_arc_refuses_typed() {
    let d = b_outer_contour() + (A_MAJOR - A_MINOR);
    let (a, b) = (a_half(45_f64.to_radians(), d, true), b_quarter());
    let seam = circle_edges(&a, A_MAJOR - A_MINOR);
    let err = sweep_traces(&a, &b, SweepStrategy::Realized, None, Tol::witness())
        .expect_err("a graze is not a crossing this lane can act on");
    let BooleanError::CurvedPierceUnsupported { edge, face, .. } = err else {
        panic!("the tangent pair keeps the frontier door: {err:?}");
    };
    assert!(seam.contains(&edge), "the refusal names the tangent seam");
    assert!(torus_faces(&b).contains(&face), "against B's wall");
}

/// **A coaxial circle**: `A`'s ring coaxial with `B`'s (centre on `B`'s
/// axis, parallel axes), lifted clear of the tube. Its residual is
/// constant along every seam, so the clearance decides and no pair is
/// accepted — and no refusal.
#[test]
fn a_coaxial_circle_clear_of_the_tube_is_no_event() {
    let a = tube(
        Point3::new(0.0, 0.4, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        B_MAJOR,
        A_MINOR,
        core::f64::consts::FRAC_PI_2,
    );
    let b = b_quarter();
    let seams: Vec<EdgeKey> = [B_MAJOR - A_MINOR, B_MAJOR + A_MINOR]
        .iter()
        .flat_map(|&r| circle_edges(&a, r))
        .collect();
    assert!(!seams.is_empty(), "the coaxial seams exist");
    assert!(accepted_against_torus(&a, &b, &seams).is_empty());
}

/// **The first-Out row, mirrored** (review of PR 3375): on the row
/// above the solver lists the in-window root FIRST, so a lane that
/// stopped at the first `Out` root still passed it. Mirroring `B`'s
/// window across `A`'s plane of symmetry — `[−90°, 0°]` instead of
/// `[0°, 90°]` — swaps which of the two crossings lands in the window
/// while the solver's order stays, so here the `Out` root is listed
/// first and the pierce after it. A lane that returned at the first
/// `Out` root reads the pair clear, silently.
#[test]
fn an_arc_whose_first_listed_root_is_outside_the_window_still_pierces() {
    let a = a_half(0.0, 1.0, false);
    let b_mirror = tube(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
        B_MAJOR,
        B_MINOR,
        core::f64::consts::FRAC_PI_2,
    );
    for (label, b) in [
        ("the window [0°, 90°]", b_quarter()),
        ("its mirror", b_mirror),
    ] {
        let seam = circle_edges(&a, A_MAJOR - A_MINOR);
        assert!(
            !accepted_against_torus(&a, &b, &seam).is_empty(),
            "{label}: the in-window crossing is a pierce"
        );
    }
}

/// `B`'s torus `F` at a world point (ring centre at the origin, axis
/// `±y`, `R`, `r`).
fn torus_f(p: Point3<f64>, big_r: f64, r: f64) -> f64 {
    let s = p.x * p.x + p.y * p.y + p.z * p.z;
    (s + big_r * big_r - r * r).powi(2) - 4.0 * big_r * big_r * (s - p.y * p.y)
}

/// A circle carrier read back off an edge: centre, axis, radius, `u_ref`
/// and the edge's parameter span.
fn carrier_of(
    body: &Body<f64>,
    edge: EdgeKey,
) -> (Point3<f64>, Vec3<f64>, f64, Vec3<f64>, (f64, f64)) {
    let e = body.get_edge(edge).unwrap();
    let c = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
    let geom::Curve3::Circle {
        center,
        axis,
        radius,
        u_ref,
    } = *c.carrier()
    else {
        panic!("a circle edge");
    };
    (center, axis, radius, u_ref, c.params())
}

fn on_carrier(c: (Point3<f64>, Vec3<f64>, f64, Vec3<f64>), theta: f64) -> Point3<f64> {
    let (center, axis, radius, u) = c;
    center + (u * theta.cos() + axis.cross(u) * theta.sin()) * radius
}

/// The carrier's roots against `B`'s torus over one turn, bisected.
fn carrier_roots(c: (Point3<f64>, Vec3<f64>, f64, Vec3<f64>), big_r: f64, r: f64) -> Vec<f64> {
    let f = |t: f64| torus_f(on_carrier(c, t), big_r, r);
    let n = 20_000;
    let tau = core::f64::consts::TAU;
    let mut out = Vec::new();
    for i in 0..n {
        let (mut a, mut b) = (
            tau * f64::from(i) / f64::from(n),
            tau * f64::from(i + 1) / f64::from(n),
        );
        if f(a) * f(b) >= 0.0 {
            continue;
        }
        for _ in 0..80 {
            let m = 0.5 * (a + b);
            if f(a) * f(m) <= 0.0 {
                b = m;
            } else {
                a = m;
            }
        }
        out.push(0.5 * (a + b));
    }
    out
}

/// **The pole beside a root off the arc, through the whole sweep**
/// (review of PR 3375, executed). `A` is a tilted half-width tube whose
/// inner seam (radius 0.5) is centred on `B`'s tube-centre circle, so
/// the seam's carrier crosses `B`'s tube four times. `A` is turned
/// about its own axis until the seam arc's ANTIPODE — the half-angle
/// map's first pole — sits `δ` from one of those crossings, which the
/// arc does not hold, while the arc holds another inside `B`'s window.
/// Unconditioned, the ladder dropped the in-arc root: the pair was
/// examined, nothing was accepted, nothing refused — a silent miss of
/// a seam dipping 0.2 m into `B`. The pierce must be found (or the pair
/// refused); here it is found.
#[test]
fn a_seam_whose_antipode_sits_beside_an_off_arc_root_still_pierces() {
    let (big_r, r) = (1.0, 0.25);
    let b = tube(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        big_r,
        r,
        2.4,
    );
    let b = {
        // Centre the window on `+x`: turn the quarter's start back 1.2.
        let _ = b;
        tube(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(1.2_f64.cos(), 0.0, -(1.2_f64.sin())),
            big_r,
            r,
            2.4,
        )
    };
    let axis = Vec3::new(1.0, 0.0, 0.0)
        .cross(Vec3::new(0.0, 0.4, 1.0))
        .normalize();
    let center = Point3::new(1.0, 0.0, 0.0);
    let build = |radial: Vec3<f64>| tube(center, axis, radial, 0.55, 0.05, 2.0);
    let rotate = |v: Vec3<f64>, beta: f64| {
        v * beta.cos() + axis.cross(v) * beta.sin() + axis * axis.dot(v) * (1.0 - beta.cos())
    };
    let a0 = build(Vec3::new(1.0, 0.0, 0.0));
    let seam0 = circle_edges(&a0, 0.5)[0];
    let (c0, n0, rho, u0, (t0, t1)) = carrier_of(&a0, seam0);
    let carrier = (c0, n0, rho, u0);
    let roots = carrier_roots(carrier, big_r, r);
    assert_eq!(
        roots.len(),
        4,
        "the seam's carrier crosses B's tube four times: {roots:?}"
    );
    let tau = core::f64::consts::TAU;
    let wrap = |x: f64| (x + core::f64::consts::PI).rem_euclid(tau) - core::f64::consts::PI;
    for delta in [-1e-4, 1e-5, 1e-6] {
        // An off-arc root whose antipodal arc holds another root.
        let (star, arc_mid) = roots
            .iter()
            .find_map(|&star| {
                let m = star + delta - core::f64::consts::PI;
                roots
                    .iter()
                    .any(|&q| wrap(q - m).abs() < 0.9)
                    .then_some((star, m))
            })
            .expect("some root's antipodal arc holds another root");
        let beta = arc_mid - 0.5 * (t0 + t1);
        let a = [beta, -beta]
            .into_iter()
            .map(|b| build(rotate(Vec3::new(1.0, 0.0, 0.0), b)))
            .find(|a| {
                let seam = circle_edges(a, 0.5)[0];
                let (c, n, rh, u, (s0, s1)) = carrier_of(a, seam);
                let antipode = on_carrier((c, n, rh, u), 0.5 * (s0 + s1) + core::f64::consts::PI);
                (antipode - on_carrier(carrier, star + delta)).norm() < 1e-9
            })
            .expect("one sense of the turn puts the antipode beside the root");
        let seam = circle_edges(&a, 0.5);
        let hits = accepted_against_torus(&a, &b, &seam);
        assert!(
            !hits.is_empty(),
            "δ = {delta}: the in-arc crossing is a pierce"
        );
    }
}

/// **A small tilted circle on the QUARTIC arm** (review of PR 3375,
/// executed): a tube `R = 0.3, r = 0.05` whose axis is tilted off `B`'s,
/// centred just outside `B`'s tube-centre circle, so its seams cross
/// `B`'s wall in a generic pose. With the ladder metered over the torus's
/// extent the margins scaled like `ρ¹⁰/(R + r)¹¹` and this escalated at
/// the discriminant; metered over the circle's own size it answers.
#[test]
fn a_small_tilted_seam_crosses_the_wall_on_the_quartic_arm() {
    let az = 20_f64.to_radians();
    let center = Point3::new(1.1 * az.cos(), 0.05, 1.1 * az.sin());
    let axis = Vec3::new(0.2, 1.0, 0.3).normalize();
    let radial = axis.cross(Vec3::new(0.0, 0.0, 1.0)).normalize();
    let a = tube(
        center,
        axis,
        radial,
        A_MAJOR,
        A_MINOR,
        core::f64::consts::PI,
    );
    let b = b_quarter();
    let seams: Vec<EdgeKey> = [A_MAJOR - A_MINOR, A_MAJOR + A_MINOR]
        .iter()
        .flat_map(|&r| circle_edges(&a, r))
        .collect();
    assert_eq!(seams.len(), 2, "two seams");
    assert!(
        !accepted_against_torus(&a, &b, &seams).is_empty(),
        "the tilted seams pierce B's wall"
    );
}

/// **The coaxial routing: a seam lying ON the torus.** `A` is coaxial
/// with `B`, its ring at height 0.1, and its inner seam's radius is the
/// radius of `B`'s outer contour at that height — a latitude circle of
/// `B`'s carrier. (Its tube crosses `B`'s transversally, so no tangency
/// elsewhere refuses first.) The seam's clearance is zero, the door
/// answers `Coaxial`, `wall_crossing` answers `Constant`, and the
/// undeclared `(Zero, Zero)` arm keeps the typed frontier door on that
/// seam: an undeclared on-carrier circle is never a silent no-event.
#[test]
fn a_coaxial_seam_on_the_torus_keeps_the_door() {
    let contour = B_MAJOR + (B_MINOR.powi(2) - A_HEIGHT.powi(2)).sqrt();
    let a = tube(
        Point3::new(0.0, A_HEIGHT, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        contour + A_MINOR,
        A_MINOR,
        core::f64::consts::FRAC_PI_4,
    );
    let b = b_quarter();
    let seam = circle_edges(&a, contour);
    assert_eq!(seam.len(), 1, "the seam on B's contour");
    let err = sweep_traces(&a, &b, SweepStrategy::Realized, None, Tol::witness())
        .expect_err("an undeclared on-carrier seam keeps the door");
    let BooleanError::CurvedPierceUnsupported { edge, .. } = err else {
        panic!("the frontier door: {err:?}");
    };
    assert!(
        seam.contains(&edge),
        "the refusal names the coaxial seam: {err:?}"
    );
}

