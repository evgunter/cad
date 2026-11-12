//! **`carrier_eq`** — the oriented-carrier-equality ladder,
//! kind-generalized (CONTACT-DESIGN C4's `Rest` table).
//!
//! [`super::oriented_plane_eq`] answers "same plane?" through two
//! rungs: definite geometric difference, else a coincidence decided by
//! its margin — Zero glues, in band refuses. `Rest` generalizes S1's
//! planar vocabulary to every carrier kind, so the ladder generalizes
//! with it: the same three verdicts ([`CarrierRelation`]) and the same
//! refusal type ([`CarrierEqError`]) for every kind. The plane, sphere,
//! cylinder and cone arms walk their data on the same rungs; the torus
//! arm reads the pair's own coincidence over the whole ring (below):
//!
//! | kind | defining data | margins, each at its named lever arm |
//! |------|---------------|--------------------------------------|
//! | plane | origin, outward normal | `bool_plane_parallel` (·arm), `bool_plane_offset` |
//! | sphere | centre, radius | `carrier_sphere_center`, `carrier_sphere_radius` |
//! | cylinder | axis line, radius | `carrier_cyl_axis_parallel` (·arm), `carrier_cyl_axis_offset`, `carrier_cyl_radius` |
//! | torus | centre, axis line, both radii | the pair's own coincidence ([`geom_brep::tori_coincide`]): `carrier_torus_axis_parallel` (·the larger major radius), `carrier_torus_center`, `carrier_torus_major_radius`, `carrier_torus_minor_radius`, `tt_coincident` |
//! | cone | apex, axis line, half-angle | `carrier_cone_apex`, `carrier_cone_axis_parallel` (·arm), `carrier_cone_half_angle` (·arm) |
//!
//! The lever arms: an ANGULAR margin (two normalized directions
//! crossed) is dimensionless, so it is metered at the extent over which
//! the verdict is consumed ([`ConsumedExtent`]), turning it into the
//! displacement the misalignment induces there. A LENGTH margin (a
//! centre separation, a radius difference, a point-to-axis distance) is
//! already in metres and is metered at unit arm; multiplying it by the
//! extent would price the same error twice. The torus is the exception:
//! its coincidence reads the whole ring, its tilt levered at the larger
//! major radius, so a datum past the band moves the ring by that much
//! somewhere, not necessarily on the faces. Its `Apart` is the ring's;
//! the face-pair door asks the faces for a witness of it
//! ([`super::carrier_pair::carrier_pair_decided`]).
//!
//! **At the face-pair door a coincidence is read as one
//! displacement**, not datum by datum: each datum just inside the band
//! would let the carriers stand nearly twice the band apart where their
//! errors add up. That door ([`pair_door_reading`]) glues a pair only
//! where the displacement's upper bound over the pair's consumed extent
//! decides Zero too. A corner site reads each datum on its own over its
//! own arm, since there every verdict but `Distinct` refuses an unglued
//! pair, and a glued pair is read through its record, never the ladder.
//!
//! **Orientation across kinds.** The plane arm's Same± verdict is a
//! statement about MATERIAL SIDES, and so is every other arm's: a
//! sphere's, cylinder's, torus's or cone's description carries the
//! `outward` bit saying whether the face's outward normal points away
//! from the centre, the axis or the tube's spine circle (S10's sense
//! fold, one dimension curved).
//! `Rest` contact is precisely [`CarrierRelation::SameOpposite`] — a
//! peg's convex wall against a bore's concave wall is the curved
//! spelling of two boxes' opposed faces. Aligned coincidence is
//! CONTAINMENT, not contact (the C1 lemma); the carrier ladder
//! reports it honestly as `SameOriented` and the CONTACT doors
//! ([`super::contact_verify::rest_pair_verdict`]) are what refuse it.
//!
//! **A margin decided Zero glues** (D10, Booleans). Two independently
//! authored spheres with equal radii and centres are one carrier
//! exactly as two equal planes are, and the decided margin rides out
//! with the verdict ([`CarrierVerdict`]) for the caller to record
//! ([`crate::coincidence`]). Whether they are one CONSTRUCTION is the
//! document's coincidence door's question, never the kernel's. A datum
//! in band is a sliver and refuses ([`CarrierEqError::Undecided`]).

use geom_brep::ladder::{IdentityLadder, Kept};
use geom_core::{Band, Decide, Decided, Indeterminate, Margin, MarginDiag, Point3, Sign, Vec3};

use crate::validate::decide_reported;

use super::plane_eq::{PlaneDesc, PlaneRung, plane_ladder};

/// The relation between two oriented carriers: the three outcomes
/// every kind's ladder produces.
///
/// This is ONE type across kinds — `plane_eq` re-exports it as
/// `PlaneRelation`, the spelling its planar callers use. A parallel
/// per-kind verdict enum would let a caller handle "same carrier" for
/// planes and forget it for cylinders.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CarrierRelation {
    /// Same carrier, same material side (the ⁺ case of Eq. 15.3 and
    /// its curved analogues) — flush walls, and the merge stage's
    /// pair.
    SameOriented,
    /// Same carrier, opposite material sides (the ⁻ case) — this, and
    /// only this, is `Rest` contact.
    SameOpposite,
    /// Definitely different carriers.
    Distinct,
}

/// **What the ladder decided of two carriers**, with the reading it
/// decided on: one carrier on the margin that decided it, apart on the
/// datum decided past the band, or two kinds, which no datum decides.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CarrierVerdict {
    /// One carrier: the decided orientation
    /// ([`CarrierRelation::SameOriented`] or
    /// [`CarrierRelation::SameOpposite`], never `Distinct`), and the margin
    /// that decided it: every datum's Zero, with the displacement's bound
    /// where the face-pair door read it.
    One {
        /// The decided orientation.
        relation: CarrierRelation,
        /// The margin that decided the coincidence.
        margin: MarginDiag,
    },
    /// Two carriers of one kind: the reading of the datum decided past
    /// the band.
    Apart(Decided),
    /// Two carriers of two kinds: a plane is not a cylinder at any
    /// radius, so no datum is read.
    TwoKinds,
}

impl CarrierVerdict {
    /// The verdict as a relation: `Distinct` for two carriers.
    #[must_use]
    pub const fn relation(self) -> CarrierRelation {
        match self {
            Self::One { relation, .. } => relation,
            Self::Apart(_) | Self::TwoKinds => CarrierRelation::Distinct,
        }
    }

    /// The orientation and margin of one carrier; `None` for two.
    #[must_use]
    pub const fn one(self) -> Option<(CarrierRelation, MarginDiag)> {
        match self {
            Self::One { relation, margin } => Some((relation, margin)),
            Self::Apart(_) | Self::TwoKinds => None,
        }
    }
}

/// Typed refusal of [`carrier_eq`]; re-exported by `plane_eq` as
/// `PlaneEqError`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CarrierEqError {
    /// A rung of the plane ladder could not decide: the only rungs that
    /// escalate here (the curved data rungs refuse `Undecided` instead).
    Escalated {
        /// The rung that could not decide.
        rung: PlaneRung,
        /// Its diagnostics: the margin in band, or the one the rung
        /// decided at zero where zero does not pass.
        diag: Indeterminate,
    },
    /// A pair whose coincidence did not decide: a datum, or the
    /// displacement's bound, in band (a sliver, F6) or poisoned, or the
    /// data decided `Zero` beside a bound decided past the band (a
    /// straddle). Carries
    /// the orientation the data rungs had ALREADY decided before the
    /// refusal (alignment is settled before the coincidence margins are
    /// taken), so a refusal can name the class a glue would take without
    /// re-running any decide on the error path.
    Undecided {
        /// What the coincidence measure read.
        coincidence: CoincidenceMeasure,
        /// The decided orientation: [`CarrierRelation::SameOriented`]
        /// or [`CarrierRelation::SameOpposite`], never `Distinct`.
        relation: CarrierRelation,
    },
    /// A torus outside its ring convention, whose coincidence the ring's
    /// bound does not hold for: which torus of the pair, and the
    /// convention's refusal, which ends in the half's lever
    /// ([`geom_brep::TorusConvention::sized`]). The body's own validity
    /// refuses such a torus (`ValidationError::DegenerateTorus`), but the
    /// ladder also reads bodies no validation has passed.
    OffRing {
        /// The torus refused, in the order the pair was read.
        torus: geom_brep::PairSide,
        /// Its refusal.
        refusal: geom_brep::TorusRefusal,
    },
}

