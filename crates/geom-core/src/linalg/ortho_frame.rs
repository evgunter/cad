//! The frame **witness**: an origin and a right-handed orthonormal
//! triple whose orthonormality was established where it was BUILT,
//! carried as a type so the fact survives a function boundary instead
//! of being re-stated as prose at every door that places geometry.
//!
//! # What the witness means, at every scalar
//!
//! An [`OrthoFrame`] is an origin plus three directions where
//!
//! - `u` and `v` are [`UnitVec3`] witnesses — each a decided normalize,
//!   or a decided normalize and its exact negation, so what "unit"
//!   means here is exactly what it means there (an `f64` value at
//!   `f64`, an enclosure of a unit vector at `Interval`, a unit value
//!   channel with the quotient rule's tangent at `Dual`);
//! - `v` was produced ORTHOGONAL to `u` by construction — a
//!   Gram–Schmidt residual against `u`, or a cross product with it —
//!   never by a caller asserting that two vectors it held happened to
//!   be perpendicular;
//! - `w = u × v`, unit and orthogonal to both by construction: at
//!   `f64` the rounded cross product, at `Interval` an enclosure of
//!   it, at `Dual` the product rule's tangent.
//!
//! As for the direction witness, this is not a bit-for-bit claim that
//! the three columns are exactly orthonormal — it is the claim that no
//! step between the decision and this value could have unmade the
//! fact.
//!
//! # The mints, and only these
//!
//! - [`OrthoFrame::gram_schmidt`] — the authored pair, orthonormalized:
//!   `u` is normalized and KEPT, `v` yields its component along `u`.
//!   Two decisions, under the caller's own funnel names.
//! - [`OrthoFrame::from_aim`] — an aim direction already held as a
//!   witness, plus a raw vector perpendicular to it (a cross product,
//!   in every caller here): the perpendicular is decided and
//!   normalized into the frame's `u`, `v` is `aim × u`, and the aim
//!   becomes `w`. One decision, under the caller's funnel name.
//! - [`OrthoFrame::axes_xy`], [`OrthoFrame::axes_yz`],
//!   [`OrthoFrame::axes_zx`] — the world frames, at any origin. These
//!   are **the only frames the type admits without a decision**,
//!   because their axes are the exact basis vectors: unit is the
//!   literal bits, and the cross products are exact at every scalar
//!   that represents 0 and 1.
//!
//! There is deliberately **no** mint from two or three vectors a
//! caller believes are orthonormal, for the reason [`UnitVec3`]'s docs
//! give for the direction: a caller holding such a pair has either a
//! decision to record — then [`OrthoFrame::gram_schmidt`] IS that
//! decision, and on an exactly-orthonormal pair it changes no bit —
//! or no decision, and the type would have nothing to witness.
//!
//! # What is NOT this type
//!
//! [`Affine3`] stays the general affine map: a frame CONVERTS into one
//! ([`OrthoFrame::to_affine`]), and an [`Affine3`] read back off a body
//! is a placement, not a witness. The geometry carriers' direction
//! fields — `Line.dir`, `Plane.normal`, the conic axes — stay bare
//! `Vec3` under the at-rest rule stated in `geom`'s crate docs, and so
//! do the carrier frames those fields make up. Two other types in the
//! workspace are called frames and are different things:
//! `editor-core`'s placement `Frame` stores nine floats read back from
//! a rotation, and `sweep`'s revolve `AxisFrame` is the sketch-to-world
//! map of a 2-D revolve.
//!
//! The order of the axes and the roll convention that fixes them have
//! one home, [`frame`](super::frame)'s module docs; nothing is
//! restated here.

use crate::linalg::{Affine3, Mat3, Point3, UnitVec3, UnitVec3Error, Vec3};
use crate::predicate::{Band, Decide};
use crate::real::Real;

