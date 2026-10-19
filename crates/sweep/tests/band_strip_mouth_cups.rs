//! **A cup whose mouth lies on a support's strip, wholly inside the
//! band's region, goes with it** (Ev's band ruling, PR 4963: a convex
//! band's result is `S − R`, a concave band's `S ∪ F`). A plane–plane
//! band cut off at two end walls takes a strip off each support, between
//! its edge and its trimline; a cup whose mouth is a ring on that strip
//! and whose faces lie inside the prism the band removes (convex) or
//! fills (concave) — a dimple under a convex band, a boss under a
//! concave one — goes with the material: its faces, edges and vertices
//! leave the body and its mouth leaves the support before the band's
//! carve takes the strip. The mouth is read against the strip three
//! ways (`surgery::strip_reading`), the screen standing down on a ring
//! that reading decides, and the cup's faces inside the band's core
//! less the support's side, on the material's side of the support
//! (`reach::prism_inside`).
//!
//! The CONVEX fixture is `band_one_cycle_members`' block `[0, k]³`, its
//! edge `(0, k, k) → (k, k, k)` blended at `D·k` between the supports
//! `z = k` (the top) and `y = k` (the side); a dimple of radius `a` at
//! `u` from the edge, square into a support to depth `h`, held
//! `π·a²·h` of air inside `R`, so `ΔV = −(A·k − π·a²·h)`, `A` the
//! section. The CONCAVE fixture is `band_concave_bar`'s die, its floor
//! crease between the pocket's floor `z = k/2` and its wall `y = k/4`,
//! its fill running `k/2` between end walls; a boss of radius `a` at `u`
//! from the crease, standing `h` off a support into `F`, held `π·a²·h`
//! of the fill already, so `ΔV = +(A·k/2 − π·a²·h)`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Bounds, Decide, Interval, Point2, Point3, Real, Vec3};
use sweep::blend::BlendError;
use sweep::test_support::{brick, prism_on, sketch_from_axes};
use topo::{AtRestPolicy, Body, EdgeKey, FaceKey};

use crate::band_concave_bar::{blend, boolean, die, floor};
use crate::band_one_cycle_members::{assert_taken, resolves_hairs};
use crate::band_planar_consume::{VERBS, section};
use crate::band_planar_cut_off::{D, Verb, edge, tol};

fn mid<T: Bounds>(x: T) -> f64 {
    0.5 * (x.lo() + x.hi())
}

/// A round post of radius `a` from `from` along the unit `dir` for
/// `len`, its section two semicircles.
pub(crate) fn post<T: Decide + AtRestPolicy>(
    from: Point3<f64>,
    dir: Vec3<f64>,
    len: f64,
    a: f64,
) -> Body<T> {
    shaft(from, dir, len, &[((-a, 0.0), 1.0), ((a, 0.0), 1.0)])
}

/// A square post of half-side `b`, as [`post`], its sides along the
/// section frame's axes.
fn square_post<T: Decide + AtRestPolicy>(
    from: Point3<f64>,
    dir: Vec3<f64>,
    len: f64,
    b: f64,
) -> Body<T> {
    let corners = [(-b, -b), (b, -b), (b, b), (-b, b)];
    shaft(from, dir, len, &corners.map(|c| (c, 0.0)))
}

/// A prism of the bulged section `verts` from `from` along the unit
/// `dir` for `len`, its section frame `x` turned square to `dir` and
/// `dir × x`.
fn shaft<T: Decide + AtRestPolicy>(
    from: Point3<f64>,
    dir: Vec3<f64>,
    len: f64,
    verts: &[((f64, f64), f64)],
) -> Body<T> {
    let seed = if dir.x.abs() < 0.9 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    let u = (seed - dir * seed.dot(dir)).normalize();
    let v = dir.cross(u);
    let f = T::from_f64;
    let lift = |w: Vec3<f64>| Vec3::new(f(w.x), f(w.y), f(w.z));
    prism_on(
        sketch_from_axes(
            Point3::new(f(from.x), f(from.y), f(from.z)),
            lift(u),
            lift(v),
            tol(),
        ),
        verts
            .iter()
            .map(|&((x, y), bulge)| (Point2::new(f(x), f(y)), f(bulge)))
            .collect(),
        f(len),
        tol(),
    )
}

