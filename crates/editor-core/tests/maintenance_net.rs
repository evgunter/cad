//! **`MaintenanceNet` — an action's maintenance, net of what the
//! action itself made moot.**
//!
//! `Applied::maintenance` answers what ONE edit did. An action of
//! several edits can report a row an edit later in the same action
//! takes back, and the net is what is true of the document the action
//! ends at. These rows drive real edits through `apply` and fold each
//! one's rows against the document it produced, the way a caller that
//! commits the action does; the fold's rebound rules that no pair of
//! real edits reaches cheaply are held on synthetic rows against a
//! document that holds the names they ask about.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::docm7_union_declare::{block, declared_union, flush_pairs};
use crate::fixture::{ang, fname, insert, len, wall};
use editor_core::{
    Applied, Attr, AttrKind, Dimension, DocEdit, DocParam, DocParamValue, Expr, LoopProgram,
    Maintenance, MaintenanceNet, Node, ParamName, ProfileDoc, ProfileProgram, ProgramStep,
    ProgramTarget, RecipeNodeId, RefusingReach, Rgba8, StableName, apply,
};
use geom_core::Tol;

fn applied(doc: &ProfileDoc, edit: DocEdit<ProfileProgram>) -> Applied<ProfileProgram> {
    apply(doc, &edit, Tol::witness(), &RefusingReach).expect("the edit lands")
}

/// The net of `edits` applied in order from `doc`, and the document
/// they end at; each edit's own rows beside it, for the premises.
fn net_of(
    doc: &ProfileDoc,
    edits: Vec<DocEdit<ProfileProgram>>,
) -> (Vec<Maintenance>, Vec<Vec<Maintenance>>, ProfileDoc) {
    let mut net = MaintenanceNet::new();
    let mut each = Vec::new();
    let mut at = doc.clone();
    for edit in edits {
        let step = applied(&at, edit);
        each.push(step.maintenance.clone());
        net.push(step.maintenance, &step.doc);
        at = step.doc;
    }
    (net.finish(&at), each, at)
}

/// A derived frame on `at` carrying `face`.
fn frame_on(doc: ProfileDoc, at: RecipeNodeId, face: StableName) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Datum(editor_core::Datum::FaceFrame {
            at,
            face,
            spin: ang(0.0),
        }),
    )
}

fn loop_wall(node: RecipeNodeId, loop_index: u32, segment: u32) -> StableName {
    fname(
        node,
        editor_core::RoleSeg::Lateral(editor_core::ProfileEdgeRef {
            loop_index,
            segment,
        }),
    )
}

fn set_value(name: &str, v: f64) -> DocEdit<ProfileProgram> {
    DocEdit::SetDocParamValue {
        name: ParamName::new(name),
        value: DocParamValue::Continuous(v),
    }
}

/// **A rebound that a later edit strands is not a rebound any more.**
/// A 2 × 2 square with a driven circular hole, framed on the hole's
/// wall. `hole_r` 0.3 → 1.5 makes the circle the OUTER loop, and the
/// frame's name is rebound to follow it; 1.5 → 0 leaves a circle that
/// encloses nothing, the numbering cannot be read, and the same name
/// is stranded at a retired coordinate. The action leaves the frame
/// holding a name that denotes nothing, so the rebound's sentence —
/// every carrier "still denotes what it did" — is false of it, and
/// only the strand survives.
#[test]
fn a_rebound_a_later_edit_strands_folds_into_the_strand() {
    let doc = ProfileDoc::empty_derived("net-rebound-then-strand", Tol::witness());
    let (doc, _) = crate::fixture::step(
        doc,
        DocEdit::SetDocParam {
            name: ParamName::new("hole_r"),
            value: DocParam::continuous(Dimension::Length, 0.3),
        },
    );
    let (doc, plane) = insert(doc, crate::fixture::xy_frame());
    let pt = |x: f64, y: f64| [len(x), len(y)];
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![
                LoopProgram::Chain(vec![
                    ProgramStep::At(pt(0.0, 0.0)),
                    ProgramStep::LineTo(ProgramTarget::Point(pt(2.0, 0.0))),
                    ProgramStep::LineTo(ProgramTarget::Point(pt(2.0, 2.0))),
                    ProgramStep::LineTo(ProgramTarget::Point(pt(0.0, 2.0))),
                    ProgramStep::LineTo(ProgramTarget::Start),
                ]),
                LoopProgram::Circle {
                    centre: pt(1.0, 1.0),
                    radius: Expr::param(ParamName::new("hole_r"), Dimension::Length),
                },
            ],
        }),
    );
    let (doc, ext) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let (doc, on_hole) = frame_on(doc, ext, loop_wall(ext, 1, 0));

    let (net, each, _) = net_of(
        &doc,
        vec![set_value("hole_r", 1.5), set_value("hole_r", 0.0)],
    );
    let [first, second] = &each[..] else {
        panic!("two edits")
    };
    let Some(Maintenance::Rebound { to: moved, .. }) = first.iter().find(
        |row| matches!(row, Maintenance::Rebound { from, .. } if *from == loop_wall(ext, 1, 0)),
    ) else {
        panic!("the premise: the jump rebinds the hole's name: {first:?}")
    };
    let stranded = second
        .iter()
        .find(|row| matches!(row, Maintenance::Strand { node, .. } if *node == on_hole))
        .unwrap_or_else(|| {
            panic!("the premise: the collapse strands the frame's name: {second:?}")
        });
    assert!(
        !second
            .iter()
            .any(|row| matches!(row, Maintenance::Rebound { from, .. } if from == moved)),
        "the premise: the second edit strands the moved name rather than moving it on"
    );
    assert!(
        net.contains(stranded),
        "the strand is true of the end: {net:?}"
    );
    assert!(
        !net.iter().any(
            |row| matches!(row, Maintenance::Rebound { from, .. } if *from == loop_wall(ext, 1, 0))
        ),
        "the rebound of the frame's name is not: {net:?}"
    );
}

