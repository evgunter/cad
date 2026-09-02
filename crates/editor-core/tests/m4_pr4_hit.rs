//! M4 PR 4 spec D4: hit-testing inversion — arena key → StableName
//! over the evaluation's tables, TOTAL for every entity the
//! evaluation exposes (asserted over the corpus, counted), with the
//! typed `Unnamed` bug door and typed refusals for unusable nodes.
//! The GUI never sees an arena key: every mesh back-ref inverts.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    BooleanOp, BooleanValue, CancelToken, EntityKey, EntityRef, EvalOptions, Evaluation,
    HitTestError, Node, ProfileDoc, RecipeNodeId, Resolution, RunCtx, SplitSide, ValuePayload,
    body_name, entity_name, evaluate, resolve,
};
use fixture::{ang, die, insert, len, on_frame, scl};
use geom_core::Tol;
use topo::{Body, FaceKey};

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// Every output body of a node's value, with its body index.
fn bodies_of(payload: &ValuePayload<f64>) -> Vec<(u32, &Body<f64>)> {
    match payload {
        ValuePayload::Body(b) => vec![(0, &**b)],
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => vec![(0, &**body)],
        ValuePayload::Boolean(BooleanValue::Empty) => vec![],
        ValuePayload::Split { above, below } => {
            let mut out = Vec::new();
            if let SplitSide::Body(b) = above {
                out.push((0, &**b));
            }
            if let SplitSide::Body(b) = below {
                out.push((1, &**b));
            }
            out
        }
        ValuePayload::Instances(v) => v
            .iter()
            .enumerate()
            .map(|(i, b)| (u32::try_from(i).unwrap(), &**b))
            .collect(),
        ValuePayload::Datum(_)
        | ValuePayload::Profile(_)
        | ValuePayload::Declarations(_)
        | ValuePayload::Mate(_)
        // Neither sink denotes a body, so neither offers an entity to
        // invert — the same answer a declaration gives.
        | ValuePayload::Measure { .. }
        | ValuePayload::Assertion(_) => vec![],
    }
}

/// The D4 totality walk: EVERY face, edge, vertex, and body of every
/// Ok node inverts to a name, and the name round-trips through
/// resolution to the same entity. Returns the number of entities
/// checked (counted, not vibes).
fn assert_total(doc: &ProfileDoc, ev: &Evaluation<f64>) -> usize {
    let ctx = RunCtx { doc, eval: ev };
    let mut checked = 0usize;
    for &node in &ev.order {
        let Some(v) = ev.value(node) else { continue };
        for (body_ix, body) in bodies_of(&v.payload) {
            let mut keys: Vec<EntityKey> = vec![EntityKey::Body];
            keys.extend(body.faces().map(|(k, _)| EntityKey::Face(k)));
            keys.extend(body.edges().map(|(k, _)| EntityKey::Edge(k)));
            keys.extend(body.vertices().map(|(k, _)| EntityKey::Vertex(k)));
            for key in keys {
                let ent = EntityRef { body: body_ix, key };
                let name = entity_name(ev, node, ent)
                    .unwrap_or_else(|e| panic!("unnamed entity at {node:?}: {e:?}"));
                assert_eq!(name.kind, key.kind(), "kind agreement at {node:?}");
                // Per-table bidirectionality: THIS node's forward row
                // gives the entity back (unique, or a tie listing it).
                match v.name_table.lookup(name) {
                    Some(editor_core::Entry::Unique(r)) => {
                        assert_eq!(*r, ent, "table round trip moved {name:?}");
                    }
                    Some(editor_core::Entry::Tied(c)) => {
                        assert!(c.contains(&ent), "tie row lost {name:?}");
                    }
                    None => panic!("reverse row without forward row for {name:?}"),
                }
                // Global resolvability: the name resolves (its first
                // carrying node may be an earlier pass-through source
                // — a split half re-indexes bodies, so only the NODE-
                // LOCAL entity is asserted above) or is honestly tied.
                match resolve(ctx, name) {
                    Resolution::Resolved(_) => {}
                    Resolution::Failed(f) => {
                        assert!(
                            matches!(
                                f.error,
                                editor_core::ResolveError::Ambiguous { ref tie, .. }
                                if tie.width >= 2
                            ),
                            "round trip failed non-ambiguously for {name:?}: {f:?}"
                        );
                    }
                    other => panic!("round trip indeterminate for {name:?}: {other:?}"),
                }
                checked += 1;
            }
        }
    }
    checked
}

#[test]
fn inversion_is_total_on_the_die() {
    let d = die();
    let ev = run(&d.doc);
    let checked = assert_total(&d.doc, &ev);
    assert!(checked > 1000, "die walk too small: {checked}");
}

