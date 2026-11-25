//! **Which of the torus arm's typed predicates a real input reaches**,
//! and what the ones nothing reaches are doing there.
//!
//! A predicate no row names is a predicate whose Zero arm could be
//! deleted, inverted or misspelled without a test noticing. The torus
//! arm added twelve, and until this module only `line_quadric_torus_disc`
//! was named by a row. What is here is one census over a pose lattice
//! plus the branch-selection rows the census cannot see from outside,
//! and an explicit disposition for each predicate the census does NOT
//! reach — because "no row names it" and "no input can reach it" are
//! very different claims and only the second one excuses the first.

#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

use super::*;
use geom_brep::quartic::depressed_quartic_roots;
use geom_core::{Band, Point3, Tol, Vec3};

const R_MAJOR: f64 = 1.0;
const R_MINOR: f64 = 0.3;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn centre() -> Point3<f64> {
    Point3::new(0.0, 0.0, 0.0)
}

fn axis() -> Vec3<f64> {
    Vec3::new(0.0, 1.0, 0.0)
}

fn roots(q: Point3<f64>, d: Vec3<f64>) -> Result<QuarticRoots<f64>, RootRefusal> {
    line_torus_roots(
        geom::Line {
            origin: q,
            dir: geom_core::UnitVec3::squared(d),
        },
        geom::Line {
            origin: centre(),
            dir: geom::stated_direction(axis()),
        },
        R_MAJOR,
        R_MINOR,
        band(),
    )
}

/// The coefficient the biquadratic branch is selected on, read from the
/// arm's own formation — so a pose can be CLASSIFIED here and the branch
/// it must take asserted, which is the only way this module can see a
/// selection the return type does not expose.
fn q_hat(q: Point3<f64>, d: Vec3<f64>) -> f64 {
    geom_brep::curve_face::line_quadric::line_torus_quartic(
        geom::Line {
            origin: q,
            dir: geom_core::UnitVec3::squared(d),
        },
        geom::Line {
            origin: centre(),
            dir: geom::stated_direction(axis()),
        },
        R_MAJOR,
        R_MINOR,
    )
    .q_hat
}

/// **The census.** Every outcome the arm can answer, reached by a real
/// pose on a real torus — so none of the sign ladder's arms is dead
/// code, and a Zero arm that stopped being taken would go red here.
#[test]
fn every_quartic_outcome_is_reached_by_a_real_pose() {
    let mut miss = 0;
    let mut two = 0;
    let mut four = 0;
    let mut uncertain = 0;
    // A lattice of origins and directions, plus the named poses: through
    // the hole (four roots), along the axis (a miss), tangent to the top
    // circle (uncertain), and a generic pierce (two).
    let mut poses = vec![
        (centre(), Vec3::new(1.0, 0.0, 0.0)),
        (centre(), Vec3::new(0.0, 1.0, 0.0)),
        (Point3::new(0.0, R_MINOR, R_MAJOR), Vec3::new(1.0, 0.0, 0.0)),
        (Point3::new(3.0, 0.2, 0.1), Vec3::new(-1.0, -0.05, 0.0)),
    ];
    for i in 0..7 {
        for j in 0..5 {
            let x = -1.8 + 0.6 * f64::from(i);
            let y = -0.6 + 0.3 * f64::from(j);
            poses.push((Point3::new(x, y, 0.35), Vec3::new(0.31, 0.17, 0.93)));
            poses.push((Point3::new(x, y, -0.7), Vec3::new(1.0, 0.02, 0.05)));
        }
    }
    for (q, d) in poses {
        match roots(q, d) {
            Ok(QuarticRoots::Miss) => miss += 1,
            Ok(QuarticRoots::Certified(found)) if found.count() == 2 => two += 1,
            Ok(QuarticRoots::Certified(found)) if found.count() == 4 => four += 1,
            Ok(QuarticRoots::Certified(found)) => {
                let count = found.count();
                panic!("a certified count is 2 or 4, never {count}")
            }
            Err(RootRefusal::Unsettled(Unsettled::CountDisagrees { .. })) => {
                panic!("the constructed roots disagree")
            }
            Err(_) => uncertain += 1,
        }
    }
    // `line_quadric_torus_disc` Negative, and `_shape`/`_depth` on the
    // positive branch, are what these three separate.
    assert!(miss > 0, "no pose missed the tube");
    assert!(two > 0, "no pose pierced it twice");
    assert!(four > 0, "no pose crossed all four walls");
    assert!(
        uncertain > 0,
        "no pose left the count uncertain — the tangency arm is then unreached and \
         nothing would notice it being deleted"
    );
}

