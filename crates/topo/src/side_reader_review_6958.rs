//! **Reviewer probes: an aimed ray over a box reads the side every
//! member has, inside out or not** (`side_reader::aimed_ray`).
//!
//! A brick at the symbolic box scalar with its top cap's height and its
//! far `x` wall's station each a box, every plane's normal the unit
//! brick's own, exact, or every one of them negated. With the brick's
//! own normals every member's side at infinity is `Out`; with every
//! normal negated every member is inside out and the side is `In`. An
//! aimed ray starts beyond every face whatever it aims at, so from any
//! aim and any normal a reading of the other side is wrong.
//!
//! The unit's own inside-out brick (`aimed_ray_tests`, the top cap
//! moved below the bottom, the stated normals kept) is not this: its
//! caps' normals point into it and its walls' out of it, so a ray that
//! meets a wall first reads `Out` or nothing and one that meets a cap
//! first reads `In` (measured below), and no side is the wrong one.

#![allow(clippy::expect_used, clippy::panic)]

use super::*;
use crate::construction::FaceSurface;
use crate::test_support_fixtures::{FaceGeometry, brick, prism_ops};
use geom::Surface;
use geom_core::{Bounds as _, Interval, ParamSymbol, Real as _, Sym, SymBudget, Tol};

type Boxed = Sym<Interval>;

const SQUARE: [(f64, f64); 4] = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];

/// The brick over `[0, right] × [0, 1] × [0, top]`, every stated normal
/// negated where `inside_out`.
fn brick_at(top: Boxed, right: Boxed, inside_out: bool, tol: Tol) -> Body<Boxed> {
    let iv = Boxed::from_f64;
    let twin = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    let mut body = Body::<Boxed>::new();
    prism_ops(
        &mut body,
        &SQUARE,
        (0.0, 1.0),
        |x, y, z| {
            Point3::new(
                if x == 1.0 { right } else { iv(x) },
                iv(y),
                if z == 1.0 { top } else { iv(z) },
            )
        },
        FaceGeometry::Declined,
        tol,
    );
    for (face, _) in twin.faces() {
        let Surface::Plane { normal, u_ref, .. } = *twin.proven_face(face).surface() else {
            panic!("a brick's face is planar");
        };
        let corner = body.proven_face(face).edges()[0].ends[0];
        let origin = *body
            .get_vertex(corner)
            .and_then(|v| body.get_point(v.point))
            .expect("the boxed brick shares the twin's keys");
        body.set_face_surface(
            face,
            FaceSurface::New {
                surface: Surface::Plane {
                    origin,
                    normal: normal.map(|x| iv(if inside_out { -x } else { x })),
                    u_ref: u_ref.map(iv),
                },
                sense: true,
            },
        )
        .expect("the face takes its plane");
    }
    body
}

/// Runs `read` on the upright brick over `top` and `right`, each
/// `(lo, hi)`.
fn over<R>(top: (f64, f64), right: (f64, f64), read: impl FnOnce(&Body<Boxed>, Band) -> R) -> R {
    over_oriented(top, right, false, read)
}

/// [`over`], the brick inside out where asked.
fn over_oriented<R>(
    top: (f64, f64),
    right: (f64, f64),
    inside_out: bool,
    read: impl FnOnce(&Body<Boxed>, Band) -> R,
) -> R {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness band");
    let budget = SymBudget {
        max_terms: 64,
        max_degree: 8,
    };
    let axis = |name: &str, (lo, hi): (f64, f64)| {
        if lo == hi {
            Boxed::from_f64(lo)
        } else {
            Sym::param_over(
                ParamSymbol::new(test_utils::symbol_id(name)),
                Interval::from_bounds(lo, hi),
                lo,
                hi,
            )
        }
    };
    geom_core::sym::with_session(budget, || {
        let body = brick_at(
            axis("review_6958_top", top),
            axis("review_6958_right", right),
            inside_out,
            tol,
        );
        read(&body, band)
    })
    .0
}

fn tally(readings: &[String]) -> String {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for r in readings {
        let kind = if r.contains("Side(In)") {
            "in"
        } else if r.contains("Side(Out)") {
            "out"
        } else if r.contains("SetAside(Crossing") {
            "aside:crossing"
        } else if r.contains("SetAside(Graze") {
            "aside:graze"
        } else if r.contains("SetAside(Corner") || r.contains("SetAside(Wedge") {
            "aside:corner"
        } else if r.contains("Pair(") {
            "pair"
        } else {
            "other"
        };
        *counts.entry(kind).or_default() += 1;
    }
    format!("{counts:?}")
}