/// What an undecided coincidence's measure read
/// ([`CarrierEqError::Undecided`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CoincidenceMeasure {
    /// A datum, or the displacement over the pair's extent, did not
    /// decide: in band, or read `Zero` from below and past the band from
    /// above, a straddle ([`geom_brep::ladder::data_beneath_sum`]).
    Undecided(Indeterminate),
    /// A datum is not finite: poisoned input, which no move of the
    /// faces reads.
    Unreadable(Indeterminate),
}

impl CoincidenceMeasure {
    /// The margin the measure read, with its band, as the payload a
    /// refusal quotes.
    #[must_use]
    pub const fn reported(self) -> Indeterminate {
        match self {
            Self::Undecided(diag) | Self::Unreadable(diag) => diag,
        }
    }

    /// One coincidence datum decided ([`ladder::decided`]): its sign and
    /// the margin the band decided, or the measure it leaves where it
    /// decides nothing. A datum that is not finite (NaN or ±∞) is
    /// unreadable whatever sign it reads: an infinite offset is no more a
    /// locus than a NaN one.
    ///
    /// [`ladder::decided`]: geom_brep::ladder::decided
    pub(crate) fn decide<T: Decide>(
        name: &'static str,
        margin: Margin<T>,
        band: Band,
    ) -> Result<Decided, Self> {
        geom_brep::ladder::decided(name, margin, band).map_err(Self::of_kept)
    }

    /// A measure that did not decide zero, as what it is ([`Kept::of`]).
    pub(crate) fn not_zero(diag: Indeterminate) -> Self {
        Self::of_kept(Kept::of(diag))
    }

    /// A kept reading's measure: a limit is unreadable, an in-band
    /// reading undecided.
    fn of_kept(kept: Kept<Indeterminate>) -> Self {
        match kept {
            Kept::InBand(diag) => Self::Undecided(diag),
            Kept::Blocked(diag) => Self::Unreadable(diag),
        }
    }
}

/// One carrier's conventional oriented description.
///
/// Every variant carries the MATERIAL side, not the chart's: the
/// plane arm folds the face sense into `normal` (S10), and the curved
/// arms carry it as `outward`. A description built from a raw chart
/// normal would make the Same± verdict a statement about nothing.
#[derive(Clone, Copy, Debug)]
pub enum CarrierDesc<T: geom_core::Real> {
    /// A plane, by a point on it and its unit outward normal.
    Plane {
        /// A point on the plane.
        origin: Point3<T>,
        /// The unit outward normal (of the face, not of the chart).
        normal: Vec3<T>,
    },
    /// A sphere, by centre and radius.
    Sphere {
        /// The centre.
        center: Point3<T>,
        /// The radius (positive).
        radius: T,
        /// Whether the face's outward normal points AWAY from the
        /// centre (a convex wall) rather than toward it (a cavity).
        outward: bool,
    },
    /// A cylinder, by a point on its axis, the unit axis direction,
    /// and the radius. The axis is a LINE: its direction sign carries
    /// no material information, so it never enters the Same± verdict.
    Cylinder {
        /// A point on the axis.
        origin: Point3<T>,
        /// The unit axis direction.
        axis: Vec3<T>,
        /// The radius (positive).
        radius: T,
        /// Whether the face's outward normal points AWAY from the
        /// axis (a shaft) rather than toward it (a bore).
        outward: bool,
    },
    /// A torus, by centre, the unit axis of revolution, and the two
    /// radii. Unlike a cylinder's, the centre is a POINT of the
    /// carrier's own data (it is where the tube's midplane meets the
    /// axis), so the two carriers' centres are compared directly and
    /// no perpendicular projection is taken. The axis is a LINE: a
    /// torus is carried onto itself by reversing it, so its direction
    /// sign holds no material information.
    Torus {
        /// The centre.
        center: Point3<T>,
        /// The unit axis of revolution.
        axis: Vec3<T>,
        /// The major radius: centre to tube centre (positive).
        major_radius: T,
        /// The minor radius: the tube radius (positive).
        minor_radius: T,
        /// Whether the face's outward normal points OUT of the tube (a
        /// pipe wall) rather than into it (a toroidal cavity).
        outward: bool,
    },
    /// A cone, by apex, the unit axis direction, and the half-angle. The
    /// carrier is the DOUBLE cone: a face's chart may place it on either
    /// nappe, so the axis is a LINE and its direction sign carries no
    /// material information. Angular data are metered from the apex,
    /// where the two carriers' displacement is the apexes' alone.
    Cone {
        /// The apex.
        apex: Point3<T>,
        /// The unit axis direction.
        axis: Vec3<T>,
        /// The half-angle between the axis and the generators, radians.
        half_angle: T,
        /// Whether the face's outward normal points AWAY from the axis
        /// (a spike) rather than toward it (a countersink). The chart
        /// normal points away from the axis on both nappes.
        outward: bool,
    },
}

impl<T: geom_core::Real> CarrierDesc<T> {
    /// The kind's name, for messages and the kind-mismatch rung: the
    /// surface kind's own spelling ([`geom::SurfaceKind::name`]).
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Plane { .. } => geom::SurfaceKind::Plane,
            Self::Sphere { .. } => geom::SurfaceKind::Sphere,
            Self::Cylinder { .. } => geom::SurfaceKind::Cylinder,
            Self::Torus { .. } => geom::SurfaceKind::Torus,
            Self::Cone { .. } => geom::SurfaceKind::Cone,
        }
        .name()
    }
}

/// **The region a carrier verdict is consumed on**: a ball enclosing
/// both faces ([`geom_brep::ExtentBall`]). The ball bounds the
/// displacement between the carriers from ABOVE at every consumed
/// point, and levers the angular data.
#[derive(Clone, Copy, Debug)]
pub struct ConsumedExtent<T: geom_core::Real> {
    /// A ball enclosing both faces.
    pub reach: geom_brep::ExtentBall<T>,
}

impl<T: geom_core::Real> ConsumedExtent<T> {
    /// The extent a ball enclosing it names.
    #[must_use]
    pub fn new(reach: geom_brep::ExtentBall<T>) -> Self {
        Self { reach }
    }

    /// The ball of radius `arm` about the origin: the extent a bare arm
    /// names, for the rows that meter at one.
    #[cfg(test)]
    pub(crate) fn arm(arm: T) -> Self {
        Self::new(geom_brep::ExtentBall::of_sphere(Point3::origin(), arm))
    }
}

/// The ladder's inventory as its refusals name it: a literal, so a
/// refusal can `concat!` it, which [`INVENTORY`]'s row checks.
macro_rules! inventory_named {
    () => {
        "plane, sphere, cylinder, torus, cone"
    };
}

/// **The surface kinds the ladder carries a description for**
/// ([`CarrierDesc`]): the `Rest` table's inventory. A face on any other
/// kind (a spline, a fitted surface) has no description to compare
/// (`carrier_pair::PairUnread::OutsideInventory`), and is not
/// declarable.
pub(crate) const INVENTORY: [geom::SurfaceKind; 5] = [
    geom::SurfaceKind::Plane,
    geom::SurfaceKind::Sphere,
    geom::SurfaceKind::Cylinder,
    geom::SurfaceKind::Torus,
    geom::SurfaceKind::Cone,
];

/// Is `kind` in the ladder's [`INVENTORY`]?
pub(crate) fn in_inventory(kind: geom::SurfaceKind) -> bool {
    INVENTORY.contains(&kind)
}

/// The refusal of a glued face outside [`INVENTORY`], at the glue door
/// and in the contact lane.
pub(crate) const OUTSIDE_REST_INVENTORY: &str = concat!(
    "a glued face's surface kind is outside the Rest ladder's inventory (",
    inventory_named!(),
    ")"
);

/// **The curved kinds the ladder decides exactly**, on its
/// sphere and cylinder data rungs: the curved surfaces whose aligned
/// pairs the census reads as one carrier. A subset of [`INVENTORY`]:
/// the census's same-side arm reads a pair's shared interior through
/// the faces' tangent cones at its witnesses, which it reads for these
/// two kinds; a torus or cone pair stays the census's
/// (`work/atrest/curved-copies-filling-one-volume-read-as-undecidable.md`).
pub(crate) fn exact_curved<T: geom_core::Real>(s: &geom::Surface<T>) -> bool {
    matches!(
        s,
        geom::Surface::Cylinder { .. } | geom::Surface::Sphere { .. }
    )
}

/// **`carrier_eq`** — module docs for the ladder. `extent` is the
/// region the verdict is consumed on, which levers the ANGULAR margins;
/// `band` the run's linear band.
///
/// # Errors
///
/// [`CarrierEqError`] — sliver escalation, or an undecided coincidence.
pub fn carrier_eq<T: Decide>(
    c1: &CarrierDesc<T>,
    c2: &CarrierDesc<T>,
    extent: &ConsumedExtent<T>,
    band: Band,
) -> Result<CarrierRelation, CarrierEqError> {
    carrier_eq_reading(c1, c2, extent, band).map(CarrierVerdict::relation)
}

