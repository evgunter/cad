//! The azimuthal frame: the one reading of `u_ref`, `axis` and an
//! angle that every axisymmetric evaluator in the crate starts from —
//! and, the other way, the one read of an angle off the geometry
//! ([`Azimuth`], read in a [`Window`]), which a point's chart
//! coordinates (`crate::chart_point`) and every placed pcurve image in
//! `geom_brep` (C4) are named through.
//!
//! `DESIGN.md`'s parameterization row ratifies **one** convention here
//! — `v_ref = axis × u_ref`, seam at `u_ref` — and it is shared by the
//! circle and ellipse arms of [`crate::curves::Curve3`] and by every
//! axisymmetric arm of [`crate::surfaces::Surface`]. One convention
//! gets one body: a second derivation of `v_ref` is how the two halves
//! drift.
//!
//! Two doors, because the arms need two different amounts of it. An
//! arm whose combination *is* the radial or the tangential takes
//! [`frame`]; an arm that combines `sin`, `cos` and `v_ref` in its own
//! documented order (the ellipse's per-axis scaling, the circle's
//! second derivative) takes [`basis`] and writes its own combination.
//! Neither door performs a combination on the caller's behalf that the
//! caller's own doc comment does not spell out — the association
//! orders are D9-fixed per arm and stay where they are read.
//!
//! [`frame`] answers an [`AzimuthFrame`] of [`Radial`] and
//! [`Tangential`], and the reason is the arms: they ask for the radial
//! alone, for the tangential alone, and for both, so a positional pair
//! of two same-typed vectors has to be destructured three ways and a
//! transposed one compiles, runs, and moves geometry.
//!
//! **Names alone would not have fixed that** — while the two fields
//! share a type, `AzimuthFrame { radial: t, tangential: r }` compiles.
//! The newtypes make the exchange E0308 **wherever the two cross a
//! typed boundary**: the constructor here, and any site that passes one
//! where the other is expected. What no newtype can stop, and what is
//! therefore stated rather than claimed away, is a local binding
//! deliberately named for the other value and unwrapped on the spot
//! (`let AzimuthFrame { radial: tangential, .. }`); that residue is one
//! visible line at the site, and the downstream suites named on the row
//! below are its evidence. `.0` unwraps the vector where an arm's
//! arithmetic needs it.

use geom_core::k_stats::decide;
use geom_core::predicate::Band;
use geom_core::{Decide, Margin, Real, Sign, Vec3};

/// `((sin u, cos u), v_ref)` — the trig pair from **one** `sin_cos`
/// call and the third frame vector `v_ref = axis × u_ref`.
pub(crate) fn basis<T: Real>(axis: Vec3<T>, u_ref: Vec3<T>, u: T) -> ((T, T), Vec3<T>) {
    ((u.sin_cos()), axis.cross(u_ref))
}

/// The unit vector at an azimuth: `u_ref·c + v_ref·s`. A newtype
/// rather than a bare `Vec3<T>` so that it and [`Tangential`] cannot be
/// exchanged (module docs).
#[derive(Clone, Copy, Debug)]
pub(crate) struct Radial<T: Real>(pub(crate) Vec3<T>);

/// The derivative of [`Radial`] in the azimuth: `u_ref·(−s) + v_ref·c`.
/// Distinct from it by type, for the reason on [`Radial`].
#[derive(Clone, Copy, Debug)]
pub(crate) struct Tangential<T: Real>(pub(crate) Vec3<T>);

/// The azimuthal frame at one angle: the unit vector at that azimuth
/// and its derivative, each in its own type.
#[derive(Clone, Copy, Debug)]
pub(crate) struct AzimuthFrame<T: Real> {
    /// The unit vector at azimuth `u`.
    pub radial: Radial<T>,
    /// Its derivative in `u`.
    pub tangential: Tangential<T>,
}