/// The faces of `body` whose every vertex lies within `reach` of the
/// line through `o` along the unit `dir`, at `[t0, t1]` along it: a
/// post's faces.
pub(crate) fn post_faces<T: Real + Bounds>(
    body: &Body<T>,
    (o, dir): (Point3<f64>, Vec3<f64>),
    reach: f64,
    (t0, t1): (f64, f64),
) -> Vec<FaceKey> {
    let at = |v: topo::VertexKey| {
        let p = *body
            .get_point(body.get_vertex(v).expect("v").point)
            .expect("p");
        Point3::new(mid(p.x), mid(p.y), mid(p.z))
    };
    let slack = 1e-9 * (1.0 + reach + t1.abs());
    let near = |p: Point3<f64>| {
        let t = (p - o).dot(dir);
        let off = (p - o) - dir * t;
        off.norm() <= reach + slack && t >= t0 - slack && t <= t1 + slack
    };
    body.faces()
        .filter(|(_, f)| {
            core::iter::once(f.outer)
                .chain(f.rings.iter().copied())
                .all(|lp| match body.get_loop(lp).expect("a loop").boundary {
                    topo::LoopBoundary::Empty { vertex } => near(at(vertex)),
                    topo::LoopBoundary::Cycle { first } => body
                        .loop_cycle(first)
                        .expect("a cycle")
                        .into_iter()
                        .all(|he| near(at(body.get_half_edge(he).expect("he").start))),
                })
        })
        .map(|(k, _)| k)
        .collect()
}

/// Which support of the band a cup stands on.
#[derive(Clone, Copy, Debug)]
pub(crate) enum On {
    /// The convex block's top `z = k`, or the die's floor `z = k/2`.
    Floor,
    /// The convex block's side `y = k`, or the die's wall `y = k/4`.
    Wall,
}

impl On {
    const BOTH: [Self; 2] = [Self::Floor, Self::Wall];
}

