//! The tilted plane × cone ellipse's exact chart image on its cone
//! (`Pcurve::FocalSection`'s cone instance): derived by `chart_pcurve`, exact against
//! the carrier through the chart map, certified with a rounding-scale
//! envelope, and refused when any of its numbers is wrong.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::surf::{at_apex, off_cone};
use crate::shared::tol::{band, eps};
use geom::{Curve3, Surface};
use geom_brep::intersect::{PlaneConeSection, plane_cone_section};
use geom_brep::{
    EnvelopeStatement, FocalImage, Pcurve, PcurveCache, chart_pcurve, chart_pcurve_over,
};
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
        semi_u,
        semi_v,
        u_ref,
    } = e
    else {
        unreachable!()
    };
    Curve3::Ellipse {
        center,
        axis: -axis,
        semi_u,
        semi_v,
        u_ref,
    }
}

/// The same ellipse seamed `k` quarter turns on, at each of its four
/// principal vertices in turn: `u_ref` turned about `axis` and, on an
/// odd turn, the semi-axes swapped.
fn seamed(e: &Curve3<f64>, k: u32) -> Curve3<f64> {
    let &Curve3::Ellipse {
        center,
        axis,
        semi_u,
        semi_v,
        u_ref,
    } = e
    else {
        unreachable!()
    };
    let (u_ref, semi_u, semi_v) = match k % 4 {
        0 => (u_ref, semi_u, semi_v),
        1 => (axis.cross(u_ref), semi_v, semi_u),
        2 => (-u_ref, semi_u, semi_v),
        _ => (-axis.cross(u_ref), semi_v, semi_u),
    };
    Curve3::Ellipse {
        center,
        axis,
        semi_u,
        semi_v,
        u_ref,
    }
}

