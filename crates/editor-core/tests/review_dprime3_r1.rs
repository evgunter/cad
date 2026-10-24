//! Reviewer R1 probes for D′3 (PR 5636). Each row prints what it
//! measured; the assertions are the claims under test.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::{
    Bodies, BodyRead, DocEdit, DocumentId, EvalOptions, Evaluation, Node, Operand, ProfileDoc,
    RecipeNodeId, ValuePayload, VarId, apply,
};
use fixture::{len, on_frame, run, square};
use geom_core::Tol;

fn block(doc: ProfileDoc, [x, y]: [f64; 2], half: f64, height: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(x, y, half)],
    );
    put(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(height),
        },
    )
}

fn beside(
    doc: ProfileDoc,
    of: RecipeNodeId,
    [x, y, z]: [f64; 3],
    half: f64,
    height: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, body) = block(doc, [x, y], half, height);
    fixture::slid(doc, of, body, [0.0, 0.0, z])
}

fn put(doc: ProfileDoc, node: editor_core::AuthoredNode) -> (ProfileDoc, RecipeNodeId) {
    let (doc, minted) = fixture::step(
        doc,
        DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        },
    );
    (doc, minted.expect("the insert mints the node"))
}

fn spelled(reads: &[RecipeNodeId]) -> Bodies<BodyRead<editor_core::Formula>> {
    Bodies::Spelled(reads.iter().map(|&r| r.into()).collect())
}

fn set_product(doc: &ProfileDoc, product: Option<Operand>) -> ProfileDoc {
    apply(doc, &DocEdit::SetProduct { product }, Tol::witness())
        .expect("the slot takes a body in the world")
        .doc
}

fn doc(label: &str) -> ProfileDoc {
    ProfileDoc::empty(DocumentId::derive(label), Tol::witness())
}

fn in_world(
    doc: ProfileDoc,
    node: impl Into<BodyRead<editor_core::Formula>>,
) -> (ProfileDoc, RecipeNodeId) {
    put(doc, fixture::stood(node))
}

fn solids_at(ev: &Evaluation<f64>, node: RecipeNodeId) -> Result<usize, String> {
    match ev.value(node).map(|v| &v.payload) {
        Some(ValuePayload::Body(b)) => Ok(b.solids().count()),
        Some(ValuePayload::Boolean(editor_core::BooleanValue::Body { body, .. })) => {
            Ok(body.solids().count())
        }
        _ => Err(format!("{:?}", ev.node_error(node))),
    }
}

fn unproven(doc: &ProfileDoc, ev: &Evaluation<f64>) -> usize {
    editor_core::run_checks(
        doc,
        ev,
        &editor_core::ChecksConfig::default(),
        Tol::witness(),
    )
    .expect("the checks run")
    .findings
    .iter()
    .filter(|f| f.check == editor_core::CheckId::UnprovenCoincidence)
    .count()
}

fn all_findings(doc: &ProfileDoc, ev: &Evaluation<f64>) -> String {
    let r = editor_core::run_checks(
        doc,
        ev,
        &editor_core::ChecksConfig::default(),
        Tol::witness(),
    )
    .expect("the checks run");
    format!("{} findings: {r}", r.findings.len())
}

/// A bolt seated on a plate, plus a far nut, each a world copy.
fn seat_world(label: &str) -> (ProfileDoc, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let doc = doc(label);
    let (doc, p) = block(doc, [0.0; 2], 1.0, 0.2);
    let (doc, bolt) = beside(doc, p, [0.0, 0.0, 0.2], 0.2, 1.0);
    let (doc, nut) = beside(doc, p, [5.0, 0.0, 0.0], 0.3, 0.3);
    let (doc, pw) = in_world(doc, p);
    let (doc, bw) = in_world(doc, bolt);
    let (doc, nw) = in_world(doc, nut);
    (doc, pw, bw, nw)
}

