//! **The plane–plane band's cut-off** (`work/band/a-plane-plane-blend-
//! cannot-end-at-an-unrequested-corner.md`, steps 1 and 2): a straight
//! band ends at a trivalent vertex of one convexity as the request
//! decides — all three edges, the corner patch; the edge alone, the
//! cut-off in the end face's plane section of the band (a chord at any
//! angle for a chamfer, an arc at a perpendicular end face for a
//! fillet); two, the turn, refused.
//!
//! Every built row is checked at its closed form and at tier 3, with
//! naming totality; the box rows are checked once more against an
//! independent boolean (the box less the half-space prism beyond each
//! chamfer plane), which reaches the same solid by another door.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Point2, Point3, Tol, Vec3};
use sweep::blend::battery::{END_FACE_CURVED, END_FACE_OBLIQUE};
use sweep::blend::build::{Blended, fillet_edges};
use sweep::blend::{BlendError, CornerConfig, RunOutPolicy};
use sweep::chamfer::chamfer_edges;
use sweep::test_support::{
    assert_naming_totality, block, cube, pocket_die, prism, prism_on, realized, sketch_from_axes,
};
use topo::{Body, EdgeKey, mass_properties, query, validate_geometric};

/// The band size every row asks for, meters.
pub(crate) const D: f64 = 0.1;

pub(crate) fn tol() -> Tol {
    Tol::witness()
}

pub(crate) fn volume(body: &Body<f64>) -> f64 {
    let p = mass_properties(body, tol()).expect("closed-form props");
    assert_eq!(p.volume_pad, 0.0, "the inventory is closed-form");
    p.volume
}

/// The edge of `body` between the two points, either way round.
pub(crate) fn edge(body: &Body<f64>, a: [f64; 3], b: [f64; 3]) -> EdgeKey {
    let (a, b) = (Point3::new(a[0], a[1], a[2]), Point3::new(b[0], b[1], b[2]));
    let at = |p: Point3<f64>, q: Point3<f64>| (p - q).norm() < 1e-12;
    query::all_edges(body)
        .into_iter()
        .find(|&e| {
            let he = body.get_edge(e).expect("an edge").he_plus;
            let s = body.get_half_edge(he).expect("a half").start;
            let t = body.half_edge_end(he).expect("an end");
            let (ps, pt) = (
                *body
                    .get_point(body.get_vertex(s).expect("v").point)
                    .expect("p"),
                *body
                    .get_point(body.get_vertex(t).expect("v").point)
                    .expect("p"),
            );
            (at(ps, a) && at(pt, b)) || (at(ps, b) && at(pt, a))
        })
        .unwrap_or_else(|| panic!("an edge between {a:?} and {b:?}"))
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Verb {
    Chamfer,
    Fillet,
}

impl Verb {
    pub(crate) fn run(
        self,
        body: &Body<f64>,
        edges: &[EdgeKey],
    ) -> Result<Blended<f64>, BlendError> {
        match self {
            Self::Chamfer => chamfer_edges(body, edges, D, tol()),
            Self::Fillet => fillet_edges(body, edges, D, tol()),
        }
        .map_err(|r| r.error)
    }

    /// The section a band removes from (or adds to) a right dihedral.
    pub(crate) fn section(self) -> f64 {
        match self {
            Self::Chamfer => D * D / 2.0,
            Self::Fillet => (1.0 - PI / 4.0) * D * D,
        }
    }

    /// What one corner patch takes back from the three prisms meeting
    /// at a right trihedron: the cube closed forms' per-corner term
    /// (`common::oracles`), `(2/3)d³` for the chamfer and
    /// `(2 − 7π/12)r³` for the fillet.
    pub(crate) fn corner(self) -> f64 {
        match self {
            Self::Chamfer => 2.0 / 3.0 * D.powi(3),
            Self::Fillet => (2.0 - 7.0 * PI / 12.0) * D.powi(3),
        }
    }
}

/// Carve, and check what holds of every built row: tier 3, Euler on one
/// genus-0 shell, naming totality, and `ΔV = removed` (negative where
/// the band adds material).
pub(crate) fn carve(
    body: &Body<f64>,
    edges: &[EdgeKey],
    verb: Verb,
    removed: f64,
    what: &str,
) -> Blended<f64> {
    let out = verb
        .run(body, edges)
        .unwrap_or_else(|e| panic!("{what} ({verb:?}): builds, got {e}"));
    validate_geometric(&out.body, tol())
        .unwrap_or_else(|e| panic!("{what} ({verb:?}): tier 3, got {e:?}"));
    let c = topo::readback::euler_counts(&out.body);
    assert_eq!(
        (c.s, c.genus()),
        (1, Ok(0)),
        "{what} ({verb:?}): one closed genus-0 shell"
    );
    assert_naming_totality(body, &out, edges, what);
    let dv = volume(body) - volume(&out.body);
    assert!(
        (dv - removed).abs() < 1e-12,
        "{what} ({verb:?}): ΔV {dv} vs the closed form {removed}"
    );
    out
}

/// The box every box row carves: `2 × 1.5 × 1`, low corner at the
/// origin, so its three edge lengths are told apart.
pub(crate) fn the_box() -> Body<f64> {
    block(2.0, 1.5, 1.0, tol())
}

/// **One edge of a box**, both verbs, at the prism closed form: the
/// band runs the edge's whole length and is cut off at both end faces,
/// perpendicular to it.
#[test]
fn one_edge_of_a_box_is_cut_off_at_both_end_faces() {
    let body = the_box();
    let e = edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let out = carve(&body, &[e], verb, verb.section() * 2.0, "one edge");
        assert_eq!(out.blend_faces.len(), 1, "one band");
        assert!(out.corner_faces.is_empty(), "no corner patch");
        let rec = out.naming.as_ref().expect("births");
        assert_eq!(rec.arcs.len(), 2, "one end curve per end");
        assert_eq!(rec.feet.len(), 4, "two feet per end");
        assert_eq!(rec.dead.vertices.len(), 2, "both old vertices retired");
    }
}

