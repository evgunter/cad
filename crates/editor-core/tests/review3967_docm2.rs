//! Review probe for PR 3967: the `docm2_part_interval` ε/16 rung's
//! union, certified at `Interval` on the head, checked against the
//! same document evaluated in f64: the same counts, every f64 vertex
//! inside an Interval vertex's enclosure, and the f64 volume inside
//! the Interval volume's enclosure.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::corpus;

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::{
    CancelToken, Dimension, Distribution, DocEdit, DocParam, EvalOptions, Node, ParamName,
    ProfileDoc, UnitSym, ValuePayload, apply, evaluate,
};
use geom_core::{Bounds, Interval, Tol};
use topo::Body;

fn widened_document(width: f64) -> ProfileDoc {
    let cd = corpus::part_select::document();
    apply(
        &cd.doc,
        &DocEdit::SetDocParam {
            name: ParamName::from_static(corpus::part_select::H),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 1.0,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: Some(Distribution::Uniform {
                    lo: -width,
                    hi: width,
                }),
            },
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .unwrap()
    .doc
}

#[test]
#[ignore = "review probe"]
fn review3967_docm2_eps16_union_against_f64() {
    let tol = Tol::witness();
    for div in [16.0, 64.0] {
        let doc = widened_document(tol.eps() / div);
        let union = *doc
            .order()
            .iter()
            .find(|id| matches!(doc.node(**id), Some(Node::Boolean { .. })))
            .unwrap();
        let f = evaluate::<f64>(
            &doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            tol,
        );
        let nominal = evaluate::<Interval>(
            &doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            tol,
        );
        let opts = EvalOptions {
            param_box: Some(Arc::new(ParamBox::of(&analyzed_box(
                &doc,
                &AnalysisPolicy::default(),
            )))),
            ..EvalOptions::default()
        };
        let w = evaluate::<Interval>(&doc, Some(&nominal), &CancelToken::new(), &opts, tol);
        assert!(
            corpus::failures(&w).is_empty(),
            "eps/{div}: {:?}",
            corpus::failures(&w)
        );
        let ValuePayload::Boolean(editor_core::BooleanValue::Body { body: bf, .. }) =
            &f.value(union).unwrap().payload
        else {
            panic!("f64")
        };
        let ValuePayload::Boolean(editor_core::BooleanValue::Body { body: bi, .. }) =
            &w.value(union).unwrap().payload
        else {
            panic!("interval")
        };
        let (bf, bi): (&Body<f64>, &Body<Interval>) = (&**bf, &**bi);
        let counts = |f: usize, e: usize, v: usize| format!("f{f} e{e} v{v}");
        let cf = counts(
            bf.faces().count(),
            bf.edges().count(),
            bf.vertices().count(),
        );
        let ci = counts(
            bi.faces().count(),
            bi.edges().count(),
            bi.vertices().count(),
        );
        println!("eps/{div}: f64 {cf}  interval {ci}");
        assert_eq!(cf, ci);
        let ip: Vec<_> = bi.points().map(|(_, p)| *p).collect();
        let mut worst: f64 = 0.0;
        for (_, p) in bf.points() {
            let inside = ip.iter().any(|q| {
                [(p.x, q.x), (p.y, q.y), (p.z, q.z)]
                    .iter()
                    .all(|(a, b)| b.lo() - 1e-15 <= *a && *a <= b.hi() + 1e-15)
            });
            assert!(inside, "eps/{div}: f64 point {p:?} in no interval point");
        }
        for q in &ip {
            worst = worst.max((q.z.hi() - q.z.lo()).max(q.x.hi() - q.x.lo()));
        }
        let vf = topo::mass_properties(bf, tol).unwrap().volume;
        let vi = topo::mass_properties(bi, tol).map(|m| m.volume);
        println!("eps/{div}: widest vertex enclosure {worst:e}; volume f64 {vf} interval {vi:?}");
        if let Ok(vi) = vi {
            assert!(vi.lo() <= vf && vf <= vi.hi(), "eps/{div}: volume");
        }
    }
}
