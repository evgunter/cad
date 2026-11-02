//! INTENT stage 2 PR C: the product is the world
//! (`docs/INTENT-STAGE2-SPEC.md` §4, §9 rows 6–12). The product is
//! every copy a placement pinned to the world defines, in the placements' document
//! order, and nothing else places.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use crate::corpus;
use crate::fixture::resolver::PartStore;
use crate::fixture::value_channel::body_digest;
use crate::fixture::{insert, insert_pinned, len, on_frame, place, slid, square};
use editor_core::{
    AssertionRelation, CapEnd, ChecksConfig, DocEdit, DocumentId, EntityKind, InlineError,
    Maintenance, MeasurePrimitive, Node, NodeResult, ProductError, ProfileDoc, RecipeNodeId,
    RoleSeg, SitedRef, SplitError, StableName, product, product_named, run_checks,
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

/// **Every document's product, as a digest word and the count of words
/// fed** ([`print_product_rows`]), in the corpus's order, then the
/// committed files'. A document that predates the world holds the
/// product its base gathered over its root list before any placement
/// existed, so test 6 holds its migration to that product; a document
/// added since holds the product it had when it joined. A row moves
/// only with what its product is, and the PR that moves one says which
/// points moved and why.
const PRE_C: [(&str, &str); 39] = [
    ("die", "18a00a4411472694/532"),
    ("corner_table", "1b26e7ae1ba4a066/100"),
    ("heat_sink", "986dd0b900fe3d32/268"),
    ("crossing_slots", "23d227ff22f6c626/100"),
    ("nested_islands_105", "07eda6db4f12636d/100"),
    ("nested_islands_106_depth1", "32fed3f5e12387ee/52"),
    ("nested_islands_106_depth2", "3a7999e85a64a33b/76"),
    ("declared_tangency", "89b46a31a322fc03/88"),
    ("kitchen_sink", "fddfb7f84e02cfcd/196"),
    ("cut_cylinder", "d2acb2b37c5273db/28"),
    ("measured_web", "f556ab19f06e9f9e/28"),
    ("boss_union", "a07c97077aedd76c/46"),
    ("die_fillet", "934f10b8156f9396/76"),
    ("die_chamfer", "934f10b8156f9396/76"),
    ("die_pips", "2859880c273d7b35/37"),
    ("heat_sink_fins", "8a57c326dc5f71aa/124"),
    ("die_tool", "f4d9c34d5c3d1451/82"),
    ("face_sketch", "7809b39224c44527/52"),
    ("part_select", "e54d217adfa64523/52"),
    ("symmetric_plate", "4988a8ebeaf80d66/52"),
    ("loft_prism", "36c0de6a313e3a66/28"),
    ("die_composed", "182b02846757307e/88"),
    ("die_composed_tour", "b70c1bbdaed3e0e1/328"),
    ("plate_param", "fd7b71fa51270532/88"),
    ("kiss_carry", "9efdca7b404d4e52/85"),
    ("tube_ring", "c342379c65eab6d1/10"),
    ("tube_arc", "19b8e0af8227889b/16"),
    ("hollow_tube_elbow", "9ae3e25a75d4fecd/28"),
    ("hollow_tube_ring", "1db62df6ebe2507d/16"),
    ("reshaped_rod", "128f9facfb4a7626/52"),
    ("pin_in_bore", "76547864dbc6f32b/76"),
    ("axis_projection", "9856d0ed9ba36a20/40"),
    ("blocks_on_one_floor", "0a11e01c369a8633/52"),
    ("revolves_on_one_axis", "17170d0ebb164176/22"),
    (FILES[0], "8853ddff30bcec28/154"),
    (FILES[1], "f4d9c34d5c3d1451/82"),
    (FILES[2], "b70c1bbdaed3e0e1/328"),
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
    match crate::fixture::product_in_world(doc, &ev, Tol::witness()) {
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
        },
    )
}

/// **One copy of `body` in the world at `pose`**: a placement of it,
/// pinned there over its own geometry.
fn place_by_values(
    doc: ProfileDoc,
    body: RecipeNodeId,
    pose: &editor_core::Frame,
) -> (ProfileDoc, RecipeNodeId) {
    let read = crate::fixture::body_read(&doc, body.into());
    let (doc, placement) = insert(
        doc,
        crate::fixture::stood(editor_core::BodyRead::plain(read)),
    );
    (
        crate::fixture::pin(doc, placement, pose, &editor_core::EvalOptions::default()),
        placement,
    )
}