/// [`carrier_eq`] with the reading that decided it, which a
/// coincidence's row records ([`crate::coincidence`]): each datum read
/// on its own, the angular ones levered at the extent
/// ([`at_consumed_extent`]).
pub(crate) fn carrier_eq_reading<T: Decide>(
    c1: &CarrierDesc<T>,
    c2: &CarrierDesc<T>,
    extent: &ConsumedExtent<T>,
    band: Band,
) -> Result<CarrierVerdict, CarrierEqError> {
    ladder_reading(c1, c2, extent, band).map(PairDoorReading::verdict)
}

/// **What the face-pair door read of two carriers**
/// ([`pair_door_reading`]): the ladder's verdict, or two tori the ring
/// reads apart, which the faces need not be.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum PairDoorReading {
    /// The ladder's verdict.
    Verdict(CarrierVerdict),
    /// Two tori whose ring a datum reads apart
    /// ([`geom_brep::TorusCoincidence::Apart`]): the door asks the faces
    /// for a witness of it ([`super::carrier_pair::carrier_pair_decided`]).
    RingApart {
        /// The orientation the two tori's senses make.
        relation: CarrierRelation,
        /// The datum's reading.
        by: Decided,
        /// The ring's upper bound, decided past the band with the datum.
        upper: Decided,
    },
}

impl PairDoorReading {
    /// The reading as the ladder's verdict: the ring apart is two
    /// carriers, on its datum's reading.
    #[must_use]
    pub(super) const fn verdict(self) -> CarrierVerdict {
        match self {
            Self::Verdict(verdict) => verdict,
            Self::RingApart { by, .. } => CarrierVerdict::Apart(by),
        }
    }
}

/// The ladder over each kind's data ([`carrier_eq_reading`]), the ring's
/// apart kept apart for the face-pair door ([`PairDoorReading`]).
fn ladder_reading<T: Decide>(
    c1: &CarrierDesc<T>,
    c2: &CarrierDesc<T>,
    extent: &ConsumedExtent<T>,
    band: Band,
) -> Result<PairDoorReading, CarrierEqError> {
    let (c1, c2, arm) = at_consumed_extent(c1, c2, extent.reach);
    let verdict = match (&c1, &c2) {
        (
            CarrierDesc::Plane {
                origin: o1,
                normal: n1,
            },
            CarrierDesc::Plane {
                origin: o2,
                normal: n2,
            },
        ) => plane_ladder(
            &PlaneDesc {
                origin: *o1,
                normal: *n1,
            },
            &PlaneDesc {
                origin: *o2,
                normal: *n2,
            },
            extent,
            band,
        ),
        (
            CarrierDesc::Sphere {
                center: p1,
                radius: r1,
                outward: w1,
            },
            CarrierDesc::Sphere {
                center: p2,
                radius: r2,
                outward: w2,
            },
        ) => data_rungs(&sphere_data(*p1, *r1, *p2, *r2), *w1 == *w2, band),
        (
            CarrierDesc::Cylinder {
                origin: p1,
                axis: a1,
                radius: r1,
                outward: w1,
            },
            CarrierDesc::Cylinder {
                origin: p2,
                axis: a2,
                radius: r2,
                outward: w2,
            },
        ) => data_rungs(
            &cylinder_data((*p1, *a1, *r1), (*p2, *a2, *r2), arm),
            *w1 == *w2,
            band,
        ),
        (
            CarrierDesc::Torus {
                center: p1,
                axis: a1,
                major_radius: r1,
                minor_radius: t1,
                outward: w1,
            },
            CarrierDesc::Torus {
                center: p2,
                axis: a2,
                major_radius: r2,
                minor_radius: t2,
                outward: w2,
            },
        ) => {
            let same = if w1 == w2 {
                CarrierRelation::SameOriented
            } else {
                CarrierRelation::SameOpposite
            };
            let undecided = |coincidence| CarrierEqError::Undecided {
                coincidence,
                relation: same,
            };
            return match geom_brep::tori_coincide(
                torus((*p1, *a1, *r1, *t1)),
                torus((*p2, *a2, *r2, *t2)),
                band,
            ) {
                geom_brep::TorusCoincidence::One(margin) => {
                    Ok(PairDoorReading::Verdict(CarrierVerdict::One {
                        relation: same,
                        margin,
                    }))
                }
                geom_brep::TorusCoincidence::Apart { by, upper, .. } => {
                    Ok(PairDoorReading::RingApart {
                        relation: same,
                        by,
                        upper,
                    })
                }
                geom_brep::TorusCoincidence::Undecided(diag) => {
                    Err(undecided(CoincidenceMeasure::not_zero(diag)))
                }
                geom_brep::TorusCoincidence::OffRing { torus, refusal } => {
                    Err(CarrierEqError::OffRing { torus, refusal })
                }
            };
        }
        (
            CarrierDesc::Cone {
                apex: p1,
                axis: a1,
                half_angle: t1,
                outward: w1,
            },
            CarrierDesc::Cone {
                apex: p2,
                axis: a2,
                half_angle: t2,
                outward: w2,
            },
        ) => data_rungs(
            &cone_data((*p1, *a1, *t1), (*p2, *a2, *t2), arm),
            *w1 == *w2,
            band,
        ),
        // Different kinds: definitely different carriers. A plane is
        // not a cylinder at any radius, so this needs no numerics.
        _ => Ok(CarrierVerdict::TwoKinds),
    };
    verdict.map(PairDoorReading::Verdict)
}

/// [`carrier_eq_reading`] at the FACE-PAIR door: a coincidence glues
/// the pair only where the displacement's upper bound over the pair's
/// extent decides Zero too ([`coincident_over_extent_as`]). A torus pair's
/// coincidence already stands on that bound
/// ([`geom_brep::tori_coincide`]), and a torus pair the ring reads apart
/// comes back as such, for the door to ask the faces for a witness
/// ([`PairDoorReading::RingApart`]).
///
/// # Errors
///
/// As [`carrier_eq`].
pub(super) fn pair_door_reading<T: Decide>(
    c1: &CarrierDesc<T>,
    c2: &CarrierDesc<T>,
    extent: &ConsumedExtent<T>,
    band: Band,
) -> Result<PairDoorReading, CarrierEqError> {
    match ladder_reading(c1, c2, extent, band)? {
        PairDoorReading::Verdict(CarrierVerdict::One { relation, margin }) => {
            let margin = coincident_over_extent_as(c1, c2, extent, (relation, margin), band)
                .map_err(|diag| CarrierEqError::Undecided {
                    coincidence: CoincidenceMeasure::not_zero(diag),
                    relation,
                })?;
            Ok(PairDoorReading::Verdict(CarrierVerdict::One {
                relation,
                margin,
            }))
        }
        reading => Ok(reading),
    }
}