/// **A strand survives only while its carrier still holds the stranded
/// name.** Deleting a block strands the frame's name on it; a `Rebind`
/// in the same action repairs it onto the kept block, so the action
/// strands nothing. The strand alone, without the repair, survives.
#[test]
fn a_strand_a_later_rebind_repairs_is_not_reported() {
    let doc = ProfileDoc::empty_derived("net-strand-rebind", Tol::witness());
    let (doc, kept) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let named = fname(victim, wall(0));
    let (doc, carrier) = frame_on(doc, kept, named.clone());

    let (alone, _, _) = net_of(&doc, vec![DocEdit::DeleteNode { id: victim }]);
    assert_eq!(
        alone,
        vec![Maintenance::Strand {
            node: carrier,
            name: named.clone(),
        }]
    );
    let (repaired, each, _) = net_of(
        &doc,
        vec![
            DocEdit::DeleteNode { id: victim },
            DocEdit::Rebind {
                from: named,
                to: fname(kept, wall(0)),
            },
        ],
    );
    assert_eq!(each[0], alone, "the premise: the delete strands");
    assert_eq!(
        repaired,
        Vec::new(),
        "the carrier holds a live name at the end"
    );
}

/// **A strand whose carrier the action deleted is not reported.**
#[test]
fn a_strand_on_a_carrier_the_action_deleted_is_not_reported() {
    let doc = ProfileDoc::empty_derived("net-strand-carrier-gone", Tol::witness());
    let (doc, kept) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, carrier) = frame_on(doc, kept, fname(victim, wall(0)));
    let (net, each, _) = net_of(
        &doc,
        vec![
            DocEdit::DeleteNode { id: victim },
            DocEdit::DeleteNode { id: carrier },
        ],
    );
    assert_eq!(each[0].len(), 1, "the premise: the first delete strands");
    assert_eq!(net, Vec::new());
}

/// **An appearance strand survives only while the store holds its
/// key.** Deleting the painted block strands the paint; clearing it in
/// the same action leaves nothing stranded.
#[test]
fn an_appearance_strand_a_later_clear_removes_is_not_reported() {
    let doc = ProfileDoc::empty_derived("net-appearance-clear", Tol::witness());
    let (doc, _) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let painted = fname(victim, wall(0));
    let doc = applied(
        &doc,
        DocEdit::SetAppearance {
            name: painted.clone(),
            attr: Attr::Color(Rgba8::opaque(200, 30, 30)),
        },
    )
    .doc;
    let (alone, _, _) = net_of(&doc, vec![DocEdit::DeleteNode { id: victim }]);
    assert_eq!(
        alone,
        vec![Maintenance::StrandedAppearance {
            name: painted.clone()
        }]
    );
    let (cleared, _, _) = net_of(
        &doc,
        vec![
            DocEdit::DeleteNode { id: victim },
            DocEdit::ClearAppearance {
                name: painted,
                kind: AttrKind::Color,
            },
        ],
    );
    assert_eq!(cleared, Vec::new());
}

