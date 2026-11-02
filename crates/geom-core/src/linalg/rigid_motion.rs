//! The rigid-motion **witness**: a proper rigid motion of 3-space —
//! orthonormal linear part, determinant +1, every number finite —
//! whose rigidity was established where it was BUILT, carried as a
//! type so the fact survives a function boundary instead of being
//! re-derived from twelve numbers at every door that moves geometry.
//!
//! A frame is the rigid motion that places the world frame, so the
//! one type is both: [`RigidMotion::origin`] and the axes
//! [`RigidMotion::u`], [`RigidMotion::v`], [`RigidMotion::w`] are the
//! frame's reading, and the map sending the coordinate origin to
//! `origin` and `x̂`, `ŷ`, `ẑ` to `u`, `v`, `w`
//! ([`RigidMotion::to_affine`]) is the motion's.
//!
//! # What the witness means, at every scalar
//!
//! - `u`, `v` and `w` are [`UnitVec3`] witnesses, pairwise orthogonal
//!   and right-handed by construction — produced by a decision, by a
//!   product that is exact in the reals, or by a rigid motion acting
//!   on a witness — never by a caller asserting that three vectors it
//!   held happened to be orthonormal. For a motion MINTED by
//!   construction, and products and inverses of minted motions, read
//!   per scalar (a motion carried across scalars means what its
//!   crossing says, "Crossing scalars" below):
//!   - at `f64`, rigid up to rounding;
//!   - at `Interval`, an enclosure of ONE rigid motion per point of
//!     the box its inputs enclose: the entries are wide, and what is
//!     rigid is each motion they enclose, not the box;
//!   - at `Sym`, the same, with the expressions carried;
//!   - at `Dual`, a rigid value channel with the tangent the
//!     construction's own arithmetic transports.
//! - **A motion ADMITTED at the decided door is rigid within the
//!   band**, not up to rounding: [`RigidMotion::check_rigid`] admits a
//!   map whose seven rigidity margins decide `Zero`, so each may be off
//!   by up to the band's zero, a dimensionless `ε`. Products are not
//!   decided again, so a product of `k` admitted factors is rigid only
//!   within about `k` bands (each factor's defect adds to the next's to
//!   first order), and enough of them make a product the door itself
//!   would refuse. `motion_rows`'s
//!   `a_product_of_admitted_factors_drifts_by_a_band_a_factor` pins the
//!   bound and the refusal past it.
//! - The origin and every entry are finite numbers: each mint and the
//!   decided door ask it exactly at every scalar ([`Real::is_bounded`]:
//!   both endpoints at an enclosure); a non-finite linear entry poisons
//!   the door's margins.
//!
//! As for the direction witness, this is not a bit-for-bit claim that
//! the three columns are exactly orthonormal — it is the claim that no
//! step between the construction and this value could have unmade the
//! fact. In particular nothing re-derives rigidity from the entries:
//! at `Interval`, `cos θ` and `sin θ` of a widened angle are two
//! independent brackets, so the entries' own `c·c − 1` and `c0·c1`
//! enclose intervals around zero that no band below their width
//! decides, although every motion they enclose is rigid.
//!
//! # The mints by construction
//!
//! Each decides only finiteness, and names what was not finite
//! ([`RigidMotionError::NonFinite`]):
//!
//! - [`RigidMotion::translation`];
//! - [`RigidMotion::rotation`] — about a [`UnitVec3`] through the
//!   coordinate origin — and [`RigidMotion::rotation_then_translation`],
//!   the same rotation with a translation stored verbatim beside it;
//! - [`RigidMotion::rotation_about_axis`] — about a [`UnitVec3`]
//!   through a point;
//! - [`RigidMotion::rotation_by`] and
//!   [`RigidMotion::rotation_about_axis_by`] — the same by an
//!   [`Angle`], whose sine and cosine are its written fraction of a
//!   turn's where it has one ([`Angle::from_turns`]);
//! - [`RigidMotion::turned`] — the world frame at an origin turned a
//!   written fraction of a turn ([`Turns`]) about a [`UnitVec3`];
//! - the exact half-turns [`RigidMotion::half_turn_x`],
//!   [`RigidMotion::half_turn_y`], [`RigidMotion::half_turn_z`] and
//!   the exact world frames [`RigidMotion::axes_xy`],
//!   [`RigidMotion::axes_yz`], [`RigidMotion::axes_zx`] (these, and
//!   [`RigidMotion::turned`], store the caller's `origin` and decide
//!   nothing at all);
//! - [`RigidMotion::translated`] — a motion followed by a translation,
//!   its axes kept;
//! - [`RigidMotion::compose`] — products of witnesses are not decided
//!   again;
//! - [`RigidMotion::inverse`] — the transpose.
//!
//! **An axis-aligned frame is a written turn.** An exact axis
//! ([`UnitVec3::exact_x`] and its siblings) turned by a whole number
//! of quarter turns has columns of `0` and `±1` exactly at every
//! scalar, and a product of such turns is not decided again, so every
//! frame whose axes are signed basis vectors is reached with no
//! decision: `(x̂, ẑ)` is a quarter turn about `x̂`. The named
//! half-turns and world frames are shortcuts for the commonest of
//! them. A motion composed with such a turn (`frame.compose(turn)`)
//! is the same turn in the frame's own coordinates, exact in those
//! coordinates and as rounded as the frame is in the world's.
//! [`RigidMotion::gram_schmidt`] is for a pair the caller computed.
//!
//! The rotation mints take their axis as a witness and do not
//! normalize it a second time ([`Mat3::rotation_about`]'s Rodrigues
//! form over the axis as given).
//!
//! # The decided mints
//!
//! - [`RigidMotion::gram_schmidt`] — a computed pair,
//!   orthonormalized: `u` is normalized and KEPT, `v` yields its
//!   component along `u`. Two decisions, under the caller's own
//!   funnel name.
//! - [`RigidMotion::from_aim_and_reference`] — an aim already held as
//!   a witness plus an arbitrary reference: the reference's residual
//!   perpendicular to the aim is decided into `u`, `v` is `aim × u`,
//!   and the aim becomes `w` verbatim. It exists so that axis is not
//!   decided a second time.
//! - [`RigidMotion::from_axis_and_reference`] — the same roles when
//!   the axis is RAW: the axis is decided first and kept as `w`, the
//!   reference yields to it.
//! - [`RigidMotion::turned_between`] — the minimal rotation carrying
//!   one direction witness onto another, as the matrix of the
//!   quaternion `(1 + a·b, a × b)` with no normalized axis and no
//!   angle. One decision, `1 + a·b > 0`: the opposed pair, where no
//!   rotation is THE minimal one.
//!
//! Which product `w` is depends on the mint: the Gram–Schmidt mint and
//! the exact world frames build `w = u × v` (at `f64` the rounded
//! cross product, at `Interval` an enclosure of it); the aim mints
//! store the AIM they were given and derive `v = w × u` from it; the
//! rotation mints (and the turns) take all three columns from the
//! Rodrigues entries, and [`RigidMotion::compose`] and
//! [`RigidMotion::inverse`] from the product or the transpose. Outside
//! the Gram–Schmidt mint and the world frames, then, `u × v` equals `w`
//! only up to the last bit, except where the entries are exact (a turn
//! by quarter turns about an exact axis).
//! [`RigidMotion::to_affine`] uses the stored `w`, whichever it is.
//!
//! # Crossing scalars
//!
//! A frame exists at a scalar only by being minted there: by an exact
//! mint, a decided mint or the decided door, over inputs held at that
//! scalar. No door carries a frame up from `f64`. Every frame that
//! would cross is the product of a decision, so at `Sym` it would
//! enter either as literals, a theorem about its rounding (E12), or as
//! unknowns, over which nothing is proved; minted at the scalar, it
//! carries its inputs' dependence on the variables instead (Ev on
//! #5068, 2026-10-14).
//!
//! The one crossing is downward, closure-free: an arbitrary map of the
//! twelve numbers could unmake the witness (negating every component
//! leaves `w` opposed to `u × v`).
//!
//! - [`RigidMotion::point_value`] — any scalar to `f64`, every
//!   component through [`SpanLocate::point_value`], `None` where one
//!   has no single finite `f64`: the read a structure selected at
//!   `f64` makes of a motion held at a lane scalar.
//!
//! # The decided door
//!
//! [`RigidMotion::check_rigid`] admits a raw [`Affine3`] — a literal
//! placement frame, a STEP placement, a library caller's matrix: the
//! map's seven rigidity margins (three column norms, three pairwise
//! orthogonalities, the determinant) classified against the linear
//! band, then its translation's finiteness. It is the one door from
//! twelve numbers into the type, and the one home of that rule. It
//! returns a margin it decided non-zero as [`RigidMotionError::NotRigid`]
//! and one it could not decide as [`RigidMotionError::Undecided`].
//!
//! There is deliberately **no** door that takes rigidity on trust, for
//! the reason [`UnitVec3`]'s docs give for the direction: a caller
//! holding a map it believes is rigid has either a construction to
//! name — then a mint above IS that construction — or twelve numbers,
//! and then [`RigidMotion::check_rigid`] is the decision.
//!
//! # What is NOT this type
//!
//! [`Affine3`] stays the general affine map: a motion CONVERTS into one
//! ([`RigidMotion::to_affine`]), and an [`Affine3`] read back off a body
//! is a placement, not a witness. There is no mirror and no scale here:
//! the kernel acts with one group. The geometry carriers' direction
//! fields — `Line.dir`, `Plane.normal`, the conic axes — stay bare
//! `Vec3` under the at-rest rule stated in `geom`'s crate docs, and so
//! do the carrier frames those fields make up. Two other types in the
//! workspace are called frames and are different things:
//! `editor-core`'s placement `Frame` stores a general linear part, so
//! an improper map stays representable and refusable, and `sweep`'s
//! revolve `AxisFrame` is the sketch-to-world map of a 2-D revolve.
//!
//! The order of the axes and the roll convention that fixes them have
//! one home, [`frame`](super::frame)'s module docs; nothing is
//! restated here.

use crate::linalg::{Affine3, Angle, Mat3, Point3, Turns, UnitVec3, UnitVec3Error, Vec3};
use crate::predicate::{Band, Decide, Indeterminate, Sign};
use crate::real::Real;
use crate::spline::SpanLocate;

