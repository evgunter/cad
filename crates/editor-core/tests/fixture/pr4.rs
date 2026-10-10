//! The M4 PR 4 diagnosis corpus (spec D7): a small set of
//! deliberately-broken documents — edits that vanish names — whose
//! typed `ResolveError`/`Diagnosis` outputs join the golden-digest
//! family. Scalar-generic: the corpus runs at f64 AND Interval, and
//! same verdicts ⇒ byte-identical resolution output (the tables agree
//! per PR 3's invariant, the verdict logs are scalar-free, and the
//! diagnosis is a function of both).
#![allow(dead_code)] // shared across test binaries

use editor_core::ExtrudeSide;
use editor_core::{
    BooleanOp, CancelToken, CapEnd, DocEdit, EntityKind, Entry, EvalOptions, Evaluation, Node,
    ProfileDoc, Qualifier, RecipeNodeId, Resolution, RoleSeg, RunCtx, SlotId, StableName, evaluate,
    resolve, resolve_with_prior,
};

use super::{ang, insert, len, minted, on_frame, scl, step};
use geom_core::Tol;

/// The corpus's evaluator — the PRODUCTION path (realized BVH sweep),
/// per Ev's 2026-07-29 ruling on the M5 PR 8 diagnosis question:
/// the diagnosis ACCEPTANCE artifacts (this corpus + the golden
/// digest in `m4_pr4_ci`) pin what production users actually get.
/// Scenario A's flip-vanish row: the vanished name is a rim-edge piece
/// named by its ends, whose group went from two to one when the slab
/// that cut it slid clear. The disjoint run's pair space is pruned
/// (the AMENDED N5 semantics), but the slab's wall, which the piece's
/// end vertex cites, is still classified against `a` and flips from
/// inside to outside, so the row diagnoses to that `PredicateFlip`,
/// which outranks the group-size rung. Engine-behavior tests
/// that are genuinely about behavior-GIVEN-verdicts stay under the
/// idealized sweep (`m4_pr4_diff`, `m4_pr4_resolve` — see their
/// headers); `m4_pr4_banked` pins both strategies side by side.
fn run<T>(doc: &editor_core::ProfileDoc, prior: Option<&Evaluation<T>>) -> Evaluation<T>
where
    T: editor_core::EvalScalar,
{
    evaluate::<T>(
        doc,
        prior,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn block(
    doc: ProfileDoc,
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(dz),
            side: ExtrudeSide::Along,
        },
    )
}