/// A cup's mouth centre on its support and its unit axis into the cup,
/// at scale `k`, `u` from the band's edge and `x` along it: into the
/// block for the dimple (`convex`), out of the die for the boss.
fn mouth(convex: bool, on: On, k: f64, (u, x): (f64, f64)) -> (Point3<f64>, Vec3<f64>) {
    match (convex, on) {
        (true, On::Floor) => (Point3::new(x, k - u, k), Vec3::new(0.0, 0.0, -1.0)),
        (true, On::Wall) => (Point3::new(x, k, k - u), Vec3::new(0.0, -1.0, 0.0)),
        (false, On::Floor) => (
            Point3::new(x, 0.25 * k + u, 0.5 * k),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        (false, On::Wall) => (
            Point3::new(x, 0.25 * k, 0.5 * k + u),
            Vec3::new(0.0, 1.0, 0.0),
        ),
    }
}

/// The block at scale `k` less a dimple of radius `a` and depth `h` on
/// `on`, `u` from the edge at `x` along it.
pub(crate) fn dimple<T: Decide + Bounds + AtRestPolicy>(
    k: f64,
    on: On,
    c: (f64, f64),
    a: f64,
    h: f64,
) -> Body<T> {
    let (o, dir) = mouth(true, on, k, c);
    let block = brick((0.0, k), (0.0, k), (0.0, k), tol());
    boolean(false, block, post(o - dir * (0.2 * k), dir, 0.2 * k + h, a))
}

/// The die at scale `k` with a boss of radius `a` standing `h` off
/// `on`, `u` from the crease at `x` along it.
pub(crate) fn boss<T: Decide + Bounds + AtRestPolicy>(
    k: f64,
    on: On,
    c: (f64, f64),
    a: f64,
    h: f64,
) -> Body<T> {
    let (o, dir) = mouth(false, on, k, c);
    boolean(
        true,
        die(k, 0.0),
        post(o - dir * (0.1 * k), dir, 0.1 * k + h, a),
    )
}

/// The convex block's edge.
fn top<T: Real + Bounds>(body: &Body<T>, k: f64) -> EdgeKey {
    edge(body, [0.0, k, k], [k, k, k])
}

/// A cup's offset from the edge, its place along it, its radius and its
/// depth, at scale 1: its far side `0.036` from the edge, where the
/// fillet's arc stands `0.0232` off the support.
const CUP: (f64, f64, f64, f64) = (0.03, 0.5, 0.006, 0.012);

/// **A dimple under a convex band and a boss under a concave one go, on
/// either support, at every scale under both verbs**:
/// `ΔV = −(A·k − π·a²·h)` and `ΔV = +(A·k/2 − π·a²·h)`.
#[test]
fn a_dimple_and_a_boss_go_with_the_band_on_either_support_at_every_scale() {
    let (u, x, a, h) = CUP;
    for k in [1e-2, 1.0, 1e2] {
        for on in On::BOTH {
            let what = format!("a dimple on the {on:?} at scale {k}");
            let body = dimple::<f64>(k, on, (u * k, x * k), a * k, h * k);
            let cups = post_faces(
                &body,
                mouth(true, on, k, (u * k, x * k)),
                a * k,
                (0.0, h * k),
            );
            assert_eq!(cups.len(), 3, "{what}: two half-walls and a floor");
            for verb in VERBS {
                let dv = -(section(verb, D * k) * k - PI * (a * k).powi(2) * h * k);
                assert_taken(
                    verb,
                    &body,
                    &[top(&body, k)],
                    D * k,
                    (&cups, 1, 0),
                    (dv, k),
                    &what,
                );
            }
            let what = format!("a boss on the {on:?} at scale {k}");
            let body = boss::<f64>(k, on, (u * k, x * k), a * k, h * k);
            let cups = post_faces(
                &body,
                mouth(false, on, k, (u * k, x * k)),
                a * k,
                (0.0, h * k),
            );
            assert_eq!(cups.len(), 3, "{what}: two half-walls and an end");
            for verb in VERBS {
                let dv = section(verb, D * k) * 0.5 * k - PI * (a * k).powi(2) * h * k;
                assert_taken(
                    verb,
                    &body,
                    &[floor(&body, k)],
                    D * k,
                    (&cups, 1, 0),
                    (dv, k),
                    &what,
                );
            }
        }
    }
}

/// **The same dimple and boss at the certified scalar** go too, each
/// volume enclosure bracketing its closed form.
#[test]
fn a_dimple_and_a_boss_go_with_the_band_at_interval() {
    let (u, x, a, h) = CUP;
    for on in On::BOTH {
        let body = dimple::<Interval>(1.0, on, (u, x), a, h);
        let cups = post_faces(&body, mouth(true, on, 1.0, (u, x)), a, (0.0, h));
        assert_eq!(cups.len(), 3, "Interval: the dimple's three faces");
        for verb in VERBS {
            let dv = -(section(verb, D) - PI * a * a * h);
            let what = format!("a dimple on the {on:?} at Interval");
            assert_taken(
                verb,
                &body,
                &[top(&body, 1.0)],
                D,
                (&cups, 1, 0),
                (dv, 1.0),
                &what,
            );
        }
        let body = boss::<Interval>(1.0, on, (u, x), a, h);
        let cups = post_faces(&body, mouth(false, on, 1.0, (u, x)), a, (0.0, h));
        assert_eq!(cups.len(), 3, "Interval: the boss's three faces");
        for verb in VERBS {
            let dv = section(verb, D) * 0.5 - PI * a * a * h;
            let what = format!("a boss on the {on:?} at Interval");
            assert_taken(
                verb,
                &body,
                &[floor(&body, 1.0)],
                D,
                (&cups, 1, 0),
                (dv, 1.0),
                &what,
            );
        }
    }
}

/// Which side of the fixture a row reads.
#[derive(Clone, Copy, Debug)]
enum Side {
    /// The convex block's dimple.
    Dimple,
    /// The concave die's boss.
    Boss,
}

impl Side {
    const BOTH: [Self; 2] = [Self::Dimple, Self::Boss];

    /// The fixture at scale 1 with this side's cup of radius `a` and
    /// depth `h` on the floor, at `c`; its requested edge; the cup's
    /// faces; and `ΔV` under `verb`.
    fn cup(
        self,
        c: (f64, f64),
        a: f64,
        h: f64,
        verb: Verb,
    ) -> (Body<f64>, EdgeKey, Vec<FaceKey>, f64) {
        let convex = matches!(self, Self::Dimple);
        let body = if convex {
            dimple(1.0, On::Floor, c, a, h)
        } else {
            boss(1.0, On::Floor, c, a, h)
        };
        let e = if convex {
            top(&body, 1.0)
        } else {
            floor(&body, 1.0)
        };
        let cups = post_faces(&body, mouth(convex, On::Floor, 1.0, c), a, (0.0, h));
        let dv = if convex {
            -(section(verb, D) - PI * a * a * h)
        } else {
            section(verb, D) * 0.5 - PI * a * a * h
        };
        (body, e, cups, dv)
    }
}

/// The refusal `verb` gives `body` on `edges`.
fn refusal(verb: Verb, body: &Body<f64>, edges: &[EdgeKey], what: &str) -> BlendError {
    blend(verb, body, edges, D)
        .err()
        .unwrap_or_else(|| panic!("{what} ({verb:?}): refuses"))
}

/// **A cup on each support of one band, a dimple beside a cap's blind
/// bore, and a boss under a concave band beside a convex band elsewhere
/// in the request, all go**: two dimples, `ΔV = −(A − 2π·a²·h)`; a
/// dimple and a bore `0.4` deep from the end face `x = 0`,
/// `ΔV = −(A − π·a²·(0.4 + h))`; the die's boss with its block's front
/// top edge `(−1, 0, 1) → (2, 0, 1)` blended too, convex, its band
/// cut off by the block's ends `3` apart, `ΔV = A/2 − π·a²·h − 3A`.
#[test]
fn cups_on_both_supports_beside_a_cap_cup_and_beside_a_convex_band_go() {
    let (u, _, a, h) = CUP;
    let (o, dir) = mouth(true, On::Wall, 1.0, (u, 0.7));
    let two = boolean(
        false,
        dimple::<f64>(1.0, On::Floor, (u, 0.3), a, h),
        post(o - dir * 0.2, dir, 0.2 + h, a),
    );
    let mut cups = post_faces(&two, mouth(true, On::Floor, 1.0, (u, 0.3)), a, (0.0, h));
    cups.extend(post_faces(&two, (o, dir), a, (0.0, h)));
    assert_eq!(cups.len(), 6, "two dimples of three faces each");
    for verb in VERBS {
        let dv = -(section(verb, D) - 2.0 * PI * a * a * h);
        let edges = [top(&two, 1.0)];
        assert_taken(
            verb,
            &two,
            &edges,
            D,
            (&cups, 2, 0),
            (dv, 1.0),
            "a dimple on each support",
        );
    }
    let bore = (Point3::new(-0.2, 0.988, 0.988), Vec3::new(1.0, 0.0, 0.0));
    let body = boolean(
        false,
        dimple::<f64>(1.0, On::Floor, (u, 0.7), a, h),
        post(bore.0, bore.1, 0.6, a),
    );
    let mut cups = post_faces(&body, mouth(true, On::Floor, 1.0, (u, 0.7)), a, (0.0, h));
    cups.extend(post_faces(&body, bore, a, (0.2, 0.6)));
    assert_eq!(cups.len(), 6, "a dimple and a bore of three faces each");
    for verb in VERBS {
        let dv = -(section(verb, D) - PI * a * a * (0.4 + h));
        let edges = [top(&body, 1.0)];
        assert_taken(
            verb,
            &body,
            &edges,
            D,
            (&cups, 2, 0),
            (dv, 1.0),
            "a dimple and a bore",
        );
    }
    let body = boss::<f64>(1.0, On::Floor, (u, 0.5), a, h);
    let cups = post_faces(&body, mouth(false, On::Floor, 1.0, (u, 0.5)), a, (0.0, h));
    let edges = [
        floor(&body, 1.0),
        edge(&body, [-1.0, 0.0, 1.0], [2.0, 0.0, 1.0]),
    ];
    for verb in VERBS {
        let dv = section(verb, D) * 0.5 - PI * a * a * h - 3.0 * section(verb, D);
        assert_taken(
            verb,
            &body,
            &edges,
            D,
            (&cups, 1, 0),
            (dv, 1.0),
            "a boss beside a convex band",
        );
    }
}

/// **A mouth a hair across the trimline crosses, and one a hair inside
/// goes where the band takes it**, on both sides: `1e-4` past the
/// trimline, and centred on it, the ring refuses `RingClearance`
/// measured under both verbs. Touching it from inside, the strip reads
/// the ring neither way, so the screen meters it
/// (`FaceClearanceUncertified`). `1e-4` inside it a cup `2·10⁻⁵` deep
/// goes under the chamfer, whose chord stands `1e-4` off the support
/// there, at `ΔV = ∓(A·L − π·a²·h)`; under the fillet, whose arc is
/// tangent to the support at the trimline and stands `5·10⁻⁸` off it
/// there, its floor crosses the band surface, measured.
#[test]
fn a_cups_mouth_a_hair_across_the_trimline_crosses_and_one_inside_goes() {
    let (_, x, a, _) = CUP;
    for side in Side::BOTH {
        for verb in VERBS {
            let what = format!("{side:?} across the trimline");
            for c in [(D - a + 1e-4, x), (D, x)] {
                let (body, e, _, _) = side.cup(c, a, 1e-5, verb);
                let err = refusal(verb, &body, &[e], &what);
                assert!(
                    matches!(err, BlendError::RingClearance { bounded: false, .. }),
                    "{what} ({verb:?}): the mouth crosses the trimline, measured, got {err:?}"
                );
            }
            let what = format!("{side:?} touching the trimline");
            let (body, e, _, _) = side.cup((D - a, x), a, 1e-5, verb);
            let err = refusal(verb, &body, &[e], &what);
            assert!(
                matches!(err, BlendError::FaceClearanceUncertified { .. }),
                "{what} ({verb:?}): the strip reads the touching mouth neither way, so the \
                 screen meters it, got {err:?}"
            );
            let what = format!("{side:?} inside the trimline");
            let (body, e, cups, dv) = side.cup((D - a - 1e-4, x), a, 2e-5, verb);
            match verb {
                Verb::Chamfer => {
                    assert_taken(verb, &body, &[e], D, (&cups, 1, 0), (dv, 1.0), &what)
                }
                Verb::Fillet => {
                    let err = refusal(verb, &body, &[e], &what);
                    assert!(
                        matches!(err, BlendError::FaceClearance { bounded: false, .. }),
                        "{what}: the floor crosses the arc, measured, got {err:?}"
                    );
                }
            }
        }
    }
}

/// **A floor a hair across the band surface crosses, and one a hair
/// inside goes**, on both sides under both verbs: the cup's far side
/// `0.036` from the edge, its floor `1e-4` past the arc (the chord), and
/// `1e-4` inside it, at `ΔV = ∓(A·L − π·a²·h)`.
#[test]
fn a_cups_floor_a_hair_across_the_band_surface_crosses_and_one_inside_goes() {
    let (u, x, a, _) = CUP;
    let s = u + a;
    for side in Side::BOTH {
        for verb in VERBS {
            let stands = match verb {
                Verb::Fillet => D - (D * D - (D - s) * (D - s)).sqrt(),
                Verb::Chamfer => D - s,
            };
            let what = format!("{side:?}'s floor past the band surface");
            let (body, e, _, _) = side.cup((u, x), a, stands + 1e-4, verb);
            let err = refusal(verb, &body, &[e], &what);
            assert!(
                matches!(err, BlendError::FaceClearance { bounded: false, .. }),
                "{what} ({verb:?}): crosses, measured, got {err:?}"
            );
            let what = format!("{side:?}'s floor inside the band surface");
            let (body, e, cups, dv) = side.cup((u, x), a, stands - 1e-4, verb);
            assert_taken(verb, &body, &[e], D, (&cups, 1, 0), (dv, 1.0), &what);
        }
    }
}

/// **A mouth a hair from the edge, or from an end wall, goes; one a
/// hair across an end wall is no ring and refuses at the screen**: a
/// mouth `1e-4` from the edge's line, or from the end face `x = 0`, lies
/// inside the strip and goes; `1e-4` across the end face it notches the
/// support's outer cycle, whose boundary the clearance screen reads
/// against the band's setback (`FaceClearanceUncertified`).
#[test]
fn a_cups_mouth_a_hair_from_the_edge_and_an_end_wall_goes_and_one_across_the_end_refuses() {
    let (u, x, a, h) = CUP;
    for side in Side::BOTH {
        for verb in VERBS {
            let ends = match side {
                Side::Dimple => (0.0, 1.0),
                Side::Boss => (0.25, 0.75),
            };
            for (what, c) in [
                ("a mouth a hair from the edge", (a + 1e-4, x)),
                ("a mouth a hair from the end wall", (u, ends.0 + a + 1e-4)),
                (
                    "a mouth a hair from the far end wall",
                    (u, ends.1 - a - 1e-4),
                ),
            ] {
                let what = format!("{side:?}: {what}");
                let (body, e, cups, dv) = side.cup(c, a, h, verb);
                assert_taken(verb, &body, &[e], D, (&cups, 1, 0), (dv, 1.0), &what);
            }
            let what = format!("{side:?}: a mouth a hair across the end wall");
            let (body, e, _, _) = side.cup((u, ends.0 + a - 1e-4), a, h, verb);
            let err = refusal(verb, &body, &[e], &what);
            assert!(
                matches!(err, BlendError::FaceClearanceUncertified { .. }),
                "{what} ({verb:?}): a notch, screened, got {err:?}"
            );
        }
    }
}

/// **A cup of the band's opposite sign on its strip refuses as a
/// bound**: a boss on the convex block's strip stands in the air the
/// band leaves, and a dimple in the die's floor runs into the material
/// under the fill, outside `F` (`S ∪ F` seals its mouth: step 4's
/// seal), so neither lies inside the region and each face standing off
/// the support reads neither inside nor across it
/// (`FaceClearance { bounded: true }`), under both verbs.
#[test]
fn a_cup_of_the_bands_opposite_sign_on_its_strip_refuses_as_a_bound() {
    let (u, x, a, h) = CUP;
    let (o, dir) = mouth(true, On::Floor, 1.0, (u, x));
    let block = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol());
    let convex = boolean(true, block, post(o + dir * 0.1, -dir, 0.1 + h, a));
    let (o, dir) = mouth(false, On::Floor, 1.0, (u, x));
    let concave = boolean(false, die(1.0, 0.0), post(o + dir * 0.1, -dir, 0.1 + h, a));
    for (what, body, e) in [
        ("a boss on a convex strip", &convex, top(&convex, 1.0)),
        (
            "a dimple in a concave strip",
            &concave,
            floor(&concave, 1.0),
        ),
    ] {
        for verb in VERBS {
            let err = refusal(verb, body, &[e], what);
            assert!(
                matches!(err, BlendError::FaceClearance { bounded: true, .. }),
                "{what} ({verb:?}): a bound, got {err:?}"
            );
        }
    }
}

