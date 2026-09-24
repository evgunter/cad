//! The unit-vector **witness**: a 3-D direction whose unit length was
//! established by a decision, carried as a type so the fact survives a
//! function boundary instead of being re-stated as prose at every door
//! that needs it.
//!
//! # What the witness means, at every scalar
//!
//! A [`UnitVec3`] is a vector produced by a normalize whose length
//! DECIDED positive under a band — finite, not underflowed out of the
//! format, and definitely nonzero — or by an operation that maps a
//! unit vector to a unit vector exactly. It is not a bit-for-bit claim
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
//! - `-u` — negation is exact at every scalar.
//! - The exact basis axes and the cross product of an orthonormal
//!   pair, both private to [`linalg`](super) and both exact by
//!   construction rather than by decision — they exist so
//!   [`OrthoFrame`](super::OrthoFrame) can build the world frames and
//!   its own third axis, and they are documented on their own doors.
//!
//! The deciding ladders in [`frame`](super::frame) mint through the
//! constructor under their own funnel names and map its refusals onto
//! theirs; nothing else in this crate can produce the type, because the
//! field is private to this module and no other door here writes it.
//!
//! There is deliberately **no** "check that it is already unit"
//! constructor: a caller holding a bare vector it believes is unit has
//! either a decision to record — then the normalizing mint is exactly
//! that decision, at the cost of one divide that changes no bit of an
//! already-unit input the format holds exactly — or no decision, in
//! which case the type has nothing to witness.
//!
//! # What is NOT this type
//!
//! The geometry carriers' direction fields — `Line.dir`, `Plane.normal`,
//! the axes of the conics — stay bare `Vec3` under the at-rest rule
//! stated in `geom`'s crate docs (conventional, unchecked, certified
//! at rest by tier 3). The witness is a FUNCTION-boundary type. The
//! 2-D analogue, `profile::path::Dir<T>`, is that crate's own and is
//! left where it is.

use core::ops::Neg;

use crate::k_stats::decide;
use crate::linalg::Vec3;
use crate::predicate::{Band, Decide, Indeterminate, Margin, Sign};
use crate::real::{Real, is_finite_length, is_underflowed_length};

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
/// own [`Decide`] machinery calls maximally definite — a `Positive`
/// answer, followed by a division that collapses the direction to
/// zero. The constructor therefore asks whether the length is a finite
/// number BEFORE asking which side of zero it lies on
/// ([`UnitVec3Error::NonFiniteLength`]); without that order the type's
/// guarantee would be false exactly where it is least visible.
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
    /// [`Decide`] and would be normalized into a zero direction; a
    /// poisoned one has no direction either.
    NonFiniteLength,
    /// The vector's length UNDERFLOWED to zero: its components are
    /// nonzero but small enough (`|v| ≲ 1e-162` at `f64`) that the
    /// squared norm is exactly zero, so the length reads as zero for
    /// a vector that has a perfectly good direction. A separate fact
    /// from [`UnitVec3Error::Degenerate`] and a separate recourse:
    /// no tolerance makes this length nonzero, and normalizing it
    /// would blow the direction up to `±∞`.
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
                 overflows the norm or is not a number). Recourse: {}",
                crate::predicate::RANGE_RECOURSE
            ),
            Self::UnderflowedLength => write!(
                f,
                "a direction vector's length underflowed to zero, though it still \
                 names a direction. Recourse: {}",
                crate::predicate::RANGE_RECOURSE
            ),
            Self::Escalated(source) => {
                write!(f, "a direction vector's length is indeterminate: {source}")
            }
        }
    }
}

impl std::error::Error for UnitVec3Error {}

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
///    value channel ([`is_underflowed_length`]), against the largest
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
/// `DATUM_UNIT_NORM`), a transform axis or a pattern direction under
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
    // `norm3` below recomputes this same value (`Vec3::norm` is
    // deterministic), so the gate and the margin are the one length;
    // it is spelled twice rather than reached into.
    let len = v.norm();
    if !is_finite_length(len) {
        return Err(UnitVec3Error::NonFiniteLength);
    }
    // `norm_witness` is the largest |component|, which is the nonzero
    // WITNESS the underflow question is asked against; the derivation
    // and why the norm brackets it live on that door.
    if is_underflowed_length(len, v.norm_witness()) {
        return Err(UnitVec3Error::UnderflowedLength);
    }
    match decide(site, Margin::norm3(v), band) {
        Ok(Sign::Positive) => Ok(v.normalize()),
        Ok(_) => Err(UnitVec3Error::Degenerate),
        Err(source) => Err(UnitVec3Error::Escalated(source)),
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
    /// ([`OrthoFrame`](super::OrthoFrame)) establishes at its own
    /// mints, and the result is a product rather than a vector handed
    /// in. A caller holding two witnesses it merely believes are
    /// perpendicular has a decision to make, not a door to call, which
    /// is why this is private to [`linalg`](super).
    pub(in crate::linalg) fn cross_of_orthonormal(a: Self, b: Self) -> Self {
        Self(a.0.cross(b.0))
    }

    /// The exact `x̂`, unit as its literal bits at every scalar that
    /// represents 0 and 1 — no decision, nothing to decide. Private to
    /// [`linalg`](super), where [`OrthoFrame`](super::OrthoFrame)'s
    /// world frames are its only readers: a public exact-axis mint
    /// would be a door with no caller, and a caller wanting a
    /// direction it computed wants [`UnitVec3::new`].
    pub(in crate::linalg) fn exact_x() -> Self {
        Self(Vec3::unit_x())
    }

    /// The exact `ŷ`; see [`UnitVec3::exact_x`].
    pub(in crate::linalg) fn exact_y() -> Self {
        Self(Vec3::unit_y())
    }

    /// The exact `ẑ`; see [`UnitVec3::exact_x`].
    pub(in crate::linalg) fn exact_z() -> Self {
        Self(Vec3::unit_z())
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

    fn bits3(v: Vec3<f64>) -> [u64; 3] {
        [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()]
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
        // refuses BEFORE the sign of the length is asked for. An
        // infinite margin is maximally definite to `sign_within`, so
        // deciding first would answer Positive and normalize the
        // direction into the zero vector.
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
        // What the refusal prevents, executed rather than asserted:
        // the normalization these components go through collapses the
        // direction to the ZERO vector, which is DEFINITELY non-unit.
        let collapsed = Vec3::new(1e200, 0.0, 0.0).normalize();
        assert_eq!((collapsed.x, collapsed.y, collapsed.z), (0.0, 0.0, 0.0));
        let tripwire = Band::new(1e-9, 2e-9).expect("a well-ordered band");
        assert_eq!(
            (collapsed.dot(collapsed) - 1.0).sign_within(tripwire),
            Ok(Sign::Negative)
        );
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
}

// The enclosure scalar's rows: an enclosure that contains unit length
// is a direction, an overflowed enclosure stays sound, only a length
// the enclosure DECIDES is zero refuses, and an enclosure that merely
// cannot tell escalates rather than refusing.
#[cfg(test)]
#[cfg(feature = "interval")]
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
                off.sign_within(band()),
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
            !matches!(off.sign_within(band()), Ok(Sign::Positive | Sign::Negative)),
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
