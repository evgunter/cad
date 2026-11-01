//! **A chamfer's overrun** (`work/band/a-non-isosceles-turn-overruns-past-the-mitre.md`,
//! step 2): two requested edges at a trivalent plane–plane vertex whose
//! third edge `L` is unrequested, the two bands' feet on `L` definitely
//! apart. The band whose foot is further, `k`, is cut off past the
//! mitre by the other band's face of `L`, `Fⱼ`: from the mitre's end `q`
//! on band `j`'s trimline there down to `k`'s foot `p_k` on `L`, where
//! `L` ends. Every vertex there is trivalent.
//!
//! **The closed form.** A chamfer of an edge of a convex polytope `B`
//! at setback `d` takes `B ∩ {m·x > c}`: the side, holding the edge, of
//! the plane through its two trimlines, each `d` in from it on its own
//! face (a cut-off at a far end is that plane's own section of the end
//! face). At a turn the two cuts' regions overlap; the mitre is the two
//! planes' line and the overrun the far plane's trace on `Fⱼ`, so a
//! chamfer built as ruled leaves exactly `B ∩ H₁ ∩ H₂`, each `Hᵢ` the
//! kept side of band `i`'s plane. Its volume is read here by vertex
//! enumeration over the half-spaces — every triple of planes met, the
//! points inside all of them kept, each face's polygon summed as
//! `⅓ · (its plane's offset) · (its area)` — which shares nothing with
//! the kernel but the input's own planes. At a concave corner the
//! chamfer FILLS the air `A` (a convex polytope) the same way, so the
//! body gains `vol(A) − vol(A ∩ H₁ ∩ H₂)`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::RigidMotion;
use geom_core::{Bounds, Interval, Point3};
use profile::SketchPlane;
use sweep::blend::build::{Blended, chamfer_edges};
use sweep::test_support::{assert_naming_totality, at_rest, realized};
use topo::boolean::BooleanOp;
use topo::{Body, ContactRecords, EdgeKey, mass_properties, validate_geometric};
use topo::{VertexKey, validate_pseudomanifold};

use crate::band_planar_cut_off::{D, edge, tol, volume};
use crate::band_turn_overrun_probe::{
    Cone, V, Verb, add, cross, dot, measure, mul, norm, sub, unit,
};
use crate::band_turn_overrun_verdict::{cone_of, corner, far_band};
use crate::common::cavity::brick;
use crate::common::operands::{
    edge_between, leaning_turn, leaning_turn_at, parallelepiped, parallelepiped_at,
};
use crate::common::poses::poses;

/// A closed half-space `{x : n·x ≤ c}`, `n` a unit.
pub(crate) type Half = (V, f64);

fn arr(p: Point3<f64>) -> V {
    [p.x, p.y, p.z]
}

pub(crate) fn point_at(body: &Body<f64>, v: VertexKey) -> V {
    arr(*body
        .get_point(body.get_vertex(v).expect("a vertex").point)
        .expect("a point"))
}

pub(crate) fn ends(body: &Body<f64>, e: EdgeKey) -> [VertexKey; 2] {
    let he = body.get_edge(e).expect("an edge").he_plus;
    [
        body.get_half_edge(he).expect("a half-edge").start,
        body.half_edge_end(he).expect("an end"),
    ]
}

/// **A convex body's half-spaces**, read off its stored planes, each
/// faced away from the mean of its vertices (inside a convex body).
pub(crate) fn halves_of(body: &Body<f64>) -> Vec<Half> {
    let points: Vec<V> = body
        .vertices()
        .map(|(v, _)| v)
        .map(|v| point_at(body, v))
        .collect();
    let inside = mul(
        points.iter().fold([0.0; 3], |a, p| add(a, *p)),
        1.0 / points.len() as f64,
    );
    let mut halves: Vec<Half> = Vec::new();
    for f in topo::query::all_faces(body) {
        let surface = body.get_face(f).and_then(|x| body.get_surface(x.surface));
        let Some(Surface::Plane { origin, normal, .. }) = surface else {
            panic!("a convex polytope's face is a plane, got {surface:?}")
        };
        let n = unit(arr(Point3::origin() + *normal));
        let c = dot(n, arr(*origin));
        let (n, c) = if dot(n, inside) > c {
            (mul(n, -1.0), -c)
        } else {
            (n, c)
        };
        if !halves
            .iter()
            .any(|(m, k)| norm(sub(*m, n)) < 1e-12 && (k - c).abs() < 1e-12)
        {
            halves.push((n, c));
        }
    }
    halves
}

