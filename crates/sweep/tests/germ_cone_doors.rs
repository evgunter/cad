//! **The cone doors under a boolean's operand lanes** — face-level
//! containment on a cone face, and the C5 gate of the face-replacement
//! door asked about the POSE of a routed pair rather than its kinds.
//!
//! Both are ground a cone operand stands on before the verbs admit it:
//! the crossing layer places every landing point with
//! `curved_face_containment`'s placement, and the offset door re-states
//! every boundary edge through the C5 table.
//!
//! - **Face containment on a cone face** answered nothing. It now reads
//!   the double-cone carrier first, then the face's OWN chart trim — the
//!   solid door's slant window and azimuth window, asked about one face
//!   rather than its surface group — and the rows hold what a wrong
//!   answer would break: two half-bands of one frustum PARTITION their
//!   wall, a quarter cone holds its quadrant and not the others, the
//!   mirror nappe and the carrier past the slant window are outside,
//!   and the apex-closed bands of one full cone partition their wall
//!   too, their windows closed at the apex.
//! - **The C5 gate** read `route(kind, kind).implemented`. A quarter
//!   cone's wedge cap passes through the apex, which the plane×cone
//!   arm serves; offset, it no longer does, and the pair cuts a
//!   hyperbola the arm routes to the general rung. The gate admitted it
//!   on the kind pair and the refusal came from the corner re-anchor
//!   downstream (`ReanchorOffCarrier`, `0.0354` m, measured before the
//!   fix). The gate now asks the arm, and refuses the pose by the arm's
//!   own grounds; an axis-normal cap, which the arm does serve, still
//!   passes it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;

use crate::common::approx::band;
use geom_core::{Point2, Point3, Tol};
use profile::{ProfileLoop, RawLoop};
use revolve_common::*;
use sweep::{Revolution, revolve};
use topo::{Body, FaceContainment, FaceKey, ReplaceFaceError};

/// The `revolve_cone` triangle: apex `(0, 1, 0)`, base radius `1` at
/// `y = 0`, half-angle `π/4`. The carrier is `ρ = 1 − y`.
fn triangle() -> ProfileLoop<f64> {
    ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(0.0, 1.0),
    ])
}

