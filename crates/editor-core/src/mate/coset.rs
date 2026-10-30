//! **The coset algebra** (A11 (1); FORK-S3O): the subgroups a pose
//! kind's symmetry generates, and how a placement's constraints meet.
//!
//! Each mate pins a copy's relative pose to a COSET: a representative
//! pose times a residual SE(3) subgroup, the target pose's symmetry.
//! The subgroups are the closure of the five pose kinds' symmetries
//! (D10: a point, a direction, an axis, a plane, a frame) under
//! intersection, ten families — [`Subgroup::Se3`],
//! [`Subgroup::Planar`], [`Subgroup::Cylindrical`],
//! [`Subgroup::Prismatic`], [`Subgroup::Revolute`],
//! [`Subgroup::Spherical`], [`Subgroup::Parallel`],
//! [`Subgroup::Translation`], [`Subgroup::PlaneTranslation`],
//! [`Subgroup::Trivial`].
//!
//! # Subgroup algebra, measuring nothing
//!
//! A placement's constraints are admitted by their families alone
//! ([`SubgroupFamily::meet`], [`SubgroupFamily::transverse`]): a
//! constraint any of whose equations the others already fix refuses,
//! and whether one does is read off the families the bundle's
//! definitions name, never off the poses' values. Two families meet in
//! general position in the family [`SubgroupFamily::meet`] answers, and
//! a pair is admitted only where that meeting is transverse — its
//! codimension the sum of theirs. The meet's dimension is checked
//! against the intersection of the two Lie algebras, computed
//! independently in general position
//! (`intent_s3_a_poses::the_family_meet_spans_the_intersection_in_general_position`).
//!
//! The only transverse meeting two mates' families have is a ball's
//! with a direction's ([`intersect`]), whose coset is constructed and
//! never checked: each side's equation holds by construction, so
//! nothing is re-measured.

use geom_core::Real;
use geom_core::k_stats::decide;
use geom_core::linalg::{Affine3, Point3, UnitVec3, UnitVec3Error, Vec3};
use geom_core::predicate::{Band, Indeterminate, Margin, Sign};

use super::LeverRefusal;
use super::solve::SolveScalar;

/// A residual SE(3) subgroup — the closure set the table is closed
/// over. Its directions are [`UnitVec3`] witnesses: the predicates
/// below lever a sine or a cosine of them by the fold's arm into a
/// decided margin, and a direction that was not unit would scale that
/// margin silently, so the fact is carried by the type rather than by
/// a constructor's diligence.
///
/// At the scalar the solve runs at: a seed run's directions carry their
/// tangents, a box run's are enclosures.
#[derive(Debug, Clone, Copy)]
pub enum Subgroup<T: Real = f64> {
    /// Unconstrained: the fold's identity element (dim 6).
    Se3,
    /// The planar group of a plane with normal `normal`: the two
    /// in-plane translations plus rotation about the normal (dim 3).
    /// Point-free — rotations about ANY axis parallel to the normal
    /// are in it, so no base point distinguishes one plane from a
    /// parallel one.
    Planar {
        /// The plane's normal.
        normal: UnitVec3<T>,
    },
    /// The cylindrical group of a line: rotation about it plus
    /// translation along it (dim 2).
    Cylindrical {
        /// A point on the line.
        point: Point3<T>,
        /// The line's direction.
        direction: UnitVec3<T>,
    },
    /// Translation along a direction (dim 1). Point-free.
    Prismatic {
        /// The translation's direction.
        direction: UnitVec3<T>,
    },
    /// Rotation about a line (dim 1).
    Revolute {
        /// A point on the axis.
        point: Point3<T>,
        /// The axis's direction.
        direction: UnitVec3<T>,
    },
    /// The rotations about a point (dim 3): a point's symmetry.
    Spherical {
        /// The fixed point.
        point: Point3<T>,
    },
    /// Every translation plus rotation about axes along a direction
    /// (dim 4): a direction's symmetry. Point-free.
    Parallel {
        /// The direction.
        direction: UnitVec3<T>,
    },
    /// Every translation and no rotation (dim 3).
    Translation,
    /// The translations within a plane with normal `normal` and no
    /// rotation (dim 2). Point-free.
    PlaneTranslation {
        /// The plane's normal.
        normal: UnitVec3<T>,
    },
    /// The identity alone: DETERMINED (dim 0).
    Trivial,
}

