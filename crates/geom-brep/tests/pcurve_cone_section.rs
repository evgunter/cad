//! The tilted plane × cone ellipse's exact chart image on its cone
//! (`Pcurve::FocalSection`'s cone instance): derived by `chart_pcurve`, exact against
//! the carrier through the chart map, certified with a rounding-scale
//! envelope, and refused when any of its numbers is wrong.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::surf::at_apex;
use crate::shared::tol::{band, eps};
use geom::{Curve3, Surface};
use geom_brep::intersect::{PlaneConeSection, plane_cone_section};
use geom_brep::{EnvelopeStatement, FocalImage, Pcurve, PcurveCache, chart_pcurve};
use geom_core::{Affine3, Bounds, Interval, Point3, Real, Vec3};

/// A cone off the origin about a skew axis, given with either axis
/// sign. The two are the same locus; the cut below lies on the `+axis`
/// nappe of the first and the mirror nappe of the second.
fn cone(axis_sign: f64) -> Surface<f64> {
    cone_at(axis_sign, 0.45)
}

/// [`cone`] at another half-angle.
fn cone_at(axis_sign: f64, half_angle: f64) -> Surface<f64> {
    let axis = Vec3::new(0.2, -0.3, 1.0).normalize() * axis_sign;
    let seed = Vec3::unit_x();
    Surface::Cone {
        apex: Point3::new(0.4, -0.7, 0.2),
        axis,
        half_angle,
        u_ref: (seed - axis * seed.dot(axis)).normalize(),
    }
}

/// The section of `cone(+1)` by a plane tilted 0.5 rad off its axis,
/// 2.5 along it from the apex: an ellipse on the `+axis` nappe.
fn section() -> Curve3<f64> {
    section_of(0.45, 0.5)
}

/// The section of `cone_at(1, half_angle)` by a plane tilted `tilt` off
/// its axis, 2.5 along it from the apex.
fn section_of(half_angle: f64, tilt: f64) -> Curve3<f64> {
    let surface = cone_at(1.0, half_angle);
    let Surface::Cone { apex, axis, .. } = surface else {
        unreachable!()
    };
    let side = axis.cross(Vec3::unit_x()).normalize();
    let normal = (axis * tilt.cos() + side * tilt.sin()).normalize();
    let plane = Surface::Plane {
        origin: apex + axis * 2.5,
        normal,
        u_ref: side.cross(normal),
    };
    let Ok(PlaneConeSection::TiltedEllipse(e)) =
        plane_cone_section(&plane, &surface, &at_apex(&surface, 3.0), band())
    else {
        panic!("a {tilt} rad tilt of a {half_angle} rad cone is an ellipse");
    };
    e
}

/// The same locus traversed backwards (the arc-side rule's cw arm).
fn reversed(e: &Curve3<f64>) -> Curve3<f64> {
    let &Curve3::Ellipse {
        center,
        axis,
        major,
        minor,
        u_ref,
    } = e
    else {
        unreachable!()
    };
    Curve3::Ellipse {
        center,
        axis: -axis,
        major,
        minor,
        u_ref,
    }
}

/// Every pairing of nappe and traversal: the image is the
/// cone instance of the `FocalSection` form, maps onto the carrier at rounding scale over two
/// periods (the azimuth's branch never jumps), and certifies over a
/// span longer than a half-turn with a rounding-scale envelope.
#[test]
fn the_image_is_exact_on_both_nappes_and_both_traversals() {
    for axis_sign in [1.0, -1.0] {
        let surface = cone(axis_sign);
        for carrier in [section(), reversed(&section())] {
            let what = format!("axis sign {axis_sign}, carrier {carrier:?}");
            let image = chart_pcurve(&carrier, &surface, band()).unwrap();
            let Pcurve::FocalSection(FocalImage { beta, sense, .. }) = image else {
                panic!("{what}: expected the cone-section image, got {image:?}");
            };
            assert!(
                beta.abs() > 0.05,
                "{what}: a genuinely eccentric projection"
            );
            assert_eq!(sense.abs(), 1.0, "{what}");
            for i in 0..=64 {
                let t = -3.0 + 9.0 * f64::from(i) / 64.0;
                let uv = image.eval(t);
                let d = surface.eval(uv.x, uv.y).distance(carrier.eval(t));
                assert!(d < 1e-12, "{what}: t {t}: {d:e}");
            }
            let cache = PcurveCache::certify(image, 0.3, 4.0, &carrier, &surface, band())
                .unwrap_or_else(|e| panic!("{what}: {e}"));
            let cert = cache.certificate();
            assert!(
                cert.envelope < 1e-12,
                "{what}: envelope {:e}",
                cert.envelope
            );
            assert!(
                matches!(cert.statement, EnvelopeStatement::MapResidualClosedForm),
                "{what}"
            );
        }
    }
}

