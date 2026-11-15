//! **An aimed ray over a box reads the side every member has, inside
//! out or not, or reads nothing** (`side_reader::aimed_ray`).
//!
//! A brick at the symbolic box scalar with its top cap's height and its
//! far `x` wall's station each a box, every plane's normal the unit
//! brick's own, exact, or every one of them negated. With the brick's
//! own normals every member's side at infinity is `Out`; with every
//! normal negated every member is inside out and the side is `In`. An
//! aimed ray starts beyond every face whatever it aims at, so from any
//! aim and any normal a reading of the other side is wrong.
//!
//! No door builds a brick whose side differs across its box: such a box
//! holds a member with a cap on its twin, and the edge between the caps
//! refuses its own span there (`interval_span_forward`).

#![allow(clippy::expect_used, clippy::panic)]

use super::*;
use crate::construction::FaceSurface;
use crate::test_support_fixtures::{FaceGeometry, brick, prism_ops};
use geom::Surface;
use geom_core::{Interval, ParamSymbol, Real as _, Sym, SymBudget, Tol};

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
            axis("boxed_brick_top", top),
            axis("boxed_brick_right", right),
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
/// boxed stations**, from a thousandth of the brick's height to three
/// tenths of it: no aim reads the side no member has.
///
/// **What this can go red on:** a reading of the side no member has, and
/// nothing else. A reading lost is not a wrong one, so a change that
/// only sets more rays aside leaves it green; and it stayed green with
/// an undecided edge gap read as an edge met in `edge_meets`, alone and
/// with the edge read off its two vertices, where
/// [`a_ray_beside_a_moving_silhouette_edge_is_set_aside_at_it`] is red
/// and eighteen of its rays read the wrong side. Its draws do not put a
/// ray beside a moving silhouette edge closely enough, so it is evidence
/// that these draws find no wrong side, not that the edge reading is
/// sound.
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
        for half in [1e-3, 0.025, 0.1, 0.3].map(|h| h * scale) {
            for right in [(1.0, 1.0), (1.0 - half.min(0.45), 1.0 + half.min(0.45))] {
                let top = (middle - half, middle + half);
                let readings = over_oriented(top, right, inside_out, |body, band| {
                    each_aim_report(body, band)
                });
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
///
/// **What this can go red on:** a reading of the side no member has, and
/// nothing else. A reading lost is not a wrong one, so a change that
/// only sets more rays aside leaves it green; and it stayed green with
/// an undecided edge gap read as an edge met in `edge_meets`, alone and
/// with the edge read off its two vertices, where
/// [`a_ray_beside_a_moving_silhouette_edge_is_set_aside_at_it`] is red
/// and eighteen of its rays read the wrong side. Its draws do not put a
/// ray beside a moving silhouette edge closely enough, so it is evidence
/// that these draws find no wrong side, not that the edge reading is
/// sound.
#[test]
fn no_ray_about_a_boxed_brick_reads_the_side_no_member_has() {
    let mut rng = test_utils::fuzz::start("boxed_brick_rays");
    let replay = test_utils::fuzz::replay();
    let eps = Tol::witness().eps();
    let boxes = test_utils::fuzz::scaled(2);
    let rays = test_utils::fuzz::scaled(30);
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
        println!("ε {eps:e} {what} rays: {} ({replay})", tally(&all));
    }
}

/// A ray at the aimed incidence about the outward axis `a` (`0` the far
/// `x` wall, `2` the top cap), entering that moving face at station `hi`
/// beside the edge it drifts toward along axis `c`, `margin` short of
/// the wall under that edge when it has dropped `hi`: `(aim, normal)`.
fn beside(a: usize, c: usize, hi: f64, margin: f64) -> ([f64; 3], [f64; 3]) {
    let mut n = [0.0; 3];
    n[a] = 1.0;
    let nv = Vec3::new(n[0], n[1], n[2]);
    let (b1, b2) = nv.orthonormal_basis();
    let (turn_sin, turn_cos) = aim_turn();
    let tangent = b1 * turn_cos + b2 * turn_sin;
    let (along, across) = AIM_INCIDENCE;
    let out = nv * along + tangent * across;
    let o = [out.x, out.y, out.z];
    let drift = -o[c] / o[a];
    let off = drift.abs() * hi + margin;
    let mut aim = [0.5; 3];
    aim[a] = hi;
    aim[c] = if drift > 0.0 { 1.0 - off } else { off };
    (aim, n)
}

/// **A ray beside a silhouette edge that moves with the box is set
/// aside at that edge, and never read for the other side.** The brick's
/// moving face (its top cap, or its far `x` wall) lies anywhere in
/// `[0.05, 0.3]`. One fixed ray comes in at the aimed incidence beside
/// one of the two edges of that face it drifts toward, 0.002 to 0.01
/// short of the wall under the edge: for every member it enters the
/// moving face clear of that edge and leaves through the face opposite,
/// so every member reads one side (three members are read at `f64`
/// beside the box, as the witness). Over the box the ray's gap from the
/// edge is an enclosure from zero up, though no member's ray comes near
/// it. Twenty-four rays: two moving faces, two edges each, three
/// margins, the brick upright and with every normal negated.
///
/// Each ray is asserted set aside on the edge's own row
/// (`side_at_infinity_edge`), and to read no wrong side.
///
/// **Red, each of the twenty-four, under
/// `Err(undecided) => Ok(Gap::InBand(undecided))` in
/// `solid_contain::edge_meets`** (an undecided gap read as the edge met
/// in band): the edge is then met and stands for its faces' crossings,
/// and the ray is set aside further on, on another row; none reads the
/// wrong side under that change alone. With that and the edge read off
/// its two vertices (`census::line_passes` without `on_its_line`), the
/// tree this was found on, eighteen read the wrong side (`In` upright,
/// `Out` inside out) and the six beside the wall's edge along `z` graze.
#[test]
fn a_ray_beside_a_moving_silhouette_edge_is_set_aside_at_it() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness band");
    let (lo, hi) = (0.05, 0.3);
    for (a, edges) in [(2_usize, [0_usize, 1]), (0, [1, 2])] {
        for c in edges {
            for margin in [0.002, 0.004, 0.01] {
                let (aim, n) = beside(a, c, hi, margin);
                let what = format!("moving face {a}, edge along {c}, margin {margin}");
                for h in [lo, 0.5 * (lo + hi), hi] {
                    let (x, z) = if a == 2 { (1.0, h) } else { (h, 1.0) };
                    let body = brick::<f64>((0.0, x), (0.0, 1.0), (0.0, z), tol);
                    let face = body.faces().next().expect("a brick has faces").0;
                    let member = ray_at_report(
                        &body,
                        face,
                        Point3::new(aim[0], aim[1], aim[2]),
                        Vec3::new(n[0], n[1], n[2]),
                        band,
                    );
                    assert_eq!(member, "Ok(Side(Out))", "{what}: the member at {h}");
                }
                let (top, right) = if a == 2 {
                    ((lo, hi), (1.0, 1.0))
                } else {
                    ((1.0, 1.0), (lo, hi))
                };
                for (inside_out, wrong) in [(false, "Side(In)"), (true, "Side(Out)")] {
                    let boxed = over_oriented(top, right, inside_out, |body, band| {
                        let face = body.faces().next().expect("a brick has faces").0;
                        let iv = Boxed::from_f64;
                        ray_at_report(
                            body,
                            face,
                            Point3::new(iv(aim[0]), iv(aim[1]), iv(aim[2])),
                            Vec3::new(iv(n[0]), iv(n[1]), iv(n[2])),
                            band,
                        )
                    });
                    assert!(
                        !boxed.contains(wrong),
                        "{what}, inside out {inside_out}: the side no member has: {boxed}"
                    );
                    assert!(
                        boxed.starts_with("Ok(SetAside(Crossing(")
                            && boxed.contains("side_at_infinity_edge"),
                        "{what}, inside out {inside_out}: set aside at the moving edge: {boxed}"
                    );
                }
            }
        }
    }
}

