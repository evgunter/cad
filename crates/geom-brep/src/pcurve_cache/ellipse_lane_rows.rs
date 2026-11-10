//! **An ellipse's general lane on a sphere, cylinder, cone and torus**
//! (D3's lane choice on an ellipse's class readings): an ellipse the
//! incidence read does not show off the chart, and which is none of the
//! chart's own classes or whose class reading is in band, takes the
//! projected image, read on the circle's terms: its harmonic form
//! `c + a·cos t + b·sin t`, whose check-4 incidence term is
//! `implicit::conic_form_sup` converted through `projected::Meters`. An
//! ellipse definitely off ends as its incidence read. Every row holds at
//! `f64` and at `Interval`, and at every ε of the run.
//!
//! Mutants (`PCERT_RADIAL_PAIR_MUTANT`): `circle-incidence` reds the
//! rows standing a measured distance off a cylinder, cone or torus and
//! the search; `charge-circular` (the cone's charge read at the first
//! semi-axis for both) reds the coefficient rows; `ellipse-term-dropped`
//! (the second harmonic's `a² − b²` part dropped) reds the coefficient
//! rows and the search; `semi-axis-unchecked` (the edge certificate's
//! positive semi-axis check skipped) reds the defect row.

use core::f64::consts::{FRAC_PI_2, TAU};

use geom_core::{Bounds, Interval};
use test_utils::fuzz;

use super::circle_lane_rows::{
    certifies_projected, check_4_terms, cone, cone_coefficient_excess, cylinder, derived,
    other_charts_within, torus, world, world_dir,
};
use super::harmonic_sup_rows::{band, sampled_residual, zero};
use super::*;

/// The ellipse centred at `c` with semi-axis `a` along the unit `u` and
/// `b` along the unit `v` (`u ⟂ v`), all in `surface`'s chart frame.
fn framed_ellipse(
    surface: &Surface<f64>,
    c: Vec3<f64>,
    (u, a): (Vec3<f64>, f64),
    (v, b): (Vec3<f64>, f64),
) -> Curve3<f64> {
    let (u, v) = (world_dir(surface, u), world_dir(surface, v));
    Curve3::Ellipse(geom::Ellipse::new(
        geom::Frame::new(world(surface, c), u.cross(v).normalize(), u.normalize()),
        a,
        b,
    ))
}

fn sphere(radius: f64) -> Surface<f64> {
    Surface::Sphere {
        center: Point3::new(0.2, -0.5, 0.1),
        radius,
        axis: Vec3::new(0.1, 0.2, 1.0).normalize(),
        u_ref: Vec3::new(1.0, 0.0, -0.1).normalize(),
    }
}

/// The needle ellipse along the cone's generator at azimuth `0` on the
/// nappe `sigma` names, centred at slant `s`: its semi-axis `a` along
/// the generator, `b` across it, turned by `chi` from the cone's tangent
/// toward its outward normal, and the whole stood `off` along that
/// normal. It leaves the cone by `b·sin χ + off` to first order, and its
/// orientation reads `b·cos χ` (metres, over its radial lever).
fn cone_needle(
    surface: &Surface<f64>,
    sigma: f64,
    s: f64,
    (a, b, chi): (f64, f64, f64),
    off: f64,
) -> Curve3<f64> {
    let Surface::Cone { half_angle, .. } = *surface else {
        unreachable!("a cone needle is a cone's")
    };
    let (sa, ca) = half_angle.sin_cos();
    let g = Vec3::new(sa, 0.0, ca * sigma);
    let n = Vec3::new(ca, 0.0, -sa * sigma);
    let (sx, cx) = chi.sin_cos();
    framed_ellipse(
        surface,
        g * s + n * off,
        (g, a),
        (Vec3::unit_y() * cx + n * sx, b),
    )
}

/// The cone's in-band section: a needle whose minor semi-axis just
/// passes the escalation band (so the carrier's speed is decided) and
/// turned so its orientation, `b·cos χ`, falls just inside it, standing
/// `b·sin χ`, under the zero band, off the cone.
fn cone_in_band_needle(surface: &Surface<f64>, sigma: f64) -> Curve3<f64> {
    let b = 1.002 * band().escalate();
    cone_needle(surface, sigma, 1.2, (0.3, b, 0.998_f64.acos()), 0.0)
}

/// The ellipse in the plane tangent to the wall along its ruling at
/// azimuth `0`: its semi-axis `a` turned `tilt` from the axis within
/// that plane, `b` across, centred `off` out from the wall. Its sagitta
/// `b²/2R` (at `tilt = 0`) is its distance from the wall; its
/// orientation reads Zero, since the plane holds the axis.
fn tangent_needle(surface: &Surface<f64>, (a, b, tilt): (f64, f64, f64), off: f64) -> Curve3<f64> {
    let Surface::Cylinder { radius, .. } = *surface else {
        unreachable!("a tangent needle is a wall's")
    };
    let (st, ct) = tilt.sin_cos();
    framed_ellipse(
        surface,
        Vec3::new(radius + off, 0.0, 0.5),
        (Vec3::unit_z() * ct + Vec3::unit_y() * st, a),
        (Vec3::unit_y() * ct - Vec3::unit_z() * st, b),
    )
}

/// The ellipse in the torus's crest plane `z = r`, centred on the axis,
/// with semi-axes `R` and `R − d`: it leaves the crest circle by `d` at
/// its minor vertices, so the torus by `d²/2r`.
fn crest_ellipse(surface: &Surface<f64>, d: f64) -> Curve3<f64> {
    let Surface::Torus {
        major_radius,
        minor_radius,
        ..
    } = *surface
    else {
        unreachable!("a crest ellipse is a torus's")
    };
    framed_ellipse(
        surface,
        Vec3::new(0.0, 0.0, minor_radius),
        (Vec3::unit_x(), major_radius),
        (Vec3::unit_y(), major_radius - d),
    )
}