/// The full cone: its two bands are APEX-CLOSED.
fn cone() -> Body<f64> {
    revolve(
        &validated(vec![triangle()]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// A frustum clear of its (virtual) apex at `(0, 2, 0)`: base radius
/// `1` at `y = 0`, top radius `0.5` at `y = 1`. The carrier is
/// `ρ = 1 − y/2`.
fn frustum() -> Body<f64> {
    let lp = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(0.5, 1.0),
        Point2::new(0.0, 1.0),
    ]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// The triangle through a quarter turn: ONE cone face over the quadrant
/// `x > 0, z < 0`, two wedge caps through the apex, and the base disc.
fn quarter_cone() -> Body<f64> {
    revolve(
        &validated(vec![triangle()]),
        axis_y(),
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        Tol::witness(),
    )
    .unwrap()
    .body
}

fn faces_where(body: &Body<f64>, pred: impl Fn(&geom::Surface<f64>) -> bool) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| body.get_surface(f.surface).is_some_and(&pred))
        .map(|(k, _)| k)
        .collect()
}

fn cone_faces(body: &Body<f64>) -> Vec<FaceKey> {
    faces_where(body, |s| matches!(s, geom::Surface::Cone { .. }))
}

/// A point at azimuth `phi` about `y` (`x = ρ cos φ`, `z = ρ sin φ`).
fn at(rho: f64, y: f64, phi: f64) -> Point3<f64> {
    let (s, c) = phi.sin_cos();
    Point3::new(rho * c, y, rho * s)
}

fn contain(body: &Body<f64>, f: FaceKey, q: Point3<f64>) -> Option<FaceContainment> {
    topo::curved_face_containment(body, f, q, band()).expect("containment decides")
}

// -------------------------------------------------------------------
// Face containment on a cone face.
// -------------------------------------------------------------------

/// **The frustum's two bands partition their wall, and nothing else is
/// in either.** Every carrier point strictly inside the wall, away from
/// the seam meridians, is `In` exactly one band and `Out` of the other;
/// the same azimuth on the MIRROR nappe above the virtual apex, the
/// carrier past the slant window, and a point a millimetre off the
/// carrier are `Out` of both.
///
/// This is the row a group-scoped trim breaks: the two bands form one
/// wrapped surface group, and reading the GROUP's window for either
/// band puts every wall point in both. A trim on the wrong nappe, or a
/// carrier test that read only one nappe as the surface, moves a
/// mirror or off-carrier point into a band.
#[test]
fn the_frustum_bands_partition_their_wall_under_face_containment() {
    let body = frustum();
    let bands = cone_faces(&body);
    assert_eq!(bands.len(), 2, "a full revolve splits its cone wall in two");
    for k in 0..12 {
        let phi = 0.1 + f64::from(k) * core::f64::consts::TAU / 12.0;
        for y in [0.1, 0.5, 0.9] {
            let p = at(1.0 - y / 2.0, y, phi);
            let verdicts: Vec<_> = bands.iter().map(|&f| contain(&body, f, p)).collect();
            let ins = verdicts
                .iter()
                .filter(|v| **v == Some(FaceContainment::In))
                .count();
            let outs = verdicts
                .iter()
                .filter(|v| **v == Some(FaceContainment::Out))
                .count();
            assert_eq!((ins, outs), (1, 1), "phi {phi}, y {y}: {verdicts:?}");
            for q in [
                // The mirror nappe: `ρ = (y − 2)/2` above the apex.
                at(0.5 * (3.0 - y), 5.0 - y, phi),
                // The carrier below the slant window.
                at(1.0 + y / 2.0, -y, phi),
                // Off the carrier.
                at(1.0 - y / 2.0 + 1e-3, y, phi),
            ] {
                for &f in &bands {
                    assert_eq!(
                        contain(&body, f, q),
                        Some(FaceContainment::Out),
                        "{q:?} is in no band"
                    );
                }
            }
        }
    }
}

/// **The quarter cone holds its own quadrant.** Its one cone face is
/// azimuth-trimmed to `x > 0, z < 0`: a carrier point there is `In`, the
/// same slant station in each of the other three quadrants is `Out`,
/// and so is the mirror nappe above the apex. An arm that read the face
/// as wrapped, or read the window a half-period away (the mirror
/// nappe's chart azimuth), answers `In` somewhere it must not.
#[test]
fn the_quarter_cone_face_holds_its_quadrant_and_no_other() {
    let body = quarter_cone();
    let [f] = cone_faces(&body)[..] else {
        panic!("one cone face");
    };
    let quarter = core::f64::consts::FRAC_PI_2;
    for y in [0.2, 0.5, 0.8] {
        let rho = 1.0 - y;
        let inside = -0.25 * core::f64::consts::PI;
        assert_eq!(
            contain(&body, f, at(rho, y, inside)),
            Some(FaceContainment::In)
        );
        for k in 1..4 {
            let phi = inside + f64::from(k) * quarter;
            assert_eq!(
                contain(&body, f, at(rho, y, phi)),
                Some(FaceContainment::Out),
                "y {y}, phi {phi}"
            );
        }
        assert_eq!(
            contain(&body, f, at(rho, 2.0 - y, inside)),
            Some(FaceContainment::Out),
            "the mirror nappe"
        );
    }
}

/// **The apex-closed bands of one full cone partition their wall, each
/// holding its own half.** The two bands meet at the apex, where every
/// azimuth lands; the apex closure pins each band's own half-period
/// window there. A carrier point away from the seam meridians is `In`
/// the band whose rim arc passes over it — read off the rim's own
/// carrier, independently of any trim — and `Out` of the other. A walk
/// that read either band as a full period answers `None` for both; one
/// that handed each band the other's window answers the swapped band.
/// The carrier test still runs FIRST: a point off the cone is `Out` of
/// both bands whatever the trim can say.
#[test]
fn the_apex_closed_bands_partition_their_wall() {
    let body = cone();
    let bands = cone_faces(&body);
    assert_eq!(bands.len(), 2);
    let rim_azimuth = |f: FaceKey| -> f64 {
        let topo::LoopBoundary::Cycle { first } = body
            .get_loop(body.get_face(f).unwrap().outer)
            .unwrap()
            .boundary
        else {
            panic!("a band's outline is a cycle")
        };
        let rims: Vec<f64> = body
            .loop_cycle(first)
            .unwrap()
            .into_iter()
            .filter_map(|he| {
                let e = body.get_edge(body.get_half_edge(he).unwrap().edge).unwrap();
                let c = body
                    .get_curve_geom(e.curve)
                    .and_then(topo::CurveGeom::certified)?;
                let geom::Curve3::Circle { .. } = c.carrier() else {
                    return None;
                };
                let (t0, t1) = c.params();
                let m = c.carrier().eval(0.5 * (t0 + t1));
                Some(m.z.atan2(m.x))
            })
            .collect();
        let [phi] = rims[..] else {
            panic!("one rim arc per band: {rims:?}")
        };
        phi
    };
    let mids: Vec<f64> = bands.iter().map(|&f| rim_azimuth(f)).collect();
    for k in 0..6 {
        let phi = 0.1 + f64::from(k) * core::f64::consts::TAU / 6.0;
        let p = at(0.5, 0.5, phi);
        for (&f, &mid) in bands.iter().zip(&mids) {
            let d = (phi - mid).rem_euclid(core::f64::consts::TAU);
            let holds = d.min(core::f64::consts::TAU - d) < core::f64::consts::FRAC_PI_2;
            let want = if holds {
                FaceContainment::In
            } else {
                FaceContainment::Out
            };
            assert_eq!(
                contain(&body, f, p),
                Some(want),
                "phi {phi}: the band whose rim runs over {mid}"
            );
            assert_eq!(
                contain(&body, f, at(0.5 + 1e-3, 0.5, phi)),
                Some(FaceContainment::Out)
            );
        }
    }
}

/// The triangle or the frustum revolved through `theta` about `y`.
fn swept(frustum_profile: bool, theta: f64) -> Body<f64> {
    let lp = if frustum_profile {
        ProfileLoop::polygon([
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 0.0),
            Point2::new(0.5, 1.0),
            Point2::new(0.0, 1.0),
        ])
    } else {
        triangle()
    };
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Partial(theta),
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// **Partial revolves partition their swept window and hold nothing
/// outside it.** For each sweep angle, on the triangle's cone and the
/// frustum's, a carrier point strictly inside the swept azimuth window
/// is `In` at most ONE cone face and `Out` of no fewer than all but one
/// of them — never `In` twice, never `Out` everywhere — and a carrier
/// point `1e-5` rad past either edge of the window (more in a coarse ε
/// row, where the trim's band is wider) is `In` no face.
/// The window is found from the face data rather than assumed: every
/// in-window probe is asked of every cone face the body has.
///
/// A trim read a half-period away, a window read as wrapped, or a
/// group-scoped window breaks one of the three.
#[test]
fn partial_revolves_partition_their_window_and_hold_nothing_outside() {
    for frustum_profile in [false, true] {
        let rho_at = |y: f64| {
            if frustum_profile {
                1.0 - y / 2.0
            } else {
                1.0 - y
            }
        };
        for theta in [1.0, core::f64::consts::PI, 4.5] {
            let body = swept(frustum_profile, theta);
            let cones = cone_faces(&body);
            assert!(!cones.is_empty());
            // The sweep runs from the profile plane (`z = 0`, `x > 0`)
            // through `theta` about `+y` by the right-hand rule, which is
            // azimuth `−theta` in `at`'s convention.
            let edge_a = 0.0;
            let edge_b = -theta;
            for y in [0.2, 0.5, 0.8] {
                let rho = rho_at(y);
                for k in 1..8 {
                    let phi = edge_b * f64::from(k) / 8.0;
                    let v: Vec<_> = cones
                        .iter()
                        .map(|&f| contain(&body, f, at(rho, y, phi)))
                        .collect();
                    let ins = v
                        .iter()
                        .filter(|x| **x == Some(FaceContainment::In))
                        .count();
                    let outs = v
                        .iter()
                        .filter(|x| **x == Some(FaceContainment::Out))
                        .count();
                    assert!(ins <= 1, "theta {theta}, y {y}, phi {phi}: in twice {v:?}");
                    assert!(
                        outs < cones.len(),
                        "theta {theta}, y {y}, phi {phi}: out of all {v:?}"
                    );
                }
                // `1e-5` rad past the edge, widened in a coarse ε row so the
                // probe stays definitely clear of the trim's band.
                let off = 1e-5_f64.max(1e3 * eps());
                for phi in [edge_a + off, edge_b - off] {
                    for &f in &cones {
                        assert_ne!(
                            contain(&body, f, at(rho, y, phi)),
                            Some(FaceContainment::In),
                            "theta {theta}, y {y}: phi {phi} is outside the sweep"
                        );
                    }
                }
            }
        }
    }
}

// -------------------------------------------------------------------
// The C5 gate, asked about the pose.
// -------------------------------------------------------------------

fn wedge_caps(body: &Body<f64>) -> Vec<FaceKey> {
    faces_where(
        body,
        |s| matches!(s, geom::Surface::Plane { normal, .. } if normal.y.abs() < 0.5),
    )
}

/// **An offset wedge cap leaves the pose its arm serves, and the gate
/// says so.** Each cap passes through the apex, which the plane×cone
/// arm serves (the generator pair); moved by `±0.05` it stands off the
/// apex and parallel to the axis, and cuts the cone in a hyperbola the
/// arm routes to the general rung. The door refuses at the C5 gate by
/// the arm's own grounds, before any corner is re-anchored, and the
/// body is untouched.
///
/// Red before the gate asked about the pose: the kind pair passed and
/// the refusal was `ReanchorOffCarrier` on the generator edge, `0.0354`
/// m of corner error — a pose admitted as served and caught by the
/// next gate down.
#[test]
fn an_offset_wedge_cap_refuses_at_the_pose_gate() {
    let body = quarter_cone();
    let caps = wedge_caps(&body);
    assert_eq!(caps.len(), 2);
    for &cap in &caps {
        for d in [0.05, -0.05] {
            let mut work = body.clone();
            let before = format!("{work:?}");
            let e = topo::replace_face_offset(&mut work, cap, d, Tol::witness())
                .expect_err("the moved cap cuts a hyperbola");
            let ReplaceFaceError::NeighborPoseUnroutable {
                kind,
                other_kind,
                why,
                ..
            } = e
            else {
                panic!("cap {cap:?}, d {d}: expected the pose refusal, got {e}");
            };
            assert_eq!(
                (kind, other_kind),
                (geom_brep::SurfaceKind::Plane, geom_brep::SurfaceKind::Cone)
            );
            assert!(why.contains("general rung"), "{why}");
            assert_eq!(before, format!("{work:?}"), "body moved across an Err");
        }
    }
}

/// **A pose the arm serves still passes, and the late refusal leaves
/// the body untouched.** The base disc is axis-normal, and an
/// axis-normal plane off the apex is the circle the plane×cone arm
/// mints; offset, it stays axis-normal. The gate must not refuse it:
/// the refusal that stops this door is the corner re-anchor at the
/// wedge caps, decided after the mint and inside the boundary plan, so
/// the body is untouched across it. The magnitude is pinned —
/// `|d|·sin α` at `α = π/4`, the rim vertex's slide along the generator
/// it must still stand on.
#[test]
fn an_offset_axis_normal_cap_passes_the_pose_gate() {
    let body = quarter_cone();
    let [disc] = faces_where(
        &body,
        |s| matches!(s, geom::Surface::Plane { normal, .. } if normal.y.abs() > 0.5),
    )[..] else {
        panic!("one base disc");
    };
    for d in [0.05, -0.05] {
        let mut work = body.clone();
        let before = format!("{work:?}");
        let e = topo::replace_face_offset(&mut work, disc, d, Tol::witness())
            .expect_err("the wedge caps cannot follow the moved disc");
        let ReplaceFaceError::ReanchorOffCarrier { gap, .. } = e else {
            panic!(
                "d {d}: the axis-normal pose is served; expected the re-anchor refusal, got {e}"
            );
        };
        assert!(
            (gap - 0.05 * core::f64::consts::FRAC_1_SQRT_2).abs() <= 1e-12,
            "d {d}: the corner error is |d|·sin(pi/4), got {gap}"
        );
        assert_eq!(before, format!("{work:?}"), "body moved across a late Err");
    }
}
