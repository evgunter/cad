//! M6-6 acceptance: the **curved sense-flip tier gate** (check 6's
//! curved arm) — the pin matrix from the executed substrate truth
//! table.
//!
//! Before this unit, NO gate caught a single-face curved sense flip:
//! flipped cylinder walls, cone laterals, rim-bearing sphere zones and
//! torus bands all certified tier-3 GREEN with volumes BIT-IDENTICAL
//! to the honest bodies, and fully inside-out washer/cone/donut/
//! lantern bodies certified green with positive volume (their planar
//! caps are arc-bounded, so the planar check-6 arm never saw them).
//! These rows pin the closure: per kind and per direction where the
//! corpus offers them, the hand-flipped body now refuses with
//! `CurvedSenseInverted` naming the flipped face — and every HONEST
//! body, including the S11 `sense: false` faces (a washer's bore, a
//! die's dimples, a countersink's conical bore), still certifies
//! green. The rimless sphere band is pinned AS the documented
//! residual: its boundary encodes no side, so a half-flipped ball
//! stays exempt (V = 0, Zero-exempt posture) while the fully-flipped
//! ball remains check 7's `NegativeVolume`. The QUADRATURE-owned
//! conic-trimmed class is likewise pinned AS residual (adversarial
//! review, M2): a wall trimmed by an ellipse rim slips BOTH layers —
//! the kernel arm exempts on the boundary parse's typed refusal, the
//! import rider finds no circle-rim pair — so `cut_cylinder`'s flips
//! (single wall AND whole body) stay green until the ellipse-rim
//! encoding lands.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{ball, cone, cube, cut_cylinder, die_pips, donut, lily_lantern, notched, washer};
use geom::Surface;
use geom_core::Point2;
use profile::RawLoop;
use topo::{Body, FaceKey, ValidationError, validate_geometric};
use geom_core::Tol;

/// The faces of `body` whose surface matches `pred`, with their S10
/// sense bits.
fn faces_where(body: &Body<f64>, pred: fn(&Surface<f64>) -> bool) -> Vec<(FaceKey, bool)> {
    body.faces()
        .filter_map(|(k, f)| {
            let s = body.get_surface(f.surface)?;
            pred(s).then_some((k, f.sense))
        })
        .collect()
}

/// One direction of one pin row: flipping exactly `face` must refuse
/// with `CurvedSenseInverted` naming exactly that face.
fn assert_flip_refuses(body: &Body<f64>, face: FaceKey, label: &str) {
    let flipped = body
        .flipped_face_sense_for_tests(face)
        .expect("live face key");
    let errs = validate_geometric(&flipped, Tol::witness())
        .expect_err(&format!("{label}: a sense-flipped curved face must refuse"));
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::CurvedSenseInverted { face: f } if *f == face)),
        "{label}: expected CurvedSenseInverted on the flipped face, got {errs:?}"
    );
}

/// Every face's sense inverted — the whole-body inversion the
/// substrate executed (curved `revert`'s at-rest image).
fn flip_all(body: &Body<f64>) -> Body<f64> {
    let keys: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
    keys.iter().fold(body.clone(), |b, &k| {
        b.flipped_face_sense_for_tests(k).expect("live face key")
    })
}