/// The parallel of the sphere at height `z` squashed by `d` along its
/// second semi-axis: it leaves the sphere by about `d·ρ` at its minor
/// vertices (`ρ` the parallel's radius over the sphere's).
fn squashed_parallel(surface: &Surface<f64>, z: f64, d: f64) -> Curve3<f64> {
    let Surface::Sphere { radius, .. } = *surface else {
        unreachable!("a squashed parallel is a sphere's")
    };
    let rho = (radius * radius - z * z).sqrt();
    framed_ellipse(
        surface,
        Vec3::new(0.0, 0.0, z),
        (Vec3::unit_x(), rho),
        (Vec3::unit_y(), rho - d),
    )
}

/// The cone's exact section by the plane through the axis point at
/// height `h` tilted by `tau` about the frame's `x` (`tau` short of the
/// plane's turning parallel to a generator): its minor semi-axis along
/// `x`, its major along the plane's slope, stored second.
fn cone_section_ellipse(surface: &Surface<f64>, h: f64, tau: f64) -> Curve3<f64> {
    let Surface::Cone { half_angle, .. } = *surface else {
        unreachable!("a cone section is a cone's")
    };
    let (sa, ca) = half_angle.sin_cos();
    let (st, ct) = tau.sin_cos();
    let n = Vec3::new(0.0, -st, ct);
    let sigma = h.signum();
    let vertex = |s_phi: f64| {
        let g = Vec3::new(0.0, sa * s_phi, ca) * sigma;
        g * (h * ct / n.dot(g))
    };
    let (p, q) = (vertex(1.0), vertex(-1.0));
    let m = (p + q) * 0.5;
    let semi_major = (p - q).norm() * 0.5;
    let semi_minor = ((m.z * sa / ca).powi(2) - m.y.powi(2)).sqrt();
    framed_ellipse(
        surface,
        m,
        (Vec3::unit_x(), semi_minor),
        ((p - q).normalize(), semi_major),
    )
}

/// The derivation reads no class: the general lane, with the class
/// reading's cause where it was in band (`undecided`).
fn takes_the_general_lane(
    what: &str,
    carrier: &Curve3<f64>,
    span: (f64, f64),
    surface: &Surface<f64>,
    undecided: bool,
) {
    let read = derive_class(
        carrier,
        surface,
        Placement::over(span, carrier, surface),
        band(),
    );
    assert!(
        matches!(
            &read,
            Err(Underived::General { undecided: u, .. }) if u.is_some() == undecided
        ),
        "{what}: no class shown (in band: {undecided}): {:?}",
        read.map(|(_, d)| d).map_err(|u| match u {
            Underived::Refused(e) => Some(e),
            Underived::General { .. } => None,
        })
    );
}

/// **A cone section whose orientation reads in band certifies through
/// the projected lane**, on both nappes, over an arc and a whole turn:
/// the needle of [`cone_in_band_needle`], whose minor semi-axis passes
/// the escalation band and whose orientation falls inside it. It stands
/// `0.63·zero` off, and its envelope covers that residual, which is
/// incidence alone (red under `circle-incidence`).
///
/// It is the cone's only such ellipse: the orientation reads `b·cos χ`
/// and the distance `b·sin χ`, so a carrier within the zero band whose
/// speed `b` the interval meter decides has its orientation within
/// `b − zero` of the escalation band. A minor semi-axis inside the band
/// takes the lane too, and ends at the meter, as on every lane.
#[test]
fn an_in_band_cone_section_certifies_through_the_projected_lane() {
    let cone = cone(0.5);
    for sigma in [1.0, -1.0] {
        let needle = cone_in_band_needle(&cone, sigma);
        for span in [(0.3, 2.5), (-1.0, TAU - 1.0)] {
            let what = format!("nappe {sigma}, span {span:?}");
            takes_the_general_lane(&what, &needle, span, &cone, true);
            certifies_projected(&what, &needle, span, &cone);
        }
        let thin = cone_needle(&cone, sigma, 1.2, (0.3, 0.5 * band().escalate(), 0.0), 0.0);
        let (image, image_i) = derived(&thin, (0.3, 2.5), &cone);
        let image = image.unwrap_or_else(|e| panic!("the thin needle derives: {e:?}"));
        assert!(
            matches!(image, Pcurve::Projected(_)) && matches!(image_i, Ok(Pcurve::Projected(_))),
            "nappe {sigma}: a needle under the band takes the projected image"
        );
        let met = PcurveCache::certify(image, 0.3, 2.5, &thin, &cone, band());
        assert!(
            matches!(
                met,
                Err(PcurveCertifyError::Escalated {
                    check: PcurveCheck::ParamSpan,
                    ..
                })
            ),
            "nappe {sigma}: a needle under the band ends at the interval meter: {:?}",
            met.map(|c| c.certificate().envelope)
        );
    }
}

/// **An ellipse within the band of a sphere or a torus certifies through
/// the projected lane** (neither chart holds an ellipse): a parallel of
/// the sphere squashed until it stands `0.7·zero` off, and an ellipse in
/// the torus's crest plane squashed until it does, each over an arc and
/// a whole turn. The envelopes cover the residual, red under
/// `circle-incidence` on the torus.
#[test]
fn an_ellipse_within_the_band_of_a_sphere_or_a_torus_certifies() {
    let (sphere, torus) = (sphere(1.0), torus(2.0, 0.5));
    let rows = [
        (
            "sphere",
            &sphere,
            squashed_parallel(&sphere, 0.6, 0.875 * zero()),
        ),
        (
            "torus",
            &torus,
            crest_ellipse(&torus, (0.7 * zero()).sqrt()),
        ),
    ];
    for (what, surface, carrier) in rows {
        for span in [(0.3, 2.5), (-1.0, TAU - 1.0)] {
            let what = format!("{what}, span {span:?}");
            takes_the_general_lane(&what, &carrier, span, surface, false);
            certifies_projected(&what, &carrier, span, surface);
        }
    }
}

