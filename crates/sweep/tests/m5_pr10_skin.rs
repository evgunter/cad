//! M5 PR 10 §2 — the §10.3/§10.4 constructions, verified by
//! EVALUATION rather than by assertion.
//!
//! The interpolation claim a skinned surface makes is precise: at each
//! section's own v-parameter, the surface reproduces that section
//! curve — the whole curve, not just its ends. These rows check that
//! by evaluating both sides on a shared u-schedule and comparing.
//!
//! # Multi-ε honesty
//!
//! Nothing here is a tolerance decision: the construction is exact in
//! ℝ (degree elevation, knot refinement, and a collocation solve whose
//! residual is float rounding only), so the probes are placed against
//! the RESOLVED band's ring tolerance and the sample counts are
//! DERIVED from the structure (control counts), never hard-coded to
//! one ε row.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::NurbsCurve3;
use geom_brep::SketchSegment;
use geom_core::Tol;
use geom_core::{Affine3, Band, Point2, Point3, Vec3};
use sweep::skin::{SkinError, lift_surface, make_compatible, segment_curve, skin, skin_parameters};

/// The band this row resolves at — every probe scales from it.
fn band() -> Band {
    Band::linear(Tol::witness()).expect("the linear band resolves")
}

/// Ring tolerance: the acceptance scale for a claim that is exact in
/// ℝ and therefore limited only by float rounding. Scaled from the
/// resolved band's outer edge so every ε row asks the same question.
fn ring() -> f64 {
    band().zero()
}

/// Three sections of one square-with-an-arc profile, at z = 0, 1, 2,
/// with the MIDDLE one scaled — a non-affine triple, so the skin is
/// genuinely curved in v (a two-section skin would be ruled).
fn sections() -> Vec<Vec<NurbsCurve3<f64>>> {
    let at = |z: f64, s: f64| {
        let place = Affine3::translation(Vec3::new(0.0, 0.0, z));
        let p = |x: f64, y: f64| Point2::new(x * s, y * s);
        let segs = [
            SketchSegment::Line {
                a: p(0.0, 0.0),
                b: p(2.0, 0.0),
            },
            SketchSegment::Arc {
                a: p(2.0, 0.0),
                b: p(2.0, 1.0),
                bulge: 0.25,
            },
            SketchSegment::Line {
                a: p(2.0, 1.0),
                b: p(0.0, 1.0),
            },
            SketchSegment::Line {
                a: p(0.0, 1.0),
                b: p(0.0, 0.0),
            },
        ];
        segs.iter()
            .enumerate()
            .map(|(i, s)| segment_curve(i, *s, place).expect("segment converts"))
            .collect::<Vec<_>>()
    };
    // Section 1 is scaled 1.6×: the middle is NOT the average of the
    // ends, so no wall of this loft is a ruled surface.
    vec![at(0.0, 1.0), at(1.0, 1.6), at(2.0, 1.0)]
}

/// The j-th segment of every section, compatible.
fn strip(j: usize) -> Vec<NurbsCurve3<f64>> {
    let all = sections();
    let raw: Vec<NurbsCurve3<f64>> = all.iter().map(|s| s[j].clone()).collect();
    make_compatible(&raw).expect("sections are compatible")
}

#[test]
fn an_arc_segment_converts_to_its_exact_carrier_circle() {
    let place = Affine3::identity();
    let seg = SketchSegment::Arc {
        a: Point2::new(1.0, 0.0),
        b: Point2::new(0.0, 1.0),
        bulge: (core::f64::consts::PI / 8.0).tan(), // a quarter turn
    };
    let c = segment_curve(0, seg, place).expect("converts");
    assert_eq!(c.degree(), 2);
    // Every sample lies on the unit circle — the rational quadratic is
    // the carrier, not a fit of it. Sample count derives from the
    // structure, not from a literal.
    let n = 8 * c.control().len();
    for i in 0..=n {
        #[allow(clippy::cast_precision_loss)]
        let t = i as f64 / n as f64;
        let p = c.eval(t);
        assert!(
            (p.x.hypot(p.y) - 1.0).abs() <= ring(),
            "sample {i} off the carrier: {p:?}"
        );
        assert!(p.z.abs() <= ring());
    }
    assert!(c.eval(0.0).distance(Point3::new(1.0, 0.0, 0.0)) <= ring());
    assert!(c.eval(1.0).distance(Point3::new(0.0, 1.0, 0.0)) <= ring());
}