/// Coordinate-wise equality of the linear types, which carry no
/// `PartialEq` of their own (the kernel keeps float comparison at the
/// decided-predicate door, deliberately). This is STRUCTURAL equality
/// for tests and diagnostics — never a geometric one; two subgroups
/// that are equal as SETS may differ here (a line stated at a
/// different base point), which is exactly why the algebra decides
/// coincidence through predicates instead of comparing values.
fn vec_eq(a: Vec3<f64>, b: Vec3<f64>) -> bool {
    a.to_array() == b.to_array()
}

fn point_eq(a: Point3<f64>, b: Point3<f64>) -> bool {
    a.to_array() == b.to_array()
}

impl PartialEq for Subgroup<f64> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Se3, Self::Se3) | (Self::Trivial, Self::Trivial) => true,
            (Self::Planar { normal: a }, Self::Planar { normal: b })
            | (Self::Prismatic { direction: a }, Self::Prismatic { direction: b }) => {
                vec_eq(a.get(), b.get())
            }
            (
                Self::Cylindrical {
                    point: pa,
                    direction: da,
                },
                Self::Cylindrical {
                    point: pb,
                    direction: db,
                },
            )
            | (
                Self::Revolute {
                    point: pa,
                    direction: da,
                },
                Self::Revolute {
                    point: pb,
                    direction: db,
                },
            ) => point_eq(*pa, *pb) && vec_eq(da.get(), db.get()),
            (Self::Spherical { point: pa }, Self::Spherical { point: pb }) => point_eq(*pa, *pb),
            (Self::Parallel { direction: a }, Self::Parallel { direction: b })
            | (Self::PlaneTranslation { normal: a }, Self::PlaneTranslation { normal: b }) => {
                vec_eq(a.get(), b.get())
            }
            (Self::Translation, Self::Translation) => true,
            // Different subgroups are unequal — spelled over the whole
            // lattice rather than swept up by a catch-all, so a
            // subgroup added to the closure must be given its own arm
            // above instead of comparing unequal to itself, which is
            // the one answer this structural equality can never give.
            (
                Self::Se3
                | Self::Planar { .. }
                | Self::Cylindrical { .. }
                | Self::Prismatic { .. }
                | Self::Revolute { .. }
                | Self::Spherical { .. }
                | Self::Parallel { .. }
                | Self::Translation
                | Self::PlaneTranslation { .. }
                | Self::Trivial,
                _,
            ) => false,
        }
    }
}

impl PartialEq for Coset<f64> {
    fn eq(&self, other: &Self) -> bool {
        // A field added to `Coset` is an E0027 at the two patterns
        // below. The placement is read through `Affine3::cols` and its
        // vectors through `Vec3::to_array` (in `vec_eq`), and each of
        // those doors binds its type's fields by pattern, so a field
        // added to `Affine3`, `Mat3` or `Vec3` is an E0027 there.
        let Self {
            subgroup,
            representative,
        } = self;
        let Self {
            subgroup: other_subgroup,
            representative: other_representative,
        } = other;
        subgroup == other_subgroup
            && representative
                .cols()
                .into_iter()
                .zip(other_representative.cols())
                .all(|(a, b)| vec_eq(a, b))
    }
}