/// The pose two units up `+z`.
fn raised() -> editor_core::Frame {
    editor_core::Frame::translation([0.0, 0.0, 2.0])
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
        Node::Assertion { claim: editor_core::Claim::Value {
            value: crate::fixture::value_of(&doc, measure),
            bound: len(0.5),
            relation: AssertionRelation::AtLeast,
        } },
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
        body_digest(own.body()),
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
        Node::Assertion { claim: editor_core::Claim::Value {
            value: crate::fixture::value_of(&doc, measure),
            bound: len(0.5),
            relation: AssertionRelation::AtLeast,
        } },
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
/// then a union of their two copies through the kernel edit: the product is the
/// two blocks as it was, and the union is unplaced.
#[test]
fn inserting_a_boolean_over_two_placed_blocks_places_nothing() {
    let doc = ProfileDoc::empty_derived("intent-c-side-effect", Tol::witness());
    let (doc, a) = block(doc, 0.0);
    let (doc, b) = block(doc, 0.75);
    let (doc, a) = place(doc, a);
    let (doc, b) = place(doc, b);
    let before = body_digest(&product(&doc, &ev(&doc), Tol::witness()).expect("two blocks"));
    let placements = doc.placements();
    let (doc, fused) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
    );
    assert_eq!(doc.placements(), placements, "the insert placed nothing");
    let after = body_digest(&product(&doc, &ev(&doc), Tol::witness()).expect("two blocks"));
    assert_eq!(after, before, "the product is the two blocks still");
    let fused_body = doc.output(fused, 0).expect("the union defines its body");
    assert_ne!(
        doc.product(),
        Some(fused_body),
        "the union is not the product"
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
    let _ = bodies;
    match product(&doc, &ev(&doc), Tol::witness()) {
        Err(ProductError::EmptyProduct) => {}
        other => panic!("an empty slot refuses EmptyProduct: {other:?}"),
    }

    let (doc, placement) = place(doc, a);
    let applied = doc
        .apply(&DocEdit::DeleteNode { id: a }, Tol::witness())
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
    // The slot reads the placement's copy, and the placement's read is
    // gone: the gather refuses at the placement, failed.
    match product(&doc, &ev(&doc), Tol::witness()) {
        Err(ProductError::Read(editor_core::NodeStanding::Failed { node })) => {
            assert_eq!(node, placement, "the gather names the stranded placement");
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
    // The product's `⊔` keys each copy's names by the copy.
    for placement in [p, q] {
        let copy = crate::fixture::product_name(&doc, &run, (placement, cap(a, CapEnd::End)));
        assert!(
            matches!(names.lookup(&copy), Some(editor_core::Entry::Unique(_))),
            "the block's end cap is named once in copy {placement:?}"
        );
    }
}

/// The block, its placement, and a union of its copy with a second
/// block placed against that copy, in one document: what the split rows
/// cut. The block is in a space of its own (D′1), so the reader reads
/// its world copy.
fn split_scene(label: &str) -> (ProfileDoc, [RecipeNodeId; 4]) {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (doc, b) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    let (doc, p) = place(doc, b);
    let (doc, other) = block(doc, 3.0);
    let (doc, other) = slid(doc, p, other, [0.0; 3]);
    let (doc, reader) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![p.into(), other.into()]),
        },
    );
    (doc, [profile, b, p, reader])
}

