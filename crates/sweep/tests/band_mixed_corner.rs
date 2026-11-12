//! **A mixed corner whose three edges are all requested**
//! (`work/band/a-boss-on-a-plate-meets-it-at-a-mixed-convexity-corner-no-run-out-rounds.md`,
//! "All three"): the odd band is cut off in `E`, the face the odd edge
//! does not lie in, and the pair runs on through a link resting on `E`
//! and on the odd band — a fillet's torus about the odd band's axis,
//! major radius `2r`, or a chamfer's strip mitred onto the pair's strips.
//!
//! The closed forms, with `A = r²(1 − π/4)`, `d = r(10 − 3π)/(12 − 3π)`
//! (the round's section and its centroid's inset), `K = 5/3 − 7√2/6`,
//! the odd edge `L` long from `E` and the pair edges `Y` and `Z` long
//! from the corner, each other end cut off square:
//!
//! - flush (the pair convex), fillet: `ΔV = A(L − Y − Z + 2r) − (π/2)·A(r + d)`
//!   — the odd fill run to `E`, the pair's lines from the link's ends,
//!   and the link's piece by Pappus at `r + d` from the odd axis;
//! - flush, chamfer: `ΔV = (s²/2)(L − Y − Z) + K·s³`, the pair's strips
//!   mitred round the chord the odd strip leaves in `E` (the mitred
//!   offset's `w² tan(α/2)` at each of its two 45° turns);
//! - inset (the pair concave): each the flush one negated, the inset's
//!   void being the flush's material reflected.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use geom::Surface;
use geom_core::{Bounds, CertifiedBounds, Decide, Point2, Point3, Tol};
use sweep::blend::build::{Blended, fillet_edges};
use sweep::blend::{BlendError, CornerConfig};
use sweep::chamfer::chamfer_edges;
use sweep::test_support::{
    assert_naming_totality, at_rest, corners, cylinder_of_arcs_at, prism_at, realized,
};
use topo::boolean::BooleanOp;
use topo::{Body, EdgeKey, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

/// The size every single-corner row asks for, meters.
const R: f64 = 0.1;

/// The round's section and its centroid's inset from each face.
fn section(r: f64) -> (f64, f64) {
    (
        r * r * (1.0 - PI / 4.0),
        r * (10.0 - 3.0 * PI) / (12.0 - 3.0 * PI),
    )
}

/// The chamfer's mitre constant at the chord's two 45° turns.
fn k() -> f64 {
    5.0 / 3.0 - 7.0 * 2f64.sqrt() / 6.0
}

#[derive(Clone, Copy, Debug)]
enum Verb {
    Fillet,
    Chamfer,
}

impl Verb {
    fn run(
        self,
        body: &Body<f64>,
        edges: &[EdgeKey],
        size: f64,
    ) -> Result<Blended<f64>, BlendError> {
        let operand = at_rest(body, tol());
        match self {
            Self::Fillet => fillet_edges(&operand, edges, size, tol()),
            Self::Chamfer => chamfer_edges(&operand, edges, size, tol()),
        }
        .map_err(|r| r.error)
    }
}

type P = (f64, f64, f64);

fn point(body: &Body<f64>, v: topo::VertexKey) -> P {
    let p = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    (p.x, p.y, p.z)
}

/// The one edge between `a` and `b`, either way round.
fn edge(body: &Body<f64>, a: P, b: P) -> EdgeKey {
    let near = |p: P, q: P| (p.0 - q.0).hypot(p.1 - q.1).hypot(p.2 - q.2) < 1e-9;
    let found: Vec<EdgeKey> = topo::query::all_edges(body)
        .into_iter()
        .filter(|&e| {
            let (s, t) = body.edge_vertices(e).unwrap();
            let (s, t) = (point(body, s), point(body, t));
            (near(s, a) && near(t, b)) || (near(s, b) && near(t, a))
        })
        .collect();
    assert_eq!(found.len(), 1, "one edge from {a:?} to {b:?}");
    found[0]
}

/// **The flush corner**: the L `{y < 0} ∪ {x < 0}` over `[−1, 1]²`, up
/// `z ∈ [0, 2]`. The odd edge is the reflex vertical at the origin, 2
/// long; `E` is the floor; the pair the floor's edges from the origin
/// along `x` and `y`, each 1 long.
fn flush() -> (Body<f64>, [EdgeKey; 3]) {
    let body = prism_at(
        corners(&[
            (-1.0, -1.0),
            (1.0, -1.0),
            (1.0, 0.0),
            (0.0, 0.0),
            (0.0, 1.0),
            (-1.0, 1.0),
        ]),
        0.0,
        2.0,
        tol(),
    );
    let o = (0.0, 0.0, 0.0);
    let edges = [
        edge(&body, o, (0.0, 0.0, 2.0)),
        edge(&body, o, (1.0, 0.0, 0.0)),
        edge(&body, o, (0.0, 1.0, 0.0)),
    ];
    (body, edges)
}

/// **The inset corner**: a plate `[−1, 1]² × [−1, 0]` and a boss
/// `[−1, 0]² × [0, 1]` in its corner, flush with its `x = −1` and
/// `y = −1` sides. The odd edge is the boss's vertical at the origin, 1
/// long; `E` is the plate's top; the pair the boss's base edges from the
/// origin, each 1 long, ending at the flush sides where each is cut off
/// alone.
fn inset() -> (Body<f64>, [EdgeKey; 3]) {
    let plate = prism_at(
        corners(&[(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
        -1.0,
        1.0,
        tol(),
    );
    let boss = prism_at(
        corners(&[(-1.0, -1.0), (0.0, -1.0), (0.0, 0.0), (-1.0, 0.0)]),
        -0.5,
        1.5,
        tol(),
    );
    let body = realized(BooleanOp::Union, &plate, &boss, tol());
    let o = (0.0, 0.0, 0.0);
    let edges = [
        edge(&body, o, (0.0, 0.0, 1.0)),
        edge(&body, o, (0.0, -1.0, 0.0)),
        edge(&body, o, (-1.0, 0.0, 0.0)),
    ];
    (body, edges)
}

/// **The bracket** of the row: a 6 × 4 × 0.5 plate, an upright
/// 0.5 × 3 boss flush with its `x = 0` side standing to `z = 2`, and a
/// two-arc bore of radius 0.3 through the plate at (3, 2). Two flush
/// mixed corners (two of three edges convex) and two inset ones (one).
fn bracket(bore: bool) -> Body<f64> {
    let plate = prism_at(
        corners(&[(0.0, 0.0), (6.0, 0.0), (6.0, 4.0), (0.0, 4.0)]),
        0.0,
        0.5,
        tol(),
    );
    let boss = prism_at(
        corners(&[(0.0, 0.5), (0.5, 0.5), (0.5, 3.5), (0.0, 3.5)]),
        0.25,
        1.75,
        tol(),
    );
    let body = realized(BooleanOp::Union, &plate, &boss, tol());
    if !bore {
        return body;
    }
    let bore = cylinder_of_arcs_at(2, 0.3, Point2::new(3.0, 2.0), -0.5, 1.5, tol());
    realized(BooleanOp::Subtract, &body, &bore, tol())
}

fn every_edge(body: &Body<f64>) -> Vec<EdgeKey> {
    let mut all = topo::query::all_edges(body);
    all.sort_unstable();
    all
}

fn volume<
    T: Decide + Bounds + CertifiedBounds + topo::AtRestPolicy<Finished = topo::AtRestBody<T>>,
>(
    body: &Body<T>,
) -> (f64, f64) {
    let p = topo::mass_properties(
        &topo::test_support::finished("the measured body", topo::Body::clone(body), tol()),
        tol(),
        topo::test_support::pinned_resolution(tol()),
    )
    .expect("certified props");
    let (lo, hi) = (p.enclosure.volume.lo(), p.enclosure.volume.hi());
    (0.5 * (lo + hi), 0.5 * (hi - lo) + p.enclosure.volume_pad)
}

/// How far `p` lies from a stored plane, cylinder, sphere or torus.
fn off(s: &Surface<f64>, p: Point3<f64>) -> f64 {
    match *s {
        Surface::Plane { origin, normal, .. } => (p - origin).dot(normal).abs(),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let d = p - origin;
            ((d - axis * d.dot(axis)).norm() - radius).abs()
        }
        Surface::Sphere { center, radius, .. } => ((p - center).norm() - radius).abs(),
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let d = p - center;
            let h = d.dot(axis);
            let rho = (d - axis * h).norm();
            ((rho - major_radius).hypot(h) - minor_radius).abs()
        }
        ref other => panic!("a corner edge joins a {other:?}"),
    }
}

/// The largest distance, over 65 points of each edge the corners minted
/// (the arcs where each band meets a patch and the patch's arcs on `E`),
/// from either face it joins.
fn corner_residual(out: &Blended<f64>) -> f64 {
    let rec = out.naming.as_ref().expect("the carve records its births");
    let mut worst = 0.0f64;
    for e in rec
        .arcs
        .iter()
        .map(|(e, _, _)| *e)
        .chain(rec.trims.iter().map(|(e, _, _)| *e))
    {
        let Some(edge) = out.body.get_edge(e) else {
            continue;
        };
        let c = out
            .body
            .get_curve_geom(edge.curve)
            .and_then(|g| g.certified())
            .expect("a certified edge");
        let surfaces = [edge.he_plus, edge.he_minus].map(|h| {
            let f = out.body.face_of_half_edge(h).expect("an edge's face");
            out.body.face_surface(f).expect("a stored surface").clone()
        });
        let (t0, t1) = c.params();
        for i in 0..=64 {
            let p = c.carrier().eval(t0 + (t1 - t0) * f64::from(i) / 64.0);
            for s in &surfaces {
                worst = worst.max(off(s, p));
            }
        }
    }
    worst
}

/// One build: tier 3, one closed shell of the source's genus, naming totality, every
/// corner edge on both its faces, and `ΔV` at `truth` within the
/// certified enclosure and `1e-12`.
fn builds(
    body: &Body<f64>,
    edges: &[EdgeKey],
    (verb, size): (Verb, f64),
    (truth, genus): (f64, i64),
    what: &str,
) -> Blended<f64> {
    let out = verb
        .run(body, edges, size)
        .unwrap_or_else(|e| panic!("{what} ({verb:?}): builds, got {e:?}"));
    validate_geometric(&out.body, tol())
        .unwrap_or_else(|e| panic!("{what} ({verb:?}): tier 3, got {e:?}"));
    let c = topo::readback::euler_counts(&out.body);
    assert_eq!(
        (c.s, c.genus()),
        (1, Ok(genus)),
        "{what} ({verb:?}): one closed shell of genus {genus}"
    );
    // The bore's seams are smooth, so the blend leaves them as they are.
    let blended: Vec<EdgeKey> = edges
        .iter()
        .copied()
        .filter(|&e| {
            let (a, b) = body.edge_vertices(e).unwrap();
            let (a, b) = (point(body, a), point(body, b));
            (a.0 - b.0).hypot(a.1 - b.1) > 1e-9 || (a.0 - 3.0).hypot(a.1 - 2.0) > 0.31
        })
        .collect();
    assert_naming_totality(body, &out, &blended, what);
    let ((v0, pad0), (v1, pad1)) = (volume(body), volume(&out.body));
    let (dv, pad) = (v1 - v0, pad0 + pad1);
    assert!(
        pad < 1e-9 && (dv - truth).abs() < 1e-12 + pad,
        "{what} ({verb:?}): ΔV {dv} ± {pad} vs the closed form {truth}"
    );
    let stray = corner_residual(&out);
    assert!(
        stray < 1e-12,
        "{what} ({verb:?}): a corner edge strays {stray} from a face it joins"
    );
    out
}

/// Each mixed corner leaves one patch, three arcs where the bands meet
/// it, the link's arc on `E` and two host feet on `E`, one per pair edge.
fn assert_corner_rows(out: &Blended<f64>, corners: usize, what: &str) {
    let rec = out.naming.as_ref().expect("births");
    assert_eq!(
        rec.host_feet.len(),
        2 * corners,
        "{what}: two host feet per mixed corner"
    );
    let mut keyed: Vec<_> = rec.host_feet.iter().map(|(_, v, e)| (*v, *e)).collect();
    keyed.sort_unstable();
    keyed.dedup();
    assert_eq!(
        keyed.len(),
        2 * corners,
        "{what}: each host foot keyed by its own pair edge"
    );
}

#[test]
fn a_flush_corner_with_all_three_edges_requested_builds_at_its_closed_form() {
    let (body, edges) = flush();
    let (a, d) = section(R);
    let fillet = a * (2.0 - 1.0 - 1.0 + 2.0 * R) - FRAC_PI_2 * a * (R + d);
    let out = builds(&body, &edges, (Verb::Fillet, R), (fillet, 0), "flush");
    assert_corner_rows(&out, 1, "flush fillet");
    let chamfer = R * R / 2.0 * (2.0 - 1.0 - 1.0) + k() * R.powi(3);
    let out = builds(&body, &edges, (Verb::Chamfer, R), (chamfer, 0), "flush");
    assert_corner_rows(&out, 1, "flush chamfer");
}

#[test]
fn an_inset_corner_with_all_three_edges_requested_builds_at_its_closed_form() {
    let (body, edges) = inset();
    let (a, d) = section(R);
    let fillet = a * (1.0 + 1.0 - 1.0 - 2.0 * R) + FRAC_PI_2 * a * (R + d);
    let out = builds(&body, &edges, (Verb::Fillet, R), (fillet, 0), "inset");
    assert_corner_rows(&out, 1, "inset fillet");
    let chamfer = R * R / 2.0 * (1.0 + 1.0 - 1.0) - k() * R.powi(3);
    let out = builds(&body, &edges, (Verb::Chamfer, R), (chamfer, 0), "inset");
    assert_corner_rows(&out, 1, "inset chamfer");
}

/// **The bracket filleted all round**, as "select all edges" asks: by
/// pieces, the straight bands between their ends (convex, 52 − 40r in
/// all, the odd verticals run down to `E`; concave, 4 − 4r, the odd
/// runs out to `E`), twelve octant corners at `r³(1 − π/6)` each, the
/// bore's two rims by Pappus at `0.3 + d`; the flush corners' links
/// and the inset ones' cancel, each `(π/2)·A(r + d)`.
#[test]
fn the_bracket_fillets_all_round_at_its_closed_form() {
    let body = bracket(true);
    let r: f64 = 0.05;
    let (a, d) = section(r);
    let truth =
        -a * (48.0 - 36.0 * r) - 12.0 * r.powi(3) * (1.0 - PI / 6.0) - 4.0 * PI * a * (0.3 + d);
    let out = builds(
        &body,
        &every_edge(&body),
        (Verb::Fillet, r),
        (truth, 1),
        "the bracket",
    );
    assert_corner_rows(&out, 4, "the bracket");
}

/// **The bracket without its bore chamfered all round** (a chamfer has
/// no band over the bore's arcs): the strips over each edge's whole
/// length, `s²/2` each (convex 52, concave 4), less `2s³/3` at each of
/// the twelve flat corners; the flush corners' `K·s³` and the inset
/// ones' cancel.
#[test]
fn the_bracket_without_its_bore_chamfers_all_round_at_its_closed_form() {
    let body = bracket(false);
    let s: f64 = 0.05;
    let truth = -s * s / 2.0 * 48.0 + 12.0 * 2.0 / 3.0 * s.powi(3);
    let out = builds(
        &body,
        &every_edge(&body),
        (Verb::Chamfer, s),
        (truth, 0),
        "the bore-less bracket",
    );
    assert_corner_rows(&out, 4, "the bore-less bracket");
}

/// The edges with an end at `p`.
fn at(body: &Body<f64>, p: P) -> Vec<EdgeKey> {
    let near = |q: P| (p.0 - q.0).hypot(p.1 - q.1).hypot(p.2 - q.2) < 1e-9;
    every_edge(body)
        .into_iter()
        .filter(|&e| {
            let (s, t) = body.edge_vertices(e).unwrap();
            near(point(body, s)) || near(point(body, t))
        })
        .collect()
}

/// **What the mixed corner does not build refuses typed**: a curved link
/// at the corner (a quarter-disc boss's base arc, the odd edge there), an
/// `E` oblique to the odd edge (the fillet's link would be a canal
/// surface; the chamfer's is not built), and an odd edge no longer than
/// the blend's size.
#[test]
fn a_mixed_corner_the_carve_does_not_build_refuses_typed() {
    let plate = |outline: &[(f64, f64)]| prism_at(corners(outline), -1.0, 1.0, tol());
    // The quarter-disc boss in the plate's corner, flush on two sides.
    let q = (FRAC_PI_2 / 4.0).tan();
    let disc = prism_at(
        vec![
            (Point2::new(-1.0, -1.0), 0.0),
            (Point2::new(0.0, -1.0), q),
            (Point2::new(-1.0, 0.0), 0.0),
        ],
        -0.5,
        1.5,
        tol(),
    );
    let square = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let body = realized(BooleanOp::Union, &plate(&square), &disc, tol());
    let got = Verb::Fillet.run(&body, &at(&body, (0.0, -1.0, 0.0)), R);
    assert!(
        matches!(
            got,
            Err(BlendError::UnsupportedCorner {
                corner: CornerConfig::MixedConvexity { convex: 2 },
                ..
            })
        ),
        "a curved odd link at a mixed corner, got {got:?}"
    );

    // An L-step whose end face `E` leans off square to the odd edge.
    let boss = prism_at(
        corners(&[(0.0, -1.0), (2.0, -1.0), (2.5, 0.0), (0.5, 0.0)]),
        -0.5,
        1.5,
        tol(),
    );
    let lean = [(0.0, -1.0), (2.0, -1.0), (3.0, 1.0), (1.0, 1.0)];
    let body = realized(BooleanOp::Union, &plate(&lean), &boss, tol());
    let edges = at(&body, (0.5, 0.0, 0.0));
    assert_eq!(edges.len(), 3, "the leaning corner's three edges");
    let got = Verb::Fillet.run(&body, &edges, R);
    assert!(
        matches!(got, Err(BlendError::SpineUnsupported { .. })),
        "a fillet's link on an oblique E is a canal surface, got {got:?}"
    );
    let got = Verb::Chamfer.run(&body, &edges, R);
    assert!(
        matches!(
            got,
            Err(BlendError::UnsupportedCorner {
                corner: CornerConfig::MixedConvexity { convex: 2 },
                ..
            })
        ),
        "a chamfer's link on an oblique E is not built, got {got:?}"
    );

    // The flush corner with its odd edge shorter than the size.
    let short = prism_at(
        corners(&[
            (-1.0, -1.0),
            (1.0, -1.0),
            (1.0, 0.0),
            (0.0, 0.0),
            (0.0, 1.0),
            (-1.0, 1.0),
        ]),
        0.0,
        0.08,
        tol(),
    );
    let edges = [
        edge(&short, (0.0, 0.0, 0.0), (0.0, 0.0, 0.08)),
        edge(&short, (0.0, 0.0, 0.0), (1.0, 0.0, 0.0)),
        edge(&short, (0.0, 0.0, 0.0), (0.0, 1.0, 0.0)),
    ];
    for verb in [Verb::Fillet, Verb::Chamfer] {
        let got = verb.run(&short, &edges, R);
        assert!(
            matches!(got, Err(BlendError::FaceClearanceUncertified { .. })),
            "an odd edge shorter than the size ({verb:?}), got {got:?}",
            got = got.as_ref().map(|_| ())
        );
    }
}
