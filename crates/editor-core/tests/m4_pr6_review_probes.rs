//! The M4 PR 6 review's adversarial probe suite, ADOPTED as pins
//! (fix pass): the three attacks that landed (MAJOR-1 all-ones NaN,
//! MAJOR-2 duplicate map keys, NOTE-5 header lenience) now assert the
//! FIXED doors; the nine that held stay verbatim as regression pins.
//! Reviewer-authored (pr6-review-probe.rs); adapted with per-map
//! duplicate pins and DescToken structure/data non-aliasing pins.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    Attr, AttrKind, BooleanOp, BranchCertification, CancelToken, Dimension, DocEdit, DocParam,
    EntityKind, EvalOptions, Expr, ExprPath, MetaValue, Node, ParamName, PersistError, ProfileDoc,
    ProfileProgram, RecipeNodeId, Rgba8, RoleSeg, SCHEMA_VERSION, SlotId, StableName, WitnessDatum,
    apply, evaluate, load, save,
};
use fixture::{desc, insert, len};
use geom_core::Tol;

fn small() -> (ProfileDoc, String) {
    let doc = ProfileDoc::empty_derived("m4_pr6_review_probes", Tol::witness());
    let (doc, p) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        )),
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(1.0),
        },
    );
    let doc = apply(
        &doc,
        &DocEdit::SetDocParam {
            name: ParamName::new("q"),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 2.5,
            },
        },
        Tol::witness(),
    )
    .unwrap()
    .doc;
    let text = save(&doc, &[], Tol::witness()).unwrap();
    (doc, text)
}

/// ATTACK 1: the save-door NaN walk skips bits == u64::MAX (the
/// float_bits loop marker), but 0xFFFF_FFFF_FFFF_FFFF is itself a real
/// NaN and the payload is pub. Does save accept it? (v4: the payload's
/// only RAW floats are the 12 plane placement values — program args
/// are Exprs whose literal door refuses non-finite at construction, so
/// the attack surface SHRANK to the placement.)
#[test]
fn attack_all_ones_nan_slips_save_door() {
    let doc = ProfileDoc::empty_derived("m4_pr6_review_probes", Tol::witness());
    let mut d = desc(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (0.5, 1.0)]],
    );
    d.plane.placement.translation.y = f64::from_bits(u64::MAX); // a NaN
    let (doc, _) = insert(doc, Node::Profile(d));
    match save(&doc, &[], Tol::witness()) {
        Err(PersistError::NonFinite {
            site: editor_core::persist::NonFiniteSite::Profile { node, index },
        }) => {
            // Placement floats in column order: c0, c1, c2, then
            // translation (x, y, z) — translation.y = index 10.
            assert_eq!(node, RecipeNodeId(0));
            assert_eq!(index, 10);
        }
        other => panic!("all-ones NaN must refuse typed at save, got {other:?}"),
    }
}

/// MAJOR-1's structural fix, restated at the v4 substrate: loop shape
/// is STRUCTURE (the program enum), so no float value can alias a
/// loop boundary — and the former NaN-payload attack is now refused
/// one layer EARLIER, at the Expr literal door, so the sentinel class
/// cannot even be built into program data.
#[test]
fn tokens_separate_structure_from_data() {
    let quad = |loops: Vec<Vec<(f64, f64)>>| {
        desc([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], loops)
    };
    // Loop shape stays structure-distinct: (2 loops of 1 point) vs
    // (1 loop of 2 points) — different programs, never a value alias.
    let two_loops = quad(vec![vec![(0.0, 0.0)], vec![(1.0, 1.0)]]);
    let one_loop = quad(vec![vec![(0.0, 0.0), (1.0, 1.0)]]);
    assert!(two_loops != one_loop);
    // The NaN payload class dies at construction (ruled door 1): a
    // program literal cannot carry the all-ones pattern at all.
    assert!(matches!(
        editor_core::Expr::literal(f64::from_bits(u64::MAX), editor_core::Dimension::Scalar),
        Err(editor_core::DimensionError::NonFiniteLiteral)
    ));
}

