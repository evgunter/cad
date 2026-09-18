//! **Where the rim-continuation condition can be reached from, measured**
//! (issue 1588). `CoherenceCondition::RimContinuation` had one witness:
//! a synthetic second circle OFF the sphere. These rows pin the one
//! door that reaches it with on-surface data, and the reason the
//! import door — where such data actually arrives — never does.
//!
//! The shape is a sphere whose one rim row is stated as TWO arcs at
//! latitudes `v` and `v + Δv`, both circles exactly on the sphere, the
//! two junction vertices at the mean latitude. Certification pins each
//! junction to each carrier within ε, so the row constructs while
//! `R·Δv/2` is inside the endpoint band, and the condition measures
//! the gap `R·Δv`: the window `ε ≤ R·Δv ≤ 2ε` is where a certifying
//! door hands the examination a finding. Both edges are decided by
//! rounding at the band's own boundary — at `R·Δv = 2ε` each junction
//! sits exactly `ε` off its carriers, which the endpoint pin escalates
//! at the default ε and admits at `ε = 1e-6` (residual
//! `9.999999995839272e-7`) — so the rows pin the interior of the
//! window and the escalation past it, never the edges. MESH-8's argument that a rim through two points is unique
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
use geom_brep::certify::{CertCheck, CertifyError};
use geom_brep::props::{PropsError, curved_face, require_iso_rectangle, require_one_chart_branch};
use geom_core::Tol;
use geom_core::{Band, Point3, Vec3};
use topo::{Body, CoherenceCondition, EulerOpError, FaceSurface, MefSite, MevSite};

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
/// typed answer, which is the row's when the junctions leave the
/// endpoint band.
fn two_level_rim_cap(dv: f64) -> Result<Body<f64>, EulerOpError> {
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
    let e1 = body.mev(
        MevSite::Lone {
            r#loop: seed.r#loop,
        },
        b,
        EdgeCurveSpec::arc_of_circle(rim(V1), 0.0, core::f64::consts::PI).unwrap(),
        tol,
    )?;
    body.mef(
        MefSite::Chords {
            he1: e1.he_minus,
            he2: e1.he_plus,
        },
        EdgeCurveSpec::arc_of_circle(rim(V1 + dv), core::f64::consts::PI, core::f64::consts::TAU)
            .unwrap(),
        FaceSurface::Inherit,
        tol,
    )?;
    Ok(body)
}

/// **The certifying doors hand the examination a rim-continuation
/// finding**: at `R·Δv = 1.5ε` and `1.9ε` the body constructs and the
/// report carries one `RimContinuation` per face, `gap = Δv`,
/// `lever = R`, `metres = R·Δv`. Below the band (`0.5ε`) the same
/// construction is quiet; past it (`3ε`, each junction `1.5ε` off its
/// carriers, inside the endpoint pin's ambiguity band) the door
/// ESCALATES at `EndpointStart` before any body exists — a typed
/// `Escalated`, not a refusal. The `2ε` edge is rounding-decided
/// (module docs) and is not pinned.
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
    let escalated = two_level_rim_cap(3.0 * eps / RS);
    assert!(
        matches!(
            &escalated,
            Err(EulerOpError::Certification {
                error: CertifyError::Escalated {
                    check: CertCheck::EndpointStart,
                    ..
                }
            })
        ),
        "a junction 1.5ε off both carriers is the endpoint pin's to escalate: {escalated:?}"
    );
}

