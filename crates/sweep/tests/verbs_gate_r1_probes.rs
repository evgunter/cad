//! Reviewer probes for PR #1001 (VERBS-GATE, head b2a8bad1) — an
//! independent consumer suite. Every row here is RED-able: each one
//! asserts an outcome the pair-scoped gate (or the boxes under it)
//! must produce, on bodies this suite authors itself.
//!
//! Rows 1–2 are the E2E the spec's acceptance only ran at
//! `boolean_reduce` depth: a body whose TORUS face is irrelevant to
//! the cut must union THROUGH THE FULL PIPELINE, and the result's
//! mass must be right; posed so the torus box genuinely overlaps, the
//! same body must refuse naming the pair.
//!
//! Row 3 documents what the full pipeline does when a torus-carrying
//! operand is admitted (correctly, per the ruling) but the operation
//! falls through to the containment fallback, whose `point_in_solid`
//! walks EVERY face of the other body regardless of box overlap. It
//! pinned a typed refusal by KIND there until issue 1011's torus half
//! landed the arm; what it pins now is the completion that refusal was
//! always standing in for, and the structural claim underneath both:
//! the gate admits, and containment must then answer for every face.
//!
//! Rows 4–6 pose a brick with one face relabelled to a cone or torus
//! at a TILTED axis. Such a relabel leaves the face's boundary on the
//! brick's lines, so the brick does not finish: the rows pin the
//! at-rest gate's refusal on the relabelled face, the one place it
//! stops.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{Profile, RawLoop, SketchPlane, test_support::bulge_loop};
use sweep::test_support::{brick, finished};
use topo::{Body, BooleanError};

fn vol(body: &Body<f64>) -> f64 {
    topo::mass_properties(body, Tol::witness()).unwrap().volume
}

/// The vase's bulge-arc data, shared by the fixture and the analytic
/// volume: bulge b = 0.6 on the vertical chord (0.5, 1) → (0.5, 1.5),
/// so sagitta s = b·c/2, arc radius R = ((c/2)² + s²)/(2s), centre
/// (0.5 + s − R, 1.25). b < 1 keeps the arc's end tangents off both
/// neighbouring segments (no undeclared tangency) and R < centre-x
/// keeps the revolved torus a ring torus (r < R).
const BULGE: f64 = 0.6;

fn bulge_arc() -> (f64, f64, f64) {
    let (c, half) = (0.5, 0.25);
    let s = BULGE * c / 2.0;
    let r_arc = (half * half + s * s) / (2.0 * s);
    let cx = 0.5 + s - r_arc;
    (cx, r_arc, half)
}

/// A "vase": a solid of revolution about the WORLD Y AXIS with, from
/// bottom to top: a hemispherical cap (arc centred ON the axis →
/// Sphere), a cylinder wall over y ∈ [0.5, 1.0], the BULGE arc over
/// y ∈ [1.0, 1.5] (centre off the axis → a genuine TORUS band), a
/// second cylinder wall over y ∈ [1.5, 2.0], and a top hemispherical
/// cap closing at (0, 2.5). No planar faces at all, so the operand is
/// maximal-faced by construction (a full revolve mints planar discs
/// as two same-key halves, which the boolean's F7 gate refuses and
/// the coplanar merge cannot re-fuse — MergedFaceRoleAmbiguous).
/// The cap-to-wall joints are exactly tangent and declared so.
fn vase() -> Body<f64> {
    vase_with_caps(true)
}

