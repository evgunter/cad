//! The unit-vector **witness**: a 3-D direction whose unit length was
//! established by a decision, carried as a type so the fact survives a
//! function boundary instead of being re-stated as prose at every door
//! that needs it.
//!
//! # What the witness means, at every scalar
//!
//! A [`UnitVec3`] is a vector produced by a normalize whose length
//! DECIDED positive under a band — finite, not underflowed out of the
//! format, and definitely nonzero, or through [`UnitVec3::levered`]
//! nonzero once levered by the caller's arm, so a long arm admits a
//! norm below the band — or by an operation that maps a unit vector to
//! a unit vector exactly. It is not a bit-for-bit claim
//! `‖u‖ == 1`; it is the same kind of claim every decided fact in this
//! kernel makes, read per scalar:
//!
//! - at `f64`, the normalized value;
//! - at `Interval`, an enclosure of a unit vector — the widths interval
//!   arithmetic carries are the enclosure's, and the normalization ran
//!   on an enclosure whose length decided positive;
//! - at `Dual`, a unit value channel with the quotient rule's tangent
//!   carried alongside.
//!
//! # The mints, and only these
//!
//! - [`UnitVec3::new`] — the normalizing constructor: decide the length
//!   under the caller's band and funnel-site name, then divide. The one
//!   place a USER's vector becomes a direction, with typed refusals.
//! - [`UnitVec3::levered`] — the same mint for a vector whose length is
//!   a pure number rather than metres (a carrier's unit-at-rest normal
//!   or axis): its length is decided levered by an arm, a positive
//!   length the caller names, with its own refusals
//!   ([`LeveredUnitError`]).
//! - [`UnitVec3::squared`] — a direction squared with no decision, poison
//!   where its length is zero; [`UnitVec3::chord`] is the chord's
//!   spelling of it, `v / |v|` beside `|v|`.
//! - **The construction doors**, each naming the construction that makes
//!   its value unit over the reals: [`UnitVec3::through_isometry`] (an
//!   isometry's image), [`RigidMotion::swing`](super::RigidMotion::swing)
//!   (a swing in one frame's plane) and [`UnitVec3::half_turned`] (a half
//!   turn about a unit axis). Each MEASURES what it made at a point
//!   scalar: within the rounding [`UnitVec3::stated`] charges the bits
//!   are kept, past it the value is squared (so a chain of rounded
//!   images does not drift off unit, and the door reports the length it
//!   divided by for a parameter that runs along the direction), and past
//!   any rounding, or not finite, it is poison. The check runs in a
//!   release build as in a debug one. **At an enclosure kind the doors
//!   rely on the caller's construction**: its argument, that every value
//!   the inputs enclose makes a unit vector, is what makes the result an
//!   enclosure of one. The doors refute only a box whose bracket of
//!   `v·v − 1` excludes zero; a point box of a rounded unit vector, which
//!   encloses none, usually passes that test, so a caller lifting an
//!   `f64` direction lifts it as a direction ([`UnitVec3::map_scalar`]),
//!   never entry by entry.
//! - [`UnitVec3::checked`] — that measurement for a vector whose
//!   construction has no door of its own (a frame squared into bare
//!   vectors, a fixture's literal).
//! - [`UnitVec3::stated`] — a stated `f64` direction adopted as its bits
//!   where it is unit to rounding, `None` past it.
//! - `-u`, [`UnitVec3::reversed`] and [`UnitVec3::turned_toward`] —
//!   negation and a sign chosen are exact at every scalar.
//! - The exact basis axes ([`UnitVec3::exact_x`], [`UnitVec3::exact_y`],
//!   [`UnitVec3::exact_z`]), exact by construction rather than by
//!   decision, and [`UnitVec3::signed_permutation`] of a witness.
//! - The cross product of an orthonormal pair, and a vector a rigid
//!   motion produced, both private to [`linalg`](super) and both exact
//!   in the reals by construction — they exist so
//!   [`RigidMotion`](super::RigidMotion) can build its axes and act on
//!   a direction, and they are documented on their own doors.
//!
//! **A witness enters a lane only by being minted there**; no read
//! carries one into a lane. Two reads take it out of the lane that
//! holds it, as for a frame ([`RigidMotion`](super::RigidMotion)'s
//! "Crossing scalars"): its point value, down to `f64` as an axis of
//! its frame ([`RigidMotion::point_value`](super::RigidMotion::point_value)),
//! and its enclosure, to `Interval` ([`UnitVec3::map_scalar`]): from
//! `Interval` the direction itself, from `Dual` or `Sym` over an
//! enclosure each component's bracket, and from `f64` the unit vector
//! nearest the stored one, `v/|v|`, enclosed component by component with
//! no decision ([`Real::enclose_unit`], which
//! normalizes the one column directly, not through a frame). No read
//! carries a direction into `Dual` or `Sym` from another scalar's lane:
//! [`UnitVec3::map_scalar`] across witness
//! kinds into any target but `Interval` fails a debug assertion, and a
//! release build answers the direction poisoned.
//!
//! The deciding doors in [`frame`](super::frame) mint through the
//! constructor under their own funnel names and map its refusals onto
//! theirs; the field is private to this module, so no door but those
//! listed writes it.
//!
//! **No door adopts a point value on trust.** A caller holding a bare
//! vector it believes is unit has a construction to name, whose door
//! measures it at a point scalar, or a decision to record, and then the
//! normalizing mint is exactly that decision. At an enclosure kind the
//! construction doors rely on the caller's construction (above), and
//! each call site states its enclosure argument.
//!
//! # What is NOT this type
//!
//! The carriers' frames are [`RigidMotion`](super::RigidMotion)s, whose
//! axes are this type, and a line's `dir` (`geom::Line`) is this type,
//! its parameter arc length. The 2-D analogue, `profile::path::Dir<T>`,
//! is that crate's own and is left where it is.

use core::ops::Neg;

use crate::k_stats::decide;
use crate::linalg::{LengthOutOfFormat, Mat3, Vec3};
use crate::predicate::{Band, Decide, Indeterminate, Lever, Margin, Sign};
use crate::real::{Real, is_finite_length};
use crate::spline::SpanLocate;

/// **A direction that cannot be unnormalized.** The only spellings are
/// the mints the [module docs](self) list; the field is private, so a
/// plane normal or an axis direction held here is unit as a property
/// of the TYPE, not of the caller's diligence, and it stays unit after
/// it is copied back out of whatever structure holds it.
///
/// The signed distance to a plane is a length only against a unit
/// normal, so an unnormalized one silently scales a DECIDED
/// predicate's comparand — a wrong [`Sign`] with no refusal. That
/// failure is unrepresentable rather than asserted.
///
/// **"Every other input" includes the ones a length comparison alone
/// cannot see.** A vector whose components overflow the norm
/// (`|v| ≳ 1e154` at `f64`) has an INFINITE length, which the scalar's
/// own [`Decide`] machinery calls maximally definite. The constructor
/// asks whether the length is a finite number BEFORE asking which side
/// of zero it lies on ([`UnitVec3Error::NonFiniteLength`]), so an
/// overflowed or poisoned length never reaches the funnel as a definite
/// margin.
///
/// The same comparison cannot see the other end either. A vector
/// whose components underflow the norm (`|v| ≲ 1e-162` at `f64`) has
/// a length of exactly zero for a direction that is perfectly good,
/// so the constructor would refuse it as degenerate — the right
/// outcome under a false cause. That case is separated before the
/// sign question too ([`UnitVec3Error::UnderflowedLength`]).
///
/// No `PartialEq`: comparison is the predicate layer's job, as for
/// every linalg type here.
///
/// The field is not reachable from outside this module:
///
/// ```compile_fail
/// use geom_core::{UnitVec3, Vec3};
/// let u: UnitVec3<f64> = UnitVec3(Vec3::unit_z());
/// ```
///
/// and a bare [`Vec3`] is not a witness — there is no conversion, so a
/// door that takes the witness cannot be handed a vector nobody
/// decided:
///
/// ```compile_fail
/// use geom_core::{UnitVec3, Vec3};
/// let (b1, b2) = UnitVec3::orthonormal_basis(Vec3::<f64>::unit_z());
/// ```
#[derive(Debug, Clone, Copy)]
pub struct UnitVec3<T: Real>(Vec3<T>);

