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
//! - `w` is the third witness the MINT produced, unit and orthogonal
//!   to both by construction. Which product that is depends on the
//!   mint: [`OrthoFrame::gram_schmidt`] and the exact world frames
//!   build `w = u × v` — at `f64` the rounded cross product, at
//!   `Interval` an enclosure of it, at `Dual` the product rule's
//!   tangent — while the aim mints store the AIM they were given and
//!   derive `v = w × u` from it, so there `u × v` equals `w` only up
//!   to the last bit. [`OrthoFrame::to_affine`] uses the stored `w`,
//!   whichever it is.
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
//! - [`OrthoFrame::from_aim_and_reference`] — an aim already held as
//!   a witness plus an arbitrary reference: the reference's residual
//!   perpendicular to the aim is decided into the frame's `u`, `v` is
//!   `aim × u`, and the aim becomes `w` verbatim. It is
//!   [`OrthoFrame::gram_schmidt`] with the kept axis already decided,
//!   and exists so that axis is not decided a second time.
//! - [`OrthoFrame::from_axis_and_reference`] — the same pair of roles
//!   when the axis is RAW: the axis is decided first and kept as `w`,
//!   the reference yields to it. One door for the spine-and-radial
//!   spelling, so a caller holding two raw vectors does not write the
//!   ladder out again.
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
/// directions nobody assembled here cannot become a frame. Every
/// value below is well typed and every name below exists — privacy is
/// the single reason this does not compile:
///
/// ```compile_fail
/// use geom_core::{Band, OrthoFrame, Point3, Tol, UnitVec3, Vec3};
/// let band = Band::linear(Tol::witness()).unwrap();
/// let x = UnitVec3::new(Vec3::unit_x(), "doc", band).unwrap();
/// let y = UnitVec3::new(Vec3::unit_y(), "doc", band).unwrap();
/// let z = UnitVec3::new(Vec3::unit_z(), "doc", band).unwrap();
/// let f: OrthoFrame<f64> = OrthoFrame { origin: Point3::origin(), u: x, v: y, w: z };
/// ```
///
/// and the one mint that takes perpendicularity on trust —
/// `from_aim`, whose `perp_raw` is perpendicular to the aim only
/// because its callers form it as a cross product with the aim — is
/// not reachable from outside `geom_core`'s `linalg`, for the same
/// single reason. The public aim door is
/// [`OrthoFrame::from_aim_and_reference`], which makes the
/// perpendicular rather than trusting one:
///
/// ```compile_fail
/// use geom_core::{Band, OrthoFrame, Point3, Tol, UnitVec3, Vec3};
/// let band = Band::linear(Tol::witness()).unwrap();
/// let aim = UnitVec3::new(Vec3::unit_z(), "doc", band).unwrap();
/// let f = OrthoFrame::from_aim(Point3::origin(), aim, Vec3::unit_x(), "doc", band).unwrap();
/// ```
#[derive(Debug, Clone, Copy)]
pub struct OrthoFrame<T: Real> {
    origin: Point3<T>,
    u: UnitVec3<T>,
    v: UnitVec3<T>,
    w: UnitVec3<T>,
}