/// **The kept side of a chamfer's plane** on the edge `a`–`b` of the
/// convex polytope `halves`: through the two trimlines, each `d` in from
/// the edge on one of its two faces, the edge on the cut side.
fn chamfer_half(halves: &[Half], a: V, b: V, d: f64) -> Half {
    let faces: Vec<&Half> = halves
        .iter()
        .filter(|(n, c)| (dot(*n, a) - c).abs() < 1e-9 && (dot(*n, b) - c).abs() < 1e-9)
        .collect();
    let [(na, _), (nb, _)] = faces[..] else {
        panic!("an edge lies on two faces, got {}", faces.len())
    };
    let tau = unit(sub(b, a));
    // In each face, the unit from the edge into it: across the edge,
    // against the other face's outward normal.
    let into = |n: V, other: V| {
        let w = unit(cross(n, tau));
        if dot(w, other) > 0.0 { mul(w, -1.0) } else { w }
    };
    let (xa, xb) = (
        add(a, mul(into(*na, *nb), d)),
        add(a, mul(into(*nb, *na), d)),
    );
    let m = unit(cross(sub(xa, xb), tau));
    let m = if dot(m, sub(a, xa)) < 0.0 {
        mul(m, -1.0)
    } else {
        m
    };
    (m, dot(m, xa))
}

/// **The volume of a bounded convex polytope** given by its half-spaces:
/// every triple of planes met, the points inside all kept, and each
/// face's polygon, ordered about its centroid, summed as
/// `⅓ · c · area`.
pub(crate) fn polytope_volume(halves: &[Half]) -> f64 {
    let n = halves.len();
    let mut points: Vec<V> = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let ((a, ca), (b, cb), (c, cc)) = (halves[i], halves[j], halves[k]);
                let det = dot(a, cross(b, c));
                if det.abs() < 1e-12 {
                    continue;
                }
                let x = mul(
                    add(
                        add(mul(cross(b, c), ca), mul(cross(c, a), cb)),
                        mul(cross(a, b), cc),
                    ),
                    1.0 / det,
                );
                if halves.iter().all(|(m, k)| dot(*m, x) <= k + 1e-9)
                    && !points.iter().any(|p| norm(sub(*p, x)) < 1e-9)
                {
                    points.push(x);
                }
            }
        }
    }
    let mut total = 0.0;
    for (m, c) in halves {
        let on: Vec<V> = points
            .iter()
            .copied()
            .filter(|p| (dot(*m, *p) - c).abs() < 1e-9)
            .collect();
        if on.len() < 3 {
            continue;
        }
        let mid = mul(
            on.iter().fold([0.0; 3], |s, p| add(s, *p)),
            1.0 / on.len() as f64,
        );
        let u = unit(sub(on[0], mid));
        let w = cross(*m, u);
        let mut ring: Vec<(f64, V)> = on
            .iter()
            .map(|p| {
                let r = sub(*p, mid);
                (dot(r, w).atan2(dot(r, u)), *p)
            })
            .collect();
        ring.sort_by(|x, y| x.0.total_cmp(&y.0));
        let area: f64 = (0..ring.len())
            .map(|i| {
                let (p, q) = (ring[i].1, ring[(i + 1) % ring.len()].1);
                0.5 * dot(cross(sub(p, mid), sub(q, mid)), *m)
            })
            .sum();
        total += c * area / 3.0;
    }
    total
}

/// What a chamfer of `edges` at `d` takes from (or, concave, gives to)
/// the convex polytope `halves`: its volume less what the two kept
/// sides leave of it.
fn closed_form(halves: &[Half], body: &Body<f64>, edges: &[EdgeKey], d: f64) -> f64 {
    let mut kept = halves.to_vec();
    for &e in edges {
        let [a, b] = ends(body, e).map(|v| point_at(body, v));
        kept.push(chamfer_half(halves, a, b, d));
    }
    polytope_volume(halves) - polytope_volume(&kept)
}

