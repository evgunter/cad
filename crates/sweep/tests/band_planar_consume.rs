//! **A feature wholly inside the material a plane–plane band removes
//! goes with it** (Ev's bore ruling, PR 4285), under both verbs. A
//! convex band cut off at its two end faces removes the prism over its
//! section between their planes; a through-feature whose rings lie
//! inside each end face's sliver (the fillet's between its arc and the
//! old vertex, the chamfer's triangle under its chord) and whose walls
//! lie inside that prism goes with the material: its faces, edges and
//! vertices leave the body and its rings leave the end faces.
//!
//! Two fixtures. The SQUARE block `[0, k]² × [0, k]`, extruded along
//! `z`, its edge at `(k, k)` blended at `D·k`; and the LEANING prism,
//! whose top front edge `(0, 0, 1) → (2, 0, 1)` ends at walls `x = s·y`
//! and `x = 2 + s·y`, bored along `x` by a Boolean. The band removes its
//! section `A` (`(1 − π/4)·r²` for the fillet, `r²/2` for the chamfer)
//! over the edge's length `L`, and a bore of radius `a` along the edge
//! between two parallel planes `L` apart along it held `π·a²·L` of air,
//! so `ΔV = −(A − π·a²)·L` at every lean.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::common::interval::iv;
use geom_core::{Bounds, Interval, Point2, Point3, Real, Tol, Vec3};
use profile::{ProfileLoop, SketchPlane, test_support::bulge_loop};
use sweep::blend::build::{Blended, chamfer_edges, fillet_edges};
use sweep::blend::{BlendError, Convexity};
use sweep::test_support::{
    assert_naming_totality, extruded, finished, prism, prism_on, realized, sketch_from_axes,
};
use topo::boolean::BooleanOp;
use topo::{
    AtRestPolicy, Body, ContactRecords, EdgeKey, FaceKey, mass_properties, query,
    validate_geometric, validate_pseudomanifold,
};

use crate::band_planar_cut_off::{D, Verb, edge, midpoint_tol, pad_ceiling, tol, volume_enclosure};

/// The section `verb` at size `r` removes from a right dihedral.
fn section(verb: Verb, r: f64) -> f64 {
    match verb {
        Verb::Fillet => (1.0 - PI / 4.0) * r * r,
        Verb::Chamfer => r * r / 2.0,
    }
}

/// `verb` at size `r` on `edges`, at any scalar.
fn blend<T: Decide + Bounds + AtRestPolicy>(
    verb: Verb,
    body: &topo::AtRestBody<T>,
    edges: &[EdgeKey],
    r: T,
) -> Result<Blended<T>, BlendError> {
    match verb {
        Verb::Fillet => fillet_edges(body, edges, r, tol()),
        Verb::Chamfer => chamfer_edges(body, edges, r, tol()),
    }
    .map_err(|e| e.error)
}

const VERBS: [Verb; 2] = [Verb::Fillet, Verb::Chamfer];

/// A closed polygonal profile loop through `pts`.
fn poly<T: Real>(pts: &[(f64, f64)]) -> ProfileLoop<T> {
    let f = T::from_f64;
    bulge_loop(
        pts.iter()
            .map(|&(x, y)| (Point2::new(f(x), f(y)), f(0.0)))
            .collect(),
    )
}

/// A round profile loop of radius `a` about `(x, y)`: two semicircles.
fn round<T: Real>(x: f64, y: f64, a: f64) -> ProfileLoop<T> {
    let f = T::from_f64;
    bulge_loop(vec![
        (Point2::new(f(x + a), f(y)), f(1.0)),
        (Point2::new(f(x - a), f(y)), f(1.0)),
    ])
}