/// **A [`Subgroup`]'s family**, without the line or normal that places
/// it: what a fold's residual is up to its parameters, and what a pose
/// kind's symmetry is (D10; [`crate::VarKind::symmetry`]): a plane is a
/// frame known up to the planar group about its normal, an axis up to
/// the cylindrical group about its line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SubgroupFamily {
    /// [`Subgroup::Se3`].
    Se3,
    /// [`Subgroup::Planar`].
    Planar,
    /// [`Subgroup::Cylindrical`].
    Cylindrical,
    /// [`Subgroup::Prismatic`].
    Prismatic,
    /// [`Subgroup::Revolute`].
    Revolute,
    /// [`Subgroup::Spherical`].
    Spherical,
    /// [`Subgroup::Parallel`].
    Parallel,
    /// [`Subgroup::Translation`].
    Translation,
    /// [`Subgroup::PlaneTranslation`].
    PlaneTranslation,
    /// [`Subgroup::Trivial`].
    Trivial,
}

impl SubgroupFamily {
    /// The family's dimension as a manifold.
    #[must_use]
    pub fn dimension(self) -> u8 {
        match self {
            Self::Se3 => 6,
            Self::Parallel => 4,
            Self::Planar | Self::Spherical | Self::Translation => 3,
            Self::Cylindrical | Self::PlaneTranslation => 2,
            Self::Prismatic | Self::Revolute => 1,
            Self::Trivial => 0,
        }
    }

    /// **How many slides a residual of this family leaves**: the
    /// dimension of the translations it frees — two beside a plane, one
    /// beside an axis, three beside a lone direction.
    #[must_use]
    pub fn slides(self) -> usize {
        match self {
            Self::Se3 | Self::Parallel | Self::Translation => 3,
            Self::Planar | Self::PlaneTranslation => 2,
            Self::Cylindrical | Self::Prismatic => 1,
            Self::Spherical | Self::Revolute | Self::Trivial => 0,
        }
    }

    /// The family's name, for messages and table rows.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Se3 => "SE(3)",
            Self::Planar => "planar",
            Self::Cylindrical => "cylindrical",
            Self::Prismatic => "prismatic",
            Self::Revolute => "revolute",
            Self::Spherical => "spherical",
            Self::Parallel => "parallel",
            Self::Translation => "translational",
            Self::PlaneTranslation => "planar translational",
            Self::Trivial => "trivial",
        }
    }
}

impl SubgroupFamily {
    /// The family's codimension in SE(3): how many equations a pose of
    /// this symmetry fixes.
    #[must_use]
    pub fn codimension(self) -> u8 {
        6 - self.dimension()
    }

    /// **Two families met in general position**: the family of the
    /// intersection of a subgroup of each, placed so that no special
    /// position holds between them (no two directions parallel or
    /// perpendicular, no point on a line). It is read off the families
    /// alone, so it measures nothing; a pair in special position meets
    /// in a larger subgroup, never a smaller one. Unordered.
    #[must_use]
    pub fn meet(self, other: Self) -> Self {
        use SubgroupFamily::{
            Cylindrical, Parallel, Planar, PlaneTranslation, Prismatic, Revolute, Se3, Spherical,
            Translation, Trivial,
        };
        match (self, other) {
            (Se3, g) | (g, Se3) => g,
            (Trivial, _) | (_, Trivial) => Trivial,
            // Two planes meet in the slide along their common line.
            (Planar, Planar) | (Planar, PlaneTranslation) | (PlaneTranslation, Planar) => Prismatic,
            (PlaneTranslation, PlaneTranslation) => Prismatic,
            (Planar, Cylindrical | Prismatic | Revolute)
            | (Cylindrical | Prismatic | Revolute, Planar) => Trivial,
            // A ball turns about the plane's normal through its point.
            (Planar, Spherical) | (Spherical, Planar) => Revolute,
            (Planar, Parallel | Translation) | (Parallel | Translation, Planar) => PlaneTranslation,
            // Two lines' groups, or a line's and a point's, share
            // nothing in general position.
            (
                Cylindrical | Prismatic | Revolute,
                Cylindrical | Prismatic | Revolute | Spherical | PlaneTranslation,
            )
            | (Spherical | PlaneTranslation, Cylindrical | Prismatic | Revolute) => Trivial,
            (Cylindrical | Prismatic, Parallel | Translation)
            | (Parallel | Translation, Cylindrical | Prismatic) => Prismatic,
            (Revolute, Parallel | Translation) | (Parallel | Translation, Revolute) => Trivial,
            // Two balls turn about the line through both points; a ball
            // and a direction about the line through the point along it.
            (Spherical, Spherical | Parallel) | (Parallel, Spherical) => Revolute,
            (Spherical, Translation | PlaneTranslation)
            | (Translation | PlaneTranslation, Spherical) => Trivial,
            (Parallel | Translation, Parallel | Translation) => Translation,
            (Parallel | Translation, PlaneTranslation)
            | (PlaneTranslation, Parallel | Translation) => PlaneTranslation,
        }
    }