/// ATTACK 2: duplicate JSON object keys in serde-derived BTreeMaps
/// (params / nodes / metadata / witnesses) — last-wins silently?
#[test]
fn attack_duplicate_json_keys() {
    let (_, text) = small();
    // Duplicate the "q" param with a different value.
    let dup_param = text.replace(
        "\"q\": {",
        "\"q\": {\"Continuous\":{\"dim\":\"Length\",\"value\":9.75}}, \"q\": {",
    );
    assert_ne!(dup_param, text, "fixture must contain the param");
    match load(&dup_param, Tol::witness()) {
        Err(PersistError::Parse { message, .. }) => {
            assert!(
                message.contains("duplicate document parameter key") && message.contains("\"q\""),
                "refusal must name key and section: {message}"
            );
        }
        other => panic!("duplicate param key must refuse typed, got {other:?}"),
    }
}

/// ATTACK 2b: duplicate node-id keys ("0" twice).
#[test]
fn attack_duplicate_node_keys() {
    let (_, text) = small();
    // Find the node map entry for id 0 and duplicate the whole entry.
    let start = text.find("\"0\":").expect("node 0 key");
    // Node 0's value runs until "\n      \"1\":" at the same level.
    let end = text.find("\"1\":").expect("node 1 key");
    let entry = &text[start..end];
    let doubled = format!(
        "{}{}{}",
        &text[..start],
        format_args!("{entry}{entry}"),
        &text[end..]
    );
    match load(&doubled, Tol::witness()) {
        Err(PersistError::Parse { message, .. }) => {
            assert!(
                message.contains("duplicate snapshot node key"),
                "refusal must name key and section: {message}"
            );
        }
        other => panic!("duplicate node key must refuse typed, got {other:?}"),
    }
}

/// ATTACK 2c: non-canonical integer key text ("00" vs "0") — two
/// spellings of one id.
#[test]
fn attack_noncanonical_integer_keys() {
    let (_, text) = small();
    let start = text.find("\"0\":").expect("node 0 key");
    let end = text.find("\"1\":").expect("node 1 key");
    let entry = &text[start..end];
    let alias = entry.replacen("\"0\":", "\"00\":", 1);
    let doubled = format!(
        "{}{}{}",
        &text[..start],
        format_args!("{entry}{alias}"),
        &text[end..]
    );
    match load(&doubled, Tol::witness()) {
        Err(_) => {}
        Ok(_) => panic!("\"0\" and \"00\" both accepted as node id 0 — silent last-wins"),
    }
}

/// ATTACK 3: long decimal strings that differ in the last ulp must
/// parse correctly rounded (float_roundtrip). Craft the file text.
#[test]
fn attack_long_decimal_strings() {
    let (_, text) = small();
    // 0.1 + tiny perturbations spelled with 25 digits: each must load
    // as the correctly-rounded nearest f64 (== Rust's own parse).
    for s in [
        "0.1000000000000000055511151231257827",  // == 0.1 exactly
        "0.1000000000000000055511151231257828",  // still 0.1
        "0.10000000000000000555111512312578271", // still 0.1
        "0.09999999999999999167332731531132594", // 0.1.next_down()
        "2.2250738585072011e-308",               // rounding-boundary
        "2.2250738585072012e-308",
        "4.9406564584124654e-324", // min subnormal spelled long
        "8.98846567431158e307",
    ] {
        let expect: f64 = s.parse().unwrap(); // Rust core: correctly rounded
        let crafted = text.replace("\"value\": 2.5", &format!("\"value\": {s}"));
        assert_ne!(crafted, text);
        let loaded = load(&crafted, Tol::witness()).expect("valid file");
        let Some(DocParam::Continuous { value, .. }) =
            loaded.doc.params().get(&ParamName::new("q"))
        else {
            panic!("param lost")
        };
        assert_eq!(
            value.to_bits(),
            expect.to_bits(),
            "long decimal {s} misparsed: got {value:?} want {expect:?}"
        );
    }
}

