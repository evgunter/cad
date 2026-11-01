//! M4 PR 4 spec D4: hit-testing inversion — arena key → StableName
//! over the evaluation's tables, TOTAL for every entity the
//! evaluation exposes (asserted over the corpus, counted), with the
//! typed `Unnamed` bug door and typed refusals for unusable nodes.
//! The GUI never sees an arena key: every mesh back-ref inverts.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::display_contract::assert_f6_every_variant;
use crate::fixture;
use test_utils::refusal::tagged;

use editor_core::NodeStanding;
use editor_core::{
    BooleanValue, CancelToken, CapEnd, DocumentId, EntityKey, EntityKind, EntityRef, EvalOptions,
    Evaluation, HitTestError, Node, PickHit, ProfileDoc, RecipeNodeId, Resolution, RoleSeg, RunCtx,
    SplitSide, StableName, UnnamedEntity, ValuePayload, body_name, entity_name, evaluate,
};
use fixture::{ang, die, insert, len, on_frame};
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
        ValuePayload::Body(b) => vec![(0, b.body())],
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => vec![(0, body.body())],
        ValuePayload::Boolean(BooleanValue::Empty) => vec![],
        ValuePayload::Split { above, below } => {
            let mut out = Vec::new();
            if let SplitSide::Body(b) = above {
                out.push((0, b.body()));
            }
            if let SplitSide::Body(b) = below {
                out.push((1, b.body()));
            }
            out
        }
        ValuePayload::Family => vec![],
        ValuePayload::Instances(v) => v
            .iter()
            .enumerate()
            .map(|(i, b)| (u32::try_from(i).unwrap(), b.body()))
            .collect(),
        ValuePayload::Profile(_)
        // Neither sink denotes a body, so neither offers an entity to
        // invert.
        | ValuePayload::Measure { .. }
        | ValuePayload::MeasureUnavailable { .. }
        | ValuePayload::Assertion(_) => vec![],
    }
}

/// The D4 totality walk: EVERY face, edge, vertex, and body of every
/// Ok node inverts to a name, and the name round-trips through
/// resolution to the same entity. Returns the number of entities
/// checked (counted, not vibes).
fn assert_total(doc: &editor_core::ProfileDoc, ev: &Evaluation<f64>) -> usize {
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
                // Per-table bidirectionality: the forward row of the
                // read holding the entity gives it back (unique, or a
                // tie listing it) — a split's half its own table.
                let (var, at) = ev
                    .entity_read(node, ent)
                    .unwrap_or_else(|| panic!("no read holds {ent:?} at {node:?}"));
                let table = ev.table(var).expect("the read's table");
                match table.lookup(name) {
                    Some(editor_core::Entry::Unique(r)) => {
                        assert_eq!(*r, at, "table round trip moved {name:?}");
                    }
                    Some(editor_core::Entry::Tied(c)) => {
                        assert!(c.contains(&at), "tie row lost {name:?}");
                    }
                    None => panic!("reverse row without forward row for {name:?}"),
                }
                // Resolvability in the read that holds the entity (N1:
                // resolution names the read): the name resolves there,
                // to this very entity of this node's value, or is
                // honestly tied.
                match editor_core::resolve(ctx, var, name).unwrap() {
                    Resolution::Resolved(r) => {
                        assert_eq!((r.node, r.entity), (node, ent), "{name:?} resolves home");
                    }
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
                profile: p.into(),
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
                profile: p.into(),
                distance: len(1.0),
            },
        )
    };
    let (doc, b) = fixture::slid(doc, a, b, [0.0; 3]);
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
    );
    // Split the union.
    let tool = fixture::plane_on(&doc, u, [0.0, 0.5, 0.0], [0.0, 1.0, 0.0]);
    let (doc, _split) = insert(
        doc,
        Node::Split {
            target: u.into(),
            tool,
        },
    );
    // A partial revolve (bands, meridians, wedge caps).
    let (doc, rp) = fixture::on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)]],
    );
    // The axis: the profile's own +y through (0, 0).
    let (doc, _rev) = insert(
        doc,
        fixture::revolve_about(
            rp,
            (0.0, 0.0),
            crate::fixture::along_y(),
            ang(std::f64::consts::FRAC_PI_2),
        ),
    );
    // A pattern of the union.
    let (doc, _pat_k, _pat) = crate::fixture::linear(doc, u, [0.0, 0.0, 1.0], 3.0, 3);
    let ev = run(&doc);
    for &id in &ev.order {
        assert!(
            ev.value(id).is_some(),
            "corpus node {id:?} failed: {:?}",
            ev.nodes.get(&id)
        );
    }
    let checked = assert_total(&doc, &ev);
    assert!(checked >= 240, "corpus walk too small: {checked}");
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
            profile: p.into(),
            distance: len(0.0), // degenerate: the extrude fails,
        },
    );
    let (doc, ext2) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
        },
    );
    // The second joins the first's space, as an operator's operands
    // share one.
    let (doc, ext2_in) = crate::fixture::stood_in(doc, ext2, ext);
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![ext.into(), ext2_in.into()]),
        },
    );
    let ev = run(&doc);
    assert_eq!(
        body_name(&ev, ext, 0),
        Err(HitTestError::Standing(NodeStanding::Failed { node: ext }))
    );
    assert_eq!(
        body_name(&ev, u, 0),
        Err(HitTestError::Standing(NodeStanding::Poisoned {
            node: u,
            through: ext
        }))
    );
    assert_eq!(
        body_name(&ev, RecipeNodeId::new(0, tagged(9999)), 0),
        Err(HitTestError::Standing(NodeStanding::NotInDocument {
            node: RecipeNodeId::new(0, tagged(9999))
        }))
    );
    // The Unnamed bug door: a node whose (legitimately empty) table
    // cannot answer for a foreign entity refuses LOUDLY with the
    // entity attached — never a silent None.
    // The sketch frame denotes no body, so its table is empty.
    let frame = doc.ids()[0];
    let ev2 = run(&doc);
    let some_face = ev2
        .table_at(ext2)
        .unwrap()
        .iter()
        .find_map(|(_, e)| match e {
            editor_core::Entry::Unique(r) if matches!(r.key, EntityKey::Face(_)) => Some(*r),
            _ => None,
        })
        .unwrap();
    assert_eq!(
        entity_name(&ev2, frame, some_face),
        Err(HitTestError::Unnamed(UnnamedEntity {
            node: frame,
            entity: some_face
        }))
    );
}

