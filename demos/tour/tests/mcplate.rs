//! `demo_tour::mcplate`'s tests.

mod tests {
    use demo_tour::mcplate::*;
    use demo_tour::plate::real_study;
    use pncad::analysis::{
        AnalysisPolicy, DEFAULT_SAMPLES, McConfig, analyzed_box, sample_offsets,
    };
    use pncad::geom_core::Tol;

    /// FNV-1a over `bytes`, folded into `h`.
    fn fnv(h: &mut u64, bytes: &[u8]) {
        for &b in bytes {
            *h ^= u64::from(b);
            *h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }

    /// **The Monte Carlo sheet does not move** (INTENT-LITERALS spec §8
    /// row 9): the draw matrix — every sample's offsets, each keyed by
    /// its variable's NAME, the varying axes in declaration order, since
    /// an id's digest moves when minting does — and the
    /// sheet's own bytes are the numbers measured on the tree before a
    /// slot held a variable. Only a distributed variable draws, so a
    /// typed value's anonymous variable enters neither.
    #[test]
    fn the_monte_carlo_sheet_is_pinned() {
        let tol = Tol::witness();
        let base = real_study(tol);
        let analyzed = analyzed_box(&base.doc, &AnalysisPolicy::default());
        let config = McConfig {
            samples: DEFAULT_SAMPLES,
            parallel: false,
            ..McConfig::default()
        };
        let mut draws = 0xcbf2_9ce4_8422_2325_u64;
        for i in 0..config.samples {
            let offsets = sample_offsets(&base.doc, &analyzed, &config, i).expect("sampleable");
            for (var, _) in analyzed.varying() {
                let name = base.doc.var_name(var).expect("the study's axes are named");
                fnv(&mut draws, name.as_str().as_bytes());
                fnv(&mut draws, &offsets[&var].to_bits().to_le_bytes());
            }
        }
        let mut sheet = 0xcbf2_9ce4_8422_2325_u64;
        fnv(&mut sheet, narration(tol).as_bytes());
        println!("mcplate draws {draws:#018x} sheet {sheet:#018x}");
        assert_eq!(
            (draws, sheet),
            (0xd633_699a_f382_1806, 0x6d3e_4b0b_0201_2542),
            "the Monte Carlo sheet moved"
        );
    }
}
