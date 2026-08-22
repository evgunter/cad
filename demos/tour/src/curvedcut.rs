//! The tilted cut — the tour's curved-surface stop, RENDERING since
//! M5 PR 11 (the milestone's demo moment).
//!
//! The body is M5 PR 5's acceptance shape (i): a cylinder sliced by a
//! tilted plane, whose section edges carry an exact `Curve3::Ellipse`
//! — semi-axes a = r/cos φ and b = r, described as the wall×plane
//! `Intersection`, with a rounding-scale certificate residual because
//! the carrier is zero-residual BY CONSTRUCTION (D4 ¶2), not fitted.
//!
//! **The PR 11 flip, executed.** This stop was STAGED behind
//! `pin_frontier` — three retire-on-closure panics asserting that
//! tier 3's volume row, exact mass properties, and tessellation all
//! refused typed at the named frontier. All three lanes landed:
//!
//! - tier 3 passes in full (check 7 consumes the certified quadrature
//!   bounds);
//! - `pncad::topo::mass_properties` returns a certified ENCLOSURE (midpoint ±
//!   pad) whose bracket contains the closed form πr²H/2 per half —
//!   asserted below;
//! - `pncad::mesh::tessellate` routes the conic-trimmed walls through the
//!   pcurve-driven trimmed lane and the halves are watertight.
//!
//! So the pins are RETIRED per their own instructions: `SceneBody::
//! staged` is gone from [`stops`], the stop runs the standard ladder
//! (props ribbon, mesh, STL, scene manifest, montage panel), and the
//! module is generic over [`Scalar`] so the K-probe sweep rebuilds it
//! at the recording scalar (`crate::probe`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::geom_core::{Point2, Point3, Vec3};
use pncad::profile::{Profile, SketchPlane, ValidatedProfile};
use pncad::sweep::{Extrusion, extrude};
use pncad::topo::splitting::{SplitPart, SplitPlane, split};
use pncad::topo::{Body, Curve3, EdgeGeometry};

use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};
use pncad::geom_core::Tol;

/// The cylinder's radius (m).
const R: f64 = 1.0;
/// The cylinder's height (m).
const H: f64 = 2.5;
/// The section plane's tilt from the cylinder's cross-section (rad).
const PHI: f64 = 0.3;

/// The disc profile: two half-circle arcs (bulge 1) of radius [`R`] —
/// extrudes to a cylinder whose two wall faces share ONE cylinder
/// surface.
fn disc<S: Scalar>(tol: Tol) -> ValidatedProfile<S> {
    // Algebra-authored (LIB-G1): the one-step circle program form.
    let p2 = |x: f64, y: f64| Point2::new(S::from_f64(x), S::from_f64(y));
    let lp = pncad::profile::circle(p2(0.0, 0.0), S::from_f64(R), tol)
        .expect("disc radius is positive")
        .into();
    Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol)
        .expect("the disc profile validates")
}

/// The cut: cylinder split by the tilted plane through mid-height.
/// Returns (above, below) — both sides carry material.
pub fn build<S: Scalar>(tol: Tol) -> (Body<S>, Body<S>) {
    let cylinder = extrude(&disc::<S>(tol), Extrusion::Distance(S::from_f64(H)), tol)
        .expect("extrude cylinder")
        .body;
    let plane = SplitPlane {
        origin: Point3::new(S::from_f64(0.0), S::from_f64(0.0), S::from_f64(H / 2.0)),
        normal: Vec3::new(
            S::from_f64(PHI.sin()),
            S::from_f64(0.0),
            S::from_f64(PHI.cos()),
        ),
    };
    let result = split(&cylinder, &plane, tol).expect("the tilted cut splits the cylinder");
    let (SplitPart::Body(above), SplitPart::Body(below)) = (&result.above, &result.below) else {
        panic!("the section plane crosses the wall: both sides must be bodies");
    };
    (above.clone(), below.clone())
}