/// The azimuthal frame at angle `u`, from one `sin_cos` call, with the
/// associations exactly as written (D9); the two formulas are on
/// [`AzimuthFrame`]'s fields.
pub(crate) fn frame<T: Real>(axis: Vec3<T>, u_ref: Vec3<T>, u: T) -> AzimuthFrame<T> {
    let ((s, c), v_ref) = basis(axis, u_ref, u);
    AzimuthFrame {
        radial: Radial(u_ref * c + v_ref * s),
        tangential: Tangential(u_ref * (-s) + v_ref * c),
    }
}

/// **The principal window of an angular channel** starts here: an
/// angle is placed in `(ANGLE_WINDOW_START, ANGLE_WINDOW_START + τ]`.
///
/// C4 places a periodic image "over the edge's span in the chart's
/// principal window" and leaves the window to the implementation. The
/// one property that matters is where its cut sits: an image whose span
/// midpoint lands on the cut has its copy picked by rounding. Images
/// cluster on the landmark angles (a seam at `0`, the half turn, the
/// quarter turns), so the cut sits at a generic angle, `−1` radian.
/// Measured over the workspace suite's harmonic images (3228 distinct,
/// 237 k reads, planes excluded): the number whose midpoint lies within
/// `1e-6` of each candidate cut, and the number a candidate moves from
/// where the derivation used to put it.
///
/// | window | moved | on the cut |
/// |---|---|---|
/// | `(−π, π]` | 11.1% | 76.9 k |
/// | `(0, τ]` | 37.8% | 87.6 k |
/// | `(−π/2, 3π/2]` | 0.9% | 21.6 k |
/// | `(−1, τ − 1]` | 0.8% | 0 |
///
/// On the torus's tube angle the same window moves 6.6% and puts none on
/// the cut. A sphere's latitude is named in its own window
/// ([`LATITUDE_WINDOW_START`]).
///
/// **The table is a measurement and is not guarded.** It reads every
/// image the workspace suite derives, which no one row sees, and a census
/// row would pin the suite's population rather than the window. It is
/// re-taken by logging each placed image's channels at its span's
/// midpoint from `chart_pcurve_over` over a full
/// `cargo nextest run --workspace --profile ci` and counting, per
/// candidate window, the images it moves and the midpoints within `1e-6`
/// of its cut.
///
/// A body can still put a midpoint on the cut (a cone rotated so its
/// seam sits at `−1`). Within the band of the cut the placement's mark
/// is undecided and takes its lower branch (the D9 tie-break), so
/// either rounding names one copy; and no reader tells the two copies
/// apart where a midpoint is decided on either side of it
/// (`topo::pcurves`' `a_meridian_on_the_windows_cut_reads_the_same_either_side_of_it`:
/// validation, the chart boundary, the polygon modulo a turn and a face's
/// overlap with itself). That a boolean's union path does not either is
/// argued, not pinned: it was measured by moving the cut onto a cone's
/// seam (to `0` and `0.5`) under `refusal_concision`'s cone union, which
/// read the same (PR 5225), and the row that would hold it is
/// `work/pcert/no-row-pins-that-the-boolean-union-path-reads-window-copies-alike`.
pub const ANGLE_WINDOW_START: f64 = -1.0;

