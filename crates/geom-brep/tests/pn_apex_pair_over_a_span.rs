//! **Plane × cone's line pair is bounded over the span its consumer
//! reads.** A plane a hair off the apex (its gap Zero) that dips into the
//! cone cuts a hyperbola, which the arm serves as the line pair through
//! the apex only where the pair stands within the zero band of it. That
//! is read over the reach the consumer hands the arm: its window along
//! the axis says how near the apex the consumed points come, and on
//! which nappes. A span clear of the apex sees the pair within about the
//! asymptote's offset of the section, however far the branches start; a
//! span reaching the apex sees each branch's start.
//!
//! The oracle never reads the arm's bound. It cuts the double cone with
//! the plane at each height of the window, in double-double arithmetic
//! (the cone, the plane and the served lines read from their stored
//! `f64`s, so the oracle's own rounding is some `1e-32` of the span),
//! and measures both ways round: each section point of the window from
//! the nearer served line, and each point of a served line in the window
//! from the section.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/intersect.rs",
    "crates/geom-brep/src/extent.rs",
    "crates/geom-brep/tests/shared/"
];

use geom::{Curve3, Surface};
use geom_brep::Reach;
use geom_brep::intersect::{PlaneConeSection, SectionError, plane_cone_section};
use geom_core::{Band, Point3, Tol, Vec3};
use test_utils::cone_cut::{ConeCut, P3};
use test_utils::fuzz;

fn p3(p: Point3<f64>) -> P3 {
    [p.x, p.y, p.z]
}

fn w3(v: Vec3<f64>) -> P3 {
    [v.x, v.y, v.z]
}

/// The oracle's cut of `cone` by `plane`, read from their stored `f64`s.
fn cut_of(cone: &Surface<f64>, plane: &Surface<f64>) -> ConeCut {
    let Surface::Cone {
        apex,
        axis,
        half_angle,
        ..
    } = *cone
    else {
        unreachable!("a cone")
    };
    let (origin, normal) = plane_of(plane);
    ConeCut::new(p3(apex), w3(axis), half_angle, (p3(origin), w3(normal)))
}

/// A served pair's stored lines.
fn lines(l1: &Curve3<f64>, l2: &Curve3<f64>) -> [(P3, P3); 2] {
    [l1, l2].map(|l| {
        let Curve3::Line { origin, dir } = *l else {
            panic!("a line outcome carries a line, got {l:?}");
        };
        (p3(origin), w3(dir))
    })
}

/// The plane `gap` off the apex at the origin, leaning `theta` off the
/// axis toward `radial`.
fn plane(axis: Vec3<f64>, radial: Vec3<f64>, theta: f64, gap: f64) -> Surface<f64> {
    plane_at(Point3::origin(), (axis, radial, theta), gap, Vec3::zero())
}

/// [`plane`] off `apex`, its origin slid by `slide`'s part across its
/// normal: a plane built through the apex whose stored origin stands
/// elsewhere in it, so its gap is read through that offset's rounding.
fn plane_at(
    apex: Point3<f64>,
    (axis, radial, theta): (Vec3<f64>, Vec3<f64>, f64),
    gap: f64,
    slide: Vec3<f64>,
) -> Surface<f64> {
    let n = (axis * theta.cos() + radial * theta.sin()).normalize();
    let u = if n.x.abs() < 0.9 {
        Vec3::unit_x()
    } else {
        Vec3::unit_y()
    };
    Surface::Plane {
        origin: apex - n * gap + (slide - n * slide.dot(n)),
        normal: n,
        u_ref: (u - n * u.dot(n)).normalize(),
    }
}

fn cone(axis: Vec3<f64>, u_ref: Vec3<f64>, alpha: f64) -> Surface<f64> {
    cone_at(Point3::origin(), axis, u_ref, alpha)
}

fn cone_at(apex: Point3<f64>, axis: Vec3<f64>, u_ref: Vec3<f64>, alpha: f64) -> Surface<f64> {
    Surface::Cone {
        apex,
        axis,
        half_angle: alpha,
        u_ref,
    }
}

/// A consumer's reach over the heights `[lo, hi]` along the axis from
/// the apex, levered at `lever`.
fn window(lo: f64, hi: f64, lever: f64) -> Reach<f64> {
    window_at(Point3::origin(), lo, hi, lever)
}

fn window_at(apex: Point3<f64>, lo: f64, hi: f64, lever: f64) -> Reach<f64> {
    Reach::Face {
        at: apex,
        below: -lo,
        above: hi,
        across: lever,
    }
}

/// The lean that puts the discriminant `D = −cos(α + θ)` at `d`.
fn lean_for(alpha: f64, d: f64) -> f64 {
    (-d).acos() - alpha
}

fn plane_of(s: &Surface<f64>) -> (Point3<f64>, Vec3<f64>) {
    let Surface::Plane { origin, normal, .. } = *s else {
        unreachable!("a plane")
    };
    (origin, normal)
}