/// Runs the whole diagnosis corpus at scalar `T`, producing labeled
/// `Resolution` outputs in a fixed order. Every scenario is
/// margin-fat at all CI ε rows (1e-6 … 1e-12): the verdicts — and
/// therefore the outputs, arena keys included — are identical across
/// rows and scalars.
pub fn diagnosis_corpus<T>() -> Vec<(&'static str, Resolution)>
where
    T: editor_core::EvalScalar,
{
    let mut out = Vec::new();

    // ---- Scenario A: a sliding slab (flip-vanish + cascade). ----
    // The slab crosses both of `a`'s top rims along x, whose genuine
    // valence-3 cuts at x = 0.45 and 0.55 hold each in two pieces named
    // by their ends; slid clear of `a`, it leaves the rims whole. No
    // vertex of the slab lies inside `a` in either run.
    let doc = ProfileDoc::empty_derived("pr4", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b0) = block(doc, (-0.05, 0.05), (-1.0, 2.0), 0.5, 2.5);
    let (doc, tr) = insert(
        doc,
        Node::transform(
            b0,
            editor_core::Step::Rigid {
                translation: [len(0.5), len(0.0), len(0.0)],
                axis: [scl(0.0), scl(0.0), scl(1.0)],
                angle: ang(0.0),
            },
        ),
    );
    let (doc, u) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: a.into(),
            b: tr.into(),
            declare: Vec::new(),
        },
    );
    let (doc, pat) = insert(
        doc,
        Node::Pattern {
            input: u.into(),
            count: editor_core::Formula::count(2),
            kind: editor_core::PatternKind::Linear {
                direction: [scl(0.0), scl(1.0), scl(0.0)],
                spacing: len(5.0),
            },
        },
    );
    let ev1 = run::<T>(&doc, None);
    // The FIRST FromA rim-edge piece in table order — a
    // deterministic probe.
    let piece: StableName = ev1
        .value(u)
        .expect("union evaluates")
        .name_table
        .iter()
        .find_map(|(n, e)| {
            let hit = n.kind == EntityKind::Edge
                && matches!(n.path.first(), Some(RoleSeg::FromA(_)))
                && matches!(n.path.last(), Some(RoleSeg::Fragment(Qualifier::Ends(_))));
            (hit && matches!(e, Entry::Unique(_))).then(|| n.clone())
        })
        .expect("a rim piece named by its ends exists");
    let inst = minted(
        EntityKind::Edge,
        pat,
        RoleSeg::Instance {
            i: 1,
            of: piece.clone().into(),
        },
    );
    let (doc2, _) = step(
        doc.clone(),
        DocEdit::SetParam {
            node: tr,
            slot: SlotId::Translation(editor_core::Axis3::X),
            value: len(2.5).into(),
            fresh: Vec::new(),
        },
    );
    let ev2 = run::<T>(&doc2, Some(&ev1));
    let new = RunCtx {
        doc: &doc2,
        eval: &ev2,
    };
    let prior = RunCtx {
        doc: &doc,
        eval: &ev1,
    };
    out.push(("flip-vanish", resolve_with_prior(new, prior, &piece)));
    out.push(("cascade", resolve_with_prior(new, prior, &inst)));

    // ---- Scenario B: pattern count shrink (StructuralParam). ----
    let (doc3, _) = step(
        doc.clone(),
        DocEdit::SetStructuralParam {
            node: pat,
            slot: SlotId::Count,
            expr: editor_core::Formula::count(1),
            fresh: Vec::new(),
        },
    );
    let ev3 = run::<T>(&doc3, Some(&ev1));
    out.push((
        "structural-param",
        resolve_with_prior(
            RunCtx {
                doc: &doc3,
                eval: &ev3,
            },
            prior,
            &inst,
        ),
    ));

    // ---- Scenario C: a name minted by a deleted node (NodeGone). ----
    let docd = ProfileDoc::empty_derived("pr4", Tol::witness());
    let (docd, _) = block(docd, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (docd, db) = block(docd, (2.0, 3.0), (0.0, 1.0), 0.0, 1.0);
    let cap_b = minted(EntityKind::Face, db, RoleSeg::Cap(CapEnd::End));
    let (docd, _) = step(docd, DocEdit::DeleteNode { id: db });
    let evd = run::<T>(&docd, None);
    out.push((
        "node-gone",
        resolve(
            RunCtx {
                doc: &docd,
                eval: &evd,
            },
            &cap_b,
        ),
    ));

    // ---- Scenario D: the symmetric U tie (Ambiguous). ----
    let docu = ProfileDoc::empty_derived("pr4", Tol::witness());
    let (docu, ua) = block(docu, (0.0, 4.0), (0.0, 4.0), 0.0, 4.0);
    let (docu, up) = on_frame(
        docu,
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (2.0, 1.0),
            (6.0, 1.0),
            (6.0, 3.0),
            (2.0, 3.0),
            (2.0, 2.5),
            (5.0, 2.5),
            (5.0, 1.5),
            (2.0, 1.5),
        ]],
    );
    let (docu, ub) = insert(
        docu,
        Node::Extrude {
            profile: up.into(),
            distance: len(2.0),
            side: ExtrudeSide::Along,
        },
    );
    let (docu, us) = insert(
        docu,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: ua.into(),
            b: ub.into(),
            declare: Vec::new(),
        },
    );
    let evu = run::<T>(&docu, None);
    let tied: StableName = evu
        .value(us)
        .expect("U subtract evaluates")
        .name_table
        .iter()
        .find_map(|(n, e)| matches!(e, Entry::Tied(_)).then(|| n.clone()))
        .expect("the U fixture ties");
    out.push((
        "ambiguous",
        resolve(
            RunCtx {
                doc: &docu,
                eval: &evu,
            },
            &tied,
        ),
    ));

    out
}
