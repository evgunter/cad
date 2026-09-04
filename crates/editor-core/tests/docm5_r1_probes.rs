//! DOCM-5 review lane R1 — probes over the subject door and the
//! gathered-product door, on documents the implementer did not choose.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Instant;

use editor_core::{
    Alignment, AssemblyError, AxisSense, CancelToken, CapEnd, ChecksConfig, ContactClass, DocEdit,
    DocRef, DocumentId, EntityKind, EvalOptions, Evaluation, MateFrame, MatePrimitive, Node,
    ProductError, ProfileDoc, RecipeNodeId, ResolveFailure, ResolveFault, RoleSeg, Severity,
    StableName, Subject, assemble, assemble_gathered, content_pin, evaluate, product_recorded,
    run_checks, run_checks_on,
};
use fixture::{ang, insert, len, on_frame, scl, step};
use geom_core::Tol;
use topo::AtRestPolicy;

#[derive(Debug, Default, Clone)]
struct StubStore {
    docs: BTreeMap<DocumentId, ProfileDoc>,
}

impl StubStore {
    fn insert(&mut self, doc: ProfileDoc, tol: Tol) -> DocRef {
        let pin = content_pin(&doc, tol).expect("the pin computes");
        let id = doc.id();
        self.docs.insert(id, doc);
        DocRef { id, pin }
    }
}

impl editor_core::PartResolver for StubStore {
    fn resolve(&self, doc_ref: &DocRef, _tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        let fail = |fault, message: &str| ResolveFailure {
            fault,
            message: message.to_string(),
        };
        let doc = self
            .docs
            .get(&doc_ref.id)
            .ok_or_else(|| fail(ResolveFault::Unresolved, "no such document"))?;
        Ok(doc.clone())
    }
}

fn opts(store: StubStore) -> EvalOptions {
    EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    }
}

fn run(doc: &ProfileDoc, o: &EvalOptions) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), o, Tol::witness())
}

fn block(
    doc: ProfileDoc,
    x: (f64, f64),
    y: (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
        },
    )
}

const PART_BODY: RecipeNodeId = RecipeNodeId(2);

fn cube_part(label: &str) -> ProfileDoc {
    let (doc, _) = block(
        ProfileDoc::empty(DocumentId::derive(label), Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        0.0,
        1.0,
    );
    doc
}

fn in_part(instance: RecipeNodeId, cap: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: Box::new(StableName {
                kind: EntityKind::Face,
                node: PART_BODY,
                path: vec![RoleSeg::Cap(cap)],
            }),
        }],
    }
}

fn frame(origin: [f64; 3], axis: [f64; 3]) -> MateFrame {
    MateFrame {
        origin,
        axis,
        reference: [1.0, 0.0, 0.0],
    }
}

fn mate(a: StableName, b: StableName, class: ContactClass, seat: f64) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a,
        b,
        class,
        alignment: Alignment {
            a: frame([0.0, 0.0, seat], [0.0, 0.0, 1.0]),
            b: frame([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Aligned,
            clocking: None,
        },
    }
}

/// Two cube instances and one mate between them.
fn stacked(label: &str, class: ContactClass, seat: f64, bogus_ref: bool) -> (ProfileDoc, StubStore) {
    let mut store = StubStore::default();
    let doc_ref = store.insert(cube_part(&format!("{label}-part")), Tol::witness());
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for _ in 0..2 {
        let (next, id) = insert(doc, Node::instantiate_part(doc_ref));
        doc = next;
        ids.push(id);
    }
    let a = if bogus_ref {
        // A reference into the part that names a node the part does not have.
        StableName {
            kind: EntityKind::Face,
            node: ids[0],
            path: vec![RoleSeg::InPart {
                of: Box::new(StableName {
                    kind: EntityKind::Face,
                    node: RecipeNodeId(77),
                    path: vec![RoleSeg::Cap(CapEnd::End)],
                }),
            }],
        }
    } else {
        in_part(ids[0], CapEnd::End)
    };
    let (doc, _) = step(
        doc,
        DocEdit::InsertNode {
            node: mate(a, in_part(ids[1], CapEnd::Start), class, seat),
        },
    );
    (doc, store)
}

