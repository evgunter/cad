//! **Cone × torus off the axis** (`geom_brep::ConeTorusPose`,
//! `ssi::cone_torus_ssi`): the pose's readings against oracles that do
//! not read them.
//!
//! - A **tangent** pose is built in closed form: a torus whose core
//!   passes through the point `r` along a generator's normal, square to
//!   it there, touches the cone at the generator's point, where both
//!   residuals vanish and the normals are opposite. The gap decides it
//!   `Zero`, and the same torus moved off along the normal reads clear.
//! - The **apex** on the torus is read against the torus's own residual
//!   round the cone's two small circles about the apex: it changes sign
//!   there exactly where the cone crosses the torus at the apex, a node,
//!   and holds one sign where the apex is an isolated touch.
//! - A **crossing** pose (the meridian triangle's cone against its
//!   torus) and a **clear** one are marched, every sampled point of each
//!   certified branch read on both surfaces by the oracle's own forms,
//!   and the clear one's torus sampled whole against the cone.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/pair_section/cone_torus.rs",
    "crates/geom-brep/src/pair_section/gap.rs",
    "crates/geom-brep/tests/shared/"
];

use core::f64::consts::{FRAC_PI_4, FRAC_PI_6, TAU};
use geom::Surface;
use geom_brep::ssi::{self, SsiError};
use geom_brep::{
    ApexContact, Components, ConeTorusPose, KtStance, PairPose, SectionError, SectionRead, TorusGap,
};
use geom_core::{Point3, Vec3};

use crate::shared::tol::band;

fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

fn cone(apex: Point3<f64>, axis: Vec3<f64>, half_angle: f64) -> Surface<f64> {
    let axis = axis.normalize();
    Surface::Cone {
        frame: geom::Frame::new(apex, axis, axis.orthonormal_basis().0),
        half_angle,
    }
}

fn torus(center: Point3<f64>, axis: Vec3<f64>, big: f64, small: f64) -> Surface<f64> {
    let axis = axis.normalize();
    Surface::Torus {
        frame: geom::Frame::new(center, axis, axis.orthonormal_basis().0),
        major_radius: big,
        minor_radius: small,
    }
}

/// The oracle's torus residual: distance from the core circle less `r`.
fn torus_distance(t: &Surface<f64>, q: Point3<f64>) -> f64 {
    let Surface::Torus {
        frame,
        major_radius,
        minor_radius,
    } = *t
    else {
        unreachable!()
    };
    let (c, a) = (frame.origin(), frame.axis().normalize());
    let w = q - c;
    let h = w.dot(a);
    let rho = (w - a * h).norm();
    ((rho - major_radius).powi(2) + h.powi(2)).sqrt() - minor_radius
}

/// The oracle's double-cone residual, its elevation
/// `ρ cos α − |h| sin α`, the signed distance from the double cone.
fn cone_elevation(k: &Surface<f64>, q: Point3<f64>) -> f64 {
    let Surface::Cone { frame, half_angle } = *k else {
        unreachable!()
    };
    let (apex, d) = (frame.origin(), frame.axis().normalize());
    let w = q - apex;
    let h = w.dot(d);
    let rho = (w - d * h).norm();
    rho * half_angle.cos() - h.abs() * half_angle.sin()
}

fn decided(k: &Surface<f64>, t: &Surface<f64>) -> ConeTorusPose<f64> {
    ConeTorusPose::decide(k, t, band())
        .expect("a cone and a torus")
        .unwrap_or_else(|e| panic!("the pose refused: {e:?}"))
}

fn gap(pose: &ConeTorusPose<f64>) -> TorusGap<f64> {
    match pose.stance() {
        KtStance::Apart(gap) => gap,
        other => panic!("not an apart pose: {other:?}"),
    }
}

