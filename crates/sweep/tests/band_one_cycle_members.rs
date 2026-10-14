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
fn cup_faces<T: Real + Bounds>(
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
/// and retired; the shell count and genus hold (a cup is no handle) and
/// each of the `mouths` leaves its end wall; and the volume moves by
/// `dv` to `1e-12` of the scale `k` cubed, the enclosure a claim and its
/// midpoint on the closed form.
#[allow(clippy::too_many_arguments)] // the row's own reads, one per argument.
fn assert_taken<T: Decide + Bounds + AtRestPolicy + geom_core::CertifiedBounds>(
    verb: Verb,
    body: &Body<T>,
    edges: &[EdgeKey],
    r: f64,
    (cups, mouths): (&[FaceKey], usize),
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
        (c1.s, c1.genus()),
        (c0.s, c0.genus()),
        "{what}: the shells and their genus hold"
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
                (&cups, 1),
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
                (&cups, 1),
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
            (&cups, 1),
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
            (&cups, 1),
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
                (&cups, 1),
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
            (&cups, 2),
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
            (&cups, 1),
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
            assert_taken(verb, &body, &[front], D, (&cups, 1), (dv, 1.0), &what);
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
            assert_taken(verb, &body, &[crease], D, (&cups, 1), (dv, 1.0), &what);
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
            assert_taken(verb, &body, &[crease], D, (&cups, 1), (dv, 1.0), &what);
        }
    }
}

/// **A pin inside a concave ruled crease's fill refuses as a bound**: the
/// band's supports are a cylinder and a plane, whose reach has no exact
/// core, so the pin's end is never read inside and its wall, which meets
/// the end, is not certified through it
/// (`work/band/a-ruled-bands-cup-has-no-core-to-read-its-far-faces-in.md`).
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