/// **The branch selection**, which the return type cannot show: a
/// midplane ray has `q̂ = 0` exactly and MUST take the biquadratic
/// closed form (`line_quadric_torus_odd` Zero), while a tilted ray off the
/// midplane has `q̂` definitely nonzero and must take Ferrari's
/// resolvent — the branch that reaches `line_quadric_torus_split_lead` and
/// the cube root. Both must produce the counts the geometry demands.
#[test]
fn both_closed_forms_are_selected_and_both_answer() {
    // Biquadratic: through the hole in the midplane. Four roots at
    // `±(R ± r)`, and `q̂` is exactly zero because `e = 0`.
    let (q, d) = (centre(), Vec3::new(1.0, 0.0, 0.0));
    assert_eq!(q_hat(q, d), 0.0, "a midplane ray has q̂ = 0 exactly");
    let Ok(QuarticRoots::Certified(found)) = roots(q, d) else {
        panic!("the four-root midplane ray must certify")
    };
    let (count, ts) = (found.count(), found.roots());
    assert_eq!(count, 4);
    let mut got: Vec<f64> = ts.to_vec();
    got.sort_by(f64::total_cmp);
    for (g, w) in got.iter().zip([
        -(R_MAJOR + R_MINOR),
        -(R_MAJOR - R_MINOR),
        R_MAJOR - R_MINOR,
        R_MAJOR + R_MINOR,
    ]) {
        assert!((g - w).abs() < 1e-9, "biquadratic roots {got:?}");
    }

    // Ferrari: a tilted ray whose `q̂` is far from the band, piercing the
    // tube twice. This is the ONLY branch that calls the cube root.
    let (q, d) = (Point3::new(-2.0, -0.4, 0.2), Vec3::new(1.0, 0.35, 0.05));
    assert!(
        q_hat(q, d).abs() > 1e-3,
        "this pose must select the resolvent, not the biquadratic form"
    );
    let Ok(QuarticRoots::Certified(found)) = roots(q, d) else {
        panic!("a definite pierce must certify")
    };
    let (count, ts) = (found.count(), found.roots());
    assert_eq!(count, 2, "a tilted pierce crosses the tube twice");
    // Each root is on the tube, to the arm's own accuracy: the check
    // that the resolvent, the cube root and the depression compose.
    for &t in &ts[..count] {
        let p = q + d.normalize() * t;
        let h = p.y;
        let rho = p.x.hypot(p.z);
        let resid = ((rho - R_MAJOR).powi(2) + h * h).sqrt() - R_MINOR;
        assert!(
            resid.abs() < 1e-9,
            "a constructed root is {resid:e} off the tube — the Ferrari branch, its \
             resolvent or the sqrt-chain cube root has drifted"
        );
    }
}

/// **On a unit torus the certified count and the constructed roots
/// agree.** Rung 5 compares the CONSTRUCTED root count against the
/// CERTIFIED one; at this scale neither review lane's oracle reached a
/// disagreement (1310 rays geometrically counted, 3000+ independently
/// bisected), and nothing here does. At a large torus a ray once did,
/// on the plain formula's rounding of `Δ`, and escalates now
/// ([`a_far_rays_in_band_discriminant_escalates`]).
///
/// What this row pins is the invariant that would have to break for it
/// to fire here: every certified answer carries exactly `count` roots,
/// and every one of them is on the tube.
#[test]
fn the_certified_count_and_the_constructed_roots_agree_on_a_lattice() {
    let mut certified = 0;
    for i in 0..12 {
        for j in 0..7 {
            let q = Point3::new(-2.2 + 0.4 * f64::from(i), -0.45 + 0.15 * f64::from(j), 0.13);
            let d = Vec3::new(0.87, 0.21, 0.44);
            if let Ok(QuarticRoots::Certified(found)) = roots(q, d) {
                let (count, ts) = (found.count(), found.roots());
                certified += 1;
                for &t in &ts[..count] {
                    let p = q + d.normalize() * t;
                    let resid = ((p.x.hypot(p.z) - R_MAJOR).powi(2) + p.y * p.y).sqrt() - R_MINOR;
                    assert!(
                        resid.abs() < 1e-8,
                        "a root the arm certified is {resid:e} off the tube"
                    );
                }
            }
        }
    }
    assert!(certified > 20, "the lattice must actually certify counts");
}

