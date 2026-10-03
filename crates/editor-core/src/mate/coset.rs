//! **The coset algebra** — the binding class×class intersection table
//! (ASM-R2a D-4; ASM-R2-SPEC-DRAFT "R2-a coset table"; A11 rule 1).
//!
//! Each mate primitive pins a pair's relative pose to a COSET: a
//! representative pose times a residual SE(3) subgroup. Folding a
//! pair's mates is exact coset intersection, and because intersection
//! is order-independent the table must cover the CLOSURE of the
//! primitive set. That closure is seven entries —
//! [`Subgroup::Se3`], [`Subgroup::Planar`], [`Subgroup::Cylindrical`],
//! [`Subgroup::Prismatic`], [`Subgroup::Revolute`],
//! [`Subgroup::Trivial`], [`Subgroup::Empty`] — and its closedness is
//! a PROOF OBLIGATION, executed as the enumeration test rather than
//! asserted in prose. No screw subgroup arises: the primitives'
//! parallel-cylinder intersection is pure translation.
//!
//! # Why the intersection is two stages, not a table of formulas
//!
//! A coset `G·R` intersects `G′·R′` in either the empty set or a coset
//! of `G ∩ G′` — elementary, and it splits the work exactly:
//!
//! 1. **The subgroup** ([`intersect_subgroups`]) is the table's case
//!    splits — every one a decided predicate through the `k_stats`
//!    funnel with a NAMED predicate, Indeterminate escalating typed.
//! 2. **The representative** is then constructed to satisfy the HELD
//!    side exactly and checked against the added side, so a failure's
//!    measured clash is the added mate's own violation rather than an
//!    averaged residue — which is what a CONTRADICTORY refusal must
//!    quote.
//!
//! Membership is itself a decided predicate per subgroup, so step 2's
//! check is the same funnel: "an unsatisfiable candidate is empty,
//! named", the table's own closing sentence.

use geom_core::k_stats::decide;
use geom_core::linalg::{Affine3, Mat3, Point3, UnitVec3, UnitVec3Error, Vec3};
use geom_core::predicate::{Band, Indeterminate, Margin, Sign};
use geom_core::{Real, is_finite_length};

use super::solve::SolveScalar;
use super::{Clash, Lever, LeverRefusal, Refuted};

/// A residual SE(3) subgroup — the closure set the table is closed
/// over. Its directions are [`UnitVec3`] witnesses: the predicates
/// below lever a sine or a cosine of them by the fold's arm into a
/// decided margin, and a direction that was not unit would scale that
/// margin silently, so the fact is carried by the type rather than by
/// a constructor's diligence. A subgroup's parameters are exactly what
/// an UNDER refusal must name.
///
/// At the scalar the solve runs at: a seed run's directions carry their
/// tangents, a box run's are enclosures. A refusal names its residual
/// at `f64` ([`SolveScalar::quoted_residual`]).
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
    /// The identity alone: DETERMINED (dim 0).
    Trivial,
    /// No pose satisfies the fold: CONTRADICTORY. Carried as a closure
    /// element rather than an error type so `X ∩ empty = empty` is a
    /// table entry that can be enumerated like every other.
    Empty,
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
            (Self::Se3, Self::Se3)
            | (Self::Trivial, Self::Trivial)
            | (Self::Empty, Self::Empty) => true,
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
                | Self::Trivial
                | Self::Empty,
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

impl<T: Real> Subgroup<T> {
    /// The subgroup's dimension as a manifold, `None` for
    /// [`Subgroup::Empty`] (which is not a subgroup at all).
    pub fn dimension(&self) -> Option<u8> {
        Some(match self {
            Self::Se3 => 6,
            Self::Planar { .. } => 3,
            Self::Cylindrical { .. } => 2,
            Self::Prismatic { .. } | Self::Revolute { .. } => 1,
            Self::Trivial => 0,
            Self::Empty => return None,
        })
    }