    /// **Whether a subgroup of `added` meets one of `self`
    /// transversally** (FORK-S3O): the meeting's family where its
    /// codimension is the sum of the two, so neither fixes an equation
    /// the other already fixes; `None` where it is not, whatever the
    /// two subgroups' positions.
    #[must_use]
    pub fn transverse(self, added: Self) -> Option<Self> {
        let met = self.meet(added);
        (self.codimension() + added.codimension() == met.codimension()).then_some(met)
    }

    /// **Whether a subgroup of this family about a line holds every one
    /// of `motion`'s about the same line**: the cylindrical group holds
    /// the slide along its line and the spin about it, and the revolute
    /// group the spin.
    #[must_use]
    pub fn holds_on_its_line(self, motion: Self) -> bool {
        matches!(
            (self, motion),
            (
                Self::Cylindrical,
                Self::Prismatic | Self::Revolute | Self::Cylindrical
            ) | (Self::Revolute, Self::Revolute)
                | (Self::Prismatic, Self::Prismatic)
        )
    }

    /// Whether the family holds every translation.
    pub(crate) fn frees_every_translation(self) -> bool {
        matches!(self, Self::Se3 | Self::Parallel | Self::Translation)
    }

    /// Whether the family holds every rotation about some point.
    pub(crate) fn frees_every_rotation(self) -> bool {
        matches!(self, Self::Se3 | Self::Spherical)
    }
}

/// **A pose value's symmetry** (D10, A11 (1)): the subgroup of rigid
/// motions the value is a frame known up to, the one a mate on it
/// folds. A point forgets rotation about itself, a direction
/// translation and spin about itself, an axis slide and spin along its
/// line, a plane in-plane motion about its normal, and a frame nothing
/// ([`crate::PoseValue`]'s arms; [`crate::VarKind::symmetry`] names
/// their families, read off the same table).
pub trait PoseSymmetry<T: Real> {
    /// The subgroup.
    fn symmetry(&self) -> Subgroup<T>;
}

impl<T: Real> Subgroup<T> {
    /// The subgroup's family.
    pub fn family(&self) -> SubgroupFamily {
        match self {
            Self::Se3 => SubgroupFamily::Se3,
            Self::Planar { .. } => SubgroupFamily::Planar,
            Self::Cylindrical { .. } => SubgroupFamily::Cylindrical,
            Self::Prismatic { .. } => SubgroupFamily::Prismatic,
            Self::Revolute { .. } => SubgroupFamily::Revolute,
            Self::Spherical { .. } => SubgroupFamily::Spherical,
            Self::Parallel { .. } => SubgroupFamily::Parallel,
            Self::Translation => SubgroupFamily::Translation,
            Self::PlaneTranslation { .. } => SubgroupFamily::PlaneTranslation,
            Self::Trivial => SubgroupFamily::Trivial,
        }
    }