/// **Why a vector has no unit direction** — the refusals of
/// [`decide_unit_direction`], which are also exactly why a vector
/// could not become a [`UnitVec3`]: the constructor adds the type, not
/// a refusal of its own. A closed enum (D4 ¶3); every arm is a fact
/// about the input, never a lane to swallow.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnitVec3Error {
    /// The vector's length decided to zero and the vector really is
    /// that small: it names no direction, and picking one for it
    /// would be invention (spec D3). The one way a length decides to
    /// zero WITHOUT this being true — a squared norm that underflowed
    /// out of the format — is [`UnitVec3Error::UnderflowedLength`],
    /// which is refused before this arm is reached.
    Degenerate,
    /// The vector's length is not a finite number — the components
    /// overflow the norm (`|v| ≳ 1e154` at `f64`), or one of them is
    /// the scalar's poison. Refused BEFORE the length is decided,
    /// because an infinite margin is maximally definite to
    /// [`Decide`] whatever the vector is; a poisoned one has no
    /// direction either.
    NonFiniteLength,
    /// The vector's length UNDERFLOWED to zero: its components are
    /// nonzero but small enough (`|v| ≲ 1e-162` at `f64`) that the
    /// squared norm is exactly zero, so the length reads as zero for
    /// a vector that has a perfectly good direction. A separate fact
    /// from [`UnitVec3Error::Degenerate`] and a separate recourse:
    /// no tolerance makes this length nonzero.
    UnderflowedLength,
    /// The length decision landed in the ambiguity band — at the
    /// interval scalar, an enclosure that straddles "has a direction"
    /// and "does not". Escalated unaltered.
    Escalated(Indeterminate),
}

impl core::fmt::Display for UnitVec3Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Degenerate => f.write_str(
                "a direction vector decided to zero length, so it names no \
                 direction to normalize",
            ),
            Self::NonFiniteLength => write!(
                f,
                "a direction vector's length is not a finite number (a component \
                 overflows the norm or is not a number). {}",
                crate::predicate::RANGE_ENDING
            ),
            Self::UnderflowedLength => write!(
                f,
                "a direction vector's length underflowed to zero, though it still \
                 names a direction. {}",
                crate::predicate::RANGE_ENDING
            ),
            // A direction's length has no declaration object at any
            // door that mints this witness, so the payload renders
            // under the levers those doors do have.
            Self::Escalated(source) => source
                .undecided(
                    crate::predicate::DIRECTION_LENGTH_SUBJECT,
                    source.lever_ending(crate::predicate::COINCIDENCE_RECOURSE),
                )
                .fmt(f),
        }
    }
}

impl std::error::Error for UnitVec3Error {}

/// The K funnel name of [`UnitVec3::levered`]'s arm decision: the arm
/// is a length, decided positive under the caller's band before it
/// levers anything.
pub const UNIT_DIRECTION_ARM: &str = "unit_direction_arm";

/// **Why [`UnitVec3::levered`] minted no witness**: its arm, or the
/// vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LeveredUnitError {
    /// The arm is not a length the band decides positive: `None` when
    /// it is no finite number or decides zero or negative, `Some` with
    /// the diagnostic when its sign is in the band
    /// ([`UNIT_DIRECTION_ARM`]). A fact about the caller's arm, never
    /// about the vector.
    Arm(Option<Indeterminate>),
    /// The vector's own refusal, as [`UnitVec3::new`]'s.
    Direction(UnitVec3Error),
}

