//! Review R1 probes of PR 5993 (the root refusal split).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::operands::framed_bar;
use crate::revolve_common;
use geom_core::{Point3, Tol, Vec3};
use revolve_common::{axis_y, validated};
use sweep::test_support::finished;
use sweep::{Revolution, revolve};
use topo::{AtRestBody, BooleanError};

fn donut() -> AtRestBody<f64> {
    let vp = validated(vec![revolve_common::donut_profile()]);
    let donut = revolve(&vp, axis_y(), Revolution::Full, Tol::witness())
        .expect("the donut revolves")
        .body;
    finished("the donut", donut, Tol::witness())
}

fn kind(r: &Result<topo::BooleanResult<f64>, BooleanError>) -> String {
    match r {
        Ok(_) => "built".into(),
        Err(BooleanError::Escalated { decision, diag }) => {
            format!("Escalated {decision:?} {:?}", diag.predicate)
        }
        Err(e) => {
            let s = format!("{e:?}");
            s.chars().take(110).collect()
        }
    }
}

/// **A legal bar never ends as a kernel defect at the line × torus door.**
/// A 2 cm bar along `z` through the donut's tube at `x = 2`, tilted by
/// `δ` toward the axis: each long edge crosses the tube twice, and its
/// quartic's odd coefficient `q̂` is small but definite, so the
/// resolvent's one real root `z ≈ q̂²/c₁` falls in the zero band
/// (`work/inside/torus-split-lead-escalates-a-legitimately-small-resolvent-root.md`).
/// On the merge base of PR 5993 the union refused
/// `CurvedPierceUnsupported` at every tilt here; on its head it ends
/// `ClassificationInvariant` ("a quartic's resolvent read no positive
/// largest root …"), the kernel-defect ending, on legal input, at every
/// ε (1e-6: δ ≥ 10^-4.5; 1e-9: 10^-7.5 ≤ δ ≤ 10^-4.5; 1e-12: δ ≤ 10^-6).
#[test]
fn review_r1_a_bar_square_to_the_torus_axis_is_no_kernel_defect() {
    let d = donut();
    for exp in [-7.0, -6.0, -5.0, -4.5, -3.0] {
        let delta = 10f64.powf(exp);
        let bar = framed_bar(
            Point3::new(2.0, 0.1, 0.0),
            Vec3::new(0.0, delta, 1.0),
            -3.0,
            3.0,
            0.02,
        );
        let bar = finished("the bar", bar, Tol::witness());
        let u = topo::union(&d, &bar, Tol::witness());
        assert!(
            !matches!(u, Err(BooleanError::ClassificationInvariant { .. })),
            "δ = 1e{exp}: a legal bar ends as a kernel defect: {}",
            kind(&u)
        );
    }
}

/// A bar all but square to the donut's axis, through the tube once:
/// each long edge is a line × torus span with two roots whose quartic's
/// odd coefficient is small. Prints the union's outcome per tilt.
#[test]
#[ignore = "review probe"]
fn review_r1_near_square_bar_through_the_tube() {
    let d = donut();
    for exp in [
        -9.0, -8.0, -7.5, -7.0, -6.5, -6.0, -5.5, -5.0, -4.5, -4.0, -3.0, -2.0,
    ] {
        let delta = 10f64.powf(exp);
        let bar = framed_bar(
            Point3::new(2.0, 0.1, 0.0),
            Vec3::new(0.0, delta, 1.0),
            -3.0,
            3.0,
            0.02,
        );
        let bar = finished("the bar", bar, Tol::witness());
        let u = topo::union(&d, &bar, Tol::witness());
        let e = std::env::var("CAD_TOLERANCE_EPS").unwrap_or_default();
        println!("eps={e} delta=1e{exp}: {}", kind(&u));
    }
}

fn cyl_spun(spin: f64, about_x: bool) -> topo::Body<f64> {
    use geom_core::{Affine3, Point2};
    use profile::{Profile, SketchPlane};
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(0.0, 0.0), 1.0, tol).unwrap();
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, -2.0));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    let body = sweep::extrude(
        &profile,
        sweep::Extrusion::Distance {
            depth: 4.0,
            side: sweep::ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body;
    let rig = |body: &topo::Body<f64>, axis: Vec3<f64>, angle: f64| {
        topo::transform_rigid(
            body,
            &geom_core::test_support::rigid(
                &Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), axis, angle),
                tol,
            ),
            tol,
        )
        .unwrap()
    };
    // Spun about its own axis first: the solid is unchanged, its seam moves.
    let body = rig(&body, Vec3::new(0.0, 0.0, 1.0), spin);
    if about_x {
        rig(&body, Vec3::new(1.0, 0.0, 0.0), core::f64::consts::PI / 2.0)
    } else {
        body
    }
}

/// The Steinmetz pair (`verbs_cylcyl_probe`), each cylinder spun about
/// its own axis: one solid pair, its seams placed elsewhere. Prints the
/// union's outcome per spin.
#[test]
#[ignore = "review probe"]
fn review_r1_steinmetz_by_seam_placement() {
    let tol = Tol::witness();
    for (sa, sb) in [
        (0.0, 0.0),
        (0.7, 0.0),
        (0.0, 0.7),
        (0.7, 1.1),
        (core::f64::consts::FRAC_PI_2, 0.0),
    ] {
        let a = finished("A", cyl_spun(sa, false), tol);
        let b = finished("B", cyl_spun(sb, true), tol);
        let u = topo::union(&a, &b, tol);
        println!("spin A {sa}, B {sb}: {}", kind(&u));
    }
}

