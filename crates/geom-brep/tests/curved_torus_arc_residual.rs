//! **The circle residual's torus arm** (`docs/CURVED-TORUS-SPEC.md`
//! §PR-2): `circle_arc_residual_range` and the certified `|(d²)″|`
//! bound behind it, against closed forms and against the lily's own
//! numbers.
//!
//! The torus is the one analytic kind whose composed residual is not
//! a trigonometric polynomial — it carries a `√` of one — so the
//! enclosure is a sampled hull plus a chord-dip charge rather than a
//! harmonic range, and what these rows check is that the charge is
//! neither unsound (row 4, and `implicit.rs`'s own
//! `the_curvature_bound_encloses_the_sampled_second_derivative`) nor
//! loose past its own formula (rows 1–2, against an independent
//! transcription of §PR-2's bound).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/implicit.rs",
    "crates/topo/src/boolean/reduce.rs",
];

use core::f64::consts::TAU;

use geom::Surface;
use geom_brep::{
    ARC_RESIDUAL_SAMPLES, circle_arc_residual_range, circle_residual_curvature_bound,
    implicit_residual,
};
use geom_core::{Point3, Vec3};
use test_utils::fuzz;

/// The step one sub-arc of a full-turn carrier spans.
fn full_turn_step() -> f64 {
    TAU / ARC_RESIDUAL_SAMPLES as f64
}

/// **§PR-2's bound, transcribed independently of the implementation.**
/// `|(d²)″| ≤ 2ρ_c² + 2·D_max·(ρ_c + 2ρ_c²/ρ_min) + 2a_h² + 2·H_max·a_h`,
/// divided by `2r`. The caller supplies the arc's true `ρ` and `|h|`
/// ranges; this adds the Lipschitz charge the door adds and nothing
/// else, so a term dropped or sign-flipped in `arc_curvature_bound`
/// shows up as a charge mismatch rather than as silence.
fn f2_oracle(
    big_r: f64,
    minor: f64,
    rho_c: f64,
    span: f64,
    rho_range: (f64, f64),
    h_abs_max: f64,
    a_h: f64,
) -> f64 {
    let lip = rho_c * (span / ARC_RESIDUAL_SAMPLES as f64).abs() / 2.0;
    let rho_min = (rho_range.0 - lip).max(0.0);
    let rho_max = rho_range.1 + lip;
    let h_max = h_abs_max + lip;
    let d_max = (rho_min - big_r).abs().max((rho_max - big_r).abs());
    let d2 = 2.0 * rho_c.powi(2)
        + 2.0 * d_max * (rho_c + 2.0 * rho_c.powi(2) / rho_min)
        + 2.0 * a_h.powi(2)
        + 2.0 * h_max * a_h;
    d2 / (2.0 * minor)
}

fn torus(center: Point3<f64>, axis: Vec3<f64>, big_r: f64, minor: f64) -> Surface<f64> {
    Surface::Torus {
        center,
        axis,
        major_radius: big_r,
        minor_radius: minor,
        u_ref: Vec3::unit_x(),
    }
}

fn circle_point(
    center: Point3<f64>,
    axis: Vec3<f64>,
    radius: f64,
    u_ref: Vec3<f64>,
    t: f64,
) -> Point3<f64> {
    let v = axis.cross(u_ref);
    center + u_ref * (radius * t.cos()) + v * (radius * t.sin())
}

/// The true residual range over an arc, by dense sampling — the
/// oracle every enclosure row compares against.
fn dense_range(
    s: &Surface<f64>,
    center: Point3<f64>,
    axis: Vec3<f64>,
    radius: f64,
    u_ref: Vec3<f64>,
    (t0, t1): (f64, f64),
) -> (f64, f64) {
    let n = 60_000;
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    for k in 0..=n {
        let t = t0 + (t1 - t0) * f64::from(k) / f64::from(n);
        let r = implicit_residual(s, circle_point(center, axis, radius, u_ref, t));
        lo = lo.min(r);
        hi = hi.max(r);
    }
    (lo, hi)
}

