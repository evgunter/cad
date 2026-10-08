//! **An operation defines variables** (D10, Operations; INTENT stage 2
//! unit A): each node's insert mints one variable per port of its
//! signature (`Node::outputs`), the variable lives exactly as long as
//! its node, no door but a rename reaches it, and the load door's
//! `OutputSignature` walk holds a file's variable table to the
//! signatures of its nodes.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, ang, axis_in_plane, insert, len, on_frame_keeping, scl, xform};
use crate::wire::{doctored, up_to_ids, wire_body};
use editor_core::{
    CarryForwardDoor, DocEdit, EditError, Formula, FreeValue, Node, OutputFault, PatternKind,
    PersistError, ProfileDoc, RecipeNodeId, SeedError, SlotId, SnapshotError, VarDef, VarKind,
    VarName, load, save,
};
use geom_core::Tol;

/// A frame, a square on it and an extrude of the square.
fn block(seed: &str) -> (ProfileDoc, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let (doc, frame, profile) = on_frame_keeping(
        ProfileDoc::empty_derived(seed, Tol::witness()),
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.5, 0.5, 0.5)],
    );
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
            side: editor_core::ExtrudeSide::Along,
        },
    );
    (doc, frame, profile, extrude)
}

/// `node`'s outputs, each as its kind, checked against the document's
/// own accessors: one variable per port, in port order, each defined
/// as that port of `node`.
fn kinds(doc: &ProfileDoc, node: RecipeNodeId) -> Vec<VarKind> {
    let outputs = doc.outputs(node);
    outputs
        .iter()
        .zip(0u8..)
        .map(|(&var, port)| {
            assert_eq!(doc.output(node, port), Some(var), "port {port} of {node:?}");
            let held = doc.var(var).expect("an output is live");
            assert_eq!(
                held.def(),
                &VarDef::Output { node, port },
                "port {port} of {node:?}"
            );
            held.kind()
        })
        .collect()
}