/// **A sphere's latitude window** starts here: a latitude is placed in
/// `(LATITUDE_WINDOW_START, LATITUDE_WINDOW_START + τ]`, that is
/// `(1 − π, 1 + π]`, centred on latitude `1`.
///
/// Every point of the chart has a latitude in `[−π/2, π/2]`, inside the
/// window. The *read* ranges further: a meridian placed on its far half
/// reads its polar angle as `π − δ`, anywhere on `(π/2, 3π/2)`. So the
/// read's one jump has to sit at a generic angle, not on a landmark: at
/// an equator start a far-half read is `π`, and `atan2`'s own cut there
/// put an `Interval` read a whole turn wide. The window's cut, at
/// `1 + π ≡ 1 − π`, is reached by a far-half read only at the generic
/// latitude `δ = −1`, the footing the angle window's offset arms are on.
/// Every latitude below `1` reads on the direct side, bit-identical to
/// `atan2`; those in `(1, π/2]` read through the turned formula
/// (`stable_azimuth`), so their bits are not `atan2`'s. No pinned row
/// reads one, but unpinned rows do (the delta-6 review counted 257 reads
/// strictly inside, in 13 rows, and 122 k exactly at `π/2`, from pole
/// starts, which are bit-equal at `f64` and a different enclosure at
/// `Interval`), and all of them pass.
///
/// Measured over the workspace suite's sphere placements (1.51 M reads,
/// 2380 distinct midpoints): no read lies within `1e-6` of this window's
/// cut, against 431 (27 distinct) within `1e-6` of `atan2`'s at `π`.
/// This window moves 10.1% of the placements (the angle at the midpoint
/// before placement lies outside it, `k ≠ 0`), as `(−π, π]` does, and
/// neither puts a midpoint on its cut (`(−π/2, 3π/2]` would put 302 k
/// there, `(0, τ]` 551 k). After placement nearly every midpoint lies in
/// `[−π/2, π/2]`: the delta-6 review counted 3 of 170 k outside, all in
/// the pole band, on the fast set. Against `(−π, π]`, 504 workspace
/// midpoints (9 distinct, all of rows that run a meridian past a full
/// turn) take another turn; on the fast set none do. The table is a
/// measurement and is not guarded, re-taken as [`ANGLE_WINDOW_START`]'s
/// is, logging each latitude placement's read and midpoint.
pub const LATITUDE_WINDOW_START: f64 = 1.0 - core::f64::consts::PI;

/// **An angle read off the geometry**: the two frame components of a
/// direction, as `atan2` would read them, with no arithmetic of its own.
/// It leaves as an angle one way: read in a window
/// ([`Azimuth::principal`] in the angle window, `principal_in` in any),
/// the stable `atan2` read whose one jump is that window's cut, and whose
/// copy a derivation then decides through the window's marks
/// (`geom_brep`'s `Placement::phase`, a `Period::decide` about the
/// window's centre).
/// No whole-period integer is read off a value.
///
/// An anchored `atan2` (the direction rotated back by a reference and
/// read there) would name the copy with no decision, but it rotates
/// every read, so a symbolic scalar no longer folds the read onto the
/// carrier's own form: on the M10-10 plate the map-residual reads went
/// from 36 theorems to 36 numeric decisions and the plate left the band
/// below its ceiling. The principal read keeps `k = 0` bit-identical.
#[derive(Clone, Copy, Debug)]
pub struct Azimuth<T> {
    y: T,
    x: T,
}

impl<T: Real> Azimuth<T> {
    /// The direction with components `y` and `x` in a chart's angular
    /// frame, in `atan2`'s argument order.
    pub fn of(y: T, x: T) -> Self {
        Self { y, x }
    }

