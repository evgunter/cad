//! **A split's port is its half** (FORK-1, DM3): a split defines two
//! bodies, read by port, and no node selects a half. A file holding a
//! `Part` node, which once did, refuses at load with the regenerate
//! recourse.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, insert, len, on_frame};
use editor_core::{Node, ProfileDoc, load, save};
use geom_core::Tol;

/// **A file holding a `Part` that selects a split's half refuses
/// `Unreadable`**, naming the variant this build no longer has, with
/// the regenerate recourse: the node is a doctored extrude, spelled as
/// the retired `Part { select: SplitHalf }`.
#[test]
fn a_file_holding_a_split_half_part_refuses_unreadable() {
    let (doc, profile) = on_frame(
        ProfileDoc::empty_derived("split-half-retires", Tol::witness()),
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.5, 0.5, 0.5)],
    );
    let (doc, block) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let at = text.find('{').expect("the JSON body follows the id header");
    let (header, body) = text.split_at(at);
    let mut body: serde_json::Value = serde_json::from_str(body).expect("the body parses");
    let mut doctored = 0;
    doctor(&mut body, &mut doctored);
    assert_eq!(doctored, 1, "the block's extrude, {block:?}");
    let old = format!(
        "{header}{}",
        serde_json::to_string_pretty(&body).expect("prints")
    );
    let refusal = load(&old, Tol::witness()).err();
    assert!(
        matches!(
            &refusal,
            Some(editor_core::PersistError::Unreadable { detail, .. }) if detail.contains("Part")
        ),
        "{refusal:?}"
    );
    let said = refusal.map(|e| e.to_string()).unwrap_or_default();
    assert!(
        said.contains(editor_core::REGENERATE_RECOURSE),
        "the sentence carries the regenerate recourse: {said}"
    );
}

/// Every node object spelled `{"Extrude": …}` re-spelled as a
/// pre-retirement `Part` selecting a split's half, counted.
fn doctor(value: &mut serde_json::Value, count: &mut usize) {
    match value {
        serde_json::Value::Object(map) => {
            if map.len() == 1 && map.contains_key("Extrude") {
                let of = map["Extrude"]["profile"].clone();
                map.clear();
                map.insert(
                    "Part".to_owned(),
                    serde_json::json!({ "of": of, "select": { "SplitHalf": "Above" } }),
                );
                *count += 1;
                return;
            }
            map.values_mut().for_each(|v| doctor(v, count));
        }
        serde_json::Value::Array(items) => items.iter_mut().for_each(|v| doctor(v, count)),
        _ => {}
    }
}

/// **A measure whose two references read the two halves of one split
/// reads each half by its own port**, in either order: the far caps of
/// a 1 × 1 × 4 column split at z = 2 are 4 apart. Red while a measure's
/// reads were projected into one map keyed by the split, where the
/// later read overwrote the earlier and its name vanished.
#[test]
fn a_measure_across_one_splits_two_halves_reads_each_by_its_port() {
    use editor_core::{
        CapEnd, EntityKind, MeasurePrimitive, NamePat, NodeResult, Operand, SegPat, SegTag,
        Selector, SplitHalf, ValuePayload, select,
    };
    let mut r = fixture::Recorder::new();
    let profile = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let column = r.insert(Node::Extrude {
        profile: profile.into(),
        distance: len(4.0),
    });
    let tool = fixture::plane_on(&r.doc, column, [0.0, 0.0, 2.0], [0.0, 0.0, 1.0]);
    let split = r.insert(Node::Split {
        target: column.into(),
        tool,
    });
    let ev = fixture::run(&r.doc, &editor_core::EvalOptions::default());
    // Each half is read by its port (NAMES N1): the end cap is the
    // upper half's, the start cap the lower's.
    let cap = |end: CapEnd| {
        let half = match end {
            CapEnd::End => SplitHalf::Above,
            CapEnd::Start => SplitHalf::Below,
        };
        let found = select(
            &ev,
            r.doc.output(split, half.port()).expect("a half's read"),
            &Selector::of(
                NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Cap).side(end)),
            ),
        );
        let [name] = found.as_slice() else {
            panic!("one {end:?} cap on the split's {half:?} half: {found:?}")
        };
        name.clone()
    };
    let on = |half: SplitHalf, end: CapEnd| Operand::Select {
        body: Box::new(Operand::output(split, half.port())),
        names: vec![cap(end)],
    };
    let top = on(SplitHalf::Above, CapEnd::End);
    let bottom = on(SplitHalf::Below, CapEnd::Start);
    for (label, refs) in [
        ("above first", [top.clone(), bottom.clone()]),
        ("below first", [bottom, top]),
    ] {
        let (doc, m) = fixture::measure(
            r.doc.clone(),
            &[MeasurePrimitive::Distance { a: 0, b: 1 }],
            &refs,
        );
        let ev = fixture::run(&doc, &editor_core::EvalOptions::default());
        match ev.nodes.get(&m.measures[0]) {
            Some(NodeResult::Ok(v)) => match &v.payload {
                ValuePayload::Measure { value, .. } => {
                    assert!((value - 4.0).abs() < 1e-9, "{label}: distance {value}");
                }
                other => panic!("{label}: {}", other.kind_name()),
            },
            other => panic!("{label}: the measure did not evaluate: {other:?}"),
        }
    }
}
