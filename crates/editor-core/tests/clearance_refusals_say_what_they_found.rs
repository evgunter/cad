//! **A clearance refusal says what it found**: the pair it could not
//! decide with the body each face is on, the cause a lower door handed
//! it, and what a scope's name denotes before how many entities answer
//! to it.
//!
//! The rendering rows pin each sentence whole, so a change of words is
//! a change of this file. The door rows drive the selection refusals
//! through `clearance` from documents that reach them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;

use editor_core::analysis::ParamBox;
use editor_core::clearance::{
    CertificateArm, ClearanceRefusal, ClearanceVerdict, EngineRefusal, FaceScope,
    MinSeparationRefusal, Selection, SelectionRefusal, SharedCell, Which, clearance,
};
use editor_core::{EntityKind, Entry, EvalOptions, Node, ProfileDoc, RecipeNodeId, StableName};
use geom_core::{BandError, Tol};
use topo::entity::FaceKey;

use fixture::{insert, len};

/// A node whose tag is `3fa9c1d2a0b1`.
fn tagged() -> RecipeNodeId {
    RecipeNodeId::new(0, 0x3fa9_c1d2_a0b1_0042)
}

/// The selection refusal a clearance between `a` and `b` answers.
fn refused_selection(doc: &ProfileDoc, a: &Selection, b: &Selection) -> SelectionRefusal {
    let leaf = ParamBox::from_axes(BTreeMap::new());
    let report = clearance(doc, &leaf, a, b, 0.1, Tol::witness());
    match report.verdict() {
        ClearanceVerdict::Refused(ClearanceRefusal::Selection(r)) => r.clone(),
        other => panic!("expected a selection refusal, got {other:?}"),
    }
}

/// **Two equal arena keys of two bodies are two subjects.** A `FaceKey`
/// keys one body's arena, so a pair across two bodies — two members
/// of one family, here — routinely carries one key value twice. The
/// engine names each face with its side, in words and in its payload;
/// the document door names each side as the selection's body.
#[test]
fn a_pair_across_bodies_names_each_face_with_its_side() {
    let k = FaceKey::default();
    let poison = EngineRefusal::PoisonEnclosure { a: k, b: k };
    assert_eq!(
        poison.to_string(),
        "the enclosure between face FaceKey(null) of the first side and face FaceKey(null) of \
         the second side did not evaluate, which is geometry, not a budget",
        "the engine's words name each face's side"
    );
    assert_eq!(
        poison.payload(),
        "first=FaceKey(null) second=FaceKey(null)",
        "the goldening form names each face's side"
    );
    assert_eq!(
        MinSeparationRefusal::PoisonEnclosure { a: k, b: k }.to_string(),
        poison.to_string(),
        "the measure door's refusal is the engine's, in the engine's words"
    );
    let uncertified = EngineRefusal::Uncertified {
        a: k,
        b: k,
        shared: SharedCell::Face(k),
        arm: CertificateArm::Monotone,
    };
    assert_eq!(
        uncertified.to_string(),
        "face FaceKey(null) of the first side and face FaceKey(null) of the second side touch \
         through face FaceKey(null), and the monotone certificate did not discharge them",
        "the injectivity pair names each face's side"
    );
    assert_eq!(
        uncertified.payload(),
        "first=FaceKey(null) second=FaceKey(null) face FaceKey(null) monotone",
        "the injectivity payload names each face's side"
    );
    let node = tagged();
    let across = ClearanceRefusal::Engine {
        refusal: poison,
        sides: [(node, 0), (node, 1)],
    };
    assert_eq!(
        across.to_string(),
        "the enclosure between face FaceKey(null) of body 0 of node 3fa9c1d2a0b1 and face \
         FaceKey(null) of body 1 of node 3fa9c1d2a0b1 did not evaluate, which is geometry, not \
         a budget",
        "the document door names each side as its selection's body"
    );
}

/// **The band refusal carries why the band did not construct**: the
/// two arms want opposite repairs (a smaller tolerance for an overflow,
/// a larger one for a collapse), so the sentence names which one.
/// Rendered rather than driven: no `Tol` constructor in the tree hands
/// out a tolerance whose band fails (`m10_5_clearance_interval`'s door).
#[test]
fn the_band_refusal_says_which_band_failed() {
    let empty = BandError::Empty {
        zero: 1e-310,
        escalate: 1e-310,
    };
    let refusal = ClearanceRefusal::ToleranceHasNoBand(empty);
    assert_eq!(
        refusal.to_string(),
        format!("the run's tolerance admits no linear band: {empty}"),
        "the band's own sentence rides the refusal"
    );
    assert_eq!(
        refusal.payload(),
        format!("{empty:?}"),
        "the goldening form carries the band error"
    );
}