/// **Every face's own aim, upright and inside out, over one and two
/// boxed stations**: no aim reads the side no member has. What each
/// width reads is printed.
#[test]
fn no_aim_of_a_boxed_brick_reads_the_side_no_member_has() {
    let eps = Tol::witness().eps();
    for (what, middle, inside_out) in [
        ("upright", 1.0, false),
        ("inside out", 1.0, true),
        ("upright, thin", 0.3, false),
        ("inside out, thin", 0.3, true),
    ] {
        let wrong = if inside_out { "Side(Out)" } else { "Side(In)" };
        let scale = middle;
        for half in [1e-6, 1e-3, 0.025, 0.1, 0.2, 0.3, 0.45, 0.6, 0.8, 0.9].map(|h| h * scale) {
            for right in [(1.0, 1.0), (1.0 - half.min(0.45), 1.0 + half.min(0.45))] {
                let top = (middle - half, middle + half);
                let readings = over_oriented(top, right, inside_out, |body, band| {
                    each_aim_report(body, band)
                });
                println!(
                    "[review 6958] ε {eps:e} {what} top ±{half:e} right {right:?}: {}",
                    tally(&readings)
                );
                for r in &readings {
                    assert!(
                        !r.contains(wrong),
                        "{what}, top {top:?}, right {right:?}, ε {eps:e}: a wrong side: {r}"
                    );
                }
            }
        }
    }
}

/// **Rays aimed anywhere about the boxed brick, from any normal, read
/// no side no member has** (counterexample search). Half the aims are
/// drawn about the brick; half lie on the top cap's boundary or the
/// moving wall's at a member of the box, corners included, so the ray
/// runs through an edge or a vertex for that member and beside it for
/// the others.
#[test]
#[ignore = "reviewer fuzz, ~1 min: cargo nextest run -p topo -E 'test(/review_6958/)' --run-ignored all"]
fn no_ray_about_a_boxed_brick_reads_the_side_no_member_has() {
    let mut rng = test_utils::fuzz::start("review_6958_boxed_brick_rays");
    let replay = test_utils::fuzz::replay();
    let eps = Tol::witness().eps();
    let boxes = test_utils::fuzz::scaled(6);
    let rays = test_utils::fuzz::scaled(60);
    for (what, inside_out) in [("upright", false), ("inside out", true)] {
        let wrong = if inside_out { "Side(Out)" } else { "Side(In)" };
        let middle = 1.0;
        let mut all = Vec::new();
        for k in 0..boxes {
            let half = rng.log_uniform(1e-4, 0.45);
            let top = (middle - half, middle + half);
            let right = if k % 2 == 0 {
                (1.0, 1.0)
            } else {
                let w = rng.log_uniform(1e-4, 0.3);
                (1.0 - w, 1.0 + w)
            };
            let draws: Vec<([f64; 3], [f64; 3])> = (0..rays)
                .map(|j| {
                    let aim = if j % 2 == 0 {
                        [
                            rng.range(-0.4, 1.5),
                            rng.range(-0.4, 1.4),
                            rng.range(-1.6, 1.6),
                        ]
                    } else {
                        let pick = |rng: &mut test_utils::fuzz::Rng, (lo, hi): (f64, f64)| match rng
                            .below(4)
                        {
                            0 => lo,
                            1 => hi,
                            2 => 0.5 * (lo + hi),
                            _ => rng.range(lo, hi),
                        };
                        let (h, r) = (pick(&mut rng, top), pick(&mut rng, right));
                        let u = rng.unit();
                        match rng.below(7) {
                            0 => [u * r, 0.0, h],
                            1 => [r, u, h],
                            2 => [u * r, 1.0, h],
                            3 => [0.0, u, h],
                            4 => [r, 1.0, h],
                            5 => [r, 0.0, u * h],
                            _ => [r, u, u * h],
                        }
                    };
                    let normal = match rng.below(8) {
                        0 => [1.0, 0.0, 0.0],
                        1 => [0.0, 1.0, 0.0],
                        2 => [0.0, 0.0, 1.0],
                        3 => [0.0, 0.0, -1.0],
                        _ => {
                            let v = [
                                rng.range(-1.0, 1.0),
                                rng.range(-1.0, 1.0),
                                rng.range(-1.0, 1.0),
                            ];
                            let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt().max(1e-3);
                            [v[0] / n, v[1] / n, v[2] / n]
                        }
                    };
                    (aim, normal)
                })
                .collect();
            let readings = over_oriented(top, right, inside_out, |body, band| {
                let face = body.faces().next().expect("a brick has faces").0;
                let iv = Boxed::from_f64;
                draws
                    .iter()
                    .map(|&(a, n)| {
                        ray_at_report(
                            body,
                            face,
                            Point3::new(iv(a[0]), iv(a[1]), iv(a[2])),
                            Vec3::new(iv(n[0]), iv(n[1]), iv(n[2])),
                            band,
                        )
                    })
                    .collect::<Vec<_>>()
            });
            for (r, (aim, normal)) in readings.iter().zip(&draws) {
                assert!(
                    !r.contains(wrong),
                    "{what}, top {top:?}, right {right:?}, ε {eps:e}, aim {aim:?} normal \
                     {normal:?}: a wrong side: {r} ({replay})"
                );
            }
            all.extend(readings);
        }
        println!("[review 6958] ε {eps:e} {what} rays: {}", tally(&all));
    }
}