/// The square block at scale `k` with the profile `hole` (in unit
/// coordinates, scaled by `k`) through it along `z`.
fn square<T: Decide + AtRestPolicy>(k: f64, hole: ProfileLoop<T>) -> topo::AtRestBody<T> {
    let outline = poly::<T>(&[(0.0, 0.0), (k, 0.0), (k, k), (0.0, k)]);
    finished(
        "square",
        extruded(
            SketchPlane::xy(),
            vec![outline, hole],
            T::from_f64(k),
            tol(),
        ),
        tol(),
    )
}

use geom_core::Decide;

fn mid<T: Bounds>(x: T) -> f64 {
    0.5 * (x.lo() + x.hi())
}

/// The square block's edge at `(k, k)`, along `z`.
fn crease<T: Real + Bounds>(body: &Body<T>, k: f64) -> EdgeKey {
    let at = |p: Point3<T>| (mid(p.x) - k).abs() < 1e-9 * k && (mid(p.y) - k).abs() < 1e-9 * k;
    query::all_edges(body)
        .into_iter()
        .find(|&e| {
            let he = body.get_edge(e).expect("an edge").he_plus;
            let p = |v| {
                *body
                    .get_point(body.get_vertex(v).expect("v").point)
                    .expect("p")
            };
            at(p(body.get_half_edge(he).expect("a half").start))
                && at(p(body.half_edge_end(he).expect("an end")))
        })
        .expect("the block's edge at (k, k)")
}

/// The faces of `body` whose every vertex `p` has `sel(p)`: a
/// feature's faces, picked by where they lie.
fn faces_where<T: Real + Bounds>(
    body: &Body<T>,
    sel: impl Fn(Point3<f64>) -> bool,
) -> Vec<FaceKey> {
    let at = |v: topo::VertexKey| {
        let p = *body
            .get_point(body.get_vertex(v).expect("v").point)
            .expect("p");
        Point3::new(mid(p.x), mid(p.y), mid(p.z))
    };
    body.faces()
        .filter(|(_, f)| {
            core::iter::once(f.outer)
                .chain(f.rings.iter().copied())
                .all(|lp| match body.get_loop(lp).expect("a loop").boundary {
                    topo::LoopBoundary::Empty { vertex } => sel(at(vertex)),
                    topo::LoopBoundary::Cycle { first } => body
                        .loop_cycle(first)
                        .expect("a cycle")
                        .into_iter()
                        .all(|he| sel(at(body.get_half_edge(he).expect("a half").start))),
                })
        })
        .map(|(k, _)| k)
        .collect()
}

/// **The feature goes with the band's material**: `verb` at `r` on
/// `edges` builds at tiers 3 and 3′ with naming totality, the faces `walls`
/// are gone and retired, no face keeps a ring, one genus-0 shell is
/// left, and the body's volume drops by `removed`, to `1e-12` of the
/// fixture's scale `k` cubed.
fn assert_consumed(
    verb: Verb,
    body: &Body<f64>,
    edges: &[EdgeKey],
    r: f64,
    walls: &[FaceKey],
    (removed, k): (f64, f64),
    what: &str,
) {
    validate_geometric(body, tol()).unwrap_or_else(|e| panic!("{what}: source tier 3, {e:?}"));
    assert!(!walls.is_empty(), "{what}: the source carries the feature");
    let at_rest = sweep::test_support::at_rest(body, tol());
    let what = format!("{what} ({verb:?})");
    let what = what.as_str();
    let out = blend(verb, &at_rest, edges, r)
        .unwrap_or_else(|e| panic!("{what}: the feature goes with the band, got {e}"));
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, {e:?}"));
    validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
        .unwrap_or_else(|e| panic!("{what}: tier 3′, {e:?}"));
    assert_naming_totality(body, &out, edges, what);
    let rec = out.naming.as_ref().expect("the surgery records its births");
    for w in walls {
        assert!(
            out.body.get_face(*w).is_none() && rec.dead.faces.contains(w),
            "{what}: the feature's face {w:?} is consumed and retired"
        );
    }
    assert!(
        out.body.faces().all(|(_, f)| f.rings.is_empty()),
        "{what}: no face keeps a ring"
    );
    let c = topo::readback::euler_counts(&out.body);
    assert_eq!(
        (c.s, c.genus()),
        (1, Ok(0)),
        "{what}: one genus-0 shell, the through-hole gone"
    );
    let ((v0, pad0), (v1, pad1)) = (volume_enclosure(body), volume_enclosure(&out.body));
    let (dv, pad) = (v0 - v1, pad0 + pad1);
    let scale = k.powi(3);
    assert!(
        pad < pad_ceiling() * scale && (dv - removed).abs() < 1e-12 * scale + pad,
        "{what}: ΔV {dv} ± {pad} vs the closed form {removed}"
    );
    assert!(
        (dv - removed).abs() < midpoint_tol() * scale,
        "{what}: ΔV's midpoint {dv} is off the closed form {removed}"
    );
}

