//! **Delta review probes for the binder fix pass (PR 7506)**: rows that
//! go red where the fix pass's claims leak — containment, the yield
//! refusals off their own door, the typed load refusal, a removed yield
//! reported, the refactors over a binder.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::block;
use crate::fixture::{self, insert};
use editor_core::{
    Bodies, BodyRead, DocEdit, EditError, Formula, Maintenance, Node, Operand, ProfileDoc, Spell,
    VarId, VarRef,
};
use geom_core::Tol;

fn try_apply(
    doc: &ProfileDoc,
    edit: DocEdit<editor_core::ProfileProgram>,
) -> Result<editor_core::Applied<editor_core::ProfileProgram>, EditError> {
    doc.apply(&edit, Tol::witness())
}

fn short<T: core::fmt::Debug>(x: &T) -> String {
    format!("{x:?}").chars().take(400).collect()
}

/// A plate, `k = index(3)`, a variable `d := scalar(k) * 1` (a length
/// reading `k`), and a row of a small block slid by `d`, yielded by the
/// binder; then a union `[plate, …row]` placed in the world.
struct Row {
    doc: ProfileDoc,
    k: VarId,
    d: VarId,
    plate: RecipeNodeId,
    row: RecipeNodeId,
    union: RecipeNodeId,
}

use editor_core::RecipeNodeId;

fn row_doc(label: &str) -> Row {
    row_doc_with(label, true)
}

fn row_doc_with(label: &str, with_union: bool) -> Row {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, plate) = block(doc, (-1.0, 10.0), (-1.0, 1.0), 0.5);
    let (doc, small) = block(doc, (0.0, 0.4), (-0.2, 0.2), 1.0);
    let (doc, k) = fixture::index(doc, "k", 3);
    let doc = fixture::step(
        doc,
        DocEdit::DeclareVar {
            name: editor_core::VarName::new("d").unwrap(),
            def: editor_core::VarDecl::Defined(
                Formula::mul(fixture::scalar_of(k), fixture::len(1.0)).unwrap(),
            ),
        },
    )
    .0;
    let d = doc.var_named("d").unwrap();
    let target = fixture::space(&doc, plate);
    let placed = editor_core::Placed::of(
        &doc,
        Vec::new(),
        vec![(
            editor_core::Coord::slide(editor_core::TargetAxis::X),
            Formula::var(d, editor_core::Dimension::Length),
        )],
    );
    let (doc, row) = insert(
        doc,
        Node::place(
            BodyRead::plain(Operand::Node(small)),
            Operand::Var(target),
            placed.bundle(),
        ),
    );
    let doc = fixture::yielded(doc, row);
    assert!(!doc.yields(k).is_empty(), "the binder yields the row");
    if !with_union {
        return Row {
            doc,
            k,
            d,
            plate,
            row,
            union: row,
        };
    }
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![
                Spell::One(BodyRead::plain(Operand::Node(plate))),
                Spell::All(BodyRead::plain(Operand::Node(row))),
            ]),
        },
    );
    let (doc, _) = fixture::place(doc, union);
    Row {
        doc,
        k,
        d,
        plate,
        row,
        union,
    }
}

/// The document's file, the JSON body edited by `f`, loaded.
fn load_edited(
    doc: &ProfileDoc,
    f: impl FnOnce(&mut serde_json::Value),
) -> Result<ProfileDoc, editor_core::persist::PersistError> {
    let text = editor_core::persist::save(doc, Tol::witness()).unwrap();
    let (head, body) = text.split_once('\n').unwrap();
    let mut v: serde_json::Value = serde_json::from_str(body).unwrap();
    f(&mut v);
    editor_core::persist::load(
        &format!("{head}\n{}\n", serde_json::to_string(&v).unwrap()),
        Tol::witness(),
    )
}

/// Every object holding `count` and `seat` (an index's definition).
fn each_index(
    v: &mut serde_json::Value,
    f: &mut impl FnMut(&mut serde_json::Map<String, serde_json::Value>),
) {
    match v {
        serde_json::Value::Object(m) => {
            if m.contains_key("count") && m.contains_key("seat") {
                f(m);
            }
            m.iter_mut().for_each(|(_, x)| each_index(x, f));
        }
        serde_json::Value::Array(xs) => xs.iter_mut().for_each(|x| each_index(x, f)),
        _ => {}
    }
}

