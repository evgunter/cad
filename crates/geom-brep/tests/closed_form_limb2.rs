//! **Limb 2 is owed by every `Intersection` carrier of every kind** (C2):
//! a line, circle, ellipse or spiric between two analytic surfaces owes
//! the composite hull bound between its samples, read through its
//! certified enclosing net, exactly as a spline carrier does.
//!
//! The teeth: a closed-form carrier whose stored fields put it off its
//! surface by more than the band's escalation threshold between two
//! schedule samples, while every one of the nine samples (and the
//! witness) reads inside the zero band on the certifier's own meter,
//! refuses on limb 2. The miss is placed between the two samples that
//! straddle the carrier's farthest point from the surface, so the
//! samples read it at a fraction of its peak. The band is chosen from
//! that fraction, measured here, so the row says what it tests rather
//! than leaning on a constant: the samples read under `zero`, the peak
//! lies past `escalate`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::surf::table;
use crate::shared::tol::band;
use core::f64::consts::PI;
use geom::{Curve3, Surface};
use geom_brep::intersect::{PlaneTorusSection, plane_torus_section};
use geom_brep::{
    AnalyticRung3Refusal, CERT_SAMPLES, CertifyError, EdgeCurve, EdgeCurveSpec,
    EdgeDescriptionSpec, NurbsLane, SurfacePair, implicit_residual, sample_param,
};
use geom_core::test_support::fixture_lever;
use geom_core::{Band, Point3, Vec3};

/// The torus of the spiric rows: centre `(1, 2, 3)`, axis `y`.
fn torus() -> Surface<f64> {
    Surface::Torus {
        center: Point3::new(1.0, 2.0, 3.0),
        axis: Vec3::unit_y(),
        major_radius: 0.75,
        minor_radius: 0.3,
        u_ref: Vec3::unit_x(),
    }
}

/// The axis-parallel plane `x = 1.1`, off the torus's axis by `0.1`.
fn spiric_plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(1.1, 2.0, 3.0),
        normal: Vec3::unit_x(),
        u_ref: Vec3::unit_y(),
    }
}

/// One of the two exact ovals `x = 1.1` cuts from [`torus`].
fn spiric() -> Curve3<f64> {
    let section =
        plane_torus_section(&spiric_plane(), &torus(), fixture_lever(1.0), band()).unwrap();
    let PlaneTorusSection::SpiricOvals { s1, .. } = section else {
        panic!("expected the two spiric ovals, got {section:?}");
    };
    s1
}

/// The cylinder `x² + y² = 1` and the plane `z = y·tan φ` through it.
fn ellipse_pair(phi: f64) -> (Surface<f64>, Surface<f64>, Curve3<f64>) {
    let (s, c) = phi.sin_cos();
    let normal = Vec3::new(0.0, -s, c);
    let cylinder = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let plane = Surface::Plane {
        origin: Point3::origin(),
        normal,
        u_ref: Vec3::unit_x(),
    };
    let ellipse = Curve3::Ellipse(geom::Ellipse::new(
        geom::Frame::new(Point3::origin(), normal, Vec3::unit_x()),
        1.0,
        1.0 / c,
    ));
    (cylinder, plane, ellipse)
}

/// The edge `carrier` over `[t0, t1]` as the intersection of `a` and
/// `b`, certified at `band` through the lane.
fn certify(
    a: &Surface<f64>,
    b: &Surface<f64>,
    carrier: &Curve3<f64>,
    (t0, t1): (f64, f64),
    band: Band,
) -> Result<EdgeCurve<f64>, CertifyError> {
    let (keys, lookup) = table(vec![a.clone(), b.clone()]);
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            pair: SurfacePair::new(keys[0], keys[1]),
            selector: geom_brep::CurveSelector::Witness(carrier.mid_point(t0, t1)),
        },
        carrier: carrier.clone(),
        param_start: t0,
        param_end: t1,
    };
    EdgeCurve::certify(
        spec,
        carrier.eval(t0),
        carrier.eval(t1),
        &lookup,
        fixture_lever(1.0),
        band,
        NurbsLane::certified(),
    )
}

