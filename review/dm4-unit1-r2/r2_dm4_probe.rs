//! Reviewer r2's probes for FORK-DM4 unit 1 (PR 4527). Not part of the PR.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};

use editor_core::{NodeResult, ProfileDoc};
use serde_json::Value;

/// Every `{"From": {"read": R, ..}}` met while walking `v`, paired with
/// the `node` of the innermost enclosing name.
fn froms(v: &Value, enclosing: Option<&str>, out: &mut Vec<(String, String)>) {
    match v {
        Value::Object(map) => {
            let here = match (map.get("node"), map.get("path"), map.get("kind")) {
                (Some(Value::String(n)), Some(_), Some(_)) => Some(n.as_str()),
                _ => enclosing,
            };
            if let Some(Value::Object(f)) = map.get("From")
                && let Some(Value::String(r)) = f.get("read")
            {
                out.push((here.unwrap_or("?").to_owned(), r.clone()));
            }
            for (_, child) in map {
                froms(child, here, out);
            }
        }
        Value::Array(xs) => xs.iter().for_each(|x| froms(x, enclosing, out)),
        _ => {}
    }
}

fn s<T: serde::Serialize>(t: &T) -> String {
    match serde_json::to_value(t).unwrap() {
        Value::String(s) => s,
        other => other.to_string(),
    }
}

/// Claim 6: every carry segment in every published table is keyed by a
/// read its minting node holds at one of its seats; no fold sentinel.
fn check(name: &str, doc: &ProfileDoc) -> (usize, usize, Vec<String>) {
    let reads: BTreeMap<String, BTreeSet<String>> = doc
        .ids()
        .into_iter()
        .map(|id| {
            let rs = doc
                .node(id)
                .unwrap()
                .operand_rows()
                .into_iter()
                .map(|(_, v)| s(&v))
                .collect();
            (s(&id), rs)
        })
        .collect();
    let ev = crate::corpus::eval::<f64>(doc);
    let (mut checked, mut foreign, mut bad) = (0, 0, Vec::new());
    for (id, res) in &ev.nodes {
        let NodeResult::Ok(value) = res else { continue };
        for (n, _) in value.name_table.iter() {
            let mut out = Vec::new();
            froms(&serde_json::to_value(n).unwrap(), None, &mut out);
            for (minter, read) in out {
                match reads.get(&minter) {
                    None => foreign += 1,
                    Some(rs) if rs.contains(&read) => checked += 1,
                    Some(_) => bad.push(format!(
                        "{name}: table of {id}: From read {read} under minter {minter} is none of its seats"
                    )),
                }
            }
        }
    }
    (checked, foreign, bad)
}

#[test]
fn r2_every_carry_is_keyed_by_a_seat_read_of_its_minter() {
    let mut all_bad = Vec::new();
    let mut total = 0;
    for d in crate::corpus::documents() {
        let (c, f, bad) = check(d.name, &d.doc);
        println!("{}: {c} carries checked, {f} under a foreign minter", d.name);
        total += c;
        all_bad.extend(bad);
    }
    for b in all_bad.iter().take(20) {
        println!("{b}");
    }
    assert!(total > 0, "the probe saw carries");
    assert!(all_bad.is_empty(), "{} bad carries", all_bad.len());
}

use crate::eval6_placers_over_instances::{cube_doc, linear};
use crate::fixture::{insert, insert_refused, len, out, step};
use editor_core::{
    Bodies, BodyRead, DocEdit, EditError, Formula, Node, NodeError, NodeErrorKind, OperandSlot,
    RecipeNodeId, SlotId,
};
use geom_core::Tol;

fn family(label: &str) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, cube) = cube_doc(label);
    let (doc, xs) = insert(doc, linear(cube, [1.0, 0.0, 0.0], 2.0, 3));
    (doc, cube, xs)
}

fn member(xs: RecipeNodeId, i: i64) -> BodyRead<Formula> {
    BodyRead::indexed(xs, vec![Formula::count(i)])
}

