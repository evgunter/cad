//! **Review probes for the binder kernel (PR 7506, lane A)**: rows that
//! go red when containment, the one-body kind refusal or the binder's
//! persist rules leak.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::block;
use crate::fixture::{self, insert};
use editor_core::{Bodies, BodyRead, DocEdit, Formula, Node, Operand, ProfileDoc, Spell, VarRef};
use geom_core::Tol;

fn try_apply(
    doc: &ProfileDoc,
    edit: DocEdit<editor_core::ProfileProgram>,
) -> Result<ProfileDoc, editor_core::EditError> {
    doc.apply(&edit, Tol::witness()).map(|a| a.doc)
}

/// The ring of `g_index_families` (yielded by the fixture's `insert`),
/// plus a lone block placed in the world; the product the fixture gives.
fn ring_and_lone() -> (
    ProfileDoc,
    editor_core::VarId,
    editor_core::RecipeNodeId,
    editor_core::RecipeNodeId,
) {
    let (doc, _block, ring) = crate::g_index_families::ring(4);
    let k = doc.var_named("k").unwrap();
    let (doc, lone) = block(doc, (-9.0, -8.0), (-0.5, 0.5), 1.0);
    let (doc, lone_copy) = fixture::place(doc, lone);
    let _ = lone_copy;
    (doc, k, ring, lone)
}

/// Containment at the product, through a later edit: the product reads a
/// union of one lone copy; `SetMembers` then adds `ring[k]`, so the
/// union reads `k` and is no yield. The product now reads a k-reader
/// outside its binder.
#[test]
fn containment_a_set_members_cannot_leave_the_product_on_a_k_reader() {
    let (doc, k, ring, lone) = ring_and_lone();
    eprintln!("yields of k: {:?}", doc.yields(k));
    let lone_copy = fixture::newest(&doc);
    let _ = lone;
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: Bodies::spelled(vec![BodyRead::plain(Operand::Node(lone_copy))]),
        },
    );
    eprintln!(
        "union reach after insert: {:?}; yields {:?}",
        doc.node_reach(u),
        doc.yields(k)
    );
    let doc = try_apply(
        &doc,
        DocEdit::SetProduct {
            product: Some(Operand::Node(u)),
        },
    )
    .expect("the product reads the union");
    let edit = DocEdit::SetMembers {
        node: u,
        members: Bodies::spelled(vec![
            BodyRead::plain(Operand::Node(lone_copy)),
            BodyRead::indexed(
                Operand::Node(ring),
                vec![Formula::var(k, editor_core::Dimension::Count)],
            ),
        ]),
    };
    match try_apply(&doc, edit) {
        Err(e) => eprintln!("REFUSED: {e:?}"),
        Ok(after) => {
            eprintln!(
                "ACCEPTED; union reach {:?}; yielding {:?}; product {:?}",
                after.node_reach(u),
                after.yielding(after.output(u, 0).unwrap()),
                after.product()
            );
            let ev = fixture::run(&after, &Default::default());
            eprintln!(
                "union result: {:?}",
                ev.result(u)
                    .map(|r| format!("{r:?}").chars().take(300).collect::<String>())
            );
            eprintln!(
                "product: {:?}",
                editor_core::product_recorded(&after, &ev, Tol::witness())
                    .map(|p| p.solid_copies.len())
                    .map_err(|e| format!("{e:?}").chars().take(300).collect::<String>())
            );
            let text = editor_core::persist::save(&after, Tol::witness()).unwrap();
            match editor_core::persist::load(&text, Tol::witness()) {
                Ok(_) => eprintln!("LOAD ACCEPTED"),
                Err(e) => eprintln!("LOAD REFUSED {e:?}"),
            }
            panic!("containment leak: a product reads a k-reader no binder yields");
        }
    }
}