    /// The subgroup's dimension as a manifold.
    pub fn dimension(&self) -> u8 {
        self.family().dimension()
    }

    /// The subgroup's family name, for messages and table rows.
    pub fn name(&self) -> &'static str {
        self.family().name()
    }

    /// Whether this subgroup determines the pose outright.
    pub fn is_determined(&self) -> bool {
        matches!(self, Self::Trivial)
    }
}

impl Subgroup<f64> {
    /// The subgroup NAMED WITH ITS PARAMETERS — what A11 rule 4's
    /// refusal quotes, so an author reads which freedom survived and
    /// along which direction, not merely that one did.
    pub fn describe(&self) -> String {
        let dir = |u: &UnitVec3<f64>| {
            let v = u.get();
            format!("[{}, {}, {}]", v.x, v.y, v.z)
        };
        let pt = |p: &Point3<f64>| format!("({}, {}, {})", p.x, p.y, p.z);
        match self {
            Self::Se3 => "the full rigid freedom SE(3) — no mate constrains this pair".to_string(),
            Self::Planar { normal } => format!(
                "the planar freedom of the plane with normal {} (two in-plane translations and \
                 rotation about the normal)",
                dir(normal)
            ),
            Self::Cylindrical { point, direction } => format!(
                "the cylindrical freedom of the axis through {} along {} (rotation about it and \
                 translation along it)",
                pt(point),
                dir(direction)
            ),
            Self::Prismatic { direction } => format!(
                "the prismatic freedom of translation along {}",
                dir(direction)
            ),
            Self::Revolute { point, direction } => format!(
                "the revolute freedom of rotation about the axis through {} along {}",
                pt(point),
                dir(direction)
            ),
            Self::Spherical { point } => format!(
                "the spherical freedom of rotation about the point {}",
                pt(point)
            ),
            Self::Parallel { direction } => format!(
                "the freedom of every translation and rotation about axes along {}",
                dir(direction)
            ),
            Self::Translation => "the freedom of every translation, with no rotation".to_string(),
            Self::PlaneTranslation { normal } => format!(
                "the freedom of translation in the plane with normal {}, with no rotation",
                dir(normal)
            ),
            Self::Trivial => "no freedom — the pose is determined".to_string(),
        }
    }
}

/// A coset of [`Subgroup`]: the set `{ g · representative : g ∈ subgroup }`
/// of relative poses a placement's mates admit.
#[derive(Debug, Clone, Copy)]
pub struct Coset<T: Real = f64> {
    /// The residual freedom.
    pub subgroup: Subgroup<T>,
    /// One admitted pose (`b`'s part coordinates into `a`'s).
    pub representative: Affine3<T>,
}

impl<T: Real> Coset<T> {
    /// The unconstrained coset — the fold's identity element.
    pub fn unconstrained() -> Self {
        Self {
            subgroup: Subgroup::Se3,
            representative: Affine3::identity(),
        }
    }
}

// ---- The lever, and the decided predicates a value's chart and the report read ----

/// **The lever a mate's angular decisions turn on**, as the predicates
/// receive it: a length minted only by [`Arm::of`], which refuses one
/// the format cannot decide over.
///
/// Every levered predicate in this module multiplies the arm by a pure
/// number of magnitude at most 1 — a sine or a cosine of two unit
/// witnesses — and [`parallel`] hands that product to the direction
/// door, which squares it. An arm whose
/// sixteenfold square is finite keeps every one of those finite, so
/// such a margin is never infinite for want of range, which `Decide`
/// would read as maximally definite.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Arm(f64);