impl core::fmt::Display for LeveredUnitError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Arm(None) => f.write_str(
                "the arm a direction's length is levered by is not a positive finite length",
            ),
            Self::Arm(Some(source)) => source
                .undecided(
                    "the arm a direction's length is levered by",
                    crate::predicate::lever_recourse(crate::predicate::COINCIDENCE_RECOURSE, None),
                )
                .fmt(f),
            Self::Direction(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for LeveredUnitError {}

/// **The direction-length decision, once**: is the length a finite
/// number, did it underflow, which side of zero is it on, and — only
/// then — the normalized ray or a typed refusal. The one
/// `Margin::norm3` decide-then-normalize spelling in the workspace.
///
/// Three questions in this order, and the order is the point.
///
/// 1. **Is the length a finite number?** Asked through the value
///    channel every scalar has ([`is_finite_length`]): a finite value
///    less itself is exactly zero, while `∞ − ∞` and `NaN − NaN` are
///    the scalar's poison. No bracket is read and no threshold is
///    invented, so an enclosure of any width passes — the arm bites at
///    the point scalars, which is where the failure is (an interval
///    whose norm overflowed still ENCLOSES the truth, so it stays
///    sound and simply refuses later, where a `f64` would answer a
///    definite wrong sign).
/// 2. **Did that length UNDERFLOW to zero?** Asked through the same
///    value channel ([`is_underflowed_length`](crate::is_underflowed_length)), against the largest
///    absolute component as the nonzero witness. Components below
///    ~1e-162 square to zero, so the norm is exactly zero for a
///    vector that has a perfectly good direction — and the decision
///    below then answers `Zero` DEFINITELY, at every ε, with a
///    refusal that names the one thing about the input that is
///    false. The fact is the overflow arm's twin, not the zero arm's:
///    the model is outside the range its own arithmetic can measure,
///    and the recourse is scale, not a different direction. Asked
///    second because a poisoned or overflowed length makes its two
///    ratios non-finite for a different reason.
/// 3. **Which side of zero is it on?** Through the scalar's own
///    decision machinery ([`Margin::norm3`] on the caller's band) —
///    [`Real`] deliberately has no comparison surface, and a
///    hand-rolled `> 0` would be wrong at the interval scalar. Only a
///    DEFINITELY zero length refuses, so a wide enclosure that
///    contains a real direction never refuses spuriously; one that
///    straddles zero escalates instead of guessing.
///
/// **`site` is the K funnel name this decision is recorded under, and
/// it is a PARAMETER because the name belongs to the layer that owns
/// the value while the decision belongs here.** A value's owner is
/// what its telemetry has to be readable by: a datum's normal is
/// decided under the datum boundary's name (`topo`'s
/// `POSE_UNIT_NORM`), a transform axis or a pattern direction under
/// the evaluation layer's (`editor-core`'s `EVAL_DIRECTION_NORM`).
/// One name for both would erase which layer decided; one body for
/// both is what keeps the arithmetic and the refusals from drifting.
///
/// **K consequence.** Both gates refuse BEFORE [`decide`], so neither
/// an overflowed nor an underflowed length contributes a sample to
/// the funnel under any site name. That is the intent for the second
/// exactly as for the first: the sample it used to contribute was an
/// exactly-zero margin recorded as a definite `Zero`, which is
/// telemetry about a length the format failed to hold rather than
/// about a direction the model does not have.
///
/// Nothing here dispatches on `site` and nothing stores it — it is
/// passed to the funnel and dropped. **A name reaching the K roster
/// this way is registered by hand or not at all**: this function will
/// decide under any string a caller passes, so a new site is a
/// `docs/K-REPORT.md` edit its author owes (that document's
/// "inventory method, restated" — the roster is hand-maintained and
/// nothing mechanical catches an omission).
///
/// # Errors
///
/// [`UnitVec3Error::NonFiniteLength`] on an overflowed or poisoned
/// length, [`UnitVec3Error::UnderflowedLength`] on one that
/// underflowed out of the format, [`UnitVec3Error::Degenerate`] on a
/// decided-zero one, [`UnitVec3Error::Escalated`] on an in-band one.
pub fn decide_unit_direction<T: Decide>(
    v: Vec3<T>,
    site: &'static str,
    band: Band,
) -> Result<Vec3<T>, UnitVec3Error> {
    decide_direction_by(v, site, band, Margin::norm3)
}

impl From<LengthOutOfFormat> for UnitVec3Error {
    fn from(e: LengthOutOfFormat) -> Self {
        match e {
            LengthOutOfFormat::NonFinite => Self::NonFiniteLength,
            LengthOutOfFormat::Underflowed => Self::UnderflowedLength,
        }
    }
}

/// The three questions of [`decide_unit_direction`], the third asked of
/// `margin(v)`.
fn decide_direction_by<T: Decide, A: crate::predicate::MarginTerms>(
    v: Vec3<T>,
    site: &'static str,
    band: Band,
    margin: impl FnOnce(Vec3<T>) -> Margin<T, A>,
) -> Result<Vec3<T>, UnitVec3Error> {
    // `margin` recomputes the plain norm the rule read (`Vec3::norm` is
    // deterministic), so the gates and the margin read the one length;
    // it is spelled twice rather than reached into.
    let (direction, _) = v.checked_direction_and_length()?;
    match decide(site, margin(v), band) {
        Ok(Sign::Positive) => Ok(direction),
        Ok(_) => Err(UnitVec3Error::Degenerate),
        Err(source) => Err(UnitVec3Error::Escalated(source)),
    }
}

impl UnitVec3<f64> {
    /// **Whether this is `other` exactly**: every component's difference
    /// zero, of either sign, so `+0` and `−0` agree. Structure read as
    /// structure, not a tolerance decision, and only at `f64`, where the
    /// number is all the scalar stands for: a scalar that stands for more
    /// (an enclosure, a tangent, a form in symbols) has no exact read off
    /// one value. The path sweep's still step asks it.
    #[must_use]
    pub fn is_exactly(self, other: Self) -> bool {
        let (a, b) = (self.0, other.0);
        [a.x - b.x, a.y - b.y, a.z - b.z]
            .into_iter()
            .all(|d| d == 0.0)
    }
}

impl<T: Real> UnitVec3<T> {
    /// The direction itself, unit.
    #[must_use]
    pub fn get(self) -> Vec3<T> {
        self.0
    }

    /// An orthonormal basis completing this direction to a
    /// right-handed frame: `(b1, b2)` with `(b1, b2, self)` orthonormal
    /// and right-handed. The construction, its evaluation order and its
    /// behaviour at every scalar are [`Vec3::orthonormal_basis`]'s;
    /// what this door adds is that its precondition — the input is
    /// unit — is the type's rather than the caller's.
    #[must_use]
    pub fn orthonormal_basis(self) -> (Vec3<T>, Vec3<T>) {
        self.0.orthonormal_basis()
    }

    /// **The cross product of an orthonormal pair, which is a witness
    /// by construction**: `a ⊥ b` both unit gives `|a × b| = 1`, so
    /// the third axis of a frame needs no decision and no divide.
    ///
    /// This is NOT a "check that it is already unit" constructor and
    /// cannot be used as one: the premise is carried by the ARGUMENT
    /// types plus the orthogonality the one caller
    /// ([`RigidMotion`](super::RigidMotion)) establishes at its own
    /// mints, and the result is a product rather than a vector handed
    /// in. A caller holding two witnesses it merely believes are
    /// perpendicular has a decision to make, not a door to call, which
    /// is why this is private to [`linalg`](super).
    pub(in crate::linalg) fn cross_of_orthonormal(a: Self, b: Self) -> Self {
        Self(a.0.cross(b.0))
    }

    /// **A unit vector a rigid motion produced**, by construction: a
    /// column or row of a rotation's Rodrigues form, or a witness
    /// carried through a rigid linear part
    /// ([`RigidMotion`](super::RigidMotion)), each unit over the reals
    /// for every value its inputs take. Like
    /// [`Self::cross_of_orthonormal`] it is not a "check that it is
    /// already unit" constructor: the premise is the construction, and
    /// its only callers are [`RigidMotion`](super::RigidMotion)'s mints
    /// and the `f64` door, which has read the column's norm within its
    /// rounding.
    pub(in crate::linalg) fn by_construction(v: Vec3<T>) -> Self {
        Self(v)
    }

    /// The `f64` this witness's value channel holds, or `None` when a
    /// component has no single finite one ([`SpanLocate::point_value`]:
    /// poison, or an enclosure wider than a point). The frame's
    /// [`point_value`](super::RigidMotion::point_value) crossing reads
    /// each axis here.
    pub(in crate::linalg) fn point_value(self) -> Option<UnitVec3<f64>>
    where
        T: SpanLocate,
    {
        let read = |x: T| x.point_value().filter(|x| x.is_finite());
        let v = self.0;
        Some(UnitVec3(Vec3::new(read(v.x)?, read(v.y)?, read(v.z)?)))
    }

    /// **A direction squared with no decision**: `v` over its own
    /// length, unit over the reals wherever that length is not zero,
    /// and poison where it is (`0/0`, the totality policy's NaN or NaI,
    /// which the predicate layer catches). Nothing is decided, so no
    /// band and no refusal: a degenerate input is a poisoned witness,
    /// not a typed one.
    ///
    /// It is the door a carrier's stated direction enters the type
    /// through until its construction mints it ([`UnitVec3::new`]); a
    /// construction that can decide its direction decides it.
    #[must_use]
    pub fn squared(v: Vec3<T>) -> Self {
        Self(v.normalize())
    }

    /// **This direction at another scalar**, `f` a lift that keeps each
    /// component's point: carried as it is between two scalars of one
    /// [`Witness`](crate::Witness) kind, and from `f64` to `Interval`
    /// the unit vector nearest the stored one, enclosed
    /// ([`Real::enclose_unit`]). Across kinds into any other target there
    /// is no read (the module docs): it fails a debug assertion, and a
    /// release build answers the direction poisoned.
    #[must_use]
    pub fn map_scalar<U: Real>(self, f: impl Fn(T) -> U) -> UnitVec3<U> {
        let v = self.0.map(f);
        if T::WITNESS == U::WITNESS {
            UnitVec3(v)
        } else {
            match U::enclose_unit([v.x, v.y, v.z]) {
                Some([x, y, z]) => UnitVec3(Vec3::new(x, y, z)),
                None => {
                    debug_assert!(
                        false,
                        "no read carries a direction into a lane: the target is not a plain enclosure"
                    );
                    UnitVec3(Vec3::new(crate::poison(), crate::poison(), crate::poison()))
                }
            }
        }
    }

    /// The exact `x̂`, unit as its literal bits at every scalar that
    /// represents 0 and 1 — no decision, nothing to decide: the world
    /// frames' axes, and the axis a caller turns about when it names a
    /// coordinate axis ([`RigidMotion::rotation`](super::RigidMotion::rotation)).
    /// A caller wanting a direction it computed wants
    /// [`UnitVec3::new`].
    #[must_use]
    pub fn exact_x() -> Self {
        Self(Vec3::unit_x())
    }

    /// The exact `ŷ`; see [`UnitVec3::exact_x`].
    #[must_use]
    pub fn exact_y() -> Self {
        Self(Vec3::unit_y())
    }

    /// The exact `ẑ`; see [`UnitVec3::exact_x`].
    #[must_use]
    pub fn exact_z() -> Self {
        Self(Vec3::unit_z())
    }

    /// **This unit vector, its coordinates signed and permuted**: its
    /// `k`th coordinate becomes coordinate `to[k].0`, negated where
    /// `to[k].1` — an orientation of the coordinate axes applied, by
    /// field shuffling and negation alone, so the result is unit exactly
    /// as this one is. `None` unless the indices are a permutation of
    /// `0, 1, 2`.
    #[must_use]
    pub fn signed_permutation(self, to: [(usize, bool); 3]) -> Option<Self> {
        let from = [self.0.x, self.0.y, self.0.z];
        let mut out = [T::zero(); 3];
        let mut seen = [false; 3];
        for (k, &(axis, negative)) in to.iter().enumerate() {
            if axis > 2 || seen[axis] {
                return None;
            }
            seen[axis] = true;
            out[axis] = if negative { -from[k] } else { from[k] };
        }
        Some(Self(Vec3::new(out[0], out[1], out[2])))
    }
}

impl<T: Decide> UnitVec3<T> {
    /// **The normalizing constructor**: `v` normalized, or a typed
    /// refusal.
    ///
    /// The decision itself — finiteness first, then underflow, then
    /// which side of zero the length lies on, then normalize or
    /// refuse — is [`decide_unit_direction`]; the three questions and
    /// the reason for their order are documented there. What this
    /// constructor adds is the TYPE: a direction that reaches it comes
    /// out unit as a property of the type rather than of the caller's
    /// diligence. `site` is the K funnel name the caller owns the
    /// value under, exactly as for [`decide_unit_direction`].
    ///
    /// # Errors
    ///
    /// [`UnitVec3Error::NonFiniteLength`] on an overflowed or poisoned
    /// length, [`UnitVec3Error::UnderflowedLength`] on one that
    /// underflowed out of the format, [`UnitVec3Error::Degenerate`] on
    /// a decided-zero one, [`UnitVec3Error::Escalated`] on an in-band
    /// one.
    pub fn new(v: Vec3<T>, site: &'static str, band: Band) -> Result<Self, UnitVec3Error> {
        decide_unit_direction(v, site, band).map(Self)
    }

    /// **`v` normalized, its length decided levered at an `f64` arm**:
    /// [`UnitVec3::new`]'s gates on `v`, and its margin `|v| · |arm|`
    /// ([`Margin::levered_at`], one sample under `site`), the levered
    /// vector's length; then the direction of `v` itself. A caller
    /// levering a pure number into a length over an arm it read as a
    /// bound keeps the arm out of the direction it mints, which is a
    /// function of `v` alone at every scalar. An arm's sign is no part
    /// of the length it levers by, so a negative arm decides as its
    /// magnitude.
    ///
    /// # Errors
    ///
    /// [`UnitVec3Error::NonFiniteLength`] for an arm whose square is not
    /// finite (no levered length of a unit-scale vector is then a
    /// number); otherwise [`UnitVec3::new`]'s gates on `v` and its
    /// refusals of the levered length.
    pub fn new_scaled(
        v: Vec3<T>,
        arm: Lever<f64>,
        site: &'static str,
        band: Band,
    ) -> Result<Self, UnitVec3Error> {
        let arm = arm.magnitude();
        if !arm.length().powi(2).is_finite() {
            return Err(UnitVec3Error::NonFiniteLength);
        }
        decide_direction_by(v, site, band, |v| Margin::levered_at(v.norm(), arm))
            .map(|_| Self(v.normalize()))
    }

    /// **The normalizing constructor for a dimensionless vector**: a
    /// carrier's unit-at-rest normal or axis, read as direction
    /// cosines. [`UnitVec3::new`]'s norm is a length; this one's is a
    /// pure number, so the length question is asked of it levered by
    /// `arm` ([`Margin::levered`]), the D4 θ·r form.
    ///
    /// **What the arm must be**: a finite length the band decides
    /// positive (refused otherwise, [`LeveredUnitError::Arm`], decided
    /// under [`UNIT_DIRECTION_ARM`]), naming a reach over which the
    /// direction is consumed, measured from the pivot its carrier's
    /// position is read at. The finiteness and underflow gates on the
    /// vector, their order and its refusals are
    /// [`decide_unit_direction`]'s.
    ///
    /// **What the witness then says**: `|v|·arm` cleared the band, so
    /// `|v|` cleared `band / arm` — not the band itself. An arm longer
    /// than a metre admits a vector [`UnitVec3::new`] refuses, and one
    /// little longer than the band refuses even a unit vector. For a
    /// given vector a shorter arm only shrinks the margin, so an arm
    /// that under-states the consumed reach never decides positive where
    /// the full reach would not.
    ///
    /// # Errors
    ///
    /// [`LeveredUnitError::Arm`] for an arm that is not a positive
    /// finite length; [`LeveredUnitError::Direction`] with
    /// [`UnitVec3::new`]'s refusal of the levered length.
    pub fn levered(
        v: Vec3<T>,
        site: &'static str,
        band: Band,
        arm: Lever<T>,
    ) -> Result<Self, LeveredUnitError> {
        if !is_finite_length(arm.length()) {
            return Err(LeveredUnitError::Arm(None));
        }
        match decide(UNIT_DIRECTION_ARM, Margin::of(arm.length()), band) {
            Ok(Sign::Positive) => {}
            Ok(_) => return Err(LeveredUnitError::Arm(None)),
            Err(source) => return Err(LeveredUnitError::Arm(Some(source))),
        }
        decide_direction_by(v, site, band, |v| Margin::levered(v.norm(), arm))
            .map(Self)
            .map_err(LeveredUnitError::Direction)
    }
}

/// How far past its rounding an `f64` image of a unit vector may stand
/// and still be squared rather than refused: `√ε`, about `1.5e-8`. One
/// isometry's image of a unit vector stands within a few
/// [`crate::rounding_charge`]s of unit, and a chain of them is squared
/// back each time it passes the charge, so no chain of rounding reaches
/// this; a value past it is no image of a unit vector, and the door
/// answers it poisoned.
pub(in crate::linalg) const SQUARING_REACH: f64 = 1.490_116_119_384_765_6e-8;

/// What a door made of a vector its construction says is unit
/// ([`UnitVec3::kept_or_squared`]).
enum Measured {
    /// Unit within its rounding, or an enclosure that holds a unit
    /// vector: its bits kept.
    Kept,
    /// Past its rounding at a point scalar, within `√ε`:
    /// squared.
    Squared,
    /// Refuted: not finite, an enclosure that holds no unit vector, or
    /// a point past [`SQUARING_REACH`].
    Refuted,
}

impl<T: Real> UnitVec3<T> {
    /// **A vector its construction says is unit, measured**: the
    /// release-time check every construction door below runs.
    ///
    /// - At an enclosure kind ([`Witness::Exact`](crate::Witness::Exact))
    ///   the construction's argument stands over every value its inputs
    ///   enclose, so the bits are kept, unless the bracket of `v·v − 1`
    ///   excludes zero or is unreadable, which refutes it.
    /// - At a point kind ([`Witness::Inexact`](crate::Witness::Inexact))
    ///   the defect `|v·v − 1|` is measured against
    ///   [`UnitVec3::stated`]'s charge, `rounding_charge(v·v + 1)`. Within
    ///   it the bits are kept; past it, within `√ε`, the
    ///   value is squared, so a chain of rounded constructions does not
    ///   drift off unit; beyond, or not finite, it is refuted.
    ///
    /// A refuted vector is answered poisoned, never adopted, in a release
    /// build as in a debug one.
    fn measured(v: Vec3<T>) -> Measured {
        let n = v.dot(v);
        let (Some((lo, hi)), Some((_, top))) = (
            (n - T::one()).told_reading().ends(),
            n.told_reading().ends(),
        ) else {
            return Measured::Refuted;
        };
        if !(lo.is_finite() && hi.is_finite() && top.is_finite()) {
            return Measured::Refuted;
        }
        match T::WITNESS {
            crate::Witness::Exact => {
                if lo <= 0.0 && 0.0 <= hi {
                    Measured::Kept
                } else {
                    Measured::Refuted
                }
            }
            crate::Witness::Inexact => {
                let defect = lo.abs().max(hi.abs());
                if defect <= crate::rounding_charge(top.abs() + 1.0) {
                    Measured::Kept
                } else if defect <= SQUARING_REACH {
                    Measured::Squared
                } else {
                    Measured::Refuted
                }
            }
        }
    }

    /// [`Self::measured`]'s verdict carried out: the direction, and the
    /// length its value was divided by (`one` where its bits are kept),
    /// which a caller whose parameter runs along the direction scales
    /// that parameter by.
    pub(in crate::linalg) fn kept_or_squared(v: Vec3<T>) -> (Self, T) {
        match Self::measured(v) {
            Measured::Kept => (Self(v), T::one()),
            Measured::Squared => {
                let (d, len) = v.direction_and_length();
                (Self(d), len)
            }
            Measured::Refuted => (Self::poisoned(), crate::poison()),
        }
    }

    /// The direction every component of which is poison: a refused
    /// construction's answer.
    fn poisoned() -> Self {
        Self(Vec3::new(crate::poison(), crate::poison(), crate::poison()))
    }

    /// **This direction's image under an isometry's linear part**: a
    /// rigid motion's rotation, or a reflection's Householder matrix.
    /// Unit over the reals for every orthogonal `linear`, so at an
    /// enclosure kind the image keeps its bits; at a point scalar the
    /// rounding of the product is measured, and an image a chain of maps
    /// has carried past its rounding is squared (the module docs'
    /// measurement). The second value is the length the image was divided
    /// by, `one` where its bits are kept: a line carried by the map scales
    /// its parameter by it.
    ///
    /// **It accepts any [`Mat3`]**, and only its callers' types keep a
    /// non-isometry out: a `RigidMotion`'s linear part, a `Reflection`'s.
    /// A `linear` off orthogonal by more than any rounding (past `√ε`) is
    /// refuted, the image answered poisoned; one off by less is read as a
    /// stretch, so `(1 + 1e-9)·I` returns its image squared and a stretch
    /// of `1 + 1e-9`, not a refusal.
    #[must_use]
    pub fn through_isometry(self, linear: Mat3<T>) -> (Self, T) {
        Self::kept_or_squared(linear * self.0)
    }

    /// **This direction turned a half turn about the unit `axis`**:
    /// `2(u·â)â − u`, the rotation by `π` with no trigonometry to round,
    /// unit over the reals; its rounding is measured as
    /// [`Self::through_isometry`]'s is, and squared past it.
    #[must_use]
    pub fn half_turned(self, axis: Self) -> Self {
        let (u, a) = (self.0, axis.0);
        Self::kept_or_squared(a * (T::from_f64(2.0) * u.dot(a)) - u).0
    }

    /// **The unit chord along `v`, and its length**: `v / |v|` and `|v|`
    /// ([`Vec3::direction_and_length`], read through its one prescale),
    /// unit over the reals wherever `v` is not zero. Nothing is decided
    /// and nothing measured, so no band and no refusal, in a release build
    /// as in a debug one. A zero chord has no direction: its quotient is
    /// `0/0`, poison, beside its true length, zero, which a span reader
    /// refuses typed. A chord with an entry that is not finite, or whose
    /// length overflows the format, has neither: the direction and the
    /// length are both poison, never one component of them. An
    /// overflowing chord of finite entries has a direction the prescale
    /// reads, but it is poisoned with its length all the same: a chord
    /// is the span `(0, |v|)` a segment stands on, and a chord longer
    /// than the format is no segment the kernel holds: its ends stand
    /// far past any session box. A finite direction beside a poisoned
    /// length would only move the refusal to the span's reader.
    #[must_use]
    pub fn chord(v: Vec3<T>) -> (Self, T) {
        let (d, len) = v.direction_and_length();
        let finite = |x: T| {
            x.told_reading()
                .ends()
                .is_some_and(|(lo, hi)| lo.is_finite() && hi.is_finite())
        };
        if finite(len) && finite(v.x) && finite(v.y) && finite(v.z) {
            (Self(d), len)
        } else {
            (Self::poisoned(), crate::poison())
        }
    }

    /// **This direction reversed, its zeros positive**: `0 − u`, exact at
    /// every scalar as negation is, except that a zero component is
    /// `+0` rather than [`Neg`]'s `−0`.
    #[must_use]
    pub fn reversed(self) -> Self {
        Self(Vec3::zero() - self.0)
    }

    /// **This direction or its reverse, whichever agrees with `like`**:
    /// [`Vec3::turned_toward`], a scaling by an exact `±1`, so the result
    /// is unit exactly as this one is. An enclosure that does not decide
    /// the sign turns by the hull `[−1, 1]`, whose image holds both
    /// directions.
    #[must_use]
    pub fn turned_toward(self, like: Vec3<T>) -> Self {
        Self(self.0.turned_toward(like))
    }

    /// **A direction unit to rounding, checked**: the construction doors'
    /// measurement of `v` (module docs), its bits kept within its rounding,
    /// squared past it within `√ε`, and poisoned beyond or where it is not
    /// finite, in a release build as in a debug one. The door for a
    /// vector whose unit length is a premise none of the typed doors
    /// names (a frame squared into bare vectors, a fixture's literal); a
    /// caller that can name its construction calls that construction's
    /// door, and one with a vector it merely holds has a decision to make
    /// ([`UnitVec3::new`]).
    ///
    /// **At an enclosure kind it relies on the caller's construction**:
    /// it keeps any box whose bracket of `v·v − 1` holds zero, which a
    /// point box of a rounded unit vector, enclosing no unit vector,
    /// usually does. Each caller at an enclosure scalar states why its
    /// box encloses a unit vector: a normalization or a Gram–Schmidt
    /// step run in interval arithmetic, an orthonormal frame's axes
    /// weighted by a sine and cosine, a witness's sign turned.
    #[must_use]
    pub fn checked(v: Vec3<T>) -> Self {
        Self::kept_or_squared(v).0
    }
}

impl UnitVec3<f64> {
    /// **A stated `f64` direction adopted as its bits**: `v` itself where
    /// it is unit within its own rounding, and `None` where it is not.
    /// Within its rounding is the frame door's column test
    /// ([`RigidMotion::check_rigid`](super::RigidMotion::check_rigid)):
    /// the defect `|v·v − 1|`, measured, at most
    /// [`crate::rounding_charge`] of its terms `v·v + 1`, which is
    /// [`crate::SHORT_CHAIN_ULPS`] half-ulps of 2. A non-finite
    /// component, or a defect beyond the charge, is no unit direction.
    ///
    /// It decides nothing and squares nothing: a direction that is unit
    /// to rounding keeps every bit, so a reading pinned to the stated
    /// bits reads them, where [`UnitVec3::squared`] would move the last
    /// place. A direction the type then carries to another scalar goes
    /// through [`UnitVec3::map_scalar`]: the polar unit enclosure at
    /// `Interval`, poison into a lane.
    #[must_use]
    pub fn stated(v: Vec3<f64>) -> Option<Self> {
        let finite = v.x.is_finite() && v.y.is_finite() && v.z.is_finite();
        let n = v.dot(v);
        let charge = crate::rounding_charge(n + 1.0);
        (finite && charge.is_finite() && (n - 1.0).abs() <= charge).then_some(Self(v))
    }
}

/// Negation is exact at every scalar, so the negated direction is unit
/// by the same decision.
impl<T: Real> Neg for UnitVec3<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// The rule at both ends and at zero: an overflowed or poisoned plain
    /// norm, an underflowed one, an in-format one, and the zero vector,
    /// whose length is the door's own question.
    #[test]
    fn a_length_leaves_the_format_at_either_end_and_not_at_zero() {
        for (v, want) in [
            (
                Vec3::new(1e200, 1e200, 0.0),
                Some(UnitVec3Error::NonFiniteLength),
            ),
            (
                Vec3::new(f64::NAN, 1.0, 0.0),
                Some(UnitVec3Error::NonFiniteLength),
            ),
            (
                Vec3::new(1e-170, 0.0, 0.0),
                Some(UnitVec3Error::UnderflowedLength),
            ),
            (Vec3::new(1e-150, 2e-150, 0.0), None),
            (Vec3::new(3.0, 4.0, 12.0), None),
            (Vec3::new(0.0, 0.0, 0.0), None),
        ] {
            assert_eq!(
                v.checked_direction_and_length()
                    .err()
                    .map(UnitVec3Error::from),
                want,
                "{v:?}"
            );
        }
    }

    fn bits3(v: Vec3<f64>) -> [u64; 3] {
        [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()]
    }

    /// **The levered door decides `|v|·arm`, after deciding the arm.**
    /// The same unit normal decides by its arm, and the arm itself is
    /// refused unless it is a positive finite length. The rows that
    /// pin what the witness claims: a vector shorter than the band
    /// passes on a long arm where [`UnitVec3::new`] refuses it, and one
    /// clearing the band refuses on a short one.
    #[test]
    fn the_levered_door_decides_the_arm_then_the_levered_length() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let ask = |v: Vec3<f64>, arm: f64| {
            UnitVec3::levered(
                v,
                "test_direction",
                band,
                crate::test_support::fixture_lever(arm),
            )
        };
        let z = |len: f64| Vec3::new(0.0, 0.0, len);
        let up = ask(z(1.0), 1e-3)
            .expect("a unit normal over a 1 mm reach")
            .get();
        assert_eq!(bits3(up), bits3(Vec3::unit_z()), "a 1 mm reach");
        for arm in [0.0, -1.0, 1e-10, f64::NAN, f64::INFINITY] {
            assert_eq!(
                ask(z(1.0), arm).err(),
                Some(LeveredUnitError::Arm(None)),
                "arm {arm:e}"
            );
        }
        assert!(
            matches!(ask(z(1.0), 5e-9), Err(LeveredUnitError::Arm(Some(_)))),
            "arm in band"
        );
        assert_eq!(
            ask(z(1e-10), 1.0).err(),
            Some(LeveredUnitError::Direction(UnitVec3Error::Degenerate)),
            "levered below the band"
        );
        assert!(
            matches!(
                ask(z(5e-9), 1.0),
                Err(LeveredUnitError::Direction(UnitVec3Error::Escalated(_)))
            ),
            "levered into the band"
        );
        assert!(
            ask(z(1e-11), 1e3).is_ok(),
            "a long arm admits a sub-band vector"
        );
        assert_eq!(
            UnitVec3::new(z(1e-11), "test_direction", band).err(),
            Some(UnitVec3Error::Degenerate),
            "which the metre door refuses"
        );
        assert_eq!(
            ask(z(2e-8), 0.01).err(),
            Some(LeveredUnitError::Direction(UnitVec3Error::Degenerate)),
            "and a short arm refuses a vector the metre door admits"
        );
        assert!(UnitVec3::new(z(2e-8), "test_direction", band).is_ok());
    }

    /// **The four answers the direction door gives, and the two that
    /// a length comparison alone cannot tell apart.**
    ///
    /// A direction whose components are below ~1e-162 squares to
    /// exactly zero, so the norm is exactly zero and the decision
    /// below answers `Zero` DEFINITELY — at every ε, because no
    /// tolerance makes an unrepresentable square nonzero. Refusing it
    /// as `Degenerate` is the right outcome under a false cause: the
    /// vector has a direction, and the recourse is the overflow arm's
    /// (scale the geometry), not "give me a nonzero direction".
    ///
    /// The rows that would stay green under a gate that swallowed the
    /// zero arm are the last two: a vector that really is zero, and
    /// one that is merely SMALLER than the band. Both must keep
    /// `Degenerate`.
    #[test]
    fn the_direction_door_tells_an_underflowed_length_from_a_zero_one() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let ask = |v: Vec3<f64>| decide_unit_direction(v, "test_direction", band).err();

        for v in [
            Vec3::new(1e-180, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1e-200),
            Vec3::new(1e-320, 1e-320, 0.0),
        ] {
            assert_eq!(
                ask(v),
                Some(UnitVec3Error::UnderflowedLength),
                "{v:?} has a direction its norm cannot measure"
            );
        }

        // The zero vector: no direction to name, and the arm that
        // says so keeps saying it.
        assert_eq!(
            ask(Vec3::new(0.0, 0.0, 0.0)),
            Some(UnitVec3Error::Degenerate)
        );
        // A length the format holds perfectly well and the BAND calls
        // zero. Nothing underflowed; the refusal is the tolerance's.
        assert_eq!(
            ask(Vec3::new(1e-30, 0.0, 0.0)),
            Some(UnitVec3Error::Degenerate)
        );
        // The other end, unmoved, and asked FIRST: an overflowed norm
        // is not a number to ask the underflow question about.
        assert_eq!(
            ask(Vec3::new(1e200, 0.0, 0.0)),
            Some(UnitVec3Error::NonFiniteLength)
        );
        assert_eq!(
            ask(Vec3::new(f64::NAN, 0.0, 0.0)),
            Some(UnitVec3Error::NonFiniteLength)
        );
        // And a direction that HAS a length still normalizes, bit for
        // bit as the bare expression does.
        let good = Vec3::new(3.0, 4.0, 0.0);
        let u =
            decide_unit_direction(good, "test_direction", band).expect("a direction with a length");
        let bare = good.normalize();
        assert_eq!(
            bits3(u),
            bits3(bare),
            "the gates are questions, not arithmetic: {u:?} vs {bare:?}"
        );
    }

    /// The constructor is the decision plus the type: it refuses the
    /// lengthless typed, and what it holds is the bare normalization's
    /// bits, at whatever scale the input arrived.
    ///
    /// Every fixture here is off unit by MORE than the assertion's own
    /// tolerance, deliberately: a wobble the f64 grid swallows
    /// (`1.0 + 1e-30` IS `1.0`; `(1e-30, 1e-30, 1.0)` has
    /// `norm_squared` exactly 1) would pass this row without the
    /// normalization ever running.
    #[test]
    fn the_constructor_refuses_the_lengthless_and_normalizes_the_rest() {
        let band = Band::new(1e-6, 1e-3).expect("a well-ordered band");
        assert!(matches!(
            UnitVec3::new(Vec3::new(0.0, 0.0, 0.0), "test_direction", band),
            Err(UnitVec3Error::Degenerate)
        ));
        // Scale is irrelevant to what comes out: a 1e-12 wobble on a
        // unit input and a 1e6 blow-up both leave unit length.
        for v in [
            Vec3::new(0.0, 0.0, 1.0 + 1e-12),
            Vec3::new(1e-6, 1e-6, 1.0),
            Vec3::new(3e6, 4e6, 0.0),
            Vec3::new(3.0, 4.0, 12.0),
        ] {
            assert!(
                (v.norm() - 1.0).abs() > 1e-15,
                "the fixture {v:?} is already unit, so it would not \
                 exercise the normalization"
            );
            let u = UnitVec3::new(v, "test_direction", band)
                .expect("a vector with a length")
                .get();
            assert!(
                (u.norm() - 1.0).abs() <= 1e-15,
                "norm {} for {v:?}",
                u.norm()
            );
            assert_eq!(bits3(u), bits3(v.normalize()), "the same divide, for {v:?}");
        }
        // The overflow class: a length that is not a finite NUMBER
        // refuses BEFORE the sign of the length is asked for: an
        // infinite margin is maximally definite to `sign_within`.
        for v in [
            Vec3::new(1e200, 0.0, 0.0),
            Vec3::new(0.0, 1e200, 1e200),
            Vec3::new(f64::INFINITY, 0.0, 0.0),
            Vec3::new(f64::NEG_INFINITY, 0.0, 0.0),
            Vec3::new(f64::NAN, 0.0, 1.0),
        ] {
            assert!(
                matches!(
                    UnitVec3::new(v, "test_direction", band),
                    Err(UnitVec3Error::NonFiniteLength)
                ),
                "{v:?} must refuse, not normalize"
            );
        }
    }

    /// The refusal text names what happened to the LENGTH and the
    /// recourse that works; an underflowed direction is not the
    /// no-direction refusal, and no smaller ε recovers it.
    #[test]
    fn the_underflow_refusal_names_the_length_and_the_recourse() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let refused = UnitVec3::new(Vec3::new(1e-180, 0.0, 0.0), "test_direction", band)
            .expect_err("a direction with no measurable length is refused");
        assert_eq!(refused, UnitVec3Error::UnderflowedLength);
        let said = refused.to_string();
        assert!(
            said.starts_with("a direction vector's length underflowed to zero"),
            "the refusal says what happened to the LENGTH: {said}"
        );
        assert!(
            said.contains(crate::predicate::RANGE_RECOURSE),
            "and it names the recourse that works: {said}"
        );
        assert!(
            !said.contains("names no direction"),
            "an underflowed direction is not the no-direction refusal: {said}"
        );
    }

    /// Negation mints the witness, and the minted vector is the bare
    /// arithmetic's bit for bit — including on the zero components,
    /// whose sign flips.
    #[test]
    fn negation_is_the_bare_negation_bit_for_bit() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        for v in [
            Vec3::new(3.0, 4.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(-1e-3, 2.5, -7.0),
        ] {
            let u = UnitVec3::new(v, "test_direction", band).expect("a direction");
            assert_eq!(bits3((-u).get()), bits3(-u.get()), "for {v:?}");
            assert_eq!(
                bits3((-(-u)).get()),
                bits3(u.get()),
                "an involution, for {v:?}"
            );
        }
    }

    /// The witness door to the orthonormal basis answers exactly what
    /// the bare construction answers on the same unit vector.
    #[test]
    fn the_witness_orthonormal_basis_is_the_bare_construction() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        for v in [
            Vec3::new(3.0, 4.0, 12.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(1.0, -2.0, 0.0),
        ] {
            let u = UnitVec3::new(v, "test_direction", band).expect("a direction");
            let (w1, w2) = u.orthonormal_basis();
            let (b1, b2) = u.get().orthonormal_basis();
            assert_eq!(bits3(w1), bits3(b1), "b1 for {v:?}");
            assert_eq!(bits3(w2), bits3(b2), "b2 for {v:?}");
        }
    }

    /// **The witness at `Dual`**: the value channel is the `f64`
    /// witness bit for bit, and the tangent is the quotient rule's —
    /// the `Dual` division's own association of `d(v/|v|)`, which the
    /// bare `Vec3::normalize` at `Dual` spells identically.
    ///
    /// The closed form `(v̇·n − v·ṅ)/n²` with `ṅ = (v·v̇)/n` is checked
    /// alongside to a rounding tolerance, so the row also says WHICH
    /// derivative the bits are of.
    #[test]
    fn at_dual_the_value_channel_is_the_f64_witness_and_the_tangent_is_the_quotient_rules() {
        use crate::dual::Dual;
        let band = Band::new(1e-9, 1e-8).unwrap();
        let v = Vec3::new(3.0, -4.0, 12.0);
        let seed = Vec3::new(0.5, -1.25, 2.0);
        let vd = Vec3::new(
            Dual::new(v.x, seed.x),
            Dual::new(v.y, seed.y),
            Dual::new(v.z, seed.z),
        );
        let u = UnitVec3::new(v, "test_direction", band)
            .expect("a direction")
            .get();
        let ud = UnitVec3::new(vd, "test_direction", band)
            .expect("a direction at Dual")
            .get();
        let value = Vec3::new(ud.x.value, ud.y.value, ud.z.value);
        let tangent = Vec3::new(ud.x.deriv, ud.y.deriv, ud.z.deriv);
        assert_eq!(
            bits3(value),
            bits3(u),
            "the value channel is the f64 witness"
        );
        let bare = vd.normalize();
        let bare_tangent = Vec3::new(bare.x.deriv, bare.y.deriv, bare.z.deriv);
        assert_eq!(
            bits3(tangent),
            bits3(bare_tangent),
            "the tangent is the Dual division's quotient rule"
        );
        let n = v.norm();
        let n_dot = v.dot(seed) / n;
        let closed = (seed * n - v * n_dot) / (n * n);
        assert!(
            (tangent - closed).norm() <= 1e-15,
            "the quotient rule's derivative: {tangent:?} vs {closed:?}"
        );
    }

    /// **An escalated direction length ends in the lever, and notes a
    /// poisoned margin**: the length is a gauge, so an in-band margin
    /// carries the lever alone and an unreadable one adds the note.
    #[test]
    fn an_escalated_length_ends_in_the_lever_and_notes_a_poisoned_margin() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let render = |margin: f64| {
            let i = crate::k_stats::decide(
                "unit_vec_test_length",
                crate::predicate::Margin::of(margin),
                band,
            )
            .unwrap_err();
            UnitVec3Error::Escalated(i).to_string()
        };
        let lever = crate::predicate::COINCIDENCE_RECOURSE;
        let in_band = render(0.5 * (band.zero() + band.escalate()));
        assert!(
            in_band.ends_with(&format!(". Recourse: {lever}")),
            "in band: the lever alone: {in_band}"
        );
        let poisoned = render(f64::NAN);
        assert!(
            poisoned.ends_with(&format!(
                ". Recourse: {lever}; {}",
                crate::predicate::UNREADABLE_MARGIN_NOTE
            )),
            "poisoned: the lever, then the note: {poisoned}"
        );
    }
}