/// The worst reading the certifier's per-sample meter takes of
/// `carrier` against `a` and `b` over its schedule, the witness included.
fn sampled_max(
    a: &Surface<f64>,
    b: &Surface<f64>,
    carrier: &Curve3<f64>,
    (t0, t1): (f64, f64),
) -> f64 {
    (0..CERT_SAMPLES)
        .map(|i| carrier.eval(sample_param(t0, t1, i)))
        .chain([carrier.mid_point(t0, t1)])
        .map(|p| {
            implicit_residual(a, p)
                .abs()
                .max(implicit_residual(b, p).abs())
        })
        .fold(0.0, f64::max)
}

/// A span whose two middle samples straddle `t = 0`, the farthest
/// point of each perturbed row below, with both its ends well inside
/// the stretch where the miss is smaller than at those two samples.
const SPAN: (f64, f64) = (-7.0 * 1.5 * PI / 16.0, 9.0 * 1.5 * PI / 16.0);

/// The band the teeth rows read at: `zero` just above the samples'
/// worst reading, `escalate` halfway from it to the carrier's true
/// distance at `t = 0`, which the row asserts lies past it.
fn teeth_band(sampled: f64, peak: f64) -> Band {
    let zero = sampled * (1.0 + 1e-3);
    let escalate = 0.5 * (zero + peak);
    assert!(
        zero < escalate && escalate < peak,
        "the samples ({sampled:e}) read a fraction of the peak ({peak:e})"
    );
    Band::new(zero, escalate).unwrap()
}

/// Whether `got` is limb 2's definite refusal against the operand `kind`.
fn refused_on_limb2(got: &Result<EdgeCurve<f64>, CertifyError>, kind: geom::SurfaceKind) -> bool {
    matches!(
        got,
        Err(CertifyError::AnalyticRung3(AnalyticRung3Refusal::Limb {
            operand,
            ..
        })) if *operand == kind
    )
}

/// Every closed-form kind certifies through the lane on the pair that
/// mints it — plane × cylinder ruling, a plane × cone ruling from the
/// apex, plane × cone and plane × torus circles (the per-part axis
/// lever), the tilted plane × cylinder ellipse and the axis-parallel
/// plane × torus spiric.
#[test]
fn every_closed_form_kind_owes_limb_2_through_the_lane() {
    let cylinder = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let side = Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_y(),
        u_ref: Vec3::unit_x(),
    };
    let ruling = Curve3::line(Point3::new(1.0, 0.0, -1.0), Vec3::unit_z());
    let cone = Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle: PI / 6.0,
        u_ref: Vec3::unit_x(),
    };
    let level = |z: f64| Surface::Plane {
        origin: Point3::new(0.0, 0.0, z),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let circle = |z: f64, radius: f64| {
        Curve3::circle(
            Point3::new(0.0, 0.0, z),
            Vec3::unit_z(),
            radius,
            Vec3::unit_x(),
        )
    };
    let cone_circle = circle(2.0, 2.0 * (PI / 6.0).tan());
    let flat_torus = Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: 0.75,
        minor_radius: 0.3,
        u_ref: Vec3::unit_x(),
    };
    let equator = circle(0.0, 1.05);
    let (ecyl, eplane, ellipse) = ellipse_pair(PI / 5.0);
    // A ruling of the cone from its apex, where the cone's metre lever
    // vanishes: the parts near the apex read `√|f|` instead.
    let (sin, cos) = (PI / 6.0).sin_cos();
    let apex_ruling = Curve3::line(Point3::origin(), Vec3::new(sin, 0.0, cos));
    let rows = [
        ("ruling", cylinder.clone(), side.clone(), ruling, (0.0, 2.0)),
        (
            "cone ruling from the apex",
            cone.clone(),
            side,
            apex_ruling,
            (0.0, 2.0),
        ),
        ("cone circle", cone, level(2.0), cone_circle, (0.3, 2.9)),
        (
            "torus circle",
            flat_torus,
            level(0.0),
            equator,
            (0.0, 1.5 * PI),
        ),
        ("ellipse", ecyl, eplane, ellipse, SPAN),
        ("spiric", torus(), spiric_plane(), spiric(), SPAN),
    ];
    for (name, a, b, carrier, span) in rows {
        let (keys, lookup) = table(vec![a.clone(), b.clone()]);
        let (t0, t1) = span;
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: SurfacePair::new(keys[0], keys[1]),
                selector: geom_brep::CurveSelector::Witness(carrier.mid_point(t0, t1)),
            },
            carrier: carrier.clone(),
            param_start: t0,
            param_end: t1,
        };
        let (start, end) = (carrier.eval(t0), carrier.eval(t1));
        let lane = NurbsLane::certified();
        let edge = EdgeCurve::certify(spec, start, end, &lookup, fixture_lever(1.0), band(), lane)
            .unwrap_or_else(|e| panic!("{name}: the exact carrier certifies: {e:?}"));
        let at_rest = edge
            .recertify(start, end, &lookup, fixture_lever(1.0), band(), lane)
            .unwrap_or_else(|e| panic!("{name}: rest re-derives the certificate: {e:?}"));
        assert_eq!(
            format!("{at_rest:?}"),
            format!("{:?}", edge.certificate()),
            "{name}: rest re-derives the door's certificate"
        );
    }
}