#[test]
fn every_operation_defines_its_signature_at_insert() {
    let (doc, frame, profile, extrude) = block("s2a-signatures");
    let (doc, axis) = insert(doc, axis_in_plane(frame, (0.0, 0.0), (0.0, 1.0)));
    let (doc, revolve) = insert(
        doc,
        Node::Revolve {
            profile,
            axis,
            angle: ang(1.0),
        },
    );
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: extrude,
            tool: frame,
        },
    );
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: extrude,
            count: Formula::count(3),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(3.0),
            },
        },
    );
    let (doc, moved_copies) = insert(doc, xform(pattern, [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, moved_body) = insert(doc, xform(extrude, [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, twice_moved) = insert(
        doc,
        xform(moved_copies, [1.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0),
    );
    for (node, want) in [
        (frame, vec![VarKind::Frame]),
        (profile, vec![VarKind::Profile]),
        (extrude, vec![VarKind::Body]),
        (axis, vec![VarKind::Axis]),
        (revolve, vec![VarKind::Body, VarKind::Axis]),
        (split, vec![VarKind::Body, VarKind::Body]),
        (pattern, vec![VarKind::Bodies]),
        (moved_copies, vec![VarKind::Bodies]),
        (moved_body, vec![VarKind::Body]),
        (twice_moved, vec![VarKind::Bodies]),
    ] {
        assert_eq!(kinds(&doc, node), want, "{:?}", doc.node(node));
    }
    assert_eq!(doc.output(revolve, 2), None, "a revolve has two ports");
    // A file holds them, and loads as written.
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let loaded = load(&text, Tol::witness()).expect("loads").doc;
    assert!(loaded.bit_eq(&doc), "the outputs round-trip");
}

#[test]
fn an_insert_reports_its_outputs_and_a_delete_takes_them_with_its_node() {
    let (doc, _, profile, _) = block("s2a-lifecycle");
    let applied = doc
        .apply(
            &DocEdit::InsertNode {
                node: Box::new(Node::Extrude {
                    profile,
                    distance: len(2.0),
                    side: editor_core::ExtrudeSide::Along,
                }),
                fresh: Vec::new(),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect("inserts");
    let node = applied.record.minted.expect("an insert mints its node");
    assert_eq!(applied.record.outputs, applied.doc.outputs(node));
    let [body] = applied.record.outputs[..] else {
        panic!("an extrude defines one body: {:?}", applied.record.outputs)
    };
    // Named, it stays named, and a later edit's sweep of unread
    // anonymous variables leaves it standing.
    let name = VarName::new("tall").expect("a name");
    let (doc, _) = fixture::step(
        applied.doc,
        DocEdit::RenameVar {
            var: body.into(),
            name: Some(name.clone()),
        },
    );
    let (doc, _) = fixture::step(
        doc,
        DocEdit::SetParam {
            node,
            slot: SlotId::Distance,
            expr: len(3.0),
            fresh: Vec::new(),
        },
    );
    assert_eq!(doc.var_named("tall"), Some(body));
    let (doc, _) = fixture::step(doc, DocEdit::DeleteNode { id: node });
    assert_eq!(doc.var(body), None, "the output goes with its node");
    assert_eq!(doc.var_named("tall"), None, "and so does its name");
}

#[test]
fn no_door_but_a_rename_writes_an_output() {
    let (doc, _, _, extrude) = block("s2a-doors");
    let body = doc.output(extrude, 0).expect("an extrude defines its body");
    let refused = |edit: DocEdit<editor_core::ProfileProgram>, door: CarryForwardDoor| match doc
        .apply(&edit, Tol::witness(), &editor_core::RefusingReach)
    {
        Err(EditError::VarIsAnOutput { var, node, door: d }) => {
            assert_eq!((var.id(), node.id(), d), (body, extrude, door));
        }
        other => panic!("{door:?} of an output refuses VarIsAnOutput, got {other:?}"),
    };
    refused(
        DocEdit::DefineVar {
            var: body.into(),
            def: editor_core::VarDecl::Free(editor_core::FreeVar::continuous(
                editor_core::Dimension::Length,
                1.0,
            )),
            fresh: Vec::new(),
        },
        CarryForwardDoor::Definition,
    );
    refused(
        DocEdit::SetVarValue {
            var: body.into(),
            value: FreeValue::Continuous(1.0),
        },
        CarryForwardDoor::Value,
    );
    refused(
        DocEdit::DeleteVar { var: body.into() },
        CarryForwardDoor::Delete,
    );
}

#[test]
fn a_formula_reading_a_body_refuses_naming_its_kind() {
    let (doc, _, _, extrude) = block("s2a-formula");
    let body = doc.output(extrude, 0).expect("an extrude defines its body");
    let (doc, _) = fixture::step(
        doc,
        DocEdit::RenameVar {
            var: body.into(),
            name: Some(VarName::new("plate").expect("a name")),
        },
    );
    let edit = DocEdit::SetParam {
        node: extrude,
        slot: SlotId::Distance,
        expr: Formula::named(
            VarName::new("plate").expect("a name"),
            editor_core::Dimension::Length,
        ),
        fresh: Vec::new(),
    };
    match doc.apply(&edit, Tol::witness(), &editor_core::RefusingReach) {
        Err(error @ EditError::SlotVarKind { declared, .. }) => {
            assert_eq!(declared, VarKind::Body);
            assert!(error.to_string().contains("body"), "{error}");
        }
        other => panic!("a length slot reading a body refuses SlotVarKind, got {other:?}"),
    }
}

#[test]
fn an_output_is_no_analysis_axis_and_no_seed() {
    let (doc, _, _, extrude) = block("s2a-analysis");
    let body = doc.output(extrude, 0).expect("an extrude defines its body");
    assert!(
        doc.free_vars().all(|(id, _)| id != body),
        "an output is not free"
    );
    match editor_core::seed_env::<f64, _>(&doc, doc.var_env(), body) {
        Err(SeedError::SeedOnNonFreeVar { var }) => assert_eq!(var.id(), body),
        other => panic!("a seed on an output refuses SeedOnNonFreeVar, got {other:?}"),
    }
}

/// The saved form of [`block`], and the key its extrude's body is held
/// under.
fn saved_block() -> (String, String, String) {
    let (doc, _, _, extrude) = block("s2a-load");
    let body = doc.output(extrude, 0).expect("an extrude defines its body");
    (
        save(&doc, &[], Tol::witness()).expect("saves"),
        body.0.to_string(),
        extrude.0.to_string(),
    )
}

fn output_fault(text: &str) -> OutputFault {
    match load(text, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::OutputSignature { fault, .. })) => *fault,
        other => panic!("the load door refuses OutputSignature, got {other:?}"),
    }
}

#[test]
fn the_load_door_holds_the_table_to_every_signature() {
    let (text, body, extrude) = saved_block();
    // Port 1 of an extrude, which has one.
    let past = doctored(&text, |wire| {
        wire["snapshot"]["vars"][&body]["def"]["Output"]["port"] = 1.into();
    });
    assert!(
        matches!(
            output_fault(&past),
            OutputFault::PortOutside {
                port: 1,
                ports: 1,
                ..
            }
        ),
        "a port past the signature"
    );
    // The row removed: the walk asks the node side too.
    let missing = doctored(&text, |wire| {
        wire["snapshot"]["vars"]
            .as_object_mut()
            .unwrap()
            .remove(&body);
    });
    assert!(
        matches!(
            output_fault(&missing),
            OutputFault::Missing { port: "body" }
        ),
        "a live extrude with no row for its body"
    );
    // The row's kind not its port's.
    let kind = doctored(&text, |wire| {
        wire["snapshot"]["vars"][&body]["kind"] = "Profile".into();
    });
    assert!(
        matches!(
            output_fault(&kind),
            OutputFault::Kind {
                stored: VarKind::Profile,
                signature: VarKind::Body,
                ..
            }
        ),
        "a profile row on a body port"
    );
    // A row naming a node the file does not hold.
    let absent = doctored(&text, |wire| {
        wire["snapshot"]["vars"][&body]["def"]["Output"]["node"] = "999:0123456789abcdef".into();
    });
    assert!(
        matches!(output_fault(&absent), OutputFault::NodeAbsent { .. }),
        "a row on an absent node"
    );
    // Two rows on one port: the extrude's profile's row re-aimed at it.
    let twice = doctored(&text, |wire| {
        let vars = wire["snapshot"]["vars"].as_object_mut().unwrap();
        let (_, other) = vars
            .iter_mut()
            .find(|(_, var)| var["kind"] == "Profile")
            .expect("the profile's row");
        other["kind"] = "Body".into();
        other["def"]["Output"]["node"] = extrude.clone().into();
    });
    assert!(
        matches!(
            output_fault(&twice),
            OutputFault::Twice { port: "body", .. }
        ),
        "two rows on one port"
    );
}

#[test]
fn a_split_carries_a_named_output_onto_its_nodes_new_one() {
    let (doc, _, _, extrude) = block("s2a-split");
    let body = doc.output(extrude, 0).expect("an extrude defines its body");
    let (doc, _) = fixture::step(
        doc,
        DocEdit::RenameVar {
            var: body.into(),
            name: Some(VarName::new("plate").expect("a name")),
        },
    );
    let cut: std::collections::BTreeSet<_> = doc.ids().iter().copied().collect();
    let out = editor_core::split(
        &doc,
        &cut,
        editor_core::DocumentId::derive("s2a-split-part"),
        Tol::witness(),
        None,
    )
    .expect("splits");
    let carried = out.node_map[&extrude];
    assert_eq!(out.part.var_named("plate"), out.part.output(carried, 0));
    assert!(out.part.var_named("plate").is_some(), "the name crossed");
    assert_eq!(
        out.remainder.var_named("plate"),
        None,
        "and left the remainder"
    );
}

/// [`up_to_ids::equal_up_to_ids`] holds a document to itself with its
/// outputs set aside, and refuses each mutant a change could make, so
/// a green comparison is a statement and not a walk that accepts
/// anything.
#[test]
fn the_up_to_ids_comparator_holds_a_document_and_refuses_each_mutant() {
    let new = wire_body(include_str!("golden/golden.cad"));
    let old = up_to_ids::without_outputs(&new);
    up_to_ids::equal_up_to_ids(&old, &new).expect("a document is itself up to ids");

    let mut value = old.clone();
    let held = value["snapshot"]["vars"]
        .as_object_mut()
        .unwrap()
        .values_mut()
        .find_map(|var| {
            var["def"]["Free"]["Continuous"]["value"]
                .as_f64()
                .map(|_| var)
        })
        .expect("the golden holds a continuous free variable");
    let v = held["def"]["Free"]["Continuous"]["value"].as_f64().unwrap();
    held["def"]["Free"]["Continuous"]["value"] = (v + 1.0).into();
    let err = up_to_ids::equal_up_to_ids(&value, &new).expect_err("a moved value");
    assert!(err.contains("value"), "{err}");

    let mut dropped = old.clone();
    let nodes = dropped["snapshot"]["nodes"].as_object_mut().unwrap();
    let last = nodes.keys().next_back().unwrap().clone();
    nodes.remove(&last);
    let err = up_to_ids::equal_up_to_ids(&dropped, &new).expect_err("a dropped node");
    assert!(err.contains("nodes"), "{err}");

    let mut swapped = old.clone();
    let roots = swapped["snapshot"]["roots"].as_array_mut().unwrap();
    roots.swap(0, 1);
    let err = up_to_ids::equal_up_to_ids(&swapped, &new).expect_err("an inconsistent renaming");
    assert!(err.contains("earlier"), "{err}");
}

/// **Every re-blessed document is its pre-outputs self up to ids**: a
/// one-shot comparison against the files as main held them before
/// operations defined variables, which this tree no longer carries.
/// Run: `CAD_PRE_OUTPUTS_DIR=<a checkout of that main> cargo nextest run
/// -p editor-core --test all --run-ignored only
/// the_re_blessed_documents_equal_their_pre_outputs_selves`.
#[test]
#[ignore = "one-shot: needs the pre-outputs files at CAD_PRE_OUTPUTS_DIR"]
fn the_re_blessed_documents_equal_their_pre_outputs_selves_up_to_ids() {
    let root =
        std::env::var("CAD_PRE_OUTPUTS_DIR").expect("CAD_PRE_OUTPUTS_DIR names the old tree");
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for file in [
        "crates/editor-core/tests/golden/golden.cad",
        "crates/editor-core/tests/corpus/die_tool.pncad",
        "crates/editor-core/tests/corpus/tour/die_composed_tour.pncad",
        "crates/pncad/tests/plate_param.pncad",
        "crates/viewer/tests/gallery_ring.pncad",
    ] {
        let old = std::fs::read_to_string(std::path::Path::new(&root).join(file)).expect(file);
        let new = std::fs::read_to_string(here.join(file)).expect(file);
        up_to_ids::equal_up_to_ids(&wire_body(&old), &wire_body(&new))
            .unwrap_or_else(|err| panic!("{file}: {err}"));
        println!("{file}: equal up to ids");
    }
}

/// One shape's signature as the table below spells it: its variant (a
/// datum's own), then each port as `name:kind`, a placer's kind as
/// `placed`.
fn signature_row(node: &editor_core::AuthoredNode) -> String {
    let variant = match node {
        Node::Datum(datum) => format!("Datum::{}", test_utils::f6::variant_identifier(datum)),
        other => test_utils::f6::variant_identifier(other),
    };
    let ports: Vec<String> = node
        .outputs()
        .into_iter()
        .map(|port| match port.kind {
            editor_core::PortKind::Of(kind) => format!("{}:{kind:?}", port.name),
            editor_core::PortKind::PlacedFrom(_) => format!("{}:placed", port.name),
        })
        .collect();
    format!("{variant} -> [{}]", ports.join(", "))
}

/// **Every operation's signature, kind for kind** (D10, Operations;
/// FORK-1, FORK-1b): one row per node shape the slot census walks, the
/// ones that define nothing included.
#[test]
fn every_node_shape_states_its_signature() {
    let got: Vec<String> = crate::switch_slots::one_of_every_node_shape()
        .iter()
        .map(signature_row)
        .collect();
    let want = [
        "Datum::Plane -> [plane:Plane]",
        "Datum::Axis -> [axis:Axis]",
        "Datum::Point -> [point:Point]",
        "Datum::AxisInPlane -> [axis:Axis]",
        "Datum::Frame -> [frame:Frame]",
        "Datum::FaceFrame -> [frame:Frame]",
        "Profile -> [profile:Profile]",
        "Extrude -> [body:Body]",
        "Revolve -> [body:Body, axis:Axis]",
        "Tube -> [body:Body]",
        "Tube -> [body:Body]",
        "HollowTube -> [body:Body]",
        "Loft -> [body:Body]",
        "Sweep -> [body:Body]",
        "Fillet -> [body:Body]",
        "Chamfer -> [body:Body]",
        "Shell -> [body:Body]",
        "Split -> [above:Body, below:Body]",
        "Boolean -> [body:Body]",
        "Union -> [body:Body]",
        "Transform -> [body:placed]",
        "Transform -> [body:placed]",
        "Transform -> [body:placed]",
        "Pattern -> [bodies:Bodies]",
        "PlacedUnion -> [body:Body]",
        "PlacedUnion -> [body:Body]",
        "Pattern -> [bodies:Bodies]",
        "PlacedUnion -> [body:Body]",
        "PlacedUnion -> [body:Body]",
        "Pattern -> [bodies:Bodies]",
        "PlacedUnion -> [body:Body]",
        "PlacedUnion -> [body:Body]",
        "Part -> [body:Body]",
        "Part -> [body:Body]",
        "InstantiatePart -> [body:Body]",
        "Gauge -> []",
        "Mate -> []",
        "Measure -> [value:Length]",
        "Assertion -> []",
    ];
    assert_eq!(got, want, "a node shape's signature moved");
}

/// **A pose kind's symmetry** (D10, A11 (1)): a frame is known
/// outright, a plane up to in-plane motion, an axis up to slide and
/// spin; a point's and a direction's subgroups are not in the family,
/// and a scalar or a shape is no pose.
#[test]
fn a_pose_kind_names_its_subgroup_family() {
    use editor_core::SubgroupFamily;
    let want = [
        (VarKind::Frame, Some(SubgroupFamily::Trivial)),
        (VarKind::Plane, Some(SubgroupFamily::Planar)),
        (VarKind::Axis, Some(SubgroupFamily::Cylindrical)),
        (VarKind::Point, None),
        (VarKind::Direction, None),
        (VarKind::Length, None),
        (VarKind::Angle, None),
        (VarKind::Scalar, None),
        (VarKind::Count, None),
        (VarKind::Body, None),
        (VarKind::Bodies, None),
        (VarKind::Profile, None),
    ];
    for (kind, family) in want {
        assert_eq!(kind.symmetry(), family, "{kind:?}");
    }
}

/// **A datum's value folds the subgroup its kind names**: each datum of
/// [`block`]'s kinds, evaluated, gives the subgroup ([`PoseSymmetry`])
/// whose family is its output kind's symmetry, so the mates, which fold
/// the same values' subgroups, and the kinds share one vocabulary.
#[test]
fn a_datums_value_folds_the_subgroup_its_kind_names() {
    use editor_core::PoseSymmetry;
    let (doc, frame, ..) = block("s2a-pose-symmetry");
    let (doc, plane) = insert(
        doc,
        Node::Datum(editor_core::Datum::Plane {
            origin: [len(0.0), len(0.0), len(0.0)],
            normal: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    let (doc, axis) = insert(doc, axis_in_plane(frame, (0.0, 0.0), (0.0, 1.0)));
    let (doc, point) = insert(
        doc,
        Node::Datum(editor_core::Datum::Point {
            position: [len(0.0), len(0.0), len(0.0)],
        }),
    );
    let ev = crate::corpus::eval::<f64>(&doc);
    for node in [frame, plane, axis, point] {
        let editor_core::ValuePayload::Datum(value) = &ev.value(node).expect("evaluates").payload
        else {
            panic!("a datum evaluates to a datum value")
        };
        let kind = doc
            .var(doc.output(node, 0).expect("a datum defines its pose"))
            .unwrap()
            .kind();
        assert_eq!(
            value.symmetry().and_then(|subgroup| subgroup.family()),
            kind.symmetry(),
            "{kind:?}"
        );
    }
}

/// `doc` with the variable `var` named `name`.
fn named(doc: ProfileDoc, var: editor_core::VarId, name: &str) -> ProfileDoc {
    fixture::step(
        doc,
        DocEdit::RenameVar {
            var: var.into(),
            name: Some(VarName::new(name).expect("a name")),
        },
    )
    .0
}

/// [`block`] split whole into a part, the remainder's instance's body
/// named `bracket`, and the part's extrude's body named `heir` first
/// when given one; returns the remainder, the instance, the store
/// holding the part, and the part's extrude.
fn instance_named_bracket(
    seed: &str,
    heir: Option<&str>,
    second_body: bool,
) -> (
    ProfileDoc,
    RecipeNodeId,
    std::sync::Arc<dyn editor_core::PartResolver>,
    RecipeNodeId,
) {
    let (mut doc, _, profile, extrude) = block(seed);
    if let Some(heir) = heir {
        let body = doc.output(extrude, 0).expect("an extrude defines its body");
        doc = named(doc, body, heir);
    }
    if second_body {
        doc = insert(
            doc,
            Node::Extrude {
                profile,
                distance: len(2.0),
                side: editor_core::ExtrudeSide::Along,
            },
        )
        .0;
    }
    let cut: std::collections::BTreeSet<_> = doc.ids().iter().copied().collect();
    let out = editor_core::split(
        &doc,
        &cut,
        editor_core::DocumentId::derive(&format!("{seed}-part")),
        Tol::witness(),
        None,
    )
    .expect("splits");
    let mut store = fixture::resolver::PartStore::default();
    store.insert(out.part.clone(), Tol::witness());
    let body = out
        .remainder
        .output(out.instance, 0)
        .expect("an instance defines its body");
    let remainder = named(out.remainder, body, "bracket");
    (
        remainder,
        out.instance,
        std::sync::Arc::new(store),
        out.node_map[&extrude],
    )
}

/// **Inline carries the name on an instance's body** onto the one body
/// of the part that stands where it did (VR2: the kernel drops no name),
/// and refuses typed where the part has no one such body, or where that
/// body's own name would make a variable hold two.
#[test]
fn inline_carries_the_name_on_an_instances_body() {
    let (host, instance, store, _) = instance_named_bracket("s2a-inline", None, false);
    let out = editor_core::inline(&host, instance, &store, Tol::witness()).expect("inlines");
    let bracket = out.doc.var_named("bracket").expect("the name crossed");
    let (node, port) = out
        .doc
        .var(bracket)
        .unwrap()
        .def()
        .output()
        .expect("onto an output");
    assert!(
        matches!(out.doc.node(node), Some(Node::Extrude { .. })),
        "{node:?}"
    );
    assert_eq!(port, 0);

    let (host, instance, store, _) = instance_named_bracket("s2a-inline-two", None, true);
    match editor_core::inline(&host, instance, &store, Tol::witness()) {
        Err(editor_core::InlineError::InstanceOutputUncarried { name, why }) => {
            assert_eq!(name.as_str(), "bracket");
            assert_eq!(why, editor_core::Uncarried::Bodies { count: 2 });
        }
        other => panic!("two body roots have no one heir, got {other:?}"),
    }

    let (host, instance, store, _) =
        instance_named_bracket("s2a-inline-held", Some("plate"), false);
    match editor_core::inline(&host, instance, &store, Tol::witness()) {
        Err(editor_core::InlineError::InstanceOutputUncarried { name, why }) => {
            assert_eq!(name.as_str(), "bracket");
            assert_eq!(
                why,
                editor_core::Uncarried::HeirNamed {
                    held: VarName::new("plate").unwrap()
                }
            );
        }
        other => panic!("a named heir would hold two names, got {other:?}"),
    }

    let (host, instance, store, _) =
        instance_named_bracket("s2a-inline-taken", Some("bracket"), false);
    match editor_core::inline(&host, instance, &store, Tol::witness()) {
        Err(editor_core::InlineError::VarNameConflict { name }) => {
            assert_eq!(name.as_str(), "bracket");
        }
        other => panic!("a carried name the host holds refuses VarNameConflict, got {other:?}"),
    }
}

/// **A slot reading a measured value refuses at evaluation saying so**:
/// the door that refuses it is unit D's (`ConstructionReadsObserved`),
/// and until then the reader's refusal names the output it read, never
/// a deleted or undeclared variable.
#[test]
fn a_slot_reading_an_output_refuses_naming_it() {
    let (doc, _, _, extrude) = block("s2a-observed");
    let (doc, measure) = insert(
        doc,
        Node::measure(editor_core::MeasureExpr::value(len(0.5)), Vec::new())
            .expect("a measured value"),
    );
    let gap = doc.output(measure, 0).expect("a measure defines its value");
    assert_eq!(doc.var(gap).unwrap().kind(), VarKind::Length);
    let doc = named(doc, gap, "gap");
    let (doc, _) = fixture::step(
        doc,
        DocEdit::SetParam {
            node: extrude,
            slot: SlotId::Distance,
            expr: Formula::named(VarName::new("gap").unwrap(), editor_core::Dimension::Length),
            fresh: Vec::new(),
        },
    );
    let ev = crate::corpus::eval::<f64>(&doc);
    let Some(editor_core::NodeResult::Failed(error)) = ev.nodes.get(&extrude) else {
        panic!("the reader refuses: {:?}", ev.nodes.get(&extrude))
    };
    let said = error.to_string();
    assert!(said.contains("an operation's output"), "{said}");
    assert!(!said.contains("deleted"), "{said}");
}
