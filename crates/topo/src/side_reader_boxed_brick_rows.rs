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
/// the edge, so every member's ray reads `Out` (three members are read
/// at `f64` beside the box, as the witness of that). Over a wide box the
/// ray's gap from that edge is an enclosure from zero up though no
/// member's ray comes near it, and the ray is set aside on it
/// (`side_at_infinity_edge`). Red against a walk that reads such a gap
/// as the edge met in band: the edge then stands for the top cap's
/// entry, counts nothing at the silhouette, and the ray reads `In` off
/// the bottom cap's exit (top in `[0.05, 0.3]`, margins 0.002 to 0.01).
#[test]
fn a_ray_beside_a_moving_silhouette_edge_reads_no_wrong_side() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness band");
    let up = [0.0, 0.0, 1.0];
    for (lo, hi) in [(0.2, 0.3), (0.1, 0.3), (0.05, 0.3), (0.02, 0.26)] {
        for margin in [0.002, 0.004, 0.01, 0.03] {
            let aim = aim_beside_the_edge_it_drifts_toward(hi, margin);
            let what = format!("top in [{lo}, {hi}], margin {margin}, aim {aim:?}");
            for h in [lo, 0.5 * (lo + hi), hi] {
                let body = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, h), tol);
                let face = body.faces().next().expect("a brick has faces").0;
                let member = ray_at_report(
                    &body,
                    face,
                    Point3::new(aim[0], aim[1], aim[2]),
                    Vec3::new(up[0], up[1], up[2]),
                    band,
                );
                assert_eq!(member, "Ok(Side(Out))", "{what}: the member at {h}");
            }
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
            assert!(
                !boxed.contains("Side(In)"),
                "{what}: a ray every member reads Out reads In over the box"
            );
            if (lo, hi) == (0.05, 0.3) && margin < 0.02 {
                assert!(
                    boxed.starts_with("Ok(SetAside(Crossing(")
                        && boxed.contains("side_at_infinity_edge"),
                    "{what}: set aside on its gap from the moving edge: {boxed}"
                );
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
    let own = over(top, (1.0, 1.0), |body, band| each_aim_report(body, band));
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