test_utils::f6_variants! {
    /// `HitTestError`'s census: one ident per variant, feeding both the
    /// wildcard-free `match` rustc checks and the identifier roster the
    /// weld compares against the rendered cases. The mechanism and what
    /// it does NOT weld are documented on
    /// [`test_utils::f6::assert_f6_every_variant`].
    const HIT_TEST_ERROR: HitTestError = [
        Standing,
        EvaluationOfAnotherDocument,
        Ambiguous,
        Unnamed,
        AcrossSpaces,
    ];
}

/// The Display contract (#1111): a consumer renders a `HitTestError`
/// through the payload's own words, so every arm must state what
/// happened in prose — the node it is about, the kind of entity where
/// there is one — and must never read as the `Debug` struct dump the
/// viewer was reduced to printing. The variant identifier and the
/// field-name punctuation are the dump's fingerprints; asserting their
/// ABSENCE is what keeps a future `write!(f, "{self:?}")` from passing
/// this test.
///
/// The shape itself is [`test_utils::f6::assert_f6`] through the
/// binary's one wrapper, not a copy of it here: this row held the
/// partial third spelling that
/// `work/view/f6-display-predicate-is-spelled-three-times-with-no-home`
/// cites, and its roster had drifted from the other one.
#[test]
fn hit_test_error_display_names_its_content_not_its_struct() {
    let node = RecipeNodeId::new(0, tagged(7));
    let through = RecipeNodeId::new(0, tagged(3));
    // Two faces of ONE node, differing only in their role path — the
    // shared-edge tie's own shape, and the case that says the
    // rendering carries the path.
    let tied_hit = |at: u32| PickHit {
        name: StableName {
            kind: EntityKind::Face,
            node,
            path: vec![RoleSeg::Cap(if at == 3 {
                CapEnd::Start
            } else {
                CapEnd::End
            })],
        },
        node,
        body: 0,
        t: f64::from(at),
        t_lo: f64::from(at),
        t_hi: f64::from(at),
        point: geom_core::Point3::new(0.0, 0.0, 0.0),
    };
    let cases = [
        (
            HitTestError::Standing(NodeStanding::NotEvaluated { node }),
            vec!["node 000000000007", "no result"],
        ),
        (
            HitTestError::Standing(NodeStanding::Failed { node }),
            vec!["node 000000000007", "failed"],
        ),
        (
            HitTestError::Standing(NodeStanding::Poisoned { node, through }),
            vec!["node 000000000007", "node 000000000003", "poisoned"],
        ),
        (
            HitTestError::EvaluationOfAnotherDocument {
                expected: DocumentId::derive("m4-hit-expected"),
                found: DocumentId::derive("m4-hit-found"),
            },
            // Two documents, both named: which one the door is about
            // and which one was handed. The ids render through their
            // own `Display`, so the prose carries the whole answer.
            vec!["document", "not"],
        ),
        (
            HitTestError::Ambiguous {
                hits: [3u32, 5].map(tied_hit).to_vec(),
            },
            // Each tied face in its words, which tell two faces of one
            // node apart, and no role path.
            vec![
                "tied between the start cap of node 000000000007",
                "and the end cap of node 000000000007",
                "so the pick names neither",
            ],
        ),
        (
            HitTestError::Unnamed(UnnamedEntity {
                node,
                entity: EntityRef {
                    body: 2,
                    key: EntityKey::Face(FaceKey::default()),
                },
            }),
            // The lookup's own sentence under this door's prefix: the
            // entity by KIND and body index — an arena key is
            // editor-core-private and says nothing to a person.
            vec![
                "hit test:",
                "name lookup:",
                "node 000000000007",
                "face",
                "body 2",
                "kernel bug",
            ],
        ),
        (
            HitTestError::AcrossSpaces {
                a: editor_core::VarId::WORLD,
                b: editor_core::VarId::new(0, tagged(7)),
            },
            vec!["different spaces", "the world", "the space #", "Recourse:"],
        ),
    ];
    assert_f6_every_variant(&cases, &HIT_TEST_ERROR, &[]);
}
