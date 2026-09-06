//! DL6 consumer probe for `Vec3::orthonormal_basis`.
//!
//! Audits the constructor against DUAL-DESIGN DL6 — "a certified lane
//! may return Invalid/NaI only when the inputs pose no real question,
//! and must take a widening path over an absorbing one where both
//! exist" — rather than merely against "is it bounded on the equator".
//!
//! The construction crosses the normal with the world axis of its
//! smallest-magnitude component and normalizes. Two places could
//! absorb, and both are audited here: the axis choice at an enclosure
//! that cannot decide it (which must HULL, not refuse), and the
//! normalization of a candidate (which is well conditioned at the
//! chosen axis and degenerate at the largest one — never the chosen).
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

    // The equator is not a seam: |n.z| is the strict smallest magnitude
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
    let n = Vec3::new(pt(0.8), iv(0.42, 0.43), iv(0.42, 0.43));
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
    let d1 = n.z.abs() - n.y.abs();
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
    // wide enough to leave a tie undecided AND to contain a NON-UNIT
    // direction parallel to a candidate's axis hulls in that
    // candidate's zero vector and comes back unbounded. Restricted to
    // unit inputs — the constructor's precondition — no such point is
    // in the box and `|e_k x n|` is at least sqrt(2/3), so a tight
    // enclosure of a real normal never reaches this. The rows below are
    // the two wide boxes: both are uninformative about the direction,
    // and both say so rather than claiming a frame.
    println!("\n== C. wide direction boxes ==");
    for (n, label) in [
        (
            Vec3::new(iv(0.5, 0.6), iv(-1.0, 1.0), iv(-1.0, 1.0)),
            "wide in y and z",
        ),
        (
            Vec3::new(iv(-1.0, 1.0), iv(-1.0, 1.0), iv(-1.0, 1.0)),
            "[-1,1]^3 (contains 0)",
        ),
        (
            Vec3::new(iv(0.59, 0.61), iv(0.79, 0.81), iv(-0.01, 0.01)),
            "a tight wall enclosure",
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
