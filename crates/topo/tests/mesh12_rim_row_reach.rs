//! **Where the rim-continuation condition can be reached from, measured**
//! (issue 1588). `CoherenceCondition::RimContinuation` had one witness:
//! a synthetic second circle OFF the sphere. These rows pin the one
//! door that reaches it with on-surface data, and the reason the
//! import door — where such data actually arrives — never does.
//!
//! The shape is a sphere whose one rim row is stated as TWO arcs at
//! latitudes `v` and `v + Δv`, both circles exactly on the sphere, the
//! two junction vertices at the mean latitude. Certification pins each
//! junction to each carrier within ε, so the row constructs whenever
//! `R·Δv/2 ≤ ε`, and the condition measures the gap `R·Δv`: the window
//! `ε ≤ R·Δv ≤ 2ε` is where a certifying door hands the examination a
//! finding. MESH-8's argument that a rim through two points is unique
//! holds for exact incidence; the endpoint band is what opens this
//! window.
//!
//! **Through the import door the window is empty at every ε row**
//! (measured at 1e-6, 1e-9, 1e-12 on the same shape as a STEP solid,
//! as a two-cap sphere and as issue 723's half-cap): `import_step`
//! refuses every `R·Δv ≥ ε` at its pcurve re-mint —
//! `pcurve_loop_continuity`, which decides the junction's chart-v jump
//! at the same band this condition reports it at, escalating in the
//! ambiguity band and refusing above it — and imports the `R·Δv < ε`
//! shape, on which the condition is quiet by the same band. Props'
//! `props_rim_level` never decides the question: the re-mint sits in
//! front of it. No fixture can be committed, and the condition's
//! import-door reach is nil by construction rather than by absence of
//! a file.
//!
//! **Through the Euler doors the shape is a rim-only cap**, which the
//! shape door admits, the flux lane refuses (`props_face_extent` in
//! the band), and `mesh::tessellate` does not mesh: the walk emits no
//! triangles for a loop with no meridian and the issue-897 cross-face
//! census panics (orchestrator-filed issue 1615, on every ε row and at
//! `Δv = 0` too — the panic is the rim-only loop's, not the gap's). So
//! the finding these rows pin is real and is consumed by nothing that
//! meshes or measures; the body below is the fixture issue 1615 can
//! lift.
//!
//! Offsets are derived from the run's own ε: this file is on CI's
//! `eps ∈ {default, 1e-6, 1e-12}` matrix.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_brep::props::{PropsError, curved_face, require_iso_rectangle, require_one_chart_branch};
use geom_core::Tol;
use geom_core::{Band, Point3, Vec3};
use topo::{Body, CoherenceCondition, FaceSurface, MefSite, MevSite};

fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
fn v3(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

/// The sphere under every row: R = 10 mm about +Z at the origin.
const RS: f64 = 0.010;
/// The lower rim's latitude.
const V1: f64 = 0.5;

/// The cap above a rim row stated as two on-sphere arcs at latitudes
/// `V1` and `V1 + dv`, and its complement, through the Euler doors;
/// the two junctions sit at the mean latitude. `Err` is the door's own
/// certification refusal, which is the row's answer when the junctions
/// leave the endpoint band.
fn two_level_rim_cap(dv: f64) -> Result<Body<f64>, String> {
    let tol = Tol::witness();
    let vm = V1 + 0.5 * dv;
    let a = p3(RS * vm.cos(), 0.0, RS * vm.sin());
    let b = p3(-RS * vm.cos(), 0.0, RS * vm.sin());
    let rim = |v: f64| Curve3::Circle {
        center: p3(0.0, 0.0, RS * v.sin()),
        axis: v3(0.0, 0.0, 1.0),
        radius: RS * v.cos(),
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New(Surface::Sphere {
            center: p3(0.0, 0.0, 0.0),
            radius: RS,
            axis: v3(0.0, 0.0, 1.0),
            u_ref: v3(1.0, 0.0, 0.0),
        }),
    )
    .unwrap();
    let e1 = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            EdgeCurveSpec::arc_of_circle(rim(V1), 0.0, core::f64::consts::PI).unwrap(),
            tol,
        )
        .map_err(|e| format!("{e:?}"))?;
    body.mef(
        MefSite::Chords {
            he1: e1.he_minus,
            he2: e1.he_plus,
        },
        EdgeCurveSpec::arc_of_circle(rim(V1 + dv), core::f64::consts::PI, core::f64::consts::TAU)
            .unwrap(),
        FaceSurface::Inherit,
        tol,
    )
    .map_err(|e| format!("{e:?}"))?;
    Ok(body)
}