/// Claim 1: a meeting inside an operand of a `⊔` is not decided again
/// by an outer `⊔`. Flat `p ⊔ b ⊔ n` against nested `(p ⊔ b) ⊔ n`:
/// the bolt–plate touch is reported the same number of times.
#[test]
fn r1_nested_assemble_does_not_redecide_the_inner_touch() {
    let (flat, pw, bw, nw) = seat_world("r1-flat");
    let (flat, three) = put(
        flat,
        Node::Assemble {
            members: spelled(&[pw, bw, nw]),
        },
    );
    let flat = set_product(&flat, Some(Operand::Node(three)));
    let ev = run(&flat, &EvalOptions::default());
    let fp = editor_core::product_recorded(&flat, &ev, Tol::witness()).unwrap();
    let flat_unproven = unproven(&flat, &ev);
    eprintln!("FLAT assemble: {:?}", editor_core::assemble(&flat, &ev, Tol::witness()).err().map(|e| format!("{e:?}").chars().take(500).collect::<String>()));
    eprintln!(
        "FLAT: rows {} census {} unproven {}",
        fp.coincidences.len(),
        fp.census.len(),
        flat_unproven
    );

    let (nested, pw, bw, nw) = seat_world("r1-nested");
    let (nested, inner) = put(
        nested,
        Node::Assemble {
            members: spelled(&[pw, bw]),
        },
    );
    let (nested, outer) = put(
        nested,
        Node::Assemble {
            members: spelled(&[inner, nw]),
        },
    );
    let nested = set_product(&nested, Some(Operand::Node(outer)));
    let ev = run(&nested, &EvalOptions::default());
    let np = editor_core::product_recorded(&nested, &ev, Tol::witness()).unwrap();
    let inner_rows: Vec<usize> = np
        .assembles
        .iter()
        .map(|g| g.gather.as_ref().map(|p| p.coincidences.len()).unwrap_or(999))
        .collect();
    let nested_unproven = unproven(&nested, &ev);
    eprintln!("NESTED census: {:?}", np.census);
    eprintln!("NESTED assemble: {:?}", editor_core::assemble(&nested, &ev, Tol::witness()).err().map(|e| format!("{e:?}").chars().take(800).collect::<String>()));
    eprintln!(
        "NESTED: outer rows {} census {}; inner gathers' rows {:?}; unproven {} | {}",
        np.coincidences.len(),
        np.census.len(),
        inner_rows,
        nested_unproven,
        all_findings(&nested, &ev)
    );
    assert_eq!(np.coincidences.len(), 0, "the outer ⊔ decides nothing inside (p ⊔ b)");
    assert_eq!(nested_unproven, flat_unproven, "the inner touch is reported once");
}