fn measure(body: &topo::Body<f64>) -> String {
    format!(
        "{}|{}|{}|{:?}",
        body.solids().count(),
        body.faces().count(),
        body.edges().count(),
        topo::mass_properties(body, Tol::witness())
    )
}

fn agree(name: &str, w: Result<editor_core::Assembly<f64>, AssemblyError>, d: Result<editor_core::Assembly<f64>, AssemblyError>) {
    match (w, d) {
        (Ok(a), Ok(b)) => {
            assert_eq!(measure(&a.body), measure(&b.body), "{name}");
            assert_eq!(format!("{:?}", a.minted), format!("{:?}", b.minted), "{name}: minted");
            assert_eq!(format!("{:?}", a.contacts), format!("{:?}", b.contacts), "{name}: contacts");
            assert_eq!(format!("{:?}", a.names), format!("{:?}", b.names), "{name}: names");
        }
        (Err(a), Err(b)) => {
            assert_eq!(a.to_string(), b.to_string(), "{name}: refusal");
            assert_eq!(format!("{a:?}"), format!("{b:?}"), "{name}: refusal debug");
        }
        (a, b) => panic!("{name}: disagree: {:?} vs {:?}", a.map(|_| "Ok"), b.map(|_| "Ok")),
    }
}

/// A2 + A3 over ASSEMBLY documents (the implementer's rows use the
/// part-only corpus): a seated Rest mate (certifies with a minted
/// declaration), a Rest mate with a gap (at-rest finding), a Tangent
/// mate (NoAtRestRecord mint refusal), and a dangling reference
/// (Reference mint refusal).
#[test]
fn r1_wrapper_and_door_agree_on_assemblies_with_every_gate_arm() {
    let tol = Tol::witness();
    let cases = [
        ("seated-rest", ContactClass::Rest, 1.0, false),
        ("gapped-rest", ContactClass::Rest, 1.5, false),
        ("tangent", ContactClass::Tangent, 1.0, false),
        ("dangling", ContactClass::Rest, 1.0, true),
    ];
    let mut arms = Vec::new();
    for (label, class, seat, bogus) in cases {
        let (doc, store) = stacked(&format!("r1-{label}"), class, seat, bogus);
        let ev = run(&doc, &opts(store));
        // A3
        let wrapped = assemble(&doc, &ev, tol);
        arms.push(format!("{label}: {}", match &wrapped {
            Ok(a) => format!("Ok minted={}", a.minted.len()),
            Err(e) => format!("Err {e}"),
        }));
        let gathered = product_recorded(&doc, &ev, tol).expect("gathers");
        let direct = assemble_gathered(gathered, tol);
        agree(label, wrapped, direct);
        // A2 — default config AND a non-default one (connectedness
        // Off, an expectation on an instance root).
        let cfgs = [
            ChecksConfig::default(),
            ChecksConfig {
                connectedness: Severity::Off,
                ..ChecksConfig::default()
            },
            ChecksConfig {
                expected_components: [((RecipeNodeId(0), 0), 2)].into_iter().collect(),
                ..ChecksConfig::default()
            },
        ];
        for (i, cfg) in cfgs.iter().enumerate() {
            let w = run_checks(&doc, &ev, cfg, tol).expect("checks run");
            let g = product_recorded(&doc, &ev, tol).expect("gathers");
            let d = run_checks_on(&doc, &ev, Subject::Product(&g), cfg, tol).expect("door runs");
            assert_eq!(w, d, "{label} cfg {i}");
        }
    }
    println!("{}", arms.join("\n"));
    // The four arms are distinct gate verdicts, so the equality above
    // covered Ok, AtRest/Uncertified, NoAtRestRecord and Reference.
    assert!(arms[0].contains("Ok minted=1"), "{}", arms[0]);
    assert!(arms[1].starts_with("gapped-rest: Err"), "{}", arms[1]);
    assert!(arms[2].contains("no at-rest") || arms[2].contains("Tangent"), "{}", arms[2]);
    assert!(arms[3].contains("names no") || arms[3].contains("reference") || arms[3].starts_with("dangling: Err"), "{}", arms[3]);
}