#[test]
fn a_half_turn_arc_splits_into_sub_arcs_and_stays_exact() {
    // bulge = tan(θ/4) with θ = 3π/2 — three quarter-turn sub-arcs.
    let bulge = (3.0 * core::f64::consts::PI / 8.0).tan();
    let seg = SketchSegment::Arc {
        a: Point2::new(1.0, 0.0),
        b: Point2::new(0.0, -1.0),
        bulge,
    };
    let c = segment_curve(0, seg, Affine3::identity()).expect("converts");
    assert!(
        c.control().len() >= 7,
        "a 3/4-turn needs ≥ 3 sub-arcs, got {} control points",
        c.control().len()
    );
    let n = 8 * c.control().len();
    for i in 0..=n {
        #[allow(clippy::cast_precision_loss)]
        let t = i as f64 / n as f64;
        let p = c.eval(t);
        assert!((p.x.hypot(p.y) - 1.0).abs() <= ring(), "sample {i}: {p:?}");
    }
}

#[test]
fn compatibility_gives_one_degree_and_one_knot_vector() {
    let compat = strip(1);
    let first = compat[0].knots();
    for (i, c) in compat.iter().enumerate() {
        assert_eq!(c.degree(), first.degree(), "section {i} degree");
        assert_eq!(c.knots().knots(), first.knots(), "section {i} knots");
        assert_eq!(c.control().len(), compat[0].control().len());
    }
}

/// Compatibility is exact in ℝ: elevating and refining changes the
/// representation, never the locus.
#[test]
fn compatibility_preserves_every_section_locus() {
    let all = sections();
    for j in 0..4 {
        let raw: Vec<NurbsCurve3<f64>> = all.iter().map(|s| s[j].clone()).collect();
        let compat = make_compatible(&raw).expect("compatible");
        for (k, (before, after)) in raw.iter().zip(&compat).enumerate() {
            let n = 8 * after.control().len();
            for i in 0..=n {
                #[allow(clippy::cast_precision_loss)]
                let t = i as f64 / n as f64;
                assert!(
                    before.eval(t).distance(after.eval(t)) <= ring(),
                    "segment {j} section {k} moved at t = {t}"
                );
            }
        }
    }
}

/// **The review charter's independent §10.3 verification.** Evaluate
/// the skinned surface at each section's own v-parameter and compare
/// against the section curve on a derived u-schedule: interpolation
/// means the surface PASSES THROUGH the sections, exactly, everywhere
/// — not merely at the ends.
#[test]
fn the_skin_passes_through_every_section_at_its_parameter() {
    for j in 0..4 {
        let compat = strip(j);
        let params = skin_parameters(&compat).expect("parameters");
        let surface = skin(&compat, 2).expect("skins");
        let samples = 8 * compat[0].control().len();
        for (k, v) in params.iter().enumerate() {
            for i in 0..=samples {
                #[allow(clippy::cast_precision_loss)]
                let u = i as f64 / samples as f64;
                let on_surface = surface.eval(u, *v);
                let on_section = compat[k].eval(u);
                assert!(
                    on_surface.distance(on_section) <= ring(),
                    "segment {j}: surface at (u={u}, v={v}) is {on_surface:?}, \
                     section {k} is {on_section:?}"
                );
            }
        }
    }
}

/// The middle section is not the average of the ends, so the wall is
/// genuinely curved in v — the loft is definitional, not ruled.
#[test]
fn the_wall_is_genuinely_curved_in_v() {
    let compat = strip(0);
    let params = skin_parameters(&compat).expect("parameters");
    let surface = skin(&compat, 2).expect("skins");
    let mid = surface.eval(0.5, params[1]);
    let ruled = {
        let a = surface.eval(0.5, 0.0);
        let b = surface.eval(0.5, 1.0);
        Point3::new(
            0.5f64.mul_add(b.x - a.x, a.x),
            0.5f64.mul_add(b.y - a.y, a.y),
            0.5f64.mul_add(b.z - a.z, a.z),
        )
    };
    // Scaled from the geometry (0.6 of a 2 m edge at the midpoint),
    // not a magic number: the deviation must be ORDERS above ring.
    assert!(
        mid.distance(ruled) > 1e3 * ring(),
        "the skin is ruled: {mid:?} vs {ruled:?}"
    );
}

