//! The tilted cut — the tour's curved-surface stop, STAGED.
//!
//! The body is real and on main today (M5 PR 5 acceptance shape (i)):
//! a cylinder sliced by a tilted plane, whose section edges carry an
//! exact `Curve3::Ellipse` — semi-axes a = r/cos φ and b = r,
//! described as the wall×plane `Intersection`, with a certificate
//! whose residual is rounding-scale because the carrier is
//! zero-residual BY CONSTRUCTION (D4 ¶2), not fitted.
//!
//! **What is staged, and on what.** Everything downstream of the
//! section edge — the exact volume/area of a curved cut face, its
//! tessellation, therefore its STL and its render — is the curved
//! quadrature + trimmed-face lane that lands at **M5 PR 11**. Today
//! both refuse TYPED, naming that frontier in the error itself
//! (`props::NotIsoRectangle` through tier 3's volume row;
//! `TessellateError::UnsupportedCurve` whose note reads "the
//! trimmed-face lane lands at M5 PR 11"). So this stop asserts what
//! is TRUE today — the body builds, the ellipse is exact, tiers 1-2
//! pass, and the two frontier lanes refuse in exactly their named
//! classes — and [`pin_frontier`] panics loudly the day either one
//! starts working.
//!
//! **PR 11's flip, in one place.** When the curved lanes land,
//! `pin_frontier`'s retire-on-closure panics fire with instructions:
//! drop `SceneBody::staged` from [`stops`], and this stop joins the
//! standard ladder (props ribbon, mesh, STL, scene manifest, montage
//! panel) like every other. Nothing else in the tour changes. This is
//! the `crates/topo/tests/m5_pr7_split_meter.rs` pattern — pin the
//! honest refusal, name the PR that flips it — carried into the demo.
//!
//! Deliberately NOT staged behind a silent skip: a skipped assertion
//! proves nothing, and a demo that quietly draws nothing is how a
//! frontier stops being visible.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tolerance, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, extrude};
use topo::splitting::{SplitPart, SplitPlane, split};
use topo::{Body, Curve3, EdgeGeometry};

use crate::{SceneBody, Stop, View};

/// The cylinder's radius (m).
const R: f64 = 1.0;
/// The cylinder's height (m).
const H: f64 = 2.5;
/// The section plane's tilt from the cylinder's cross-section (rad).
const PHI: f64 = 0.3;

/// The disc profile: two half-circle arcs (bulge 1) of radius [`R`] —
/// extrudes to a cylinder whose two wall faces share ONE cylinder
/// surface.
fn disc() -> ValidatedProfile<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex {
            pos: Point2::new(-R, 0.0),
            bulge: 1.0,
        },
        ProfileVertex {
            pos: Point2::new(R, 0.0),
            bulge: 1.0,
        },
    ]);
    Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .expect("the disc profile validates")
}

/// The cut: cylinder split by the tilted plane through mid-height.
/// Returns (above, below) — both sides carry material.
pub fn build() -> (Body<f64>, Body<f64>) {
    let cylinder = extrude(&disc(), Extrusion::Distance(H))
        .expect("extrude cylinder")
        .body;
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.0, H / 2.0),
        normal: Vec3::new(PHI.sin(), 0.0, PHI.cos()),
    };
    let result = split(&cylinder, &plane).expect("the tilted cut splits the cylinder");
    let (SplitPart::Body(above), SplitPart::Body(below)) = (&result.above, &result.below) else {
        panic!("the section plane crosses the wall: both sides must be bodies");
    };
    (above.clone(), below.clone())
}

/// Narration for the minted section: how many exact ellipse arcs the
/// half carries, their semi-axes against the closed form (a = r/cos φ,
/// b = r), and the worst certificate residual across them.
fn section_narration(label: &str, body: &Body<f64>) -> String {
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
            (major - R / PHI.cos()).abs() < 1e-12 && (minor - R).abs() < 1e-12,
            "{label}: section semi-axes must be the closed form \
             (a = r/cos phi, b = r), got a = {major}, b = {minor}"
        );
        assert!(
            matches!(curve.description(), EdgeGeometry::Intersection { .. }),
            "{label}: a section edge must be described as the wall x plane intersection"
        );
        worst = worst.max(curve.certificate().max_residual);
        arcs += 1;
    }
    assert!(arcs > 0, "{label}: the tilted cut must mint ellipse arcs");
    format!(
        "{arcs} exact Ellipse arc(s): a = r/cos {PHI} = {:.6} m, b = r = {R} m, \
         described as the wall x plane intersection; worst certified residual \
         {worst:.2e} m (zero in R — the carrier is constructed, not fitted)",
        R / PHI.cos()
    )
}

