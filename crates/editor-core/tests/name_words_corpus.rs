//! **A name's words tell it apart, within a readable sentence**, over
//! every name the corpus mints (`work/recipe/names-render-a-faces-leaf-
//! role-in-words.md`, "A name's words tell it apart", ruled on #3906).
//!
//! Every node's name table of every corpus document, evaluated once:
//!
//! - the full form — what a speaker holding no table says — never says
//!   two names of one table alike, from the document or by tag;
//! - the words a speaker holding the table says ([`Speaker::within`])
//!   never say two entities of one body alike;
//! - every refusal that forwards a name through a speaker, said with
//!   the longest names that speaker says over the corpus, meets the
//!   refusal standard (`test_utils::refusal::problems`), but for the
//!   rows [`OVER_BUDGET`] admits by name.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::corpus;
use crate::fixture;
use editor_core::{
    Diagnosis, EntityKind, Entry, EvalOptions, Evaluation, FaceName, HitTestError,
    InterrogateError, NodeErrorKind, PickHit, RecipeEditRef, RecipeNodeId, ResolveError, Said,
    SelectRefusal, Speaker, StableName,
};

/// **The rows admitted over the word budget, and the most words each
/// may render**: a ratchet, so a row that grows fails and a row that
/// shrinks lowers its number. Said with the corpus's longest scoped
/// names, a refusal's own prose and a name of 40-odd words do not fit
/// 75 words; filed as
/// `work/recipe/refusals-with-the-longest-scoped-names-overrun-the-budget.md`.
const OVER_BUDGET: &[(&str, usize)] = &[
    ("ResolveError::Vanished", 135),
    ("SelectRefusal::InBand", 96),
    ("SelectRefusal::TiedDisagrees", 81),
    ("SelectRefusal::Unreadable", 76),
    ("SelectRefusal::PairInBand", 137),
    ("NodeErrorKind::CrossingUnverified", 124),
    ("HitTestError::Ambiguous", 133),
];

/// A name the scoped speaker said, with the document it was said from.
struct Scoped {
    words: usize,
    doc: usize,
    name: StableName,
}

