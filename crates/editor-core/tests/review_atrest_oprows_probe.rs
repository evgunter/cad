//! Review probes for PR #5882 (not for merge).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, run};
use crate::fixture::{against, axes_at, insert, len, union_over};
use editor_core::{
    AssertionRelation, AssertionVerdict, CheckEvidence, CheckId, ChecksConfig, LoopProgram,
    MeasurePrimitive, NamedCell, Node, ProfileDoc, ProfileProgram, RecipeNodeId, SitedRef,
    ValuePayload,
};
use geom_core::Tol;
use topo::DecisionSite;

fn turned() -> editor_core::Frame {
    turned_at(0.0)
}

fn turned_at(t: f64) -> editor_core::Frame {
    let k = {
        let n = 14f64.sqrt();
        [1.0 / n, 2.0 / n, 3.0 / n]
    };
    let (s, c) = 0.7f64.sin_cos();
    let rot = |e: [f64; 3]| {
        let dot = k[0] * e[0] + k[1] * e[1] + k[2] * e[2];
        let cross = [
            k[1] * e[2] - k[2] * e[1],
            k[2] * e[0] - k[0] * e[2],
            k[0] * e[1] - k[1] * e[0],
        ];
        [0, 1, 2].map(|i| e[i] * c + cross[i] * s + k[i] * dot * (1.0 - c))
    };
    let w = rot([0.0, 0.0, 1.0]);
    axes_at(
        [0.3 + t * w[0], -0.2 + t * w[1], 0.5 + t * w[2]],
        rot([1.0, 0.0, 0.0]),
        rot([0.0, 1.0, 0.0]),
    )
}

fn rod(doc: ProfileDoc, cx: f64, r: f64, h: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![
                LoopProgram::circle_split(cx, 0.0, r, 2, std::f64::consts::FRAC_PI_2)
                    .expect("a finite circle"),
            ],
            ids: Vec::new(),
        }),
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(h),
        },
    )
}

fn gap_zero(doc: &ProfileDoc, outer: &SitedRef, inner: &SitedRef) -> (ProfileDoc, RecipeNodeId) {
    let (doc, measure) = crate::fixture::measure_node(
        doc,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        vec![outer.clone(), inner.clone()],
    );
    let value = crate::fixture::value_of(&doc, measure);
    insert(
        doc,
        Node::Assertion {
            value,
            bound: len(0.0),
            relation: AssertionRelation::Equal,
        },
    )
}

fn site(doc: &ProfileDoc, cell: &NamedCell) -> SitedRef {
    let NamedCell::Entity { input, name } = cell else {
        panic!("an entity cell: {cell:?}")
    };
    SitedRef::new(doc.operation_of(*input).expect("a live read"), name.clone())
}