/// **A motion that cannot be non-rigid.** An origin and a right-handed
/// orthonormal triple; the only spellings are the mints the
/// [module docs](self) list, and the fields are private, so a
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
/// directions nobody assembled here cannot become a motion. Every
/// value below is well typed and every name below exists — privacy is
/// the single reason this does not compile:
///
/// ```compile_fail
/// use geom_core::{Band, Point3, RigidMotion, Tol, UnitVec3, Vec3};
/// let band = Band::linear(Tol::witness()).unwrap();
/// let x = UnitVec3::new(Vec3::unit_x(), "doc", band).unwrap();
/// let y = UnitVec3::new(Vec3::unit_y(), "doc", band).unwrap();
/// let z = UnitVec3::new(Vec3::unit_z(), "doc", band).unwrap();
/// let f: RigidMotion<f64> = RigidMotion { origin: Point3::origin(), u: x, v: y, w: z };
/// ```
///
/// and the one mint that takes perpendicularity on trust —
/// `from_aim`, whose `perp_raw` is perpendicular to the aim only
/// because its callers form it as a cross product with the aim — is
/// not reachable from outside `geom_core`'s `linalg`, for the same
/// single reason. The public aim door is
/// [`RigidMotion::from_aim_and_reference`], which makes the
/// perpendicular rather than trusting one:
///
/// ```compile_fail
/// use geom_core::{Band, Point3, RigidMotion, Tol, UnitVec3, Vec3};
/// let band = Band::linear(Tol::witness()).unwrap();
/// let aim = UnitVec3::new(Vec3::unit_z(), "doc", band).unwrap();
/// let f = RigidMotion::from_aim(Point3::origin(), aim, Vec3::unit_x(), "doc", band).unwrap();
/// ```
#[derive(Debug, Clone, Copy)]
pub struct RigidMotion<T: Real> {
    origin: Point3<T>,
    u: UnitVec3<T>,
    v: UnitVec3<T>,
    w: UnitVec3<T>,
}

/// Which of an authored pair a decided mint refused, so a caller can
/// name the axis in its own vocabulary. It is the ROLE in the mint,
/// not the slot in the frame: the kept axis lands in `u` for
/// [`RigidMotion::gram_schmidt`] and in `w` for the axis-and-reference
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
pub struct OrthonormalizeError {
    /// Which authored axis the refusal is about.
    pub axis: OrthoAxis,
    /// What the direction door said.
    pub error: UnitVec3Error,
}

impl core::fmt::Display for OrthonormalizeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let axis = match self.axis {
            OrthoAxis::U => "a frame's first axis",
            OrthoAxis::V => "a frame's second axis, as its residual perpendicular to the first",
        };
        write!(f, "{axis}: {}", self.error)
    }
}

impl std::error::Error for OrthonormalizeError {}

/// **What a mint by construction found not finite** — the part of the
/// motion it names ([`RigidMotionError::NonFinite`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionPart {
    /// A rotation's angle: its sine and cosine would be poison.
    Angle,
    /// The translation handed to a mint.
    Translation,
    /// The point a rotation's axis passes through, or the translation
    /// that rotation about it makes.
    Anchor,
    /// The translation of a composition of two motions.
    Product,
    /// The translation of an inverse.
    Inverse,
    /// An entry of the linear part of a map the decided door was
    /// handed ([`RigidMotion::check_rigid`]).
    Linear,
}

impl MotionPart {
    fn noun(self) -> &'static str {
        match self {
            Self::Angle => "rotation angle",
            Self::Translation => "translation",
            Self::Anchor => "rotation axis's anchor point, or the translation it makes",
            Self::Product => "translation of a composition of two motions",
            Self::Inverse => "translation of an inverse motion",
            Self::Linear => "linear part",
        }
    }
}

/// **Why no rigid motion was minted** — a mint by construction's
/// finiteness refusal, or the decided door's verdict on a raw map
/// (closed enum, D4 ¶3).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RigidMotionError {
    /// A part that is not a finite number: a mint by construction's
    /// input or product, or a part of a map the decided door was handed
    /// ([`MotionPart::Linear`] before any margin is decided,
    /// [`MotionPart::Translation`] after).
    NonFinite(MotionPart),
    /// [`RigidMotion::check_rigid`] decided a rigidity margin NON-zero:
    /// the map scales an axis, shears, or mirrors at tolerance.
    NotRigid {
        /// The named predicate that refused ([`not_rigid_reading`]
        /// puts it in words).
        check: &'static str,
    },
    /// [`RigidMotion::check_rigid`] could not decide a rigidity margin:
    /// its enclosure straddles the band, or it is poison. A
    /// maybe-rigid map is not admitted, and the refusal says it is
    /// undecided rather than that the map is not rigid.
    Undecided {
        /// The named predicate that could not be decided.
        check: &'static str,
        /// The decision's own diagnostic, unaltered.
        source: Indeterminate,
    },
}

/// The recourse every rigidity refusal of a raw map ends with.
const RIGID_RECOURSE: &str = "use only a rotation and a translation";

impl core::fmt::Display for RigidMotionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonFinite(part) => write!(
                f,
                "a rigid motion's {} is not a finite number. Recourse: {}",
                part.noun(),
                crate::predicate::RANGE_RECOURSE
            ),
            Self::NotRigid { check } => write!(
                f,
                "the map is not rigid at tolerance: {}. Recourse: {RIGID_RECOURSE}",
                rigid_defect(check)
            ),
            // The door refuses a non-finite entry before any margin, so a
            // poisoned margin here is the kernel's arithmetic; at f64 none
            // is reachable past that probe, since finite entries give
            // finite or infinite (decided) column norms and decided-unit
            // columns bound the rest.
            Self::Undecided { check, source } => source
                .undecided(
                    format_args!("whether the map is rigid ({})", not_rigid_reading(check)),
                    source.lever_ending(RIGID_RECOURSE),
                )
                .fmt(f),
        }
    }
}

impl std::error::Error for RigidMotionError {}

/// **What a rigidity check found possible**, in words — the one
/// reading of [`RigidMotion::check_rigid`]'s check names for a check
/// that did NOT decide `Zero`, whether it decided a defect or could
/// not decide, so the reading says what the check found possible, not
/// a verdict. The checks run in order: the mirror check is reached
/// only by columns decided orthonormal.
#[must_use]
pub fn not_rigid_reading(check: &str) -> &'static str {
    match check {
        "transform_rigid_col01_orth"
        | "transform_rigid_col12_orth"
        | "transform_rigid_col02_orth" => "it may shear",
        "transform_rigid_det_plus_one" => "it may mirror",
        _ => "it may scale an axis",
    }
}

/// The defect a check DECIDED non-zero names.
fn rigid_defect(check: &str) -> &'static str {
    match check {
        "transform_rigid_col01_orth"
        | "transform_rigid_col12_orth"
        | "transform_rigid_col02_orth" => "it shears",
        "transform_rigid_det_plus_one" => "it mirrors",
        _ => "it scales an axis",
    }
}

/// `Ok(p)` when every component of `p` [`Real::is_bounded`] — a
/// finite number at a point scalar, both endpoints finite at an
/// enclosure, the value channel's reading at a carrier — `Err` naming
/// `part` otherwise. The one spelling of the finiteness question: the
/// mints and the decided door both ask it here.
fn finite<T: Real>(p: Point3<T>, part: MotionPart) -> Result<Point3<T>, RigidMotionError> {
    if p.x.is_bounded() && p.y.is_bounded() && p.z.is_bounded() {
        Ok(p)
    } else {
        Err(RigidMotionError::NonFinite(part))
    }
}

/// The point whose coordinates are `v`'s components, copied — no
/// arithmetic, so every bit (a `−0.0` included) is `v`'s.
fn at<T: Real>(v: Vec3<T>) -> Point3<T> {
    Point3::new(v.x, v.y, v.z)
}

impl<T: Real> RigidMotion<T> {
    /// The frame's base point: the image of the coordinate origin.
    #[must_use]
    pub fn origin(self) -> Point3<T> {
        self.origin
    }

    /// The frame's first axis: the image of `x̂`.
    #[must_use]
    pub fn u(self) -> UnitVec3<T> {
        self.u
    }

    /// The frame's second axis, orthogonal to [`Self::u`]: the image
    /// of `ŷ`.
    #[must_use]
    pub fn v(self) -> UnitVec3<T> {
        self.v
    }

    /// The frame's third axis, the image of `ẑ`: `u × v` as the
    /// Gram–Schmidt and exact mints built it, or the aim verbatim as
    /// the aim mints stored it.
    #[must_use]
    pub fn w(self) -> UnitVec3<T> {
        self.w
    }

    /// The linear part: columns `u`, `v`, `w` ([`Mat3::from_cols`]).
    #[must_use]
    pub fn linear(self) -> Mat3<T> {
        Mat3::from_cols(self.u.get(), self.v.get(), self.w.get())
    }

    /// The translation: `origin − Point3::origin()`, componentwise
    /// `x − (+0.0)`, which IEEE 754 leaves at `x` for every bit
    /// pattern — `−0.0`, `±inf` and NaN payloads included (the corpus
    /// row in `affine.rs` measures it).
    fn offset(self) -> Vec3<T> {
        self.origin - Point3::origin()
    }

    /// The motion as a general affine map: [`Self::linear`] and the
    /// translation `origin − Point3::origin()`, in exactly that order
    /// (D9). Nothing here
    /// re-derives an axis the motion already carries, so for an
    /// aim-minted frame the third column is the caller's aim and not
    /// the rounded `u × v`, and a motion read back from the stored
    /// map, columns and translation, is bitwise the one that was
    /// written.
    #[must_use]
    pub fn to_affine(self) -> Affine3<T> {
        Affine3::from_parts(self.linear(), self.offset())
    }

    /// The motion applied to a point: [`Affine3::transform_point`]'s
    /// arithmetic on [`Self::to_affine`].
    #[must_use]
    pub fn transform_point(self, p: Point3<T>) -> Point3<T> {
        self.to_affine().transform_point(p)
    }

    /// The motion applied to a tangent vector: the linear part only.
    #[must_use]
    pub fn transform_vec(self, x: Vec3<T>) -> Vec3<T> {
        self.linear() * x
    }

    /// **The motion acting on a direction**: a rigid linear part
    /// carries a unit vector to a unit vector, so the image is a
    /// witness by construction — `linear · d`, [`Mat3`]'s fixed
    /// matrix-vector order.
    #[must_use]
    pub fn turn(self, d: UnitVec3<T>) -> UnitVec3<T> {
        UnitVec3::by_construction(self.linear() * d.get())
    }

