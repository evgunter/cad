//! INTENT stage 2 PR C: the product is the world
//! (`docs/INTENT-STAGE2-SPEC.md` §4, §9 rows 6–12). The product is
//! every copy a `PlaceInWorld` defines, in the placements' document
//! order, and nothing else places.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use crate::corpus;
use crate::fixture::resolver::PartStore;
use crate::fixture::value_channel::body_digest;
use crate::fixture::{insert, len, on_frame, place, square};
use editor_core::{
    AssertionDir, BooleanOp, CapEnd, ChecksConfig, DocEdit, DocumentId, EntityKind, ExtrudeSide,
    InlineError, Maintenance, MeasureExpr, MeasurePrimitive, Node, NodeResult, ProductError,
    ProfileDoc, RecipeNodeId, RoleSeg, SitedRef, SplitError, StableName, product, product_named,
    run_checks,
};
use geom_core::Tol;

/// The committed files test 6 regenerates, relative to the repo root.
const FILES: [&str; 5] = [
    "crates/editor-core/tests/golden/golden.cad",
    "crates/editor-core/tests/corpus/die_tool.pncad",
    "crates/editor-core/tests/corpus/tour/die_composed_tour.pncad",
    "crates/viewer/tests/gallery_ring.pncad",
    "crates/pncad/tests/plate_param.pncad",
];

/// Every document's product before the world, as the base gathered it
/// over its A10 root list: the aggregate's digest word and the count
/// of words fed. Taken by [`print_product_rows`] at the base (stage 2
/// unit B's head, `5fed25fe7`), before any placement existed.
const PRE_C: [(&str, &str); 34] = [
    ("die", "fa9ad86b30e8d844/532"),
    ("corner_table", "1b26e7ae1ba4a066/100"),
    ("heat_sink", "986dd0b900fe3d32/268"),
    ("crossing_slots", "23d227ff22f6c626/100"),
    ("nested_islands_105", "07eda6db4f12636d/100"),
    ("nested_islands_106_depth1", "32fed3f5e12387ee/52"),
    ("nested_islands_106_depth2", "3a7999e85a64a33b/76"),
    ("declared_tangency", "ea1ce7f968a62d47/88"),
    ("kitchen_sink", "a4152d07cf293bb9/196"),
    ("cut_cylinder", "cb8ccadf87044e7f/28"),
    ("measured_web", "f556ab19f06e9f9e/28"),
    ("boss_union", "a07c97077aedd76c/46"),
    ("die_fillet", "934f10b8156f9396/76"),
    ("die_chamfer", "934f10b8156f9396/76"),
    ("die_pips", "2859880c273d7b35/37"),
    ("heat_sink_fins", "8a57c326dc5f71aa/124"),
    ("die_tool", "1ecb739a2d22c24d/82"),
    ("face_sketch", "c7144cb82dbe5c86/28"),
    ("part_select", "e54d217adfa64523/52"),
    ("loft_prism", "36c0de6a313e3a66/28"),
    ("die_composed", "26837bfbed2b6372/91"),
    ("die_composed_tour", "1ee773821965aac1/391"),
    ("plate_param", "fd7b71fa51270532/88"),
    ("kiss_carry", "fabe53fd5101f503/88"),
    ("tube_ring", "c342379c65eab6d1/10"),
    ("tube_arc", "19b8e0af8227889b/16"),
    ("hollow_tube_elbow", "9ae3e25a75d4fecd/28"),
    ("hollow_tube_ring", "1db62df6ebe2507d/16"),
    ("reshaped_rod", "128f9facfb4a7626/52"),
    (FILES[0], "8853ddff30bcec28/154"),
    (FILES[1], "1ecb739a2d22c24d/82"),
    (FILES[2], "1ee773821965aac1/391"),
    (FILES[3], "07c4e18f181e9b69/16"),
    (FILES[4], "fd7b71fa51270532/88"),
];