/// A washer with a conical bore: the corpus's missing native-`.F.`
/// cone (the F→T pin direction). Revolving [(1,0),(2,0),(2,1),(0.5,1)]
/// about the sketch axis slants the inner wall — a truncated-cone bore
/// whose material lies radially OUTSIDE it, so S11 mints `sense:
/// false` with the winding still interior-left.
fn countersink() -> Body<f64> {
    use geom_core::Tolerance;
    use profile::{Profile, ProfileLoop, SketchPlane};
    use sweep::{Revolution, RevolveAxis, revolve};
    let lp = ProfileLoop::polygon([
        Point2::new(1.0, 0.0),
        Point2::new(2.0, 0.0),
        Point2::new(2.0, 1.0),
        Point2::new(0.5, 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    revolve(
        &profile,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: geom_core::Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// Cylinder, both directions: the washer's outer wall (T→F) and its
/// native-`.F.` bore (F→T). The honest body — including the bore, the
/// gate's defining negative control — certifies green.
#[test]
fn washer_cylinder_walls_both_directions() {
    let body = washer();
    assert!(validate_geometric(&body, Tol::witness()).is_ok(), "honest washer green");
    let walls = faces_where(&body, |s| matches!(s, Surface::Cylinder { .. }));
    assert_eq!(walls.len(), 2, "washer: outer wall + bore");
    assert!(
        walls.iter().any(|&(_, s)| s) && walls.iter().any(|&(_, s)| !s),
        "S11: outer wall sense true, bore sense false: {walls:?}"
    );
    for (k, sense) in walls {
        let dir = if sense { "outer T->F" } else { "bore F->T" };
        assert_flip_refuses(&body, k, &format!("washer {dir}"));
    }
}

/// Cylinder on an extrude, both directions: the notched prism's convex
/// bulge (T→F) and concave bite (F→T).
#[test]
fn notched_walls_both_directions() {
    let body = notched();
    assert!(validate_geometric(&body, Tol::witness()).is_ok(), "honest notched green");
    let walls = faces_where(&body, |s| matches!(s, Surface::Cylinder { .. }));
    assert_eq!(walls.len(), 2, "notched: convex + concave wall");
    assert!(
        walls.iter().any(|&(_, s)| s) && walls.iter().any(|&(_, s)| !s),
        "S11: convex true, concave false: {walls:?}"
    );
    for (k, sense) in walls {
        let dir = if sense { "convex T->F" } else { "concave F->T" };
        assert_flip_refuses(&body, k, &format!("notched {dir}"));
    }
}

/// Cone T→F: the apex cone's lateral half-faces.
#[test]
fn cone_lateral_flip_refuses() {
    let body = cone();
    assert!(validate_geometric(&body, Tol::witness()).is_ok(), "honest cone green");
    let laterals = faces_where(&body, |s| matches!(s, Surface::Cone { .. }));
    assert!(!laterals.is_empty(), "cone: lateral faces present");
    for (k, sense) in laterals {
        assert!(sense, "cone lateral is sense true");
        assert_flip_refuses(&body, k, "cone lateral T->F");
    }
}

/// Cone F→T: the corpus has no native-`.F.` cone, so one is MINTED —
/// the countersink revolve's conical bore (implementer's-call arm of
/// the pin matrix, reported in the unit writeup).
#[test]
fn countersink_cone_bore_both_directions() {
    let body = countersink();
    assert!(
        validate_geometric(&body, Tol::witness()).is_ok(),
        "honest countersink green (S11 negative control for the cone arm)"
    );
    let bores = faces_where(&body, |s| matches!(s, Surface::Cone { .. }));
    assert!(!bores.is_empty(), "countersink: conical bore present");
    for (k, sense) in bores {
        assert!(!sense, "the conical bore is native sense false (S11)");
        assert_flip_refuses(&body, k, "countersink bore F->T");
    }
}

/// Cone T→F on the lantern's pucker + rim-bearing sphere T→F on its
/// doubly-truncated zone (the corpus's pole-free sphere face).
#[test]
fn lily_lantern_zone_and_pucker_flips_refuse() {
    let body = lily_lantern();
    assert!(validate_geometric(&body, Tol::witness()).is_ok(), "honest lantern green");
    let zones = faces_where(&body, |s| matches!(s, Surface::Sphere { .. }));
    let cones = faces_where(&body, |s| matches!(s, Surface::Cone { .. }));
    assert!(!zones.is_empty() && !cones.is_empty(), "zone + pucker");
    for (k, sense) in zones {
        assert!(sense, "lantern zone is sense true");
        assert_flip_refuses(&body, k, "lantern sphere zone T->F");
    }
    for (k, sense) in cones {
        assert!(sense, "lantern pucker is sense true");
        assert_flip_refuses(&body, k, "lantern cone pucker T->F");
    }
}

/// Rim-bearing sphere F→T: one of the die's forty native-`.F.` dimple
/// caps (the sphere arm's S11 negative control is the honest die).
#[test]
fn die_pips_dimple_flip_refuses() {
    let body = die_pips();
    assert!(validate_geometric(&body, Tol::witness()).is_ok(), "honest die green");
    let dimples = faces_where(&body, |s| matches!(s, Surface::Sphere { .. }));
    let (k, sense) = *dimples.first().expect("die: dimple caps present");
    assert!(!sense, "dimple caps are native sense false (S11)");
    assert_flip_refuses(&body, k, "die dimple F->T");
}

/// Torus T→F: both of the donut's half-band faces.
#[test]
fn donut_torus_flips_refuse() {
    let body = donut();
    assert!(validate_geometric(&body, Tol::witness()).is_ok(), "honest donut green");
    let bands = faces_where(&body, |s| matches!(s, Surface::Torus { .. }));
    assert_eq!(bands.len(), 2, "donut: two torus half-bands");
    for (k, sense) in bands {
        assert!(sense, "donut band is sense true");
        assert_flip_refuses(&body, k, "donut torus T->F");
    }
}

/// The RESIDUAL, pinned as residual (gap 1 of the unit): a rimless
/// sphere band's boundary encodes no material side (`Unencoded`), so a
/// half-flipped ball is invisible to the gate BY DESIGN — its volume
/// meters exactly 0 and the Zero-exempt posture (ratified: escalation
/// never flips valid → invalid) keeps it green. Closing it would need
/// a shell-level orientation-agreement check, bigger than this unit.
#[test]
fn ball_half_flip_stays_exempt_residual() {
    let body = ball();
    assert!(validate_geometric(&body, Tol::witness()).is_ok(), "honest ball green");
    let bands = faces_where(&body, |s| matches!(s, Surface::Sphere { .. }));
    assert_eq!(bands.len(), 2, "ball: two rimless half-bands");
    let flipped = body
        .flipped_face_sense_for_tests(bands[0].0)
        .expect("live face key");
    assert!(
        validate_geometric(&flipped, Tol::witness()).is_ok(),
        "the half-flipped rimless ball is the documented residual: V = 0, Zero-exempt"
    );
}

/// The HEADLINE pins: whole-body inversions that certified GREEN with
/// positive, bit-identical volume before this unit (their planar caps
/// are arc-bounded — exempt from the planar check-6 arm — and the
/// volume is traversal-derived, blind to the bits) now refuse with
/// `CurvedSenseInverted` on every rim-bearing curved face.
#[test]
fn whole_body_inversions_refuse() {
    for (label, body) in [
        ("washer", washer()),
        ("cone", cone()),
        ("donut", donut()),
        ("lily_lantern", lily_lantern()),
    ] {
        let inverted = flip_all(&body);
        let errs = validate_geometric(&inverted, Tol::witness())
            .expect_err(&format!("{label}: whole-body inversion must refuse"));
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::CurvedSenseInverted { .. })),
            "{label}: expected CurvedSenseInverted, got {errs:?}"
        );
    }
}

/// The fully-inverted ball stays check 7's: both rimless bands flip,
/// the volume is genuinely −4πr³/3, and `NegativeVolume` (not the
/// curved arm, which has nothing to read on a rimless band) refuses.
#[test]
fn whole_body_inverted_ball_stays_negative_volume() {
    let inverted = flip_all(&ball());
    let errs = validate_geometric(&inverted, Tol::witness()).expect_err("inverted ball must refuse");
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::NegativeVolume)),
        "expected NegativeVolume, got {errs:?}"
    );
    assert!(
        !errs
            .iter()
            .any(|e| matches!(e, ValidationError::CurvedSenseInverted { .. })),
        "the rimless bands are Unencoded — the curved arm must stay silent: {errs:?}"
    );
}