    /// The identity motion: the world xy frame at the coordinate
    /// origin.
    #[must_use]
    pub fn identity() -> Self {
        Self::axes_xy(Point3::origin())
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

    /// The half-turn about `x̂` through the coordinate origin:
    /// `diag(1, −1, −1)`, exact at every scalar that represents 0 and
    /// ±1.
    #[must_use]
    pub fn half_turn_x() -> Self {
        Self::exact(Point3::origin(), UnitVec3::exact_x(), -UnitVec3::exact_y())
    }

    /// The half-turn about `ŷ`: `diag(−1, 1, −1)`, exact.
    #[must_use]
    pub fn half_turn_y() -> Self {
        Self::exact(Point3::origin(), -UnitVec3::exact_x(), UnitVec3::exact_y())
    }

    /// The half-turn about `ẑ`: `diag(−1, −1, 1)`, exact.
    #[must_use]
    pub fn half_turn_z() -> Self {
        Self::exact(Point3::origin(), -UnitVec3::exact_x(), -UnitVec3::exact_y())
    }

    /// **The world xy frame at `origin`, turned `turns` of a turn about
    /// `axis`** — [`Self::rotation_by`]'s linear part over
    /// [`Angle::from_turns`], with `origin` stored as the world frames
    /// store theirs. A written fraction's value is finite, so nothing is
    /// refused. About an exact axis by a whole number of quarter turns
    /// the columns are `0` and `±1` exactly, at every scalar: every
    /// axis-aligned frame is one of these or a product of two.
    #[must_use]
    pub fn turned(origin: Point3<T>, axis: UnitVec3<T>, turns: Turns) -> Self {
        Self::from_linear(
            Mat3::rotation_about_unit_by(axis, Angle::from_turns(turns)),
            origin,
        )
    }

    /// The motion whose linear part is `m`'s columns, taken as
    /// witnesses because the CALLER's construction made them
    /// orthonormal and right-handed; private, so only the mints in
    /// this module are callers.
    fn from_linear(m: Mat3<T>, origin: Point3<T>) -> Self {
        Self {
            origin,
            u: UnitVec3::by_construction(m.c0),
            v: UnitVec3::by_construction(m.c1),
            w: UnitVec3::by_construction(m.c2),
        }
    }

    /// **The translation by `t`**: identity linear part, origin at
    /// `t`'s components verbatim.
    ///
    /// # Errors
    ///
    /// [`RigidMotionError::NonFinite`] naming [`MotionPart::Translation`]
    /// for a component that is not a finite number.
    pub fn translation(t: Vec3<T>) -> Result<Self, RigidMotionError> {
        Ok(Self::axes_xy(finite(at(t), MotionPart::Translation)?))
    }

    /// **The rotation by `angle` radians (right-hand rule) about
    /// `axis` through the coordinate origin** — [`Mat3::rotation_about`]'s
    /// Rodrigues entries over the witness as given, not normalized a
    /// second time. Over the reals every such matrix is a rotation, at
    /// every angle; so at `Interval` the entries enclose one rotation
    /// per angle and axis of the box.
    ///
    /// # Errors
    ///
    /// [`RigidMotionError::NonFinite`] naming [`MotionPart::Angle`].
    pub fn rotation(axis: UnitVec3<T>, angle: T) -> Result<Self, RigidMotionError> {
        Self::rotation_then_translation(axis, angle, Vec3::zero())
    }

    /// [`Self::rotation`] by an [`Angle`], whose sine and cosine are its
    /// written form's where it has one.
    ///
    /// # Errors
    ///
    /// [`RigidMotionError::NonFinite`] naming [`MotionPart::Angle`].
    pub fn rotation_by(axis: UnitVec3<T>, angle: Angle<T>) -> Result<Self, RigidMotionError> {
        if !angle.get().is_bounded() {
            return Err(RigidMotionError::NonFinite(MotionPart::Angle));
        }
        Ok(Self::from_linear(
            Mat3::rotation_about_unit_by(axis, angle),
            Point3::origin(),
        ))
    }

    /// **[`Self::rotation`], then the translation by `t`**, built in
    /// one step: linear part the rotation's, translation `t` stored
    /// verbatim — the bits [`Self::compose`] of a translation onto a
    /// rotation would round through an identity product.
    ///
    /// # Errors
    ///
    /// [`RigidMotionError::NonFinite`] naming [`MotionPart::Angle`] or
    /// [`MotionPart::Translation`].
    pub fn rotation_then_translation(
        axis: UnitVec3<T>,
        angle: T,
        t: Vec3<T>,
    ) -> Result<Self, RigidMotionError> {
        if !angle.is_bounded() {
            return Err(RigidMotionError::NonFinite(MotionPart::Angle));
        }
        let origin = finite(at(t), MotionPart::Translation)?;
        Ok(Self::from_linear(
            Mat3::rotation_about_unit(axis, angle),
            origin,
        ))
    }

    /// **The rotation by `angle` radians about the axis through
    /// `point` along `axis`** — [`Affine3::rotation_about_axis`]'s
    /// arithmetic (linear part the rotation, translation `(I − R)·q`
    /// for `q = point − Point3::origin()`, the anchor mentioned once)
    /// over the witness as given.
    ///
    /// # Errors
    ///
    /// [`RigidMotionError::NonFinite`] naming [`MotionPart::Angle`], or
    /// [`MotionPart::Anchor`] for a point, or a translation it makes,
    /// that is not finite.
    pub fn rotation_about_axis(
        point: Point3<T>,
        axis: UnitVec3<T>,
        angle: T,
    ) -> Result<Self, RigidMotionError> {
        Self::rotation_about_axis_by(point, axis, Angle::radians(angle))
    }

    /// [`Self::rotation_about_axis`] by an [`Angle`], whose sine and
    /// cosine are its written form's where it has one.
    ///
    /// # Errors
    ///
    /// As [`Self::rotation_about_axis`]'s.
    pub fn rotation_about_axis_by(
        point: Point3<T>,
        axis: UnitVec3<T>,
        angle: Angle<T>,
    ) -> Result<Self, RigidMotionError> {
        if !angle.get().is_bounded() {
            return Err(RigidMotionError::NonFinite(MotionPart::Angle));
        }
        let q = finite(point, MotionPart::Anchor)? - Point3::origin();
        let map = Affine3::anchored_rotation_by(q, axis.get(), angle);
        Ok(Self::from_linear(
            map.linear,
            finite(at(map.translation), MotionPart::Anchor)?,
        ))
    }

    /// **`self ∘ inner`**: `inner` acts first. The product of two
    /// rigid motions is rigid, so nothing is decided again: the axes
    /// are `self` acting on `inner`'s ([`Self::turn`]) and the origin
    /// is `self` acting on `inner`'s. The arithmetic is
    /// [`Affine3`]'s own product, bit for bit (`linear · linear`
    /// column by column, then `linear · t + t`).
    ///
    /// # Errors
    ///
    /// [`RigidMotionError::NonFinite`] naming [`MotionPart::Product`]
    /// when the composed origin overflows.
    pub fn compose(self, inner: Self) -> Result<Self, RigidMotionError> {
        let origin = finite(self.transform_point(inner.origin), MotionPart::Product)?;
        Ok(Self {
            origin,
            u: self.turn(inner.u),
            v: self.turn(inner.v),
            w: self.turn(inner.w),
        })
    }

    /// **The inverse, as the transpose**: linear part `Rᵀ`, translation
    /// `−(Rᵀ·t)`, in that order. The rows of a rigid linear part are
    /// orthonormal and right-handed, so they are witnesses as the
    /// columns are; the transpose is pure field shuffling, so no
    /// rounding enters the linear part, and at `Interval` the
    /// transposed box encloses each enclosed motion's inverse.
    ///
    /// # Errors
    ///
    /// [`RigidMotionError::NonFinite`] naming [`MotionPart::Inverse`]
    /// when the inverse's translation overflows.
    pub fn inverse(self) -> Result<Self, RigidMotionError> {
        let rt = self.linear().transpose();
        let origin = finite(at(-(rt * self.offset())), MotionPart::Inverse)?;
        Ok(Self::from_linear(rt, origin))
    }

    /// **This motion, then the translation by `t`**: the axes kept,
    /// the origin moved to `origin + t` — the bits
    /// [`Self::compose`] of a translation onto this motion would round
    /// through an identity product.
    ///
    /// # Errors
    ///
    /// [`RigidMotionError::NonFinite`] naming [`MotionPart::Translation`]
    /// when the moved origin is not finite.
    pub fn translated(self, t: Vec3<T>) -> Result<Self, RigidMotionError> {
        Ok(Self {
            origin: finite(self.origin + t, MotionPart::Translation)?,
            ..self
        })
    }
}

impl<T: SpanLocate> RigidMotion<T> {
    /// **The `f64` motion this one's value channel holds**: every
    /// component through [`SpanLocate::point_value`], or `None` when a
    /// component has no single finite `f64` — poison, or an enclosure
    /// wider than a point. The one crossing between scalars
    /// ([module docs](self)), and the read a structure selected at
    /// `f64` makes of a placement held at a lane scalar.
    #[must_use]
    pub fn point_value(self) -> Option<RigidMotion<f64>> {
        let read = |x: T| x.point_value().filter(|x| x.is_finite());
        let o = self.origin;
        Some(RigidMotion {
            origin: Point3::new(read(o.x)?, read(o.y)?, read(o.z)?),
            u: self.u.point_value()?,
            v: self.v.point_value()?,
            w: self.w.point_value()?,
        })
    }
}

impl RigidMotion<f64> {
    /// **This motion entered at `T` as the computed values it is**:
    /// every component through [`Real::from_computed`]. The crossing
    /// changes no component's value, so the axes stay witnesses; at an
    /// enclosure scalar each component is the point it was. The
    /// crossing the other way is [`RigidMotion::point_value`].
    #[must_use]
    pub fn computed_at<T: Real>(self) -> RigidMotion<T> {
        let axis = |a: UnitVec3<f64>| UnitVec3::by_construction(a.get().map(T::from_computed));
        RigidMotion {
            origin: self.origin.map(T::from_computed),
            u: axis(self.u),
            v: axis(self.v),
            w: axis(self.w),
        }
    }
}

impl<T: Decide> RigidMotion<T> {
    /// **The decided door**: a raw [`Affine3`] admitted as a rigid
    /// motion when every rigidity margin classifies `Zero` against the
    /// linear band and every translation component is finite — the one
    /// home of that rule, asked of a literal placement frame, a STEP
    /// placement, or a library caller's matrix.
    ///
    /// First the linear part's finiteness, asked of each entry as the
    /// mints ask it ([`Real::is_bounded`]): a non-finite entry is the
    /// caller's input, refused [`RigidMotionError::NonFinite`] naming
    /// [`MotionPart::Linear`] rather than read as a poisoned margin.
    ///
    /// The seven margins, in order: the three column norms less one,
    /// the three pairwise orthogonalities, the determinant less one.
    /// A margin decided non-zero refuses [`RigidMotionError::NotRigid`];
    /// one the band cannot decide refuses [`RigidMotionError::Undecided`]
    /// with the decision's own diagnostic. Then the translation's
    /// finiteness, asked exactly as the mints ask it
    /// ([`Real::is_bounded`] of each component: both endpoints at an
    /// enclosure), which is not a band decision and logs no row.
    ///
    /// The admitted motion carries the map's columns and translation
    /// verbatim, so [`Self::to_affine`] gives back `map`, bit for bit.
    ///
    /// # Errors
    ///
    /// [`RigidMotionError::NotRigid`] or [`RigidMotionError::Undecided`]
    /// naming the check, or [`RigidMotionError::NonFinite`] naming
    /// [`MotionPart::Linear`] or [`MotionPart::Translation`].
    pub fn check_rigid(map: &Affine3<T>, band: Band) -> Result<Self, RigidMotionError> {
        let l = &map.linear;
        for column in [l.c0, l.c1, l.c2] {
            finite(at(column), MotionPart::Linear)?;
        }
        let one = T::one();
        // A table of K row names: they reach the funnel through the loop
        // variable below, so no grep for a literal at the decide site finds
        // them. A row added here is a roster change (`docs/K-REPORT.md`,
        // "The inventory method, restated").
        let checks: [(&'static str, T); 7] = [
            ("transform_rigid_col0_unit", l.c0.dot(l.c0) - one),
            ("transform_rigid_col1_unit", l.c1.dot(l.c1) - one),
            ("transform_rigid_col2_unit", l.c2.dot(l.c2) - one),
            ("transform_rigid_col01_orth", l.c0.dot(l.c1)),
            ("transform_rigid_col12_orth", l.c1.dot(l.c2)),
            ("transform_rigid_col02_orth", l.c0.dot(l.c2)),
            ("transform_rigid_det_plus_one", l.determinant() - one),
        ];
        for (check, margin) in checks {
            // Ledger row F10: the rigidity residuals of the linear map are
            // DIMENSIONLESS (unit-column/orthogonality/det defects) against
            // the metre band; the natural arm is the model/session-box
            // extent — an arm-policy question deferred by the row. No door
            // fits; flagged, not cast.
            match crate::k_stats::decide_flagged(check, margin, band, "F10") {
                Ok(Sign::Zero) => {}
                Ok(Sign::Positive | Sign::Negative) => {
                    return Err(RigidMotionError::NotRigid { check });
                }
                Err(source) => return Err(RigidMotionError::Undecided { check, source }),
            }
        }
        let origin = finite(at(map.translation), MotionPart::Translation)?;
        Ok(Self::from_linear(*l, origin))
    }

    /// **A computed pair, orthonormalized** — Gram–Schmidt, with `u`
    /// normalized and KEPT while `v` yields its component along `u`. A
    /// pair of signed basis vectors needs no decision: it is a written
    /// turn ([`Self::turned`]).
    ///
    /// Two decisions, both through [`UnitVec3::new`] under the one
    /// funnel name the CALLER owns the pair under: the first axis's
    /// own length, then the second axis's residual perpendicular to
    /// it. One name and not two, because which of the pair a refusal
    /// is about is [`OrthonormalizeError`]'s own field and a K reader is
    /// after the layer that owns the value rather than the leg. A pair
    /// that spans no plane is a decided-zero residual, so "these two
    /// are parallel" is stated as a length at the door every direction
    /// length is decided at, rather than as a perpendicularity
    /// predicate of its own.
    ///
    /// Evaluation order (fixed, D9): `u = normalize(u_raw)`, then
    /// `v_perp = v_raw − u·(v_raw·u)`, then `v = normalize(v_perp)`,
    /// then `w = u × v`. On a pair that is already exactly orthonormal
    /// every step is exact and the
    /// frame's axes are the caller's vectors bit for bit, with one
    /// exception: a `−0.0` component of `v_raw` comes back `+0.0`,
    /// because the residual subtracts `−0.0` from it and IEEE 754
    /// gives that sum the positive sign.
    ///
    /// # Errors
    ///
    /// [`OrthonormalizeError`] naming [`OrthoAxis::U`] or [`OrthoAxis::V`]
    /// and carrying the direction door's own fact.
    pub fn gram_schmidt(
        origin: Point3<T>,
        u_raw: Vec3<T>,
        v_raw: Vec3<T>,
        site: &'static str,
        band: Band,
    ) -> Result<Self, OrthonormalizeError> {
        let u = UnitVec3::new(u_raw, site, band).map_err(|error| OrthonormalizeError {
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
    ) -> Result<UnitVec3<T>, OrthonormalizeError> {
        let k = kept.get();
        UnitVec3::new(raw - k * raw.dot(k), site, band).map_err(|error| OrthonormalizeError {
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
    /// [`OrthonormalizeError`] under [`OrthoAxis::V`] when `perp_raw` has
    /// no decided direction — it lies on the aim line, or it
    /// overflowed the norm, underflowed out of the format, or landed
    /// in the band.
    pub(in crate::linalg) fn from_aim(
        origin: Point3<T>,
        aim: UnitVec3<T>,
        perp_raw: Vec3<T>,
        site: &'static str,
        band: Band,
    ) -> Result<Self, OrthonormalizeError> {
        let u = UnitVec3::new(perp_raw, site, band).map_err(|error| OrthonormalizeError {
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
    /// [`OrthonormalizeError`] under [`OrthoAxis::V`] when the residual
    /// has no decided direction — `reference` lies on the aim line, or
    /// the residual overflowed, underflowed, or landed in the band.
    /// [`OrthoAxis::U`] cannot arise: the aim is a witness already.
    pub fn from_aim_and_reference(
        origin: Point3<T>,
        aim: UnitVec3<T>,
        reference: Vec3<T>,
        site: &'static str,
        band: Band,
    ) -> Result<Self, OrthonormalizeError> {
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
    /// axis, and a reference radial the angles are measured from.
    ///
    /// Evaluation order (fixed, D9): `w = normalize(axis_raw)`, then
    /// [`Self::from_aim_and_reference`]'s order against it.
    ///
    /// # Errors
    ///
    /// [`OrthonormalizeError`] under [`OrthoAxis::U`] when `axis_raw` has
    /// no decided direction, and under [`OrthoAxis::V`] when the
    /// reference's residual perpendicular to that axis has none.
    pub fn from_axis_and_reference(
        origin: Point3<T>,
        axis_raw: Vec3<T>,
        reference: Vec3<T>,
        site: &'static str,
        band: Band,
    ) -> Result<Self, OrthonormalizeError> {
        let aim = UnitVec3::new(axis_raw, site, band).map_err(|error| OrthonormalizeError {
            axis: OrthoAxis::U,
            error,
        })?;
        Self::from_aim_and_reference(origin, aim, reference, site, band)
    }

    /// **The minimal rotation carrying `a` onto `b`, about the axis
    /// through `about`** — the matrix of the unit quaternion along
    /// `(1 + a·b, a × b)`, in its homogeneous form (every entry over
    /// the quaternion's squared norm): no normalized axis and no
    /// angle. The matrix is orthogonal to rounding at every angle,
    /// because each entry is a quadratic in the quaternion over its own
    /// squared norm, and it is the identity, with no rounding, on a pair
    /// that is bitwise one direction (`a × (a + a)` is exactly zero).
    ///
    /// The quaternion's two parts are evaluated through `h = a + b`, as
    /// `1 + a·b = |h|²/2` and `a × b = a × h`. Near opposed `h` is
    /// short and its components carry no cancellation, so both parts
    /// hold to a few ulps RELATIVE to their own size, and `a` lands on
    /// `b` to a few ulps at every angle the decision admits (read off
    /// `a·b`, `1 + a·b` would carry an ulp absolute, and turn `a` off
    /// `b` by about `u/√(1 + a·b)`).
    ///
    /// Its one decision is `1 + a·b > 0`, under `site`: the margin that
    /// degrades as the pair turns toward opposed, where every axis
    /// perpendicular to them is a minimal rotation and none is THE
    /// one. A dimensionless margin against the linear band (ledger row
    /// F22). The carry needs no threshold of its own: the miss is a
    /// few `u` plus `(|a| − |b|)²/(2|a × b|)`, of order `u²/√ε` for a
    /// margin the band decides positive, far below `ε`.
    ///
    /// Evaluation order (fixed, D9): `h = a + b`, `d = (h·h)/2`,
    /// `k = a × h`, `n² = d² + kx² + ky² + kz²`, the columns of the
    /// quaternion matrix of `(d, k)` over `n²`, then the translation
    /// `q − R·q` for `q = about − Point3::origin()`.
    ///
    /// # Errors
    ///
    /// [`TurnError::Opposed`] when `1 + a·b` decides zero or negative,
    /// [`TurnError::Undecided`] when it cannot be decided, and
    /// [`TurnError::NonFinite`] naming [`MotionPart::Anchor`] for an
    /// `about` or a translation that is not finite.
    pub fn turned_between(
        a: UnitVec3<T>,
        b: UnitVec3<T>,
        about: Point3<T>,
        site: &'static str,
        band: Band,
    ) -> Result<Self, TurnError> {
        let (a, b) = (a.get(), b.get());
        let two = T::one() + T::one();
        let h = a + b;
        let d = h.dot(h) / two;
        let k = a.cross(h);
        // Ledger row F22: `1 + a·b` is a dimensionless defect of the
        // pair against the metre band; the turn has no arm of its own.
        match crate::k_stats::decide_flagged(site, d, band, "F22") {
            Ok(Sign::Positive) => {}
            Ok(Sign::Zero | Sign::Negative) => return Err(TurnError::Opposed),
            Err(source) => return Err(TurnError::Undecided(source)),
        }
        let (x, y, z) = (k.x, k.y, k.z);
        let (dd, xx, yy, zz) = (d.powi(2), x.powi(2), y.powi(2), z.powi(2));
        let n2 = dd + xx + yy + zz;
        let r = Mat3::from_cols(
            Vec3::new(
                dd + xx - yy - zz,
                two * (x * y + d * z),
                two * (x * z - d * y),
            ) / n2,
            Vec3::new(
                two * (x * y - d * z),
                dd - xx + yy - zz,
                two * (y * z + d * x),
            ) / n2,
            Vec3::new(
                two * (x * z + d * y),
                two * (y * z - d * x),
                dd - xx - yy + zz,
            ) / n2,
        );
        let anchor = |_| TurnError::NonFinite(MotionPart::Anchor);
        let q = finite(about, MotionPart::Anchor).map_err(anchor)? - Point3::origin();
        let origin = finite(at(q - r * q), MotionPart::Anchor).map_err(anchor)?;
        Ok(Self::from_linear(r, origin))
    }
}

/// **Why no minimal rotation carries one direction onto another**
/// ([`RigidMotion::turned_between`]).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TurnError {
    /// `1 + a·b` decided zero or below: the two directions are
    /// opposed, so every axis perpendicular to them turns one onto the
    /// other and no rotation is THE minimal one.
    Opposed,
    /// `1 + a·b` could not be decided; the decision's own diagnostic.
    Undecided(Indeterminate),
    /// The axis's anchor point, or the translation it makes, is not a
    /// finite number.
    NonFinite(MotionPart),
}

impl core::fmt::Display for TurnError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Opposed => write!(
                f,
                "the two directions are opposed, so no one minimal rotation turns one onto the \
                 other. Recourse: {TURN_RECOURSE}"
            ),
            Self::Undecided(source) => source
                .undecided(
                    "whether the two directions are opposed",
                    source.lever_ending(TURN_RECOURSE),
                )
                .fmt(f),
            Self::NonFinite(part) => write!(
                f,
                "a rigid motion's {} is not a finite number. Recourse: {}",
                part.noun(),
                crate::predicate::RANGE_RECOURSE
            ),
        }
    }
}

impl std::error::Error for TurnError {}

/// The recourse every opposed-turn refusal ends with.
const TURN_RECOURSE: &str = "turn through an intermediate direction, or state the axis";

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::interval::Interval;
    use crate::real::Bounds;
    use crate::tolerance::Tol;

    pub(super) const SITE: &str = "rigid_motion_test_axis";

    /// The witness tolerance's linear band, shared by this module's
    /// rows and `motion_rows`.
    pub(super) fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// A map's twelve components' bits, in [`Affine3::components`]'s
    /// order.
    pub(super) fn bits12(a: &Affine3<f64>) -> [u64; 12] {
        a.components().map(f64::to_bits)
    }

    fn cols(f: RigidMotion<f64>) -> [Vec3<f64>; 3] {
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
            let f = RigidMotion::gram_schmidt(o, u_raw, v_raw, SITE, band()).unwrap();
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
                RigidMotion::from_axis_and_reference(o, axis_raw, ref_raw, SITE, band()).unwrap();
            let aim = axis_raw.normalize();
            let x = (ref_raw - aim * ref_raw.dot(aim)).normalize();
            let hand =
                Affine3::from_parts(Mat3::from_cols(x, aim.cross(x), aim), o - Point3::origin());
            assert_eq!(bits12(&f.to_affine()), bits12(&hand), "from_axis at {o:?}");
        }
    }

    /// **An aim-minted frame's `w` is the aim, not `u × v`** — the
    /// distinction the module docs and [`RigidMotion::to_affine`] state,
    /// asserted where it is visible: over the corpus the two differ in
    /// at least one component's bits for at least one frame, so a doc
    /// that promised `w = u × v` for every mint would be false.
    #[test]
    fn the_aim_mints_store_the_aim_rather_than_the_cross_product() {
        let mut differed = 0_u32;
        for (o, axis_raw, ref_raw) in decided_corpus() {
            let f =
                RigidMotion::from_axis_and_reference(o, axis_raw, ref_raw, SITE, band()).unwrap();
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
            "no aim-minted frame in the corpus separated w from u × v, so this row cannot witness the distinction its subject is about"
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
                let f = RigidMotion::gram_schmidt(o, u_raw, v_raw, SITE, band()).unwrap();
                let got = cols(f).map(|c| [c.x, c.y, c.z].map(f64::to_bits));
                let want =
                    [u_raw, v_raw, u_raw.cross(v_raw)].map(|c| [c.x, c.y, c.z].map(f64::to_bits));
                assert_eq!(got, want, "at {u_raw:?} {v_raw:?}");
            }
        }
    }

    /// **A quarter turn about `x̂` IS the frame Gram–Schmidt decides
    /// from `(x̂, ẑ)`** — bit for bit at `f64`, signed zeros included,
    /// and as the same exact points at `Interval` — so a caller that
    /// means that plane writes the turn and pays no decision. The turn
    /// is [`RigidMotion::rotation_by`]'s linear part with the origin
    /// stored verbatim.
    #[test]
    fn a_quarter_turn_about_x_is_the_decided_x_z_frame() {
        let o = Point3::new(-0.0, 2.5, -7.0);
        let placed = RigidMotion::turned(o, UnitVec3::exact_x(), Turns::quarters(1));
        let by =
            RigidMotion::rotation_by(UnitVec3::exact_x(), Angle::from_turns(Turns::quarters(1)))
                .unwrap();
        assert_eq!(
            bits12(&placed.to_affine())[..9],
            bits12(&by.to_affine())[..9],
            "the linear part is rotation_by's"
        );
        assert_eq!(
            [placed.origin().x, placed.origin().y, placed.origin().z].map(f64::to_bits),
            [o.x, o.y, o.z].map(f64::to_bits),
            "the origin is stored verbatim"
        );
        let turned = RigidMotion::turned(Point3::origin(), UnitVec3::exact_x(), Turns::quarters(1));
        let decided = RigidMotion::gram_schmidt(
            Point3::origin(),
            Vec3::unit_x(),
            Vec3::unit_z(),
            SITE,
            band(),
        )
        .unwrap();
        assert_eq!(
            bits12(&turned.to_affine()),
            bits12(&decided.to_affine()),
            "f64"
        );
        let turned = RigidMotion::<Interval>::turned(
            Point3::origin(),
            UnitVec3::exact_x(),
            Turns::quarters(1),
        );
        let decided = RigidMotion::gram_schmidt(
            Point3::origin(),
            Vec3::unit_x().map(Interval::from_f64),
            Vec3::unit_z().map(Interval::from_f64),
            SITE,
            band(),
        )
        .unwrap();
        let ends =
            |a: Affine3<Interval>| a.components().map(|x| (x.lo().to_bits(), x.hi().to_bits()));
        assert_eq!(
            ends(turned.to_affine()),
            ends(decided.to_affine()),
            "Interval"
        );
    }

    /// **Every frame whose axes are signed basis vectors is a product of
    /// quarter turns about the exact axes**, with entries `0` and `±1`
    /// exactly and the axes Gram–Schmidt would decide from its own
    /// `(u, v)`: the 64 products `R_z^a ∘ R_y^b ∘ R_x^c` reach all 24.
    #[test]
    fn every_signed_basis_frame_is_a_product_of_quarter_turns() {
        let vals = |f: RigidMotion<f64>| cols(f).map(|v| [v.x, v.y, v.z]);
        let mut reached: Vec<[[f64; 3]; 3]> = Vec::new();
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    let turn =
                        |axis, n| RigidMotion::turned(Point3::origin(), axis, Turns::quarters(n));
                    let f = turn(UnitVec3::exact_z(), a)
                        .compose(
                            turn(UnitVec3::exact_y(), b)
                                .compose(turn(UnitVec3::exact_x(), c))
                                .unwrap(),
                        )
                        .unwrap();
                    let axes = vals(f);
                    for x in axes.iter().flatten() {
                        assert!(
                            *x == 0.0 || x.abs() == 1.0,
                            "an entry {x} of R_z^{a} R_y^{b} R_x^{c} is not exact"
                        );
                    }
                    let decided = RigidMotion::gram_schmidt(
                        Point3::origin(),
                        f.u().get(),
                        f.v().get(),
                        SITE,
                        band(),
                    )
                    .unwrap();
                    assert_eq!(vals(decided), axes, "R_z^{a} R_y^{b} R_x^{c}");
                    if !reached.contains(&axes) {
                        reached.push(axes);
                    }
                }
            }
        }
        assert_eq!(reached.len(), 24, "the signed basis frames reached");
    }

