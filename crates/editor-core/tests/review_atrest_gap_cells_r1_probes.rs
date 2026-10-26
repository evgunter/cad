//! Reviewer probes (ATREST R1, PR 5746): `Gap` over vertex and edge
//! cells read on REAL bodies, where the unit rows build their wedges
//! and corners by hand. Each row discriminates the fold (`max` against
//! `min`) and the face sense (`convex`) the PR reads off topology.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture::{ename, fname, in_world, insert, len, place, vname, vpiece};

use editor_core::{
    CapEnd, Frame, LoopProgram, MeasurePrimitive, Node, NodeResult, ProfileDoc, ProfileProgram,
    RecipeNodeId, RoleSeg, SitedRef, ValuePayload,
};
use geom_core::{Tol, Turns};

fn extrude(
    doc: ProfileDoc,
    loops: Vec<LoopProgram<editor_core::Formula>>,
    h: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops,
            ids: Vec::new(),
        }),
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(h),
        },
    )
}

fn poly(pts: &[(f64, f64)]) -> LoopProgram<editor_core::Formula> {
    LoopProgram::polygon(pts.to_vec()).expect("finite corners")
}

fn at(v: [f64; 3]) -> Frame {
    Frame::translation(v)
}

/// The gap's value, or the refusal's Debug text.
fn gap(doc: &ProfileDoc, outer: &SitedRef, inner: &SitedRef) -> Result<f64, String> {
    let (doc, m) = crate::fixture::measure_node(
        doc,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        vec![outer.clone(), inner.clone()],
    );
    let run = corpus::eval::<f64>(&doc);
    match run.nodes.get(&m) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Measure { value, .. } => Ok(*value),
            other => panic!("a measure, not {}", other.kind_name()),
        },
        other => Err(format!("{other:?}")),
    }
}

fn near(got: Result<f64, String>, want: f64, what: &str) {
    let g = got.unwrap_or_else(|e| panic!("{what}: refused {e}"));
    assert!((g - want).abs() < 1e-9, "{what}: read {g}, want {want}");
}

const SQUARE: [(f64, f64); 4] = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
const UNIT: [(f64, f64); 4] = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];

/// A block `[-1,1]² × [0,1]` placed as-is, and a unit cube whose corner
/// vertex 0 (its material in `+x, +y, +z`) stands at `corner`.
fn two_blocks(
    corner: [f64; 3],
) -> (
    ProfileDoc,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
) {
    let doc = ProfileDoc::empty_derived("review-gap-cells-r1", Tol::witness());
    let (doc, a) = extrude(doc, vec![poly(&SQUARE)], 1.0);
    let (doc, b) = extrude(doc, vec![poly(&UNIT)], 1.0);
    let (doc, pa) = place(doc, a);
    let (doc, pb) = in_world(doc, b, &at(corner));
    (doc, a, b, pa, pb)
}

/// **A real block's convex top corner reads a vertex as the MAX of its
/// face offsets.** The cube's corner stands 0.01 off the block's +x
/// face and 0.02 inside its +y and +z planes: apart, `+0.01`. A `min`
/// fold (the convexity rule's sign flipped on real topology) reads
/// `−0.02`.
#[test]
fn probe_real_convex_corner_reads_a_vertex_by_max() {
    for (dx, want) in [(0.01, 0.01), (-0.01, -0.01)] {
        let (doc, a, b, pa, pb) = two_blocks([1.0 + dx, 0.98, 0.98]);
        let ca = vpiece(&doc, a, 0, 2);
        let cb = vpiece(&doc, b, 0, 0);
        let corner = SitedRef::new(
            pa,
            vname(a, RoleSeg::CapVertex(CapEnd::End, ca)).in_copy(pa),
        );
        let tip = SitedRef::new(
            pb,
            vname(b, RoleSeg::CapVertex(CapEnd::Start, cb)).in_copy(pb),
        );
        near(
            gap(&doc, &corner, &tip),
            want,
            &format!("corner reads vertex, dx {dx}"),
        );
        // The block's +x/+y strut edge at (1, 1) reads the vertex too, in
        // either seat.
        let strut = SitedRef::new(pa, ename(a, RoleSeg::LateralEdge(ca)).in_copy(pa));
        near(
            gap(&doc, &strut, &tip),
            want,
            &format!("strut reads vertex, dx {dx}"),
        );
        near(
            gap(&doc, &tip, &strut),
            want,
            &format!("vertex/strut seat, dx {dx}"),
        );
    }
}