/// **The re-mint admits no gap the examination reports — the record,
/// pinned on one body with no file.** `topo::mint_pcurves` is the gate
/// `import_step` refuses at (`pcurve_loop_continuity`, the junction's
/// chart-v jump at the linear band), and the examination reports the
/// same gap at the same band. Bisected on `R·Δv` over the same
/// construction: the largest gap the re-mint admits and the smallest
/// the examination reports are adjacent at `ε`, the examination is
/// quiet at the former and the re-mint refuses the latter, so the
/// intersection an import fixture would need is empty at this ε.
#[test]
fn the_remint_admits_no_gap_the_examination_reports() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let mint_ok = |f: f64| {
        two_level_rim_cap(f * eps / RS)
            .ok()
            .is_some_and(|mut body| topo::mint_pcurves(&mut body, tol).is_ok())
    };
    let reports = |f: f64| {
        two_level_rim_cap(f * eps / RS).ok().is_some_and(|body| {
            topo::examine_chart_coherence(&body, tol)
                .findings
                .iter()
                .any(|c| matches!(c.condition, CoherenceCondition::RimContinuation { .. }))
        })
    };
    assert!(
        mint_ok(0.5) && !reports(0.5),
        "below the band: minted, quiet"
    );
    assert!(
        !mint_ok(1.9) && reports(1.9),
        "inside the window: refused, reported"
    );
    let (mut lo, mut hi) = (0.5_f64, 1.9_f64);
    for _ in 0..80 {
        let m = 0.5 * (lo + hi);
        if mint_ok(m) { lo = m } else { hi = m }
    }
    let admits_up_to = lo;
    let (mut quiet, mut loud) = (0.5_f64, 1.9_f64);
    for _ in 0..80 {
        let m = 0.5 * (quiet + loud);
        if reports(m) { loud = m } else { quiet = m }
    }
    let reports_from = loud;
    assert!(
        admits_up_to < reports_from,
        "the re-mint admits up to {admits_up_to:.17}ε, the examination reports from {reports_from:.17}ε"
    );
    assert!(
        !reports(admits_up_to),
        "quiet at the last admitted gap {admits_up_to:.17}ε"
    );
    assert!(
        !mint_ok(reports_from),
        "refused at the first reported gap {reports_from:.17}ε"
    );
    for (what, f) in [("admission", admits_up_to), ("report", reports_from)] {
        assert!(
            (f - 1.0).abs() < 1e-6,
            "the {what} threshold sits at ε: {f:.17}"
        );
    }
}

/// **Nothing that meshes or measures consumes the discarded
/// coordinate**, and the two doors and the flux lane each answer the
/// rim-only cap on their own terms. The shape door and the branch door
/// ADMIT it — a rim row is at its own extreme by definition and
/// contains no pole in any span — and the flux lane's answer turns on
/// the GAP, which is the coordinate this unit's row is about:
///
/// * **`Δv = 0`** — the two arcs state ONE rim circle, so the body is a
///   sphere split by one rim into two caps, and each face MEASURES.
///   Its levels hold one latitude and carry no extent, so the missing
///   extreme is the pole the rims' shared traversal points at
///   (`props_rim_interior_side`'s σ; issue 1250, PROPS
///   sphere-pole-side). The two caps sum to `4πR³/3`, which is what
///   `topo/tests/props_sphere_cap_door.rs` weighs directly.
/// * **`R·Δv` inside the ambiguity band** — the levels carry an extent
///   that is neither definitely zero nor definitely positive, so
///   whether this is a cap whose pole should be folded in or a zone of
///   sub-band height is exactly what cannot be decided:
///   `props_rim_only_extent` escalates, typed, before any pole is
///   pushed. That escalation is the row, and it is the same margin at
///   the same lever `props_face_extent` would have read one step
///   later.
///
/// What the walk does with the admitted face is issue 1615's.
#[test]
fn the_shape_door_admits_the_rim_only_cap_and_the_flux_lane_reads_the_gap() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let mut caps = Vec::new();
    for (f, in_band) in [(1.5, true), (0.0, false)] {
        let body = two_level_rim_cap(f * tol.eps() / RS).unwrap();
        for (_, face) in body.faces() {
            let surface = body.get_surface(face.surface).unwrap();
            let (outer, _) = topo::props::loop_edges(&body, face.outer).unwrap();
            assert_eq!(outer.len(), 2, "a rim-only loop: two arcs, no meridian");
            assert_eq!(require_iso_rectangle(surface, &outer, band), Ok(()));
            assert_eq!(require_one_chart_branch(surface, &outer, band), Ok(()));
            let flux = curved_face(surface, &outer, face.sense, band);
            if in_band {
                assert!(
                    matches!(
                        &flux,
                        Err(PropsError::Escalated { cause })
                            if cause.predicate == Some("props_rim_only_extent")
                    ),
                    "{flux:?}"
                );
            } else {
                caps.push(flux.unwrap_or_else(|e| panic!("a rim-only cap measures: {e:?}")));
            }
        }
    }
    // The two caps of one sphere: their areas sum to `4πR²` and their
    // fluxes to `3V = 4πR³`, which is the closed form the gap was
    // hiding.
    let area: f64 = caps.iter().map(|c| c.area).sum();
    let flux: f64 = caps.iter().map(|c| c.flux).sum();
    let tau = core::f64::consts::TAU;
    assert_eq!(caps.len(), 2, "one sphere, two rim-only caps");
    assert!(
        (area - 2.0 * tau * RS * RS).abs() < 1e-12 * area,
        "the two caps' areas sum to 4πR²: {area}"
    );
    assert!(
        (flux - 2.0 * tau * RS.powi(3)).abs() < 1e-12 * flux,
        "and their fluxes to 3V = 4πR³: {flux}"
    );
}