/// ATTACK 4: inf via out-of-range exponent in the file text.
#[test]
fn attack_inf_via_big_exponent() {
    let (_, text) = small();
    for s in ["1e999", "-1e999", "1e309"] {
        let crafted = text.replace("\"value\": 2.5", &format!("\"value\": {s}"));
        assert_ne!(crafted, text);
        match load(&crafted, Tol::witness()) {
            Err(PersistError::Parse { .. }) => {}
            Ok(l) => panic!(
                "{s} loaded as {:?}",
                l.doc.params().get(&ParamName::new("q"))
            ),
            Err(e) => panic!("unexpected refusal for {s}: {e:?}"),
        }
    }
}

/// ATTACK 5: every DocEdit variant in one log — round-trip + replay
/// fingerprint identity (independent of the shipped kitchen sink).
#[test]
fn attack_all_fourteen_edit_variants_round_trip() {
    let quad = |o: f64| {
        desc(
            [o, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        )
    };
    let mut doc = ProfileDoc::empty_derived("m4_pr6_review_probes", Tol::witness());
    let mut edits: Vec<DocEdit<ProfileProgram>> = Vec::new();
    let mut push = |doc: &mut ProfileDoc, e: DocEdit<ProfileProgram>| -> Option<RecipeNodeId> {
        let a = apply(doc, &e, Tol::witness()).unwrap_or_else(|err| panic!("edit {e:?} refused: {err:?}"));
        *doc = a.doc;
        edits.push(e);
        a.record.minted
    };
    // 1 SetDocParam
    push(
        &mut doc,
        DocEdit::SetDocParam {
            name: ParamName::new("d"),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 1.5,
            },
        },
    );
    // 2 InsertNode xN
    let p0 = push(
        &mut doc,
        DocEdit::InsertNode {
            node: Node::Profile(quad(0.0)),
        },
    )
    .unwrap();
    let e0 = push(
        &mut doc,
        DocEdit::InsertNode {
            node: Node::Extrude {
                profile: p0,
                distance: Expr::param(ParamName::new("d"), Dimension::Length),
            },
        },
    )
    .unwrap();
    let p1 = push(
        &mut doc,
        DocEdit::InsertNode {
            node: Node::Profile(quad(1.0)),
        },
    )
    .unwrap();
    let e1 = push(
        &mut doc,
        DocEdit::InsertNode {
            node: Node::Extrude {
                profile: p1,
                distance: len(1.5),
            },
        },
    )
    .unwrap();
    let boole = push(
        &mut doc,
        DocEdit::InsertNode {
            node: Node::Boolean {
                op: BooleanOp::Union,
                a: e0,
                b: e1,
                declare: None,
            },
        },
    )
    .unwrap();
    // A doomed node for DeleteNode.
    let doomed = push(
        &mut doc,
        DocEdit::InsertNode {
            node: Node::Profile(quad(5.0)),
        },
    )
    .unwrap();
    // 3 DeleteNode
    push(&mut doc, DocEdit::DeleteNode { id: doomed });
    // 4 SetParam (continuous slot)
    push(
        &mut doc,
        DocEdit::SetParam {
            node: e1,
            slot: SlotId::Distance,
            expr: len(2.0),
        },
    );
    // 5 SetExpression (whole-slot path)
    push(
        &mut doc,
        DocEdit::SetExpression {
            path: ExprPath {
                node: e1,
                slot: SlotId::Distance,
                path: vec![],
            },
            expr: len(1.75),
        },
    );
    // 6 pattern for SetStructuralParam
    let pat = push(
        &mut doc,
        DocEdit::InsertNode {
            node: Node::Pattern {
                input: boole,
                count: Expr::count(2),
                kind: editor_core::PatternKind::Linear {
                    direction: [
                        Expr::literal(1.0, Dimension::Scalar).unwrap(),
                        Expr::literal(0.0, Dimension::Scalar).unwrap(),
                        Expr::literal(0.0, Dimension::Scalar).unwrap(),
                    ],
                    spacing: len(4.0),
                },
            },
        },
    )
    .unwrap();
    push(
        &mut doc,
        DocEdit::SetStructuralParam {
            node: pat,
            slot: SlotId::Count,
            expr: Expr::count(3),
        },
    );
    // 7 ReWitness
    push(
        &mut doc,
        DocEdit::ReWitness {
            node: p0,
            witness: WitnessDatum {
                schema: 1,
                bytes: vec![0xff; 8],
            }, // NaN-looking bytes: opaque, must NOT be scanned
        },
    );
    // 8 ReWitnessBulk
    push(
        &mut doc,
        DocEdit::ReWitnessBulk {
            entries: vec![
                (
                    p0,
                    WitnessDatum {
                        schema: 1,
                        bytes: vec![0x00, 0x80],
                    },
                ),
                (
                    p1,
                    WitnessDatum {
                        schema: 1,
                        bytes: vec![],
                    },
                ),
            ],
            certification: BranchCertification {
                schema: 1,
                bytes: vec![0xaa, 0xbb],
            },
        },
    );
    // 9 SetAppearance + 10 SetAppearanceMeta on the union body
    let body = StableName {
        kind: EntityKind::Body,
        node: boole,
        path: vec![RoleSeg::OutputBody],
    };
    push(
        &mut doc,
        DocEdit::SetAppearance {
            name: body.clone(),
            attr: Attr::Color(Rgba8::opaque(1, 2, 3)),
        },
    );
    let mut m = std::collections::BTreeMap::new();
    m.insert("v".to_owned(), MetaValue::Int(1));
    m.insert("neg".to_owned(), MetaValue::Float(-0.0));
    push(
        &mut doc,
        DocEdit::SetAppearanceMeta {
            name: body.clone(),
            key: "adv/probe".into(),
            value: MetaValue::Map(m.clone()),
        },
    );
    push(
        &mut doc,
        DocEdit::SetAppearanceMeta {
            name: body.clone(),
            key: "adv/probe2".into(),
            value: MetaValue::Map(m),
        },
    );
    // 11 ClearAppearanceMeta
    push(
        &mut doc,
        DocEdit::ClearAppearanceMeta {
            name: body.clone(),
            key: "adv/probe2".into(),
        },
    );
    // 12 Rebind: repair the body name onto the pattern's output body.
    let to = StableName {
        kind: EntityKind::Body,
        node: pat,
        path: vec![RoleSeg::OutputBody],
    };
    push(
        &mut doc,
        DocEdit::Rebind {
            from: body,
            to: to.clone(),
        },
    );
    // 13 ClearAppearance (attr moved with the record to `to`)
    push(
        &mut doc,
        DocEdit::ClearAppearance {
            name: to,
            kind: AttrKind::Color,
        },
    );
    // 14 SetTolerance (ambient value: keeps this process evaluable)
    let amb = geom_core::Tol::witness().get().eps;
    push(&mut doc, DocEdit::SetTolerance { eps: amb });

    // Round-trip as a FULL LOG from an empty snapshot.
    let text = save(&ProfileDoc::empty_derived("m4_pr6_review_probes", Tol::witness()), &edits, Tol::witness()).expect("save log");
    let loaded = load(&text, Tol::witness()).expect("load log");
    assert_eq!(loaded.edits, edits, "edit log round-trip");
    assert!(loaded.doc.bit_eq(&doc), "replayed doc bit-identical");
    // AND as a snapshot.
    let text2 = save(&doc, &[], Tol::witness()).expect("save snapshot");
    let loaded2 = load(&text2, Tol::witness()).expect("load snapshot");
    assert!(loaded2.doc.bit_eq(&doc));
    // Replay identity fingerprint.
    let fp = |d: &ProfileDoc| {
        let ev = evaluate::<f64>(d, None, &CancelToken::new(), &EvalOptions::default(), Tol::witness());
        format!("{:?}|{:?}|{:?}", ev.order, ev.nodes, ev.appearance)
    };
    assert_eq!(fp(&doc), fp(&loaded.doc), "log replay fingerprint");
    assert_eq!(fp(&doc), fp(&loaded2.doc), "snapshot replay fingerprint");
    // Canonical bytes.
    assert_eq!(save(&loaded2.doc, &[], Tol::witness()).unwrap(), text2);
}