/// **Row 1 — a coaxial circle, against the closed form.** A circle
/// coaxial with the torus at radius `ρ_c` and height `h` has the
/// CONSTANT residual `((ρ_c − R)² + h² − r²)/2r`, so the enclosure's
/// two ends are that constant ∓ the charge and nothing else. Both
/// halves are pinned: the constant (which the `h²` term's sign
/// decides) and the charge (which every term of §PR-2's bound does).
#[test]
fn a_coaxial_circle_reads_its_closed_form_residual_within_the_charge() {
    let (big_r, minor) = (1.0, 0.2);
    let s = torus(Point3::origin(), Vec3::unit_z(), big_r, minor);
    let (rho_c, h) = (1.5, 0.3);
    let center = Point3::new(0.0, 0.0, h);
    let closed = ((rho_c - big_r).powi(2) + h.powi(2) - minor.powi(2)) / (2.0 * minor);
    assert!((closed - 0.75).abs() < 1e-12, "fixture: {closed}");
    let (lo, hi) =
        circle_arc_residual_range(&s, center, Vec3::unit_z(), rho_c, Vec3::unit_x(), 0.0, TAU)
            .expect("the torus arm answers");
    assert!(lo <= closed && closed <= hi, "[{lo}, {hi}] misses {closed}");
    // `a_h = 0`: the circle's plane is perpendicular to the axis, so
    // the axial channel contributes nothing and the whole width is
    // the radial channel's charge.
    let charge = f2_oracle(big_r, minor, rho_c, TAU, (rho_c, rho_c), h, 0.0)
        * full_turn_step().powi(2)
        * 0.125;
    let width = (hi - lo) / 2.0;
    assert!(
        (width - charge).abs() <= 1e-9 * charge,
        "the charge is §PR-2's bound to the last term: got {width}, the \
         transcription says {charge}"
    );
}

/// **Row 2 — a circle in the ring plane, offset from the axis.** Its
/// radial distance sweeps `[|w₀| − ρ_c, |w₀| + ρ_c]` and both ends
/// are attained AT samples, so the enclosure is the two closed-form
/// residuals widened by exactly the charge — pinned against the same
/// independent transcription, which is what makes a dropped `ρ″` term
/// or a dropped `D_max` visible here rather than only in the
/// soundness row.
#[test]
fn a_ring_plane_circle_reaches_both_closed_form_extremes() {
    let (big_r, minor) = (1.0, 0.2);
    let s = torus(Point3::origin(), Vec3::unit_z(), big_r, minor);
    let (offset, rho_c) = (3.0, 0.5);
    let center = Point3::new(offset, 0.0, 0.0);
    let residual_at_rho = |rho: f64| ((rho - big_r).powi(2) - minor.powi(2)) / (2.0 * minor);
    let (near, far) = (
        residual_at_rho(offset - rho_c),
        residual_at_rho(offset + rho_c),
    );
    let (lo, hi) =
        circle_arc_residual_range(&s, center, Vec3::unit_z(), rho_c, Vec3::unit_x(), 0.0, TAU)
            .expect("the torus arm answers");
    let charge = f2_oracle(
        big_r,
        minor,
        rho_c,
        TAU,
        (offset - rho_c, offset + rho_c),
        0.0,
        0.0,
    ) * full_turn_step().powi(2)
        * 0.125;
    assert!(
        (lo - (near - charge)).abs() <= 1e-9 * charge.max(1.0),
        "lo {lo} is not the near extreme {near} less the charge {charge}"
    );
    assert!(
        (hi - (far + charge)).abs() <= 1e-9 * charge.max(1.0),
        "hi {hi} is not the far extreme {far} plus the charge {charge}"
    );
}