/// A spiric whose stored major radius is `δ` past its torus's sits `δ·cos t`
/// off the torus (to second order) and exactly on its plane: the nine
/// samples straddle its farthest point, read inside the zero band, and
/// limb 2 refuses it against the torus.
#[test]
fn a_spiric_off_its_torus_between_samples_refuses_on_limb_2() {
    let Curve3::Spiric {
        center,
        axis,
        u_ref,
        major_radius,
        minor_radius,
        offset,
    } = spiric()
    else {
        unreachable!("the section's oval is a spiric")
    };
    let delta = 1e-4;
    let moved = Curve3::Spiric {
        center,
        axis,
        u_ref,
        major_radius: major_radius + delta,
        minor_radius,
        offset,
    };
    let (tor, plane) = (torus(), spiric_plane());
    let sampled = sampled_max(&tor, &plane, &moved, SPAN);
    let peak = implicit_residual(&tor, moved.eval(0.0)).abs();
    let got = certify(&tor, &plane, &moved, SPAN, teeth_band(sampled, peak));
    assert!(
        refused_on_limb2(&got, geom::SurfaceKind::Torus),
        "limb 2 against the torus refuses the moved spiric: {got:?}"
    );
}

/// An ellipse whose stored `semi_u` is `δ` past the cylinder's radius
/// sits `δ·cos² θ` off the cylinder and exactly on its plane: the same
/// teeth as the spiric's. The ellipse's residual against a quadric is a
/// trigonometric polynomial of degree 2, so the samples bound it up to
/// a fixed factor and no ellipse is far off between samples while near
/// on all nine; the factor is what the band reads past here.
#[test]
fn an_ellipse_off_its_cylinder_between_samples_refuses_on_limb_2() {
    let (cylinder, plane, ellipse) = ellipse_pair(PI / 5.0);
    let Curve3::Ellipse(geom::Ellipse {
        frame:
            geom::Frame {
                origin: center,
                axis,
                u_ref,
            },
        semi_u,
        semi_v,
    }) = ellipse
    else {
        unreachable!("built as an ellipse")
    };
    let moved = Curve3::Ellipse(geom::Ellipse::new(
        geom::Frame::new(center, axis, u_ref),
        semi_u + 1e-4,
        semi_v,
    ));
    let sampled = sampled_max(&cylinder, &plane, &moved, SPAN);
    let peak = implicit_residual(&cylinder, moved.eval(0.0)).abs();
    let got = certify(&cylinder, &plane, &moved, SPAN, teeth_band(sampled, peak));
    assert!(
        refused_on_limb2(&got, geom::SurfaceKind::Cylinder),
        "limb 2 against the cylinder refuses the moved ellipse: {got:?}"
    );
}