#[test]
fn inversion_is_total_on_boolean_split_revolve_and_pattern() {
    // A compact corpus exercising every op family's table shapes:
    // overlapping union (fragments/seams), split (both halves),
    // partial revolve (bands/meridians/caps), linear pattern
    // (instances).
    let doc = ProfileDoc::empty_derived("m4_pr4_hit", Tol::witness());
    let (doc, a) = {
        let (doc, p) = on_frame(
            doc,
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        );
        insert(
            doc,
            Node::Extrude {
                profile: p,
                distance: len(1.0),
            },
        )
    };
    let (doc, b) = {
        let (doc, p) = on_frame(
            doc,
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.5, 0.0), (1.5, 0.0), (1.5, 1.0), (0.5, 1.0)]],
        );
        insert(
            doc,
            Node::Extrude {
                profile: p,
                distance: len(1.0),
            },
        )
    };
    let (doc, decl) = fixture::declare_x_offset_flush(doc, a, b);
    let (doc, u) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: Some(decl),
        },
    );
    // Split the union.
    let (doc, plane) = insert(
        doc,
        Node::Datum(editor_core::Datum::Plane {
            origin: [len(0.0), len(0.5), len(0.0)],
            normal: [scl(0.0), scl(1.0), scl(0.0)],
        }),
    );
    let (doc, _split) = insert(
        doc,
        Node::Split {
            target: u,
            tool: plane,
        },
    );
    // A partial revolve (bands, meridians, wedge caps).
    let (doc, rp) = on_frame(
        doc,
        [0.0, 3.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)]],
    );
    let (doc, axis) = insert(
        doc,
        Node::Datum(editor_core::Datum::Axis {
            origin: [len(0.0), len(3.0), len(0.0)],
            direction: [scl(0.0), scl(1.0), scl(0.0)],
        }),
    );
    let (doc, _rev) = insert(
        doc,
        Node::Revolve {
            profile: rp,
            axis,
            angle: ang(std::f64::consts::FRAC_PI_2),
        },
    );
    // A pattern of the union.
    let (doc, _pat) = insert(
        doc,
        Node::Pattern {
            input: u,
            count: editor_core::Expr::count(3),
            kind: editor_core::PatternKind::Linear {
                direction: [scl(0.0), scl(0.0), scl(1.0)],
                spacing: len(3.0),
            },
        },
    );
    let ev = run(&doc);
    for &id in &ev.order {
        assert!(
            ev.value(id).is_some(),
            "corpus node {id:?} failed: {:?}",
            ev.nodes.get(&id)
        );
    }
    let checked = assert_total(&doc, &ev);
    assert!(checked > 300, "corpus walk too small: {checked}");
}

#[test]
fn unusable_nodes_refuse_typed_and_unnamed_is_loud() {
    // Failed / poisoned doors.
    let doc = ProfileDoc::empty_derived("m4_pr4_hit", Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, ext) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(0.0), // degenerate: the extrude fails
        },
    );
    let (doc, ext2) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(1.0),
        },
    );
    let (doc, u) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: ext,
            b: ext2,
            declare: None,
        },
    );
    let ev = run(&doc);
    assert_eq!(
        body_name(&ev, ext, 0),
        Err(HitTestError::NodeFailed { node: ext })
    );
    assert_eq!(
        body_name(&ev, u, 0),
        Err(HitTestError::NodePoisoned {
            node: u,
            through: ext
        })
    );
    assert_eq!(
        body_name(&ev, RecipeNodeId(9999), 0),
        Err(HitTestError::NodeNotEvaluated {
            node: RecipeNodeId(9999)
        })
    );
    // The Unnamed bug door: a node whose (legitimately empty) table
    // cannot answer for a foreign entity refuses LOUDLY with the
    // entity attached — never a silent None.
    let (doc2, decl) = insert(doc, Node::declare_rest(vec![]));
    let ev2 = run(&doc2);
    let some_face = ev2
        .value(ext2)
        .unwrap()
        .name_table
        .iter()
        .find_map(|(_, e)| match e {
            editor_core::Entry::Unique(r) if matches!(r.key, EntityKey::Face(_)) => Some(*r),
            _ => None,
        })
        .unwrap();
    assert_eq!(
        entity_name(&ev2, decl, some_face),
        Err(HitTestError::Unnamed {
            node: decl,
            entity: some_face
        })
    );
}

/// The Display contract (#1111): a consumer renders a `HitTestError`
/// through the payload's own words, so every arm must state what
/// happened in prose — the node it is about, the kind of entity where
/// there is one — and must never read as the `Debug` struct dump the
/// viewer was reduced to printing. The variant identifier and the
/// field-name punctuation are the dump's fingerprints; asserting their
/// ABSENCE is what keeps a future `write!(f, "{self:?}")` from passing
/// this test.
#[test]
fn hit_test_error_display_names_its_content_not_its_struct() {
    let node = RecipeNodeId(7);
    let through = RecipeNodeId(3);
    let cases = [
        (
            HitTestError::NodeNotEvaluated { node },
            vec!["node 7", "no result"],
        ),
        (HitTestError::NodeFailed { node }, vec!["node 7", "failed"]),
        (
            HitTestError::NodePoisoned { node, through },
            vec!["node 7", "node 3", "poisoned"],
        ),
        (
            HitTestError::Unnamed {
                node,
                entity: EntityRef {
                    body: 2,
                    key: EntityKey::Face(FaceKey::default()),
                },
            },
            // The entity renders by KIND and body index — an arena key
            // is editor-core-private and says nothing to a person.
            vec!["node 7", "face", "body 2", "kernel bug"],
        ),
    ];
    for (err, wants) in cases {
        let shown = err.to_string();
        for want in wants {
            assert!(
                shown.contains(want),
                "{err:?} renders as {shown:?}, missing {want:?}"
            );
        }
        for dump in ["NodeNotEvaluated", "NodeFailed", "NodePoisoned", "Unnamed"] {
            assert!(
                !shown.contains(dump),
                "{err:?} renders as {shown:?} — that is the variant name, i.e. a struct dump"
            );
        }
        assert!(
            !shown.contains('{') && !shown.contains("node:"),
            "{err:?} renders as {shown:?} — that is Debug punctuation, not a sentence"
        );
        assert_ne!(shown, format!("{err:?}"));
    }
}