/// The bore's centre `(1 − d, 1 − d)` and radius in the square's unit
/// coordinates: `0.006` off each support, `≈ 0.0184` inside the arc
/// (`(0.1 − d)·√2 − 0.1 − a`) and `≈ 0.048` inside the chamfer's chord
/// (`(0.1 − 2d)/√2 − a`).
const BORE: (f64, f64) = (0.012, 0.006);

/// **A bore wholly inside the square block's sliver goes with it, at
/// every scale and under both verbs**: `ΔV = −(A − π·a²)·k`, `A` and
/// `a` scaled by `k`.
#[test]
fn a_bore_inside_the_planar_bands_sliver_goes_with_it_at_every_scale() {
    let (d, a) = BORE;
    for (verb, k) in VERBS
        .into_iter()
        .flat_map(|v| [1e-2, 1.0, 1e2].map(|k| (v, k)))
    {
        let what = format!("bore at scale {k}");
        let body = square::<f64>(k, round((1.0 - d) * k, (1.0 - d) * k, a * k));
        let walls = faces_where(&body, |p| {
            p.x.min(p.y) > (1.0 - d - a) * k - 1e-9 * k
                && p.x.max(p.y) < (1.0 - d + a) * k + 1e-9 * k
        });
        assert_eq!(walls.len(), 2, "{what}: the bore's two half-walls");
        let removed = (section(verb, D * k) - PI * (a * k).powi(2)) * k;
        assert_consumed(
            verb,
            &body,
            &[crease(&body, k)],
            D * k,
            &walls,
            (removed, k),
            &what,
        );
    }
}

/// **A square through-slot wholly inside the sliver goes with it**: its
/// four plane walls lie along the spine and its rings are squares
/// `0.006` off each support, their nearest corner `≈ 0.016` past the
/// arc and `≈ 0.044` inside the chamfer's chord. `ΔV = −(A − w²)·1`.
#[test]
fn a_square_slot_inside_the_planar_bands_sliver_goes_with_it() {
    let (lo, hi) = (0.982, 0.994);
    let body = square::<f64>(1.0, poly(&[(lo, lo), (hi, lo), (hi, hi), (lo, hi)]));
    let walls = faces_where(&body, |p| {
        p.x.min(p.y) > lo - 1e-9 && p.x.max(p.y) < hi + 1e-9
    });
    assert_eq!(walls.len(), 4, "the slot's four walls");
    for verb in VERBS {
        let removed = section(verb, D) - (hi - lo) * (hi - lo);
        assert_consumed(
            verb,
            &body,
            &[crease(&body, 1.0)],
            D,
            &walls,
            (removed, 1.0),
            "square slot",
        );
    }
}

