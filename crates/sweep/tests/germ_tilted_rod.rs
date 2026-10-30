//! **A tilted rod entering the half donut's cap and poking a lens out
//! of the inner equator.** No event lies on the lens's rim, so the
//! crossings the rod makes through the cap are the only ones the join
//! sees, and the body it builds drops the lens. That body is valid at
//! tier 3 and passes the volume backstop, so the section certificate is
//! what refuses it. The rod's axis turns in the torus's equatorial
//! plane, square to its axis at every tilt, so the certificate's
//! square-wall arm classifies each torus × rod-wall pair: it certifies
//! the lens's rim a loop interior to both faces (R-loop) where the rod
//! reaches past the inner equator, and clears every pair where it stops
//! short, so those poses answer.
//!
//! Other spins of the same rod, and steeper tilts, refuse earlier, in
//! the join, before any body is built.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::germ_pair;
use crate::common::oracles::volume_slack;
use crate::revolve_common::{self, axis_y, validated};

use core::f64::consts::FRAC_PI_2;
use geom::SurfaceKind;
use geom_core::{Point3, Tol, Vec3};
use sweep::test_support::finished;
use sweep::{Revolution, revolve};
use topo::{AtRestBody, Body, BooleanError, BooleanOp as Op, BooleanResult, FaceKey};

fn half_donut() -> AtRestBody<f64> {
    let vp = validated(vec![revolve_common::donut_profile()]);
    let half = revolve(
        &vp,
        axis_y(),
        Revolution::Partial(core::f64::consts::PI),
        Tol::witness(),
    )
    .expect("the half donut revolves")
    .body;
    finished("the half donut", half, Tol::witness())
}

/// The rod's axis direction at tilt `beta` about `y`.
fn dir(beta: f64) -> Vec3<f64> {
    Vec3::new(-beta.sin(), 0.0, -beta.cos())
}

/// The rod: radius 0.15, its axis from `(1.8, 0, 0)` along [`dir`] over
/// `t ∈ [−0.3, 1.4]`, spun `spin` about its own axis (which moves only
/// its seam).
fn rod(beta: f64, spin: f64) -> AtRestBody<f64> {
    let c = germ_pair::spin(&germ_pair::cyl(0.15, 0.85), Vec3::unit_z(), spin);
    let c = germ_pair::spin(&c, Vec3::unit_y(), beta);
    let centre = Vec3::new(1.8, 0.0, 0.0) + dir(beta) * 0.55;
    let moved = topo::transform_rigid(
        &c,
        &geom_core::RigidMotion::translation(centre).unwrap(),
        Tol::witness(),
    )
    .expect("the rod moves");
    finished("the rod", moved, Tol::witness())
}

fn in_solid(b: &Body<f64>, q: Point3<f64>) -> Option<bool> {
    let band = geom_core::Band::linear(Tol::witness()).expect("the run's band");
    match topo::point_in_solid(b, q, band, Tol::witness()) {
        Ok(topo::SolidContainment::In) => Some(true),
        Ok(topo::SolidContainment::Out) => Some(false),
        _ => None,
    }
}

fn kind_of(b: &Body<f64>, f: FaceKey) -> Option<SurfaceKind> {
    topo::query::face_surface_kind(b, f)
}

type Run = (
    Op,
    bool,
    &'static str,
    Result<BooleanResult<f64>, BooleanError>,
);

/// Every op, in both operand orders of ∖ and ∩; the flag says whether
/// the rod is operand A.
fn every_op(h: &AtRestBody<f64>, c: &AtRestBody<f64>) -> [Run; 5] {
    let t = Tol::witness();
    [
        (Op::Union, false, "h ∪ c", topo::union(h, c, t)),
        (Op::Intersect, false, "h ∩ c", topo::intersect(h, c, t)),
        (Op::Intersect, true, "c ∩ h", topo::intersect(c, h, t)),
        (Op::Subtract, false, "h ∖ c", topo::subtract(h, c, t)),
        (Op::Subtract, true, "c ∖ h", topo::subtract(c, h, t)),
    ]
}

fn outcome(r: &Result<BooleanResult<f64>, BooleanError>) -> String {
    match r {
        Err(e) => format!("{e:?}"),
        Ok(r) => format!("answered {:?}", r.body().map(|b| b.kind)),
    }
}