/// Claim 1, interference: `(a ⊔ a') ⊔ n` with `a`, `a'` overlapping.
#[test]
fn r1_nested_assemble_interference_reported_once() {
    let doc0 = doc("r1-nested-overlap");
    let (doc0, a) = block(doc0, [0.0; 2], 0.5, 1.0);
    let (doc0, a2) = beside(doc0, a, [0.3, 0.0, 0.0], 0.5, 1.0);
    let (doc0, n) = beside(doc0, a, [5.0, 0.0, 0.0], 0.3, 0.3);
    let (doc0, aw) = in_world(doc0, a);
    let (doc0, a2w) = in_world(doc0, a2);
    let (doc0, nw) = in_world(doc0, n);
    let (flat, three) = put(
        doc0.clone(),
        Node::Assemble {
            members: spelled(&[aw, a2w, nw]),
        },
    );
    let flat = set_product(&flat, Some(Operand::Node(three)));
    let ev = run(&flat, &EvalOptions::default());
    let flat_res = editor_core::assemble(&flat, &ev, Tol::witness());
    let flat_s = match &flat_res {
        Err(e) => format!("{e:?}"),
        Ok(_) => "ok".into(),
    };
    eprintln!("FLAT overlap: {}", &flat_s[..flat_s.len().min(600)]);

    let (nested, inner) = put(
        doc0,
        Node::Assemble {
            members: spelled(&[aw, a2w]),
        },
    );
    let (nested, outer) = put(
        nested,
        Node::Assemble {
            members: spelled(&[inner, nw]),
        },
    );
    let nested = set_product(&nested, Some(Operand::Node(outer)));
    let ev = run(&nested, &EvalOptions::default());
    let pr = editor_core::product_recorded(&nested, &ev, Tol::witness());
    match &pr {
        Ok(p) => eprintln!(
            "NESTED overlap product: outer census {} rows {}; inner gathers {:?}",
            p.census.len(),
            p.coincidences.len(),
            p.assembles
                .iter()
                .map(|g| g.gather.as_ref().map(|p| p.census.len()).map_err(|e| format!("{e:?}")))
                .collect::<Vec<_>>()
        ),
        Err(e) => eprintln!("NESTED overlap product refused: {e:?}"),
    }
    let res = editor_core::assemble(&nested, &ev, Tol::witness());
    let count = |r: &Result<editor_core::Assembly<f64>, editor_core::AssemblyError>| match r {
        Err(editor_core::AssemblyError::AtRest { findings }) => findings.len() as i64,
        Err(_) => -1,
        Ok(_) => 0,
    };
    eprintln!("AT-REST findings: flat {} nested {}", count(&flat_res), count(&res));
    eprintln!("CHECKS flat: {} | nested: {}", all_findings(&flat, &run(&flat, &EvalOptions::default())).lines().next().unwrap(), all_findings(&nested, &ev).lines().next().unwrap());
    let s = match &res {
        Err(e) => format!("{e:?}"),
        Ok(_) => "ok".into(),
    };
    eprintln!("NESTED overlap assemble: {}", &s[..s.len().min(900)]);
    assert!(res.is_err(), "the overlap inside the inner ⊔ is still a finding");
    assert!(
        pr.as_ref().is_ok_and(|p| p.census.is_empty()),
        "the outer ⊔ decides nothing inside its operand"
    );
}

/// Claim 2: a cut, union or meet of a seated `⊔` that leaves two solids
/// touching each other (no solid touching itself) does not refuse.
#[test]
fn r1_booleans_over_a_seat_that_bridge_nothing_keep_its_pieces() {
    let doc0 = doc("r1-seat-cut");
    let (doc0, plate) = block(doc0, [0.0; 2], 1.0, 0.2);
    let (doc0, bolt) = beside(doc0, plate, [0.0, 0.0, 0.2], 0.2, 1.0);
    let (doc0, seat) = put(
        doc0,
        Node::Assemble {
            members: spelled(&[bolt, plate]),
        },
    );
    // A tool through the plate's corner only.
    let (doc0, corner) = beside(doc0, plate, [0.9, 0.0, -0.5], 0.3, 2.0);
    // A tool through the bolt and the plate where they meet.
    let (doc0, through) = beside(doc0, plate, [0.2, 0.0, -0.5], 0.1, 3.0);
    let (doc0, through2) = beside(doc0, plate, [0.17, 0.03, -0.37], 0.07, 2.9);
    // The same tool through a bolt standing clear of the plate.
    let (doc0, lifted) = beside(doc0, plate, [0.0, 0.0, 0.25], 0.2, 1.0);
    let (doc0, gap) = put(
        doc0,
        Node::Assemble {
            members: spelled(&[lifted, plate]),
        },
    );
    // A box holding everything.
    let (doc0, big) = beside(doc0, plate, [0.0, 0.0, -1.0], 3.0, 4.0);
    // A box far away.
    let (doc0, far) = beside(doc0, plate, [9.0, 0.0, 0.0], 0.3, 0.3);
    let mut doc1 = doc0;
    let mut rows = Vec::new();
    for (label, node) in [
        (
            "seat - corner",
            Node::Subtract {
                from: seat.into(),
                tool: corner.into(),
            },
        ),
        (
            "seat - through",
            Node::Subtract {
                from: seat.into(),
                tool: through.into(),
            },
        ),
        (
            "seat - through2",
            Node::Subtract {
                from: seat.into(),
                tool: through2.into(),
            },
        ),
        (
            "gap - through2",
            Node::Subtract {
                from: gap.into(),
                tool: through2.into(),
            },
        ),
        (
            "seat ∩ big",
            Node::Intersect {
                members: spelled(&[seat, big]),
            },
        ),
        (
            "seat ∪ far",
            Node::Union {
                members: spelled(&[seat, far]),
            },
        ),
    ] {
        let (d, id) = put(doc1, node);
        doc1 = d;
        rows.push((label, id));
    }
    let ev = run(&doc1, &EvalOptions::default());
    let mut bad = Vec::new();
    for (label, id) in rows {
        let got = solids_at(&ev, id);
        eprintln!("{label}: {got:?}");
        let want = if label == "seat ∪ far" { 3 } else { 2 };
        if got != Ok(want) {
            bad.push((label, got));
        }
    }
    assert!(bad.is_empty(), "refused or merged: {bad:?}");
}

