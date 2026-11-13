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

/// The L `{y < 0} ∪ {x < 0}` over `[−1, 1]²`, up `z ∈ [0, h]`.
fn l_prism(h: f64) -> Body<f64> {
    prism_at(
        corners(&[
            (-1.0, -1.0),
            (1.0, -1.0),
            (1.0, 0.0),
            (0.0, 0.0),
            (0.0, 1.0),
            (-1.0, 1.0),
        ]),
        0.0,
        h,
        tol(),
    )
}

/// The edges of the L's flush corner: the reflex vertical at the origin,
/// `h` long, and the floor's edges from the origin along `x` and `y`.
fn l_corner(body: &Body<f64>, h: f64) -> [EdgeKey; 3] {
    let o = (0.0, 0.0, 0.0);
    [
        edge(body, o, (0.0, 0.0, h)),
        edge(body, o, (1.0, 0.0, 0.0)),
        edge(body, o, (0.0, 1.0, 0.0)),
    ]
}

/// **The flush corner**: the L prism 2 tall. The odd edge is the reflex
/// vertical at the origin, 2 long; `E` is the floor; the pair the
/// floor's edges from the origin along `x` and `y`, each 1 long.
fn flush() -> (Body<f64>, [EdgeKey; 3]) {
    let body = l_prism(2.0);
    let edges = l_corner(&body, 2.0);
    (body, edges)
}

/// **The flush corner at plan angle `φ`**: the material outside the
/// wedge of angle `φ` between `+x` and the direction at `φ`, up
/// `z ∈ [0, 2]`. The odd edge is the wedge's vertical at the origin, 2
/// long; `E` the floor; the pair the floor's edges along the wedge's
/// sides, each 1 long and ending square at a side of the outline.
fn flush_at(phi: f64) -> (Body<f64>, [EdgeKey; 3]) {
    let a = (1.0, 0.0);
    let c = (phi.cos(), phi.sin());
    // Each pair edge's end side runs from its end away from the wedge.
    let (a_out, c_out) = ((1.0, -1.0), (c.0 - phi.sin(), c.1 + phi.cos()));
    let back = (-1.5 * (phi / 2.0).cos(), -1.5 * (phi / 2.0).sin());
    let body = prism_at(
        corners(&[back, a_out, a, (0.0, 0.0), c, c_out]),
        0.0,
        2.0,
        tol(),
    );
    let o = (0.0, 0.0, 0.0);
    let edges = [
        edge(&body, o, (0.0, 0.0, 2.0)),
        edge(&body, o, (a.0, a.1, 0.0)),
        edge(&body, o, (c.0, c.1, 0.0)),
    ];
    (body, edges)
}

/// **The inset corner at plan angle `φ`**: a boss whose plan is the
/// wedge of angle `φ` at the origin between `−y` and the direction `φ`
/// clockwise from it, each side 1 long and closed square, standing on a
/// plate whose sides run flush with the boss's two closing sides. The
/// odd edge is the boss's vertical at the origin, 1 long; `E` the
/// plate's top; the pair the boss's base edges from the origin.
fn inset_at(phi: f64) -> (Body<f64>, [EdgeKey; 3]) {
    let a = (0.0, -1.0);
    let c = (-phi.sin(), -phi.cos());
    // The far corner, square to both sides: `b·a = b·c = 1`.
    let b = ((phi.cos() - 1.0) / phi.sin(), -1.0);
    let far = |p: (f64, f64), k: f64| (b.0 + k * (p.0 - b.0), b.1 + k * (p.1 - b.1));
    let (a3, c3) = (far(a, 3.0), far(c, 3.0));
    let plate = prism_at(
        corners(&[b, a3, (a3.0 + c3.0 - b.0, a3.1 + c3.1 - b.1), c3]),
        -1.0,
        1.0,
        tol(),
    );
    let boss = prism_at(corners(&[b, a, (0.0, 0.0), c]), -0.5, 1.5, tol());
    let body = realized(BooleanOp::Union, &plate, &boss, tol());
    let o = (0.0, 0.0, 0.0);
    let edges = [
        edge(&body, o, (0.0, 0.0, 1.0)),
        edge(&body, o, (a.0, a.1, 0.0)),
        edge(&body, o, (c.0, c.1, 0.0)),
    ];
    (body, edges)
}