    /// The three world frames ARE the exact basis vectors, in the
    /// cyclic order their names say, with the origin stored bitwise.
    #[test]
    fn the_world_frames_are_the_exact_basis_triples() {
        let o = Point3::new(-0.0, 2.0, f64::MIN_POSITIVE);
        let bits = |v: [Vec3<f64>; 3]| v.map(|c| [c.x, c.y, c.z].map(f64::to_bits));
        assert_eq!(
            bits(cols(RigidMotion::axes_xy(o))),
            bits([Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()])
        );
        assert_eq!(
            bits(cols(RigidMotion::axes_yz(o))),
            bits([Vec3::unit_y(), Vec3::unit_z(), Vec3::unit_x()])
        );
        assert_eq!(
            bits(cols(RigidMotion::axes_zx(o))),
            bits([Vec3::unit_z(), Vec3::unit_x(), Vec3::unit_y()])
        );
        assert_eq!(
            bits12(&RigidMotion::axes_xy(o).to_affine())[9..],
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
                RigidMotion::gram_schmidt(o, Vec3::unit_x(), v_raw, SITE, band()).unwrap_err(),
                OrthonormalizeError {
                    axis: OrthoAxis::V,
                    error: UnitVec3Error::Degenerate,
                },
                "at {v_raw:?}"
            );
        }
        assert_eq!(
            RigidMotion::gram_schmidt(o, Vec3::new(0.0, 0.0, 0.0), Vec3::unit_y(), SITE, band())
                .unwrap_err(),
            OrthonormalizeError {
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
        let f = RigidMotion::from_aim(Point3::new(1.0, 2.0, 3.0), aim, perp, SITE, band()).unwrap();
        let x = perp.normalize();
        assert_eq!(
            cols(f).map(|c| [c.x, c.y, c.z].map(f64::to_bits)),
            [x, aim.get().cross(x), aim.get()].map(|c| [c.x, c.y, c.z].map(f64::to_bits))
        );
        assert_eq!(
            RigidMotion::from_aim(
                Point3::origin(),
                aim,
                Vec3::new(0.0, 0.0, 0.0),
                SITE,
                band()
            )
            .unwrap_err(),
            OrthonormalizeError {
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
            RigidMotion::from_axis_and_reference(o, axis, long_on_axis, SITE, band()).unwrap_err(),
            OrthonormalizeError {
                axis: OrthoAxis::V,
                error: UnitVec3Error::Degenerate,
            }
        );
        assert_eq!(
            RigidMotion::from_axis_and_reference(
                o,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::unit_x(),
                SITE,
                band()
            )
            .unwrap_err(),
            OrthonormalizeError {
                axis: OrthoAxis::U,
                error: UnitVec3Error::Degenerate,
            }
        );
        // The axis is kept as `w` and the reference yields, which is
        // the role swap the Gram-Schmidt door does not do.
        let f =
            RigidMotion::from_axis_and_reference(o, axis, Vec3::new(2.0, 0.0, 7.0), SITE, band())
                .unwrap();
        let bits = |v: Vec3<f64>| [v.x, v.y, v.z].map(f64::to_bits);
        assert_eq!(bits(f.w().get()), bits(Vec3::unit_z()));
        assert_eq!(bits(f.u().get()), bits(Vec3::unit_x()));
        assert_eq!(bits(f.v().get()), bits(Vec3::unit_y()));
    }
}

/// The motion's own rows: the mints by construction, the decided
/// door, and the action, at `f64` and at `Interval`.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod motion_rows {
    use super::tests::{SITE, band, bits12};
    use super::*;
    use crate::interval::Interval;
    use crate::real::Bounds;

    fn unit(v: Vec3<f64>) -> UnitVec3<f64> {
        UnitVec3::new(v, SITE, band()).unwrap()
    }

    /// **A composition is `Affine3`'s product, bit for bit**, so a
    /// chain of placements typed as witnesses moves a body by exactly
    /// the bits the untyped product did — over rotations, translations
    /// with a `−0.0` component and the half-turns.
    #[test]
    fn compose_is_the_affine_product_bit_for_bit() {
        let axis = unit(Vec3::new(0.3, -0.7, 0.2));
        let motions = [
            RigidMotion::rotation(axis, 1.234).unwrap(),
            RigidMotion::translation(Vec3::new(-0.0, 2.5, -1e-3)).unwrap(),
            RigidMotion::rotation_about_axis(Point3::new(1.0, -2.0, 0.5), -axis, 0.4).unwrap(),
            RigidMotion::rotation_then_translation(axis, -2.0, Vec3::new(3.0, 0.0, -0.0)).unwrap(),
            RigidMotion::half_turn_x(),
            RigidMotion::half_turn_y(),
            RigidMotion::half_turn_z(),
        ];
        for a in motions {
            for b in motions {
                let typed = a.compose(b).unwrap().to_affine();
                let untyped = a.to_affine() * b.to_affine();
                assert_eq!(bits12(&typed), bits12(&untyped), "{a:?} ∘ {b:?}");
            }
        }
    }

    /// **The rotation mints are `Affine3`'s constructions over the
    /// witness as given**: on an axis the format holds exactly unit,
    /// where normalizing changes no bit, the typed and untyped
    /// spellings agree bit for bit.
    #[test]
    fn the_rotation_mints_are_the_affine_constructions_on_an_exact_axis() {
        let z = unit(Vec3::unit_z());
        for angle in [0.0, 0.3, -1.7, core::f64::consts::PI, 5.0] {
            let p = Point3::new(1.5, -0.25, 3.0);
            assert_eq!(
                bits12(
                    &RigidMotion::rotation_about_axis(p, z, angle)
                        .unwrap()
                        .to_affine()
                ),
                bits12(&Affine3::rotation_about_axis(p, Vec3::unit_z(), angle)),
                "about an axis through {p:?} at {angle}"
            );
            assert_eq!(
                bits12(&RigidMotion::rotation(z, angle).unwrap().to_affine()),
                bits12(&Affine3::from_parts(
                    Mat3::rotation_about(Vec3::unit_z(), angle),
                    Vec3::zero()
                )),
                "about z at {angle}"
            );
        }
    }

    /// **The inverse is the transpose, and it is exact on the
    /// half-turns**: `m ∘ m⁻¹` is the identity bit for bit there, at
    /// `f64` — and at `Interval` a widened rotation's `m ∘ m⁻¹`
    /// encloses the identity, entry by entry.
    #[test]
    fn the_transpose_inverse_is_exact_on_the_half_turns_and_encloses_at_interval() {
        let identity = bits12(&Affine3::identity());
        for m in [
            RigidMotion::half_turn_x(),
            RigidMotion::half_turn_y(),
            RigidMotion::half_turn_z(),
        ] {
            let back = m.compose(m.inverse().unwrap()).unwrap().to_affine();
            assert_eq!(bits12(&back), identity, "{m:?} ∘ {m:?}⁻¹");
        }
        let axis = UnitVec3::new(
            Vec3::new(0.3, -0.7, 0.2).map(Interval::from_f64),
            "rigid_motion_test_axis",
            band(),
        )
        .unwrap();
        let angle = Interval::from_bounds(0.4, 0.43);
        let m = RigidMotion::rotation_about_axis(
            Point3::new(1.0, 2.0, -3.0).map(Interval::from_f64),
            axis,
            angle,
        )
        .unwrap();
        let inverse = m.inverse().unwrap();
        // The transpose is field shuffling: the inverse's linear part is
        // the operand's, transposed, bound for bound — no wider.
        let ends = |x: Interval| (x.lo().to_bits(), x.hi().to_bits());
        let (got, want) = (inverse.linear(), m.linear().transpose());
        for (k, (g, w)) in got.cols().iter().zip(want.cols()).enumerate() {
            assert_eq!(
                [g.x, g.y, g.z].map(ends),
                [w.x, w.y, w.z].map(ends),
                "column {k} of the inverse is the operand's row, bound for bound"
            );
        }
        let width = |x: Interval| x.hi() - x.lo();
        let widest = m
            .linear()
            .cols()
            .iter()
            .flat_map(|c| [c.x, c.y, c.z])
            .map(width)
            .fold(0.0_f64, f64::max);
        let back = m.compose(inverse).unwrap().to_affine();
        for (k, got) in back
            .linear
            .cols()
            .iter()
            .flat_map(|c| [c.x, c.y, c.z])
            .enumerate()
        {
            assert!(
                width(got) <= 8.0 * widest,
                "entry {k} of m ∘ m⁻¹ is {} wide, past eight times m's widest entry {widest}",
                width(got)
            );
        }
        for (k, (got, want)) in back
            .components()
            .into_iter()
            .zip(Affine3::<f64>::identity().components())
            .enumerate()
        {
            assert!(
                got.lo() <= want && want <= got.hi(),
                "component {k} of m ∘ m⁻¹ must enclose the identity's {want}: {got:?}"
            );
        }
    }

    /// **The decided door refuses a raw shear or scale**, naming the
    /// check, and it RETURNS AN UNDECIDED MARGIN AS UNDECIDED: an
    /// interval column norm straddling the band is not a decided
    /// defect. A rigid map passes, and comes back carrying its bits.
    #[test]
    fn the_decided_door_refuses_shear_and_scale_and_keeps_undecided_apart() {
        let shear = Affine3::from_parts(
            Mat3::from_cols(Vec3::unit_x(), Vec3::new(0.1, 1.0, 0.0), Vec3::unit_z()),
            Vec3::zero(),
        );
        assert!(matches!(
            RigidMotion::check_rigid(&shear, band()),
            Err(RigidMotionError::NotRigid {
                check: "transform_rigid_col1_unit"
            })
        ));
        let skew = Affine3::from_parts(
            Mat3::from_cols(Vec3::unit_x(), Vec3::new(0.6, 0.8, 0.0), Vec3::unit_z()),
            Vec3::zero(),
        );
        assert!(matches!(
            RigidMotion::check_rigid(&skew, band()),
            Err(RigidMotionError::NotRigid {
                check: "transform_rigid_col01_orth"
            })
        ));
        let scale = Affine3::from_parts(
            Mat3::from_cols(Vec3::new(2.0, 0.0, 0.0), Vec3::unit_y(), Vec3::unit_z()),
            Vec3::zero(),
        );
        assert!(matches!(
            RigidMotion::check_rigid(&scale, band()),
            Err(RigidMotionError::NotRigid {
                check: "transform_rigid_col0_unit"
            })
        ));
        let (zero, escalate) = (band().zero(), band().escalate());
        let straddle = Affine3::from_parts(
            Mat3::from_cols(
                Vec3::new(
                    Interval::from_bounds(1.0 - escalate, 1.0 + escalate),
                    Interval::from_f64(0.0),
                    Interval::from_f64(0.0),
                ),
                Vec3::unit_y().map(Interval::from_f64),
                Vec3::unit_z().map(Interval::from_f64),
            ),
            Vec3::zero().map(Interval::from_f64),
        );
        assert!(
            matches!(
                RigidMotion::check_rigid(&straddle, band()),
                Err(RigidMotionError::Undecided {
                    check: "transform_rigid_col0_unit",
                    ..
                })
            ),
            "a column norm straddling ({zero}, {escalate}) is undecided, not decided off unit"
        );
        let turn = Affine3::rotation_about_axis(
            Point3::new(-0.0, 1.0, 2.0),
            Vec3::new(1.0, 1.0, 0.0),
            0.9,
        );
        let admitted = RigidMotion::check_rigid(&turn, band()).expect("a rotation is rigid");
        assert_eq!(
            bits12(&admitted.to_affine()),
            bits12(&turn),
            "carried verbatim"
        );
    }

    /// **A non-finite linear entry is the caller's input, refused by
    /// name before any margin is decided**, in every column and at an
    /// enclosure, so its refusal never reads as a poisoned margin.
    #[test]
    fn a_non_finite_linear_entry_is_refused_as_the_input() {
        for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            for column in 0..3 {
                let mut cols = [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()];
                cols[column].y = bad;
                let map =
                    Affine3::from_parts(Mat3::from_cols(cols[0], cols[1], cols[2]), Vec3::zero());
                assert_eq!(
                    RigidMotion::check_rigid(&map, band()).map(|_| ()),
                    Err(RigidMotionError::NonFinite(MotionPart::Linear)),
                    "{bad} in column {column}"
                );
            }
        }
        let unbounded = Affine3::from_parts(
            Mat3::from_cols(
                Vec3::new(
                    Interval::from_bounds(1.0, f64::INFINITY),
                    Interval::from_f64(0.0),
                    Interval::from_f64(0.0),
                ),
                Vec3::unit_y().map(Interval::from_f64),
                Vec3::unit_z().map(Interval::from_f64),
            ),
            Vec3::zero().map(Interval::from_f64),
        );
        assert_eq!(
            RigidMotion::check_rigid(&unbounded, band()).map(|_| ()),
            Err(RigidMotionError::NonFinite(MotionPart::Linear)),
            "an enclosure with an unbounded endpoint"
        );
    }

