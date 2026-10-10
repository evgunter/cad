//! Review probes (reviewer 2) for INTENT stage 5 B.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{ang, insert, len, on_frame, place, square};
use editor_core::{
    CapEnd, DocEdit, EntityKind, ExtrudeSide, MeasurePrimitive, Node, ProfileDoc, RecipeNodeId,
    RoleSeg, SitedRef, StableName,
};
use geom_core::Tol;

fn block(doc: ProfileDoc, cx: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(cx, 0.0, 0.5)],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    )
}

fn cap(node: RecipeNodeId, end: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::Cap(end)],
    }
}

/// Claim 6: a construction seat that reads, by variable, the selection
/// a measure minted over a world copy. The edit door must refuse
/// `ReadsWorldCopy` (and agree with the load door).
#[test]
fn r2_a_construction_reading_a_measures_world_copy_selection_by_var() {
    let doc = ProfileDoc::empty_derived("r2-probe", Tol::witness());
    let (doc, a) = block(doc, 0.0);
    let (doc, b) = block(doc, 3.0);
    let (doc, p) = place(doc, a);
    let (doc, q) = place(doc, b);
    let (doc, measure) = crate::fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![
            SitedRef::new(p, cap(a, CapEnd::End).in_copy(p)),
            SitedRef::new(q, cap(b, CapEnd::End).in_copy(q)),
        ],
    );
    let Some(Node::Measure {
        primitive: MeasurePrimitive::Distance { a: sel, .. },
    }) = doc.node(measure)
    else {
        panic!("a distance measure")
    };
    let sel = *sel;
    let (doc, _) = crate::fixture::step(
        doc,
        DocEdit::RenameVar {
            var: sel.into(),
            name: Some(editor_core::VarName::from_static("face_of_copy")),
        },
    );
    let edited = doc.apply(
        &DocEdit::InsertNode {
            node: Box::new(Node::Datum(editor_core::Datum::FaceFrame {
                face: editor_core::Operand::Var(sel),
                spin: ang(0.0),
            })),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    let doc = match edited {
        Err(editor_core::EditError::ReadsWorldCopy { .. }) => return,
        Err(other) => panic!("refused otherwise: {other}"),
        Ok(o) => o.doc,
    };
    let saved = editor_core::persist::save(&doc, &[], Tol::witness()).map(|_| "saves");
    panic!(
        "the edit door admitted a face frame (a construction) reading a world copy's selection \
         by variable; the save door then says: {saved:?}"
    );
}

/// Claim 6, the orphan: a named selection of a world copy that a
/// measure read, the measure then deleted. The edit door takes the
/// delete; does the document still save and load?
#[test]
fn r2_a_named_world_copy_selection_outlives_its_measure() {
    let doc = ProfileDoc::empty_derived("r2-orphan", Tol::witness());
    let (doc, a) = block(doc, 0.0);
    let (doc, b) = block(doc, 3.0);
    let (doc, p) = place(doc, a);
    let (doc, q) = place(doc, b);
    let (doc, measure) = crate::fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![
            SitedRef::new(p, cap(a, CapEnd::End).in_copy(p)),
            SitedRef::new(q, cap(b, CapEnd::End).in_copy(q)),
        ],
    );
    let Some(Node::Measure {
        primitive: MeasurePrimitive::Distance { a: sel, .. },
    }) = doc.node(measure)
    else {
        panic!("a distance measure")
    };
    let sel = *sel;
    let (doc, _) = crate::fixture::step(
        doc,
        DocEdit::RenameVar {
            var: sel.into(),
            name: Some(editor_core::VarName::from_static("face_of_copy")),
        },
    );
    let deleted = doc.apply(
        &DocEdit::DeleteNode { id: measure },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    let doc = match deleted {
        Ok(o) => o.doc,
        Err(e) => {
            eprintln!("delete refused: {e}");
            return;
        }
    };
    eprintln!("selection still held: {}", doc.var(sel).is_some());
    let saved = editor_core::persist::save(&doc, &[], Tol::witness());
    let loaded = saved.as_ref().map_err(|e| format!("{e:?}")).and_then(|t| {
        editor_core::persist::load(t, Tol::witness())
            .map(|_| ())
            .map_err(|e| e.to_string())
    });
    assert!(
        loaded.is_ok(),
        "an edit-door document that no longer saves/loads: {loaded:?}"
    );
}

mod interference {
    #![allow(dead_code)]
    use crate::corpus;
    use crate::fixture::{fname, insert, len, on_frame, place, square, wall};
    use editor_core::{
        Assembly, AssertionRelation, ExtrudeSide, Formula, MeasurePrimitive, Node, ProfileDoc,
        RecipeNodeId, RoleSeg, SitedRef, StableName, assemble,
    };
    use geom_core::Tol;

    /// A block extruded `height` from a square of half-side `h` at
    /// `(cx, cy)`, on a frame `z` up the world's z axis: the extrude.
    fn sblock(
        doc: ProfileDoc,
        (cx, cy, z): (f64, f64, f64),
        h: f64,
        height: f64,
    ) -> (ProfileDoc, RecipeNodeId) {
        let (doc, profile) = on_frame(
            doc,
            [0.0, 0.0, z],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![square(cx, cy, h)],
        );
        insert(
            doc,
            Node::Extrude {
                profile: profile.into(),
                distance: len(height),
                side: ExtrudeSide::Along,
            },
        )
    }

    /// The `square` wall facing +x (segment 1) and the one facing −x
    /// (segment 3), as `block`'s extrude names them.
    fn plus_x(doc: &ProfileDoc, b: RecipeNodeId) -> StableName {
        fname(b, wall(doc, b, 1))
    }

    fn minus_x(doc: &ProfileDoc, b: RecipeNodeId) -> StableName {
        fname(b, wall(doc, b, 3))
    }

    fn assembled(doc: &ProfileDoc) -> Assembly<f64> {
        let run = corpus::eval::<f64>(doc);
        assemble(doc, &run, Tol::witness()).unwrap_or_else(|e| panic!("the gate refused: {e}"))
    }

    /// A `Gap` over two faces read at `sites`, and an assertion bounding it
    /// by `relation` and `bound` metres: the document after them, and the
    /// assertion.
    fn asserted_gap(
        doc: ProfileDoc,
        sites: [SitedRef; 2],
        relation: AssertionRelation,
        bound: f64,
    ) -> (ProfileDoc, RecipeNodeId) {
        let (doc, measure) = crate::fixture::measure_node(
            &doc,
            MeasurePrimitive::Gap { outer: 0, inner: 1 },
            sites.to_vec(),
        );
        let value: Formula = crate::fixture::value_of(&doc, measure);
        insert(
            doc,
            Node::Assertion {
                value,
                bound: len(bound),
                relation,
            },
        )
    }

    /// A `Gap` over faces of two world copies, read where each copy holds
    /// it (`face.in_copy(placement)`), bounded by `relation` and `bound`.
    fn gap_at_copies(
        doc: ProfileDoc,
        (p, outer): (RecipeNodeId, StableName),
        (q, inner): (RecipeNodeId, StableName),
        relation: AssertionRelation,
        bound: f64,
    ) -> (ProfileDoc, RecipeNodeId) {
        asserted_gap(
            doc,
            [
                SitedRef::new(p, outer.in_copy(p)),
                SitedRef::new(q, inner.in_copy(q)),
            ],
            relation,
            bound,
        )
    }

    /// A plate with a 0.5 bore and a pin of radius `pin_r` through it,
    /// each placed once: `(doc, plate, pin, copy of plate, copy of pin,
    /// the bore's wall, the pin's wall)`.
    fn bore_and_pin(
        pin_r: f64,
    ) -> (
        ProfileDoc,
        RecipeNodeId,
        RecipeNodeId,
        RecipeNodeId,
        StableName,
        StableName,
    ) {
        bore_pin_and_holes(pin_r, &[])
    }

    /// [`bore_and_pin`] with further holes `(cx, cy, r)` through the plate.
    fn bore_pin_and_holes(
        pin_r: f64,
        holes: &[(f64, f64, f64)],
    ) -> (
        ProfileDoc,
        RecipeNodeId,
        RecipeNodeId,
        RecipeNodeId,
        StableName,
        StableName,
    ) {
        use crate::fixture::{frame, piece};
        use editor_core::{LoopProgram, ProfileProgram};
        let circle = |r| LoopProgram::<Formula>::circle(0.0, 0.0, r).expect("a literal circle");
        let prism = |doc: ProfileDoc, z, loops, h| {
            let (doc, plane) = insert(doc, frame([0.0, 0.0, z], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
            let (doc, profile) = insert(
                doc,
                Node::Profile(ProfileProgram {
                    frame: plane.into(),
                    loops,
                    ids: Vec::new(),
                }),
            );
            insert(
                doc,
                Node::Extrude {
                    profile: profile.into(),
                    distance: len(h),
                    side: ExtrudeSide::Along,
                },
            )
        };
        let doc = ProfileDoc::empty_derived("r2-press", Tol::witness());
        let outline = crate::fixture::desc(RecipeNodeId::new(0, 0), vec![square(0.0, 0.0, 1.0)])
            .loops
            .remove(0);
        let mut loops = vec![outline, circle(0.5)];
        loops.extend(holes.iter().map(|&(cx, cy, r)| {
            LoopProgram::<Formula>::circle(cx, cy, r).expect("a literal circle")
        }));
        let (doc, plate) = prism(doc, 0.0, loops, 1.0);
        let (doc, pin) = prism(doc, -0.5, vec![circle(pin_r)], 2.0);
        let (doc, p) = place(doc, plate);
        let (doc, q) = place(doc, pin);
        let lateral = |doc: &ProfileDoc, node, l| {
            fname(node, RoleSeg::Lateral(piece(doc, node, l, 0).into()))
        };
        let (bore, wall) = (lateral(&doc, plate, 1), lateral(&doc, pin, 0));
        (doc, p, q, plate, bore, wall)
    }

    fn report(doc: &ProfileDoc) -> Vec<(bool, String)> {
        let run = corpus::eval::<f64>(doc);
        match assemble(doc, &run, Tol::witness()) {
            Ok(a) => a
                .interference
                .iter()
                .map(|f| (f.is_loud(), format!("{:?}", f.overlap)))
                .collect(),
            Err(e) => panic!("refused: {e}"),
        }
    }

    /// Roles swapped on the press fit: Gap(pin, bore). Must stay loud.
    #[test]
    fn r2_press_fit_roles_swapped_is_loud() {
        let (doc, p, q, _, bore, pinwall) = bore_and_pin(0.505);
        let (doc, _) = gap_at_copies(
            doc,
            (q, pinwall),
            (p, bore),
            AssertionRelation::AtMost,
            -0.001,
        );
        let r = report(&doc);
        assert!(r.iter().all(|(loud, _)| *loud), "{r:?}");
    }

    /// Back-to-back planes: b runs through a's -x wall (b: x in [-0.7, 0.3]).
    /// The overlap x in [-0.5, 0.3] is bounded by a's -x wall and b's +x
    /// wall, whose outward normals are antiparallel (pointing apart).
    /// Gap(a -x, b +x) = (0.3 - -0.5) * -1 = -0.8.
    #[test]
    fn r2_back_to_back_planes() {
        let doc = ProfileDoc::empty_derived("r2-b2b", Tol::witness());
        let (doc, a) = sblock(doc, (0.0, 0.0, 0.0), 0.5, 1.0);
        let (doc, b) = sblock(doc, (-0.2, 0.0, 0.25), 0.5, 0.5);
        // b's square: half-side 0.5 at cx -0.2 -> x in [-0.7, 0.3]; y [-0.5,0.5] coincident with a's y walls. Shrink y:
        let _ = b;
        let doc2 = doc.clone();
        let (doc, b) = {
            let (d, profile) = on_frame(
                doc2,
                [0.0, 0.0, 0.25],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                vec![vec![(-0.7, -0.3), (0.3, -0.3), (0.3, 0.3), (-0.7, 0.3)]],
            );
            insert(
                d,
                Node::Extrude {
                    profile: profile.into(),
                    distance: len(0.5),
                    side: ExtrudeSide::Along,
                },
            )
        };
        let (doc, p) = place(doc, a);
        let (doc, q) = place(doc, b);
        let loud = report(&doc);
        eprintln!("unasserted: {loud:?}");
        let (doc, _) = gap_at_copies(
            doc.clone(),
            (p, minus_x(&doc, a)),
            (q, plus_x(&doc, b)),
            AssertionRelation::AtMost,
            -0.01,
        );
        let r = report(&doc);
        eprintln!("asserted Gap(a -x, b +x) <= -0.01: {r:?}");
    }

    /// Two parallel convex shafts side by side, clashing: A r=1 at the
    /// origin, B r=0.3 at x=1.2, both 1 long. Gap(A, B) = 1 - 0.3 - 1.2 =
    /// -0.5, which holds under <= -0.01, though the clash is 0.1 deep and
    /// neither face is a socket. Both lateral faces bound the crescent
    /// overlap, and it lies between their carriers. Only `opposed`'s sense
    /// test (outer concave, inner convex) keeps it loud.
    #[test]
    fn r2_two_convex_shafts_side_by_side_stay_loud() {
        use crate::fixture::{frame, piece};
        use editor_core::{LoopProgram, ProfileProgram};
        let rod = |doc: ProfileDoc, cx: f64, r: f64, z: f64| {
            let (doc, plane) = insert(doc, frame([0.0, 0.0, z], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
            let (doc, profile) = insert(
                doc,
                Node::Profile(ProfileProgram {
                    frame: plane.into(),
                    loops: vec![LoopProgram::<Formula>::circle(cx, 0.0, r).expect("a circle")],
                    ids: Vec::new(),
                }),
            );
            insert(
                doc,
                Node::Extrude {
                    profile: profile.into(),
                    distance: len(if z == 0.0 { 1.0 } else { 0.5 }),
                    side: ExtrudeSide::Along,
                },
            )
        };
        let doc = ProfileDoc::empty_derived("r2-shafts", Tol::witness());
        let (doc, a) = rod(doc, 0.0, 1.0, 0.0);
        let (doc, b) = rod(doc, 1.2, 0.3, 0.25);
        let (doc, p) = place(doc, a);
        let (doc, q) = place(doc, b);
        let lateral =
            |doc: &ProfileDoc, node| fname(node, RoleSeg::Lateral(piece(doc, node, 0, 0).into()));
        let (fa, fb) = (lateral(&doc, a), lateral(&doc, b));
        let unasserted = report(&doc);
        assert!(
            !unasserted.is_empty() && unasserted.iter().all(|(l, _)| *l),
            "{unasserted:?}"
        );
        let (doc, assertion) =
            gap_at_copies(doc, (p, fa), (q, fb), AssertionRelation::AtMost, -0.01);
        let run = corpus::eval::<f64>(&doc);
        eprintln!(
            "assertion verdict: {:?}",
            run.value(assertion).map(|v| &v.payload)
        );
        let r = report(&doc);
        assert!(
            r.iter().all(|(l, _)| *l),
            "side-by-side shafts quieted: {r:?}"
        );
    }

    /// The sunk flange with the `Gap`'s roles swapped (a plane pair's gap
    /// is symmetric, so the assertion holds the same): the foot now lies
    /// beyond the OUTER face's carrier. Pins the outer half of the
    /// containment check, which no PR row crosses.
    #[test]
    fn r2_sunk_flange_beyond_the_outer_carrier_stays_loud() {
        let doc = ProfileDoc::empty_derived("r2-flange-swapped", Tol::witness());
        let (doc, a) = sblock(doc, (0.0, 0.0, 0.0), 0.5, 1.0);
        let (doc, profile) = on_frame(
            doc,
            [0.0, 0.0, 0.25],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![
                (0.45, -0.3),
                (1.05, -0.3),
                (1.05, 0.3),
                (0.3, 0.3),
                (0.3, 0.1),
                (0.45, 0.1),
            ]],
        );
        let (doc, b) = insert(
            doc,
            Node::Extrude {
                profile: profile.into(),
                distance: len(0.5),
                side: ExtrudeSide::Along,
            },
        );
        let (doc, p) = place(doc, a);
        let (doc, q) = place(doc, b);
        let (doc, assertion) = gap_at_copies(
            doc.clone(),
            (q, fname(b, wall(&doc, b, 5))),
            (p, plus_x(&doc, a)),
            AssertionRelation::AtMost,
            -0.01,
        );
        let run = corpus::eval::<f64>(&doc);
        assert!(
            matches!(
                run.value(assertion).map(|v| &v.payload),
                Some(editor_core::ValuePayload::Assertion(
                    editor_core::AssertionVerdict::Holds { .. }
                ))
            ),
            "the swapped gap holds"
        );
        let r = report(&doc);
        assert!(
            r.iter().all(|(l, _)| *l),
            "the foot is beyond the outer carrier: {r:?}"
        );
    }

    /// Observation, not a claim: a press-fit pin FLUSH with the plate (pin
    /// length = plate thickness, caps coplanar). Prints what the gate says.
    #[test]
    fn r2_observe_flush_press_fit() {
        use crate::fixture::{frame, piece};
        use editor_core::{LoopProgram, ProfileProgram};
        let circle = |r| LoopProgram::<Formula>::circle(0.0, 0.0, r).expect("a literal circle");
        let prism = |doc: ProfileDoc, z, loops, h| {
            let (doc, plane) = insert(doc, frame([0.0, 0.0, z], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
            let (doc, profile) = insert(
                doc,
                Node::Profile(ProfileProgram {
                    frame: plane.into(),
                    loops,
                    ids: Vec::new(),
                }),
            );
            insert(
                doc,
                Node::Extrude {
                    profile: profile.into(),
                    distance: len(h),
                    side: ExtrudeSide::Along,
                },
            )
        };
        let doc = ProfileDoc::empty_derived("r2-flush", Tol::witness());
        let outline = crate::fixture::desc(RecipeNodeId::new(0, 0), vec![square(0.0, 0.0, 1.0)])
            .loops
            .remove(0);
        let (doc, plate) = prism(doc, 0.0, vec![outline, circle(0.5)], 1.0);
        let (doc, pin) = prism(doc, 0.0, vec![circle(0.505)], 1.0);
        let (doc, _) = place(doc, plate);
        let (doc, _) = place(doc, pin);
        let _ = piece;
        let run = corpus::eval::<f64>(&doc);
        match assemble(&doc, &run, Tol::witness()) {
            Ok(a) => eprintln!("FLUSH: Ok, {} finding(s)", a.interference.len()),
            Err(e) => eprintln!(
                "FLUSH: refused: {}",
                e.to_string()
                    .lines()
                    .take(3)
                    .collect::<Vec<_>>()
                    .join(" | ")
            ),
        }
    }
}