/// What a draw met: the pair, the open conic, or a refusal.
fn kind(got: &Result<PlaneConeSection<f64>, SectionError>) -> String {
    match got {
        Ok(PlaneConeSection::ApexLinePair { .. }) => "pair".to_owned(),
        Ok(PlaneConeSection::OpenConic(s)) => format!("open {:?}", s.conic()),
        Ok(other) => format!("{other:?}")
            .split(['(', ' ', '{'])
            .next()
            .unwrap()
            .to_owned(),
        Err(r) => format!("{r:?}")
            .split(['(', ' ', '{'])
            .next()
            .unwrap()
            .to_owned(),
    }
}

/// **Counterexample search.** Planes through and near the apex that dip
/// into cones from needles through ordinary to near-flat, at the three
/// ε rows, each read over a random span: clear of the apex on one nappe,
/// reaching it from one nappe, or through it. The apex stands off the
/// origin half the time, and the plane's stored origin slides within it
/// half the time, so its gap is read through that offset's rounding (a
/// plane built through the apex reads rounding alone). A contradiction is
/// a served pair that stands, by the oracle ([`ConeCut::miss`]), past the zero band
/// of the section over the span, beyond `64·f64::EPSILON` per unit of
/// the span's reach from the apex: the rounding in a served line's
/// direction that its slope to the plane does not amplify. The amplified
/// part (the azimuth read off `acos` near tangency) is the arm's to
/// bound, so it gets no slack here.
///
/// The outcome counts, by span, print on every run.
#[test]
fn a_pair_over_a_span_agrees_with_the_oracle() {
    let k = Tol::witness().k();
    let mut g = fuzz::start("pn_apex_pair_over_a_span");
    let mut counts = std::collections::BTreeMap::<(String, &str), usize>::new();
    let mut contradictions = Vec::new();
    let unit = |g: &mut fuzz::Rng| {
        Vec3::new(g.range(-1.0, 1.0), g.range(-1.0, 1.0), g.range(-1.0, 1.0)).normalize()
    };
    for _ in 0..fuzz::scaled(200) {
        let e = [1e-9, 1e-6, 1e-12][g.below(3)];
        let band = Band::new(e, k * e).unwrap();
        let extent = 10f64.powf(g.range(-0.5, 1.0));
        let axis = unit(&mut g);
        // Half the seams tilt off square to the axis, by up to `1e-10`
        // (no check holds a cone's `u_ref ⊥ axis`).
        let seam = axis.cross(unit(&mut g)).normalize();
        let tilt = if g.below(2) == 0 {
            0.0
        } else {
            g.range(-1.0, 1.0) * 10f64.powf(g.range(-16.0, -10.0))
        };
        let u_ref = seam + axis * tilt;
        let thin = 10f64.powf(g.range((11.0 * e / extent).log10(), -1.3));
        let alpha = match g.below(6) {
            0 | 1 => thin.asin(),
            2 => thin.acos(),
            _ => g.range(0.02, 1.5),
        };
        let theta = if g.below(3) == 0 {
            core::f64::consts::FRAC_PI_2
        } else {
            let d = 10f64.powf(g.range(1.0, 6.0)) * e / extent;
            lean_for(alpha, d.min(alpha.sin() * g.range(0.5, 1.0)))
        };
        if !(0.0..=core::f64::consts::FRAC_PI_2).contains(&theta) {
            continue;
        }
        let azimuth = g.range(0.0, core::f64::consts::TAU);
        let radial = seam * azimuth.cos() + axis.cross(seam) * azimuth.sin();
        let facing = if g.below(2) == 0 { 1.0 } else { -1.0 };
        let gap = match g.below(8) {
            0 => g.range(-1.0, 1.0) * 1e-16,
            1 => 0.0,
            _ => g.range(-1.2, 1.2) * e,
        };
        let apex = if g.below(2) == 0 {
            Point3::origin()
        } else {
            Point3::origin() + unit(&mut g) * g.range(0.0, 4.0)
        };
        let slide = if g.below(2) == 0 {
            Vec3::zero()
        } else {
            unit(&mut g) * 10f64.powf(g.range(-1.0, 1.8))
        };
        let p = plane_at(apex, (axis * facing, radial, theta), gap, slide);
        let c = cone_at(apex, axis, u_ref, alpha);
        // The span's nearest and farthest distances from the apex, read
        // as heights along the axis.
        let near = extent * 10f64.powf(g.range((e / extent).log10(), -0.3)) * alpha.cos();
        let far = near + extent * g.range(0.1, 1.0) * alpha.cos();
        let (span, (lo, hi)) = match g.below(3) {
            0 => ("clear", (near, far)),
            1 => ("to the apex", (0.0, far)),
            _ => ("through the apex", (-near, far)),
        };
        let (lo, hi) = if g.below(2) == 0 {
            (lo, hi)
        } else {
            (-hi, -lo)
        };
        let reach = lo.abs().max(hi.abs()) / alpha.cos();
        let got = plane_cone_section(&p, &c, &window_at(apex, lo, hi, reach), band);
        *counts.entry((kind(&got), span)).or_default() += 1;
        let Ok(PlaneConeSection::ApexLinePair { l1, l2 }) = &got else {
            continue;
        };
        let worst = cut_of(&c, &p).miss(&lines(l1, l2), (lo / alpha.cos(), hi / alpha.cos()));
        let slack = 64.0 * f64::EPSILON * reach;
        if worst > e * (1.0 + 1e-6) + slack {
            contradictions.push(format!(
                "ε {e:e}, α {alpha:?}, θ {theta:?}, axis {axis:?}, u_ref {u_ref:?}, radial \
                 {radial:?}, facing {facing}, gap {gap:?}, apex {apex:?}, slide {slide:?}, {span} [{lo:?}, {hi:?}]: the pair \
                 stands {:.4}·ε off the section",
                worst / e
            ));
        }
    }
    eprintln!("pair over a span: {counts:?} ({})", fuzz::replay());
    assert!(
        contradictions.is_empty(),
        "{} contradictions ({}), first: {}",
        contradictions.len(),
        fuzz::replay(),
        contradictions[0]
    );
}