/// **The bore at the certified scalar** goes with the sliver too, under
/// both verbs: tiers 3 and 3′, both half-walls retired, no face keeping
/// a ring, the census of the carve of the block whose hole is clear of
/// the band, and `ΔV` bracketing the closed form.
#[test]
fn a_bore_inside_the_planar_bands_sliver_goes_with_it_at_interval() {
    let (d, a) = BORE;
    let tol: Tol = tol();
    let body = square::<Interval>(1.0, round(1.0 - d, 1.0 - d, a));
    let walls = curved_walls(&body);
    assert_eq!(walls.len(), 2, "Interval: the bore's two half-walls");
    let plain_body =
        square::<Interval>(1.0, poly(&[(0.4, 0.4), (0.6, 0.4), (0.6, 0.6), (0.4, 0.6)]));
    let census = |b: &Body<Interval>| (b.faces().count(), b.edges().count(), b.vertices().count());
    for verb in VERBS {
        let out = blend(verb, &body, &[crease(&body, 1.0)], iv(D))
            .unwrap_or_else(|e| panic!("{verb:?}: the bore goes at Interval, got {e}"));
        validate_geometric(&out.body, tol)
            .unwrap_or_else(|e| panic!("{verb:?} at Interval: tier 3, {e:?}"));
        validate_pseudomanifold(&out.body, &ContactRecords::default(), tol)
            .unwrap_or_else(|e| panic!("{verb:?} at Interval: tier 3′, {e:?}"));
        let rec = out.naming.as_ref().expect("the surgery records its births");
        assert!(
            walls
                .iter()
                .all(|w| out.body.get_face(*w).is_none() && rec.dead.faces.contains(w)),
            "{verb:?} at Interval: the bore's walls go and are retired"
        );
        assert!(
            out.body.faces().all(|(_, f)| f.rings.is_empty()),
            "{verb:?} at Interval: no face keeps a ring"
        );
        let plain = blend(verb, &plain_body, &[crease(&plain_body, 1.0)], iv(D))
            .unwrap_or_else(|e| panic!("{verb:?}: the block with a clear slot carves, got {e}"));
        let (f, e, v) = census(&plain.body);
        assert_eq!(
            census(&out.body),
            (f - 4, e - 12, v - 8),
            "{verb:?} at Interval: the bored block's carve is that of the block with no hole"
        );
        let v0 = mass_properties(&body, tol).expect("interval props").volume;
        let v1 = mass_properties(&out.body, tol)
            .expect("interval props")
            .volume;
        let want = section(verb, D) - PI * a * a;
        let dv = v0 - v1;
        assert!(
            dv.lo() - 1e-12 <= want && want <= dv.hi() + 1e-12 && dv.hi() - dv.lo() < 1e-6,
            "{verb:?} at Interval: ΔV {dv:?} brackets {want}"
        );
    }
}

/// The faces of `body` on no plane: a bore's walls.
fn curved_walls<T: Real>(body: &Body<T>) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            !matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Plane { .. })
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// The prism whose top front edge runs `(0, 0, 1) → (2, 0, 1)` between
/// two walls leaning `s` along `x` per unit of `y`.
fn leaning(s: f64) -> Body<f64> {
    prism(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.0),
            (Point2::new(2.0 + s, 1.0), 0.0),
            (Point2::new(s, 1.0), 0.0),
        ],
        1.0,
        tol(),
    )
}

/// [`leaning`] less a round rod of radius `a` whose axis runs from
/// `(0, p)` to `(2, q)` in `(x, (y, z))`, past both walls, its profile's
/// diameter (and so its two seams) along `u` turned square to the axis.
fn bored(s: f64, p: (f64, f64), q: (f64, f64), a: f64, u: Vec3<f64>) -> topo::AtRestBody<f64> {
    let t = Vec3::new(2.0, q.0 - p.0, q.1 - p.1);
    let th = t.normalize();
    let over = 1.0 + s.abs();
    let o = Point3::new(0.0, p.0, p.1) - th * (over / th.x);
    let u = (u - th * u.dot(th)).normalize();
    let pin = prism_on(
        sketch_from_axes(o, u, th.cross(u), tol()),
        vec![(Point2::new(-a, 0.0), 1.0), (Point2::new(a, 0.0), 1.0)],
        t.norm() + 2.0 * over / th.x,
        tol(),
    );
    finished(
        "bored",
        realized(BooleanOp::Subtract, &leaning(s), &pin, tol()),
        tol(),
    )
}

