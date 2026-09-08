//! BOOL-10 second-round review probes at the document layer: the
//! `ArcTo { spec, splits }` wire shape and the content key.
//!
//! - The split count is in the memo key: a document differing from a
//!   prior evaluation's ONLY in `splits` must not be served the prior's
//!   profile.
//! - The wire field is REQUIRED, not defaulted: a document without
//!   `splits`, and one in the pre-BOOL-10 `"ArcTo": {"Bulge": …}` shape,
//!   refuse to load (D365's append-only rule is about tags, not about
//!   optional fields — this row records which the field is).
//! - A persisted `splits: 0` is a typed refusal at evaluation, not a
//!   panic.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    CancelToken, EvalOptions, Evaluation, LoopProgram, Node, ProfileDoc, ProfileProgram,
    ProgramArcData, ProgramStep, ProgramTarget, RecipeNodeId, ValuePayload, evaluate, persist,
};
use fixture::{insert, len, scl};
use geom_core::Tol;

fn p2(x: f64, y: f64) -> [editor_core::Expr; 2] {
    [len(x), len(y)]
}

fn run(doc: &ProfileDoc, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        prior,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// The half-disc: south pole → north pole as one semicircle with the
/// given split count, then the diameter back.
fn half_disc(splits: u32) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("bool10r2_probes", Tol::witness());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![LoopProgram::Chain(vec![
                ProgramStep::At(p2(0.0, -0.5)),
                ProgramStep::ArcTo {
                    spec: ProgramArcData::Bulge {
                        target: ProgramTarget::Point(p2(0.0, 0.5)),
                        b: scl(1.0),
                    },
                    splits,
                },
                ProgramStep::LineTo(ProgramTarget::Start),
            ])],
        }),
    )
}

fn vertex_count(ev: &Evaluation<f64>, id: RecipeNodeId) -> usize {
    let ValuePayload::Profile(p) = &ev
        .value(id)
        .unwrap_or_else(|| panic!("the profile evaluated: {:?}", ev.nodes.get(&id)))
        .payload
    else {
        panic!("a profile payload");
    };
    p.validated.loops().iter().map(|lp| lp.vertices().len()).sum()
}

/// **The split count is in the content key.** Evaluating the split
/// document against the unsplit one's memo must produce the split
/// loop (3 vertices), and back again the unsplit (2).
#[test]
fn the_split_count_distinguishes_the_memo_key() {
    let (plain, p_id) = half_disc(1);
    let (split, s_id) = half_disc(2);
    assert_eq!(p_id, s_id, "the two documents mint the same node id");
    let ev1 = run(&plain, None);
    assert_eq!(vertex_count(&ev1, p_id), 2);
    let ev2 = run(&split, Some(&ev1));
    assert_eq!(
        vertex_count(&ev2, s_id),
        3,
        "the memo served the unsplit loop for the split document"
    );
    let ev3 = run(&plain, Some(&ev2));
    assert_eq!(vertex_count(&ev3, p_id), 2);
}

/// Rewrites every `"ArcTo"` object in the saved body with `f`.
fn rewrite_arc_to(text: &str, f: &dyn Fn(&mut serde_json::Value)) -> String {
    let (header, body) = text.split_once('\n').expect("an id line then the body");
    let mut v: serde_json::Value = serde_json::from_str(body).expect("the body parses");
    fn walk(v: &mut serde_json::Value, f: &dyn Fn(&mut serde_json::Value)) {
        match v {
            serde_json::Value::Object(m) => {
                for (k, x) in m.iter_mut() {
                    if k == "ArcTo" {
                        f(x);
                    }
                    walk(x, f);
                }
            }
            serde_json::Value::Array(a) => a.iter_mut().for_each(|x| walk(x, f)),
            _ => {}
        }
    }
    walk(&mut v, f);
    format!("{header}\n{}", serde_json::to_string_pretty(&v).unwrap())
}

/// **`splits` is a REQUIRED wire field, and the old shape refuses.**
#[test]
fn the_wire_field_is_required_and_the_old_shape_refuses() {
    let (split, id) = half_disc(2);
    let text = persist::save(&split, &[], Tol::witness()).expect("saves");
    assert!(text.contains("\"splits\": 2"), "the count is on the wire");
    let back = persist::load(&text, Tol::witness()).expect("round-trips");
    assert_eq!(vertex_count(&run(&back.doc, None), id), 3);

    // Without the field.
    let without = rewrite_arc_to(&text, &|arc| {
        arc.as_object_mut().unwrap().remove("splits");
    });
    match persist::load(&without, Tol::witness()) {
        Err(e) => eprintln!("without splits: refused: {e}"),
        Ok(_) => panic!("a document without `splits` loaded — the field is defaulted"),
    }

    // The pre-BOOL-10 shape: the spec directly under "ArcTo".
    let old_shape = rewrite_arc_to(&text, &|arc| {
        let spec = arc.as_object_mut().unwrap().remove("spec").unwrap();
        *arc = spec;
    });
    match persist::load(&old_shape, Tol::witness()) {
        Err(e) => eprintln!("old shape: refused: {e}"),
        Ok(_) => panic!("the pre-BOOL-10 ArcTo shape loaded"),
    }

    // A persisted zero: loads (structurally a u32), refuses typed at
    // evaluation rather than panicking.
    let zero = rewrite_arc_to(&text, &|arc| {
        arc["splits"] = serde_json::json!(0);
    });
    match persist::load(&zero, Tol::witness()) {
        Err(e) => eprintln!("splits 0: refused at load: {e}"),
        Ok(loaded) => {
            let ev = run(&loaded.doc, None);
            assert!(
                ev.value(id).is_none(),
                "splits: 0 must not evaluate to a profile"
            );
            eprintln!("splits 0: loaded; node result {:?}", ev.nodes.get(&id));
        }
    }
}
