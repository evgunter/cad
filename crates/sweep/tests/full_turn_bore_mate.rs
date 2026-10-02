//! **A shaft set into a full-turn bore, its cylindrical `Rest`
//! declared, unions.**
//!
//! The collar is the rectangle `ρ ∈ [0.5, 1.5]`, `y ∈ [1, 2]` revolved
//! a full turn about `y`: its bore is ONE face with a self-mated seam
//! ruling at azimuth 0, bounded by two rim circles that each carry one
//! vertex. The shaft is the three-arc peg of radius 0.5 (three wall
//! thirds, three seam rulings) turned so its axis is `y`, its first
//! ruling at azimuth `a`. Every bore × peg-wall pair is declared `Rest`.
//!
//! What the mate makes the kernel do, by the shaft's azimuth and span:
//!
//! - a peg ruling passing a rim away from the rim's vertex has both its
//!   ends past the bore while the rim crosses it — recorded where the
//!   ruling crosses the collar's flat cap, and read as no event against
//!   the bore once its interior is certified clear of the bore's
//!   boundary;
//! - the bore is one face where the peg is three, so the two solids
//!   divide the contact band differently, and the zip's seam runs where
//!   only one of them has an edge;
//! - at an azimuth off the collar's seam, the collar's seam vertices sit
//!   inside a peg wall third, with the collar's flat caps leaving them
//!   radially;
//! - with the bore split by a circle on its own carrier, the rulings
//!   cross that circle where no other face meets it, and only the
//!   crossing layer records them.
//!
//! Each union runs in both operand orders.
//! Every pose of [`crate::common::poses::poses`] moves both operands.
//! The oracle is closed form: the interiors are disjoint, so the union
//! is the collar's annulus volume plus the shaft's disc volume, the
//! intersection is empty, and each difference is its minuend whole.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::mate2_common::{peg_at, wall_decls};
use core::f64::consts::{FRAC_PI_2, PI};
use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, BooleanOp, BooleanResult, mass_properties};

/// The collar's bore and outer radii and its span in `y`.
const BORE: f64 = 0.5;
const OUTER: f64 = 1.5;
const COLLAR: (f64, f64) = (1.0, 2.0);

