//! **A node's content key at a symbolic scalar hashes each input's
//! expression beside its number** (DESIGN `[determinism]`: equal keys
//! mean equal inputs, and at `Sym` an input is a number and the
//! expression the node's decisions are proved over).
//!
//! The fixture is one slab whose depth reads one of two variables that
//! hold the same value and the same tolerance: as numbers the two
//! spellings are one input, as expressions they are two.
//!
//! And a value with no expression, which the key hashes by its name, is
//! named by the node that minted it (`geom_core::sym::mint`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::{
    CancelToken, ContentKey, Dimension, Distribution, DocEdit, EvalOptions, EvalScalar, Evaluation,
    Formula, FreeVar, LoopProgram, NamingKey, Node, OwnRecord, ProfileDoc, ProfileProgram,
    RecipeNodeId, SlotId, SlotValue, UnitSym, VarDecl, VarName, evaluate,
};
use geom_core::{Interval, Sym, SymBudget, Tol};

use crate::fixture::{Recorder, step};

fn toleranced(name: &'static str) -> DocEdit<ProfileProgram> {
    DocEdit::DeclareVar {
        name: VarName::from_static(name),
        def: VarDecl::Free(FreeVar::Continuous {
            dim: Dimension::Length,
            value: 1.0,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(Distribution::Uniform {
                lo: -half_width(),
                hi: half_width(),
            }),
        }),
    }
}

/// A tolerance narrow enough that the slab builds over its box at
/// plain `Interval`, which sees through no identity.
fn half_width() -> f64 {
    Tol::witness().eps() / 64.0
}

fn reads(name: &'static str) -> Formula {
    Formula::named(VarName::from_static(name), Dimension::Length)
}

/// A unit square extruded by `a`, in a document that also declares `b`
/// with the same value and tolerance; and the extrude.
fn slab_by_a() -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    r.push(toleranced("a"));
    r.push(toleranced("b"));
    let profile = r.insert(Node::Profile(ProfileProgram {
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]).unwrap(),
        ],
        ids: Vec::new(),
        on: None,
    }));
    let extrude = r.insert(Node::Extrude {
        profile: profile.into(),
        distance: reads("a"),
    });
    (r.doc, extrude)
}

/// The same document with the extrude's depth reading `b`: one edit,
/// so the node ids are the same.
fn slab_by_b(doc: ProfileDoc, extrude: RecipeNodeId) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetParam {
            node: extrude,
            slot: SlotId::Distance,
            value: SlotValue::Formula(reads("b")),
            fresh: Vec::new(),
        },
    )
    .0
}

/// The whole tolerance box of `doc`: both variables over `1 ± ε/64`.
fn over_the_box(doc: &ProfileDoc) -> EvalOptions {
    EvalOptions {
        param_box: Some(Arc::new(ParamBox::of(&analyzed_box(
            doc,
            &AnalysisPolicy::default(),
        )))),
        ..EvalOptions::default()
    }
}

/// Each variable a symbol at its nominal: the symbolic lane with no box.
fn as_symbols() -> EvalOptions {
    EvalOptions {
        symbols: true,
        ..EvalOptions::default()
    }
}

fn session<R>(f: impl FnOnce() -> R) -> R {
    geom_core::sym::with_session(
        SymBudget {
            max_terms: 4096,
            max_degree: 128,
        },
        f,
    )
    .0
}

fn run<T: OwnRecord>(
    doc: &ProfileDoc,
    prior: Option<&Evaluation<T>>,
    opts: &EvalOptions,
) -> Evaluation<T> {
    evaluate(doc, prior, &CancelToken::new(), opts, Tol::witness())
}

fn keys<T: EvalScalar>(ev: &Evaluation<T>, node: RecipeNodeId) -> (ContentKey, NamingKey) {
    let value = ev
        .value(node)
        .unwrap_or_else(|| panic!("node {node:?} built no value"));
    (value.content_key, value.naming_key)
}

/// The extrude's keys under each spelling of its depth, at `T`.
fn both_spellings<T: OwnRecord>(
    opts: impl Fn(&ProfileDoc) -> EvalOptions,
) -> [(ContentKey, NamingKey); 2] {
    let (by_a, extrude) = slab_by_a();
    let by_b = slab_by_b(by_a.clone(), extrude);
    [by_a, by_b].map(|doc| keys(&run::<T>(&doc, None, &opts(&doc)), extrude))
}

/// Two inputs equal as numbers and different as expressions are one
/// key at a numeric scalar, where the number is the whole input, and
/// two keys at a symbolic one; the naming key, which embeds the content
/// key, follows.
///
/// Mutant: `ContentBits for Sym` feeding the wrapped value alone makes
/// the two symbolic rows equal.
#[test]
fn equal_numbers_under_two_expressions_are_one_key_at_a_number_and_two_at_a_symbol() {
    let [a, b] = both_spellings::<f64>(|_| EvalOptions::default());
    assert_eq!(a, b, "f64: the depth is a number, the same one");
    let [a, b] = both_spellings::<Interval>(over_the_box);
    assert_eq!(a, b, "Interval: the depth is an enclosure, the same one");

    let [a, b] = both_spellings::<Sym<f64>>(|_| as_symbols());
    assert_ne!(a.0, b.0, "Sym<f64>: content key, two symbols");
    assert_ne!(a.1, b.1, "Sym<f64>: naming key, two symbols");
    let [a, b] = session(|| both_spellings::<Sym<Interval>>(over_the_box));
    assert_ne!(a.0, b.0, "Sym<Interval>: content key, two symbols");
    assert_ne!(a.1, b.1, "Sym<Interval>: naming key, two symbols");
}