/// **A frame that cannot be non-orthonormal.** An origin and a
/// right-handed orthonormal triple; the only spellings are the mints
/// the [module docs](self) list, and the fields are private, so a
/// placement built from one is rigid as a property of the TYPE rather
/// than of the caller's diligence.
///
/// A non-orthonormal "frame" is not poison — it is a well-defined skew
/// map, which is exactly what makes it dangerous: the sketch
/// coordinates drawn on it are silently sheared and every length
/// measured through it is silently scaled, with no refusal anywhere.
/// That failure is unrepresentable here rather than asserted.
///
/// No `PartialEq`: comparison is the predicate layer's job, as for
/// every linalg type here.
///
/// The fields are not reachable from outside this module, so three
/// vectors nobody decided cannot become a frame:
///
/// ```compile_fail
/// use geom_core::{OrthoFrame, Point3, Vec3};
/// let f: OrthoFrame<f64> = OrthoFrame {
///     origin: Point3::origin(),
///     u: Vec3::unit_x(),
///     v: Vec3::unit_y(),
///     w: Vec3::unit_z(),
/// };
/// ```
///
/// and there is no conversion into the type from vectors a caller
/// believes are orthonormal:
///
/// ```compile_fail
/// use geom_core::{OrthoFrame, Point3, Vec3};
/// let f = OrthoFrame::<f64>::new(Point3::origin(), Vec3::unit_x(), Vec3::unit_y());
/// ```
#[derive(Debug, Clone, Copy)]
pub struct OrthoFrame<T: Real> {
    origin: Point3<T>,
    u: UnitVec3<T>,
    v: UnitVec3<T>,
    w: UnitVec3<T>,
}

/// Which of an authored pair [`OrthoFrame::gram_schmidt`] refused, so
/// a caller can name the axis in its own vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrthoAxis {
    /// The first axis, normalized and kept.
    U,
    /// The second axis, as its residual perpendicular to the first —
    /// so a refusal here says the two span no plane.
    V,
}

/// **Why an authored pair is no frame** — the direction door's refusal,
/// plus which of the two axes it was about.
///
/// The fact is [`UnitVec3Error`]'s and is carried unaltered: a pair
/// that spans no plane refuses as a decided-zero RESIDUAL under
/// [`OrthoAxis::V`], at the same door every other direction length is
/// decided at, rather than under a perpendicularity predicate invented
/// for frames.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrthoFrameError {
    /// Which authored axis the refusal is about.
    pub axis: OrthoAxis,
    /// What the direction door said.
    pub error: UnitVec3Error,
}

impl core::fmt::Display for OrthoFrameError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let axis = match self.axis {
            OrthoAxis::U => "a frame's first axis",
            OrthoAxis::V => "a frame's second axis, as its residual perpendicular to the first",
        };
        write!(f, "{axis}: {}", self.error)
    }
}

impl std::error::Error for OrthoFrameError {}

impl<T: Real> OrthoFrame<T> {
    /// The frame's base point.
    #[must_use]
    pub fn origin(self) -> Point3<T> {
        self.origin
    }

    /// The frame's first axis.
    #[must_use]
    pub fn u(self) -> UnitVec3<T> {
        self.u
    }

    /// The frame's second axis, orthogonal to [`Self::u`].
    #[must_use]
    pub fn v(self) -> UnitVec3<T> {
        self.v
    }

    /// The frame's third axis, `u × v`.
    #[must_use]
    pub fn w(self) -> UnitVec3<T> {
        self.w
    }

    /// The placement of this frame: the map sending the coordinate
    /// origin to [`Self::origin`], `x̂` to `u`, `ŷ` to `v` and `ẑ` to
    /// `w` — linear columns `u`, `v`, `w` ([`Mat3::from_cols`]) and
    /// translation `origin − Point3::origin()`, in exactly that order
    /// (D9).
    ///
    /// The third column is the frame's own `w`, computed at the mint
    /// as `u × v` — the same product, in the same evaluation order, at
    /// the same scalar, so nothing here re-derives an axis the frame
    /// already carries.
    ///
    /// The translation is the subtraction `origin − Point3::origin()`:
    /// componentwise `x − (+0.0)`, which IEEE 754 leaves at `x` for
    /// every bit pattern — `−0.0`, `±inf` and NaN payloads included
    /// (the corpus row in `affine.rs` measures it) — so a frame read
    /// back from the stored map, columns and translation, is bitwise
    /// the frame that was written.
    #[must_use]
    pub fn to_affine(self) -> Affine3<T> {
        Affine3::from_parts(
            Mat3::from_cols(self.u.get(), self.v.get(), self.w.get()),
            self.origin - Point3::origin(),
        )
    }