/// **Every proper subset this suite names**, each ending at unrequested
/// corners: two parallel edges of one face (the rim between their
/// ends split twice), the box's four edges along `x` (every rim of both
/// end faces split twice), and the three edges of one corner — a patch
/// at the corner and a cut-off at each far end.
#[test]
fn proper_subsets_of_a_box_end_at_their_unrequested_corners() {
    let body = the_box();
    let top_front = edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    let top_back = edge(&body, [0.0, 1.5, 1.0], [2.0, 1.5, 1.0]);
    let bottom_front = edge(&body, [0.0, 0.0, 0.0], [2.0, 0.0, 0.0]);
    let bottom_back = edge(&body, [0.0, 1.5, 0.0], [2.0, 1.5, 0.0]);
    let up = edge(&body, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let across = edge(&body, [0.0, 0.0, 1.0], [0.0, 1.5, 1.0]);
    for verb in [Verb::Chamfer, Verb::Fillet] {
        carve(
            &body,
            &[top_front, top_back],
            verb,
            verb.section() * 4.0,
            "two parallel edges of the top",
        );
        carve(
            &body,
            &[top_front, top_back, bottom_front, bottom_back],
            verb,
            verb.section() * 8.0,
            "the four edges along x",
        );
        let out = carve(
            &body,
            &[top_front, up, across],
            verb,
            verb.section() * (2.0 + 1.0 + 1.5) - verb.corner(),
            "three edges of one corner",
        );
        assert_eq!(out.corner_faces.len(), 1, "one corner patch");
        assert_eq!(out.blend_faces.len(), 3, "three bands");
    }
}

/// **The same carves, by another door**: the box less, per chamfered
/// edge, the prism beyond its chamfer plane — whose other faces lie
/// outside the box and whose ends lie past its end faces, so the
/// boolean cuts exactly the band's region between them.
#[test]
fn a_chamfered_box_edge_matches_the_boolean_less_its_prism() {
    let body = the_box();
    // The prism beyond the chamfer plane of the edge at `y = y0`,
    // `z = 1` along `x`, the material on the `toward` side in `y`.
    let beyond = |y0: f64, toward: f64| {
        let out = -toward;
        let plane = sketch_from_axes(
            Point3::new(-0.5, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            tol(),
        );
        let mut verts = vec![
            (Point2::new(y0 + out * 0.5, 0.5 - D), 0.0),
            (Point2::new(y0 + toward * (D + 0.5), 1.5), 0.0),
            (Point2::new(y0 + out * 0.5, 1.5), 0.0),
        ];
        if toward < 0.0 {
            verts.reverse();
        }
        prism_on(plane, verts, 3.0, tol())
    };
    let front = edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    let back = edge(&body, [0.0, 1.5, 1.0], [2.0, 1.5, 1.0]);
    let by_boolean = realized(
        topo::boolean::BooleanOp::Subtract,
        &realized(
            topo::boolean::BooleanOp::Subtract,
            &body,
            &beyond(0.0, 1.0),
            tol(),
        ),
        &beyond(1.5, -1.0),
        tol(),
    );
    let carved = chamfer_edges(&body, &[front, back], D, tol()).expect("both edges chamfer");
    let (v_bool, v_carve) = (volume(&by_boolean), volume(&carved.body));
    assert!(
        (v_bool - v_carve).abs() < 1e-12,
        "the boolean's {v_bool} and the carve's {v_carve} agree"
    );
}

/// **An oblique end face**: a parallelogram prism's top front edge
/// ends at two parallel slanted side walls. The chamfer is cut off in a
/// chord across each at the prism closed form — the end planes are
/// parallel, so the band's length is the edge's — and the fillet,
/// whose section there is an ellipse, refuses typed.
#[test]
fn an_oblique_end_face_cuts_the_chamfer_off_and_refuses_the_fillet() {
    let body = prism(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.0),
            (Point2::new(2.5, 1.0), 0.0),
            (Point2::new(0.5, 1.0), 0.0),
        ],
        1.0,
        tol(),
    );
    let e = edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    carve(
        &body,
        &[e],
        Verb::Chamfer,
        Verb::Chamfer.section() * 2.0,
        "oblique ends",
    );
    match Verb::Fillet.run(&body, &[e]) {
        Err(BlendError::UnsupportedRunOut { detail, .. }) => {
            assert_eq!(detail, END_FACE_OBLIQUE);
        }
        other => panic!("an oblique fillet end refuses as a run-out, got {other:?}"),
    }
}