/// ATTACK 6: truncation ladder — typed refusal at every cut, never a
/// partial document.
#[test]
fn attack_truncation_ladder() {
    let (_, text) = small();
    let n = text.len();
    for i in 1..=10 {
        let cut = &text[..(n * i / 11)];
        match load(cut, Tol::witness()) {
            Err(
                PersistError::Parse { .. }
                | PersistError::Header { .. }
                | PersistError::UnknownSchema { .. },
            ) => {}
            Ok(_) => panic!("truncation at {} loaded", n * i / 11),
            Err(e) => panic!("untyped-ish refusal at {}: {e:?}", n * i / 11),
        }
    }
}

/// ATTACK 7: structural byte flips refuse; float-digit flips either
/// refuse or load a VALID different value (no panics, no partials).
#[test]
fn attack_byte_flips() {
    let (_, text) = small();
    let bytes = text.as_bytes();
    for pos in (10..text.len()).step_by(text.len() / 17) {
        let mut b = bytes.to_vec();
        b[pos] = match b[pos] {
            b'{' => b'[',
            b'"' => b'\'',
            b':' => b';',
            b',' => b'.',
            c if c.is_ascii_digit() => b'x',
            c => c ^ 0x01,
        };
        let Ok(s) = String::from_utf8(b) else {
            continue;
        };
        let _ = load(&s, Tol::witness()); // must not panic; Result either way
    }
}

