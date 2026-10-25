//! Review R2 of PR 5697: the side reader's verdict on every corpus body,
//! both ways out, to diff base against head. Evidence, not pins.

#![allow(clippy::expect_used, clippy::panic, clippy::print_stdout)]

use geom_core::{Band, Tol};

use super::review_5697_r2_corpus::corpus;

#[test]
#[ignore = "review probe"]
fn review_5697_r2_reader_verdicts() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("band");
    for (name, body, _) in corpus(tol) {
        for (out, b) in [("as built", body.clone()), ("inside out", body.revert().expect("reverts"))] {
            let r: Vec<String> = topo::test_support::aimed_rays(&b, band)
                .iter()
                .map(|s| s.chars().take(60).collect())
                .collect();
            println!("R2V eps={:e} {name} {out}: {r:?}", tol.eps());
        }
    }
}