    /// **An undecided rigidity check ends in the map's lever alone**, a
    /// residual being no size the user chose, and a poisoned margin adds
    /// the unreadable-margin note after it. The poisoned margin is
    /// injected through the funnel: past the door's finiteness probe
    /// none is reachable at f64.
    #[test]
    fn an_undecided_check_ends_in_the_lever_and_notes_a_poisoned_margin() {
        let check = "transform_rigid_col0_unit";
        let render = |margin: f64| {
            let source =
                crate::k_stats::decide(check, crate::predicate::Margin::of(margin), band())
                    .unwrap_err();
            RigidMotionError::Undecided { check, source }.to_string()
        };
        let in_band = render(0.5 * (band().zero() + band().escalate()));
        assert!(
            in_band.ends_with(". Recourse: use only a rotation and a translation"),
            "in band: the lever alone: {in_band}"
        );
        let poisoned = render(f64::NAN);
        assert!(
            poisoned.ends_with(&format!(
                ". Recourse: use only a rotation and a translation; {}",
                crate::predicate::UNREADABLE_MARGIN_NOTE
            )),
            "poisoned: the lever, then the note: {poisoned}"
        );
    }

    /// **A rotation by a widened angle is a witness, where the decided
    /// door cannot read one off its entries**: `cos` and `sin` of the
    /// angle box are independent brackets, so `c0·c1` encloses about
    /// `±2·sin h` and the door refuses it, undecided; the mint carries
    /// the rotation and decides only that the angle is a number.
    #[test]
    fn a_widened_angle_mints_where_its_entries_cannot_be_decided() {
        let z = UnitVec3::new(
            Vec3::unit_z().map(Interval::from_f64),
            "rigid_motion_test_axis",
            band(),
        )
        .unwrap();
        let angle = Interval::from_bounds(-0.03, 0.03);
        let m = RigidMotion::rotation(z, angle).expect("a finite angle mints");
        assert!(matches!(
            RigidMotion::check_rigid(&m.to_affine(), band()),
            Err(RigidMotionError::Undecided { .. })
        ));
    }