/// The tangent fixture: cone apex at the origin about `z`, `α = 30°`;
/// the generator point `x` at slant `3` in the `xz` half-plane, its
/// outward normal `n`; the torus `R = 2`, `r = 1/2` whose core passes
/// through `x + (r + lift)·n`, square there to `n` and to `y`, its centre
/// `R` back along `n`.
fn tangent_fixture(lift: f64) -> (Surface<f64>, Surface<f64>, Point3<f64>, Vec3<f64>) {
    let alpha = FRAC_PI_6;
    let (s, c) = alpha.sin_cos();
    let x = p(3.0 * s, 0.0, 3.0 * c);
    let n = Vec3::new(c, 0.0, -s);
    let (big, small) = (2.0, 0.5);
    let core = x + n * (small + lift);
    let center = core - n * big;
    let axis = n.cross(Vec3::unit_y());
    (
        cone(Point3::origin(), Vec3::unit_z(), alpha),
        torus(center, axis, big, small),
        x,
        n,
    )
}

/// **A torus built tangent to the cone reads `Zero` on its gap**, in
/// either order, and the door refuses it as the pair's tangency before
/// any rung; lifted off along the normal it reads transverse. The
/// oracle checks the contact itself: both residuals vanish at `x` and
/// the torus's normal there is the cone's, reversed.
#[test]
fn a_torus_built_tangent_to_the_cone_reads_its_gap_zero() {
    let (k, t, x, n) = tangent_fixture(0.0);
    assert!(
        torus_distance(&t, x).abs() < 1e-14 && cone_elevation(&k, x).abs() < 1e-14,
        "the oracle: the contact point lies on both"
    );
    for pose in [decided(&k, &t), decided(&t, &k)] {
        assert!(
            matches!(gap(&pose), TorusGap::Tangent { .. }),
            "the tangent torus reads its gap Zero: {:?}",
            pose.stance()
        );
        assert!(pose.tangency().is_some(), "the pose names its tangency");
    }
    let domain = PairPose::ConeTorus(decided(&k, &t)).domain();
    let got = ssi::cone_torus_ssi(&k, &t, domain, band());
    assert!(
        matches!(
            got,
            Err(SsiError::PairTangent {
                pair: ssi::TangentPair::ConeTorus,
                ..
            })
        ),
        "the door refuses the tangency before any rung: {got:?}"
    );
    let (k, t, _, _) = tangent_fixture(0.05);
    assert!(
        (torus_distance(&t, x) - 0.05).abs() < 1e-14 && cone_elevation(&k, x + n * 0.5) > 0.0,
        "the oracle: the lifted torus stands its lift off the contact"
    );
    assert!(
        matches!(gap(&decided(&k, &t)), TorusGap::Transverse),
        "the lifted torus reads transverse"
    );
}

/// How the torus's residual changes sign round the cone's two circles of
/// slant `slant` about the apex: the number of sign changes on each.
fn sign_changes_about_apex(k: &Surface<f64>, t: &Surface<f64>, slant: f64) -> [usize; 2] {
    let Surface::Cone { frame, half_angle } = *k else {
        unreachable!()
    };
    let (apex, d) = (frame.origin(), frame.axis().normalize());
    let (u, v) = d.orthonormal_basis();
    let (s, c) = half_angle.sin_cos();
    let mut changes = [0; 2];
    for (nappe, sense) in [1.0, -1.0].into_iter().enumerate() {
        let at = |k: usize| {
            let phi = TAU * k as f64 / 4096.0;
            let q = apex + d * (sense * slant * c) + (u * phi.cos() + v * phi.sin()) * (slant * s);
            torus_distance(t, q).signum()
        };
        changes[nappe] = (0..4096).filter(|&k| at(k) != at(k + 1)).count();
    }
    changes
}