/// **The concave side**: one floor edge of the pocketed die, whose ends
/// are concave trihedra with the pocket's end walls as end faces. The
/// band ADDS its section along the edge's length, and each end wall
/// LOSES the sliver between the end curve and the old vertex, which the
/// fill now covers, just as a convex end face loses the one its cut
/// takes away: the wall's outline trades the old vertex for the two
/// feet, and a chamfer's end wall, a polygon still, gives up exactly
/// `d²/2` of its area.
#[test]
fn a_concave_edge_is_cut_off_with_its_end_walls_losing_the_sliver() {
    let body = pocket_die(0.0, 0.0, 0.0, tol());
    let e = edge(&body, [0.25, 0.25, 0.5], [0.75, 0.25, 0.5]);
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let out = carve(
            &body,
            &[e],
            verb,
            -verb.section() * 0.5,
            "a pocket floor edge",
        );
        for x in [0.25, 0.75] {
            let before = end_wall(&body, x);
            let after = end_wall(&out.body, x);
            let v = Point3::new(x, 0.25, 0.5);
            let feet = [Point3::new(x, 0.25 + D, 0.5), Point3::new(x, 0.25, 0.5 + D)];
            let has =
                |ps: &[Point3<f64>], p: Point3<f64>| ps.iter().any(|q| (*q - p).norm() < 1e-12);
            assert!(
                has(&before, v) && !feet.iter().any(|f| has(&before, *f)),
                "x = {x}: the source wall has the old vertex and no feet"
            );
            assert!(
                !has(&after, v) && feet.iter().all(|f| has(&after, *f)),
                "x = {x} ({verb:?}): the wall loses the old vertex to the two feet, got {after:?}"
            );
            assert_eq!(
                after.len(),
                before.len() + 1,
                "x = {x} ({verb:?}): one corner becomes two"
            );
            if let Verb::Chamfer = verb {
                let lost = shoelace_yz(&before) - shoelace_yz(&after);
                assert!(
                    (lost - D * D / 2.0).abs() < 1e-14,
                    "x = {x}: the end wall loses the sliver d²/2, got {lost}"
                );
            }
        }
    }
}

