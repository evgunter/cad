//! **A bar wholly inside the material a concave band fills is absorbed**
//! (Ev's band ruling, PR 4963: a concave band's result is `S ∪ F`, `F`
//! the material it adds). A concave straight band cut off at two end
//! walls fills the prism over its section between their planes, and
//! buries each wall's sliver; a through-protrusion between the two walls
//! whose cycles lie inside each wall's sliver and whose walls lie inside
//! that prism is absorbed by the fill: its faces, edges and vertices
//! leave the body and its cycles leave the end walls. It is the convex
//! carve's through-bore (`band_planar_consume`, `band_ruled_cap_ring`)
//! with the material side flipped, and every reading is the convex one's.
//!
//! Two fixtures, both built by the Boolean, a bar unioned through a
//! pocket's end walls. The PLANAR one: the block `[−k, 2k] × [0, k] ×
//! [0, k]` less the pocket over the parallelogram `(¼, ¼)k, (¾, ¼)k,
//! (¾ + s/2, ¾)k, (¼ + s/2, ¾)k` from `z = k/2` up, whose floor edge
//! `(¼, ¼, ½)k → (¾, ¼, ½)k` is a right concave plane–plane crease
//! between end walls `x = k/4 + s·(y − k/4)` and `x = 3k/4 + s·(y − k/4)`
//! — the pocketed die at `s = 0`. The fill's section `A` (`(1 − π/4)·r²`
//! for the fillet, `r²/2` for the chamfer) runs the length `L = k/2`
//! between the walls along `x` at every lean, as does a bar along `x`, so
//! a bar of section `A_bar` gives `ΔV = +(A − A_bar)·L`. The RULED one: a
//! D-shaped pocket, the rod's D (`ROD_R`, `ROD_FLAT`) along `x` between
//! end walls `x = ¼` and `x = ¾`, open at its top, whose chord meets its
//! arc in a concave cylinder–plane crease; its fill is the D-rod's
//! convex cut-off section, `ΔV = +(A − π·a²)·L`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::RigidMotion;
use geom_core::{Bounds, Decide, Interval, Point2, Point3, Real, Vec3};
use profile::SketchPlane;
use sweep::blend::build::{Blended, chamfer_edges, fillet_edges};
use sweep::blend::{BlendError, Convexity};
use sweep::test_support::{
    ROD_FILLET, ROD_FLAT, ROD_R, assert_naming_totality, brick, finished, prism_on, rod_chord_at,
    rod_section_cut, sketch_from_axes,
};
use topo::{
    AtRestPolicy, Body, ContactRecords, EdgeKey, FaceKey, query, validate_geometric,
    validate_pseudomanifold,
};

use crate::band_planar_consume::{self, VERBS, section};
use crate::band_planar_cut_off::{D, Verb, edge, midpoint_tol, pad_ceiling, tol, volume_enclosure};

/// The bar's offset from both supports, and its radius, in units of the
/// fixture's scale: it lies `0.0245` inside the fillet's arc and
/// `≈ 0.054` inside the chamfer's chord.
pub(crate) const BAR: (f64, f64) = (0.012, 0.006);

fn mid<T: Bounds>(x: T) -> f64 {
    0.5 * (x.lo() + x.hi())
}

/// The union or difference of two fixtures at any scalar, through the
/// public door.
pub(crate) fn boolean<T: Decide + Bounds + AtRestPolicy>(
    union: bool,
    a: Body<T>,
    b: Body<T>,
) -> Body<T> {
    let (a, b) = (
        finished("operand A", a, tol()),
        finished("operand B", b, tol()),
    );
    if union {
        topo::union(&a, &b, tol())
    } else {
        topo::subtract(&a, &b, tol())
    }
    .unwrap_or_else(|e| panic!("the fixture's boolean builds, got {e}"))
    .body()
    .expect("the fixture's boolean is a body")
    .body
    .clone()
    .into_body()
}

/// The planar fixture's die at scale `k` and lean `s`, without a bar.
pub(crate) fn die<T: Decide + Bounds + AtRestPolicy>(k: f64, s: f64) -> Body<T> {
    let f = |x: f64| T::from_f64(x * k);
    let block = brick((-k, 2.0 * k), (0.0, k), (0.0, k), tol());
    let pocket = prism_on(
        SketchPlane::xy_at(Point3::new(f(0.0), f(0.0), f(0.5))),
        [
            (0.25, 0.25),
            (0.75, 0.25),
            (0.75 + s / 2.0, 0.75),
            (0.25 + s / 2.0, 0.75),
        ]
        .into_iter()
        .map(|(x, y)| (Point2::new(f(x), f(y)), T::zero()))
        .collect(),
        f(1.0),
        tol(),
    );
    boolean(false, block, pocket)
}

/// A bar's section.
#[derive(Clone, Copy)]
pub(crate) enum Section {
    /// Round, of this radius: two semicircles, its two seams at the ends
    /// of its diameter along the sketch's `u`.
    Round(f64),
    /// Square, of this half-side, its sides along `u` and square to it.
    Square(f64),
}

