//! **A chamfer's overrun against two oracles** that share nothing with
//! `band_turn_overrun_chamfer`'s vertex enumeration, nor with each
//! other.
//!
//! - **The clip oracle.** A convex body is written down as its own
//!   analytic half-spaces, from its construction rather than off the
//!   body; each chamfer adds the half-space through its two trimlines;
//!   the volume is a large cube clipped plane by plane (Sutherland–
//!   Hodgman per face, plus the cap polygon each plane leaves) and
//!   summed by the divergence theorem.
//! - **The Boolean-slab oracle.** A chamfer of a convex edge with unit
//!   outward normals `nₐ`, `n_b` and interior dihedral `β` at setback `d`
//!   keeps `{m·x ≤ m·X₀ − d·cos(β/2)}`, `m = (nₐ + n_b)/|nₐ + n_b|`,
//!   `X₀` on the edge; the expected body is the operand less one slab
//!   per such plane through the kernel's Boolean, and the built body's
//!   vertex set and volume are compared with it.
//!
//! The rows: the bracket's corner against a hand-integrated closed form,
//! leaning prisms over leans, setbacks and scales, generic hexahedra
//! (convex and as voids), whole rims (a band the far band at one turn
//! and the near band at the next), two turns on one `L`, the bracket's
//! corners at `Interval`, features in and around the overrun, and the
//! two refusals the overrun's sliver makes where the overrun itself
//! fits (each pinned with its filed item).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Bounds, Interval, Point2, Point3, Vec3};
use geom_core::{RigidMotion, Turns, UnitVec3};
use profile::SketchPlane;
use sweep::blend::BlendError;
use sweep::blend::build::{Blended, chamfer_edges};
use sweep::test_support::{
    assert_naming_totality, at_rest, corners, prism_on, realized, sketch_from_axes,
};
use topo::boolean::BooleanOp;
use topo::{Body, ContactRecords, EdgeKey, VertexKey, validate_geometric, validate_pseudomanifold};

use crate::band_planar_cut_off::{tol, volume};
use crate::band_turn_overrun_probe::{Cone, V, add, cross, dot, mul, norm, sub, unit};
use crate::band_turn_overrun_verdict::corner;
use crate::common::cavity::brick;
use crate::common::interval::iv;
use crate::common::operands::{leaning_turn, leaning_turn_at};

const D: f64 = 0.1;

/// `{x : n·x ≤ c}`, `n` a unit.
type Half = (V, f64);

fn pt(v: V) -> Point3<f64> {
    Point3::new(v[0], v[1], v[2])
}

fn vc(v: V) -> Vec3<f64> {
    Vec3::new(v[0], v[1], v[2])
}

fn point_at(body: &Body<f64>, v: VertexKey) -> V {
    let p = body
        .get_point(body.get_vertex(v).expect("a vertex").point)
        .expect("a point");
    [p.x, p.y, p.z]
}

fn ends(body: &Body<f64>, e: EdgeKey) -> [VertexKey; 2] {
    let he = body.get_edge(e).expect("an edge").he_plus;
    [
        body.get_half_edge(he).expect("a half-edge").start,
        body.half_edge_end(he).expect("an end"),
    ]
}

/// The edge whose ends lie within `1e-9` of `a` and `b`, relative to
/// their scale.
pub(crate) fn edge_near(body: &Body<f64>, a: V, b: V) -> EdgeKey {
    let scale = norm(a).max(norm(b)).max(1.0);
    let near = |p: V, q: V| norm(sub(p, q)) < 1e-9 * scale;
    topo::query::all_edges(body)
        .into_iter()
        .find(|&e| {
            let [s, t] = ends(body, e).map(|v| point_at(body, v));
            (near(s, a) && near(t, b)) || (near(s, b) && near(t, a))
        })
        .unwrap_or_else(|| panic!("an edge between {a:?} and {b:?}"))
}

/// Chamfer `edges` at `d`; a build is held to tiers 3 and 3′ and to
/// naming totality before it is returned.
fn chamfered(
    body: &Body<f64>,
    edges: &[EdgeKey],
    d: f64,
    what: &str,
) -> Result<Blended<f64>, BlendError> {
    let out = chamfer_edges(&at_rest(body, tol()), edges, d, tol()).map_err(|e| e.error)?;
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, {e:?}"));
    validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
        .unwrap_or_else(|e| panic!("{what}: tier 3′, {e:?}"));
    assert_naming_totality(body, &out, edges, what);
    Ok(out)
}

/// How many of a body's vertices are not trivalent.
pub(crate) fn non_trivalent(body: &Body<f64>) -> usize {
    body.vertices()
        .filter(|(v, _)| body.edges_of_vertex(*v).expect("a fan").len() != 3)
        .count()
}

/// Whether a refusal is one of the clearance meters'.
pub(crate) fn a_clearance(e: &BlendError) -> bool {
    matches!(
        e,
        BlendError::FaceClearance { .. }
            | BlendError::FaceClearanceUncertified { .. }
            | BlendError::RingClearance { .. }
    )
}

/// Whether a refusal is a cut-off's foot landing inside a face
/// (`end_face.rs`'s `FOOT_INSIDE_A_FACE`).
pub(crate) fn foot_inside_a_face(e: &BlendError) -> bool {
    matches!(e, BlendError::UnsupportedRunOut { detail, .. }
        if detail.contains("trimline meets the end face's plane past its rim"))
}

// ------------------------------------------------------------------
// The clip oracle.
// ------------------------------------------------------------------

/// The half-space through `p` with outward normal along `n`.
pub(crate) fn half(n: V, p: V) -> Half {
    let n = unit(n);
    (n, dot(n, p))
}

