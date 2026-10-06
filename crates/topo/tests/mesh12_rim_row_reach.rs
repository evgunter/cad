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
//! **The pcurve re-mint does not close the window.** It decides each
//! junction's deck element and not its chart-v jump (a joint's 3-D
//! coincidence follows from the rows' envelopes and the endpoint
//! pinning), so it mints every gap the certifying doors construct, and
//! the import door, which re-mints through it, no longer refuses
//! `R·Δv ≥ ε` there. Its reach through import is unmeasured since
//! (`work/tess/rim-continuation-import-reach-reopened-by-the-deck-element-walk.md`).
//!
//! **Through the Euler doors the shape is a rim-only cap**: a sphere
//! face whose one loop is two rim arcs and no meridian. With the gap
//! inside the band, the shape door and the flux lane answer on the same
//! undecidable gap (the shape door on `props_rim_side`, the flux lane
//! on `props_rim_only_extent`); at `Δv = 0` the door admits the face
//! and the flux lane measures it — the two caps sum to the sphere's
//! closed forms. `mesh::tessellate` refuses it typed either way: in the
//! band on the shape door's escalation (`UnsupportedCurvedShape`), and
//! at `Δv = 0` as `MeridianFreeCurvedFace`, the walk's own refusal of a
//! loop with no meridian. So the finding these rows pin is real and is
//! consumed by a measure at `Δv = 0` and by no mesh.
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
    let a = Point3::new(RS * vm.cos(), 0.0, RS * vm.sin());
    let b = Point3::new(-RS * vm.cos(), 0.0, RS * vm.sin());
    let rim = |v: f64| Curve3::Circle {
        center: Point3::new(0.0, 0.0, RS * v.sin()),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: RS * v.cos(),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a, true).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: Surface::Sphere {
                center: Point3::new(0.0, 0.0, 0.0),
                radius: RS,
                axis: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            sense: true,
        },
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

/// **The re-mint admits the gaps the examination reports — the record,
/// pinned on one body with no file.** `topo::mint_pcurves` is the gate
/// `import_step` re-mints through, and it no longer decides the
/// junction's chart-v jump: a joint's 3-D coincidence follows from the
/// two rows' envelopes and the endpoint pinning, and the walk decides
/// only its deck element. So every gap the certifying doors construct
/// (`R·Δv` inside the endpoint band) mints, while the examination
/// reports from `R·Δv = ε`; the intersection an import fixture would
/// need is no longer empty at this ε.
#[test]
fn the_remint_admits_the_gaps_the_examination_reports() {
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
    for f in [1.5, 1.9] {
        assert!(
            mint_ok(f) && reports(f),
            "inside the window at {f}ε: minted, reported"
        );
    }
    // Past the joint bound nothing reaches the walk: the door that
    // builds the cap refuses a 100ε gap (its endpoint pin), and a
    // minted rim row moved 100ε along the meridian does not certify, so
    // no row can carry the gap to a joint.
    assert!(
        two_level_rim_cap(100.0 * eps / RS).is_err(),
        "a 100ε gap is refused where the cap is built"
    );
    let mut body = two_level_rim_cap(0.5 * eps / RS).unwrap();
    topo::mint_pcurves(&mut body, tol).unwrap();
    let band = Band::linear(tol).unwrap();
    let moved = body
        .half_edges()
        .find_map(|(he, h)| {
            let row = body.pcurve(he)?;
            let geom_brep::Pcurve::Harmonic { p0, pa, pb, pl } = row.pcurve() else {
                return None;
            };
            let edge = body.get_edge(h.edge)?;
            let Some(topo::CurveGeom::Certified(curve)) = body.get_curve_geom(edge.curve) else {
                return None;
            };
            let face = body.face_of_half_edge(he)?;
            let sphere = body.get_surface(body.get_face(face)?.surface)?;
            let shifted = geom_brep::Pcurve::Harmonic {
                p0: geom_core::Point2::new(p0.x, p0.y + 100.0 * eps / RS),
                pa: *pa,
                pb: *pb,
                pl: *pl,
            };
            let (t0, t1) = row.params();
            Some(geom_brep::PcurveCache::certify(
                shifted,
                t0,
                t1,
                curve.carrier(),
                sphere,
                band,
            ))
        })
        .expect("the cap stores a harmonic rim row");
    assert!(
        moved.is_err(),
        "a rim row 100ε off its carrier does not certify"
    );
}