/// **A conic in a plane along a cylinder's axis certifies through the
/// projected lane**: its radial part moves and its orientation reads
/// Zero, which is no class of the wall (a cross-section's and an oblique
/// section's orientation is the radius), so the derivation hands it to
/// the general lane. In the plane tangent along a ruling: a needle
/// upright, its sagitta `0.7·zero`; an ellipse of ratio `1/2` tilted `0.4`
/// within the plane and moved `0.2·zero` in; and a circle. Before, the derivation read the Zero as a winding of
/// `0`, whose constant-azimuth image misses the carrier by its swing
/// (red under `circle-incidence`).
#[test]
fn a_conic_in_a_plane_along_a_cylinders_axis_certifies() {
    let wall = cylinder(1.0);
    let b = (1.4 * zero()).sqrt();
    let rows = [
        (
            "an upright needle",
            tangent_needle(&wall, (0.3, b, 0.0), 0.0),
        ),
        (
            "a tilted ellipse",
            tangent_needle(&wall, (1.6 * b, 0.8 * b, 0.4), -0.2 * zero()),
        ),
        (
            "a circle",
            tangent_needle(&wall, (b, b, 0.0), 0.0)
                .conic()
                .map(|c| Curve3::circle(c.frame().origin, c.frame().axis, b, c.frame().u_ref))
                .unwrap_or_else(|| unreachable!("a needle is a conic")),
        ),
    ];
    for (what, carrier) in rows {
        for span in [(0.3, 2.5), (-1.0, TAU - 1.0)] {
            let what = format!("{what}, span {span:?}");
            takes_the_general_lane(&what, &carrier, span, &wall, false);
            certifies_projected(&what, &carrier, span, &wall);
        }
    }
}

/// **An ellipse definitely off its chart ends as its incidence read**
/// before any lane, at both scalars, and renders the definite miss: the
/// kernel's defect at a build, the stored culprit at rest, the damaged
/// file at the import door: each lane row's carrier moved past
/// the escalation band (the cone's needle and the wall's stood `4·escalate`
/// out, the sphere's parallel squashed and the crest ellipse narrowed
/// until each stands that far off).
#[test]
fn an_ellipse_definitely_off_ends_as_its_incidence_read() {
    let far = 4.0 * band().escalate();
    let (cone, wall, sphere, torus) = (cone(0.5), cylinder(1.0), sphere(1.0), torus(2.0, 0.5));
    let rows = [
        (
            "cone",
            &cone,
            cone_needle(&cone, 1.0, 1.2, (0.3, 1.002 * band().escalate(), 0.06), far),
        ),
        (
            "wall",
            &wall,
            tangent_needle(&wall, (0.3, (1.4 * zero()).sqrt(), 0.0), far),
        ),
        (
            "sphere",
            &sphere,
            squashed_parallel(&sphere, 0.6, 1.25 * far),
        ),
        ("torus", &torus, crest_ellipse(&torus, far.sqrt())),
    ];
    use crate::recourse::{ReadAt, Reading};
    let doors = [
        (ReadAt::Run(Reading::Build), geom_core::KERNEL_DEFECT_ENDING),
        (
            ReadAt::Run(Reading::AtRest),
            geom_core::KERNEL_OR_FILE_DEFECT_ENDING,
        ),
        (
            ReadAt::File(geom_core::FileCoincidence::new(1e-6)),
            geom_core::DAMAGED_FILE_ENDING,
        ),
    ];
    for (what, surface, carrier) in rows {
        let (at_f64, at_interval) = derived(&carrier, (0.3, 2.5), surface);
        for (lane, got) in [
            ("f64", at_f64.map(|_| ())),
            ("Interval", at_interval.map(|_| ())),
        ] {
            let Err(err) = &got else {
                panic!("{what} ({lane}): ends as its incidence read: {got:?}")
            };
            assert!(
                matches!(IncidenceRead::of(err), Some(IncidenceRead::Off(_))),
                "{what} ({lane}): the definite miss: {err:?}"
            );
            for (at, culprit) in doors {
                assert_eq!(err.ending(at), culprit, "{what} ({lane}) at {at:?}");
            }
        }
    }
}

/// A unit vector of the draw.
fn unit(s: &mut fuzz::Rng) -> Vec3<f64> {
    Vec3::new(s.range(-1.0, 1.0), s.range(-1.0, 1.0), s.range(-1.0, 1.0)).normalize()
}

/// An ellipse of the draw, centred at `centre` in the plane of normal
/// `n`: its larger semi-axis `big`, its ratio `b/a` log-uniform down to
/// `1e-6`, stored larger-first or larger-second.
fn drawn_ellipse(s: &mut fuzz::Rng, centre: Point3<f64>, n: Vec3<f64>, big: f64) -> Curve3<f64> {
    let small = big * s.log_uniform(1e-6, 1.0);
    let (a, b) = if s.below(2) == 0 {
        (big, small)
    } else {
        (small, big)
    };
    let u = n.cross(unit(s)).normalize();
    Curve3::Ellipse(geom::Ellipse::new(geom::Frame::new(centre, n, u), a, b))
}

/// A row of the sphere, wall and torus coefficient check: its name, the
/// ellipse, and the charts' centre, axis, radius and tube radius.
type OtherChartsRow = (String, Curve3<f64>, Point3<f64>, Vec3<f64>, f64, f64);