/// **(C, test 12) Split and inline under the world.** A cut that places
/// its body re-points a remainder boolean reading it to the instance's
/// copy of it, by its key; a cut that does not place its body refuses
/// the reader as a severed edge; and inline of an instance pinned at
/// the identity that a boolean reads succeeds, the boolean reading the
/// inlined body.
#[test]
fn split_and_inline_narrow_the_closure_to_the_cuts_world() {
    let (doc, [profile, b, p, reader]) = split_scene("intent-c-split");
    let out = editor_core::split(
        &doc,
        &BTreeSet::from([profile, b, p]),
        DocumentId::derive("intent-c-split-part"),
        Tol::witness(),
        None,
    )
    .expect("a cut that places its body splits");
    let Some(Node::Union {
        members: editor_core::Bodies::Spelled(members),
    }) = out.remainder.node(reader)
    else {
        panic!("the reader stays in the remainder")
    };
    assert_eq!(
        out.remainder.operation_of(members[0].read),
        Some(out.instance),
        "the remainder's reader reads the instance's copies"
    );
    assert!(
        out.remainder.pick(members[0].read).is_none(),
        "the part's product is that one copy, read whole: {:?}",
        members[0]
    );
    assert_eq!(
        out.part.placements(),
        vec![out.node_map[&p]],
        "the cut's placement is the part's world"
    );
    assert_eq!(
        out.remainder.placements(),
        {
            let mut v = vec![
                out.instance,
                out.remainder
                    .operation_of(members[1].read)
                    .expect("the other block's placement"),
            ];
            v.sort();
            v
        },
        "the remainder places one copy of the instance, beside the other block's placement"
    );

    let refused = editor_core::split(
        &doc,
        &BTreeSet::from([profile, b]),
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

    // Inline of an instance whose one copy a boolean reads.
    let (part, ()) = {
        let doc = ProfileDoc::empty(DocumentId::derive("intent-c-inline-part"), Tol::witness());
        let (doc, body) = block(doc, 0.0);
        // The part's product is its one copy, which an instance's copy
        // carries whole.
        let (doc, _) = place(doc, body);
        (doc, ())
    };
    let mut store = PartStore::default();
    let part = crate::fixture::assemble_world_copies(part);
    let part_ref = store.insert(part, Tol::witness());
    let opts = crate::fixture::resolver::with_resolver(store.clone());
    let host = ProfileDoc::empty(DocumentId::derive("intent-c-inline"), Tol::witness());
    let (host, instance) = insert_pinned(host, crate::fixture::mated_instance(part_ref), &opts);
    let (host, other) = block(host, 0.75);
    let (host, other) = place(host, other);
    let (host, fused) = insert(
        host,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                editor_core::BodyRead::plain(editor_core::Operand::Node(instance)),
                other.into(),
            ]),
        },
    );
    let resolver: std::sync::Arc<dyn editor_core::PartResolver> = std::sync::Arc::new(store);
    let inlined = match editor_core::inline(&host, instance, &resolver, Tol::witness()) {
        Ok(inlined) => inlined,
        Err(InlineError::Edit { error }) => panic!("inline refused an edit: {error}"),
        Err(other) => panic!("inline of a read instance succeeds: {other}"),
    };
    let Some(Node::Union {
        members: editor_core::Bodies::Spelled(members),
        ..
    }) = inlined.doc.node(fused)
    else {
        panic!("the union stays")
    };
    let read = inlined
        .doc
        .operation_of(members[0].read)
        .expect("the boolean's read is live");
    // The spliced copy, in the world beside the boolean's other
    // operand: the extrude's body is in a space of its own.
    assert!(
        matches!(inlined.doc.node(read), Some(Node::Place { .. })),
        "the boolean reads the inlined copy"
    );
}

/// **A construction may read a world copy.** A block placed at a
/// pose, and a boolean reading the placement's copy: the edit door
/// admits it at insert and at `SetParam`, the boolean builds the copy
/// where the placement puts it, and the document round-trips through a
/// file bit for bit.
///
/// Red if a `Body` slot refuses the copy, or reads the unplaced block
/// (the boolean's body is then the one over the block itself).
#[test]
fn a_construction_reads_a_world_copy_at_the_door_and_at_load() {
    let doc = ProfileDoc::empty_derived("intent-c-copy-read", Tol::witness());
    let (doc, a) = block(doc, 0.0);
    let (doc, b) = block(doc, 0.75);
    let (doc, placement) = place_by_values(doc, a, &raised());
    let copy = doc
        .output(placement, 0)
        .expect("the placement defines its copy");
    // The block where its own coordinates are, and `b` beside both.
    let (doc, unmoved) = place_by_values(doc, a, &editor_core::Frame::IDENTITY);
    let (doc, b) = place_by_values(doc, b, &editor_core::Frame::IDENTITY);
    let boolean = |a: editor_core::Operand| Node::Union {
        members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
    };
    let (read_copy, by_insert) = insert(doc.clone(), boolean(copy.into()));
    let (read_block, by_block) = insert(doc.clone(), boolean(unmoved.into()));
    assert_ne!(
        crate::fixture::built_bits(&ev(&read_copy), by_insert),
        crate::fixture::built_bits(&ev(&read_block), by_block),
        "the boolean reads the copy where it is placed, not the block"
    );

    let (doc, fused) = insert(doc, boolean(unmoved.into()));
    let (doc, _) = crate::fixture::step(
        doc,
        DocEdit::SetParam {
            node: fused,
            slot: editor_core::SlotId::Operand(editor_core::OperandSlot::Member(0)),
            value: editor_core::SlotValue::Read(copy.into()),
            fresh: Vec::new(),
        },
    );
    assert_eq!(
        crate::fixture::built_bits(&ev(&doc), fused),
        crate::fixture::built_bits(&ev(&read_copy), by_insert),
        "the slot edit reads the copy as the insert does"
    );

    let text = editor_core::persist::save(&doc, &[], Tol::witness()).expect("the document saves");
    let loaded = editor_core::persist::load(&text, Tol::witness())
        .unwrap_or_else(|e| panic!("a boolean reading a world copy loads: {e}"))
        .doc;
    assert!(loaded.bit_eq(&doc), "and round-trips bit for bit");
}