/// Containment at the product, by un-yielding: the product reads the
/// ring's copy, which the binder yields; `SetYields` then drops it in
/// favour of another k-reader.
#[test]
fn containment_set_yields_cannot_leave_the_product_on_a_k_reader() {
    let (doc, k, ring, _lone) = ring_and_lone();
    let doc = try_apply(
        &doc,
        DocEdit::SetProduct {
            product: Some(Operand::Node(ring)),
        },
    );
    let doc = match doc {
        Ok(d) => d,
        Err(e) => panic!("product of the ring refused: {e:?}"),
    };
    // Drop the fixture's world assemble, which splices the ring.
    let assembles: Vec<_> = doc
        .ids()
        .into_iter()
        .filter(|&id| matches!(doc.node(id), Some(Node::Assemble { .. })))
        .collect();
    let doc = assembles.into_iter().fold(doc, |doc, id| {
        try_apply(&doc, DocEdit::DeleteNode { id }).unwrap_or_else(|e| panic!("delete: {e:?}"))
    });
    // A second k-reader to yield instead: a row of the lone block.
    let lone = _lone;
    let slide = Formula::mul(fixture::scalar_of(k), fixture::len(2.0)).unwrap();
    let (doc, other) = fixture::family_of(
        doc,
        lone,
        vec![(editor_core::Coord::slide(editor_core::TargetAxis::X), slide)],
    );
    let other_out = doc.output(other, 0).unwrap();
    eprintln!("yields now {:?}", doc.yields(k));
    let edit = DocEdit::SetYields {
        binder: VarRef::from(k),
        yields: vec![VarRef::from(other_out)],
    };
    match try_apply(&doc, edit) {
        Err(e) => eprintln!("REFUSED: {e:?}"),
        Ok(after) => {
            let ev = fixture::run(&after, &Default::default());
            eprintln!(
                "product: {:?}",
                editor_core::product_recorded(&after, &ev, Tol::witness())
                    .map(|p| p.solid_copies.len())
                    .map_err(|e| format!("{e:?}").chars().take(300).collect::<String>())
            );
            panic!("containment leak: SetYields left the product on a k-reader");
        }
    }
}

/// `Spell` round-trip sanity for the probe file's imports.
#[allow(dead_code)]
fn _spell(_: Spell<BodyRead<Formula>>) {}

/// A body, its row of three under a binder (the fixture yields it), and
/// the row's index.
fn body_and_row() -> (
    ProfileDoc,
    editor_core::RecipeNodeId,
    editor_core::RecipeNodeId,
    editor_core::VarId,
) {
    use crate::fixture::{desc, len, square};
    let doc = ProfileDoc::empty_derived("review-binders-a", Tol::witness());
    let (doc, profile) = insert(doc, Node::Profile(desc(vec![square(0.0, 0.0, 1.0)])));
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    let (doc, k, row) = fixture::linear(doc, body, [3.0, 0.0, 0.0], 3.0, 3);
    (doc, body, row, k)
}

fn insert_raw(
    doc: &ProfileDoc,
    node: editor_core::AuthoredNode,
) -> Result<ProfileDoc, editor_core::EditError> {
    try_apply(
        doc,
        DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        },
    )
}

