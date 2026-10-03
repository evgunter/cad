//! M5 S13 acceptance: the die-pips enablers — the containment-fallback
//! re-cut (§1) and the plane×sphere germ arm (§2).
//!
//! **The die-pips smoke row is GREEN**: `slab ∖ ball` cuts a pip-shaped
//! spherical cavity with the exact closed-form volume, tier-3 valid,
//! certified pcurve on the seam circle, both sweep strategies
//! bit-identical — WITHOUT the fitted-chord lane: a pip's section curve
//! is the exact plane×sphere Circle (C5), so no chord is ever fitted.
//!
//! The pip fixtures deliberately keep the constructor's natural chart
//! (the revolve ball's poles sit on a HORIZONTAL axis, its seam great
//! circle in a horizontal plane), so no edge of either operand crosses
//! the other — the S12 finding's poking-but-not-crossing shape. Every
//! row here therefore exercises §1's re-cut end to end: the extent scan
//! certifies the escape, the group is rigidly re-charted about the
//! escape normal (a rotation about the sphere's own center — the same
//! point set), and the re-entered pipeline finds the section circles
//! with the ordinary crossing layer and joins them through §2's arm.
//!
//! **Tolerance shape.** Volume slack derives from the resolved band;
//! the in-band escalation row PLACES its fixture from the resolved
//! band's own [zero, escalate] window, so the row is honest at every ε
//! (the FitSampleBudget-precedent multi-ε discipline).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::operands::slab;
use core::f64::consts::PI;