/// **Row 2b — a circle whose plane holds the axis direction, so the
/// AXIAL channel of the bound is the one on trial.** Rows 1 and 2 both
/// have `a_h = 0` and cannot see the `2a_h² + 2·H_max·a_h` terms at
/// all. Here the circle stands in a plane parallel to the axis, 2.5 m
/// out, so `a_h = ρ_c` is maximal while the radial channel is mild —
/// and both its `ρ` extremes and its `|h|` maximum are attained AT
/// samples, which makes the charge comparison exact.
#[test]
fn a_circle_parallel_to_the_axis_pins_the_bounds_axial_channel() {
    let (big_r, minor) = (1.0, 0.2);
    let s = torus(Point3::origin(), Vec3::unit_z(), big_r, minor);
    let (offset, rho_c) = (2.5, 0.5);
    let center = Point3::new(offset, 0.0, 0.0);
    // Frame: the circle's own axis is radial, so its plane is spanned
    // by the torus axis and the tangential direction.
    let (axis, u_ref) = (Vec3::unit_x(), Vec3::unit_z());
    let (lo, hi) =
        circle_arc_residual_range(&s, center, axis, rho_c, u_ref, 0.0, TAU).expect("the torus arm");
    let rho_far = (offset.powi(2) + rho_c.powi(2)).sqrt();
    let charge = f2_oracle(big_r, minor, rho_c, TAU, (offset, rho_far), rho_c, rho_c)
        * full_turn_step().powi(2)
        * 0.125;
    let residual_at_rho =
        |rho: f64, h: f64| ((rho - big_r).powi(2) + h.powi(2) - minor.powi(2)) / (2.0 * minor);
    // The residual's extremes over this circle: `ρ² + h²` is CONSTANT
    // on it (the circle's own centre stands on the axis-through-centre
    // plane), so `d² = (ρ−R)² + h² = ρ² + h² + R² − 2Rρ` falls as `ρ`
    // rises and both ends of the `ρ` range are samples. The maximum
    // is therefore the CLOSEST-to-the-axis point and the minimum the
    // farthest.
    let top = residual_at_rho(offset, rho_c);
    let bottom = residual_at_rho(rho_far, 0.0);
    assert!(
        bottom < top,
        "the fixture's residual falls as rho rises: {bottom}, {top}"
    );
    assert!(
        (lo - (bottom - charge)).abs() <= 1e-9 * charge.max(1.0),
        "lo {lo} is not the far-point extreme {bottom} less the charge {charge}"
    );
    assert!(
        (hi - (top + charge)).abs() <= 1e-9 * charge.max(1.0),
        "hi {hi} is not the near-point extreme {top} plus the charge {charge}"
    );
}

/// **Row 3 — a meridian of the torus itself, and a concentric tube.**
/// A meridian circle lies ON the locus, so the enclosure must
/// straddle zero; a circle concentric with it at a different tube
/// radius has a constant one-sided residual and the enclosure must
/// stay on that side once the charge is paid.
#[test]
fn a_meridian_straddles_zero_and_a_concentric_tube_stays_one_sided() {
    let (big_r, minor) = (1.0, 0.2);
    let s = torus(Point3::origin(), Vec3::unit_z(), big_r, minor);
    // The meridian at azimuth 0: centre on the spine, in the plane
    // spanned by the radial direction and the axis.
    let spine = Point3::new(big_r, 0.0, 0.0);
    let meridian_axis = Vec3::unit_y();
    for (tube, name) in [(minor, "on the locus"), (0.05, "inside"), (0.4, "outside")] {
        let (lo, hi) =
            circle_arc_residual_range(&s, spine, meridian_axis, tube, Vec3::unit_x(), 0.0, TAU)
                .expect("the torus arm answers");
        let closed = (tube.powi(2) - minor.powi(2)) / (2.0 * minor);
        assert!(
            lo <= closed && closed <= hi,
            "{name}: [{lo}, {hi}] misses {closed}"
        );
        if name == "on the locus" {
            assert!(
                lo <= 0.0 && 0.0 <= hi,
                "a meridian must straddle: [{lo}, {hi}]"
            );
        } else if name == "inside" {
            assert!(hi < 0.0, "an inside tube must stay inside: [{lo}, {hi}]");
        } else {
            assert!(lo > 0.0, "an outside tube must stay outside: [{lo}, {hi}]");
        }
    }
}