fn front(body: &Body<f64>) -> EdgeKey {
    edge(body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0])
}

/// **A bore along the edge goes with the sliver at every lean of its
/// end walls, under both verbs**: each ring is an ellipse on a wall
/// `x = s·y`, read inside that wall's sliver, whose projection along the
/// spine is the section's by `|n·τ| = 1/√(1 + s²)`.
/// `ΔV = −(A − π·a²)·2`.
#[test]
fn a_bore_inside_the_planar_bands_sliver_goes_with_it_at_every_lean() {
    let (d, a) = BORE;
    let at = (d, 1.0 - d);
    for s in [0.0, 0.3, 1.0, -1.0] {
        let what = format!("bore at lean {s}");
        let body = bored(s, at, at, a, Vec3::new(0.0, 1.0, 0.0));
        let walls = curved_walls(&body);
        assert_eq!(walls.len(), 2, "{what}: the bore's two half-walls");
        for verb in VERBS {
            let removed = (section(verb, D) - PI * a * a) * 2.0;
            assert_consumed(
                verb,
                &body,
                &[front(&body)],
                D,
                &walls,
                (removed, 1.0),
                &what,
            );
        }
    }
}

/// The refusal `verb` on the square block's edge owes, at unit scale.
fn refusal(verb: Verb, body: &Body<f64>) -> BlendError {
    let at_rest = sweep::test_support::at_rest(body, tol());
    match blend(verb, &at_rest, &[crease(body, 1.0)], D) {
        Err(e) => e,
        Ok(out) => panic!(
            "expected a refusal, the carve built ({} faces)",
            out.body.faces().count()
        ),
    }
}

/// The square block's ball centre `c` and the unit ray from it towards
/// the old vertex `V = (1, 1)`.
fn on_the_ray(dist: f64) -> (f64, f64) {
    let c = 1.0 - D;
    (c + dist / 2f64.sqrt(), c + dist / 2f64.sqrt())
}

/// **A hair either side of the arc.** On the ray from `c` towards `V`,
/// a bore of radius `0.004` whose near side is `1e-4` short of the arc
/// crosses it and refuses `RingClearance` as a measurement at the cap;
/// one `1e-4` past it lies wholly inside and goes with it. A bore
/// centred on the arc crosses too.
#[test]
fn a_bore_a_hair_across_the_planar_slivers_arc_crosses_and_one_inside_goes_with_it() {
    let (a, hair) = (0.004, 1e-4);
    for (what, dist) in [("a hair short of the arc", D + a - hair), ("on the arc", D)] {
        let (x, y) = on_the_ray(dist);
        let got = refusal(Verb::Fillet, &square::<f64>(1.0, round(x, y, a)));
        assert!(
            matches!(
                got,
                BlendError::RingClearance {
                    bounded: false,
                    chain: Convexity::Convex,
                    ..
                }
            ),
            "{what}: a measured RingClearance at the cap, got {got:?}"
        );
    }
    let (x, y) = on_the_ray(D + a + hair);
    let body = square::<f64>(1.0, round(x, y, a));
    let walls = curved_walls(&body);
    let removed = section(Verb::Fillet, D) - PI * a * a;
    assert_consumed(
        Verb::Fillet,
        &body,
        &[crease(&body, 1.0)],
        D,
        &walls,
        (removed, 1.0),
        "a hair past the arc",
    );
}

/// The point `dist` from the chamfer's chord, on its normal through the
/// chord's midpoint `(1 − D/2, 1 − D/2)` towards the old vertex.
fn past_the_chord(dist: f64) -> (f64, f64) {
    let m = 1.0 - D / 2.0;
    (m + dist / 2f64.sqrt(), m + dist / 2f64.sqrt())
}

