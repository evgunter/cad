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
    AssertionRelation, BooleanOp, CapEnd, ChecksConfig, DocEdit, DocumentId, EntityKind,
    ExtrudeSide, InlineError, Maintenance, MeasurePrimitive, Node, NodeResult, ProductError,
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
/// of words fed. Taken at the base, before any placement existed: main
/// with stage 2 unit B merged (`0811289515`), by [`print_product_rows`]'s
/// gather over the root list there. The composed die's two rows moved
/// from B's branch record with main's blend change (`d5a518b1b2`).
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
    ("die_composed", "182b02846757307e/88"),
    ("die_composed_tour", "ca0b8371cb38e973/328"),
    ("plate_param", "fd7b71fa51270532/88"),
    ("kiss_carry", "fabe53fd5101f503/88"),
    ("tube_ring", "c342379c65eab6d1/10"),
    ("tube_arc", "19b8e0af8227889b/16"),
    ("hollow_tube_elbow", "9ae3e25a75d4fecd/28"),
    ("hollow_tube_ring", "1db62df6ebe2507d/16"),
    ("reshaped_rod", "128f9facfb4a7626/52"),
    (FILES[0], "8853ddff30bcec28/154"),
    (FILES[1], "1ecb739a2d22c24d/82"),
    (FILES[2], "ca0b8371cb38e973/328"),
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
    let (doc, measure) = crate::fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![
            SitedRef::new(b, cap(b, CapEnd::Start)),
            SitedRef::new(b, cap(b, CapEnd::End)),
        ],
    );
    let (doc, _) = insert(
        doc.clone(),
        Node::Assertion {
            value: crate::fixture::value_of(&doc, measure),
            bound: len(0.5),
            relation: AssertionRelation::AtLeast,
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
    let (doc, measure) = crate::fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![
            SitedRef::new(b, cap(b, CapEnd::Start)),
            SitedRef::new(b, vanished),
        ],
    );
    let (doc, assertion) = insert(
        doc.clone(),
        Node::Assertion {
            value: crate::fixture::value_of(&doc, measure),
            bound: len(0.5),
            relation: AssertionRelation::AtLeast,
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

/// **Construction never reads the world** (D10). A block placed, and a
/// boolean that would read the placement's copy: the edit door refuses
/// it at insert and at `SetParam`, naming the placement, and a file
/// that holds such a read refuses at load. Only the gather and export
/// read a placement's pose.
///
/// Red if a `Body` slot admits the copy: the boolean then evaluates,
/// and its geometry moves with the pose.
#[test]
fn no_slot_reads_a_world_copy_at_the_door_or_at_load() {
    let doc = ProfileDoc::empty_derived("intent-c-copy-read", Tol::witness());
    let (doc, a) = block(doc, 0.0);
    let (doc, b) = block(doc, 0.75);
    let (doc, placement) = place(doc, a);
    let copy = doc
        .output(placement, 0)
        .expect("the placement defines its copy");
    let boolean = |a: editor_core::Operand| Node::Boolean {
        op: BooleanOp::Union,
        a,
        b: b.into(),
        declare: Vec::new(),
    };
    let refused_naming_placement =
        |result: Result<_, editor_core::EditError>, at: &str| match result {
            Err(editor_core::EditError::ReadsWorldCopy {
                placement: named, ..
            }) => assert_eq!(
                named.id(),
                placement,
                "{at}: the refusal names the placement"
            ),
            Err(other) => panic!("{at}: refused otherwise: {other}"),
            Ok(_) => panic!("{at}: a slot read the world copy"),
        };
    refused_naming_placement(
        doc.apply(
            &DocEdit::InsertNode {
                node: Box::new(boolean(copy.into())),
                fresh: Vec::new(),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        ),
        "insert",
    );

    let (doc, fused) = insert(doc, boolean(a.into()));
    refused_naming_placement(
        doc.apply(
            &DocEdit::SetParam {
                node: fused,
                slot: editor_core::SlotId::Operand(editor_core::OperandSlot::A),
                value: editor_core::SlotValue::Read(copy.into()),
                fresh: Vec::new(),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        ),
        "SetParam",
    );

    // The same read written into a file: the boolean's `a` swapped for
    // the copy, on the wire.
    let text = editor_core::persist::save(&doc, &[], Tol::witness()).expect("the document saves");
    let wire = |var| serde_json::to_string(&var).expect("an id serializes");
    let body_a = doc.output(a, 0).expect("the block's body");
    let read = format!("\"a\": {}", wire(body_a));
    assert_eq!(
        text.matches(&read).count(),
        1,
        "the boolean's one read of the block"
    );
    let forged = text.replace(&read, &format!("\"a\": {}", wire(copy)));
    match editor_core::persist::load(&forged, Tol::witness()) {
        Err(error) => {
            let said = error.to_string();
            assert!(
                said.contains("reads the world copy"),
                "the load names the read: {said}"
            );
        }
        Ok(_) => panic!("a file whose boolean reads a world copy loads"),
    }
}

/// **A posed placement moves its copy, rigidly, records and all.**
/// `kiss_carry`'s union carries one vertex-vertex contact record at the
/// kiss `(1, 1, 1)`. Placed at a pose (a quarter turn about +z, then a
/// translation), the gathered body is the body a `Transform` of the
/// union by the same step places at the identity, bit for bit, and not
/// the unposed one; and the copy's record resolves in the gathered
/// body to two vertices standing at the kiss, moved by the pose.
///
/// Red if the copy ignores its pose (the digest equals the unposed
/// one, and the kiss stays at `(1, 1, 1)`), or if the placement drops
/// or mis-keys the records it carries (no record, or a key the
/// gathered body does not hold).
#[test]
fn a_posed_placement_moves_its_copy_and_its_records_rigidly() {
    use crate::fixture::{ang, scl, xform};
    let corpus::CorpusDoc { doc, result, .. } = corpus::documents()
        .into_iter()
        .find(|d| d.name == "kiss_carry")
        .expect("kiss_carry is registered");
    let union = result.expect("kiss_carry names its union");
    let doc = doc.placements().into_iter().fold(doc, |doc, p| {
        crate::fixture::step(doc, DocEdit::DeleteNode { id: p }).0
    });
    let (t, axis, angle) = (
        [5.0, -3.0, 2.0],
        [0.0, 0.0, 1.0],
        std::f64::consts::FRAC_PI_2,
    );
    let pose: editor_core::Placement<editor_core::Formula> = editor_core::Step::Rigid {
        translation: t.map(len),
        axis: axis.map(scl),
        angle: ang(angle),
    }
    .into();

    let digest = |doc: &ProfileDoc| {
        body_digest(&product(doc, &ev(doc), Tol::witness()).expect("the world gathers"))
    };
    let (unposed, _) = place(doc.clone(), union);
    let (posed, _) = crate::fixture::step(doc.clone(), DocEdit::place(union, Some(pose)));
    let (moved, by) = insert(doc, xform(union, t, axis, angle));
    let (moved, _) = place(moved, by);
    assert_eq!(
        digest(&posed),
        digest(&moved),
        "the posed copy is the transform's body at the identity"
    );
    assert_ne!(digest(&posed), digest(&unposed), "and the pose moved it");

    let run = ev(&posed);
    let gathered = editor_core::product_recorded(&posed, &run, Tol::witness()).expect("gathers");
    let [kiss] = gathered.contacts.vv[..] else {
        panic!(
            "the copy carries the union's one v-v record: {:?}",
            gathered.contacts
        );
    };
    let at = |key| {
        gathered
            .body
            .vertex_points()
            .find(|&(k, _)| k == key)
            .map(|(_, p)| [p.x, p.y, p.z])
            .unwrap_or_else(|| panic!("the record's key {key:?} is a vertex of the product"))
    };
    // A quarter turn about +z takes (1, 1, 1) to (−1, 1, 1).
    let want = [-1.0 + t[0], 1.0 + t[1], 1.0 + t[2]];
    for got in [at(kiss.a), at(kiss.b)] {
        assert!(
            got.iter().zip(want).all(|(g, w)| (g - w).abs() < 1e-9),
            "the kiss moved with the pose: {got:?} vs {want:?}"
        );
    }
}

/// **Inline leaves no heir for a posed part placement.** The part
/// places its one block at a pose of its own, and a host boolean reads
/// the instance: what stood where the instance's body did is the
/// placement's world copy, which no slot reads, so inline refuses
/// naming the part's placement rather than re-pointing the boolean at
/// the copy.
///
/// Red if inline re-points the reader at the copy (the replay's
/// `SetParam` refuses `ReadsWorldCopy` as a forwarded edit instead) or
/// at the unposed block (the boolean's geometry moves).
#[test]
fn inline_refuses_a_reader_of_an_instance_whose_part_places_at_a_pose() {
    let pose: editor_core::Placement<editor_core::Formula> = editor_core::Step::Rigid {
        translation: [0.0, 0.0, 2.0].map(len),
        axis: [0.0, 0.0, 1.0].map(crate::fixture::scl),
        angle: crate::fixture::ang(0.0),
    }
    .into();
    let (part, placement) = {
        let doc = ProfileDoc::empty(DocumentId::derive("intent-c-posed-part"), Tol::witness());
        let (doc, body) = block(doc, 0.0);
        crate::fixture::step(doc, DocEdit::place(body, Some(pose)))
    };
    let placement = placement.expect("the placement's node");
    let mut store = PartStore::default();
    let part_ref = store.insert(part, Tol::witness());
    let host = ProfileDoc::empty(DocumentId::derive("intent-c-posed-inline"), Tol::witness());
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
    match editor_core::inline(&host, instance, &resolver, Tol::witness()) {
        Err(InlineError::InstanceReadUncarried {
            reader,
            why: editor_core::Uncarried::Posed { placement: named },
        }) => {
            assert_eq!(reader.id(), fused, "the refusal names the reader");
            assert_eq!(named.id(), placement, "and the part's posed placement");
        }
        other => panic!("inline refuses the posed heir: {other:?}"),
    }
}

/// **An empty world names a pattern among the unplaced.** A block and a
/// pattern of it, nothing placed: `EmptyProduct` lists both outputs,
/// the pattern's `Bodies` included, and the recourse says how a list of
/// bodies is placed.
///
/// Red if `Doc::unplaced` lists `Body` outputs only (the pattern is
/// missing), or the recourse does not name the `Part` pick.
#[test]
fn an_empty_world_names_a_pattern_and_how_to_place_one_of_its_bodies() {
    let doc = ProfileDoc::empty_derived("intent-c-unplaced-pattern", Tol::witness());
    let (doc, a) = block(doc, 0.0);
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: a.into(),
            count: editor_core::Formula::count(2),
            kind: editor_core::PatternKind::Linear {
                direction: [1.0, 0.0, 0.0].map(crate::fixture::scl),
                spacing: len(2.0),
            },
        },
    );
    let want = vec![doc.output(a, 0).unwrap(), doc.output(pattern, 0).unwrap()];
    assert_eq!(
        doc.unplaced(),
        want,
        "the block's body and the pattern's bodies"
    );
    let refused = product(&doc, &ev(&doc), Tol::witness()).expect_err("nothing is placed");
    assert!(
        matches!(&refused, ProductError::EmptyProduct { unplaced } if *unplaced == want),
        "{refused:?}"
    );
    let said = refused.to_string();
    assert!(said.contains("through a Part pick"), "{said}");
}

/// A split scene whose cut is `{frame, profile, b, placement}`: `b`
/// placed (at `pose` when given), and a kept `Transform` of `b`, itself
/// placed, reading `b` across the seam. With `second`, the cut also
/// holds a second block `c` placed at the identity. Answers the
/// document, the cut and the reader.
fn cut_with_a_reader(
    label: &str,
    pose: Option<editor_core::Placement<editor_core::Formula>>,
    second: bool,
) -> (
    ProfileDoc,
    BTreeSet<RecipeNodeId>,
    RecipeNodeId,
    RecipeNodeId,
) {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let before = doc.ids().len();
    let (doc, b) = block(doc, 0.0);
    let (doc, p) = crate::fixture::step(doc, DocEdit::place(b, pose));
    let p = p.expect("the placement's node");
    let mut cut: BTreeSet<RecipeNodeId> = doc.ids()[before..].iter().copied().collect();
    let doc = if second {
        let mark = doc.ids().len();
        let (doc, c) = block(doc, 3.0);
        let (doc, _) = place(doc, c);
        cut.extend(doc.ids()[mark..].iter().copied());
        doc
    } else {
        doc
    };
    let (doc, reader) = insert(
        doc,
        crate::fixture::xform(b, [10.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0),
    );
    let (doc, _) = place(doc, reader);
    (doc, cut, reader, p)
}

/// **Split shares inline's re-point rule** ([`editor_core::Uncarried`]).
/// A kept reader of a body the cut places is re-pointed to the
/// instance's body only when the cut's world is one placement at the
/// identity; the instance's body is the part's whole world, so a cut
/// placing its body at a pose, or placing two bodies, refuses naming
/// the reader rather than moving or widening what it reads.
///
/// Red if split re-points the reader anyway: the reader's geometry then
/// moves with the pose, or takes in the second block (the review
/// probes' digests).
#[test]
fn split_refuses_a_remainder_reader_whose_cut_world_is_posed_or_several() {
    let pose: editor_core::Placement<editor_core::Formula> = editor_core::Step::Rigid {
        translation: [0.0, 0.0, 2.0].map(len),
        axis: [0.0, 0.0, 1.0].map(crate::fixture::scl),
        angle: crate::fixture::ang(0.0),
    }
    .into();
    let (doc, cut, reader, p) = cut_with_a_reader("intent-c-split-posed", Some(pose), false);
    match editor_core::split(
        &doc,
        &cut,
        DocumentId::derive("intent-c-split-posed-part"),
        Tol::witness(),
        None,
    ) {
        Err(SplitError::RemainderReadUncarried {
            reader: named,
            why: editor_core::Uncarried::Posed { placement },
        }) => {
            assert_eq!(named.id(), reader, "the refusal names the reader");
            assert_eq!(placement.id(), p, "and the posed cut placement");
        }
        other => panic!("a posed cut world refuses its remainder reader: {other:?}"),
    }

    let (doc, cut, reader, _) = cut_with_a_reader("intent-c-split-two", None, true);
    match editor_core::split(
        &doc,
        &cut,
        DocumentId::derive("intent-c-split-two-part"),
        Tol::witness(),
        None,
    ) {
        Err(SplitError::RemainderReadUncarried {
            reader: named,
            why: editor_core::Uncarried::Bodies { count: 2 },
        }) => assert_eq!(named.id(), reader, "the refusal names the reader"),
        other => panic!("a cut placing two bodies refuses its remainder reader: {other:?}"),
    }
}

/// **Inline keeps every copy the host places.** The host places one
/// instance twice at the identity, over a part that places its block
/// once: the inlined host still places two copies, and its product is
/// the one it had. A part that places two bodies cannot stand twice
/// where one instance body did, so the second host placement refuses as
/// a reader the part's world does not carry.
///
/// Red if inline deletes every identity host placement and splices the
/// part's world once (one copy, the product moves), or drops the second
/// placement silently.
#[test]
fn inline_keeps_both_copies_of_an_instance_placed_twice() {
    let host_of = |part: ProfileDoc, label: &str| {
        let mut store = PartStore::default();
        let part_ref = store.insert(part, Tol::witness());
        let host = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
        let (host, instance) = insert(host, Node::instantiate_part(part_ref));
        let (host, first) = place(host, instance);
        let (host, second) = place(host, instance);
        (host, instance, [first, second], store)
    };
    let part = {
        let doc = ProfileDoc::empty(DocumentId::derive("intent-c-twice-part"), Tol::witness());
        let (doc, body) = block(doc, 0.0);
        place(doc, body).0
    };
    let (host, instance, _, store) = host_of(part, "intent-c-twice");
    let opts = crate::fixture::resolver::with_resolver(store.clone());
    let before = product(&host, &crate::fixture::run(&host, &opts), Tol::witness())
        .expect("the host gathers");
    let resolver: std::sync::Arc<dyn editor_core::PartResolver> = std::sync::Arc::new(store);
    let inlined = match editor_core::inline(&host, instance, &resolver, Tol::witness()) {
        Ok(inlined) => inlined,
        Err(other) => panic!("inline of an instance placed twice succeeds: {other}"),
    };
    assert_eq!(inlined.doc.placements().len(), 2, "both copies stay placed");
    let after = product(&inlined.doc, &ev(&inlined.doc), Tol::witness()).expect("gathers");
    assert_eq!(
        body_digest(&after),
        body_digest(&before),
        "the product is unchanged"
    );

    let two = {
        let doc = ProfileDoc::empty(
            DocumentId::derive("intent-c-twice-two-part"),
            Tol::witness(),
        );
        let (doc, a) = block(doc, 0.0);
        let (doc, b) = block(doc, 3.0);
        let (doc, _) = place(doc, a);
        place(doc, b).0
    };
    let (host, instance, [_, second], store) = host_of(two, "intent-c-twice-two");
    let resolver: std::sync::Arc<dyn editor_core::PartResolver> = std::sync::Arc::new(store);
    match editor_core::inline(&host, instance, &resolver, Tol::witness()) {
        Err(InlineError::InstanceReadUncarried {
            reader,
            why: editor_core::Uncarried::Bodies { count: 2 },
        }) => assert_eq!(reader.id(), second, "the second copy is the reader refused"),
        other => panic!("a two-body part placed twice refuses: {other:?}"),
    }
}

/// **No measure is sited at a world placement.** A distance between two
/// copies' end caps, read at the placements, would move with the pose,
/// which only the gather and export read: the edit door refuses it
/// naming the placement, and a file holding such a site refuses at
/// load. Measuring between placed copies is stage 3's to design.
///
/// Red if the site is admitted: the review's probe measured `0.0` with
/// the copy at `dz = 0` and `2.0` at `dz = 2`.
#[test]
fn no_measure_is_sited_at_a_world_placement_at_the_door_or_at_load() {
    let doc = ProfileDoc::empty_derived("intent-c-measure-copy", Tol::witness());
    let (doc, a) = block(doc, 0.0);
    let (doc, b) = block(doc, 3.0);
    let (doc, p) = place(doc, a);
    let (doc, q) = place(doc, b);
    let measure = |sites: [(RecipeNodeId, StableName); 2]| {
        let [a, b] = sites.map(|(at, name)| SitedRef::new(at, name).into());
        Node::Measure {
            primitive: MeasurePrimitive::Distance { a, b },
        }
    };
    let at_copies = measure([
        (p, cap(a, CapEnd::End).in_copy(p)),
        (q, cap(b, CapEnd::End).in_copy(q)),
    ]);
    match doc.apply(
        &DocEdit::InsertNode {
            node: Box::new(at_copies),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(editor_core::EditError::ReadsWorldCopy { placement, .. }) => {
            assert_eq!(placement.id(), p, "the refusal names the first placement");
        }
        Err(other) => panic!("refused otherwise: {other}"),
        Ok(_) => panic!("a measure sited at a placement was admitted"),
    }

    // The same site on the wire: a measure at the bodies, its first
    // site moved onto the placement.
    let (doc, _) = insert(
        doc,
        measure([(a, cap(a, CapEnd::End)), (b, cap(b, CapEnd::End))]),
    );
    let text = editor_core::persist::save(&doc, &[], Tol::witness()).expect("the document saves");
    let wire = |id| serde_json::to_string(&id).expect("an id serializes");
    let body_of = |node| doc.output(node, 0).expect("a body");
    let select_at = text
        .find("\"Select\"")
        .expect("the measure's selection is on the wire");
    let site = format!("\"body\": {}", wire(body_of(a)));
    let offset = select_at + text[select_at..].find(&site).expect("its body read");
    let forged = format!(
        "{}\"body\": {}{}",
        &text[..offset],
        wire(body_of(p)),
        &text[offset + site.len()..]
    );
    match editor_core::persist::load(&forged, Tol::witness()) {
        Err(error) => {
            let said = error.to_string();
            assert!(
                said.contains("which only the product and export read"),
                "the load names the site: {said}"
            );
        }
        Ok(_) => panic!("a file whose measure is sited at a placement loads"),
    }
}