/// Each number of the image is load-bearing: one moved by `1e-6`, or
/// the image offered against a different cone, refuses certification.
#[test]
fn a_wrong_number_in_the_image_refuses() {
    let surface = cone(1.0);
    let carrier = section();
    let image = chart_pcurve(&carrier, &surface, band()).unwrap();
    let Pcurve::FocalSection(FocalImage {
        u0,
        t0,
        v0,
        va,
        vb,
        vl,
        beta,
        sense,
    }) = image
    else {
        unreachable!()
    };
    let h = 1e-6;
    let wrong = [
        ("u0", (u0 + h, t0, v0, va, vb, vl, beta, sense)),
        ("t0", (u0, t0 + h, v0, va, vb, vl, beta, sense)),
        ("v0", (u0, t0, v0 + h, va, vb, vl, beta, sense)),
        ("va", (u0, t0, v0, va + h, vb, vl, beta, sense)),
        ("vb", (u0, t0, v0, va, vb + h, vl, beta, sense)),
        ("vl", (u0, t0, v0, va, vb, vl + h, beta, sense)),
        ("beta", (u0, t0, v0, va, vb, vl, beta + h, sense)),
        ("sense", (u0, t0, v0, va, vb, vl, beta, -sense)),
    ];
    for (name, (u0, t0, v0, va, vb, vl, beta, sense)) in wrong {
        let bad = Pcurve::FocalSection(FocalImage {
            u0,
            t0,
            v0,
            va,
            vb,
            vl,
            beta,
            sense,
        });
        assert!(
            PcurveCache::certify(bad, 0.3, 4.0, &carrier, &surface, band()).is_err(),
            "{name} moved by {h} still certifies"
        );
    }
    let Surface::Cone {
        apex,
        axis,
        half_angle,
        u_ref,
    } = surface
    else {
        unreachable!()
    };
    let other = Surface::Cone {
        apex,
        axis,
        half_angle: half_angle + h,
        u_ref,
    };
    assert!(
        PcurveCache::certify(image, 0.3, 4.0, &carrier, &other, band()).is_err(),
        "the image certified against another cone"
    );
}

/// The envelope BOUNDS what it certifies. An image with `va` or `vb`
/// moved by a quarter of ε still certifies, and its residual is then
/// the moved slant `δ·(cos t | sin t)` along a unit generator — the
/// harmonic part carries only `cos α` of it, the Kepler remainder the
/// `sin α` rest. The certified envelope must cover the residual sampled
/// densely over the span. At `α = 1.2`, `cos α + sin α / 2 < 1`, so even
/// a HALVED remainder leaves the envelope under the residual: a
/// remainder dropped or shrunk is red here.
#[test]
fn the_envelope_covers_an_admitted_slant_error() {
    for (half_angle, tilt) in [(0.45, 0.5), (1.2, 0.2)] {
        let surface = cone_at(1.0, half_angle);
        let carrier = section_of(half_angle, tilt);
        let image = chart_pcurve(&carrier, &surface, band()).unwrap();
        let Pcurve::FocalSection(FocalImage {
            u0,
            t0: vertex,
            v0,
            va,
            vb,
            vl,
            beta,
            sense,
        }) = image
        else {
            unreachable!()
        };
        let h = 0.25 * crate::shared::tol::eps();
        for (name, va, vb) in [("va", va + h, vb), ("vb", va, vb + h)] {
            let what = format!("alpha {half_angle}, {name} moved by eps/4");
            let moved = Pcurve::FocalSection(FocalImage {
                u0,
                t0: vertex,
                v0,
                va,
                vb,
                vl,
                beta,
                sense,
            });
            let (t0, t1) = (0.3, 4.0);
            let cache = PcurveCache::certify(moved.clone(), t0, t1, &carrier, &surface, band())
                .unwrap_or_else(|e| panic!("{what}: {e}"));
            let envelope = cache.certificate().envelope;
            let sup = (0..=4096)
                .map(|i| {
                    let t = t0 + (t1 - t0) * f64::from(i) / 4096.0;
                    let uv = moved.eval(t);
                    surface.eval(uv.x, uv.y).distance(carrier.eval(t))
                })
                .fold(0.0, f64::max);
            assert!(sup > 0.9 * h, "{what}: off its carrier by {sup:e}");
            assert!(
                envelope >= sup,
                "{what}: envelope {envelope:e} under the sampled residual {sup:e}"
            );
        }
    }
}

