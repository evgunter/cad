//! **Which of the torus arm's typed predicates a real input reaches**,
//! and what the ones nothing reaches are doing there.
//!
//! A predicate no row names is a predicate whose Zero arm could be
//! deleted, inverted or misspelled without a test noticing. The torus
//! arm added twelve, and until this module only `bool_ray_torus_disc`
//! was named by a row. What is here is one census over a pose lattice
//! plus the branch-selection rows the census cannot see from outside,
//! and an explicit disposition for each predicate the census does NOT
//! reach — because "no row names it" and "no input can reach it" are
//! very different claims and only the second one excuses the first.

#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

use super::*;
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
    line_torus_roots(q, d.normalize(), centre(), axis(), R_MAJOR, R_MINOR, band())
}

/// The coefficient the biquadratic branch is selected on, recomputed in
/// the row rather than read out of the arm — so a pose can be CLASSIFIED
/// here and the branch it must take asserted, which is the only way this
/// module can see a selection the return type does not expose.
fn q_hat(q: Point3<f64>, d: Vec3<f64>) -> f64 {
    let d = d.normalize();
    let w0 = q - centre();
    let e = d.dot(axis());
    let perp = w0 - d * w0.dot(d);
    8.0 * R_MAJOR.powi(2) * e * perp.dot(axis())
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
    // `bool_ray_torus_disc` Negative, and `_shape`/`_depth` on the
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
/// closed form (`bool_ray_torus_odd` Zero), while a tilted ray off the
/// midplane has `q̂` definitely nonzero and must take Ferrari's
/// resolvent — the branch that reaches `bool_ray_torus_split_lead` and
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
/// bisected), and nothing here does. At a large torus a ray does
/// ([`a_far_ray_reaches_the_count_disagreement`]).
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
/// classifying sign definite — so `bool_ray_torus_shape` and
/// `bool_ray_torus_depth` are not merely reached but reached on BOTH
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

/// **The quartic ladder's two counts come apart on coefficients an input
/// gives**, so a disagreement is reachable and typed, not a panic. These
/// are the depressed quartics the half-angle lane hands
/// [`depressed_quartic_roots`] for `circle_wall_rows`' and
/// `ellipse_rows`' `a_graze_is_read_by_its_depth` (a circle crossing a
/// 500 m wall by 1e-8 m; ellipses on 50 m and 500 m walls), at their
/// band. Counted exactly (`work/inside/ray-torus-count-certifies-on-an-uncharged-discriminant.md`),
/// they have 2, 4, 4 and 4 real roots: the construction is right each
/// time and the certified count is the wrong one — rung 1's `Δ/L¹¹`
/// carries rounding of 3e-7 to 6e-6, far past the band, and two of the
/// four are in band exactly.
#[test]
fn the_ladder_disagrees_with_itself_on_a_lanes_coefficients() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    for (p, q, s, lever, certified, constructed) in [
        (
            -13_656_854.407_337_9,
            -413.763_656_616_210_94,
            46_627_416_997_969.55,
            1000.0,
            4,
            2,
        ),
        (
            -49_164.676_076_010_146,
            -0.121_574_014_425_277_71,
            604_291_324.293_685_2,
            60.0,
            2,
            4,
        ),
        (
            -4_916_467.545_374_04,
            -24.314_849_853_515_625,
            6_042_913_242_936.852,
            600.0,
            2,
            4,
        ),
        (
            -49_164.676_853_847_835,
            -0.243_148_043_751_716_6,
            604_291_324.293_685_2,
            60.0,
            2,
            4,
        ),
    ] {
        match depressed_quartic_roots(p, q, s, lever, &RAY_TORUS_ROWS, band) {
            Err(RootRefusal::Unsettled(Unsettled::CountDisagrees {
                certified: c,
                constructed: b,
            })) => assert_eq!((c, b), (certified, constructed), "({p}, {q}, {s}, {lever})"),
            other => panic!("({p}, {q}, {s}, {lever}): {other:?}"),
        }
    }
}

/// **A ray at a large torus reaches the disagreement.** Probed by the
/// review of PR 4607 among 400k near-tangent rays at `R = 1e6`,
/// `r = 1e4`: this ray's quartic has four real roots (exactly, on the
/// kernel's own coefficients and on the geometry), rung 1 certifies two
/// on `Δ`'s rounding, and the factorization builds four.
#[test]
fn a_far_ray_reaches_the_count_disagreement() {
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
    let band = Band::new(1e-9, 1e-8).unwrap();
    let roots = line_torus_roots(q, d, centre(), axis(), 1e6, 1e4, band);
    assert!(
        matches!(
            roots,
            Err(RootRefusal::Unsettled(Unsettled::CountDisagrees {
                certified: 2,
                constructed: 4
            }))
        ),
        "{roots:?}"
    );
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
        let text = e.to_string();
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
