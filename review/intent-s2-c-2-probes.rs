// Second review of #4359 (head 23bad579f): probes appended to
// crates/editor-core/tests/intent_s2_c_world.rs to run. Not part of any build.

// ---- REVIEW PROBES (second reviewer, local only) ----

fn reader_digest(doc: &ProfileDoc, opts: &editor_core::EvalOptions, id: RecipeNodeId) -> (u64, usize) {
    let run = crate::fixture::run(doc, opts);
    match &run.value(id).expect("the reader evaluates").payload {
        editor_core::ValuePayload::Body(b) => body_digest(b),
        other => panic!("not one body: {other:?}"),
    }
}

/// Split of a cut whose placement is POSED: the remainder reader of the
/// placed body is re-pointed to the instance's body, which is the part's
/// world (the body at the pose). Does the reader's geometry move?
#[test]
fn probe_split_posed_placement_moves_a_repointed_reader() {
    let pose: editor_core::Placement<editor_core::Formula> = editor_core::Step::Rigid {
        translation: [0.0, 0.0, 2.0].map(len),
        axis: [0.0, 0.0, 1.0].map(crate::fixture::scl),
        angle: crate::fixture::ang(0.0),
    }
    .into();
    let doc = ProfileDoc::empty_derived("probe-split-posed", Tol::witness());
    let (doc, profile) = on_frame(doc, [0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![square(0.0, 0.0, 0.5)]);
    let frame = doc.ids()[0];
    let (doc, b) = insert(doc, Node::Extrude { profile: profile.into(), distance: len(1.0), side: ExtrudeSide::Along });
    let (doc, p) = crate::fixture::step(doc, DocEdit::place(b, Some(pose)));
    let p = p.unwrap();
    let (doc, reader) = insert(doc, crate::fixture::xform(b, [5.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, _) = place(doc, reader);
    let before = reader_digest(&doc, &editor_core::EvalOptions::default(), reader);
    let out = editor_core::split(&doc, &BTreeSet::from([frame, profile, b, p]), DocumentId::derive("probe-split-posed-part"), Tol::witness(), None);
    match out {
        Err(e) => eprintln!("PROBE split refused: {e:?}"),
        Ok(out) => {
            let mut store = PartStore::default();
            store.insert(out.part.clone(), Tol::witness());
            let opts = crate::fixture::resolver::with_resolver(store);
            let after = reader_digest(&out.remainder, &opts, reader);
            eprintln!("PROBE posed: before {before:?} after {after:?}");
            assert_eq!(before, after, "split moved the remainder reader's geometry (posed cut placement)");
        }
    }
}

/// Split of a cut placing TWO bodies at the identity: the remainder reader
/// of one is re-pointed to the instance's body, the part's whole product.
#[test]
fn probe_split_two_placed_bodies_widen_a_repointed_reader() {
    let doc = ProfileDoc::empty_derived("probe-split-two", Tol::witness());
    let (doc, profile) = on_frame(doc, [0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![square(0.0, 0.0, 0.5)]);
    let frame = doc.ids()[0];
    let (doc, b) = insert(doc, Node::Extrude { profile: profile.into(), distance: len(1.0), side: ExtrudeSide::Along });
    let (doc, p) = place(doc, b);
    let (doc, profile2) = on_frame(doc, [0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![square(3.0, 0.0, 0.5)]);
    let frame2 = doc.ids()[doc.ids().len() - 2];
    let (doc, c) = insert(doc, Node::Extrude { profile: profile2.into(), distance: len(1.0), side: ExtrudeSide::Along });
    let (doc, pc) = place(doc, c);
    let (doc, reader) = insert(doc, crate::fixture::xform(b, [10.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, _) = place(doc, reader);
    let before = reader_digest(&doc, &editor_core::EvalOptions::default(), reader);
    let out = editor_core::split(&doc, &BTreeSet::from([frame, profile, b, p, frame2, profile2, c, pc]), DocumentId::derive("probe-split-two-part"), Tol::witness(), None);
    match out {
        Err(e) => eprintln!("PROBE split refused: {e:?}"),
        Ok(out) => {
            let mut store = PartStore::default();
            store.insert(out.part.clone(), Tol::witness());
            let opts = crate::fixture::resolver::with_resolver(store);
            let after = reader_digest(&out.remainder, &opts, reader);
            eprintln!("PROBE two: before {before:?} after {after:?}");
            assert_eq!(before, after, "split widened the remainder reader to the part's whole world");
        }
    }
}

/// A union's member list reading a copy: refused like a scalar slot?
#[test]
fn probe_a_union_member_list_cannot_read_a_copy() {
    let doc = ProfileDoc::empty_derived("probe-union-copy", Tol::witness());
    let (doc, a) = block(doc, 0.0);
    let (doc, b) = block(doc, 0.75);
    let (doc, placement) = place(doc, a);
    let copy = doc.output(placement, 0).unwrap();
    let r = doc.apply(
        &DocEdit::InsertNode {
            node: Box::new(Node::Union { members: vec![copy.into(), b.into()], declare: Vec::new() }),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    eprintln!("PROBE union: {:?}", r.as_ref().map(|_| "accepted").map_err(|e| e.to_string()));
    assert!(matches!(r, Err(editor_core::EditError::ReadsWorldCopy { .. })));
}

/// A measure whose sites are read AT a placement (the world copy), and
/// whether its value moves with the pose. A measure is no construction,
/// but nothing besides the gather and export is meant to read a pose.
#[test]
fn probe_a_measure_sited_at_a_placement() {
    let mk = |dz: f64| {
        let pose: editor_core::Placement<editor_core::Formula> = editor_core::Step::Rigid {
            translation: [0.0, 0.0, dz].map(len),
            axis: [0.0, 0.0, 1.0].map(crate::fixture::scl),
            angle: crate::fixture::ang(0.0),
        }
        .into();
        let doc = ProfileDoc::empty_derived("probe-measure-copy", Tol::witness());
        let (doc, a) = block(doc, 0.0);
        let (doc, b) = block(doc, 3.0);
        let (doc, p) = crate::fixture::step(doc, DocEdit::place(a, Some(pose)));
        let p = p.unwrap();
        let (doc, pb) = place(doc, b);
        let r = doc.apply(
            &DocEdit::InsertNode {
                node: Box::new(Node::measure(
                    MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
                    vec![
                        SitedRef::new(p, cap(a, CapEnd::End).in_copy(p)),
                        SitedRef::new(pb, cap(b, CapEnd::End).in_copy(pb)),
                    ],
                ).unwrap()),
                fresh: Vec::new(),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        );
        match r {
            Err(e) => { eprintln!("PROBE measure at copy refused at door: {e}"); None }
            Ok(applied) => {
                let doc = applied.doc;
                let m = *doc.ids().last().unwrap();
                let run = ev(&doc);
                let v = format!("{:?}", run.result(m).map(|r| match r { NodeResult::Ok(v) => format!("{:?}", v.payload), other => format!("{other:?}") }));
                eprintln!("PROBE measure at copy dz={dz}: {}", &v[..v.len().min(300)]);
                Some(v)
            }
        }
    };
    let (x, y) = (mk(0.0), mk(2.0));
    eprintln!("PROBE measure moved with pose: {}", x != y);
}

/// Inline of an instance the host places TWICE at the identity: do the
/// two copies survive?
#[test]
fn probe_inline_of_an_instance_placed_twice_keeps_both_copies() {
    let part = {
        let doc = ProfileDoc::empty(DocumentId::derive("probe-inline-twice-part"), Tol::witness());
        let (doc, body) = block(doc, 0.0);
        place(doc, body).0
    };
    let mut store = PartStore::default();
    let part_ref = store.insert(part, Tol::witness());
    let host = ProfileDoc::empty(DocumentId::derive("probe-inline-twice"), Tol::witness());
    let (host, instance) = insert(host, Node::instantiate_part(part_ref));
    let (host, _) = place(host, instance);
    let (host, _) = place(host, instance);
    let opts = crate::fixture::resolver::with_resolver(store.clone());
    let before = crate::fixture::run(&host, &opts);
    let b = product(&host, &before, Tol::witness()).expect("gathers");
    let resolver: std::sync::Arc<dyn editor_core::PartResolver> = std::sync::Arc::new(store);
    match editor_core::inline(&host, instance, &resolver, Tol::witness()) {
        Err(e) => eprintln!("PROBE inline twice refused: {e}"),
        Ok(inlined) => {
            let after = ev(&inlined.doc);
            let a = product(&inlined.doc, &after, Tol::witness()).expect("gathers");
            eprintln!("PROBE inline twice: placements before 2 after {}; digest before {:?} after {:?}", inlined.doc.placements().len(), body_digest(&b), body_digest(&a));
            assert_eq!(inlined.doc.placements().len(), 2, "inline dropped a copy");
        }
    }
}