/// **An ellipse's form coefficients stay within their charge on every
/// chart**, exactly ([`cone_coefficient_excess`] on the cone,
/// [`other_charts_within`] on the sphere, wall and torus): the extremes
/// written down (a needle cone of half-angle `1e-4` and a flat one of
/// `π/2 − 1e-4`, `1e5` off the origin, a torus of tube ratio `1e-4`, and
/// a cone's exact section near the parabola, each with an ellipse of
/// ratio `1e-6` stored larger-second), and draws of the circle rows'
/// search widened to ellipses of ratio down to `1e-6`, stored either
/// way round. Red under `charge-circular` and `ellipse-term-dropped`.
#[test]
fn an_ellipses_coefficients_stay_within_their_charge_on_every_chart() {
    let far = 1e5;
    let at = Point3::new(0.3 + far, -0.7 - 0.6 * far, 0.2 + 0.5 * far);
    let axis = Vec3::new(0.2, -0.4, 1.0).normalize();
    let u_ref = axis.cross(Vec3::unit_x()).normalize();
    let cone_at = |alpha: f64| Surface::Cone {
        apex: at,
        axis,
        half_angle: alpha,
        u_ref,
    };
    let eccentric = |centre: Point3<f64>, n: Vec3<f64>, big: f64| {
        Curve3::Ellipse(geom::Ellipse::new(
            geom::Frame::new(centre, n, n.cross(Vec3::unit_x()).normalize()),
            big * 1e-6,
            big,
        ))
    };
    let tilted = Vec3::new(0.1, 0.9, 0.3).normalize();
    let mut cones: Vec<(String, Surface<f64>, Curve3<f64>)> = Vec::new();
    for alpha in [1e-4, FRAC_PI_2 - 1e-4, 0.5] {
        let surface = cone_at(alpha);
        cones.push((
            format!("α {alpha:e}, a ratio-1e-6 ellipse"),
            surface.clone(),
            eccentric(at + axis * 2.0, tilted, 0.7),
        ));
        cones.push((
            format!("α {alpha:e}, the section near the parabola"),
            surface.clone(),
            cone_section_ellipse(&surface, 1.0, (FRAC_PI_2 - alpha) * (1.0 - 1e-6)),
        ));
    }
    let mut others: Vec<OtherChartsRow> = vec![(
        "a thin torus's ratio-1e-6 ellipse".into(),
        eccentric(at + axis * 0.3, tilted, 2.0),
        at,
        axis,
        2.0,
        2e-4,
    )];
    let mut s = fuzz::start("pcurve_cache::ellipse_lane_rows::coefficients");
    for i in 0..fuzz::scaled(300) {
        let far = if s.below(2) == 0 {
            0.0
        } else {
            s.log_uniform(1.0, 1e5)
        };
        let at = Point3::new(0.3 + far, -0.7 - 0.6 * far, 0.2 + 0.5 * far);
        let axis = unit(&mut s);
        let scale = s.log_uniform(1e-3, 1e3);
        let n = if s.below(3) == 0 { axis } else { unit(&mut s) };
        let along = s.range(-3.0, 3.0) * scale * if s.below(2) == 0 { 1.0 } else { 1e3 };
        let off = unit(&mut s) * scale * s.log_uniform(1e-6, 2.0);
        let big = scale * s.log_uniform(1e-4, 3.0);
        let carrier = drawn_ellipse(&mut s, at + axis * along + off, n, big);
        let alpha = match s.below(3) {
            0 => s.log_uniform(1e-4, 0.05),
            1 => s.range(1.52, FRAC_PI_2 - 1e-4),
            _ => s.range(0.05, 1.5),
        };
        let u_ref = axis.cross(Vec3::new(0.3, 1.0, -0.2)).normalize();
        let what = format!("draw {i}: far {far:e}, scale {scale:e}, {carrier:?}");
        cones.push((
            format!("{what}, α {alpha:e}"),
            Surface::Cone {
                apex: at,
                axis,
                half_angle: alpha,
                u_ref,
            },
            carrier.clone(),
        ));
        let r = scale * s.log_uniform(0.1, 3.0);
        let small = r * s.log_uniform(1e-4, 0.99999);
        others.push((what, carrier, at, axis, r, small));
    }
    for (what, surface, carrier) in cones {
        let (within, charge) = cone_coefficient_excess(&surface, &carrier);
        assert!(
            within,
            "{what}: the cone's coefficients' exact excess is over their charge {charge:e} — {}",
            fuzz::replay()
        );
    }
    for (what, carrier, at, axis, r, small) in others {
        let Some(conic) = carrier.conic() else {
            unreachable!("an ellipse")
        };
        let held = other_charts_within(conic, at, axis, r, small);
        for (chart, within) in ["sphere", "cylinder", "torus"].into_iter().zip(held) {
            assert!(
                within,
                "{what}: the {chart}'s coefficients' exact excess is over their charge — {}",
                fuzz::replay()
            );
        }
    }
}

/// One draw of [`an_ellipses_projected_envelope_covers_its_residual`].
struct Draw {
    chart: usize,
    scale: f64,
    far: f64,
    surface: Surface<f64>,
    carrier: Curve3<f64>,
    span: (f64, f64),
}