/// The probe's generic rigid motion: a skew pose holds no exact zero
/// off the plane of the axis and the plane's normal.
fn skew() -> Affine3<f64> {
    Affine3::rotation_about_axis(
        Point3::new(0.37, -0.21, 0.13),
        Vec3::new(0.3, 0.5, 0.8).normalize(),
        0.7,
    )
}

/// A frustum's cone (half-angle `alpha`, height `s`, top radius `s/2`,
/// about `−y`) cut by the plane through `(0, s/2, 0)` whose normal
/// stands `f·(π/2 − α)` off the axis, all moved by [`skew`]: the
/// section, its cone, and `k = √(1 − e²)` for the projection's
/// eccentricity `e = tan α·tan φ`; or what the arm answered instead.
fn skew_section(alpha: f64, s: f64, f: f64) -> Result<(Curve3<f64>, Surface<f64>, f64), String> {
    let m = skew();
    let (top, foot) = (0.5 * s, 0.5 * s + s * alpha.tan());
    let surface = Surface::Cone {
        apex: m.transform_point(Point3::new(0.0, s + top / alpha.tan(), 0.0)),
        axis: m.transform_vec(Vec3::new(0.0, -1.0, 0.0)),
        half_angle: alpha,
        u_ref: m.transform_vec(Vec3::unit_x()),
    };
    let phi = f * (core::f64::consts::FRAC_PI_2 - alpha);
    let plane = Surface::Plane {
        origin: m.transform_point(Point3::new(0.0, 0.5 * s, 0.0)),
        normal: m.transform_vec(Vec3::new(phi.sin(), phi.cos(), 0.0)),
        u_ref: m.transform_vec(Vec3::unit_z()),
    };
    let across = (2.0 * foot).max((s * s + (foot + top).powi(2)).sqrt());
    let e = match plane_cone_section(&plane, &surface, &at_apex(&surface, across), band()) {
        Ok(PlaneConeSection::TiltedEllipse(e)) => e,
        other => return Err(format!("the arm answers {other:?}")),
    };
    let ecc = alpha.tan() * phi.tan();
    Ok((e, surface, (1.0 - ecc * ecc).sqrt()))
}

/// `image` with its vertex azimuth read off the cancelling sum
/// `r0 + ra` instead of off `ra`: the kernel's old arithmetic replayed
/// (`radial(center − apex) + radial(e_u·major)`, turned by the nappe's
/// sign), with `atan2` for `stable_azimuth`, which agrees with it mod τ.
fn read_off_the_sum(
    image: &Pcurve<f64>,
    carrier: &Curve3<f64>,
    surface: &Surface<f64>,
) -> Pcurve<f64> {
    let (
        &Pcurve::FocalSection(focal),
        &Curve3::Ellipse {
            center,
            major,
            u_ref: e_u,
            ..
        },
        &Surface::Cone {
            apex, axis, u_ref, ..
        },
    ) = (image, carrier, surface)
    else {
        unreachable!()
    };
    let radial = |v: Vec3<f64>| v - axis * v.dot(axis);
    let w = center - apex;
    let start = (radial(w) + radial(e_u * major)) * w.dot(axis).signum();
    let u0 = start.dot(axis.cross(u_ref)).atan2(start.dot(u_ref));
    Pcurve::FocalSection(FocalImage { u0, ..focal })
}