/// A bar of `section` whose axis crosses `x = x0` at `p` and `x = x1` at
/// `q` (both `(y, z)`), run on `over` past each along the axis's `x`, its
/// section's `u` axis the direction `u` turned square to the axis.
pub(crate) fn bar<T: Decide + AtRestPolicy>(
    (x0, p): (f64, (f64, f64)),
    (x1, q): (f64, (f64, f64)),
    over: f64,
    section: Section,
    u: Vec3<f64>,
) -> Body<T> {
    let t = Vec3::new(x1 - x0, q.0 - p.0, q.1 - p.1);
    let th = t.normalize();
    let o = Point3::new(x0, p.0, p.1) - th * (over / th.x);
    let u = (u - th * u.dot(th)).normalize();
    let v = th.cross(u);
    let lift = |w: Vec3<f64>| Vec3::new(T::from_f64(w.x), T::from_f64(w.y), T::from_f64(w.z));
    let f = T::from_f64;
    let verts = match section {
        Section::Round(a) => vec![
            (Point2::new(f(-a), f(0.0)), f(1.0)),
            (Point2::new(f(a), f(0.0)), f(1.0)),
        ],
        Section::Square(b) => [(-b, -b), (b, -b), (b, b), (-b, b)]
            .into_iter()
            .map(|(x, y)| (Point2::new(f(x), f(y)), f(0.0)))
            .collect(),
    };
    prism_on(
        sketch_from_axes(Point3::new(f(o.x), f(o.y), f(o.z)), lift(u), lift(v), tol()),
        verts,
        f(t.norm() + 2.0 * over / th.x),
        tol(),
    )
}

/// The planar fixture: [`die`] at scale `k` and lean `s` with a bar of
/// `section` through its end walls, its axis at `p` (pocket-local
/// `(y − k/4, z − k/2)`) at `x = k/4` and at `q` at `x = 3k/4`.
fn barred<T: Decide + Bounds + AtRestPolicy>(
    k: f64,
    s: f64,
    (p, q): ((f64, f64), (f64, f64)),
    section: Section,
    u: Vec3<f64>,
) -> Body<T> {
    let at = |(y, z): (f64, f64)| (0.25 * k + y, 0.5 * k + z);
    boolean(
        true,
        die(k, s),
        bar((0.25 * k, at(p)), (0.75 * k, at(q)), 0.15 * k, section, u),
    )
}

/// [`barred`] with a straight bar along `x` at pocket-local `c`.
fn straight<T: Decide + Bounds + AtRestPolicy>(
    k: f64,
    s: f64,
    c: (f64, f64),
    section: Section,
) -> Body<T> {
    barred(k, s, (c, c), section, Vec3::new(0.0, 1.0, 0.0))
}

/// The planar fixture's floor edge.
pub(crate) fn floor<T: Real + Bounds>(body: &Body<T>, k: f64) -> EdgeKey {
    edge(
        body,
        [0.25 * k, 0.25 * k, 0.5 * k],
        [0.75 * k, 0.25 * k, 0.5 * k],
    )
}

/// The faces of `body` on a cylinder of radius `a`, or on a plane that
/// is not axis-aligned or lies strictly inside the pocket's side walls:
/// the bar's walls.
fn bar_walls<T: Real + Bounds>(body: &Body<T>, section: Section, k: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| match (body.get_surface(f.surface), section) {
            (Some(geom::Surface::Cylinder { radius, .. }), Section::Round(a)) => {
                (mid(*radius) - a).abs() < 1e-9 * k
            }
            (Some(geom::Surface::Plane { origin, normal, .. }), Section::Square(_)) => {
                let (y, z) = (mid(origin.y), mid(origin.z));
                mid(normal.x).abs() < 1e-12
                    && y > 0.25 * k + 1e-9 * k
                    && z > 0.5 * k + 1e-9 * k
                    && y < 0.5 * k
                    && z < 0.75 * k
            }
            _ => false,
        })
        .map(|(key, _)| key)
        .collect()
}

/// `verb` at size `r` on `edges`, at any scalar.
pub(crate) fn blend<T: Decide + Bounds + AtRestPolicy>(
    verb: Verb,
    body: &Body<T>,
    edges: &[EdgeKey],
    r: f64,
) -> Result<Blended<T>, BlendError> {
    let body = finished("the blend's operand", body.clone(), tol());
    match verb {
        Verb::Fillet => fillet_edges(&body, edges, T::from_f64(r), tol()),
        Verb::Chamfer => chamfer_edges(&body, edges, T::from_f64(r), tol()),
    }
    .map_err(|e| e.error)
}

pub(crate) fn rings<T: Real>(body: &Body<T>) -> usize {
    body.faces().map(|(_, f)| f.rings.len()).sum()
}