/// Claim 2 probe: two parallel rods, externally tangent, unioned on a
/// turned frame. Their `TangentWitness` row is a cylinder pair `Gap`
/// reads (parallel axes), so `contact_quietable` says true; but
/// `Gap = r_b - r_p - d` with `d = r1 + r2` is `-2r` in either order,
/// so no `Gap = 0` over the row's cells holds.
#[test]
fn external_cylinder_tangency_is_quietable_but_no_gap_zero_holds() {
    for (r1, r2) in [(0.5, 0.5), (0.5, 0.3)] {
        let doc = ProfileDoc::empty_derived("probe-rods", Tol::witness());
        let (doc, anchor) = block(doc, (10.0, 11.0), (10.0, 11.0), 1.0);
        let (doc, a) = rod(doc, 0.0, r1, 1.0);
        let (doc, b) = rod(doc, r1 + r2, r2, 0.5);
        let (doc, a) = against(doc, anchor, a, &turned());
        let (doc, b) = against(doc, anchor, b, &turned_at(0.25));
        let (doc, union) = union_over(doc, &[a, b]);
        let ev = run(&doc);
        // Today the boolean refuses this fixture (CurvedPierceUnsupported),
        // so the counterexample is latent: the probe records that and
        // goes red the day the union builds and the row reads
        // quietable with no holding Gap = 0.
        if let Some(e) = crate::blocks::failure(&ev, union) {
            eprintln!("({r1},{r2}) union refused: {e:?}");
            continue;
        }
        let rows = ev
            .value(union)
            .expect("union evaluated")
            .coincidences
            .to_vec();
        eprintln!("rows ({r1},{r2}): {rows:#?}");
        let tangent = rows
            .iter()
            .find(|r| r.site == DecisionSite::TangentWitness)
            .unwrap_or_else(|| panic!("a tangency row: {rows:?}"))
            .clone();
        let report =
            editor_core::run_checks(&doc, &ev, &ChecksConfig::default(), Tol::witness()).unwrap();
        let found = report
            .findings
            .iter()
            .find(|f| {
                f.check == CheckId::UnprovenCoincidence
                    && matches!(&f.evidence, CheckEvidence::UnprovenCoincidence { row, .. } if **row == tangent)
            })
            .unwrap_or_else(|| panic!("a finding over the tangency: {report}"));
        let CheckEvidence::UnprovenCoincidence { quietable, .. } = &found.evidence else {
            unreachable!()
        };
        eprintln!("({r1},{r2}) quietable = {quietable}");
        let [ca, cb] = tangent.cells.as_slice() else {
            panic!("two cells")
        };
        let (sa, sb) = (site(&doc, ca), site(&doc, cb));
        let mut held = Vec::new();
        for (o, i) in [(&sa, &sb), (&sb, &sa)] {
            let (doc2, assertion) = gap_zero(&doc, o, i);
            let ev2 = run(&doc2);
            let verdict = ev2.value(assertion).map(|v| v.payload.clone());
            eprintln!("({r1},{r2}) Gap=0 verdict: {verdict:?}");
            held.push(matches!(
                verdict,
                Some(ValuePayload::Assertion(AssertionVerdict::Holds { .. }))
            ));
        }
        assert!(
            !*quietable || held.iter().any(|h| *h),
            "({r1},{r2}): the row reads quietable, but no Gap = 0 over its cells holds in either order"
        );
    }
}

/// Claim 2 probe, at rest: two parallel rods placed side by side,
/// externally tangent, as two copies of the world product.
#[test]
fn at_rest_external_cylinder_tangency_probe() {
    for (r1, r2) in [(0.5, 0.5), (0.5, 0.3)] {
        let doc = ProfileDoc::empty_derived("probe-rods-rest", Tol::witness());
        let (doc, a) = rod(doc, 0.0, r1, 1.0);
        let (doc, b) = rod(doc, 0.0, r2, 1.0);
        let (doc, _p) = crate::fixture::place(doc, a);
        let (doc, _q) = crate::fixture::in_world(
            doc,
            b,
            &editor_core::Frame::translation([r1 + r2, 0.0, 0.0]),
        );
        let ev = run(&doc);
        let report =
            editor_core::run_checks(&doc, &ev, &ChecksConfig::default(), Tol::witness()).unwrap();
        eprintln!("({r1},{r2}) report: {report}");
        for f in &report.findings {
            if let CheckEvidence::UnprovenCoincidence { row, quietable, .. } = &f.evidence {
                eprintln!(
                    "({r1},{r2}) row {:?} {:?} quietable={quietable}",
                    row.relation, row.site
                );
                if !quietable || row.cells.len() != 2 {
                    continue;
                }
                let (sa, sb) = (site(&doc, &row.cells[0]), site(&doc, &row.cells[1]));
                let mut held = false;
                for (o, i) in [(&sa, &sb), (&sb, &sa)] {
                    let (doc2, assertion) = gap_zero(&doc, o, i);
                    let ev2 = run(&doc2);
                    let verdict = ev2.value(assertion).map(|v| v.payload.clone());
                    eprintln!("({r1},{r2}) Gap=0 verdict: {verdict:?}");
                    held |= matches!(
                        verdict,
                        Some(ValuePayload::Assertion(AssertionVerdict::Holds { .. }))
                    );
                }
                assert!(
                    held,
                    "({r1},{r2}) quietable row {row:?}: no Gap = 0 over its cells holds"
                );
            }
        }
    }
}