    /// **Every mint by construction decides finiteness, and names what
    /// was not finite.**
    #[test]
    fn the_mints_refuse_what_is_not_finite_by_name() {
        let z = unit(Vec3::unit_z());
        let nonfinite = |r: Result<RigidMotion<f64>, RigidMotionError>| match r {
            Err(RigidMotionError::NonFinite(part)) => part,
            other => panic!("a non-finite input must refuse: {other:?}"),
        };
        assert_eq!(
            nonfinite(RigidMotion::rotation(z, f64::NAN)),
            MotionPart::Angle
        );
        assert_eq!(
            nonfinite(RigidMotion::translation(Vec3::new(0.0, f64::INFINITY, 0.0))),
            MotionPart::Translation
        );
        assert_eq!(
            nonfinite(RigidMotion::rotation_about_axis(
                Point3::new(f64::NAN, 0.0, 0.0),
                z,
                1.0
            )),
            MotionPart::Anchor
        );
        let far = RigidMotion::translation(Vec3::new(f64::MAX, 0.0, 0.0)).unwrap();
        assert_eq!(nonfinite(far.compose(far)), MotionPart::Product);
    }

    /// **Each mint decides its own parts**, one row a mint, so a check
    /// dropped from any one of them reds here and not only through a
    /// sibling that shares its body.
    #[test]
    fn each_mint_names_its_own_non_finite_part() {
        let z = unit(Vec3::unit_z());
        let nonfinite = |r: Result<RigidMotion<f64>, RigidMotionError>| match r {
            Err(RigidMotionError::NonFinite(part)) => part,
            other => panic!("a non-finite part must refuse: {other:?}"),
        };
        assert_eq!(
            nonfinite(RigidMotion::rotation_then_translation(
                z,
                f64::NAN,
                Vec3::zero()
            )),
            MotionPart::Angle,
            "rotation_then_translation's angle"
        );
        assert_eq!(
            nonfinite(RigidMotion::rotation_then_translation(
                z,
                1.0,
                Vec3::new(f64::INFINITY, 0.0, 0.0)
            )),
            MotionPart::Translation,
            "rotation_then_translation's translation"
        );
        assert_eq!(
            nonfinite(RigidMotion::rotation_about_axis(
                Point3::origin(),
                z,
                f64::NAN
            )),
            MotionPart::Angle,
            "rotation_about_axis's angle"
        );
        // A finite turn and a finite translation whose transposed
        // product overflows: `Rᵀ·t` sums two components near `MAX`.
        let m = RigidMotion::rotation_then_translation(
            z,
            core::f64::consts::FRAC_PI_4,
            Vec3::new(f64::MAX, f64::MAX, 0.0),
        )
        .unwrap();
        assert_eq!(nonfinite(m.inverse()), MotionPart::Inverse, "the inverse");
    }