/// **The bars are absorbed by the fill**: `verb` at `r` on `edges`
/// builds at tiers 3 and 3′ with naming totality; the bars' faces
/// `walls` are gone and retired; the source's one shell, of genus `g`
/// (each bar a handle across the pocket), is left one genus-0 shell, and
/// each bar's two cycles leave the end walls (`2g` fewer rings); and the
/// volume rises by `added` to `1e-12` of the scale `k` cubed, with the
/// enclosure a claim and its midpoint on the closed form. Returns the
/// build.
#[allow(clippy::too_many_arguments)] // the row's own reads, one per argument.
fn assert_absorbed<T: Decide + Bounds + AtRestPolicy + geom_core::CertifiedBounds>(
    verb: Verb,
    body: &Body<T>,
    edges: &[EdgeKey],
    r: f64,
    walls: &[FaceKey],
    (added, k): (f64, f64),
    what: &str,
) -> Blended<T> {
    let what = format!("{what} ({verb:?})");
    let what = what.as_str();
    validate_geometric(body, tol()).unwrap_or_else(|e| panic!("{what}: source tier 3, {e:?}"));
    assert!(!walls.is_empty(), "{what}: the source carries the bar");
    let c0 = topo::readback::euler_counts(body);
    let g = match (c0.s, c0.genus()) {
        (1, Ok(g)) if g > 0 => g,
        got => panic!("{what}: the source is one shell, each bar a handle, got {got:?}"),
    };
    let out = blend(verb, body, edges, r)
        .unwrap_or_else(|e| panic!("{what}: the fill absorbs the bar, got {e}"));
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, {e:?}"));
    validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
        .unwrap_or_else(|e| panic!("{what}: tier 3′, {e:?}"));
    assert_naming_totality(body, &out, edges, what);
    let rec = out.naming.as_ref().expect("the surgery records its births");
    for w in walls {
        assert!(
            out.body.get_face(*w).is_none() && rec.dead.faces.contains(w),
            "{what}: the bar's face {w:?} is absorbed and retired"
        );
    }
    assert_eq!(
        rings(&out.body) + 2 * usize::try_from(g).expect("a positive genus"),
        rings(body),
        "{what}: each bar's two cycles leave the end walls"
    );
    let c1 = topo::readback::euler_counts(&out.body);
    assert_eq!(
        (c1.s, c1.genus()),
        (1, Ok(0)),
        "{what}: one genus-0 shell, the handle gone"
    );
    let ((v0, pad0), (v1, pad1)) = (volume_enclosure(body), volume_enclosure(&out.body));
    let (dv, pad) = (v1 - v0, pad0 + pad1);
    let scale = k.powi(3);
    assert!(
        pad < pad_ceiling() * scale && (dv - added).abs() < 1e-12 * scale + pad,
        "{what}: ΔV {dv} ± {pad} vs the closed form {added}"
    );
    assert!(
        (dv - added).abs() < midpoint_tol() * scale,
        "{what}: ΔV's midpoint {dv} is off the closed form {added}"
    );
    out
}

/// **A bar between the pocket's end walls is absorbed at every scale,
/// under both verbs**: `ΔV = +(A − π·a²)·k/2`, `A` and `a` at scale `k`.
#[test]
fn a_bar_inside_a_concave_bands_fill_is_absorbed_at_every_scale() {
    let (d, a) = BAR;
    for k in [1e-2, 1.0, 1e2] {
        let body = straight::<f64>(k, 0.0, (d * k, d * k), Section::Round(a * k));
        let walls = bar_walls(&body, Section::Round(a * k), k);
        assert_eq!(walls.len(), 2, "k = {k}: the bar's two half-walls");
        for verb in VERBS {
            let added = (section(verb, D * k) - PI * (a * k).powi(2)) * 0.5 * k;
            assert_absorbed(
                verb,
                &body,
                &[floor(&body, k)],
                D * k,
                &walls,
                (added, k),
                &format!("a bar at scale {k}"),
            );
        }
    }
}

/// **The same bar at the certified scalar** is absorbed too, its volume
/// enclosure bracketing the closed form.
#[test]
fn a_bar_inside_a_concave_bands_fill_is_absorbed_at_interval() {
    let (d, a) = BAR;
    let body = straight::<Interval>(1.0, 0.0, (d, d), Section::Round(a));
    let walls = bar_walls(&body, Section::Round(a), 1.0);
    assert_eq!(walls.len(), 2, "Interval: the bar's two half-walls");
    for verb in VERBS {
        let added = (section(verb, D) - PI * a * a) * 0.5;
        assert_absorbed(
            verb,
            &body,
            &[floor(&body, 1.0)],
            D,
            &walls,
            (added, 1.0),
            "a bar at Interval",
        );
    }
}

/// **At every lean of the end walls**, under both verbs: each of the
/// bar's cycles is an ellipse on a wall `x = x₀ + s·(y − k/4)`, read
/// inside that wall's sliver, whose projection along the spine is the
/// section's by `|n·τ| = 1/√(1 + s²)`; `ΔV = +(A − π·a²)/2`.
#[test]
fn a_bar_inside_a_concave_bands_fill_is_absorbed_at_every_lean() {
    let (d, a) = BAR;
    for s in [0.3, 1.0, -1.0] {
        let what = format!("a bar at lean {s}");
        let body = straight::<f64>(1.0, s, (d, d), Section::Round(a));
        let walls = bar_walls(&body, Section::Round(a), 1.0);
        assert_eq!(walls.len(), 2, "{what}: the bar's two half-walls");
        for verb in VERBS {
            let added = (section(verb, D) - PI * a * a) * 0.5;
            assert_absorbed(
                verb,
                &body,
                &[floor(&body, 1.0)],
                D,
                &walls,
                (added, 1.0),
                &what,
            );
        }
    }
}

