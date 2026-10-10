//! **A split's port is its half** (FORK-1, DM3): a split defines two
//! bodies, read by port, and no `Part` selects a half. A file written
//! while one did refuses at load with the regenerate recourse.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, insert, len, on_frame_keeping, scl};
use editor_core::{Formula, Node, PartSelect, PatternKind, ProfileDoc, load, save};
use geom_core::Tol;

/// **A file holding a `Part` that selects a split's half refuses
/// `Unreadable`**, naming the variant this build no longer has, with
/// the regenerate recourse: the selector is a doctored
/// `Part { Instance }` spelled as the retired `SplitHalf`.
#[test]
fn a_file_holding_a_split_half_part_refuses_unreadable() {
    let (doc, _, profile) = on_frame_keeping(
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
            side: editor_core::ExtrudeSide::Along,
        },
    );
    let (doc, copies) = insert(
        doc,
        Node::Pattern {
            input: block.into(),
            count: Formula::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc, part) = insert(
        doc,
        Node::Part {
            of: copies.into(),
            select: PartSelect::Instance(Formula::count(0)),
        },
    );
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let at = text.find('{').expect("the JSON body follows the id header");
    let (header, body) = text.split_at(at);
    let mut body: serde_json::Value = serde_json::from_str(body).expect("the body parses");
    let mut doctored = 0;
    doctor(&mut body, &mut doctored);
    assert_eq!(doctored, 1, "the one Part's selector, {part:?}");
    let old = format!(
        "{header}{}",
        serde_json::to_string_pretty(&body).expect("prints")
    );
    let refusal = load(&old, Tol::witness()).err();
    assert!(
        matches!(
            &refusal,
            Some(editor_core::PersistError::Unreadable { detail, .. }) if detail.contains("SplitHalf")
        ),
        "{refusal:?}"
    );
    let said = refusal.map(|e| e.to_string()).unwrap_or_default();
    assert!(
        said.contains(editor_core::REGENERATE_RECOURSE),
        "the sentence carries the regenerate recourse: {said}"
    );
}

/// Every `select` object spelled `{"Instance": …}` re-spelled as a
/// pre-retirement half selector, counted.
fn doctor(value: &mut serde_json::Value, count: &mut usize) {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(select) = map.get_mut("select")
                && select.get("Instance").is_some()
            {
                *select = serde_json::json!({ "SplitHalf": "Above" });
                *count += 1;
            }
            map.values_mut().for_each(|v| doctor(v, count));
        }
        serde_json::Value::Array(items) => items.iter_mut().for_each(|v| doctor(v, count)),
        _ => {}
    }
}