use geom::Curve3;
use geom::Surface;
use geom_core::Tol;
use geom_core::{Band, Point2, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::ball_poled_y;
use sweep::{Extrusion, extrude};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::test_support::GraftBridge;
use topo::{Body, BooleanDeclarations, BooleanError};

// ---------------------------------------------------------------------
// Fixtures and helpers (the S12 suite's, radius-generalized).
// ---------------------------------------------------------------------

fn slack() -> f64 {
    (1e3 * Tol::witness().get().eps).max(1e-9)
}

fn vol(body: &Body<f64>) -> f64 {
    topo::mass_properties(body, Tol::witness()).unwrap().volume
}

/// Spherical cap volume, height `h` off a radius-`r` ball.
fn cap(r: f64, h: f64) -> f64 {
    PI * h * h * (3.0 * r - h) / 3.0
}

/// Both sweep strategies, bit-identical, tier-3 valid (the S12 door).
fn both_lanes(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let decls = BooleanDeclarations::none();
    let realized = boolean_op_with(op, a, b, &decls, SweepStrategy::Realized, Tol::witness())
        .unwrap_or_else(|e| panic!("{op:?} (realized): {e}"));
    let idealized = boolean_op_with(op, a, b, &decls, SweepStrategy::Idealized, Tol::witness())
        .unwrap_or_else(|e| panic!("{op:?} (idealized): {e}"));
    let rb = realized.body().expect("a body").body.clone();
    let ib = idealized.body().expect("a body").body.clone();
    assert_eq!(
        format!("{rb:?}"),
        format!("{ib:?}"),
        "{op:?}: the two sweep strategies must produce bit-identical results"
    );
    if let Err(errs) = topo::validate_geometric(&rb, Tol::witness()) {
        panic!("{op:?} result is not tier-3 valid: {errs:?}");
    }
    rb
}

/// The pip: radius 0.5, center 0.2 above the top face — the cavity is
/// a cap of height 0.3, seam circle radius √0.21 at z = 1.
const PIP_R: f64 = 0.5;
const PIP_H: f64 = 0.3;

/// The pip's ball, y-poled: its poles lie on a horizontal axis, which
/// is the chart the §1 re-cut must re-align.
fn pip_ball(x: f64, y: f64) -> Body<f64> {
    ball_poled_y(PIP_R, Vec3::new(x, y, 1.0 + PIP_R - PIP_H), Tol::witness())
}

// ---------------------------------------------------------------------
// §2: the die-pips smoke row and its twins.
// ---------------------------------------------------------------------

/// **The die-pips smoke row, GREEN** (flipped from the S12 refusal pin
/// per that pin's own doc comment): `slab ∖ ball` bites a pip-shaped
/// spherical cap out of the top face. Exact volume, one shell, the
/// cavity's sphere fragments reversed (the S12 audited-answer arm live
/// for the sphere class), the seam circle carried as an exact Circle
/// with a CERTIFIED pcurve on the plane chart, both lanes bit-identical.
#[test]
fn die_pip_subtract_is_green() {
    let a = slab();
    let b = pip_ball(2.0, 2.0);
    let cut = both_lanes(BooleanOp::Subtract, &a, &b);

    assert!(
        (vol(&cut) - (16.0 - cap(PIP_R, PIP_H))).abs() < slack(),
        "pip volume: got {}, want {}",
        vol(&cut),
        16.0 - cap(PIP_R, PIP_H)
    );
    assert_eq!(cut.shells().count(), 1, "a pip is a dent, not a void");

    // The cavity wall keeps the reverted bit (mef inheritance, S12).
    let reversed = cut
        .faces()
        .filter(|(_, f)| {
            matches!(cut.get_surface(f.surface), Some(Surface::Sphere { .. })) && !f.sense
        })
        .count();
    assert!(reversed > 0, "the pip wall must be sense: false");

    // The seam circle: an exact Circle carrier of radius √(r² − d²)
    // (never a fitted chord), described as a surface intersection.
    let rho = (PIP_R * PIP_R - (PIP_R - PIP_H) * (PIP_R - PIP_H)).sqrt();
    let band = Band::linear(Tol::witness()).unwrap();
    let mut seam_arcs = 0;
    for (ek, e) in cut.edges() {
        let Some(curve) = cut.get_curve_geom(e.curve).and_then(|g| g.certified()) else {
            continue;
        };
        let &Curve3::Circle { radius, center, .. } = curve.carrier() else {
            continue;
        };
        if (radius - rho).abs() < slack() && (center.z - 1.0).abs() < slack() {
            seam_arcs += 1;
            // Certified pcurve on the seam circle: the plane-side
            // chart image certifies through the ordinary PR 6 gate.
            let (t0, t1) = curve.params();
            let plane = Surface::Plane {
                origin: geom_core::Point3::new(0.0, 0.0, 1.0),
                normal: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            };
            let p = geom_brep::chart_pcurve(curve.carrier(), &plane, band)
                .unwrap_or_else(|e| panic!("seam pcurve on the plane chart: {e:?}"));
            let cache = geom_brep::PcurveCache::certify(p, t0, t1, curve.carrier(), &plane, band)
                .unwrap_or_else(|e| panic!("seam pcurve certification: {e:?} (edge {ek:?})"));
            assert!(cache.certificate().max_residual < 1e-9);
        }
    }
    assert!(
        seam_arcs >= 2,
        "the seam circle survives as exact circle arcs, got {seam_arcs}"
    );
}

/// `slab ∩ ball` is the cap itself (closed form), and ∖/∩ are additive
/// against the slab — the S12 oracle, now on the sphere class.
#[test]
fn die_pip_intersect_is_the_cap_and_additive() {
    let a = slab();
    let b = pip_ball(2.0, 2.0);
    let met = both_lanes(BooleanOp::Intersect, &a, &b);
    assert!(
        (vol(&met) - cap(PIP_R, PIP_H)).abs() < slack(),
        "cap volume: got {}",
        vol(&met)
    );
    assert_eq!(met.shells().count(), 1);

    let cut = both_lanes(BooleanOp::Subtract, &a, &b);
    assert!(
        (vol(&cut) + vol(&met) - 16.0).abs() < slack(),
        "V(A∖B) + V(A∩B) = V(A)"
    );
}

/// **The TWO-pip row**: two disjoint balls out of one slab — the
/// closed-sphere GROUP arm under multiple sphere surfaces. The two-ball
/// operand is itself §1 output (a no-crossing ∪ whose extent scan
/// certifies the balls disjoint and assembles two shells), and the
/// subtract re-cuts EACH group about its own escape normal. The
/// assembly and the re-cut graft carry certificates and the seam zip
/// re-certifies, each graft's bridge read off its record.
#[test]
fn two_pips_cut_under_the_group_arm() {
    // Placements: disjoint balls, both far enough from every slab
    // CORNER that the idealized lane's conservative line-vs-sphere
    // clearance bound (endpoint residual minus the span-length dip,
    // `bool_line_cylinder_clearance`) stays definite on the 4-long
    // edges -- the pre-existing PR 9 frontier's honest envelope, not
    // this unit's.
    let b1 = pip_ball(2.0, 1.2);
    let b2 = pip_ball(2.0, 2.8);
    // The two-ball operand assembles in the REALIZED lane (the
    // idealized sweep EXAMINES every pair, and a conic edge against a
    // curved face is the pre-existing pierce frontier -- typed, not
    // this unit's; the realized tree prunes those distant pairs).
    let _ = topo::test_support::take_graft_bridges();
    let pair = topo::union(&b1, &b2, Tol::witness())
        .expect("disjoint balls assemble through the certified scan")
        .body()
        .expect("a body")
        .body
        .clone();
    assert_eq!(pair.shells().count(), 2, "two disjoint balls, two shells");

    // The assembly carries its kept ball's certificates.
    assert_eq!(
        topo::test_support::take_graft_bridges(),
        [GraftBridge::RemapKeys],
        "the disjoint union's assembly graft"
    );
    let cut = both_lanes(BooleanOp::Subtract, &slab(), &pair);
    // Per lane: the re-cut grafts the second turned ball back onto the
    // first, carrying, and the seam zip then re-certifies.
    assert_eq!(
        topo::test_support::take_graft_bridges(),
        [
            GraftBridge::RemapKeys,
            GraftBridge::Recertify,
            GraftBridge::RemapKeys,
            GraftBridge::Recertify,
        ],
        "the re-cut graft, then the seam zip's, in each lane"
    );
    assert!(
        (vol(&cut) - (16.0 - 2.0 * cap(PIP_R, PIP_H))).abs() < slack(),
        "two pips: got {}",
        vol(&cut)
    );
    assert_eq!(cut.shells().count(), 1);
    let sphere_surfaces: std::collections::BTreeSet<String> = cut
        .faces()
        .filter_map(|(_, f)| match cut.get_surface(f.surface) {
            Some(s @ Surface::Sphere { .. }) => Some(format!("{s:?}")),
            _ => None,
        })
        .collect();
    assert_eq!(
        sphere_surfaces.len(),
        2,
        "both pip walls survive, each on its own sphere surface"
    );
}

// ---------------------------------------------------------------------
// §1: the extent scan's refusal/escalation arms.
// ---------------------------------------------------------------------

/// **The in-band extent escalation row (band-scaled)**: a ball whose
/// surface reaches into the top face's carrier by exactly the middle of
/// the resolved band's [zero, escalate) window. The extent trilean
/// (`bool_sphere_extent_gap`, margin `r − |s|`) can neither call it
/// clear nor a definite escape — the two-tolerance shape's escalation
/// half, placed FROM the band so the row is honest at every ε.
#[test]
fn in_band_extent_escalates_instead_of_answering() {
    let band = Band::linear(Tol::witness()).unwrap();
    let mid = 0.5 * (band.zero() + band.escalate());
    // Center above the slab so that r − |s| = mid for the top face.
    let b = ball_poled_y(1.0, Vec3::new(2.0, 2.0, 2.0 - mid), Tol::witness());
    let err =
        topo::union(&slab(), &b, Tol::witness()).expect_err("an in-band extent must not answer");
    let BooleanError::Escalated { .. } = err else {
        panic!("expected the extent trilean's escalation, got {err:?}");
    };
}

/// A ball clear of the slab still assembles through the fallback (the
/// scan certifies every pair disjoint and gets out of the way), and a
/// ball wholly inside still classifies as contained — the fallback's
/// sound configurations keep their whole-shell answers.
#[test]
fn certified_disjoint_and_contained_shells_keep_their_answers() {
    // Disjoint: assembly, volumes add.
    let far = ball_poled_y(0.5, Vec3::new(2.0, 2.0, 3.0), Tol::witness());
    let joined = both_lanes(BooleanOp::Union, &slab(), &far);
    assert!(
        (vol(&joined) - (16.0 + 4.0 * PI * 0.125 / 3.0)).abs() < slack(),
        "disjoint union adds volumes: {}",
        vol(&joined)
    );
    assert_eq!(joined.shells().count(), 2);

    // Contained: ∪ is the slab; ∖ voids the ball out.
    let buried = ball_poled_y(0.4, Vec3::new(2.0, 2.0, 0.5), Tol::witness());
    let joined = both_lanes(BooleanOp::Union, &slab(), &buried);
    assert!((vol(&joined) - 16.0).abs() < slack(), "{}", vol(&joined));
    let cut = both_lanes(BooleanOp::Subtract, &slab(), &buried);
    let ball_v = 4.0 * PI * 0.4_f64.powi(3) / 3.0;
    assert!(
        (vol(&cut) - (16.0 - ball_v)).abs() < slack(),
        "buried subtract voids the ball: {}",
        vol(&cut)
    );
    assert_eq!(cut.shells().count(), 2, "outer shell + reverted void");
}

/// Sphere-vs-sphere boundaries the scan cannot certify refuse TYPED.
/// The section circle is exact and the germ frame names it; what is
/// absent is the JOIN's arm for a curved×curved germ pair. The pair is
/// offset VERTICALLY, so neither ball's seam edges enter the other's
/// certified box — the poking-but-not-crossing shape again, this time
/// between two spheres, which only the scan can see. That direction is
/// not a depth choice: a seam great circle lies in the plane `z = c_z`
/// with the ball's own radius, so a Z-offset seam is equidistant from
/// the other centre all the way round and never crosses it. Every
/// offset that DOES cross a seam pierces a curved face and stops a
/// layer higher.
#[test]
fn overlapping_sphere_pair_refuses_typed_at_the_scan() {
    let b1 = ball_poled_y(1.0, Vec3::new(2.0, 2.0, 0.5), Tol::witness());
    let b2 = ball_poled_y(1.0, Vec3::new(2.0, 2.0, 1.9), Tol::witness());
    let err = topo::union(&b1, &b2, Tol::witness()).expect_err("no sphere×sphere seam lane");
    let BooleanError::SpheresMeet { .. } = err else {
        panic!("expected the scan's typed refusal, got {err:?}");
    };
}

/// **The scan's TRIMMED-GROUP arm, and where it actually bites.** A pip
/// RESULT carries a trimmed sphere face group (the cap), and using it as
/// an operand in a second no-crossing boolean used to refuse typed at
/// the scan: the extent certificate `center ± r` is the whole group's,
/// so it was only spent for a CLOSED group.
///
/// That certificate is asked WHERE IT IS USED now, not on arrival, and
/// the trimmed group is served for everything it does not need: a box
/// that proves the whole sphere clear proves the trimmed subset clear,
/// because the face is a subset of the sphere and over-claiming is the
/// sound direction for a SEPARATION test. What still needs closedness
/// is the plane arm's ESCAPE conclusion — whole-circle membership is
/// the carrier's, and the re-chart it feeds rotates a closed group
/// about its centre — and that arm keeps the refusal.
///
/// So this pair assembles: two shells, volumes add, and sequential pip
/// authoring has its live route.
#[test]
fn trimmed_sphere_group_operand_assembles_with_a_clear_partner() {
    let pip = both_lanes(BooleanOp::Subtract, &slab(), &pip_ball(2.0, 2.0));
    let pip_v = vol(&pip);
    let far = ball_poled_y(0.5, Vec3::new(2.0, 2.0, 3.5), Tol::witness());
    let joined = both_lanes(BooleanOp::Union, &pip, &far);
    assert_eq!(joined.shells().count(), 2, "the pipped slab plus the ball");
    let want = pip_v + 4.0 * PI * 0.125 / 3.0;
    assert!(
        (vol(&joined) - want).abs() < slack(),
        "disjoint union adds volumes: {} vs {want}",
        vol(&joined)
    );
}

/// **A ball in the corner of the cylinder wall's box.** The wall's box
/// is the rectangular prism around a ROUND slab, so it over-claims at
/// its own corners; the ball is parked in one of them, radially 0.43
/// out from the axis against a 0.35 wall, genuinely clear of the solid
/// while its box meets the wall's. No edge pair is examined, the
/// fallback fires, and the section pass certifies the sphere × wall
/// pair apart (its carriers have no section), so the union answers a
/// two-solid assembly of the two volumes.
#[test]
fn a_ball_in_the_wall_boxs_corner_is_certified_separated() {
    let disc = bulge_loop(vec![
        (Point2::new(0.35, 0.0), 1.0),
        (Point2::new(-0.35, 0.0), 1.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![disc])
        .validate(Tol::witness())
        .unwrap();
    let cyl = extrude(&vp, Extrusion::Distance(1.3), Tol::witness())
        .unwrap()
        .body;
    let ball = ball_poled_y(0.05, Vec3::new(0.34, 0.34, 0.65), Tol::witness());
    let out = topo::union(&cyl, &ball, Tol::witness())
        .expect("a genuinely separated pair must be certified, not refused");
    let built = out.body().expect("a non-empty union");
    assert!(
        matches!(built.kind, topo::boolean::BooleanResultKind::Assembly),
        "two disjoint solids union to an assembly, got {:?}",
        built.kind
    );
    assert_eq!(
        topo::validate_geometric(&built.body, Tol::witness()),
        Ok(()),
        "tier 3"
    );
    let want = PI * 0.35_f64.powi(2) * 1.3 + 4.0 / 3.0 * PI * 0.05_f64.powi(3);
    assert!(
        (vol(&built.body) - want).abs() < slack(),
        "the union adds the volumes: {} vs {want}",
        vol(&built.body)
    );
}

/// **The other side of that boundary, and the consumer-visible half
/// of #862**: the same ball hovering over the same cylinder's top
/// cap, clear of it by less than the radius, is CERTIFIED separated
/// and the union answers a two-solid assembly. An axial over-claim in
/// the wall's box turns exactly this pair into a false
/// `FallbackExtentUnsupported`.
#[test]
fn a_ball_above_the_cylinders_cap_is_certified_separated() {
    let disc = bulge_loop(vec![
        (Point2::new(0.35, 0.0), 1.0),
        (Point2::new(-0.35, 0.0), 1.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![disc])
        .validate(Tol::witness())
        .unwrap();
    let cyl = extrude(&vp, Extrusion::Distance(1.3), Tol::witness())
        .unwrap()
        .body;
    // Ball bottom at z = 1.55, cap at z = 1.3: a gap of 0.25, less
    // than the wall's 0.35 radius.
    let ball = ball_poled_y(0.2, Vec3::new(0.0, 0.0, 1.75), Tol::witness());
    let out = topo::union(&cyl, &ball, Tol::witness())
        .expect("a genuinely separated pair must be certified, not refused");
    let kind = out.body().expect("a non-empty union").kind;
    assert!(
        matches!(kind, topo::boolean::BooleanResultKind::Assembly),
        "two disjoint solids union to an assembly, got {kind:?}"
    );
}

/// **A ball straddling a notched wall's carrier, clear of the wall
/// face, builds.** The slab's wall turns three quarters of the way
/// round; the ball (radius 0.05) sits on the wall's cylinder in the
/// missing quarter, so it straddles the CARRIER while missing the
/// trimmed face, 0.247 from each flat face. Its box meets the wall's
/// (which spans the whole turn's square), no edge pair crosses, and the
/// fallback hands the sphere × wall pair to the section pass, which
/// certifies the section out of the face. Every op in both orders,
/// tier 3, against `¾·π·0.35²·1.3` and `4π·0.05³/3` within the volume
/// enclosure's own half-width. A carrier-only
/// certificate (clear of the whole cylinder, or inside it) cannot speak
/// here; this pose refused `FallbackExtentUnsupported` under one.
#[test]
fn a_ball_straddling_a_notched_walls_carrier_builds() {
    let notched = bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(0.35, 0.0), (3.0 * PI / 8.0).tan()),
        (Point2::new(0.0, -0.35), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![notched])
        .validate(Tol::witness())
        .unwrap();
    let slab = extrude(&vp, Extrusion::Distance(1.3), Tol::witness())
        .unwrap()
        .body;
    let at = 0.35 * core::f64::consts::FRAC_1_SQRT_2;
    let ball = ball_poled_y(0.05, Vec3::new(at, -at, 0.65), Tol::witness());
    let (v_slab, v_ball) = (
        0.75 * PI * 0.35_f64.powi(2) * 1.3,
        4.0 * PI * 0.05_f64.powi(3) / 3.0,
    );
    for (op, x, y, want) in [
        (BooleanOp::Union, &slab, &ball, Some(v_slab + v_ball)),
        (BooleanOp::Union, &ball, &slab, Some(v_slab + v_ball)),
        (BooleanOp::Intersect, &slab, &ball, None),
        (BooleanOp::Intersect, &ball, &slab, None),
        (BooleanOp::Subtract, &slab, &ball, Some(v_slab)),
        (BooleanOp::Subtract, &ball, &slab, Some(v_ball)),
    ] {
        let out = boolean_op_with(
            op,
            x,
            y,
            &BooleanDeclarations::none(),
            SweepStrategy::Realized,
            Tol::witness(),
        )
        .unwrap_or_else(|e| panic!("{op:?}: refused {e:?}"));
        match (out.body(), want) {
            (Some(b), Some(w)) => {
                assert_eq!(
                    topo::validate_geometric(&b.body, Tol::witness()),
                    Ok(()),
                    "{op:?}: tier 3"
                );
                // Against the enclosure's own half-width, which must be
                // able to see the ball: `slack()` is 1e-3 at ε = 1e-6,
                // twice the ball's whole volume.
                let props = topo::mass_properties(&b.body, Tol::witness()).unwrap();
                assert!(
                    props.volume_pad < v_ball / 10.0,
                    "{op:?}: a pad of {} cannot see a ball of {v_ball}",
                    props.volume_pad
                );
                assert!(
                    (props.volume - w).abs() <= props.volume_pad + 1e-12,
                    "{op:?}: volume {} against {w} (pad {})",
                    props.volume,
                    props.volume_pad
                );
            }
            (None, None) => {}
            (got, _) => panic!("{op:?}: {:?} against {want:?}", got.map(|b| vol(&b.body))),
        }
    }
}