    /// The world **xy** frame at `origin`: `u = x̂`, `v = ŷ`, `w = ẑ`.
    #[must_use]
    pub fn axes_xy(origin: Point3<T>) -> Self {
        Self::exact(origin, UnitVec3::exact_x(), UnitVec3::exact_y())
    }

    /// The world **yz** frame at `origin`: `u = ŷ`, `v = ẑ`, `w = x̂`.
    #[must_use]
    pub fn axes_yz(origin: Point3<T>) -> Self {
        Self::exact(origin, UnitVec3::exact_y(), UnitVec3::exact_z())
    }

    /// The world **zx** frame at `origin`: `u = ẑ`, `v = x̂`, `w = ŷ`.
    #[must_use]
    pub fn axes_zx(origin: Point3<T>) -> Self {
        Self::exact(origin, UnitVec3::exact_z(), UnitVec3::exact_x())
    }

    /// The three world frames' shared body: two exact basis vectors and
    /// the cross product they determine, which is the third basis
    /// vector exactly.
    fn exact(origin: Point3<T>, u: UnitVec3<T>, v: UnitVec3<T>) -> Self {
        Self {
            origin,
            u,
            v,
            w: UnitVec3::cross_of_orthonormal(u, v),
        }
    }
}

impl<T: Decide> OrthoFrame<T> {
    /// **The authored pair, orthonormalized** — Gram–Schmidt, with `u`
    /// normalized and KEPT while `v` yields its component along `u`.
    ///
    /// Two decisions, each through [`UnitVec3::new`] under the funnel
    /// name the CALLER owns the value under: `site_u` for the first
    /// axis's own length, `site_v` for the second axis's residual
    /// perpendicular to the first. A pair that spans no plane is a
    /// decided-zero residual, so "these two are parallel" is stated as
    /// a length at the door every direction length is decided at,
    /// rather than as a perpendicularity predicate of its own.
    ///
    /// Evaluation order (fixed, D9): `u = normalize(u_raw)`, then
    /// `v_perp = v_raw − u·(v_raw·u)`, then `v = normalize(v_perp)`,
    /// then `w = u × v`. On a pair that is already exactly orthonormal
    /// — two signed basis vectors, say — every step is exact and the
    /// frame's axes are the caller's vectors bit for bit, with one
    /// exception: a `−0.0` component of `v_raw` comes back `+0.0`,
    /// because the residual subtracts `−0.0` from it and IEEE 754
    /// gives that sum the positive sign.
    ///
    /// # Errors
    ///
    /// [`OrthoFrameError`] naming [`OrthoAxis::U`] or [`OrthoAxis::V`]
    /// and carrying the direction door's own fact.
    pub fn gram_schmidt(
        origin: Point3<T>,
        u_raw: Vec3<T>,
        v_raw: Vec3<T>,
        site_u: &'static str,
        site_v: &'static str,
        band: Band,
    ) -> Result<Self, OrthoFrameError> {
        let u = UnitVec3::new(u_raw, site_u, band).map_err(|error| OrthoFrameError {
            axis: OrthoAxis::U,
            error,
        })?;
        let v_perp = v_raw - u.get() * v_raw.dot(u.get());
        let v = UnitVec3::new(v_perp, site_v, band).map_err(|error| OrthoFrameError {
            axis: OrthoAxis::V,
            error,
        })?;
        Ok(Self {
            origin,
            u,
            v,
            w: UnitVec3::cross_of_orthonormal(u, v),
        })
    }

