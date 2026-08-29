//! R2 consumer probe for PCURVE P-2 (PR #1177), item 1 / #1157.
//!
//! Audits `Vec3::orthonormal_basis` against DUAL-DESIGN DL6 — "a
//! certified lane may return Invalid/NaI only when the inputs pose no
//! real question, and must take a widening path over an absorbing one
//! where both exist" — rather than merely against "does it still
//! poison at n.z = [0,0]".
//!
//! Run:  cargo run -p geom-core --features interval --example r2_onb_dl6
#![allow(clippy::print_stdout)]

#[cfg(not(feature = "interval"))]
fn main() {
    println!("needs --features interval");
}

#[cfg(feature = "interval")]
fn main() {
    use geom_core::Vec3;
    use geom_core::interval::Interval;
    use geom_core::real::{Bounds, Real};

    let iv = |lo: f64, hi: f64| Interval::from_bounds(lo, hi);
    let pt = |x: f64| Interval::from_f64(x);

    // The TRUE range of the denominator magnitude 1 + |n.z| over any
    // enclosure of n.z is [1, 1 + max|n.z|] — bounded away from zero
    // for every input, unit or not. So `r = 1/(1+|n.z|)` is bounded
    // in (0, 1] at EVERY input; a real question is always posed.
    println!("== A. the shipped spelling vs. the spec's own `1 + |n.z|` ==");
    println!(
        "{:<26} {:>34} {:>34} {:>8}",
        "n.z enclosure", "shipped b1.x", "abs-spelled b1.x", "unbdd?"
    );
    let mut absorbing = 0usize;
    for (lo, hi, label) in [
        (0.0, 0.0, "[0, 0]  (#1157's case)"),
        (-0.0, -0.0, "[-0, -0]"),
        (-1e-9, 1e-9, "[-1e-9, 1e-9]"),
        (-0.25, 0.25, "[-0.25, 0.25]"),
        (-0.5, 0.5, "[-0.5, 0.5]"),
        (-0.9, 0.9, "[-0.9, 0.9]"),
        (-1.0, 1.0, "[-1, 1] (z unknown)"),
        (-1.0, 0.0, "[-1, 0]"),
        (0.0, 1.0, "[0, 1]"),
    ] {
        let n = Vec3::new(pt(0.6), pt(0.8), iv(lo, hi));
        let (b1, _) = n.orthonormal_basis();
        // The same construction with the correlation FULLY restored,
        // exactly as `docs/PCURVE-P2-SPEC.md` item 1 spells it:
        // "compute the magnitude `1 + |n.z|` and apply the sign".
        let s = Interval::one().copysign(n.z);
        let r_abs = Interval::one() / (Interval::one() + n.z.abs());
        let b1x_abs = Interval::one() - n.x.powi(2) * r_abs;
        let _ = s;
        let unb = !(b1.x.lo().is_finite() && b1.x.hi().is_finite());
        if unb {
            absorbing += 1;
        }
        println!(
            "{label:<26} {:>34} {:>34} {:>8}",
            format!("[{:.6}, {:.6}]", b1.x.lo(), b1.x.hi()),
            format!("[{:.6}, {:.6}]", b1x_abs.lo(), b1x_abs.hi()),
            if unb { "YES" } else { "" }
        );
    }
    println!("\nabsorbing (unbounded) rows under the shipped spelling: {absorbing}");

    println!("\n== B. decoration at #1157's own input ==");
    let n = Vec3::new(pt(0.0), pt(-1.0), pt(0.0));
    let (b1, b2) = n.orthonormal_basis();
    let r_abs = Interval::one() / (Interval::one() + n.z.abs());
    let b1x_abs = Interval::one() - n.x.powi(2) * r_abs;
    println!(
        "shipped  b1.x certified={} b1.y certified={} b2.y certified={}",
        b1.x.is_certified(),
        b1.y.is_certified(),
        b2.y.is_certified()
    );
    println!("abs-spelled b1.x certified={}", b1x_abs.is_certified());

    println!("\n== C. is a straddling n.z reachable from a UNIT normal? ==");
    // A unit normal whose z is known only to lie in [-1, 1] is a
    // perfectly ordinary enclosure: it is what any interval-lane
    // normal computation returns for a direction that is not pinned
    // down. Every such n has a bounded frame in reality.
    let n = Vec3::new(iv(-1.0, 1.0), iv(-1.0, 1.0), iv(-1.0, 1.0));
    let (b1, b2) = n.orthonormal_basis();
    println!(
        "n = ([-1,1],[-1,1],[-1,1]):  b1 = ([{}, {}], [{}, {}], [{}, {}])",
        b1.x.lo(),
        b1.x.hi(),
        b1.y.lo(),
        b1.y.hi(),
        b1.z.lo(),
        b1.z.hi()
    );
    println!(
        "                             b2 = ([{}, {}], [{}, {}], [{}, {}])",
        b2.x.lo(),
        b2.x.hi(),
        b2.y.lo(),
        b2.y.hi(),
        b2.z.lo(),
        b2.z.hi()
    );
    println!(
        "b1.x poison(NaI/empty)? {}   certified? {}",
        b1.x.is_poison(),
        b1.x.is_certified()
    );

    println!("\n== D. the constructor doc's own sentence, tested ==");
    println!("doc: \"`r` is bounded in `(0, 1]` at every input, so no component can be");
    println!("      unbounded and none is decorated below `Def` by this construction.\"");
    for (lo, hi, label) in [
        (0.0, 1.0, "n.z = [0, 1]"),
        (-1.0, 1.0, "n.z = [-1, 1]"),
        (-0.9, 0.9, "n.z = [-0.9, 0.9]"),
    ] {
        let z = iv(lo, hi);
        let s = Interval::one().copysign(z);
        let r = Interval::one() / (Interval::one() + s * z);
        let n = Vec3::new(pt(0.6), pt(0.8), z);
        let (b1, _) = n.orthonormal_basis();
        println!(
            "{label:<20} r = [{}, {}]  in (0,1]? {}   b1.x certified(>=Def)? {}",
            r.lo(),
            r.hi(),
            r.lo() > 0.0 && r.hi() <= 1.0,
            b1.x.is_certified()
        );
    }
}

/// The constructor's shipped doc says, verbatim: "`r` is bounded in
/// `(0, 1]` at every input, so no component can be unbounded and none
/// is decorated below `Def` by this construction." This is the direct
/// test of that sentence.
#[cfg(feature = "interval")]
#[allow(dead_code)]
fn r_claim() {}