/// [`vase`] with a cap choice: `true` = hemispherical caps (Sphere
/// faces, exactly tangent to the walls, declared); `false` = 45-degree
/// conical caps (Cone faces, no tangencies). Both authorships carry
/// the same torus band and the same cylinder walls.
fn vase_with_caps(sphere: bool) -> Body<f64> {
    use sweep::{Revolution, RevolveAxis, revolve};
    let cap = if sphere {
        (core::f64::consts::PI / 8.0).tan() // quarter-turn arc
    } else {
        0.0 // straight generator: a cone with its apex on the axis
    };
    let mut lp = bulge_loop(vec![
        (Point2::new(0.0, 0.0), cap),   // bottom cap → (0.5, 0.5)
        (Point2::new(0.5, 0.5), 0.0),   // wall → (0.5, 1.0)
        (Point2::new(0.5, 1.0), BULGE), // torus arc → (0.5, 1.5)
        (Point2::new(0.5, 1.5), 0.0),   // wall → (0.5, 2.0)
        (Point2::new(0.5, 2.0), cap),   // top cap → (0, 2.5)
        (Point2::new(0.0, 2.5), 0.0),   // axis seam → start
    ]);
    if sphere {
        lp = lp.with_tangent_joints(vec![1, 4]);
    }
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    };
    revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