/// **Every one-body seat handed the row whole refuses by kind** at the
/// door (found `Bodies`), and `xs[k]` at the same seat is admitted.
#[test]
fn every_one_body_seat_refuses_a_family_at_the_door() {
    use crate::fixture::{ename, fname, len, vpiece};
    use editor_core::{CapEnd, RoleSeg};
    let (doc, body, row, k) = body_and_row();
    assert!(
        doc.yielding(doc.output(row, 0).unwrap()).is_some(),
        "the row is a yield"
    );
    let kf = || vec![Formula::var(k, editor_core::Dimension::Count)];
    let whole = || BodyRead::plain(Operand::Node(row));
    let member = || BodyRead::indexed(Operand::Node(row), kf());
    let plane = || {
        editor_core::pose::RoleRead::<Operand>::plain(fixture::plane(Operand::select(
            body,
            vec![fname(body, RoleSeg::Cap(CapEnd::Start))],
        )))
    };
    let edge = fixture::ename(body, RoleSeg::LateralEdge(vpiece(&doc, body, 0, 0)));
    let cap = fname(body, RoleSeg::Cap(CapEnd::End));
    let _ = ename;
    let sel = |read: BodyRead<Formula>, n: editor_core::StableName| Operand::Select {
        body: Box::new(read),
        names: vec![n],
    };
    type Mk<'a> = Box<dyn Fn(BodyRead<Formula>) -> editor_core::AuthoredNode + 'a>;
    let seats: Vec<(&str, Mk)> = vec![
        (
            "split target",
            Box::new(|r| Node::Split {
                target: r,
                tool: plane(),
            }),
        ),
        (
            "mirror body",
            Box::new(|r| Node::Mirror {
                body: r,
                plane: plane(),
            }),
        ),
        (
            "subtract from",
            Box::new(|r| Node::Subtract {
                from: r,
                tool: BodyRead::plain(Operand::Node(body)).into(),
            }),
        ),
        ("placement source", Box::new(fixture::stood)),
        (
            "fillet selection",
            Box::new(|r| Node::Fillet {
                selection: sel(r, edge.clone()),
                radius: len(0.05),
            }),
        ),
        (
            "chamfer selection",
            Box::new(|r| Node::Chamfer {
                selection: sel(r, edge.clone()),
                distance: len(0.05),
            }),
        ),
        (
            "shell open",
            Box::new(|r| Node::Shell {
                open: sel(r, cap.clone()),
                thickness: len(0.1),
            }),
        ),
    ];
    let mut leaks = Vec::new();
    for (what, mk) in &seats {
        match insert_raw(&doc, mk(whole())) {
            Err(e) => eprintln!(
                "{what}: whole REFUSED {}",
                format!("{e:?}").chars().take(260).collect::<String>()
            ),
            Ok(after) => {
                let id = fixture::newest(&after);
                eprintln!("{what}: whole ACCEPTED, reach {:?}", after.node_reach(id));
                leaks.push(*what);
            }
        }
        match insert_raw(&doc, mk(member())) {
            Err(e) => eprintln!(
                "{what}: member REFUSED {}",
                format!("{e:?}").chars().take(260).collect::<String>()
            ),
            Ok(_) => eprintln!("{what}: member ok"),
        }
    }
    assert!(
        leaks.is_empty(),
        "one-body seats taking a family whole: {leaks:?}"
    );
}