/// Every document test 6 compares: the corpus, then the files.
fn documents() -> Vec<(String, ProfileDoc)> {
    let mut out: Vec<(String, ProfileDoc)> = corpus::documents()
        .into_iter()
        .map(|d| (d.name.to_string(), d.doc))
        .collect();
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let process_epsilon = editor_core::persist::save(
        &ProfileDoc::empty_derived("intent-s2-c-epsilon", Tol::witness()),
        &[],
        Tol::witness(),
    )
    .expect("an empty document saves")
    .lines()
    .find(|l| l.trim_start().starts_with("\"epsilon\":"))
    .expect("a saved document records its ε")
    .to_owned();
    for file in FILES {
        // A file records the ε it was saved at, and one process holds
        // one ε: the record is read at the process's, so the document
        // is the process's at every ε row.
        let text: String = std::fs::read_to_string(here.join(file))
            .expect("reads")
            .lines()
            .map(|l| match l.trim_start().starts_with("\"epsilon\":") {
                true => format!("{}\n", process_epsilon),
                false => format!("{l}\n"),
            })
            .collect();
        let doc = editor_core::persist::load(&text, Tol::witness())
            .unwrap_or_else(|e| panic!("{file} loads: {e}"))
            .doc;
        out.push((file.to_string(), doc));
    }
    out
}

/// One document's product, as a digest word and the count of words
/// fed, or the refusal's class.
fn product_row(doc: &ProfileDoc) -> String {
    let ev = corpus::eval::<f64>(doc);
    match product(doc, &ev, Tol::witness()) {
        Ok(body) => {
            let (word, fed) = body_digest(&body);
            format!("{word:016x}/{fed}")
        }
        Err(e) => format!("refused {:?}", e.kind()),
    }
}

/// Prints every document's product row: the record [`PRE_C`] holds was
/// taken by this at the base.
#[test]
#[ignore = "a probe: prints the product rows"]
fn print_product_rows() {
    for (name, doc) in documents() {
        println!("(\"{name}\", \"{}\"),", product_row(&doc));
    }
}

/// **(C, test 6) The one-time migration.** Every corpus document and
/// committed file, regenerated with one placement per body its base
/// gathered, in its base's root order, has the base's product: the
/// same aggregate, solid for solid and in order.
#[test]
fn the_migrated_world_is_the_pre_c_product() {
    let docs = documents();
    assert_eq!(docs.len(), PRE_C.len(), "one record per document");
    for ((name, doc), (recorded, row)) in docs.iter().zip(PRE_C) {
        assert_eq!(name, recorded, "the record is in the corpus's order");
        assert!(
            !doc.placements().is_empty(),
            "{name}: the regenerated document places its bodies"
        );
        assert_eq!(product_row(doc), row, "{name}: the product moved");
    }
}

/// A `1 × 1 × 1` block extruded from a square at `cx`: the extrude.
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

fn ev(doc: &ProfileDoc) -> editor_core::Evaluation<f64> {
    corpus::eval::<f64>(doc)
}

/// A cap of `node`, as its own table names it.
fn cap(node: RecipeNodeId, end: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::Cap(end)],
    }
}

/// **(C, test 7) A measured part stays in the product.** A block whose
/// caps a `Measure` reads and an `Assertion` bounds, placed: the
/// product holds the block. Under the sink rule the measure read it,
/// so it was no root, and the product refused.
#[test]
fn a_measured_and_asserted_block_placed_is_the_product() {
    let doc = ProfileDoc::empty_derived("intent-c-measured", Tol::witness());
    let (doc, b) = block(doc, 0.0);
    let (doc, measure) = insert(
        doc,
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            vec![
                SitedRef::new(b, cap(b, CapEnd::Start)),
                SitedRef::new(b, cap(b, CapEnd::End)),
            ],
        )
        .expect("indices in range"),
    );
    let (doc, _) = insert(
        doc,
        Node::Assertion {
            measure: measure.into(),
            bound: len(0.5),
            dir: AssertionDir::AtLeast,
        },
    );
    let (doc, _) = place(doc, b);
    let run = ev(&doc);
    let body = product(&doc, &run, Tol::witness()).expect("the placed block is the product");
    let Some(NodeResult::Ok(value)) = run.result(b) else {
        panic!("the block evaluates")
    };
    let editor_core::ValuePayload::Body(own) = &value.payload else {
        panic!("the block is a body")
    };
    assert_eq!(
        body_digest(&body),
        body_digest(own),
        "the product is the measured block's body"
    );
}