/// `π/12`, leaning to `D = 1/512` at a unit extent: `M ≈ 0.52`, so the
/// near nappe's section starts 270 times nearer the apex than the far
/// one's.
fn leaning(gap: f64) -> (Surface<f64>, Surface<f64>, f64) {
    let alpha = core::f64::consts::PI / 12.0;
    let (axis, u_ref) = (Vec3::unit_z(), Vec3::unit_x());
    let p = plane(axis, u_ref, lean_for(alpha, 1.0 / 512.0), gap);
    (p, cone(axis, u_ref, alpha), alpha)
}

/// **A span reaching the apex on the nearer nappe is served where that
/// nappe's start and the asymptote's offset are within the band.** The
/// leaning plane `0.04·ε` off the apex starts its section `≈ 0.08·ε` off
/// on the nappe its normal leans from and `≈ 20·ε` off on the other. A
/// span from the apex along the near nappe is served the pair, and the
/// oracle holds it within ε of the section; along the far nappe, or
/// through the apex, it is served the hyperbola. The same span
/// moved clear of the apex is served on either nappe.
#[test]
fn a_span_reaching_the_apex_is_bounded_by_its_own_nappe() {
    let (eps, k) = (Tol::witness().eps(), Tol::witness().k());
    let band = Band::new(eps, k * eps).unwrap();
    let (p, c, alpha) = leaning(0.04 * eps);
    let cut = cut_of(&c, &p);
    // The normal leans toward `+z`, and the plane stands below the apex
    // (`δ > 0`): the `−z` nappe meets it steepest.
    for (label, (lo, hi), served) in [
        ("from the apex on the near nappe", (-1.0, 0.0), true),
        ("from the apex on the far nappe", (0.0, 1.0), false),
        ("through the apex", (-1.0, 1.0), false),
        ("clear of the apex on the near nappe", (-1.0, -0.25), true),
        ("clear of the apex on the far nappe", (0.25, 1.0), true),
    ] {
        let got = plane_cone_section(&p, &c, &window(lo, hi, 1.0), band);
        match &got {
            Ok(PlaneConeSection::ApexLinePair { l1, l2 }) if served => {
                let slant = (lo / alpha.cos(), hi / alpha.cos());
                let worst = cut.miss(&lines(l1, l2), slant);
                assert!(
                    worst <= eps,
                    "{label}: the pair stands {:.4}·ε off the section",
                    worst / eps
                );
            }
            Ok(PlaneConeSection::OpenConic(s)) if !served => assert!(
                s.conic() == geom_brep::intersect::OpenConic::Hyperbola,
                "{label}: wanted the hyperbola, got {:?}",
                s.conic()
            ),
            other => panic!("{label}: served {served}, got {other:?}"),
        }
    }
}