/// **A square dimple and a round one leaning off their support's normal
/// go**, under both verbs. Each axis leans `0.05` rad along the edge and
/// across it: the square dimple's seams run along the axis, between a
/// mouth vertex and a floor vertex, and stray from the line along the
/// support's normal by `0.05` per unit of length, which the reading
/// charges twice; along the spine they would stray by nearly their
/// length. The round dimple's mouth is an ellipse, read across the edge
/// in closed form. A bar holds its section per unit of its axis whatever
/// its ends' planes, so `ΔV = −(A − 4b²·h)` and `−(A − π·a²·h)`, `h` its
/// axis from the support to the floor. Past `ε = 1e-9` the round
/// fixture's Boolean does not resolve the leaning cylinder against the
/// top, and that half skips loudly.
#[test]
fn a_square_and_a_round_dimple_leaning_off_their_supports_normal_go() {
    let (u, x, a, h) = CUP;
    let b = 0.004;
    let (o, _) = mouth(true, On::Floor, 1.0, (u, x));
    for (what, lean) in [
        ("along the edge", (0.05f64, 0.0)),
        ("across it", (0.0, 0.05f64)),
    ] {
        let dir = Vec3::new(lean.0.sin(), lean.1.sin(), -1.0).normalize();
        let block = || -> Body<f64> { brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()) };
        let body = boolean(false, block(), square_post(o - dir * 0.2, dir, 0.2 + h, b));
        let cups = post_faces(&body, (o, dir), b * 2f64.sqrt(), (-b, h));
        assert_eq!(
            cups.len(),
            5,
            "a square dimple leaning {what}: four walls and a floor"
        );
        for verb in VERBS {
            let dv = -(section(verb, D) - 4.0 * b * b * h);
            let what = format!("a square dimple leaning {what}");
            assert_taken(
                verb,
                &body,
                &[top(&body, 1.0)],
                D,
                (&cups, 1, 0),
                (dv, 1.0),
                &what,
            );
        }
        if !resolves_hairs(&format!("a round dimple leaning {what}")) {
            continue;
        }
        let body = boolean(false, block(), post(o - dir * 0.2, dir, 0.2 + h, a));
        let cups = post_faces(&body, (o, dir), a, (-a, h));
        assert_eq!(
            cups.len(),
            3,
            "a round dimple leaning {what}: two half-walls and a floor"
        );
        for verb in VERBS {
            let dv = -(section(verb, D) - PI * a * a * h);
            let what = format!("a round dimple leaning {what}");
            assert_taken(
                verb,
                &body,
                &[top(&body, 1.0)],
                D,
                (&cups, 1, 0),
                (dv, 1.0),
                &what,
            );
        }
    }
}
