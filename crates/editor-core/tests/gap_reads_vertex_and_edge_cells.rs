//! **`Gap` reads vertex and edge cells** (DESIGN `[assertions.quiet]`:
//! a contact's two cells "at whatever dimension they are").
//!
//! A triangular prism rests apex down on a block, the two placed as two
//! arrivals of the world: once with its apex edge lying on the block's
//! top, once tilted so only an end of that edge, a vertex, touches it.
//! A `Gap = 0` over the two cells, read where each copy holds them,
//! holds at rest, and the gap reads the prism's lift when it is moved
//! apart and its depth, negative, when moved into the block.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture::{ename, fname, in_world, insert, len, on_frame, place, vname, vpiece};

use editor_core::{
    AssertionRelation, AssertionVerdict, CapEnd, Frame, MeasurePrimitive, Node, NodeResult,
    ProfileDoc, RecipeNodeId, RoleSeg, SitedRef, StableName, ValuePayload,
};
use geom_core::{Tol, Turns};

/// An extrude of `loop_` on the profile's own xy, `height` up its z.
fn extrude(doc: ProfileDoc, loop_: Vec<(f64, f64)>, height: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![loop_],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(height),
        },
    )
}

/// Which of the prism's cells rests on the block.
#[derive(Clone, Copy, Debug)]
enum Rest {
    /// The apex edge, lying along the block's top: the prism turned a
    /// quarter turn about x, so its profile's y is the world's z and
    /// its extrusion runs along −y.
    Edge,
    /// The apex edge's end on the prism's end cap, alone: the prism
    /// turned a third of a turn about x, which tips that end lowest,
    /// 0.2 below the apex's start.
    Vertex,
}

/// **The block, the prism resting on it `lift` above contact, and the
/// two cells in contact**, as the copies hold them: `(doc, the block's
/// top, the prism's resting cell)`.
fn resting(rest: Rest, lift: f64) -> (ProfileDoc, SitedRef, SitedRef) {
    let doc = ProfileDoc::empty_derived("gap-reads-vertex-and-edge-cells", Tol::witness());
    let (doc, block) = extrude(
        doc,
        vec![(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)],
        1.0,
    );
    // The apex first, so it is the profile's canonical vertex 0.
    let (doc, prism) = extrude(doc, vec![(0.0, 0.0), (0.2, 0.3), (-0.2, 0.3)], 0.4);
    let (turns, at) = match rest {
        Rest::Edge => (Turns::new(1, 4), [0.0, 0.2, 1.0 + lift]),
        Rest::Vertex => (Turns::new(1, 3), [0.0, 0.3, 1.2 + lift]),
    };
    let frame = Frame::rotate_by_turns_then_translate(
        [1.0, 0.0, 0.0],
        turns.expect("a fraction of a turn"),
        at,
        crate::fixture::band(),
    )
    .expect("the x axis is a direction");
    let (doc, p) = place(doc, block);
    let (doc, q) = in_world(doc, prism, &frame);
    let apex = vpiece(&doc, prism, 0, 0);
    let cell: StableName = match rest {
        Rest::Edge => ename(prism, RoleSeg::LateralEdge(apex)),
        Rest::Vertex => vname(prism, RoleSeg::CapVertex(CapEnd::End, apex)),
    };
    let top = fname(block, RoleSeg::Cap(CapEnd::End));
    (doc, SitedRef::new(p, top), SitedRef::new(q, cell))
}

/// A `Gap(outer, inner)` and a `Gap = 0` over it: the document after
/// them, the measure and the assertion.
fn asserted_contact(
    doc: &ProfileDoc,
    outer: &SitedRef,
    inner: &SitedRef,
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, measure) = crate::fixture::measure_node(
        doc,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        vec![outer.clone(), inner.clone()],
    );
    let value = crate::fixture::value_of(&doc, measure);
    let (doc, assertion) = insert(
        doc,
        Node::Assertion(editor_core::Claim::Value {
            value,
            bound: len(0.0),
            relation: AssertionRelation::Equal,
        }),
    );
    (doc, measure, assertion)
}

