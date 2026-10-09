//! **A split's port is its half** (FORK-1, DM3): a split defines two
//! bodies, read by port, and no `Part` selects a half. A file written
//! while one did refuses at load with the regenerate recourse.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, insert, len, on_frame_keeping, scl};
use editor_core::{
    BooleanOp, Formula, Node, Operand, PartSelect, PatternKind, ProfileDoc, SitedRef, SplitHalf,
    load, save,
};
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
    let old = format!("{header}{}", serde_json::to_string_pretty(&body).expect("prints"));
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

/// A unit block cut at mid-height, the block, and the split's section
/// face bounding `side`.
fn halves(
    name: &str,
) -> (
    ProfileDoc,
    editor_core::RecipeNodeId,
    editor_core::RecipeNodeId,
    impl Fn(SplitHalf) -> editor_core::StableName,
) {
    let block = |doc, x: f64| {
        let (doc, _, profile) = on_frame_keeping(
            doc,
            [x, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![fixture::square(0.5, 0.5, 0.5)],
        );
        insert(
            doc,
            Node::Extrude {
                profile: profile.into(),
                distance: len(1.0),
                side: editor_core::ExtrudeSide::Along,
            },
        )
    };
    let (doc, target) = block(ProfileDoc::empty_derived(name, Tol::witness()), 0.0);
    let (doc, plane) = insert(
        doc,
        Node::Datum(editor_core::Datum::Plane {
            origin: [len(0.0), len(0.0), len(0.5)],
            normal: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: target.into(),
            tool: plane.into(),
        },
    );
    let section = move |side| crate::corpus::part_select::section_face(split, side);
    (doc, split, target, section)
}

/// **A declared pair between a split's two halves is sided by the half
/// that holds each name**: both are read at the split's one site, so a
/// boolean and a union of `split.0` and `split.1` under a declared rest
/// across the section rejoin the block whichever order the pair names
/// them in; a side naming what neither half holds — a wall of the
/// block the cut renamed in both — refuses, at the boolean as a site no
/// operand's table answers and at the union as the vanished name a
/// member site is.
#[test]
fn a_pair_declared_across_one_splits_halves_is_sided_by_table() {
    let (doc, split, target, section) = halves("split-half-siding");
    let before = fixture::run(&doc, &editor_core::EvalOptions::default());
    let held = |id| before.value(id).expect("a value").name_table.clone();
    let (whole, cut) = (held(target), held(split));
    let wall = whole
        .iter()
        .map(|(name, _)| name.clone())
        .find(|name| name.kind == editor_core::EntityKind::Face && cut.lookup(name).is_none())
        .expect("a wall the cut renamed");
    let half = |h: SplitHalf| Operand::output(split, h.port());
    let rest = |first: SplitHalf, second: SplitHalf| {
        editor_core::declare_rest(vec![(
            SitedRef::new(split, section(first)),
            SitedRef::new(split, section(second)),
        )])
    };
    let (doc, boolean) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: half(SplitHalf::Above),
            b: half(SplitHalf::Below),
            declare: rest(SplitHalf::Below, SplitHalf::Above),
        },
    );
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: vec![half(SplitHalf::Below), half(SplitHalf::Above)],
            declare: rest(SplitHalf::Above, SplitHalf::Below),
        },
    );
    let stray = || {
        editor_core::declare_rest(vec![(
            SitedRef::new(split, section(SplitHalf::Above)),
            SitedRef::new(split, wall.clone()),
        )])
    };
    let (doc, stray_boolean) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: half(SplitHalf::Above),
            b: half(SplitHalf::Below),
            declare: stray(),
        },
    );
    let (doc, stray_union) = insert(
        doc,
        Node::Union {
            members: vec![half(SplitHalf::Above), half(SplitHalf::Below)],
            declare: stray(),
        },
    );
    let ev = fixture::run(&doc, &editor_core::EvalOptions::default());
    for (id, what) in [(boolean, "the boolean"), (union, "the union")] {
        let body = crate::corpus::body_of(&ev, id);
        let volume = topo::mass_properties(body, Tol::witness())
            .expect("mass properties")
            .volume;
        assert!((volume - 1.0).abs() < 1e-12, "{what} is the whole block: {volume}");
    }
    assert!(
        matches!(
            ev.node_error(stray_boolean).map(|e| &e.kind),
            Some(editor_core::NodeErrorKind::DeclareSiteNotAnOperand { at }) if *at == split
        ),
        "{:?}",
        ev.node_error(stray_boolean)
    );
    assert!(
        matches!(
            ev.node_error(stray_union).map(|e| &e.kind),
            Some(editor_core::NodeErrorKind::DeclareResolve { .. })
        ),
        "{:?}",
        ev.node_error(stray_union)
    );
}