/// **A square bar** — plane walls along the spine, each bounded by two
/// cycle edges and two lines between cycle vertices — is absorbed at
/// `ΔV = +(A − 4b²)/2`, at the square and the leaning walls.
#[test]
fn a_square_bar_inside_a_concave_bands_fill_is_absorbed() {
    let (c, b) = (0.016, 0.006);
    for s in [0.0, 1.0] {
        let what = format!("a square bar at lean {s}");
        let body = straight::<f64>(1.0, s, (c, c), Section::Square(b));
        let walls = bar_walls(&body, Section::Square(b), 1.0);
        assert_eq!(walls.len(), 4, "{what}: the bar's four walls");
        for verb in VERBS {
            let added = (section(verb, D) - 4.0 * b * b) * 0.5;
            assert_absorbed(
                verb,
                &body,
                &[floor(&body, 1.0)],
                D,
                &walls,
                (added, 1.0),
                &what,
            );
        }
    }
}

/// A round rod of radius `a` along `x` over `x`, its axis at
/// pocket-local `c`, at scale one.
fn rod(x: (f64, f64), c: (f64, f64), a: f64) -> Body<f64> {
    let at = (0.25 + c.0, 0.5 + c.1);
    bar(
        (x.0, at),
        (x.1, at),
        0.0,
        Section::Round(a),
        Vec3::new(0.0, 1.0, 0.0),
    )
}

/// An axis-aligned box over `x`, and over `u` and `v` pocket-local, at
/// scale one.
fn pocket_box(x: (f64, f64), u: (f64, f64), v: (f64, f64)) -> Body<f64> {
    brick(x, (0.25 + u.0, 0.25 + u.1), (0.5 + v.0, 0.5 + v.1), tol())
}

/// **Two bars in one fill are both absorbed**, under both verbs: genus
/// 2 → 0, four rings leave, `ΔV = +(A − π·a² − π·a′²)/2`.
#[test]
fn two_bars_inside_a_concave_bands_fill_are_both_absorbed() {
    let (d, a) = BAR;
    let b = 0.004;
    let body = boolean(
        true,
        straight(1.0, 0.0, (d, d), Section::Round(a)),
        rod((0.1, 0.9), (0.03, 0.008), b),
    );
    let mut walls = bar_walls(&body, Section::Round(a), 1.0);
    walls.extend(bar_walls(&body, Section::Round(b), 1.0));
    assert_eq!(walls.len(), 4, "the two bars' four half-walls");
    for verb in VERBS {
        let added = (section(verb, D) - PI * (a * a + b * b)) * 0.5;
        assert_absorbed(
            verb,
            &body,
            &[floor(&body, 1.0)],
            D,
            &walls,
            (added, 1.0),
            "two bars",
        );
    }
}

/// Those of `faces` that carry an edge from one end wall to the other at
/// lean zero: a bar's walls between them.
fn between_the_walls(body: &Body<f64>, faces: &[FaceKey]) -> Vec<FaceKey> {
    let x = |v| {
        body.get_point(body.get_vertex(v).expect("a vertex").point)
            .expect("a point")
            .x
    };
    let spans: Vec<FaceKey> = query::all_edges(body)
        .into_iter()
        .filter_map(|e| body.get_edge(e))
        .filter(|e| {
            let (s, t) = (
                x(body.get_half_edge(e.he_plus).expect("a half").start),
                x(body.half_edge_end(e.he_plus).expect("an end")),
            );
            ((s - 0.25).abs() < 1e-9 && (t - 0.75).abs() < 1e-9)
                || ((s - 0.75).abs() < 1e-9 && (t - 0.25).abs() < 1e-9)
        })
        .flat_map(|e| [e.he_plus, e.he_minus])
        .filter_map(|h| body.face_of_half_edge(h))
        .collect();
    faces
        .iter()
        .copied()
        .filter(|f| spans.contains(f))
        .collect()
}

/// **A bar run on out through the block's outer faces** is absorbed
/// between the end walls and keeps its stubs outside the block, under
/// both verbs: its segment in the pocket goes, at `ΔV = +(A − π·a²)/2`,
/// and its two cycles on the outer faces and its walls outside them
/// stay.
#[test]
fn a_bar_run_out_through_the_outer_faces_is_absorbed_between_the_walls() {
    let (d, a) = BAR;
    let body = boolean(true, die(1.0, 0.0), rod((-1.3, 2.3), (d, d), a));
    let all = bar_walls(&body, Section::Round(a), 1.0);
    let walls = between_the_walls(&body, &all);
    assert_eq!(walls.len(), 2, "the bar's two half-walls in the pocket");
    for verb in VERBS {
        let added = (section(verb, D) - PI * a * a) * 0.5;
        let out = assert_absorbed(
            verb,
            &body,
            &[floor(&body, 1.0)],
            D,
            &walls,
            (added, 1.0),
            "a bar out through the block",
        );
        assert_eq!(
            bar_walls(&out.body, Section::Round(a), 1.0).len() + 2,
            all.len(),
            "{verb:?}: the stubs' walls stay"
        );
    }
}