    /// The subgroup's family name, for messages and table rows.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Se3 => "SE(3)",
            Self::Planar { .. } => "planar",
            Self::Cylindrical { .. } => "cylindrical",
            Self::Prismatic { .. } => "prismatic",
            Self::Revolute { .. } => "revolute",
            Self::Trivial => "trivial",
            Self::Empty => "empty",
        }
    }

    /// Whether this subgroup determines the pose outright.
    pub fn is_determined(&self) -> bool {
        matches!(self, Self::Trivial)
    }

    /// The rotations this subgroup contains, as the axis they all fix.
    fn rotations(&self) -> Rotations<T> {
        match self {
            Self::Se3 => Rotations::Free,
            Self::Planar { normal } => Rotations::About(*normal),
            Self::Cylindrical { direction, .. } | Self::Revolute { direction, .. } => {
                Rotations::About(*direction)
            }
            Self::Prismatic { .. } | Self::Trivial | Self::Empty => Rotations::Fixed,
        }
    }

    /// How many independent PURE translations this subgroup contains —
    /// the freedom a candidate translation may keep.
    fn translation_dimension(&self) -> u8 {
        match self {
            Self::Se3 => 3,
            Self::Planar { .. } => 2,
            Self::Cylindrical { .. } | Self::Prismatic { .. } => 1,
            Self::Revolute { .. } | Self::Trivial | Self::Empty => 0,
        }
    }

    /// The point `q` this subgroup's translation CONSTRAINT is anchored
    /// at, given the candidate's rotation `a` relative to the coset
    /// representative's and the representative's translation `r`:
    /// membership reads `P·(t − q) = 0`, with `P` the projector onto
    /// the directions the subgroup does not translate along — the
    /// normal of a planar, the plane across a prismatic's or
    /// cylindrical's direction, every direction for the rest.
    ///
    /// Why a point enters at all: a rotation about a line NOT through
    /// the origin carries a translation `(I − Q)p`, so the affine part
    /// cannot be split off and read separately. A revolute's elements
    /// fix their axis pointwise, a cylindrical's move it only along
    /// itself, a planar's move no point across the plane.
    fn translation_anchor(&self, a: Mat3<T>, r: Vec3<T>) -> Vec3<T> {
        let ar = a * r;
        match self {
            Self::Se3
            | Self::Trivial
            | Self::Empty
            | Self::Prismatic { .. }
            | Self::Planar { .. } => ar,
            Self::Cylindrical { point, .. } | Self::Revolute { point, .. } => {
                let p = *point - Point3::origin();
                p - a * p + ar
            }
        }
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
            Self::Trivial => "no freedom — the pose is determined".to_string(),
            Self::Empty => "nothing — the mates cannot both hold".to_string(),
        }
    }
}

/// The rotations a subgroup contains.
#[derive(Debug, Clone, Copy)]
enum Rotations<T: Real> {
    /// All of SO(3).
    Free,
    /// Exactly the rotations about this axis.
    About(UnitVec3<T>),
    /// The identity alone.
    Fixed,
}

/// A coset of [`Subgroup`]: the set `{ g · representative : g ∈ subgroup }`
/// of relative poses a pair's mates admit.
///
/// The representative is meaningless when the subgroup is
/// [`Subgroup::Empty`]; every consumer must read the subgroup first.
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

/// Why a fold step could not produce a coset.
#[derive(Debug, Clone, PartialEq)]
pub enum FoldStop {
    /// A case split landed in the ambiguity band.
    Indeterminate(Box<Indeterminate>),
    /// The intersection is empty: the named predicate measured a clash
    /// where the cosets would have had to meet.
    Clash {
        /// The membership predicate that refused.
        predicate: &'static str,
        /// What it measured.
        clash: Clash,
    },
    /// The cosets meet further away than the solve can measure a
    /// distance: the candidate's translation, or a length membership
    /// measures on it, squares past the format (a length is a square
    /// root, so a translation whose every component is representable
    /// can still have none). Over an arm the angles are decidable at
    /// ([`FoldStop::Unleverable`] answers the rest first), the solve
    /// divides only by numbers the table decided away from zero
    /// ([`candidate_translation`]), so this is the meeting point's own
    /// distance, not the solve's conditioning.
    OutOfRange,
    /// The fold's arm is no lever an angle can be decided over at
    /// this band ([`Arm::decides_over`]); refused before any angle is.
    Unleverable(LeverRefusal),
}

impl From<Indeterminate> for FoldStop {
    fn from(value: Indeterminate) -> Self {
        Self::Indeterminate(Box::new(value))
    }
}

// ---- The lever, and the table's case splits, each a named decided predicate ----

/// **The lever a mate's angular decisions turn on**, as the predicates
/// receive it: a length minted only by [`Arm::of`], which refuses one
/// the format cannot decide over.
///
/// Every levered predicate in this module multiplies the arm by a pure
/// number of magnitude below 4 — a sine or a cosine of two unit
/// witnesses, a departure of a rotation from the identity (at most
/// `2√3`), a reachability defect (at most 2) — and [`parallel`] hands
/// that product to the direction door, which squares it. An arm whose
/// sixteenfold square is finite keeps every one of those finite, so
/// such a margin is never infinite for want of range, which `Decide`
/// would read as maximally definite. **One exception**: the clocking
/// rider's roll (`Lever::Roll`, `mate/solve.rs`) levers the authored
/// clocking unreduced, which no bound holds, so its margin can still
/// overflow (`work/msolve/a-clocking-rider-is-levered-unreduced.md`).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Arm(f64);