/// **A hand-written file whose one-body seat reads a family whole
/// refuses at load**: each seat written `xs[k]`, saved, then the
/// member's index list emptied on the wire.
#[test]
fn every_one_body_seat_refuses_a_family_at_load() {
    use crate::fixture::{ename, fname, len, vpiece};
    use editor_core::{CapEnd, RoleSeg};
    let (doc, body, row, k) = body_and_row();
    let kf = || vec![Formula::var(k, editor_core::Dimension::Count)];
    let member = || BodyRead::indexed(Operand::Node(row), kf());
    let plane = || {
        editor_core::pose::RoleRead::<Operand>::plain(fixture::plane(Operand::select(
            body,
            vec![fname(body, RoleSeg::Cap(CapEnd::Start))],
        )))
    };
    let edge = ename(body, RoleSeg::LateralEdge(vpiece(&doc, body, 0, 0)));
    let cap = fname(body, RoleSeg::Cap(CapEnd::End));
    let sel = |read: BodyRead<Formula>, n: editor_core::StableName| Operand::Select {
        body: Box::new(read),
        names: vec![n],
    };
    type Mk<'a> = Box<dyn Fn(BodyRead<Formula>) -> editor_core::AuthoredNode + 'a>;
    let seats: Vec<(&str, Mk)> = vec![
        (
            "split target",
            Box::new(|r| Node::Split {
                target: r,
                tool: plane(),
            }),
        ),
        (
            "mirror body",
            Box::new(|r| Node::Mirror {
                body: r,
                plane: plane(),
            }),
        ),
        (
            "subtract from",
            Box::new(|r| Node::Subtract {
                from: r,
                tool: BodyRead::plain(Operand::Node(body)).into(),
            }),
        ),
        ("placement source", Box::new(fixture::stood)),
        (
            "fillet selection",
            Box::new(|r| Node::Fillet {
                selection: sel(r, edge.clone()),
                radius: len(0.05),
            }),
        ),
        (
            "chamfer selection",
            Box::new(|r| Node::Chamfer {
                selection: sel(r, edge.clone()),
                distance: len(0.05),
            }),
        ),
        (
            "shell open",
            Box::new(|r| Node::Shell {
                open: sel(r, cap.clone()),
                thickness: len(0.1),
            }),
        ),
    ];
    let split = |text: String| {
        let (head, body) = text.split_once('\n').unwrap();
        (
            head.to_string(),
            serde_json::from_str::<serde_json::Value>(body).unwrap(),
        )
    };
    let (_, base) = split(editor_core::persist::save(&doc, Tol::witness()).unwrap());
    let mut leaks = Vec::new();
    for (what, mk) in &seats {
        let after = insert_raw(&doc, mk(member())).unwrap_or_else(|e| panic!("{what}: {e:?}"));
        let text = editor_core::persist::save(&after, Tol::witness()).unwrap();
        let (head, mut v) = split(text);
        // Every `at` list naming k, emptied, in what this insert added.
        let kj = serde_json::to_value(k).unwrap();
        let mut hits = 0;
        fn bare(x: &mut serde_json::Value, kj: &serde_json::Value, hits: &mut usize) -> bool {
            if let serde_json::Value::Object(m) = x
                && let (Some(read), Some(serde_json::Value::Array(at))) =
                    (m.get("read"), m.get("at"))
                && at.len() == 1
                && at[0] == *kj
            {
                let read = read.clone();
                *x = read;
                *hits += 1;
                return true;
            }
            false
        }
        fn walk(v: &mut serde_json::Value, kj: &serde_json::Value, hits: &mut usize) {
            if bare(v, kj, hits) {
                return;
            }
            match v {
                serde_json::Value::Object(map) => {
                    for (_, x) in map.iter_mut() {
                        walk(x, kj, hits);
                    }
                }
                serde_json::Value::Array(xs) => xs.iter_mut().for_each(|x| walk(x, kj, hits)),
                _ => {}
            }
        }
        walk(&mut v, &kj, &mut hits);
        if hits == 0 {
            eprintln!("{what}: no `at` found; vars diff:");
            if let (Some(a), Some(b)) = (v.get("vars"), base.get("vars")) {
                eprintln!(
                    "{}",
                    serde_json::to_string(a).unwrap().len() as i64
                        - serde_json::to_string(b).unwrap().len() as i64
                );
            }
            let s = serde_json::to_string(&v).unwrap();
            let kk = serde_json::to_string(&kj).unwrap();
            for (i, _) in s.match_indices(&kk).take(6) {
                eprintln!("   …{}…", &s[i.saturating_sub(120)..(i + 60).min(s.len())]);
            }
            continue;
        }
        match editor_core::persist::load(
            &format!("{head}\n{}\n", serde_json::to_string(&v).unwrap()),
            Tol::witness(),
        ) {
            Err(e) => eprintln!(
                "{what}: load REFUSED ({hits} at) {}",
                format!("{e:?}").chars().take(260).collect::<String>()
            ),
            Ok(loaded) => {
                let ev = fixture::run(&loaded, &Default::default());
                let id = fixture::newest(&loaded);
                let result = ev
                    .result(id)
                    .map(|r| format!("{r:?}").chars().take(200).collect::<String>());
                eprintln!("{what}: load ACCEPTED; eval {result:?}");
                leaks.push(*what);
            }
        }
    }
    assert!(
        leaks.is_empty(),
        "loads with a family at a one-body seat: {leaks:?}"
    );
}

