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
pub(super) const SITES: [&str; 36] = [
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
    "radial-fold",
    "segment-form-ends",
    "circle-incidence",
    "form-uncharged",
    "lane-envelope-bound",
    "cone-terms-unprojected",
    "semi-axis-unchecked",
    "charge-circular",
    "ellipse-term-dropped",
    "ruling-per-parameter",
    "cone-ruling-gate",
    "ruling-unplaced",
    "meridian-standoff-unread",
    "spline-floor-net",
    "spline-charge-dropped",
    "spline-cut-uncharged",
    "spline-bound-undercut",
    "spline-on-cut-short",
    "spline-product-uncharged",
    "sampled-line-incidence",
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
pub(super) fn radial_amplitude<T: Real>(a_r: Vec3<T>, b_r: Vec3<T>) -> Option<T> {
    on("radial-amplitude").then(|| a_r.norm() + b_r.norm())
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

/// A spline chart's floor read as the nearest control point's distance,
/// with no hull: a sample between control points reads off a patch it
/// lies on (its rows are `spline_incidence_rows`).
pub(super) fn spline_floor_net() -> bool {
    on("spline-floor-net")
}

/// A spline chart's floor read at `f64` with no rounding charge at all:
/// a sample a band past a far or large patch reads off a patch it lies
/// within the band of (its rows are `spline_incidence_rows`).
pub(super) fn spline_charge_dropped() -> bool {
    on("spline-charge-dropped")
}

/// A spline chart's floor charged the read's rounding but not the cut's
/// (its rows are `spline_incidence_rows`).
pub(super) fn spline_cut_uncharged() -> bool {
    on("spline-cut-uncharged")
}

/// A spline chart cut's insertion blends charged the difference's
/// rounding but not the product's by the ratio (its rows are
/// `spline_incidence_rows`'s knotted cut-drift row).
pub(super) fn spline_product_uncharged() -> bool {
    on("spline-product-uncharged")
}

/// A spline chart cut's point rounding bound cut to a quarter (its rows
/// are `spline_incidence_rows`'s cut-drift rows).
pub(super) fn spline_bound_undercut() -> bool {
    on("spline-bound-undercut")
}

/// A spline chart's walk stopped four cells past its first point read in
/// band, before it reaches a point within the zero band (its row is
/// `spline_incidence_rows`'s on-chart row).
pub(super) fn spline_on_cut_short() -> bool {
    on("spline-on-cut-short")
}

/// The cylinder arm's ruling reading folded again: an in-band reading
/// takes the meridian form rather than the general lane (its rows are
/// `cylinder_line_rows`).
pub(super) fn radial_fold() -> bool {
    on("radial-fold")
}

/// A line's ruling reading per unit of its parameter, the span unread
/// (its rows are `ruling_reading_rows`).
pub(super) fn ruling_per_parameter() -> bool {
    on("ruling-per-parameter")
}

/// A line's ruling reading metered over the span's length from its
/// origin, where the span lies unread: on a cylinder from the ruling
/// through the span's start, on a cone the origin's distance plus the
/// rate over the length (its rows are `ruling_reading_rows`).
pub(super) fn ruling_unplaced() -> bool {
    on("ruling-unplaced")
}

/// The cylinder arm's line class read on its lean alone, the origin's
/// radial standoff unread (its row is `ruling_reading_rows`).
pub(super) fn meridian_standoff_unread() -> bool {
    on("meridian-standoff-unread")
}

/// A line's incidence read at the sampled schedule again, its critical
/// points unread (its rows are `ruling_reading_rows`).
pub(super) fn sampled_line_incidence() -> bool {
    on("sampled-line-incidence")
}

/// The cone arm images a line as the ruling at its anchor's azimuth
/// with no ruling reading (its rows are `ruling_reading_rows`).
pub(super) fn cone_ruling_gate() -> bool {
    on("cone-ruling-gate")
}

/// A line's chart form read at the segment's two ends only, dropping
/// the middle Bernstein coefficient: the sup misses a chord that dips
/// toward the axis (its rows are `cylinder_line_rows` and
/// `cone_line_rows`).
pub(super) fn segment_form_ends<T: Real>(f0: T, f1: T, _middle: T) -> Option<T> {
    on("segment-form-ends").then(|| f0.abs().max(f1.abs()))
}

/// A circle's distance from a cylinder, cone or torus dropped from its
/// projected envelope: the envelope misses a circle off the chart (its
/// rows are `circle_lane_rows`).
pub(super) fn circle_incidence<T: Real>(surface: &Surface<T>) -> Option<T> {
    (on("circle-incidence") && !matches!(surface, Surface::Sphere { .. })).then(T::zero)
}

/// A circle's chart form read at its computed harmonics alone, its
/// coefficients' rounding uncharged: the bound misses a thin torus's
/// cancellation at `f64` (its rows are `circle_lane_rows`).
pub(super) fn form_uncharged<T: Real>(harmonics: T) -> Option<T> {
    on("form-uncharged").then_some(harmonics)
}

/// A lane choice's envelope refusal left as the envelope's own (a bound
/// on a miss, a tolerance offer) rather than the lane's gap (its rows
/// are `circle_lane_rows`).
pub(crate) fn lane_envelope_bound() -> bool {
    on("lane-envelope-bound")
}

/// A cone section's form coefficients charged at their parts' reaches
/// alone, the projection's `|d|`-scale rounding uncharged: the charge
/// misses a needle cone's section far from its apex (its rows are
/// `circle_lane_rows`).
pub(crate) fn cone_terms_unprojected() -> bool {
    on("cone-terms-unprojected")
}

/// The edge certificate's semi-axis check skipped: a conic stored with
/// a negative semi-axis reaches the span meter, which ends
/// it as a span that does not run forward (its row is
/// `ellipse_lane_rows::a_conic_stored_with_a_negative_semi_axis_refuses_typed`).
pub(crate) fn semi_axis_unchecked() -> bool {
    on("semi-axis-unchecked")
}

/// A conic's charge read as a circle's, `|a| = |b|`: the reach and the
/// cone's levers read the first semi-axis for both, so an ellipse whose
/// second is the larger is under-charged (its rows are
/// `ellipse_lane_rows`).
pub(crate) fn charge_circular() -> bool {
    on("charge-circular")
}

/// The second harmonic's cosine coefficient read as a circle's, the
/// ellipse's `(a² − b²)` part dropped: the form's bound misses an
/// eccentric ellipse's (its rows are `ellipse_lane_rows`).
pub(crate) fn ellipse_term_dropped() -> bool {
    on("ellipse-term-dropped")
}