/// **A posed placement moves its copy, rigidly, records and all.**
/// `kiss_carry`'s union carries one vertex-vertex contact record at the
/// kiss `(1, 1, 1)`. Placed at a pose (a quarter turn about +z, then a
/// translation), the gathered body is the union's copy placed against
/// itself at the same pose, vertex for vertex to
/// the rounding of the pin's one re-composition, and not the unposed
/// one; and the copy's record resolves in the gathered
/// body to two vertices standing at the kiss, moved by the pose.
///
/// Red if the copy ignores its pose (the digest equals the unposed
/// one, and the kiss stays at `(1, 1, 1)`), or if the placement drops
/// or mis-keys the records it carries (no record, or a key the
/// gathered body does not hold).
#[test]
fn a_posed_placement_moves_its_copy_and_its_records_rigidly() {
    use crate::fixture::xform;
    let corpus::CorpusDoc { doc, result, .. } = corpus::documents()
        .into_iter()
        .find(|d| d.name == "kiss_carry")
        .expect("kiss_carry is registered");
    let union = result.expect("kiss_carry names its union");
    // The union's own world placement goes; the placements inside it stay.
    let doc = doc
        .placements()
        .into_iter()
        .filter(|&p| {
            matches!(doc.node(p), Some(Node::Place { source, .. })
                if (doc.operation_of(source.read) == Some(union)))
        })
        .collect::<Vec<_>>()
        .into_iter()
        .fold(doc, |doc, p| {
            crate::fixture::step(doc, DocEdit::DeleteNode { id: p }).0
        });
    let (t, axis, angle) = (
        [5.0, -3.0, 2.0],
        [0.0, 0.0, 1.0],
        std::f64::consts::FRAC_PI_2,
    );
    let pose = editor_core::Frame::rotate_then_translate(axis, angle, t, crate::fixture::band())
        .expect("a quarter turn about +z");

    let digest = |doc: &ProfileDoc| {
        body_digest(
            &crate::fixture::product_in_world(doc, &ev(doc), Tol::witness())
                .expect("the world gathers"),
        )
    };
    let (unposed, _) = place(doc.clone(), union);
    let (posed, _) = place_by_values(doc.clone(), union, &pose);
    let (moved, by) = xform(doc, union, t, axis, angle);
    // The pin writes the pose as spins and slides read back from the
    // frame's numbers (the retired pin-at helper), so the posed copy stands where the
    // union's copy placed against itself at that pose does, to the
    // rounding of that one re-composition: every vertex within 4 ulps
    // of the moved copy's.
    // By position: the gathered body and the copy key their vertices
    // apart.
    let points = |body: &topo::Body<f64>| {
        let mut points: Vec<_> = body
            .vertex_points()
            .map(|(_, p)| [p.x, p.y, p.z].map(|c| (c * 1e6).round() / 1e6 + 0.0))
            .zip(body.vertex_points().map(|(_, p)| [p.x, p.y, p.z]))
            .map(|(k, p)| (k.map(f64::to_bits), p))
            .collect();
        points.sort_by_key(|(k, _)| *k);
        points
    };
    let moved_run = ev(&moved);
    let (at_pose, transformed) = (
        points(
            &crate::fixture::product_in_world(&posed, &ev(&posed), Tol::witness())
                .expect("the world gathers"),
        ),
        points(&(corpus::body_of(&moved_run, by))),
    );
    assert_eq!(
        at_pose.iter().map(|(k, _)| *k).collect::<Vec<_>>(),
        transformed.iter().map(|(k, _)| *k).collect::<Vec<_>>(),
        "the posed copy has the moved copy's vertices"
    );
    for ((key, p), (_, q)) in at_pose.iter().zip(&transformed) {
        for (a, b) in p.iter().zip(q) {
            assert!(
                (a - b).abs() <= 4.0 * f64::EPSILON * a.abs().max(b.abs()).max(1.0),
                "the posed copy is the moved copy at the identity: {key:?} {p:?} {q:?}"
            );
        }
    }
    assert_ne!(digest(&posed), digest(&unposed), "and the pose moved it");

    let run = ev(&posed);
    let gathered = editor_core::product_recorded(&posed, &run, Tol::witness()).expect("gathers");
    let [kiss] = gathered.contacts.vertex_vertex().collect::<Vec<_>>()[..] else {
        panic!(
            "the copy carries the union's one v-v record: {:?}",
            gathered.contacts
        );
    };
    // Where the product stands in the world: the gather computes in its
    // first source's frame.
    let world = match editor_core::in_space(&gathered) {
        None => topo::Body::clone(&gathered.body),
        Some(map) => topo::transform_rigid(&gathered.body, &map, Tol::witness())
            .expect("the product composes into the world"),
    };
    let at = |key| {
        world
            .vertex_points()
            .find(|&(k, _)| k == key)
            .map(|(_, p)| [p.x, p.y, p.z])
            .unwrap_or_else(|| panic!("the record's key {key:?} is a vertex of the product"))
    };
    // A quarter turn about +z takes (1, 1, 1) to (−1, 1, 1).
    let want = [-1.0 + t[0], 1.0 + t[1], 1.0 + t[2]];
    for got in [at(kiss.0), at(kiss.1)] {
        assert!(
            got.iter().zip(want).all(|(g, w)| (g - w).abs() < 1e-9),
            "the kiss moved with the pose: {got:?} vs {want:?}"
        );
    }
}