/// Claim 3: the slot's door is held after the edit that set it.
#[test]
fn r1_product_moved_out_of_the_world_after_setproduct() {
    let doc0 = doc("r1-moved-product");
    let (doc0, p) = block(doc0, [0.0; 2], 0.5, 0.5);
    let (doc0, pw) = in_world(doc0, p);
    let doc0 = set_product(&doc0, Some(Operand::Node(pw)));
    let space = fixture::space(&doc0, p);
    let edit = DocEdit::SetParam {
        node: pw,
        slot: editor_core::SlotId::Operand(editor_core::OperandSlot::Target),
        value: editor_core::SlotValue::Read(Operand::Var(space)),
        fresh: Vec::new(),
    };
    match apply(&doc0, &edit, Tol::witness()) {
        Err(e) => eprintln!("the retarget refuses: {e}"),
        Ok(out) => {
            let doc1 = out.doc;
            let var = doc1.product().unwrap();
            eprintln!(
                "retarget admitted; product var {var:?} now in space {:?} (world {:?})",
                doc1.space_of(var),
                VarId::WORLD
            );
            let ev = run(&doc1, &EvalOptions::default());
            let pr = editor_core::product(&doc1, &ev, Tol::witness());
            eprintln!("product of the moved slot: {:?}", pr.as_ref().map(|b| b.solids().count()));
            let checks = editor_core::run_checks(
                &doc1,
                &ev,
                &editor_core::ChecksConfig::default(),
                Tol::witness(),
            );
            eprintln!("checks: {:?}", checks.map(|r| r.to_string()));
            assert!(
                pr.is_err() || doc1.space_of(var) == Some(VarId::WORLD),
                "a product outside the world gathers"
            );
        }
    }
}

/// Claim 3/7: a stored document whose slot reads a body outside the
/// world, or a var that does not exist, loads.
#[test]
fn r1_load_holds_the_product_door() {
    let doc0 = doc("r1-load-product");
    let (doc0, p) = block(doc0, [0.0; 2], 0.5, 0.5);
    let (doc0, pw) = in_world(doc0, p);
    let doc0 = set_product(&doc0, Some(Operand::Node(pw)));
    let text = editor_core::persist::save(&doc0, &[], Tol::witness()).unwrap();
    let product = doc0.product().unwrap();
    let inner = doc0.output(p, 0).unwrap();
    let ser = |v: VarId| serde_json::to_string(&v).unwrap();
    let needle = format!("\"product\":{}", ser(product));
    let needle_pretty = format!("\"product\": {}", ser(product));
    eprintln!("product {} inner {} found {} / {}", ser(product), ser(inner), text.contains(&needle), text.contains(&needle_pretty));
    for (what, to) in [("inner body", ser(inner)), ("no var", "999999".to_string())] {
        let swapped = text
            .replace(&needle, &format!("\"product\":{to}"))
            .replace(&needle_pretty, &format!("\"product\": {to}"));
        assert_ne!(swapped, text, "the needle is in the text");
        match editor_core::persist::load(&swapped, Tol::witness()) {
            Ok(l) => {
                let ev = run(&l.doc, &EvalOptions::default());
                eprintln!(
                    "LOAD with product = {what}: admitted; product gathers {:?}",
                    editor_core::product(&l.doc, &ev, Tol::witness()).map(|b| b.solids().count())
                );
            }
            Err(e) => eprintln!("LOAD with product = {what}: refused {e}"),
        }
    }
    let no_slot = text
        .replace(&format!(",\"product\":{}", ser(product)), "")
        .replace(&needle_pretty, "\"labels_r1_gone\": 0");
    eprintln!("drop-slot text differs: {}", no_slot != text);
}