/// The sign ladder's own arithmetic, at the poses that make each
/// classifying sign definite — so `line_quadric_torus_shape` and
/// `line_quadric_torus_depth` are not merely reached but reached on BOTH
/// sides of the four-versus-none split they exist to make.
#[test]
fn the_four_versus_none_split_is_exercised_both_ways() {
    // Positive discriminant, four real roots: the hole.
    assert!(matches!(
        roots(centre(), Vec3::new(1.0, 0.0, 0.0)),
        Ok(QuarticRoots::Certified(found)) if found.count() == 4
    ));
    // Positive discriminant, NO real roots: a ray through the hole but
    // tilted steeply enough to leave through the middle without meeting
    // the tube.
    assert!(
        matches!(
            roots(centre(), Vec3::new(0.02, 1.0, 0.0)),
            Ok(QuarticRoots::Miss)
        ),
        "a steep ray up the hole meets nothing"
    );
    // And the ladder never answers on a Zero: an exactly axial ray.
    let axial = roots(centre(), Vec3::new(0.0, 1.0, 0.0));
    assert!(
        matches!(
            axial,
            Ok(QuarticRoots::Miss) | Err(RootRefusal::Unsettled(_))
        ),
        "the axis meets no tube, and must never be answered as a crossing: {:?}",
        axial.map(|r| match r {
            QuarticRoots::Certified(found) => found.count(),
            _ => 0,
        })
    );
}

/// **On a lane's coefficients the ladder certifies the exact count or
/// none.** These are the depressed quartics the half-angle lane hands
/// [`depressed_quartic_roots`] for `circle_wall_rows`' and
/// `ellipse_rows`' `a_graze_is_read_by_its_depth` (a circle crossing a
/// 500 m wall by 1e-8 m; ellipses on 50 m and 500 m walls), at their
/// band. Counted exactly on their `f64` coefficients they have 2, 4, 4
/// and 4 real roots, and two of the four are in band. Read off the plain
/// formula, `Δ/L¹¹` rounds by 3e-7 to 6e-6, far past the band, and rung 1
/// certified the wrong count on every one, which the factorization then
/// contradicted. Read exactly, each certifies its own count or reads in
/// band: never another count, never a contradiction.
#[test]
fn a_lanes_coefficients_certify_their_exact_count_or_none() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    for (p, q, s, lever, exact) in [
        (
            -13_656_854.407_337_9,
            -413.763_656_616_210_94,
            46_627_416_997_969.55,
            1000.0,
            2,
        ),
        (
            -49_164.676_076_010_146,
            -0.121_574_014_425_277_71,
            604_291_324.293_685_2,
            60.0,
            4,
        ),
        (
            -4_916_467.545_374_04,
            -24.314_849_853_515_625,
            6_042_913_242_936.852,
            600.0,
            4,
        ),
        (
            -49_164.676_853_847_835,
            -0.243_148_043_751_716_6,
            604_291_324.293_685_2,
            60.0,
            4,
        ),
    ] {
        let at = format!("({p}, {q}, {s}, {lever})");
        assert_eq!(
            geom_brep::quartic::exact_root_count(p, q, s),
            Some(exact),
            "{at}: the fixture's exact count"
        );
        let answer = depressed_quartic_roots(p, q, s, lever, &geom_brep::rows::LINE_TORUS, band);
        let read = match &answer {
            Ok(QuarticRoots::Certified(found)) => found.count() == exact,
            Ok(QuarticRoots::Miss) => exact == 0,
            Err(RootRefusal::Escalated(_) | RootRefusal::Unsettled(Unsettled::Tangency)) => true,
            _ => false,
        };
        assert!(read, "{at}: not its count, {answer:?}");
    }
}