/// **Nothing that meshes or measures consumes the discarded
/// coordinate**, and the branch door, the shape door and the flux lane
/// each answer the rim-only cap on their own terms. The BRANCH door
/// admits it either way — no span contains a pole, which is a question
/// about arcs and not about the gap — while the other two turn on the
/// GAP, which is the coordinate this unit's row is about:
///
/// * **`Δv = 0`** — the two arcs state ONE rim circle, so the body is a
///   sphere split by one rim into two caps, and each face MEASURES.
///   Its levels hold one latitude and carry no extent, so the missing
///   extreme is the pole the rims' shared traversal points at
///   (`props_rim_interior_side`'s σ; issue 1250, PROPS
///   sphere-pole-side). The two caps sum to `4πR³/3`, which is what
///   `topo/tests/props_sphere_cap_door.rs` weighs directly. The shape
///   door admits it: with `lo == hi` no rim sits at an extreme rather
///   than the other, so its rim-side companion has nothing to compare
///   and extent is not a shape question.
/// * **`R·Δv` inside the ambiguity band** — the levels carry an extent
///   that is neither definitely zero nor definitely positive, so
///   whether this is a cap whose pole should be folded in or a zone of
///   sub-band height is exactly what cannot be decided. **Both** the
///   shape door and the flux lane say so, one predicate apart: the
///   door's rim-side unanimity asks which extreme each rim sits at and
///   escalates `props_rim_side` typed, and the flux lane escalates
///   `props_rim_only_extent` typed before any pole is pushed. One
///   undecidable gap read at one lever under three names — the third
///   being `props_face_extent`, which would have read it one step
///   later still.
///
/// What `mesh::tessellate` answers for the same two bodies is the next
/// row.
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
            assert_eq!(require_one_chart_branch(surface, &outer, band), Ok(()));
            let door = require_iso_rectangle(surface, &outer, band);
            let flux = curved_face(surface, &outer, face.sense, band);
            if in_band {
                assert!(
                    matches!(
                        &door,
                        Err(PropsError::Escalated { cause, .. })
                            if cause.predicate == Some("props_rim_side")
                    ),
                    "the gap is undecidable at the door too: {door:?}"
                );
                assert!(
                    matches!(
                        &flux,
                        Err(PropsError::Escalated { cause, .. })
                            if cause.predicate == Some("props_rim_only_extent")
                    ),
                    "{flux:?}"
                );
            } else {
                assert_eq!(door, Ok(()));
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

/// **The mesh lane refuses the rim-only cap typed, at zero gap and in
/// the band, by two different doors.** Both faces of the body are
/// rim-only sphere caps, so `tessellate` answers for the first in arena
/// order.
///
/// * **`Δv = 0`** — both doors in front of the walk admit the face, and
///   the walk refuses it on its traversal kinds:
///   `MeridianFreeCurvedFace`.
/// * **`R·Δv = 1.5ε`** — the shape door escalates `props_rim_side`
///   (the row above), and that escalation is what `tessellate` answers,
///   as `UnsupportedCurvedShape`; the walk is never reached.
#[test]
fn the_mesh_lane_refuses_the_rim_only_cap_typed_at_zero_gap_and_in_the_band() {
    let tol = Tol::witness();
    let first_face = |body: &Body<f64>| {
        let (first, _) = body.faces().next().expect("the body has two faces");
        first
    };

    let one_rim = two_level_rim_cap(0.0).unwrap();
    assert_eq!(
        mesh::tessellate(&one_rim, 1e-4, tol).map(|_| ()),
        Err(mesh::TessellateError::MeridianFreeCurvedFace {
            face: first_face(&one_rim),
            surface: geom::SurfaceKind::Sphere,
        }),
    );

    let in_band = two_level_rim_cap(1.5 * tol.eps() / RS).unwrap();
    let answered = mesh::tessellate(&in_band, 1e-4, tol).map(|_| ());
    assert!(
        matches!(
            &answered,
            Err(mesh::TessellateError::UnsupportedCurvedShape {
                face,
                source: PropsError::Escalated { cause, .. },
            }) if *face == first_face(&in_band) && cause.predicate == Some("props_rim_side")
        ),
        "{answered:?}"
    );
}