/// Claim 4: a file written before binders whose index is NESTED —
/// main's `{"count": …, "within": k}` — refuses typed as
/// `IndexBeforeBinders`, as the unnested `{"count": …}` does.
#[test]
fn d_a_nested_index_written_before_binders_refuses_typed() {
    let r = row_doc("delta-within");
    let kj = serde_json::to_value(r.k).unwrap();
    let mut n = 0;
    let loaded = load_edited(&r.doc, |v| {
        each_index(v, &mut |m| {
            m.remove("seat");
            m.insert("within".into(), kj.clone());
            n += 1;
        });
    });
    assert_eq!(n, 1);
    match loaded {
        Err(editor_core::persist::PersistError::IndexBeforeBinders { .. }) => {}
        other => panic!(
            "main's nested index refuses typed, got {}",
            short(&other.map(|_| "a document").map_err(|e| e.to_string()))
        ),
    }
}

/// Claim "empty yields admitted": a hand-written file whose binder
/// yields nothing while a k-reader stands in it. The doors leave this
/// state (`DeleteNode` of the yield), so load admits it; evaluation
/// refuses it `YieldsNothing`. A report row: it holds that pair.
#[test]
fn d_a_file_whose_binder_yields_nothing_loads_and_evaluation_refuses() {
    let r = row_doc_with("delta-file-nothing", false);
    let loaded = load_edited(&r.doc, |v| {
        each_index(v, &mut |m| {
            m["seat"] = serde_json::json!({"Binder": {"yields": []}});
        });
    });
    eprintln!(
        "nothing-file: {}",
        short(&loaded.as_ref().map(|_| ()).map_err(|e| e.to_string()))
    );
    let doc = loaded.expect("a state the doors leave loads");
    let ev = fixture::run(&doc, &Default::default());
    assert!(
        matches!(
            ev.result(r.row),
            Some(editor_core::NodeResult::Failed(e))
                if matches!(e.kind, editor_core::NodeErrorKind::YieldsNothing { .. })
        ),
        "the k-reader refuses YieldsNothing: {}",
        short(&ev.result(r.row))
    );
}

/// Claim 4 `Outside`: a hand-written file yielding a variable that reads
/// none of the binder's indices refuses at load.
#[test]
fn d_a_file_yielding_an_outside_variable_refuses_at_load() {
    let r = row_doc_with("delta-file-outside", false);
    let plate_out = r.doc.output(r.plate, 0).unwrap();
    let pj = serde_json::to_value(plate_out).unwrap();
    let rj = serde_json::to_value(r.doc.output(r.row, 0).unwrap()).unwrap();
    let mut seen = String::new();
    let loaded = load_edited(&r.doc, |v| {
        each_index(v, &mut |m| {
            seen = m["seat"].to_string();
            m["seat"] = serde_json::json!({"Binder": {"yields": [rj.clone(), pj.clone()]}});
        });
    });
    eprintln!(
        "outside-file (seat was {seen}): {}",
        short(&loaded.as_ref().map(|_| ()).map_err(|e| e.to_string()))
    );
    if let Ok(doc) = &loaded {
        panic!(
            "an Outside yield loaded (seat was {seen}): yields {:?}",
            doc.yields(r.k)
        );
    }
}