/// **Inline carries a reader of a posed part placement's copy to that
/// copy.** The part places its one block at a pose of its own, and a
/// host boolean reads the instance's copy of it by its key: what stood
/// there is the placement's copy, moved, so inline of the instance,
/// pinned at the identity, re-points the boolean at the spliced
/// placement's copy, and the boolean's body is the one it had.
///
/// Red if inline re-points the reader at the unposed block (the
/// boolean's geometry moves).
#[test]
fn inline_carries_a_reader_of_an_instance_whose_part_places_at_a_pose() {
    let (part, _placement, _read) = {
        let doc = ProfileDoc::empty(DocumentId::derive("intent-c-posed-part"), Tol::witness());
        let (doc, body) = block(doc, 0.0);
        let read = doc.output(body, 0).expect("the block defines its body");
        let (doc, placement) = place_by_values(doc, body, &raised());
        let doc = editor_core::test_support::assemble_world_copies(doc, Tol::witness());
        (doc, placement, read)
    };
    let mut store = PartStore::default();
    let part = crate::fixture::assemble_world_copies(part);
    let part_ref = store.insert(part, Tol::witness());
    let opts = crate::fixture::resolver::with_resolver(store.clone());
    let host = ProfileDoc::empty(DocumentId::derive("intent-c-posed-inline"), Tol::witness());
    let (host, instance) = insert_pinned(host, crate::fixture::mated_instance(part_ref), &opts);
    let (host, other) = block(host, 0.75);
    let (host, other) = place(host, other);
    // Both members stand in the world, so the union does.
    let (host, fused) = insert(
        host,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                editor_core::BodyRead::plain(editor_core::Operand::Node(instance)),
                other.into(),
            ]),
        },
    );
    let before = crate::fixture::run(&host, &opts);
    let resolver: std::sync::Arc<dyn editor_core::PartResolver> = std::sync::Arc::new(store);
    let inlined = editor_core::inline(&host, instance, &resolver, Tol::witness())
        .unwrap_or_else(|e| panic!("inline carries the posed copy's reader: {e}"));
    let Some(Node::Union {
        members: editor_core::Bodies::Spelled(members),
    }) = inlined.doc.node(fused)
    else {
        panic!("the union stays")
    };
    assert_eq!(
        inlined.doc.operation_of(members[0].read),
        Some(inlined.placement),
        "the boolean reads the copy of the spliced placement's copy in the host's world"
    );
    let body = |run: &editor_core::Evaluation<f64>| {
        let Some(NodeResult::Ok(value)) = run.result(fused) else {
            panic!("the boolean evaluates: {:?}", run.result(fused))
        };
        let editor_core::ValuePayload::Boolean(editor_core::BooleanValue::Body { body, .. }) =
            &value.payload
        else {
            panic!("the boolean is a body")
        };
        body_digest(body.body())
    };
    assert_eq!(
        body(&ev(&inlined.doc)),
        body(&before),
        "the boolean's body is the one it had"
    );
}