/// **An apex on the torus is a node where the torus crosses the cone
/// there, a touch where it does not.** Torus `R = 2`, `r = 1` about `z`,
/// the apex on its outer equator `(3, 0, 0)`: a cone about `z` (its
/// rulings in the tangent plane `x = 3`) is a node, one about `x` (the
/// tangent plane meets it at the apex alone) a touch. The oracle: the
/// torus's residual round the cone's small circles about the apex
/// changes sign four times in all at the node, never at the touch. The
/// section refuses typed, naming which, and the door does not march it.
#[test]
fn an_apex_on_the_torus_reads_node_or_touch_as_the_torus_crosses_it() {
    let t = torus(Point3::origin(), Vec3::unit_z(), 2.0, 1.0);
    let apex = p(3.0, 0.0, 0.0);
    for (name, axis, node) in [
        ("node", Vec3::unit_z(), true),
        ("touch", Vec3::unit_x(), false),
    ] {
        let k = cone(apex, axis, FRAC_PI_6);
        let changes = sign_changes_about_apex(&k, &t, 1e-3);
        assert_eq!(
            changes.iter().sum::<usize>() == 4,
            node,
            "{name}: the oracle's sign changes {changes:?}"
        );
        let pose = decided(&k, &t);
        let read = match pose.stance() {
            KtStance::Apex(ApexContact::Node { .. }) => true,
            KtStance::Apex(ApexContact::Touch { .. }) => false,
            other => panic!("{name}: not an apex pose: {other:?}"),
        };
        assert_eq!(read, node, "{name}: the pose's form");
        assert!(
            pose.tangency().is_none(),
            "{name}: no apex contact is a tangency"
        );
        let section = geom_brep::pair_section(&k, &t, None, None::<geom_core::NoLane>, band());
        assert!(
            matches!(section, Err(SectionError::RoutesToGeneralRung { .. })),
            "{name}: the section refuses typed: {section:?}"
        );
        let got = ssi::cone_torus_ssi(&k, &t, PairPose::ConeTorus(pose).domain(), band());
        assert!(
            matches!(got, Err(SsiError::UnsupportedCertificate { .. })),
            "{name}: the door does not march it: {got:?}"
        );
    }
}

/// **A coaxial cone is the closed form's**: the section value reads it
/// elsewhere, and the door does not march it.
#[test]
fn a_coaxial_cone_is_read_elsewhere() {
    let t = torus(p(0.0, 0.0, 1.0), Vec3::unit_z(), 2.0, 0.5);
    let k = cone(Point3::origin(), Vec3::unit_z(), FRAC_PI_4);
    let pose = decided(&k, &t);
    assert!(
        matches!(pose.stance(), KtStance::Coaxial { .. }),
        "{:?}",
        pose.stance()
    );
    let read = geom_brep::pair_section(&k, &t, None, None::<geom_core::NoLane>, band());
    assert!(matches!(read, Ok(SectionRead::Elsewhere(_))), "{read:?}");
    let got = ssi::cone_torus_ssi(&k, &t, PairPose::ConeTorus(pose).domain(), band());
    assert!(matches!(got, Err(SsiError::WrongLane { .. })), "{got:?}");
}

/// The worst of each oracle residual over every certified branch.
fn worst(out: &geom_brep::SsiOutcome, k: &Surface<f64>, t: &Surface<f64>) -> [f64; 2] {
    let mut worst = [0.0f64; 2];
    for branch in &out.branches {
        let (t0, t1) = branch.params;
        for i in 0..=256 {
            let q = branch.carrier.eval(t0 + (t1 - t0) * f64::from(i) / 256.0);
            worst[0] = worst[0].max(cone_elevation(k, q).abs());
            worst[1] = worst[1].max(torus_distance(t, q).abs());
        }
    }
    worst
}

