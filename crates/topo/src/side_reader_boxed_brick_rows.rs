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
        let Surface::Plane {
            frame: carrier_frame,
            ..
        } = *twin.proven_face(face).surface()
        else {
            panic!("a brick's face is planar");
        };
        let (normal, u_ref) = (carrier_frame.axis(), carrier_frame.u_ref());
        let corner = body.proven_face(face).edges()[0].ends[0];
        let origin = *body
            .get_vertex(corner)
            .and_then(|v| body.get_point(v.point))
            .expect("the boxed brick shares the twin's keys");
        body.set_face_surface(
            face,
            FaceSurface::New {
                surface: Surface::Plane {
                    frame: geom::Frame::new(
                        origin,
                        normal.map(|x| iv(if inside_out { -x } else { x })),
                        u_ref.map(iv),
                    ),
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
/// `.or(Ok(Magnitude::Zero))` on the gap's decision in
/// `solid_contain::edge_meets`** (an undecided gap handed to the door
/// as decided, so read as the edge met): the edge is then met and stands for its faces' crossings,
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

/// [`brick_at`] upright over `[0, 1] × [0, 1] × [0, top]`, every edge's
/// line stated as a sweep door states it: from the edge's own start
/// along the unit brick's exact direction, as long as the edge is
/// (`top` for an upright edge). [`brick_at`]'s own lines are minted from
/// their two ends (`EdgeCurveSpec::line_between`), so a moving edge's
/// direction there is as wide as its chord. This restates how a door
/// states a line; nothing keeps it in step with the doors, and the
/// door-built rows are `sweep`'s `an_aimed_ray_over_boxed_door_bodies`.
fn lined_brick(top: Boxed, tol: Tol) -> Body<Boxed> {
    let mut body = brick_at(top, Boxed::from_f64(1.0), false, tol);
    let twin = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    let mut stated = std::collections::BTreeSet::new();
    for (face, _) in twin.faces() {
        for e in twin.proven_face(face).edges() {
            let plus = twin
                .edges
                .get(e.edge)
                .and_then(|edge| edge.claim(e.half))
                .expect("the twin's half claims its edge")
                .plus;
            if !plus || !stated.insert(e.edge) {
                continue;
            }
            let at = |b: &Body<f64>, v| {
                *b.get_point(b.get_vertex(v).expect("a vertex").point)
                    .expect("a point")
            };
            let dir = at(&twin, e.ends[1]) - at(&twin, e.ends[0]);
            let ends = e.ends.map(|v| {
                *body
                    .get_point(body.get_vertex(v).expect("the twin's keys").point)
                    .expect("a point")
            });
            let mut spec = geom_brep::EdgeCurveSpec::line_between(ends[0], ends[1]);
            spec.carrier = geom::Curve3::line(
                ends[0],
                // An axis direction of the unit brick: unit, exactly.
                geom_core::UnitVec3::checked(dir.map(Boxed::from_f64)),
            );
            spec.param_end = if dir.z == 0.0 {
                Boxed::from_f64(1.0)
            } else {
                top
            };
            body.set_edge_curve(e.edge, spec, tol)
                .expect("the edge takes its line");
        }
    }
    body
}

/// Runs `read` on [`lined_brick`] with its top cap over `1 ± half`.
fn over_lined<R>(half: f64, read: impl FnOnce(&Body<Boxed>, Band) -> R) -> R {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness band");
    let budget = SymBudget {
        max_terms: 64,
        max_degree: 8,
    };
    geom_core::sym::with_session(budget, || {
        let top = Sym::param_over(
            ParamSymbol::new(test_utils::symbol_id("lined_brick_top")),
            Interval::from_bounds(1.0 - half, 1.0 + half),
            1.0 - half,
            1.0 + half,
        );
        read(&lined_brick(top, tol), band)
    })
    .0
}

/// The brick's top cap: the face whose stated normal runs up `z`, its
/// `z` read as a number by `low` (the value at `f64`, an enclosure's
/// lower end over a box).
fn top_cap<T: Decide>(body: &Body<T>, low: impl Fn(T) -> f64) -> (FaceKey, Vec3<T>) {
    body.faces()
        .find_map(|(face, _)| {
            let normal = crate::face_normal::face_plane(body, face)?.normal;
            (low(normal.z) > 0.5).then_some((face, normal))
        })
        .expect("a brick has a top cap")
}

/// The upright `f64` brick whose top cap stands at `top`.
fn member(top: f64) -> Body<f64> {
    brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, top), Tol::witness())
}

/// Whether the enclosure of `x` holds `value` and is no wider than
/// `width`, as the reason it does not.
fn held(x: Boxed, value: f64, width: f64) -> Result<(), String> {
    let (lo, hi) = (geom_core::Bounds::lo(x), geom_core::Bounds::hi(x));
    if lo <= value && value <= hi && hi - lo <= width {
        Ok(())
    } else {
        Err(format!(
            "[{lo}, {hi}] (width {:e}) against the member's {value} and a width of {width:e}",
            hi - lo
        ))
    }
}

/// The half-widths the moving cap's readings are held at: a twentieth,
/// a quarter and nine twentieths of the brick's height.
const HALVES: [f64; 3] = [0.05, 0.25, 0.45];

/// **The turn of a moving cap's loop about a point of it is `2π` over
/// the box, and about a point beside it `0`, each as narrow as at a
/// member** (`solid_contain::loop_turn`). The cap's four vertices and
/// the point each carry the cap's boxed height, and the turn holds none
/// of it: every member's is `±2π` or `0` to rounding. Held within
/// `1e-9` of the members' value at each of [`HALVES`].
///
/// **Red where the turn is read on all three components**
/// (`u.dot(w)` for the second reading of `solid_contain::planar_cross_dot`):
/// `u·w` then sums the height's width squared, and at a quarter of the
/// height the turn about the inside point encloses `[−0.6, 13]`.
#[test]
fn a_moving_caps_turn_is_as_narrow_as_a_members() {
    use crate::solid_contain::planar_turn;
    let tau = core::f64::consts::TAU;
    for (what, x, turns) in [("inside", 0.37, 1.0), ("beside", 1.5, 0.0)] {
        for top in [0.55, 1.0, 1.45] {
            let body = member(top);
            let (face, normal) = top_cap(&body, |z| z);
            let turn = planar_turn(&body, face, normal, Point3::new(x, 0.41, top))
                .expect("a brick's cap is four lines");
            assert!(
                (turn - turns * tau).abs() < 1e-12,
                "{what}, the member at {top}: its turn is {turn}"
            );
        }
        for half in HALVES {
            let turn = over_lined(half, |body, _| {
                let (face, normal) = top_cap(body, geom_core::Bounds::lo);
                let height = crate::face_normal::face_plane(body, face)
                    .expect("a cap is planar")
                    .origin
                    .z;
                let p = Point3::new(Boxed::from_f64(x), Boxed::from_f64(0.41), height);
                planar_turn(body, face, normal, p).expect("a brick's cap is four lines")
            });
            if let Err(why) = held(turn, turns * tau, 1e-9) {
                panic!("{what}, top 1 ± {half}: the cap's turn is {why}");
            }
        }
    }
}

/// **A moving cap's aim is stepped in by its members' step**
/// (`side_reader::loop_aim`): across the cap it stands where every
/// member's does, to `1e-12`, and along the normal it is as wide as the
/// box. The step is half the least clearance of the cap's other edges,
/// `0.19` at every height, read between points that each carry the
/// cap's height.
///
/// **Red where the step reads `solid_contain::edge_clearance` with no
/// normal** (`None` for `Some(normal)` in `loop_aim`): a clearance then
/// carries the height's width along the normal, and at a twentieth of
/// the height the aim's `x` is `[0.191, 0.216]`.
#[test]
fn a_moving_caps_aim_is_stepped_by_its_members_step() {
    let at = |top: f64| {
        let body = member(top);
        let (face, normal) = top_cap(&body, |z| z);
        loop_aim(&body, face, normal).expect("a brick's cap aims")
    };
    let (low, high) = (at(0.55), at(1.45));
    assert!(
        (low.x - high.x).abs() < 1e-15 && (low.y - high.y).abs() < 1e-15,
        "the members' aims stand at one place across the cap: {low:?}, {high:?}"
    );
    for half in HALVES {
        let aim = over_lined(half, |body, _| {
            let (face, normal) = top_cap(body, geom_core::Bounds::lo);
            loop_aim(body, face, normal).expect("a brick's cap aims")
        });
        for (axis, got, want, width) in [
            ("x", aim.x, low.x, 1e-12),
            ("y", aim.y, low.y, 1e-12),
            ("z", aim.z, 1.0, 2.0 * half + 1e-12),
        ] {
            if let Err(why) = held(got, want, width) {
                panic!("top 1 ± {half}: the aim's {axis} is {why}");
            }
        }
    }
}

/// **A brick whose lines are stated as a door states them reads its
/// side by its caps' rays over a box half its height wide**
/// ([`lined_brick`]): both caps' aims read `Out` at `1 ± 0.25`, the
/// walls' are set aside, and no aim reads `In`.
///
/// **Red where the loops' turn is read on all three components**
/// (`u.dot(w)` for the second reading of `solid_contain::planar_cross_dot`):
/// neither cap's ray reads.
///
/// A wall's ray is not asked for: its aim hangs a fixed step under its
/// moving top edge
/// (`work/boxfinish/a-walls-aim-hangs-from-its-moving-edge-so-its-ray-reads-a-box-a-tenth-as-wide.md`).
#[test]
fn a_lined_boxed_brick_reads_by_its_caps_own_rays() {
    let own = over_lined(0.25, each_aim_report);
    assert_eq!(own.len(), 6, "a brick offers six aims");
    for read in &own {
        assert!(!read.contains("Side(In)"), "a wrong side: {read}");
    }
    let out = own
        .iter()
        .filter(|read| read.ends_with(": Ok(Side(Out))"))
        .count();
    assert_eq!(
        out, 2,
        "top 1 ± 0.25: both caps' rays read, the walls' do not: {own:#?}"
    );
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
///
/// The box is no wider because this brick's lines are minted from their
/// two ends (`EdgeCurveSpec::line_between`): a top edge's direction is
/// as wide as its chord, and past `± 0.04` a wall's ray is set aside on
/// its own top edge. [`a_lined_boxed_brick_reads_by_its_caps_own_rays`]
/// reads a brick whose lines are stated as a door states them.
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

#[cfg(test)]
#[path = "side_reader_tilted_cap_rows.rs"]
mod tilted_cap_rows;