/// Is the spec's named refusal — `ProductError::Naming`, "two roots
/// collide in the name table" — reachable through the edit door? Two
/// transforms of one extrude, both roots, carry the extrude's minted
/// names into one product table.
#[test]
fn r1_is_a_naming_collision_reachable_and_where_does_it_land() {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-naming"), tol);
    let (doc, extrude) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let transform = |doc, dx: f64| {
        insert(
            doc,
            Node::Transform {
                input: extrude,
                translation: [len(dx), len(0.0), len(0.0)],
                rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
                rotation_angle: ang(0.0),
            },
        )
    };
    let (doc, _t1) = transform(doc, 3.0);
    let (doc, _t2) = transform(doc, 6.0);
    let ev = run(&doc, &EvalOptions::default());
    let refusal = product_recorded(&doc, &ev, tol);
    println!("roots = {:?}", doc.roots());
    println!("gather = {:?}", refusal.as_ref().err());
    let refusal = refusal.expect_err("two transforms of one extrude collide");
    assert!(matches!(refusal, ProductError::Naming { .. }), "{refusal:?}");
    // Through the wrapper: Product; through assemble: Product.
    match run_checks(&doc, &ev, &ChecksConfig::default(), tol) {
        Err(editor_core::ChecksError::Product { reason }) => {
            assert_eq!(reason, refusal.to_string());
        }
        other => panic!("{other:?}"),
    }
    match assemble(&doc, &ev, tol) {
        Err(AssemblyError::Product(e)) => assert_eq!(e.to_string(), refusal.to_string()),
        other => panic!("{:?}", other.map(|_| "Ok")),
    }
}

/// The census figure the two claim sites still quote (~1.1 s) was
/// never re-taken by the unit; take it here, in the same process as
/// the two terms the unit did measure, at the pinned size.
#[test]
#[ignore]
fn r1_census_term_at_the_pinned_point() {
    let tol = Tol::witness();
    let entry = crate::corpus::documents()
        .into_iter()
        .find(|d| d.name == "heat_sink")
        .expect("heat sink");
    let doc = editor_core::apply(
        &entry.doc,
        &DocEdit::SetDocParam {
            name: editor_core::ParamName::new("fins"),
            value: editor_core::DocParam::Count { value: 160 },
        },
        tol,
    )
    .expect("fins")
    .doc;
    let ev: Evaluation<f64> = crate::corpus::eval(&doc);
    let product = product_recorded(&doc, &ev, tol).expect("gathers");
    println!(
        "size: {} solids / {} faces",
        product.body.solids().count(),
        product.body.faces().count()
    );
    let mut gather = Vec::new();
    let mut checks = Vec::new();
    let mut census = Vec::new();
    for _ in 0..5 {
        let t = Instant::now();
        let g = product_recorded(&doc, &ev, tol).expect("gathers");
        gather.push(t.elapsed().as_secs_f64() * 1e3);
        drop(g);
        let t = Instant::now();
        run_checks_on(&doc, &ev, Subject::Product(&product), &ChecksConfig::default(), tol)
            .expect("runs");
        checks.push(t.elapsed().as_secs_f64() * 1e3);
        let t = Instant::now();
        let verdict = <f64 as AtRestPolicy>::gate_at_rest_declared(&product.body, &product.contacts, tol);
        census.push(t.elapsed().as_secs_f64() * 1e3);
        println!("census verdict: {:?}", verdict.as_ref().map(|_| "ok").map_err(|e| e.len()));
    }
    let med = |mut v: Vec<f64>| {
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        (v[2], v[0], v[4])
    };
    println!("gather ms (med,min,max) = {:?}", med(gather));
    println!("checks ms (med,min,max) = {:?}", med(checks));
    println!("census ms (med,min,max) = {:?}", med(census));
}