/// The torus × rod-wall verdicts of the section report at a pose.
fn torus_wall(h: &AtRestBody<f64>, c: &AtRestBody<f64>) -> Vec<String> {
    topo::test_support::section_report(Op::Union, h, c, Tol::witness())
        .expect("the reduction runs")
        .into_iter()
        .filter(|(fa, fb, _)| {
            kind_of(h, *fa) == Some(SurfaceKind::Torus)
                && kind_of(c, *fb) == Some(SurfaceKind::Cylinder)
        })
        .map(|(.., v)| v)
        .collect()
}

/// **The rod that pokes the lens refuses every op on the certified
/// loop**, in both operand orders of ∖ and ∩ (tilt 0.5, spin π/2): the
/// lens's rim is the one torus × rod-wall pair the certificate does not
/// clear, and it reads `Err(Loop)`, a loop certified inside both faces;
/// every other pair clears. Red against a lens pair cleared (its witness
/// read `Out`, or its loop flagged essential): every op then answers a
/// `Seamed` body valid at tier 3 that the volume backstop passes, and
/// wrong (∪ drops the lens, `c ∖ h` keeps only the stub above the cap).
#[test]
fn the_rod_that_pokes_the_lens_refuses_every_op_on_its_certified_loop() {
    let (h, c) = (half_donut(), rod(0.5, FRAC_PI_2));
    for (op, rod_first, what, r) in every_op(&h, &c) {
        let (kind, other) = if rod_first {
            (SurfaceKind::Cylinder, SurfaceKind::Torus)
        } else {
            (SurfaceKind::Torus, SurfaceKind::Cylinder)
        };
        match r {
            Err(BooleanError::CurvedPairUnsupported {
                op: Some(o),
                site,
                kind: k,
                other_kind: ok,
                ..
            }) => assert_eq!(
                (site, o, k, ok),
                (topo::PairRefusalSite::InteriorLoopGuard, op, kind, other),
                "{what}"
            ),
            r => panic!("{what}: not refused at the certificate: {}", outcome(&r)),
        }
    }
    let verdicts = torus_wall(&h, &c);
    let loops = verdicts.iter().filter(|v| *v == "Err(Loop)").count();
    assert!(
        loops == 1
            && verdicts
                .iter()
                .all(|v| v == "Err(Loop)" || v.starts_with("Ok(")),
        "the torus × rod-wall pairs: {verdicts:?}"
    );
}

/// **The rod short of the inner equator answers every op in closed
/// form** (tilts 0.3 and 0.4, every spin): it comes no nearer the donut's
/// axis than `1.8 cos β − 0.15 > 1.5`, so it pokes out nowhere, and below
/// the cap (`z = 0`) it lies wholly inside the tube. The plane through
/// its axis point `t = 0` leaves `πr²·1.4` of it inside the half donut
/// (`π²/2`), of its `πr²·1.7`. Probes: on the axis above the cap (the
/// rod only), on it below the cap (both), and in the far half of the
/// tube (the half donut only), each answered. Each torus × rod-wall pair
/// clears.
/// Refused on reach before the square-wall arm.
#[test]
fn the_rod_short_of_the_inner_equator_answers_every_op() {
    use core::f64::consts::PI;
    let h = half_donut();
    let rod_area = PI * 0.15 * 0.15;
    let (vh, vc, vo) = (PI * PI / 2.0, rod_area * 1.7, rod_area * 1.4);
    for (beta, spin) in [0.0, 0.5, 1.0, FRAC_PI_2, 2.5]
        .iter()
        .flat_map(|&s| [(0.3, s), (0.4, s)])
    {
        let c = rod(beta, spin);
        let pose = format!("β {beta}, spin {spin:.4}");
        let axis = |t: f64| Point3::new(1.8, 0.0, 0.0) + dir(beta) * t;
        let probes = [axis(-0.15), axis(0.7), Point3::new(-2.0, 0.0, -0.1)];
        for (op, rod_first, what, r) in every_op(&h, &c) {
            let (want, inside) = match (op, rod_first) {
                (Op::Union, _) => (vh + vc - vo, [true, true, true]),
                (Op::Intersect, _) => (vo, [false, true, false]),
                (Op::Subtract, false) => (vh - vo, [false, false, true]),
                (Op::Subtract, true) => (vc - vo, [true, false, false]),
            };
            let r = r.unwrap_or_else(|e| panic!("{what} at {pose}: {e:?}"));
            let b = r
                .body()
                .unwrap_or_else(|| panic!("{what} at {pose}: empty"));
            assert_eq!(
                topo::validate_geometric(&b.body, Tol::witness()),
                Ok(()),
                "{what} at {pose}: tier 3"
            );
            let m = topo::mass_properties(
                &b.body,
                Tol::witness(),
                topo::test_support::pinned_resolution(Tol::witness()),
            )
            .expect("the volume integrates");
            assert!(
                (m.enclosure.volume - want).abs() <= volume_slack(&m.enclosure, want),
                "{what} at {pose}: volume {} against the closed form {want}",
                m.enclosure.volume
            );
            for (q, inside) in probes.into_iter().zip(inside) {
                assert_eq!(
                    in_solid(&b.body, q),
                    Some(inside),
                    "{what} at {pose}, {q:?}"
                );
            }
        }
        let verdicts = torus_wall(&h, &c);
        assert!(
            !verdicts.is_empty() && verdicts.iter().all(|v| v.starts_with("Ok(")),
            "the torus × rod-wall pairs at {pose}: {verdicts:?}"
        );
    }
}

