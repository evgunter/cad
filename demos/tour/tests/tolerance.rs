//! `demo_tour::tolerance`'s tests.

mod tests {
    use demo_tour::plate::{Plate, RADIUS_SIGMA, SPACING_HALF_WIDTH, WEB, WEB_BOUND, plate};
    use demo_tour::tolerance::*;
    use pncad::analysis::{AnalysisPolicy, McConfig, analyzed_box, drive, monte_carlo};
    use pncad::document::{CancelToken, EvalOptions, ValuePayload, evaluate};
    use pncad::geom_core::Tol;

    /// The verdict bits a plate's assertion lands on: its nominal `f64`
    /// verdict, the Monte Carlo lane's verdict tally and its population
    /// of the asserted web, and — at the certifiable box — what the
    /// requirement decides over the certified leaves, with its masses.
    /// The pins, one per ε row CI gates (1e-6, the default 1e-9,
    /// 1e-12): the studies are built from `tol.eps()`, so their verdict
    /// bits are a function of the row. At 1e-6 the Monte Carlo lane
    /// leaves 24 of the real plate's samples unevaluated, where the
    /// finer rows evaluate all of them.
    const PINNED: [(f64, &str); 3] = [
        (
            1e-6,
            "real: nominal Holds Holds { measured: 4558673246493684320, bound: 4557750909289998842 }; mc holds=482 violated=6 unevaluated=24 web 3f438e85babb60ac 3f0e4e721371c0fa 3f3fb4c4f36116c8 3f47f8928a0e25e4 512\n\
cut: nominal Holds Holds { measured: 4558673246493684320, bound: 4557750909289998842 }; mc holds=482 violated=6 unevaluated=24 web 3f438e85babb60ac 3f0e4e721371c0fa 3f3fb4c4f36116c8 3f47f8928a0e25e4 512\n\
certifiable: nominal Holds Holds { measured: 4558673246493684320, bound: 4558673090415105098 }; mc holds=512 violated=0 unevaluated=0 web 3f43a929d0544c58 3e33818d12ab46b9 3f43a90e02f5d8a0 3f43a948c8f04d5c 512; leaves HOLDS over every certified leaf holds 3fefd3d2aadd38d5 violated 0000000000000000 unevaluated 0000000000000000",
        ),
        (
            1e-9,
            "real: nominal Holds Holds { measured: 4558673246493684320, bound: 4557750909289998842 }; mc holds=499 violated=13 unevaluated=0 web 3f438e85babb60ac 3f0e4e721371c0fa 3f3fb4c4f36116c8 3f47f8928a0e25e4 512\n\
cut: nominal Holds Holds { measured: 4558673246493684320, bound: 4557750909289998842 }; mc holds=499 violated=13 unevaluated=0 web 3f438e85babb60ac 3f0e4e721371c0fa 3f3fb4c4f36116c8 3f47f8928a0e25e4 512\n\
certifiable: nominal Holds Holds { measured: 4558673246493684320, bound: 4558673246337605741 }; mc holds=512 violated=0 unevaluated=0 web 3f43a92a303c9eb1 3d93f96572a381d8 3f43a92a291e92cc 3f43a92a382a5808 512; leaves HOLDS over every certified leaf holds 3fefd3d2aadd38d5 violated 0000000000000000 unevaluated 0000000000000000",
        ),
        (
            1e-12,
            "real: nominal Holds Holds { measured: 4558673246493684320, bound: 4557750909289998842 }; mc holds=499 violated=13 unevaluated=0 web 3f438e85babb60ac 3f0e4e721371c0fa 3f3fb4c4f36116c8 3f47f8928a0e25e4 512\n\
cut: nominal Holds Holds { measured: 4558673246493684320, bound: 4557750909289998842 }; mc holds=499 violated=13 unevaluated=0 web 3f438e85babb60ac 3f0e4e721371c0fa 3f3fb4c4f36116c8 3f47f8928a0e25e4 512\n\
certifiable: nominal Holds Holds { measured: 4558673246493684320, bound: 4558673246493528241 }; mc holds=512 violated=0 unevaluated=0 web 3f43a92a30552c16 3cf474203799a69b 3f43a92a305359a4 3f43a92a305733b0 512; leaves HOLDS over every certified leaf holds 3fefd3d2aadd38d3 violated 0000000000000000 unevaluated 0000000000000000",
        ),
    ];