/// The outline of the one face whose outer cycle lies in the plane
/// `x = x0`, as its vertices in cycle order.
fn end_wall(body: &Body<f64>, x0: f64) -> Vec<Point3<f64>> {
    let outlines: Vec<Vec<Point3<f64>>> = query::all_faces(body)
        .into_iter()
        .filter_map(|f| {
            let lp = body.get_face(f)?.outer;
            let topo::LoopBoundary::Cycle { first } = body.get_loop(lp)?.boundary else {
                return None;
            };
            let ps: Vec<Point3<f64>> = body
                .loop_cycle(first)?
                .into_iter()
                .map(|h| {
                    let v = body.get_half_edge(h).expect("a half").start;
                    *body
                        .get_point(body.get_vertex(v).expect("v").point)
                        .expect("p")
                })
                .collect();
            ps.iter().all(|p| (p.x - x0).abs() < 1e-12).then_some(ps)
        })
        .collect();
    let [outline] = &outlines[..] else {
        panic!("one face lies in x = {x0}, got {}", outlines.len());
    };
    outline.clone()
}

/// The area a polygon in a plane `x = const` encloses.
fn shoelace_yz(ps: &[Point3<f64>]) -> f64 {
    let n = ps.len();
    let twice: f64 = (0..n)
        .map(|i| ps[i].y * ps[(i + 1) % n].z - ps[(i + 1) % n].y * ps[i].z)
        .sum();
    twice.abs() / 2.0
}

/// **The refusals the cut-off leaves**, each typed: a curved end face,
/// a foot that lands inside a face rather than on the end face's rim,
/// an end vertex of valence four, a mixed-convexity end, and the turn.
#[test]
fn every_end_the_cut_off_does_not_build_refuses_typed() {
    // A curved end face: a prism whose right side is an arc.
    let d_prism = prism(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.5),
            (Point2::new(2.0, 1.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        1.0,
        tol(),
    );
    let e = edge(&d_prism, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    for verb in [Verb::Chamfer, Verb::Fillet] {
        match verb.run(&d_prism, &[e]) {
            Err(BlendError::UnsupportedRunOut { detail, .. }) => {
                assert_eq!(detail, END_FACE_CURVED, "{verb:?}");
            }
            other => panic!("{verb:?}: a curved end face refuses, got {other:?}"),
        }
    }

    // A foot inside a face: a triangular prism whose top's short side
    // at the edge's end turns back, so the trimline meets the end
    // face's plane past that side.
    let wedge = prism(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.0),
            (Point2::new(1.9, 0.05), 0.0),
        ],
        1.0,
        tol(),
    );
    let e = edge(&wedge, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    match Verb::Chamfer.run(&wedge, &[e]) {
        Err(BlendError::UnsupportedRunOut { detail, .. }) => {
            assert!(detail.contains("foot lands inside a face"), "{detail}");
        }
        other => panic!("a foot inside a face refuses as a run-out, got {other:?}"),
    }

    // Valence four: a chamfered cube's patch vertex, which a trimline of
    // the chamfer ends at.
    let cube_body = cube(1.0, tol());
    let chamfered = chamfer_edges(&cube_body, &query::all_edges(&cube_body), D, tol())
        .expect("the cube chamfers")
        .body;
    let rec_edge = query::all_edges(&chamfered)
        .into_iter()
        .find(|&e| {
            let he = chamfered.get_edge(e).expect("e").he_plus;
            let s = chamfered.get_half_edge(he).expect("h").start;
            chamfered.edges_of_vertex(s).is_some_and(|es| es.len() == 4)
        })
        .expect("a valence-four vertex");
    match Verb::Chamfer.run(&chamfered, &[rec_edge]) {
        Err(BlendError::UnsupportedCorner {
            corner: CornerConfig::NEdgeVertex { valence: 4 },
            ..
        }) => {}
        other => panic!("a valence-four end refuses its configuration, got {other:?}"),
    }

    // Mixed convexity: an L prism's concave inner edge ends where two
    // convex outline edges meet it.
    let l_prism = prism(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.0),
            (Point2::new(2.0, 1.0), 0.0),
            (Point2::new(1.0, 1.0), 0.0),
            (Point2::new(1.0, 2.0), 0.0),
            (Point2::new(0.0, 2.0), 0.0),
        ],
        1.0,
        tol(),
    );
    let inner = edge(&l_prism, [1.0, 1.0, 0.0], [1.0, 1.0, 1.0]);
    for verb in [Verb::Chamfer, Verb::Fillet] {
        match verb.run(&l_prism, &[inner]) {
            Err(BlendError::UnsupportedCorner {
                corner: CornerConfig::MixedConvexity { .. },
                ..
            }) => {}
            other => panic!("{verb:?}: a mixed end refuses, got {other:?}"),
        }
    }

    // The turn: two edges of the box's top meeting at a corner.
    let body = the_box();
    let a = edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    let b = edge(&body, [2.0, 0.0, 1.0], [2.0, 1.5, 1.0]);
    for verb in [Verb::Chamfer, Verb::Fillet] {
        match verb.run(&body, &[a, b]) {
            Err(BlendError::UnsupportedCorner {
                corner: CornerConfig::Turn,
                policy: Some(RunOutPolicy::Mitre),
                ..
            }) => {}
            other => panic!("{verb:?}: two edges of a corner refuse as a turn, got {other:?}"),
        }
    }
}