impl Arm {
    /// **The one door a lever is formed through**: the copy's reach
    /// `parts` plus the mates' poses' own distances from their origins
    /// `datum` (`mate::solve::bundle_arm`).
    ///
    /// # Errors
    ///
    /// [`LeverRefusal::OutOfRange`], carrying both halves, when the sum
    /// is not a number, or is a length whose sixteenfold square
    /// overflows the format.
    pub fn of(parts: f64, datum: f64) -> Result<Self, LeverRefusal> {
        let arm = parts + datum;
        if (arm.powi(2) * 16.0).is_finite() {
            Ok(Self(arm))
        } else {
            Err(LeverRefusal::OutOfRange { parts, datum })
        }
    }

    /// The arm, in metres.
    pub fn get(self) -> f64 {
        self.0
    }

    /// **The arm in `T`**: a bound read off the resolved origins, so it
    /// enters as a computed value ([`Real::from_computed`]), at `Sym`
    /// an unknown of its own. Each predicate mints it once and its levered
    /// product multiplies by that one value.
    pub fn lever<T: Real>(self) -> T {
        T::from_computed(self.0)
    }

    /// The longer of two arms, which is one of them.
    pub fn max(self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }

    /// **Whether an angle can be decided over this arm at `band`.** A
    /// levered margin is a pure number of magnitude at most a few
    /// times the arm, so at an arm no longer than the band's zero
    /// threshold every one of them reads zero: every pair of
    /// directions is parallel and perpendicular at once, and a verdict
    /// would be the band's, not the geometry's. Every site that decides
    /// an angle over an arm asks this first.
    ///
    /// # Errors
    ///
    /// [`LeverRefusal::BelowZeroBand`], carrying the arm and the
    /// threshold, when the arm is at or below `band.zero()`.
    pub fn decides_over(self, band: Band) -> Result<Self, LeverRefusal> {
        if self.0 > band.zero() {
            Ok(self)
        } else {
            Err(LeverRefusal::BelowZeroBand {
                arm: self.0,
                zero: band.zero(),
            })
        }
    }
}

/// Predicate: two directions are parallel (either sense), decided as
/// ONE mint. The margin is the sine of the angle between them — which
/// the cross product's length IS because both are witnesses — levered
/// by `arm` into the displacement it induces there, spelled as the
/// length of the levered vector `(u × v) · arm` and decided by the door
/// that mints the direction the non-parallel verdict hands out
/// ([`UnitVec3::new_scaled`]): `None` when parallel, `Some(line)` when
/// not — `line` the unit direction of `u × v`, the line two planes with
/// these normals meet in. It does not carry the arm, so a map built
/// along it is a function of its poses alone at every scalar, and the
/// arm stays opaque to the decision it levers.
///
/// The direction door's arms in this predicate's vocabulary: a
/// decided-zero length is the parallel verdict, and so is a levered
/// sine that underflowed the format (below ~1e-162, a length the band
/// calls zero at every ε; the door refuses it before the funnel, so
/// that vanishing sample is not recorded — reachable by no pair of
/// witnesses at any arm the reach bounds). A length that is no number
/// has no input to come from: the arm is an [`Arm`].
///
/// `lever` is that arm in `T` ([`Arm::lever`]), the decision's alone.
fn parallel<T: SolveScalar>(
    u: UnitVec3<T>,
    v: UnitVec3<T>,
    band: Band,
    lever: T,
) -> Result<Option<UnitVec3<T>>, Indeterminate> {
    match UnitVec3::new_scaled(u.get().cross(v.get()), lever, "mate_axes_parallel", band) {
        Ok(line) => Ok(Some(line)),
        Err(UnitVec3Error::Degenerate | UnitVec3Error::UnderflowedLength) => Ok(None),
        Err(UnitVec3Error::Escalated(diag)) => Err(diag),
        Err(UnitVec3Error::NonFiniteLength) => unreachable!(
            "a sine of two unit witnesses levered by an `Arm` has a finite squared length: \
             `Arm::of` admits no arm whose sixteenfold square overflows"
        ),
    }
}