/// **The report's human form says the refusal in words**, not its
/// goldening payload: a second scope naming no face refuses before
/// anything is evaluated, and the line names that scope's body.
#[test]
fn a_refused_report_renders_the_refusal_in_words() {
    let nothing = Selection {
        at: tagged(),
        body: 0,
        faces: FaceScope::Named(Vec::new()),
    };
    let whole = Selection::body_of(RecipeNodeId::new(0, 7));
    let doc = ProfileDoc::empty_derived("cleartext_render", Tol::witness());
    let leaf = ParamBox::from_axes(BTreeMap::new());
    let report = clearance(&doc, &leaf, &whole, &nothing, 0.1, Tol::witness());
    let rendered = report.render();
    assert_eq!(
        rendered.lines().next(),
        Some(
            "REFUSED (empty_scope): the scope of body 0 of node 3fa9c1d2a0b1 names no face, so \
             there is nothing to measure"
        ),
        "the refused line names the second scope: {rendered}"
    );
}

/// **A refusal about one face names its side too**: two bodies'
/// arenas carry the same key values, so a bare key locates nothing in
/// a question over two of them. The measure door says the engine's
/// words with its operands as the sides, and a face name whose row
/// holds another kind of key reads as the kernel defect it is.
#[test]
fn a_one_face_refusal_names_its_side() {
    let k = FaceKey::default();
    let unsupported = EngineRefusal::Unsupported {
        carrier: "a free-form face, which no carrier kind makes injective",
        face: k,
        side: Which::Second,
    };
    assert_eq!(
        unsupported.to_string(),
        "the clearance engine cannot answer for face FaceKey(null) of the second side: it is a \
         free-form face, which no carrier kind makes injective",
        "the words claim no missing window and name the side"
    );
    assert_eq!(
        unsupported.payload(),
        "a free-form face, which no carrier kind makes injective second=FaceKey(null)",
        "the payload names the side"
    );
    assert_eq!(
        EngineRefusal::TorusNotRing {
            face: k,
            side: Which::First,
        }
        .to_string(),
        "face FaceKey(null) of the first side lies on a torus not decided a ring (R > r), so its \
         carrier meets itself",
        "the torus names its side"
    );
    let collapsed = EngineRefusal::CollapsedChart {
        face: k,
        side: Which::Second,
        vertex: topo::VertexKey::default(),
    };
    assert_eq!(
        collapsed.to_string(),
        "the chart of face FaceKey(null) of the second side collapses an edge to its vertex \
         VertexKey(null) near a touching pair, and the monotone certificate reads no collapsed \
         chart",
        "the collapsed chart names its side"
    );
    assert_eq!(
        collapsed.payload(),
        "second=FaceKey(null) VertexKey(null)",
        "the collapsed chart's payload names its side"
    );
    let measured = editor_core::NodeErrorKind::MeasureClearanceRefused(
        MinSeparationRefusal::PoisonEnclosure { a: k, b: k },
    );
    assert_eq!(
        measured.to_string(),
        "the clearance engine refused: the enclosure between face FaceKey(null) of the first \
         operand and face FaceKey(null) of the second operand did not evaluate, which is \
         geometry, not a budget",
        "the measure door says the engine's words, its operands the sides"
    );
    let name = StableName {
        kind: EntityKind::Face,
        node: tagged(),
        path: Vec::new(),
    };
    let defect = SelectionRefusal::KeyOfAnotherKind {
        name: Box::new(name),
        found: EntityKind::Edge,
    }
    .to_string();
    assert!(
        defect.contains(" names a face, and its row in the name table holds an edge")
            && defect.ends_with(geom_core::KERNEL_DEFECT_ENDING),
        "a row against its name's kind is the kernel's defect: {defect}"
    );
}

/// **A node whose value carries no bodies is not a bad index.** The
/// R1 reviewer fixture of WIRE PR 2474, adopted with its authorship:
/// a clearance pointed at a profile node used to answer "carries no
/// body at index 0", a different and false fact, because the door
/// threw away the `InterrogateError` that named the payload family.
#[test]
fn a_clearance_over_a_profile_node_says_the_value_carries_no_bodies() {
    let doc = ProfileDoc::empty_derived("cleartext_no_bodies", Tol::witness());
    let (doc, profile) = insert(
        doc,
        Node::Profile(fixture::desc(vec![fixture::square(0.0, 0.0, 1.0)])),
    );
    let (doc, block) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(2.0),
        },
    );
    let refusal = refused_selection(
        &doc,
        &Selection::body_of(profile),
        &Selection::body_of(block),
    );
    assert_eq!(
        refusal,
        SelectionRefusal::NoBodies {
            node: profile,
            payload: "profile",
        },
        "the payload family, not an index"
    );
    let said = refusal.to_string();
    assert!(
        said.contains("is a profile value and carries no bodies at all") && !said.contains("index"),
        "the words name the family and no index: {said}"
    );
}