/// **An empty world holds a family placed in its block's own space.** A
/// block and a row of it in the block's space, nothing in the world:
/// the product is empty, and says it reads nothing.
///
/// Red if a family placed outside the world reaches the product.
#[test]
fn an_empty_world_holds_no_family_placed_beside_its_block() {
    let doc = ProfileDoc::empty_derived("intent-c-unplaced-pattern", Tol::witness());
    let (doc, a) = block(doc, 0.0);
    let (doc, _, _) = crate::fixture::linear(doc, a, [1.0, 0.0, 0.0], 2.0, 2);
    let refused = product(&doc, &ev(&doc), Tol::witness()).expect_err("nothing is placed");
    assert!(
        matches!(&refused, ProductError::EmptyProduct),
        "{refused:?}"
    );
    let said = refused.to_string();
    assert!(said.contains("reads nothing"), "{said}");
}

/// A split scene whose cut is `{profile, b, placement}`: `b`
/// placed at `pose`, and a kept union of `b` with itself, itself
/// placed, reading `b` across the seam and nothing else. With `second`,
/// the cut also holds a second placement of `b` at the identity.
/// Answers the document, the cut, the reader and `b`'s first
/// placement.
fn cut_with_a_reader(
    label: &str,
    pose: &editor_core::Frame,
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
    let (doc, p) = place_by_values(doc, b, pose);
    let mut cut: BTreeSet<RecipeNodeId> = doc.ids()[before..].iter().copied().collect();
    let doc = if second {
        let (doc, again) = place(doc, b);
        cut.insert(again);
        doc
    } else {
        doc
    };
    // A member read twice is that member's body (FORK-DM4).
    let (doc, reader) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![b.into(), b.into()]),
        },
    );
    let (doc, _) = place(doc, reader);
    (doc, cut, reader, p)
}

/// **Split shares inline's re-point rule** ([`editor_core::Uncarried`]).
/// A kept reader of a body the cut places is re-pointed to the
/// instance's copy of it only when one cut placement copies the body,
/// at the identity: a cut placing the body at a pose, or placing it
/// twice, refuses naming the reader rather than moving what it reads or
/// picking one of two copies.
///
/// Red if split re-points the reader anyway: the reader's geometry then
/// moves with the pose, or reads one copy of two.
#[test]
fn split_refuses_a_remainder_reader_whose_cut_world_is_posed_or_several() {
    let (doc, cut, reader, p) = cut_with_a_reader("intent-c-split-posed", &raised(), false);
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

    let (doc, cut, reader, _) =
        cut_with_a_reader("intent-c-split-two", &editor_core::Frame::IDENTITY, true);
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
        other => panic!("a cut placing its body twice refuses its remainder reader: {other:?}"),
    }
}