#[test]
fn every_corpus_name_reads_apart_and_forwards_within_the_refusal_budget() {
    let mut docs = corpus::documents();
    docs.push(corpus::cup::document());
    docs.push(corpus::vessel::document());
    let evals: Vec<Evaluation<f64>> = docs
        .iter()
        .map(|d| fixture::run(&d.doc, &EvalOptions::default()))
        .collect();
    let mut alike = Vec::new();
    let mut scoped_alike = Vec::new();
    let mut longest: Vec<Scoped> = Vec::new();
    let mut longest_full = (0, String::new());
    for (di, (d, ev)) in docs.iter().zip(&evals).enumerate() {
        let full = Speaker::of(&d.doc);
        let scoped = full.within(ev);
        for &id in d.doc.order() {
            let Some(value) = ev.value(id) else { continue };
            let mut by_doc: BTreeMap<String, usize> = BTreeMap::new();
            let mut by_tag: BTreeMap<String, usize> = BTreeMap::new();
            let mut by_body: BTreeMap<(u32, String), usize> = BTreeMap::new();
            for (name, entry) in value.name_table.iter() {
                let said = full.name(name).to_string();
                let words = said.split_whitespace().count();
                if words > longest_full.0 {
                    longest_full = (words, said.clone());
                }
                *by_doc.entry(said).or_default() += 1;
                *by_tag.entry(name.to_string()).or_default() += 1;
                // A name another node holds is said within that node's
                // table, which a pass-through repeats row for row; a
                // tied name is one name over several entities.
                let (Entry::Unique(row), true) = (entry, name.node == id) else {
                    continue;
                };
                let said = scoped.name(name).to_string();
                longest.push(Scoped {
                    words: said.split_whitespace().count(),
                    doc: di,
                    name: name.clone(),
                });
                *by_body.entry((row.body, said)).or_default() += 1;
            }
            for (form, groups) in [("from the document", &by_doc), ("by tag", &by_tag)] {
                alike.extend(
                    groups
                        .iter()
                        .filter(|(_, n)| **n > 1)
                        .map(|(said, n)| format!("[{}] {form}, x{n}: {said}", d.name)),
                );
            }
            scoped_alike.extend(
                by_body
                    .iter()
                    .filter(|(_, n)| **n > 1)
                    .map(|((body, said), n)| format!("[{}] body {body}, x{n}: {said}", d.name)),
            );
        }
    }
    assert!(
        alike.is_empty(),
        "the full form says distinct names of one table alike:\n{}",
        alike.join("\n")
    );
    assert!(
        scoped_alike.is_empty(),
        "the table-scoped words say distinct entities of one body alike:\n{}",
        scoped_alike.join("\n")
    );
    println!(
        "the longest full form, reported and not gated ({} words): {}",
        longest_full.0, longest_full.1
    );

    longest.sort_by_key(|said| core::cmp::Reverse(said.words));
    // A face refusal is said over the two longest face names one
    // document holds; a resolve refusal, of any kind, over the two
    // longest names.
    let mut rows = Vec::new();
    for faces in [false, true] {
        let mut kept = longest
            .iter()
            .filter(|said| !faces || said.name.kind == EntityKind::Face);
        let one = kept.next().expect("the corpus names a face");
        let two = kept
            .find(|other| other.doc == one.doc)
            .expect("the longest-named document holds two names");
        let (doc, ev) = (&docs[one.doc].doc, &evals[one.doc]);
        let by = Speaker::of(doc).within(ev);
        println!(
            "the longest scoped{} names: {} words, {} words: {} | {}",
            if faces { " face" } else { "" },
            one.words,
            two.words,
            by.name(&one.name),
            by.name(&two.name)
        );
        rows.extend(
            refusals(&one.name, &two.name, by)
                .into_iter()
                .filter(|(row, _)| row.starts_with("ResolveError::") != faces),
        );
    }
    let mut over = Vec::new();
    for (row, text) in &rows {
        let words = text.split_whitespace().count();
        let allowed = OVER_BUDGET
            .iter()
            .find_map(|(admitted, most)| (admitted == row).then_some(*most));
        println!("{row}: {words} words");
        match allowed {
            None if words > test_utils::refusal::BUDGET => {
                over.push(format!(
                    "{row} renders {words} words, over the budget: {text}"
                ));
            }
            Some(most) if words > most => {
                over.push(format!(
                    "{row} renders {words} words, over its admitted {most}: {text}"
                ));
            }
            Some(most) if words <= test_utils::refusal::BUDGET || words < most => {
                over.push(format!(
                    "{row} renders {words} words; lower its admission of {most}"
                ));
            }
            _ => {}
        }
    }
    assert!(
        over.is_empty(),
        "a refusal forwarding the corpus's longest scoped names misses the budget:\n{}",
        over.join("\n")
    );
}

/// Every refusal a speaker says that forwards a name, naming `a` (and
/// `b` where it names two). The three kind refusals of `NodeErrorKind`
/// carry a kind only evaluation constructs, so they are not here; each
/// says its name in a sentence shorter than `InBand`'s.
fn refusals(a: &StableName, b: &StableName, by: Speaker<'_>) -> Vec<(&'static str, String)> {
    let face = |n: &StableName| {
        FaceName::new(StableName {
            kind: EntityKind::Face,
            node: n.node,
            path: n.path.clone(),
        })
        .expect("a face kind is a face name")
    };
    let band = || geom_core::Indeterminate {
        margin: geom_core::MarginDiag::value(3e-11),
        band: geom_core::Band::new(1e-12, 1e-9).expect("zero < escalate"),
        predicate: Some("bool_plane_offset"),
        terminal_sliver: false,
    };
    let hit = |name: &StableName| PickHit {
        name: name.clone(),
        node: name.node,
        body: 0,
        t: 1.0,
        t_lo: 1.0,
        t_hi: 1.0,
        point: geom_core::Point3::new(0.0, 0.0, 0.0),
    };
    let said = |value: &dyn editor_core::Say| Said(value, by).to_string();
    vec![
        (
            "ResolveError::Vanished",
            said(&ResolveError::Vanished {
                name: a.clone(),
                diagnosis: Diagnosis::Cascade { through: b.clone() },
                last_good: None,
            }),
        ),
        (
            "ResolveError::NodeGone",
            said(&ResolveError::NodeGone {
                name: a.clone(),
                edit: RecipeEditRef::NodeDeleted { node: a.node },
            }),
        ),
        (
            "SelectRefusal::InBand",
            said(&SelectRefusal::InBand {
                name: Box::new(a.clone()),
                predicate: "sel_datum_distance",
                source: band(),
            }),
        ),
        (
            "SelectRefusal::TiedDisagrees",
            said(&SelectRefusal::TiedDisagrees {
                name: Box::new(a.clone()),
                matched: 1,
                candidates: 3,
            }),
        ),
        (
            "SelectRefusal::Unreadable",
            said(&SelectRefusal::Unreadable {
                name: Box::new(a.clone()),
                error: InterrogateError::WholeBody,
            }),
        ),
        (
            "SelectRefusal::PairInBand",
            said(&SelectRefusal::PairInBand {
                pair: Box::new((a.clone(), b.clone())),
                predicate: "bool_plane_offset",
                source: band(),
            }),
        ),
        (
            "NodeErrorKind::CrossingUnverified",
            said(&NodeErrorKind::CrossingUnverified {
                instance: RecipeNodeId(test_utils::refusal::tagged(1)),
                outer: Box::new(face(a)),
                name: Box::new(b.clone()),
            }),
        ),
        (
            "HitTestError::Ambiguous",
            said(&HitTestError::Ambiguous {
                hits: vec![hit(a), hit(b)],
            }),
        ),
    ]
}