impl Arm {
    /// **The one door a lever is formed through**: the mated parts'
    /// reach `parts` (`R_a + R_b`) plus the datum's own terms `datum`
    /// ([`super::Alignment::lever_arm`]).
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
/// length of the levered vector `(u × v) · arm` so that the direction
/// the non-parallel verdict hands out is minted by the very decision
/// that made it: `None` when parallel, `Some((line, sine))` when not
/// — `line` the unit direction of `u × v`, the line two planes with
/// these normals meet in, and `sine` the levered vector's length, in
/// metres, which the door decided at least `K·ε`. A second decision
/// of the same number at a mint would be spelled a rounding apart and
/// could land in the band where this one did not; the length handed
/// out is the door's own measure of that vector (`Vec3::norm`, which
/// is deterministic).
///
/// The direction door's arms in this predicate's vocabulary: a
/// decided-zero length is the parallel verdict, and so is a levered
/// sine that underflowed the format (below ~1e-162, a length the band
/// calls zero at every ε; the door refuses it before the funnel, so
/// that vanishing sample is not recorded — reachable by no pair of
/// witnesses at any arm the reach bounds). A length that is no number
/// has no input to come from: the arm is an [`Arm`].
fn parallel<T: SolveScalar>(
    u: UnitVec3<T>,
    v: UnitVec3<T>,
    band: Band,
    arm: Arm,
) -> Result<Option<(UnitVec3<T>, T)>, Indeterminate> {
    let levered = u.get().cross(v.get()) * T::from_f64(arm.get());
    match UnitVec3::new(levered, "mate_axes_parallel", band) {
        Ok(line) => Ok(Some((line, levered.norm()))),
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
/// displacement at `arm`: `None` when perpendicular, `Some(cosine)`
/// when not, the cosine the funnel decided out of the zero band and so
/// nonzero.
fn perpendicular<T: SolveScalar>(
    u: UnitVec3<T>,
    n: UnitVec3<T>,
    band: Band,
    arm: Arm,
) -> Result<Option<T>, Indeterminate> {
    let cosine = u.get().dot(n.get());
    Ok(
        match decide(
            "mate_axis_normal_perpendicular",
            Margin::levered(cosine, T::from_f64(arm.get())),
            band,
        )? {
            Sign::Zero => None,
            Sign::Positive | Sign::Negative => Some(cosine),
        },
    )
}

/// **The number a table verdict decided away from zero to separate a
/// pair's freedoms** — handed to the translation stage, which divides
/// by it and by nothing else that could be small.
#[derive(Debug, Clone, Copy)]
enum Separated<T: Real> {
    /// No angular verdict separated them: the residual keeps every
    /// freedom the pair shares, or the verdict was the parallel or
    /// perpendicular one.
    Not,
    /// Two directions called non-parallel: the levered sine
    /// `|u × v| · arm`, in metres, at least `K·ε` ([`parallel`]).
    Sine(T),
    /// A direction and a normal called non-perpendicular: their
    /// cosine, out of the zero band once levered ([`perpendicular`]).
    Cosine(T),
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

/// **The subgroup half of the binding table**: `G ∩ G′`, every case
/// split decided through the funnel.
///
/// Unordered as the table is: every entry states BOTH orders as one
/// arm, which is also what makes the match EXHAUSTIVE without a
/// catch-all — a new closure element would fail to compile here rather
/// than fall into a silent default, which is the enumeration's whole
/// point. `arm` is the lever the angular splits turn on (D4 ¶1: an
/// angle decides only through the displacement it induces at a named
/// length).
///
/// # Errors
///
/// [`Indeterminate`] when a case split lands in the ambiguity band —
/// the typed escalation the spec demands, never a silent pick.
pub fn intersect_subgroups<T: SolveScalar>(
    g1: Subgroup<T>,
    g2: Subgroup<T>,
    band: Band,
    arm: Arm,
) -> Result<Subgroup<T>, Indeterminate> {
    table(g1, g2, band, arm).map(|(residual, _)| residual)
}

/// [`intersect_subgroups`], with the number its verdict decided away
/// from zero ([`Separated`]).
fn table<T: SolveScalar>(
    g1: Subgroup<T>,
    g2: Subgroup<T>,
    band: Band,
    arm: Arm,
) -> Result<(Subgroup<T>, Separated<T>), Indeterminate> {
    use Subgroup::{Cylindrical, Empty, Planar, Prismatic, Revolute, Se3, Trivial};
    let not = |g| (g, Separated::Not);
    Ok(match (g1, g2) {
        // The universal entries: empty absorbs, SE(3) is the identity,
        // trivial is the zero. Stated first, and between them they
        // cover every tuple touching those three — so what remains
        // below is exactly the four proper families.
        (Empty, _) | (_, Empty) => not(Empty),
        (Se3, other) | (other, Se3) => not(other),
        (Trivial, _) | (_, Trivial) => not(Trivial),
        // 1. planar ∩ planar — the flush merge and the V-block.
        (Planar { normal: n1 }, Planar { normal: n2 }) => {
            match parallel(n1, n2, band, arm)? {
                None => not(Planar { normal: n1 }),
                // The line the two planes meet in, as the verdict
                // minted it.
                Some((direction, sine)) => (Prismatic { direction }, Separated::Sine(sine)),
            }
        }
        // 2. planar ∩ cylindrical — pin-in-hole, slot, and the generic
        //    angle that kills both freedoms.
        (
            Planar { normal: n },
            Cylindrical {
                point: p,
                direction: u,
            },
        )
        | (
            Cylindrical {
                point: p,
                direction: u,
            },
            Planar { normal: n },
        ) => {
            if parallel(u, n, band, arm)?.is_none() {
                not(Revolute {
                    point: p,
                    direction: u,
                })
            } else {
                match perpendicular(u, n, band, arm)? {
                    None => not(Prismatic { direction: u }),
                    Some(cosine) => (Trivial, Separated::Cosine(cosine)),
                }
            }
        }
        // 3. planar ∩ prismatic.
        (Planar { normal: n }, Prismatic { direction: d })
        | (Prismatic { direction: d }, Planar { normal: n }) => {
            match perpendicular(d, n, band, arm)? {
                None => not(Prismatic { direction: d }),
                Some(cosine) => (Trivial, Separated::Cosine(cosine)),
            }
        }
        // 4. planar ∩ revolute.
        (
            Planar { normal: n },
            Revolute {
                point: p,
                direction: u,
            },
        )
        | (
            Revolute {
                point: p,
                direction: u,
            },
            Planar { normal: n },
        ) => {
            if parallel(u, n, band, arm)?.is_none() {
                not(Revolute {
                    point: p,
                    direction: u,
                })
            } else {
                not(Trivial)
            }
        }
        // 5. cylindrical ∩ cylindrical — same line, parallel-distinct,
        //    or concurrent/skew.
        (
            Cylindrical {
                point: p1,
                direction: u1,
            },
            Cylindrical {
                point: p2,
                direction: u2,
            },
        ) => match parallel(u1, u2, band, arm)? {
            None => {
                if point_on_line(p2, p1, u1, band)? {
                    not(Cylindrical {
                        point: p1,
                        direction: u1,
                    })
                } else {
                    not(Prismatic { direction: u1 })
                }
            }
            Some((_, sine)) => (Trivial, Separated::Sine(sine)),
        },
        // 6. cylindrical ∩ prismatic.
        (Cylindrical { direction: u, .. }, Prismatic { direction: d })
        | (Prismatic { direction: d }, Cylindrical { direction: u, .. }) => {
            match parallel(d, u, band, arm)? {
                None => not(Prismatic { direction: d }),
                Some((_, sine)) => (Trivial, Separated::Sine(sine)),
            }
        }
        // 7. cylindrical ∩ revolute.
        (
            Cylindrical {
                point: pc,
                direction: uc,
            },
            Revolute {
                point: pr,
                direction: ur,
            },
        )
        | (
            Revolute {
                point: pr,
                direction: ur,
            },
            Cylindrical {
                point: pc,
                direction: uc,
            },
        ) => {
            if parallel(uc, ur, band, arm)?.is_none() && point_on_line(pr, pc, uc, band)? {
                not(Revolute {
                    point: pr,
                    direction: ur,
                })
            } else {
                not(Trivial)
            }
        }
        // 8. prismatic ∩ prismatic.
        (Prismatic { direction: d1 }, Prismatic { direction: d2 }) => {
            match parallel(d1, d2, band, arm)? {
                None => not(Prismatic { direction: d1 }),
                Some((_, sine)) => (Trivial, Separated::Sine(sine)),
            }
        }
        // 9. prismatic ∩ revolute — a translation is never a rotation.
        (Prismatic { .. }, Revolute { .. }) | (Revolute { .. }, Prismatic { .. }) => not(Trivial),
        // 10. revolute ∩ revolute.
        (
            Revolute {
                point: p1,
                direction: u1,
            },
            Revolute {
                point: p2,
                direction: u2,
            },
        ) => {
            if parallel(u1, u2, band, arm)?.is_none() && point_on_line(p2, p1, u1, band)? {
                not(Revolute {
                    point: p1,
                    direction: u1,
                })
            } else {
                not(Trivial)
            }
        }
    })
}

// ---- Membership: the decided predicate the representative meets ----

/// **What one predicate measured, before it is decided** — a length
/// outright, or a pure number it levers by an arm. The one home of
/// the levered margin in the solve: every levered site names its
/// number here, decides [`Measured::margin`] and, refusing, quotes
/// [`Measured::clash`], so the number the sentence prints is the
/// number the funnel decided on.
///
/// At the solve's scalar; the arm is the reach's `f64` upper bound and
/// an authored roll is the datum's own `f64`, so only what the solve
/// computed is at `T`. The refusal quotes it at `f64`
/// ([`SolveScalar::quoted`]), the number the margin's own lane reads.
#[derive(Debug, Clone, Copy)]
pub(super) enum Measured<T: Real> {
    /// A length, in metres.
    Length(T),
    /// A dimensionless residual and the arm that levers it
    /// ([`Lever::Residual`]).
    Residual {
        /// The residual, a pure number.
        value: T,
        /// The arm, in metres.
        arm: f64,
    },
    /// An authored roll and the arm that levers it ([`Lever::Roll`]).
    Roll {
        /// The roll, in radians.
        radians: f64,
        /// The arm, in metres.
        arm: f64,
    },
}

impl<T: SolveScalar> Measured<T> {
    /// The margin the predicate decides: [`Lever::margin`]'s product at
    /// `T` for a levered number, the length itself for a length.
    pub(super) fn margin(self) -> Margin<T> {
        match self {
            Self::Length(m) => Margin::of(m),
            Self::Residual { value, arm } => Margin::levered(value, T::from_f64(arm)),
            Self::Roll { radians, arm } => Margin::levered(T::from_f64(radians), T::from_f64(arm)),
        }
    }

    /// The measurement, as the refusal quotes it.
    pub(super) fn clash(self) -> Clash {
        match self {
            Self::Length(metres) => Clash::Length {
                metres: metres.quoted(),
            },
            Self::Residual { value, arm } => Clash::Levered(Lever::Residual {
                value: value.quoted(),
                arm,
            }),
            Self::Roll { radians, arm } => Clash::Levered(Lever::Roll { radians, arm }),
        }
    }
}

/// Whether `x` lies in `g` (as a subgroup of SE(3), not a coset).
///
/// # Errors
///
/// [`FoldStop::Clash`] naming the failing predicate with what it
/// measured — the CONTRADICTORY refusal's own quotation —,
/// [`FoldStop::Indeterminate`] when a check landed in the band, or
/// [`FoldStop::OutOfRange`] when a length it measures has none the
/// format can hold.
fn member_of<T: SolveScalar>(
    g: Subgroup<T>,
    x: Affine3<T>,
    band: Band,
    arm: Arm,
) -> Result<(), FoldStop> {
    let arm = arm.get();
    let residual = |value: T| Measured::Residual { value, arm };
    let axis_fixed = |axis: UnitVec3<T>| {
        (
            Refuted::AxisFixed,
            residual((x.linear * axis.get() - axis.get()).norm()),
        )
    };
    let rotation_identity = || {
        (
            Refuted::RotationIdentity,
            residual(rotation_residual(x.linear)),
        )
    };
    let checks: Vec<(Refuted, Measured<T>)> = match g {
        // The empty set holds nothing, and no margin decides that —
        // the answer is structural, so it never reaches the funnel.
        Subgroup::Empty => {
            return Err(FoldStop::Clash {
                predicate: super::MATE_MEMBER_EMPTY,
                clash: Clash::Structural,
            });
        }
        Subgroup::Se3 => Vec::new(),
        Subgroup::Trivial => vec![
            rotation_identity(),
            (
                Refuted::TranslationZero,
                Measured::Length(x.translation.norm()),
            ),
        ],
        Subgroup::Prismatic { direction } => vec![
            rotation_identity(),
            (
                Refuted::TranslationAlong,
                Measured::Length(x.translation.reject_from(direction.get()).norm()),
            ),
        ],
        Subgroup::Planar { normal } => vec![
            axis_fixed(normal),
            (
                Refuted::TranslationInPlane,
                Measured::Length(x.translation.dot(normal.get())),
            ),
        ],
        Subgroup::Cylindrical { point, direction } => vec![
            axis_fixed(direction),
            (
                Refuted::PointOnAxis,
                Measured::Length(
                    (x.transform_point(point) - point)
                        .reject_from(direction.get())
                        .norm(),
                ),
            ),
        ],
        Subgroup::Revolute { point, direction } => vec![
            axis_fixed(direction),
            (
                Refuted::PointFixed,
                Measured::Length((x.transform_point(point) - point).norm()),
            ),
        ],
    };
    for (predicate, measured) in checks {
        if matches!(measured, Measured::Length(m) if !is_finite_length(m)) {
            return Err(FoldStop::OutOfRange);
        }
        if decide(predicate.name(), measured.margin(), band)? != Sign::Zero {
            return Err(FoldStop::Clash {
                predicate: predicate.name(),
                clash: measured.clash(),
            });
        }
    }
    Ok(())
}

/// **Whether `x` is the identity**, decided as membership of the
/// trivial subgroup over `arm` — the test a checked offset meets
/// against the solve (A11 (2)).
///
/// # Errors
///
/// [`member_of`]'s, and [`FoldStop::Unleverable`] when `arm` decides no
/// angle at `band` ([`Arm::decides_over`]).
pub(super) fn trivial_member<T: SolveScalar>(
    x: Affine3<T>,
    band: Band,
    arm: Arm,
) -> Result<(), FoldStop> {
    let arm = arm.decides_over(band).map_err(FoldStop::Unleverable)?;
    member_of(Subgroup::Trivial, x, band, arm)
}

/// A rotation's departure from the identity as a pure number: the
/// Frobenius norm of `Q − I`. The caller levers it, so the number a
/// refusal quotes is the one the predicate decided on.
fn rotation_residual<T: Real>(q: Mat3<T>) -> T {
    let d = sub(q, Mat3::identity());
    (d.c0.norm_squared() + d.c1.norm_squared() + d.c2.norm_squared()).sqrt()
}

// ---- The intersection proper ----

/// **Intersect two cosets** — the table, executed.
///
/// The result's representative satisfies `held` exactly by
/// construction and `added` up to the decided membership check, so a
/// [`FoldStop::Clash`] quotes the ADDED mate's own violation. `arm` is
/// the lever the angular decisions turn on.
///
/// # Errors
///
/// [`FoldStop::Indeterminate`] when a case split or membership check
/// lands in the ambiguity band; [`FoldStop::Clash`] when the
/// intersection is empty; [`FoldStop::OutOfRange`] when the meeting
/// point is further than a length can say; [`FoldStop::Unleverable`]
/// when `arm` decides no angle at `band`.
pub fn intersect<T: SolveScalar>(
    held: Coset<T>,
    added: Coset<T>,
    band: Band,
    arm: Arm,
) -> Result<Coset<T>, FoldStop> {
    if matches!(held.subgroup, Subgroup::Empty) || matches!(added.subgroup, Subgroup::Empty) {
        return Ok(Coset {
            subgroup: Subgroup::Empty,
            representative: held.representative,
        });
    }
    // X ∩ SE(3) = X, verbatim: nothing to construct and nothing to
    // check, so the fold's identity element costs no arithmetic.
    if matches!(held.subgroup, Subgroup::Se3) {
        return Ok(added);
    }
    if matches!(added.subgroup, Subgroup::Se3) {
        return Ok(held);
    }
    let arm = arm.decides_over(band).map_err(FoldStop::Unleverable)?;
    let (residual, separated) = table(held.subgroup, added.subgroup, band, arm)?;
    let rotation = candidate_rotation(held, added, band, arm)?;
    let translation = candidate_translation(held, added, residual, separated, rotation, arm);
    if !is_finite_length(translation.norm()) {
        return Err(FoldStop::OutOfRange);
    }
    let x = Affine3::from_parts(rotation, translation);
    for coset in [held, added] {
        member_of(
            coset.subgroup,
            x * coset.representative.inverse(),
            band,
            arm,
        )?;
    }
    Ok(Coset {
        subgroup: residual,
        representative: x,
    })
}

/// Stage one: the candidate's rotation.
///
/// The rotation constraint of a coset `G·R` is that `Q·Q_R⁻¹` lie in
/// `G`'s rotations, which are all of SO(3), the rotations about one
/// axis, or the identity alone. Two one-axis constraints meet through
/// the two-axis condition below; everything else is forced.
fn candidate_rotation<T: SolveScalar>(
    held: Coset<T>,
    added: Coset<T>,
    band: Band,
    arm: Arm,
) -> Result<Mat3<T>, FoldStop> {
    let q1 = held.representative.linear;
    let q2 = added.representative.linear;
    Ok(
        match (held.subgroup.rotations(), added.subgroup.rotations()) {
            (Rotations::Free, _) => q2,
            (_, Rotations::Free) => q1,
            // The tighter side forces the rotation; the looser side's
            // violation, if any, surfaces at the membership check.
            (Rotations::Fixed, _) => q1,
            (_, Rotations::Fixed) => q2,
            (Rotations::About(a1), Rotations::About(a2)) => {
                if parallel(a1, a2, band, arm)?.is_none() {
                    // PARALLEL rotation axes: the held side admits EVERY
                    // rotation about its own axis, so the candidate's
                    // clocking about that axis is free — and free means it
                    // must be SOLVED, not inherited. Pinning it to the
                    // held representative's clocking yields a pose that is
                    // merely IN the held coset rather than a witness of
                    // the intersection, and the (sound) membership check
                    // then refuses an assemblable pair: review MAJOR-1's
                    // two-pin pattern, inter-axis invariants matching but
                    // the patterns clocked apart.
                    clocking_about(held, added, a1) * q1
                } else {
                    // Two distinct rotation axes. Write the unknown as
                    // `rot(a1, α)·Q1`; requiring it to fix a2 after the
                    // added side's representative is undone gives one
                    // equation in α, solvable iff the two vectors share
                    // their a1-component — the reachability predicate.
                    let (a1, a2) = (a1.get(), a2.get());
                    let m = q1 * q2.transpose();
                    let v = m * a2;
                    let reach = Measured::Residual {
                        value: v.dot(a1) - a2.dot(a1),
                        arm: arm.get(),
                    };
                    let refuted = Refuted::TwoAxisReachable;
                    if decide(refuted.name(), reach.margin(), band)? != Sign::Zero {
                        return Err(FoldStop::Clash {
                            predicate: refuted.name(),
                            clash: reach.clash(),
                        });
                    }
                    let vp = v.reject_from(a1);
                    let tp = a2.reject_from(a1);
                    let alpha = T::solve_atan2(vp.cross(tp).dot(a1), vp.dot(tp));
                    Mat3::rotation_about(a1, alpha) * q1
                }
            }
        },
    )
}

/// **The free clocking about a shared axis, solved** (review MAJOR-1).
///
/// When both sides constrain rotation to one shared axis `u`, the
/// candidate may be turned by any angle α about the HELD side's own
/// axis and stay in the held coset — `rot(line(p₁,u), α)` is an
/// element of every subgroup this arm sees. So α is a free parameter,
/// and the added side's own axis-POSITION constraint is what fixes it.
///
/// The equation, in one line: the added side asks that its axis point
/// `p₂` come back to itself (up to sliding along `u`), so the candidate
/// must carry `w = R₁R₂⁻¹(p₂)` onto `p₂`. Turning about `(p₁,u)` moves
/// `w` on a circle, so the condition is a planar rotation taking
/// `(w−p₁)⊥` onto `(p₂−p₁)⊥` — one angle, closed form.
///
/// **Solvable exactly when the two radii agree**, which is the table's
/// own "inter-axis displacement invariants match A vs B" predicate.
/// This function does NOT decide that: `atan2` gives the direction
/// change regardless, the radius mismatch survives into the candidate,
/// and the membership check refuses it with the mismatch as the
/// measured clash — the table's "an unsatisfiable candidate is empty,
/// named", reached through the one gate rather than a second one.
///
/// Two arms need no angle: a side whose subgroup carries NO axis point
/// states no position constraint (planar), and a held side with
/// perpendicular translation freedom (planar again) can satisfy the
/// added position constraint by TRANSLATING in stage two. Both return
/// the identity, and `atan2(0, 0) = 0` makes the degenerate radii fall
/// out the same way with no branch.
fn clocking_about<T: SolveScalar>(held: Coset<T>, added: Coset<T>, u: UnitVec3<T>) -> Mat3<T> {
    let (Some(p1), Some(p2)) = (axis_point(held.subgroup), axis_point(added.subgroup)) else {
        return Mat3::identity();
    };
    let u = u.get();
    let w = (held.representative * added.representative.inverse()).transform_point(p2);
    let from = (w - p1).reject_from(u);
    let to = (p2 - p1).reject_from(u);
    Mat3::rotation_about(u, T::solve_atan2(from.cross(to).dot(u), from.dot(to)))
}

/// The point a subgroup's axis is pinned through, `None` for the
/// point-free subgroups (whose membership states no position
/// constraint at all).
fn axis_point<T: Real>(g: Subgroup<T>) -> Option<Point3<T>> {
    match g {
        Subgroup::Cylindrical { point, .. } | Subgroup::Revolute { point, .. } => Some(point),
        _ => None,
    }
}

/// Stage two: the candidate's translation, with the rotation fixed.
///
/// Each side contributes an affine condition `P·(t − q) = 0`
/// ([`Subgroup::translation_anchor`]). The HELD side is met exactly —
/// `t = q₁ + δ` with `δ` in the held subgroup's translation freedom —
/// and the added side in the part of that freedom the residual does
/// not keep, so a failure's margin is the added mate's own clash. That
/// part is at most a plane, and each shape is solved in closed form in
/// its own frame, by least squares where the added side over-asks.
///
/// **Its only divisor is the number the table's verdict decided away
/// from zero** ([`Separated`]): the levered sine of a non-parallel
/// verdict, at least `K·ε`, or the cosine of a non-perpendicular one,
/// out of the zero band — exactly where the held freedom and the
/// added constraint nearly align, so the conditioning is that number
/// and not its square. Where a verdict called two directions parallel
/// or perpendicular instead, the shape it named is solved as named,
/// with no division: a line along the normal meets the plane at its
/// anchor's foot, and a line lying in the plane holds its anchor's
/// foot. So a pair the table separated has a candidate as finite as
/// the meeting point it names, which [`intersect`] measures.
fn candidate_translation<T: SolveScalar>(
    held: Coset<T>,
    added: Coset<T>,
    residual: Subgroup<T>,
    separated: Separated<T>,
    rotation: Mat3<T>,
    arm: Arm,
) -> Vec3<T> {
    use Subgroup::{Cylindrical, Planar, Prismatic, Revolute, Trivial};
    let a1 = rotation * held.representative.linear.transpose();
    let a2 = rotation * added.representative.linear.transpose();
    let q1 = held
        .subgroup
        .translation_anchor(a1, held.representative.translation);
    let q2 = added
        .subgroup
        .translation_anchor(a2, added.representative.translation);
    let delta = q2 - q1;
    if held.subgroup.translation_dimension() == residual.translation_dimension() {
        // The residual keeps every translation the held side frees.
        return q1;
    }
    let step = match (held.subgroup, added.subgroup, separated) {
        // Two planes meeting in the residual's line `d`: cross it
        // within the held plane, along `e = d × n`, to the added
        // plane. `m·e = d·(n × m)` is the sine the verdict levered.
        (Planar { normal: n }, Planar { normal: m }, Separated::Sine(sine)) => {
            let Prismatic { direction: d } = residual else {
                unreachable!("two planes the table separates meet in its prismatic line")
            };
            let e = d.get().cross(n.get());
            e * (m.get().dot(delta) * (T::from_f64(arm.get()) / sine))
        }
        // A line crossing the plane at the cosine the verdict decided:
        // the point where it does, either side held.
        (
            Planar { normal: n },
            Cylindrical { direction: u, .. } | Prismatic { direction: u },
            Separated::Cosine(cosine),
        ) => delta - u.get() * (n.get().dot(delta) / cosine),
        (
            Cylindrical { direction: u, .. } | Prismatic { direction: u },
            Planar { normal: n },
            Separated::Cosine(cosine),
        ) => u.get() * (n.get().dot(delta) / cosine),
        // A line the verdict called along the normal (a revolute
        // residual) or in the plane (a slide along it): either way the
        // anchor's foot on the held plane, which is where the first
        // meets the plane and a point of the second. So is the point of
        // the plane nearest an added side that pins the translation.
        (Planar { normal: n }, _, Separated::Not) => delta - n.get() * n.get().dot(delta),
        // A held line along the normal of an added plane (a revolute
        // residual): the point where it crosses, the line being the
        // normal up to its sense.
        (
            Cylindrical { direction: u, .. } | Prismatic { direction: u },
            Planar { normal: n },
            Separated::Not,
        ) => u.get() * (u.get().dot(n.get()) * n.get().dot(delta)),
        // Two lines the table called non-parallel: the point of the
        // held line nearest the added one. Along `u` the added side's
        // projector has length `|u × v|`, the sine the verdict
        // levered into `sine`, so the step is `((v × w)·Δ) · arm / sine²`
        // with `w = (u × v)·arm`, whose length is `sine`.
        (
            Cylindrical { direction: u, .. } | Prismatic { direction: u },
            Cylindrical { direction: v, .. } | Prismatic { direction: v },
            Separated::Sine(sine),
        ) => {
            let arm = T::from_f64(arm.get());
            let w = u.get().cross(v.get()) * arm;
            u.get() * (v.get().cross(w).dot(delta) * (arm / sine / sine))
        }
        // An added side that pins the translation outright: the point
        // of the held line nearest it.
        (
            Cylindrical { direction: u, .. } | Prismatic { direction: u },
            Revolute { .. } | Trivial,
            Separated::Not,
        ) => u.get() * u.get().dot(delta),
        (held_subgroup, added_subgroup, separated) => unreachable!(
            "the table hands no such verdict to a held side with freedom its residual does not \
             keep: held {}, added {}, {separated:?}; SE(3) and the empty coset return before \
             this stage, and a revolute or trivial held side keeps no translation",
            held_subgroup.name(),
            added_subgroup.name()
        ),
    };
    q1 + step
}

/// `a − b`, columnwise, in one written order (D9).
fn sub<T: Real>(a: Mat3<T>, b: Mat3<T>) -> Mat3<T> {
    Mat3::from_cols(a.c0 - b.c0, a.c1 - b.c1, a.c2 - b.c2)
}