/// The band of a run at `ε = 1e-12`, the gate's finest row.
fn fine_band() -> Band {
    Band::new(1e-12, 1e-11).unwrap()
}

/// `p` moved by `d`.
fn moved(p: Point3<f64>, d: Vec3<f64>) -> Point3<f64> {
    p + d
}

/// **Limb 2 reads a part the same wherever it sits.** The net is framed
/// at each operand before anything is combined, so its controls round at
/// their distance from the operand's origin: a plane × cylinder circle,
/// ellipse and ruling certify at the gate's finest band at the origin
/// and moved 1 km along every axis (where a net enclosed in world
/// coordinates bounded about 1e-12 m and escalated).
#[test]
fn a_part_moved_a_kilometre_reads_as_at_the_origin() {
    for d in [Vec3::new(0.0, 0.0, 0.0), Vec3::new(1000.0, -1000.0, 1000.0)] {
        let cylinder = Surface::Cylinder {
            origin: moved(Point3::origin(), d),
            axis: Vec3::unit_z(),
            radius: 0.5,
            u_ref: Vec3::unit_x(),
        };
        let cap = Surface::Plane {
            origin: moved(Point3::new(0.0, 0.0, 0.25), d),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let rim = Curve3::circle(
            moved(Point3::new(0.0, 0.0, 0.25), d),
            Vec3::unit_z(),
            0.5,
            Vec3::unit_x(),
        );
        let side = Surface::Plane {
            origin: moved(Point3::origin(), d),
            normal: Vec3::unit_y(),
            u_ref: Vec3::unit_x(),
        };
        let ruling = Curve3::line(moved(Point3::new(0.5, 0.0, 0.0), d), Vec3::unit_z());
        let (s, c) = (PI / 5.0).sin_cos();
        let tilted = Surface::Plane {
            origin: moved(Point3::origin(), d),
            normal: Vec3::new(0.0, -s, c),
            u_ref: Vec3::unit_x(),
        };
        let ellipse = Curve3::Ellipse(geom::Ellipse::new(
            geom::Frame::new(
                moved(Point3::origin(), d),
                Vec3::new(0.0, -s, c),
                Vec3::unit_x(),
            ),
            0.5,
            0.5 / c,
        ));
        let rows = [
            ("rim", &cylinder, &cap, &rim, (0.0, 1.5 * PI)),
            ("ruling", &cylinder, &side, &ruling, (0.0, 0.5)),
            ("ellipse", &cylinder, &tilted, &ellipse, SPAN),
        ];
        for (name, a, b, carrier, span) in rows {
            let got = certify(a, b, carrier, span, fine_band());
            assert!(got.is_ok(), "{name} moved by {d:?}: {got:?}");
        }
    }
}

/// Limb 2's kernel limit on `got`: its operand, bound, width and extent.
fn kernel_limit(
    got: &Result<EdgeCurve<f64>, CertifyError>,
) -> Option<(geom::SurfaceKind, f64, f64, f64)> {
    match got {
        Err(CertifyError::AnalyticRung3(AnalyticRung3Refusal::KernelLimit {
            operand,
            bound,
            width,
            extent,
            ..
        })) => Some((*operand, *bound, *width, *extent)),
        _ => None,
    }
}

/// **A long line between two planes, past the session box, is the
/// kernel's limit**: a 12 km line where two skew planes cross, at the
/// gate's finest band. Its distance from each plane is affine along it,
/// so the sup is at an end, and each end's distance is measured in
/// certification arithmetic, the data lifted through its certified
/// brackets. At 12 km that enclosure is `[0, 9.1e-12]`, the data's own
/// rounding: it reaches zero, so it is only an upper bound, which says
/// nothing about the input, and the composite's stall stands — at `f64`
/// the kernel's limit ([`AnalyticRung3Refusal::KernelLimit`]), never the
/// measured escalation. Red under an ends rule read in round-to-nearest
/// `f64` (which passed this line on dot products rounding past the band),
/// and under one that escalates on an end enclosure reaching zero.
#[test]
fn a_twelve_kilometre_line_between_planes_refuses_as_the_kernel_limit() {
    let unit = |v: Vec3<f64>| v / v.norm();
    let at = Point3::new(0.3, -0.2, 0.1);
    let (n1, n2) = (
        unit(Vec3::new(1.0, 2.0, 3.0)),
        unit(Vec3::new(-2.0, 1.0, 0.5)),
    );
    let plane = |n: Vec3<f64>| Surface::Plane {
        origin: at,
        normal: n,
        u_ref: unit(n.cross(Vec3::unit_z())),
    };
    let line = Curve3::line(at, unit(n1.cross(n2)));
    let got = certify(&plane(n1), &plane(n2), &line, (0.0, 12_000.0), fine_band());
    assert!(
        matches!(
            got,
            Err(CertifyError::AnalyticRung3(
                AnalyticRung3Refusal::KernelLimit { .. }
            ))
        ),
        "{got:?}"
    );
}

/// **A long edge's own rounding is the limit**: a 12 km ruling of a
/// skew cylinder, where a plane through its axis cuts it, at the gate's
/// finest band, rounds at about `ulp(12000)` in the cylinder's frame, so
/// limb 2's bound cannot clear `1e-12` m. It refuses as the kernel's
/// approximation limit, never as a miss, with a bound within
/// [`geom_brep::KERNEL_LIMIT_ROUNDING_FACTOR`] of the net's own width,
/// and its story names shorter edges.
#[test]
fn a_twelve_kilometre_edge_refuses_as_the_kernel_limit_at_its_extent() {
    let unit = |v: Vec3<f64>| v / v.norm();
    let at = Point3::new(0.3, -0.2, 0.1);
    let axis = unit(Vec3::new(1.0, 2.0, 3.0));
    let across = unit(axis.cross(Vec3::unit_z()));
    let cylinder = Surface::Cylinder {
        origin: at,
        axis,
        radius: 1.0,
        u_ref: across,
    };
    let plane = Surface::Plane {
        origin: at,
        normal: unit(axis.cross(across)),
        u_ref: axis,
    };
    let line = Curve3::line(at + across, axis);
    let got = certify(&cylinder, &plane, &line, (0.0, 12_000.0), fine_band());
    let Some((operand, bound, width, extent)) = kernel_limit(&got) else {
        panic!("the kernel's limit: {got:?}");
    };
    assert_eq!(operand, geom::SurfaceKind::Cylinder);
    assert!(
        bound > 1e-12,
        "the bound does not clear the band: {bound:e}"
    );
    assert!(
        width > 0.0 && bound <= geom_brep::KERNEL_LIMIT_ROUNDING_FACTOR * width,
        "the bound is the net's own rounding: {bound:e} against {width:e}"
    );
    assert!(
        (11_999.0..12_001.0).contains(&extent),
        "the extent: {extent}"
    );
    let ending = got.unwrap_err().ending(geom_brep::recourse::Reading::Build);
    assert!(
        ending
            .as_deref()
            .is_some_and(|e| e.contains("shorter edges")),
        "{ending:?}"
    );
}

/// **A torus's composite is the limit**: a circle where a level plane
/// cuts a torus with `R/r = 50`, at the gate's finest band, reads a bound far above the
/// net's own rounding (the quartic composite's conditioning). It refuses
/// as the kernel's approximation limit with the last-resort story, which
/// claims no cause it cannot read off the bound and the width.
#[test]
fn a_thin_torus_circle_refuses_as_the_kernel_limit_of_its_composite() {
    let torus = Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: 10.0,
        minor_radius: 0.2,
        u_ref: Vec3::unit_x(),
    };
    let level = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.1),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let outer = Curve3::circle(
        Point3::new(0.0, 0.0, 0.1),
        Vec3::unit_z(),
        10.0 + 0.03_f64.sqrt(),
        Vec3::unit_x(),
    );
    let got = certify(&torus, &level, &outer, (0.25, 1.75), fine_band());
    let Some((operand, bound, width, extent)) = kernel_limit(&got) else {
        panic!("the kernel's limit: {got:?}");
    };
    assert_eq!(operand, geom::SurfaceKind::Torus);
    assert!(
        bound > geom_brep::KERNEL_LIMIT_ROUNDING_FACTOR * width,
        "the bound is far above the net's rounding: {bound:e} against {width:e}"
    );
    assert!(extent > 1.0 && extent < 20.0, "the extent: {extent}");
    let ending = got.unwrap_err().ending(geom_brep::recourse::Reading::Build);
    assert!(
        ending.as_deref().is_some_and(|e| e.contains("far above")
            && !e.contains("conditioning")
            && e.contains("last resort")),
        "{ending:?}"
    );
}