/// **Two concave bands in one request**: the floor edge and the
/// opposite floor edge `(¼, ¾, ½) → (¾, ¾, ½)`, the bar in the first
/// band's fill, under both verbs: the bar is absorbed by its band, at
/// `ΔV = +(2A − π·a²)/2`.
#[test]
fn a_bar_under_one_of_two_concave_bands_is_absorbed() {
    let (d, a) = BAR;
    let body = straight::<f64>(1.0, 0.0, (d, d), Section::Round(a));
    let walls = bar_walls(&body, Section::Round(a), 1.0);
    let edges = [
        floor(&body, 1.0),
        edge(&body, [0.25, 0.75, 0.5], [0.75, 0.75, 0.5]),
    ];
    for verb in VERBS {
        let added = (2.0 * section(verb, D) - PI * a * a) * 0.5;
        assert_absorbed(
            verb,
            &body,
            &edges,
            D,
            &walls,
            (added, 1.0),
            "a bar under one of two bands",
        );
    }
}

/// `verb`'s refusal on the planar fixture `body` at scale one.
fn refusal(verb: Verb, body: &Body<f64>) -> BlendError {
    match blend(verb, body, &[floor(body, 1.0)], D) {
        Err(e) => e,
        Ok(_) => panic!("{verb:?}: the request refuses"),
    }
}

/// The pocket-local centre of a bar of radius `a` on the bisector,
/// `dist` from the fillet's arc towards the old vertex (negative: past
/// the arc, into the ball).
pub(crate) fn past_the_arc(dist: f64, a: f64) -> (f64, f64) {
    let rho = (D + dist + a) / 2f64.sqrt();
    (D - rho, D - rho)
}

/// The pocket-local centre of a bar `dist` from the chamfer's chord, on
/// its normal through the chord's midpoint, towards the old vertex: the
/// convex suite's point, mirrored through its corner `(1, 1)` onto the
/// pocket's `(0, 0)`.
pub(crate) fn past_the_chord(dist: f64) -> (f64, f64) {
    let (u, v) = band_planar_consume::past_the_chord(dist);
    (1.0 - u, 1.0 - v)
}

/// **A hair either side of the fill's boundary, at every lean.** A bar
/// of radius `0.004` whose far side is `1e-4` short of the fillet's arc,
/// or the chamfer's chord, crosses the band surface and refuses
/// `RingClearance` as a measurement at the end wall, as does one centred
/// on it; one `1e-4` inside is absorbed — but the fillet's at a leaning
/// wall, which reads as a bound: there the cycle and the arc are
/// ellipses, and the arc's inside reading charges the cycle its major
/// semi-axis across the section's minor one, which at lean `s` costs
/// `a·(√(1 + s²) − 1)`, `1.8e-4` at `0.3`
/// (`work/bandmeter/an-oblique-caps-inside-reading-charges-a-ring-its-major-semi-axis.md`;
/// the convex bore reads the same).
#[test]
fn a_bar_a_hair_across_the_fills_boundary_crosses_and_one_inside_is_absorbed() {
    let (a, hair) = (0.004, 1e-4);
    for s in [0.0, 0.3, 1.0, -1.0] {
        for verb in VERBS {
            let at = |dist: f64| match verb {
                Verb::Fillet => past_the_arc(dist, a),
                Verb::Chamfer => past_the_chord(dist + a),
            };
            for (what, dist) in [("a hair short of the band", -hair), ("on the band", -a)] {
                let what = format!("{what} at lean {s}");
                let got = refusal(verb, &straight(1.0, s, at(dist), Section::Round(a)));
                assert!(
                    matches!(
                        got,
                        BlendError::RingClearance {
                            bounded: false,
                            chain: Convexity::Concave,
                            ..
                        }
                    ),
                    "{verb:?}, {what}: a measured RingClearance at the end wall, got {got:?}"
                );
            }
            let what = format!("a hair inside the band at lean {s}");
            let body = straight::<f64>(1.0, s, at(hair), Section::Round(a));
            if matches!(verb, Verb::Fillet) && s != 0.0 {
                let got = refusal(verb, &body);
                assert!(
                    matches!(
                        got,
                        BlendError::FaceClearance {
                            bounded: true,
                            chain: Convexity::Concave,
                            ..
                        }
                    ),
                    "{verb:?}, {what}: a bound at the bar's wall, got {got:?}"
                );
                continue;
            }
            let walls = bar_walls(&body, Section::Round(a), 1.0);
            let added = (section(verb, D) - PI * a * a) * 0.5;
            assert_absorbed(
                verb,
                &body,
                &[floor(&body, 1.0)],
                D,
                &walls,
                (added, 1.0),
                &what,
            );
        }
    }
}

