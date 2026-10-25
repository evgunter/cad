//! PR 5684 review probes (PRED S18): what the writer's new multi-shell
//! gate does on bodies the PR's rows do not reach. Printed evidence for
//! the reviewer; assertions pin what was measured.

use crate::common;
use geom_core::Tol;
use step_export::{StepExportError, StepOptions, step_string};
use topo::{AtRestBody, classify_shells_of};

fn shape(body: &topo::Body<f64>) -> Vec<usize> {
    body.solids().map(|(_, s)| s.shells.len()).collect()
}

#[test]
fn probe_kiss_assembly_shape_and_gate() {
    let body = common::kiss_assembly();
    let shells_per_solid = shape(&body);
    eprintln!("kiss: shells per solid = {shells_per_solid:?}");
    let gate = AtRestBody::validate(body.clone(), Tol::witness());
    eprintln!("kiss: validate = {:?}", gate.as_ref().map(|_| ()));
    if let Ok(fin) = &gate {
        let all: Vec<_> = fin.shells().map(|(k, _)| k).collect();
        let rows = classify_shells_of(fin, &all, Tol::witness());
        eprintln!(
            "kiss: roles = {:?}",
            rows.map(|r| r.iter().map(|c| (c.role, c.volume)).collect::<Vec<_>>())
        );
    }
    let r = step_string(&body, &StepOptions::default(), Tol::witness());
    eprintln!("kiss: export ok = {}", r.is_ok());
}

/// The single-shell twin of `an_inside_out_multi_shell_body_refuses_typed`.
#[test]
fn probe_inside_out_single_shell_brick() {
    let brick: topo::Body<f64> = common::brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), Tol::witness());
    let reverted = brick.revert().expect("reverts");
    let gate = AtRestBody::validate(reverted.clone(), Tol::witness());
    eprintln!(
        "reverted brick: validate = {:?}",
        gate.as_ref().map(|_| ()).map_err(|e| e.clone())
    );
    let r = step_string(&reverted, &StepOptions::default(), Tol::witness());
    eprintln!("reverted brick: export = {:?}", r.as_ref().map(|_| ()));
}

/// A voided body with an inside-out brick grafted beside it: which
/// refusal does the writer name?
#[test]
fn probe_voided_plus_far_inside_out_brick() {
    let mut body = common::voided();
    let brick: topo::Body<f64> =
        common::brick((10.0, 11.0), (0.0, 1.0), (0.0, 1.0), Tol::witness());
    let reverted = brick.revert().expect("reverts");
    topo::graft_disjoint(&mut body, &reverted).expect("graft");
    eprintln!("voided+brick: shells per solid = {:?}", shape(&body));
    let r = step_string(&body, &StepOptions::default(), Tol::witness());
    match &r {
        Err(StepExportError::MultiShellNotAtRest { findings }) => {
            eprintln!("voided+brick: MultiShellNotAtRest {findings:?}")
        }
        other => eprintln!("voided+brick: {:?}", other.as_ref().map(|_| ())),
    }
}