fn err(ev: &editor_core::Evaluation<f64>, id: RecipeNodeId) -> String {
    match ev.nodes.get(&id) {
        Some(NodeResult::Failed(NodeError { kind, .. })) => format!("{kind:?}"),
        other => format!("not failed: {}", other.map_or("none".into(), |o| format!("{:?}", o.value().is_some()))),
    }
}

fn vol(ev: &editor_core::Evaluation<f64>, id: RecipeNodeId) -> f64 {
    topo::mass_properties(crate::corpus::body_of(ev, id), Tol::witness())
        .expect("mass")
        .volume
}

/// Claim 2: a negative index refuses `InstanceOutOfRange`.
#[test]
fn r2_a_negative_index_refuses_out_of_range() {
    let (doc, _, xs) = family("r2-neg");
    let (doc, u) = insert(
        doc,
        Node::Union { members: Bodies::Spelled(vec![member(xs, -1)]), declare: Vec::new() },
    );
    let ev = crate::corpus::eval::<f64>(&doc);
    let e = err(&ev, u);
    println!("negative index: {e}");
    assert!(e.contains("InstanceOutOfRange"), "{e}");
}

/// Claim 3 / DM5 on indexed reads: `[xs[0], xs[0]]` glues; `[xs[0], xs[1]]`
/// are two members named apart.
#[test]
fn r2_a_repeated_indexed_read_glues() {
    let (doc, _, xs) = family("r2-rep-idx");
    let (doc, u) = insert(
        doc,
        Node::Union { members: Bodies::Spelled(vec![member(xs, 0), member(xs, 0)]), declare: Vec::new() },
    );
    let (doc, i) = insert(
        doc,
        Node::Intersect { members: Bodies::Spelled(vec![member(xs, 1), member(xs, 1)]), declare: Vec::new() },
    );
    let (doc, s) = insert(
        doc,
        Node::Subtract { from: member(xs, 2), tool: member(xs, 2), declare: Vec::new() },
    );
    let ev = crate::corpus::eval::<f64>(&doc);
    println!("u: {} i: {} s: {}", err(&ev, u), err(&ev, i), err(&ev, s));
    assert!((vol(&ev, u) - 1.0).abs() < 1e-9);
    assert!((vol(&ev, i) - 1.0).abs() < 1e-9);
    assert!(matches!(
        ev.value(s).map(|v| &v.payload),
        Some(editor_core::ValuePayload::Boolean(editor_core::BooleanValue::Empty))
    ));
}