/// Claim 4: an instance of a part whose product overlaps itself.
#[test]
fn r1_instance_of_an_overlapping_part_reports_once() {
    let part = doc("r1-overlap-part");
    let (part, a) = block(part, [0.0; 2], 0.5, 1.0);
    let (part, a2) = beside(part, a, [0.3, 0.0, 0.0], 0.5, 1.0);
    let (part, aw) = in_world(part, a);
    let (part, a2w) = in_world(part, a2);
    let (part, both) = put(
        part,
        Node::Assemble {
            members: spelled(&[aw, a2w]),
        },
    );
    let part = set_product(&part, Some(Operand::Node(both)));
    let ev = run(&part, &EvalOptions::default());
    let own = editor_core::assemble(&part, &ev, Tol::witness());
    eprintln!("PART assemble: {:?}", own.as_ref().err().map(|e| format!("{e:?}").chars().take(400).collect::<String>()));
    eprintln!("PART checks: {}", all_findings(&part, &ev));
    let mut store = fixture::resolver::PartStore::default();
    let part_ref = store.insert(part, Tol::witness());
    let host = doc("r1-overlap-host");
    let (host, i1) = put(
        host,
        Node::instantiate_part(part_ref, Operand::Var(VarId::WORLD), editor_core::stand().bundle()),
    );
    let host = set_product(&host, Some(Operand::Node(i1)));
    let ev = run(&host, &fixture::resolver::with_resolver(store));
    let hp = editor_core::product_recorded(&host, &ev, Tol::witness());
    match &hp {
        Ok(p) => eprintln!(
            "HOST product: census {} carried_interference {} carried_at_rest {} census_pairs {}",
            p.census.len(),
            p.carried_interference.len(),
            p.carried_at_rest.len(),
            p.census_pairs.len()
        ),
        Err(e) => eprintln!("HOST product refused {e:?}"),
    }
    let res = editor_core::assemble(&host, &ev, Tol::witness());
    eprintln!("HOST assemble: {:?}", res.as_ref().err().map(|e| format!("{e:?}").chars().take(600).collect::<String>()));
    eprintln!("HOST checks: {}", all_findings(&host, &ev));
}