/// The M5 PR 11 gate, in one place.
///
/// Asserts the two frontier lanes refuse in exactly their named
/// classes, and panics with flip instructions if either has landed.
/// `delta` is the chordal tolerance the stop will tessellate at once
/// the lane exists.
pub fn pin_frontier(label: &str, body: &Body<f64>, delta: f64) {
    let (v, e, f, r, s, genus) = crate::census(body);
    println!(
        "   [{label}] topology: {v} vertices, {e} edges, {f} faces, {r} rings, \
         {s} shell(s) -> genus {genus}; validation: tiers 1-2 (structural, \
         closed-solid census) — tier 3's volume row is staged below"
    );
    println!("   [{label}] section: {}", section_narration(label, body));

    // (1) The props lane. Tier 3 runs everything: certification and
    // dihedral rows must PASS (a failure there would be a real
    // regression, not a frontier), and the ONE residue must be the
    // volume row refusing typed on the cut wall pieces.
    match topo::validate_geometric(body) {
        Ok(()) => panic!(
            "{label}: tier 3 now PASSES — the M5 PR 11 curved-props lane has \
             landed. Retire this gate: drop `SceneBody::staged` in \
             curvedcut::stops() and let the standard ladder run."
        ),
        Err(errs) => {
            assert_eq!(
                errs.len(),
                1,
                "{label}: the staged residue is the volume row ALONE; \
                 something else is hiding behind it: {errs:?}"
            );
            let msg = format!("{:?}", errs[0]);
            assert!(
                msg.contains("VolumeUncomputable") && msg.contains("NotIsoRectangle"),
                "{label}: the staged tier-3 residue moved OFF the \
                 VolumeUncomputable/NotIsoRectangle class: {msg} — re-diagnose \
                 before re-pinning"
            );
        }
    }
    let props_err = topo::mass_properties(body).expect_err(
        "mass properties on a curved cut face are M5 PR 11's; a success here \
         means the lane landed — retire this gate (see above)",
    );
    println!(
        "   [{label}] exact mass properties: refused typed ({props_err:?}) — \
         the curved quadrature lane lands at M5 PR 11"
    );

    // (2) The render lane. The iso-rectangle UV walk cannot traverse a
    // conic cut boundary, and says so with the frontier named in the
    // error itself.
    match mesh::tessellate(body, delta) {
        Ok(_) => panic!(
            "{label}: tessellation now SUCCEEDS — the M5 PR 11 trimmed-face \
             lane has landed. Retire this gate: drop `SceneBody::staged` in \
             curvedcut::stops(), pick the panel's view, and re-render \
             (demos/render.sh) so the montage carries the cut."
        ),
        Err(e @ mesh::TessellateError::UnsupportedCurve { note, .. }) => {
            assert!(
                note.contains("PR 11"),
                "{label}: the refusal must keep naming the PR that lands the \
                 lane, got {note:?}"
            );
            println!(
                "   [{label}] tessellation (delta = {delta:.0e}): refused typed \
                 ({e:?}) — no STL, no STEP, no scene: this stop narrates until \
                 M5 PR 11 lands the trimmed-face lane, then renders"
            );
        }
        Err(other) => panic!(
            "{label}: tessellation refused OUTSIDE the staged frontier class: \
             {other:?} — re-diagnose before re-pinning"
        ),
    }
}

/// The staged stop. `montage: false` and the view are already chosen
/// so PR 11's flip is one field (`SceneBody::staged`) plus a render.
pub fn stops() -> Vec<Stop> {
    let (above, below) = build();
    vec![Stop {
        name: "tiltedcut",
        caption: "tilted cut — a cylinder sectioned on an exact ellipse".to_string(),
        montage: false,
        story: "STAGED (M5 PR 11): a cylinder cut by a tilted plane — the section \
                edges carry an EXACT ellipse, not a fitted spline, and both halves \
                are closed solids today",
        ops: "extrude(disc) -> topo::split(tilted plane); exact Curve3::Ellipse \
              section carriers (M5 PR 5 shape (i))",
        delta: 1e-2,
        note: Some(
            "cutting a cylinder at an angle produces an ellipse — so this kernel \
             stores an ellipse: exact semi-axes, zero residual by construction, \
             no fitted spline standing in for a shape we can write down. What is \
             not here yet is the measuring and drawing of a curved cut face: \
             volume, area and tessellation of these two halves refuse typed \
             today, and both refusals name M5 PR 11, the PR that lands them. \
             When it does, this stop renders."
                .to_string(),
        ),
        view: View {
            elev: 18.0,
            azim: -60.0,
            up: 'z',
        },
        bodies: vec![
            SceneBody::staged("tiltedcut_above", [0.62, 0.44, 0.80], above),
            SceneBody::staged("tiltedcut_below", [0.40, 0.62, 0.80], below),
        ],
    }]
}