/// Predicate: a direction is perpendicular to a normal. The margin is
/// the cosine — the dot product of two witnesses — levered into a
/// displacement at the arm `lever` ([`Arm::lever`]): `None` when
/// perpendicular, `Some(cosine)` when not, the cosine the funnel
/// decided out of the zero band and so nonzero.
fn perpendicular<T: SolveScalar>(
    u: UnitVec3<T>,
    n: UnitVec3<T>,
    band: Band,
    lever: T,
) -> Result<Option<T>, Indeterminate> {
    let cosine = u.get().dot(n.get());
    Ok(
        match decide(
            "mate_axis_normal_perpendicular",
            Margin::levered(cosine, lever),
            band,
        )? {
            Sign::Zero => None,
            Sign::Positive | Sign::Negative => Some(cosine),
        },
    )
}

/// Predicate: a point lies on a line. The margin is its perpendicular
/// offset — already a length, because the projector `d − u(d·u)` is
/// the orthogonal one exactly when `u` is unit, which the witness
/// carries.
fn point_on_line<T: SolveScalar>(
    p: Point3<T>,
    origin: Point3<T>,
    direction: UnitVec3<T>,
    band: Band,
) -> Result<bool, Indeterminate> {
    let d = p - origin;
    let u = direction.get();
    let off = d - u * d.dot(u);
    Ok(decide("mate_axis_point_offset", Margin::norm3(off), band)? == Sign::Zero)
}

// ---- The intersection proper ----

/// **Two cosets met where their families meet transversally**
/// ([`SubgroupFamily::transverse`]), `None` where they do not. The
/// coset is constructed, never checked: SE(3) meets anything as that
/// thing, and a ball meets a group whose rotations are fixed but for
/// at most a turn about one direction — a direction's, or the
/// translations' — in that group's rotation, its representative
/// carrying the ball's point home. Each side's equation holds by
/// construction, so nothing is measured, and the answer is the same in
/// either order.
pub fn intersect<T: Real>(held: Coset<T>, added: Coset<T>) -> Option<Coset<T>> {
    match (held.subgroup, added.subgroup) {
        (Subgroup::Se3, _) => Some(added),
        (_, Subgroup::Se3) => Some(held),
        (
            Subgroup::Spherical { point },
            turning @ (Subgroup::Parallel { .. } | Subgroup::Translation),
        ) => Some(ball_meets(point, held, turning, added)),
        (
            turning @ (Subgroup::Parallel { .. } | Subgroup::Translation),
            Subgroup::Spherical { point },
        ) => Some(ball_meets(point, added, turning, held)),
        _ => None,
    }
}

/// [`intersect`]'s constructed meeting of a ball's coset `ball` about
/// `point` with `other`'s, whose subgroup `turning` frees every
/// translation: the other's rotation, translated so `point` stays where
/// the ball puts it. What is left is the other's turns about the line
/// through `point` (a direction's), or nothing (the translations').
fn ball_meets<T: Real>(
    point: Point3<T>,
    ball: Coset<T>,
    turning: Subgroup<T>,
    other: Coset<T>,
) -> Coset<T> {
    let rotation = other.representative.linear;
    // The turn about `point` that carries the ball's representative to
    // `rotation`, applied to it.
    let turn = rotation * ball.representative.linear.transpose();
    let p = point - Point3::origin();
    let translation = p - turn * p + turn * ball.representative.translation;
    let subgroup = match turning {
        Subgroup::Parallel { direction } => Subgroup::Revolute { point, direction },
        _ => Subgroup::Trivial,
    };
    Coset {
        subgroup,
        representative: Affine3::from_parts(rotation, translation),
    }
}

// ---- A value's freedom, inside a residual ----