/// `sup |S(P(t)) − C(t)|` over a whole turn.
fn whole_turn_miss(image: &Pcurve<f64>, carrier: &Curve3<f64>, surface: &Surface<f64>) -> f64 {
    (0..1024)
        .map(|i| {
            let t = core::f64::consts::TAU * f64::from(i) / 1024.0;
            let uv = image.eval(t);
            surface.eval(uv.x, uv.y).distance(carrier.eval(t))
        })
        .fold(0.0, f64::max)
}

/// **The vertex azimuth is read off `ra`, not off `r0 + ra`.** Near a
/// parabola `r0 + ra = (1 − e)·ra` cancels to `A·k²/2`, so in a skew
/// pose the centre's off-plane rounding (about `u·A`) turns the image by
/// about `u/k²` and it misses its far side by about `u·A/k²`. At
/// `α = atan ½`, scale `0.1`, `k ≈ 2.4e-3` (`A ≈ 3e4`) the image read off
/// the sum misses by about `1e-6`, and the minted one by about `u·A/k`.
///
/// What certification makes of the two depends on ε, so each verdict is
/// asserted where its miss puts it: the image read off the sum refuses
/// wherever its miss clears the band, and the minted one certifies
/// wherever its miss is under a quarter of ε — at `f64` and lifted to
/// `Interval`. At ε = 1e-9 both are asserted. The pose itself exists
/// only near that ε: at 1e-6 the arm's own band refuses it, and at 1e-12
/// the carrier's rounding (`u·A ≈ 7e-12`) is past the chart's incidence
/// band; the row says so, and is required to reach its verdicts at the
/// default ε. (Minted AT `Interval` it certifies too:
/// [`a_near_parabola_section_minted_at_interval_certifies`].)
#[test]
fn a_skew_section_near_the_parabola_certifies() {
    let at_default = eps() == geom_core::tolerance::DEFAULT_EPS;
    let reached = skew_section(0.5_f64.atan(), 0.1, 0.999_999).and_then(|(c, s, k)| {
        let image = chart_pcurve(&c, &s, band()).map_err(|e| format!("the chart answers {e}"))?;
        Ok((c, s, k, image))
    });
    let (carrier, surface, k, image) = match reached {
        Ok(found) => found,
        Err(why) => {
            assert!(!at_default, "the pose is unreached at the default ε: {why}");
            println!("SKIPPED at ε {:e}: no image to certify ({why})", eps());
            return;
        }
    };
    assert!((2e-3..3e-3).contains(&k), "k {k:e}");
    let (t0, t1) = (-1.0, 1.0);
    let minted = whole_turn_miss(&image, &carrier, &surface);
    let summed = read_off_the_sum(&image, &carrier, &surface);
    let summed_miss = whole_turn_miss(&summed, &carrier, &surface);
    assert!(
        summed_miss > 1000.0 * minted,
        "read off the sum {summed_miss:e}, minted {minted:e}"
    );

    let lift = <Interval as Real>::from_f64;
    let (lc, ls) = (carrier.map_scalar(lift), surface.map_scalar(lift));
    let certify_lifted = |image: &Pcurve<f64>| {
        let &Pcurve::FocalSection(f) = image else {
            unreachable!()
        };
        let lifted = Pcurve::FocalSection(FocalImage {
            u0: lift(f.u0),
            t0: lift(f.t0),
            v0: lift(f.v0),
            va: lift(f.va),
            vb: lift(f.vb),
            vl: lift(f.vl),
            beta: lift(f.beta),
            sense: lift(f.sense),
        });
        PcurveCache::certify(lifted, lift(t0), lift(t1), &lc, &ls, band())
    };
    let refuses = summed_miss > band().escalate();
    if refuses {
        assert!(
            PcurveCache::certify(summed.clone(), t0, t1, &carrier, &surface, band()).is_err(),
            "the image read off the sum ({summed_miss:e}) certifies"
        );
        assert!(
            certify_lifted(&summed).is_err(),
            "the image read off the sum ({summed_miss:e}) certifies at Interval"
        );
    }
    let certifies = minted < 0.25 * eps();
    if certifies {
        let cache = PcurveCache::certify(image.clone(), t0, t1, &carrier, &surface, band())
            .unwrap_or_else(|e| panic!("f64: {e}"));
        let envelope = cache.certificate().envelope;
        assert!(envelope < 0.5 * eps(), "f64 envelope {envelope:e}");
        let cache = certify_lifted(&image).unwrap_or_else(|e| panic!("Interval: {e}"));
        let envelope = cache.certificate().envelope.hi();
        assert!(envelope < 0.5 * eps(), "Interval envelope {envelope:e}");
    }
    println!(
        "eps {:e}: the sum's refusal {}, the mint's certificate {}",
        eps(),
        if refuses {
            "asserted"
        } else {
            "not reached (its miss is inside the band)"
        },
        if certifies {
            "asserted"
        } else {
            "not reached (its miss is past ε/4)"
        },
    );
    assert!(
        refuses || certifies,
        "neither verdict is reached at ε {:e}",
        eps()
    );
}