/// **A span clear of the apex by less than where its nappe's section
/// starts is bounded by the start it falls short of.** The plane along
/// the axis of the cone `sin α = 1/64`, `0.4·ε` off the apex, starts
/// both branches `25.6·ε` off, past the band, and stands within
/// `2·0.4·ε` of the pair wherever the section is. A span from half the
/// start along the axis holds the pair's points from there, `12.8·ε`
/// short of the section's nearest: served the hyperbola. From
/// twice the start it is served, and the oracle holds it within ε.
#[test]
fn a_span_short_of_the_start_reads_the_start() {
    let (eps, k) = (Tol::witness().eps(), Tol::witness().k());
    let band = Band::new(eps, k * eps).unwrap();
    let alpha = (1.0_f64 / 64.0).asin();
    let (axis, u_ref) = (Vec3::unit_z(), Vec3::unit_x());
    let p = plane(axis, u_ref, core::f64::consts::FRAC_PI_2, 0.4 * eps);
    let c = cone(axis, u_ref, alpha);
    let cut = cut_of(&c, &p);
    let start = 0.4 * eps * 64.0;
    for side in [1.0, -1.0] {
        let short = (side * 0.5 * start * alpha.cos(), side);
        let got = plane_cone_section(
            &p,
            &c,
            &window(short.0.min(short.1), short.0.max(short.1), 1.0),
            band,
        );
        assert!(
            matches!(&got, Ok(PlaneConeSection::OpenConic(s)) if s.conic() == geom_brep::intersect::OpenConic::Hyperbola),
            "from half the start, side {side}: got {got:?}"
        );
        let clear = (side * 2.0 * start * alpha.cos(), side);
        let (lo, hi) = (clear.0.min(clear.1), clear.0.max(clear.1));
        let got = plane_cone_section(&p, &c, &window(lo, hi, 1.0), band);
        let Ok(PlaneConeSection::ApexLinePair { l1, l2 }) = &got else {
            panic!("from twice the start, side {side}: got {got:?}");
        };
        let slant = (lo / alpha.cos(), hi / alpha.cos());
        let worst = cut.miss(&lines(l1, l2), slant);
        assert!(
            worst <= eps,
            "from twice the start, side {side}: the pair stands {:.4}·ε off",
            worst / eps
        );
    }
}

/// **A span starting at a branch's vertex reads the vertex's miss, not
/// the asymptote's.** The plane along the axis of the cone
/// `sin α = 1/64`, `0.9·ε` off the apex, starts both branches `57.6·ε`
/// off, at the vertices, and the oracle stands each vertex `≈ 1.27·ε`
/// from the pair: more than the asymptote's `0.9·ε` offset. A span
/// starting at the vertices escalates; one starting ten times as far
/// out is served, and the oracle holds it within ε.
#[test]
fn a_span_from_the_vertex_reads_the_vertex() {
    let (eps, k) = (Tol::witness().eps(), Tol::witness().k());
    let band = Band::new(eps, k * eps).unwrap();
    let alpha = (1.0_f64 / 64.0).asin();
    let (axis, u_ref) = (Vec3::unit_z(), Vec3::unit_x());
    let p = plane(axis, u_ref, core::f64::consts::FRAC_PI_2, 0.9 * eps);
    let c = cone(axis, u_ref, alpha);
    let cut = cut_of(&c, &p);
    let start = 0.9 * eps * 64.0;
    let vertex = cut.start(1.0);
    assert!(
        (vertex - start).abs() <= 1e-6 * start,
        "the oracle's vertex {vertex:e} is the start {start:e}"
    );
    let out = |times: f64| (times * start * alpha.cos(), 1.0);
    let (lo, hi) = out(1.0);
    let got = plane_cone_section(&p, &c, &window(lo, hi, 1.0), band);
    assert!(
        matches!(&got, Err(SectionError::Escalated(d)) if d.predicate == Some("pn_apex_pair_reach")),
        "from the vertex: got {got:?}"
    );
    let (lo, hi) = out(10.0);
    let got = plane_cone_section(&p, &c, &window(lo, hi, 1.0), band);
    let Ok(PlaneConeSection::ApexLinePair { l1, l2 }) = &got else {
        panic!("from ten times the vertex: got {got:?}");
    };
    let slant = (lo / alpha.cos(), hi / alpha.cos());
    let worst = cut.miss(&lines(l1, l2), slant);
    assert!(worst <= eps, "the pair stands {:.4}·ε off", worst / eps);
    let at_vertex = cut.miss(&lines(l1, l2), (vertex, vertex * (1.0 + 1e-9)));
    assert!(
        at_vertex > eps,
        "the vertex stands {:.4}·ε off the pair, past the band",
        at_vertex / eps
    );
}

/// **A plane near tangency is not served the pair its rounding turns
/// off the section.** The fuzz's counterexample (seed
/// `0xe823fdaa841f0564`, effort 20, read at its own ε of `1e-12`): a plane
/// a rounding gap off the apex of an ordinary cone, `D ≈ 5e-10` past
/// tangency, over a span clear of the apex. The pair's own bound is
/// well inside the band, but the lines' azimuth is read by `acos` an
/// `f64::EPSILON/sin γ` off, which over the span's reach moved them
/// `3·ε` off the section; the arm reads that rounding beside the bound
/// and does not serve them.
#[test]
fn a_pair_near_tangency_reads_its_azimuths_rounding() {
    let e = 1e-12;
    let band = Band::new(e, Tol::witness().k() * e).unwrap();
    let alpha = 0.465_751_145_876_335;
    let axis = Vec3::new(
        0.030263501113899235,
        0.15625974967437592,
        0.9872522530397336,
    );
    let u_ref = Vec3::new(
        0.35598917718464784,
        -0.9246238176084163,
        0.13543449205662333,
    );
    let radial = Vec3::new(
        -0.9995416420737109,
        0.005513127257457118,
        0.02976761979790632,
    );
    let p = plane(axis, radial, 1.1050451814070406, -2.2086621265754468e-17);
    let c = cone(axis, u_ref, alpha);
    let (lo, hi) = (7.137026553315642e-5, 1.305527430405652);
    let got = plane_cone_section(&p, &c, &window(lo, hi, hi / alpha.cos()), band);
    if let Ok(PlaneConeSection::ApexLinePair { l1, l2 }) = &got {
        let cut = cut_of(&c, &p);
        let worst = cut.miss(&lines(l1, l2), (lo / alpha.cos(), hi / alpha.cos()));
        panic!("served a pair {:.4}·ε off the section", worst / e);
    }
}