/// **(The unit's acceptance) A vertex resting on a face and an edge
/// lying on one, between two arrivals, each hold a `Gap = 0` over their
/// two cells, and the gap reads the prism's lift**: `+0.01` moved
/// apart, `−0.01` moved into the block, in either seat. Red if `Gap`
/// still admits faces only (the measure is refused at its door), if a
/// seat flips the sign, or if the edge reads at its higher end.
#[test]
fn a_vertex_and_an_edge_resting_on_a_face_read_their_gap() {
    for rest in [Rest::Edge, Rest::Vertex] {
        for lift in [0.0, 0.01, -0.01] {
            let (doc, top, cell) = resting(rest, lift);
            for (outer, inner, seat) in [(&top, &cell, "face outer"), (&cell, &top, "cell outer")] {
                let (doc, measure, assertion) = asserted_contact(&doc, outer, inner);
                let run = corpus::eval::<f64>(&doc);
                let value = match run.nodes.get(&measure) {
                    Some(NodeResult::Ok(v)) => match &v.payload {
                        ValuePayload::Measure { value, .. } => *value,
                        other => panic!("{rest:?} {seat}: a measure, not {}", other.kind_name()),
                    },
                    other => panic!("{rest:?} {seat}, lift {lift}: the gap reads: {other:?}"),
                };
                assert!(
                    (value - lift).abs() < 1e-9,
                    "{rest:?} {seat}: the gap at lift {lift} reads {value}"
                );
                let holds = matches!(
                    run.value(assertion).map(|v| &v.payload),
                    Some(ValuePayload::Assertion(AssertionVerdict::Holds { .. }))
                );
                assert_eq!(
                    holds,
                    lift == 0.0,
                    "{rest:?} {seat}: `Gap = 0` holds exactly at rest, lift {lift}"
                );
            }
        }
    }
}

