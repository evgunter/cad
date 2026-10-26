//! **Two operands that touch only along a line, at every op**: each
//! difference returns its operand, the intersection is empty, and the
//! union is the two solids, checked for 3′ on its own records, one of
//! which is the edge–edge record of the two edges on the line
//! (`crates/topo/README.md` C7).
//!
//! 3′ on a union of two curved solids within reach of each other ends
//! in the census's cross-solid backstop, which cannot yet decide a
//! curved pair (`work/restread/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`):
//! the check admits that refusal on face pairs across the two solids,
//! with the solid pair it leaves unexamined, and no other finding.

use geom_core::{Point3, Tol};
use topo::{
    AtRestBody, BooleanBody, BooleanResult, BooleanResultKind, ContactKind, EdgeKey, EntityId,
    SolidContainment, ValidationError,
};

use super::differential::every_op_both_orders;

fn tol() -> Tol {
    Tol::witness()
}

fn volume(body: &AtRestBody<f64>) -> f64 {
    topo::test_support::pinned_volume("the measured body", body, tol()).volume
}

fn body<'r>(
    what: &str,
    got: &'r Result<BooleanResult<f64>, topo::BooleanError>,
) -> &'r BooleanBody<f64> {
    match got {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("{what}: expected a body, got {other:?}"),
    }
}

/// The census backstop's reason for a curved face of one solid within
/// reach of another's (`census::Undecided::CurvedWithinReach`); a plane
/// face bounded by an arc counts as curved there.
const CURVED_WITHIN_REACH: &str = "a curved face of one is within reach of the other";

/// The instance arm's reason for a pair whose boundaries another
/// finding left unchecked (`census::Undecided::Unexamined`).
const UNEXAMINED: &str = "another finding left their boundaries unchecked";

/// Whether every finding in `errs` is the frontier the RESTREAD row
/// holds open, and nothing else: the backstop's `CurvedWithinReach` on
/// two faces of `body`'s two different solids, and the instance arm's
/// `Unexamined` on those two solids, which only follows the first.
pub fn only_curved_within_reach(body: &AtRestBody<f64>, errs: &[ValidationError]) -> bool {
    let solid = |f| body.solid_of_face(f);
    let reach = |e: &ValidationError| {
        matches!(
            e,
            ValidationError::CensusUndecidable {
                a: EntityId::Face(a),
                b: EntityId::Face(b),
                what,
                ..
            } if what.starts_with(CURVED_WITHIN_REACH)
                && solid(*a).is_some()
                && solid(*a) != solid(*b)
        )
    };
    let unexamined = |e: &ValidationError| {
        matches!(
            e,
            ValidationError::CensusUndecidable {
                a: EntityId::Solid(a),
                b: EntityId::Solid(b),
                what,
                ..
            } if what.starts_with(UNEXAMINED) && a != b
        )
    };
    errs.iter().all(|e| reach(e) || unexamined(e)) && (errs.is_empty() || errs.iter().any(reach))
}

/// Asserts every op on `a`, `b`, in both operand orders, answers as a
/// touch along the segment `line`; `inside[0]` lies strictly inside `a`
/// and `inside[1]` strictly inside `b`.
pub fn assert_touch_at_every_op(
    what: &str,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    line: [Point3<f64>; 2],
    inside: [Point3<f64>; 2],
) {
    let (va, vb) = (volume(a), volume(b));
    let band = super::approx::band();
    let holds = |r: &AtRestBody<f64>, p: Point3<f64>| match topo::point_in_solid(r, p, band, tol())
    {
        Ok(SolidContainment::In) => true,
        Ok(SolidContainment::Out) => false,
        other => panic!("{what}: point membership at {p:?}: {other:?}"),
    };
    for (op, got, want) in every_op_both_orders(a, b, (va, vb, 0.0), tol()) {
        let label = format!("{what}, {op}");
        if op.contains('∩') {
            assert!(
                matches!(got, Ok(BooleanResult::Empty)),
                "{label}: empty, got {got:?}"
            );
            continue;
        }
        let r = body(&label, &got);
        let v = volume(&r.body);
        assert!(
            (v - want).abs() < 1e-9 * want,
            "{label}: volume {v}, want {want}"
        );
        if op.contains('∖') {
            // Which operand came back: the first, by membership.
            let kept = usize::from(op.starts_with('B'));
            assert_eq!(r.kind, BooleanResultKind::OperandA, "{label}");
            assert!(
                holds(&r.body, inside[kept]) && !holds(&r.body, inside[1 - kept]),
                "{label}: the first operand's material, and not the second's"
            );
            continue;
        }
        assert_union(&label, r, line, inside, holds);
    }
}

