//! The cyl×cyl germ arm's BODY-LEVEL poses at the CERTIFIED scalar
//!.
//!
//! **What this file pins, and what it does NOT.** Every row here drives
//! whole BODIES through the public union door, so a row reaches a
//! predicate only if the layers ahead of it admit the pose. The
//! meeting-axes pose does reach the join and builds there, its nodes
//! minted as rings, and the skew pose reaches the frame dispatch's
//! no-arm door.
//!
//! The new predicate's two-arm pin is still not here. It is
//! `topo::boolean::join`'s `frame_dispatch_interval_tests`, which calls
//! `pair_section_frame` at `Interval` directly and reaches both arms —
//! meeting axes to `Zero`/the nodes, skew axes to a definite sign/`NoArm`.
//! That is the certified-scalar statement about the frame's coplanarity
//! row (`cc_axes_coplanar`, the section table's, which the frame asks);
//! what this file adds is that the
//! BODY-level poses behave at `Interval` as they do at `f64`.
//!
//! The poses are the `f64` suite's, built through the same public
//! doors, so the two lanes differ in the SCALAR and in nothing else.
//! At the intersecting poses the axis-to-axis displacement is near zero
//! by construction — both operands are built centred on the origin, and
//! the seam-clearing spins are rotations ABOUT those axes, so what
//! reaches the margin is rotation crumbs rather than an exactly zero
//! vector. What was measured is the VERDICT: the coplanarity split
//! answers `Zero` at every ε row this suite is sampled at, down to the
//! 1e-12 band. The skew row's displacement is the dyadic `0.375` along
//! `â₁ × â₂`.
//!
//! The re-posed twins are held to the `f64` suite's own EQUALITY at
//! every ε row but the narrowest, and to a narrowly-shaped escape only
//! at `1e-12`, where a general rotation makes the fixture's enclosures
//! wider than the zero band and the certified lane refuses the FIXTURE
//! rather than disagreeing about the geometry.
//! [`same_door_or_escalated`] carries the measurement and both
//! narrowings.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use sweep::ExtrudeSide;

use crate::common::interval::{iv, p2, p3, v3};
use geom_core::{Affine3, Bounds, Interval, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::test_support::finished;
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanError};