    /// The direction's length: the radius its angle is read at.
    pub fn length(self) -> T {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T: Decide> Azimuth<T> {
    /// The principal angle in the angle window ([`Window::Angle`],
    /// `principal_in`): `atan2`'s, read on whichever side of the
    /// window's cut line the direction lies (`stable_azimuth`): a choice
    /// between two formulas equal modulo a turn, so the angle is one of
    /// the direction's names. A direction decided on either side reads in
    /// `(−1, τ − 1]`, and one on the cut reads the closed end `τ − 1`. One
    /// in band just inside the open end, at angle `−1 + δ`, reads
    /// `τ − 1 + δ`: the turned formula, a hair past the window, which
    /// placement's marks take back. A direction of no length (or, at `Interval`,
    /// one whose enclosure reaches zero length: no bounded reciprocal)
    /// names no angle, and every angle names its zero-length vector alike:
    /// it reads `0`, the chart's `u_ref`, at every scalar, as `atan2(0, 0)`
    /// does at `f64` (a pole's zero-radius parallel, a sub-band rim's
    /// centre on the axis). At `Interval` the rule broadens the same way:
    /// an enclosure of the length that straddles zero has no bounded
    /// reciprocal, so it reads `0` too, although the direction it encloses
    /// may have length.
    pub fn principal(self, band: Band) -> T {
        self.principal_in(Window::Angle, band)
    }

    /// The angle read in `window`: as [`Azimuth::principal`] reads in the
    /// angle window, with the cut line through `window`'s own start, so a
    /// direction decided on either side reads in that window and the
    /// read's one jump is the window's cut. A direction of no length reads
    /// `0`, in every window.
    pub fn principal_in(self, window: Window, band: Band) -> T {
        if !(T::one() / self.length()).is_bounded() {
            return T::zero();
        }
        stable_azimuth(self.y, self.x, window.start(), band)
    }
}

/// **The window a placed channel is named in**, and so the window its
/// angle is read in: each window carries its start, the reader takes its
/// cut there (`Azimuth::principal_in`), and the marks are read about its
/// centre. Every angle of a periodic chart is named in the angular
/// window, `(ANGLE_WINDOW_START, ANGLE_WINDOW_START + τ]`, but a sphere's
/// latitude is not an angle every value of which is the chart's: only
/// `[−π/2, π/2]` is (`chart_region` refuses an image past it as unbounded
/// in its sphere arm). Its window, `(LATITUDE_WINDOW_START,
/// LATITUDE_WINDOW_START + τ]`, holds every valid latitude, so each is
/// named as itself, and its cut sits at a generic angle past both
/// poles (`1 + π` beyond the north, `1 − π` beyond the south): the
/// meridian's placed polar angle, the parallel's `asin` and the
/// projected image's latitude, which is never moved, are one convention.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Window {
    /// An angle of a periodic chart.
    Angle,
    /// A sphere's latitude.
    Latitude,
}

/// Every window's start lies in `(−π, 0]`, which `stable_azimuth`'s two
/// formulas need to cover `(start, start + τ]`; a start moved outside it
/// fails the build.
const _: () = {
    use core::f64::consts::PI;
    assert!(-PI < ANGLE_WINDOW_START && ANGLE_WINDOW_START <= 0.0);
    assert!(-PI < LATITUDE_WINDOW_START && LATITUDE_WINDOW_START <= 0.0);
};

impl Window {
    /// The window's open end, its cut: the one place its start is spelled.
    pub fn start<T: Real>(self) -> T {
        match self {
            Window::Angle => T::from_f64(ANGLE_WINDOW_START),
            Window::Latitude => T::from_f64(LATITUDE_WINDOW_START),
        }
    }

    /// The window's centre, half a turn from its cut either way.
    pub fn centre<T: Real>(self) -> T {
        T::pi() + self.start()
    }
}

/// The angle of `(x, y)` (M5 S13, shared by every chart's derivation
/// since M6-3), by one of two formulas equal modulo a turn:
/// `atan2(y, x)`, its cut on the negative-`x` axis, or
/// `atan2(−y, −x) + π`, its cut on the positive-`x` axis.
///
/// Which one is decided across the line through the angles `start` and
/// `start + π` (`pcurve_chart_cut_side`), `start` in `(−π, 0]`: on the
/// side that holds angle `0`, the first; on the other, in band or
/// undecided, the second. So a direction decided on either side reads in
/// `(start, start + τ]`, and the read jumps only across the line's band
/// at `start`, on the angle-`0` side of it (`Azimuth::principal`'s doc
/// says what a direction in that band reads); the geom-brep README's C4
/// says why the windows' starts are generic angles.
fn stable_azimuth<T: Decide>(y: T, x: T, start: T, band: Band) -> T {
    let (s, c) = start.sin_cos();
    match decide("pcurve_chart_cut_side", Margin::of(c * y - s * x), band) {
        Ok(Sign::Positive) => y.atan2(x),
        Ok(Sign::Negative | Sign::Zero) | Err(_) => (T::zero() - y).atan2(T::zero() - x) + T::pi(),
    }
}

#[cfg(test)]
mod tests {
    use geom_core::Real;

    use super::*;