/// A DONUT: a full circle of radius 0.15 about (0.5, 1.25) in the
/// profile, revolved fully about the y axis — every face a torus
/// band, no sphere and no plane, which is what lets the no-crossings
/// fallback (row 3) be reached without the sphere extent scan
/// refusing first on a trimmed sphere group. Its two faces are the
/// containment door's CLOSED-GROUP torus class: they share both
/// full-period parallels and each wraps the major azimuth through its
/// own self-mated seam, so neither carries a chart window.
fn donut() -> Body<f64> {
    use sweep::{Revolution, RevolveAxis, revolve};
    let lp = bulge_loop(vec![
        (Point2::new(0.5, 1.10), 1.0),
        (Point2::new(0.5, 1.40), 1.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    };
    revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

/// The vase carries a torus face, minted by the kernel itself.
#[test]
fn the_vase_fixture_actually_carries_a_torus_face() {
    let v = vase();
    assert!(
        v.faces()
            .any(|(_, f)| matches!(v.get_surface(f.surface), Some(geom::Surface::Torus { .. }))),
        "the fixture must carry a real torus band"
    );
}

/// **E2E row 1 — the granted crossing case, full pipeline.** The
/// torus band's box clears the brick, the brick genuinely crosses
/// only CYLINDER walls (wired germ), so the pair-scoped gate admits
/// the union — and the spec's ruling says the torus's kind is then
/// irrelevant. To the CUT it always was; to CONTAINMENT it was not,
/// because the join's geometric role resolution
/// (`resolve_roles_geometric`, join.rs) probes `point_in_solid`
/// against the whole PRISTINE other body, whose boundary pre-pass
/// walks EVERY face — the out-of-reach caps and band included — since
/// a ray from the query point crosses the whole boundary and box reach
/// does not enter that question.
///
/// **So this row pinned a refusal, and it is now the completion it
/// always said it should be.** The caps were the first blocker, then
/// the band: the sphere-capped vase's caps answered once the sphere
/// chart's own rectangle landed, the cone-capped vase's once the
/// ray×cone arm did, and the band answers now that issue 1011's torus
/// half has. Every face of both vases is served, so the admitted union
/// completes — and what the row asserts is the analytic volume it must
/// then carry, which is the reviewer's own closed form and was written
/// here before any of the three arms existed.
#[test]
fn a_granted_crossing_union_with_a_torus_band_completes_in_containment() {
    for sphere_caps in [true, false] {
        let a = finished("the vase", vase_with_caps(sphere_caps), Tol::witness());
        let b = finished(
            "the brick",
            brick((-1.0, 1.0), (0.55, 0.93), (-1.0, 1.0), Tol::witness()),
            Tol::witness(),
        );
        let out = match topo::union(&a, &b, Tol::witness()) {
            Err(
                BooleanError::CurvedPairUnsupported { .. }
                | BooleanError::CurvedBooleanUnsupported { .. },
            ) => panic!(
                "sphere_caps = {sphere_caps}: the torus band and caps clear the \
                 brick — the pair-scoped gate must not refuse this union"
            ),
            Err(BooleanError::Containment(e)) => panic!(
                "sphere_caps = {sphere_caps}: the containment door still refuses \
                 by kind — issue 1011 retired that refusal for every analytic \
                 kind this body carries: {e}"
            ),
            Err(other) => {
                panic!("sphere_caps = {sphere_caps}: unexpected refusal shape: {other:?}")
            }
            Ok(out) => out,
        };
        // The completion must carry the analytic volume (the reviewer's
        // own integral, unchanged from when this was the aspirational
        // branch).
        let body = &out.body().expect("a non-empty union").body;
        let (cx, r_arc, a_h) = bulge_arc();
        let iq = 2.0
            * (a_h / 2.0 * (r_arc * r_arc - a_h * a_h).sqrt()
                + r_arc * r_arc / 2.0 * (a_h / r_arc).asin());
        let top = PI
            * (cx * cx * 2.0 * a_h
                + 2.0 * cx * iq
                + (r_arc * r_arc * 2.0 * a_h - 2.0 * a_h * a_h * a_h / 3.0));
        let caps_vol = if sphere_caps {
            2.0 * (2.0 / 3.0) * PI * 0.125
        } else {
            2.0 * PI * 0.25 * 0.5 / 3.0
        };
        let vase_vol = caps_vol + 2.0 * PI * 0.25 * 0.5 + top;
        let want = vase_vol + 2.0 * 2.0 * 0.38 - PI * 0.25 * 0.38;
        let got = vol(body);
        assert!(
            (got - want).abs() < 1e-6,
            "sphere_caps = {sphere_caps}: completed union volume {got} != \
             analytic {want}"
        );
    }
}

/// **E2E row 2 — the refused pose.** The same brick raised into the
/// torus band (y ∈ [1.05, 1.45] against the band's y ∈ [1.25 ± R_arc]
/// ≈ [0.97, 1.53]). The torus is on the union's KIND roster, so the
/// gate admits the pair; the crossing layer records the brick's plane
/// against the torus band, and the union stops at the JOIN's germ
/// frame, which has no `(Torus, Plane)` arm. That is the join-side door
/// the torus gate admission was told to expect, measured.
#[test]
fn the_same_union_posed_into_the_torus_band_stops_at_the_germ_frame() {
    let a = finished("the vase", vase(), Tol::witness());
    let b = finished(
        "the brick",
        brick((-1.0, 1.0), (1.05, 1.45), (-1.0, 1.0), Tol::witness()),
        Tol::witness(),
    );
    let err =
        topo::union(&a, &b, Tol::witness()).expect_err("a torus × plane germ has no join arm");
    let BooleanError::GermFrameUnsupported {
        a_kind: geom::SurfaceKind::Torus,
        b_kind: geom::SurfaceKind::Plane,
        ..
    } = err
    else {
        panic!("expected the germ frame's (Torus, Plane) refusal, got {err:?}");
    };
}

/// **Row 3 — what the ruling's own blind spot answered, and what it
/// answers now.** A torus face whose box clears a DISJOINT other
/// operand is admitted (correct per the spec's ruling: it can enter no
/// crossing). But with no crossings at all the pipeline falls through
/// to the containment fallback, and `point_in_solid` walks EVERY face
/// of the classified-against body — box reach never enters it, because
/// a ray crosses the whole boundary. So the admitted union used to
/// refuse in `face_geo`, naming the TORUS kind and the missing arm: a
/// healthy body and an honest capability boundary, which is what this
/// row pinned.
///
/// **That boundary is gone.** Issue 1011's torus half landed the
/// ray×torus arm, so the fallback answers and the union completes as
/// the two-solid assembly this row always said it should — which is
/// what the pin was FOR. What survives, and is what the row now
/// asserts, is the structural claim underneath it: the pair-scoped
/// gate admits the operation (nothing here is refused by kind at the
/// gate), and the containment door then has to answer for every face
/// of a body whose only curved faces are torus faces.
#[test]
fn a_disjoint_union_with_a_torus_face_is_admitted_and_now_answered() {
    let a = finished("the donut", donut(), Tol::witness());
    let b = finished(
        "the brick",
        brick((5.0, 6.0), (0.0, 1.0), (0.0, 1.0), Tol::witness()),
        Tol::witness(),
    );
    let out = match topo::union(&a, &b, Tol::witness()) {
        Err(
            BooleanError::CurvedPairUnsupported { .. }
            | BooleanError::CurvedBooleanUnsupported { .. },
        ) => panic!(
            "the donut clears a body five units away — the pair-scoped \
             gate must not refuse this"
        ),
        Err(BooleanError::Containment(e)) => panic!(
            "the containment door still refuses a torus operand — this is the \
             refusal issue 1011's torus half retires: {e}"
        ),
        Err(other) => panic!("unexpected refusal shape for the fallback path: {other:?}"),
        Ok(out) => out,
    };
    let result = out.body().expect("a disjoint union is not empty");
    assert_eq!(result.kind, topo::BooleanResultKind::Assembly);
    assert_eq!(topo::validate_closed(&result.body), Ok(()));
    // The honest assembly: both operands' volumes, summed.
    //
    // **This does not witness the four-root ray**, and the comment used
    // to say it did. The assertion is on the assembled body's VOLUME,
    // which the props lane computes from the boundary in closed form and
    // never asks a containment door about; mutating the quartic's
    // biquadratic factor sign leaves this row green. What it witnesses
    // is that the containment door ANSWERED for every face of a
    // torus-only operand — that is what the fallback needed and what
    // this row's own frontier was about. The four-root ray has its own
    // witness in `bool3_torus_doors::the_four_root_ray_through_the_
    // hole_reads_the_nearest_wall`, which probes the hole directly.
    let want = vol(&a) + vol(&b);
    let got = vol(&result.body);
    assert!(
        (got - want).abs() < 1e-9,
        "the assembly must carry both operands' volume: {got} != {want}"
    );
}

/// A brick whose `x = x1` face is relabelled to `surface`, the face's
/// boundary left on the brick's lines, and that face.
fn brick_with_face(surface: geom::Surface<f64>) -> (Body<f64>, topo::FaceKey) {
    let mut b = brick::<f64>((2.0, 3.0), (0.0, 1.0), (0.0, 1.0), Tol::witness());
    let face = b
        .faces()
        .find(|(_, f)| match b.get_surface(f.surface) {
            Some(geom::Surface::Plane { origin, normal, .. }) => {
                (origin.x - 3.0).abs() < 1e-9 && normal.x.abs() > 0.5
            }
            _ => false,
        })
        .map(|(k, _)| k)
        .expect("the brick has an x = 3 face");
    // Lifts RechartStrandsDescriptions: the relabel is the at-rest gate's input.
    b.set_face_surface_unvouched_for_tests(
        face,
        topo::FaceSurface::New {
            surface,
            sense: true,
        },
    )
    .unwrap();
    (b, face)
}

/// The relabelled brick's at-rest refusal, checked to sit on the
/// relabelled face and nowhere else: one `DescriptionNotAdjacent` for
/// each of the face's four boundary edges (lines the relabelled
/// surface does not hold), and one pcurve finding on one of the face's
/// own half-edges, which is returned.
fn refused_on_the_relabelled_face(b: Body<f64>, face: topo::FaceKey) -> topo::PcurveMintError {
    let outer = b.get_face(face).unwrap().outer;
    let topo::LoopBoundary::Cycle { first } = b.get_loop(outer).unwrap().boundary else {
        panic!("the relabelled face's outer loop is a cycle")
    };
    let hes = b.loop_cycle(first).unwrap();
    let mut edges: Vec<_> = hes
        .iter()
        .map(|&he| b.get_half_edge(he).unwrap().edge)
        .collect();
    edges.sort();
    assert_eq!(edges.len(), 4);
    let errors = topo::AtRestBody::validate(b, Tol::witness())
        .expect_err("a relabel over a brick's lines does not finish");
    assert_eq!(errors.len(), 5, "four edges and one pcurve: {errors:?}");
    let mut not_adjacent: Vec<_> = errors
        .iter()
        .filter_map(|e| match e {
            topo::ValidationError::DescriptionNotAdjacent { edge } => Some(*edge),
            _ => None,
        })
        .collect();
    not_adjacent.sort();
    assert_eq!(not_adjacent, edges, "{errors:?}");
    let [finding] = &errors
        .iter()
        .filter_map(|e| match e {
            topo::ValidationError::Pcurve { finding } => Some(finding.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()[..]
    else {
        panic!("one pcurve finding: {errors:?}")
    };
    let half_edge = match finding {
        topo::PcurveMintError::LoopDiscontinuity { half_edge }
        | topo::PcurveMintError::Certify { half_edge, .. } => *half_edge,
        other => panic!("a loop or certification finding, got {other:?}"),
    };
    assert!(
        hes.contains(&half_edge),
        "the pcurve finding sits on the relabelled face's loop: {errors:?}"
    );
    finding.clone()
}

/// The tilted cone of rows 4 and 6, relabelled onto the brick's
/// `x = 3` face.
fn tilted_cone_brick() -> (Body<f64>, topo::FaceKey) {
    brick_with_face(geom::Surface::Cone {
        apex: Point3::new(2.5, 0.5, 2.0),
        axis: Vec3::new(0.6, 0.0, 0.8), // unit
        half_angle: 0.4,
        u_ref: Vec3::new(0.8, 0.0, -0.6),
    })
}

/// **Row 4 — the cone at a TILTED axis**: the cone-relabelled brick
/// does not finish, so no probe meets its face's box at the boolean.
/// The at-rest gate refuses it on the relabelled face: its four lines
/// lie off the cone, and its loop's pcurves do not close on the cone's
/// chart (a loop discontinuity).
#[test]
fn a_probe_on_a_tilted_cones_locus_is_always_refused() {
    let (a, face) = tilted_cone_brick();
    let finding = refused_on_the_relabelled_face(a, face);
    assert!(
        matches!(finding, topo::PcurveMintError::LoopDiscontinuity { .. }),
        "{finding:?}"
    );
}

/// **Row 5 — the torus twin**: the brick with a TILTED torus relabelled
/// onto its face does not finish either. The at-rest gate refuses it
/// on that face: its four lines lie off the torus, and a line pcurve
/// cannot be charted on a torus at all (`CarrierOffChart`, torus chart,
/// line carrier).
#[test]
fn a_probe_on_a_tilted_toruss_locus_is_always_examined() {
    let axis = Vec3::new(1.0, 2.0, 2.0).normalize();
    let (a, face) = brick_with_face(geom::Surface::Torus {
        center: Point3::new(2.5, 0.5, 3.0),
        axis,
        major_radius: 0.8,
        minor_radius: 0.2,
        u_ref: axis.orthonormal_basis().0,
    });
    let finding = refused_on_the_relabelled_face(a, face);
    assert!(
        matches!(
            finding,
            topo::PcurveMintError::Certify {
                error: geom_brep::PcurveCertifyError::CarrierOffChart {
                    chart: geom::SurfaceKind::Torus,
                    carrier: geom::CurveKind::Line,
                    ..
                },
                ..
            }
        ),
        "{finding:?}"
    );
}

/// **Row 6 — the admit side at a tilted axis**: no probe is admitted
/// against the cone-relabelled brick, however far clear of the cone's
/// slab, because the brick itself does not finish; the at-rest gate
/// refuses it on the relabelled face, as in row 4.
#[test]
fn a_cone_relabelled_brick_clear_of_the_probe_is_refused_at_rest() {
    let (a, face) = tilted_cone_brick();
    let finding = refused_on_the_relabelled_face(a, face);
    assert!(
        matches!(finding, topo::PcurveMintError::LoopDiscontinuity { .. }),
        "{finding:?}"
    );
}