/// A margin the ladder reads, by its predicate's name.
type Datum<T> = (&'static str, Margin<T>);

/// The names the sphere's, cylinder's and cone's data are decided under.
const CARRIER_SPHERE_CENTER: &str = "carrier_sphere_center";
const CARRIER_SPHERE_RADIUS: &str = "carrier_sphere_radius";
pub(super) const CARRIER_CYL_AXIS_PARALLEL: &str = "carrier_cyl_axis_parallel";
const CARRIER_CYL_AXIS_OFFSET: &str = "carrier_cyl_axis_offset";
const CARRIER_CYL_RADIUS: &str = "carrier_cyl_radius";
const CARRIER_CONE_APEX: &str = "carrier_cone_apex";
const CARRIER_CONE_AXIS_PARALLEL: &str = "carrier_cone_axis_parallel";
const CARRIER_CONE_HALF_ANGLE: &str = "carrier_cone_half_angle";

/// The sphere's data: centre separation, radius difference.
fn sphere_data<T: Decide>(p1: Point3<T>, r1: T, p2: Point3<T>, r2: T) -> Vec<Datum<T>> {
    vec![
        (CARRIER_SPHERE_CENTER, Margin::norm3(p1 - p2)),
        (CARRIER_SPHERE_RADIUS, Margin::of(r1 - r2)),
    ]
}

/// The cylinder's data. The axis LINE, not the axis ray: parallelism
/// is metered on the cross product (sign-free by construction) at
/// `arm`, and the offset is the perpendicular distance from the second
/// axis point to the first axis — the two data a cylinder's axis
/// actually has.
fn cylinder_data<T: Decide>(
    (p1, a1, r1): (Point3<T>, Vec3<T>, T),
    (p2, a2, r2): (Point3<T>, Vec3<T>, T),
    arm: geom_core::Lever<T>,
) -> Vec<Datum<T>> {
    vec![
        (
            CARRIER_CYL_AXIS_PARALLEL,
            Margin::levered(a1.cross(a2).norm(), arm),
        ),
        (
            CARRIER_CYL_AXIS_OFFSET,
            Margin::norm3(perpendicular(p2 - p1, a1)),
        ),
        (CARRIER_CYL_RADIUS, Margin::of(r1 - r2)),
    ]
}

/// The cone's data: the apexes' separation, and the axis lines'
/// parallelism and the half-angles' difference, each levered at `arm`,
/// the reach from the apex.
fn cone_data<T: Decide>(
    (p1, a1, t1): (Point3<T>, Vec3<T>, T),
    (p2, a2, t2): (Point3<T>, Vec3<T>, T),
    arm: geom_core::Lever<T>,
) -> Vec<Datum<T>> {
    vec![
        (CARRIER_CONE_APEX, Margin::norm3(p1 - p2)),
        (
            CARRIER_CONE_AXIS_PARALLEL,
            Margin::levered(a1.cross(a2).norm(), arm),
        ),
        (CARRIER_CONE_HALF_ANGLE, Margin::levered(t1 - t2, arm)),
    ]
}

/// A torus description's data as the pair's coincidence reads them.
fn torus<T: geom_core::Real>(
    (center, axis, major_radius, minor_radius): (Point3<T>, Vec3<T>, T, T),
) -> geom_brep::TorusData<T> {
    geom_brep::TorusData {
        center,
        axis,
        major_radius,
        minor_radius,
    }
}

/// `v`'s part perpendicular to the unit `axis`.
fn perpendicular<T: geom_core::Real>(v: Vec3<T>, axis: Vec3<T>) -> Vec3<T> {
    v - axis * v.dot(axis)
}

pub(crate) use geom_brep::line_tilt;

/// **The arm the ladder levers a pair's angular data at** over
/// `reach` ([`at_consumed_extent`]'s third part): the reach from the
/// pivot each kind's position datum is read at.
pub(super) fn consumed_arm<T: geom_core::Real>(
    c1: &CarrierDesc<T>,
    c2: &CarrierDesc<T>,
    reach: geom_brep::ExtentBall<T>,
) -> geom_core::Lever<T> {
    at_consumed_extent(c1, c2, reach).2
}

/// **The pair as the ladder reads it when its verdict is
/// consumed over `reach`**: each description anchored where its
/// position datum is read nearest the extent's centre, and the lever
/// arm for its angular data, the extent's farthest reach from that
/// pivot ([`geom_brep::ExtentBall`]'s module docs). Each datum is
/// decided on its own: a definite one says the carriers differ, and the
/// face-pair door then reads the data as one sum
/// ([`coincident_over_extent_as`]).
///
/// - plane — the plane ladder reads the extent itself (offsets at its
///   centre, the arm its radius) and the pair passes through;
/// - cylinder — the axis offset is read at the second description's
///   axis point, which moves along its axis to the foot of the
///   extent's centre;
/// - torus — the pair's coincidence reads no extent
///   ([`geom_brep::tori_coincide`]), and the pair passes through;
/// - cone — the angular data are read from the first apex, widened by
///   the apexes' separation so the arm holds from either.
///
/// Sphere pairs and mixed kinds carry no angular datum; they pass
/// through with the arm read from the extent's centre.
fn at_consumed_extent<T: geom_core::Real>(
    c1: &CarrierDesc<T>,
    c2: &CarrierDesc<T>,
    reach: geom_brep::ExtentBall<T>,
) -> (CarrierDesc<T>, CarrierDesc<T>, geom_core::Lever<T>) {
    let centre = reach.center();
    let (c1, c2, pivot) = match (*c1, *c2) {
        (
            c1,
            CarrierDesc::Cylinder {
                origin,
                axis,
                radius,
                outward,
            },
        ) if matches!(c1, CarrierDesc::Cylinder { .. }) => {
            let foot = reach.foot_on(geom::Line { origin, dir: axis });
            (
                c1,
                CarrierDesc::Cylinder {
                    origin: foot,
                    axis,
                    radius,
                    outward,
                },
                foot,
            )
        }
        (CarrierDesc::Cone { apex: p1, .. }, CarrierDesc::Cone { apex: p2, .. }) => {
            return (*c1, *c2, reach.lever_from_within(p1, (p1 - p2).norm()));
        }
        (c1, c2) => (c1, c2, centre),
    };
    (c1, c2, reach.lever_from(pivot))
}

/// **A coincidence read as one displacement over the pair's extent.**
/// Every datum decided zero says "one carrier"; the face-pair door reads
/// the data's SUM, the displacement's upper bound at every point of the
/// extent's ball, so the coincidence stands only where that sum decides
/// zero too, whatever K. Where it decides zero, the coincidence's margin
/// is `data`, the data's `Zero` as the identity ladder keeps it (the
/// first datum's reading), standing for the sum's Zero too
/// ([`MarginDiag::jointly`]). Where it decides past the band beside the
/// data, the two straddle it under the kind's straddle row
/// ([`geom_brep::ladder::data_beneath_sum`]); the data bound the
/// carriers' separation from below, not the displacement over the
/// extent, so the straddle says only that neither answers for it. Where
/// it decides nothing, the refusal carries its reading.
///
/// Per kind (`c` the ball's centre, `R` its radius, `t` the tilt as
/// [`line_tilt`]'s chord, which bounds how far a unit offset turns):
///
/// - plane — the offset `n̂₁·(o₁ − c) − σ·n̂₂·(o₂ − c)` read at `c`,
///   plus `|n̂₁ − σ·n̂₂|·R`; σ is the decided orientation;
/// - sphere — the centre separation plus the radius difference;
/// - cylinder — the axis offset at `p` (the foot of `c` on the second
///   axis) plus `t` times the reach from `p` (widened by that offset,
///   so it holds from either axis), plus the radius difference;
/// - cone — the apexes' separation plus `t` and the half-angles'
///   difference, times the reach from the first apex widened by that
///   separation: a point at distance `s` from one apex has a partner on
///   the other cone at the same distance from its apex, on the nappe
///   `t` turns its own to, its generator turned by no more than those
///   two angles;
/// - torus — the pair's coincidence already stands on its own bound
///   ([`geom_brep::tori_coincide`]), and reads nothing more here.
fn coincident_over_extent_as<T: Decide>(
    c1: &CarrierDesc<T>,
    c2: &CarrierDesc<T>,
    extent: &ConsumedExtent<T>,
    (relation, data): (CarrierRelation, MarginDiag),
    band: Band,
) -> Result<MarginDiag, Indeterminate> {
    let sigma = match relation {
        CarrierRelation::SameOpposite => -T::one(),
        CarrierRelation::SameOriented | CarrierRelation::Distinct => T::one(),
    };
    // Two tori's coincidence already stands on this bound.
    let Some(((row, straddle), upper)) = reading(c1, c2, extent.reach, sigma) else {
        return Ok(data);
    };
    match decide_reported(row, Margin::of(upper), band) {
        Ok(Decided {
            sign: Sign::Zero,
            margin,
        }) => Ok(data.jointly(margin)),
        upper => Err(geom_brep::ladder::data_beneath_sum(
            straddle, data, upper, band,
        )),
    }
}

/// [`reading`]'s rows: each kind's predicate and its straddle row.
const PLANE_REACH: (&str, &str) = ("bool_plane_reach", "bool_plane_reach_straddle");
const SPHERE_REACH: (&str, &str) = ("carrier_sphere_reach", "carrier_sphere_reach_straddle");
const CYL_REACH: (&str, &str) = ("carrier_cyl_reach", "carrier_cyl_reach_straddle");
const CONE_REACH: (&str, &str) = ("carrier_cone_reach", "carrier_cone_reach_straddle");

/// The displacement's upper bound between a same-kind, non-torus pair
/// over `reach`, with its predicate's name and its straddle row's
/// ([`coincident_over_extent_as`]), the planes' orientation sign `sigma`
/// decided; `None` for a kind mismatch or two tori.
fn reading<T: Decide>(
    c1: &CarrierDesc<T>,
    c2: &CarrierDesc<T>,
    reach: geom_brep::ExtentBall<T>,
    sigma: T,
) -> Option<((&'static str, &'static str), T)> {
    Some(match (*c1, *c2) {
        (
            CarrierDesc::Plane {
                origin: o1,
                normal: n1,
            },
            CarrierDesc::Plane {
                origin: o2,
                normal: n2,
            },
        ) => {
            let centre = reach.center();
            let offset = n1.dot(o1 - centre) - sigma * n2.dot(o2 - centre);
            let swing = Margin::levered((n1 - n2 * sigma).norm(), reach.lever()).value();
            (PLANE_REACH, offset.abs() + swing)
        }
        (
            CarrierDesc::Sphere {
                center: p1,
                radius: r1,
                ..
            },
            CarrierDesc::Sphere {
                center: p2,
                radius: r2,
                ..
            },
        ) => (SPHERE_REACH, (r1 - r2).abs() + (p1 - p2).norm()),
        (
            CarrierDesc::Cylinder {
                origin: o1,
                axis: a1,
                radius: r1,
                ..
            },
            CarrierDesc::Cylinder {
                origin: o2,
                axis: a2,
                radius: r2,
                ..
            },
        ) => {
            let pivot = reach.foot_on(geom::Line {
                origin: o2,
                dir: a2,
            });
            let apart = perpendicular(pivot - o1, a1).norm();
            let arm = reach.lever_from_within(pivot, apart);
            let core = apart + Margin::levered(line_tilt(a1, a2), arm).value();
            (CYL_REACH, (r1 - r2).abs() + core)
        }
        (
            CarrierDesc::Cone {
                apex: p1,
                axis: a1,
                half_angle: t1,
                ..
            },
            CarrierDesc::Cone {
                apex: p2,
                axis: a2,
                half_angle: t2,
                ..
            },
        ) => {
            let apart = (p1 - p2).norm();
            let arm = reach.lever_from_within(p1, apart);
            let core = apart + Margin::levered(line_tilt(a1, a2) + (t1 - t2).abs(), arm).value();
            (CONE_REACH, core)
        }
        _ => return None,
    })
}

/// The curved arms' rungs: the kind's margin list read as an identity
/// ladder ([`geom_brep::ladder::read_identity`]). A datum decided apart
/// means `Distinct`, every datum decided Zero is one carrier, and
/// otherwise the ladder's undecided datum is the typed `Undecided`
/// refusal.
fn data_rungs<T: Decide>(
    margins: &[Datum<T>],
    aligned: bool,
    band: Band,
) -> Result<CarrierVerdict, CarrierEqError> {
    let same = if aligned {
        CarrierRelation::SameOriented
    } else {
        CarrierRelation::SameOpposite
    };
    match geom_brep::ladder::read_identity(margins.iter().copied(), band) {
        IdentityLadder::Apart { decided, .. } => Ok(CarrierVerdict::Apart(decided)),
        IdentityLadder::Zero(Some(margin)) => Ok(CarrierVerdict::One {
            relation: same,
            margin,
        }),
        IdentityLadder::Zero(None) => {
            unreachable!("every curved kind's ladder reads at least one datum")
        }
        IdentityLadder::Undecided(diag) => Err(CarrierEqError::Undecided {
            coincidence: CoincidenceMeasure::not_zero(diag),
            // The alignment this traversal was run under.
            relation: same,
        }),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Tol;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn sphere(c: [f64; 3], r: f64, outward: bool) -> CarrierDesc<f64> {
        CarrierDesc::Sphere {
            center: Point3::from_array(c),
            radius: r,
            outward,
        }
    }

    fn cyl(o: [f64; 3], a: [f64; 3], r: f64, outward: bool) -> CarrierDesc<f64> {
        CarrierDesc::Cylinder {
            origin: Point3::from_array(o),
            axis: Vec3::from_array(a),
            radius: r,
            outward,
        }
    }

    fn torus(c: [f64; 3], a: [f64; 3], major: f64, minor: f64, outward: bool) -> CarrierDesc<f64> {
        CarrierDesc::Torus {
            center: Point3::from_array(c),
            axis: Vec3::from_array(a),
            major_radius: major,
            minor_radius: minor,
            outward,
        }
    }

    fn cone(apex: [f64; 3], a: [f64; 3], half_angle: f64, outward: bool) -> CarrierDesc<f64> {
        CarrierDesc::Cone {
            apex: Point3::from_array(apex),
            axis: Vec3::from_array(a),
            half_angle,
            outward,
        }
    }

    /// **Two cone descriptions are one carrier when they describe one
    /// double cone**: its axis reversed is the same carrier, since a
    /// face's chart may put it on either nappe. An axis tilted so the
    /// generators part by a thousand bands across the extent is another
    /// carrier, and a half-angle off by a few bands is in band.
    /// **The inventory's refusals name the inventory**: the literal the
    /// refusals `concat!` is [`INVENTORY`] in its order, and each kind
    /// in it has a description.
    #[test]
    fn the_inventory_named_is_the_inventory() {
        let names: Vec<&str> = INVENTORY.iter().map(|k| k.name()).collect();
        assert_eq!(inventory_named!(), names.join(", "));
        // The complement is the definition the inventory answers to: the
        // ladder describes every kind with a closed-form carrier, and the
        // kinds without one are the spline and the fitted surface, which
        // `geom` holds as a net rather than as data a rung can compare
        // (`geom::Surface::nurbs`), and the spindle sheet, whose rung is
        // not written yet (`face_carrier` answers it `None`). A new
        // analytic kind turns this red until the ladder carries it or
        // is named here.
        for kind in geom::SurfaceKind::ALL {
            assert_eq!(
                in_inventory(kind),
                !matches!(
                    kind,
                    geom::SurfaceKind::Nurbs
                        | geom::SurfaceKind::Approx
                        | geom::SurfaceKind::Spindle
                ),
                "{kind:?}: the ladder describes every analytic kind"
            );
        }
    }

    #[test]
    fn a_cone_is_one_carrier_under_its_reversed_axis() {
        let b = band();
        let arm = 2.0;
        let one = cone([0.0, 1.0, 0.0], [0.0, 1.0, 0.0], 0.5, true);
        let read = |other: &CarrierDesc<f64>| {
            pair_door_reading(&one, other, &at(arm), b).map(PairDoorReading::verdict)
        };
        for (what, other) in [
            ("itself", one),
            (
                "reversed",
                cone([0.0, 1.0, 0.0], [0.0, -1.0, 0.0], 0.5, true),
            ),
        ] {
            assert!(
                matches!(
                    read(&other),
                    Ok(CarrierVerdict::One {
                        relation: CarrierRelation::SameOriented,
                        ..
                    })
                ),
                "{what}: one carrier, {:?}",
                read(&other)
            );
        }
        let tilt = 1000.0 * b.escalate() / arm;
        let tilted = cone(
            [0.0, 1.0, 0.0],
            Vec3::new(tilt, 1.0, 0.0).normalize().to_array(),
            0.5,
            true,
        );
        assert!(
            matches!(
                read(&tilted),
                Ok(CarrierVerdict::Apart(Decided {
                    sign: Sign::Positive,
                    ..
                }))
            ),
            "tilted, on the tilt's reading: {:?}",
            read(&tilted)
        );
        let wider = cone(
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            0.5 + 3.0 * b.zero() / arm,
            true,
        );
        assert!(
            matches!(read(&wider), Err(CarrierEqError::Undecided { .. })),
            "a half-angle off by a few bands: {:?}",
            read(&wider)
        );
    }

    /// A ball of radius `arm` about the origin.
    fn at(arm: f64) -> ConsumedExtent<f64> {
        ConsumedExtent::arm(arm)
    }

    /// **The summed bound reads at the face-pair door only.** Two
    /// spheres whose centres stand `0.6·ε` apart and whose radii differ
    /// by `0.6·ε`: each datum decides zero, their sum (`1.2·ε`) does not.
    /// The ladder as the corner sites read it calls the pair one carrier
    /// (every margin decided at zero); the face-pair door, which glues
    /// only where the displacement over the extent decides zero, refuses
    /// it with the sum's in-band reading instead.
    #[test]
    fn the_summed_bound_reads_at_the_pair_door_and_not_at_the_corners() {
        let e = band().zero();
        let a = sphere([0.0, 0.0, 0.0], 2.0, true);
        let b = sphere([0.6 * e, 0.0, 0.0], 2.0 + 0.6 * e, true);
        match carrier_eq_reading(&a, &b, &at(1.0), band()) {
            Ok(CarrierVerdict::One {
                relation: CarrierRelation::SameOriented,
                margin,
            }) => {
                assert!(
                    !margin.is_invalid(),
                    "the first datum's decided margin rides"
                );
            }
            other => panic!("the corner sites' ladder: {other:?}"),
        }
        match pair_door_reading(&a, &b, &at(1.0), band()) {
            Err(CarrierEqError::Undecided {
                coincidence: CoincidenceMeasure::Undecided(diag),
                ..
            }) => {
                assert!(
                    !diag.margin.is_invalid() && diag.predicate == Some(SPHERE_REACH.0),
                    "the sum in band: {diag:?}"
                );
            }
            other => panic!("the pair door: {other:?}"),
        }
    }

    /// **A coincidence at the face-pair door stands for every Zero it
    /// rests on**: the data's and the reach's. In a deferring session each
    /// Zero is left on the list; two copies of one sphere read one carrier
    /// on the centre's and the radius's Zeros at the corner, and on the
    /// reach's too at the face-pair door, so the door's margin names three
    /// forms where the corner's names two. Mutant: the door's margin the
    /// data's alone names two, the reach's alone one.
    #[test]
    fn a_coincidence_at_the_pair_door_stands_for_the_data_and_the_reach() {
        use geom_core::sym::deferred::{render, with_deferred_session};
        use geom_core::{Real, Sym, SymBudget, SymRetry, SymRules};
        type S = Sym<f64>;
        let budget = SymBudget {
            max_terms: 256,
            max_degree: 64,
        };
        let (forms, _) =
            with_deferred_session(budget, SymRules::shipped(), SymRetry::none(), || {
                let k = |v: f64| S::from_f64(v);
                let ball = CarrierDesc::Sphere {
                    center: Point3::new(k(1.0), k(2.0), k(3.0)),
                    radius: k(2.0),
                    outward: true,
                };
                let extent = ConsumedExtent::arm(k(1.0));
                let margin = |read: Result<CarrierVerdict, CarrierEqError>| {
                    read.unwrap().one().expect("one carrier").1
                };
                let at_corner = margin(carrier_eq_reading(&ball, &ball, &extent, band()));
                let at_door = margin(
                    pair_door_reading(&ball, &ball, &extent, band()).map(PairDoorReading::verdict),
                );
                (render(at_corner).len(), render(at_door).len())
            });
        assert_eq!(forms, (2, 3), "the corner's Zeros, and the door's");
    }

    /// **A curved datum that is not finite is unreadable, and is
    /// reported ahead of a datum in band.** Two spheres whose centres
    /// stand apart inside the ambiguity band, the second's radius NaN
    /// or `+∞`: the radius decides no sign, so the pair is neither
    /// apart nor in band, and the refusal names the radius.
    #[test]
    fn an_unreadable_curved_datum_is_reported_ahead_of_one_in_band() {
        let b = band();
        let apart = (b.zero() + b.escalate()) / 2.0;
        let a = sphere([0.0, 0.0, 0.0], 2.0, true);
        for radius in [f64::NAN, f64::INFINITY] {
            let poisoned = sphere([apart, 0.0, 0.0], radius, true);
            match carrier_eq(&a, &poisoned, &at(1.0), b) {
                Err(CarrierEqError::Undecided {
                    coincidence: CoincidenceMeasure::Unreadable(diag),
                    ..
                }) => {
                    assert!(diag.margin.is_invalid(), "{radius}: {diag:?}");
                    assert_eq!(diag.predicate, Some(CARRIER_SPHERE_RADIUS), "{radius}");
                }
                other => panic!("a radius of {radius} is unreadable: {other:?}"),
            }
        }
    }

    /// **An infinite coincidence datum is a limit, recorded as an
    /// escalation and never as a verdict**: the ladder's one non-finite
    /// rule ([`geom_brep::ladder::decided`]), read here too.
    #[test]
    fn an_infinite_datum_is_unreadable_and_recorded_as_an_escalation() {
        for poison in [f64::INFINITY, f64::NEG_INFINITY, f64::NAN] {
            let bracket = geom_core::k_stats::Bracket::open();
            let got = CoincidenceMeasure::decide("datum", Margin::of(poison), band());
            let recorded = bracket.finish();
            let Err(CoincidenceMeasure::Unreadable(diag)) = got else {
                panic!("{poison}: unreadable: {got:?}")
            };
            assert!(diag.margin.is_invalid(), "{poison}: {diag:?}");
            assert!(
                recorded.verdicts.is_empty(),
                "{poison}: no verdict recorded"
            );
            assert_eq!(recorded.escalations.len(), 1, "{poison}: one escalation");
        }
    }

    /// The peg-in-bore row: value-equal radii, opposed material
    /// sides. A margin decided Zero is the `Rest` verdict.
    #[test]
    fn sphere_value_equal_is_one_carrier() {
        let a = sphere([0.0, 0.0, 0.0], 2.0, true);
        let b = sphere([0.0, 0.0, 0.0], 2.0, false);
        assert_eq!(
            carrier_eq(&a, &b, &at(1.0), band()).unwrap(),
            CarrierRelation::SameOpposite
        );
    }

    /// Aligned coincidence is reported honestly as `SameOriented` —
    /// the carrier ladder's job is the carrier; refusing containment
    /// is the contact door's job (module docs, C1 lemma).
    #[test]
    fn sphere_aligned_is_same_oriented_not_a_carrier_error() {
        let a = sphere([1.0, 0.0, 0.0], 2.0, true);
        let b = sphere([1.0, 0.0, 0.0], 2.0, true);
        assert_eq!(
            carrier_eq(&a, &b, &at(1.0), band()).unwrap(),
            CarrierRelation::SameOriented
        );
    }

    /// ε-row on the sphere's radius: in band it is undecided, typed
    /// (in-band is never a silent pass); a definite difference is two
    /// carriers.
    #[test]
    fn sphere_radius_epsilon_row() {
        let b = band();
        let a = sphere([0.0, 0.0, 0.0], 2.0, true);
        let in_band = sphere(
            [0.0, 0.0, 0.0],
            2.0 + (b.zero() + b.escalate()) * 0.5,
            false,
        );
        assert!(
            matches!(
                carrier_eq(&a, &in_band, &at(1.0), band()),
                Err(CarrierEqError::Undecided { .. })
            ),
            "in band: undecided"
        );
        let definite = sphere([0.0, 0.0, 0.0], 2.001, false);
        assert_eq!(
            carrier_eq(&a, &definite, &at(1.0), band()).unwrap(),
            CarrierRelation::Distinct,
            "definite: two carriers"
        );
    }

    /// A cylinder's axis is a LINE: reversing the stored direction is
    /// the same carrier, and the material side comes from `outward`
    /// alone.
    #[test]
    fn cylinder_axis_direction_sign_is_not_material() {
        let a = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0, true);
        let b = cyl([0.0, 0.0, 5.0], [0.0, 0.0, -1.0], 3.0, false);
        assert_eq!(
            carrier_eq(&a, &b, &at(1.0), band()).unwrap(),
            CarrierRelation::SameOpposite
        );
    }

    /// A parallel-but-offset axis is a definitely different cylinder.
    #[test]
    fn cylinder_offset_axis_is_distinct() {
        let a = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0, true);
        let b = cyl([0.5, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0, false);
        assert_eq!(
            carrier_eq(&a, &b, &at(1.0), band()).unwrap(),
            CarrierRelation::Distinct
        );
    }

    /// ε-row on the cylinder's own margins — the row the sphere already
    /// had, owed to every new margin. The radius datum carries it: in
    /// band the pair is undecided; definite, it is two carriers.
    #[test]
    fn cylinder_radius_epsilon_row() {
        // Derived from the RUN's band, never a literal: the same
        // number is a gap at one ε and nothing at another, and a row
        // that only means what it says at one ε is not an ε row.
        let b = band();
        let a = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0, true);
        let in_band = cyl(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            3.0 + (b.zero() + b.escalate()) * 0.5,
            false,
        );
        assert!(
            matches!(
                carrier_eq(&a, &in_band, &at(1.0), band()),
                Err(CarrierEqError::Undecided { .. })
            ),
            "in band: undecided"
        );
        let definite = cyl(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            3.0 + b.escalate() * 1000.0,
            false,
        );
        assert_eq!(
            carrier_eq(&a, &definite, &at(1.0), band()).unwrap(),
            CarrierRelation::Distinct,
            "definite: two carriers"
        );
    }

    /// The cylinder's ANGULAR margin is metered at its named lever
    /// arm, which is the arm the caller passes: a tilt that is
    /// indecisive over a 1 m consumption extent is definite over a
    /// 1000 km one. Same geometry, two arms, two honest answers.
    #[test]
    fn cylinder_axis_tilt_is_decided_at_the_lever_arm() {
        let b = band();
        let a = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0, true);
        // A tilt whose displacement is sub-band over 1 m and decisive
        // over 1000 km — stated in band units so the row survives
        // every ε the matrix runs.
        let tilt = b.zero() * 0.1;
        let tilted = cyl([0.0, 0.0, 0.0], [tilt, 0.0, 1.0], 3.0, false);
        assert_eq!(
            carrier_eq(&a, &tilted, &at(1.0), band()).unwrap(),
            CarrierRelation::SameOpposite,
            "at a 1 m arm the tilt is below the band: one carrier"
        );
        assert_eq!(
            carrier_eq(&a, &tilted, &at(1e6), band()).unwrap(),
            CarrierRelation::Distinct,
            "at a 1000 km arm the tilt is definite: two carriers"
        );
    }

    /// The toroidal peg-in-bore row: one carrier, opposed material
    /// sides — the `Rest` verdict its margins decide, the torus arm's
    /// version of the sphere and cylinder rows above.
    #[test]
    fn torus_value_equal_is_one_carrier() {
        let a = torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 0.06, true);
        let b = torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 0.06, false);
        assert_eq!(
            carrier_eq(&a, &b, &at(1.0), band()).unwrap(),
            CarrierRelation::SameOpposite
        );
    }

    /// A torus's axis is a LINE: reversing the stored direction leaves
    /// the same point set, and the material side comes from `outward`
    /// alone.
    #[test]
    fn torus_axis_direction_sign_is_not_material() {
        let a = torus([1.0, 2.0, 3.0], [0.0, 0.0, 1.0], 5.0, 0.06, true);
        let b = torus([1.0, 2.0, 3.0], [0.0, 0.0, -1.0], 5.0, 0.06, false);
        assert_eq!(
            carrier_eq(&a, &b, &at(1.0), band()).unwrap(),
            CarrierRelation::SameOpposite
        );
    }

    /// **The centre is a POINT, not an axis anchor.** Sliding a torus
    /// ALONG its own axis moves the carrier — the datum a cylinder's
    /// perpendicular-projection offset would have thrown away, which
    /// is why the torus arm compares the whole centre separation.
    #[test]
    fn torus_slid_along_its_own_axis_is_a_different_carrier() {
        let a = torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 0.06, true);
        let b = torus([0.0, 0.0, 0.5], [0.0, 0.0, 1.0], 5.0, 0.06, false);
        assert_eq!(
            carrier_eq(&a, &b, &at(1.0), band()).unwrap(),
            CarrierRelation::Distinct
        );
    }

    /// The two radii are INDEPENDENT data, and each parts the carriers
    /// on its own: a bore of the right ring but the wrong tube is not
    /// the same carrier, and neither is the reverse.
    #[test]
    fn each_torus_radius_parts_the_carriers_on_its_own() {
        let a = torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 0.06, true);
        for (b, which) in [
            (
                torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.5, 0.06, false),
                "the major radius",
            ),
            (
                torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 0.09, false),
                "the minor radius",
            ),
        ] {
            assert_eq!(
                carrier_eq(&a, &b, &at(1.0), band()).unwrap(),
                CarrierRelation::Distinct,
                "{which}"
            );
        }
    }

    /// ε-row on the torus's own margins — owed to every new margin, and
    /// stated in BAND UNITS so it means the same thing at every ε the
    /// matrix runs. The minor radius carries it.
    #[test]
    fn torus_minor_radius_epsilon_row() {
        let b = band();
        let a = torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 0.06, true);
        let in_band = torus(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            5.0,
            0.06 + (b.zero() + b.escalate()) * 0.5,
            false,
        );
        assert!(
            matches!(
                carrier_eq(&a, &in_band, &at(1.0), band()),
                Err(CarrierEqError::Undecided { .. })
            ),
            "in band: undecided"
        );
        let definite = torus(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            5.0,
            0.06 + b.escalate() * 1000.0,
            false,
        );
        assert_eq!(
            carrier_eq(&a, &definite, &at(1.0), band()).unwrap(),
            CarrierRelation::Distinct,
            "definite: two carriers"
        );
    }

    /// The torus's ANGULAR margin is levered at its own ring, whatever
    /// extent the caller names: a tilt about a diameter swings the far
    /// side of the ring by the tilt times `R`. A tilt of `0.3·Kε`, in
    /// band at a metre, stands `1.5·Kε` off at the 5 m ring: two
    /// carriers, read over a metre.
    #[test]
    fn torus_axis_tilt_is_levered_at_the_ring() {
        let b = band();
        let a = torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 0.06, true);
        let tilt = b.escalate() * 0.3;
        let tilted = torus([0.0, 0.0, 0.0], [tilt, 0.0, 1.0], 5.0, 0.06, false);
        assert_eq!(
            carrier_eq(&a, &tilted, &at(1.0), band()).unwrap(),
            CarrierRelation::Distinct,
            "the swing at the ring is definite"
        );
    }

    /// **Two torus faces are one carrier exactly where their pair's pose
    /// decides them one surface** ([`geom_brep::tori_coincide`]), whatever
    /// extent the faces span, read at the face-pair door
    /// ([`super::super::carrier_pair_decided`]). The poses are the ring
    /// of `R = 2`, `r = 0.5` tilted so its far side swings `0.3ε`, slid
    /// `0.6ε` along and across its axis, its tube in the band, and its
    /// tube a thousand escalation widths wider, whose faces' corners
    /// witness it. Each pair is two opposed patches of one chart window,
    /// a quarter turn and a smaller one.
    #[test]
    fn the_ladder_reads_two_tori_as_their_pose_does() {
        use crate::boxes::tests::torus_wall;
        let b = band();
        let e = b.zero();
        let surface = |(center, axis, major, minor): (Point3<f64>, Vec3<f64>, f64, f64)| {
            let axis = axis.normalize();
            geom::Surface::Torus {
                center,
                axis,
                major_radius: major,
                minor_radius: minor,
                u_ref: axis.orthonormal_basis().0,
            }
        };
        let ring = (Point3::origin(), Vec3::unit_z(), 2.0, 0.5);
        for (what, other, one) in [
            (
                "tilted",
                (Point3::origin(), Vec3::new(0.148 * e, 0.0, 1.0), 2.0, 0.5),
                Some(true),
            ),
            (
                "slid",
                (Point3::new(0.6 * e, 0.0, 0.6 * e), Vec3::unit_z(), 2.0, 0.5),
                Some(true),
            ),
            (
                "in band",
                (
                    Point3::origin(),
                    Vec3::unit_z(),
                    2.0,
                    0.5 + 0.5 * (e + b.escalate()),
                ),
                None,
            ),
            (
                "apart",
                (
                    Point3::origin(),
                    Vec3::unit_z(),
                    2.0,
                    0.5 + 1000.0 * b.escalate(),
                ),
                Some(false),
            ),
        ] {
            let pose = match geom_brep::TorusTorusPose::decide(&surface(ring), &surface(other), b)
                .expect("two tori")
            {
                Err(geom_brep::TorusPoseRefusal::Coincident) => Some(true),
                Err(geom_brep::TorusPoseRefusal::Undecided { .. }) => None,
                Ok(_) => Some(false),
                Err(refusal) => panic!("{what}: the pose refused {refusal:?}"),
            };
            assert_eq!(pose, one, "{what}: the pose");
            for us in [(0.3, 1.9), (0.3, 0.9)] {
                let wall = |(center, axis, major, minor): (Point3<f64>, Vec3<f64>, f64, f64)| {
                    let axis = axis.normalize();
                    let u_ref = axis.orthonormal_basis().0;
                    torus_wall(center, axis, u_ref, major, minor, us, (-0.7, 0.8))
                };
                let (a, fa) = wall(ring);
                let (mut o, fo) = wall(other);
                let sense = a.get_face(fa).unwrap().sense;
                o.set_face_sense(fo, !sense).unwrap();
                let read = match crate::boolean::carrier_pair_decided(&a, fa, &o, fo, b) {
                    Ok(Ok(CarrierVerdict::One {
                        relation: CarrierRelation::SameOpposite,
                        ..
                    })) => Some(true),
                    Ok(Ok(CarrierVerdict::Apart(_))) => Some(false),
                    Ok(Err(CarrierEqError::Undecided { .. })) => None,
                    other => panic!("{what}: the door read {other:?}"),
                };
                assert_eq!(read, pose, "{what}: the door over {us:?}");
            }
        }
    }

    /// **Data each within ε whose bound sums past it are undecided at
    /// the corner sites too**: the corner door reads the pair's one
    /// coincidence, which stands on the sum, where a datum-by-datum
    /// reading called this pair one carrier.
    #[test]
    fn torus_data_in_zero_band_summing_past_it_is_undecided_at_every_reader() {
        let b = band();
        let e = b.zero();
        let ring = torus([0.0; 3], [0.0, 0.0, 1.0], 2.0, 1.0, true);
        let other = torus(
            [0.0, 0.0, 0.9 * e],
            [0.0, 0.0, 1.0],
            2.0 + 0.9 * e,
            1.0,
            true,
        );
        let ext = ConsumedExtent::new(geom_brep::ExtentBall::of_sphere(
            Point3::new(1.0, 0.0, 0.0),
            1e-3,
        ));
        assert!(matches!(
            carrier_eq(&ring, &other, &ext, b),
            Err(CarrierEqError::Undecided { .. })
        ));
    }

    /// **A torus off its ring convention has no carrier to compare**:
    /// a spindle (`r > R`) is refused at the ladder on the convention's
    /// own decided verdict, naming which torus of the pair it is, as the
    /// pose refuses it, never one carrier on a bound that holds only for
    /// a ring.
    #[test]
    fn a_spindle_torus_is_refused_on_its_convention_at_the_ladder() {
        let spindle = torus([0.0; 3], [0.0, 0.0, 1.0], 1.0, 1.5, true);
        let ring = torus([0.0; 3], [0.0, 0.0, 1.0], 2.0, 0.5, true);
        for (x, y, side) in [
            (ring, spindle, geom_brep::PairSide::Second),
            (spindle, ring, geom_brep::PairSide::First),
        ] {
            match carrier_eq(&x, &y, &at(1.0), band()) {
                Err(CarrierEqError::OffRing {
                    torus,
                    refusal:
                        geom_brep::TorusRefusal {
                            convention: geom_brep::TorusConvention::Ring,
                            verdict: geom_brep::TorusVerdict::Refused(_),
                        },
                }) => assert_eq!(torus, side, "the refusal names the spindle"),
                other => panic!("the spindle on the {side:?} side: {other:?}"),
            }
        }
    }

    /// **A torus off its ring convention ends at the Boolean as the
    /// pierce normal ends it** (`boolean::off_ring_torus`), never as
    /// poison: the face it names refused in band as the half's
    /// escalation ([`super::super::BooleanDecision::Torus`]) and decided as
    /// [`super::super::BooleanError::DegenerateTorus`], each ending in the
    /// half's lever, on the operand and face of the torus the ladder
    /// named.
    #[test]
    fn a_torus_off_its_ring_ends_in_its_halfs_lever_at_the_boolean() {
        use super::super::{BooleanDecision, BooleanError, Operand, off_ring_torus};
        use geom_brep::TorusConvention::{Ring, Tube};
        let b = band();
        let mid = 0.5 * (b.zero() + b.escalate());
        let key = |n: u64| crate::entity::FaceKey::from(slotmap::KeyData::from_ffi((1 << 32) | n));
        let ring = torus([0.0; 3], [0.0, 0.0, 1.0], 2.0, 0.5, true);
        let tube = "reshape the torus so its tube is clearly thicker than the tolerance";
        let ring_lever = "make the tube radius clearly smaller than the ring radius";
        for (what, major, minor, half, in_band, lever) in [
            ("tube in band", 2.0, mid, Tube, true, tube),
            ("tube negative", 2.0, -0.5, Tube, false, tube),
            ("ring in band", 0.5 + mid, 0.5, Ring, true, ring_lever),
            ("horn", 0.5, 0.5, Ring, false, ring_lever),
            ("spindle", 0.5, 2.0, Ring, false, ring_lever),
        ] {
            let off = torus([0.0; 3], [0.0, 0.0, 1.0], major, minor, true);
            for (side, (x, y), named) in [
                (
                    geom_brep::PairSide::Second,
                    (ring, off),
                    (Operand::B, key(2)),
                ),
                (
                    geom_brep::PairSide::First,
                    (off, ring),
                    (Operand::A, key(1)),
                ),
            ] {
                let what = format!("{what}, {side:?}");
                let Err(CarrierEqError::OffRing { torus, refusal }) =
                    carrier_eq(&x, &y, &at(1.0), b)
                else {
                    panic!("{what}: the ladder refuses the convention");
                };
                assert_eq!(torus, side, "{what}");
                let err =
                    off_ring_torus(torus, refusal, [(Operand::A, key(1)), (Operand::B, key(2))]);
                match (&err, in_band) {
                    (
                        BooleanError::Escalated {
                            decision: BooleanDecision::Torus(got),
                            ..
                        },
                        true,
                    ) => assert_eq!(*got, half, "{what}"),
                    (
                        BooleanError::DegenerateTorus {
                            convention,
                            operand,
                            face,
                            ..
                        },
                        false,
                    ) => {
                        assert_eq!(*convention, half, "{what}");
                        assert_eq!(
                            (*operand, *face),
                            named,
                            "{what}: the face the ladder named"
                        );
                    }
                    _ => panic!("{what}: the half's refusal, got {err:?}"),
                }
                let text = err.render(geom_brep::recourse::Reading::Build);
                assert!(
                    text.contains(lever) && !text.contains("not finite"),
                    "{what}: the half's lever: {text}"
                );
            }
        }
    }

    /// Aligned coincidence is the carrier ladder's honest
    /// `SameOriented`, on the torus arm as on every other: two
    /// same-carrier walls facing the SAME way glue as a continuation.
    #[test]
    fn torus_aligned_is_same_oriented() {
        let a = torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 0.06, true);
        let b = torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 0.06, true);
        assert_eq!(
            carrier_eq(&a, &b, &at(1.0), band()).unwrap(),
            CarrierRelation::SameOriented
        );
    }

    /// A torus is not a cylinder at any pair of radii: the kind rung
    /// answers structurally, on no datum, in both directions.
    #[test]
    fn torus_against_cylinder_is_a_kind_mismatch() {
        let t = torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 0.06, true);
        let c = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, false);
        assert_eq!(
            carrier_eq_reading(&t, &c, &at(1.0), band()),
            Ok(CarrierVerdict::TwoKinds)
        );
        assert_eq!(
            carrier_eq_reading(&c, &t, &at(1.0), band()),
            Ok(CarrierVerdict::TwoKinds)
        );
    }

    /// Kinds do not compare: a plane is not a cylinder at any radius.
    #[test]
    fn kind_mismatch_is_distinct() {
        let p = CarrierDesc::Plane {
            origin: Point3::origin(),
            normal: Vec3::new(0.0, 0.0, 1.0),
        };
        let c = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0, true);
        assert_eq!(
            carrier_eq(&p, &c, &at(1.0), band()).unwrap(),
            CarrierRelation::Distinct
        );
    }

    /// The plane arm is the plane ladder's: `carrier_eq` on two plane
    /// descriptions agrees with `oriented_plane_eq` called directly,
    /// verdict for verdict.
    #[test]
    fn plane_arm_delegates_unchanged() {
        let p1 = PlaneDesc {
            origin: Point3::origin(),
            normal: Vec3::new(0.0, 0.0, 1.0),
        };
        let p2 = PlaneDesc {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, -1.0),
        };
        let via_carrier = carrier_eq(
            &CarrierDesc::Plane {
                origin: p1.origin,
                normal: p1.normal,
            },
            &CarrierDesc::Plane {
                origin: p2.origin,
                normal: p2.normal,
            },
            &at(1.0),
            band(),
        )
        .unwrap();
        let direct =
            crate::boolean::plane_eq::oriented_plane_eq(&p1, &p2, &at(1.0), band()).unwrap();
        assert_eq!(via_carrier, direct);
        assert_eq!(direct, CarrierRelation::SameOpposite);
    }

    /// **The curved ladder's material side is the faces' `outward`
    /// bits.** One cylinder face against itself, against its reverted
    /// body, and against a twin with its sense flipped, each pair both
    /// ways: the reverted and the flipped pair read opposed.
    #[test]
    fn the_curved_ladder_reads_a_reverted_face_as_opposed() {
        use CarrierRelation::{SameOpposite, SameOriented};
        let mut body = crate::Body::<f64>::new();
        let (face, _) = crate::test_support_fixtures::unit_cyl_sheet(
            &mut body,
            None,
            (0.0, 1.0),
            (0.0, 1.0),
            true,
            Tol::witness(),
        );
        let reverted = body.revert().unwrap();
        let mut flipped = body.clone();
        flipped.set_face_sense(face, false).unwrap();
        for (name, other, rung) in [
            ("itself", &body, SameOriented),
            ("its reverted body", &reverted, SameOpposite),
            ("its sense flipped", &flipped, SameOpposite),
        ] {
            for (x, y) in [(&body, other), (other, &body)] {
                assert_eq!(
                    crate::boolean::carrier_pair::carrier_pair_relation(x, face, y, face, band())
                        .unwrap()
                        .unwrap(),
                    rung,
                    "the curved ladder, a face against {name}"
                );
            }
        }
    }
}