/// **A bar an escalate width either side of the fillet's arc**, at
/// `dist ∈ ±{1e-9, 5e-9, 2e-8}`, radius `0.004`, on the bisector: one
/// past the arc is never absorbed, at every eps (it refuses at the end
/// wall's meter or the reach); one inside is absorbed at
/// `ΔV = +(A − π·a²)/2` or refuses as a bound, never as a measurement.
#[test]
fn a_bar_an_escalate_width_past_the_arc_is_never_absorbed() {
    let a = 0.004;
    for dist in [-2e-8, -5e-9, -1e-9, 1e-9, 5e-9, 2e-8] {
        let what = format!("a bar {dist:e} inside the arc");
        let body = straight::<f64>(1.0, 0.0, past_the_arc(dist, a), Section::Round(a));
        match blend(Verb::Fillet, &body, &[floor(&body, 1.0)], D) {
            Err(e) => {
                let ok = match e {
                    BlendError::RingClearance {
                        chain: Convexity::Concave,
                        bounded,
                        ..
                    }
                    | BlendError::FaceClearance {
                        chain: Convexity::Concave,
                        bounded,
                        ..
                    } => dist < 0.0 || bounded,
                    _ => false,
                };
                assert!(
                    ok,
                    "{what}: a clearance refusal, a bound if inside, got {e:?}"
                );
            }
            Ok(_) => {
                assert!(dist > 0.0, "{what}: a bar past the arc is never absorbed");
                let walls = bar_walls(&body, Section::Round(a), 1.0);
                let added = (section(Verb::Fillet, D) - PI * a * a) * 0.5;
                assert_absorbed(
                    Verb::Fillet,
                    &body,
                    &[floor(&body, 1.0)],
                    D,
                    &walls,
                    (added, 1.0),
                    &what,
                );
            }
        }
    }
}

/// The tilted bar at lean `s`: its axis a tangent of the fillet arc's
/// circle, touching it at mid-length (the arc's point nearest the old
/// vertex), its ends `h = 0.0255` either side along the tangent on the
/// two end walls, of radius `a = 0.002`, its two seams along the
/// tangent; and its radius and the axis's length between the walls.
fn tilted_bar(s: f64) -> (Body<f64>, f64, f64) {
    let (r, h, a) = (D, 0.0255, 0.002);
    let s2 = 2f64.sqrt();
    let touch = (r - r / s2, r - r / s2);
    let p = (touch.0 - h / s2, touch.1 + h / s2);
    let q = (touch.0 + h / s2, touch.1 - h / s2);
    let on = |x0: f64, (y, z): (f64, f64)| (x0 + s * y, (0.25 + y, 0.5 + z));
    let (start, end) = (on(0.25, p), on(0.75, q));
    let axis =
        ((end.0 - start.0).powi(2) + (end.1.0 - start.1.0).powi(2) + (end.1.1 - start.1.1).powi(2))
            .sqrt();
    let body = boolean(
        true,
        die(1.0, s),
        bar(
            start,
            end,
            0.15,
            Section::Round(a),
            Vec3::new(0.0, 1.0, -1.0),
        ),
    );
    (body, a, axis)
}

/// The leans the tilted bar is read at.
const LEANS: [f64; 4] = [0.0, 0.3, 1.0, -1.0];

/// **A tilted bar inside the fill at both end walls that pokes out of
/// the band surface between them refuses** at every lean, as a bound at
/// its wall: each cycle is `√(r² + h²) − r − a ≈ 1.2e-3` inside the
/// sliver, and the bar runs `a` past the arc into the ball at
/// mid-length. Its seams run along the arc's tangent, never nearer its
/// centre than `r`, so every sampled point of the wall's boundary lies
/// in the fill and the exact core reads no crossing: only
/// `prism_inside`'s line slack and its `diagonal·|a × τ|` term hold this
/// row. Without them the bar is absorbed through the band surface.
#[test]
fn a_tilted_bar_inside_at_both_end_walls_through_the_band_surface_refuses() {
    for s in LEANS {
        let (body, ..) = tilted_bar(s);
        let got = refusal(Verb::Fillet, &body);
        assert!(
            matches!(
                got,
                BlendError::FaceClearance {
                    bounded: true,
                    chain: Convexity::Concave,
                    ..
                }
            ),
            "lean {s}: the tilted bar refuses as a bound at its wall, got {got:?}"
        );
    }
}

/// **The same tilted bar is absorbed by the chamfer's fill** at every
/// lean, at `ΔV = +(A·L − π·a²·ℓ)`, `ℓ` the axis's length between the
/// walls (a round bar between parallel planes holds `π·a²` per unit of
/// its axis). The fill is a triangular prism, convex, so a wall whose
/// every point lies on a ruling between points of cycles inside it lies
/// inside it too, and the reading owes no slack for the lateral run.
#[test]
fn a_tilted_bar_inside_the_chamfers_fill_is_absorbed() {
    for s in LEANS {
        let (body, a, axis) = tilted_bar(s);
        let walls = bar_walls(&body, Section::Round(a), 1.0);
        assert_eq!(walls.len(), 2, "lean {s}: the tilted bar's two half-walls");
        let added = section(Verb::Chamfer, D) * 0.5 - PI * a * a * axis;
        assert_absorbed(
            Verb::Chamfer,
            &body,
            &[floor(&body, 1.0)],
            D,
            &walls,
            (added, 1.0),
            &format!("a tilted bar at lean {s}"),
        );
    }
}

/// What a conservative refusal is pinned at.
#[derive(Clone, Copy, Debug)]
enum Pinned {
    /// `FaceClearance { bounded: true }` on a chain of this convexity.
    Bound(Convexity),
    /// `WhollyInside` on a concave chain.
    WhollyInside,
}