/// The exact section of the cone `ρ = tan α·z` (apex at the origin,
/// axis `z`) by the plane through `(0, 0, h)` tilted `φ` about `y`, its
/// vertices `z = h/(1 ∓ tan α·tan φ)` on `y = 0`, with its centre moved
/// `off` along `y` (off the plane of the axis and the major axis), all
/// moved by [`skew`]. Built by hand because at `|e| ~ 1e-11` the arm
/// decides a circle.
fn skew_near_circle(alpha: f64, h: f64, phi: f64, off: f64) -> (Curve3<f64>, Surface<f64>) {
    let m = skew();
    let (ta, tp) = (alpha.tan(), phi.tan());
    let (z1, z2) = (h / (1.0 - ta * tp), h / (1.0 + ta * tp));
    let (p1, p2) = (
        Point3::new(ta * z1, 0.0, z1),
        Point3::new(-ta * z2, 0.0, z2),
    );
    let center = p2 + (p1 - p2) * 0.5;
    let (cx, cz) = (center.x, center.z);
    let u_ref = (p1 - p2).normalize();
    let carrier = Curve3::Ellipse {
        center: m.transform_point(center + Vec3::unit_y() * off),
        axis: m.transform_vec(u_ref.cross(Vec3::unit_y())),
        major: 0.5 * (p1 - p2).norm(),
        minor: (ta * ta * cz * cz - cx * cx).sqrt(),
        u_ref: m.transform_vec(u_ref),
    };
    let surface = Surface::Cone {
        apex: m.transform_point(Point3::new(0.0, 0.0, 0.0)),
        axis: m.transform_vec(Vec3::unit_z()),
        half_angle: alpha,
        u_ref: m.transform_vec(Vec3::unit_x()),
    };
    (carrier, surface)
}

/// **Near a circle the sum does not cancel, and reading `ra` alone is
/// still no worse.** A centre `δ` off the plane of the axis and the
/// major axis (inside the band, as check 1 admits it) puts `r0` off
/// `ra`'s line by `δ`. Read off the sum, `u0` turns the whole image to
/// meet the carrier at `t = 0` and misses its far side by about `2δ`;
/// read off `ra`, it misses by about `δ` everywhere.
#[test]
fn a_near_circular_section_off_its_plane_misses_by_its_offset_not_twice() {
    let alpha = 0.5_f64.atan();
    let off = 0.1 * eps();
    let (carrier, surface) = skew_near_circle(alpha, 2.0, 2e-11 / alpha.tan(), off);
    let image = chart_pcurve(&carrier, &surface, band()).unwrap();
    let Pcurve::FocalSection(FocalImage { beta, .. }) = image else {
        unreachable!()
    };
    assert!(beta.abs() < 1e-10, "a near-circle: beta {beta:e}");
    let minted = whole_turn_miss(&image, &carrier, &surface);
    let summed = whole_turn_miss(
        &read_off_the_sum(&image, &carrier, &surface),
        &carrier,
        &surface,
    );
    assert!(
        minted < 1.5 * off && summed > 1.5 * off,
        "minted {minted:e}, read off the sum {summed:e}, centre off by {off:e}"
    );
}

