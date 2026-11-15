//! **A leaf the driver certifies holds every member of its box**, read
//! off the walls a document's extrude stores. Two slabs, each a
//! four-sided polygon extruded:
//!
//! - **the identity hop**: corners `(0, 0) (w, 0) (w, 1) (0, 1)`, `w =
//!   1 ± 0.05`. The moving side `(w, 0) → (w, 1)` differs by `w − w`,
//!   an identity, so its director is the axis wherever the tier proves
//!   that zero;
//! - **the quotient hop**: corners `(0, 0) (w, 0) (v, 1) (0, 1)` with
//!   `w` and a second variable `v` both `1 ± 1e-10`, equal at the
//!   nominal and not an identity. Over the box `v − w` straddles zero at
//!   the mint's least band, so its director is the chord over its length
//!   and carries the box's width; at a corner member it reads definite.
//!
//! The drive certifies leaves of each, and each member of a leaf (its
//! corners and middle) is built at `f64`. Every member's stored wall
//! normal lies, face by face, in what the two sessions a drive's
//! readers open store for it at `Sym<Interval>`: the drive's own eager
//! session over the whole leaf, and the deferring session the
//! coincidence door replays in, at the member itself (that replay runs
//! at a point; over a box a deferring session proves nothing as it
//! decides, so the profile's simplicity gates escalate there).
//!
//! The mutants, each run: the quotient row goes red when `profile`'s
//! axis mint (`seg::axis_or`) takes an escalated component as the zero
//! of an axis — the whole leaf certifies with the hop's wall normal
//! stored exactly, and a corner member's leans by `2e-10`; the identity
//! row goes red when the proven axis takes `+1` whatever the sign the
//! other component read, but through its first assertion, not a member:
//! the box lane stores the side `(w, 1) → (0, 1)` running the wrong way
//! and the drive certifies no leaf (16 refused). No lane-dependent
//! mutant of the identity hop reaches a certified leaf that a member
//! then reads; that row holds the proven axis's lane, not a mutant's.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::fixture::{Recorder, len};
use editor_core::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box};
use editor_core::drive::{DriveConfig, drive};
use editor_core::{
    CancelToken, Dimension, Distribution, DocEdit, EvalOptions, Evaluation, Formula, FreeVar,
    LoopProgram, Node, ProfileDoc, ProfileProgram, RecipeNodeId, UnitSym, ValuePayload, VarName,
    evaluate,
};
use geom::Surface;
use geom_core::sym::deferred::with_deferred_session;
use geom_core::sym::with_session_rules;
use geom_core::{Bounds, Interval, Sym, SymBudget, SymRetry, SymRules, Tol, Vec3};

/// The drive's own budget.
const BUDGET: SymBudget = SymBudget {
    max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
    max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
};

fn declare(r: &mut Recorder, name: &'static str, half: f64) {
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static(name),
        def: editor_core::VarDecl::Free(FreeVar::Continuous {
            dim: Dimension::Length,
            value: 1.0,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(Distribution::Uniform {
                lo: -half,
                hi: half,
            }),
        }),
    });
}

fn named(name: &'static str) -> Formula {
    Formula::named(VarName::from_static(name), Dimension::Length)
}

/// The slab, its far top corner at `x = v` where `two` names a second
/// variable and at `x = w` otherwise.
fn slab(two: bool) -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    // The quotient hop's boxes sit inside the band, so `v − w` reads
    // Zero over the whole box with nothing to prove it.
    let half = if two { 1e-10 } else { 0.05 };
    declare(&mut r, "w", half);
    if two {
        declare(&mut r, "v", half);
    }
    let top = if two { named("v") } else { named("w") };
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::polygon_expr([
            [len(0.0), len(0.0)],
            [named("w"), len(0.0)],
            [top, len(1.0)],
            [len(0.0), len(1.0)],
        ])],
        ids: Vec::new(),
        on: None,
    }));
    let e = r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(1.0),
    });
    (r.doc, e)
}