/// The lifted surface is the SAME surface at another scalar — Q8's
/// "the produced NURBS is the definition", checked bit-for-bit at f64
/// and by enclosure containment wherever the lane is wider.
#[test]
fn lifting_the_structure_reproduces_it_exactly() {
    let compat = strip(1);
    let surface = skin(&compat, 2).expect("skins");
    let lifted = lift_surface::<f64>(&surface).expect("lifts");
    assert_eq!(lifted.knots_u().knots(), surface.knots_u().knots());
    assert_eq!(lifted.knots_v().knots(), surface.knots_v().knots());
    assert_eq!(lifted.weights(), surface.weights());
    for (a, b) in lifted.control().iter().zip(surface.control()) {
        assert_eq!(a.x.to_bits(), b.x.to_bits());
        assert_eq!(a.y.to_bits(), b.y.to_bits());
        assert_eq!(a.z.to_bits(), b.z.to_bits());
    }
}

/// Determinism (D9): same sections in, same control BITS out.
#[test]
fn the_construction_is_bit_deterministic() {
    let a = skin(&strip(1), 2).expect("skins");
    let b = skin(&strip(1), 2).expect("skins");
    assert_eq!(a.knots_v().knots(), b.knots_v().knots());
    for (p, q) in a.control().iter().zip(b.control()) {
        assert_eq!(p.x.to_bits(), q.x.to_bits());
        assert_eq!(p.y.to_bits(), q.y.to_bits());
        assert_eq!(p.z.to_bits(), q.z.to_bits());
    }
    assert_eq!(a.weights(), b.weights());
}

// ---------------------------------------------------------------------
// The compatibility-refusal probes (§2's typed door)
// ---------------------------------------------------------------------

#[test]
fn one_section_is_not_a_skin() {
    let one = vec![strip(0)[0].clone()];
    assert!(matches!(
        make_compatible(&one),
        Err(SkinError::TooFewSections { have: 1, need: 2 })
    ));
    assert!(matches!(
        skin(&one, 1),
        Err(SkinError::TooFewSections { have: 1, need: 2 })
    ));
}

#[test]
fn a_v_degree_the_sections_cannot_carry_refuses_typed() {
    let compat = strip(0);
    let k = compat.len();
    assert!(matches!(
        skin(&compat, 0),
        Err(SkinError::BadDegree { degree: 0, .. })
    ));
    match skin(&compat, k) {
        Err(SkinError::BadDegree { degree, sections }) => {
            assert_eq!(degree, k);
            assert_eq!(sections, k);
        }
        other => panic!("expected BadDegree, got {}", describe(&other)),
    }
}

#[test]
fn incompatible_sections_refuse_at_the_skin_door() {
    // Straight from `segment_curve`, no compatibility pass: a line
    // (degree 1) and an arc (degree 2) cannot skin.
    let line = segment_curve(
        0,
        SketchSegment::Line {
            a: Point2::new(0.0, 0.0),
            b: Point2::new(1.0, 0.0),
        },
        Affine3::identity(),
    )
    .expect("line");
    let arc = segment_curve(
        1,
        SketchSegment::Arc {
            a: Point2::new(0.0, 1.0),
            b: Point2::new(1.0, 1.0),
            bulge: 0.3,
        },
        Affine3::identity(),
    )
    .expect("arc");
    assert!(matches!(
        skin(&[line.clone(), arc.clone()], 1),
        Err(SkinError::SectionShapeMismatch { section: 1, .. })
    ));
    // And the compatibility pass is exactly the repair.
    let compat = make_compatible(&[line, arc]).expect("compatible");
    assert!(skin(&compat, 1).is_ok());
}

#[test]
fn coincident_sections_refuse_with_the_shared_recourse() {
    let one = strip(0)[0].clone();
    match skin(&[one.clone(), one], 1) {
        Err(e @ SkinError::DegenerateSection { .. }) => {
            let msg = e.to_string();
            assert_eq!(
                msg.matches(geom_core::COINCIDENCE_RECOURSE).count(),
                1,
                "{msg}"
            );
        }
        other => panic!("expected DegenerateSection, got {}", describe(&other)),
    }
}

#[test]
fn a_degenerate_segment_refuses_with_the_shared_recourse() {
    let seg = SketchSegment::Line {
        a: Point2::new(1.0, 1.0),
        b: Point2::new(1.0, 1.0),
    };
    match segment_curve(3, seg, Affine3::identity()) {
        Err(e @ SkinError::DegenerateSection { section: 3, .. }) => {
            let msg = e.to_string();
            assert_eq!(
                msg.matches(geom_core::COINCIDENCE_RECOURSE).count(),
                1,
                "{msg}"
            );
        }
        other => panic!("expected DegenerateSection, got {}", describe(&other)),
    }
}

fn describe<T: core::fmt::Debug>(r: &Result<T, SkinError>) -> String {
    format!("{r:?}")
}