/// ATTACK 8: ε as -0.0 and tiny-but-positive in the file text.
#[test]
fn attack_epsilon_doors_in_file() {
    let (doc, text) = small();
    let amb = format!("{:?}", doc.epsilon());
    let neg = text.replace(&format!("\"epsilon\": {amb}"), "\"epsilon\": -0.0");
    assert_ne!(neg, text, "epsilon field present as {amb}");
    match load(&neg, Tol::witness()) {
        Err(PersistError::Snapshot(_)) => {}
        other => panic!("-0.0 epsilon must refuse as snapshot invariant, got {other:?}"),
    }
}

/// ATTACK 9: header lenience — non-canonical spellings.
#[test]
fn attack_header_spellings() {
    let (_, text) = small();
    let body = text.split_once('\n').unwrap().1;
    let v = SCHEMA_VERSION;
    for h in [
        format!("schema: +{v}"),
        format!("schema:{v}"),
        format!("schema:  {v}"),
        format!("schema: 0{v}"),
    ] {
        let t = format!("{h}\n{body}");
        match load(&t, Tol::witness()) {
            Err(PersistError::Header { .. }) => {}
            other => panic!("non-canonical header {h:?} must refuse typed, got {other:?}"),
        }
    }
    // The canonical spelling still loads. (M5 PR 10: the version is
    // read from `SCHEMA_VERSION`, not a literal — a schema bump must
    // not silently turn this row into a too-old refusal probe.)
    assert!(load(&format!("schema: {v}\n{body}"), Tol::witness()).is_ok());
}

/// ATTACK 10: appearance key referencing a DELETED node (< next_id,
/// not live) in a crafted snapshot — and metadata insertion-order
/// canonicalization.
#[test]
fn attack_meta_order_canonical() {
    let (doc, _) = small();
    let name = StableName {
        kind: EntityKind::Body,
        node: RecipeNodeId(1),
        path: vec![RoleSeg::OutputBody],
    };
    let tree = |order: bool| {
        let mut m = std::collections::BTreeMap::new();
        if order {
            m.insert("v".to_owned(), MetaValue::Int(1));
            m.insert("a".to_owned(), MetaValue::Int(2));
        } else {
            m.insert("a".to_owned(), MetaValue::Int(2));
            m.insert("v".to_owned(), MetaValue::Int(1));
        }
        MetaValue::Map(m)
    };
    let mk = |order: bool| {
        let d = apply(
            &doc,
            &DocEdit::SetAppearanceMeta {
                name: name.clone(),
                key: "k".into(),
                value: tree(order),
            },
            Tol::witness(),
        )
        .unwrap()
        .doc;
        save(&d, &[], Tol::witness()).unwrap()
    };
    assert_eq!(
        mk(true),
        mk(false),
        "insertion order must not leak into bytes"
    );
}