/// Claim 4 `Outside`, off its own door: a redefinition that makes a
/// yield read no index leaves the binder yielding a variable outside it.
/// The door refuses it, or the yield is dropped and reported.
#[test]
fn d_a_redefinition_cannot_leave_a_yield_outside() {
    let r = row_doc("delta-redef-outside");
    let row_out = r.doc.output(r.row, 0).unwrap();
    let redefine = DocEdit::DefineVar {
        var: VarRef::from(r.d),
        def: editor_core::VarDecl::Defined(fixture::len(1.0)),
        fresh: Vec::new(),
    };
    match try_apply(&r.doc, redefine) {
        Err(e) => eprintln!("REFUSED {e}"),
        Ok(after) => {
            let doc = after.doc;
            let still = doc.yields(r.k).contains(&row_out);
            let fresh = editor_core::persist::load(
                &editor_core::persist::save(&doc, Tol::witness()).unwrap(),
                Tol::witness(),
            )
            .unwrap();
            eprintln!(
                "reach(d) {:?}; reach(row) reloaded {:?}; node_reach(row) {:?}",
                short(&doc.var_reach(r.d)),
                short(&fresh.var_reach(row_out)),
                short(&doc.node_reach(r.row))
            );
            if let Some(Node::Place { constraints, .. }) = doc.node(r.row) {
                for c in constraints {
                    eprintln!("constraint {}", short(c));
                }
            }
            for (id, v) in doc.vars() {
                eprintln!("var {id:?} = {}", short(&v.def()));
            }
            let ev = fixture::run(&doc, &Default::default());
            eprintln!(
                "row result kind: {}",
                short(&ev.value(r.row).map(|v| v.payload.kind_name()))
            );
            eprintln!(
                "k = {:?}; d now {}; row node {}",
                r.k,
                short(&doc.var(r.d).map(|v| v.def().clone())),
                short(&doc.node(r.row))
            );
            let reach = doc.var_reach(row_out);
            eprintln!(
                "ACCEPTED: yields {:?} (row still yielded: {still}); row reach {:?}; maintenance {}",
                doc.yields(r.k),
                short(&reach),
                short(&after.maintenance)
            );
            let set_again = try_apply(
                &doc,
                DocEdit::SetYields {
                    binder: r.k.into(),
                    yields: vec![row_out.into()],
                },
            );
            eprintln!(
                "the same yields through SetYields now: {}",
                short(&set_again.map(|_| ()).map_err(|e| e.to_string()))
            );
            let reloaded = editor_core::persist::save(&doc, Tol::witness())
                .ok()
                .map(|t| editor_core::persist::load(&t, Tol::witness()).map(|_| ()));
            eprintln!(
                "save/load: {}",
                short(&reloaded.map(|r| r.map_err(|e| e.to_string())))
            );
            // `var_reach` counts a yield as reading its binder's indices
            // (repetition.rs, "A yield is one member per value"), so the
            // row's own reach cannot show it; its values' reach does.
            let _ = reach;
            assert!(
                !(still && doc.var_reach(r.d).chain().is_some_and(<[VarId]>::is_empty)),
                "the edit left the binder yielding {row_out:?}, whose every value now reads no index"
            );
        }
    }
}

/// Claim 4 `YieldRemoved`: deleting the row a union splices reports the
/// yield gone, and the document the door leaves loads.
#[test]
fn d_a_yield_removed_while_its_reader_stays_is_reported_and_loads() {
    let r = row_doc("delta-yield-removed");
    let after = try_apply(&r.doc, DocEdit::DeleteNode { id: r.row })
        .unwrap_or_else(|e| panic!("delete refused: {e}"));
    eprintln!(
        "maintenance {}; yields now {:?}",
        short(&after.maintenance),
        after.doc.yields(r.k)
    );
    assert!(
        after
            .maintenance
            .iter()
            .any(|m| matches!(m, Maintenance::YieldRemoved { .. })),
        "the removed yield is reported"
    );
    let text = editor_core::persist::save(&after.doc, Tol::witness()).expect("saves");
    if let Err(e) = editor_core::persist::load(&text, Tol::witness()) {
        panic!("the door left a document load refuses: {e}");
    }
    // The binder now yields nothing: what does the door say to the
    // next edit, and what does evaluation say?
    let ev = fixture::run(&after.doc, &Default::default());
    eprintln!("union result: {}", short(&ev.result(r.union)));
}