/// **A seam the kernel placed is not the user's lever.** The Steinmetz
/// pair (two unit cylinders, axes `z` and `y`, length 4): with each
/// cylinder spun about its own axis, the same two solids, and their
/// seams away from the tangent rulings, the union builds at the closed
/// form `8π − 16/3`. With the seams where the extrusion puts them, one
/// seam ruling of each is exactly tangent to the partner wall. On PR
/// 5993's merge base that refused `CurvedPierceUnsupported` (the
/// kernel's frontier); on its head it escalates
/// `WallRoots(Discriminant)`, "move the parts so the edge clearly
/// crosses the wall or clearly misses it": a lever on an edge the user
/// never drew, for a pair the kernel builds once the seam is elsewhere.
#[test]
fn review_r1_a_seam_touch_is_not_the_users_lever() {
    let tol = Tol::witness();
    let spun = topo::union(
        &finished("A", cyl_spun(0.7, false), tol),
        &finished("B", cyl_spun(1.1, true), tol),
        tol,
    )
    .expect("the spun Steinmetz pair builds");
    let v = topo::test_support::pinned_volume(
        "the spun union",
        &spun.body().expect("a body").body,
        tol,
    )
    .volume;
    let want = 8.0 * core::f64::consts::PI - 16.0 / 3.0;
    // Within the band's reach over the walls: `1e-6` at the default ε.
    assert!(
        (v - want).abs() < 1e3 * tol.eps(),
        "the spun union: {v} vs {want}"
    );
    let seamed = topo::union(
        &finished("A", cyl_spun(0.0, false), tol),
        &finished("B", cyl_spun(0.0, true), tol),
        tol,
    );
    assert!(
        !matches!(seamed, Err(BooleanError::Escalated { .. })),
        "the seam's touch offers the user a lever: {}",
        kind(&seamed)
    );
}

/// Review 3 probe (PR 5993): where a `CurvedPierceEscalated` says it
/// stands, read back from the operands: the edge's curve in `operand`,
/// the face's surface in the other, and how far the edge's ends lie
/// from that surface's carrier.
pub(crate) fn review3_locate(
    label: &str,
    a: &topo::Body<f64>,
    b: &topo::Body<f64>,
    err: &BooleanError,
) {
    let BooleanError::CurvedPierceEscalated {
        operand,
        face,
        edge,
        decision,
        diag,
    } = err
    else {
        println!("{label}: not located: {err:?}");
        return;
    };
    let (x, y) = match operand {
        topo::Operand::A => (a, b),
        topo::Operand::B => (b, a),
    };
    let e = x.get_edge(*edge).expect("the edge is the operand's");
    let c = x.get_curve_geom(e.curve).expect("its curve");
    let f = y.get_face(*face).expect("the face is the other's");
    let s = y.get_surface(f.surface).expect("its surface");
    let short = |s: String| s.chars().take(90).collect::<String>();
    println!(
        "{label}: {operand:?} {decision:?} {:?}\n  edge {edge:?}: {}\n  face {face:?}: {}",
        diag.predicate,
        short(format!("{c:?}")),
        short(format!("{s:?}"))
    );
}

#[test]
#[ignore = "review probe"]
fn review3_where_the_escalations_stand() {
    let tol = Tol::witness();
    let d = donut();
    for exp in [-8.0, -5.5, -5.0, -4.0] {
        let bar = framed_bar(
            Point3::new(2.0, 0.1, 0.0),
            Vec3::new(0.0, 10f64.powf(exp), 1.0),
            -3.0,
            3.0,
            0.02,
        );
        let bar = finished("the bar", bar, tol);
        for (name, r, a, b) in [
            ("donut ∪ bar", topo::union(&d, &bar, tol), &*d, &*bar),
            ("bar ∖ donut", topo::subtract(&bar, &d, tol), &*bar, &*d),
        ] {
            if let Err(e) = &r {
                review3_locate(&format!("δ 1e{exp} {name}"), a, b, e);
            }
        }
    }
}

/// **Review 3 (PR 5993): the seamed Steinmetz pair keeps the frontier,
/// located or not.** [`review_r1_a_seam_touch_is_not_the_users_lever`]
/// rejects only `Escalated`, but the head routes every span-path door
/// escalation through `CurvedPierceEscalated`, so a seam touch read in
/// the gap would pass it. Its seam ruling is exactly tangent to the
/// partner wall, a decided Zero, which under A2 is the frontier.
#[test]
fn review3_the_seam_touch_keeps_the_frontier() {
    let tol = Tol::witness();
    let seamed = topo::union(
        &finished("A", cyl_spun(0.0, false), tol),
        &finished("B", cyl_spun(0.0, true), tol),
        tol,
    );
    assert!(
        matches!(seamed, Err(BooleanError::CurvedPierceUnsupported { .. })),
        "the seam's decided touch keeps the frontier: {}",
        kind(&seamed)
    );
}