/// Where one of the other spins' runs stops, by refusal variant.
fn stop_word(r: &Result<BooleanResult<f64>, BooleanError>) -> String {
    use geom_brep::PropsError;
    use topo::{MassPropsError, ValidationError};
    match r {
        Err(BooleanError::GermFrameUnsupported { .. }) => "frame".to_owned(),
        Err(BooleanError::CurvedBooleanUnsupported {
            kind: SurfaceKind::Torus,
            ..
        }) => "curved join".to_owned(),
        Err(BooleanError::JoinCarrierUnsupported {
            carrier: geom::CurveKind::Nurbs,
            ..
        }) => "spline join".to_owned(),
        Err(BooleanError::ResultInvalid { errors }) => match errors.as_slice() {
            [
                ValidationError::VolumeUncomputable {
                    source: MassPropsError::RingOnCurvedFace { .. },
                    ..
                },
            ] => "ring volume".to_owned(),
            [
                ValidationError::VolumeUncomputable {
                    source:
                        MassPropsError::Face {
                            source: PropsError::QuadratureUnsupported { .. },
                            ..
                        },
                    ..
                },
            ] => "quadrature volume".to_owned(),
            other => format!("result {other:?}"),
        },
        other => outcome(other),
    }
}

/// The other spins' stops by pose and op ([`every_op`]'s order),
/// measured alike at the compiled ε, 1e-6 and 1e-12: the edge join on
/// the wall's spline chord, or the result gate's volume, its quadrature
/// on a face the chords bound (a projected image, a conic trim or a
/// spiric's lifted image) or a ring left on a curved face.
const OTHER_SPINS_STOPS: [[&str; 5]; 8] = [
    [
        "spline join",
        "quadrature volume",
        "quadrature volume",
        "ring volume",
        "spline join",
    ],
    [
        "spline join",
        "quadrature volume",
        "quadrature volume",
        "ring volume",
        "spline join",
    ],
    [
        "spline join",
        "quadrature volume",
        "quadrature volume",
        "ring volume",
        "spline join",
    ],
    [
        "spline join",
        "quadrature volume",
        "quadrature volume",
        "ring volume",
        "spline join",
    ],
    [
        "spline join",
        "quadrature volume",
        "quadrature volume",
        "quadrature volume",
        "spline join",
    ],
    [
        "spline join",
        "quadrature volume",
        "quadrature volume",
        "quadrature volume",
        "spline join",
    ],
    [
        "spline join",
        "quadrature volume",
        "quadrature volume",
        "quadrature volume",
        "spline join",
    ],
    [
        "spline join",
        "quadrature volume",
        "quadrature volume",
        "quadrature volume",
        "spline join",
    ],
];