/// **(C, test 8) A failing check gates nothing.** The measure names a
/// face its block's table does not carry, so it refuses and the
/// assertion over it has no verdict; the placed block is the product
/// all the same, and the checks registry runs.
#[test]
fn a_failing_measure_and_its_assertion_gate_no_placement() {
    let doc = ProfileDoc::empty_derived("intent-c-failing", Tol::witness());
    let (doc, b) = block(doc, 0.0);
    let vanished = StableName {
        kind: EntityKind::Face,
        node: b,
        path: vec![RoleSeg::OutputBody],
    };
    let (doc, measure) = insert(
        doc,
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            vec![
                SitedRef::new(b, cap(b, CapEnd::Start)),
                SitedRef::new(b, vanished),
            ],
        )
        .expect("indices in range"),
    );
    let (doc, assertion) = insert(
        doc,
        Node::Assertion {
            measure: measure.into(),
            bound: len(0.5),
            dir: AssertionDir::AtLeast,
        },
    );
    let (doc, _) = place(doc, b);
    let run = ev(&doc);
    assert!(
        matches!(run.result(measure), Some(NodeResult::Failed(_))),
        "the premise: the measure refuses"
    );
    assert!(
        !matches!(run.result(assertion), Some(NodeResult::Ok(_))),
        "and the assertion over it has no verdict"
    );
    product(&doc, &run, Tol::witness()).expect("the placed block is the product");
    run_checks(&doc, &run, &ChecksConfig::default(), Tol::witness())
        .expect("the checks run over the world");
}

/// **(C, test 9) Nothing places as a side effect.** Two placed blocks,
/// then a union of the two through the kernel edit: the product is the
/// two blocks as it was, and the union is unplaced.
#[test]
fn inserting_a_boolean_over_two_placed_blocks_places_nothing() {
    let doc = ProfileDoc::empty_derived("intent-c-side-effect", Tol::witness());
    let (doc, a) = block(doc, 0.0);
    let (doc, b) = block(doc, 0.75);
    let (doc, _) = place(doc, a);
    let (doc, _) = place(doc, b);
    let before = body_digest(&product(&doc, &ev(&doc), Tol::witness()).expect("two blocks"));
    let placements = doc.placements();
    let (doc, fused) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: a.into(),
            b: b.into(),
            declare: Vec::new(),
        },
    );
    assert_eq!(doc.placements(), placements, "the insert placed nothing");
    let after = body_digest(&product(&doc, &ev(&doc), Tol::witness()).expect("two blocks"));
    assert_eq!(after, before, "the product is the two blocks still");
    let fused_body = doc.output(fused, 0).expect("the union defines its body");
    assert!(
        doc.unplaced().contains(&fused_body),
        "the union is unplaced: {:?}",
        doc.unplaced()
    );
}