const L: [(f64, f64); 6] = [
    (0.0, 0.0),
    (3.0, 0.0),
    (3.0, 1.0),
    (1.0, 1.0),
    (1.0, 3.0),
    (0.0, 3.0),
];

/// **A real L-bracket's reflex strut reads a vertex as the MIN**, and
/// its reflex top vertex (two convex rims, one concave strut) refuses
/// as a saddle. The cube's corner at `(1.02, 0.99)` is inside the
/// bracket's long leg (`y < 1`): `−0.01`. A `max` fold reads `+0.02`.
#[test]
fn probe_real_reflex_strut_reads_by_min_and_its_vertex_is_a_saddle() {
    let doc = ProfileDoc::empty_derived("review-gap-cells-r1-l", Tol::witness());
    let (doc, l) = extrude(doc, vec![poly(&L)], 1.0);
    let (doc, b) = extrude(doc, vec![poly(&UNIT)], 1.0);
    let (doc, pl) = place(doc, l);
    for (y, want) in [(0.99, -0.01), (1.01, 0.01)] {
        let (doc, pb) = in_world(doc.clone(), b, &at([1.02, y, 0.5]));
        let reflex = vpiece(&doc, l, 0, 3);
        let cb = vpiece(&doc, b, 0, 0);
        let strut = SitedRef::new(pl, ename(l, RoleSeg::LateralEdge(reflex)).in_copy(pl));
        let tip = SitedRef::new(
            pb,
            vname(b, RoleSeg::CapVertex(CapEnd::Start, cb)).in_copy(pb),
        );
        near(
            gap(&doc, &strut, &tip),
            want,
            &format!("reflex strut, y {y}"),
        );
        near(
            gap(&doc, &tip, &strut),
            want,
            &format!("reflex strut seat, y {y}"),
        );
        let top = SitedRef::new(
            pl,
            vname(l, RoleSeg::CapVertex(CapEnd::End, reflex)).in_copy(pl),
        );
        let refused = gap(&doc, &top, &tip).expect_err("a saddle refuses");
        assert!(
            refused.contains("MeasureUnsupported") && refused.contains("a saddle vertex"),
            "the reflex top vertex refuses as a saddle: {refused}"
        );
    }
}

/// **Two real ridges crossing at right angles.** The block's vertical
/// `(1, 1)` strut (material `x, y ≤ 1`) and a wedge prism's apex ridge
/// along `y` at `x = 1 + h, z = 0.5` (material `x ≥ 1 + h`): the ridges
/// are `h` apart. Each seat reads the sign of `h` (the magnitudes
/// differ: each is its own wedge's surrogate).
#[test]
fn probe_real_crossing_ridges_read_the_sign_in_both_seats() {
    for h in [0.0, 0.01, -0.01] {
        let doc = ProfileDoc::empty_derived("review-gap-cells-r1-x", Tol::witness());
        let (doc, a) = extrude(doc, vec![poly(&SQUARE)], 1.0);
        let (doc, w) = extrude(doc, vec![poly(&[(0.0, 0.0), (1.0, -0.3), (1.0, 0.3)])], 2.0);
        let (doc, pa) = place(doc, a);
        let frame = Frame::rotate_by_turns_then_translate(
            [1.0, 0.0, 0.0],
            Turns::new(1, 4).expect("a quarter"),
            [1.0 + h, 2.0, 0.5],
            crate::fixture::band(),
        )
        .expect("x is a direction");
        let (doc, pw) = in_world(doc, w, &frame);
        let ca = vpiece(&doc, a, 0, 2);
        let apex = vpiece(&doc, w, 0, 0);
        let strut = SitedRef::new(pa, ename(a, RoleSeg::LateralEdge(ca)).in_copy(pa));
        let ridge = SitedRef::new(pw, ename(w, RoleSeg::LateralEdge(apex)).in_copy(pw));
        let one = gap(&doc, &strut, &ridge).expect("strut outer reads");
        let two = gap(&doc, &ridge, &strut).expect("ridge outer reads");
        eprintln!("h {h}: strut-outer {one}, ridge-outer {two}");
        for (g, seat) in [(one, "strut outer"), (two, "ridge outer")] {
            if h == 0.0 {
                assert!(g.abs() < 1e-12, "{seat}: contact reads {g}");
            } else {
                assert_eq!(g.signum(), h.signum(), "{seat}: h {h} reads {g}");
            }
        }
    }
}