    /// **At `Interval` the mints decide finiteness of the enclosure**:
    /// an overflow to `[MAX, ∞]` refuses at the mint, naming the part,
    /// rather than reaching the body door as an unbounded witness.
    #[test]
    fn an_interval_overflow_refuses_at_the_mint() {
        let zero = Interval::from_f64(0.0);
        let unbounded = Interval::from_bounds(f64::MAX, f64::INFINITY);
        assert_eq!(
            RigidMotion::translation(Vec3::new(unbounded, zero, zero)).map(|_| ()),
            Err(RigidMotionError::NonFinite(MotionPart::Translation))
        );
        let far = RigidMotion::translation(Vec3::new(Interval::from_f64(f64::MAX), zero, zero))
            .expect("a finite translation");
        assert_eq!(
            far.compose(far).map(|_| ()),
            Err(RigidMotionError::NonFinite(MotionPart::Product))
        );
    }

    /// **The decided door asks finiteness as the mints do**: at
    /// `Interval` an unbounded translation enclosure refuses at the door
    /// exactly where it refuses at `translation`, though `0·t` of each
    /// is the finite `[0, 0]`. A finite one is admitted by both.
    #[test]
    fn the_door_refuses_an_unbounded_translation_as_the_mint_does() {
        let one = Interval::from_f64(1.0);
        let zero = Interval::from_f64(0.0);
        let id = Mat3::from_cols(
            Vec3::new(one, zero, zero),
            Vec3::new(zero, one, zero),
            Vec3::new(zero, zero, one),
        );
        let refused = Err(RigidMotionError::NonFinite(MotionPart::Translation));
        for t in [
            Interval::from_bounds(f64::MAX, f64::INFINITY),
            Interval::from_bounds(0.0, f64::INFINITY),
            Interval::from_bounds(f64::NEG_INFINITY, f64::INFINITY),
        ] {
            for at in [
                Vec3::new(t, zero, zero),
                Vec3::new(zero, t, zero),
                Vec3::new(zero, zero, t),
            ] {
                let map = Affine3::from_parts(id, at);
                assert_eq!(
                    RigidMotion::check_rigid(&map, band()).map(|_| ()),
                    refused,
                    "{at:?}"
                );
                assert_eq!(RigidMotion::translation(at).map(|_| ()), refused, "{at:?}");
            }
        }
        let far = Vec3::new(Interval::from_bounds(-f64::MAX, f64::MAX), zero, zero);
        assert!(RigidMotion::check_rigid(&Affine3::from_parts(id, far), band()).is_ok());
        assert!(RigidMotion::translation(far).is_ok());
        let map = Affine3::from_parts(Mat3::identity(), Vec3::new(f64::INFINITY, 0.0, 0.0));
        assert_eq!(
            RigidMotion::check_rigid(&map, band()).map(|_| ()),
            Err(RigidMotionError::NonFinite(MotionPart::Translation))
        );
    }

    /// **A product of door-admitted factors is rigid within about one
    /// band a factor**, and is not decided again: a stretch whose column
    /// margin is `0.8` of the band's zero is admitted, the `k`-fold
    /// product's margin stays within `k` zeros, and past the band's
    /// escalation the door refuses the product it never asked about.
    #[test]
    fn a_product_of_admitted_factors_drifts_by_a_band_a_factor() {
        let (zero, escalate) = (band().zero(), band().escalate());
        let s = 1.0 + 0.4 * zero;
        let near = Affine3::from_parts(
            Mat3::from_cols(Vec3::new(s, 0.0, 0.0), Vec3::unit_y(), Vec3::unit_z()),
            Vec3::zero(),
        );
        let one = RigidMotion::check_rigid(&near, band()).expect("inside the band: admitted");
        let mut product = one;
        let mut refused_from = None;
        for k in 2..=64_u32 {
            product = product.compose(one).unwrap();
            let c0 = product.u().get();
            let defect = (c0.dot(c0) - 1.0).abs();
            assert!(
                defect <= f64::from(k) * zero,
                "the {k}-fold product's column margin {defect:e} is past {k} zeros"
            );
            let refused = matches!(
                RigidMotion::check_rigid(&product.to_affine(), band()),
                Err(RigidMotionError::NotRigid {
                    check: "transform_rigid_col0_unit"
                })
            );
            if refused && refused_from.is_none() {
                refused_from = Some(k);
            }
        }
        let k = refused_from.expect("past the escalation the door refuses the product");
        assert!(
            0.8 * f64::from(k) * zero >= 0.9 * escalate,
            "the door refuses the {k}-fold product only once its margin nears the escalation"
        );
    }

    /// **The action on a direction is the linear part on its vector**,
    /// and a witness comes back.
    #[test]
    fn a_motion_turns_a_direction_into_a_direction() {
        let m = RigidMotion::rotation(unit(Vec3::new(1.0, 2.0, -0.5)), 0.8).unwrap();
        let d = unit(Vec3::new(-0.2, 0.4, 1.0));
        let turned = m.turn(d).get();
        let by_hand = m.to_affine().transform_vec(d.get());
        assert_eq!(
            [turned.x, turned.y, turned.z].map(f64::to_bits),
            [by_hand.x, by_hand.y, by_hand.z].map(f64::to_bits)
        );
    }
}

/// The scalar crossings and the turn between two directions.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod crossing_and_turn_rows {
    use super::tests::{SITE, band, bits12};
    use super::*;
    use crate::interval::Interval;

