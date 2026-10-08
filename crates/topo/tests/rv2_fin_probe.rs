//! Review 2 probe (PR 4289): the fin-void scene's tier 3′ on this tree.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use crate::common;
use common::meeting::{apex_pyramid, corners, fin, posed_box, posed_crown, poses};
use geom_core::Tol;
use topo::{
    BooleanResult, intersect, subtract, union, validate_geometric, validate_pseudomanifold,
};

fn t() -> Tol {
    Tol::witness()
}

#[test]
fn rv2_fin_void_three_prime() {
    for pose in poses() {
        let block = posed_box("block", [(1.0, 2.0), (0.5, 1.5), (0.3, 1.5)], &pose, t());
        let fin_b = posed_crown(&fin(), [0.0, 0.2, -0.6], &pose, t());
        let fin_void = match subtract(&block, &fin_b, t()) {
            Ok(BooleanResult::Body(r)) => r.body,
            other => panic!("fin void: {:?}", other.map(|_| ())),
        };
        let over = apex_pyramid(&corners(50.0, 0.7, 0.5), &pose, t());
        for (op, res) in [
            ("union", union(&fin_void, &over, t())),
            ("intersect", intersect(&fin_void, &over, t())),
            ("void - over", subtract(&fin_void, &over, t())),
            ("over - void", subtract(&over, &fin_void, t())),
        ] {
            match res {
                Ok(BooleanResult::Body(r)) => {
                    let g = validate_geometric(&r.body, t()).is_ok();
                    let p = validate_pseudomanifold(&r.body, &r.contacts, t());
                    println!(
                        "RV2FIN {} {op}: tier3 {g} tier3' {:?}",
                        pose.label,
                        p.map_err(|e| e
                            .iter()
                            .map(|e| format!("{e:?}").chars().take(160).collect::<String>())
                            .collect::<Vec<_>>())
                    );
                }
                other => println!(
                    "RV2FIN {} {op}: {:?}",
                    pose.label,
                    other
                        .map(|_| ())
                        .err()
                        .map(|e| format!("{e:?}").chars().take(200).collect::<String>())
                ),
            }
        }
    }
}
