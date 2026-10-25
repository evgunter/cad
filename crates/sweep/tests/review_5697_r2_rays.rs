//! Review R2 of PR 5697 (head only): aimed rays at and about every edge
//! of a mixed curved corpus, in uniformly random directions, both ways
//! out. A double or missing count in the first cluster reads the wrong
//! side. Evidence, not pins.

#![allow(clippy::expect_used, clippy::panic, clippy::print_stdout)]

use geom_core::{Band, Tol};

use super::review_5697_r2_corpus::{Rng, corpus};

#[test]
#[ignore = "review probe"]
fn review_5697_r2_rays_about_edges() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("band");
    let k_eps = tol.k() * tol.eps();
    let only = std::env::var("R2_ONLY").ok();
    let per: usize = std::env::var("R2_PER").ok().and_then(|s| s.parse().ok()).unwrap_or(6);
    for (name, body, _) in corpus(tol) {
        if only.as_deref().is_some_and(|o| !name.contains(o)) {
            continue;
        }
        let Ok(rev) = body.revert() else {
            println!("R2R {name}: no revert");
            continue;
        };
        for (inside_out, b, want) in [(false, &body, "Side(Out)"), (true, &rev, "Side(In)")] {
            let mut rng = Rng::new(11 + u64::from(inside_out) + name.len() as u64);
            let mut tally: std::collections::BTreeMap<&str, usize> = Default::default();
            for f in topo::test_support::edge_frames(b, &[0.13, 0.5, 0.87], band) {
                for _ in 0..per {
                    let o = rng.range(-3.0, 3.0) * k_eps;
                    let jitter = rng.unit() * (rng.range(0.0, 2.0) * k_eps);
                    let u = rng.unit();
                    let aim = f.point + f.into * o + jitter;
                    let r = topo::test_support::ray_at(b, f.face, aim, u, band);
                    let k = if r.contains(want) {
                        "right"
                    } else if r.contains("Side(") {
                        println!(
                            "R2R WRONG eps={:e} {name} inside_out={inside_out} {} aim={aim:?} u={u:?}: {r}",
                            tol.eps(),
                            f.label
                        );
                        "WRONG"
                    } else if r.contains("Pair(") {
                        "pair"
                    } else if r.contains("SetAside(Crossing") {
                        "crossing"
                    } else if r.contains("Wedge") {
                        "wedge"
                    } else if r.contains("Corner") {
                        "corner"
                    } else if r.contains("Graze") {
                        "graze"
                    } else if r.contains("Limit") {
                        "limit"
                    } else {
                        "other"
                    };
                    *tally.entry(k).or_default() += 1;
                }
            }
            println!("R2R eps={:e} {name} inside_out={inside_out}: {tally:?}", tol.eps());
        }
    }
}