/// The lemon sheet `R = −½, r = 1` about `z` through the origin, its
/// apexes at `(0, 0, ±√3/2)`, and the plane `y = 0` through its axis.
fn lemon_and_its_meridian_plane() -> (Surface<f64>, Surface<f64>) {
    let sheet = Surface::Spindle {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: -0.5,
        minor_radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let plane = Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_y(),
        u_ref: Vec3::unit_x(),
    };
    (sheet, plane)
}

/// The unit circle about `(c, 0, 0)` in `y = 0`, from `+x` toward `+z`.
fn meridian(c: f64) -> Curve3<f64> {
    Curve3::Circle(geom::Circle {
        frame: geom::Frame::new(Point3::new(c, 0.0, 0.0), -Vec3::unit_y(), Vec3::unit_x()),
        radius: 1.0,
    })
}

/// **Limb 2 against a sheet reads that sheet up to its apexes.** The
/// lemon's meridian from one apex to the other (the arc of the circle
/// about `(−½, 0, 0)` with `|t| ≤ π/3`) certifies, its parts at the
/// apexes read by their distance from the apex. The other sheet's
/// meridian through the same two apexes (the circle about `(½, 0, 0)`,
/// `|t| ≤ 2π/3`), and its last micro-radian before the upper apex, lie
/// on the torus's implicit form, where `|f|` reads zero; limb 2 refuses
/// them against the sheet, its bound the distance from an apex. It
/// reads no floor, as its floors are the implicit form's. Reading `|f|` at the apexes, as the cone's
/// reading does, passes both.
#[test]
fn limb_2_against_a_sheet_reads_that_sheet_up_to_its_apexes() {
    let (sheet, plane) = lemon_and_its_meridian_plane();
    let third = PI / 3.0;
    let limbs =
        |c: f64, span| geom_brep::analytic_edge_limbs(&meridian(c), span, &sheet, &plane, band());
    let own = limbs(-0.5, (-third, third));
    assert!(own.is_ok(), "the lemon's own meridian: {own:?}");
    for (what, span) in [
        ("the other sheet's meridian", (-2.0 * third, 2.0 * third)),
        ("its last micro-radian", (2.0 * third - 1e-6, 2.0 * third)),
    ] {
        let got = limbs(0.5, span);
        assert!(
            matches!(
                got,
                Err(AnalyticRung3Refusal::KernelLimit {
                    operand: geom::SurfaceKind::Spindle,
                    ..
                })
            ),
            "{what}: {got:?}"
        );
    }
}