/// Equal expressions and equal numbers are equal keys in every replay
/// of one document: in two sessions, and under no session at all. An
/// expression's id is a hash of what it says, never of the session that
/// built it, and the drive, the stackup's tie and a reader with no tier
/// each replay under a session of their own kind. The document is the
/// holed slab, whose extrude mints values with no expression: their
/// names are the session's inside one and the thread's outside, and no
/// key feeds one.
///
/// Mutant: a word of the session fed beside the id (whether one is
/// installed) makes the replay under no session key apart.
#[test]
fn every_replay_of_one_document_has_one_key_per_node() {
    let doc = holed_slab();
    let opts = over_the_box(&doc);
    let replay = || {
        let ev = run::<Sym<Interval>>(&doc, None, &opts);
        ev.order.iter().map(|&id| keys(&ev, id)).collect::<Vec<_>>()
    };
    let first = session(replay);
    assert_eq!(first.len(), 2, "the profile and the extrude");
    assert_eq!(
        session(replay),
        first,
        "a second session: every node, both keys"
    );
    assert_eq!(replay(), first, "no session: every node, both keys");
}

/// An evaluation at a symbolic scalar handed a prior whose node read an
/// equal number under a different expression runs the op again: the
/// prior's value was built over the other symbol, and a body served
/// from it would carry that symbol's expressions into this build.
///
/// Mutant: `ContentBits for Sym` feeding the wrapped value alone serves
/// the extrude from the prior (`reused == 2`).
#[test]
fn a_prior_built_over_another_expression_of_the_same_number_is_not_served() {
    let (by_a, extrude) = slab_by_a();
    let by_b = slab_by_b(by_a.clone(), extrude);
    let opts = over_the_box(&by_a);
    session(|| {
        let prior = run::<Sym<Interval>>(&by_a, None, &opts);
        assert_eq!((prior.recomputed, prior.reused), (2, 0), "the prior");

        let same = run::<Sym<Interval>>(&by_a, Some(&prior), &opts);
        assert_eq!(
            (same.recomputed, same.reused),
            (0, 2),
            "the same expressions and numbers: both nodes served"
        );

        let other = run::<Sym<Interval>>(&by_b, Some(&prior), &opts);
        assert_eq!(
            (other.recomputed, other.reused),
            (1, 1),
            "the profile reads neither variable and is served; the extrude reads `b` where \
             the prior read `a`, and runs"
        );
        assert_ne!(
            keys(&other, extrude),
            keys(&prior, extrude),
            "the extrude's keys"
        );
    });
}

/// A plate with a round hole, extruded by `a`: the hole's wall projects
/// its pcurves, which re-enters computed samples as values with no
/// expression.
fn holed_slab() -> ProfileDoc {
    let mut r = Recorder::new();
    r.push(toleranced("a"));
    let profile = r.insert(Node::Profile(ProfileProgram {
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]).unwrap(),
            LoopProgram::Circle {
                centre: [crate::fixture::len(1.0), crate::fixture::len(1.0)],
                radius: crate::fixture::len(0.25),
            },
        ],
        ids: Vec::new(),
        on: None,
    }));
    r.insert(Node::Extrude {
        profile: profile.into(),
        distance: reads("a"),
    });
    r.doc
}

/// The names of every value with no expression one session minted.
fn names_minted(work: impl FnOnce()) -> std::collections::BTreeSet<u128> {
    geom_core::sym::profile::start_profile();
    session(work);
    let mut sessions = geom_core::sym::profile::take_profile().opaque_ids;
    assert_eq!(sessions.len(), 1, "one session");
    sessions.remove(0)
}

/// The values a node mints with no expression are named by the node,
/// so a replay that minted one more value before its nodes ran holds
/// the same names and that one more: the name a session's first mint
/// under no node takes.
///
/// Mutants: a name counted over the whole replay, or `eval_node` opening
/// no scope for its node, shifts every node's names by the early mint,
/// and the name the second replay adds is the count's last, not the
/// root's first.
#[test]
fn a_nodes_values_with_no_expression_keep_their_names_whatever_was_minted_before() {
    let doc = holed_slab();
    let opts = over_the_box(&doc);
    let replay = |early: bool| {
        names_minted(|| {
            if early {
                let _ = Sym::<Interval>::opaque(Interval::from_bounds(7.0, 7.0));
            }
            let ev = run::<Sym<Interval>>(&doc, None, &opts);
            assert_eq!((ev.recomputed, ev.reused), (2, 0), "both nodes ran");
            assert!(
                ev.order.iter().all(|&id| ev.value(id).is_some()),
                "both nodes built"
            );
        })
    };
    let alone = names_minted(|| {
        let _ = Sym::<Interval>::opaque(Interval::from_bounds(7.0, 7.0));
    });
    assert_eq!(alone.len(), 1, "the root scope's first name");

    let plain = replay(false);
    assert!(!plain.is_empty(), "the hole's wall mints");
    assert!(plain.is_disjoint(&alone), "no node mints the root's name");
    let after_one = replay(true);
    assert_eq!(
        after_one.difference(&plain).copied().collect::<Vec<_>>(),
        alone.iter().copied().collect::<Vec<_>>(),
        "the early mint's name is all the second replay adds"
    );
    assert!(plain.is_subset(&after_one), "and every node's name is kept");
}
