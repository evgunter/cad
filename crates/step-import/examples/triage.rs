//! Scratch triage driver for the M7 wild corpus (lane-only; never committed).
use std::panic::{self, AssertUnwindSafe};

fn main() {
    let path = std::env::args().nth(1).expect("usage: triage <file.step>");
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => {
            println!("READ-ERROR\t{path}\t{e}");
            return;
        }
    };
    let opts = step_import::ImportOptions { eps_in: None };
    let result = panic::catch_unwind(AssertUnwindSafe(|| step_import::import_step(&text, &opts)));
    match result {
        Ok(Ok(step_import::StepImport::Solid { body, eps_in, normalizations })) => {
            println!(
                "IMPORTS\t{path}\tsolids={} shells={} faces={} edges={} vertices={} eps_in={eps_in:.3e} normalizations={normalizations:?}",
                body.solids().count(),
                body.shells().count(),
                body.faces().count(),
                body.edges().count(),
                body.vertices().count()
            );
        }
        Ok(Ok(step_import::StepImport::Wireframe { curves, eps_in })) => {
            println!("WIREFRAME\t{path}\tcurves={} eps_in={eps_in:.3e}", curves.len());
        }
        Ok(Err(e)) => println!("REFUSES\t{path}\t{e}"),
        Err(_) => println!("PANIC\t{path}"),
    }
}