/// **A closed rim that turns once**: a teardrop prism's top rim — two
/// lines meeting at a corner, closed by an arc tangent to both. Its one
/// plane–plane junction is a definite turn and its two others are
/// tangent, so chain G1 breaks the closed chain into ONE open chain
/// whose head and tail are both the corner; the end there names two of
/// the corner's three edges, the turn, refused typed at that vertex.
#[test]
fn a_closed_rim_with_one_turn_breaks_there_and_refuses_the_turn() {
    use profile::RawLoop;
    let s3 = 3f64.sqrt();
    let body = sweep::test_support::extruded(
        profile::SketchPlane::xy(),
        vec![
            profile::test_support::bulge_loop(vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(1.5, -s3 / 2.0), s3),
                (Point2::new(1.5, s3 / 2.0), 0.0),
            ])
            .with_tangent_joints(vec![1, 2]),
        ],
        1.0,
        tol(),
    );
    let at = |p: Point3<f64>| p.z == 1.0;
    let rim: Vec<EdgeKey> = query::all_edges(&body)
        .into_iter()
        .filter(|&e| {
            let he = body.get_edge(e).expect("e").he_plus;
            let p = |v| {
                *body
                    .get_point(body.get_vertex(v).expect("v").point)
                    .expect("p")
            };
            at(p(body.get_half_edge(he).expect("h").start))
                && at(p(body.half_edge_end(he).expect("end")))
        })
        .collect();
    assert_eq!(rim.len(), 3, "two lines and the arc");
    match Verb::Fillet.run(&body, &rim) {
        Err(BlendError::UnsupportedCorner {
            vertex,
            corner: CornerConfig::Turn,
            policy: Some(RunOutPolicy::Mitre),
        }) => {
            let p = *body
                .get_point(body.get_vertex(vertex).expect("v").point)
                .expect("p");
            assert!(
                (p - Point3::new(0.0, 0.0, 1.0)).norm() == 0.0,
                "the turn is the teardrop's corner, got {p:?}"
            );
        }
        other => panic!("a rim that turns once refuses the turn, got {other:?}"),
    }
}