/// **The named scope asks what a name denotes before how many entities
/// answer to it**, over the U cutter, whose subtract ties a face. A name
/// nothing answers to is unresolved; a name of another kind is not a
/// face, and the refusal names the kind; a tied face name is a tie,
/// naming every candidate and a tie's recourse, never a name that
/// resolved to nothing.
#[test]
fn a_named_scope_asks_what_a_name_denotes_before_how_many_answer() {
    let (doc, sub) = fixture::u_cutter_tie(ProfileDoc::empty_derived(
        "cleartext_u_cutter",
        Tol::witness(),
    ));
    let ev = fixture::run(&doc, &EvalOptions::default());
    let table = ev.table_at(sub).expect("the U subtract evaluates");
    let tied_of = |kind: EntityKind| {
        table.iter().find_map(|(n, e)| match e {
            Entry::Tied(c) if n.kind == kind => Some((n.clone(), c.len())),
            _ => None,
        })
    };
    let (tied_face, width) = tied_of(EntityKind::Face).expect("the U fixture ties a face");
    let edge = table
        .iter()
        .find_map(|(n, _)| (n.kind == EntityKind::Edge).then(|| n.clone()))
        .expect("the U subtract names an edge");
    let whole = Selection::body_of(sub);
    let scoped = |name: &StableName| Selection {
        at: sub,
        body: 0,
        faces: FaceScope::Named(vec![name.clone()]),
    };

    let refusal = refused_selection(&doc, &scoped(&tied_face), &whole);
    let SelectionRefusal::Tied { name, candidates } = &refusal else {
        panic!("a tied face name is its own refusal, got {refusal:?}");
    };
    assert_eq!(**name, tied_face, "the tie names the tied name");
    assert_eq!(candidates.len(), width, "the tie names every candidate");
    let said = refusal.to_string();
    assert!(
        said.starts_with(&format!("{width} faces answer to "))
            && said.contains("refine the reference until one candidate remains"),
        "the tie's words give the count and a tie's recourse: {said}"
    );
    for c in candidates {
        let editor_core::EntityKey::Face(k) = c.key else {
            panic!("a tied face name's candidates are faces: {c:?}");
        };
        let listed = format!("face {k:?} of body 0");
        assert!(
            said.contains(&listed),
            "the tie's words list {listed}: {said}"
        );
    }

    let refusal = refused_selection(&doc, &scoped(&edge), &whole);
    assert_eq!(
        refusal,
        SelectionRefusal::NotAFace {
            name: Box::new(edge.clone()),
            found: EntityKind::Edge,
        },
        "an edge name is not a face"
    );
    let said = refusal.to_string();
    assert!(
        said.contains(" denotes an edge, and a clearance is measured between faces"),
        "the not-a-face words name the edge: {said}"
    );

    let mut stranger = edge;
    stranger.node = tagged();
    let refusal = refused_selection(&doc, &scoped(&stranger), &whole);
    assert_eq!(
        refusal,
        SelectionRefusal::Unresolved {
            name: Box::new(stranger),
        },
        "a name no row answers to is unresolved, before its kind is asked"
    );
    assert!(
        refusal
            .to_string()
            .starts_with("nothing in the selected node's name table answers to "),
        "the unresolved words: {refusal}"
    );
}

/// **A face of another body is not "not a face"**: a family's
/// members are bodies of one value under one table, and a face name
/// of member 1 handed to a selection of body 0 refuses naming the
/// body its face is on.
#[test]
fn a_face_of_another_output_body_refuses_naming_that_body() {
    let doc = ProfileDoc::empty_derived("cleartext_other_body", Tol::witness());
    let (doc, profile) = insert(
        doc,
        Node::Profile(fixture::desc(vec![fixture::square(0.0, 0.0, 1.0)])),
    );
    let (doc, cube) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    let (doc, _, row) = fixture::linear(doc, cube, [1.0, 0.0, 0.0], 3.0, 2);
    let ev = fixture::run(&doc, &EvalOptions::default());
    let table = ev.table_at(row).expect("the row evaluates");
    let elsewhere = table
        .iter()
        .find_map(|(n, e)| match e {
            Entry::Unique(ent) if n.kind == EntityKind::Face && ent.body == 1 => Some(n.clone()),
            _ => None,
        })
        .expect("member 1's faces are named");
    let scope = Selection {
        at: row,
        body: 0,
        faces: FaceScope::Named(vec![elsewhere.clone()]),
    };
    let refusal = refused_selection(&doc, &scope, &Selection::body_of(row));
    assert_eq!(
        refusal,
        SelectionRefusal::OtherBody {
            name: Box::new(elsewhere),
            body: 1,
        },
        "the body the face is on"
    );
    let said = refusal.to_string();
    assert!(
        said.ends_with(" denotes a face of body 1, not of the selected body"),
        "the words name the body: {said}"
    );
}
