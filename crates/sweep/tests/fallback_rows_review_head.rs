//! Review probe (PR 5706), head only: each run's `uncut` rows, and each
//! fallback's substituted fragment beside its holder, by area.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use topo::{BooleanResult, BooleanResultKind, Operand};

#[test]
#[ignore = "review probe; run with --ignored --nocapture"]
fn fallback_rows_review_uncut() {
    let tol = Tol::witness();
    let mut corpus = super::fallback_rows_review::corpus();
    {
        use super::fallback_rows_review::{circle, extruded, square};
        let fin = |w: &str, b| sweep::test_support::finished(w, b, tol);
        let boss = || extruded(vec![circle(0.0, 0.0, 0.3, 0.0)], 0.0, 1.0);
        let holed = fin(
            "holed",
            extruded(vec![square(1.0), circle(0.0, 0.0, 0.1, 0.0)], -0.5, 0.5),
        );
        let lid = fin("lid", extruded(vec![square(1.0)], 1.0, 0.5));
        let plain = fin("plain", extruded(vec![square(1.0)], -0.5, 0.5));
        let two = |a: &topo::AtRestBody<f64>, b: &topo::AtRestBody<f64>| {
            fin(
                "two",
                topo::union(a, b, tol)
                    .expect("pair")
                    .body()
                    .expect("material")
                    .body
                    .clone()
                    .into_body(),
            )
        };
        corpus.push(("boss/holed+lid", fin("boss", boss()), two(&holed, &lid)));
        corpus.push(("boss/plain+lid", fin("boss", boss()), two(&plain, &lid)));
        corpus.push(("boss/lid", fin("boss", boss()), lid.clone()));
    }
    for (label, x, y) in corpus {
        for (op, p, q, ord) in [
            ("∪", &x, &y, "xy"),
            ("∪", &y, &x, "yx"),
            ("∩", &x, &y, "xy"),
            ("∩", &y, &x, "yx"),
            ("∖", &x, &y, "xy"),
            ("∖", &y, &x, "yx"),
        ] {
            let r = match op {
                "∪" => topo::union(p, q, tol),
                "∩" => topo::intersect(p, q, tol),
                _ => topo::subtract(p, q, tol),
            };
            let Ok(BooleanResult::Body(out)) = r else {
                println!("RES {label} {op} {ord}: {:?}", r.map(|_| ()).err());
                continue;
            };
            println!(
                "RES {label} {op} {ord}: {:?} rows {} discards {}",
                out.kind,
                out.naming.substituted.len(),
                out.naming.discards.len()
            );
            let n = &out.naming;
            if !n.uncut.is_empty() {
                println!("UNCUT {label} {op} {ord} {:?}: {:?}", out.kind, n.uncut);
            }
            if matches!(
                out.kind,
                BooleanResultKind::OperandA | BooleanResultKind::OperandB
            ) {
                // Each row: the fragment's area against its holder's.
                for s in &n.substituted {
                    let (fb, hb) = match s.operand {
                        Operand::A => (p, q),
                        Operand::B => (q, p),
                    };
                    let _ = (fb, hb);
                    println!(
                        "ROW {label} {op} {ord}: {:?} root {:?}",
                        s,
                        match s.operand {
                            Operand::A => n.face_fragments_a.root(s.face),
                            Operand::B => n.face_fragments_b.root(s.face),
                        }
                    );
                }
            }
        }
    }
}