/// **Shapes the rule defines and the carve does not build refuse**,
/// under both verbs, each at the meter that reads it first:
/// - a sealed tube, a bar with a void inside it wholly inside `F`
///   (`S ∪ F` fills the void): `WhollyInside`, at the void's shell, which
///   has no cap cycle;
/// - an island, a free box wholly inside `F` (absorbed): `WhollyInside`;
/// - a blind hole drilled through one end wall, or one through each,
///   its mouth inside the sliver (`S ∪ F` seals it into a void): a
///   bound, the hole's wall meeting the wall's plane;
/// - the convex mirror, a pin out of a box's end face from inside its
///   convex sliver (`S − M` cuts it free): a bound.
#[test]
fn shapes_the_carve_does_not_build_refuse_conservatively() {
    let (d, a) = BAR;
    let barred = || straight::<f64>(1.0, 0.0, (d, d), Section::Round(a));
    let die = || die::<f64>(1.0, 0.0);
    let pin_out = boolean(
        true,
        brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()),
        bar(
            (-0.2, (1.0 - d, 1.0 - d)),
            (0.05, (1.0 - d, 1.0 - d)),
            0.0,
            Section::Round(a),
            Vec3::new(0.0, 1.0, 0.0),
        ),
    );
    let pin_edge = edge(&pin_out, [0.0, 1.0, 1.0], [1.0, 1.0, 1.0]);
    let rows = [
        (
            "a sealed tube",
            boolean(false, barred(), rod((0.3, 0.7), (d, d), 0.003)),
            None,
            Pinned::WhollyInside,
        ),
        (
            "an island",
            boolean(
                true,
                die(),
                pocket_box((0.4, 0.6), (0.005, 0.015), (0.005, 0.015)),
            ),
            None,
            Pinned::WhollyInside,
        ),
        (
            "a blind hole in one end wall",
            boolean(false, die(), rod((0.1, 0.3), (d, d), a)),
            None,
            Pinned::Bound(Convexity::Concave),
        ),
        (
            "a blind hole in each end wall",
            boolean(
                false,
                boolean(false, die(), rod((0.1, 0.3), (d, d), a)),
                rod((0.7, 0.9), (d, d), a),
            ),
            None,
            Pinned::Bound(Convexity::Concave),
        ),
        (
            "a pin out of a convex sliver",
            pin_out,
            Some(pin_edge),
            Pinned::Bound(Convexity::Convex),
        ),
    ];
    for (what, body, crease, want) in rows {
        let crease = crease.unwrap_or_else(|| floor(&body, 1.0));
        for verb in VERBS {
            let got = blend(verb, &body, &[crease], D).map(|_| ());
            let held = match (want, &got) {
                (
                    Pinned::Bound(c),
                    Err(BlendError::FaceClearance {
                        bounded: true,
                        chain,
                        ..
                    }),
                ) => *chain == c,
                (
                    Pinned::WhollyInside,
                    Err(BlendError::WhollyInside {
                        chain: Convexity::Concave,
                        ..
                    }),
                ) => true,
                _ => false,
            };
            assert!(held, "{what} ({verb:?}): refuses {want:?}, got {got:?}");
        }
    }
}

/// **A bump on a bar, both wholly inside the fill, is absorbed with it**
/// under both verbs: the box `[0.45, 0.55] × [0.009, 0.015] ×
/// [0.012, 0.024]` (pocket-local `u`, `v`) stands on the bar's wall, which
/// carries its footprint as a ring; the carve bridges the ring to the
/// wall's outer cycle and takes the bar and the bump as one annulus. Over
/// the bump's `0.1` the bar's disk holds `c·√(a² − c²) + a²·asin(c/a)` of
/// the box's section (`c = 0.003` its half-width, the box's far side
/// above the disk), so
/// `ΔV = +(A − π·a²)/2 − 0.1·(2c·0.012 − c·√(a² − c²) − a²·asin(c/a))`.
#[test]
fn a_bump_on_a_bar_inside_the_fill_is_absorbed_with_it() {
    let (d, a) = BAR;
    let c: f64 = 0.003;
    let body = boolean(
        true,
        straight::<f64>(1.0, 0.0, (d, d), Section::Round(a)),
        pocket_box((0.45, 0.55), (0.009, 0.015), (0.012, 0.024)),
    );
    validate_geometric(&body, tol()).expect("the bumped bar is tier-3 valid");
    let held = c * (a * a - c * c).sqrt() + a * a * (c / a).asin();
    for verb in VERBS {
        let out = blend(verb, &body, &[floor(&body, 1.0)], D)
            .unwrap_or_else(|e| panic!("{verb:?}: the bump goes with the bar, got {e}"));
        validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{verb:?}: tier 3, {e:?}"));
        validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
            .unwrap_or_else(|e| panic!("{verb:?}: tier 3′, {e:?}"));
        assert_naming_totality(&body, &out, &[floor(&body, 1.0)], "a bump on a bar");
        assert_eq!(
            rings(&out.body) + 3,
            rings(&body),
            "{verb:?}: the bar's two cycles and the footprint leave"
        );
        let c1 = topo::readback::euler_counts(&out.body);
        assert_eq!((c1.s, c1.genus()), (1, Ok(0)), "{verb:?}: the handle goes");
        let want = (section(verb, D) - PI * a * a) * 0.5 - 0.1 * (2.0 * c * 0.012 - held);
        let ((v0, _), (v1, _)) = (volume_enclosure(&body), volume_enclosure(&out.body));
        assert!(
            (v1 - v0 - want).abs() < midpoint_tol(),
            "{verb:?}: ΔV {} vs the closed form {want}",
            v1 - v0
        );
    }
}