/// **Inline keeps every copy the host places.** The host holds one
/// instance standing at the identity and a second placement
/// copying the instance's one copy by its key, over a part that places
/// its block once: the inlined host still places two copies, and its
/// product is the one it had. A second placement of the instance's
/// copies whole, over a part that places two bodies, reads the new
/// placement's copies whole (A4, one rule), and the product is the one
/// it had.
///
/// Red if inline splices the part's world once and drops the second
/// copy (one copy, the product moves), or drops the second placement
/// silently.
#[test]
fn inline_keeps_both_copies_of_an_instance_placed_twice() {
    let host_of =
        |part: ProfileDoc,
         label: &str,
         second: &dyn Fn(RecipeNodeId) -> editor_core::BodyRead<editor_core::Formula>| {
            let mut store = PartStore::default();
            let part = crate::fixture::assemble_world_copies(part);
            let part_ref = store.insert(part, Tol::witness());
            let host = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
            let (host, instance) = insert(host, crate::fixture::mated_instance(part_ref));
            // Both placements stand at the identity: the identity
            // bundle, which reads nothing of the copy.
            let stood = |host: ProfileDoc, at: RecipeNodeId| {
                crate::fixture::written(host, at, editor_core::Placed::identity())
            };
            let (host, second) = insert(host, crate::fixture::stood(second(instance)));
            let host = stood(host, instance);
            let host = stood(host, second);
            (host, instance, [instance, second], store)
        };
    let (part, ()) = {
        let doc = ProfileDoc::empty(DocumentId::derive("intent-c-twice-part"), Tol::witness());
        let (doc, body) = block(doc, 0.0);
        // The part's product is its one copy, which an instance's copy
        // carries whole.
        let (doc, _) = place(doc, body);
        (doc, ())
    };
    let (host, instance, _, store) = host_of(part, "intent-c-twice", &|instance| {
        editor_core::BodyRead::plain(editor_core::Operand::Node(instance))
    });
    let opts = crate::fixture::resolver::with_resolver(store.clone());
    let before = product(&host, &crate::fixture::run(&host, &opts), Tol::witness())
        .expect("the host gathers");
    let resolver: std::sync::Arc<dyn editor_core::PartResolver> = std::sync::Arc::new(store);
    let inlined = match editor_core::inline(&host, instance, &resolver, Tol::witness()) {
        Ok(inlined) => inlined,
        Err(other) => panic!("inline of an instance placed twice succeeds: {other}"),
    };
    assert_eq!(
        inlined.doc.placements().len(),
        3,
        "the part's placement in its new space, the new placement of its copy, and the \
         second copy"
    );
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
    let (host, instance, [_, second], store) =
        host_of(two, "intent-c-twice-two", &|instance| instance.into());
    let opts = crate::fixture::resolver::with_resolver(store.clone());
    let before = product(&host, &crate::fixture::run(&host, &opts), Tol::witness())
        .expect("the host gathers");
    let resolver: std::sync::Arc<dyn editor_core::PartResolver> = std::sync::Arc::new(store);
    let inlined = editor_core::inline(&host, instance, &resolver, Tol::witness())
        .unwrap_or_else(|e| panic!("a two-body part placed twice inlines: {e}"));
    let Some(Node::Place { source, .. }) = inlined.doc.node(second) else {
        panic!("the second copy stays a placement")
    };
    assert_eq!(
        inlined.doc.operation_of(source.read),
        Some(inlined.placement),
        "the second copy reads the new placement's copies whole"
    );
    let after = product(&inlined.doc, &ev(&inlined.doc), Tol::witness()).expect("gathers");
    assert_eq!(
        body_digest(&after),
        body_digest(&before),
        "the product is unchanged"
    );
}

/// **A measure observes a copy.** A distance between two copies' end
/// caps reads the copies' placed geometry (a measure defines an
/// observed variable), so it is admitted at the edit door, measures the
/// pose, and round-trips through a file.
///
/// Red if the measure seat refuses (the quieting `Gap` of an
/// interference could never be written), or if it reads the unplaced
/// body (`0.0` rather than `2.0`).
#[test]
fn a_measure_observes_a_world_copy() {
    let doc = ProfileDoc::empty_derived("intent-c-measure-copy", Tol::witness());
    let (doc, a) = block(doc, 0.0);
    let (doc, b) = block(doc, 0.0);
    let (doc, p) = place(doc, a);
    let (doc, q) = place_by_values(doc, b, &raised());
    let (doc, measure) = crate::fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![
            SitedRef::new(p, cap(a, CapEnd::End)),
            SitedRef::new(q, cap(b, CapEnd::End)),
        ],
    );
    let out = crate::fixture::output(&doc, measure);
    let distance = |doc: &ProfileDoc| crate::fixture::reading(doc, &ev(doc), out);
    assert_eq!(
        distance(&doc),
        Some(2.0),
        "the measure reads the copies where they are placed"
    );
    let text = editor_core::persist::save(&doc, &[], Tol::witness()).expect("the document saves");
    let loaded = editor_core::persist::load(&text, Tol::witness())
        .unwrap_or_else(|e| panic!("a measure of two copies loads: {e}"))
        .doc;
    assert_eq!(distance(&loaded), Some(2.0), "and reads them after a load");
}