/// **A hair either side of the chord.** A bore of radius `0.004` whose
/// near side is `1e-4` short of the chamfer's chord crosses it and
/// refuses `RingClearance` as a measurement at the cap, as does one
/// centred on it; one `1e-4` past it lies wholly inside the triangle and
/// goes with it.
#[test]
fn a_bore_a_hair_across_the_chamfers_chord_crosses_and_one_inside_goes_with_it() {
    let (a, hair) = (0.004, 1e-4);
    for (what, dist) in [
        ("a hair short of the chord", a - hair),
        ("on the chord", 0.0),
    ] {
        let (x, y) = past_the_chord(dist);
        let got = refusal(Verb::Chamfer, &square::<f64>(1.0, round(x, y, a)));
        assert!(
            matches!(
                got,
                BlendError::RingClearance {
                    bounded: false,
                    chain: Convexity::Convex,
                    ..
                }
            ),
            "{what}: a measured RingClearance at the cap, got {got:?}"
        );
    }
    let (x, y) = past_the_chord(a + hair);
    let body = square::<f64>(1.0, round(x, y, a));
    let walls = curved_walls(&body);
    let removed = section(Verb::Chamfer, D) - PI * a * a;
    assert_consumed(
        Verb::Chamfer,
        &body,
        &[crease(&body, 1.0)],
        D,
        &walls,
        (removed, 1.0),
        "a hair past the chord",
    );
}

/// **A blind bore wholly inside the sliver refuses** under both verbs, as a bound at its
/// wall: its floor is neither a cap's cycle nor a line between two, so
/// the prism reading does not take it, and the carve takes only a
/// through-feature between the caps.
#[test]
fn a_blind_bore_inside_the_planar_sliver_refuses_as_a_bound() {
    let (d, a) = BORE;
    let tool = crate::common::cavity::rod(Point2::new(1.0 - d, 1.0 - d), a, -0.5, 0.5);
    let body = crate::common::cavity::cut(
        "blind bore",
        &square::<f64>(1.0, poly(&[(0.4, 0.4), (0.6, 0.4), (0.6, 0.6), (0.4, 0.6)])),
        &tool,
    );
    for verb in VERBS {
        let got = refusal(verb, &body);
        assert!(
            matches!(got, BlendError::FaceClearance { bounded: true, .. }),
            "{verb:?}: a blind bore refuses as a bound, got {got:?}"
        );
    }
}

/// **A tilted bore inside the sliver at both caps that dips into the
/// kept ball between them refuses**, as a bound at its wall. Its axis
/// is a chord of the arc's circle, touching it at mid-length (the ray
/// from `c` towards `V`), its ends `h = 0.0255` either side, so each
/// ring is `√(r² + h²) − r − a ≈ 1.2e-3` inside the sliver; the wall
/// runs `a = 0.002` into the ball there. Its two seams run beside the
/// axis along the chord, never nearer `c` than `r`, so every sampled
/// point of the wall's boundary lies in the band's material and the
/// exact core reads no crossing: only `prism_inside`'s line slack and
/// its `diagonal·|a × τ|` term hold this row. Without them the bore is
/// consumed.
#[test]
fn a_tilted_bore_inside_at_both_planar_caps_through_the_ball_refuses() {
    let (r, h, a) = (D, 0.0255, 0.002);
    let s2 = 2f64.sqrt();
    // In `(y, z)`: `c = (r, 1 − r)`, the ray towards the old vertex
    // `(0, 1)` is `(−1, 1)/√2`, the chord along `(1, 1)/√2`.
    let mid = (r - r / s2, 1.0 - r + r / s2);
    let p = (mid.0 - h / s2, mid.1 - h / s2);
    let q = (mid.0 + h / s2, mid.1 + h / s2);
    let body = bored(0.0, p, q, a, Vec3::new(0.0, 1.0, 1.0));
    let got = Verb::Fillet
        .run_finished(&body, &[front(&body)])
        .map(|_| ());
    assert!(
        matches!(got, Err(BlendError::FaceClearance { bounded: true, .. })),
        "the tilted bore refuses as a bound at its wall, got {got:?}"
    );
}

