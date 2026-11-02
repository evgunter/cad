//! **The box-scalar switch keeps the drive's two lanes logging alike**
//! (`topo::AtRestPolicy::leaf`, `docs/predicate-dimension-audit.md`
//! F31; PR 6371's first review, M1 and M2).
//!
//! At the box scalars a sweep door's leaf is tier 2 alone, and the
//! consumer that takes a finished body runs tier 3 there, logged. At
//! `f64` the leaf is finished where it is built, its finish's decisions
//! held off that node's log and relogged by the taker. So both lanes
//! log tier 3 at the taker, once, and a box leaf's verdict vector is the
//! witness's on every document where main's were: these rows certify in
//! one leaf where main did.
//!
//! The connectedness check takes each placement's body the same way,
//! and a body that does not finish at a box scalar is that output's
//! finding (`CheckEvidence::NotAtRest`); the walk goes on.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::Recorder;
use crate::m10_3_driver_interval::{param, uniform, unit_square};

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DEFAULT_MAX_DEPTH, DriveConfig, drive};
use editor_core::{
    Bodies, CancelToken, CheckEvidence, ChecksConfig, Dimension, DocEdit, EvalOptions, Evaluation,
    FreeVar, LoopProgram, Node, ProfileDoc, ProfileProgram, RecipeNodeId, UnitSym, VarName,
    evaluate, run_checks,
};
use geom_core::{Interval, Tol};

/// The slab's depth parameter, uniform over `1 ± half`.
fn depth(r: &mut Recorder, half: f64) {
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static("depth"),
        def: editor_core::VarDecl::Free(FreeVar::Continuous {
            dim: Dimension::Length,
            value: 1.0,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(uniform(half)),
        }),
    });
}

/// A prism over `lp`, extruded `depth`.
fn prism(r: &mut Recorder, lp: LoopProgram<editor_core::Formula>) -> RecipeNodeId {
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![lp],
        ids: Vec::new(),
        on: None,
    }));
    r.insert(Node::Extrude {
        profile: p.into(),
        distance: param("depth"),
    })
}

/// The leaf the drive certifies, at the shipped depth and 32 leaves:
/// the receipt's certified count.
fn certified(doc: &ProfileDoc) -> usize {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    let v = drive(
        doc,
        &analyzed,
        &DriveConfig {
            max_depth: DEFAULT_MAX_DEPTH,
            max_leaves: 32,
            ..DriveConfig::default()
        },
        Tol::witness(),
    )
    .expect("the drive runs");
    assert!(v.receipt().holds(), "{:?}", v.receipt());
    assert!(
        v.refused().is_empty(),
        "nothing refuses: {:?}",
        v.refused().iter().map(|l| &l.reason).collect::<Vec<_>>()
    );
    v.receipt().certified
}

/// **The reviewer's probe**: a slab of depth `1 ± half` under a
/// one-member union, so its leaf feeds a taker. Main certifies it in one
/// leaf at both half-widths; a lane that logged tier 3 at the taker on
/// one side only would refuse `FlipCrossing` (`bool_ring_run_winding`
/// and the battery's rows diverging).
#[test]
fn a_slab_under_a_union_certifies_in_one_leaf() {
    for half in [1e-6, 1e-9] {
        let mut r = Recorder::new();
        depth(&mut r, half);
        let slab = prism(&mut r, unit_square());
        r.insert(Node::Union {
            members: Bodies::Spelled(vec![slab.into()]),
        });
        assert_eq!(certified(&r.doc), 1, "half {half}");
    }
}

/// **A sweep read through a placement, then taken**: the slab, a copy
/// of it slid clear along x, and the union of the two. The union reads
/// the copy through the placement's map (`Framed::read_in`, ending in
/// the scalar's leaf) and takes both; the two lanes log the take alike.
#[test]
fn a_slab_placed_then_unioned_certifies_in_one_leaf() {
    for half in [1e-6, 1e-9] {
        let mut r = Recorder::new();
        depth(&mut r, half);
        let slab = prism(&mut r, unit_square());
        let copy = crate::m10_5_clearance_interval::translated(
            &mut r,
            slab,
            editor_core::Formula::literal(3.0, Dimension::Length).expect("a length"),
        );
        r.insert(Node::Union {
            members: Bodies::Spelled(vec![slab.into(), copy.into()]),
        });
        assert_eq!(certified(&r.doc), 1, "half {half}");
    }
}