/// **Past a thin lemon's apex, limb 2 reads the sheet, not its circle.**
/// The lemon `R = −0.999, r = 1` (apexes at `z = ±h`, `h ≈ 0.0447`), and
/// a segment of the line `x = 3·10⁻⁵, y = 0` from `z = h + 5·10⁻⁴` up
/// `10⁻⁵`, above the upper apex beside the axis. The lever is positive
/// there and the meridian circle passes within `6·10⁻⁵` of every point,
/// but the circle's nearest points lie across the axis, past the apex;
/// the sheet's nearest point is the apex itself, `5·10⁻⁴` away. At a band
/// whose zero is `10⁻⁴` and whose threshold is `3·10⁻⁴`, the side check
/// (`|m − r|·|R| ≤ r·ρ`) is what refuses it: without it, the circle's
/// distance would clear the band.
#[test]
fn past_a_thin_lemons_apex_limb_2_reads_the_sheet_not_its_circle() {
    let (major, minor) = (-0.999_f64, 1.0_f64);
    let h = geom::sheet_apex(major, minor).height;
    let sheet = Surface::Spindle {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: major,
        minor_radius: minor,
        u_ref: Vec3::unit_x(),
    };
    let (_, plane) = lemon_and_its_meridian_plane();
    let line = Curve3::Line(geom::Line {
        origin: Point3::new(3e-5, 0.0, h + 5e-4),
        dir: Vec3::unit_z(),
    });
    let band = Band::new(1e-4, 3e-4).unwrap();
    let got = geom_brep::analytic_edge_limbs(&line, (0.0, 1e-5), &sheet, &plane, band);
    assert!(
        matches!(
            got,
            Err(AnalyticRung3Refusal::KernelLimit {
                operand: geom::SurfaceKind::Spindle,
                ..
            })
        ),
        "{got:?}"
    );
}