/// Claim 2: no index slot at a family argument.
#[test]
fn r2_no_index_slot_at_a_family_argument() {
    let (doc, _, xs) = family("r2-fam-slot");
    let (doc, u) = insert(
        doc,
        Node::Union { members: Bodies::Family(xs.into()), declare: Vec::new() },
    );
    let r = editor_core::apply(
        &doc,
        &DocEdit::SetStructuralParam {
            node: u,
            slot: SlotId::Index { seat: OperandSlot::Members, k: 0 },
            expr: Formula::count(0),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    println!("index slot at Members: {:?}", r.as_ref().err());
    assert!(matches!(r, Err(EditError::UnknownSlot { .. })));
    // And SetMembers refuses an indexed family argument.
    let r = editor_core::apply(
        &doc,
        &DocEdit::SetMembers { node: u, members: Bodies::Family(member(xs, 0)) },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    println!("SetMembers indexed family: {:?}", r.as_ref().err());
    assert!(matches!(r, Err(EditError::IndexedRead { .. })));
    // A Bodies in a member place.
    let r = editor_core::apply(
        &doc,
        &DocEdit::SetMembers { node: u, members: Bodies::Spelled(vec![xs.into()]) },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    println!("SetMembers family in member place: {:?}", r.as_ref().err());
    assert!(matches!(r, Err(EditError::SlotVarKind { .. })));
}

/// Claim 2: the load door refuses what the insert door refuses.
#[test]
fn r2_the_load_door_refuses_bad_indices() {
    let (doc, cube, xs) = family("r2-load");
    let (doc, cut) = insert(
        doc,
        Node::Subtract { from: member(xs, 2), tool: member(xs, 0), declare: Vec::new() },
    );
    let _ = cut;
    let text = editor_core::persist::save(&doc, &[], Tol::witness()).unwrap();
    let xs_read = serde_json::to_value(out(&doc, xs)).unwrap();
    let cube_read = serde_json::to_value(out(&doc, cube)).unwrap();
    let xs_s = xs_read.as_str().unwrap();
    let cube_s = cube_read.as_str().unwrap();
    // Find the from seat's "at" list: one index var. Rank two: duplicate it.
    let v: serde_json::Value = serde_json::from_str(text.split_once('\n').map_or(&text[..], |(_, r)| r).trim_start_matches(|c| c != '{')).unwrap_or(serde_json::Value::Null);
    let _ = v;
    let at_line = text.lines().position(|l| l.contains("\"at\": [")).expect("an at list");
    let lines: Vec<&str> = text.lines().collect();
    let idx_line = lines[at_line + 1].trim().trim_end_matches(',').to_owned();
    println!("at list first entry: {idx_line}");
    // (a) rank two
    let mut rank2: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
    rank2.insert(at_line + 1, format!("{idx_line},"));
    let rank2 = rank2.join("\n") + "\n";
    let r = editor_core::persist::load(&rank2, Tol::witness());
    println!("load rank 2: {:?}", r.as_ref().err().map(ToString::to_string));
    assert!(r.is_err(), "a rank-two index loads");
    // (b) an indexed body (cube instead of xs as the read)
    let body_indexed = text.replacen(&format!("\"read\": \"{xs_s}\""), &format!("\"read\": \"{cube_s}\""), 1);
    assert_ne!(body_indexed, text, "rewrote the read");
    let r = editor_core::persist::load(&body_indexed, Tol::witness());
    println!("load indexed body: {:?}", r.as_ref().err().map(ToString::to_string));
    assert!(r.is_err(), "an indexed body loads");
    // (c) empty at
    let mut empty: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
    empty[at_line] = empty[at_line].replace("\"at\": [", "\"at\": [],\"zz\": [");
    let r = editor_core::persist::load(&(empty.join("\n") + "\n"), Tol::witness());
    println!("load junk at: {:?}", r.as_ref().err().map(ToString::to_string));
    assert!(r.is_err());
}

fn block(doc: ProfileDoc, (x0, x1): (f64, f64), (y0, y1): (f64, f64), h: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = crate::fixture::on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]],
    );
    insert(
        doc,
        Node::Extrude { profile: p.into(), distance: len(h), side: editor_core::ExtrudeSide::Along },
    )
}

/// DM4/DM5: a declared pair sited at a read spelled twice sites both of
/// its members (the work item's pinned row; no test in the PR pins it).
#[test]
fn r2_a_declared_pair_at_a_read_spelled_twice() {
    let doc = ProfileDoc::empty_derived("r2-decl-twice", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block(doc, (1.0, 2.0), (0.0, 1.0), 1.0);
    let ev = crate::corpus::eval::<f64>(&doc);
    let declare = editor_core::declared_pairs(
        &editor_core::find_flush_candidates(&ev, &doc, a, b, Tol::witness()).expect("findings"),
    );
    assert!(!declare.is_empty(), "a and b touch");
    let mut fails = Vec::new();
    let mut doc = doc;
    let mut nodes = Vec::new();
    for order in [[a, a, b], [a, b, a], [b, a, a]] {
        let (d, u) = insert(
            doc,
            Node::Union {
                members: Bodies::Spelled(order.iter().map(|&m| m.into()).collect()),
                declare: declare.clone(),
            },
        );
        doc = d;
        nodes.push((order, u));
    }
    // And undeclared: refuses UndeclaredContact in every order.
    let (d, und) = insert(
        doc,
        Node::Union { members: Bodies::Spelled(vec![a.into(), a.into(), b.into()]), declare: Vec::new() },
    );
    doc = d;
    let ev = crate::corpus::eval::<f64>(&doc);
    let tag = |m: RecipeNodeId| if m == a { "a" } else { "b" };
    for (order, u) in &nodes {
        let o: Vec<&str> = order.iter().map(|&m| tag(m)).collect();
        match ev.value(*u) {
            Some(_) => println!("{o:?}: builds, volume {}", vol(&ev, *u)),
            None => {
                println!("{o:?}: {}", err(&ev, *u));
                fails.push(o);
            }
        }
    }
    println!("undeclared [a, a, b]: {}", err(&ev, und));
    assert!(fails.is_empty(), "orders that refused: {fails:?}");
}

fn census(ev: &editor_core::Evaluation<f64>, id: RecipeNodeId) -> BTreeMap<&'static str, usize> {
    let mut m = BTreeMap::new();
    for v in ev.value(id).expect("built").verdicts.iter() {
        *m.entry(v.predicate).or_default() += 1;
    }
    m
}

/// Claim 5: where a third member covers a declared contact, the fold never
/// meets the contact, so the pairwise judgement is the only place it is
/// decided; the node's log is compared against the two-member union of
/// the contact pair, whose log is that pair's decisions.
#[test]
fn r2_a_covered_contact_leaves_the_log() {
    let doc = ProfileDoc::empty_derived("r2-covered", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, c) = block(doc, (1.0, 2.0), (0.0, 1.0), 1.0);
    // b covers the a|c contact face (x = 1, y,z in [0,1]) in its interior.
    let (doc, b0) = block(doc, (0.5, 1.5), (-1.0, 2.0), 3.0);
    // shift b down by 1 in z through a transform-free trick: make it taller
    // and start at z = 0; its floor z = 0 is coplanar with a's and c's, so
    // instead raise a and c? Keep it simple: b spans z in [0, 3] and the
    // contact face's bottom edge lies on b's floor; report what happens.
    let b = b0;
    let ev = crate::corpus::eval::<f64>(&doc);
    let mut declare = editor_core::declared_pairs(
        &editor_core::find_flush_candidates(&ev, &doc, a, c, Tol::witness()).expect("ac"),
    );
    declare.extend(editor_core::declared_pairs(
        &editor_core::find_flush_candidates(&ev, &doc, a, b, Tol::witness()).expect("ab"),
    ));
    declare.extend(editor_core::declared_pairs(
        &editor_core::find_flush_candidates(&ev, &doc, b, c, Tol::witness()).expect("bc"),
    ));
    let sp = |ms: &[RecipeNodeId]| Bodies::Spelled(ms.iter().map(|&m| m.into()).collect());
    let ac = editor_core::declared_pairs(
        &editor_core::find_flush_candidates(&ev, &doc, a, c, Tol::witness()).expect("ac"),
    );
    let (doc, pair) = insert(doc, Node::Union { members: sp(&[a, c]), declare: ac });
    let (doc, three) = insert(doc, Node::Union { members: sp(&[a, b, c]), declare: declare.clone() });
    let ev = crate::corpus::eval::<f64>(&doc);
    println!("pair: {}", err(&ev, pair));
    println!("three: {}", err(&ev, three));
    if ev.value(three).is_none() || ev.value(pair).is_none() {
        return;
    }
    let (p, t) = (census(&ev, pair), census(&ev, three));
    let total = |m: &BTreeMap<&str, usize>| m.values().sum::<usize>();
    println!("pair log: {} verdicts; three log: {} verdicts", total(&p), total(&t));
    for (k, n) in &p {
        let m = t.get(k).copied().unwrap_or(0);
        println!("  {k}: pair {n}, three {m}");
    }
}

/// Claim 6 / Q4: a stored name whose carry segment names a read the
/// document never minted (or the fold sentinel) — is it refused at the
/// load door, as the base refused a never-minted `FromMember` member?
#[test]
fn r2_the_load_door_holds_carry_reads_to_the_document() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/corpus/tour/die_composed_tour.pncad");
    let text = std::fs::read_to_string(path).unwrap();
    let is_epsilon = |line: &str| line.trim_start().starts_with("\"epsilon\":");
    let probe = editor_core::persist::save(
        &ProfileDoc::empty_derived("r2-eps", Tol::witness()),
        &[],
        Tol::witness(),
    )
    .unwrap();
    let wanted = probe.lines().find(|l| is_epsilon(l)).unwrap().to_owned();
    let text: String = text
        .lines()
        .map(|l| if is_epsilon(l) { wanted.as_str() } else { l })
        .map(|l| format!("{l}\n"))
        .collect();
    let base = editor_core::persist::load(&text, Tol::witness());
    println!("unmodified loads: {}", base.is_ok());
    assert!(base.is_ok(), "{:?}", base.err().map(|e| e.to_string()));
    for bogus in ["77777:00000000deadbeef", "0:0000000000000000"] {
        let edited = text.replace(
            "\"read\": \"29:9117322ce802f579\"",
            &format!("\"read\": \"{bogus}\""),
        );
        assert_ne!(edited, text);
        let r = editor_core::persist::load(&edited, Tol::witness());
        println!(
            "carry read {bogus}: {}",
            match &r {
                Ok(_) => "LOADS".to_owned(),
                Err(e) => format!("refused: {e}"),
            }
        );
    }
}

