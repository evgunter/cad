//! **A feature attached to a band's region by one cycle on an end wall,
//! wholly inside the region, goes with it** (Ev's band ruling, PR 4963:
//! a convex band's result is `S − R`, a concave band's `S ∪ F`). A
//! plane–plane band cut off at two end walls removes (convex) or fills
//! (concave) the prism over its section between their planes; a cup
//! whose mouth lies inside one wall's sliver and whose faces lie inside
//! that prism — a blind bore under a convex band, a pin under a concave
//! one — goes with the material: its faces, edges and vertices leave the
//! body and its mouth leaves the wall. The faces that do not meet the
//! wall (the floor, the pin's end, an undercut's shoulder) are read
//! inside the band's exact core; the faces that do, through the mouth
//! and the edges those faces certify (`reach::prism_inside`).
//!
//! The CONVEX fixture is the block `[0, k]³`, its edge `(0, k, k) →
//! (k, k, k)` blended at `D·k`, its end faces `x = 0` and `x = k`; a
//! bore of radius `a` along `x` from `x = 0` to depth `h` held `π·a²·h`
//! of air, so `ΔV = −(A·k − π·a²·h)`, `A` the section (`(1 − π/4)·r²`
//! for the fillet, `r²/2` for the chamfer). The CONCAVE fixture is
//! `band_concave_bar`'s die, whose fill `A` runs `k/2` between end walls
//! `x = k/4 + s·(y − k/4)` and `x = 3k/4 + s·(y − k/4)`; a pin of radius
//! `a` along `x` from the first wall to `x = x₁` holds `π·a²` per unit of
//! its axis, so `ΔV = +(A·k/2 − π·a²·ℓ)`, `ℓ` its axis between the wall
//! and `x₁`. Both are spelled in corner coordinates `(u, v)`, the
//! distances from the two supports.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Bounds, Decide, Interval, Point3, Real, Vec3};
use sweep::blend::{BlendError, Convexity};
use sweep::test_support::{assert_naming_totality, brick};
use topo::{
    AtRestPolicy, Body, ContactRecords, EdgeKey, FaceKey, validate_geometric,
    validate_pseudomanifold,
};

use crate::band_concave_bar::{
    BAR, Section, bar, blend, boolean, d_pocket, die, floor, past_the_arc, past_the_chord, rings,
};
use crate::band_planar_consume::{VERBS, section};
use crate::band_planar_cut_off::{D, Verb, edge, midpoint_tol, pad_ceiling, tol, volume_enclosure};

fn mid<T: Bounds>(x: T) -> f64 {
    0.5 * (x.lo() + x.hi())
}

/// A round bar of radius `a` along `x` over `[x0, x1]`, its axis at
/// `(y, z)`.
fn rod<T: Decide + AtRestPolicy>((x0, x1): (f64, f64), (y, z): (f64, f64), a: f64) -> Body<T> {
    bar(
        (x0, (y, z)),
        (x1, (y, z)),
        0.0,
        Section::Round(a),
        Vec3::new(0.0, 1.0, 0.0),
    )
}

/// The convex block at scale `k`.
fn block<T: Decide + Bounds + AtRestPolicy>(k: f64) -> Body<T> {
    brick((0.0, k), (0.0, k), (0.0, k), tol())
}

/// The convex block's edge `(0, k, k) → (k, k, k)`.
fn top<T: Real + Bounds>(body: &Body<T>, k: f64) -> EdgeKey {
    edge(body, [0.0, k, k], [k, k, k])
}

/// The convex block's `(y, z)` at corner coordinates `(u, v)`.
fn convex_at(k: f64, (u, v): (f64, f64)) -> (f64, f64) {
    (k - u, k - v)
}

/// The concave die's `(y, z)` at corner coordinates `(u, v)`.
fn concave_at(k: f64, (u, v): (f64, f64)) -> (f64, f64) {
    (0.25 * k + u, 0.5 * k + v)
}

/// The convex block at scale `k` less a blind bore of radius `a` at
/// corner coordinates `c`, from the end face `x = 0` to depth `h`.
fn blind_bore<T: Decide + Bounds + AtRestPolicy>(k: f64, c: (f64, f64), a: f64, h: f64) -> Body<T> {
    boolean(false, block(k), rod((-0.2 * k, h), convex_at(k, c), a))
}

/// The die at scale `k` and lean `s` with a pin of radius `a` at corner
/// coordinates `c`, from inside the first end wall's material to
/// `x = x1`.
fn pin<T: Decide + Bounds + AtRestPolicy>(
    k: f64,
    s: f64,
    c: (f64, f64),
    a: f64,
    x1: f64,
) -> Body<T> {
    boolean(true, die(k, s), rod((0.1 * k, x1), concave_at(k, c), a))
}

