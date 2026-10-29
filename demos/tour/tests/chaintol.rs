//! `demo_tour::chaintol`'s tests.

mod tests {
    use demo_tour::chain::{
        CERTIFIABLE_FRACTION_BY_LINKS, JOINT_SIGMA, LINKS, POSITION_BOUND, chain,
    };
    use demo_tour::chaintol::*;
    use pncad::geom_core::Tol;

    /// **The plain lane's reach, bisected** — the header's
    /// `σ = 6.49e-9` at one link and `6.73e-10` at four: one leaf over
    /// the whole box at plain `Interval`, bisected in `log10 σ` to 0.005
    /// between a certifying and a refusing σ, printing the bracket and
    /// the refusal just above it, and the first refusal at the study's
    /// σ. `#[ignore]`d, and why at the header's claim. Run from
    /// `demos/tour`: `cargo test --release --bins
    /// the_plain_lanes_reach_is_bisected -- --ignored --nocapture`
    /// (`CHAIN_REACH_LINKS=1,4` is the default).
    #[test]
    #[ignore]
    fn the_plain_lanes_reach_is_bisected() {
        let tol = Tol::witness();
        let links_list: Vec<usize> = std::env::var("CHAIN_REACH_LINKS")
            .map(|v| v.split(',').map(|x| x.parse().unwrap()).collect())
            .unwrap_or_else(|_| vec![1, 4]);
        let leaf = |links: usize, sigma: f64| {
            let built = chain(links, sigma, POSITION_BOUND, tol);
            interval_leaf(links, &built.doc)
        };
        for links in links_list {
            let at = leaf(links, JOINT_SIGMA);
            println!(
                "{links} link(s) at σ = {JOINT_SIGMA:e}: certifies={} refused={} first={:?}",
                at.certifies, at.refused, at.first
            );
            let (mut lo, mut hi) = (-14.0_f64, JOINT_SIGMA.log10());
            assert!(
                leaf(links, 10f64.powf(lo)).certifies,
                "the lower end of the bisection must certify"
            );
            while hi - lo >= 0.005 {
                let mid = 0.5 * (lo + hi);
                if leaf(links, 10f64.powf(mid)).certifies {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            println!(
                "{links} link(s): certifies at σ = {:.4e}, refuses at {:.4e}: {:?}",
                10f64.powf(lo),
                10f64.powf(hi),
                leaf(links, 10f64.powf(hi)).first
            );
        }
    }

    /// **What BOUNDS the certifiable box is `dihedral_wedge`.**
    ///
    /// Not `dihedral_arm`, which was the first refusal over the WHOLE
    /// study at three and four links while each link was placed by a
    /// world-pin mate holding a literal placement, and what this cell's
    /// header claimed for a while. The question the box answers is
    /// asked just above the wall, and this is that question executed:
    /// at `1.02×` and `1.10×` of each link count's certifiable
    /// fraction, the first refusal is the wedge's POISONED margin, at
    /// two, three and four links alike.
    #[test]
    fn the_wall_is_the_wedge_not_the_arm() {
        let tol = Tol::witness();
        for (i, f) in CERTIFIABLE_FRACTION_BY_LINKS.iter().enumerate().skip(1) {
            let links = i + 1;
            for over in [1.02, 1.10] {
                let built = chain(links, JOINT_SIGMA * f * over, POSITION_BOUND, tol);
                let row = sym_leaf(links, &built.doc);
                assert!(
                    !row.certifies,
                    "{over}× the {links}-link wall must be past it; it certified"
                );
                let first = row.first.expect("a refusing row names its first refusal");
                assert!(
                    first.contains("predicate: Some(\"dihedral_wedge\")"),
                    "the header says the WALL is `dihedral_wedge`; at {links} links, \
                     {over}× the fraction, the first refusal was: {first}"
                );
                assert!(
                    first.contains("tangent planes at sample") && first.contains("Invalid"),
                    "the header says the wall's margin is POISONED (no tangent plane is \
                     defined), not straddling; at {links} links, {over}× the fraction: {first}"
                );
            }
        }
    }

    /// **The tip's certified box is half the PIN RADIUS**, at every
    /// link count whose box the wall sets.
    ///
    /// This is the statement the four fractions are four spellings of,
    /// and it is the one that was got wrong: the header read the
    /// invariance as a property of the tier (an angle) when it is a
    /// property of this document's geometry. Pinned because a reader
    /// who changes `PIN_RADIUS` should be told by a test, not by a
    /// caption that quietly stops being true.
    ///
    /// The one-link chain is out of scope by construction: its widest
    /// box is the STUDY, not the wall, and it sits at `0.450`.
    #[test]
    fn the_certified_tip_box_is_half_the_pin_radius() {
        let tol = Tol::witness();
        let mut ratios = Vec::new();
        for (i, f) in CERTIFIABLE_FRACTION_BY_LINKS.iter().enumerate().skip(1) {
            let links = i + 1;
            let boxes = certified_pin_boxes(links, *f, tol);
            let tip = boxes.last().expect("a chain has a tip pin").1;
            ratios.push(tip / demo_tour::chain::PIN_RADIUS);
        }
        let published = demo_tour::chain::CERTIFIED_TIP_OVER_PIN_RADIUS;
        let drifted = ratios
            .iter()
            .any(|r| (r - published).abs() > 0.02 * published);
        assert!(
            !drifted,
            "chain::CERTIFIED_TIP_OVER_PIN_RADIUS is {published:e}; the certified tip \
             half-width over the pin radius measures {ratios:?} at 2..{LINKS} links (ε = \
             {}). Re-baseline the constant and say in the PR what moved.",
            tol.eps()
        );
    }

    /// **The published fractions are the measured ones**, at every
    /// link count.
    ///
    /// The last of them is drawn to scale by the density sheet, so a
    /// number that drifted from what the tier actually reaches would
    /// be a caption about a box that does not exist; and the four
    /// together are what say the wall is ONE threshold, so a drift in
    /// any of them is a change in that claim.
    ///
    /// The whole table is at the AMBIENT ε — `ci.yml` runs this row at
    /// the default — because the fractions move with ε: `1.110e-1` at
    /// the default against `1.083e-1` at `1e-6`, measured, for the
    /// reason the module header's "What sets the wall" gives.
    ///
    /// It also runs the check the bisection's monotonicity assumption
    /// owes (see `certifiable_fraction`): each answer is re-certified
    /// at 1/2 and 1/4 of its width, so a refusal returning at a
    /// NARROWER box — which the poisoned wall gives no argument
    /// against — reds here.
    #[test]
    fn the_published_certifiable_fractions_are_the_measured_ones() {
        let tol = Tol::witness();
        let measured: Vec<f64> = (1..=LINKS).map(|n| certifiable_fraction(n, tol)).collect();
        // The check the bisection's monotonicity assumption owes: a
        // narrower box must still certify. A refusal other than the
        // wall's would be seen here if one came back narrower.
        for (i, f) in measured.iter().enumerate() {
            let links = i + 1;
            for half in [0.5, 0.25] {
                let built = chain(links, JOINT_SIGMA * f * half, POSITION_BOUND, tol);
                let row = sym_leaf(links, &built.doc);
                assert!(
                    row.certifies,
                    "the fraction is reported as the WIDEST box that certifies, which \
                     assumes narrower ones do; at {links} link(s), {half}× of {f:e} of the \
                     study REFUSES: {:?}",
                    row.first
                );
            }
        }
        let drifted = measured
            .iter()
            .zip(CERTIFIABLE_FRACTION_BY_LINKS)
            .any(|(m, p)| (m - p).abs() > 0.02 * p);
        assert!(
            !drifted,
            "chain::CERTIFIABLE_FRACTION_BY_LINKS is {CERTIFIABLE_FRACTION_BY_LINKS:?}; the \
             widest whole-certifying boxes measure {measured:?} at ε = {}. Re-baseline the \
             constant and say in the PR what moved.",
            tol.eps()
        );
        assert_eq!(
            demo_tour::chain::CERTIFIABLE_FRACTION,
            CERTIFIABLE_FRACTION_BY_LINKS[LINKS - 1],
            "the sheet's fraction is the table's last row"
        );
    }

    /// **The widest box of a chain that certifies whole, as a fraction of
    /// the real study** — the plate's `CERTIFIABLE_FRACTION`, measured the
    /// same way: ONE leaf over the whole box, at the study's σ scaled by
    /// `f`, with nothing failed and nothing poisoned.
    ///
    /// Searched in the log: `f` is halved from 1 until a leaf certifies
    /// (or the floor is reached), then the bracket is bisected in the
    /// exponent. A ratio is the right variable here for the reason it is
    /// on the plate — what the number says is how much of the study the
    /// certified answer covers, and "how much" is scale-free.
    ///
    /// In the test suite because it is the MEASUREMENT, not the narration:
    /// the numbers it produces are published as
    /// [`CERTIFIABLE_FRACTION_BY_LINKS`] and the cell prints those, so a
    /// tour walk pays one leaf to say whether the published box still
    /// certifies at its ε rather than paying a whole bisection per link
    /// count on every run of `eps_regression`.
    fn certifiable_fraction(links: usize, tol: Tol) -> f64 {
        let certifies = |f: f64| {
            let built = chain(links, JOINT_SIGMA * f, POSITION_BOUND, tol);
            sym_leaf(links, &built.doc).certifies
        };
        if certifies(1.0) {
            return 1.0;
        }
        // The floor: 2^-40 of a 0.01 rad study is 9e-15 rad, narrower than
        // any box a reader would call a study. A chain that has not
        // certified by there has not certified.
        const FLOOR: f64 = -40.0;
        let mut hi = 0.0f64; // log2 of a fraction that REFUSES
        let lo = loop {
            let next = hi - 1.0;
            if next < FLOOR {
                return 0.0;
            }
            if certifies(next.exp2()) {
                break next;
            }
            hi = next;
        };
        // Eight bisections of the EXPONENT — the answer to about a third of
        // a percent, which is more than the two digits it is reported to.
        //
        // **What the answer establishes, and what it assumes.** Both ends
        // of it are executed: the width returned CERTIFIES (the search's
        // last accepted probe, re-measured by
        // `the_published_certifiable_fractions_are_the_measured_ones`),
        // and 1.02× that width REFUSES
        // (`the_wall_is_the_wedge_not_the_arm`). What is ASSUMED is
        // monotonicity BELOW the answer — that no narrower box refuses.
        //
        // That assumption cannot be argued from the shape of the refusal at
        // the default ε. The refusal at the wall is the wedge's POISONED
        // margin (the module header's "What sets the wall") — a NaN, which
        // is not classified against a band at all — so a straddle argument
        // (a sub-box's enclosure lies inside the wider box's) does not
        // transfer to it, and nothing here establishes that a poison
        // cannot reappear at a narrower width.
        //
        // So the assumption is carried as an assumption, and the guard is
        // the half and quarter rows: the fractions row re-certifies each
        // answer at 1/2 and 1/4 of its width, so a refusal returning
        // anywhere in that range reds. Outside it the
        // claim the constants actually carry is the executed one —
        // certifies at this width, refuses 2% above.
        let mut lo = lo;
        for _ in 0..8 {
            let mid = f64::midpoint(lo, hi);
            if certifies(mid.exp2()) {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        lo.exp2()
    }
}