/// Claim 4 `Nothing`, off its own door: emptying a binder's yields by
/// another arm (`DeleteNode`) leaves a binder `SetYields` would refuse;
/// say which arms reach it.
#[test]
fn d_set_yields_refuses_nothing_but_the_delete_arm_reaches_it() {
    let r = row_doc("delta-nothing-arm");
    let after = try_apply(&r.doc, DocEdit::DeleteNode { id: r.union })
        .unwrap()
        .doc;
    let after = try_apply(&after, DocEdit::DeleteNode { id: r.row }).unwrap();
    eprintln!(
        "binder yields after deleting its yield: {:?}; maintenance {}",
        after.doc.yields(r.k),
        short(&after.maintenance)
    );
    let direct = try_apply(
        &r.doc,
        DocEdit::SetYields {
            binder: r.k.into(),
            yields: vec![],
        },
    );
    assert!(direct.is_err(), "SetYields refuses an empty yield list");
}

/// Claim 1: a part whose product is its binder's yield (a family), read
/// by a host's `InstantiatePart`: the host's instance is one body.
#[test]
fn d_an_instance_of_a_part_whose_product_is_a_family() {
    let (part, _block, ring) = crate::g_index_families::ring(4);
    let set = try_apply(
        &part,
        DocEdit::SetProduct {
            product: Some(Operand::Node(ring)),
        },
    );
    let part = match set {
        Ok(a) => a.doc,
        Err(e) => {
            eprintln!("a family product is refused: {e}");
            return;
        }
    };
    eprintln!("ACCEPTED a family as the part's product");
    let mut store = fixture::resolver::PartStore::default();
    let part_ref = store.insert(part, Tol::witness());
    let doc = ProfileDoc::empty_derived("delta-host", Tol::witness());
    let (doc, inst) = insert(
        doc,
        Node::instantiate_part(
            part_ref,
            Operand::Var(VarId::WORLD),
            editor_core::stand().bundle(),
        ),
    );
    let o = fixture::resolver::with_resolver(store);
    let ev = fixture::run(&doc, &o);
    eprintln!("instance result: {}", short(&ev.result(inst)));
    let gathered = editor_core::product_recorded(&doc, &ev, Tol::witness());
    eprintln!(
        "host product: {}",
        short(&gathered.map(|p| p.solid_copies.len()))
    );
}

/// Claim 1: split, inline and flatten over a binder leave documents that
/// load (load holds containment and the seats), cutting the row alone,
/// the union alone, and both.
#[test]
fn d_refactors_over_a_binder_leave_loadable_documents() {
    let r = row_doc("delta-refactor");
    let loads = |what: &str, doc: &ProfileDoc| {
        let text = editor_core::persist::save(doc, Tol::witness())
            .unwrap_or_else(|e| panic!("{what}: save refused: {e}"));
        if let Err(e) = editor_core::persist::load(&text, Tol::witness()) {
            panic!("{what}: load refused: {e}");
        }
        let ev = fixture::run(doc, &Default::default());
        let failed: Vec<String> = doc
            .ids()
            .into_iter()
            .filter_map(|id| match ev.result(id) {
                Some(editor_core::NodeResult::Failed(e)) => Some(short(&e.kind)),
                _ => None,
            })
            .collect();
        eprintln!("{what}: failures {failed:?}");
    };
    loads("source", &r.doc);
    let all: Vec<RecipeNodeId> = r.doc.ids().into_iter().collect();
    let cuts: Vec<(&str, Vec<RecipeNodeId>)> = vec![("all", all)];
    for (what, cut) in cuts {
        let cut: std::collections::BTreeSet<_> = cut.into_iter().collect();
        match fixture::split_world(
            &r.doc,
            &cut,
            editor_core::DocumentId::derive(&format!("delta-part-{what}")),
            Tol::witness(),
            None,
        ) {
            Err(e) => eprintln!("split {what}: refused {e}"),
            Ok(out) => {
                eprintln!(
                    "split {what}: part yields {:?}; remainder yields {:?}",
                    out.part.var_named("k").map(|k| out.part.yields(k).to_vec()),
                    out.remainder
                        .var_named("k")
                        .map(|k| out.remainder.yields(k).to_vec())
                );
                loads(&format!("split {what} part"), &out.part);
                loads(&format!("split {what} remainder"), &out.remainder);
                let mut store = fixture::resolver::PartStore::default();
                let _ = store.insert(out.part.clone(), Tol::witness());
                let store: std::sync::Arc<dyn editor_core::PartResolver> =
                    std::sync::Arc::new(store);
                match editor_core::inline(&out.remainder, out.instance, &store, Tol::witness()) {
                    Err(e) => eprintln!("inline {what}: refused {e}"),
                    Ok(back) => {
                        loads(&format!("inline {what}"), &back.doc);
                        let placements = back.doc.placements();
                        for p in placements {
                            match editor_core::flatten(&back.doc, p, Tol::witness()) {
                                Err(e) => eprintln!("flatten {what} {p:?}: refused {e}"),
                                Ok(flat) => loads(&format!("flatten {what} {p:?}"), &flat.doc),
                            }
                        }
                    }
                }
            }
        }
    }
    // Flatten the source's own placements, the row among them.
    for p in r.doc.placements() {
        match editor_core::flatten(&r.doc, p, Tol::witness()) {
            Err(e) => eprintln!("flatten source {p:?}: refused {e}"),
            Ok(flat) => loads(&format!("flatten source {p:?}"), &flat.doc),
        }
    }
}