/// **The certifying doors hand the examination a rim-continuation
/// finding**: at `R·Δv = 1.5ε` and `1.9ε` the body constructs and the
/// report carries one `RimContinuation` per face, `gap = Δv`,
/// `lever = R`, `metres = R·Δv`. Below the band (`0.5ε`) the same
/// construction is quiet; past the endpoint band (`3ε`, each junction
/// `1.5ε` off its carriers) the door refuses before any body exists.
#[test]
fn a_two_level_rim_row_from_the_certifying_doors_reports_its_gap() {
    let tol = Tol::witness();
    let eps = tol.eps();
    for f in [1.5, 1.9] {
        let body = two_level_rim_cap(f * eps / RS).unwrap();
        let report = topo::examine_chart_coherence(&body, tol);
        assert!(report.unexamined.is_empty(), "{:?}", report.unexamined);
        assert_eq!(
            report.findings.len(),
            2,
            "one per face at {f} ε: {:?}",
            report.findings
        );
        for c in &report.findings {
            assert!(
                matches!(c.condition, CoherenceCondition::RimContinuation { .. }),
                "{c:?}"
            );
            assert!((c.lever - RS).abs() < 1e-15, "lever {}", c.lever);
            let want = f * eps;
            assert!(
                (c.metres - want).abs() < want * 1e-6,
                "expected {want:e} m, got {:e} (gap {:e} rad)",
                c.metres,
                c.gap
            );
            assert_eq!(c.eps, eps);
        }
    }
    let quiet = topo::examine_chart_coherence(&two_level_rim_cap(0.5 * eps / RS).unwrap(), tol);
    assert!(quiet.findings.is_empty(), "{:?}", quiet.findings);
    let refused = two_level_rim_cap(3.0 * eps / RS);
    assert!(
        matches!(&refused, Err(e) if e.contains("carrier_endpoint")),
        "a junction 1.5ε off both carriers is the endpoint pin's to refuse: {refused:?}"
    );
}

/// **Nothing that meshes or measures consumes the discarded
/// coordinate.** The shape door and the branch door ADMIT the rim-only
/// cap — a rim row is at its own extreme by definition and contains no
/// pole in any span — and the flux lane refuses it: the face's
/// latitude extent is the very gap, `R·Δv` inside the ambiguity band,
/// so `props_face_extent` escalates (and is coincident with zero at
/// `Δv = 0`). What the walk does with the admitted face is issue
/// 1615's.
#[test]
fn the_shape_door_admits_the_rim_only_cap_and_the_flux_lane_refuses_it() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    for (f, degenerate) in [(1.5, false), (0.0, true)] {
        let body = two_level_rim_cap(f * tol.eps() / RS).unwrap();
        for (_, face) in body.faces() {
            let surface = body.get_surface(face.surface).unwrap();
            let (outer, _) = topo::props::loop_edges(&body, face.outer).unwrap();
            assert_eq!(outer.len(), 2, "a rim-only loop: two arcs, no meridian");
            assert_eq!(require_iso_rectangle(surface, &outer, band), Ok(()));
            assert_eq!(require_one_chart_branch(surface, &outer, band), Ok(()));
            let flux = curved_face(surface, &outer, face.sense_sign(), band);
            if degenerate {
                assert!(matches!(flux, Err(PropsError::DegenerateFace)), "{flux:?}");
            } else {
                assert!(
                    matches!(
                        &flux,
                        Err(PropsError::Escalated { cause })
                            if cause.predicate == Some("props_face_extent")
                    ),
                    "{flux:?}"
                );
            }
        }
    }
}