/// **A window at the apex alone reads the nearer start.** The plane
/// along the axis of a `π/12` cone `0.4·ε` off the apex starts both
/// branches `≈ 1.55·ε` off, in the band, and stands within `0.8·ε` of
/// the pair wherever the section is. A reach whose window is the apex
/// alone still consumes the apex, which stands the start off the
/// section: the arm escalates.
#[test]
fn a_window_at_the_apex_alone_reads_the_nearer_start() {
    let (eps, k) = (Tol::witness().eps(), Tol::witness().k());
    let band = Band::new(eps, k * eps).unwrap();
    let alpha = core::f64::consts::PI / 12.0;
    let (axis, u_ref) = (Vec3::unit_z(), Vec3::unit_x());
    let p = plane(axis, u_ref, core::f64::consts::FRAC_PI_2, 0.4 * eps);
    let got = plane_cone_section(&p, &cone(axis, u_ref, alpha), &window(0.0, 0.0, 1.0), band);
    assert!(
        matches!(&got, Err(SectionError::Escalated(d)) if d.predicate == Some("pn_apex_pair_reach")),
        "got {got:?}"
    );
}

/// **A plane nearly square to a near-flat cone's axis is not served the
/// pair its trace's rounding turns off the section.** The fuzz's
/// counterexample (seed `0x1111`, effort 20, read at its own ε of
/// `1e-9`): the cone `cos α ≈ 4e-8`, the plane's normal `1.5e-7` off the
/// axis, a rounding gap off the apex, over a span reaching `5.4` from
/// it. The pair's bound reads `≈ 0.4·ε`, but the lines' azimuth is
/// read off the normal's components across the axis, `1.5e-7` long,
/// whose rounding over that reach moved them `3.4·ε` off the section;
/// the arm reads that rounding beside the bound. Whatever it serves
/// stands within ε of the section.
#[test]
fn a_plane_square_to_a_flat_cones_axis_reads_its_traces_rounding() {
    let e = 1e-9;
    let band = Band::new(e, Tol::witness().k() * e).unwrap();
    let alpha = 1.5707962843525753;
    let axis = Vec3::new(-0.3737984588069682, -0.8132873153956929, 0.4459130574562737);
    let u_ref = Vec3::new(-0.4959592170981176, 0.5815220301577921, 0.6448694312933295);
    let radial = Vec3::new(
        -0.8850975702662192,
        0.16905946466908706,
        -0.43361410091766717,
    );
    let p = plane(axis, radial, 1.4744785081788336e-7, 3.793366991684415e-17);
    let c = cone(axis, u_ref, alpha);
    let (lo, hi) = (-2.2774502414708424e-7, -3.665207256388041e-10);
    let got = plane_cone_section(&p, &c, &window(lo, hi, lo.abs() / alpha.cos()), band);
    if let Ok(PlaneConeSection::ApexLinePair { l1, l2 }) = &got {
        let cut = cut_of(&c, &p);
        let worst = cut.miss(&lines(l1, l2), (lo / alpha.cos(), hi / alpha.cos()));
        assert!(
            worst <= e,
            "served a pair {:.4}·ε off the section",
            worst / e
        );
    }
}

/// **A seam off square to the axis does not turn the pair.** A flat
/// cone (`α = 1.56698`) whose `u_ref` leans `−7.6e-15` along its axis,
/// cut through its apex by a plane leaning `s = 0.0168` off the axis
/// toward `axis × u_ref`, where the lean turns the lines' azimuth most:
/// `τ·|a·n|/s`. Read off the seam as stored, that turn moved the served
/// pair `2.7·ε` off the section over a span out to `6` from the apex at
/// `ε = 1e-12`. The pair is served, within ε of the section. The spans
/// stop at `6`: at `10` the arm's own bound on the pair, step 5's
/// rounding charges for the squared frame among it, reaches past the
/// zero band, and the arm escalates rather than serves.
#[test]
fn a_seam_off_square_to_the_axis_does_not_turn_the_pair() {
    let e = 1e-12;
    let band = Band::new(e, Tol::witness().k() * e).unwrap();
    let alpha = 1.56698_f64;
    let axis = Vec3::unit_z();
    let c = cone(axis, Vec3::new(1.0, 0.0, -7.6e-15), alpha);
    let p = plane(axis, Vec3::unit_y(), 0.0168_f64.asin(), 0.0);
    for (lo, hi) in [(0.0, 6.0), (-6.0, 6.0), (-6.0, -1.0)] {
        let (lo, hi) = (lo * alpha.cos(), hi * alpha.cos());
        let got = plane_cone_section(&p, &c, &window(lo, hi, 6.0), band);
        let Ok(PlaneConeSection::ApexLinePair { l1, l2 }) = &got else {
            panic!("[{lo:e}, {hi:e}]: the pair, got {got:?}");
        };
        let worst = cut_of(&c, &p).miss(&lines(l1, l2), (lo / alpha.cos(), hi / alpha.cos()));
        assert!(
            worst <= e,
            "[{lo:e}, {hi:e}]: served a pair {:.4}·ε off the section",
            worst / e
        );
    }
}