/// A polytope given as outward-oriented polygons, clipped by `{n·x ≤ c}`:
/// each polygon clipped, and the cut's own polygon added.
fn clip(polys: Vec<Vec<V>>, (n, c): Half) -> Vec<Vec<V>> {
    let mut out = Vec::new();
    let mut cut: Vec<V> = Vec::new();
    for poly in polys {
        let mut kept = Vec::new();
        for i in 0..poly.len() {
            let (p, q) = (poly[i], poly[(i + 1) % poly.len()]);
            let (dp, dq) = (dot(n, p) - c, dot(n, q) - c);
            if dp <= 0.0 {
                kept.push(p);
            }
            if (dp < 0.0 && dq > 0.0) || (dp > 0.0 && dq < 0.0) {
                let x = add(p, mul(sub(q, p), dp / (dp - dq)));
                kept.push(x);
                cut.push(x);
            } else if dp == 0.0 {
                cut.push(p);
            }
        }
        if kept.len() >= 3 {
            out.push(kept);
        }
    }
    if cut.len() >= 3 {
        let mid = mul(
            cut.iter().fold([0.0; 3], |s, p| add(s, *p)),
            1.0 / cut.len() as f64,
        );
        let far = cut
            .iter()
            .copied()
            .max_by(|a, b| norm(sub(*a, mid)).total_cmp(&norm(sub(*b, mid))))
            .expect("a cut polygon");
        let u = unit(sub(far, mid));
        let w = cross(n, u);
        let mut ring: Vec<(f64, V)> = cut
            .iter()
            .map(|p| (dot(sub(*p, mid), w).atan2(dot(sub(*p, mid), u)), *p))
            .collect();
        ring.sort_by(|a, b| a.0.total_cmp(&b.0));
        out.push(ring.into_iter().map(|x| x.1).collect());
    }
    out
}

/// The volume of `⋂ halves`: a cube of half-size `10⁴` clipped plane by
/// plane, each polygon's cone to the origin summed.
fn clipped_volume(halves: &[Half]) -> f64 {
    let r = 1.0e4;
    let c = |x: f64, y: f64, z: f64| [x * r, y * r, z * r];
    let mut polys = vec![
        vec![
            c(-1., -1., -1.),
            c(-1., 1., -1.),
            c(1., 1., -1.),
            c(1., -1., -1.),
        ],
        vec![
            c(-1., -1., 1.),
            c(1., -1., 1.),
            c(1., 1., 1.),
            c(-1., 1., 1.),
        ],
        vec![
            c(-1., -1., -1.),
            c(1., -1., -1.),
            c(1., -1., 1.),
            c(-1., -1., 1.),
        ],
        vec![
            c(-1., 1., -1.),
            c(-1., 1., 1.),
            c(1., 1., 1.),
            c(1., 1., -1.),
        ],
        vec![
            c(-1., -1., -1.),
            c(-1., -1., 1.),
            c(-1., 1., 1.),
            c(-1., 1., -1.),
        ],
        vec![
            c(1., -1., -1.),
            c(1., 1., -1.),
            c(1., 1., 1.),
            c(1., -1., 1.),
        ],
    ];
    for h in halves {
        polys = clip(polys, *h);
    }
    polys
        .iter()
        .map(|p| {
            (1..p.len() - 1)
                .map(|i| dot(p[0], cross(p[i], p[i + 1])) / 6.0)
                .sum::<f64>()
        })
        .sum()
}

/// The kept side of a chamfer of the edge `a`–`b` lying on the faces of
/// outward normals `na`, `nb`: the plane through the two lines `d` in
/// from the edge on each face, the edge on the cut side.
fn chamfer_plane(na: V, nb: V, a: V, b: V, d: f64) -> Half {
    let tau = unit(sub(b, a));
    let into = |n: V, other: V| {
        let w = unit(cross(n, tau));
        if dot(w, other) > 0.0 { mul(w, -1.0) } else { w }
    };
    let (xa, xb) = (add(a, mul(into(na, nb), d)), add(a, mul(into(nb, na), d)));
    let m = unit(cross(sub(xa, xb), tau));
    let m = if dot(m, sub(a, xa)) > 0.0 {
        m
    } else {
        mul(m, -1.0)
    };
    (m, dot(m, xa))
}

/// The point where three planes meet.
pub(crate) fn corner_of(h: [Half; 3]) -> V {
    let [(a, ca), (b, cb), (c, cc)] = h;
    let det = dot(a, cross(b, c));
    mul(
        add(
            add(mul(cross(b, c), ca), mul(cross(c, a), cb)),
            mul(cross(a, b), cc),
        ),
        1.0 / det,
    )
}

/// A requested edge as the two faces it lies on and its two ends' third
/// faces, each an index into a [`Poly`]'s halves.
#[derive(Clone, Copy)]
pub(crate) struct Req {
    pub(crate) faces: [usize; 2],
    pub(crate) ends: [usize; 2],
}

pub(crate) const fn req(faces: [usize; 2], ends: [usize; 2]) -> Req {
    Req { faces, ends }
}

/// A convex polytope fixture: the body and its own analytic half-spaces.
pub(crate) struct Poly {
    pub(crate) body: Body<f64>,
    pub(crate) halves: Vec<Half>,
}

impl Poly {
    pub(crate) fn edge(&self, r: Req) -> (EdgeKey, V, V) {
        let h = |i: usize| self.halves[i];
        let a = corner_of([h(r.faces[0]), h(r.faces[1]), h(r.ends[0])]);
        let b = corner_of([h(r.faces[0]), h(r.faces[1]), h(r.ends[1])]);
        (edge_near(&self.body, a, b), a, b)
    }

    fn cut(&self, r: Req, d: f64) -> Half {
        let (_, a, b) = self.edge(r);
        chamfer_plane(
            self.halves[r.faces[0]].0,
            self.halves[r.faces[1]].0,
            a,
            b,
            d,
        )
    }

    /// What a chamfer of `reqs` at `d` takes from the polytope.
    fn taken(&self, reqs: &[Req], d: f64) -> f64 {
        let mut kept = self.halves.clone();
        kept.extend(reqs.iter().map(|r| self.cut(*r, d)));
        clipped_volume(&self.halves) - clipped_volume(&kept)
    }

    pub(crate) fn edges(&self, reqs: &[Req]) -> Vec<EdgeKey> {
        reqs.iter().map(|r| self.edge(*r).0).collect()
    }
}