fn collar() -> Body<f64> {
    let (y0, y1) = COLLAR;
    let lp = ProfileLoop::polygon([
        Point2::new(BORE, y0),
        Point2::new(OUTER, y0),
        Point2::new(OUTER, y1),
        Point2::new(BORE, y1),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

/// The three-arc peg, first ruling at azimuth `deg` (from `+x`, the
/// collar's seam), spanning `y ∈ [y0, y0 + h]`.
fn shaft(deg: f64, y0: f64, h: f64) -> Body<f64> {
    // A quarter turn about `x` takes the peg's `z` axis to `y` and its
    // sketch azimuth `θ` (from `+x` toward `+y`) to the azimuth `θ` from
    // `+x` toward `−z`, which is the revolve's own sense about `+y`.
    let up = Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        -FRAC_PI_2,
    );
    topo::transform_rigid(&peg_at(deg, y0, h), &up, Tol::witness()).unwrap()
}

fn placed(b: &Body<f64>, pose: &Affine3<f64>) -> Body<f64> {
    topo::transform_rigid(b, pose, Tol::witness()).unwrap()
}

fn volume(b: &Body<f64>) -> f64 {
    mass_properties(b, Tol::witness()).unwrap().volume
}

/// The collar's closed-form volume, `π(R² − r²)·h`.
fn collar_volume() -> f64 {
    PI * (OUTER * OUTER - BORE * BORE) * (COLLAR.1 - COLLAR.0)
}

/// The shaft's closed-form volume, `π r² h`: its three arcs close one
/// circle.
fn shaft_volume(h: f64) -> f64 {
    PI * BORE * BORE * h
}

/// The shaft spans, by name: through both rims; exactly the bore; and
/// flush at one rim, proud of the other. A shaft ending inside the
/// bore is `work/zip/blind-shaft-in-a-full-turn-bore-revisits-the-seam-vertex.md`.
const SPANS: [(&str, f64, f64); 4] = [
    ("through", 0.5, 2.0),
    ("flush", 1.0, 1.0),
    ("proud above", 1.0, 1.5),
    ("proud below", 0.5, 1.5),
];

/// `got` against the closed form, relative to the larger operand.
fn agrees(got: f64, want: f64) -> bool {
    (got - want).abs() <= 1e-12 * collar_volume()
}

/// `c ∪ p` and `p ∪ c`, each with its declarations in its own operand
/// order: a body, its volume the closed form, one shell, tier 3 and the
/// pseudomanifold census clean.
fn unions_both_ways(c: &Body<f64>, p: &Body<f64>, h: f64, tag: &str) {
    let tol = Tol::witness();
    for (order, a, b) in [("collar ∪ shaft", c, p), ("shaft ∪ collar", p, c)] {
        let tag = format!("{tag}, {order}");
        let decls = wall_decls(a, b);
        let bb = match topo::union_with(a, b, &decls, tol) {
            Ok(BooleanResult::Body(bb)) => bb,
            other => panic!("{tag}: the mate does not union: {:?}", other.err()),
        };
        let (got, want) = (volume(&bb.body), collar_volume() + shaft_volume(h));
        assert!(agrees(got, want), "{tag}: union volume {got} vs {want}");
        assert_eq!(bb.body.shells().count(), 1, "{tag}: one shell");
        assert_eq!(
            topo::validate_geometric(&bb.body, tol),
            Ok(()),
            "{tag}: tier 3"
        );
        assert_eq!(
            topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol),
            Ok(()),
            "{tag}: the census"
        );
    }
}

/// **The union** at every span and pose, in both operand orders.
fn unions_at(collar: &Body<f64>, deg: f64, what: &str) {
    for (pose_name, pose) in crate::common::poses::poses() {
        let c = placed(collar, &pose);
        for (span, y0, h) in SPANS {
            let p = placed(&shaft(deg, y0, h), &pose);
            let tag = format!("{what}, azimuth {deg}, {span}, pose {pose_name}");
            unions_both_ways(&c, &p, h, &tag);
        }
    }
}

#[test]
fn a_shaft_on_the_bores_seam_unions() {
    unions_at(&collar(), 0.0, "one-face bore");
}

#[test]
fn a_shaft_off_the_bores_seam_unions() {
    unions_at(&collar(), 60.0, "one-face bore");
}

#[test]
fn a_shaft_a_quarter_turn_off_the_bores_seam_unions() {
    unions_at(&collar(), 90.0, "one-face bore");
}

/// The collar with its bore split into two full-turn faces by the
/// circle at `y = 1.5`: the circle is an edge between two faces of ONE
/// carrier, so no other face of the collar meets it.
fn split_collar() -> Body<f64> {
    let tol = Tol::witness();
    let mut c = collar();
    let bore = crate::mate2_common::walls_at(&c, BORE)[0];
    let skey = c.get_face(bore).unwrap().surface;
    let carrier = |c: &Body<f64>, e: &topo::Edge| {
        c.get_curve_geom(e.curve)
            .and_then(topo::null::CurveGeom::certified)
            .map(|g| g.carrier().clone())
    };
    // The bore's self-mated seam ruling, split at the circle's height.
    let (seam, origin, dir) = c
        .edges()
        .find_map(|(k, e)| match carrier(&c, e) {
            Some(geom::Curve3::Line { origin, dir })
                if c.face_of_half_edge(e.he_plus) == Some(bore)
                    && c.face_of_half_edge(e.he_minus) == Some(bore) =>
            {
                Some((k, origin, dir))
            }
            _ => None,
        })
        .expect("the bore's seam ruling");
    let w = c
        .split_edge(seam, (1.5 - origin.y) / dir.y, tol)
        .unwrap()
        .vertex;
    // The circle: the lower rim's carrier, lifted to `y = 1.5`.
    let rim = c
        .edges()
        .find_map(|(_, e)| match carrier(&c, e) {
            Some(geom::Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            }) if (radius - BORE).abs() < 1e-12 && (center.y - COLLAR.0).abs() < 1e-12 => {
                Some(geom::Curve3::Circle {
                    center: Point3::new(center.x, 1.5, center.z),
                    axis,
                    radius,
                    u_ref,
                })
            }
            _ => None,
        })
        .expect("the bore's lower rim");
    let pw = *c.get_point(c.get_vertex(w).unwrap().point).unwrap();
    let t0 = rim.param_near(pw, 0.0).unwrap();
    let spec = geom_brep::EdgeCurveSpec::arc_of_circle(rim, t0, t0 + core::f64::consts::TAU)
        .unwrap()
        .at_rest_in_chart(skey, false);
    // `w`'s two visits in the bore's loop, one per side of the seam.
    let visits: Vec<_> = c
        .half_edges()
        .filter(|&(k, h)| h.start == w && c.face_of_half_edge(k) == Some(bore))
        .map(|(k, _)| k)
        .collect();
    let [down, up] = visits[..] else {
        panic!("the seam vertex is visited twice: {visits:?}");
    };
    c.mef(
        topo::MefSite::Chords { he1: up, he2: down },
        spec,
        topo::FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    assert_eq!(
        topo::validate_geometric(&c, tol),
        Ok(()),
        "the split collar"
    );
    assert_eq!(
        crate::mate2_common::walls_at(&c, BORE).len(),
        2,
        "two bore faces"
    );
    c
}

/// **Only the crossing layer sees the shaft's rulings cross the bore's
/// split circle.** The circle's two faces are both on the shared
/// carrier, so no transverse face meets the rulings there — the
/// one-face bore's rims have its flat caps, which do. At the bore's
/// seam azimuth every shaft span unions in both orders and at every
/// pose. Off the seam, the circle's own vertex sits inside a shaft wall
/// third with no counterpart for the zip
/// (`work/zip/a-vertex-of-one-solid-inside-the-rest-contact-has-no-twin.md`).
#[test]
fn a_bore_split_on_its_own_carrier_unions_at_the_seam_azimuth() {
    unions_at(&split_collar(), 0.0, "split bore");
}

/// **The other three ops refuse typed, never answer wrong.** The
/// closed forms are `∩` empty and each difference its minuend whole;
/// what the kernel answers today, at every azimuth, span and pose, is
/// `FallbackExtentUnsupported`
/// (`work/reach/declared-rest-mate-intersect-and-differences-refuse-at-the-fallback-extent.md`).
#[test]
fn intersect_and_differences_refuse_at_the_fallback_extent() {
    let tol = Tol::witness();
    for (pose_name, pose) in crate::common::poses::poses() {
        let c = placed(&collar(), &pose);
        for deg in [0.0, 60.0, 90.0] {
            for (span, y0, h) in SPANS {
                let p = placed(&shaft(deg, y0, h), &pose);
                let tag = format!("azimuth {deg}, {span}, pose {pose_name}");
                let ab = wall_decls(&c, &p);
                let ba = wall_decls(&p, &c);
                for (op, a, b, decls) in [
                    (BooleanOp::Intersect, &c, &p, &ab),
                    (BooleanOp::Subtract, &c, &p, &ab),
                    (BooleanOp::Subtract, &p, &c, &ba),
                ] {
                    let out =
                        topo::boolean_op_with(op, a, b, decls, topo::SweepStrategy::Realized, tol);
                    assert!(
                        matches!(
                            out,
                            Err(topo::BooleanError::FallbackExtentUnsupported { .. })
                        ),
                        "{tag}: {op:?}: {:?}",
                        out.as_ref().err()
                    );
                }
            }
        }
    }
}