/// Claim 5: a pick through an instance is held at the door or at
/// evaluation; a pick of a `⊔` reading one body twice.
#[test]
fn r1_pick_paths_the_door_cannot_see() {
    // The part: (bolt ∪ plate) and a far nut, ⊔'d, the product.
    let part = doc("r1-pick-part");
    let (part, plate) = block(part, [0.0; 2], 1.0, 0.2);
    let (part, bolt) = beside(part, plate, [0.0, 0.0, 0.2], 0.2, 1.0);
    let (part, nut) = beside(part, plate, [5.0, 0.0, 0.0], 0.3, 0.3);
    let (part, fused) = put(
        part,
        Node::Union {
            members: spelled(&[bolt, plate]),
        },
    );
    let (part, fw) = in_world(part, fused);
    let (part, nw) = in_world(part, nut);
    let (part, both) = put(
        part,
        Node::Assemble {
            members: spelled(&[fw, nw]),
        },
    );
    let part = set_product(&part, Some(Operand::Node(both)));
    let bolt_var = part.output(bolt, 0).unwrap();
    let mut store = fixture::resolver::PartStore::default();
    let part_ref = store.insert(part, Tol::witness());
    let host = doc("r1-pick-host");
    let (host, inst) = put(
        host,
        Node::instantiate_part(part_ref, Operand::Var(VarId::WORLD), editor_core::stand().bundle()),
    );
    let attempt = |host: &ProfileDoc, of: Vec<VarId>, label: &str| {
        let node = fixture::stood(BodyRead::plain(Operand::pick(Operand::Node(inst), of)));
        match apply(
            host,
            &DocEdit::InsertNode {
                node: Box::new(node),
                fresh: Vec::new(),
            },
            Tol::witness(),
        ) {
            Err(e) => {
                eprintln!("{label}: door refuses: {e}");
                None
            }
            Ok(out) => {
                let id = out.record.minted.unwrap();
                eprintln!("{label}: door admits");
                Some((out.doc, id))
            }
        }
    };
    for (label, of) in [
        ("through the part's ∪ to the bolt", vec![bolt_var]),
        ("a var the part never had", vec![VarId::WORLD]),
    ] {
        if let Some((d, id)) = attempt(&host, of, label) {
            let ev = run(&d, &fixture::resolver::with_resolver(store.clone()));
            eprintln!("{label}: evaluates to {:?}", solids_at(&ev, id));
            assert!(solids_at(&ev, id).is_err(), "{label}: a bad pick reads a body");
        }
    }

    // A `⊔` reading one body twice, placed, picked by that body.
    let d = doc("r1-pick-twice");
    let (d, a) = block(d, [0.0; 2], 0.5, 1.0);
    let (d, twice) = put(
        d,
        Node::Assemble {
            members: spelled(&[a, a]),
        },
    );
    let (d, place) = put(d, fixture::stood(twice));
    let a_var = d.output(a, 0).unwrap();
    let node = fixture::stood(BodyRead::plain(Operand::pick(Operand::Node(place), vec![a_var])));
    match apply(
        &d,
        &DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        },
        Tol::witness(),
    ) {
        Err(e) => eprintln!("pick of A ⊔ A by A: door refuses {e}"),
        Ok(out) => {
            let id = out.record.minted.unwrap();
            let ev = run(&out.doc, &EvalOptions::default());
            eprintln!("pick of A ⊔ A by A: admitted, evaluates to {:?}", solids_at(&ev, id));
        }
    }
}

/// Claim 1/4: a seated sub-assembly `bolt ⊔ plate`, placed once in the
/// world and made the product — the spelling a list placement became.
#[test]
fn r1_a_placed_seat_is_at_rest() {
    let d = doc("r1-placed-seat");
    let (d, plate) = block(d, [0.0; 2], 1.0, 0.2);
    let (d, bolt) = beside(d, plate, [0.0, 0.0, 0.2], 0.2, 1.0);
    let (d, seat) = put(
        d,
        Node::Assemble {
            members: spelled(&[bolt, plate]),
        },
    );
    let (d, placed) = in_world(d, seat);
    let d = set_product(&d, Some(Operand::Node(placed)));
    let ev = run(&d, &EvalOptions::default());
    let res = editor_core::assemble(&d, &ev, Tol::witness());
    eprintln!(
        "PLACED SEAT assemble: {:?}",
        res.as_ref().err().map(|e| format!("{e:?}").chars().take(700).collect::<String>())
    );
    eprintln!("PLACED SEAT checks: {}", all_findings(&d, &ev));
    assert!(res.is_ok(), "a placed seat is at rest");
}

/// Claim 7: main's stored documents refuse, typed, at load.
#[test]
fn r1_main_documents_refuse_at_load() {
    for path in [
        "/home/user/r1-main/crates/editor-core/tests/golden/golden.cad",
        "/home/user/r1-main/crates/editor-core/tests/corpus/die_tool.pncad",
        "/home/user/r1-main/crates/editor-core/tests/corpus/tour/die_composed_tour.pncad",
        "/home/user/r1-main/crates/pncad/tests/plate_param.pncad",
        "/home/user/r1-main/crates/viewer/tests/gallery_ring.pncad",
        "/home/user/r1-main/crates/editor-core/tests/r1_main_instance_only.pncad",
    ] {
        let Ok(text) = std::fs::read_to_string(path) else {
            eprintln!("MAINLOAD {path}: absent");
            continue;
        };
        match editor_core::persist::load(&text, Tol::witness()) {
            Ok(l) => eprintln!("MAINLOAD {path}: ADMITTED, product {:?}", l.doc.product()),
            Err(e) => eprintln!(
                "MAINLOAD {path}: refused: {}",
                e.to_string().chars().take(250).collect::<String>()
            ),
        }
    }
}
