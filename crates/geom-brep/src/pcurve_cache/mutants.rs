//! Check 4 read wrong again, one site at a time: a test build takes the
//! old reading at the site `PCERT_RADIAL_PAIR_MUTANT` names at compile
//! time ([`SITES`]). Two kinds of site:
//! - **sums**: an exact norm read as the sum of its parts (the rows
//!   that refuse each are in `radial_pair_rows`); `fidelity-cone` reads
//!   the cone's two fidelity channels as orthogonal, and
//!   `pole-start-under` under-reads the pole start;
//! - **routing**: `envelope-fit` routes the envelope's refusal as a
//!   miss rather than a bound on one (its row is
//!   `a_refused_envelope_reads_as_a_bound_at_rest`).
//!
//! Each production site keeps a one-line hook that asks its own
//! function here, which answers nothing unless its site is named.
//! `option_env!` because `scripts/gates/no-ambient-env.sh` forbids a
//! runtime read in `crates/*/src`.

use geom::Surface;
use geom_core::{Real, Vec3};

use super::{angle_rate, harmonic_sup, harmonic_sup_scalar, norm_gap};
use crate::pcurve_cache::FidelityLegs;

const NAME: Option<&str> = option_env!("PCERT_RADIAL_PAIR_MUTANT");

/// Every site this crate's hooks name. topo's `chord_join` declares its
/// own two (`topo-gate`, `topo-v-var`) under the same variable.
pub(super) const SITES: [&str; 16] = [
    "class-circle",
    "zero-winding",
    "torus-parallel",
    "torus-parallel-constant",
    "torus-meridian",
    "torus-meridian-centre",
    "sphere-meridian",
    "pole-start",
    "pole-start-under",
    "fidelity",
    "fidelity-cone",
    "cone-drift",
    "radial-amplitude",
    "frame",
    "envelope-fit",
    "skip-incidence",
];

fn on(site: &str) -> bool {
    debug_assert!(SITES.contains(&site), "an unlisted mutant site: {site}");
    NAME == Some(site)
}

/// `class_circle`: the radius gap, the orientation defect and the plane's
/// tilt, three slots.
pub(super) fn class_circle<T: Real>(
    (rho, rho_sq): (T, T),
    a_r: Vec3<T>,
    b_r: Vec3<T>,
    turn: Vec3<T>,
    (aa, bb): (T, T),
) -> Option<T> {
    on("class-circle")
        .then(|| norm_gap(rho, rho_sq, a_r) + (b_r - turn).norm() + harmonic_sup_scalar(aa, bb))
}

/// A still class's drift: its radial and axial halves.
pub(super) fn zero_winding<T: Real>(a_r: Vec3<T>, b_r: Vec3<T>, (aa, bb): (T, T)) -> Option<T> {
    on("zero-winding").then(|| harmonic_sup(a_r, b_r) + harmonic_sup_scalar(aa, bb))
}

/// The torus parallel: centre, tube gap, orientation defect and tilt.
pub(super) fn torus_parallel<T: Real>(
    w_r: Vec3<T>,
    tube: T,
    b_r: Vec3<T>,
    turn: Vec3<T>,
    (aa, bb): (T, T),
) -> Option<T> {
    on("torus-parallel")
        .then(|| w_r.norm() + tube + (b_r - turn).norm() + harmonic_sup_scalar(aa, bb))
}

/// The torus parallel's constant and pair read by the triangle
/// inequality, `‖C‖ + σ(A, B)`.
pub(super) fn torus_parallel_constant<T: Real>(c: Vec3<T>, a: Vec3<T>, b: Vec3<T>) -> Option<T> {
    on("torus-parallel-constant").then(|| c.norm() + harmonic_sup(a, b))
}

