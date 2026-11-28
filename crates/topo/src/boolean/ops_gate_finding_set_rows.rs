//! **The Boolean's gate answers the finish's finding set where
//! completion refuses** (`crate::finish::fed`): a body whose completion
//! refuses, fed to the gate, answers the same findings as `finish` on
//! that body, and never `Ok` — one body, one finding set at every door.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;

use crate::body::Body;
use crate::entity::HalfEdgeKey;

fn set(r: Vec<crate::ValidationError>) -> Vec<String> {
    let mut v: Vec<String> = r.iter().map(|e| format!("{e:?}")).collect();
    v.sort();
    v
}

/// Every half-edge with a stored joint element: its face's images
/// dropped and the element made stale, so completion walks the loop on
/// the kept element. Where completion refuses, the gate's answer and
/// the finish's are compared.
#[test]
fn r2_gate_answers_the_finish_set_where_completion_refuses() {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let mut base: Body<f64> = crate::test_support_fixtures::seamless_drum(1.0, 2.0, tol);
    assert!(
        crate::finish::complete_in_place(&mut base, tol)
            .unwrap()
            .is_empty()
    );
    crate::finish(base.clone(), tol).expect("the drum finishes");
    let jointed: Vec<HalfEdgeKey> = base
        .half_edges
        .keys()
        .filter(|&he| base.joint(he).is_some())
        .collect();
    assert!(!jointed.is_empty(), "the drum stores a joint element");
    let mut exercised = 0;
    let mut report = Vec::new();
    for he in jointed {
        let face = base.face_of_half_edge(he).unwrap();
        for shift in [1i32, -1, 2] {
            let mut body = base.clone();
            for h in body.proven_face(face).half_edges() {
                body.detach_pcurve(h);
            }
            let el = body.joint(he).unwrap().deck();
            body.attach_joint(
                he,
                crate::joint::JointElement(crate::joint::Deck {
                    u: el.u + shift,
                    ..el
                }),
            );
            let mut probe = body.clone();
            let refused = crate::finish::complete_in_place(&mut probe, tol).unwrap();
            let finish = crate::finish(body.clone(), tol)
                .map(drop)
                .map_err(|e| e.as_slice().to_vec());
            let gate = super::gate(body.clone(), &super::Held::nothing(), band, tol);
            let gate = match gate {
                Ok(_) => Ok(()),
                Err(crate::boolean::BooleanError::ResultInvalid { findings }) => {
                    Err(findings.as_slice().to_vec())
                }
                Err(other) => panic!("{he:?} shift {shift}: the gate answered {other:?}"),
            };
            report.push(format!(
                "{he:?} shift {shift}: refused {} finish {:?} gate {:?}",
                refused.len(),
                finish.as_ref().map_err(|e| e.len()),
                gate.as_ref().map_err(|e| e.len())
            ));
            if refused.is_empty() {
                continue;
            }
            exercised += 1;
            let finish = finish.expect_err("completion refused, the finish refuses");
            let gate = gate.expect_err("completion refused, the gate refuses");
            assert_eq!(set(gate), set(finish), "{he:?} shift {shift}");
        }
    }
    println!("{}", report.join("\n"));
    assert!(
        exercised > 0,
        "no plant made completion refuse:\n{}",
        report.join("\n")
    );
}