/// **A span is read at its distance from the apex, no farther.** The
/// plane along the axis of a `π/3` cone `0.9·ε` off the apex starts both
/// branches `≈ 1.04·ε` off, and bounds the pair over a span `ρ` or more
/// from the apex by `1.8·ε/(1 + √(1 − 2·start/ρ))`. A span from
/// `ρ = start/0.3` stands at half that height up the axis, `cos α = 1/2`,
/// and is bounded by `≈ 1.10·ε`, in the band: the arm escalates, where
/// a reading that divided the height by `cos α` twice, and so stood the
/// span twice as far out, would serve it. From
/// `ρ = start/0.15` it is served, and the oracle holds it within ε.
#[test]
fn a_span_is_read_at_its_distance_from_the_apex_no_farther() {
    let (eps, k) = (Tol::witness().eps(), Tol::witness().k());
    let band = Band::new(eps, k * eps).unwrap();
    let alpha = core::f64::consts::FRAC_PI_3;
    let (axis, u_ref) = (Vec3::unit_z(), Vec3::unit_x());
    let p = plane(axis, u_ref, core::f64::consts::FRAC_PI_2, 0.9 * eps);
    let c = cone(axis, u_ref, alpha);
    let start = 0.9 * eps / alpha.sin();
    let from = |x: f64| (start / x * alpha.cos(), 1.0);
    let (lo, hi) = from(0.3);
    let got = plane_cone_section(&p, &c, &window(lo, hi, 2.0), band);
    assert!(
        matches!(&got, Err(SectionError::Escalated(d)) if d.predicate == Some("pn_apex_pair_reach")),
        "from start/0.3: got {got:?}"
    );
    let (lo, hi) = from(0.15);
    let got = plane_cone_section(&p, &c, &window(lo, hi, 2.0), band);
    let Ok(PlaneConeSection::ApexLinePair { l1, l2 }) = &got else {
        panic!("from start/0.15: got {got:?}");
    };
    let cut = cut_of(&c, &p);
    let worst = cut.miss(&lines(l1, l2), (lo / alpha.cos(), hi / alpha.cos()));
    assert!(worst <= eps, "the pair stands {:.4}·ε off", worst / eps);
}

/// **A needle's plane near tangency is not served the pair its normal's
/// rounding turns off the section.** The fuzz's counterexample (seed
/// `0x1d2c3b4a59687706`, effort 50, read at its own ε of `1e-12`): the
/// cone `α ≈ 6.7e-4`, the plane `D ≈ 1.4e-11` past tangency a rounding
/// gap off the apex, over a span from `2.2e-12` to `7.1` up the axis.
/// The pair's own bound is `0.9·ε`, but `cos γ` is read off `a·n`, a
/// `6.7e-4` dot product carrying its rounding whole, which `acos` near
/// tangency turned the lines by enough to stand them `1.13·ε` off; the
/// arm reads that rounding beside the bound. Whatever it serves stands
/// within ε of the section.
#[test]
fn a_needles_plane_near_tangency_reads_its_normals_rounding() {
    let e = 1e-12;
    let band = Band::new(e, Tol::witness().k() * e).unwrap();
    let alpha = 0.0006679744654476203;
    let axis = Vec3::new(0.5162029411022415, -0.5069460497530679, -0.6903189308118066);
    let u_ref = Vec3::new(
        -0.24299173505528693,
        0.6862016424361179,
        -0.6856254973473459,
    );
    let radial = Vec3::new(-0.5918936476407264, 0.3714021211021089, -0.715347729655592);
    let p = plane(-axis, radial, 1.5701283523433798, 9.192356398007253e-17);
    let c = cone(axis, u_ref, alpha);
    let (lo, hi) = (2.1810750976419317e-12, 7.106305111166084);
    let got = plane_cone_section(&p, &c, &window(lo, hi, hi / alpha.cos()), band);
    if let Ok(PlaneConeSection::ApexLinePair { l1, l2 }) = &got {
        let cut = cut_of(&c, &p);
        let worst = cut.miss(&lines(l1, l2), (lo / alpha.cos(), hi / alpha.cos()));
        assert!(
            worst <= e,
            "served a pair {:.4}·ε off the section",
            worst / e
        );
    }
}

/// A plane built through a cone's apex, read at its stored `f64`s:
/// `(apex, axis, u_ref, α, plane origin, plane normal)`.
type Pose = (P3, P3, P3, f64, P3, P3);