/// **Minted AT `Interval`, the vertex azimuth is tight.** Read off the
/// cancelling sum, `u0`'s enclosure widened like `u/k²` (1.6e-13 at
/// `f = 0.99`, 1.3e-11 at 0.9999, 1.3e-9 at 0.999999 on this pose), and
/// at `f = 0.9999` the Interval mint escalated; read off `ra` it is
/// about 4e-15 throughout and that mint certifies. Each `f` the arm
/// reaches at this ε is asserted, and the default ε must reach
/// `f = 0.9999`. (`β`'s enclosure:
/// [`beta_encloses_to_rounding_however_near_the_parabola`].)
#[test]
fn a_skew_section_minted_at_interval_reads_a_tight_vertex_azimuth() {
    let lift = <Interval as Real>::from_f64;
    let at_default = eps() == geom_core::tolerance::DEFAULT_EPS;
    let mut certified = false;
    for f in [0.99, 0.9999, 0.999_999] {
        let reached = skew_section(0.5_f64.atan(), 0.1, f).and_then(|(c, s, _)| {
            let (lc, ls) = (c.map_scalar(lift), s.map_scalar(lift));
            let image =
                chart_pcurve(&lc, &ls, band()).map_err(|e| format!("the chart answers {e}"))?;
            Ok((lc, ls, image))
        });
        let (lc, ls, image) = match reached {
            Ok(found) => found,
            Err(why) => {
                println!("SKIPPED f {f} at ε {:e}: {why}", eps());
                continue;
            }
        };
        let Pcurve::FocalSection(focal) = &image else {
            unreachable!()
        };
        let width = focal.u0.hi() - focal.u0.lo();
        assert!(width < 100.0 * f64::EPSILON, "f {f}: u0 width {width:e}");
        if f == 0.9999 {
            let cache =
                PcurveCache::certify(image.clone(), lift(-1.0), lift(1.0), &lc, &ls, band())
                    .unwrap_or_else(|e| panic!("f {f}: {e}"));
            let envelope = cache.certificate().envelope.hi();
            assert!(envelope < 0.5 * eps(), "f {f}: envelope {envelope:e}");
            certified = true;
        }
    }
    assert!(
        certified || !at_default,
        "f 0.9999 is unreached at the default ε"
    );
}

/// The skew poses of [`skew_section`] the arm reaches at this ε, minted
/// at `Interval` on the lifted carrier and chart, with what did not
/// reach printed. The default ε must reach all of them.
fn interval_mints(
    poses: &[(f64, f64)],
) -> Vec<(
    f64,
    f64,
    f64,
    Curve3<Interval>,
    Surface<Interval>,
    FocalImage<Interval>,
)> {
    let lift = <Interval as Real>::from_f64;
    let at_default = eps() == geom_core::tolerance::DEFAULT_EPS;
    let mut reached = Vec::new();
    for &(scale, f) in poses {
        let minted = skew_section(0.5_f64.atan(), scale, f).and_then(|(c, s, k)| {
            let (lc, ls) = (c.map_scalar(lift), s.map_scalar(lift));
            match chart_pcurve(&lc, &ls, band()) {
                Ok(Pcurve::FocalSection(image)) => Ok((k, lc, ls, image)),
                Ok(other) => Err(format!("the chart answers {other:?}")),
                Err(e) => Err(format!("the chart answers {e}")),
            }
        });
        match minted {
            Ok((k, lc, ls, image)) => reached.push((scale, f, k, lc, ls, image)),
            Err(why) => {
                assert!(
                    !at_default,
                    "scale {scale}, f {f} is unreached at the default ε: {why}"
                );
                println!("SKIPPED scale {scale}, f {f} at ε {:e}: {why}", eps());
            }
        }
    }
    reached
}