/// Which of an authored pair a mint refused, so a caller can name the
/// axis in its own vocabulary. It is the ROLE in the mint, not the
/// slot in the frame: the kept axis lands in `u` for
/// [`OrthoFrame::gram_schmidt`] and in `w` for the axis-and-reference
/// mints, and [`OrthoAxis::U`] is the kept one either way.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrthoAxis {
    /// The first authored axis, normalized and kept. The aim mints
    /// never name it: their axis is a witness already.
    U,
    /// The second authored axis, as its residual perpendicular to the
    /// first — so a refusal here says the two span no plane, or, for
    /// the mints that take an axis and a reference, that the
    /// reference has no component perpendicular to the axis.
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

    /// The frame's third axis: `u × v` as the Gram–Schmidt and exact
    /// mints built it, or the aim verbatim as the aim mints stored it.
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
    /// The third column is the frame's own stored `w`, whatever the
    /// mint put there: `u × v` for [`Self::gram_schmidt`] and the
    /// exact world frames, the aim verbatim for the aim mints. Nothing
    /// here re-derives an axis the frame already carries, so for an
    /// aim-minted frame this column is the caller's aim and not the
    /// rounded `u × v`, which differs from it in the last bit.
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
    /// Two decisions, both through [`UnitVec3::new`] under the one
    /// funnel name the CALLER owns the pair under: the first axis's
    /// own length, then the second axis's residual perpendicular to
    /// it. One name and not two, because which of the pair a refusal
    /// is about is [`OrthoFrameError`]'s own field and a K reader is
    /// after the layer that owns the value rather than the leg. A pair
    /// that spans no plane is a decided-zero residual, so "these two
    /// are parallel" is stated as a length at the door every direction
    /// length is decided at, rather than as a perpendicularity
    /// predicate of its own.
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
        site: &'static str,
        band: Band,
    ) -> Result<Self, OrthoFrameError> {
        let u = UnitVec3::new(u_raw, site, band).map_err(|error| OrthoFrameError {
            axis: OrthoAxis::U,
            error,
        })?;
        let v = Self::residual(u, v_raw, site, band)?;
        Ok(Self {
            origin,
            u,
            v,
            w: UnitVec3::cross_of_orthonormal(u, v),
        })
    }

    /// The Gram–Schmidt residual of `raw` against the kept witness
    /// `kept`, decided — the one spelling of the second authored
    /// axis, shared by every mint that has one.
    fn residual(
        kept: UnitVec3<T>,
        raw: Vec3<T>,
        site: &'static str,
        band: Band,
    ) -> Result<UnitVec3<T>, OrthoFrameError> {
        let k = kept.get();
        UnitVec3::new(raw - k * raw.dot(k), site, band).map_err(|error| OrthoFrameError {
            axis: OrthoAxis::V,
            error,
        })
    }

    /// **An aim already decided, plus a perpendicular the CALLER
    /// guarantees**: `perp_raw ⊥ aim` is this door's unchecked
    /// precondition, which is why it is not public — the public aim
    /// door is [`Self::from_aim_and_reference`], which forms the
    /// perpendicular itself.
    ///
    /// Evaluation order (fixed, D9): `u = normalize(perp_raw)`, then
    /// `v = aim × u`, with `w` the aim itself.
    ///
    /// # Errors
    ///
    /// [`OrthoFrameError`] under [`OrthoAxis::V`] when `perp_raw` has
    /// no decided direction — it lies on the aim line, or it
    /// overflowed the norm, underflowed out of the format, or landed
    /// in the band.
    pub(in crate::linalg) fn from_aim(
        origin: Point3<T>,
        aim: UnitVec3<T>,
        perp_raw: Vec3<T>,
        site: &'static str,
        band: Band,
    ) -> Result<Self, OrthoFrameError> {
        let u = UnitVec3::new(perp_raw, site, band).map_err(|error| OrthoFrameError {
            axis: OrthoAxis::V,
            error,
        })?;
        Ok(Self {
            origin,
            u,
            v: UnitVec3::cross_of_orthonormal(aim, u),
            w: aim,
        })
    }

    /// **[`Self::gram_schmidt`] with the kept axis already decided**:
    /// the frame whose third axis is `aim` and whose first is the part
    /// of `reference` perpendicular to it.
    ///
    /// The axis is not re-decided. Routing this through
    /// [`Self::gram_schmidt`] instead would normalize a witness a
    /// second time, and a witness's own norm is not exactly `1.0`, so
    /// the axis the caller holds and the axis the frame carries would
    /// differ in their last bits. A caller whose axis is still RAW
    /// wants [`Self::from_axis_and_reference`], which decides it here
    /// rather than at the call site.
    ///
    /// Evaluation order (fixed, D9): the residual
    /// `reference − aim·(reference·aim)`, then `u` its normalize,
    /// then `v = aim × u`, with `w` the aim itself.
    ///
    /// # Errors
    ///
    /// [`OrthoFrameError`] under [`OrthoAxis::V`] when the residual
    /// has no decided direction — `reference` lies on the aim line, or
    /// the residual overflowed, underflowed, or landed in the band.
    /// [`OrthoAxis::U`] cannot arise: the aim is a witness already.
    pub fn from_aim_and_reference(
        origin: Point3<T>,
        aim: UnitVec3<T>,
        reference: Vec3<T>,
        site: &'static str,
        band: Band,
    ) -> Result<Self, OrthoFrameError> {
        let u = Self::residual(aim, reference, site, band)?;
        Ok(Self {
            origin,
            u,
            v: UnitVec3::cross_of_orthonormal(aim, u),
            w: aim,
        })
    }

    /// **A raw axis and a raw reference**: the spine-and-radial
    /// spelling, where the axis is KEPT as the frame's `w` and the
    /// reference yields whatever component of it lies along that axis.
    ///
    /// This is [`Self::gram_schmidt`]'s decision with the kept axis
    /// landing in `w` instead of `u`, which is the shape every caller
    /// that places a tube, a skin section or a ring has: a centre, an
    /// axis, and a reference radial the angles are measured from. It
    /// exists so those callers stop spelling the two-step ladder
    /// (decide the axis, then mint from the aim) out by hand.
    ///
    /// Evaluation order (fixed, D9): `w = normalize(axis_raw)`, then
    /// [`Self::from_aim_and_reference`]'s order against it.
    ///
    /// # Errors
    ///
    /// [`OrthoFrameError`] under [`OrthoAxis::U`] when `axis_raw` has
    /// no decided direction, and under [`OrthoAxis::V`] when the
    /// reference's residual perpendicular to that axis has none.
    pub fn from_axis_and_reference(
        origin: Point3<T>,
        axis_raw: Vec3<T>,
        reference: Vec3<T>,
        site: &'static str,
        band: Band,
    ) -> Result<Self, OrthoFrameError> {
        let aim = UnitVec3::new(axis_raw, site, band).map_err(|error| OrthoFrameError {
            axis: OrthoAxis::U,
            error,
        })?;
        Self::from_aim_and_reference(origin, aim, reference, site, band)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::tolerance::Tol;

    const SITE: &str = "ortho_frame_test_axis";

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
    /// spelling the frame out by hand gets, for BOTH families of mint.
    ///
    /// For the Gram–Schmidt and exact mints the hand spelling is
    /// `from_cols(u, v, u × v)`: the frame's `w` IS that cross
    /// product, evaluated once at the mint. For the aim mints it is
    /// `from_cols(x, aim × x, aim)` — the arithmetic the retired
    /// aiming frame constructor spelled — where `w` is the aim
    /// VERBATIM and `u × v` is a rounding away from it, which is the
    /// reason this row carries both corpora rather than recomputing
    /// `u × v` for all of them.
    #[test]
    fn to_affine_is_the_hand_spelling_bit_for_bit() {
        for (o, u_raw, v_raw) in decided_corpus() {
            let f = OrthoFrame::gram_schmidt(o, u_raw, v_raw, SITE, band()).unwrap();
            let (u, v) = (f.u().get(), f.v().get());
            let hand = Affine3::from_parts(Mat3::from_cols(u, v, u.cross(v)), o - Point3::origin());
            assert_eq!(
                bits12(&f.to_affine()),
                bits12(&hand),
                "gram_schmidt at {o:?}"
            );
        }
        for (o, axis_raw, ref_raw) in decided_corpus() {
            let f =
                OrthoFrame::from_axis_and_reference(o, axis_raw, ref_raw, SITE, band()).unwrap();
            let aim = axis_raw.normalize();
            let x = (ref_raw - aim * ref_raw.dot(aim)).normalize();
            let hand =
                Affine3::from_parts(Mat3::from_cols(x, aim.cross(x), aim), o - Point3::origin());
            assert_eq!(bits12(&f.to_affine()), bits12(&hand), "from_axis at {o:?}");
        }
    }

    /// **An aim-minted frame's `w` is the aim, not `u × v`** — the
    /// distinction the module docs and [`OrthoFrame::to_affine`] state,
    /// asserted where it is visible: over the corpus the two differ in
    /// at least one component's bits for at least one frame, so a doc
    /// that promised `w = u × v` for every mint would be false.
    #[test]
    fn the_aim_mints_store_the_aim_rather_than_the_cross_product() {
        let mut differed = 0_u32;
        for (o, axis_raw, ref_raw) in decided_corpus() {
            let f =
                OrthoFrame::from_axis_and_reference(o, axis_raw, ref_raw, SITE, band()).unwrap();
            let bits = |v: Vec3<f64>| [v.x, v.y, v.z].map(f64::to_bits);
            assert_eq!(
                bits(f.w().get()),
                bits(axis_raw.normalize()),
                "the stored w is the decided axis verbatim, at {o:?}"
            );
            if bits(f.w().get()) != bits(f.u().get().cross(f.v().get())) {
                differed += 1;
            }
        }
        assert!(
            differed > 0,
            "no aim-minted frame in the corpus separated w from u × v, so this row              cannot witness the distinction its subject is about"
        );
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
                let f = OrthoFrame::gram_schmidt(o, u_raw, v_raw, SITE, band()).unwrap();
                let got = cols(f).map(|c| [c.x, c.y, c.z].map(f64::to_bits));
                let want =
                    [u_raw, v_raw, u_raw.cross(v_raw)].map(|c| [c.x, c.y, c.z].map(f64::to_bits));
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
                OrthoFrame::gram_schmidt(o, Vec3::unit_x(), v_raw, SITE, band()).unwrap_err(),
                OrthoFrameError {
                    axis: OrthoAxis::V,
                    error: UnitVec3Error::Degenerate,
                },
                "at {v_raw:?}"
            );
        }
        assert_eq!(
            OrthoFrame::gram_schmidt(o, Vec3::new(0.0, 0.0, 0.0), Vec3::unit_y(), SITE, band())
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
    /// offset's own length, under the SECOND authored axis, because
    /// the aim was decided before it arrived.
    #[test]
    fn from_aim_orders_its_columns_and_refuses_an_offset_on_the_line() {
        let aim = UnitVec3::new(Vec3::new(0.0, 0.0, 2.0), SITE, band()).unwrap();
        let perp = Vec3::new(3.0, 0.0, 0.0);
        let f = OrthoFrame::from_aim(Point3::new(1.0, 2.0, 3.0), aim, perp, SITE, band()).unwrap();
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
                SITE,
                band()
            )
            .unwrap_err(),
            OrthoFrameError {
                axis: OrthoAxis::V,
                error: UnitVec3Error::Degenerate,
            }
        );
    }

    /// **The raw-axis door decides the axis and then the reference's
    /// residual**, and names which of the two refused: a reference
    /// lying ALONG a perfectly good axis is the second authored axis's
    /// refusal, an axis with no direction the first's. That is the
    /// distinction a caller's message needs — "this vector has zero
    /// length" is false of a reference five units long that happens to
    /// lie on the axis.
    #[test]
    fn from_axis_and_reference_names_which_of_the_two_refused() {
        let o = Point3::new(-1.0, 0.5, 2.0);
        let axis = Vec3::new(0.0, 0.0, 4.0);
        let long_on_axis = Vec3::new(0.0, 0.0, 5.0);
        assert_eq!(
            OrthoFrame::from_axis_and_reference(o, axis, long_on_axis, SITE, band()).unwrap_err(),
            OrthoFrameError {
                axis: OrthoAxis::V,
                error: UnitVec3Error::Degenerate,
            }
        );
        assert_eq!(
            OrthoFrame::from_axis_and_reference(
                o,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::unit_x(),
                SITE,
                band()
            )
            .unwrap_err(),
            OrthoFrameError {
                axis: OrthoAxis::U,
                error: UnitVec3Error::Degenerate,
            }
        );
        // The axis is kept as `w` and the reference yields, which is
        // the role swap the Gram-Schmidt door does not do.
        let f =
            OrthoFrame::from_axis_and_reference(o, axis, Vec3::new(2.0, 0.0, 7.0), SITE, band())
                .unwrap();
        let bits = |v: Vec3<f64>| [v.x, v.y, v.z].map(f64::to_bits);
        assert_eq!(bits(f.w().get()), bits(Vec3::unit_z()));
        assert_eq!(bits(f.u().get()), bits(Vec3::unit_x()));
        assert_eq!(bits(f.v().get()), bits(Vec3::unit_y()));
    }
}