/// **What a pose built through the apex is served over `[0, 4]` up the
/// axis**: refused, or the pair within ε of the section by the oracle;
/// otherwise how far off the pair stands.
/// Also checks the fixture: the kernel's gap reads the opposite sign to
/// the oracle's, so a nappe read off it is the wrong one.
fn through_the_apex_reads_its_gaps_rounding(label: &str, pose: Pose) -> Option<String> {
    let e = 1e-9;
    let band = Band::new(e, Tol::witness().k() * e).unwrap();
    let (apex, axis, u_ref, alpha, origin, normal) = pose;
    let v3 = |[x, y, z]: P3| Vec3::new(x, y, z);
    let (apex, origin) = (Point3::origin() + v3(apex), Point3::origin() + v3(origin));
    let (axis, normal) = (v3(axis), v3(normal));
    let c = cone_at(apex, axis, v3(u_ref), alpha);
    let p = Surface::Plane {
        origin,
        normal,
        u_ref: (Vec3::unit_x() - normal * normal.x).normalize(),
    };
    let cut = cut_of(&c, &p);
    let kernel = (apex - origin).dot(normal);
    assert!(
        kernel * cut.gap() < 0.0,
        "{label}: the fixture reads its gap {kernel:e}, against the oracle's {:e}",
        cut.gap()
    );
    let reach = Reach::Measured {
        at: apex + axis * 2.0,
        lever: 2.0,
    };
    if let Ok(PlaneConeSection::ApexLinePair { l1, l2 }) = plane_cone_section(&p, &c, &reach, band)
    {
        let worst = cut.miss(&lines(&l1, &l2), (0.0, 4.0 / alpha.cos()));
        if worst > e {
            return Some(format!(
                "{label}: served a pair {:.4}·ε off the section",
                worst / e
            ));
        }
    }
    None
}

/// **A plane built through the apex does not read its nappe off a gap
/// that is rounding alone.** The review's pose, at ε = 1e-9: its plane's
/// stored origin stands some 55 off the apex, so the gap reads
/// `−1.78e-15` where it is `+1.84e-15`, and the near start `|δ|/M`,
/// read on the nappe the window reaches, stood the pair `24·ε` off a
/// section starting `67.7·ε` out.
#[test]
fn a_plane_through_the_apex_reads_its_gaps_sign_to_its_rounding() {
    let missed = through_the_apex_reads_its_gaps_rounding(
        "the review's pose",
        (
            [2.288555631545491, -1.6126513517833638, -0.31005890382800105],
            [
                0.09983488462206984,
                -0.1622776398934971,
                -0.9816817016747814,
            ],
            [
                0.9950040179881176,
                0.016282315611415507,
                0.09849817453045641,
            ],
            0.39661458990091586,
            [30.01237900854066, 46.07881013715315, -21.375307242251008],
            [-0.8190023497470599, 0.24743249568078224, -0.517699054654318],
        ),
    );
    assert!(missed.is_none(), "{}", missed.unwrap_or_default());
}