// NOT `common::germ_pair`: the `Interval`-typed twin of that fixture,
// lifted at every literal — see that module's list.
fn cyl(r: f64, h: f64) -> Body<Interval> {
    let tol = Tol::witness();
    let lp = profile::circle(p2(0.0, 0.0), iv(r), tol).unwrap();
    let plane = SketchPlane::xy_at(p3(0.0, 0.0, -h));
    let vp = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(
        &vp,
        Extrusion::Distance {
            depth: iv(2.0 * h),
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body
}

fn spin(b: &Body<Interval>, axis: Vec3<Interval>, angle: f64) -> Body<Interval> {
    topo::transform_rigid(
        b,
        &geom_core::test_support::rigid(
            &Affine3::rotation_about_axis(p3(0.0, 0.0, 0.0), axis, iv(angle)),
            Tol::witness(),
        ),
        Tol::witness(),
    )
    .unwrap()
}

fn x_axis() -> Vec3<Interval> {
    v3(1.0, 0.0, 0.0)
}

fn y_axis() -> Vec3<Interval> {
    v3(0.0, 1.0, 0.0)
}

fn z_axis() -> Vec3<Interval> {
    v3(0.0, 0.0, 1.0)
}

fn repose(b: &Body<Interval>) -> Body<Interval> {
    let r = Affine3::rotation_about_axis(p3(0.0, 0.0, 0.0), v3(1.0, 2.0, 3.0).normalize(), iv(0.7));
    topo::transform_rigid(
        b,
        &geom_core::test_support::rigid(
            &Affine3::from_parts(r.linear, r.translation + v3(0.3, -0.45, 0.6)),
            Tol::witness(),
        ),
        Tol::witness(),
    )
    .unwrap()
}

fn union_err(a: &Body<Interval>, b: &Body<Interval>) -> BooleanError {
    let tol = Tol::witness();
    let (a, b) = (
        finished("operand A", a.clone(), tol),
        finished("operand B", b.clone(), tol),
    );
    topo::union(&a, &b, tol).expect_err("this pose refuses")
}

/// **The re-posed twin's obligation at the CERTIFIED scalar**, which is
/// the `f64` suite's EQUALITY at every ε row but the narrowest, and a
/// narrowly-shaped escape only there.
///
/// The re-pose is a general rotation, so the transformed operand's
/// coordinates are irrational and its enclosures are intervals rather
/// than points. At `eps = 1e-9` (the compiled default) and `eps = 1e-6`
/// those enclosures are far inside the zero band and the twin answers
/// the direct pose's door key for key — measured, so it is asserted as
/// an equality rather than assumed away. At `eps = 1e-12` the
/// enclosures are wider than the band itself, and the certified lane
/// refuses the FIXTURE before the geometry is ever in question: an
/// edge's certification escalates, at the crossing insertion or at the
/// join's edge — measured, on the meeting-axes pose:
///
/// ```text
/// Join(BooleanJoin(Euler(Certification { error: Escalated {
///   check: EndpointStart, sample: 0, cause: Indeterminate {
///     margin: Enclosure { lo: 0.0, hi: 1.5324803774465046e-12 },
///     band: Band { zero: 1e-12, escalate: 1e-11 },
///     predicate: Some("carrier_endpoint_start") } } })))
/// ```
///
/// **Both narrowings matter, and neither is cosmetic.**
///
/// - The escape is gated on the BAND, read from the run's own
///   tolerance, so at the two wider rows an escalation is a FAILURE.
///   A blanket "or escalates" would have accepted an escalation where
///   the measurement says the door is answered exactly.
/// - The escape matches the escalation's SHAPE, not a substring of its
///   rendering. It admits an edge certification escalation like the
///   one above, at the crossing insertion or the join's edge, and it
///   explicitly refuses an
///   escalation of either germ-frame predicate, because those are the
///   very predicates this unit's rows exist to pin: a substring test
///   would have greened a `cc_axes_coplanar` escalation,
///   i.e. the arm going indeterminate, as if it were noise.
fn same_door_or_escalated(direct: &BooleanError, reposed: &BooleanError, what: &str) {
    if crate::common::germ_pair::same_door(direct, reposed) {
        return;
    }
    assert_the_fixture_escape(&format!("{direct:?}"), reposed, what);
}

/// The one escape [`same_door_or_escalated`] admits, against the direct
/// pose's answer `d`: an edge certification escalation at the narrowest
/// band, of no germ-frame predicate, at the crossing insertion or the
/// join's edge.
fn assert_the_fixture_escape(d: &str, reposed: &BooleanError, what: &str) {
    let r = format!("{reposed:?}");
    let band = geom_core::Band::linear(Tol::witness()).expect("a linear band");
    assert!(
        band.zero() <= 1e-12,
        "{what}: at eps with zero band {:e} the re-posed twin must answer the direct \
         pose's door key for key; direct {d}, re-posed {r}",
        band.zero()
    );
    // Each site's own predicate, as measured: the crossing insertion's
    // mapped source, the join edge's start.
    let (source, site_predicate) = match reposed {
        BooleanError::CrossingInsertion { source, .. } => (source, "carrier_matches_mapped_source"),
        BooleanError::Join(j) => match j.error() {
            topo::SplitJoinError::Euler(source) => (source, "carrier_endpoint_start"),
            _ => panic!("{what}: the only admitted escape is a certification escalation; got {r}"),
        },
        _ => panic!(
            "{what}: the only admitted escape is a crossing-insertion or join edge's \
             escalation; got {r}"
        ),
    };
    let topo::EulerOpError::Certification { error } = source else {
        panic!("{what}: the only admitted escape is a certification refusal; got {r}");
    };
    let geom_brep::CertifyError::Escalated { cause, .. } = error else {
        panic!("{what}: the only admitted escape is an ESCALATION; got {r}");
    };
    assert!(
        !matches!(
            cause.predicate,
            // The frame asks the section table's own rows,
            // `cc_axes_coplanar` and `cc_axes_parallel`
            // (`geom_brep::cylinder_axes_coplanar` / `_parallel`).
            Some("cc_axes_coplanar" | "cc_axes_parallel")
        ),
        "{what}: a germ-frame predicate going indeterminate is the defect this unit \
         pins, never an accepted escape; direct {d}, re-posed {r}"
    );
    assert_eq!(
        cause.predicate,
        Some(site_predicate),
        "{what}: the escape admits its site's measured predicate alone; direct {d}, re-posed {r}"
    );
}

/// **The meeting-axes arm.** The pose whose seams sit off the nodes
/// reaches the join at the certified scalar too, and builds there as
/// the `f64` lane does: both nodes are minted as rings and the union
/// comes out. A refusal on the DIRECT pose would say the enclosures
/// decided the lane rather than the geometry, so that half is an
/// equality; the re-posed twin builds too, or takes the fixture escape
/// [`same_door_or_escalated`] writes the reason for.
#[test]
fn the_meeting_axes_pose_builds_at_the_certified_scalar() {
    // Pinned red since PR 5664 (at 1e-12 since PR 5156): the pose escalates on
    // `bool_ruled_region_span` at the certified scalar
    // (`work/charttail/main-red-the-meeting-axes-pose-refuses-on-a-ruled-region-span-since-5664`,
    // `work/germ/main-red-the-meeting-axes-pose-refuses-its-endpoint-at-1e-12`).
    test_utils::pinned_red::row(
        geom_core::Tol::witness().eps(),
        &test_utils::pinned_red::EVERY_ROW,
        "the meeting-axes pose",
        &[
            "the meeting-axes pose builds: Err(Escalated { decision: Containment { decision: None, escalation: Margin }, diag: Indeterminate { margin: MarginDiag(Enclosure { lo: -1.5707963267956369, hi: 4.712388980385431, tag: None, zero: ZeroWord(0) }), terminal_sliver: false",
            "predicate: Some(\"bool_ruled_region_span\")",
        ],
        the_meeting_axes_pose_builds_at_the_certified_scalar_row,
    );
}

/// The row [`the_meeting_axes_pose_builds_at_the_certified_scalar`] runs, under its pin.
fn the_meeting_axes_pose_builds_at_the_certified_scalar_row() {
    let a = spin(&cyl(1.0, 1.2), z_axis(), PI / 4.0);
    let b = spin(
        &spin(&cyl(1.0, 1.2), x_axis(), PI / 2.0),
        y_axis(),
        PI / 4.0,
    );
    let tol = Tol::witness();
    let union = |a: &Body<Interval>, b: &Body<Interval>| {
        topo::union(
            &finished("operand A", a.clone(), tol),
            &finished("operand B", b.clone(), tol),
            tol,
        )
    };
    // Two walls of half-height 1.2 less the bicylinder `16/3`, whole
    // inside both; each node `±x̂` (the re-pose moves them) one vertex
    // where the union's four faces meet.
    let want = 2.0 * PI * 2.4 - 16.0 / 3.0;
    let assert_union = |out: &topo::BooleanBody<Interval>, nodes: [Point3<f64>; 2], what: &str| {
        let m = topo::mass_properties(&out.body, tol, topo::test_support::pinned_resolution(tol))
            .expect("interval props");
        assert!(
            m.enclosure.volume.lo() - m.enclosure.volume_pad <= want
                && want <= m.enclosure.volume.hi() + m.enclosure.volume_pad,
            "{what}: volume {:?} ± {} brackets {want}",
            m.enclosure.volume,
            m.enclosure.volume_pad
        );
        for node in nodes {
            let at: Vec<_> = out
                .body
                .vertex_points()
                .filter(|(_, p)| {
                    let mid = |x: Interval| 0.5 * (x.lo() + x.hi());
                    (Point3::new(mid(p.x), mid(p.y), mid(p.z)) - node).norm() < 1e-9
                })
                .map(|(v, _)| out.body.half_edges().filter(|(_, h)| h.start == v).count())
                .collect();
            assert_eq!(
                at,
                vec![4],
                "{what}: the node {node:?} is one vertex of valence 4"
            );
        }
    };
    let direct = union(&a, &b);
    let Ok(topo::BooleanResult::Body(out)) = &direct else {
        panic!("the meeting-axes pose builds: {direct:?}");
    };
    assert_union(
        out,
        [Point3::new(1.0, 0.0, 0.0), Point3::new(-1.0, 0.0, 0.0)],
        "direct",
    );
    match union(&repose(&a), &repose(&b)) {
        Ok(topo::BooleanResult::Body(out)) => {
            // [`repose`]'s motion, at `f64`.
            let r = Affine3::rotation_about_axis(
                Point3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 2.0, 3.0).normalize(),
                0.7,
            );
            let moved =
                |x: f64| r.transform_point(Point3::new(x, 0.0, 0.0)) + Vec3::new(0.3, -0.45, 0.6);
            assert_union(&out, [moved(1.0), moved(-1.0)], "re-posed");
        }
        Err(e) => assert_the_fixture_escape("a built body", &e, "the meeting-axes pose"),
        Ok(other) => panic!("the re-posed meeting-axes pose: {other:?}"),
    }
}

/// **The skew pose at the BODY level, at the certified scalar.** Slide
/// the pair along the common perpendicular `â₁ × â₂` by the dyadic
/// `0.375`: the crossings are found, the germ pair is minted, and the
/// frame dispatch's skew arm answers it — no arm, not the nodes.
/// The dispatch's verdict itself is pinned at the certified scalar in
/// `topo::boolean::join`'s `frame_dispatch_interval_tests`; this row
/// says a body reaches it there.
#[test]
fn a_skew_pair_stays_off_the_nodes_at_the_certified_scalar() {
    let a = cyl(1.0, 2.0);
    let skew = topo::transform_rigid(
        &spin(&cyl(1.0, 2.0), x_axis(), PI / 2.0),
        &geom_core::RigidMotion::translation(v3(0.375, 0.0, 0.0)).unwrap(),
        Tol::witness(),
    )
    .unwrap();
    let err = union_err(&a, &skew);
    assert!(
        matches!(
            err,
            BooleanError::GermFrameUnsupported {
                a_kind: geom::SurfaceKind::Cylinder,
                b_kind: geom::SurfaceKind::Cylinder,
                ..
            }
        ),
        "a skew pair must reach the frame dispatch's no-arm door, never the nodes: {err:?}"
    );
    // The re-posed twin, on the same obligation as every other row
    // here: a rigid motion moves no contact, so a pose whose direct and
    // re-posed copies disagree is a defect by construction.
    same_door_or_escalated(
        &err,
        &union_err(&repose(&a), &repose(&skew)),
        "the skew pose",
    );
}

/// The Steinmetz pose itself keeps the crossing layer's tangency door
/// at the certified scalar: its seams are ON the pinch points, and a
/// tangency is not a crossing at any order this lane reads.
#[test]
fn the_steinmetz_pose_keeps_the_tangency_door_at_the_certified_scalar() {
    let a = cyl(1.0, 2.0);
    let b = spin(&cyl(1.0, 2.0), x_axis(), PI / 2.0);
    let err = union_err(&a, &b);
    assert!(
        matches!(err, BooleanError::CurvedPierceUnsupported { .. }),
        "{err:?}"
    );
    same_door_or_escalated(
        &err,
        &union_err(&repose(&a), &repose(&b)),
        "the Steinmetz pose",
    );
}