/// **A placement that does not finish is its own output's finding, and
/// the walk goes on** (M2). At `Interval` a loft over NURBS walls is a
/// tier-2 leaf, and the shell door's take runs tier 3 on it, where check
/// 7's quadrature refuses a wall whose pcurve ends are not exact
/// (`work/onehome/check-7s-quadrature-refuses-interval-nurbs-walls-with-inexact-pcurve-ends.md`):
/// `NotAtRest`, at each of the two placements of it, so the walk did not
/// stop at the first. (The slab's orientation leaf cannot carry this
/// row: plain `Interval` does not build the slab over a box at all, and
/// the checks have no `Sym` lane.)
#[test]
fn a_placement_that_does_not_finish_is_its_outputs_finding() {
    use editor_core::analysis::BoxAxis;
    let (doc, loft) = crate::eval10_section_reads_the_nominal::square_loft_doc(0.5);
    let (doc, first) = crate::fixture::place(doc, loft);
    let (doc, second) = crate::fixture::place(doc, loft);
    let w = Tol::witness().get().eps / 256.0;
    let mut axes = std::collections::BTreeMap::new();
    axes.insert(
        crate::eval10_section_reads_the_nominal::var(&doc),
        BoxAxis::Varying { lo: -w, hi: w },
    );
    let opts = EvalOptions {
        param_box: Some(std::sync::Arc::new(ParamBox::from_axes(axes))),
        ..EvalOptions::default()
    };
    let ev: Evaluation<Interval> = evaluate(&doc, None, &CancelToken::new(), &opts, Tol::witness());
    assert!(
        ev.node_error(loft).is_none(),
        "the loft's leaf builds: {:?}",
        ev.node_error(loft)
    );
    // The residents that read the gathered product refuse the run as a
    // whole when its aggregate does not finish (`ProductInvalid`), as on
    // main: off here, so the row reads the connectedness walk alone.
    let cfg = ChecksConfig {
        at_rest: editor_core::Severity::Off,
        chart_coherence: editor_core::Advisory::Off,
        unproven_coincidence: editor_core::Advisory::Off,
        unused: editor_core::Advisory::Off,
        ..ChecksConfig::default()
    };
    let report = run_checks(&doc, &ev, &cfg, Tol::witness())
        .expect("the walk does not abort on a placement that does not finish");
    for placement in [first, second] {
        let findings: Vec<_> = report
            .findings
            .iter()
            .filter(|f| {
                matches!(f.subject, editor_core::FindingSubject::Output { placement: p, .. } if p == placement)
            })
            .map(|f| &f.evidence)
            .collect();
        assert!(
            findings
                .iter()
                .any(|e| matches!(e, CheckEvidence::NotAtRest { errors } if !errors.is_empty())),
            "each placement of the loft is its own NotAtRest finding: {findings:?}"
        );
    }
}

/// **The reviewer's probe, asserting** (PR 6371 review r1, M1): a slab of
/// depth `1 ± half` alone, under a one-member union, and under a union
/// with a fixed cube. Main (merge base `8ae2b9bf`) certifies every case
/// in one leaf; `6b946403` refused the union cases `FlipCrossing`.
#[test]
fn review_probe_6371_union_after_a_box_extrude() {
    use editor_core::Operand;
    let square = |x0: f64, s: f64| {
        LoopProgram::polygon([(x0, 0.0), (x0 + s, 0.0), (x0 + s, s), (x0, s)]).expect("a square")
    };
    for (members, half) in [(0usize, 1e-6), (1, 1e-6), (0, 1e-9), (1, 1e-9)] {
        let mut r = Recorder::new();
        depth(&mut r, half);
        let slab = prism(&mut r, square(0.0, 1.0));
        if members > 0 {
            let mut ms = vec![Operand::output(slab, 0).into()];
            if members == 2 {
                let q = r.insert(Node::Profile(ProfileProgram {
                    loops: vec![square(5.0, 1.0)],
                    ids: Vec::new(),
                    on: None,
                }));
                let cube = r.insert(Node::Extrude {
                    profile: q.into(),
                    distance: crate::fixture::len(1.0),
                });
                // The cube joins the slab's space where it is drawn, so
                // the union reads two members of one space.
                let cube = r.against(slab, cube, &editor_core::Frame::IDENTITY);
                ms.push(Operand::output(cube, 0).into());
            }
            r.insert(Node::Union {
                members: Bodies::Spelled(ms),
            });
        }
        assert_eq!(certified(&r.doc), 1, "members {members}, half {half}");
    }
}