/// **The cell arms on real bodies**: each row reads a wedge or a corner
/// off a body's own topology, so the fold (`max` against `min`) and the
/// face sense are read as a document builds them, where the unit rows
/// in `eval::measure` state them by hand. Each row's either-seat pair is
/// two match arms, so each assertion fails on its own.
mod real_bodies {
    use crate::corpus;
    use crate::fixture::{ename, in_world, insert, len, place, vname, vpiece};

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
                on: None,
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
        let doc = ProfileDoc::empty_derived("gap-cells-real", Tol::witness());
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
    fn real_convex_corner_reads_a_vertex_by_max() {
        for (dx, want) in [(0.01, 0.01), (-0.01, -0.01)] {
            let (doc, a, b, pa, pb) = two_blocks([1.0 + dx, 0.98, 0.98]);
            let ca = vpiece(&doc, a, 0, 2);
            let cb = vpiece(&doc, b, 0, 0);
            let corner = SitedRef::new(pa, vname(a, RoleSeg::CapVertex(CapEnd::End, ca)));
            let tip = SitedRef::new(pb, vname(b, RoleSeg::CapVertex(CapEnd::Start, cb)));
            near(
                gap(&doc, &corner, &tip),
                want,
                &format!("corner reads vertex, dx {dx}"),
            );
            // The block's +x/+y strut edge at (1, 1) reads the vertex too, in
            // either seat.
            let strut = SitedRef::new(pa, ename(a, RoleSeg::LateralEdge(ca)));
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

    /// **A real L-bracket's reflex strut reads a vertex by its link, and
    /// so does its reflex top vertex**, a saddle (two convex rims, one
    /// concave strut). The cube's corner at `(1.02, 0.99)` is inside the
    /// bracket's long leg (`y < 1`), `0.01` from its notch wall: `−0.01`;
    /// at `(1.02, 1.01)` it is in the notch, `+0.01`. Reading the strut's
    /// two face planes as an intersection reads `+0.02` in the leg, and
    /// reading the vertex's three as either fold reads a wrong sign at
    /// one of the two.
    #[test]
    fn real_reflex_strut_and_its_saddle_vertex_read_by_their_links() {
        let doc = ProfileDoc::empty_derived("gap-cells-real-l", Tol::witness());
        let (doc, l) = extrude(doc, vec![poly(&L)], 1.0);
        let (doc, b) = extrude(doc, vec![poly(&UNIT)], 1.0);
        let (doc, pl) = place(doc, l);
        for (y, want) in [(0.99, -0.01), (1.01, 0.01)] {
            let (doc, pb) = in_world(doc.clone(), b, &at([1.02, y, 0.5]));
            let reflex = vpiece(&doc, l, 0, 3);
            let cb = vpiece(&doc, b, 0, 0);
            let strut = SitedRef::new(pl, ename(l, RoleSeg::LateralEdge(reflex)));
            let tip = SitedRef::new(pb, vname(b, RoleSeg::CapVertex(CapEnd::Start, cb)));
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
            let top = SitedRef::new(pl, vname(l, RoleSeg::CapVertex(CapEnd::End, reflex)));
            near(
                gap(&doc, &top, &tip),
                want,
                &format!("saddle vertex, y {y}"),
            );
        }
    }

    /// **Two real ridges crossing at right angles.** The block's vertical
    /// `(1, 1)` strut (material `x, y ≤ 1`) and a wedge prism's apex ridge
    /// along `y` at `x = 1 + h, z = 0.5` (material `x ≥ 1 + h`): the ridges
    /// are `h` apart. Each seat reads the sign of `h`; the magnitudes are
    /// each outer link's own distance to the inner segment.
    #[test]
    fn real_crossing_ridges_read_the_sign_in_both_seats() {
        for h in [0.0, 0.01, -0.01] {
            let doc = ProfileDoc::empty_derived("gap-cells-real-x", Tol::witness());
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
            let strut = SitedRef::new(pa, ename(a, RoleSeg::LateralEdge(ca)));
            let ridge = SitedRef::new(pw, ename(w, RoleSeg::LateralEdge(apex)));
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

    /// **Two real ridges whose lines meet read, in both seats, about any
    /// axis.** The wedge prism turned a quarter turn about 24 axes off the
    /// coordinate ones, placed so its apex ridge passes through
    /// `(1, 1, 0.5)` on the block's strut. Each inner edge passes through
    /// a point of the outer's boundary, so its lowest offset is zero or
    /// below (below where its line enters the outer's cone, as a generic
    /// line through a convex edge does). Every other row here is
    /// axis-aligned, which hid the double roots rounding scatters about a
    /// contact. Red if a seat refuses or reads above zero.
    #[test]
    fn real_ridges_whose_lines_meet_read_about_any_axis() {
        let mut bad = Vec::new();
        for k in 0..24 {
            let kf = f64::from(k);
            let a = [
                1.0 + 0.37 * kf.sin(),
                0.6 * (1.3 * kf).cos(),
                0.25 + 0.5 * (0.7 * kf).sin(),
            ];
            let n = (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt();
            let a = [a[0] / n, a[1] / n, a[2] / n];
            // The quarter turn about `a` takes `+z` to `a × z + a·a_z`.
            let d = [a[1] + a[0] * a[2], -a[0] + a[1] * a[2], a[2] * a[2]];
            let doc = ProfileDoc::empty_derived("gap-cells-real-turned", Tol::witness());
            let (doc, blk) = extrude(doc, vec![poly(&SQUARE)], 1.0);
            let (doc, w) = extrude(doc, vec![poly(&[(0.0, 0.0), (1.0, -0.3), (1.0, 0.3)])], 2.0);
            let (doc, pa) = place(doc, blk);
            let frame = Frame::rotate_by_turns_then_translate(
                a,
                Turns::new(1, 4).expect("a quarter"),
                [1.0 - d[0], 1.0 - d[1], 0.5 - d[2]],
                crate::fixture::band(),
            )
            .expect("a unit axis");
            let (doc, pw) = in_world(doc, w, &frame);
            let ca = vpiece(&doc, blk, 0, 2);
            let apex = vpiece(&doc, w, 0, 0);
            let strut = SitedRef::new(pa, ename(blk, RoleSeg::LateralEdge(ca)));
            let ridge = SitedRef::new(pw, ename(w, RoleSeg::LateralEdge(apex)));
            for (o, i, seat) in [
                (&strut, &ridge, "strut outer"),
                (&ridge, &strut, "ridge outer"),
            ] {
                match gap(&doc, o, i) {
                    Ok(g) if g <= 1e-9 => {}
                    got => bad.push(format!("axis {k} {a:.3?}, {seat}: {got:?}")),
                }
            }
        }
        assert!(
            bad.is_empty(),
            "{} of 48 seats:\n{}",
            bad.len(),
            bad.join("\n")
        );
    }

    /// **A real pin and a real bore read a vertex by their sense.** A disc
    /// extrude's wall (a pin, radius 0.5) and a plate's circular hole (a
    /// bore, radius 0.5), each about the z axis; a cube corner at
    /// `(0.5 + d, 0, 0.25)`. Outside the pin is `+d`; at `d < 0` the corner
    /// is in the bore's hollow, `+|d|`, and at `d > 0` in the plate, `−d`.
    #[test]
    fn real_pin_and_bore_read_a_vertex_by_their_sense() {
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
            let doc = ProfileDoc::empty_derived("gap-cells-real-cyl", Tol::witness());
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
            let tip = SitedRef::new(pb, vname(b, RoleSeg::CapVertex(CapEnd::Start, cb)));
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

    /// **The Interval lane reads the cell arms, and encloses the f64
    /// value**: on the real convex corner (vertex x vertex, edge x vertex
    /// in both seats) and the real crossing ridges (edge x edge, through
    /// `lowest_on`'s candidates, at contact too), every reading reads and
    /// its enclosure contains the f64 value. Red if one refuses, or misses.
    /// (An enclosure over a box of inputs is `topo::link`'s
    /// `an_interval_offset_encloses_its_box`.)
    #[test]
    fn interval_lane_encloses_the_f64_gap() {
        let read = core::cell::Cell::new(0);
        let check = |doc: &ProfileDoc, o: &SitedRef, i: &SitedRef, what: &str| {
            let f = gap(doc, o, i).expect("f64 reads");
            let (lo, hi) =
                gap_interval(doc, o, i).unwrap_or_else(|e| panic!("{what}: refused {e}"));
            assert!(lo <= f && f <= hi, "{what}: [{lo}, {hi}] misses {f}");
            read.set(read.get() + 1);
        };
        for dx in [0.01, -0.01, 0.0] {
            let (doc, a, b, pa, pb) = two_blocks([1.0 + dx, 0.98, 0.98]);
            let ca = vpiece(&doc, a, 0, 2);
            let cb = vpiece(&doc, b, 0, 0);
            let corner = SitedRef::new(pa, vname(a, RoleSeg::CapVertex(CapEnd::End, ca)));
            let tip = SitedRef::new(pb, vname(b, RoleSeg::CapVertex(CapEnd::Start, cb)));
            let strut = SitedRef::new(pa, ename(a, RoleSeg::LateralEdge(ca)));
            check(&doc, &corner, &tip, &format!("corner/vertex dx {dx}"));
            check(&doc, &strut, &tip, &format!("strut/vertex dx {dx}"));
            check(&doc, &tip, &strut, &format!("vertex/strut dx {dx}"));
        }
        for h in [0.0, 0.01, -0.01] {
            let doc = ProfileDoc::empty_derived("gap-cells-real-xi", Tol::witness());
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
            let strut = SitedRef::new(pa, ename(a, RoleSeg::LateralEdge(ca)));
            let ridge = SitedRef::new(pw, ename(w, RoleSeg::LateralEdge(apex)));
            check(&doc, &strut, &ridge, &format!("strut/ridge h {h}"));
            check(&doc, &ridge, &strut, &format!("ridge/strut h {h}"));
        }
        assert_eq!(read.get(), 15, "every reading read");
    }
}
