//! REVIEW 2 PROBE (PR 4300), scratch branch only: dumps every body the
//! existing star rows build, for a main-vs-head diff.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stderr)]

use crate::common;
use common::meeting::{
    Hole, PLATE, Pose, ell_and_wedges, four_wedges, inner_rows, notch, notch_rows, orders,
    posed_box, posed_prism, poses, three_wedges, two_wedges, wedge, wedges_on_one_side,
};
use geom_core::Tol;
use std::fmt::Write as _;
use topo::{AtRestBody, BooleanResult, intersect, subtract, union};

fn t() -> Tol {
    Tol::witness()
}

fn dump(b: &AtRestBody<f64>) -> String {
    let mut s = String::new();
    for (k, v) in b.shells() {
        writeln!(s, "S {k:?} {v:?}").unwrap();
    }
    for (k, v) in b.faces() {
        writeln!(s, "F {k:?} {v:?}").unwrap();
    }
    for (k, v) in b.loops() {
        writeln!(s, "L {k:?} {v:?}").unwrap();
    }
    for (k, v) in b.half_edges() {
        writeln!(s, "H {k:?} {v:?}").unwrap();
    }
    for (k, v) in b.edges() {
        writeln!(s, "E {k:?} {v:?}").unwrap();
    }
    for (k, v) in b.vertices() {
        let p = topo::readback::vertex_point(b, k).unwrap();
        writeln!(s, "V {k:?} {v:?} {:?}", [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()]).unwrap();
    }
    s
}

fn out(name: &str, r: &Result<BooleanResult<f64>, topo::BooleanError>) {
    let dir = std::env::var("REVIEW2_DUMP").unwrap();
    let text = match r {
        Ok(BooleanResult::Body(b)) => dump(&b.body),
        Ok(BooleanResult::Empty) => "EMPTY".into(),
        Err(e) => format!("ERR {e:?}"),
    };
    let name: String = name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    std::fs::write(format!("{dir}/{name}.txt"), text).unwrap();
}

fn body(r: Result<BooleanResult<f64>, topo::BooleanError>) -> AtRestBody<f64> {
    match r {
        Ok(BooleanResult::Body(r)) => r.body,
        _ => panic!("{r:?}"),
    }
}

#[test]
fn review2_dump_star_rows() {
    let rest = Pose::rest();
    let rows: Vec<(&str, Vec<Hole>)> = vec![
        ("two", two_wedges()),
        ("three", three_wedges()),
        ("four", four_wedges()),
        ("oneside", wedges_on_one_side()),
        ("ell", ell_and_wedges()),
    ];
    for (label, holes) in &rows {
        let mut members = vec![posed_box("the plate", PLATE, &rest, t())];
        members.extend(holes.iter().map(|h| posed_prism(h, &rest, t())));
        for order in orders(members.len()) {
            let mut b = members[order[0]].clone();
            for (k, &i) in order.iter().enumerate().skip(1) {
                let r = union(&b, &members[i], t());
                out(&format!("order {label} {order:?} step {k}"), &r);
                b = body(r);
            }
        }
    }
    for pose in poses() {
        let p = posed_box("the plate", PLATE, &pose, t());
        for (fixture, holes) in inner_rows().into_iter().chain(notch_rows()) {
            let label = format!("{fixture} {}", pose.label);
            let prisms: Vec<_> = holes.iter().map(|h| posed_prism(h, &pose, t())).collect();
            let u = prisms[1..]
                .iter()
                .fold(prisms[0].clone(), |u, q| body(union(&u, q, t())));
            out(&format!("{label} PiU"), &intersect(&p, &u, t()));
            out(&format!("{label} UiP"), &intersect(&u, &p, t()));
            out(&format!("{label} PuU"), &union(&p, &u, t()));
            out(&format!("{label} UuP"), &union(&u, &p, t()));
            out(&format!("{label} PmU"), &subtract(&p, &u, t()));
            out(&format!("{label} UmP"), &subtract(&u, &p, t()));
        }
    }
    // grid(3) at rest
    let p = posed_box("the plate", PLATE, &rest, t());
    for a in 1..8usize {
        for b2 in a + 1..8 {
            let c = [0, a, b2];
            for mask in 0..8u32 {
                let holes: Vec<Hole> = c
                    .iter()
                    .enumerate()
                    .map(|(j, &s)| {
                        let a = 45.0f64.mul_add(s as f64, 5.0);
                        if mask >> j & 1 == 1 {
                            notch(a, a + 30.0, j, 2.0)
                        } else {
                            wedge(a, a + 30.0, j)
                        }
                    })
                    .collect();
                let prisms: Vec<_> = holes.iter().map(|h| posed_prism(h, &rest, t())).collect();
                let u = prisms[1..]
                    .iter()
                    .fold(prisms[0].clone(), |u, q| body(union(&u, q, t())));
                out(&format!("grid {c:?} {mask}"), &subtract(&p, &u, t()));
            }
        }
    }
}