/// **The meridian triangle's cone crosses its torus, marched and
/// certified.** The torus `R = 2`, `r = 1` about `y`; the cone of the
/// triangle's edge from `(1, 0)` to `(1/2, 1/2)` revolved about the line
/// through `(2, 0, 0)` along `z`: apex `(2, 0, 1)`, `α = 45°`, its rim
/// circle the torus's meridian circle at `(2, 0, 0)`. The pose is apart
/// and transverse, the section marched; every branch certifies, its
/// samples on both surfaces by the oracle's forms, and one of them is
/// that meridian circle.
#[test]
fn the_meridian_triangles_cone_crosses_its_torus_certified() {
    let t = torus(Point3::origin(), Vec3::unit_y(), 2.0, 1.0);
    let k = cone(p(2.0, 0.0, 1.0), -Vec3::unit_z(), FRAC_PI_4);
    let pose = decided(&k, &t);
    assert!(
        matches!(gap(&pose), TorusGap::Transverse),
        "{:?}",
        pose.stance()
    );
    let read = geom_brep::pair_section(&k, &t, None, None::<geom_core::NoLane>, band());
    let Ok(SectionRead::Built(section)) = read else {
        panic!("built: {read:?}")
    };
    assert!(matches!(section.components(), Components::Marched));
    {
        let out = ssi::cone_torus_ssi(&k, &t, section.pose().domain(), band())
            .unwrap_or_else(|e| panic!("the door marches the crossing: {e}"));
        eprintln!(
            "the meridian cone: {} branches, {} cells examined",
            out.branches.len(),
            out.exhaustiveness.examined
        );
        assert!(!out.branches.is_empty(), "the cone crosses the torus");
        let [wk, wt] = worst(&out, &k, &t);
        assert!(
            wk <= band().zero() && wt <= band().zero(),
            "sampled residuals {wk:e} (cone), {wt:e} (torus) past the band"
        );
        let on_meridian = |q: Point3<f64>| {
            q.z.abs() <= band().zero()
                && ((q - p(2.0, 0.0, 0.0)).norm() - 1.0).abs() <= band().zero()
        };
        assert!(
            out.branches.iter().any(|b| {
                let (t0, t1) = b.params;
                (0..=64).all(|i| on_meridian(b.carrier.eval(t0 + (t1 - t0) * f64::from(i) / 64.0)))
            }),
            "one branch is the meridian circle"
        );
    }
}

/// **A cone clear of the torus finds nothing, accounted.** A tilted cone
/// standing off a torus: the oracle samples the whole torus and finds
/// the cone's elevation one-signed and clear of zero; the pose reads
/// transverse, and the door marches no branch.
#[test]
fn a_cone_clear_of_the_torus_finds_nothing() {
    let t = torus(Point3::origin(), Vec3::unit_z(), 2.0, 0.5);
    let k = cone(p(8.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 1.0), FRAC_PI_6);
    let Surface::Torus { frame, .. } = t else {
        unreachable!()
    };
    let (u, v) = (frame.u_ref(), frame.axis().cross(frame.u_ref()));
    let least = (0..256)
        .flat_map(|i| (0..64).map(move |j| (i, j)))
        .map(|(i, j)| {
            let (phi, theta) = (TAU * f64::from(i) / 256.0, TAU * f64::from(j) / 64.0);
            let radial = u * phi.cos() + v * phi.sin();
            let q = Point3::origin()
                + radial * (2.0 + 0.5 * theta.cos())
                + frame.axis() * (0.5 * theta.sin());
            cone_elevation(&k, q)
        })
        .fold(f64::INFINITY, f64::min);
    assert!(least > 1.0, "the oracle: the torus stands clear ({least})");
    let pose = decided(&k, &t);
    assert!(matches!(gap(&pose), TorusGap::Transverse));
    let out = ssi::cone_torus_ssi(&k, &t, PairPose::ConeTorus(pose).domain(), band())
        .unwrap_or_else(|e| panic!("{e}"));
    assert!(out.branches.is_empty(), "no branch");
}

/// **The table routes the pair to its door.**
#[test]
fn the_table_routes_cone_torus_to_its_door() {
    let arm = geom_brep::route(geom::SurfaceKind::Cone, geom::SurfaceKind::Torus);
    assert!(arm.implemented, "{}", arm.note);
    assert!(arm.note.contains("cone_torus_ssi"), "{}", arm.note);
}