/// Claim 1, through a variable chain: the product reads a block whose
/// depth is a named length `d`; `d` is redefined to read `k`.
#[test]
fn containment_a_redefinition_cannot_make_the_products_body_a_k_reader() {
    use crate::fixture::{desc, square};
    let doc = ProfileDoc::empty_derived("review-binders-a-chain", Tol::witness());
    let (doc, k) = fixture::index(doc, "k", 3);
    let doc = fixture::step(
        doc,
        DocEdit::DeclareVar {
            name: editor_core::VarName::new("d").unwrap(),
            def: editor_core::VarDecl::Defined(fixture::len(1.0)),
        },
    )
    .0;
    let d = doc.var_named("d").unwrap();
    let (doc, profile) = insert(doc, Node::Profile(desc(vec![square(0.0, 0.0, 1.0)])));
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: Formula::var(d, editor_core::Dimension::Length),
        },
    );
    // A yield so the binder is not empty: a row of the block.
    let (doc, _k2, _row) = (doc, k, ());
    let slide = Formula::mul(fixture::scalar_of(k), fixture::len(3.0)).unwrap();
    let (doc, row) = fixture::family_of(
        doc,
        body,
        vec![(editor_core::Coord::slide(editor_core::TargetAxis::X), slide)],
    );
    let _ = row;
    let (doc, copy) = fixture::place(doc, body);
    let doc = try_apply(
        &doc,
        DocEdit::SetProduct {
            product: Some(Operand::Node(copy)),
        },
    )
    .unwrap_or_else(|e| panic!("product: {e:?}"));
    let redefine = DocEdit::DefineVar {
        var: VarRef::from(d),
        def: editor_core::VarDecl::Defined(
            Formula::add(
                fixture::len(1.0),
                Formula::mul(fixture::scalar_of(k), fixture::len(0.5)).unwrap(),
            )
            .unwrap(),
        ),
        fresh: Vec::new(),
    };
    match try_apply(&doc, redefine) {
        Err(e) => eprintln!(
            "REFUSED {}",
            format!("{e:?}").chars().take(300).collect::<String>()
        ),
        Ok(after) => {
            let ev = fixture::run(&after, &Default::default());
            eprintln!(
                "ACCEPTED; copy reach {:?}; product gathers {:?}",
                after.node_reach(copy),
                editor_core::product_recorded(&after, &ev, Tol::witness())
                    .map(|p| p.solid_copies.len())
                    .map_err(|e| format!("{e:?}").chars().take(300).collect::<String>())
            );
            panic!("containment leak: a redefinition made the product's body a k-reader");
        }
    }
}

/// Claim 3: an orphan k-reader (inside the binder, read by nothing, no
/// yield) is not refused, and the unused check reports it.
#[test]
fn an_orphan_k_reader_is_admitted_and_reported_unused() {
    use crate::fixture::{fname, len};
    use editor_core::{CapEnd, RoleSeg};
    let (doc, body, row, k) = body_and_row();
    let plane = editor_core::pose::RoleRead::<Operand>::plain(fixture::plane(Operand::select(
        body,
        vec![fname(body, RoleSeg::Cap(CapEnd::Start))],
    )));
    let _ = len;
    let doc = insert_raw(
        &doc,
        Node::Mirror {
            body: BodyRead::indexed(
                Operand::Node(row),
                vec![Formula::var(k, editor_core::Dimension::Count)],
            ),
            plane,
        },
    )
    .expect("an orphan k-reader is admitted");
    let orphan = fixture::newest(&doc);
    let ev = fixture::run(&doc, &Default::default());
    eprintln!("orphan result ok: {:?}", ev.value(orphan).is_some());
    let unused = doc.unused();
    eprintln!("unused ops {:?}", unused.operations);
    assert!(
        unused.operations.contains(&orphan),
        "the orphan is reported unused"
    );
}