/// A draw of the search (its doc lists the draws).
fn draw(s: &mut fuzz::Rng) -> Draw {
    let b = band();
    let sign = |s: &mut fuzz::Rng| if s.below(2) == 0 { 1.0 } else { -1.0 };
    let off = |s: &mut fuzz::Rng| {
        if s.below(3) == 0 {
            s.log_uniform(b.escalate(), 1e-4)
        } else {
            b.zero() * s.range(0.01, 0.9)
        }
    };
    let scale = s.log_uniform(0.05, 20.0);
    let far = if s.below(2) == 0 {
        0.0
    } else {
        s.log_uniform(1.0, 1e3)
    };
    let place = Point3::new(far, -0.5 * far, 0.3 * far);
    let axis = Vec3::new(s.range(-1.0, 1.0), s.range(-1.0, 1.0), 1.0).normalize();
    let u_ref = axis.cross(Vec3::new(0.3, 1.0, -0.2)).normalize();
    let chart = s.below(4);
    let (surface, carrier) = match chart {
        0 => {
            let alpha = s.log_uniform(0.003, 1.567);
            let surface = Surface::Cone {
                apex: place,
                axis,
                half_angle: alpha,
                u_ref,
            };
            let carrier = if s.below(2) == 0 {
                let h = s.log_uniform(1e-4, 2.0) * scale * sign(s);
                let tau = (FRAC_PI_2 - alpha) * s.range(0.0, 0.999);
                let section = cone_section_ellipse(&surface, h, tau);
                let Some(conic) = section.conic() else {
                    unreachable!("a section is a conic")
                };
                let (a, bb) = conic.semi_axes();
                let moved = conic.frame().origin + unit(s) * off(s);
                Curve3::Ellipse(geom::Ellipse::new(
                    geom::Frame::new(moved, conic.frame().axis, conic.frame().u_ref),
                    a,
                    bb,
                ))
            } else {
                let minor = s.log_uniform(1.01 * b.escalate(), 1e-3 * scale);
                let chi = s.range(-1.5, 1.5);
                cone_needle(
                    &surface,
                    sign(s),
                    s.log_uniform(1e-3, 2.0) * scale,
                    (s.log_uniform(1e-3, 0.5) * scale, minor, chi),
                    sign(s) * off(s),
                )
            };
            (surface, carrier)
        }
        1 => {
            let ratio = if s.below(2) == 0 {
                s.log_uniform(1e-3, 0.5)
            } else {
                1.0 - s.log_uniform(1e-5, 0.5)
            };
            let surface = Surface::Torus {
                center: place,
                axis,
                major_radius: scale,
                minor_radius: scale * ratio,
                u_ref,
            };
            let d = (2.0 * scale * ratio * off(s)).sqrt();
            let crest = crest_ellipse(&surface, d);
            let carrier = if s.below(2) == 0 {
                crest
            } else {
                // The bottom crest: the same ellipse mirrored.
                let Some(conic) = crest.conic() else {
                    unreachable!("a crest ellipse is a conic")
                };
                let (a, bb) = conic.semi_axes();
                let lift = (conic.frame().origin - place).dot(axis);
                Curve3::Ellipse(geom::Ellipse::new(
                    geom::Frame::new(
                        conic.frame().origin - axis * (2.0 * lift),
                        conic.frame().axis,
                        conic.frame().u_ref,
                    ),
                    a,
                    bb,
                ))
            };
            (surface, carrier)
        }
        2 => {
            let surface = Surface::Cylinder {
                origin: place,
                axis,
                radius: scale,
                u_ref,
            };
            let carrier = if s.below(2) == 0 {
                let minor = (2.0 * scale * off(s)).sqrt();
                tangent_needle(
                    &surface,
                    (s.log_uniform(1e-3, 1.0) * scale, minor, s.range(-1.5, 1.5)),
                    0.0,
                )
            } else {
                let phi = s.range(0.0, 1.5);
                let (sp, cp) = phi.sin_cos();
                framed_ellipse(
                    &surface,
                    Vec3::new(0.0, 0.0, s.range(-1.0, 1.0) * scale),
                    (Vec3::unit_x(), scale + sign(s) * off(s)),
                    (Vec3::new(0.0, cp, sp), scale / cp),
                )
            };
            (surface, carrier)
        }
        _ => {
            let surface = Surface::Sphere {
                center: place,
                radius: scale,
                axis,
                u_ref,
            };
            let z = scale * s.range(-0.95, 0.95);
            let rho = (scale * scale - z * z).sqrt();
            let carrier = squashed_parallel(&surface, z, off(s) * scale / rho);
            (surface, carrier)
        }
    };
    // Stored larger-first or larger-second, the same set of points.
    let carrier = match carrier {
        Curve3::Ellipse(e) if s.below(2) == 0 => {
            let (n, u) = (e.frame.axis, e.frame.u_ref);
            Curve3::Ellipse(geom::Ellipse::new(
                geom::Frame::new(e.frame.origin, n, n.cross(u)),
                e.semi_v,
                e.semi_u,
            ))
        }
        other => other,
    };
    let offset = unit(s) * (0.3 * b.zero() * f64::from(u8::from(s.below(2) == 0)));
    let Curve3::Ellipse(e) = carrier else {
        unreachable!("the draws are ellipses")
    };
    let carrier = Curve3::Ellipse(geom::Ellipse::new(
        geom::Frame::new(e.frame.origin + offset, e.frame.axis, e.frame.u_ref),
        e.semi_u,
        e.semi_v,
    ));
    let t0 = s.range(-4.0, 4.0);
    let t1 = t0
        + if s.below(4) == 0 {
            TAU
        } else {
            s.range(0.2, 6.0)
        };
    Draw {
        chart,
        scale,
        far,
        surface,
        carrier,
        span: (t0, t1),
    }
}

/// **An ellipse's projected envelope covers its residual** (a
/// counterexample search; the seed varies): the circle rows' search
/// (`circle_lane_rows`) widened to ellipses. Each draw places a chart at
/// a random scale and axis, at the origin or up to `1e3` from it, and an
/// ellipse near it, stored larger-first or larger-second, and mints its
/// projected image directly (`projected::project`, past the class
/// readings, so a carrier at any distance is read):
///
/// - a cone of half-angle `0.003` to `1.567`: its exact section at a
///   height down to `1e-4` of the scale, tilted up to the parabola's
///   threshold less `1e-3` of it (ratio down to about `0.03`), moved off
///   inside the zero band or past it; or a generator needle of minor
///   semi-axis from the escalation band to `1e-3` of the scale;
/// - a torus with `r/R` from `1e-3` to `1 − 1e-5`, and a crest ellipse
///   (top or bottom) narrowed to stand inside the zero band or past it;
/// - a cylinder and a needle in the plane tangent along a ruling, tilted
///   within it, of sagitta inside the zero band or past it; or an oblique
///   section, tilted up to `1.5`, resized;
/// - a sphere and a parallel at a random height squashed to stand inside
///   the zero band or past it.
///
/// Each carrier is moved by an offset inside the zero band and read over
/// a random arc that may cross the azimuth window's cut or make a whole
/// turn. Wherever check 4 reads the envelope, its `f64` and `Interval`
/// envelopes cover the densely sampled residual. Red under
/// `circle-incidence` and `ellipse-term-dropped`.
#[test]
fn an_ellipses_projected_envelope_covers_its_residual() {
    let mut s = fuzz::start("pcurve_cache::ellipse_lane_rows::search");
    let b = band();
    let f = Interval::from_f64;
    let mut tally = [[0usize; 3]; 4];
    for trial in 0..fuzz::scaled(300) {
        let Draw {
            chart,
            scale,
            far,
            surface,
            carrier,
            span: (t0, t1),
        } = draw(&mut s);
        let context = || {
            format!(
                "trial {trial}, chart {chart}, scale {scale:e}, far {far:e}, span ({t0}, {t1}), \
                 carrier {carrier:?} on {surface:?} — {}",
                fuzz::replay()
            )
        };
        let (Ok(image), Ok(image_i)) = (
            projected::project(&carrier, Some((t0, t1)), &surface, b),
            projected::project(
                &carrier.map_scalar(f),
                Some((f(t0), f(t1))),
                &surface.map_scalar(f),
                b,
            ),
        ) else {
            tally[chart][0] += 1;
            continue;
        };
        let (image, image_i) = (
            Pcurve::Projected(Box::new(image)),
            Pcurve::Projected(Box::new(image_i)),
        );
        let Ok((envelope, hi)) = check_4_terms(&image, &image_i, &carrier, (t0, t1), &surface)
        else {
            tally[chart][1] += 1;
            continue;
        };
        tally[chart][2] += 1;
        let sup = sampled_residual(&image, (t0, t1), &carrier, &surface, 2048);
        let noise = 16.0 * f64::EPSILON * (scale + far);
        assert!(
            envelope >= sup - noise && hi >= sup - noise,
            "{}: envelopes {envelope:e} / {hi:e} under the residual {sup:e}",
            context()
        );
    }
    for (chart, [none, refused, read]) in tally.iter().enumerate() {
        println!(
            "[fuzz] ellipse_lane_rows::search, chart {chart}: {none} took no projected image, \
             {refused} refused at check 4, {read} read check 4's envelope"
        );
    }
}