/// **`β` is read off the projection's axis ratio.** `β = e/(1 + √(1 − e²))`
/// with the root taken of `1 − e²` gains `1/k` on `e`'s rounding near a
/// parabola (`k² = 1 − e²`): minted at `Interval` its enclosure was
/// 5.3e-14 at `k = 0.23`, 6.1e-13 at 2.4e-2 and 5.7e-12 at 2.4e-3 on
/// this pose. Read as `|rb|/|ra|` it is rounding-wide at every `k`.
#[test]
fn beta_encloses_to_rounding_however_near_the_parabola() {
    let poses = [
        (0.1, 0.99),
        (0.1, 0.9999),
        (1.0, 0.9999),
        (1.0, 0.99999),
        (0.1, 0.99999),
        (0.1, 0.999_999),
    ];
    for (scale, f, k, _, _, image) in interval_mints(&poses) {
        let width = image.beta.hi() - image.beta.lo();
        assert!(
            width < 128.0 * f64::EPSILON,
            "scale {scale}, f {f}, k {k:e}: beta width {width:e}"
        );
    }
}

/// **Minted at `Interval`, a near-parabola section certifies.** With
/// `β`'s enclosure widening like `u/k`, the envelope (which levers `β`
/// by about `1/k` near `|β| → 1`) escalated at each of these poses:
/// enclosures `[0, 2.7e-9]`, `[0, 6.8e-8]`, `[0, 6.5e-9]` and
/// `[0, 2.0e-7]` in turn.
#[test]
fn a_near_parabola_section_minted_at_interval_certifies() {
    let lift = <Interval as Real>::from_f64;
    let poses = [
        (1.0, 0.9999),
        (1.0, 0.99999),
        (0.1, 0.99999),
        (0.1, 0.999_999),
    ];
    for (scale, f, k, lc, ls, image) in interval_mints(&poses) {
        if let Err(e) = PcurveCache::certify(
            Pcurve::FocalSection(image),
            lift(-1.0),
            lift(1.0),
            &lc,
            &ls,
            band(),
        ) {
            panic!("scale {scale}, f {f}, k {k:e}: {e}");
        }
    }
}

/// **Off a section, the image misses by the carrier's own distance.** A
/// carrier `cone_incidence_fuzz` admits (its centre, axes and frame
/// perturbed together, 8.7e-10 off the cone): `|rb|/|ra|` and
/// `√(1 − e²)` read off it differ by 4e-10. The image with `β` read off
/// the axis ratio misses by 1.0e-9; with `β` read off `√(1 − e²)` it
/// misses by 1.7e-9, twice the carrier's distance.
#[test]
fn an_off_section_carrier_is_imaged_to_its_own_distance_from_the_cone() {
    let half_angle = 0.5_f64;
    let surface = Surface::Cone {
        apex: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        half_angle,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let carrier = Curve3::Ellipse {
        center: Point3::new(
            -1.126_628_255_287_795_3,
            2.820_706_427_674_058e-10,
            2.622_026_259_816_683_7,
        ),
        axis: Vec3::new(
            0.821_317_503_841_775_5,
            2.191_440_285_974_831e-10,
            0.570_471_347_118_429_1,
        ),
        major: 2.510_940_261_446_088,
        minor: 0.884_609_702_539_610_8,
        u_ref: Vec3::new(
            0.570_471_347_118_429_1,
            -1.028_029_997_851_775_2e-10,
            -0.821_317_503_841_775_5,
        ),
    };
    let off = (0..1024)
        .map(|i| {
            let p = carrier.eval(core::f64::consts::TAU * f64::from(i) / 1024.0);
            (p.x.hypot(p.y) * half_angle.cos() - p.z * half_angle.sin()).abs()
        })
        .fold(0.0, f64::max);
    let image = match chart_pcurve(&carrier, &surface, band()) {
        Ok(image) => image,
        Err(e) => {
            assert!(
                eps() != geom_core::tolerance::DEFAULT_EPS,
                "refused at the default ε: {e}"
            );
            println!(
                "SKIPPED at ε {:e}: a carrier {off:e} off the cone is refused ({e})",
                eps()
            );
            return;
        }
    };
    let miss = whole_turn_miss(&image, &carrier, &surface);
    assert!(
        miss < 1.25 * off,
        "carrier {off:e} off the cone, image misses by {miss:e}"
    );
}