/// **The line a turn about `dir` must take to stay in `g`**
/// (ruling C-2): `None` where `g` holds no rotation about that
/// direction; `Some(Some(p))` where its rotations about it fix the
/// line through `p`; `Some(None)` where any line along `dir` will do
/// (`g` holds every translation across it).
///
/// # Errors
///
/// [`Indeterminate`] when the parallel verdict lands in the band.
pub(crate) fn turn_line<T: SolveScalar>(
    g: Subgroup<T>,
    dir: UnitVec3<T>,
    band: Band,
    arm: Arm,
) -> Result<Option<Option<Point3<T>>>, Indeterminate> {
    let lever = arm.lever();
    let along = |axis: UnitVec3<T>| parallel(dir, axis, band, lever).map(|apart| apart.is_none());
    Ok(match g {
        Subgroup::Se3 => Some(None),
        Subgroup::Spherical { point } => Some(Some(point)),
        Subgroup::Planar { normal: axis } | Subgroup::Parallel { direction: axis } => {
            along(axis)?.then_some(None)
        }
        Subgroup::Cylindrical { point, direction } | Subgroup::Revolute { point, direction } => {
            along(direction)?.then_some(Some(point))
        }
        Subgroup::Prismatic { .. }
        | Subgroup::Translation
        | Subgroup::PlaneTranslation { .. }
        | Subgroup::Trivial => None,
    })
}

/// **Whether `g` holds the translations along `v`** (ruling C-2).
///
/// # Errors
///
/// [`Indeterminate`] when the verdict lands in the band.
pub(crate) fn slides_along<T: SolveScalar>(
    g: Subgroup<T>,
    v: UnitVec3<T>,
    band: Band,
    arm: Arm,
) -> Result<bool, Indeterminate> {
    Ok(match g {
        Subgroup::Se3 | Subgroup::Parallel { .. } | Subgroup::Translation => true,
        Subgroup::Planar { normal } | Subgroup::PlaneTranslation { normal } => {
            perpendicular(v, normal, band, arm.lever())?.is_none()
        }
        Subgroup::Cylindrical { direction, .. } | Subgroup::Prismatic { direction } => {
            parallel(v, direction, band, arm.lever())?.is_none()
        }
        Subgroup::Revolute { .. } | Subgroup::Spherical { .. } | Subgroup::Trivial => false,
    })
}

/// **What `g` leaves once a value fixes its turn about its rotations'
/// direction**: `None` where the rest is no subgroup of the table (a
/// ball's or a free body's turns, less one).
pub(crate) fn after_turn<T: Real>(g: Subgroup<T>) -> Option<Subgroup<T>> {
    match g {
        Subgroup::Planar { normal } => Some(Subgroup::PlaneTranslation { normal }),
        Subgroup::Cylindrical { direction, .. } => Some(Subgroup::Prismatic { direction }),
        Subgroup::Revolute { .. } => Some(Subgroup::Trivial),
        Subgroup::Parallel { .. } => Some(Subgroup::Translation),
        Subgroup::Se3
        | Subgroup::Spherical { .. }
        | Subgroup::Prismatic { .. }
        | Subgroup::Translation
        | Subgroup::PlaneTranslation { .. }
        | Subgroup::Trivial => None,
    }
}

/// **Whether two directions are parallel**, decided over `arm`
/// ([`parallel`]'s verdict as a yes or no).
///
/// # Errors
///
/// [`Indeterminate`] when the verdict lands in the band.
pub(crate) fn directions_parallel<T: SolveScalar>(
    u: UnitVec3<T>,
    v: UnitVec3<T>,
    band: Band,
    arm: Arm,
) -> Result<bool, Indeterminate> {
    Ok(parallel(u, v, band, arm.lever())?.is_none())
}

/// **Whether `p` lies on the line through `origin` along `direction`**
/// ([`point_on_line`]).
///
/// # Errors
///
/// [`Indeterminate`] when the verdict lands in the band.
pub(crate) fn on_the_line<T: SolveScalar>(
    p: Point3<T>,
    origin: Point3<T>,
    direction: UnitVec3<T>,
    band: Band,
) -> Result<bool, Indeterminate> {
    point_on_line(p, origin, direction, band)
}