/// The planes' normals of the body `node` built, by face.
fn normals<T: geom_core::Decide + topo::AtRestPolicy>(
    ev: &Evaluation<T>,
    node: RecipeNodeId,
) -> BTreeMap<topo::FaceKey, Vec3<T>> {
    let Some(ValuePayload::Body(b)) = ev.value(node).map(|v| &v.payload) else {
        panic!("the extrude builds: {:?}", ev.node_error(node).map(|e| &e.kind));
    };
    let body = b.body();
    body.faces()
        .filter_map(|(f, _)| match body.face_surface(f) {
            Some(Surface::Plane { normal, .. }) => Some((f, *normal)),
            _ => None,
        })
        .collect()
}

fn opts(b: &ParamBox) -> EvalOptions {
    EvalOptions {
        param_box: Some(Arc::new(b.clone())),
        ..EvalOptions::default()
    }
}

/// The leaf's corners and middle, each a box of no width.
fn members(leaf: &ParamBox) -> Vec<ParamBox> {
    let axes: Vec<_> = leaf.varying().collect();
    let n = axes.len();
    let mut out = Vec::new();
    for corner in 0..=(1usize << n) {
        let at = axes
            .iter()
            .enumerate()
            .map(|(k, &(var, lo, hi))| {
                let x = if corner == 1 << n {
                    0.5 * (lo + hi)
                } else if corner >> k & 1 == 0 {
                    lo
                } else {
                    hi
                };
                (var, BoxAxis::Varying { lo: x, hi: x })
            })
            .collect();
        out.push(ParamBox::from_axes_in(at, leaf.order()));
    }
    out
}

fn holds_every_member(two: bool) {
    let tol = Tol::witness();
    let (doc, extrude) = slab(two);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(
        &doc,
        &analyzed,
        &DriveConfig {
            max_leaves: 16,
            ..DriveConfig::default()
        },
        &editor_core::Stop::default(),
        tol,
    )
    .expect("the nominal builds");
    let certified = verdict.certified();
    assert!(
        !certified.is_empty(),
        "two={two}: the drive certifies a leaf: {:?}",
        verdict.receipt()
    );
    let mut read = 0;
    let mut check = |session: &str, held: &BTreeMap<_, Vec3<Sym<Interval>>>, m: &ParamBox| {
        let ev: Evaluation<f64> = evaluate(&doc, None, &CancelToken::new(), &opts(m), tol);
        let member = normals(&ev, extrude);
        assert_eq!(
            held.keys().collect::<Vec<_>>(),
            member.keys().collect::<Vec<_>>(),
            "two={two} {session}: the replay and the member store the same planes"
        );
        for (face, n) in &member {
            let h = held[face];
            for (axis, c, x) in [("x", h.x, n.x), ("y", h.y, n.y), ("z", h.z, n.z)] {
                let slack = 4.0 * f64::EPSILON;
                assert!(
                    c.lo() - slack <= x && x <= c.hi() + slack,
                    "two={two} {session}: face {face:?}'s normal.{axis} {x} of member {m:?} \
                     lies in the stored [{}, {}]",
                    c.lo(),
                    c.hi()
                );
            }
            read += 1;
        }
    };
    for leaf in certified {
        let leaf_box = &leaf.box_;
        let eager = with_session_rules(BUDGET, SymRules::shipped(), || {
            let ev: Evaluation<Sym<Interval>> =
                evaluate(&doc, None, &CancelToken::new(), &opts(leaf_box), tol);
            normals(&ev, extrude)
        })
        .0;
        for m in members(leaf_box) {
            check("eager, over the leaf", &eager, &m);
            let deferring =
                with_deferred_session(BUDGET, SymRules::shipped(), SymRetry::none(), || {
                    let ev: Evaluation<Sym<Interval>> =
                        evaluate(&doc, None, &CancelToken::new(), &opts(&m), tol);
                    normals(&ev, extrude)
                })
                .0;
            check("deferring, at the member", &deferring, &m);
        }
    }
    assert!(read > 0, "two={two}: a normal was read");
}

#[test]
fn a_certified_leaf_holds_every_members_wall_normals_where_the_hop_is_an_identity() {
    holds_every_member(false);
}

#[test]
fn a_certified_leaf_holds_every_members_wall_normals_where_the_hop_is_a_quotient() {
    holds_every_member(true);
}