/// **A real pin and a real bore read a vertex by their sense.** A disc
/// extrude's wall (a pin, radius 0.5) and a plate's circular hole (a
/// bore, radius 0.5), each about the z axis; a cube corner at
/// `(0.5 + d, 0, 0.25)`. Outside the pin is `+d`; at `d < 0` the corner
/// is in the bore's hollow, `+|d|`, and at `d > 0` in the plate, `−d`.
#[test]
fn probe_real_pin_and_bore_read_a_vertex_by_their_sense() {
    use editor_core::{EntityKind, GeomPred, NamePat, Selector, SurfaceKindSet, select_where};
    let circle = || LoopProgram::Circle {
        centre: [len(0.0), len(0.0)],
        radius: len(0.5),
    };
    for (bore, d, want) in [
        (false, 0.01, 0.01),
        (false, -0.01, -0.01),
        (true, 0.01, -0.01),
        (true, -0.01, 0.01),
    ] {
        let doc = ProfileDoc::empty_derived("review-gap-cells-r1-cyl", Tol::witness());
        let loops = if bore {
            vec![
                poly(&[(-2.0, -2.0), (2.0, -2.0), (2.0, 2.0), (-2.0, 2.0)]),
                circle(),
            ]
        } else {
            vec![circle()]
        };
        let (doc, c) = extrude(doc, loops, 0.5);
        let (doc, b) = extrude(
            doc,
            vec![poly(&[(0.0, -0.1), (0.1, -0.1), (0.1, 0.1), (0.0, 0.1)])],
            0.1,
        );
        let (doc, pc) = place(doc, c);
        let (doc, pb) = in_world(doc, b, &at([0.5 + d, 0.1, 0.25]));
        let run = corpus::eval::<f64>(&doc);
        let mut walls = select_where(
            &run,
            pc,
            &Selector::of(NamePat::of_kind(EntityKind::Face)),
            &[GeomPred::SurfaceKind(SurfaceKindSet::just(
                geom::SurfaceKind::Cylinder,
            ))],
            &ProfileDoc::empty_derived("np", Tol::witness()).var_env::<f64>(),
            Tol::witness(),
        )
        .expect("exact");
        walls.sort();
        let wall = SitedRef::new(pc, walls.remove(0));
        // The cube's start-cap vertex 0, local (0, −0.1, 0), stands at
        // (0.5 + d, 0, 0.25).
        let cb = vpiece(&doc, b, 0, 0);
        let tip = SitedRef::new(
            pb,
            vname(b, RoleSeg::CapVertex(CapEnd::Start, cb)).in_copy(pb),
        );
        near(gap(&doc, &wall, &tip), want, &format!("bore {bore}, d {d}"));
        near(
            gap(&doc, &tip, &wall),
            want,
            &format!("bore {bore}, d {d}, seat"),
        );
    }
}

/// The gap's Interval enclosure, or the refusal's Debug text.
fn gap_interval(
    doc: &ProfileDoc,
    outer: &SitedRef,
    inner: &SitedRef,
) -> Result<(f64, f64), String> {
    use geom_core::{Bounds, Interval};
    let (doc, m) = crate::fixture::measure_node(
        doc,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        vec![outer.clone(), inner.clone()],
    );
    let run = corpus::eval::<Interval>(&doc);
    match run.nodes.get(&m) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Measure { value, .. } => Ok((value.lo(), value.hi())),
            other => panic!("a measure, not {}", other.kind_name()),
        },
        other => Err(format!("{other:?}")),
    }
}