/// **An orphan survives only while nothing consumes the declaration.**
/// Deleting the union orphans its declaration; a new union over it in
/// the same action consumes it again, so nothing is orphaned. And the
/// cascade — the union, then the declaration itself — leaves no
/// declaration to be orphaned.
#[test]
fn an_orphan_a_later_edit_consumes_or_deletes_is_not_reported() {
    let doc = ProfileDoc::empty_derived("net-orphan", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, union, decl) = declared_union(doc, &[a, b], flush_pairs((a, a), (b, b)));

    let (alone, _, _) = net_of(&doc, vec![DocEdit::DeleteNode { id: union }]);
    assert_eq!(alone, vec![Maintenance::OrphanedDeclare { declare: decl }]);
    let (reconsumed, _, _) = net_of(
        &doc,
        vec![
            DocEdit::DeleteNode { id: union },
            DocEdit::InsertNode {
                node: Node::Union {
                    members: vec![a, b],
                    declare: Some(decl),
                },
            },
        ],
    );
    assert_eq!(
        reconsumed,
        Vec::new(),
        "the declaration has a consumer again"
    );
    let (cascaded, _, _) = net_of(
        &doc,
        vec![
            DocEdit::DeleteNode { id: union },
            DocEdit::DeleteNode { id: decl },
        ],
    );
    assert_eq!(cascaded, Vec::new(), "the declaration went with the action");
}

// ---------------------------------------------------------------------
// The rebound fold, on synthetic rows.
// ---------------------------------------------------------------------

/// A document whose payloads hold three face names on one node's walls
/// 0, 1 and 2 — the spellings the synthetic rebounds move between —
/// and that node's id.
fn holding_three_walls() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("net-synthetic", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, _) = frame_on(doc, body, fname(body, wall(0)));
    let (doc, _) = frame_on(doc, body, fname(body, wall(1)));
    let (doc, _) = frame_on(doc, body, fname(body, wall(2)));
    (doc, body)
}

fn moved(node: RecipeNodeId, from: u32, to: u32) -> Maintenance {
    Maintenance::Rebound {
        from: fname(node, wall(from)),
        to: fname(node, wall(to)),
    }
}

/// **A name moved by one edit and moved on by a later one is ONE
/// move**, and a move that ends where it began is none.
#[test]
fn rebounds_across_edits_fold_into_the_actions_move() {
    let (doc, n) = holding_three_walls();
    let mut net = MaintenanceNet::new();
    net.push(vec![moved(n, 0, 2)], &doc);
    net.push(vec![moved(n, 2, 1)], &doc);
    assert_eq!(net.finish(&doc), vec![moved(n, 0, 1)]);

    let mut round = MaintenanceNet::new();
    round.push(vec![moved(n, 0, 2)], &doc);
    round.push(vec![moved(n, 2, 0)], &doc);
    assert_eq!(round.finish(&doc), Vec::new());
}

/// **One edit's rebounds are one map**: a swap is two moves, and
/// neither folds into the other.
#[test]
fn one_edits_swap_is_two_moves() {
    let (doc, n) = holding_three_walls();
    let swap = vec![moved(n, 0, 2), moved(n, 2, 0)];
    let mut net = MaintenanceNet::new();
    net.push(swap.clone(), &doc);
    assert_eq!(net.finish(&doc), swap);
}

/// **A later edit that lands another name on a rebound's spelling
/// ended that rebound**: an injective map lands nothing on a spelling
/// it kept, so the earlier name was not kept there.
#[test]
fn a_later_rebound_onto_the_same_spelling_ends_the_earlier_one() {
    let (doc, n) = holding_three_walls();
    let mut net = MaintenanceNet::new();
    net.push(vec![moved(n, 0, 2)], &doc);
    net.push(vec![moved(n, 1, 2)], &doc);
    assert_eq!(net.finish(&doc), vec![moved(n, 1, 2)]);
}

/// **A rebound whose spelling no carrier holds after a later edit
/// ended there**, and is not reported.
#[test]
fn a_rebound_no_carrier_holds_any_more_is_not_reported() {
    let (doc, n) = holding_three_walls();
    let mut net = MaintenanceNet::new();
    net.push(vec![moved(n, 0, 7)], &doc);
    net.push(Vec::new(), &doc);
    assert_eq!(net.finish(&doc), Vec::new());
}

/// **Two surviving rebounds naming one spelling is a bug, and says
/// so** — no accepted edit merges two names, so the fold refuses to
/// carry a report that claims one did.
#[test]
#[should_panic(expected = "two surviving rebounds name one spelling")]
fn two_rebounds_onto_one_spelling_is_refused_loudly() {
    let (doc, n) = holding_three_walls();
    MaintenanceNet::new().push(vec![moved(n, 0, 2), moved(n, 1, 2)], &doc);
}