/// The flush corner's fillet at plan angle `φ`, the odd edge `l` long
/// and the pair `y` and `z`: the odd run's section
/// `A_φ = r²(cot(φ/2) − (π − φ)/2)`, the pair's lines from the link's
/// ends `r·cot(φ/2)` out, and the link's piece by Pappus over `π − φ`.
fn fillet_flush(phi: f64, r: f64, (l, y, z): (f64, f64, f64)) -> f64 {
    let (a, d) = section(r);
    let cot = 1.0 / (phi / 2.0).tan();
    let a_phi = r * r * (cot - (PI - phi) / 2.0);
    a_phi * l - a * (y + z - 2.0 * r * cot) - (PI - phi) * a * (r + d)
}

/// The flush corner's chamfer at plan angle `φ`, the odd edge `l` long
/// and the pair `y` and `z`.
fn chamfer_flush(phi: f64, s: f64, (l, y, z): (f64, f64, f64)) -> f64 {
    let half = s * s / 2.0;
    half * phi.sin() * l
        - half * (y + z - 2.0 * s + 2.0 * s * (phi / 2.0).sin())
        - 2.0 * s.powi(3) / 3.0 * ((PI - phi) / 4.0).tan()
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
/// from either face it joins, and how many such edges it read.
fn corner_residual(out: &Blended<f64>) -> (f64, usize) {
    let rec = out.naming.as_ref().expect("the carve records its births");
    let mut worst = 0.0f64;
    let mut read = 0;
    for e in rec
        .arcs
        .iter()
        .map(|(e, _, _)| *e)
        .chain(rec.trims.iter().map(|(e, _, _)| *e))
    {
        let Some(edge) = out.body.get_edge(e) else {
            continue;
        };
        read += 1;
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
    (worst, read)
}

/// The requested edges whose two faces are both cylinders: a bore's
/// smooth seams, which the blend leaves as they are.
fn seams(body: &Body<f64>, edges: &[EdgeKey]) -> Vec<EdgeKey> {
    edges
        .iter()
        .copied()
        .filter(|&e| {
            let sides = topo::readback::edge_sides(body, e).expect("an edge's two faces");
            let (f, g) = sides.faces();
            [f, g].iter().all(|&x| {
                matches!(
                    body.face_surface(x).expect("a stored surface"),
                    Surface::Cylinder { .. }
                )
            })
        })
        .collect()
}

/// One build: tier 3, one closed shell of the source's genus, naming
/// totality over every requested edge but a bore's seams, every corner
/// edge the carve minted on both its faces, and `ΔV` at `truth` within
/// the certified enclosure and `1e-12`.
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
    let smooth = seams(body, edges);
    let blended: Vec<EdgeKey> = edges
        .iter()
        .copied()
        .filter(|e| !smooth.contains(e))
        .collect();
    assert_naming_totality(body, &out, &blended, what);
    let ((v0, pad0), (v1, pad1)) = (volume(body), volume(&out.body));
    let (dv, pad) = (v1 - v0, pad0 + pad1);
    assert!(
        pad < 1e-9 && (dv - truth).abs() < 1e-12 + pad,
        "{what} ({verb:?}): ΔV {dv} ± {pad} vs the closed form {truth}"
    );
    let (stray, read) = corner_residual(&out);
    let rec = out.naming.as_ref().expect("births");
    let mixed = rec.host_feet.len() / 2;
    assert!(
        read == rec.arcs.len() + rec.trims.len() && read >= 4 * mixed && mixed > 0,
        "{what} ({verb:?}): {read} corner edges read, every one recorded and the four of \
         each of {mixed} mixed corners among them"
    );
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
    // A square boss in the plate's corner whose base edge along `y = 0`
    // bows out: a curved PAIR link at the inset corner at the origin.
    let bowed = prism_at(
        vec![
            (Point2::new(-1.0, -1.0), 0.0),
            (Point2::new(0.0, -1.0), 0.0),
            (Point2::new(0.0, 0.0), 0.2),
            (Point2::new(-1.0, 0.0), 0.0),
        ],
        -0.5,
        1.5,
        tol(),
    );
    let body = realized(BooleanOp::Union, &plate(&square), &bowed, tol());
    let edges = at(&body, (0.0, 0.0, 0.0));
    assert_eq!(edges.len(), 3, "the bowed corner's three edges");
    let got = Verb::Fillet.run(&body, &edges, R);
    assert!(
        matches!(
            got,
            Err(BlendError::UnsupportedCorner {
                corner: CornerConfig::MixedConvexity { convex: 1 },
                ..
            })
        ),
        "a fillet's curved pair link at a mixed corner, got {got:?}"
    );
    let got = Verb::Chamfer.run(&body, &edges, R);
    assert!(
        matches!(got, Err(BlendError::ChamferArmUnsupported { .. })),
        "a chamfer's curved pair link, got {got:?}"
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

    // The flush corner with its odd edge shorter than the size: the
    // support screen reads the pair's trimlines `r` up the odd edge's
    // faces.
    let short = l_prism(0.08);
    let edges = l_corner(&short, 0.08);
    for verb in [Verb::Fillet, Verb::Chamfer] {
        let got = verb.run(&short, &edges, R);
        assert!(
            matches!(got, Err(BlendError::FaceClearanceUncertified { .. })),
            "an odd edge shorter than the size ({verb:?}), got {got:?}",
            got = got.as_ref().map(|_| ())
        );
    }
}

/// **A mixed corner at a plan angle other than a right one** builds at
/// the general closed forms, flush and inset (the inset's each negated),
/// both verbs, at `φ = 60°` and `120°`.
#[test]
fn a_mixed_corner_at_a_non_right_plan_angle_builds_at_its_closed_form() {
    for phi in [PI / 3.0, 2.0 * PI / 3.0] {
        let what = format!("flush at {:.0}°", phi.to_degrees());
        let (body, edges) = flush_at(phi);
        let fillet = fillet_flush(phi, R, (2.0, 1.0, 1.0));
        builds(&body, &edges, (Verb::Fillet, R), (fillet, 0), &what);
        let chamfer = chamfer_flush(phi, R, (2.0, 1.0, 1.0));
        builds(&body, &edges, (Verb::Chamfer, R), (chamfer, 0), &what);
        let what = format!("inset at {:.0}°", phi.to_degrees());
        let (body, edges) = inset_at(phi);
        let fillet = -fillet_flush(phi, R, (1.0, 1.0, 1.0));
        builds(&body, &edges, (Verb::Fillet, R), (fillet, 0), &what);
        let chamfer = -chamfer_flush(phi, R, (1.0, 1.0, 1.0));
        builds(&body, &edges, (Verb::Chamfer, R), (chamfer, 0), &what);
    }
}

/// **A pair edge too short for the corner's feet and its far end's**
/// refuses at its feet (`junction::feet_clear`): the flush corner's `+x`
/// pair edge, `y` long, ends in a turn with the floor edge leaving its
/// far end at 45°, whose feet stand `r·cot(22.5°)` back along it. At
/// `y = 0.3` the feet cross, for either verb; at `0.35` they stand
/// apart and the request builds.
#[test]
fn a_pair_edge_too_short_for_its_two_ends_feet_refuses_at_them() {
    let turned = |y: f64| {
        let body = prism_at(
            corners(&[
                (-1.0, -1.0),
                (y - 1.0, -1.0),
                (y, 0.0),
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
            edge(&body, o, (y, 0.0, 0.0)),
            edge(&body, o, (0.0, 1.0, 0.0)),
            edge(&body, (y, 0.0, 0.0), (y - 1.0, -1.0, 0.0)),
        ];
        (body, edges)
    };
    for verb in [Verb::Fillet, Verb::Chamfer] {
        let (body, edges) = turned(0.3);
        let got = verb.run(&body, &edges, R);
        assert!(
            matches!(
                got,
                Err(BlendError::UnsupportedRunOut { detail, .. })
                    if detail.contains("shorter than its two ends' feet need")
            ),
            "the pair edge 0.3 long ({verb:?}), got {got:?}",
            got = got.as_ref().map(|_| ())
        );
        let (body, edges) = turned(0.35);
        let out = verb
            .run(&body, &edges, R)
            .unwrap_or_else(|e| panic!("the pair edge 0.35 long ({verb:?}): builds, got {e:?}"));
        validate_geometric(&out.body, tol())
            .unwrap_or_else(|e| panic!("the pair edge 0.35 long ({verb:?}): tier 3, got {e:?}"));
    }
}

/// The flush L bored through at plan point `(x, y)` with radius `b`.
fn bored(x: f64, y: f64, b: f64) -> (Body<f64>, [EdgeKey; 3]) {
    let bore = cylinder_of_arcs_at(2, b, Point2::new(x, y), -0.5, 3.0, tol());
    let body = realized(BooleanOp::Subtract, &l_prism(2.0), &bore, tol());
    let edges = l_corner(&body, 2.0);
    (body, edges)
}

/// **A ring of the floor behind one pair edge's line** (the `+y` arm,
/// behind the `+x` edge) is still read against the other pair edge's
/// strip, which runs `x ∈ [−s, 0]` there: a bore inside that strip, or
/// straddling its trimline, refuses at the support screen, and one
/// `0.01` clear of the trimline builds at the corner's closed form.
#[test]
fn a_bore_behind_a_pair_edges_line_is_read_against_the_other_pair_strip() {
    let refuses = |x: f64, what: &str| {
        let (body, edges) = bored(x, 0.5, 0.03);
        for verb in [Verb::Fillet, Verb::Chamfer] {
            let got = verb.run(&body, &edges, R);
            assert!(
                matches!(got, Err(BlendError::FaceClearanceUncertified { .. })),
                "a bore {what} ({verb:?}), got {got:?}",
                got = got.as_ref().map(|_| ())
            );
        }
    };
    refuses(-0.05, "inside the other pair edge's strip");
    refuses(-0.1, "straddling the other pair edge's trimline");
    let (body, edges) = bored(-0.14, 0.5, 0.03);
    let (a, d) = section(R);
    let fillet = a * (2.0 - 1.0 - 1.0 + 2.0 * R) - FRAC_PI_2 * a * (R + d);
    builds(
        &body,
        &edges,
        (Verb::Fillet, R),
        (fillet, 1),
        "a bore 0.01 clear",
    );
    let chamfer = R * R / 2.0 * (2.0 - 1.0 - 1.0) + k() * R.powi(3);
    builds(
        &body,
        &edges,
        (Verb::Chamfer, R),
        (chamfer, 1),
        "a bore 0.01 clear",
    );
}

/// **A ring wholly behind a pair edge's line clears that edge's ring
/// meter** (`surgery::ring_clearance_pass`, arm (a)): a bore in the
/// `+y` arm `0.01` behind the `+x` edge's line, well clear of the other
/// pair edge's strip, builds at the corner's closed form. One grazing
/// that line within the band, from either side, is not certainly
/// behind it, and refuses `RingClearance` on the definite reading of its
/// lowest reach past the trimline.
#[test]
fn a_bore_wholly_behind_a_pair_edges_line_clears_its_ring_meter() {
    let (body, edges) = bored(-0.5, 0.06, 0.05);
    let (a, d) = section(R);
    let fillet = a * (2.0 - 1.0 - 1.0 + 2.0 * R) - FRAC_PI_2 * a * (R + d);
    builds(
        &body,
        &edges,
        (Verb::Fillet, R),
        (fillet, 1),
        "a bore behind the line",
    );
    let chamfer = R * R / 2.0 * (2.0 - 1.0 - 1.0) + k() * R.powi(3);
    builds(
        &body,
        &edges,
        (Verb::Chamfer, R),
        (chamfer, 1),
        "a bore behind the line",
    );
    for graze in [5e-9, -5e-9] {
        let (body, edges) = bored(-0.5, 0.05 + graze, 0.05);
        for verb in [Verb::Fillet, Verb::Chamfer] {
            let got = verb.run(&body, &edges, R);
            assert!(
                matches!(
                    got,
                    Err(BlendError::RingClearance { ref margin, .. }) if margin.sign == geom_core::Sign::Negative
                ),
                "a bore grazing the line by {graze} ({verb:?}), got {got:?}",
                got = got.as_ref().map(|_| ())
            );
        }
    }
}

/// **The reach meters a mixed corner as a uniform corner's** and so
/// refuses `FaceClearance` where the corner would clear
/// (`work/band/the-reach-meters-a-mixed-corner-as-a-uniform-one-and-refuses-short-edges.md`):
/// the flush L 0.25 tall chamfered at `0.1` (it builds from 0.26), the
/// bore-less bracket chamfered all round at `0.14`, and the bracket
/// filleted all round at `0.149`. Each is pinned as it refuses today.
#[test]
fn a_mixed_corner_reach_metered_as_a_uniform_corner_refuses_typed() {
    let bounded = |got: &Result<Blended<f64>, BlendError>| {
        matches!(got, Err(BlendError::FaceClearance { bounded: true, .. }))
    };
    let short = l_prism(0.25);
    let got = Verb::Chamfer.run(&short, &l_corner(&short, 0.25), R);
    assert!(
        bounded(&got),
        "the flush L 0.25 tall, chamfered at 0.1, got {got:?}",
        got = got.as_ref().map(|_| ())
    );
    let body = bracket(false);
    let got = Verb::Chamfer.run(&body, &every_edge(&body), 0.14);
    assert!(
        bounded(&got),
        "the bore-less bracket chamfered at 0.14, got {got:?}",
        got = got.as_ref().map(|_| ())
    );
    let body = bracket(true);
    let got = Verb::Fillet.run(&body, &every_edge(&body), 0.149);
    assert!(
        bounded(&got),
        "the bracket filleted at 0.149, got {got:?}",
        got = got.as_ref().map(|_| ())
    );
}