/// **Row 4 — a tangency placed BETWEEN two samples.** The circle
/// touches the tube at exactly one parameter, half a cell off the
/// nearest sample: the bare sample hull reports a clear positive
/// minimum and would certify a miss over a genuine contact. The
/// charge is what stops it, and this row is the one that fails if the
/// charge is ever dropped.
#[test]
fn a_tangency_between_samples_is_not_certified_clear() {
    let (big_r, minor) = (1.0, 0.2);
    let s = torus(Point3::origin(), Vec3::unit_z(), big_r, minor);
    let (offset, rho_c) = (0.5, 0.3);
    assert!(
        (offset + rho_c - (big_r - minor)).abs() < 1e-15,
        "the fixture must touch the tube's inner equator"
    );
    // The touch is where the circle point is farthest from the axis,
    // which the frame below places at HALF a sub-arc past t = 0.
    let touch = full_turn_step() / 2.0;
    let u_ref = Vec3::new(touch.cos(), -touch.sin(), 0.0);
    let center = Point3::new(offset, 0.0, 0.0);
    let (mut bare_lo, mut bare_hi) = (f64::INFINITY, f64::NEG_INFINITY);
    for k in 0..=ARC_RESIDUAL_SAMPLES {
        let t = full_turn_step() * k as f64;
        let r = implicit_residual(&s, circle_point(center, Vec3::unit_z(), rho_c, u_ref, t));
        bare_lo = bare_lo.min(r);
        bare_hi = bare_hi.max(r);
    }
    assert!(
        bare_lo > 0.0,
        "the uncharged hull must claim a clear miss for this row to mean \
         anything: {bare_lo}"
    );
    let contact = implicit_residual(
        &s,
        circle_point(center, Vec3::unit_z(), rho_c, u_ref, touch),
    );
    assert!(contact.abs() < 1e-12, "the fixture must touch: {contact}");
    let (lo, hi) = circle_arc_residual_range(&s, center, Vec3::unit_z(), rho_c, u_ref, 0.0, TAU)
        .expect("the torus arm answers");
    assert!(
        lo <= 0.0 && 0.0 <= hi,
        "the charged enclosure must contain the contact it cannot see: [{lo}, {hi}]"
    );
}

/// The lily's three weld-plane pairs, with the numbers
/// `docs/CURVED-TORUS-SPEC.md` §PR-2 measured them at. `deg(22)` on a
/// 5 m ring is the stem's spine; the arch's ring centre is the walked
/// value `demos/tour`'s `stem_joints_are_g1_in_the_stored_geometry`
/// pins.
const D22: f64 = 22.0 * core::f64::consts::PI / 180.0;
const STEM_R: f64 = 0.060;
const ARCH_R: f64 = 0.052;

