//! EXCH-H1 Phase-1 probe (throwaway, uncommitted): dm1's import
//! outcome at the ambient witness band, printed verbatim.
#![allow(clippy::unwrap_used, clippy::print_stdout)]

use geom_core::Tol;
use step_import::{ImportOptions, import_step};

fn main() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/wild/stepcode/dm1-id-214.stp"
    );
    let text = std::fs::read_to_string(path).unwrap();
    let tol = Tol::witness();
    println!("ambient eps = {:e}", tol.get().eps);
    match import_step(&text, &ImportOptions::default(), tol) {
        Ok(imported) => {
            println!(
                "IMPORTED ok; promotions: {}",
                imported.curve_promotions().len()
            );
        }
        Err(e) => {
            println!("REFUSED (debug): {e:?}");
            println!("REFUSED (display): {e}");
        }
    }
}