/// Claim 2/6: a subtract whose two seats read one family (`xs[1] − xs[0]`)
/// keys both seats' entities by the one read `xs`; the pair emitter sides a
/// seam vertex's incident edges by read (`emit_topo`), so overlapping
/// instances are the case to run. Reference: the same geometry with the
/// tool read through the prototype's own read (instance 0 is the
/// prototype unmoved).
#[test]
fn r2_a_subtract_of_two_members_of_one_family() {
    for (dir, label) in [([1.0, 1.0, 1.0], "diag"), ([1.0, 0.0, 0.0], "x")] {
        let (doc, cube) = cube_doc(&format!("r2-sub-fam-{label}"));
        let (doc, xs) = insert(doc, linear(cube, dir, 0.5, 2));
        let (doc, same) = insert(
            doc,
            Node::Subtract { from: member(xs, 1), tool: member(xs, 0), declare: Vec::new() },
        );
        let (doc, refr) = insert(
            doc,
            Node::Subtract { from: member(xs, 1), tool: cube.into(), declare: Vec::new() },
        );
        let (doc, uni) = insert(
            doc,
            Node::Union { members: Bodies::Spelled(vec![member(xs, 1), member(xs, 0)]), declare: Vec::new() },
        );
        let part = |doc: ProfileDoc, i: i64| {
            insert(
                doc,
                Node::Part { of: xs.into(), select: editor_core::PartSelect::Instance(Formula::count(i)) },
            )
        };
        let (doc, p1) = part(doc, 1);
        let (doc, p0) = part(doc, 0);
        let (doc, parts) = insert(
            doc,
            Node::Subtract { from: p1.into(), tool: p0.into(), declare: Vec::new() },
        );
        let (doc, same_rev) = insert(
            doc,
            Node::Subtract { from: member(xs, 0), tool: member(xs, 1), declare: Vec::new() },
        );
        let ev = crate::corpus::eval::<f64>(&doc);
        for (what, id) in [("xs[1] - xs[0]", same), ("xs[0] - xs[1]", same_rev), ("xs[1] - cube", refr), ("Part(xs,1) - Part(xs,0)", parts), ("[xs[1], xs[0]]", uni)] {
            match ev.value(id) {
                Some(_) => println!("{label}: {what}: builds, volume {:.6}, {} names", vol(&ev, id), ev.value(id).unwrap().name_table.iter().count()),
                None => println!("{label}: {what}: {}", err(&ev, id)),
            }
        }
    }
}