/// **The search's draws as rows for a 50-digit oracle**: each draw whose
/// envelope check 4 reads, one line of its chart, carrier, span and
/// `f64` and `Interval` envelopes, every number in `{:e}` (which rounds
/// trips the `f64`). Evidence for a reviewer, not a gate: run with
/// `CAD_FUZZ_SEED=<seed> cargo nextest run -p geom-brep --lib --run-ignored only ellipse_lane_rows::the_search_as_rows --no-capture`
/// and read the lines with `scripts/oracles/ellipse_lane_50_digit.py`,
/// which holds each envelope against the carrier's 50-digit distance
/// from its chart.
#[test]
#[ignore = "a dump for the 50-digit oracle, read by hand"]
fn the_search_as_rows() {
    let mut s = fuzz::start("pcurve_cache::ellipse_lane_rows::rows");
    let f = Interval::from_f64;
    for _ in 0..fuzz::scaled(600) {
        let Draw {
            surface,
            carrier,
            span: (t0, t1),
            ..
        } = draw(&mut s);
        let (Ok(image), Ok(image_i)) = (
            projected::project(&carrier, Some((t0, t1)), &surface, band()),
            projected::project(
                &carrier.map_scalar(f),
                Some((f(t0), f(t1))),
                &surface.map_scalar(f),
                band(),
            ),
        ) else {
            continue;
        };
        let (image, image_i) = (
            Pcurve::Projected(Box::new(image)),
            Pcurve::Projected(Box::new(image_i)),
        );
        let Ok((envelope, hi)) = check_4_terms(&image, &image_i, &carrier, (t0, t1), &surface)
        else {
            continue;
        };
        let nappe = image.projected().and_then(|p| p.chart().nappe());
        println!(
            "{}",
            oracle_row(&surface, &carrier, nappe, (t0, t1), (envelope, hi))
        );
    }
}

/// One line of [`the_search_as_rows`] for the 50-digit oracle; a cone's
/// carries the image's nappe as the sign `σ` its residual is read on.
fn oracle_row(
    surface: &Surface<f64>,
    carrier: &Curve3<f64>,
    nappe: Option<crate::offset::Nappe>,
    (t0, t1): (f64, f64),
    (envelope, hi): (f64, f64),
) -> String {
    let v = |x: Vec3<f64>| format!("{:e} {:e} {:e}", x.x, x.y, x.z);
    let p = |x: Point3<f64>| v(x - Point3::origin());
    let Curve3::Ellipse(e) = carrier else {
        unreachable!("the rows are ellipses")
    };
    let chart = match *surface {
        Surface::Sphere { center, radius, .. } => format!("sphere {} {radius:e}", p(center)),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => format!("cylinder {} {} {radius:e}", p(origin), v(axis)),
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let Some(nappe) = nappe else {
                unreachable!("a cone's image is read on a nappe")
            };
            format!(
                "cone {} {} {half_angle:e} {:e}",
                p(apex),
                v(axis),
                projected::nappe_sign(nappe)
            )
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => format!(
            "torus {} {} {major_radius:e} {minor_radius:e}",
            p(center),
            v(axis)
        ),
        _ => unreachable!("the rows' charts"),
    };
    format!(
        "ROW {chart} | {} {} {} {:e} {:e} | {t0:e} {t1:e} | {envelope:e} {hi:e}",
        p(e.frame.origin),
        v(e.frame.axis),
        v(e.frame.u_ref),
        e.semi_u,
        e.semi_v
    )
}