/// **The other spins, and the steeper tilts, refuse typed** before any
/// body is built, on every op, each at its pinned door
/// ([`OTHER_SPINS_STOPS`], the same at every ε row): each wall germ takes its site's foot on the
/// carried section, which each op marches once, and the run passes the
/// join to the edge join on the wall's closed spline chord (`JoinCarrierUnsupported`,
/// `work/splicecarrier/joining-a-spline-carrier-is-unbuilt.md`) or the
/// result gate's volume (a face the fitted chords bound, a conic trim, a
/// spiric's lifted image, or a ring left on a curved face; SPIRICARM's
/// `a-projected-boundary-on-an-analytic-chart-has-no-volume-lane`).
#[test]
fn other_spins_and_steeper_tilts_refuse_at_their_germs() {
    // Pinned red at 1e-6 since PR 6011: all eight poses, β 0.5 spin 0
    // to β 0.7 spin π/2, refuse their germ section's winding
    // (`work/sect/main-red-five-torus-section-rows-refuse-at-1e-6-since-components-by-foot.md`).
    test_utils::pinned_red::row(
        geom_core::Tol::witness().eps(),
        &[1e-6],
        "the other spins and steeper tilts",
        &[
            "at ε 1e-6:\nβ 0.5, spin 0: [\"GermSectionUnbuilt {",
            "\nβ 0.7, spin 1.5707963267948966: [\"GermSectionUnbuilt {",
            "why: Winds(Unseparated) }",
        ],
        other_spins_and_steeper_tilts_refuse_at_their_germs_row,
    );
}

/// The row [`other_spins_and_steeper_tilts_refuse_at_their_germs`] runs, under its pin.
fn other_spins_and_steeper_tilts_refuse_at_their_germs_row() {
    let h = half_donut();
    let poses = [
        (0.5, 0.0),
        (0.5, 0.5),
        (0.5, 1.0),
        (0.5, 2.5),
        (0.6, 0.0),
        (0.6, FRAC_PI_2),
        (0.7, 0.0),
        (0.7, FRAC_PI_2),
    ];
    let eps = Tol::witness().eps();
    let want = OTHER_SPINS_STOPS;
    let mut got = Vec::new();
    for &(beta, spin) in &poses {
        let c = rod(beta, spin);
        let before = geom_brep::implicit_pair_marches();
        let mut row = Vec::new();
        for (_, _, _, r) in every_op(&h, &c) {
            row.push(stop_word(&r));
        }
        let marches = geom_brep::implicit_pair_marches() - before;
        if marches != 5 {
            row.push(format!("{marches} marches over the five ops"));
        }
        got.push(row);
    }
    let off: Vec<String> = got
        .iter()
        .zip(want)
        .zip(poses)
        .filter(|((g, w), _)| g.iter().map(String::as_str).ne(w.iter().copied()))
        .map(|((g, _), (beta, spin))| format!("β {beta}, spin {spin}: {g:?}"))
        .collect();
    assert!(off.is_empty(), "at ε {eps:e}:\n{}", off.join("\n"));
}

/// **The lens is real.** At the rod axis's closest approach to the
/// donut's axis (`t = 1.8 sin β`, radius `1.8 cos β ≈ 1.5796`), the
/// axis point lies in both operands, and a point `0.11` toward the hole
/// (radius `≈ 1.4696`, inside the rod, short of the inner equator at
/// 1.5) lies in the rod and outside the half donut: the region the
/// join's body drops from ∪.
#[test]
fn the_rod_pokes_a_lens_out_of_the_inner_equator() {
    let (h, c) = (half_donut(), rod(0.5, FRAC_PI_2));
    let axis = Point3::new(1.8, 0.0, 0.0) + dir(0.5) * (1.8 * 0.5_f64.sin());
    let lens = axis + Vec3::new(-axis.x, 0.0, -axis.z).normalize() * 0.11;
    assert_eq!(
        (in_solid(&h, axis), in_solid(&c, axis)),
        (Some(true), Some(true)),
        "the closest-approach axis point {axis:?}"
    );
    assert_eq!(
        (in_solid(&h, lens), in_solid(&c, lens)),
        (Some(false), Some(true)),
        "the lens point {lens:?}"
    );
}