// The enclosure scalar's rows: an enclosure that contains unit length
// is a direction, an overflowed enclosure stays sound, only a length
// the enclosure DECIDES is zero refuses, and an enclosure that merely
// cannot tell escalates rather than refusing.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod interval_tests {
    use super::{Band, Decide, Real, Sign, UnitVec3, UnitVec3Error, Vec3};
    use crate::interval::Interval;

    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    /// An enclosure that CONTAINS unit length is a direction — the
    /// widths interval arithmetic carries are not a reason to refuse —
    /// and what comes back DECIDES unit at the band.
    ///
    /// The assertion direction is deliberate. "Not definitely off
    /// unit" would be satisfied by every degradation, including a
    /// normalization that never ran on a wide enough input: a claim
    /// that gets easier as the enclosure gets worse is not evidence.
    /// `Ok(Sign::Zero)` on `‖u‖ − 1` is the opposite — it holds only
    /// while the whole enclosure sits inside the band, so it fails
    /// loudly if the normalization is skipped, if the result is
    /// collapsed, or if the enclosure blows up. Both fixtures are
    /// therefore tight; the price is that this row says nothing about
    /// SLOPPY enclosures, which the escalation row below covers.
    #[test]
    fn an_enclosure_containing_unit_length_passes() {
        // Straddles unit length; a scale away from it.
        let wobbled = Vec3::new(
            Interval::from_bounds(-1e-13, 1e-13),
            Interval::from_bounds(-1e-13, 1e-13),
            Interval::from_bounds(1.0 - 1e-13, 1.0 + 1e-13),
        );
        let scaled = Vec3::new(
            Interval::from_f64(3e4),
            Interval::from_f64(4e4),
            Interval::from_bounds(-1e-9, 1e-9),
        );
        for v in [wobbled, scaled] {
            let u = UnitVec3::new(v, "test_direction", band())
                .expect("an enclosure with a length")
                .get();
            let off = u.norm() - Interval::from_f64(1.0);
            assert_eq!(
                off.sign_within(band()).map(|d| d.sign),
                Ok(Sign::Zero),
                "the normalized enclosure must DECIDE unit length: {off:?}"
            );
        }
    }

    /// The overflow class at the enclosure scalar, stated honestly: an
    /// interval whose norm overflows still ENCLOSES the true length,
    /// so it is not unsound and the constructor does not refuse it —
    /// what it loses is precision, and the loss surfaces downstream as
    /// an escalation rather than as a definite wrong sign. Poison is
    /// the arm that does bite here: an empty/NaI component has no
    /// length at all.
    #[test]
    fn an_overflowed_enclosure_stays_sound_and_poison_refuses() {
        let huge = Vec3::new(
            Interval::from_f64(1e200),
            Interval::from_f64(0.0),
            Interval::from_f64(0.0),
        );
        let u = UnitVec3::new(huge, "test_direction", band())
            .expect("an overflowing enclosure still encloses its direction")
            .get();
        // Containment, the interval contract: unit length is inside
        // what comes back, so nothing downstream can certify a wrong
        // side from it.
        let off = u.norm() - Interval::from_f64(1.0);
        assert!(
            !matches!(
                off.sign_within(band()).map(|d| d.sign),
                Ok(Sign::Positive | Sign::Negative)
            ),
            "an overflowed enclosure must not DECIDE off-unit: {off:?}"
        );
        let poisoned = Vec3::new(
            Interval::from_f64(f64::NAN),
            Interval::from_f64(0.0),
            Interval::from_f64(1.0),
        );
        assert!(
            matches!(
                UnitVec3::new(poisoned, "test_direction", band()),
                Err(UnitVec3Error::NonFiniteLength)
            ),
            "a poisoned component names no direction"
        );
    }

    /// A DECIDED zero length refuses; an enclosure that straddles the
    /// band escalates instead of picking an arm.
    #[test]
    fn a_decided_zero_refuses_and_a_straddling_enclosure_escalates() {
        let zero = Vec3::new(
            Interval::from_f64(0.0),
            Interval::from_f64(0.0),
            Interval::from_f64(0.0),
        );
        assert!(
            matches!(
                UnitVec3::new(zero, "test_direction", band()),
                Err(UnitVec3Error::Degenerate)
            ),
            "a length the enclosure decides is zero"
        );
        let straddling = Vec3::new(
            Interval::from_f64(0.0),
            Interval::from_f64(0.0),
            Interval::from_bounds(0.0, 1e-7),
        );
        assert!(
            matches!(
                UnitVec3::new(straddling, "test_direction", band()),
                Err(UnitVec3Error::Escalated(_))
            ),
            "an enclosure that cannot tell escalates"
        );
        // Its sentence names the decision and offers only the levers a
        // direction door has: no door that mints this witness takes a
        // declaration.
        let said = UnitVec3::new(straddling, "test_direction", band())
            .map(|_| ())
            .unwrap_err()
            .to_string();
        assert!(
            said.contains("whether a direction has any length is undecided"),
            "{said}"
        );
        assert!(
            said.contains(crate::predicate::COINCIDENCE_RECOURSE),
            "{said}"
        );
        assert!(!said.contains("declare"), "{said}");
    }

    /// **The underflow gate is a POINT-scalar gate**, exactly as the
    /// finiteness gate is, and this row is what says so.
    ///
    /// A `1e-180` component squares to zero at `f64`, so the norm is
    /// exactly zero and the direction is unrecoverable. At the
    /// enclosure scalar the same component squares to `[0, 1e-323]`
    /// and the norm comes back `[0, 3.1e-162]` — an enclosure that
    /// still CONTAINS the true length. Nothing underflowed out of the
    /// format; the enclosure is simply wide, and refusing it here
    /// would refuse a sound enclosure for being wide.
    ///
    /// So this scalar goes on deciding against the band, and for this
    /// input the band answers `Degenerate` — the whole enclosure sits
    /// inside it. That is the pinned claim, and it is the one that
    /// distinguishes a no-op gate from a gate that fires: an
    /// `UnderflowedLength` here would mean the value channel had been
    /// swapped for a bracket read.
    #[test]
    fn an_underflowed_component_does_not_fire_the_gate_at_the_enclosure_scalar() {
        let tiny = Interval::from_f64(1e-180);
        let zero = Interval::from_f64(0.0);
        assert_eq!(
            UnitVec3::new(Vec3::new(tiny, zero, zero), "test_direction", band()).err(),
            Some(UnitVec3Error::Degenerate),
            "the enclosure lane decides against the band, as it did before"
        );
        // And the reason, measured: the enclosed norm is not the
        // point scalar's exact zero.
        let n = Vec3::new(tiny, zero, zero).norm();
        assert!(
            crate::real::Bounds::hi(n) > 0.0,
            "the enclosure retains the length the f64 lane loses"
        );
        // The zero vector is still the zero vector here too.
        assert_eq!(
            UnitVec3::new(Vec3::new(zero, zero, zero), "test_direction", band()).err(),
            Some(UnitVec3Error::Degenerate)
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod new_scaled_rows {
    use super::{Band, UnitVec3, UnitVec3Error, Vec3};
    use crate::interval::Interval;
    use crate::test_support::fixture_lever;

    /// **The levered length is decided as `|v · arm|` was, over the arm
    /// grid**: a unit-scale `v` levered at an ordinary arm mints its
    /// direction; an arm that puts the length inside the zero band, or
    /// between the band's edges, refuses or escalates; a negative arm is
    /// its magnitude; a non-finite or NaN arm, or one whose square
    /// overflows, is no length. Each at `f64` and at a point `Interval`.
    #[test]
    fn new_scaled_decides_the_levered_length_at_each_arm() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        fn rows<T: crate::predicate::Decide>(label: &str, band: Band) {
            let v = Vec3::new(T::from_f64(0.6), T::from_f64(0.8), T::from_f64(0.0));
            let at = |arm: f64| UnitVec3::new_scaled(v, fixture_lever(arm), "row", band);
            assert!(at(1.0).is_ok(), "{label}: an ordinary arm");
            assert!(at(-1.0).is_ok(), "{label}: a negative arm is its magnitude");
            assert!(
                matches!(at(0.0), Err(UnitVec3Error::Degenerate)),
                "{label}: zero"
            );
            assert!(
                matches!(at(1e-10), Err(UnitVec3Error::Degenerate)),
                "{label}: in the zero band"
            );
            assert!(
                matches!(at(5e-9), Err(UnitVec3Error::Escalated(_))),
                "{label}: between the edges"
            );
            for arm in [f64::INFINITY, f64::NEG_INFINITY, 1e300, f64::MAX, f64::NAN] {
                assert!(
                    matches!(at(arm), Err(UnitVec3Error::NonFiniteLength)),
                    "{label}: arm {arm:e} levers no length"
                );
            }
        }
        rows::<f64>("f64", band);
        rows::<Interval>("interval", band);
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod stated_rows {
    use super::{Mat3, UnitVec3, Vec3};
    use crate::dual::Dual64;
    use crate::interval::Interval;
    use crate::real::{Bounds, Real};

    fn bits(v: Vec3<f64>) -> [u64; 3] {
        [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()]
    }

    /// `x̂` stretched so that `v·v − 1` is `k` units of roundoff.
    fn off_unit(k: f64) -> Vec3<f64> {
        Vec3::new((1.0 + k * f64::EPSILON).sqrt(), 0.0, 0.0)
    }

    /// **The stated door's tolerance, from both sides**: a direction whose
    /// `v·v − 1` is within the charge of its terms (`rounding_charge` of
    /// `v·v + 1`, `16ε` about a unit vector) is adopted bit for bit, and
    /// one past it is refused, as is a direction that is not a finite
    /// number or names no direction. Red with the charge halved (the
    /// `14ε` direction refused) or doubled (the `18ε` one adopted).
    #[test]
    fn a_stated_direction_is_adopted_within_its_rounding_and_refused_past_it() {
        let within = off_unit(14.0);
        assert!((within.dot(within) - 1.0).abs() > 12.0 * f64::EPSILON);
        let adopted = UnitVec3::stated(within).expect("14ε: within the 16ε charge");
        assert_eq!(bits(adopted.get()), bits(within), "adopted bit for bit");
        let past = off_unit(18.0);
        assert!((past.dot(past) - 1.0).abs() < 20.0 * f64::EPSILON);
        assert!(UnitVec3::stated(past).is_none(), "18ε: past the 16ε charge");
        for (what, v) in [
            ("the zero vector", Vec3::new(0.0, 0.0, 0.0)),
            ("a chord", Vec3::new(3.0, 0.0, 4.0)),
            ("a NaN", Vec3::new(f64::NAN, 0.0, 1.0)),
            ("an infinity", Vec3::new(f64::INFINITY, 0.0, 0.0)),
        ] {
            assert!(UnitVec3::stated(v).is_none(), "{what}: no stated direction");
        }
    }

    /// **The stated door keeps a direction's bits where squaring it moves
    /// them**: the first of a fixed run of normalized directions whose
    /// square is not itself bit for bit (the corpus stores a few hundred
    /// line directions a last place off unit) is adopted as stated. Red
    /// with the door squaring ([`UnitVec3::squared`]) what it adopts.
    #[test]
    fn squaring_a_stated_direction_moves_its_bits_and_the_stated_door_does_not() {
        let mut state: u64 = 0x9e37_79b9_7f4a_7c15;
        let mut next = || {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            (state >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0
        };
        let moved = (0..10_000)
            .map(|_| Vec3::new(next(), next(), next()).normalize())
            .find(|v| bits(UnitVec3::squared(*v).get()) != bits(*v))
            .expect("some normalized direction squares to other bits");
        let adopted = UnitVec3::stated(moved).expect("a normalized direction is unit to rounding");
        assert_eq!(bits(adopted.get()), bits(moved), "the stated bits, kept");
    }

    /// **A checked direction keeps its bits at every scalar**: a
    /// construction unit over the reals passes the door as it was made,
    /// at `f64`, at `Interval` (an enclosure of a unit vector, the polar
    /// lift of a stated one) and at `Dual`, its tangent channel included,
    /// and the length it reports is exactly one. Red under the door
    /// squaring what it keeps.
    #[test]
    fn a_checked_direction_keeps_its_bits_at_every_scalar() {
        let (s, c) = 0.3_f64.sin_cos();
        let v = Vec3::new(s, 0.0, c);
        assert_eq!(bits(UnitVec3::checked(v).get()), bits(v), "f64");
        // At `Interval` from a genuine enclosure: the polar lift of the
        // stated direction, which a point box of its entries is not.
        let iv = UnitVec3::stated(v)
            .unwrap()
            .map_scalar(Interval::from_f64)
            .get();
        let got = UnitVec3::checked(iv).get();
        assert!(
            [got.x, got.y, got.z]
                .iter()
                .zip([iv.x, iv.y, iv.z])
                .all(|(a, b)| a.lo() == b.lo() && a.hi() == b.hi()),
            "Interval: {got:?}"
        );
        let t = Dual64::variable(0.3);
        let (ds, dc) = t.sin_cos();
        let dv = Vec3::new(ds, Dual64::constant(0.0), dc);
        let got = UnitVec3::checked(dv).get();
        assert!(
            got.x.value == s && got.x.deriv == c && got.z.deriv == -s,
            "Dual: {got:?}"
        );
        let (_, one) = UnitVec3::checked(v).through_isometry(Mat3::identity());
        assert_eq!(one.to_bits(), 1.0_f64.to_bits(), "kept, so no stretch");
    }

    /// **A refuted direction is poison in every build**: a chord of
    /// length 5, a NaN entry, and an enclosure that holds no unit vector
    /// each come out of the door poisoned, never a direction carried on,
    /// with no debug assertion between; a chord whose length overflowed or
    /// with an infinite entry is poison whole, its length too, and a zero
    /// chord's direction is poison beside its length, zero. Red under the
    /// door adopting a value its measurement refutes, and under the chord
    /// keeping a half-poisoned direction.
    #[test]
    fn a_refuted_direction_is_poison_in_every_build() {
        let poisoned = |u: UnitVec3<f64>| {
            let u = u.get();
            u.x.is_nan() && u.y.is_nan() && u.z.is_nan()
        };
        assert!(
            poisoned(UnitVec3::checked(Vec3::new(3.0, 0.0, 4.0))),
            "a chord"
        );
        assert!(
            poisoned(UnitVec3::checked(Vec3::new(f64::NAN, 0.0, 1.0))),
            "NaN"
        );
        for (v, what) in [
            (
                Vec3::new(1.7e308, 1.7e308, 0.0),
                "a chord whose length overflows",
            ),
            (Vec3::new(f64::INFINITY, 1.0, 0.0), "an infinite entry"),
        ] {
            let (d, len) = UnitVec3::chord(v);
            assert!(poisoned(d) && len.is_nan(), "{what}: {d:?}, {len}");
        }
        let (d, len) = UnitVec3::chord(Vec3::new(0.0, 0.0, 0.0));
        assert!(poisoned(d) && len == 0.0, "a zero chord: {d:?}, {len}");
        let far = Vec3::new(1.0, 0.0, 0.0).map(|x| Interval::from_bounds(x + 0.5, x + 0.6));
        let got = UnitVec3::checked(far).get();
        assert!(
            [got.x, got.y, got.z]
                .iter()
                .all(|e| e.told_reading().ends().is_none()),
            "an enclosure off the sphere: {got:?}"
        );
    }

    /// **An image past its rounding is squared, and says by how much**: a
    /// direction `1 + 40ε` long, past the door's `16ε` charge and far
    /// inside its reach, comes out squared, unit to rounding, with the
    /// length it was divided by; one within the charge keeps its bits
    /// and reports exactly one. Red under the door keeping what it should
    /// square, and under it squaring what it should keep.
    #[test]
    fn an_image_past_its_rounding_is_squared_and_says_by_how_much() {
        let long = Vec3::new(0.6, 0.8, 0.0) * (1.0 + 40.0 * f64::EPSILON);
        let (u, len) = UnitVec3::checked(Vec3::new(0.6, 0.8, 0.0)).through_isometry(
            Mat3::<f64>::identity().map(|x: f64| x * (1.0 + 40.0 * f64::EPSILON)),
        );
        assert!(
            UnitVec3::stated(u.get()).is_some(),
            "squared to unit: {u:?}"
        );
        assert!((len - long.norm()).abs() <= 2.0 * f64::EPSILON, "{len}");
        assert!(len > 1.0 + 30.0 * f64::EPSILON, "a stretch reported: {len}");
        let near = Vec3::new(0.6, 0.8, 0.0) * (1.0 + 2.0 * f64::EPSILON);
        let (kept, one) = UnitVec3::checked(Vec3::new(0.6, 0.8, 0.0))
            .through_isometry(Mat3::<f64>::identity().map(|x: f64| x * (1.0 + 2.0 * f64::EPSILON)));
        assert_eq!(bits(kept.get()), bits(near), "kept within the charge");
        assert_eq!(one.to_bits(), 1.0_f64.to_bits());
    }

    /// **The lift of a stated direction, per target** (D2's read): to
    /// `Interval` the polar enclosure of the unit vector nearest the
    /// stored one, which holds the stored bits; carried as it is between
    /// two scalars of one kind.
    #[test]
    fn a_stated_direction_lifts_to_its_enclosure_and_within_its_kind() {
        let v = UnitVec3::stated(Vec3::new(0.6, 0.8, 0.0)).unwrap();
        let at = v.map_scalar(Interval::from_f64).get();
        for (e, x) in [(at.x, 0.6), (at.y, 0.8), (at.z, 0.0)] {
            assert!(e.lo() <= x && x <= e.hi(), "the enclosure holds {x}: {e:?}");
        }
        let kept = v.map_scalar(|x: f64| x).get();
        assert_eq!(bits(kept), bits(v.get()), "f64 to f64, as it is");
        let lane = UnitVec3::checked(Vec3::new(
            Dual64::constant(0.6),
            Dual64::constant(0.8),
            Dual64::constant(0.0),
        ));
        let same = lane.map_scalar(|x: Dual64| x).get();
        assert!(
            same.x.value == 0.6 && same.y.value == 0.8,
            "Dual to Dual, as it is"
        );
    }

    /// **No read carries a stated direction into another kind's lane**:
    /// `f64` to a `Dual` over `Interval` fails a debug assertion (a release
    /// build answers poison). Into `f64`'s own lane, `Dual<f64>`, it is
    /// carried as it is, a constant.
    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "no read carries a direction into a lane")]
    fn a_stated_direction_does_not_lift_into_another_kinds_lane() {
        let v = UnitVec3::stated(Vec3::new(0.6, 0.8, 0.0)).unwrap();
        let constant = v.map_scalar(Dual64::constant).get();
        assert!(constant.x.value == 0.6 && constant.x.deriv == 0.0);
        let _ = v.map_scalar(|x| crate::dual::Dual::<Interval>::constant(Interval::from_f64(x)));
    }
}