    /// **An aim already decided, plus a raw perpendicular**: the frame
    /// whose third axis is `aim` and whose first axis is `perp_raw`
    /// normalized.
    ///
    /// `perp_raw` is the caller's offset off the aim line — a cross
    /// product with the aim in every caller here — so it is
    /// perpendicular to `aim` by construction and the only open
    /// question is its LENGTH, which is the one decision this mint
    /// makes, under the caller's funnel name.
    ///
    /// Evaluation order (fixed, D9): `u = normalize(perp_raw)`, then
    /// `v = aim × u`, with `w` the aim itself.
    ///
    /// # Errors
    ///
    /// [`UnitVec3Error`] when `perp_raw` has no decided direction: the
    /// caller's reference lies on the aim line, or the offset
    /// overflowed the norm, underflowed out of the format, or landed
    /// in the band.
    pub fn from_aim(
        origin: Point3<T>,
        aim: UnitVec3<T>,
        perp_raw: Vec3<T>,
        site: &'static str,
        band: Band,
    ) -> Result<Self, UnitVec3Error> {
        let u = UnitVec3::new(perp_raw, site, band)?;
        Ok(Self {
            origin,
            u,
            v: UnitVec3::cross_of_orthonormal(aim, u),
            w: aim,
        })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::tolerance::Tol;

    const SITE_U: &str = "ortho_frame_test_u";
    const SITE_V: &str = "ortho_frame_test_v";

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn bits12(a: &Affine3<f64>) -> [u64; 12] {
        let l = a.linear;
        [
            l.c0.x,
            l.c0.y,
            l.c0.z,
            l.c1.x,
            l.c1.y,
            l.c1.z,
            l.c2.x,
            l.c2.y,
            l.c2.z,
            a.translation.x,
            a.translation.y,
            a.translation.z,
        ]
        .map(f64::to_bits)
    }

    fn cols(f: OrthoFrame<f64>) -> [Vec3<f64>; 3] {
        [f.u().get(), f.v().get(), f.w().get()]
    }

    /// The frames every mint can reach, as the raw material each was
    /// minted from: an origin, a first axis and a second.
    fn decided_corpus() -> Vec<(Point3<f64>, Vec3<f64>, Vec3<f64>)> {
        let mut v = vec![
            (Point3::origin(), Vec3::unit_x(), Vec3::unit_y()),
            (Point3::new(-0.0, 3.5, -2.0), Vec3::unit_z(), Vec3::unit_x()),
            (
                Point3::new(1e6, -1e-6, 0.0),
                Vec3::new(1.0, 1.0, 0.0),
                Vec3::new(-1.0, 1.0, 2.0),
            ),
            (
                Point3::new(0.25, 0.5, -0.75),
                Vec3::new(3.0, -4.0, 12.0),
                Vec3::new(0.0, 1.0, 0.0),
            ),
        ];
        for k in 0..32 {
            let t = f64::from(k) * 0.19;
            v.push((
                Point3::new(t, -t, t * t),
                Vec3::new(t.cos(), t.sin(), 0.3 * t),
                Vec3::new(-t.sin(), t.cos(), 1.0),
            ));
        }
        v
    }

    /// **The placement is the three columns and the origin, bit for
    /// bit** — the same arithmetic, in the same order, that a caller
    /// spelling `from_cols(u, v, u × v)` by hand gets. The frame's `w`
    /// IS that cross product, evaluated once at the mint, so nothing is
    /// re-derived at the conversion and the two spellings cannot drift.
    #[test]
    fn to_affine_is_the_hand_spelling_bit_for_bit() {
        for (o, u_raw, v_raw) in decided_corpus() {
            let f = OrthoFrame::gram_schmidt(o, u_raw, v_raw, SITE_U, SITE_V, band()).unwrap();
            let (u, v) = (f.u().get(), f.v().get());
            let hand =
                Affine3::from_parts(Mat3::from_cols(u, v, u.cross(v)), o - Point3::origin());
            assert_eq!(bits12(&f.to_affine()), bits12(&hand), "at {o:?}");
        }
    }

    /// **Gram–Schmidt on an exactly orthonormal pair changes no bit.**
    /// That is what lets a caller which used to hand two exact axes to
    /// a frame constructor route through the mint instead: the
    /// normalize divides by exactly `1.0`, and the residual subtracts
    /// exactly zero.
    #[test]
    fn gram_schmidt_keeps_an_exact_pair_bit_for_bit() {
        // Written as literals with `+0.0` in the zero slots, which is
        // how a call site spells a signed axis. A `−0.0` slot in the
        // YIELDED axis is the one thing the residual does not carry:
        // `−0.0 − (−0.0)` is `+0.0`, so the subtraction's sign wins.
        let axes = [
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
        ];
        for u_raw in axes {
            for v_raw in axes {
                if u_raw.cross(v_raw).norm() == 0.0 {
                    continue;
                }
                let o = Point3::new(1.5, -0.0, 7.25);
                let f = OrthoFrame::gram_schmidt(o, u_raw, v_raw, SITE_U, SITE_V, band()).unwrap();
                let got = cols(f).map(|c| [c.x, c.y, c.z].map(f64::to_bits));
                let want = [u_raw, v_raw, u_raw.cross(v_raw)]
                    .map(|c| [c.x, c.y, c.z].map(f64::to_bits));
                assert_eq!(got, want, "at {u_raw:?} {v_raw:?}");
            }
        }
    }

    /// The three world frames ARE the exact basis vectors, in the
    /// cyclic order their names say, with the origin stored bitwise.
    #[test]
    fn the_world_frames_are_the_exact_basis_triples() {
        let o = Point3::new(-0.0, 2.0, f64::MIN_POSITIVE);
        let bits = |v: [Vec3<f64>; 3]| v.map(|c| [c.x, c.y, c.z].map(f64::to_bits));
        assert_eq!(
            bits(cols(OrthoFrame::axes_xy(o))),
            bits([Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()])
        );
        assert_eq!(
            bits(cols(OrthoFrame::axes_yz(o))),
            bits([Vec3::unit_y(), Vec3::unit_z(), Vec3::unit_x()])
        );
        assert_eq!(
            bits(cols(OrthoFrame::axes_zx(o))),
            bits([Vec3::unit_z(), Vec3::unit_x(), Vec3::unit_y()])
        );
        assert_eq!(
            bits12(&OrthoFrame::axes_xy(o).to_affine())[9..],
            [o.x, o.y, o.z].map(f64::to_bits)
        );
    }

    /// **A pair that spans no plane refuses as the SECOND axis's
    /// residual** — the perpendicular component is zero, which is a
    /// length, so the refusal is the direction door's `Degenerate`
    /// under [`OrthoAxis::V`] rather than a perpendicularity verdict of
    /// its own. A first axis with no direction refuses under
    /// [`OrthoAxis::U`].
    #[test]
    fn a_parallel_pair_refuses_as_the_second_axis_residual() {
        let o = Point3::origin();
        for v_raw in [Vec3::new(2.0, 0.0, 0.0), Vec3::new(-0.5, 0.0, 0.0)] {
            assert_eq!(
                OrthoFrame::gram_schmidt(o, Vec3::unit_x(), v_raw, SITE_U, SITE_V, band())
                    .unwrap_err(),
                OrthoFrameError {
                    axis: OrthoAxis::V,
                    error: UnitVec3Error::Degenerate,
                },
                "at {v_raw:?}"
            );
        }
        assert_eq!(
            OrthoFrame::gram_schmidt(
                o,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::unit_y(),
                SITE_U,
                SITE_V,
                band()
            )
            .unwrap_err(),
            OrthoFrameError {
                axis: OrthoAxis::U,
                error: UnitVec3Error::Degenerate,
            }
        );
    }

    /// The aim mint's columns are `(normalize(perp), aim × that, aim)`
    /// — the recipe the aiming frame constructors hand it — and a
    /// reference whose offset lies on the aim line refuses at that
    /// offset's own length.
    #[test]
    fn from_aim_orders_its_columns_and_refuses_an_offset_on_the_line() {
        let aim = UnitVec3::new(Vec3::new(0.0, 0.0, 2.0), SITE_U, band()).unwrap();
        let perp = Vec3::new(3.0, 0.0, 0.0);
        let f =
            OrthoFrame::from_aim(Point3::new(1.0, 2.0, 3.0), aim, perp, SITE_V, band()).unwrap();
        let x = perp.normalize();
        assert_eq!(
            cols(f).map(|c| [c.x, c.y, c.z].map(f64::to_bits)),
            [x, aim.get().cross(x), aim.get()].map(|c| [c.x, c.y, c.z].map(f64::to_bits))
        );
        assert_eq!(
            OrthoFrame::from_aim(
                Point3::origin(),
                aim,
                Vec3::new(0.0, 0.0, 0.0),
                SITE_V,
                band()
            )
            .unwrap_err(),
            UnitVec3Error::Degenerate
        );
    }
}