/// The leaning prism, generalised: the profile `(0,0) (2,0) (2,h) (s,h)`
/// in `xz`, extruded from `y = 0` to `y = −1.5`, all scaled by `k`.
/// Halves: 0 bottom, 1 right, 2 top, 3 leaning wall, 4 front (`y = 0`),
/// 5 back.
pub(crate) fn leaning(s: f64, h: f64, k: f64) -> Poly {
    let plane = SketchPlane::from_frame(RigidMotion::turned(
        pt([0.0; 3]),
        UnitVec3::exact_x(),
        Turns::quarters(1),
    ));
    let body = prism_on(
        plane,
        corners(&[(0.0, 0.0), (2.0 * k, 0.0), (2.0 * k, h * k), (s * k, h * k)]),
        1.5 * k,
        tol(),
    );
    let halves = vec![
        half([0., 0., -1.], [0., 0., 0.]),
        half([1., 0., 0.], [2. * k, 0., 0.]),
        half([0., 0., 1.], [0., 0., h * k]),
        half([-h, 0., s], [0., 0., 0.]),
        half([0., 1., 0.], [0., 0., 0.]),
        half([0., -1., 0.], [0., -1.5 * k, 0.]),
    ];
    Poly { body, halves }
}

/// A generic hexahedron: the leaning prism (`x ≤ x1`) with its top
/// tilted (`z ≤ h + a·x`) and its back at `y = −depth`, met with a slab
/// whose front wall `y ≤ b·z + c·x` leans and yaws (`c` makes the top
/// face's angle at the turn vertex obtuse or acute). Halves as
/// [`leaning`]'s.
#[derive(Clone, Copy)]
pub(crate) struct Hex {
    pub(crate) s: f64,
    pub(crate) h: f64,
    pub(crate) a: f64,
    pub(crate) b: f64,
    pub(crate) c: f64,
    pub(crate) x1: f64,
    pub(crate) depth: f64,
}

pub(crate) const HEX: Hex = Hex {
    s: 0.4,
    h: 1.0,
    a: 0.2,
    b: 0.3,
    c: 0.0,
    x1: 2.0,
    depth: 1.5,
};

pub(crate) fn hexahedron(x: Hex) -> Poly {
    let Hex {
        s,
        h,
        a,
        b,
        c,
        x1,
        depth,
    } = x;
    let top_at = |x: f64| h + a * x;
    let prism = prism_on(
        SketchPlane::from_frame(RigidMotion::turned(
            pt([0.0, 6.0, 0.0]),
            UnitVec3::exact_x(),
            Turns::quarters(1),
        )),
        corners(&[(0.0, 0.0), (x1, 0.0), (x1, top_at(x1)), (s, top_at(s))]),
        6.0 + depth,
        tol(),
    );
    let n_f = unit([-c, 1.0, -b]);
    let w = unit(cross(n_f, [1.0, 0.0, 0.0]));
    let v = cross(w, n_f);
    let o = mul(w, -20.0);
    let slab = prism_on(
        sketch_from_axes(pt(o), vc(n_f), vc(v), tol()),
        corners(&[(-20.0, -20.0), (0.0, -20.0), (0.0, 20.0), (-20.0, 20.0)]),
        40.0,
        tol(),
    );
    let body = realized(BooleanOp::Intersect, &prism, &slab, tol());
    let wall_n = cross([s, 0.0, top_at(s)], [0.0, 1.0, 0.0]);
    let wall_n = if dot(wall_n, [1.0, 0.0, 0.1]) > 0.0 {
        mul(wall_n, -1.0)
    } else {
        wall_n
    };
    let halves = vec![
        half([0., 0., -1.], [0., 0., 0.]),
        half([1., 0., 0.], [x1, 0., 0.]),
        half([-a, 0., 1.], [0., 0., h]),
        half(wall_n, [0., 0., 0.]),
        half(n_f, [0., 0., 0.]),
        half([0., -1., 0.], [0., -depth, 0.]),
    ];
    Poly { body, halves }
}

/// The top face's angle at the turn vertex, degrees.
pub(crate) fn theta_s(p: &Poly) -> f64 {
    let h = |i: usize| p.halves[i];
    let v = corner_of([h(2), h(3), h(4)]);
    let e1 = sub(corner_of([h(2), h(4), h(1)]), v);
    let e2 = sub(corner_of([h(2), h(3), h(5)]), v);
    (dot(e1, e2) / norm(e1) / norm(e2)).acos().to_degrees()
}

/// The turn at the top–front–wall vertex: the top edge along the front
/// (wall to right face), and the top edge along the wall (front to back).
pub(crate) const TURN: [Req; 2] = [req([2, 4], [3, 1]), req([2, 3], [4, 5])];

/// Chamfer `reqs` of the convex `p` at `d`, built, its `ΔV` the clip
/// oracle's to `1e-10` relative to the cube of the scale; returns the
/// count of non-trivalent vertices.
fn builds_at_the_clip_oracle(p: &Poly, reqs: &[Req], d: f64, scale: f64, what: &str) -> usize {
    let out = chamfered(&p.body, &p.edges(reqs), d, what)
        .unwrap_or_else(|e| panic!("{what}: builds, got {e}"));
    let removed = volume(&p.body) - volume(&out.body);
    let want = p.taken(reqs, d);
    assert!(
        (removed - want).abs() < 1e-10 * scale.powi(3).max(1.0),
        "{what}: ΔV {removed} against the clip oracle's {want}"
    );
    non_trivalent(&out.body)
}

// ------------------------------------------------------------------
// The Boolean-slab oracle.
// ------------------------------------------------------------------

/// A large slab occupying `{n·x ≥ c}` near the origin (`r` its half-size).
fn slab(n: V, c: f64, r: f64) -> Body<f64> {
    let n = unit(n);
    let seed = if n[0].abs() < 0.9 {
        [1.0, 0.0, 0.0]
    } else {
        [0.0, 1.0, 0.0]
    };
    let u = unit(cross(seed, n));
    let v = cross(n, u);
    let o = sub(sub(mul(n, c), mul(u, r)), mul(v, r));
    prism_on(
        sketch_from_axes(pt(o), vc(u), vc(v), tol()),
        corners(&[
            (0.0, 0.0),
            (2.0 * r, 0.0),
            (2.0 * r, 2.0 * r),
            (0.0, 2.0 * r),
        ]),
        r,
        tol(),
    )
}

/// `body ∩ {n·x ≤ c}`, through the kernel's Boolean.
fn cut(body: &Body<f64>, n: V, c: f64, r: f64) -> Body<f64> {
    realized(BooleanOp::Subtract, body, &slab(n, c, r), tol())
}

