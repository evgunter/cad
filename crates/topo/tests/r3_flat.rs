//! review-3 probe: a partner corner whose bounds all read On.
#![allow(clippy::all, clippy::pedantic, clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code, unused, unreachable_pub)]
use crate::common::meeting::{MEET, PLATE, Pose, at, posed_box, posed_pyramid, poses};
use geom_core::{Tol, Vec3};
use topo::{AtRestBody, BooleanResult, Operand, SideCode, intersect, readback, subtract, union, validate_geometric};
include!("../../../review3-probes/r3_scenes.rs");

#[test]
#[ignore]
fn r3_flat() {
    let t = Tol::witness();
    for pose in poses() {
        let plate = posed_box("plate", PLATE, &pose, t);
        let b1 = posed_box("b1", [(1.5, 3.6), (1.0, 1.8), (0.5, 1.0)], &pose, t);
        let b2 = posed_box("b2", [(-0.6, 1.5), (1.0, 1.8), (0.5, 1.0)], &pose, t);
        let u1 = match union(&plate, &b1, t) { Ok(BooleanResult::Body(u)) => u, o => { eprintln!("u1 fail {:?}", o.map(|_| ())); return } };
        let u1 = u1.body;
        let u2 = match union(&u1, &b2, t) { Ok(BooleanResult::Body(u)) => u, o => { eprintln!("u2 fail {:?}", o.map(|_| ())); return } };
        let flat = u2.body;
        let meet = at(pose.at(MEET));
        let mut deg = 0;
        for (v, _) in flat.vertices() {
            if at(readback::vertex_point(&flat, v).unwrap()) == meet {
                for (e, _) in flat.edges() {
                    let edge = flat.get_edge(e).unwrap();
                    let plus = flat.get_half_edge(edge.he_plus).unwrap();
                    let next = flat.get_half_edge(plus.next).unwrap();
                    if plus.start == v || next.start == v { deg += 1; }
                }
            }
        }
        eprintln!("{}: flat faces {} vertex-at-MEET degree {}", pose.label, flat.faces().count(), deg);
        for (n, b) in [("hang", r3_corners(240.0, -0.6, 0.5)), ("cone", r3_corners(240.0, 0.7, 0.5)), ("hang_over", r3_corners(130.0, -0.6, 0.5))] {
            let x = r3_apex(&b, &pose, t);
            for (what, r) in [("x-y", subtract(&x, &flat, t)), ("y-x", subtract(&flat, &x, t)), ("xUy", union(&x, &flat, t)), ("xNy", intersect(&x, &flat, t))] {
                match r {
                    Ok(BooleanResult::Body(r)) => {
                        let mut rows: Vec<String> = r.naming.edge_classes.iter().map(|row| format!("{:?}/{:?}/{:?}/{}/{:?}", row.operand, row.vertex, row.edge, row.starts, row.class)).collect();
                        rows.sort();
                        eprintln!("ROW {} {n} {what}: {} rows: {}", pose.label, rows.len(), rows.join(" "));
                    }
                    other => eprintln!("ROW {} {n} {what}: {:?}", pose.label, other.map(|_| ()).map_err(|e| format!("{e:?}").chars().take(150).collect::<String>())),
                }
            }
        }
    }
}
