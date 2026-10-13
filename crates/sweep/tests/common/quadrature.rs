//! **The 7–15 Gauss–Kronrod panel** the suites' volume oracles integrate
//! with: one panel's Kronrod estimate and its distance from the embedded
//! Gauss rule, which each caller's own refinement (recursive halving,
//! or the globally adaptive worst-panel split) reads as the panel's
//! error.
//!
//! **Deliberately not absorbed**: nothing. The suites' other
//! quadratures (`a_ring_on_a_cone_face`'s 5-point and
//! `band_turn_overrun_fillet_model`'s 10-point Gauss–Legendre,
//! `verbs_germarms`'s solved 20-point one) are other rules, not copies
//! of this one.

/// The Kronrod nodes on `[0, 1]`, the odd-indexed ones the Gauss nodes.
const XK: [f64; 8] = [
    0.991_455_371_120_812_6,
    0.949_107_912_342_758_5,
    0.864_864_423_359_769_1,
    0.741_531_185_599_394_4,
    0.586_087_235_467_691_1,
    0.405_845_151_377_397_2,
    0.207_784_955_007_898_5,
    0.0,
];
const WK: [f64; 8] = [
    0.022_935_322_010_529_22,
    0.063_092_092_629_978_55,
    0.104_790_010_322_250_18,
    0.140_653_259_715_525_92,
    0.169_004_726_639_267_9,
    0.190_350_578_064_785_4,
    0.204_432_940_075_298_9,
    0.209_482_141_084_727_83,
];
const WG: [f64; 4] = [
    0.129_484_966_168_869_7,
    0.279_705_391_489_276_7,
    0.381_830_050_505_118_9,
    0.417_959_183_673_469_4,
];

/// `∫ f` over `[a, b]` by one 7–15 panel: the Kronrod estimate and
/// `|Kronrod − Gauss|`.
pub fn gk15(f: &dyn Fn(f64) -> f64, a: f64, b: f64) -> (f64, f64) {
    let (c, h) = ((a + b) / 2.0, (b - a) / 2.0);
    let fc = f(c);
    let (mut k, mut g) = (WK[7] * fc, WG[3] * fc);
    for i in 0..7 {
        let s = f(c - h * XK[i]) + f(c + h * XK[i]);
        k += WK[i] * s;
        if i % 2 == 1 {
            g += WG[i / 2] * s;
        }
    }
    (k * h, ((k - g) * h).abs())
}
