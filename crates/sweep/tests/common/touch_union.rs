//! **Two operands that touch only along a line, at every op**: each
//! difference returns its operand, the intersection is empty, and the
//! union is the two solids, checked for 3′ on its own records, one of
//! which is the edge–edge record of the two edges on the line
//! (`crates/topo/README.md` C7).
//!
//! 3′ on a union of two curved solids within reach of each other ends
//! in the census's cross-solid backstop, which cannot yet decide a
//! curved pair (`work/restread/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`):
//! the check admits that refusal and no other finding.

use geom_core::{Point3, Tol};
use topo::{AtRestBody, BooleanBody, BooleanResult, BooleanResultKind, ContactKind, EdgeKey};

fn tol() -> Tol {
    Tol::witness()
}

fn volume(body: &AtRestBody<f64>) -> f64 {
    topo::mass_properties(body, tol()).unwrap().volume
}

fn body(what: &str, got: Result<BooleanResult<f64>, topo::BooleanError>) -> BooleanBody<f64> {
    match got {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("{what}: expected a body, got {other:?}"),
    }
}

/// Whether `p` lies on the line through `line` (within `1e-9` m).
fn on_line(p: Point3<f64>, [a, b]: [Point3<f64>; 2]) -> bool {
    let d = (b - a).normalize();
    let w = p - a;
    (w - d * w.dot(d)).norm() < 1e-9
}

/// Asserts every op on `a`, `b` answers as a touch along `line`.
pub fn assert_touch_at_every_op(
    what: &str,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    line: [Point3<f64>; 2],
) {
    let (va, vb) = (volume(a), volume(b));
    for (op, got, want) in [
        ("a ∖ b", topo::subtract(a, b, tol()), va),
        ("b ∖ a", topo::subtract(b, a, tol()), vb),
    ] {
        let r = body(&format!("{what}, {op}"), got);
        assert_eq!(r.kind, BooleanResultKind::OperandA, "{what}, {op}");
        let v = volume(&r.body);
        assert!(
            (v - want).abs() < 1e-9 * want,
            "{what}, {op}: volume {v}, want {want}"
        );
    }
    assert!(
        matches!(topo::intersect(a, b, tol()), Ok(BooleanResult::Empty)),
        "{what}, a ∩ b: empty"
    );

    let r = body(&format!("{what}, a ∪ b"), topo::union(a, b, tol()));
    assert_eq!(r.kind, BooleanResultKind::Assembly, "{what}, a ∪ b");
    assert_eq!(r.body.solids().count(), 2, "{what}, a ∪ b: two solids");
    let v = volume(&r.body);
    assert!(
        (v - va - vb).abs() < 1e-9 * (va + vb),
        "{what}, a ∪ b: volume {v}, want {}",
        va + vb
    );
    let undecidable =
        |e: &topo::ValidationError| matches!(e, topo::ValidationError::CensusUndecidable { .. });
    let verdict = topo::validate_pseudomanifold(&r.body, &r.contacts, tol());
    assert!(
        verdict
            .as_ref()
            .err()
            .is_none_or(|errs| errs.iter().all(undecidable)),
        "{what}, a ∪ b: 3′ on its records: {verdict:?}\n{:?}",
        r.contacts
    );
    let ee: Vec<(EdgeKey, EdgeKey)> = r.contacts.edge_edge().collect();
    let [pair] = ee[..] else {
        panic!("{what}, a ∪ b: one edge–edge record, got {ee:?}");
    };
    for e in [pair.0, pair.1] {
        let edge = r.body.get_edge(e).expect("a recorded edge is live");
        for he in [edge.he_plus, edge.he_minus] {
            let p = r.body.half_edge_start_point(he).expect("a live edge's end");
            assert!(
                on_line(p, line),
                "{what}, a ∪ b: {e:?} ends at {p:?}, off the line"
            );
        }
    }
    let stripped: topo::ContactRecords = r
        .contacts
        .rows()
        .iter()
        .filter(|c| !matches!(c.record.kind(), ContactKind::EdgeEdge(..)))
        .cloned()
        .collect();
    let errs = topo::validate_pseudomanifold(&r.body, &stripped, tol())
        .err()
        .unwrap_or_default();
    let overlap = |e: &topo::ValidationError| {
        matches!(
            e,
            topo::ValidationError::UnrecordedContact {
                contact: topo::CensusContact::EdgeEdgeOverlap { .. },
                ..
            }
        )
    };
    assert!(
        errs.iter().any(overlap) && errs.iter().all(|e| overlap(e) || undecidable(e)),
        "{what}, a ∪ b: without its edge–edge record the census names the overlap: {errs:?}"
    );
}