/// The kept half-space of an equal-setback chamfer of the convex edge
/// through `x0` between faces of unit outward normals `na`, `nb`: the
/// bisector plane, offset `d·cos(β/2)` in, `β` the interior dihedral.
fn chamfer_keep(na: V, nb: V, x0: V, d: f64) -> Half {
    let (na, nb) = (unit(na), unit(nb));
    let beta = core::f64::consts::PI - norm(cross(na, nb)).atan2(dot(na, nb));
    let m = unit(add(na, nb));
    (m, dot(m, x0) - d * (beta / 2.0).cos())
}

/// The vertex sets of `a` and `b` agree to `eps`.
fn same_vertices(a: &Body<f64>, b: &Body<f64>, eps: f64) -> Result<(), String> {
    let pa: Vec<V> = a.vertices().map(|(v, _)| point_at(a, v)).collect();
    let pb: Vec<V> = b.vertices().map(|(v, _)| point_at(b, v)).collect();
    let missing = |xs: &[V], ys: &[V]| -> Vec<V> {
        xs.iter()
            .copied()
            .filter(|p| !ys.iter().any(|q| norm(sub(*p, *q)) < eps))
            .collect()
    };
    let (only_a, only_b) = (missing(&pa, &pb), missing(&pb, &pa));
    if only_a.is_empty() && only_b.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "built {} against the oracle's {} vertices; built only {only_a:?}; oracle only {only_b:?}",
            pa.len(),
            pb.len()
        ))
    }
}

/// The operand less one slab per requested edge of the convex `body`
/// whose half-spaces are `halves`, each edge's two faces read off
/// `halves` by incidence.
fn slab_oracle(body: &Body<f64>, edges: &[EdgeKey], halves: &[Half], d: f64, r: f64) -> Body<f64> {
    let mut out = body.clone();
    for &e in edges {
        let [a, b] = ends(body, e).map(|v| point_at(body, v));
        let scale = norm(a).max(norm(b)).max(1.0);
        let on: Vec<V> = halves
            .iter()
            .filter(|(n, c)| {
                (dot(*n, a) - c).abs() < 1e-9 * scale && (dot(*n, b) - c).abs() < 1e-9 * scale
            })
            .map(|(n, _)| *n)
            .collect();
        let [na, nb] = on[..] else {
            panic!("an edge on two of the half-spaces, got {}", on.len())
        };
        let (m, c) = chamfer_keep(na, nb, a, d);
        out = cut(&out, m, c, r);
    }
    out
}

/// The built body against the slab oracle: its vertex set to `1e-9`
/// relative to `k`, its volume to `1e-10·k³`.
fn matches_the_slab_oracle(
    body: &Body<f64>,
    edges: &[EdgeKey],
    halves: &[Half],
    built: &Body<f64>,
    d: f64,
    k: f64,
) -> Result<(), String> {
    let want = slab_oracle(body, edges, halves, d, 20.0 * k);
    same_vertices(built, &want, 1e-9 * k)?;
    let (got, exp) = (volume(built), volume(&want));
    if (got - exp).abs() > 1e-10 * k * k * k {
        return Err(format!("volume {got} against the oracle's {exp}"));
    }
    Ok(())
}

// ------------------------------------------------------------------
// The rows.
// ------------------------------------------------------------------

/// **The bracket's `y = 0` corner** (`leaning_turn(−1)`, the top–front
/// edge at `90°` and the extrusion edge at the `45°` wall) against a
/// closed form integrated by hand: `ΔV = V₁ + V₂ − V₁₂`, `V₁ = 3d²/2 −
/// d³/6` the top–front band's, `V₂ = 1.5·d²/(2√2)` the wall band's, and
/// `V₁₂ = d³·(1/√2 − (√2+1)/4 + 1/6)` what both take; every vertex
/// trivalent.
#[test]
fn the_bracket_corner_matches_a_hand_integrated_closed_form() {
    for d in [0.05, 0.1, 0.3, 0.6] {
        let what = format!("d {d}");
        let (body, edges) = leaning_turn(-1.0);
        let out = chamfered(&body, &edges, d, &what)
            .unwrap_or_else(|e| panic!("{what}: builds, got {e}"));
        assert_eq!(
            non_trivalent(&out.body),
            0,
            "{what}: every vertex trivalent"
        );
        let r2 = core::f64::consts::SQRT_2;
        let v1 = 1.5 * d * d - d * d * d / 6.0;
        let v2 = 1.5 * d * d / (2.0 * r2);
        let v12 = d * d * d * (1.0 / r2 - (r2 + 1.0) / 4.0 + 1.0 / 6.0);
        let removed = volume(&body) - volume(&out.body);
        assert!(
            (removed - (v1 + v2 - v12)).abs() < 1e-12,
            "{what}: ΔV {removed} against the hand closed form {}",
            v1 + v2 - v12
        );
    }
}

/// **Leaning prisms over leans, setbacks and scales**, both request
/// orders, against the slab oracle: every one builds, every vertex
/// trivalent, the vertex set and volume the oracle's. The small scale
/// is `10⁷` zero widths (`0.01` at the default band), so its shortest
/// edges stand the same count of band widths long at every eps row.
#[test]
fn leaning_prisms_match_the_slab_oracle_over_leans_setbacks_and_scales() {
    // Pinned since PR 5728 (limb 2 for every carrier): at ε = 1e-12 a leaning
    // prism (extent 2000) refuses to extrude as limb 2's kernel limit on a
    // plane, bound 1.08e-12. Whether that is an accepted kernel limit or a
    // regression is the question of
    // `work/offset/main-red-five-slow-rows-refuse-at-limb-2s-kernel-limit-since-5728`;
    // until it closes, the row stops there, and the pin goes red the day the
    // row passes.
    test_utils::kernel_limit::pinned_row(
        geom_core::Tol::witness().eps(),
        &[1e-12],
        "the leaning prisms",
        leaning_prisms_match_the_slab_oracle_over_leans_setbacks_and_scales_row,
    );
}