/// Narration for the minted section: how many exact ellipse arcs the
/// half carries, their semi-axes against the closed form (a = r/cos φ,
/// b = r), the worst certificate residual, and the certified volume
/// bracket against πr²H/2.
fn section_narration<S: Scalar>(label: &str, body: &Body<S>, tol: Tol) -> String {
    let mut arcs = 0usize;
    let mut worst = 0.0f64;
    for (_, edge) in body.edges() {
        let Some(curve) = body.get_curve_geom(edge.curve).and_then(|g| g.certified()) else {
            continue;
        };
        let Curve3::Ellipse { major, minor, .. } = *curve.carrier() else {
            continue;
        };
        assert!(
            (major.f() - R / PHI.cos()).abs() < 1e-12 && (minor.f() - R).abs() < 1e-12,
            "{label}: section semi-axes must be the closed form \
             (a = r/cos phi, b = r), got a = {}, b = {}",
            major.f(),
            minor.f()
        );
        assert!(
            matches!(curve.description(), EdgeGeometry::Intersection { .. }),
            "{label}: a section edge must be described as the wall x plane intersection"
        );
        worst = worst.max(curve.certificate().max_residual.f());
        arcs += 1;
    }
    assert!(arcs > 0, "{label}: the tilted cut must mint ellipse arcs");

    // The PR 11 certified enclosure, asserted against the closed form:
    // the tilted plane passes through the axis midpoint, so EACH half
    // encloses exactly πr²H/2.
    let m = pncad::topo::mass_properties(body, tol).expect("the PR 11 quadrature lane computes");
    let half_exact = core::f64::consts::PI * R * R * H / 2.0;
    let (v, pad) = (m.volume.f(), m.volume_pad);
    assert!(
        v - pad <= half_exact && half_exact <= v + pad,
        "{label}: certified bracket [{}, {}] must contain πr²H/2 = {half_exact}",
        v - pad,
        v + pad
    );
    format!(
        "{arcs} exact Ellipse arc(s): a = r/cos {PHI} = {:.6} m, b = r = {R} m, \
         described as the wall x plane intersection; worst certified residual \
         {worst:.2e} m; certified volume enclosure {v:.9} ± {pad:.2e} m^3 \
         BRACKETS the closed form pi*r^2*H/2 = {half_exact:.9} (the kernel's \
         first quadrature, interval-remainder certified)",
        R / PHI.cos()
    )
}

/// The rendering stop (montage panel; the PR 11 flip, one place).
pub fn stops(tol: Tol) -> Vec<Stop> {
    let (above, below) = build::<f64>(tol);
    let narration_above = section_narration("tiltedcut_above", &above, tol);
    let narration_below = section_narration("tiltedcut_below", &below, tol);
    vec![Stop {
        name: "tiltedcut",
        caption: "tilted cut (exact ellipse section)".to_string(),
        montage: true,
        story: "the tilted cut RENDERS (M5 PR 11): the section edges carry an EXACT \
                ellipse, the cut walls tessellate through the pcurve-driven trimmed \
                lane, and the volume is a certified quadrature enclosure",
        ops: "extrude(disc) -> topo::split(tilted plane); exact Curve3::Ellipse \
              section carriers (M5 PR 5 shape (i)); pcurve trim loops + certified \
              quadrature (M5 PR 11)",
        delta: 1e-2,
        note: Some(format!(
            "cutting a cylinder at an angle produces an ellipse — this kernel stores \
             an ellipse: exact semi-axes, zero residual by construction. What PR 11 \
             adds is the measuring and drawing of the curved cut faces: certified \
             volume/area enclosures (divergence theorem + the kernel's first \
             quadrature) and watertight tessellation of the trimmed walls.\n   \
             [tiltedcut_above] section: {narration_above}\n   \
             [tiltedcut_below] section: {narration_below}"
        )),
        view: View {
            elev: 18.0,
            azim: -60.0,
            up: 'z',
        },
        bodies: vec![
            SceneBody::plain("tiltedcut_above", [0.62, 0.44, 0.80], above),
            SceneBody::plain("tiltedcut_below", [0.40, 0.62, 0.80], below),
        ],
    }]
}
