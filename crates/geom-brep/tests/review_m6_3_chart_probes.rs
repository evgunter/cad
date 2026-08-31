//! M6-3 adversarial review probes (Leg E, walk row 4): the new
//! cone/sphere/torus chart closed forms attacked at their stated
//! boundaries — the pole-crossing meridian branch, the mirror-nappe
//! cone rim, the apex limit, the torus inner equator, and the sphere
//! envelope's domination of a corrupted pcurve's true sup.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unreachable
)]

use core::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

use geom::Curve3;
use geom::Surface;
use geom_brep::{ChartWindow, Pcurve, PcurveCache, PcurveCertifyError, chart_pcurve};
use geom_core::Tol;
use geom_core::{Band, Point2, Point3, Vec2, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn wide() -> ChartWindow<f64> {
    ChartWindow {
        u_min: -10.0,
        u_max: 10.0,
        v_min: -10.0,
        v_max: 10.0,
    }
}

fn sphere(r: f64) -> Surface<f64> {
    Surface::Sphere {
        center: Point3::new(0.3, -0.2, 1.1),
        radius: r,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

/// Evaluate the sphere chart by hand (the probe's own oracle).
fn sphere_eval(s: &Surface<f64>, u: f64, v: f64) -> Point3<f64> {
    let Surface::Sphere {
        center,
        radius,
        axis,
        u_ref,
    } = *s
    else {
        panic!("sphere only")
    };
    let cv = axis.cross(u_ref);
    let rad = u_ref * u.cos() + cv * u.sin();
    center + rad * (radius * v.cos()) + axis * (radius * v.sin())
}

/// The completion report says pole-crossing sphere meridian arcs are
/// EXACT harmonics ("the chart formula extends smoothly past the
/// pole"); the in-code comment at the sphere arm says the residual
/// schedule REFUSES an arc whose interior crosses a pole. Both cannot
/// be true. Executed verdict: the arc CERTIFIES with an ~ulp envelope
/// — the completion report is right and the comment is stale.
#[test]
fn probe_pole_crossing_meridian_arc_certifies() {
    let r = 0.7;
    let s = sphere(r);
    let Surface::Sphere { center, .. } = s else {
        unreachable!()
    };
    // Great circle in the xz-plane of the chart frame: v(t) = t,
    // azimuth 0. The window [pi/4, 3pi/4] crosses the north pole
    // (v = pi/2) in its INTERIOR.
    let carrier = Curve3::Circle {
        center,
        axis: Vec3::unit_y() * -1.0,
        radius: r,
        u_ref: Vec3::unit_x(),
    };
    let p = chart_pcurve(&carrier, &s, band()).unwrap();
    let Pcurve::Harmonic { p0, pl, .. } = p else {
        panic!("harmonic expected")
    };
    assert!(p0.y.abs() < 1e-12, "delta should be 0, got {}", p0.y);
    assert_eq!(pl.y, 1.0, "sigma should be +1");
    let cache = PcurveCache::certify(p, FRAC_PI_4, 3.0 * FRAC_PI_4, &carrier, &s, wide(), band())
        .expect("a pole-crossing meridian arc is an exact harmonic and must certify");
    assert!(
        cache.certificate().envelope < 1e-12,
        "envelope {} not ~0",
        cache.certificate().envelope
    );
}

/// Mirror-nappe cone rim (h < 0): the chart azimuth must be the
/// spatial azimuth + pi, and the closed form must certify exactly.
#[test]
fn probe_mirror_nappe_cone_rim_certifies_at_azimuth_plus_pi() {
    let half_angle: f64 = 0.5;
    let apex = Point3::new(1.0, 2.0, 3.0);
    let cone = Surface::Cone {
        apex,
        axis: Vec3::unit_z(),
        half_angle,
        u_ref: Vec3::unit_x(),
    };
    let h: f64 = -0.4;
    let rho = h.abs() * half_angle.tan();
    let carrier = Curve3::Circle {
        center: apex + Vec3::unit_z() * h,
        axis: Vec3::unit_z(),
        radius: rho,
        u_ref: Vec3::unit_x(),
    };
    let p = chart_pcurve(&carrier, &cone, band()).unwrap();
    let Pcurve::Harmonic { p0, pl, .. } = p else {
        panic!("harmonic expected")
    };
    // Spatial azimuth of the t=0 point is 0; the mirror nappe's chart
    // azimuth is pi.
    assert!(
        (p0.x - PI).abs() < 1e-12 || (p0.x + PI).abs() < 1e-12,
        "nappe azimuth should be +-pi, got {}",
        p0.x
    );
    assert!(p0.y < 0.0, "mirror-nappe slant v0 must be negative");
    assert_eq!(pl.x, 1.0, "spatial CCW keeps beta = +1 on either nappe");
    let cache = PcurveCache::certify(p, 0.0, 2.0, &carrier, &cone, wide(), band())
        .expect("mirror-nappe rim is in the closed-form class");
    assert!(cache.certificate().envelope < 1e-12);
}

/// The apex limit: an apex-level "rim" (h = 0) refuses as an
/// unsupported carrier rather than deriving a degenerate image.
#[test]
fn probe_apex_level_rim_refuses() {
    let apex = Point3::new(0.0, 0.0, 0.0);
    let cone = Surface::Cone {
        apex,
        axis: Vec3::unit_z(),
        half_angle: 0.5,
        u_ref: Vec3::unit_x(),
    };
    let carrier = Curve3::Circle {
        center: apex,
        axis: Vec3::unit_z(),
        radius: 1e-3,
        u_ref: Vec3::unit_x(),
    };
    match chart_pcurve(&carrier, &cone, band()) {
        Err(PcurveCertifyError::UnsupportedCarrier) => {}
        other => panic!("expected UnsupportedCarrier, got {other:?}"),
    }
}

/// Torus inner equator: the parallel at v = pi (radius R - r) derives
/// v0 = pi exactly (atan2 of the negative residual radius) and
/// certifies.
#[test]
fn probe_torus_inner_equator_certifies_at_v_pi() {
    let (major, minor) = (1.0, 0.3);
    let center = Point3::new(-1.0, 0.5, 2.0);
    let torus = Surface::Torus {
        center,
        axis: Vec3::unit_z(),
        major_radius: major,
        minor_radius: minor,
        u_ref: Vec3::unit_x(),
    };
    let carrier = Curve3::Circle {
        center,
        axis: Vec3::unit_z(),
        radius: major - minor,
        u_ref: Vec3::unit_x(),
    };
    let p = chart_pcurve(&carrier, &torus, band()).unwrap();
    let Pcurve::Harmonic { p0, .. } = p else {
        panic!("harmonic expected")
    };
    assert!(
        (p0.y.abs() - PI).abs() < 1e-12,
        "inner equator v0 should be +-pi, got {}",
        p0.y
    );
    let cache = PcurveCache::certify(p, 0.0, 3.0, &carrier, &torus, wide(), band())
        .expect("inner equator parallel certifies");
    assert!(cache.certificate().envelope < 1e-12);
}

/// Envelope domination on the NEW sphere arm: a corrupted meridian
/// pcurve (polar-channel harmonic + linear drift) must either refuse
/// certification, and its stated envelope formula must dominate a
/// densely sampled true sup — the cylinder-lane law transposed.
#[test]
fn probe_sphere_envelope_dominates_a_corrupted_meridian() {
    let r = 0.7;
    let s = sphere(r);
    let Surface::Sphere { center, .. } = s else {
        unreachable!()
    };
    let carrier = Curve3::Circle {
        center,
        axis: Vec3::unit_y() * -1.0,
        radius: r,
        u_ref: Vec3::unit_x(),
    };
    let Pcurve::Harmonic { p0, pa, pb, pl } = chart_pcurve(&carrier, &s, band()).unwrap() else {
        panic!("harmonic expected")
    };
    let d = 1e-4;
    let (t0, t1) = (0.0, FRAC_PI_2);
    for (ca, cb, cl, c0) in [
        (d, -d, 0.0, 0.0),
        (d, -d, 2.0 * d / PI, -d),
        (-d, d, -2.0 * d / PI, d),
        (0.0, 0.0, d, d),
    ] {
        let bad = Pcurve::Harmonic {
            p0: Point2::new(p0.x, p0.y + c0),
            pa: Vec2::new(pa.x, pa.y + ca),
            pb: Vec2::new(pb.x, pb.y + cb),
            pl: Vec2::new(pl.x, pl.y + cl),
        };
        // True sup of |S(P(t)) - C(t)| by dense sampling.
        let mut sup: f64 = 0.0;
        for i in 0..=2000 {
            let t = t0 + (t1 - t0) * (i as f64) / 2000.0;
            let Pcurve::Harmonic { p0, pa, pb, pl } = bad else {
                unreachable!()
            };
            let u = p0.x + pa.x * t.cos() + pb.x * t.sin() + pl.x * t;
            let v = p0.y + pa.y * t.cos() + pb.y * t.sin() + pl.y * t;
            let e = sphere_eval(&s, u, v) - carrier.eval(t);
            sup = sup.max(e.norm());
        }
        // The certificate's envelope statement for the snapped image:
        // channel drifts at the polar arm r (all corruption is in the
        // v channel here).
        let env = (c0.abs() + ca.abs() + cb.abs() + cl.abs() * t1.abs().max(t0.abs())) * r;
        assert!(
            sup <= env * (1.0 + 1e-9),
            "true sup {sup:e} beats the envelope bound {env:e} for ({ca},{cb},{cl},{c0})"
        );
        assert!(
            PcurveCache::certify(bad, t0, t1, &carrier, &s, wide(), band()).is_err(),
            "an off-by-1e-4 sphere pcurve certified"
        );
    }
}

/// Deviation 1's area door attacked where O(h) width matters: a
/// violently warped degree-3 patch. The fixed-resolution (16x16)
/// first-order hull-rule enclosure must CONTAIN a fine independent
/// oracle of the true area — the honesty is containment, not
/// tightness — and the width must be visibly nonzero (the O(h) cost
/// is real, reported, and carried into `area_pad`, not hidden).
#[test]
fn probe_dev1_area_enclosure_contains_a_violent_patch_oracle() {
    use geom_brep::props::quad::nurbs_patch_face;
    use geom_core::RingInterval;
    use geom_core::spline::KnotVector;
    let band = band();
    let eps = 1e-9;
    let kv = KnotVector::unit_segment(3);
    let p = |x: f64, y: f64, z: f64| {
        [
            RingInterval::point(x),
            RingInterval::point(y),
            RingInterval::point(z),
        ]
    };
    // 4x4 cubic Bezier net, z warped hard (amplitude 3 on a unit
    // footprint - slopes of order 10).
    let z = [
        [0.0, 3.0, -3.0, 0.0],
        [-3.0, 0.0, 3.0, -3.0],
        [3.0, -3.0, 0.0, 3.0],
        [0.0, 3.0, -3.0, 0.0],
    ];
    let mut control = Vec::new();
    for (iu, zr) in z.iter().enumerate() {
        for (iv, zz) in zr.iter().enumerate() {
            control.push(p(iu as f64 / 3.0, iv as f64 / 3.0, *zz));
        }
    }
    let out = nurbs_patch_face::<f64>(
        &kv,
        &kv,
        &control,
        &[1.0; 16],
        (0.0, 1.0, 0.0, 1.0),
        4.0,
        0.0,
        eps,
        band,
    );
    // The flux side may refuse at the budget on such a patch; the
    // AREA question is only posed when the call answers.
    let Ok(out) = out else {
        println!("violent patch refused typed at the flux budget: {out:?} (area not reachable)");
        return;
    };
    // Independent oracle: tensor Bernstein evaluation + central
    // differences on a fine midpoint grid.
    let bern = |i: usize, t: f64| -> f64 {
        let c = [1.0, 3.0, 3.0, 1.0][i];
        c * t.powi(i as i32) * (1.0 - t).powi(3 - i as i32)
    };
    let eval = |u: f64, v: f64| -> [f64; 3] {
        let mut s = [0.0; 3];
        for iu in 0..4 {
            for iv in 0..4 {
                let w = bern(iu, u) * bern(iv, v);
                let c = &control[iu * 4 + iv];
                s[0] += w * c[0].lo();
                s[1] += w * c[1].lo();
                s[2] += w * c[2].lo();
            }
        }
        s
    };
    let n = 600usize;
    let h = 1.0 / n as f64;
    let mut oracle = 0.0f64;
    for i in 0..n {
        for j in 0..n {
            let (u, v) = ((i as f64 + 0.5) * h, (j as f64 + 0.5) * h);
            let du: [f64; 3] =
                core::array::from_fn(|k| (eval(u + 1e-6, v)[k] - eval(u - 1e-6, v)[k]) / 2e-6);
            let dv: [f64; 3] =
                core::array::from_fn(|k| (eval(u, v + 1e-6)[k] - eval(u, v - 1e-6)[k]) / 2e-6);
            let cr = [
                du[1] * dv[2] - du[2] * dv[1],
                du[2] * dv[0] - du[0] * dv[2],
                du[0] * dv[1] - du[1] * dv[0],
            ];
            oracle += (cr[0].powi(2) + cr[1].powi(2) + cr[2].powi(2)).sqrt() * h * h;
        }
    }
    println!(
        "violent patch: oracle area {oracle}, enclosure [{}, {}] width {}",
        out.area.lo(),
        out.area.hi(),
        out.area.width()
    );
    assert!(
        out.area.lo() - 1e-3 <= oracle && oracle <= out.area.hi() + 1e-3,
        "oracle {oracle} escapes the area enclosure [{}, {}]",
        out.area.lo(),
        out.area.hi()
    );
    // A deliberate LOWER bound, and the only row in the tree that
    // reads the area enclosure's width directly. It says the O(h) cost
    // is REPORTED rather than hidden, so it goes red if the width ever
    // silently collapses.
    //
    // Re-derived on the post-CERT-5 tree: the width is 2.6340 on an
    // enclosure of [1.8587, 4.4927] against a 2.9726 oracle, clearing
    // this floor by 2634x. It was scheduled to be invalidated by an
    // area-refining funnel — S-CERT Q1 ruled that funnel out (a
    // wide-but-sound bracket is sound; an ε-scale area target is a
    // 10³–10⁴x piece-count multiplier), so nothing is now queued to
    // drive this width down and the row is a standing invariant rather
    // than a dependency waiting on a fix.
    //
    // This face is the widest bracket in the corpus the A2 gauge was
    // calibrated on by BOTH readings — relative width 1.42, and the
    // integral lane's largest gauge value — and the gauge is silent on
    // it: 1.23e-2 of a perimeter against a ceiling of 1, an 81x margin.
    // A row that must stay wide and a tripwire for widths that mean a
    // bug are not in tension; that separation is what the ceiling's
    // generosity buys.
    assert!(
        out.area.width() > 1e-3,
        "a violent patch at fixed resolution must pay a visible O(h) width, got {}",
        out.area.width()
    );
}