/// **What sets each face's own ray aside as the top cap's box widens**
/// (a measurement, printed; it asserts only the wrong side): each
/// face's reading, the set-aside ones with the row that raised them.
#[test]
fn what_sets_a_boxed_bricks_rays_aside() {
    for half in [0.05, 0.1, 0.2, 0.3] {
        let readings = over((1.0 - half, 1.0 + half), (1.0, 1.0), |body, band| {
            let caps: Vec<String> = body
                .faces()
                .map(|(face, _)| match *body.proven_face(face).surface() {
                    Surface::Plane { normal, origin, .. } => {
                        format!(
                            "{face:?} n.z=[{}, {}] o.z=[{}, {}]",
                            normal.z.lo(),
                            normal.z.hi(),
                            origin.z.lo(),
                            origin.z.hi()
                        )
                    }
                    _ => String::new(),
                })
                .collect();
            (caps, each_aim_report(body, band))
        });
        println!("[review 6958] top ±{half}: faces {:#?}", readings.0);
        for r in &readings.1 {
            assert!(!r.contains("Side(In)"), "a wrong side: {r}");
            let short: String = r.chars().take(420).collect();
            println!("[review 6958] top ±{half}: {short}");
        }
    }
}

/// How far a ray aimed along `+z` drifts in `x` and `y` per unit it
/// drops: the aimed incidence and turn, as [`aimed_ray`] builds them.
fn drift_per_drop() -> (f64, f64) {
    let n = Vec3::new(0.0, 0.0, 1.0);
    let (b1, b2) = n.orthonormal_basis();
    let (turn_sin, turn_cos) = aim_turn();
    let tangent = b1 * turn_cos + b2 * turn_sin;
    let (along, across) = AIM_INCIDENCE;
    let out = n * along + tangent * across;
    (-out.x / out.z, -out.y / out.z)
}

/// The aim of [`a_ray_beside_a_moving_silhouette_edge_reads_no_wrong_side`]:
/// over the middle of the top edge the ray drifts toward, `margin`
/// further from that edge's wall than the ray drifts in dropping `hi`,
/// at height `hi`. Every member with its top at or below `hi` is entered
/// through its top cap, clear of that edge, and left through its bottom
/// cap, `margin` short of the wall.
fn aim_beside_the_edge_it_drifts_toward(hi: f64, margin: f64) -> [f64; 3] {
    let (dx, dy) = drift_per_drop();
    if dx.abs() >= dy.abs() {
        let off = dx.abs() * hi + margin;
        [if dx > 0.0 { 1.0 - off } else { off }, 0.5, hi]
    } else {
        let off = dy.abs() * hi + margin;
        [0.5, if dy > 0.0 { 1.0 - off } else { off }, hi]
    }
}