/// The faces of `body` whose every vertex lies within `reach` of the
/// line along `x` through `(y, z)`, over `x ∈ [x0, x1]`: a cup's faces.
pub(crate) fn cup_faces<T: Real + Bounds>(
    body: &Body<T>,
    (y, z): (f64, f64),
    reach: f64,
    (x0, x1): (f64, f64),
) -> Vec<FaceKey> {
    let at = |v: topo::VertexKey| {
        let p = *body
            .get_point(body.get_vertex(v).expect("v").point)
            .expect("p");
        Point3::new(mid(p.x), mid(p.y), mid(p.z))
    };
    let slack = 1e-9 * (1.0 + reach + x1.abs());
    let near = |p: Point3<f64>| {
        (p.y - y).hypot(p.z - z) <= reach + slack && p.x >= x0 - slack && p.x <= x1 + slack
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

/// **The cups go with the band**: `verb` at `r` on `edges` builds at
/// tiers 3 and 3′ with naming totality; the cups' faces `cups` are gone
/// and retired; the shell count holds and the genus drops by the
/// `through` features (a cup is no handle, a through bore is one) and
/// each of the `mouths` leaves its end wall; and the volume moves by
/// `dv` to `1e-12` of the scale `k` cubed, the enclosure a claim and its
/// midpoint on the closed form.
#[allow(clippy::too_many_arguments)] // the row's own reads, one per argument.
pub(crate) fn assert_taken<T: Decide + Bounds + AtRestPolicy + geom_core::CertifiedBounds>(
    verb: Verb,
    body: &Body<T>,
    edges: &[EdgeKey],
    r: f64,
    (cups, mouths, through): (&[FaceKey], usize, i64),
    (dv_want, k): (f64, f64),
    what: &str,
) {
    let what = format!("{what} ({verb:?})");
    let what = what.as_str();
    validate_geometric(body, tol()).unwrap_or_else(|e| panic!("{what}: source tier 3, {e:?}"));
    assert!(!cups.is_empty(), "{what}: the source carries the cup");
    let c0 = topo::readback::euler_counts(body);
    let out = blend(verb, body, edges, r)
        .unwrap_or_else(|e| panic!("{what}: the cup goes with the band, got {e}"));
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, {e:?}"));
    validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
        .unwrap_or_else(|e| panic!("{what}: tier 3′, {e:?}"));
    assert_naming_totality(body, &out, edges, what);
    let rec = out.naming.as_ref().expect("the surgery records its births");
    for f in cups {
        assert!(
            out.body.get_face(*f).is_none() && rec.dead.faces.contains(f),
            "{what}: the cup's face {f:?} goes and is retired"
        );
    }
    let held: usize = cups
        .iter()
        .map(|f| body.get_face(*f).expect("a cup's face").rings.len())
        .sum();
    assert_eq!(
        rings(&out.body) + mouths + held,
        rings(body),
        "{what}: each mouth leaves its end wall, and the cups' faces their rings"
    );
    let c1 = topo::readback::euler_counts(&out.body);
    assert_eq!(
        (c1.s, c1.genus().expect("a genus") + through),
        (c0.s, c0.genus().expect("a genus")),
        "{what}: the shells hold and each through feature's handle goes"
    );
    let ((v0, pad0), (v1, pad1)) = (volume_enclosure(body), volume_enclosure(&out.body));
    let (dv, pad) = (v1 - v0, pad0 + pad1);
    let scale = k.powi(3);
    assert!(
        pad < pad_ceiling() * scale && (dv - dv_want).abs() < 1e-12 * scale + pad,
        "{what}: ΔV {dv} ± {pad} vs the closed form {dv_want}"
    );
    assert!(
        (dv - dv_want).abs() < midpoint_tol() * scale,
        "{what}: ΔV's midpoint {dv} is off the closed form {dv_want}"
    );
}

/// The blind bore's depth and the pin's end, in units of the scale.
const DEPTH: f64 = 0.5;
const PIN_END: f64 = 0.5;

/// **A blind bore inside the convex sliver, and a pin inside the
/// concave fill, go at every scale under both verbs**:
/// `ΔV = −(A − π·a²/2)·k³`-scaled for the bore and
/// `ΔV = +(A/2 − π·a²/4)` for the pin, `A` and `a` at scale `k`.
#[test]
fn a_blind_bore_and_a_pin_go_with_the_band_at_every_scale() {
    let (d, a) = BAR;
    for k in [1e-2, 1.0, 1e2] {
        let what = format!("a blind bore at scale {k}");
        let body = blind_bore::<f64>(k, (d * k, d * k), a * k, DEPTH * k);
        let cups = cup_faces(&body, convex_at(k, (d * k, d * k)), a * k, (0.0, DEPTH * k));
        assert_eq!(cups.len(), 3, "{what}: two half-walls and a floor");
        for verb in VERBS {
            let dv = -(section(verb, D * k) * k - PI * (a * k).powi(2) * DEPTH * k);
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
        let what = format!("a pin at scale {k}");
        let body = pin::<f64>(k, 0.0, (d * k, d * k), a * k, PIN_END * k);
        let cups = cup_faces(
            &body,
            concave_at(k, (d * k, d * k)),
            a * k,
            (0.25 * k, PIN_END * k),
        );
        assert_eq!(cups.len(), 3, "{what}: two half-walls and an end");
        for verb in VERBS {
            let dv = section(verb, D * k) * 0.5 * k - PI * (a * k).powi(2) * 0.25 * k;
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

/// **The same bore and pin at the certified scalar** go too, each volume
/// enclosure bracketing its closed form.
#[test]
fn a_blind_bore_and_a_pin_go_with_the_band_at_interval() {
    let (d, a) = BAR;
    let body = blind_bore::<Interval>(1.0, (d, d), a, DEPTH);
    let cups = cup_faces(&body, convex_at(1.0, (d, d)), a, (0.0, DEPTH));
    assert_eq!(cups.len(), 3, "Interval: the bore's three faces");
    for verb in VERBS {
        let dv = -(section(verb, D) - PI * a * a * DEPTH);
        assert_taken(
            verb,
            &body,
            &[top(&body, 1.0)],
            D,
            (&cups, 1, 0),
            (dv, 1.0),
            "a blind bore at Interval",
        );
    }
    let body = pin::<Interval>(1.0, 0.0, (d, d), a, PIN_END);
    let cups = cup_faces(&body, concave_at(1.0, (d, d)), a, (0.25, PIN_END));
    assert_eq!(cups.len(), 3, "Interval: the pin's three faces");
    for verb in VERBS {
        let dv = section(verb, D) * 0.5 - PI * a * a * 0.25;
        assert_taken(
            verb,
            &body,
            &[floor(&body, 1.0)],
            D,
            (&cups, 1, 0),
            (dv, 1.0),
            "a pin at Interval",
        );
    }
}

/// **A pin from a leaning end wall goes** under both verbs: its mouth an
/// ellipse on `x = ¼ + s·(y − ¼)`, its axis `ℓ = ¼ − s·u` long between
/// the wall and its end, `ΔV = +(A/2 − π·a²·ℓ)`.
#[test]
fn a_pin_from_a_leaning_end_wall_goes_with_the_fill() {
    let (d, a) = BAR;
    for s in [0.3, 1.0, -1.0] {
        let what = format!("a pin at lean {s}");
        let body = pin::<f64>(1.0, s, (d, d), a, PIN_END);
        let cups = cup_faces(&body, concave_at(1.0, (d, d)), a, (0.0, PIN_END));
        assert_eq!(cups.len(), 3, "{what}: two half-walls and an end");
        for verb in VERBS {
            let dv = section(verb, D) * 0.5 - PI * a * a * (0.25 - s * d);
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

/// **Two cups, one from each end wall, both go**, and a square pin —
/// plane walls along the spine, each bounded by its mouth's edge, its
/// end's, and two lines between their vertices — goes too: the convex
/// block bored from `x = 0` to `0.3` and from `x = 1` back to `0.6`,
/// `ΔV = −(A − π·a²·0.7)`; the die with a pin of half-side `b` to `0.4`,
/// `ΔV = +(A/2 − 4b²·0.15)`.
#[test]
fn two_cups_and_a_square_pin_go_with_the_band() {
    let (d, a) = BAR;
    let c = convex_at(1.0, (d, d));
    let body = boolean(
        false,
        blind_bore::<f64>(1.0, (d, d), a, 0.3),
        rod((0.6, 1.2), c, a),
    );
    let mut cups = cup_faces(&body, c, a, (0.0, 0.3));
    cups.extend(cup_faces(&body, c, a, (0.6, 1.0)));
    assert_eq!(cups.len(), 6, "two bores of three faces each");
    for verb in VERBS {
        let dv = -(section(verb, D) - PI * a * a * 0.7);
        assert_taken(
            verb,
            &body,
            &[top(&body, 1.0)],
            D,
            (&cups, 2, 0),
            (dv, 1.0),
            "two blind bores",
        );
    }
    let b = 0.004;
    let at = concave_at(1.0, (0.016, 0.016));
    let body = boolean(
        true,
        die::<f64>(1.0, 0.0),
        bar(
            (0.1, at),
            (0.4, at),
            0.0,
            Section::Square(b),
            Vec3::new(0.0, 1.0, 0.0),
        ),
    );
    let cups = cup_faces(&body, at, b * 2f64.sqrt(), (0.25, 0.4));
    assert_eq!(cups.len(), 5, "the square pin's four walls and its end");
    for verb in VERBS {
        let dv = section(verb, D) * 0.5 - 4.0 * b * b * 0.15;
        assert_taken(
            verb,
            &body,
            &[floor(&body, 1.0)],
            D,
            (&cups, 1, 0),
            (dv, 1.0),
            "a square pin",
        );
    }
}

/// The leaning prism of `band_planar_consume`: its top front edge
/// `(0, 0, 1) → (2, 0, 1)` between walls `x = s·y` and `x = 2 + s·y`.
fn leaning(s: f64) -> Body<f64> {
    sweep::test_support::prism(
        vec![
            (geom_core::Point2::new(0.0, 0.0), 0.0),
            (geom_core::Point2::new(2.0, 0.0), 0.0),
            (geom_core::Point2::new(2.0 + s, 1.0), 0.0),
            (geom_core::Point2::new(s, 1.0), 0.0),
        ],
        1.0,
        tol(),
    )
}

/// **A blind bore from a leaning end wall goes** under both verbs: its
/// mouth an ellipse on `x = s·y`, its axis `1 − s·u` long between the
/// wall and its floor at `x = 1`, over a band of length `2`:
/// `ΔV = −(2A − π·a²·(1 − s·u))`.
#[test]
fn a_blind_bore_from_a_leaning_end_wall_goes_with_the_band() {
    let (d, a) = BAR;
    let c = (d, 1.0 - d);
    for s in [0.3, 1.0, -1.0] {
        let what = format!("a blind bore at lean {s}");
        let body = boolean(false, leaning(s), rod((-1.5, 1.0), c, a));
        let cups = cup_faces(&body, c, a, (-1.0, 1.0));
        assert_eq!(cups.len(), 3, "{what}: two half-walls and a floor");
        let front = edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
        for verb in VERBS {
            let dv = -(2.0 * section(verb, D) - PI * a * a * (1.0 - s * d));
            assert_taken(verb, &body, &[front], D, (&cups, 1, 0), (dv, 1.0), &what);
        }
    }
}

/// Which side of the fixture a hair row reads.
#[derive(Clone, Copy, Debug)]
enum Side {
    /// The convex block's blind bore.
    Bore,
    /// The concave die's pin.
    Pin,
}

impl Side {
    const BOTH: [Self; 2] = [Self::Bore, Self::Pin];

    fn convexity(self) -> Convexity {
        match self {
            Self::Bore => Convexity::Convex,
            Self::Pin => Convexity::Concave,
        }
    }

    /// The cup at corner coordinates `c`: a stem of radius `a` from the
    /// end wall to `x = 0.3`, and, given `head = Some(b)`, a head of
    /// radius `b` on the same axis over `x ∈ [0.3, 0.35]` — an undercut
    /// in the bore, a mushroom on the pin. Its body, crease, faces and
    /// `ΔV`, at scale one, under `verb`.
    fn cup(self, verb: Verb, c: (f64, f64), a: f64, head: Option<f64>) -> Cup {
        let held = PI
            * a
            * a
            * match self {
                Self::Bore => 0.3,
                Self::Pin => 0.05,
            }
            + head.map_or(0.0, |b| PI * b * b * 0.05);
        match self {
            Self::Bore => {
                let at = convex_at(1.0, c);
                let mut body = blind_bore::<f64>(1.0, c, a, 0.3);
                if let Some(b) = head {
                    body = boolean(false, body, rod((0.3, 0.35), at, b));
                }
                let cups = cup_faces(&body, at, head.unwrap_or(a), (0.0, 0.35));
                let crease = top(&body, 1.0);
                (body, crease, cups, -(section(verb, D) - held))
            }
            Self::Pin => {
                let at = concave_at(1.0, c);
                let mut body = pin::<f64>(1.0, 0.0, c, a, 0.3);
                if let Some(b) = head {
                    body = boolean(true, body, rod((0.3, 0.35), at, b));
                }
                let cups = cup_faces(&body, at, head.unwrap_or(a), (0.25, 0.35));
                let crease = floor(&body, 1.0);
                (body, crease, cups, section(verb, D) * 0.5 - held)
            }
        }
    }
}

/// A cup's body, crease, faces and closed-form `ΔV`.
type Cup = (Body<f64>, EdgeKey, Vec<FaceKey>, f64);

/// Corner coordinates of a disk of radius `b` whose far side lies
/// `dist` inside the band surface (negative: past it), on the fillet's
/// bisector or on the chamfer chord's normal through its midpoint.
fn inside_by(verb: Verb, dist: f64, b: f64) -> (f64, f64) {
    match verb {
        Verb::Fillet => past_the_arc(dist, b),
        Verb::Chamfer => past_the_chord(dist + b),
    }
}

/// The hair the rows below read either side of a boundary.
const HAIR: f64 = 1e-4;

/// **A cup whose mouth lies a hair across the sliver's edge refuses as a
/// measurement, and one a hair inside goes**, under both verbs and on
/// both sides: a mouth of radius `0.004` whose far side is `1e-4` past
/// the band surface, or centred on it, refuses `RingClearance` with
/// `bounded: false` at its end wall; one `1e-4` inside goes.
#[test]
fn a_cups_mouth_a_hair_across_the_sliver_crosses_and_one_inside_goes() {
    let a = 0.004;
    for side in Side::BOTH {
        for verb in VERBS {
            for (what, dist) in [("a hair past the band", -HAIR), ("on the band", -a)] {
                let (body, crease, ..) = side.cup(verb, inside_by(verb, dist, a), a, None);
                let got = blend(verb, &body, &[crease], D).map(|_| ());
                assert!(
                    matches!(
                        got,
                        Err(BlendError::RingClearance { bounded: false, chain, .. })
                            if chain == side.convexity()
                    ),
                    "{side:?} {verb:?}, {what}: a measured RingClearance at the end wall, \
                     got {got:?}"
                );
            }
            let (body, crease, cups, dv) = side.cup(verb, inside_by(verb, HAIR, a), a, None);
            assert_eq!(cups.len(), 3, "{side:?} {verb:?}: the cup's three faces");
            let what = format!("{side:?}, a mouth a hair inside");
            assert_taken(verb, &body, &[crease], D, (&cups, 1, 0), (dv, 1.0), &what);
        }
    }
}

/// **A cup whose head lies a hair across the band surface refuses as a
/// measurement, and one a hair inside goes**, under both verbs and on
/// both sides: a stem of radius `0.004` well inside the sliver, under a
/// head of radius `0.012` whose far side lies `1e-4` past the band
/// surface, refuses `FaceClearance` with `bounded: false` at the head's
/// wall; `1e-4` inside, the head is read inside the band's core and the
/// stem through its mouth and the head's edges, and the cup goes. The
/// head never meets the end wall, so only the core reading certifies
/// it.
#[test]
fn a_cups_head_a_hair_across_the_band_surface_crosses_and_one_inside_goes() {
    let (a, b) = (0.004, 0.012);
    for side in Side::BOTH {
        for verb in VERBS {
            let (body, crease, ..) = side.cup(verb, inside_by(verb, -HAIR, b), a, Some(b));
            let got = blend(verb, &body, &[crease], D).map(|_| ());
            assert!(
                matches!(
                    got,
                    Err(BlendError::FaceClearance { bounded: false, chain, .. })
                        if chain == side.convexity()
                ),
                "{side:?} {verb:?}: the head a hair past the band crosses, got {got:?}"
            );
            let (body, crease, cups, dv) = side.cup(verb, inside_by(verb, HAIR, b), a, Some(b));
            assert_eq!(
                cups.len(),
                6,
                "{side:?} {verb:?}: the stem's two half-walls, the shoulder, the head's \
                 two half-walls and its end"
            );
            let what = format!("{side:?}, a head a hair inside");
            assert_taken(verb, &body, &[crease], D, (&cups, 1, 0), (dv, 1.0), &what);
        }
    }
}

/// **A pin inside a concave ruled crease's fill refuses as a bound**: the
/// band's supports are a cylinder and a plane, whose reach has no exact
/// core, so the pin's end is never read inside and its wall, which meets
/// the end, is not certified through it
/// (`work/bandrule/a-ruled-bands-cup-has-no-core-to-read-its-far-faces-in.md`).
#[test]
fn a_pin_inside_a_concave_ruled_creases_fill_refuses_as_a_bound() {
    let (body, crease) = d_pocket(None, 0.002);
    let body = boolean(true, body, rod((0.1, 0.5), (0.297, -0.395), 0.002));
    let got = blend(
        Verb::Fillet,
        &body,
        &[crease],
        sweep::test_support::ROD_FILLET,
    )
    .map(|_| ());
    assert!(
        matches!(
            got,
            Err(BlendError::FaceClearance {
                bounded: true,
                chain: Convexity::Concave,
                ..
            })
        ),
        "the ruled pin refuses as a bound, got {got:?}"
    );
}

/// The convex block at scale one less a round bore of radius `a` whose
/// axis runs from corner coordinates `p` at the end face `x = 0` to `q`
/// at its square floor `x = h`, and its axis length `ℓ` there.
fn leaning_bore(p: (f64, f64), (h, q): (f64, (f64, f64)), a: f64) -> (Body<f64>, f64) {
    let t = -0.2 / h;
    let s = (p.0 + (q.0 - p.0) * t, p.1 + (q.1 - p.1) * t);
    let bore = bar(
        (-0.2, convex_at(1.0, s)),
        (h, convex_at(1.0, q)),
        0.0,
        Section::Round(a),
        Vec3::new(0.0, 1.0, 0.0),
    );
    let ell = (h * h + (q.0 - p.0).powi(2) + (q.1 - p.1).powi(2)).sqrt();
    (boolean(false, block(1.0), bore), ell)
}

/// Whether `ε` resolves the near-coincidences a hair-scale fixture is
/// built on; past `1e-9` the fixture's own Boolean refuses them as
/// undecided, so the row says so and asserts nothing.
pub(crate) fn resolves_hairs(what: &str) -> bool {
    let eps = tol().eps();
    let fine = eps <= 1e-9;
    if !fine {
        println!("SKIPPED at ε {eps:e}: {what}: its fixture's Boolean does not resolve its parts");
    }
    fine
}

/// `verb` refuses typed, as `want` reads it, and never from inside the
/// surgery.
fn assert_refuses(
    verb: Verb,
    body: &Body<f64>,
    edges: &[EdgeKey],
    want: impl Fn(&BlendError) -> bool,
    what: &str,
) {
    validate_geometric(body, tol()).unwrap_or_else(|e| panic!("{what}: source tier 3, {e:?}"));
    match blend(verb, body, edges, D) {
        Err(e) => assert!(want(&e), "{what} ({verb:?}): refused, but {e:?}"),
        Ok(_) => panic!("{what} ({verb:?}): refuses, but built"),
    }
}

fn bounded(e: &BlendError) -> bool {
    matches!(e, BlendError::FaceClearance { bounded: true, .. })
}

fn crossing(e: &BlendError) -> bool {
    matches!(e, BlendError::FaceClearance { bounded: false, .. })
}

fn wholly_inside(e: &BlendError) -> bool {
    matches!(e, BlendError::WhollyInside { .. })
}

/// **A cup whose axis leans off the spine goes where the reading's
/// terms allow**: a bore from `(0.012, 0.012)` at `x = 0` to its floor
/// at `x = ½`, its axis turned `σ` in each corner coordinate, holds
/// `π·a²·ℓ` (`ℓ` its axis), so `ΔV = −(A − π·a²·ℓ)`. At `σ = 0.0011` it
/// goes under both verbs: its floor circle reads at its depth, its
/// pieces halved until each box's radius is within an eighth of it. At
/// `σ = 0.0031` the fillet charges the walls their box's diagonal times
/// the lean, which outweighs the depth, and refuses as a bound; the
/// chamfer's triangle, convex, owes no such term and takes it.
#[test]
fn a_leaning_blind_bore_goes_where_its_lean_is_paid() {
    if !resolves_hairs("a leaning blind bore") {
        return;
    }
    let (d, a) = BAR;
    for (shift, fillet) in [(0.0011, true), (0.0031, false)] {
        let (body, ell) = leaning_bore((d, d), (0.5, (d + shift, d - shift)), a);
        let cups = cup_faces(&body, convex_at(1.0, (d, d)), a + 2.0 * shift, (0.0, 0.51));
        assert_eq!(cups.len(), 3, "lean {shift}: two half-walls and a floor");
        let what = format!("a bore leaning {shift}");
        for verb in VERBS {
            if matches!(verb, Verb::Fillet) && !fillet {
                assert_refuses(verb, &body, &[top(&body, 1.0)], bounded, &what);
                continue;
            }
            let dv = -(section(verb, D) - PI * a * a * ell);
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

/// **A leaning cup whose axis dips through the fillet's disk crosses,
/// and the chamfer's triangle takes it**: a bore of radius `0.003` from
/// `(0.005, 0.055)` at `x = 0` to `(0.055, 0.005)` at `x = ½` lies in
/// the fillet's sliver at both ends but crosses its arc midway, and
/// refuses as a measurement; under the chamfer (chord `u + v = 0.1`) it
/// goes at `ΔV = −(A − π·a²·ℓ)`. A pin of radius `0.002` across the
/// fill along `u + v = 0.06` (outside the fillet's arc midway) does the
/// same, `ΔV = +(A/2 − π·a²·ℓ)`; one along `u + v = 0.054`, inside
/// both, refuses as a bound under the fillet (its 26° lean is charged)
/// and goes under the chamfer.
#[test]
fn a_cup_leaning_across_the_fillets_disk_crosses_and_the_chamfer_takes_it() {
    let a = 0.003;
    let (body, ell) = leaning_bore((0.005, 0.055), (0.5, (0.055, 0.005)), a);
    let cups = cup_faces(&body, convex_at(1.0, (0.03, 0.03)), 0.04, (0.0, 0.51));
    assert_eq!(cups.len(), 3, "the leaning bore's two half-walls and floor");
    let crease = top(&body, 1.0);
    assert_refuses(
        Verb::Fillet,
        &body,
        &[crease],
        crossing,
        "a bore across the disk",
    );
    let dv = -(section(Verb::Chamfer, D) - PI * a * a * ell);
    assert_taken(
        Verb::Chamfer,
        &body,
        &[crease],
        D,
        (&cups, 1, 0),
        (dv, 1.0),
        "a bore across the disk",
    );
    let a = 0.002;
    for (c, fillet) in [
        (0.056, crossing as fn(&BlendError) -> bool),
        (0.05, bounded),
    ] {
        let (p, q) = ((0.004, c), (c, 0.004));
        let leg = |x: f64, (u, v): (f64, f64)| (x, concave_at(1.0, (u, v)));
        let t = (0.15 - 0.25) / 0.15;
        let s = (p.0 + (q.0 - p.0) * t, p.1 + (q.1 - p.1) * t);
        let body = boolean(
            true,
            die::<f64>(1.0, 0.0),
            bar(
                leg(0.15, s),
                leg(0.4, q),
                0.0,
                Section::Round(a),
                Vec3::new(0.0, 1.0, 0.0),
            ),
        );
        let ell = (0.15f64.powi(2) + 2.0 * (q.0 - p.0).powi(2)).sqrt();
        let what = format!("a pin leaning along u + v = {}", c + 0.004);
        let crease = floor(&body, 1.0);
        assert_refuses(Verb::Fillet, &body, &[crease], fillet, &what);
        let mid = concave_at(1.0, (0.5 * (p.0 + q.0), 0.5 * (p.1 + q.1)));
        let cups = cup_faces(&body, mid, 0.04, (0.25, 0.41));
        assert_eq!(cups.len(), 3, "{what}: two half-walls and an end");
        let dv = section(Verb::Chamfer, D) * 0.5 - PI * a * a * ell;
        assert_taken(
            Verb::Chamfer,
            &body,
            &[crease],
            D,
            (&cups, 1, 0),
            (dv, 1.0),
            &what,
        );
    }
}

/// **A cup with a handle, and a feature with two mouths on one cap,
/// refuse `WhollyInside`**: the rule takes them (each lies inside the
/// region, which goes whole), but `plan` admits only a disk on one cap
/// or an annulus between two
/// (`work/bandrule/a-feature-inside-a-region-that-is-no-disk-or-annulus-is-not-carved.md`).
/// - A square blind bore of half-side `0.004` with a square cross bar
///   spanning it at `x = 0.2`: one mouth, genus 1, so
///   `V − E + F − R = −1`, under both verbs.
/// - A square pin of half-side `0.006` with a square hole across it at
///   `x = 0.4`: the chamfer reads it whole and refuses; under the
///   fillet its mouth crosses the arc (`RingClearance`, measured).
/// - Two square bores from one end face meeting at `x = 0.4` (a U-tube,
///   two mouths on one cap), under the chamfer; the fillet's arc
///   crosses their mouths.
#[test]
fn a_cup_with_a_handle_and_a_u_tube_refuse_wholly_inside() {
    if !resolves_hairs("a cup with a handle and a U-tube") {
        return;
    }
    let (d, h, b, c) = (0.012, 0.4, 0.004, 0.0015);
    let (y, z) = convex_at(1.0, (d, d));
    let bored = boolean(
        false,
        block::<f64>(1.0),
        brick((-0.2, h), (y - b, y + b), (z - b, z + b), tol()),
    );
    let body = boolean(
        true,
        bored,
        brick(
            (0.2 - c, 0.2 + c),
            (y - 0.006, y + 0.006),
            (z - c, z + c),
            tol(),
        ),
    );
    assert_eq!(
        topo::readback::euler_counts(&body)
            .genus()
            .expect("a genus"),
        1,
        "the bore carries a handle"
    );
    for verb in VERBS {
        assert_refuses(
            verb,
            &body,
            &[top(&body, 1.0)],
            wholly_inside,
            "a cup with a handle",
        );
    }
    let (y, z) = concave_at(1.0, (0.025, 0.025));
    let pinned = boolean(
        true,
        die::<f64>(1.0, 0.0),
        bar(
            (0.1, (y, z)),
            (0.5, (y, z)),
            0.0,
            Section::Square(0.006),
            Vec3::new(0.0, 1.0, 0.0),
        ),
    );
    let body = boolean(
        false,
        pinned,
        brick(
            (0.398, 0.402),
            (y - 0.012, y + 0.012),
            (z - 0.002, z + 0.002),
            tol(),
        ),
    );
    assert_eq!(
        topo::readback::euler_counts(&body)
            .genus()
            .expect("a genus"),
        1,
        "the pin carries a handle"
    );
    let crease = floor(&body, 1.0);
    assert_refuses(
        Verb::Chamfer,
        &body,
        &[crease],
        wholly_inside,
        "a pin with a cross hole",
    );
    assert_refuses(
        Verb::Fillet,
        &body,
        &[crease],
        |e| matches!(e, BlendError::RingClearance { bounded: false, .. }),
        "a pin with a cross hole",
    );
    let meet = (0.4, (0.03, 0.03));
    let leg = |from: (f64, f64)| {
        let t = -0.2 / 0.4;
        let s = (
            from.0 + (meet.1.0 - from.0) * t,
            from.1 + (meet.1.1 - from.1) * t,
        );
        bar(
            (-0.2, convex_at(1.0, s)),
            (meet.0, convex_at(1.0, meet.1)),
            0.0,
            Section::Square(0.003),
            Vec3::new(0.0, 1.0, 0.0),
        )
    };
    let body = boolean(
        false,
        boolean(false, block::<f64>(1.0), leg((0.015, 0.045))),
        leg((0.045, 0.015)),
    );
    assert_refuses(
        Verb::Chamfer,
        &body,
        &[top(&body, 1.0)],
        wholly_inside,
        "a U-tube",
    );
}

/// **Cups beside other features, and under other bands, go**:
/// - a blind bore of radius `0.004` to `0.4` beside a through bore of
///   radius `0.003` in one sliver — one disk and one annulus in one
///   request — `ΔV = −(A − π·a²·0.4 − π·b²)`;
/// - a blind bore to `½` under each of two bands in one request (the
///   block's top edges at `y = 1` and `y = 0`), each read in its own
///   band, `ΔV = −2·(A − π·a²/2)`;
/// - a blind bore whose floor stops `1e-4` short of the far end face,
///   `ΔV = −(A − π·a²·(1 − 10⁻⁴))`, and one `1e-4` past it, a through
///   bore, `ΔV = −(A − π·a²)`;
/// - a blind bore from the far end of a mitre's band (the top edges at
///   `y = 1` and `x = 0`), `ΔV` the plain mitre's plus `π·a²·0.3`. One
///   from the mitre's own end refuses at the screen: its mouth lies on
///   the other band's strip
///   (`work/band/a-cup-whose-mouth-lies-on-a-supports-strip-is-not-carved.md`).
#[test]
fn cups_beside_a_bore_under_two_bands_and_by_a_mitre_go() {
    let (d, a) = BAR;
    let (p, q) = (convex_at(1.0, (0.012, 0.03)), convex_at(1.0, (0.03, 0.012)));
    let (a1, b1) = (0.004, 0.003);
    let body = boolean(
        false,
        boolean(false, block::<f64>(1.0), rod((-0.2, 0.4), p, a1)),
        rod((-0.2, 1.2), q, b1),
    );
    let mut cups = cup_faces(&body, p, a1, (0.0, 0.4));
    cups.extend(cup_faces(&body, q, b1, (0.0, 1.0)));
    assert_eq!(cups.len(), 5, "the cup's three faces and the bore's two");
    for verb in VERBS {
        let dv = -(section(verb, D) - PI * a1 * a1 * 0.4 - PI * b1 * b1);
        let what = "a cup beside a through bore";
        assert_taken(
            verb,
            &body,
            &[top(&body, 1.0)],
            D,
            (&cups, 3, 1),
            (dv, 1.0),
            what,
        );
    }
    let near = convex_at(1.0, (d, d));
    let far = (d, 1.0 - d);
    let body = boolean(
        false,
        blind_bore::<f64>(1.0, (d, d), a, 0.5),
        rod((-0.2, 0.5), far, a),
    );
    let mut cups = cup_faces(&body, near, a, (0.0, 0.5));
    cups.extend(cup_faces(&body, far, a, (0.0, 0.5)));
    let edges = [
        top(&body, 1.0),
        edge(&body, [0.0, 0.0, 1.0], [1.0, 0.0, 1.0]),
    ];
    for verb in VERBS {
        let dv = -2.0 * (section(verb, D) - PI * a * a * 0.5);
        assert_taken(
            verb,
            &body,
            &edges,
            D,
            (&cups, 2, 0),
            (dv, 1.0),
            "a cup under each of two bands",
        );
    }
    for (end, mouths, through) in [(1.0 - 1e-4, 1, 0), (1.0 + 1e-4, 2, 1)] {
        let body = blind_bore::<f64>(1.0, (d, d), a, end);
        let cups = cup_faces(&body, near, a, (0.0, end.min(1.0)));
        let what = format!("a bore to {end}");
        for verb in VERBS {
            let dv = -(section(verb, D) - PI * a * a * end.min(1.0));
            assert_taken(
                verb,
                &body,
                &[top(&body, 1.0)],
                D,
                (&cups, mouths, through),
                (dv, 1.0),
                &what,
            );
        }
    }
    let mitre = |b: &Body<f64>| [top(b, 1.0), edge(b, [0.0, 0.0, 1.0], [0.0, 1.0, 1.0])];
    let plain = block::<f64>(1.0);
    let from_far = boolean(false, block::<f64>(1.0), rod((0.7, 1.2), near, a));
    let from_near = blind_bore::<f64>(1.0, (d, d), a, 0.3);
    let cups = cup_faces(&from_far, near, a, (0.7, 1.0));
    for verb in VERBS {
        let out = blend(verb, &plain, &mitre(&plain), D).expect("the plain mitre builds");
        let (v0, v1) = (volume_enclosure(&plain).0, volume_enclosure(&out.body).0);
        let dv = v1 - v0 + PI * a * a * 0.3;
        let what = "a cup on a mitred band's far end";
        assert_taken(
            verb,
            &from_far,
            &mitre(&from_far),
            D,
            (&cups, 1, 0),
            (dv, 1.0),
            what,
        );
        assert_refuses(
            verb,
            &from_near,
            &mitre(&from_near),
            |e| matches!(e, BlendError::FaceClearanceUncertified { .. }),
            "a cup on a mitred band's own end",
        );
    }
}

/// **A bore that breaks through a support is no cup**: a bore of radius
/// `0.006` `0.004` from the support `z = 1` opens a slot in it, and
/// refuses at the screen.
#[test]
fn a_bore_breaking_through_a_support_refuses_at_the_screen() {
    let body = blind_bore::<f64>(1.0, (0.004, 0.02), 0.006, 0.5);
    for verb in VERBS {
        assert_refuses(
            verb,
            &body,
            &[top(&body, 1.0)],
            |e| matches!(e, BlendError::FaceClearanceUncertified { .. }),
            "a slot",
        );
    }
}

/// **A cup that crosses the band surface between the places its
/// boundary is sampled at still refuses**, as a bound: the crossing is
/// not measured there, and neither the curve's pieces nor the lines'
/// slack certify it inside. A head crossing the fillet's arc in the
/// convex block:
/// - round, of radius `0.012` over `x ∈ [0.3, 0.35]` on a stem of
///   radius `0.004`, its far side `1e-4` past the arc and turned
///   `5π/16` from its seam, midway between two of its circles'
///   samples: each piece of a circle is read over a box reaching
///   `σ·w/2` from its middle point, and the piece over the crossing
///   never certifies;
/// - a diamond (a square of half-side `0.012` turned 45°) over
///   `x ∈ [0.2, 0.25]`, its axis leaning `2·10⁻⁴` so no wall is parallel
///   to the spine, the far edge of its end dipping `2·10⁻⁶` past the arc
///   midway between two samples: its ends are certified, and the line
///   between them is charged its run across the spine.
#[test]
fn a_head_crossing_between_its_boundary_samples_refuses_as_a_bound() {
    if !resolves_hairs("a head crossing between its samples") {
        return;
    }
    let (a, b) = (0.004, 0.012);
    let (u, v) = inside_by(Verb::Fillet, -HAIR, b);
    let at = convex_at(1.0, (u, v));
    let turn = -0.75 * PI - 5.0 * PI / 16.0;
    let body = boolean(
        false,
        boolean(false, block::<f64>(1.0), rod((-0.2, 0.3), at, a)),
        bar(
            (0.3, at),
            (0.35, at),
            0.0,
            Section::Round(b),
            Vec3::new(0.0, turn.cos(), turn.sin()),
        ),
    );
    assert_refuses(
        Verb::Fillet,
        &body,
        &[top(&body, 1.0)],
        bounded,
        "a round head",
    );
    let s2 = 2f64.sqrt();
    let c = 2.0 * (D - (D - 2e-6) / s2);
    let mid = (c / 2.0 + b / (8.0 * s2), c / 2.0 - b / (8.0 * s2));
    let end = (mid.0 - b / s2, mid.1 - b / s2);
    let start = (end.0 - 2e-4, end.1 - 1e-4);
    let body = boolean(
        false,
        boolean(
            false,
            block::<f64>(1.0),
            rod((-0.2, 0.22), convex_at(1.0, start), 0.003),
        ),
        bar(
            (0.2, convex_at(1.0, start)),
            (0.25, convex_at(1.0, end)),
            0.0,
            Section::Square(b),
            Vec3::new(0.0, 1.0, 1.0),
        ),
    );
    assert_refuses(
        Verb::Fillet,
        &body,
        &[top(&body, 1.0)],
        bounded,
        "a diamond head",
    );
}