/// **A bar sunk into the side wall refuses** under both verbs, at the
/// battery's screen of the support it joins: its walls cross that
/// support's strip, so it is not attached by its two cycles alone.
#[test]
fn a_bar_sunk_into_a_support_refuses() {
    let a = 0.006;
    for verb in VERBS {
        let got = refusal(verb, &straight(1.0, 0.0, (0.002, 0.03), Section::Round(a)));
        assert!(
            matches!(got, BlendError::FaceClearanceUncertified { .. }),
            "{verb:?}: the sunk bar refuses at the support's screen, got {got:?}"
        );
    }
}

/// The D-shaped pocket: the block `[0, 1] × [−1, 1] × [−1, 0.3]` less the
/// rod's D in `(y, z)` along `x` from `¼` to `¾`, open at the block's top,
/// with a round bar of radius `a` at `(y, z) = c` along `x` through its
/// end walls; and its concave crease, the chord's lower end.
pub(crate) fn d_pocket(c: Option<(f64, f64)>, a: f64) -> (Body<f64>, EdgeKey) {
    let ch = rod_chord_at(ROD_FLAT);
    let d = prism_on(
        SketchPlane::from_frame(RigidMotion::axes_yz(Point3::new(0.25, 0.0, 0.0))),
        vec![
            (Point2::new(ROD_FLAT, ch.half), ch.wall_bulge),
            (Point2::new(ROD_FLAT, -ch.half), 0.0),
        ],
        0.5,
        tol(),
    );
    let pocket = boolean(false, brick((0.0, 1.0), (-1.0, 1.0), (-1.0, 0.3), tol()), d);
    let body = match c {
        Some(c) => boolean(
            true,
            pocket,
            bar(
                (0.25, c),
                (0.75, c),
                0.15,
                Section::Round(a),
                Vec3::new(0.0, 1.0, 0.0),
            ),
        ),
        None => pocket,
    };
    let crease = edge(
        &body,
        [0.25, ROD_FLAT, -ch.half],
        [0.75, ROD_FLAT, -ch.half],
    );
    (body, crease)
}

/// **A bar inside a concave ruled crease's fill is absorbed** at
/// `ΔV = +(A − π·a²)/2`, `A` the D-rod's cut-off section (the same ball
/// in the same D, on the other material side). The bar at
/// `(0.297, −0.395)` is `0.003` from the flat, `0.0033` from the D's
/// wall and `≈ 0.1085` from the ball's centre.
#[test]
fn a_bar_inside_a_concave_ruled_creases_fill_is_absorbed() {
    let a = 0.002;
    let (body, crease) = d_pocket(Some((0.297, -0.395)), a);
    let walls = bar_walls(&body, Section::Round(a), 1.0);
    assert_eq!(walls.len(), 2, "the bar's two half-walls");
    let added = (rod_section_cut(ROD_R, ROD_FLAT, ROD_FILLET) - PI * a * a) * 0.5;
    assert_absorbed(
        Verb::Fillet,
        &body,
        &[crease],
        ROD_FILLET,
        &walls,
        (added, 1.0),
        "a bar in the D pocket",
    );
}

/// **A bar across a concave ruled crease's fill boundary refuses**:
/// centred on the band's arc, `RingClearance` as a measurement at the end
/// wall; across the D's flat, at the battery's screen of that support.
#[test]
fn a_bar_across_a_concave_ruled_creases_fill_refuses() {
    let a = 0.002;
    let (cx, cy) = (
        ROD_FLAT - ROD_FILLET,
        -((ROD_R - ROD_FILLET).powi(2) - (ROD_FLAT - ROD_FILLET).powi(2)).sqrt(),
    );
    let v = (ROD_FLAT, -rod_chord_at(ROD_FLAT).half);
    let n = (v.0 - cx).hypot(v.1 - cy);
    let on_arc = (
        cx + (v.0 - cx) / n * ROD_FILLET,
        cy + (v.1 - cy) / n * ROD_FILLET,
    );
    let (body, crease) = d_pocket(Some(on_arc), a);
    let got = blend(Verb::Fillet, &body, &[crease], ROD_FILLET).map(|_| ());
    assert!(
        matches!(
            got,
            Err(BlendError::RingClearance {
                bounded: false,
                chain: Convexity::Concave,
                ..
            })
        ),
        "on the arc: a measured RingClearance at the end wall, got {got:?}"
    );
    let (body, crease) = d_pocket(Some((0.299, -0.395)), a);
    let got = blend(Verb::Fillet, &body, &[crease], ROD_FILLET).map(|_| ());
    assert!(
        matches!(got, Err(BlendError::FaceClearanceUncertified { .. })),
        "across the flat: refused at the support's screen, got {got:?}"
    );
}