/// **The overrun's own cells** in a built body: the turn foot is a
/// trivalent vertex at `p_k`, the mitre's end a trivalent vertex at
/// `q`, and one edge joins them — the far band's cut-off chord, named
/// as its `EndArc`, its end a `FootVertex` on the near face.
pub(crate) fn overrun_cells(out: &Blended<f64>, p_k: V, q: V, what: &str) {
    let body = &out.body;
    let at = |x: V| {
        body.vertices()
            .map(|(v, _)| v)
            .find(|v| norm(sub(point_at(body, *v), x)) < 1e-9)
            .unwrap_or_else(|| panic!("{what}: a vertex at {x:?}"))
    };
    let (foot, end) = (at(p_k), at(q));
    for (v, name) in [(foot, "the turn foot"), (end, "the mitre's end")] {
        let fan = body.edges_of_vertex(v).expect("a fan");
        assert_eq!(fan.len(), 3, "{what}: {name} is trivalent");
    }
    let joined = topo::query::all_edges(body)
        .into_iter()
        .filter(|e| {
            let [a, b] = ends(body, *e);
            (a, b) == (foot, end) || (a, b) == (end, foot)
        })
        .count();
    assert_eq!(
        joined, 1,
        "{what}: one edge from the turn foot to the mitre's end"
    );
    let rec = out.naming.as_ref().expect("births");
    assert_eq!(rec.mitres.len(), 1, "{what}: one mitre");
    assert_eq!(rec.turn_feet.len(), 1, "{what}: one turn foot");
    assert_eq!(rec.turn_feet[0].0, foot, "{what}: the turn foot is p_k");
    assert!(
        rec.feet.iter().any(|(v, _, _)| *v == end),
        "{what}: the mitre's end is a foot on the near face"
    );
}

/// Chamfer `edges` of `body`, asserting tiers 3 and 3′ and the naming's
/// totality.
fn chamfer(body: &Body<f64>, edges: &[EdgeKey], what: &str) -> Blended<f64> {
    let out = chamfer_edges(&at_rest(body, tol()), edges, D, tol())
        .unwrap_or_else(|e| panic!("{what}: the overrun builds, got {}", e.error));
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, {e:?}"));
    validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
        .unwrap_or_else(|e| panic!("{what}: tier 3′, {e:?}"));
    assert_naming_totality(body, &out, edges, what);
    assert!(
        out.coincidences
            .iter()
            .all(|r| r.relation != topo::Relation::CommonFoot),
        "{what}: an overrun records no common foot"
    );
    out
}

/// The probe's turn of `verb` on `body`'s corner, placed: the far band,
/// its foot `p_k` and the mitre's end `q`, in the body's frame.
pub(crate) fn placed(body: &Body<f64>, edges: [EdgeKey; 2], verb: Verb) -> (usize, V, V) {
    let c = cone_of(body, edges);
    let t = measure(&c, verb, D, 2);
    let k = t.far.expect("an overrun");
    let [a, b] = [ends(body, edges[0]), ends(body, edges[1])];
    let vertex = a
        .into_iter()
        .find(|v| b.contains(v))
        .expect("a shared vertex");
    let o = point_at(body, vertex);
    (k, add(o, mul(c.l, t.feet[k])), add(o, t.q[1 - k]))
}