/// **The Interval lane reads the new arms soundly**: on the real convex
/// corner (vertex x vertex, edge x vertex in both seats) and the real
/// crossing ridges (edge x edge, through `gap_wedge_crossing`'s
/// division), the enclosure contains the f64 value, or refuses typed.
#[test]
fn probe_interval_lane_encloses_the_f64_gap() {
    let check = |doc: &ProfileDoc, o: &SitedRef, i: &SitedRef, what: &str| {
        let f = gap(doc, o, i).expect("f64 reads");
        match gap_interval(doc, o, i) {
            Ok((lo, hi)) => {
                eprintln!("{what}: f64 {f}, interval [{lo}, {hi}]");
                assert!(lo <= f && f <= hi, "{what}: [{lo}, {hi}] misses {f}");
            }
            Err(e) => {
                eprintln!("{what}: interval refused {e}");
                assert!(!e.contains("panic"), "{what}: {e}");
            }
        }
    };
    for dx in [0.01, -0.01, 0.0] {
        let (doc, a, b, pa, pb) = two_blocks([1.0 + dx, 0.98, 0.98]);
        let ca = vpiece(&doc, a, 0, 2);
        let cb = vpiece(&doc, b, 0, 0);
        let corner = SitedRef::new(
            pa,
            vname(a, RoleSeg::CapVertex(CapEnd::End, ca)).in_copy(pa),
        );
        let tip = SitedRef::new(
            pb,
            vname(b, RoleSeg::CapVertex(CapEnd::Start, cb)).in_copy(pb),
        );
        let strut = SitedRef::new(pa, ename(a, RoleSeg::LateralEdge(ca)).in_copy(pa));
        check(&doc, &corner, &tip, &format!("corner/vertex dx {dx}"));
        check(&doc, &strut, &tip, &format!("strut/vertex dx {dx}"));
        check(&doc, &tip, &strut, &format!("vertex/strut dx {dx}"));
    }
    for h in [0.0, 0.01, -0.01] {
        let doc = ProfileDoc::empty_derived("review-gap-cells-r1-xi", Tol::witness());
        let (doc, a) = extrude(doc, vec![poly(&SQUARE)], 1.0);
        let (doc, w) = extrude(doc, vec![poly(&[(0.0, 0.0), (1.0, -0.3), (1.0, 0.3)])], 2.0);
        let (doc, pa) = place(doc, a);
        let frame = Frame::rotate_by_turns_then_translate(
            [1.0, 0.0, 0.0],
            Turns::new(1, 4).expect("a quarter"),
            [1.0 + h, 2.0, 0.5],
            crate::fixture::band(),
        )
        .expect("x is a direction");
        let (doc, pw) = in_world(doc, w, &frame);
        let ca = vpiece(&doc, a, 0, 2);
        let apex = vpiece(&doc, w, 0, 0);
        let strut = SitedRef::new(pa, ename(a, RoleSeg::LateralEdge(ca)).in_copy(pa));
        let ridge = SitedRef::new(pw, ename(w, RoleSeg::LateralEdge(apex)).in_copy(pw));
        check(&doc, &strut, &ridge, &format!("strut/ridge h {h}"));
        check(&doc, &ridge, &strut, &format!("ridge/strut h {h}"));
    }
}

/// **The face's carrier, not the face, reads the vertex: a vertex
/// apart from the block, beside it and below its top plane, reads
/// interference.** The cube's corner at `(1.005, 0.5, 0.99)` is 0.005
/// clear of the block's `+x` side (the cube lies wholly in `x ≥ 1.005`),
/// yet `Gap(top, corner)` reads `−0.01` in either seat, while the
/// block's `(1, 1)` strut's wedge reads the same corner `+0.005`.
/// Green = the extrapolated sign is what the head reads (the face-face
/// arm has the same surrogate; recorded, not a regression).
#[test]
fn probe_a_vertex_beside_a_face_reads_the_extended_plane() {
    let (doc, a, b, pa, pb) = two_blocks([1.005, 0.5, 0.99]);
    let top = SitedRef::new(pa, fname(a, RoleSeg::Cap(CapEnd::End)).in_copy(pa));
    let cb = vpiece(&doc, b, 0, 0);
    let tip = SitedRef::new(
        pb,
        vname(b, RoleSeg::CapVertex(CapEnd::Start, cb)).in_copy(pb),
    );
    near(gap(&doc, &top, &tip), -0.01, "top reads the corner");
    near(gap(&doc, &tip, &top), -0.01, "either seat");
    let ca = vpiece(&doc, a, 0, 2);
    let strut = SitedRef::new(pa, ename(a, RoleSeg::LateralEdge(ca)).in_copy(pa));
    near(
        gap(&doc, &strut, &tip),
        0.005,
        "the strut's wedge reads it apart",
    );
}
