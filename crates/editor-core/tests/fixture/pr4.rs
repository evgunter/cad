//! The M4 PR 4 diagnosis corpus (spec D7): a small set of
//! deliberately-broken documents — edits that vanish names — whose
//! typed `ResolveError`/`Diagnosis` outputs join the golden-digest
//! family. Scalar-generic: the corpus runs at f64 AND Interval, and
//! same verdicts ⇒ byte-identical resolution output (the tables agree
//! per PR 3's invariant, the verdict logs are scalar-free, and the
//! diagnosis is a function of both).
#![allow(dead_code)] // shared across test binaries

use editor_core::eval::ContentBits;
use editor_core::{
    BooleanOp, CancelToken, CapEnd, DocEdit, EntityKind, Entry, EvalOptions, Evaluation, Node,
    ProfileDoc, Qualifier, RecipeNodeId, Resolution, RoleSeg, RunCtx, SlotId, StableName, evaluate,
    resolve, resolve_with_prior,
};
use geom_core::Decide;

use super::{ang, desc, insert, len, scl, step};
use geom_core::Tol;

/// The corpus's evaluator — the PRODUCTION path (realized BVH sweep),
/// per Evan's 2026-07-29 ruling on the M5 PR 8 diagnosis question:
/// the diagnosis ACCEPTANCE artifacts (this corpus + the golden
/// digest in `m4_pr4_ci`) pin what production users actually get.
/// Scenario A's flip-vanish row therefore exercises the AMENDED N5
/// semantics: the disjoint run's pair space is pruned, the flip
/// evidence is never computed, and the row diagnoses to the
/// documented evidence-free minting-node fallback (NAMING-DESIGN N5
/// as amended; recovery rung banked as #134). Engine-behavior tests
/// that are genuinely about behavior-GIVEN-verdicts stay under the
/// idealized sweep (`m4_pr4_diff`, `m4_pr4_resolve` — see their
/// headers); `m4_pr4_banked` pins both strategies side by side.
fn run<T>(doc: &ProfileDoc, prior: Option<&Evaluation<T>>) -> Evaluation<T>
where
    T: Decide + ContentBits + geom_core::Bounds + Send + Sync + topo::AtRestPolicy,
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
    let (doc, p) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, z0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]],
        )),
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
        },
    )
}

fn name1(kind: EntityKind, node: RecipeNodeId, seg: RoleSeg) -> StableName {
    StableName {
        kind,
        node,
        path: vec![seg],
    }
}

/// Runs the whole diagnosis corpus at scalar `T`, producing labeled
/// `Resolution` outputs in a fixed order. Every scenario is
/// margin-fat at all CI ε rows (1e-6 … 1e-12): the verdicts — and
/// therefore the outputs, arena keys included — are identical across
/// rows and scalars.
pub fn diagnosis_corpus<T>() -> Vec<(&'static str, Resolution)>
where
    T: Decide + ContentBits + geom_core::Bounds + Send + Sync + topo::AtRestPolicy,
{
    let mut out = Vec::new();

    // ---- Scenario A: sliding union (flip-vanish + cascade). ----
    let doc = ProfileDoc::empty_derived("pr4", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b0) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, tr) = insert(
        doc,
        Node::Transform {
            input: b0,
            translation: [len(0.5), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    );
    // M4 PR 5: the sliding overlap's flush planes are DECLARED (the
    // recipe intent; the retired bit rung no longer infers them).
    let (doc, decl) = super::declare_x_offset_flush(doc, a, b0);
    let (doc, u) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b: tr,
            declare: Some(decl),
        },
    );
    let (doc, pat) = insert(
        doc,
        Node::Pattern {
            input: u,
            count: editor_core::Expr::count(2),
            kind: editor_core::PatternKind::Linear {
                direction: [scl(0.0), scl(1.0), scl(0.0)],
                spacing: len(5.0),
            },
        },
    );
    let ev1 = run::<T>(&doc, None);
    // The FIRST ranked FromA rim-edge fragment in table order — a
    // deterministic probe.
    let ranked: StableName = ev1
        .value(u)
        .expect("union evaluates")
        .name_table
        .iter()
        .find_map(|(n, e)| {
            let hit = n.kind == EntityKind::Edge
                && matches!(n.path.first(), Some(RoleSeg::FromA(_)))
                && matches!(
                    n.path.last(),
                    Some(RoleSeg::Fragment(Qualifier::OrderAlong { .. }))
                );
            (hit && matches!(e, Entry::Unique(_))).then(|| n.clone())
        })
        .expect("ranked rim fragment exists");
    let inst = name1(
        EntityKind::Edge,
        pat,
        RoleSeg::Instance {
            i: 1,
            of: Box::new(ranked.clone()),
        },
    );
    let (doc2, _) = step(
        doc.clone(),
        DocEdit::SetParam {
            node: tr,
            slot: SlotId::Translation(editor_core::Axis3::X),
            expr: len(2.5),
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
    out.push(("flip-vanish", resolve_with_prior(new, prior, &ranked)));
    out.push(("cascade", resolve_with_prior(new, prior, &inst)));

    // ---- Scenario B: pattern count shrink (StructuralParam). ----
    let (doc3, _) = step(
        doc.clone(),
        DocEdit::SetStructuralParam {
            node: pat,
            slot: SlotId::Count,
            expr: editor_core::Expr::count(1),
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

    // ---- Scenario C: Declare stranded by DeleteNode (NodeGone). ----
    let docd = ProfileDoc::empty_derived("pr4", Tol::witness());
    let (docd, da) = block(docd, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (docd, db) = block(docd, (2.0, 3.0), (0.0, 1.0), 0.0, 1.0);
    let cap_b = name1(EntityKind::Face, db, RoleSeg::Cap(CapEnd::Top));
    let (docd, _) = insert(
        docd,
        Node::declare_rest(vec![(
            name1(EntityKind::Face, da, RoleSeg::Cap(CapEnd::Top)),
            cap_b.clone(),
        )]),
    );
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
    let (docu, up) = insert(
        docu,
        Node::Profile(desc(
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
        )),
    );
    let (docu, ub) = insert(
        docu,
        Node::Extrude {
            profile: up,
            distance: len(2.0),
        },
    );
    let (docu, us) = insert(
        docu,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: ua,
            b: ub,
            declare: None,
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