fn stem_torus() -> Surface<f64> {
    torus(
        Point3::new(-5.0, 0.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        5.0,
        STEM_R,
    )
}

fn arch_torus() -> Surface<f64> {
    torus(
        Point3::new(-1.383_982_967_189_529_2, 0.0, 1.460_965_714_322_057),
        Vec3::new(0.0, -1.0, 0.0),
        1.1,
        ARCH_R,
    )
}

/// The weld point — the stem's spine end, the arch's spine start.
fn weld() -> Point3<f64> {
    Point3::new(-5.0 + 5.0 * D22.cos(), 0.0, 5.0 * D22.sin())
}

/// The spine tangent there.
fn weld_tangent() -> Vec3<f64> {
    Vec3::new(-D22.sin(), 0.0, D22.cos())
}

/// **Row 6 — the lily's three pairs resolve at the ratified count, at
/// every band.** These are the measurements the constant was set by:
/// the stem's outer-equator seam clears the arch's carrier by 8.6 mm
/// over a 22° arc, and the two weld meridians sit 8.6 mm outside and
/// 7.5 mm inside their opposite numbers. Each is a definite
/// one-sidedness, and the row asserts the margin against every band
/// the run matrix uses.
#[test]
fn the_lily_weld_pairs_resolve_at_the_ratified_sample_count() {
    // The run matrix's three eps cells (`DEFAULT_EPS`, 1e-6, 1e-12).
    let bands = [1e-9, 1e-6, 1e-12];
    // (a) the stem's outer-equator seam, a 22° arc, against the arch.
    let seam = circle_arc_residual_range(
        &arch_torus(),
        Point3::new(-5.0, 0.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        5.0 + STEM_R,
        Vec3::unit_x(),
        0.0,
        D22,
    )
    .expect("the torus arm answers");
    // (b) the stem's end meridian against the arch: every point is
    // STEM_R from a spine point of the arch, so the residual is the
    // constant (STEM_R² − ARCH_R²)/2·ARCH_R.
    let end_circle = circle_arc_residual_range(
        &arch_torus(),
        weld(),
        weld_tangent(),
        STEM_R,
        Vec3::new(D22.cos(), 0.0, D22.sin()),
        0.0,
        TAU,
    )
    .expect("the torus arm answers");
    // (c) the arch's start meridian against the stem, one-sided INSIDE.
    let start_circle = circle_arc_residual_range(
        &stem_torus(),
        weld(),
        weld_tangent(),
        ARCH_R,
        Vec3::new(D22.cos(), 0.0, D22.sin()),
        0.0,
        TAU,
    )
    .expect("the torus arm answers");
    for band in bands {
        assert!(
            seam.0 > band,
            "the seam must clear OUTSIDE at band {band}: {seam:?}"
        );
        assert!(
            end_circle.0 > band,
            "the stem's end meridian must clear OUTSIDE at band {band}: {end_circle:?}"
        );
        assert!(
            -start_circle.1 > band,
            "the arch's start meridian must clear INSIDE at band {band}: {start_circle:?}"
        );
    }
    // The numbers themselves, so a drift is legible rather than just
    // still-positive: the true clearances are 8.615 mm, 8.615 mm and
    // −7.467 mm, and the enclosures sit within a millimetre of them.
    let (true_lo, _) = dense_range(
        &arch_torus(),
        Point3::new(-5.0, 0.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        5.0 + STEM_R,
        Vec3::unit_x(),
        (0.0, D22),
    );
    assert!(
        (true_lo - 0.008_615).abs() < 1e-5 && true_lo - seam.0 < 1e-3,
        "seam: true {true_lo}, enclosed {seam:?}"
    );
    let closed_out = (STEM_R.powi(2) - ARCH_R.powi(2)) / (2.0 * ARCH_R);
    let closed_in = (ARCH_R.powi(2) - STEM_R.powi(2)) / (2.0 * STEM_R);
    assert!(
        (closed_out - 0.008_615_384_6).abs() < 1e-9 && (closed_in + 0.007_466_666_7).abs() < 1e-9,
        "the weld's two closed forms: {closed_out}, {closed_in}"
    );
    assert!(
        end_circle.0 <= closed_out && closed_out <= end_circle.1,
        "the end meridian's enclosure must hold its closed form"
    );
    assert!(
        start_circle.0 <= closed_in && closed_in <= start_circle.1,
        "the start meridian's enclosure must hold its closed form"
    );
}

/// **The arc-scoped `f2` is what makes the seam resolve, and by how
/// much.** The whole-carrier bound is the one §PR-2 quotes; scoping
/// the `ρ` range to the 22° arc divides the dominant `D_max` term by
/// the ratio of the two spans. The row states both numbers, because
/// the resolution law at [`ARC_RESIDUAL_SAMPLES`] is read against
/// them.
#[test]
fn the_arc_scoped_curvature_bound_beats_the_whole_carrier_one() {
    let s = arch_torus();
    let (c, axis, radius, u) = (
        Point3::new(-5.0, 0.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        5.0 + STEM_R,
        Vec3::unit_x(),
    );
    let whole = circle_residual_curvature_bound(&s, c, axis, radius, u).expect("torus arm");
    // The arc's own bound, read off the door's charge.
    let (lo, hi) = circle_arc_residual_range(&s, c, axis, radius, u, 0.0, D22).expect("torus arm");
    let (dense_lo, dense_hi) = dense_range(&s, c, axis, radius, u, (0.0, D22));
    let arc_charge = ((hi - dense_hi) + (dense_lo - lo)) / 2.0;
    let arc_f2 = arc_charge / ((D22 / ARC_RESIDUAL_SAMPLES as f64).powi(2) * 0.125);
    assert!(
        (7_000.0..9_000.0).contains(&whole),
        "the whole-carrier bound is the spec's 7.9e3: {whole}"
    );
    assert!(
        (1_000.0..2_000.0).contains(&arc_f2),
        "the arc-scoped bound is ~1.4e3: {arc_f2}"
    );
    assert!(
        arc_charge < 1e-3 && arc_charge > 1e-4,
        "the seam's charge is ~0.39 mm, two orders under its 8.6 mm clearance: {arc_charge}"
    );
}

/// **Row 8 — the K-sample door against the two-endpoint one it
/// replaces, on random arcs.** The theorem is
/// `margin_K ≥ margin_1 − f2·(Δθ/K)²/8`, NOT `margin_K ≥ margin_1`:
/// the sample hull's minimum can sit below the endpoint minimum, so a
/// finer `h` does not dominate on its own. See
/// `the_k_sample_door_is_not_monotone_and_this_is_the_bound` for the
/// exact family where the old bound wins, and by how little.
#[test]
fn the_k_sample_margin_never_falls_below_the_two_endpoint_one_by_more_than_a_cell() {
    let mut rng = fuzz::start("curved_torus_arc_residual::k_sample_vs_two_endpoint");
    let cases = fuzz::scaled(240);
    let mut worst_deficit: f64 = 0.0;
    let mut improved = 0usize;
    for case in 0..cases {
        let axis = {
            let v = Vec3::new(
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
            );
            if v.norm() < 1e-3 {
                Vec3::unit_z()
            } else {
                v.normalize()
            }
        };
        let u_ref = {
            let mut u = Vec3::new(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), 1.0);
            u = (u - axis * axis.dot(u)).normalize();
            u
        };
        let center = Point3::new(
            rng.range(-2.0, 2.0),
            rng.range(-2.0, 2.0),
            rng.range(-2.0, 2.0),
        );
        let radius = rng.range(0.05, 3.0);
        let s: Surface<f64> = match case % 3 {
            0 => Surface::Plane {
                origin: Point3::new(rng.range(-1.0, 1.0), 0.0, rng.range(-1.0, 1.0)),
                normal: axis.cross(u_ref).normalize(),
                u_ref: Vec3::unit_x(),
            },
            1 => Surface::Sphere {
                center: Point3::new(
                    rng.range(-3.0, 3.0),
                    rng.range(-3.0, 3.0),
                    rng.range(-3.0, 3.0),
                ),
                radius: rng.range(0.05, 2.5),
                axis: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
            _ => Surface::Cylinder {
                origin: Point3::new(
                    rng.range(-3.0, 3.0),
                    rng.range(-3.0, 3.0),
                    rng.range(-3.0, 3.0),
                ),
                axis: u_ref,
                radius: rng.range(0.05, 2.5),
                u_ref: Vec3::unit_x(),
            },
        };
        let t0 = rng.range(0.0, TAU);
        let t1 = t0 + rng.range(1e-3, TAU);
        let f2 = circle_residual_curvature_bound(&s, center, axis, radius, u_ref)
            .expect("a harmonic kind");
        let old = {
            let dip = f2 * (t1 - t0).powi(2) * 0.125;
            let a = implicit_residual(&s, circle_point(center, axis, radius, u_ref, t0));
            let b = implicit_residual(&s, circle_point(center, axis, radius, u_ref, t1));
            (a.min(b) - dip).max(-(a.max(b) + dip))
        };
        let (lo, hi) = circle_arc_residual_range(&s, center, axis, radius, u_ref, t0, t1)
            .expect("a harmonic kind");
        let new = lo.max(-hi);
        let cell_charge = f2 * ((t1 - t0) / ARC_RESIDUAL_SAMPLES as f64).powi(2) * 0.125;
        let deficit = old - new;
        assert!(
            deficit <= cell_charge + 1e-12 * (1.0 + old.abs()),
            "case {case}: the K-sample margin {new} falls {deficit} below the \
             two-endpoint {old}, past one cell's charge {cell_charge} — {}",
            fuzz::replay()
        );
        worst_deficit = worst_deficit.max(deficit);
        if new > old {
            improved += 1;
        }
    }
    // The accepting direction: the door is not merely no-worse, it is
    // the tighter one nearly everywhere. A build where it is not has
    // lost the subdivision.
    assert!(
        improved * 10 >= cases * 9,
        "only {improved}/{cases} arcs got a TIGHTER margin from subdivision \
         (worst deficit {worst_deficit}) — {}",
        fuzz::replay()
    );
}

/// **The K-sample door is NOT monotone over the two-endpoint one, and
/// this is the family where it loses.** `docs/CURVED-TORUS-SPEC.md`
/// §PR-2 argues monotonicity from "the same `f2`, a finer `h`"; that
/// step assumes the sample hull's minimum is at an endpoint, which it
/// is not. On a short arc CENTRED on the residual's minimum the
/// two-endpoint chord-dip bound is exactly tight, so the K-sample
/// door's own charge is pure loss.
///
/// The loss is bounded by one cell's charge — `f2·Δθ²/8` divided by
/// `K²` — which is why no existing clearance pin moves: at `K = 256`
/// that is the old charge over 65536.
#[test]
fn the_k_sample_door_is_not_monotone_and_this_is_the_bound() {
    // Residual `2 + cos θ` along the unit circle: a plane whose
    // harmonic amplitude is 1, so `f2 = 1` exactly and the chord-dip
    // bound is attained. The arc stands OUTSIDE the plane, so the
    // margin is the `lo` branch and the minimum is what decides it.
    let s = Surface::Plane {
        origin: Point3::new(0.0, 0.0, -2.0),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let (center, axis, radius, u_ref) = (Point3::origin(), Vec3::unit_x(), 1.0, Vec3::unit_z());
    let span = 0.02;
    let (t0, t1) = (
        core::f64::consts::PI - span / 2.0,
        core::f64::consts::PI + span / 2.0,
    );
    let f2 = circle_residual_curvature_bound(&s, center, axis, radius, u_ref).expect("plane arm");
    assert!(
        (f2 - 1.0).abs() < 1e-12,
        "the fixture's f2 must be exactly 1: {f2}"
    );
    let ends = |t: f64| implicit_residual(&s, circle_point(center, axis, radius, u_ref, t));
    let old = {
        let dip = f2 * span.powi(2) * 0.125;
        let (a, b) = (ends(t0), ends(t1));
        (a.min(b) - dip).max(-(a.max(b) + dip))
    };
    let (lo, hi) =
        circle_arc_residual_range(&s, center, axis, radius, u_ref, t0, t1).expect("plane arm");
    let new = lo.max(-hi);
    let cell_charge = f2 * (span / ARC_RESIDUAL_SAMPLES as f64).powi(2) * 0.125;
    assert!(
        new < old,
        "this fixture exists to show the K-sample door LOSING: {new} vs {old}"
    );
    assert!(
        old - new <= cell_charge,
        "and losing by at most one cell's charge: {} vs {cell_charge}",
        old - new
    );
    assert!(
        cell_charge < 1e-9,
        "which at K = {ARC_RESIDUAL_SAMPLES} is below any band: {cell_charge}"
    );
}

/// **What a COINCIDENT torus pair now reads, and why it is still a
/// refusal.** MATE-7a's boundary row
/// (`sweep/tests/mate7a_torus_rest.rs`'s
/// `the_admitted_torus_lane_stops_at_the_curved_pierce_frontier`) puts
/// two identical tori through the declared-Rest lane. Its edges are
/// seam meridians of the torus they ride, so the residual is
/// identically zero along them — and the sampled enclosure is
/// therefore `±charge`, whose one-sidedness margin is `−charge`.
///
/// `charge` is 1.8e-5 m on that fixture, which is larger than every
/// band the run matrix uses, so the margin decides **Negative** and
/// the circle rung takes the typed frontier at its
/// `Zero | Negative` arm. The declared-cover rung behind it needs a
/// `Zero`, and a sampled enclosure of a coincident pair cannot
/// produce one at any `K`: the charge shrinks as `K⁻²` but the band
/// does not move with it. That is
/// `work/curved/torus-coincident-pair-cannot-reach-the-covered-rung.md`.
#[test]
fn a_coincident_torus_pair_encloses_pm_charge_and_reads_negative() {
    let (big_r, minor) = (5.0, 0.06);
    let s = torus(Point3::origin(), Vec3::unit_z(), big_r, minor);
    // The u = 0 seam meridian: centre on the spine, in the plane the
    // radial direction and the axis span.
    let (lo, hi) = circle_arc_residual_range(
        &s,
        Point3::new(big_r, 0.0, 0.0),
        Vec3::unit_y(),
        minor,
        Vec3::unit_x(),
        0.0,
        TAU,
    )
    .expect("the torus arm answers");
    assert!(
        lo < 0.0 && hi > 0.0 && (lo + hi).abs() < 1e-12 * hi,
        "a coincident pair encloses symmetrically about zero: [{lo}, {hi}]"
    );
    let margin = lo.max(-hi);
    assert!(
        (-margin - 1.8e-5).abs() < 2e-6,
        "the charge on this fixture is 1.8e-5 m: {}",
        -margin
    );
    // Every eps cell in the matrix is orders under the charge, so the
    // verdict is Negative at all three and the covered rung — which
    // needs a Zero — is out of reach on this fixture.
    for band in [1e-9, 1e-6, 1e-12] {
        assert!(
            margin < -band,
            "at band {band} the margin {margin} must be a definite NEGATIVE, not a Zero"
        );
    }
}