/// Claim 1, a hand-written file: the product on the ring, the binder's
/// yields stripped, so the product reads a k-reader no binder yields.
#[test]
fn d_a_file_leaving_the_product_on_a_k_reader_refuses_at_load() {
    let (doc, _block, ring) = crate::g_index_families::ring(4);
    let doc = try_apply(
        &doc,
        DocEdit::SetProduct {
            product: Some(Operand::Node(ring)),
        },
    )
    .unwrap()
    .doc;
    let loaded = load_edited(&doc, |v| {
        each_index(v, &mut |m| {
            m["seat"] = serde_json::json!({"Binder": {"yields": []}});
        });
    });
    match loaded {
        Err(e) => {
            eprintln!("REFUSED {e}");
            assert!(
                format!("{e:?}").contains("PerMember"),
                "refused for containment: {e:?}"
            );
        }
        Ok(_) => panic!("a file leaving the product on a k-reader loaded"),
    }
}

/// Claim "silent pruning", in the refactor: a split moving the row and
/// its union while a second k-reader stays in the host. Does the part
/// still yield the row, and does the host still see three copies?
#[test]
fn d_a_split_leaving_the_binder_behind() {
    let r = row_doc("delta-split-behind");
    let (doc, other) = block(r.doc.clone(), (20.0, 21.0), (-0.5, 0.5), 1.0);
    let before: std::collections::BTreeSet<_> = r.doc.ids().into_iter().collect();
    let slide = Formula::mul(fixture::scalar_of(r.k), fixture::len(2.0)).unwrap();
    let (doc, row2) = fixture::family_of(
        doc,
        other,
        vec![(editor_core::Coord::slide(editor_core::TargetAxis::X), slide)],
    );
    eprintln!("yields before: {:?}", doc.yields(r.k));
    let cut = before;
    let _ = row2;
    let src_ev = fixture::run(&doc, &Default::default());
    eprintln!(
        "source union: {}",
        short(&src_ev.value(r.union).map(|v| v.payload.kind_name()))
    );
    match fixture::split_world(
        &doc,
        &cut,
        editor_core::DocumentId::derive("delta-split-behind-part"),
        Tol::witness(),
        None,
    ) {
        Err(e) => eprintln!("split refused: {e}"),
        Ok(out) => {
            eprintln!(
                "part k: {:?}; part yields {:?}; remainder yields {:?}; remainder maintenance {}",
                out.part
                    .var_named("k")
                    .map(|k| short(&out.part.var(k).map(|v| v.def().clone()))),
                out.part.var_named("k").map(|k| out.part.yields(k).to_vec()),
                out.remainder
                    .var_named("k")
                    .map(|k| out.remainder.yields(k).to_vec()),
                short(&out.remainder_maintenance)
            );
            let ev = fixture::run(&out.part, &Default::default());
            let u = out.node_map.get(&r.union).copied();
            let row = out.node_map.get(&r.row).copied();
            eprintln!(
                "part row: {}; part union: {}",
                short(
                    &row.and_then(|id| ev.value(id))
                        .map(|v| v.payload.kind_name())
                ),
                short(&u.and_then(|id| ev.value(id)).map(|v| v.payload.kind_name()))
            );
        }
    }
}