/// **The family**: planes built through the apex at random poses, each
/// leaning to `D` in `1e-9…1e-5` and stored with its origin 10 to 60
/// off the apex, so its gap reads the opposite sign to the truth. A
/// search over 20,000 such draws found 892 sign flips, 104 of them
/// served past the band where the nappe was read off that sign; these
/// twelve span `1.8·ε` to `105·ε`.
#[test]
fn planes_through_the_apex_read_their_gaps_sign_to_its_rounding() {
    let poses: [Pose; 12] = [
        // 1.8·ε off at the frozen head
        (
            [2.4025411935435748, -1.4746658448701817, 0.8834801343833865],
            [-0.46335346098759544, 0.1214560959451559, 0.8778109061458385],
            [
                -0.8169353690138936,
                -0.4423903122676662,
                -0.37001001941278594,
            ],
            0.37531639149235674,
            [23.740520577428374, -17.9845255052883, -20.69201918763105],
            [0.285578889988243, 0.8762594151862328, -0.3880903694895873],
        ),
        // 4.1·ε off at the frozen head
        (
            [0.33811282640334606, 0.0705031131770702, -0.6541660345019022],
            [
                -0.43608861298099855,
                -0.8565281754303055,
                -0.27601848909509835,
            ],
            [0.8744023620530446, -0.3308061905276273, -0.3549475645002449],
            0.26501204217155294,
            [-39.18288867723395, -24.883279540191527, -3.484074243622697],
            [
                -0.39958884548626294,
                0.6999818335724339,
                -0.5919072454629433,
            ],
        ),
        // 12.8·ε off at the frozen head
        (
            [
                -0.0043283772545303755,
                -0.0027131576124703022,
                -0.0034406492220980145,
            ],
            [0.38796946808235805, 0.7716441887128723, -0.5040287073784044],
            [-0.8852771642605318, 0.4641487594882353, 0.0291594153638846],
            0.1482496231192877,
            [-1.6361625594969529, 10.27116667036495, -18.315702535873992],
            [-0.493420525504461, 0.7390614977324577, 0.45861125976185757],
        ),
        // 14.9·ε off at the frozen head
        (
            [1.1450073433041927, -1.470290949107634, -1.9690077905877401],
            [0.8845020738867584, 0.04812792823315296, 0.4640471784355638],
            [0.31723907649858113, 0.6672655552858093, -0.6738813301104638],
            0.5544603354090435,
            [-21.504689827808953, 26.46976141174052, -9.29441746528368],
            [-0.7651749747376162, -0.5211929939230157, 0.3779750271127073],
        ),
        // 81.3·ε off at the frozen head
        (
            [0.44672867708527514, 1.0209509214543344, 0.4588076200816059],
            [
                -0.5966782866199086,
                -0.7661753650258358,
                -0.23864268751390125,
            ],
            [-0.7936589033168566, 0.5194480210963413, 0.31666906790051275],
            1.364469885805073,
            [46.389216698599725, -23.622460711526394, -1.0221020641034988],
            [0.47244048059956234, 0.8738792944013594, 0.11460790159250295],
        ),
        // 13.9·ε off at the frozen head
        (
            [
                -1.4838748188562778,
                -1.9719939336101773,
                0.16337893238141646,
            ],
            [0.908978076050037, -0.1459130976994547, -0.3904717981879391],
            [-0.04483468102771216, 0.897076952080374, -0.4395938960260138],
            0.15165477688491907,
            [-20.82604917450526, 0.8042963624066759, 19.91259044801621],
            [0.4334738885732321, 0.8478570037279327, 0.3053504366374697],
        ),
        // 19.5·ε off at the frozen head
        (
            [-1.691011195189976, 0.9692680912757569, 1.3604110758632577],
            [
                -0.25347651304860025,
                0.9507923492225291,
                -0.17816724163725264,
            ],
            [
                -0.35606462343842243,
                0.07954634946579181,
                0.9310694722856724,
            ],
            0.8645934457087794,
            [17.28912634503809, 28.319406753140164, -19.40594081445473],
            [-0.8001451164677668, 0.5972530802862486, 0.05528608035818744],
        ),
        // 6.6·ε off at the frozen head
        (
            [0.3139012303844912, 0.13490430234222692, 0.10550107028224068],
            [-0.637747782083508, 0.4759159672518881, -0.6056250973682202],
            [
                -0.6249533001993683,
                -0.7793240506103604,
                0.04568803683874353,
            ],
            0.06978365165819356,
            [31.992640033861818, -6.6627169886897395, 38.073026050872144],
            [0.7725595723465767, 0.0886772247224962, -0.6287193785713866],
        ),
        // 16.4·ε off at the frozen head
        (
            [1.4656228125662287, 1.5496188873272976, -1.5447589902650534],
            [0.6161501987006509, -0.7008341584355319, 0.3594306817887257],
            [
                -0.11079168910347062,
                -0.5289278454816692,
                -0.8414039077040939,
            ],
            0.2920383764992235,
            [-35.8909209413708, 22.861633849462642, -16.891997630825227],
            [-0.574499528093725, -0.6904060009306594, 0.4396474111137489],
        ),
        // 35.4·ε off at the frozen head
        (
            [0.088414819670411, 0.13182394094887634, 0.26946539334152203],
            [
                -0.983770026807748,
                -0.1537279709604785,
                -0.09254320774134125,
            ],
            [-0.16195732871091623, 0.982761958376845, 0.08915580095615049],
            0.643741663741751,
            [42.724018539596955, 10.650236707822742, -27.212051583400367],
            [0.5037997410621137, 0.1704307582163594, 0.8468407037687455],
        ),
        // 105.1·ε off at the frozen head
        (
            [0.5601129491246063, -1.4671334782525358, -2.1699573812835866],
            [
                0.7354368942258926,
                -0.6762066427198851,
                -0.04332610013461952,
            ],
            [0.5934328114862354, 0.6736373100188265, -0.4405113764730051],
            1.3551155721322217,
            [-25.971567348084893, -38.58123221339789, -28.236565495244257],
            [0.8475988973854985, -0.5143819732136776, -0.1303353167168855],
        ),
        // 17.1·ε off at the frozen head
        (
            [-0.8418815734099635, 0.7732260717814133, 0.820968180901873],
            [0.9815974436078495, 0.09990303613265694, 0.16274471442726465],
            [0.18181958912131121, -0.749480945640441, -0.6365688879718109],
            0.5469094446436249,
            [-16.036344433161776, 7.894336987622844, 12.090454158823077],
            [
                -0.435204826907871,
                -0.9001519257049213,
                -0.017979690926483438,
            ],
        ),
    ];
    let missed: Vec<String> = poses
        .into_iter()
        .enumerate()
        .filter_map(|(i, pose)| {
            through_the_apex_reads_its_gaps_rounding(&format!("pose {i}"), pose)
        })
        .collect();
    assert!(missed.is_empty(), "{} of 12: {missed:#?}", missed.len());
}