/// A pinned extreme: its name, chart, ellipse, span and 50-digit distance.
type Extreme = (&'static str, Surface<f64>, Curve3<f64>, (f64, f64), f64);

/// The pinned extremes of [`the_extremes_are_covered_at_their_50_digit_distance`]:
/// a name, the chart, the ellipse, the span, and the carrier's 50-digit
/// distance from the chart over the span
/// (`scripts/oracles/ellipse_lane_50_digit.py`), rounded to the nearest
/// `f64`.
fn extremes() -> Vec<Extreme> {
    let far = |d: f64| Point3::new(0.3 + d, -0.7 - 0.6 * d, 0.2 + 0.5 * d);
    let axis = Vec3::new(0.2, -0.4, 1.0).normalize();
    let u_ref = axis.cross(Vec3::unit_x()).normalize();
    let cone_at = |d: f64, alpha: f64| Surface::Cone {
        apex: far(d),
        axis,
        half_angle: alpha,
        u_ref,
    };
    let needle = cone_at(1e5, 0.003);
    let flat = cone_at(1e5, FRAC_PI_2 - 1e-3);
    let thin = Surface::Torus {
        center: far(1e3),
        axis,
        major_radius: 2.0,
        minor_radius: 2e-3,
        u_ref,
    };
    let ball = Surface::Sphere {
        center: far(1e3),
        radius: 4.6,
        axis,
        u_ref,
    };
    let wall = Surface::Cylinder {
        origin: far(1e3),
        axis,
        radius: 1.0,
        u_ref,
    };
    vec![
        (
            "a needle cone's section 1e5 out",
            needle.clone(),
            cone_section_ellipse(&needle, 1.0, 0.5 * (FRAC_PI_2 - 0.003)),
            (0.0, TAU),
            5.465_458_614_727_484e-12,
        ),
        (
            "a flat cone's section 1e5 out",
            flat.clone(),
            cone_section_ellipse(&flat, 1e-3, 1e-4),
            (0.3, 2.5),
            1.873_556_395_183_807_6e-12,
        ),
        (
            "a thin torus's crest ellipse 1e3 out",
            thin.clone(),
            crest_ellipse(&thin, 4.5e-7),
            (0.0, TAU),
            5.068_641_677_846_517e-11,
        ),
        (
            "a sphere's squashed parallel 1e3 out",
            ball.clone(),
            squashed_parallel(&ball, 1.1, 7.2e-10),
            (-1.0, 2.0),
            6.991_683_275_536_129e-10,
        ),
        (
            "a wall's tangent needle 1e3 out",
            wall.clone(),
            tangent_needle(&wall, (0.3, 3.7e-5, 0.0), 0.0),
            (0.0, TAU),
            6.845_443_184_123_417e-10,
        ),
    ]
}

/// **The extremes are covered at their 50-digit distance**: each pinned
/// ellipse's `f64` envelope, and the upper end of its `Interval`
/// envelope (the end the certificate reads), is at least the carrier's
/// distance from its chart over the span, measured at 50 digits from the
/// stored values. The carriers are literals, so every ε reads the same
/// distances.
#[test]
fn the_extremes_are_covered_at_their_50_digit_distance() {
    let f = Interval::from_f64;
    let b = band();
    for (what, surface, carrier, (t0, t1), exact) in extremes() {
        let p = projected::project(&carrier, Some((t0, t1)), &surface, b)
            .unwrap_or_else(|e| panic!("{what}: projects: {e:?}"));
        let (t, _) = projected::projected_envelope(
            &p,
            t0,
            t1,
            &p.chart_box(t0, t1),
            &carrier,
            &surface,
            b,
            None,
        )
        .unwrap_or_else(|e| panic!("{what}: the envelope reads: {e:?}"));
        let (ci, si) = (carrier.map_scalar(f), surface.map_scalar(f));
        let pi = projected::project(&ci, Some((f(t0), f(t1))), &si, b)
            .unwrap_or_else(|e| panic!("{what}: projects at Interval: {e:?}"));
        let (ti, _) = projected::projected_envelope(
            &pi,
            f(t0),
            f(t1),
            &pi.chart_box(f(t0), f(t1)),
            &ci,
            &si,
            b,
            None,
        )
        .unwrap_or_else(|e| panic!("{what}: the envelope reads at Interval: {e:?}"));
        println!(
            "{}",
            oracle_row(
                &surface,
                &carrier,
                p.chart().nappe(),
                (t0, t1),
                (t.total(), ti.total().hi())
            )
        );
        assert!(
            t.total() >= exact && ti.total().hi() >= exact,
            "{what}: the envelopes {:e} (f64) and {:?} (Interval) are under the 50-digit \
             distance {exact:e}",
            t.total(),
            ti.total()
        );
    }
}

/// The edge certificate of `carrier` over `t ∈ [0.3, 2.5]`, described on
/// the unit sphere's chart, at the carrier's scalar `$t`: the door the
/// semi-axis rows below read. A macro, so each scalar names its own
/// certificate rather than a compound bound written over both.
macro_rules! certify_on_the_unit_sphere {
    ($t:ty, $carrier:expr) => {{
        use crate::{EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, SurfaceKey};
        let carrier: &Curve3<$t> = $carrier;
        let sphere = Surface::Sphere {
            center: Point3::origin(),
            radius: 1.0,
            axis: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        }
        .map_scalar(<$t>::from_f64);
        let mut surfaces: slotmap::SlotMap<SurfaceKey, Surface<$t>> = slotmap::SlotMap::with_key();
        let key = surfaces.insert(sphere);
        let (t0, t1) = (<$t>::from_f64(0.3), <$t>::from_f64(2.5));
        EdgeCurve::certify(
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::chart(key),
                carrier: carrier.clone(),
                param_start: t0,
                param_end: t1,
            },
            carrier.eval(t0),
            carrier.eval(t1),
            |k| surfaces.get(k).cloned(),
            geom_core::test_support::fixture_lever(<$t>::one()),
            band(),
            crate::NurbsLane::certified(),
        )
        .map(|_| ())
    }};
}

/// The frame of the sphere's parallel at height `0.6` the semi-axis rows
/// store their conics in.
fn parallel_frame<T: Real>() -> geom::Frame<T> {
    geom::Frame::new(
        Point3::new(T::zero(), T::zero(), T::from_f64(0.6)),
        Vec3::unit_z(),
        Vec3::unit_x(),
    )
}

/// The three door endings a stored semi-axis refusal renders: the lever
/// at a build, where the record is the caller's input (the kernel mints
/// none), the stored culprit's defect at rest and a damaged file at the
/// import door.
fn semi_axis_doors(err: &crate::CertifyError, what: &str) {
    use crate::recourse::{ReadAt, Reading};
    let lever = geom_core::lever_recourse(crate::certify::SEMI_AXIS_LEVER, None);
    for (at, culprit) in [
        (ReadAt::Run(Reading::Build), lever.as_str()),
        (
            ReadAt::Run(Reading::AtRest),
            geom_core::KERNEL_OR_FILE_DEFECT_ENDING,
        ),
        (
            ReadAt::File(geom_core::FileCoincidence::new(1e-6)),
            geom_core::DAMAGED_FILE_ENDING,
        ),
    ] {
        assert_eq!(err.ending(at).as_deref(), Some(culprit), "{what} at {at:?}");
    }
}