/// **A kept name spoken again after its step was dropped says the step
/// by its tag**, never the row it sat at, which is now another step's;
/// a kept step that only moved is said at its new row. Either way the
/// name reads as it would spoken fresh from the new document.
#[test]
fn respoken_after_a_dropped_step() {
    use crate::corpus::reshaped_rod::{CREASE, bump_ids, lateral_edge, rod_ids, rod_loop};
    use editor_core::{
        DocEdit, Node, ProfileDoc, ProfileProgram, ProfileVertexRef, RefusingReach, RoleSeg, apply,
    };

    let tol = fixture::tol();
    let doc = ProfileDoc::empty_derived("respoken-after-a-dropped-step", tol);
    let (doc, plane) = fixture::insert(doc, fixture::xy_frame());
    let (doc, profile) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![rod_loop(false)],
            ids: Vec::new(),
        }),
    );
    let (doc, rod) = fixture::insert(
        doc,
        Node::Extrude {
            profile,
            distance: fixture::len(2.0),
        },
    );
    let crease = lateral_edge(&doc, rod, CREASE);
    let [RoleSeg::LateralEdge(ProfileVertexRef::Piece { step, .. })] = crease.path.as_slice()
    else {
        panic!("the crease is a lateral edge over a step's piece: {crease:?}");
    };
    let row = rod_ids(&doc, profile)
        .iter()
        .position(|id| id == step)
        .expect("the plain rod draws the crease's step");
    let kept = doc.spoken_name(&crease);
    assert!(
        kept.to_string().contains(&format!("loop 0 step {row} ")),
        "spoken from the plain rod, the step is its row: {kept}"
    );
    let reshape = |drop: bool| {
        let mut ids = bump_ids(&doc, profile);
        if drop {
            let at = ids[0]
                .iter()
                .position(|id| *id == Some(*step))
                .expect("the bump keeps the crease's step");
            ids[0][at] = None;
        }
        apply(
            &doc,
            &DocEdit::SetProgram {
                node: profile,
                loops: vec![rod_loop(true)],
                ids,
            },
            tol,
            &RefusingReach,
        )
        .expect("the reshaping applies")
        .doc
    };

    let dropped = reshape(true);
    let respoken = kept.respoken(&dropped).to_string();
    assert!(
        !respoken.contains(&format!("loop 0 step {row} ")),
        "a dropped step keeps no row another step now holds: {respoken}"
    );
    assert!(
        respoken.contains(&format!("the profile step {step}")),
        "a dropped step is said by its tag: {respoken}"
    );
    assert_eq!(respoken, dropped.spoken_name(&crease).to_string());

    let moved = reshape(false);
    let respoken = kept.respoken(&moved).to_string();
    assert!(
        respoken.contains(&format!("loop 0 step {} ", row + 2)),
        "a kept step that moved is said at its new row: {respoken}"
    );
    assert_eq!(respoken, moved.spoken_name(&crease).to_string());
}