/// **A brick over a box a twentieth of its height wide reads its side
/// by every face's own ray, and an inside-out one reads `In` by a ray
/// through it.** Upright, each of the six aims reads `Out`. An
/// inside-out brick's own aims step out of their faces and read nothing,
/// so it is read by two rays given through it: one aimed at a point
/// inside it, and one aimed at a point below it, which crosses both caps
/// behind its aim. Red against a walk that reads only what lies ahead of
/// the ray's origin (the second ray then reads nothing), and against a
/// reading that takes the side from anything but the count (the upright
/// brick's same two rays read `Out`).
#[test]
fn a_boxed_brick_reads_the_side_its_members_have() {
    let top = (0.975, 1.025);
    let own = over(top, (1.0, 1.0), each_aim_report);
    assert_eq!(own.len(), 6, "a brick offers six aims");
    for read in &own {
        assert!(
            read.ends_with(": Ok(Side(Out))"),
            "upright, its own aim: {read}"
        );
    }
    for (inside_out, side) in [(false, "Ok(Side(Out))"), (true, "Ok(Side(In))")] {
        for (what, aim) in [
            ("aimed inside it", [0.37, 0.41, 0.5]),
            ("aimed below it", [0.37, 0.41, -0.5]),
        ] {
            let read = over_oriented(top, (1.0, 1.0), inside_out, |body, band| {
                let face = body.faces().next().expect("a brick has faces").0;
                let iv = Boxed::from_f64;
                ray_at_report(
                    body,
                    face,
                    Point3::new(iv(aim[0]), iv(aim[1]), iv(aim[2])),
                    Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
                    band,
                )
            });
            assert_eq!(read, side, "inside out {inside_out}, a ray {what}");
        }
    }
}

/// **The faces' own aims of random boxed bricks read no wrong side**
/// (counterexample search over what [`cast_aims`] casts): bricks from a
/// twentieth to three across and deep, the top cap's height and the far
/// wall's station each over a box up to nine tenths of itself, upright
/// and inside out.
///
/// **What this can go red on:** a reading of the side no member has, and
/// nothing else. A reading lost is not a wrong one, so a change that
/// only sets more rays aside leaves it green; and it stayed green with
/// an undecided edge gap read as an edge met in `edge_meets`, alone and
/// with the edge read off its two vertices, where
/// [`a_ray_beside_a_moving_silhouette_edge_is_set_aside_at_it`] is red
/// and eighteen of its rays read the wrong side. Its draws do not put a
/// ray beside a moving silhouette edge closely enough, so it is evidence
/// that these draws find no wrong side, not that the edge reading is
/// sound.
#[test]
fn no_own_aim_of_a_random_boxed_brick_reads_the_side_no_member_has() {
    let mut rng = test_utils::fuzz::start("random_boxed_bricks");
    let replay = test_utils::fuzz::replay();
    let eps = Tol::witness().eps();
    let bricks = test_utils::fuzz::scaled(24);
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
        "ε {eps:e} random bricks' own aims: {} ({replay})",
        tally(&all)
    );
}