/// MAJOR-2, pinned PER MAP (fix pass): every serde-derived map in the
/// format refuses duplicate keys typed, naming key + section.
#[test]
fn duplicate_keys_refuse_in_every_map() {
    // A fixture whose save exercises witnesses, doc metadata slot,
    // appearance attrs and metadata, and a MetaValue map.
    let (doc, _) = small();
    let doc = apply(
        &doc,
        &DocEdit::ReWitness {
            node: RecipeNodeId(0),
            witness: WitnessDatum {
                schema: 1,
                bytes: vec![0x11],
            },
        },
        Tol::witness(),
    )
    .unwrap()
    .doc;
    let body = StableName {
        kind: EntityKind::Body,
        node: RecipeNodeId(1),
        path: vec![RoleSeg::OutputBody],
    };
    let doc = apply(
        &doc,
        &DocEdit::SetAppearance {
            name: body.clone(),
            attr: Attr::Color(Rgba8::opaque(1, 2, 3)),
        },
        Tol::witness(),
    )
    .unwrap()
    .doc;
    let mut m = std::collections::BTreeMap::new();
    m.insert("v".to_owned(), MetaValue::Int(1));
    let doc = apply(
        &doc,
        &DocEdit::SetAppearanceMeta {
            name: body,
            key: "k".into(),
            value: MetaValue::Map(m),
        },
        Tol::witness(),
    )
    .unwrap()
    .doc;
    let text = save(&doc, &[], Tol::witness()).unwrap();

    // (surgery pattern, expected section words)
    let cases: [(&str, &str, &str); 5] = [
        (
            "\"witnesses\": {",
            "\"witnesses\": {\"0\": {\"schema\": 9, \"bytes\": \"22\"}, ",
            "duplicate witness node key",
        ),
        (
            "\"metadata\": {},",
            "\"metadata\": {\"a\": \"x\", \"a\": \"y\"},",
            "duplicate document metadata key",
        ),
        (
            "\"attrs\": {",
            "\"attrs\": {\"Color\": {\"Color\": {\"r\":9,\"g\":9,\"b\":9,\"a\":255}}, ",
            "duplicate appearance attribute key",
        ),
        (
            "\"metadata\": {\n            \"k\"",
            "\"metadata\": {\n            \"k\": {\"Map\":{\"v\":{\"Int\":2}}},\n            \"k\"",
            "duplicate appearance metadata key",
        ),
        (
            "\"v\": {",
            "\"v\": {\"Int\": 7}, \"v\": {",
            "duplicate metadata map key",
        ),
    ];
    for (needle, replacement, expected) in cases {
        let crafted = text.replacen(needle, replacement, 1);
        assert_ne!(crafted, text, "fixture must contain {needle:?}");
        match load(&crafted, Tol::witness()) {
            Err(PersistError::Parse { message, .. }) => assert!(
                message.contains(expected),
                "expected {expected:?} in refusal, got: {message}"
            ),
            other => panic!("{expected}: must refuse typed, got {other:?}"),
        }
    }
}

/// MAJOR-2's interchange twin: the ε-audit verdict summaries refuse
/// duplicate keys too (same strict-map door).
#[test]
fn duplicate_keys_refuse_in_verdict_summaries() {
    let json = r#"{"nodes":{"0":{"status":"Ok","populations":{"p":[1,0,0],"p":[0,1,0]}}}}"#;
    let r: Result<editor_core::VerdictSummary, _> = serde_json::from_str(json);
    let e = r
        .expect_err("duplicate population key must refuse")
        .to_string();
    assert!(e.contains("duplicate verdict population key"), "{e}");
    let json = r#"{"nodes":{"0":{"status":"Ok","populations":{}},"0":{"status":"Failed","populations":{}}}}"#;
    let r: Result<editor_core::VerdictSummary, _> = serde_json::from_str(json);
    let e = r
        .expect_err("duplicate summary node key must refuse")
        .to_string();
    assert!(e.contains("duplicate verdict summary node key"), "{e}");
}