/// The torus meridian: tube gap, tilt out of the meridian plane and
/// orientation defect.
pub(super) fn torus_meridian<T: Real>(
    (minor, minor_sq): (T, T),
    (n, n_sq): (T, T),
    tilt: Vec3<T>,
    turn_defect: Vec3<T>,
) -> Option<T> {
    on("torus-meridian")
        .then(|| (minor_sq - n_sq).abs() / (minor + n) + tilt.norm() + turn_defect.norm())
}

/// The torus meridian's centre: its radial and axial parts.
pub(super) fn torus_meridian_centre<T: Real>(spine: T, h: T) -> Option<T> {
    on("torus-meridian-centre").then(|| spine + h.abs())
}

/// The sphere meridian: the radius gap and the orientation defect, and
/// at a pole start the drift `3·‖a_r‖` and the orientation's axial and
/// radial parts.
pub(super) fn sphere_meridian<T: Real>(
    pole: bool,
    radius: T,
    (a, b): (Vec3<T>, Vec3<T>),
    (a_r, b_r): (Vec3<T>, Vec3<T>),
    (aa, bb): (T, T),
    sigma: i32,
    jb: Vec3<T>,
) -> Option<T> {
    let gap = || norm_gap(radius, radius.powi(2), a);
    if pole {
        on("pole-start").then(|| {
            gap()
                + a_r.norm() * T::from_f64(3.0)
                + bb.abs()
                + (aa * angle_rate::<T>(sigma) + b_r.norm()).abs()
        })
    } else {
        on("sphere-meridian").then(|| gap() + (jb - b).norm())
    }
}

/// An under-read, not a sum: the pole start's `cos t` difference read
/// as `a·k`, the non-pole meridian's, dropping `a′ − a`. The
/// off-circle sweep (`envelope_lemma_fuzz`) is what refuses it.
pub(super) fn pole_start_under<T: Real>(a: Vec3<T>, k: T) -> Option<Vec3<T>> {
    on("pole-start-under").then(|| a * k)
}

/// The two fidelity channels added.
pub(super) fn fidelity<T: Real>(slots: &[T]) -> Option<T> {
    on("fidelity").then(|| slots.iter().fold(T::zero(), |sum, &term| sum + term))
}

/// The cone's fidelity channels read as orthogonal.
pub(super) fn fidelity_legs<T: Real>(surface: &Surface<T>, legs: FidelityLegs) -> FidelityLegs {
    if matches!(surface, Surface::Cone { .. }) && on("fidelity-cone") {
        FidelityLegs::Orthogonal
    } else {
        legs
    }
}

/// The projected cone's slant drift: its radial and axial components.
pub(super) fn cone_drift<T: Real>(d_sin: T, d_cos: T) -> Option<T> {
    on("cone-drift").then(|| d_sin.abs() + d_cos.abs())
}

/// The cylinder class selector: the radial pair's coefficients added.
pub(super) fn radial_amplitude<T: Real>(a_r: Vec3<T>, b_r: Vec3<T>, l_r: Vec3<T>) -> Option<T> {
    on("radial-amplitude").then(|| a_r.norm() + b_r.norm() + l_r.norm())
}

/// The sphere's and the torus tube's frame defects: radial and axial
/// added.
pub(super) fn frame<T: Real>(d_radial: T, d_axis: T) -> Option<T> {
    on("frame").then(|| d_radial + d_axis)
}

/// The envelope's refusal routed as a miss, not a bound on one.
pub(super) fn envelope_fit() -> bool {
    on("envelope-fit")
}

/// The variable names a site this crate or topo's `chord_join` hooks.
#[test]
fn the_named_mutant_is_a_site() {
    assert!(
        NAME.is_none_or(|n| SITES.contains(&n) || matches!(n, "topo-gate" | "topo-v-var")),
        "PCERT_RADIAL_PAIR_MUTANT names no site: {NAME:?}"
    );
}

/// `incidence::incident`: the incidence test skipped, so a carrier off
/// the chart reaches the general lane and a class reading's "not yet"
/// (its rows are `off_chart_class_rows`).
pub(super) fn skip_incidence() -> bool {
    on("skip-incidence")
}