/// The union's half of [`assert_touch_at_every_op`].
fn assert_union(
    what: &str,
    r: &BooleanBody<f64>,
    line: [Point3<f64>; 2],
    inside: [Point3<f64>; 2],
    holds: impl Fn(&AtRestBody<f64>, Point3<f64>) -> bool,
) {
    assert_eq!(r.kind, BooleanResultKind::Assembly, "{what}");
    // `Assembly` says only "more than one solid"; a union that split an
    // operand in two along a touch would keep the volume, the membership
    // and the record.
    assert_eq!(r.body.solids().count(), 2, "{what}: two solids");
    assert!(
        inside.iter().all(|&p| holds(&r.body, p)),
        "{what}: both operands' material"
    );
    let verdict = topo::validate_pseudomanifold(&r.body, &r.contacts, tol());
    assert!(
        verdict
            .as_ref()
            .err()
            .is_none_or(|errs| only_curved_within_reach(&r.body, errs)),
        "{what}: 3′ on its records: {verdict:?}\n{:?}",
        r.contacts
    );

    let ee: Vec<(EdgeKey, EdgeKey)> = r.contacts.edge_edge().collect();
    let [pair] = ee[..] else {
        panic!("{what}: one edge–edge record, got {ee:?}");
    };
    let len = (line[1] - line[0]).norm();
    let dir = (line[1] - line[0]) * (1.0 / len);
    let mut spans = Vec::new();
    let mut solids = Vec::new();
    for e in [pair.0, pair.1] {
        let edge = r.body.get_edge(e).expect("a recorded edge is live");
        let mut ts = Vec::new();
        for he in [edge.he_plus, edge.he_minus] {
            let p = r.body.half_edge_start_point(he).expect("a live edge's end");
            let w = p - line[0];
            assert!(
                (w - dir * w.dot(dir)).norm() < 1e-9,
                "{what}: {e:?} ends at {p:?}, off the line"
            );
            ts.push(w.dot(dir) / len);
        }
        spans.push((ts[0].min(ts[1]), ts[0].max(ts[1])));
        solids.push(
            r.body
                .face_of_half_edge(edge.he_plus)
                .and_then(|f| r.body.solid_of_face(f))
                .expect("a live edge's solid"),
        );
    }
    assert_ne!(
        solids[0], solids[1],
        "{what}: the record's edges, one per solid"
    );
    spans.sort_by(|p, q| p.0.total_cmp(&q.0));
    let covers = spans[0].0 < 1e-9
        && spans[1].0 <= spans[0].1 + 1e-9
        && spans[0].1.max(spans[1].1) > 1.0 - 1e-9;
    assert!(
        covers,
        "{what}: the record's edges cover the touch segment: {spans:?}"
    );

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
    let overlap = |e: &ValidationError| {
        matches!(
            e,
            ValidationError::UnrecordedContact {
                contact: topo::CensusContact::EdgeEdgeOverlap { .. },
                ..
            }
        )
    };
    for (x, y) in r.contacts.vertex_vertex() {
        let key = |v| {
            r.body
                .get_vertex(v)
                .expect("a recorded vertex is live")
                .point
        };
        assert_eq!(
            key(x),
            key(y),
            "{what}: a recorded vertex pair shares one key"
        );
    }
    assert!(
        !errs.iter().any(overlap) && only_curved_within_reach(&r.body, &errs),
        "{what}: without its edge–edge record the overlap between the two shared \
         keys reads as structure (the shared ends ratified on PR 4793: the records are \
         the lint's evidence, not the census's): {errs:?}"
    );
}