    fn verdict_bits(name: &str, p: &Plate, leaves: bool, tol: Tol) -> String {
        let ev = evaluate::<f64>(
            &p.doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            tol,
        );
        let nominal = match ev.value(p.assertion).map(|v| &v.payload) {
            Some(ValuePayload::Assertion(v)) => {
                format!("{} {:?}", v.label(), v.clone().map(f64::to_bits))
            }
            other => panic!("{name}: the assertion has a verdict, got {other:?}"),
        };
        let analyzed = analyzed_box(&p.doc, &AnalysisPolicy::default());
        let mc = monte_carlo(&p.doc, &analyzed, &McConfig::default(), tol)
            .expect("the Monte-Carlo lane answers");
        let tally = mc
            .assertions
            .iter()
            .find(|a| a.node == p.assertion)
            .expect("the assertion has a row");
        let web = {
            let v = mc
                .values
                .iter()
                .find(|v| v.var == p.web)
                .expect("the web has a row");
            (v.mean, v.sigma, v.min, v.max, v.measured)
        };
        let mut line = format!(
            "{name}: nominal {nominal}; mc holds={} violated={} unevaluated={} web \
             {:016x} {:016x} {:016x} {:016x} {}",
            tally.holds,
            tally.violated,
            tally.unevaluated,
            web.0.to_bits(),
            web.1.to_bits(),
            web.2.to_bits(),
            web.3.to_bits(),
            web.4
        );
        if leaves {
            let verdict = drive(&p.doc, &analyzed, &parallel(), tol).expect("the nominal builds");
            let (decided, masses) =
                requirement_over_leaves(&p.doc, &analyzed, &verdict, p.assertion, tol);
            line.push_str(&format!(
                "; leaves {} holds {:016x} violated {:016x} unevaluated {:016x}",
                describe(&decided),
                masses.holds.to_bits(),
                masses.violated.to_bits(),
                masses.unevaluated.to_bits()
            ));
        }
        line
    }

    /// **The plate's verdicts are the bits they were before a measure
    /// became one primitive** (INTENT stage 2 test 13): the web is a
    /// definition over the distance measure now, and every verdict
    /// over it — nominal, Monte Carlo and per leaf — lands where the
    /// measure's own arithmetic landed. Each ε row has its own pin
    /// ([`PINNED`]).
    #[test]
    fn the_plate_verdicts_hold_their_bits() {
        let tol = Tol::witness();
        let spread = tol.eps() / 64.0;
        let (half, sigma) = (0.05 * spread, 0.2 * spread);
        let worst = 2.0 * half + 2.0 * (3.0 * sigma);
        let rss3 = 3.0 * ((2.0 * half / 3.0_f64.sqrt()).powi(2) + 2.0 * sigma.powi(2)).sqrt();
        let lines = [
            verdict_bits("real", &demo_tour::plate::real_study(tol), false, tol),
            verdict_bits(
                "cut",
                &demo_tour::plate::cut_plate(SPACING_HALF_WIDTH, RADIUS_SIGMA, WEB_BOUND, tol),
                false,
                tol,
            ),
            verdict_bits(
                "certifiable",
                &plate(half, sigma, WEB - 0.5 * (worst + rss3), tol),
                true,
                tol,
            ),
        ]
        .join("\n");
        println!("{lines}");
        let eps = tol.eps();
        let Some((_, pinned)) = PINNED.iter().find(|(at, _)| *at == eps) else {
            panic!(
                "no plate verdicts are pinned at eps={eps}: run this row with --nocapture \
                 and add the printed lines as that ε's pin"
            );
        };
        assert_eq!(lines, *pinned, "a plate verdict moved at eps={eps}");
    }
}