    /// The two doors agree **bit for bit** with the formula
    /// `DESIGN.md`'s parameterization row and the two module headers
    /// state: `radial = u_ref·c + v_ref·s`,
    /// `tangential = u_ref·(−s) + v_ref·c`, `v_ref = axis × u_ref`,
    /// from one `sin_cos`.
    ///
    /// This is the row that makes the shared body safe to share. Every
    /// arm that used to spell the formula inline now reads it from
    /// here, so a "simplification" of these four lines — reassociating
    /// the sum, folding the scale in, computing `v_ref` differently —
    /// would move real geometry while looking like tidying. The
    /// comparison is written out longhand on purpose: it is an
    /// independent transcription of the documented formula, not a call
    /// back into the code under test.
    ///
    /// `Real::sin_cos` is named explicitly because on a bare `f64` the
    /// INHERENT `std` method wins over the trait one and the two differ
    /// by an ulp. Generic bodies only ever see the trait method; this
    /// row must too, or it measures the dispatch rather than the frame.
    ///
    /// **What it does not have to pin, and what it still does.**
    /// Exchanging the two across a typed boundary is E0308 since they
    /// became [`Radial`] and [`Tangential`], so the shape the module
    /// used to concede is gone from every site that passes or
    /// constructs them. A binding renamed at the destructure and
    /// unwrapped immediately still compiles (module docs), and THAT
    /// residue is what stays covered indirectly, downstream, by
    /// the derivative-vs-dual rows in
    /// `tests/curves/nurbs_differential.rs`, the hand-computed
    /// partials in `tests/surfaces/review_m2_pr1.rs` and the
    /// locus/periodicity property rows on each enum. Those suites are
    /// still this body's downstream evidence; they are no longer its
    /// only guard against a transposition.
    #[test]
    fn both_doors_are_bitwise_the_documented_formula() {
        let axis = Vec3::new(0.31, -0.72, 0.61).normalize();
        let u_ref = {
            let raw = Vec3::new(0.83, 0.19, -0.52);
            (raw - axis * raw.dot(axis)).normalize()
        };
        let bits = |v: Vec3<f64>| (v.x.to_bits(), v.y.to_bits(), v.z.to_bits());

        for k in -400i32..400 {
            let u = f64::from(k) * 0.017_37;
            let (s, c) = Real::sin_cos(u);
            let v_ref = axis.cross(u_ref);

            let ((bs, bc), bv) = basis(axis, u_ref, u);
            assert_eq!(bs.to_bits(), s.to_bits(), "basis sin at {u}");
            assert_eq!(bc.to_bits(), c.to_bits(), "basis cos at {u}");
            assert_eq!(bits(bv), bits(v_ref), "basis v_ref at {u}");

            let f = frame(axis, u_ref, u);
            assert_eq!(
                bits(f.radial.0),
                bits(u_ref * c + v_ref * s),
                "radial at {u}"
            );
            assert_eq!(
                bits(f.tangential.0),
                bits(u_ref * (-s) + v_ref * c),
                "tangential at {u}"
            );
        }
    }

    /// CERT-N2 R1 reviewer probe, adopted with its **expectation
    /// updated by the fix pass**, which is the record of what moved.
    /// As the reviewer wrote it the row asserted that a transposition
    /// still COMPILES — true then, because both fields were `Vec3<T>`,
    /// and the falsification of the claim that they could not be
    /// exchanged. Since [`Radial`] and [`Tangential`] are distinct
    /// types the struct-literal swap no longer compiles at all (the
    /// two arms of it are E0308 in both directions, which is why they
    /// cannot be written here). What SURVIVES the newtypes is the
    /// half the module docs now name out loud: a destructure that
    /// renames the fields and unwraps them on the spot still compiles,
    /// because after `.0` both are vectors again.
    #[test]
    fn probe_the_renaming_destructure_is_the_residue_the_types_do_not_close() {
        let axis = Vec3::new(0.0, 0.0, 1.0);
        let u_ref = Vec3::new(1.0, 0.0, 0.0);
        let f = frame(axis, u_ref, 0.3f64);
        let AzimuthFrame {
            radial: tangential,
            tangential: radial,
        } = frame(axis, u_ref, 0.3f64);
        assert_eq!(radial.0.x, f.tangential.0.x);
        assert_eq!(tangential.0.x, f.radial.0.x);
    }
}