/// **A conic stored with a negative semi-axis refuses typed at the edge
/// certificate** (`CertifyError::SemiAxisNegative`), never as a span
/// that does not run forward: the parallel of the unit sphere at height
/// `0.6` stored as an ellipse with its first, its second or both
/// semi-axes negative, and as a circle of negative radius, each described
/// on the sphere's chart, at `f64` and at `Interval`. The positive
/// spelling certifies, and a circle of zero radius, a collapsed length,
/// is the span check's (`StatedInterval`). Each refusal renders its own
/// lever at a build, where it is the caller's input (the kernel mints
/// none), the stored culprit's defect at rest and a damaged file at the
/// import door. Red under `semi-axis-unchecked`.
#[test]
fn a_conic_stored_with_a_negative_semi_axis_refuses_typed() {
    use crate::CertifyError;
    let frame = parallel_frame::<f64>();
    let ellipse = |a: f64, b: f64| Curve3::Ellipse(geom::Ellipse::new(frame, a, b));
    let circle = |r: f64| Curve3::Circle(geom::Circle::new(frame, r));
    let (rho, squash) = (0.8, 0.8 - 0.25 * zero());
    let certify = |carrier: &Curve3<f64>| {
        [
            ("f64", certify_on_the_unit_sphere!(f64, carrier)),
            (
                "Interval",
                certify_on_the_unit_sphere!(Interval, &carrier.map_scalar(Interval::from_f64)),
            ),
        ]
    };
    for (lane, got) in certify(&circle(0.0)) {
        assert!(
            matches!(got, Err(CertifyError::StatedInterval { .. })),
            "a circle of zero radius is the span check's ({lane}): {got:?}"
        );
    }
    for (lane, got) in certify(&ellipse(rho, squash)) {
        assert!(
            got.is_ok(),
            "the positive spelling certifies ({lane}): {got:?}"
        );
    }
    for (what, carrier) in [
        ("the first semi-axis negative", ellipse(-rho, squash)),
        ("the second negative", ellipse(rho, -squash)),
        ("both negative", ellipse(-rho, -squash)),
        ("a circle of negative radius", circle(-rho)),
    ] {
        for (lane, got) in certify(&carrier) {
            let Err(err @ CertifyError::SemiAxisNegative { .. }) = &got else {
                panic!("{what} ({lane}): refused as a negative semi-axis: {got:?}")
            };
            semi_axis_doors(err, &format!("{what} ({lane})"));
        }
    }
}

/// **A conic stored with a semi-axis that is not a finite number refuses
/// as that** (`CertifyError::SemiAxisNotFinite`), the same ending at
/// `f64` and at `Interval`: NaN, `+∞` and `−∞` as either semi-axis of an
/// ellipse and as a circle's radius, on the sphere's chart. None is read
/// as a sign: `−∞` is no more a negative length than NaN is. Each renders
/// the semi-axis lever at a build, the stored culprit's defect at rest
/// and a damaged file at the import door. Red under
/// `semi-axis-unchecked`.
#[test]
fn a_conic_stored_with_a_semi_axis_not_finite_refuses_as_that() {
    use crate::CertifyError;
    let frame = parallel_frame::<f64>();
    let rho = 0.8;
    for (name, bad) in [
        ("NaN", f64::NAN),
        ("+∞", f64::INFINITY),
        ("−∞", f64::NEG_INFINITY),
    ] {
        for (what, carrier) in [
            (
                "the first semi-axis",
                Curve3::Ellipse(geom::Ellipse::new(frame, bad, rho)),
            ),
            (
                "the second",
                Curve3::Ellipse(geom::Ellipse::new(frame, rho, bad)),
            ),
            (
                "a circle's radius",
                Curve3::Circle(geom::Circle::new(frame, bad)),
            ),
        ] {
            for (lane, got) in [
                ("f64", certify_on_the_unit_sphere!(f64, &carrier)),
                (
                    "Interval",
                    certify_on_the_unit_sphere!(Interval, &carrier.map_scalar(Interval::from_f64)),
                ),
            ] {
                let Err(err @ CertifyError::SemiAxisNotFinite { .. }) = &got else {
                    panic!("{what} {name} ({lane}): refused as not finite: {got:?}")
                };
                semi_axis_doors(err, &format!("{what} {name} ({lane})"));
            }
        }
    }
}

/// **A semi-axis read in band is not a definite defect**: an `Interval`
/// ellipse on the sphere's parallel whose first semi-axis encloses
/// `[−1e-20, 0.8]` (its sign undecided) is not refused as a negative
/// semi-axis, a stored defect at rest; it falls through to the span
/// meter, which reads its winding in band (`Escalated`), and that ends
/// in a lever at every door (E1d). The
/// definite spelling, `[−0.8, −1e-20]`, is the defect.
#[test]
fn an_interval_semi_axis_straddling_zero_ends_in_its_lever() {
    use crate::CertifyError;
    use crate::recourse::{ReadAt, Reading};
    let frame = parallel_frame::<Interval>();
    let f = Interval::from_f64;
    let ellipse = |a: Interval| Curve3::Ellipse(geom::Ellipse::new(frame, a, f(0.8)));
    let got = certify_on_the_unit_sphere!(Interval, &ellipse(Interval::from_bounds(-1e-20, 0.8)));
    let Err(err) = &got else {
        panic!("a straddling semi-axis is not certified: {got:?}")
    };
    assert!(
        !matches!(
            err,
            CertifyError::SemiAxisNegative { .. } | CertifyError::SemiAxisNotFinite { .. }
        ),
        "a straddling semi-axis is not a definite stored defect: {err:?}"
    );
    assert!(
        matches!(err, CertifyError::Escalated { .. }),
        "the span meter reads the straddle in band: {err:?}"
    );
    for at in [
        ReadAt::Run(Reading::Build),
        ReadAt::Run(Reading::AtRest),
        ReadAt::File(geom_core::FileCoincidence::new(1e-6)),
    ] {
        let ending = err.ending(at).unwrap_or_default();
        assert!(
            ending.contains("Recourse: ")
                && ending != geom_core::KERNEL_OR_FILE_DEFECT_ENDING
                && ending != geom_core::DAMAGED_FILE_ENDING,
            "a straddling semi-axis ends in its lever at {at:?}: {ending:?} ({err:?})"
        );
    }
    let definite =
        certify_on_the_unit_sphere!(Interval, &ellipse(Interval::from_bounds(-0.8, -1e-20)));
    assert!(
        matches!(definite, Err(CertifyError::SemiAxisNegative { .. })),
        "a definitely negative enclosure is the defect: {definite:?}"
    );
}
