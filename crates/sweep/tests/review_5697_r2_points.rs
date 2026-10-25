//! Review R2 of PR 5697: `point_in_solid` about every edge of a mixed
//! curved corpus against its analytic truth. Run on base and head and
//! diffed. Evidence, not pins:
//! `cargo nextest run -p sweep --run-ignored only review_5697_r2_points --no-capture`.

#![allow(clippy::expect_used, clippy::panic, clippy::print_stdout)]

use geom_core::{Band, Tol};
use topo::SolidContainment;

use super::review_5697_r2_corpus::{Rng, corpus, edge_points, robust};

#[test]
#[ignore = "review probe"]
fn review_5697_r2_points_about_edges() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("band");
    let k_eps = tol.k() * tol.eps();
    let only = std::env::var("R2_ONLY").ok();
    for (name, body, truth) in corpus(tol) {
        if only.as_deref().is_some_and(|o| !name.contains(o)) {
            continue;
        }
        let mut rng = Rng::new(name.bytes().fold(7u64, |h, b| h.wrapping_mul(31).wrapping_add(u64::from(b))));
        let (mut codes, mut wrong, mut robust_n, mut read_robust, mut in_band_read) =
            (String::new(), 0, 0, 0, 0);
        let mut errs: std::collections::BTreeMap<String, usize> = Default::default();
        for (label, p) in edge_points(&body, &[0.1, 0.3, 0.5, 0.7, 0.9]) {
            for _ in 0..8 {
                let q = p + rng.unit() * (rng.range(0.2, 5.0) * k_eps);
                let want = robust(&truth, q, 1.05 * k_eps);
                let got = topo::point_in_solid(&body, q, band, tol);
                if want.is_some() {
                    robust_n += 1;
                }
                let c = match (&got, want) {
                    (Ok(SolidContainment::In), Some(true)) | (Ok(SolidContainment::Out), Some(false)) => {
                        read_robust += 1;
                        'r'
                    }
                    (Ok(SolidContainment::In | SolidContainment::Out), Some(_)) => {
                        wrong += 1;
                        println!("R2P WRONG eps={:e} {name}: {label} q={q:?} got {got:?} truth in={:?}", tol.eps(), want);
                        'W'
                    }
                    (Ok(SolidContainment::In | SolidContainment::Out), None) => {
                        in_band_read += 1;
                        let t = truth(q);
                        if (got == Ok(SolidContainment::In)) == t { 'b' } else { 'x' }
                    }
                    (Ok(SolidContainment::OnBoundary), _) => '.',
                    (Ok(_), _) => '?',
                    (Err(e), _) => {
                        let k = format!("{e:?}").split(['(', ' ', '{']).next().unwrap_or("").to_owned();
                        *errs.entry(k).or_default() += 1;
                        'E'
                    }
                };
                codes.push(c);
            }
        }
        println!(
            "R2P eps={:e} {name}: robust {robust_n} read-right {read_robust} WRONG {wrong} in-band-read {in_band_read} errs {errs:?}",
            tol.eps()
        );
        println!("R2PC eps={:e} {name}: {codes}", tol.eps());
    }
}