/// **A ray at a large torus whose discriminant is in band escalates.**
/// Probed by the review of PR 4607 among 400k near-tangent rays at
/// `R = 1e6`, `r = 1e4`: this ray's quartic has four real roots (exactly,
/// on the kernel's own coefficients and on the geometry). Read off the
/// plain formula, rung 1 certified two, which the factorization
/// contradicted (`CountDisagrees`); read exactly, `Δ/L¹¹` is 6.5e-9, in
/// the band's gap.
#[test]
fn a_far_rays_in_band_discriminant_escalates() {
    let q = Point3::new(
        -191_510.386_776_324_04,
        7_625.099_170_928_492,
        -454_591.902_880_141_8,
    );
    let d = Vec3::new(
        0.634_206_811_213_916_8,
        0.003_546_043_054_673_617_2,
        -0.773_155_318_282_510_4,
    );
    let answer = far_ray(q, d);
    assert!(
        matches!(
            &answer,
            Err(RootRefusal::Escalated(diag))
                if diag.predicate == Some("line_quadric_torus_disc")
        ),
        "{answer:?}"
    );
}

/// [`line_torus_roots`] at the `R = 1e6`, `r = 1e4` torus about the
/// origin, at the band `(1e-9, 1e-8)`.
fn far_ray(q: Point3<f64>, d: Vec3<f64>) -> Result<QuarticRoots<f64>, RootRefusal> {
    line_torus_roots(
        geom::Line {
            origin: q,
            dir: geom::stated_direction(d),
        },
        geom::Line {
            origin: centre(),
            dir: geom::stated_direction(axis()),
        },
        1e6,
        1e4,
        Band::new(1e-9, 1e-8).unwrap(),
    )
}

/// **A ray that crosses the tube is not certified a miss on its quartic's
/// rounding.** Seven near-tangent rays at the `R = 1e6`, `r = 1e4` torus,
/// each aimed `depth` inside the tube at its tangent point (`depth` 3.4e-8
/// to 6.4e-7 m, past the rounding of placing it): each one's quartic, on
/// the coefficients [`line_torus_roots`] forms ([`far_coefficients`]
/// reads its formation), has exactly two real roots, and read off the plain
/// formula for `Δ` rung 1 certified [`QuarticRoots::Miss`] for every one
/// — a silent wrong answer, since a miss folds as no crossing. Each must
/// now answer two crossings or no count.
#[test]
fn a_crossing_ray_is_no_miss_on_its_quartics_rounding() {
    for (depth, q, d) in [
        (
            6.45e-7,
            [
                -186_689.342_377_749_5,
                -10_107.797_434_978_947,
                418_944.816_205_863_43,
            ],
            [
                -0.720_065_348_789_403_2,
                0.000_192_516_199_071_661_74,
                0.693_906_230_272_007_5,
            ],
        ),
        (
            6.03e-7,
            [
                1_389_985.555_750_069_7,
                15_264.031_639_281_602,
                -209_933.784_523_896_12,
            ],
            [
                -0.989_293_749_475_981_8,
                -0.012_167_500_736_121_458,
                -0.145_429_808_408_007_45,
            ],
        ),
        (
            1.71e-7,
            [
                1_163_334.885_147_786_7,
                5_430.152_229_364_13,
                580_145.420_003_653_2,
            ],
            [
                -0.951_941_165_740_203_3,
                0.015_008_748_074_768_985,
                -0.305_912_985_096_106_76,
            ],
        ),
        (
            6.50e-8,
            [
                1_340_085.469_964_106_6,
                253.951_759_561_665_3,
                228_666.567_362_518_46,
            ],
            [
                -0.907_359_933_636_184,
                0.025_825_269_227_473_76,
                -0.419_560_491_825_753_7,
            ],
        ),
        (
            5.16e-8,
            [
                657_568.738_112_575_1,
                513.964_547_544_679_6,
                -225_037.893_066_800_78,
            ],
            [
                0.938_552_644_018_461_6,
                0.031_098_660_949_110_356,
                -0.343_732_174_364_179_85,
            ],
        ),
        (
            4.01e-8,
            [
                1_451_336.129_213_225_8,
                1_432.756_761_093_380_2,
                6_266.228_990_917_694,
            ],
            [
                -0.999_654_077_342_287_2,
                -0.025_326_745_373_918_966,
                0.007_090_953_511_699_898,
            ],
        ),
        (
            3.36e-8,
            [
                -2_048_976.774_870_816_2,
                50_964.908_536_634_33,
                131_372.375_433_093_02,
            ],
            [
                0.997_882_511_179_054_8,
                -0.057_736_900_261_684_41,
                -0.029_949_027_215_520_133,
            ],
        ),
    ] {
        let (q, d) = (Point3::new(q[0], q[1], q[2]), Vec3::new(d[0], d[1], d[2]));
        let (p, q_hat, s) = far_coefficients(q, d);
        assert_eq!(
            geom_brep::quartic::exact_root_count(p, q_hat, s),
            Some(2),
            "the ray {depth:e} inside: its quartic's exact count"
        );
        let answer = far_ray(q, d);
        let read = match &answer {
            Ok(QuarticRoots::Certified(found)) => found.count() == 2,
            Err(RootRefusal::Escalated(_) | RootRefusal::Unsettled(Unsettled::Tangency)) => true,
            _ => false,
        };
        assert!(read, "the ray {depth:e} inside the tube: {answer:?}");
    }
}