    fn unit(v: Vec3<f64>) -> UnitVec3<f64> {
        UnitVec3::new(v, SITE, band()).unwrap()
    }

    fn turn(
        a: UnitVec3<f64>,
        b: UnitVec3<f64>,
        about: Point3<f64>,
    ) -> Result<RigidMotion<f64>, TurnError> {
        RigidMotion::turned_between(a, b, about, SITE, band())
    }

    /// **The turn carries `a` onto `b`, is rigid, and leaves its anchor
    /// where it was**, over pairs from a few degrees to a few degrees
    /// short of opposed, and is the identity bit for bit on a pair that
    /// is one direction.
    #[test]
    fn the_turn_carries_one_direction_onto_the_other_about_its_anchor() {
        let about = Point3::new(0.75, -2.5, 1e3);
        for k in 0..48 {
            let t = 0.05 + f64::from(k) * 0.0645;
            let a = unit(Vec3::new(t.cos(), 0.4 * t.sin(), 0.3));
            let b = unit(Vec3::new((2.0 * t).sin(), -t.cos(), 0.5 - t));
            let m = turn(a, b, about).unwrap_or_else(|e| panic!("pair {k}: {e}"));
            let carried = m.turn(a).get() - b.get();
            assert!(
                carried.norm() < 1e-14,
                "pair {k}: a lands {carried:?} off b"
            );
            assert!(
                RigidMotion::check_rigid(&m.to_affine(), band()).is_ok(),
                "pair {k}: the turn is rigid"
            );
            let moved = m.transform_point(about) - about;
            assert!(moved.norm() < 1e-12, "pair {k}: the anchor moved {moved:?}");
        }
        let a = unit(Vec3::new(0.3, -0.7, 0.2));
        let same = turn(a, a, about).unwrap();
        assert_eq!(
            bits12(&same.to_affine()),
            bits12(&RigidMotion::identity().to_affine()),
            "a pair that is one direction turns by the identity, with no rounding"
        );
    }

    /// **The one decision is `1 + a·b`, on the band's own thresholds**:
    /// an opposed pair refuses decided, a pair within the band of
    /// opposed refuses undecided, and one past the band builds.
    #[test]
    fn a_pair_near_opposed_refuses_by_where_its_divisor_lies_in_the_band() {
        let b = band();
        // 1 + a·b = 1 − cos δ ≈ δ²/2 for b at π − δ from a.
        let at = |margin: f64| {
            let d = (2.0 * margin).sqrt();
            unit(Vec3::new(-d.cos(), d.sin(), 0.0))
        };
        let x = UnitVec3::exact_x();
        let origin = Point3::origin();
        let opposed = UnitVec3::by_construction(Vec3::new(-1.0, 0.0, 0.0));
        assert_eq!(turn(x, opposed, origin).err(), Some(TurnError::Opposed));
        assert_eq!(
            turn(x, at(0.25 * b.zero()), origin).err(),
            Some(TurnError::Opposed),
            "inside the coincidence threshold the pair is opposed"
        );
        assert!(
            matches!(
                turn(x, at(0.5 * (b.zero() + b.escalate())), origin),
                Err(TurnError::Undecided(_))
            ),
            "between the thresholds the turn cannot be decided"
        );
        assert!(
            turn(x, at(4.0 * b.escalate()), origin).is_ok(),
            "past the escalation threshold the turn builds"
        );
    }

    /// **Up to the band's escalation edge, the turn is rigid and carries
    /// `a` onto `b` to rounding**, at `f64` and at `Interval`: over six
    /// generic pairs at `1 + a·b` from `0.5` down to four escalations,
    /// every turn builds, the rigidity door admits it at both scalars,
    /// its seven rigidity defects stay below `1e-14`, and `a` lands
    /// within `1e-14` of `b` (at `Interval`, the enclosure of the miss
    /// within `1e-13`). Neither bound depends on how near opposed the
    /// pair is or on the run's ε, so the row holds at every ε row.
    #[test]
    fn near_opposed_the_turn_is_rigid_and_carries_a_onto_b_to_rounding() {
        use crate::real::Bounds;
        let edge = 4.0 * band().escalate();
        let pairs = [
            (Vec3::new(0.36, -0.48, 0.8), Vec3::new(0.2, 0.9, -0.3)),
            (Vec3::new(1.0, 2.0, 3.0), Vec3::new(-0.7, 0.1, 0.4)),
            (Vec3::new(-0.3, 0.77, -0.1), Vec3::new(0.5, 0.5, 0.9)),
            (Vec3::new(0.9, 0.05, -0.6), Vec3::new(0.0, -1.0, 0.3)),
            (
                Vec3::new(0.123, 0.456, 0.789),
                Vec3::new(0.987, -0.654, 0.321),
            ),
            (Vec3::new(-2.0, 0.3, 1.1), Vec3::new(0.4, 0.4, -1.0)),
        ];
        let defects = |l: Mat3<f64>| {
            let (c0, c1, c2) = (l.c0, l.c1, l.c2);
            [
                c0.dot(c0) - 1.0,
                c1.dot(c1) - 1.0,
                c2.dot(c2) - 1.0,
                c0.dot(c1),
                c1.dot(c2),
                c0.dot(c2),
                l.determinant() - 1.0,
            ]
            .into_iter()
            .fold(0.0_f64, |m, x| m.max(x.abs()))
        };
        let mut rows = 0;
        let mut margin = 0.5_f64;
        while margin >= edge {
            // b at π − δ from a, 1 + a·b = 1 − cos δ ≈ δ²/2.
            let delta = (2.0 * margin).sqrt();
            for (k, (av, pv)) in pairs.into_iter().enumerate() {
                let a = unit(av);
                let p = unit(a.get().cross(pv));
                let b = unit(-(a.get() * delta.cos()) + p.get() * delta.sin());
                let at = format!("pair {k} at 1 + a·b ≈ {margin:e}");
                let m = turn(a, b, Point3::origin()).unwrap_or_else(|e| panic!("{at}: {e}"));
                let defect = defects(m.linear());
                assert!(defect < 1e-14, "{at}: rigidity defect {defect:e}");
                let miss = (m.turn(a).get() - b.get()).norm();
                assert!(miss < 1e-14, "{at}: a lands {miss:e} off b");
                assert!(
                    RigidMotion::check_rigid(&m.to_affine(), band()).is_ok(),
                    "{at}: the rigidity door admits the f64 turn"
                );
                // The same two directions at `Interval`, as point
                // enclosures: each is the aim of an `f64` frame, admitted
                // at `Interval` by the decided door, which keeps the
                // map's entries verbatim.
                let at_interval = |d: UnitVec3<f64>| {
                    let frame = RigidMotion::from_aim_and_reference(
                        Point3::origin(),
                        d,
                        Vec3::new(0.3, -0.5, 0.8),
                        SITE,
                        band(),
                    )
                    .unwrap_or_else(|e| panic!("{at}: a frame aims along the input: {e:?}"));
                    RigidMotion::check_rigid(&frame.to_affine().map(Interval::from_f64), band())
                        .unwrap_or_else(|e| panic!("{at}: the frame is rigid at Interval: {e}"))
                        .w()
                };
                let (ai, bi) = (at_interval(a), at_interval(b));
                let mi = RigidMotion::turned_between(ai, bi, Point3::origin(), SITE, band())
                    .unwrap_or_else(|e| panic!("{at}, Interval: {e}"));
                assert!(
                    RigidMotion::check_rigid(&mi.to_affine(), band()).is_ok(),
                    "{at}: the rigidity door admits the Interval turn"
                );
                let off = mi.turn(ai).get() - bi.get();
                let reach = [off.x, off.y, off.z]
                    .into_iter()
                    .fold(0.0_f64, |r, c| r.max(c.lo().abs()).max(c.hi().abs()));
                assert!(reach < 1e-13, "{at}, Interval: the miss reaches {reach:e}");
            }
            rows += 1;
            margin /= 10_f64.sqrt();
        }
        assert!(rows >= 8, "the ladder reaches near opposed: {rows} rungs");
    }

    /// **`translated` keeps the axes bit for bit and adds to the
    /// origin**, and refuses an origin that leaves the format.
    #[test]
    fn translated_moves_only_the_origin() {
        let m = RigidMotion::rotation(unit(Vec3::new(1.0, 2.0, -0.5)), 0.8).unwrap();
        let t = Vec3::new(-0.0, 3.25, -1e-3);
        let moved = m.translated(t).unwrap();
        let axes = |f: RigidMotion<f64>| {
            [f.u().get(), f.v().get(), f.w().get()].map(|c| [c.x, c.y, c.z].map(f64::to_bits))
        };
        assert_eq!(axes(moved), axes(m), "the axes are kept");
        let want = m.origin() + t;
        let got = moved.origin();
        assert_eq!(
            [got.x, got.y, got.z].map(f64::to_bits),
            [want.x, want.y, want.z].map(f64::to_bits)
        );
        assert_eq!(
            m.translated(Vec3::new(f64::MAX, 0.0, 0.0))
                .and_then(|f| f.translated(Vec3::new(f64::MAX, 0.0, 0.0)))
                .err(),
            Some(RigidMotionError::NonFinite(MotionPart::Translation))
        );
    }

    /// **`point_value` reads a frame minted at `Interval` from point
    /// inputs back to the `f64` frame, bit for bit**, and refuses a
    /// motion whose components are boxes. The decided door admits the
    /// map verbatim, so the frame it mints at `Interval` from the `f64`
    /// frame's point entries is that frame's point enclosure.
    #[test]
    fn point_value_reads_a_point_frame_and_a_box_has_no_point() {
        let m = RigidMotion::gram_schmidt(
            Point3::new(-0.0, 1e-300, 7.5),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(-1.0, 1.0, 2.0),
            SITE,
            band(),
        )
        .unwrap();
        let at_interval: RigidMotion<Interval> =
            RigidMotion::check_rigid(&m.to_affine().map(Interval::from_f64), band())
                .expect("a rigid f64 map is rigid as point intervals");
        let back = at_interval
            .point_value()
            .expect("a point motion has its f64 back");
        assert_eq!(bits12(&back.to_affine()), bits12(&m.to_affine()));
        let wide = RigidMotion::axes_xy(Point3::new(
            Interval::from_bounds(0.0, 1.0),
            Interval::from_f64(0.0),
            Interval::from_f64(0.0),
        ));
        assert!(
            wide.point_value().is_none(),
            "a box has no single f64 point"
        );
    }
}