/// **A ray beside a silhouette edge that moves with the box reads no
/// wrong side.** The upright brick's top cap lies anywhere in `[lo, hi]`.
/// One fixed ray comes down at the aimed incidence beside the top edge
/// it drifts toward: for every member it enters the top cap clear of
/// that edge and leaves through the bottom cap, short of the wall under
/// the edge, so every member's ray reads `Out` (each of three members is
/// read at `f64` beside the box, as the witness of that). The edge's
/// direction is read off two ends that share the boxed height, so over a
/// wide box its gap from the ray encloses zero though no member's ray
/// comes near it.
///
/// RED at `e51e52c9`: over the top in `[0.05, 0.3]` the ray reads
/// `Side(In)` at margins 0.002, 0.004 and 0.01, at ε 1e-9, 1e-6 and
/// 1e-12. With the ray rebuilt from a boxed start (the construction
/// before `Ray::starting`) the same rays are set aside.
#[test]
#[ignore = "red at e51e52c9, the review's finding: cargo nextest run -p topo -E 'test(/a_ray_beside_a_moving_silhouette_edge/)' --run-ignored all"]
fn a_ray_beside_a_moving_silhouette_edge_reads_no_wrong_side() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness band");
    let up = [0.0, 0.0, 1.0];
    let mut wrong = Vec::new();
    for (lo, hi) in [
        (0.28, 0.30),
        (0.2, 0.3),
        (0.1, 0.3),
        (0.05, 0.3),
        (0.02, 0.26),
        (0.1, 0.5),
        (0.2, 1.0),
    ] {
        for margin in [0.002, 0.004, 0.01, 0.03] {
            let aim = aim_beside_the_edge_it_drifts_toward(hi, margin);
            let members: Vec<String> = [lo, 0.5 * (lo + hi), hi]
                .into_iter()
                .map(|h| {
                    let body = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, h), tol);
                    let face = body.faces().next().expect("a brick has faces").0;
                    let r = ray_at_report(
                        &body,
                        face,
                        Point3::new(aim[0], aim[1], aim[2]),
                        Vec3::new(up[0], up[1], up[2]),
                        band,
                    );
                    r.chars().take(40).collect()
                })
                .collect();
            let boxed = over((lo, hi), (1.0, 1.0), |body, band| {
                let face = body.faces().next().expect("a brick has faces").0;
                let iv = Boxed::from_f64;
                ray_at_report(
                    body,
                    face,
                    Point3::new(iv(aim[0]), iv(aim[1]), iv(aim[2])),
                    Vec3::new(iv(up[0]), iv(up[1]), iv(up[2])),
                    band,
                )
            });
            let short: String = boxed.chars().take(300).collect();
            println!(
                "[review 6958] ε {:e} top in [{lo}, {hi}], margin {margin}, aim {aim:?}: \
                 members {members:?}; the box: {short}",
                tol.eps()
            );
            if boxed.contains("Side(In)") {
                wrong.push(format!("top in [{lo}, {hi}], margin {margin}, aim {aim:?}"));
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "a ray every member reads Out reads In over the box: {wrong:#?}"
    );
}

/// **The unit's own inside-out brick has no one side** (a measurement
/// of `aimed_ray_tests`' fixture, the top cap moved below the bottom
/// with the stated normals kept): its caps' rays read `In` and its
/// walls' rays read `Out` or nothing, at a width where every ray reads.
#[test]
fn the_units_inside_out_brick_reads_in_by_its_caps_and_not_by_its_walls() {
    let readings = over((-0.3 - 0.03, -0.3 + 0.03), (1.0, 1.0), |body, band| {
        each_aim_report(body, band)
    });
    let (ins, outs) = (
        readings.iter().filter(|r| r.contains("Side(In)")).count(),
        readings.iter().filter(|r| r.contains("Side(Out)")).count(),
    );
    println!(
        "[review 6958] the unit's inside-out brick, top in [-0.33, -0.27]: {}",
        tally(&readings)
    );
    assert_eq!(
        (ins, outs),
        (2, 4),
        "two caps read In, four walls Out: {readings:#?}"
    );
}

/// **The faces' own aims of random boxed bricks read no wrong side**
/// (counterexample search over what [`cast_aims`] casts): bricks from a
/// twentieth to three across and deep, the top cap's height and the far
/// wall's station each over a box up to nine tenths of itself, upright
/// and inside out.
#[test]
#[ignore = "reviewer fuzz, ~1 min: cargo nextest run -p topo -E 'test(/review_6958/)' --run-ignored all"]
fn no_own_aim_of_a_random_boxed_brick_reads_the_side_no_member_has() {
    let mut rng = test_utils::fuzz::start("review_6958_random_bricks");
    let replay = test_utils::fuzz::replay();
    let eps = Tol::witness().eps();
    let bricks = test_utils::fuzz::scaled(250);
    let mut all = Vec::new();
    for k in 0..bricks {
        let inside_out = k % 4 == 3;
        let wrong = if inside_out { "Side(Out)" } else { "Side(In)" };
        let boxed = |rng: &mut test_utils::fuzz::Rng| {
            let middle = rng.log_uniform(0.05, 3.0);
            let half = match rng.below(4) {
                0 => 0.0,
                _ => rng.log_uniform(1e-3, 0.9) * middle,
            };
            (middle - half, middle + half)
        };
        let (top, right) = (boxed(&mut rng), boxed(&mut rng));
        let readings = over_oriented(top, right, inside_out, |body, band| {
            each_aim_report(body, band)
        });
        for r in &readings {
            assert!(
                !r.contains(wrong),
                "top {top:?}, right {right:?}, inside out {inside_out}, ε {eps:e}: a wrong side: \
                 {r} ({replay})"
            );
        }
        all.extend(readings);
    }
    println!(
        "[review 6958] ε {eps:e} random bricks' own aims: {}",
        tally(&all)
    );
}