/// **(C, test 10) An empty world, and a stranded placement.** Bodies
/// with no placement refuse `EmptyProduct` naming every unplaced body
/// in document order; deleting a placed body is accepted, reporting the
/// placement's stranded read, and the gather refuses naming it.
#[test]
fn an_empty_world_names_the_unplaced_and_a_stranded_placement_refuses() {
    let doc = ProfileDoc::empty_derived("intent-c-empty", Tol::witness());
    let (doc, a) = block(doc, 0.0);
    let (doc, b) = block(doc, 3.0);
    let bodies = vec![doc.output(a, 0).unwrap(), doc.output(b, 0).unwrap()];
    match product(&doc, &ev(&doc), Tol::witness()) {
        Err(ProductError::EmptyProduct { unplaced }) => {
            assert_eq!(unplaced, bodies, "every unplaced body, in document order");
        }
        other => panic!("an empty world refuses EmptyProduct: {other:?}"),
    }

    let (doc, placement) = place(doc, a);
    let applied = doc
        .apply(
            &DocEdit::DeleteNode { id: a },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect("deleting a placed body is accepted");
    assert!(
        applied.maintenance.iter().any(|m| matches!(
            m,
            Maintenance::StrandedRead { node, .. } if node.id() == placement
        )),
        "the placement's read is reported stranded: {:?}",
        applied.maintenance
    );
    let doc = applied.doc;
    match product(&doc, &ev(&doc), Tol::witness()) {
        Err(ProductError::StrandedPlacement { placement: named }) => {
            assert_eq!(named, placement, "the gather names the placement");
        }
        other => panic!("a stranded placement refuses: {other:?}"),
    }
}

/// **(C, test 11) Two copies.** Two identity placements of one block:
/// two solids, each copy an output of its own, and each name of the
/// block reached once per copy.
#[test]
fn two_identity_placements_of_one_body_are_two_copies() {
    let doc = ProfileDoc::empty_derived("intent-c-copies", Tol::witness());
    let (doc, a) = block(doc, 0.0);
    let (doc, p) = place(doc, a);
    let (doc, q) = place(doc, a);
    let (cp, cq) = (doc.output(p, 0).unwrap(), doc.output(q, 0).unwrap());
    assert_ne!(cp, cq, "two copies, two output ids");
    let run = ev(&doc);
    let (body, names) = product_named(&doc, &run, Tol::witness()).expect("two copies gather");
    assert_eq!(body.solids().count(), 2, "two solids");
    for placement in [p, q] {
        let copy = StableName {
            kind: EntityKind::Face,
            node: placement,
            path: vec![RoleSeg::Placed {
                of: editor_core::NameRef::new(cap(a, CapEnd::End)),
            }],
        };
        assert!(
            matches!(names.lookup(&copy), Some(editor_core::Entry::Unique(_))),
            "the block's end cap is named once in copy {placement:?}"
        );
    }
}

/// The block, its placement and a transform of it nothing places, in
/// one document: what the split rows cut.
fn split_scene(label: &str) -> (ProfileDoc, [RecipeNodeId; 5]) {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let frame = doc.ids()[0];
    let (doc, b) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, p) = place(doc, b);
    let (doc, reader) = insert(
        doc,
        crate::fixture::xform(b, [5.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0),
    );
    (doc, [frame, profile, b, p, reader])
}

/// **(C, test 12) Split and inline under the world.** A cut that places
/// its body re-points a remainder reader of it to the instance's body;
/// a cut that does not place its body refuses the reader as a severed
/// edge; and inline of an instance a boolean reads succeeds, the
/// boolean reading the inlined body.
#[test]
fn split_and_inline_narrow_the_closure_to_the_cuts_world() {
    let (doc, [frame, profile, b, p, reader]) = split_scene("intent-c-split");
    let out = editor_core::split(
        &doc,
        &BTreeSet::from([frame, profile, b, p]),
        DocumentId::derive("intent-c-split-part"),
        Tol::witness(),
        None,
    )
    .expect("a cut that places its body splits");
    let Some(Node::Transform { input, .. }) = out.remainder.node(reader) else {
        panic!("the reader stays in the remainder")
    };
    assert_eq!(
        out.remainder.operation_of(*input),
        Some(out.instance),
        "the remainder's reader reads the instance's body"
    );
    assert_eq!(
        out.part.placements(),
        vec![out.node_map[&p]],
        "the cut's placement is the part's world"
    );
    assert_eq!(
        out.remainder.placements().len(),
        1,
        "the remainder places one copy of the instance"
    );

    let refused = editor_core::split(
        &doc,
        &BTreeSet::from([frame, profile, b]),
        DocumentId::derive("intent-c-split-unplaced"),
        Tol::witness(),
        None,
    )
    .expect_err("a cut that does not place its body severs its readers");
    assert!(
        matches!(
            refused,
            SplitError::SeveredEdge {
                consumer_is_cut: false,
                ..
            }
        ),
        "{refused:?}"
    );

    // Inline of an instance a boolean reads.
    let part = {
        let doc = ProfileDoc::empty(DocumentId::derive("intent-c-inline-part"), Tol::witness());
        let (doc, body) = block(doc, 0.0);
        place(doc, body).0
    };
    let mut store = PartStore::default();
    let part_ref = store.insert(part, Tol::witness());
    let host = ProfileDoc::empty(DocumentId::derive("intent-c-inline"), Tol::witness());
    let (host, instance) = insert(host, Node::instantiate_part(part_ref));
    let (host, other) = block(host, 0.75);
    let (host, fused) = insert(
        host,
        Node::Boolean {
            op: BooleanOp::Union,
            a: instance.into(),
            b: other.into(),
            declare: Vec::new(),
        },
    );
    let (host, _) = place(host, fused);
    let resolver: std::sync::Arc<dyn editor_core::PartResolver> = std::sync::Arc::new(store);
    let inlined = match editor_core::inline(&host, instance, &resolver, Tol::witness()) {
        Ok(inlined) => inlined,
        Err(InlineError::Edit { error }) => panic!("inline refused an edit: {error}"),
        Err(other) => panic!("inline of a read instance succeeds: {other}"),
    };
    let Some(Node::Boolean { a, .. }) = inlined.doc.node(fused) else {
        panic!("the boolean stays")
    };
    let read = inlined
        .doc
        .operation_of(*a)
        .expect("the boolean's read is live");
    assert!(
        matches!(inlined.doc.node(read), Some(Node::Extrude { .. })),
        "the boolean reads the inlined body"
    );
}