/// A witness: its name, its body, and its two requested edges.
pub(crate) type Witness = (&'static str, Body<f64>, [EdgeKey; 2]);

/// The skewed box's overrun corner: on `parallelepiped(0.3)`, the top
/// edge and the lateral edge out of `(s, s, 1)`, the other top edge
/// unrequested — face angles `90°` and `≈106.7°` at it.
pub(crate) fn skewed_box() -> Witness {
    let s = 0.3;
    let body = parallelepiped(s);
    let v = [s, s, 1.0];
    let edges = [
        edge(&body, v, [2.0 + s, s, 1.0]),
        edge(&body, v, [0.0, 0.0, 0.0]),
    ];
    ("skewed box", body, edges)
}

/// The witnesses: the bracket's two section-face corners — `(45°, 90°)`
/// and `(135°, 90°)` at `γ = 90°`, a leaning prism's corner at a lean
/// of `∓1` — the leaning prism at `±0.5`, and the skewed box.
pub(crate) fn witnesses() -> Vec<Witness> {
    let lean = |what, s| {
        let (body, edges) = leaning_turn(s);
        (what, body, edges)
    };
    vec![
        lean("bracket y = 0", -1.0),
        lean("bracket y = 1", 1.0),
        lean("leaning prism 0.5", 0.5),
        lean("leaning prism −0.5", -0.5),
        skewed_box(),
    ]
}

/// **The three witnesses build** ([`witnesses`]) — tiers 3 and
/// 3′, naming total, `ΔV` the two chamfer half-spaces' (module docs) to
/// `1e-10`, the overrun's cells where the probe puts them, and the far
/// band the probe's.
#[test]
fn the_three_witnesses_build_at_their_closed_form_volumes() {
    for (what, body, edges) in witnesses() {
        let out = chamfer(&body, &edges, what);
        let want = closed_form(&halves_of(&body), &body, &edges, D);
        let removed = volume(&body) - volume(&out.body);
        assert!(
            (removed - want).abs() < 1e-10,
            "{what}: ΔV {removed} against the half-spaces' {want}"
        );
        let (k, p_k, q) = placed(&body, edges, Verb::Chamfer);
        assert_eq!(
            k,
            far_band(&body, edges, Verb::Chamfer),
            "{what}: the far band"
        );
        overrun_cells(&out, p_k, q, what);
        let rec = out.naming.as_ref().expect("births");
        let vertex = {
            let [a, b] = [ends(&body, edges[0]), ends(&body, edges[1])];
            a.into_iter()
                .find(|v| b.contains(v))
                .expect("a shared vertex")
        };
        assert!(
            rec.arcs
                .iter()
                .any(|(_, v, e)| *v == vertex && *e == edges[k]),
            "{what}: the overrun is the far band's EndArc at the vertex"
        );
    }
}

/// **At poses**: each witness moved rigidly through the six-pose set
/// builds tier-3 valid at the same `ΔV`.
#[test]
fn the_witnesses_build_at_every_pose() {
    for (what, body, edges) in witnesses() {
        let want = closed_form(&halves_of(&body), &body, &edges, D);
        for (pose, map) in poses() {
            let moved =
                topo::transform_rigid(&body, &geom_core::test_support::rigid(&map, tol()), tol())
                    .expect("a rigid pose");
            let what = format!("{what} at {pose}");
            let out = chamfer(&moved, &edges, &what);
            let removed = volume(&moved) - volume(&out.body);
            assert!(
                (removed - want).abs() < 1e-9,
                "{what}: ΔV {removed} against {want}"
            );
        }
    }
}

/// A certified replay: its name, its `f64` body, and its `Interval`
/// body with the two requested edges.
type IntervalCase = (String, Body<f64>, Body<Interval>, [EdgeKey; 2]);

/// **At the certified scalar**: the leaning prism at `±0.5` and the
/// skewed box at `Interval` build, tier-3 valid and naming total, the
/// volume enclosure bracketing the closed form.
#[test]
fn the_overrun_carves_at_the_certified_scalar() {
    use crate::common::interval::iv;
    let s = 0.3;
    let v = [s, s, 1.0];
    let mut cases: Vec<IntervalCase> = [0.5, -0.5]
        .into_iter()
        .map(|lean| {
            let (ibody, iedges) = leaning_turn_at::<Interval>(lean);
            (
                format!("leaning prism {lean}"),
                leaning_turn(lean).0,
                ibody,
                iedges,
            )
        })
        .collect();
    let ibox = parallelepiped_at::<Interval>(s);
    let iedges = [
        edge_between(&ibox, v, [2.0 + s, s, 1.0]),
        edge_between(&ibox, v, [0.0, 0.0, 0.0]),
    ];
    cases.push(("skewed box".into(), parallelepiped(s), ibox, iedges));
    for (what, body, ibody, iedges) in cases {
        let fedges = iedges.map(|e| {
            let [a, b] = ends_iv(&ibody, e);
            edge_between(&body, a, b)
        });
        let want = closed_form(&halves_of(&body), &body, &fedges, D);
        let ibody = sweep::test_support::finished("operand", ibody, tol());
        let out = chamfer_edges(&ibody, &iedges, iv(D), tol())
            .unwrap_or_else(|e| panic!("{what}: builds at Interval, got {:?}", e.error));
        validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, {e:?}"));
        assert_naming_totality(&ibody, &out, &iedges, &what);
        let v0 = mass_properties(&ibody, tol(), topo::test_support::pinned_resolution(tol()))
            .expect("props")
            .met()
            .expect("the pinned resolution is met")
            .volume;
        let p = topo::mass_properties(
            &topo::test_support::finished("the measured body", topo::Body::clone(&out.body), tol()),
            tol(),
            topo::test_support::pinned_resolution(tol()),
        )
        .expect("props");
        let (removed, pad) = (v0 - p.enclosure.volume, p.enclosure.volume_pad.hi());
        assert!(
            removed.lo() - pad <= want && want <= removed.hi() + pad,
            "{what}: ΔV {removed:?} ± {pad} brackets {want}"
        );
        assert!(
            removed.hi() - removed.lo() + 2.0 * pad < 1e-5,
            "{what}: the enclosure is a claim: {removed:?} ± {pad}"
        );
    }
}

/// An `Interval` edge's two ends, at their midpoints.
pub(crate) fn ends_iv(body: &Body<Interval>, e: EdgeKey) -> [[f64; 3]; 2] {
    let he = body.get_edge(e).expect("an edge").he_plus;
    [
        body.get_half_edge(he).expect("a half-edge").start,
        body.half_edge_end(he).expect("an end"),
    ]
    .map(|v| {
        let p = body
            .get_point(body.get_vertex(v).expect("a vertex").point)
            .expect("a point");
        [p.x, p.y, p.z].map(|c| 0.5 * (c.lo() + c.hi()))
    })
}

/// **Concave**: the leaning prism cut up into a block from `z = 0.5`,
/// its corner now a pocket's ceiling corner, and the skewed box as a
/// void inside a block — the
/// chamfer fills the overrun, tier 3 and 3′, naming total, the body
/// gaining what the two half-spaces leave of the air (module docs).
#[test]
fn a_concave_overrun_fills_at_its_closed_form() {
    let s = 0.3;
    let block = |lo: [f64; 3], hi: [f64; 3]| {
        brick(
            Point3::new(lo[0], lo[1], lo[2]),
            Point3::new(hi[0], hi[1], hi[2]),
        )
    };
    let floor: Half = ([0.0, 0.0, -1.0], -0.5);
    let leaning = |lean: f64| {
        let (prism, _) = leaning_turn(lean);
        let corner = [lean, 0.0, 1.0];
        (
            prism,
            block([-1.0, -2.5, 0.5], [3.0, 1.0, 2.0]),
            [corner, [2.0, 0.0, 1.0]],
            [corner, [lean, -1.5, 1.0]],
        )
    };
    let v = [s, s, 1.0];
    let mut cases = vec![
        ("leaning prism 0.5", leaning(0.5)),
        ("leaning prism −0.5", leaning(-0.5)),
    ];
    // The skewed box's lateral edge runs to its floor, so its air is a
    // void: the block holds the whole box.
    cases.push((
        "skewed box",
        (
            parallelepiped(s),
            block([-1.0, -1.0, -1.0], [4.0, 3.0, 2.0]),
            [v, [2.0 + s, s, 1.0]],
            [v, [0.0, 0.0, 0.0]],
        ),
    ));
    for (what, (convex, block, a, b)) in cases {
        let pocket = realized(BooleanOp::Subtract, &block, &convex, tol());
        let edges = [edge(&pocket, a[0], a[1]), edge(&pocket, b[0], b[1])];
        let out = chamfer(&pocket, &edges, what);
        let mut air = halves_of(&convex);
        if what != "skewed box" {
            air.push(floor);
        }
        let want = closed_form(&air, &pocket, &edges, D);
        let added = volume(&out.body) - volume(&pocket);
        assert!(
            (added - want).abs() < 1e-10,
            "{what}, concave: ΔV {added} against the air's {want}"
        );
        let (_, p_k, q) = placed(&pocket, edges, Verb::Chamfer);
        overrun_cells(&out, p_k, q, what);
    }
}

/// **The sweep**: over face-angle pairs `φ₁, φ₂ ∈ 30°…150°` (20° steps)
/// at `γ ∈ {60°, 90°, 120°}`, every corner whose chamfer overruns builds — tiers 3
/// and 3′, naming total — at the half-spaces' `ΔV`, its turn foot at the
/// probe's far foot and the mitre's end at the probe's `q`, the far band
/// the probe's; the overrun chord `|q − p_k|` the probe's area's
/// triangle reads. Convex here; concave in
/// [`a_concave_overrun_fills_at_its_closed_form`] and
/// [`the_sweep_fills_concave_overruns`].
#[test]
fn the_sweep_builds_every_convex_overrun_at_the_probes_closed_forms() {
    let mut built = 0;
    for gamma in [60.0f64, 90.0, 120.0] {
        for p1 in (30..=150).step_by(20) {
            for p2 in (30..=150).step_by(20) {
                let c = Cone::angles(
                    [f64::from(p1).to_radians(), f64::from(p2).to_radians()],
                    gamma.to_radians(),
                );
                let t = measure(&c, Verb::Chamfer, D, 2);
                if t.far.is_none() || t.gap < 1e-6 {
                    continue;
                }
                let what = format!("({p1}°, {p2}°, γ {gamma}°)");
                let (body, edges) = corner(&c);
                let out = chamfer(&body, &edges, &what);
                let want = closed_form(&halves_of(&body), &body, &edges, D);
                let removed = volume(&body) - volume(&out.body);
                assert!(
                    (removed - want).abs() < 1e-10,
                    "{what}: ΔV {removed} against the half-spaces' {want}"
                );
                let (_, p_k, q) = placed(&body, edges, Verb::Chamfer);
                overrun_cells(&out, p_k, q, &what);
                built += 1;
            }
        }
    }
    // Per γ, 49 pairs less the isosceles diagonal (7) and the
    // supplementary anti-diagonal (6 off the diagonal).
    assert_eq!(built, 3 * (49 - 13), "every overrun corner built");
}

/// **The sweep, concave**: the corners of
/// [`the_sweep_builds_every_convex_overrun_at_the_probes_closed_forms`]
/// at 40° steps, each the air of a void inside a block, its overrun
/// filled at the half-spaces' volume of the air, its cells where the
/// probe puts them.
#[test]
fn the_sweep_fills_concave_overruns() {
    let mut built = 0;
    for gamma in [60.0f64, 90.0, 120.0] {
        for p1 in (30..=150).step_by(40) {
            for p2 in (30..=150).step_by(40) {
                let c = Cone::angles(
                    [f64::from(p1).to_radians(), f64::from(p2).to_radians()],
                    gamma.to_radians(),
                );
                let t = measure(&c, Verb::Chamfer, D, 2);
                if t.far.is_none() || t.gap < 1e-6 {
                    continue;
                }
                let what = format!("({p1}°, {p2}°, γ {gamma}°), concave");
                let (wedge, wedge_edges) = corner(&c);
                let ends = wedge_edges.map(|e| ends(&wedge, e).map(|v| point_at(&wedge, v)));
                let block = brick(
                    Point3::new(-50.0, -50.0, -50.0),
                    Point3::new(50.0, 50.0, 50.0),
                );
                let void = realized(BooleanOp::Subtract, &block, &wedge, tol());
                let edges = ends.map(|[a, b]| edge(&void, a, b));
                let out = chamfer(&void, &edges, &what);
                let want = closed_form(&halves_of(&wedge), &void, &edges, D);
                let added = volume(&out.body) - volume(&void);
                assert!(
                    (added - want).abs() < 1e-9,
                    "{what}: ΔV {added} against the air's {want}"
                );
                let (_, p_k, q) = placed(&void, edges, Verb::Chamfer);
                overrun_cells(&out, p_k, q, &what);
                built += 1;
            }
        }
    }
    // Per γ, 16 pairs less the isosceles diagonal (4) and the
    // supplementary anti-diagonal (4).
    assert_eq!(built, 3 * 8, "every concave overrun corner built");
}

/// The bracket's `y = 0` corner (`leaning_turn(−1)`) with a round pin
/// of radius `0.004` standing `0.03` out of its leaning wall, at the
/// wall's point `a` along `L` and `b` across it (along `−y`) from the
/// vertex `(−1, 0, 1)`: a boss whose one ring lies on the wall and whose
/// side stands in the air.
fn bossed(a: f64, b: f64) -> (Body<f64>, [EdgeKey; 2]) {
    let (body, _) = leaning_turn(-1.0);
    let r2 = core::f64::consts::FRAC_1_SQRT_2;
    let inward = [r2, 0.0, r2];
    let at = [-1.0 + a * r2, -b, 1.0 - a * r2];
    let start = add(at, mul(inward, 0.01));
    let pin = sweep::test_support::prism_on(
        sweep::test_support::sketch_from_axes(
            Point3::new(start[0], start[1], start[2]),
            geom_core::Vec3::new(0.0, 1.0, 0.0),
            geom_core::Vec3::new(r2, 0.0, -r2),
            tol(),
        ),
        vec![
            (geom_core::Point2::new(-0.004, 0.0), 1.0),
            (geom_core::Point2::new(0.004, 0.0), 1.0),
        ],
        0.04,
        tol(),
    );
    let bossed = realized(BooleanOp::Union, &body, &pin, tol());
    validate_geometric(&bossed, tol()).expect("the bossed corner is tier-3 valid");
    let corner = [-1.0, 0.0, 1.0];
    let edges = [
        edge(&bossed, corner, [2.0, 0.0, 1.0]),
        edge(&bossed, corner, [-1.0, -1.5, 1.0]),
    ];
    (bossed, edges)
}

/// **A boss standing in the overrun refuses.** At the bracket's `y = 0`
/// corner the top edge's band reaches further down `L` (`√2·d` against
/// `d`) and is cut off by the leaning wall: the overrun is the right
/// triangle there with legs `(√2 − 1)·d` along `L` and `(1 − 1/√2)·d`
/// along the near band's trimline, its incircle of radius `≈ 0.1·d`
/// about `(1.1·d, 0.1·d)` in the wall's `(along L, across)` frame. A
/// boss standing on the wall there has its ring past the near band's
/// trimline, so that band's own meters pass it. The far band's overrun
/// sliver reads the ring inside its triangle, an inside reading no carve
/// takes (the sliver carries no crease), and the reach refuses the
/// boss's side as a bound (`FaceClearance`), which outranks it. The same
/// boss moved clear of the sliver, to `(2·d, 0.1·d)`, builds at the
/// half-spaces' volume.
#[test]
fn a_boss_in_the_overrun_refuses_and_one_clear_of_the_sliver_builds() {
    let (inside, edges) = bossed(1.1 * D, 0.1 * D);
    match chamfer_edges(&at_rest(&inside, tol()), &edges, D, tol()) {
        Err(e) => assert!(
            matches!(
                e.error,
                sweep::blend::BlendError::FaceClearance { bounded: true, .. }
            ),
            "a boss in the overrun refuses as a bound at its side, got {:?}",
            e.error
        ),
        Ok(out) => panic!(
            "a boss in the overrun must refuse, built one whose tier 3 reads {:?}",
            validate_geometric(&out.body, tol())
        ),
    }
    let (clear, edges) = bossed(2.0 * D, 0.1 * D);
    let out = chamfer(&clear, &edges, "a boss clear of the overrun");
    let (plain, plain_edges) = leaning_turn(-1.0);
    let want = closed_form(&halves_of(&plain), &plain, &plain_edges, D);
    let removed = volume(&clear) - volume(&out.body);
    assert!(
        (removed - want).abs() < 1e-10,
        "a boss clear of the overrun: ΔV {removed} against {want}"
    );
}

/// **A slot through the overrun is never consumed.** At the bracket's
/// `y = 0` corner a square slot of half-side `0.003` runs along the top
/// edge's band, at the overrun's incentre `(1.1·d, 0.1·d)` in the
/// leaning wall's `(along L, across)` frame, out through the square
/// end `x = 2`. Its ring there lies inside that cut-off's triangle, and
/// its ring on the leaning wall (lines, as a support's rings must be)
/// inside the triangle the far band's
/// overrun sliver is read through. That triangle is a bound larger than
/// the overrun, not the far band's removed slice, so its crease is none
/// and a cycle inside it is never a ring the carve takes: the slot's
/// walls, read through the one ring at `x = 2`, refuse as a bound.
/// Given the far band's crease there, the carve takes the slot.
#[test]
fn a_slot_through_the_overrun_is_never_consumed() {
    let (body, edges) = leaning_turn(-1.0);
    let r2 = core::f64::consts::FRAC_1_SQRT_2;
    let (y, z, h) = (-0.1 * D, 1.0 - 1.1 * D * r2, 0.003);
    let pin = sweep::test_support::prism_on(
        SketchPlane::from_frame(RigidMotion::axes_yz(Point3::new(-1.5, y, z))),
        sweep::test_support::corners(&[(-h, -h), (h, -h), (h, h), (-h, h)]),
        4.0,
        tol(),
    );
    let bored = realized(BooleanOp::Subtract, &body, &pin, tol());
    validate_geometric(&bored, tol()).expect("the slotted corner is tier-3 valid");
    let corner = [-1.0, 0.0, 1.0];
    let edges = edges.map(|e| {
        let [a, b] = ends(&body, e).map(|v| point_at(&body, v));
        let far = if norm(sub(a, corner)) < 1e-12 { b } else { a };
        edge(&bored, corner, far)
    });
    match chamfer_edges(&at_rest(&bored, tol()), &edges, D, tol()) {
        Err(e) => assert!(
            matches!(
                e.error,
                sweep::blend::BlendError::FaceClearance { bounded: true, .. }
            ),
            "the slot refuses as a bound at its wall, which the prism reading cannot take \
             through a ring on the overrun's sliver, got {:?}",
            e.error
        ),
        Ok(out) => panic!(
            "a slot through the overrun must refuse, built one whose tier 3 reads {:?}",
            validate_geometric(&out.body, tol())
        ),
    }
}

/// **L blended in a later call** ends at the overrun's turn foot, a
/// trivalent vertex, in the far band's face: its cut-off's feet land on
/// that face's two rims there, the far band's trimline on L's other face
/// and the overrun edge. At each of the leaning prism's overrun corners
/// (`±0.5`, `±1`) a second chamfer of L at the first's setback refuses
/// at the clearance screen, which cannot certify L's band clear of the
/// faces about the short overrun edge (`FaceClearanceUncertified`); at a
/// tenth of it L's foot lands on the overrun edge and the chamfer builds,
/// tier-3 valid, taking what its half-space takes of the first chamfer's
/// body (still a convex polytope, read off its own planes).
#[test]
fn a_later_chamfer_of_l_builds_where_its_foot_lands_on_the_overrun_edge() {
    for s in [0.5, -0.5, 1.0, -1.0] {
        let what = format!("lean {s}");
        let (body, edges) = leaning_turn(s);
        let out = chamfer(&body, &edges, &what);
        let foot = out.naming.as_ref().expect("births").turn_feet[0].0;
        let b = &out.body;
        let l = b
            .edges_of_vertex(foot)
            .expect("a fan")
            .into_iter()
            .find(|e| ends(b, *e).iter().any(|v| norm(point_at(b, *v)) < 1e-12))
            .expect("L's piece runs from the turn foot to the origin");
        match chamfer_edges(&at_rest(b, tol()), &[l], D, tol()) {
            Err(e) => assert!(
                matches!(
                    e.error,
                    sweep::blend::BlendError::FaceClearanceUncertified { .. }
                ),
                "{what}: L at the first setback refuses at the clearance screen, got {}",
                e.error
            ),
            Ok(_) => panic!("{what}: L at the first setback must refuse"),
        }
        let later = chamfer_edges(&at_rest(b, tol()), &[l], 0.1 * D, tol()).unwrap_or_else(|e| {
            panic!(
                "{what}: L at a tenth of the setback builds, got {}",
                e.error
            )
        });
        validate_geometric(&later.body, tol())
            .unwrap_or_else(|e| panic!("{what}: L later: tier 3, {e:?}"));
        let want = closed_form(&halves_of(b), b, &[l], 0.1 * D);
        let removed = volume(b) - volume(&later.body);
        assert!(
            (removed - want).abs() < 1e-12,
            "{what}: L later takes ΔV {removed} against its half-space's {want}"
        );
    }
}