/// Every pairing of nappe and traversal, seamed at each principal
/// vertex: the image is the cone instance of the `FocalSection` form,
/// maps onto the carrier at rounding scale over two periods (the
/// azimuth's branch never jumps), and certifies over a span longer than
/// a half-turn with a rounding-scale envelope. A carrier seamed at a
/// minor vertex is read in its major frame through the image's `t0`.
#[test]
fn the_image_is_exact_on_both_nappes_and_both_traversals() {
    for axis_sign in [1.0, -1.0] {
        let surface = cone(axis_sign);
        let carriers = [section(), reversed(&section())]
            .iter()
            .flat_map(|e| (0..4).map(|k| seamed(e, k)))
            .collect::<Vec<_>>();
        for carrier in carriers {
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
            // The row is the image placed over the span it certifies on
            // (C4).
            let row = chart_pcurve_over(&carrier, 0.3, 4.0, &surface, band()).unwrap();
            let cache = PcurveCache::certify(row, 0.3, 4.0, &carrier, &surface, band())
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
            semi_u,
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
    let start = (radial(w) + radial(e_u * semi_u)) * w.dot(axis).signum();
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
        semi_u: 0.5 * (p1 - p2).norm(),
        semi_v: (ta * ta * cz * cz - cx * cx).sqrt(),
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

/// A skew pose minted at `Interval` on its lifted carrier and chart.
struct IntervalMint {
    scale: f64,
    f: f64,
    k: f64,
    carrier: Curve3<f64>,
    surface: Surface<f64>,
    image: FocalImage<Interval>,
}

/// The skew poses of [`skew_section`] the arm reaches at this ε, minted
/// at `Interval` on the lifted carrier and chart, with what did not
/// reach printed. The default ε must reach all of them.
fn interval_mints(poses: &[(f64, f64)]) -> Vec<IntervalMint> {
    let lift = <Interval as Real>::from_f64;
    let at_default = eps() == geom_core::tolerance::DEFAULT_EPS;
    let mut reached = Vec::new();
    for &(scale, f) in poses {
        let minted = skew_section(0.5_f64.atan(), scale, f).and_then(|(carrier, surface, k)| {
            let lifted = (carrier.map_scalar(lift), surface.map_scalar(lift));
            match chart_pcurve(&lifted.0, &lifted.1, band()) {
                Ok(Pcurve::FocalSection(image)) => Ok(IntervalMint {
                    scale,
                    f,
                    k,
                    carrier,
                    surface,
                    image,
                }),
                Ok(other) => Err(format!("the chart answers {other:?}")),
                Err(e) => Err(format!("the chart answers {e}")),
            }
        });
        match minted {
            Ok(mint) => reached.push(mint),
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
    for IntervalMint {
        scale, f, k, image, ..
    } in interval_mints(&poses)
    {
        let width = image.beta.hi() - image.beta.lo();
        assert!(
            width < 128.0 * f64::EPSILON,
            "scale {scale}, f {f}, k {k:e}: beta width {width:e}"
        );
    }
}

/// **Minted at `Interval`, a near-parabola section certifies, at a cost
/// of its own rounding.** With `β`'s enclosure widening like `u/k`, the
/// envelope (which levers `β` by about `1/k` near `|β| → 1`) escalated
/// at the last four poses (`[0, 2.7e-9]`, `[0, 6.8e-8]`, `[0, 6.5e-9]`,
/// `[0, 2.0e-7]`) and at the first certified at 58× the f64 mint's
/// lifted envelope (2.1e-10 against 3.6e-12). With `β` rounding-wide the
/// Interval envelope is 3.3–3.9× the lifted one at every pose, so the
/// ceiling is 8×.
#[test]
fn a_near_parabola_section_minted_at_interval_certifies() {
    let lift = <Interval as Real>::from_f64;
    let poses = [
        (0.1, 0.9999),
        (1.0, 0.9999),
        (1.0, 0.99999),
        (0.1, 0.99999),
        (0.1, 0.999_999),
    ];
    let certify = |image: Pcurve<Interval>, c: &Curve3<f64>, s: &Surface<f64>| {
        let (lc, ls) = (c.map_scalar(lift), s.map_scalar(lift));
        PcurveCache::certify(image, lift(-1.0), lift(1.0), &lc, &ls, band())
            .map(|cache| cache.certificate().envelope.hi())
    };
    for IntervalMint {
        scale,
        f,
        k,
        carrier,
        surface,
        image,
    } in interval_mints(&poses)
    {
        let at = format!("scale {scale}, f {f}, k {k:e}");
        let minted = certify(Pcurve::FocalSection(image), &carrier, &surface)
            .unwrap_or_else(|e| panic!("{at}: minted at Interval: {e}"));
        let Ok(Pcurve::FocalSection(at_f64)) = chart_pcurve(&carrier, &surface, band()) else {
            panic!("{at}: the f64 mint");
        };
        let lifted = Pcurve::FocalSection(FocalImage {
            u0: lift(at_f64.u0),
            t0: lift(at_f64.t0),
            v0: lift(at_f64.v0),
            va: lift(at_f64.va),
            vb: lift(at_f64.vb),
            vl: lift(at_f64.vl),
            beta: lift(at_f64.beta),
            sense: lift(at_f64.sense),
        });
        let lifted = certify(lifted, &carrier, &surface)
            .unwrap_or_else(|e| panic!("{at}: the f64 mint, lifted: {e}"));
        assert!(
            minted < 8.0 * lifted,
            "{at}: envelope {minted:e} minted at Interval, {lifted:e} lifted"
        );
    }
}

/// **Off a section, the image misses by the carrier's own radial gap.**
/// The image puts each carrier point on the cone at the point's own
/// height and azimuth, so it misses by the radial gap `off/cos α`
/// (`off` the distance along the cone's normal) plus its rounding, about
/// `u·A/k`. The carrier is a near-parabola section (`α = atan ½`,
/// `k = 2.4e-2`, `A = 3.0e3`) with its centre moved `0.3·ε` along its
/// major axis: that parts `√(1 − e²)` from `|rb|/|ra|` at first order,
/// by `δ/(A·k)`, and `β` read off the root misses by 5.7e-9 there,
/// 24× the carrier's distance, against 2.68e-10 = `off/cos α`.
#[test]
fn an_off_section_carrier_is_imaged_to_its_own_distance_from_the_cone() {
    let alpha = 0.5_f64.atan();
    let refused = |why: String| {
        assert!(
            eps() != geom_core::tolerance::DEFAULT_EPS,
            "refused at the default ε: {why}"
        );
        println!("SKIPPED at ε {:e}: {why}", eps());
    };
    let (exact, surface, k) = match skew_section(alpha, 1.0, 0.9999) {
        Ok(pose) => pose,
        Err(why) => return refused(why),
    };
    let Curve3::Ellipse {
        center,
        axis,
        semi_u,
        semi_v,
        u_ref,
    } = exact
    else {
        unreachable!()
    };
    let carrier = Curve3::Ellipse {
        center: center + u_ref * (0.3 * eps()),
        axis,
        semi_u,
        semi_v,
        u_ref,
    };
    let off = off_cone(&carrier, &surface);
    let image = match chart_pcurve(&carrier, &surface, band()) {
        Ok(image) => image,
        Err(e) => return refused(format!("a carrier {off:e} off the cone: {e}")),
    };
    let miss = whole_turn_miss(&image, &carrier, &surface);
    let bound = off / alpha.cos() + 4.0 * f64::EPSILON * semi_u / k;
    assert!(
        miss < bound,
        "carrier {off:e} off the cone (k {k:e}, A {semi_u:e}): image misses by {miss:e}, \
         past {bound:e}"
    );
}

/// How many `u` of a flat cone's section `size` its image's envelope may
/// read: three times the 16 `u` (`RESOLUTION_ULPS`) that
/// `flat_cone_tilted_ellipse.rs` allows the carrier itself off the cut,
/// on the same `size`, `max(larger semi-axis, |centre|, |apex|)`.
const FLAT_IMAGE_ULPS: f64 = 48.0;

/// **A flat cone's section images and certifies at its own resolution**
/// (`work/pcert/focal-section-image-on-a-flat-cone-misses-by-a-cos-squared-factor`,
/// fixed by #5043's `K = −D·M` and its apex-foot centre; this row guards
/// both halves). The item's own frustum pose, [`skew_section`], at
/// half-angle 1.55 (`1/cos²α ≈ 2300`), height 0.1, the plane leaning
/// `f·(π/2 − α)` off square to the axis for `f` of 0.5, 0.8 and 0.9.
/// Each section's `FocalSection` image certifies over a whole turn on a
/// band of its own, `(1e-12, 1e-11)`, so the row reads the same at every
/// ε row, with an envelope within [`FLAT_IMAGE_ULPS`] `u` of its `size`.
///
/// The ceiling binds at every pose: the band's zero is 700 `u` of the
/// largest `size` here and more of the others, so the ceiling, not the
/// band, decides. Today's envelopes read 37, 27 and 31 `u`. What reds the
/// row:
/// - the arm's old `K = c² − sin²α`, at every pose;
/// - the arm's old centre `apex − (c·a − sin²α·n)·δ/K`, at every pose
///   (465 `u` at `f = 0.8`, a refusal at the others);
/// - the centre moved 30 `u` of the semi-axis back along the major axis,
///   which reads 54, 63 and 70 `u`.
#[test]
fn a_flat_cones_section_images_and_certifies_at_its_own_resolution() {
    let band = geom_core::Band::new(1e-12, 1e-11).unwrap();
    let mut wrong = Vec::new();
    for f in [0.5, 0.8, 0.9] {
        let (carrier, surface, _) = skew_section(1.55, 0.1, f).unwrap();
        let Curve3::Ellipse {
            center,
            semi_u,
            semi_v,
            ..
        } = carrier
        else {
            unreachable!("skew_section answers an ellipse")
        };
        let Surface::Cone { apex, .. } = surface else {
            unreachable!("skew_section answers a cone")
        };
        let size = Curve3::semi_axes_by_size(semi_u, semi_v)
            .0
            .max(center.distance(Point3::origin()))
            .max(apex.distance(Point3::origin()));
        let ceiling = FLAT_IMAGE_ULPS * 0.5 * f64::EPSILON * size;
        assert!(
            ceiling < band.zero(),
            "f {f}: the ceiling {ceiling:e} must bind below the band's zero"
        );
        let got = chart_pcurve(&carrier, &surface, band).and_then(|image| {
            PcurveCache::certify(image, 0.0, core::f64::consts::TAU, &carrier, &surface, band)
        });
        match got {
            Ok(cache) if cache.certificate().envelope <= ceiling => {}
            Ok(cache) => wrong.push(format!(
                "f {f}: envelope {:e} past {FLAT_IMAGE_ULPS} u of size {size:e}",
                cache.certificate().envelope
            )),
            Err(e) => wrong.push(format!("f {f}, size {size:e}: {e:?}")),
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}
