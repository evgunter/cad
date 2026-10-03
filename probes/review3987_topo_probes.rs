//! PR #3987 dual review r1 — in-crate probes (mounted as a child of
//! `boolean::refusal_routes`, beside `offer_rows`, to reach the stranded fixture):
//! `#[cfg(test)] #[path = "review3987_topo_probes.rs"] mod review3987_topo_probes;`
//! Mutants M1 / M3' / the seat meter are described in review.md.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use crate::boolean::{BooleanError, BooleanOp};
use geom_core::{Point3, Tol, Vec3};

fn plane_through(p: Point3<f64>, along: Vec3<f64>, n: Vec3<f64>) -> geom::Surface<f64> {
    crate::test_support_fixtures::plane(&[p, p + along, p + n.cross(along)], Tol::witness())
}

fn stranded() -> crate::body::Body<f64> {
    let up = Vec3::new(0.0, 0.0, 1.0);
    let offset = 1e3 * Tol::witness().get().eps;
    super::tests::top_split_redescribed(|p0, along, _| plane_through(p0 + up * offset, along, up))
}

/// Claim 2: the stranded operand refuses at validate — but the public
/// `boolean_reduce` door still takes `&Body`. Where does it end?
#[test]
fn probe_stranded_operand_through_public_boolean_reduce() {
    let tol = Tol::witness();
    let brick = crate::test_support_fixtures::brick::<f64>((0.3, 2.0), (0.2, 0.7), (0.5, 1.5), tol);
    let mut invariant = 0;
    for op in [BooleanOp::Union, BooleanOp::Subtract, BooleanOp::Intersect] {
        let got = crate::boolean::boolean_reduce(op, &stranded(), &brick, tol);
        let text = match &got { Ok(_) => "Ok(reduction)".to_string(), Err(e) => format!("{e:?}") };
        eprintln!("boolean_reduce {op:?} stranded × brick → {}", &text[..text.len().min(300)]);
        if matches!(got, Err(BooleanError::ClassificationInvariant { .. })) { invariant += 1; }
    }
    assert_eq!(invariant, 0, "the stranded operand reaches ClassificationInvariant through a public door");
}
