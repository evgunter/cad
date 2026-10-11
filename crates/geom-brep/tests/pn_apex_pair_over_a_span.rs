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

/// What a draw met: the pair, refused naming a conic or escalated.
fn kind(got: &Result<PlaneConeSection<f64>, SectionError>) -> String {
    match got {
        Ok(PlaneConeSection::ApexLinePair { .. }) => "pair".to_owned(),
        Ok(other) => format!("{other:?}")
            .split(['(', ' ', '{'])
            .next()
            .unwrap()
            .to_owned(),
        Err(r) => match r.outside_conic() {
            Some(conic) => format!("refused {conic:?}"),
            None => format!("{r:?}")
                .split(['(', ' ', '{'])
                .next()
                .unwrap()
                .to_owned(),
        },
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
        let u_ref = axis.cross(unit(&mut g)).normalize();
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
        let radial = u_ref * azimuth.cos() + axis.cross(u_ref) * azimuth.sin();
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
/// through the apex, it is refused naming the hyperbola. The same span
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
                let worst = cut.miss(
            &lines(l1, l2),
            slant);
                assert!(
                    worst <= eps,
                    "{label}: the pair stands {:.4}·ε off the section",
                    worst / eps
                );
            }
            Err(e) if !served => assert!(
                e.outside_conic() == Some(geom_brep::intersect::OutsideConic::Hyperbola),
                "{label}: wanted the hyperbola refusal, got {e:?}"
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
/// short of the section's nearest: refused naming the hyperbola. From
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
            matches!(&got, Err(e) if e.outside_conic() == Some(geom_brep::intersect::OutsideConic::Hyperbola)),
            "from half the start, side {side}: got {got:?}"
        );
        let clear = (side * 2.0 * start * alpha.cos(), side);
        let (lo, hi) = (clear.0.min(clear.1), clear.0.max(clear.1));
        let got = plane_cone_section(&p, &c, &window(lo, hi, 1.0), band);
        let Ok(PlaneConeSection::ApexLinePair { l1, l2 }) = &got else {
            panic!("from twice the start, side {side}: got {got:?}");
        };
        let slant = (lo / alpha.cos(), hi / alpha.cos());
        let worst = cut.miss(
            &lines(l1, l2),
            slant);
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
    let worst = cut.miss(
            &lines(l1, l2),
            slant);
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
        let worst = cut.miss(
            &lines(l1, l2),
            (lo / alpha.cos(), hi / alpha.cos()),
        );
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
        let worst = cut.miss(
            &lines(l1, l2),
            (lo / alpha.cos(), hi / alpha.cos()),
        );
        assert!(
            worst <= e,
            "served a pair {:.4}·ε off the section",
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
    let worst = cut.miss(
            &lines(l1, l2),
            (lo / alpha.cos(), hi / alpha.cos()),
    );
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
        let worst = cut.miss(
            &lines(l1, l2),
            (lo / alpha.cos(), hi / alpha.cos()),
        );
        assert!(
            worst <= e,
            "served a pair {:.4}·ε off the section",
            worst / e
        );
    }
}