/// **Limb 2 against a horn reads the sheet up to its cusp.** The horn
/// `R = r = 1` (its cusp at the centre) and its meridian, the circle
/// about `(1, 0, 0)`, from its top to the cusp on either side
/// (`π/2 ≤ t ≤ π` and `−π ≤ t ≤ −π/2`), certify at a band whose zero
/// is `10⁻⁹` and at one whose zero is `10⁻¹²`. Beside the cusp the sheet
/// runs at `ρ ≈ z²/2`, tangent to the axis: a part there is read at its
/// own height, `|ρ − ρ_s(z)|` (`Meters::certified`), where `|f|` over the
/// lever (`≈ z²`) reads only the net's rounding over `z²`. The same
/// circle moved `10⁻⁶` off the axis, `10⁻⁶` from the horn at the cusp,
/// refuses at both, read off (`Limb`) or not shown on (`KernelLimit`).
#[test]
fn limb_2_against_a_horn_reads_the_sheet_up_to_its_cusp() {
    let horn = Surface::Spindle {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: 1.0,
        minor_radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let (_, plane) = lemon_and_its_meridian_plane();
    for band in [Band::new(1e-9, 3e-9).unwrap(), Band::new(1e-12, 3e-12).unwrap()] {
        for span in [(PI / 2.0, PI), (-PI, -PI / 2.0)] {
            let own = geom_brep::analytic_edge_limbs(&meridian(1.0), span, &horn, &plane, band);
            assert!(own.is_ok(), "band {band:?}, the horn's meridian over {span:?}: {own:?}");
            let off = geom_brep::analytic_edge_limbs(&meridian(1.0 + 1e-6), span, &horn, &plane, band);
            assert!(
                matches!(
                    off,
                    Err(AnalyticRung3Refusal::KernelLimit {
                        operand: geom::SurfaceKind::Spindle,
                        ..
                    } | AnalyticRung3Refusal::Limb {
                        operand: geom::SurfaceKind::Spindle,
                        ..
                    })
                ),
                "band {band:?}, a circle 1e-6 off over {span:?}: {off:?}"
            );
        }
    }
}