/// The row [`leaning_prisms_match_the_slab_oracle_over_leans_setbacks_and_scales`]
/// runs, under its pin.
fn leaning_prisms_match_the_slab_oracle_over_leans_setbacks_and_scales_row() {
    let mut failures = Vec::new();
    let small = 1e7
        * geom_core::Band::linear(tol())
            .expect("the run's band")
            .zero();
    for k in [1.0, small, 100.0] {
        for s in [-2.0, -1.0, -0.5, 0.25, 0.5, 1.0, 1.5] {
            let p = leaning(s, 1.0, k);
            for d in [0.05, 0.1, 0.3] {
                for order in [[0, 1], [1, 0]] {
                    let what = format!("lean {s} scale {k} d {d} order {order:?}");
                    let edges = p.edges(&order.map(|i| TURN[i]));
                    match chamfered(&p.body, &edges, d * k, &what) {
                        Ok(out) => {
                            if non_trivalent(&out.body) != 0 {
                                failures.push(format!("{what}: a vertex not trivalent"));
                            }
                            if let Err(e) = matches_the_slab_oracle(
                                &p.body,
                                &edges,
                                &p.halves,
                                &out.body,
                                d * k,
                                k,
                            ) {
                                failures.push(format!("{what}: {e}"));
                            }
                        }
                        Err(e) => failures.push(format!("{what}: refused {e}")),
                    }
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// **Generic hexahedra**: seven corners with no right angle, the top
/// face's angle at the turn from `≈49°` to `≈130°`, at three setbacks
/// and both request orders, build at the clip oracle's `ΔV`, every vertex
/// trivalent; two of them as voids fill at it.
#[test]
fn generic_hexahedra_build_and_fill_at_the_clip_oracle() {
    for (s, a, b, c) in [
        (0.4, 0.2, 0.3, 0.0),
        (-0.6, -0.15, 0.25, 0.0),
        (0.7, 0.3, -0.2, 0.0),
        (-0.3, 0.25, -0.35, 0.0),
        (0.4, 0.2, 0.3, 0.8),
        (-0.6, -0.15, 0.25, -0.7),
        (0.5, 0.1, 0.2, -0.5),
    ] {
        let p = hexahedron(Hex { s, a, b, c, ..HEX });
        let theta = theta_s(&p);
        for d in [0.05, 0.1, 0.2] {
            for order in [[0, 1], [1, 0]] {
                let what =
                    format!("hexahedron ({s}, {a}, {b}, {c}), θ_S {theta:.1}°, d {d}, {order:?}");
                let n = builds_at_the_clip_oracle(&p, &order.map(|i| TURN[i]), d, 1.0, &what);
                assert_eq!(n, 0, "{what}: every vertex trivalent");
            }
        }
    }
    for (s, a, b, c) in [(0.4, 0.2, 0.3, 0.5), (-0.6, -0.15, 0.25, -0.5)] {
        let what = format!("the hexahedron ({s}, {a}, {b}, {c}) as a void");
        let p = hexahedron(Hex { s, a, b, c, ..HEX });
        let block = brick(Point3::new(-5.0, -5.0, -5.0), Point3::new(5.0, 5.0, 5.0));
        let void = realized(BooleanOp::Subtract, &block, &p.body, tol());
        let edges: Vec<EdgeKey> = TURN
            .iter()
            .map(|r| {
                let (_, a, b) = p.edge(*r);
                edge_near(&void, a, b)
            })
            .collect();
        let out =
            chamfered(&void, &edges, D, &what).unwrap_or_else(|e| panic!("{what}: fills, got {e}"));
        assert_eq!(
            non_trivalent(&out.body),
            0,
            "{what}: every vertex trivalent"
        );
        let added = volume(&out.body) - volume(&void);
        let want = p.taken(&TURN, D);
        assert!(
            (added - want).abs() < 1e-10,
            "{what}: ΔV {added} against the clip oracle's {want}"
        );
    }
}

/// **Whole rims**, so one band is the far band at one turn and the near
/// band at the next (two overruns and two mitres per rim of the leaning
/// prism), and with both rims requested two turns share each lateral
/// edge; prisms down to `0.2` tall push those turns' feet together. Every
/// build is at the clip oracle's `ΔV`, its only non-trivalent vertices
/// the isosceles mitres' common feet at `x = 2`. Both rims of the
/// `0.2`-tall prism at `d = 0.1` meet on the front face and refuse at a
/// clearance meter; everything else builds. Two hexahedra's top rims
/// build at the oracle too.
#[test]
fn whole_rims_build_at_the_clip_oracle() {
    let top = [
        req([2, 4], [3, 1]),
        req([2, 1], [4, 5]),
        req([2, 5], [1, 3]),
        req([2, 3], [5, 4]),
    ];
    let bottom = [
        req([0, 4], [3, 1]),
        req([0, 1], [4, 5]),
        req([0, 5], [1, 3]),
        req([0, 3], [5, 4]),
    ];
    let both: Vec<Req> = top.iter().chain(&bottom).copied().collect();
    for s in [-1.0, -0.5, 0.5, 1.0] {
        for h in [1.0, 0.5, 0.4, 0.33, 0.3, 0.27, 0.25, 0.22, 0.2] {
            let p = leaning(s, h, 1.0);
            for (name, reqs) in [("top", &top[..]), ("both", &both[..])] {
                let what = format!("lean {s}, height {h}, {name} rim");
                if name == "both" && h <= 0.2 {
                    match chamfered(&p.body, &p.edges(reqs), D, &what) {
                        Err(e) => assert!(
                            a_clearance(&e),
                            "{what}: refuses at a clearance meter, got {e}"
                        ),
                        Ok(_) => panic!("{what}: the two rims' strips meet on the front face"),
                    }
                    continue;
                }
                let out = chamfered(&p.body, &p.edges(reqs), D, &what)
                    .unwrap_or_else(|e| panic!("{what}: builds, got {e}"));
                let removed = volume(&p.body) - volume(&out.body);
                let want = p.taken(reqs, D);
                assert!(
                    (removed - want).abs() < 1e-10,
                    "{what}: ΔV {removed} against the clip oracle's {want}"
                );
                for (v, _) in out.body.vertices() {
                    let x = point_at(&out.body, v);
                    assert!(
                        out.body.edges_of_vertex(v).expect("a fan").len() == 3
                            || (x[0] - 2.0).abs() < 1e-12,
                        "{what}: a vertex at {x:?} off the isosceles mitres is not trivalent"
                    );
                }
            }
        }
    }
    for (s, a, b, c) in [(0.4, 0.2, 0.3, 0.5), (-0.6, -0.15, 0.25, -0.5)] {
        let p = hexahedron(Hex { s, a, b, c, ..HEX });
        builds_at_the_clip_oracle(
            &p,
            &top,
            D,
            1.0,
            &format!("hexahedron ({s}, {a}, {b}, {c}) top rim"),
        );
    }
}

/// **Two turns on one `L`**: `L` along `z` from `V = 0` to `W = (0, 0, ℓ)`,
/// its faces the planes through `z` of outward normals `n₁`, `n₂`, the
/// shared face at `V` tilted below and at `W` above, and all four of
/// those faces' edges at `V` and `W` requested, so each end of `L` is a
/// turn and the two overruns meet on `L`'s faces. Every build matches the
/// slab oracle; the rest refuse typed (the two cut-offs' feet crossing on
/// a rim, or a clearance meter). Of the 27 polytopes 23 build at the
/// default band; the row holds at least 20 to.
#[test]
fn two_turns_on_one_l_match_the_slab_oracle() {
    let mut failures = Vec::new();
    let mut built = 0;
    for (a1, a2) in [(200.0f64, 300.0f64), (190.0, 320.0), (210.0, 290.0)] {
        for (tv, tw) in [
            ([0.6, 0.2], [-0.5, 0.3]),
            ([-0.7, 0.1], [0.4, -0.6]),
            ([0.3, -0.8], [0.9, 0.2]),
        ] {
            for ell in [0.25, 0.4, 0.8] {
                let what = format!("({a1}°, {a2}°) {tv:?} {tw:?} ℓ {ell}");
                let n1 = [a1.to_radians().cos(), a1.to_radians().sin(), 0.0];
                let n2 = [a2.to_radians().cos(), a2.to_radians().sin(), 0.0];
                let nv = unit([tv[0], tv[1], -1.0]);
                let nw = unit([tw[0], tw[1], 1.0]);
                let w = [0.0, 0.0, ell];
                let halves = vec![
                    (n1, 0.0),
                    (n2, 0.0),
                    (nv, 0.0),
                    (nw, dot(nw, w)),
                    ([1.0, 0.0, 0.0], 3.0),
                    ([-1.0, 0.0, 0.0], 3.0),
                    ([0.0, 1.0, 0.0], 3.0),
                    ([0.0, -1.0, 0.0], 3.0),
                ];
                let mut body = brick(Point3::new(-3.0, -3.0, -3.0), Point3::new(3.0, 3.0, 3.0));
                for (n, c) in &halves[..4] {
                    body = cut(&body, *n, *c, 20.0);
                }
                let at = |p: V| {
                    body.vertices()
                        .map(|(v, _)| v)
                        .find(|v| norm(sub(point_at(&body, *v), p)) < 1e-9)
                        .unwrap_or_else(|| panic!("{what}: a vertex at {p:?}"))
                };
                let on = |e: EdgeKey, n: V, c: f64| {
                    ends(&body, e)
                        .iter()
                        .all(|v| (dot(n, point_at(&body, *v)) - c).abs() < 1e-9)
                };
                let mut edges = Vec::new();
                for (x, n, c) in [(at([0.0; 3]), nv, 0.0), (at(w), nw, dot(nw, w))] {
                    for e in body.edges_of_vertex(x).expect("a fan") {
                        if on(e, n, c) && (on(e, n1, 0.0) || on(e, n2, 0.0)) {
                            edges.push(e);
                        }
                    }
                }
                assert_eq!(
                    edges.len(),
                    4,
                    "{what}: two requested edges at each end of L"
                );
                match chamfered(&body, &edges, D, &what) {
                    Ok(out) => {
                        built += 1;
                        if let Err(e) =
                            matches_the_slab_oracle(&body, &edges, &halves, &out.body, D, 1.0)
                        {
                            failures.push(format!("{what}: {e}"));
                        }
                    }
                    Err(e) => {
                        let typed =
                            a_clearance(&e) || matches!(e, BlendError::UnsupportedRunOut { .. });
                        if !typed {
                            failures.push(format!("{what}: refused untyped, {e}"));
                        }
                    }
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    assert!(built >= 20, "two turns on one L: {built} of 27 built");
}

/// **The bracket's two corners at `Interval`** (leaning `∓1`): built,
/// tier 3, the volume enclosure holding the clip oracle and narrow.
#[test]
fn the_bracket_corners_carve_at_the_certified_scalar() {
    for s in [-1.0, 1.0] {
        let p = leaning(s, 1.0, 1.0);
        let want = p.taken(&TURN, D);
        let (body, edges) = leaning_turn_at::<Interval>(s);
        let body = sweep::test_support::finished("operand", body, tol());
        let out = chamfer_edges(&body, &edges, iv(D), tol())
            .unwrap_or_else(|e| panic!("lean {s}: builds at Interval, got {:?}", e.error));
        validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("lean {s}: tier 3, {e:?}"));
        let before =
            topo::mass_properties(&body, tol(), topo::test_support::pinned_resolution(tol()))
                .expect("mass properties")
                .met()
                .expect("mass properties")
                .volume;
        let after = topo::mass_properties(
            &topo::test_support::finished("the measured body", topo::Body::clone(&out.body), tol()),
            tol(),
            topo::test_support::pinned_resolution(tol()),
        )
        .expect("mass properties");
        let (removed, pad) = (
            before - after.enclosure.volume,
            after.enclosure.volume_pad.hi(),
        );
        assert!(
            removed.lo() - pad <= want && want <= removed.hi() + pad,
            "lean {s}: ΔV {removed:?} ± {pad} misses the clip oracle's {want}"
        );
        assert!(
            removed.hi() - removed.lo() + 2.0 * pad < 1e-6,
            "lean {s}: the enclosure is a claim, not a bracket of rounding"
        );
    }
}

/// The bracket's `y = 0` corner with a round pin of radius `0.004` on
/// the face through `at` of unit outward normal `n`: a boss standing
/// `0.03` out (`boss`), or a blind hole `depth` deep.
pub(crate) fn pinned(at: V, n: V, boss: bool, depth: f64) -> (Body<f64>, [EdgeKey; 2]) {
    let (plain, _) = leaning_turn(-1.0);
    let n = unit(n);
    let seed = if n[0].abs() < 0.9 {
        [1.0, 0.0, 0.0]
    } else {
        [0.0, 1.0, 0.0]
    };
    let u = unit(cross(seed, n));
    let (axis, start, h) = if boss {
        (n, sub(at, mul(n, 0.01)), 0.04)
    } else {
        (mul(n, -1.0), add(at, mul(n, 0.01)), 0.01 + depth)
    };
    let tool = prism_on(
        sketch_from_axes(pt(start), vc(u), vc(cross(axis, u)), tol()),
        vec![
            (Point2::new(-0.004, 0.0), 1.0),
            (Point2::new(0.004, 0.0), 1.0),
        ],
        h,
        tol(),
    );
    let op = if boss {
        BooleanOp::Union
    } else {
        BooleanOp::Subtract
    };
    let body = realized(op, &plain, &tool, tol());
    validate_geometric(&body, tol()).expect("the featured operand is tier-3 valid");
    let v = [-1.0, 0.0, 1.0];
    let edges = [
        edge_near(&body, v, [2.0, 0.0, 1.0]),
        edge_near(&body, v, [-1.0, -1.5, 1.0]),
    ];
    (body, edges)
}

/// The point `(a·d along L, b·d across)` on the bracket's `y = 0`
/// corner's leaning wall, from its vertex `(−1, 0, 1)`, and the wall's
/// outward normal.
pub(crate) fn on_wall(a: f64, b: f64) -> (V, V) {
    let r2 = core::f64::consts::FRAC_1_SQRT_2;
    (
        [-1.0 + a * D * r2, -b * D, 1.0 - a * D * r2],
        [-r2, 0.0, -r2],
    )
}

/// **Features in and around the overrun** at the bracket's `y = 0`
/// corner. On the leaning wall (`Fⱼ`) in the frame `(a along L, b
/// across)`, in units of `d`: band `j`'s trimline is `a = 1`, the
/// overrun the triangle `(1, 0)`, `(√2, 0)`, `q = (1, 1 − 1/√2)`, its
/// chord on `a + √2·b = √2`. On the front face (`F_k`, `y = 0`) band
/// `k`'s strip is `z ≥ 1 − d`. A feature the chamfer cuts into refuses;
/// one clear of everything it takes builds, at the feature-free `ΔV`.
#[test]
fn features_in_and_around_the_overrun_refuse_or_build_as_their_regions_say() {
    let (plain, plain_edges) = leaning_turn(-1.0);
    let plain_out =
        chamfered(&plain, &plain_edges, D, "feature-free").expect("the bracket corner builds");
    let free = volume(&plain) - volume(&plain_out.body);
    let front = |x: f64, z: f64| ([x, 0.0, z], [0.0, 1.0, 0.0]);
    // (what, where, boss or hole depth, builds)
    let rows = [
        ("a wall boss in the overrun", on_wall(1.1, 0.1), None, false),
        (
            "a shallow wall hole in the overrun",
            on_wall(1.1, 0.1),
            Some(0.002),
            false,
        ),
        (
            "a deep wall hole in the overrun",
            on_wall(1.1, 0.1),
            Some(0.05),
            false,
        ),
        ("a wall boss on the chord", on_wall(1.3, 0.1), None, false),
        (
            "a wall hole on the chord",
            on_wall(1.3, 0.1),
            Some(0.002),
            false,
        ),
        ("a wall boss past the chord", on_wall(1.45, 0.1), None, true),
        (
            "a wall hole past the chord",
            on_wall(1.45, 0.1),
            Some(0.002),
            true,
        ),
        ("a wall boss near p_k", on_wall(1.52, 0.05), None, true),
        ("a wall boss far along L", on_wall(3.0, 0.1), None, true),
        ("a front boss below p_k", front(-0.85, 0.885), None, true),
        (
            "a front boss on k's trimline",
            front(-0.85, 0.898),
            None,
            false,
        ),
        (
            "a front hole below p_k",
            front(-0.83, 0.89),
            Some(0.002),
            true,
        ),
    ];
    let mut failures = Vec::new();
    for (what, (at, n), hole, builds) in rows {
        let (body, edges) = pinned(at, n, hole.is_none(), hole.unwrap_or(0.0));
        match (chamfered(&body, &edges, D, what), builds) {
            (Ok(out), true) => {
                let removed = volume(&body) - volume(&out.body);
                if (removed - free).abs() > 1e-10 {
                    failures.push(format!(
                        "{what}: ΔV {removed} against the feature-free {free}"
                    ));
                }
            }
            (Ok(_), false) => failures.push(format!("{what}: built where the chamfer cuts it")),
            (Err(e), true) => failures.push(format!("{what}: refused where clear, {e}")),
            (Err(e), false) => {
                if !a_clearance(&e) {
                    failures.push(format!("{what}: refuses at a clearance meter, got {e}"));
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// **A void in the overrun's volume, touching no face**: a cube of
/// half-size `h` about a point `0.003` in from the leaning wall over the
/// overrun triangle, inside band `k`'s cut region and on band `j`'s kept
/// side. No meter sees it but the far band's reach, whose cap stays at
/// `Fⱼ`: it refuses the void as wholly inside the material that band
/// removes.
#[test]
fn a_void_in_the_overruns_volume_refuses() {
    let (plain, _) = leaning_turn(-1.0);
    let c = [-0.913, -0.005, 0.917];
    let s2 = 2f64.sqrt();
    assert!(c[1] + c[2] > 1.0 - D, "in band k's cut region");
    assert!(c[0] + (1.0 - s2) * c[2] > D - s2, "on band j's kept side");
    for h in [0.0012, 0.0005] {
        let void = brick(
            Point3::new(c[0] - h, c[1] - h, c[2] - h),
            Point3::new(c[0] + h, c[1] + h, c[2] + h),
        );
        let body = realized(BooleanOp::Subtract, &plain, &void, tol());
        let v = [-1.0, 0.0, 1.0];
        let edges = [
            edge_near(&body, v, [2.0, 0.0, 1.0]),
            edge_near(&body, v, [-1.0, -1.5, 1.0]),
        ];
        match chamfered(&body, &edges, D, "a void in the overrun") {
            Err(e) => assert!(
                matches!(e, BlendError::WhollyInside { .. }),
                "h {h}: refuses as wholly inside, got {e}"
            ),
            Ok(out) => panic!(
                "h {h}: a void in the overrun built, ΔV {}",
                volume(&body) - volume(&out.body)
            ),
        }
    }
}

/// **A boss clear of the overrun, inside the far band's whole cut-off
/// sliver, refuses** (`work/bandmeter/a-turns-overrun-meters-the-whole-cut-off-sliver.md`).
/// At `(1.15·d, 0.4·d)` on the bracket's `y = 0` leaning wall the boss
/// stands `0.13·d` past the overrun chord's line and `0.11·d` past band
/// `j`'s trimline, so nothing the chamfer takes reaches it; but it lies
/// in the bound arm (c) reads for the far band's whole sliver on that
/// wall, the triangle `(0, 0)`, `(√2·d, 0)`, `(0, d)`, and refuses
/// `RingClearance` there. Without the turn's sliver in the ring pass it
/// builds at the feature-free `ΔV`. The row pins the refusal; the item's
/// fix flips it.
#[test]
fn a_boss_clear_of_the_overrun_inside_the_far_bands_sliver_refuses() {
    let (at, n) = on_wall(1.15, 0.4);
    let (body, edges) = pinned(at, n, true, 0.0);
    match chamfered(&body, &edges, D, "a boss past q") {
        Err(e) => assert!(
            matches!(e, BlendError::RingClearance { .. }),
            "a boss past q refuses at the far band's sliver, got {e}"
        ),
        Ok(_) => panic!(
            "a boss past q built: `a-turns-overrun-meters-the-whole-cut-off-sliver` is fixed, \
             flip this row to hold the build to the feature-free ΔV"
        ),
    }
}

/// **A short near edge at an obtuse shared-face angle refuses
/// `FOOT_INSIDE_A_FACE` where the overrun itself fits**
/// (`work/bandmeter/an-overruns-sliver-refuses-a-short-near-edge.md`). The
/// far band's whole sliver on the near face needs that band's trimline on
/// the shared face to meet the near band's requested edge inside its
/// span, at `d / sin θ_S` from the vertex; a near edge shorter than that
/// but long enough for its own band refuses at the sliver's foot there.
///
/// - The hexahedron `(s 0.4, c 1)`, `θ_S ≈ 136.1°`, its near edge cut to
///   `0.13` and `0.11`, refuses so; at `0.15` it builds at the clip
///   oracle. At each refusal the far band's half-space takes beyond the
///   near band's exactly what it takes on the long fixture, so the true
///   body is the ordinary overrun.
/// - The cone `φ = (60°, 90°)`, `γ = 150°` (`θ_S ≈ 138.6°`, `d / sin θ_S
///   ≈ 0.151`), its near edge `e₂` cut to `0.12` and `0.14`, refuses so
///   while the near band alone builds; at `0.16` the turn builds.
///
/// The row pins the refusals; the item's fix flips them.
#[test]
fn a_short_near_edge_refuses_foot_inside_a_face_where_the_overrun_fits() {
    // What band `k`'s half-space takes beyond band `j`'s: `vol(B ∩ Hⱼ) −
    // vol(B ∩ Hⱼ ∩ H_k)`, the near band `j` the top–wall edge here.
    let beyond_near = |p: &Poly| {
        let mut hj = p.halves.clone();
        hj.push(p.cut(TURN[1], D));
        let mut hjk = hj.clone();
        hjk.push(p.cut(TURN[0], D));
        clipped_volume(&hj) - clipped_volume(&hjk)
    };
    let base = Hex {
        c: 1.0,
        depth: 4.0,
        x1: 3.0,
        ..HEX
    };
    let long = hexahedron(base);
    let theta = theta_s(&long);
    assert!(
        (theta - 136.1).abs() < 0.05,
        "the shared face's angle, {theta}°"
    );
    let v = corner_of([long.halves[2], long.halves[3], long.halves[4]]);
    let local = beyond_near(&long);
    let short = |len: f64| {
        hexahedron(Hex {
            depth: len - v[1],
            ..base
        })
    };
    for len in [0.13, 0.11] {
        let p = short(len);
        let what = format!("the hexahedron's near edge at {len}");
        assert!(
            (beyond_near(&p) - local).abs() < 1e-12,
            "{what}: the far band takes beyond the near one what it takes on the long fixture"
        );
        match chamfered(&p.body, &p.edges(&TURN), D, &what) {
            Err(e) => assert!(
                foot_inside_a_face(&e),
                "{what}: refuses FOOT_INSIDE_A_FACE, got {e}"
            ),
            Ok(_) => panic!(
                "{what} built: `an-overruns-sliver-refuses-a-short-near-edge` is fixed, flip this row \
                 to hold the build to the clip oracle"
            ),
        }
    }
    builds_at_the_clip_oracle(
        &short(0.15),
        &TURN,
        D,
        1.0,
        "the hexahedron's near edge at 0.15",
    );

    let cone = Cone::angles(
        [60f64.to_radians(), 90f64.to_radians()],
        150f64.to_radians(),
    );
    for (ell, refuses) in [(0.12, true), (0.14, true), (0.16, false)] {
        let what = format!("the cone's near edge at {ell}");
        let (wedge, wedge_edges) = corner(&cone);
        let [a, b] = ends(&wedge, wedge_edges[1]).map(|v| point_at(&wedge, v));
        let e2 = if norm(a) < 1e-12 { unit(b) } else { unit(a) };
        let body = cut(&wedge, e2, ell, 40.0);
        let out_of_the_vertex = |dir: V| {
            topo::query::all_edges(&body)
                .into_iter()
                .find(|&e| {
                    let [x, y] = ends(&body, e).map(|v| point_at(&body, v));
                    (norm(x) < 1e-12 && norm(sub(unit(y), dir)) < 1e-9)
                        || (norm(y) < 1e-12 && norm(sub(unit(x), dir)) < 1e-9)
                })
                .unwrap_or_else(|| panic!("{what}: an edge out of the vertex"))
        };
        let edges = [out_of_the_vertex(cone.e[0]), out_of_the_vertex(cone.e[1])];
        let turn = chamfered(&body, &edges, D, &what);
        if refuses {
            match turn {
                Err(e) => assert!(
                    foot_inside_a_face(&e),
                    "{what}: refuses FOOT_INSIDE_A_FACE, got {e}"
                ),
                Ok(_) => panic!(
                    "{what} built: `an-overruns-sliver-refuses-a-short-near-edge` is fixed, flip \
                     this row"
                ),
            }
            chamfered(&body, &edges[1..], D, &what)
                .unwrap_or_else(|e| panic!("{what}: the near band alone builds, got {e}"));
        } else {
            turn.unwrap_or_else(|e| panic!("{what}: builds, got {e}"));
        }
    }
}
