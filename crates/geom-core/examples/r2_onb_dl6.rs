//! DL6 consumer probe for `Vec3::orthonormal_basis`.
//!
//! Audits the constructor against DUAL-DESIGN DL6 — "a certified lane
//! may return Invalid/NaI only when the inputs pose no real question,
//! and must take a widening path over an absorbing one where both
//! exist" — rather than merely against "is it bounded on the equator".
//!
//! The construction crosses the normal with `e_z` when
//! `|n.z| ≤ max(|n.x|, |n.y|)` and with `e_y` otherwise, and
//! normalizes. Two places could absorb, and both are audited here: the
//! axis choice at an enclosure that cannot decide it (which must HULL,
//! not refuse), and the normalization of a candidate (well conditioned
//! at the chosen axis, degenerate only at the one never chosen).
//!
//! Run:  cargo run -p geom-core --features interval --example r2_onb_dl6
#![allow(clippy::print_stdout)]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

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
    let show = |e: Interval| format!("[{:.6}, {:.6}]", e.lo(), e.hi());
    let bounded = |e: Interval| e.lo().is_finite() && e.hi().is_finite();

    // The equator is not a seam: |n.z| is far below max(|n.x|, |n.y|)
    // at a wall, so the axis choice decides and the frame is exact.
    println!("== A. the equator, where the sign-transfer spelling hulled ==");
    println!(
        "{:<26} {:>24} {:>10} {:>10}",
        "n.z enclosure", "b1", "bounded", "cert"
    );
    let mut absorbing = 0usize;
    for (lo, hi, label) in [
        (0.0, 0.0, "[0, 0]"),
        (-0.0, -0.0, "[-0, -0]"),
        (-1e-9, 1e-9, "[-1e-9, 1e-9]"),
        (-0.25, 0.25, "[-0.25, 0.25]"),
        (-0.5, 0.5, "[-0.5, 0.5]"),
        (-1.0, 1.0, "[-1, 1] (z unknown)"),
        (-1.0, 0.0, "[-1, 0]"),
        (0.0, 1.0, "[0, 1]"),
    ] {
        let n = Vec3::new(pt(0.6), pt(0.8), iv(lo, hi));
        let (b1, _) = n.orthonormal_basis();
        let unb = !(bounded(b1.x) && bounded(b1.y) && bounded(b1.z));
        if unb {
            absorbing += 1;
        }
        println!(
            "{label:<26} {:>24} {:>10} {:>10}",
            show(b1.x),
            if unb { "NO" } else { "yes" },
            b1.x.is_certified()
        );
    }
    println!("\nabsorbing (unbounded) rows: {absorbing}");

    // The undecided axis choice: the door hulls two UNIT candidates.
    // Normalizing after the selection instead would divide a hull that
    // contains the zero vector by its own norm — the absorbing path
    // DL6 forbids where a widening one exists.
    println!("\n== B. a straddled tie: hulled, not absorbed ==");
    let half = core::f64::consts::FRAC_1_SQRT_2;
    let n = Vec3::new(pt(half), pt(0.0), iv(half - 1e-6, half + 1e-6));
    let (b1, b2) = n.orthonormal_basis();
    println!(
        "shipped   b1 = ({}, {}, {})  certified={} poison={}",
        show(b1.x),
        show(b1.y),
        show(b1.z),
        b1.x.is_certified(),
        b1.x.is_poison()
    );
    println!(
        "          b2 = ({}, {}, {})",
        show(b2.x),
        show(b2.y),
        show(b2.z)
    );
    let d1 = n.z.abs() - n.x.abs().max(n.y.abs());
    let (cz, cy) = (
        Vec3::new(-n.y, n.x, Interval::zero()),
        Vec3::new(n.z, Interval::zero(), -n.x),
    );
    let late = Vec3::new(
        d1.select_le_zero(cz.x, cy.x),
        d1.select_le_zero(cz.y, cy.y),
        d1.select_le_zero(cz.z, cy.z),
    )
    .normalize();
    println!(
        "late-norm b1.x = {}  bounded={} certified={}   <- the rejected ordering",
        show(late.x),
        bounded(late.x),
        late.x.is_certified()
    );

    // The construction's measured limit, stated rather than left to be
    // discovered. `normalize` reads each candidate's OWN norm, so a box
    // wide enough to leave the comparison undecided AND to reach a
    // direction parallel to the axis it is choosing away from hulls in
    // that candidate's zero vector and comes back unbounded. That takes
    // a box spanning most of a meridian; a tight enclosure of a real
    // normal — including one straddling the cone — never reaches it.
    println!("\n== C. wide direction boxes ==");
    for (n, label) in [
        (
            Vec3::new(pt(0.0), pt(1.0), iv(0.0, 1.0)),
            "a whole meridian",
        ),
        (
            Vec3::new(iv(-1.0, 1.0), iv(-1.0, 1.0), iv(-1.0, 1.0)),
            "[-1,1]^3 (contains 0)",
        ),
        (
            Vec3::new(iv(0.59, 0.61), iv(0.79, 0.81), iv(-0.01, 0.01)),
            "a tight wall enclosure",
        ),
        (
            Vec3::new(pt(half), pt(0.0), iv(half - 1e-9, half + 1e-9)),
            "a tight cone straddle",
        ),
    ] {
        let (b1, _) = n.orthonormal_basis();
        println!(
            "{label:<26} b1 = ({}, {}, {})  poison={} certified={}",
            show(b1.x),
            show(b1.y),
            show(b1.z),
            b1.x.is_poison(),
            b1.x.is_certified()
        );
    }

    // The only inputs that pose no question: the ones that name no
    // direction. Poison there is the honest answer, not an absorption.
    println!("\n== D. inputs that pose no question ==");
    for (n, label) in [
        (Vec3::new(pt(0.0), pt(0.0), pt(0.0)), "the zero vector"),
        (
            Vec3::new(Interval::from_f64(f64::NAN), pt(0.0), pt(1.0)),
            "a NaI component",
        ),
    ] {
        let (b1, _) = n.orthonormal_basis();
        println!(
            "{label:<20} b1.x poison={} certified={}",
            b1.x.is_poison(),
            b1.x.is_certified()
        );
    }
}