/// Planar control, unchanged: a cube face flip is still the planar
/// arm's `LoopRoleInverted`, never the curved variant.
#[test]
fn planar_control_unchanged() {
    let body = cube();
    let face = body.faces().next().map(|(k, _)| k).expect("cube face");
    let flipped = body
        .flipped_face_sense_for_tests(face)
        .expect("live face key");
    let errs = validate_geometric(&flipped, Tol::witness()).expect_err("flipped cube face must refuse");
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::LoopRoleInverted { .. })),
        "planar arm unchanged: {errs:?}"
    );
    assert!(
        !errs
            .iter()
            .any(|e| matches!(e, ValidationError::CurvedSenseInverted { .. })),
        "the curved arm must not fire on a plane: {errs:?}"
    );
}

/// The conic-trim RESIDUAL, pinned as residual (adversarial review M2;
/// the review's executed counterexample): `cut_cylinder`'s wall is
/// trimmed by a tilted-section ELLIPSE, so its flux is quadrature-owned
/// (winding-derived, bit-free) and its boundary parse refuses typed —
/// the curved arm EXEMPTS it by the inherited posture, and the import
/// rider finds no circle-rim pair to read. A flipped wall AND the
/// whole-body inversion therefore certify GREEN with positive volume:
/// the one corpus body on which the headline defect class survives.
/// This row flips to a refusal when the ellipse-rim material-side
/// encoding lands (follow-up unit); until then, green here is the
/// honest recorded posture, not an accident.
#[test]
fn cut_cylinder_conic_trim_residual_stays_green() {
    let body = cut_cylinder();
    assert!(
        validate_geometric(&body, Tol::witness()).is_ok(),
        "honest cut_cylinder green"
    );
    let walls = faces_where(&body, |s| matches!(s, Surface::Cylinder { .. }));
    assert!(!walls.is_empty(), "cut_cylinder: trimmed wall present");
    for &(k, _) in &walls {
        let flipped = body.flipped_face_sense_for_tests(k).expect("live face key");
        assert!(
            validate_geometric(&flipped, Tol::witness()).is_ok(),
            "conic-trimmed wall flip is the documented residual: both layers exempt"
        );
    }
    let inverted = flip_all(&body);
    assert!(
        validate_geometric(&inverted, Tol::witness()).is_ok(),
        "whole-body-inverted cut_cylinder stays green (residual): quadrature volume \
         is winding-derived and the trimmed wall is exempt"
    );
    let v = topo::mass_properties(&inverted, Tol::witness())
        .expect("volume computes")
        .volume;
    assert!(
        v > 0.0,
        "the inverted body even keeps its positive volume ({v}) — the residual's teeth"
    );
}