/// The depressed quartic's `(p, q̂, s)` for [`far_ray`]'s ray and torus,
/// as [`line_torus_roots`] forms them (its one formation).
fn far_coefficients(q: Point3<f64>, d: Vec3<f64>) -> (f64, f64, f64) {
    let quartic = geom_brep::curve_face::line_quadric::line_torus_quartic(
        geom::Line {
            origin: q,
            dir: geom::stated_direction(d),
        },
        geom::Line {
            origin: centre(),
            dir: geom::stated_direction(axis()),
        },
        1e6,
        1e4,
    );
    (quartic.p, quartic.q_hat, quartic.s)
}

/// **A ray whose crossings could not be built to their certified count is
/// blocked, not the query refused**, through the arm's own reading of the
/// quartic ([`torus_crossings`], which `cast_ray` calls). The refusal
/// carries both counts and tells one story at every door — the solid
/// door, the census and the validator: its lever, and that it may
/// indicate a kernel bug.
#[test]
fn a_count_disagreement_blocks_the_ray_and_says_so() {
    let face = FaceKey::default();
    for (certified, constructed) in [(2, 4), (4, 2), (2, 0)] {
        let roots: Result<QuarticRoots<f64>, Unsettled> = Err(Unsettled::CountDisagrees {
            certified,
            constructed,
        });
        let Err(RayFault::Blocked(e)) = torus_crossings(face, roots) else {
            panic!("({certified}, {constructed}): the ray is blocked");
        };
        assert_eq!(
            e,
            PointInSolidError::RootCountDisagrees {
                face,
                certified,
                constructed
            }
        );
        assert!(e.confined() && !e.in_band(), "{e:?}");
        let text = e.render(geom_brep::recourse::Reading::Build);
        for want in [
            format!("({constructed}) disagree"),
            format!("certify ({certified})"),
            "Recourse: test a point farther from that face".to_owned(),
        ] {
            assert!(text.contains(&want), "{want}: {text}");
        }
        let census = crate::census::Undecided::of_point_in_solid(&e).what();
        let at_rest = crate::validate::classify_point_in_solid(&e).1.into_owned();
        for (door, ending) in [
            ("solid", text.as_str()),
            ("census", census),
            ("validator", at_rest.as_str()),
        ] {
            assert!(ending.contains("Recourse: "), "{door}: {ending}");
            assert!(ending.ends_with(KERNEL_BUG_NOTE), "{door}: {ending}");
            assert!(!ending.contains("unreadable"), "{door}: {ending}");
        }
    }
}