/// **The same tilted bore goes with the chamfer's material**, at
/// `ΔV = −(A·2 − π·a²·ℓ)`, `ℓ = √(4 + (2h)²)` the axis's length
/// between the walls. Its rings lie inside the chamfer's triangle at
/// both caps (`≈ 0.027` from the chord, `≈ 0.0093` from a support), and
/// the material it removes, a triangular prism, is convex, so a wall
/// whose every point lies on a ruling between two points of rings inside
/// it lies inside it too: the prism reading owes no slack for the
/// lines' lateral run of `2h`, which is twenty times the rings' depth
/// and would refuse the bore as a bound.
#[test]
fn a_tilted_bore_inside_the_chamfers_prism_goes_with_it() {
    let (r, h, a) = (D, 0.0255, 0.002);
    let s2 = 2f64.sqrt();
    let mid = (r - r / s2, 1.0 - r + r / s2);
    let p = (mid.0 - h / s2, mid.1 - h / s2);
    let q = (mid.0 + h / s2, mid.1 + h / s2);
    let body = bored(0.0, p, q, a, Vec3::new(0.0, 1.0, 1.0));
    let walls = curved_walls(&body);
    assert_eq!(walls.len(), 2, "the tilted bore's two half-walls");
    let axis = (4.0 + (q.0 - p.0).powi(2) + (q.1 - p.1).powi(2)).sqrt();
    let removed = section(Verb::Chamfer, D) * 2.0 - PI * a * a * axis;
    assert_consumed(
        Verb::Chamfer,
        &body,
        &[front(&body)],
        D,
        &walls,
        (removed, 1.0),
        "a tilted bore",
    );
}

/// **A hole whose mouth lies wholly inside a concave sliver refuses as a
/// bound**, under both verbs. The pocketed die's floor edge
/// `(0.25, 0.25, 0.5) → (0.75, 0.25, 0.5)` is concave; its band fills the
/// corner of the pocket and buries the part of the pocket's end wall
/// `x = 0.25` between the end curve and the old vertex. A hole of radius
/// `0.006` drilled from the die's outer face `x = 0` into the pocket,
/// `0.012` off the floor and the side wall, opens on the end wall
/// wholly inside that sliver (`≈ 0.018` past the fillet's arc, `≈ 0.011`
/// past the chamfer's chord), and the cap reads its mouth inside. But the
/// hole runs into the die behind the wall, not into the fill: it is not
/// wholly inside the material the band adds, and its wall meets the end
/// wall's plane, which the reach reads as a bound. What the fill should
/// do with a feature it would seal is the question step 4 stopped at.
#[test]
fn a_hole_whose_mouth_lies_inside_a_concave_sliver_refuses_as_a_bound() {
    let (d, a) = BORE;
    let pin = prism_on(
        sketch_from_axes(
            Point3::new(-0.1, 0.25 + d, 0.5 + d),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            tol(),
        ),
        vec![(Point2::new(-a, 0.0), 1.0), (Point2::new(a, 0.0), 1.0)],
        0.45,
        tol(),
    );
    let body = realized(
        BooleanOp::Subtract,
        &sweep::test_support::pocket_die(0.0, 0.0, 0.0, tol()),
        &pin,
        tol(),
    );
    validate_geometric(&body, tol()).unwrap_or_else(|e| panic!("the drilled die: tier 3, {e:?}"));
    let floor = edge(&body, [0.25, 0.25, 0.5], [0.75, 0.25, 0.5]);
    for verb in VERBS {
        let got = verb.run(&body, &[floor]).map(|_| ());
        assert!(
            matches!(
                got,
                Err(BlendError::FaceClearance {
                    bounded: true,
                    chain: Convexity::Concave,
                    ..
                })
            ),
            "{verb:?}: a bound at the hole's wall, got {got:?}"
        );
    }
}