/// Claim 4: `xs[i]` of a grid is row `i` (DESIGN `[repetition]`).
#[test]
#[ignore = "xs[i] of a grid waits on work/refs/an-indexed-read-of-rank-two-has-no-family.md"]
fn a_grid_read_at_its_outer_index_is_a_row() {
    use editor_core::{Coord, TargetAxis};
    let doc = ProfileDoc::empty_derived("review-grid", Tol::witness());
    let (doc, i) = fixture::index(doc, "i", 3);
    let (doc, j) = fixture::index_of(doc, "j", Formula::count(2), Some(i));
    let (doc, cell) = crate::g_index_families::unit_block(doc);
    let slide = |index| Formula::mul(fixture::scalar_of(index), fixture::len(1.0)).unwrap();
    let (doc, grid) = fixture::family_of(
        doc,
        cell,
        vec![
            (Coord::slide(TargetAxis::X), slide(i)),
            (Coord::slide(TargetAxis::Y), slide(j)),
        ],
    );
    assert_eq!(doc.binder_indices(i), vec![i, j], "one binder, i outer");
    let row1 = BodyRead::indexed(Operand::Node(grid), vec![Formula::count(1)]);
    let mut fails = Vec::new();
    for (what, members) in [
        (
            "[…grid[1]]",
            Bodies::Spelled(vec![Spell::All(row1.clone())]),
        ),
        ("[grid[1]]", Bodies::Spelled(vec![Spell::One(row1.clone())])),
        (
            "[grid[1, 0]]",
            Bodies::spelled(vec![BodyRead::indexed(
                Operand::Node(grid),
                vec![Formula::count(1), Formula::count(0)],
            )]),
        ),
    ] {
        match insert_raw(&doc, Node::Union { members }) {
            Err(e) => {
                eprintln!(
                    "{what}: REFUSED {}",
                    format!("{e}").chars().take(300).collect::<String>()
                );
                fails.push(what);
            }
            Ok(after) => {
                let id = fixture::newest(&after);
                let ev = fixture::run(&after, &Default::default());
                eprintln!(
                    "{what}: ok, reach {:?}, value {:?}",
                    after.node_reach(id),
                    ev.value(id).map(|v| v.payload.kind_name())
                );
            }
        }
    }
    assert!(
        !fails.contains(&"[…grid[1]]"),
        "a row of a grid splices: {fails:?}"
    );
}

/// Claim 5: a file written before binders, `index(N)` as
/// `{"count": …}` with no seat, loads to what?
#[test]
fn an_index_written_before_binders_refuses_at_load() {
    let (doc, _body, _row, k) = body_and_row();
    let text = editor_core::persist::save(&doc, Tol::witness()).unwrap();
    let (head, body) = text.split_once('\n').unwrap();
    let mut v: serde_json::Value = serde_json::from_str(body).unwrap();
    let kj = serde_json::to_value(k).unwrap();
    // Find the index's definition and strip its seat.
    let mut found = 0;
    fn walk(v: &mut serde_json::Value, found: &mut usize) {
        match v {
            serde_json::Value::Object(m) => {
                if m.contains_key("count") && m.contains_key("seat") {
                    m.remove("seat");
                    *found += 1;
                }
                m.iter_mut().for_each(|(_, x)| walk(x, found));
            }
            serde_json::Value::Array(xs) => xs.iter_mut().for_each(|x| walk(x, found)),
            _ => {}
        }
    }
    walk(&mut v, &mut found);
    let _ = kj;
    eprintln!("stripped {found} seats");
    let loaded = editor_core::persist::load(
        &format!("{head}\n{}\n", serde_json::to_string(&v).unwrap()),
        Tol::witness(),
    );
    assert_eq!(found, 1, "the document holds one index");
    match loaded {
        Err(err @ editor_core::persist::PersistError::IndexBeforeBinders { .. }) => assert!(
            err.to_string().contains("rewrite its pattern as a binder"),
            "the refusal says how to rewrite it: {err}"
        ),
        other => panic!(
            "an index without a seat refuses typed, got {:?}",
            other.map(|_| "a document")
        ),
    }
}

/// Claim 5: binders save and load byte-equal, a grid's two indices and
/// its yields included.
#[test]
fn binders_round_trip_byte_equal() {
    let (doc, _body, _row, _k) = body_and_row();
    let a = editor_core::persist::save(&doc, Tol::witness()).unwrap();
    let b = editor_core::persist::save(
        &editor_core::persist::load(&a, Tol::witness()).unwrap(),
        Tol::witness(),
    )
    .unwrap();
    assert_eq!(a, b);
    assert!(
        a.contains("Binder") && a.contains("yields"),
        "{}",
        &a[..a.len().min(400)]
    );
}
