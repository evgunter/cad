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
//! somewhere, not necessarily on the faces. Its `Distinct` is the ring's;
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
//! with the verdict ([`CarrierReading`]) for the caller to record
//! ([`crate::coincidence`]). Whether they are one CONSTRUCTION is the
//! document's coincidence door's question, never the kernel's. A datum
//! in band is a sliver and refuses ([`CarrierEqError::Undecided`]).

use geom_core::{Band, Decide, Decided, Indeterminate, Margin, Point3, Sign, Vec3};

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
    /// displacement's bound, in band (a sliver, F6) or poisoned. Carries
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
}

/// What an undecided coincidence's measure read
/// ([`CarrierEqError::Undecided`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CoincidenceMeasure {
    /// A datum, or the displacement's bound over the pair's extent, did
    /// not decide zero: in band, or past it where the bound stands off.
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

    /// One coincidence datum decided: its sign and the margin the band
    /// decided, or the measure it leaves where it decides nothing. A
    /// datum that is not finite (NaN or ±∞) is unreadable whatever sign
    /// it reads: an infinite offset is no more a locus than a NaN one.
    pub(crate) fn decide<T: Decide>(
        name: &'static str,
        margin: Margin<T>,
        band: Band,
    ) -> Result<Decided, Self> {
        let finite = geom_core::is_finite_length(margin.value());
        match decide_reported(name, margin, band) {
            Ok(decided) if finite => Ok(decided),
            Ok(_) => Err(Self::Unreadable(Indeterminate {
                margin: geom_core::MarginDiag::INVALID,
                band,
                predicate: Some(name),
                terminal_sliver: false,
            })),
            Err(diag) => Err(Self::not_zero(diag)),
        }
    }

    /// A measure that did not decide zero, as what it is: unreadable
    /// where its margin is poisoned, undecided otherwise.
    pub(crate) fn not_zero(diag: Indeterminate) -> Self {
        if diag.margin.is_invalid() {
            Self::Unreadable(diag)
        } else {
            Self::Undecided(diag)
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
        Self::new(geom_brep::ExtentBall::new(Point3::origin(), arm))
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
    carrier_eq_reading(c1, c2, extent, band).map(|(rel, _)| rel)
}

/// A carrier verdict and the margin that decided the two carriers one:
/// the first datum decided Zero, with the displacement's bound where
/// the face-pair door read it. `None` for a `Distinct` verdict, which
/// glues nothing.
pub(crate) type CarrierReading = (CarrierRelation, Option<geom_core::MarginDiag>);

/// [`carrier_eq`] with the margin that decided a coincidence, which
/// the coincidence's row records ([`crate::coincidence`]): each datum
/// read on its own, the angular ones levered at the extent
/// ([`at_consumed_extent`]).
pub(crate) fn carrier_eq_reading<T: Decide>(
    c1: &CarrierDesc<T>,
    c2: &CarrierDesc<T>,
    extent: &ConsumedExtent<T>,
    band: Band,
) -> Result<CarrierReading, CarrierEqError> {
    let (c1, c2, arm) = at_consumed_extent(c1, c2, extent.reach);
    match (&c1, &c2) {
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
            match geom_brep::tori_coincide(
                torus((*p1, *a1, *r1, *t1)),
                torus((*p2, *a2, *r2, *t2)),
                band,
            ) {
                geom_brep::TorusCoincidence::One(margin) => Ok((same, Some(margin))),
                geom_brep::TorusCoincidence::Apart => Ok((CarrierRelation::Distinct, None)),
                geom_brep::TorusCoincidence::Undecided(diag) => {
                    Err(undecided(CoincidenceMeasure::not_zero(diag)))
                }
                geom_brep::TorusCoincidence::OffRing(refusal) => {
                    Err(undecided(off_ring(&refusal, band)))
                }
            }
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
        _ => Ok((CarrierRelation::Distinct, None)),
    }
}

/// [`carrier_eq_reading`] at the FACE-PAIR door: a coincidence glues
/// the pair only where the displacement's upper bound over the pair's
/// extent decides Zero too ([`coincident_over_extent`]). A torus pair's
/// coincidence already stands on that bound
/// ([`geom_brep::tori_coincide`]). The corner sites read their own arm,
/// not a pair's extent, and take [`carrier_eq_reading`] as it is.
///
/// # Errors
///
/// As [`carrier_eq`].
pub(super) fn pair_door_reading<T: Decide>(
    c1: &CarrierDesc<T>,
    c2: &CarrierDesc<T>,
    extent: &ConsumedExtent<T>,
    band: Band,
) -> Result<CarrierReading, CarrierEqError> {
    match carrier_eq_reading(c1, c2, extent, band) {
        Ok((
            relation @ (CarrierRelation::SameOriented | CarrierRelation::SameOpposite),
            margin,
        )) => {
            let sum = coincident_over_extent(c1, c2, extent, relation, band).map_err(|diag| {
                CarrierEqError::Undecided {
                    coincidence: CoincidenceMeasure::not_zero(diag),
                    relation,
                }
            })?;
            Ok((relation, margin.map(|m| sum.map_or(m, |s| m.jointly(s)))))
        }
        verdict => verdict,
    }
}

/// A margin the ladder reads, by its predicate's name.
type Datum<T> = (&'static str, Margin<T>);

/// The sphere's data: centre separation, radius difference.
fn sphere_data<T: Decide>(p1: Point3<T>, r1: T, p2: Point3<T>, r2: T) -> Vec<Datum<T>> {
    vec![
        ("carrier_sphere_center", Margin::norm3(p1 - p2)),
        ("carrier_sphere_radius", Margin::of(r1 - r2)),
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
    arm: T,
) -> Vec<Datum<T>> {
    vec![
        (
            "carrier_cyl_axis_parallel",
            Margin::levered(a1.cross(a2).norm(), arm),
        ),
        (
            "carrier_cyl_axis_offset",
            Margin::norm3(perpendicular(p2 - p1, a1)),
        ),
        ("carrier_cyl_radius", Margin::of(r1 - r2)),
    ]
}

/// The cone's data: the apexes' separation, and the axis lines'
/// parallelism and the half-angles' difference, each levered at `arm`,
/// the reach from the apex.
fn cone_data<T: Decide>(
    (p1, a1, t1): (Point3<T>, Vec3<T>, T),
    (p2, a2, t2): (Point3<T>, Vec3<T>, T),
    arm: T,
) -> Vec<Datum<T>> {
    vec![
        ("carrier_cone_apex", Margin::norm3(p1 - p2)),
        (
            "carrier_cone_axis_parallel",
            Margin::levered(a1.cross(a2).norm(), arm),
        ),
        ("carrier_cone_half_angle", Margin::levered(t1 - t2, arm)),
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

/// A torus off its ring convention as the coincidence it leaves
/// unread: in band where the convention's margin is, and otherwise
/// unreadable, since such a torus has no carrier to compare (the body's
/// own validity refuses it, `ValidationError::DegenerateTorus`, but the
/// ladder also reads bodies no validation has passed).
fn off_ring(refusal: &geom_brep::TorusRefusal, band: Band) -> CoincidenceMeasure {
    match &refusal.verdict {
        geom_brep::TorusVerdict::Undecided(diag) => CoincidenceMeasure::not_zero(*diag),
        geom_brep::TorusVerdict::Refused(decided) => {
            CoincidenceMeasure::Unreadable(Indeterminate {
                margin: decided.margin(),
                band,
                predicate: Some(match refusal.convention {
                    geom_brep::TorusConvention::Tube => "torus_tube_positive",
                    geom_brep::TorusConvention::Ring => "ring_torus_convention",
                }),
                terminal_sliver: false,
            })
        }
    }
}

/// `v`'s part perpendicular to the unit `axis`.
fn perpendicular<T: geom_core::Real>(v: Vec3<T>, axis: Vec3<T>) -> Vec3<T> {
    v - axis * v.dot(axis)
}

pub(crate) use geom_brep::line_tilt;

/// **The pair as the ladder reads it when its verdict is
/// consumed over `reach`**: each description anchored where its
/// position datum is read nearest the extent's centre, and the lever
/// arm for its angular data, the extent's farthest reach from that
/// pivot ([`geom_brep::ExtentBall`]'s module docs). Each datum is
/// decided on its own: a definite one says the carriers differ, and the
/// face-pair door then reads the data as one sum
/// ([`coincident_over_extent`]).
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
) -> (CarrierDesc<T>, CarrierDesc<T>, T) {
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
            let foot = reach.foot_on(origin, axis);
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
            return (*c1, *c2, reach.lever_from(p1) + (p1 - p2).norm());
        }
        (c1, c2) => (c1, c2, centre),
    };
    (c1, c2, reach.lever_from(pivot))
}

/// **A coincidence read as one displacement over the pair's extent.**
/// Every datum decided zero says "one carrier"; the face-pair door reads
/// the data's SUM, the displacement's upper bound at every point of the
/// extent's ball, so the coincidence stands only where that sum decides
/// zero too, whatever K. Where it does not, the refusal carries its
/// reading, in band or past it; where it does, its reading, which the
/// coincidence's margin stands for too.
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
fn coincident_over_extent<T: Decide>(
    c1: &CarrierDesc<T>,
    c2: &CarrierDesc<T>,
    extent: &ConsumedExtent<T>,
    relation: CarrierRelation,
    band: Band,
) -> Result<Option<geom_core::MarginDiag>, Indeterminate> {
    // Two tori's coincidence already stands on this bound.
    if matches!(
        (c1, c2),
        (CarrierDesc::Torus { .. }, CarrierDesc::Torus { .. })
    ) {
        return Ok(None);
    }
    let sigma = match relation {
        CarrierRelation::SameOpposite => -T::one(),
        CarrierRelation::SameOriented | CarrierRelation::Distinct => T::one(),
    };
    let Some((upper_name, upper)) = reading(c1, c2, extent.reach, sigma) else {
        return Ok(None);
    };
    match decide_reported(upper_name, Margin::of(upper), band) {
        Ok(Decided {
            sign: Sign::Zero,
            margin,
        }) => Ok(Some(margin)),
        Ok(Decided { margin, .. }) => Err(Indeterminate {
            margin,
            band,
            predicate: Some(upper_name),
            terminal_sliver: false,
        }),
        Err(diag) => Err(diag),
    }
}

/// The displacement's upper bound between a same-kind, non-torus pair
/// over `reach`, with its predicate's name ([`coincident_over_extent`]),
/// the planes' orientation sign `sigma` decided; `None` for a kind
/// mismatch or two tori.
fn reading<T: Decide>(
    c1: &CarrierDesc<T>,
    c2: &CarrierDesc<T>,
    reach: geom_brep::ExtentBall<T>,
    sigma: T,
) -> Option<(&'static str, T)> {
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
            let (centre, radius) = (reach.center(), reach.radius());
            let offset = n1.dot(o1 - centre) - sigma * n2.dot(o2 - centre);
            let swing = Margin::levered((n1 - n2 * sigma).norm(), radius).value();
            ("bool_plane_reach", offset.abs() + swing)
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
        ) => ("carrier_sphere_reach", (r1 - r2).abs() + (p1 - p2).norm()),
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
            let pivot = reach.foot_on(o2, a2);
            let apart = perpendicular(pivot - o1, a1).norm();
            let arm = reach.lever_from(pivot) + apart;
            let core = apart + Margin::levered(line_tilt(a1, a2), arm).value();
            ("carrier_cyl_reach", (r1 - r2).abs() + core)
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
            let arm = reach.lever_from(p1) + apart;
            let core = apart + Margin::levered(line_tilt(a1, a2) + (t1 - t2).abs(), arm).value();
            ("carrier_cone_reach", core)
        }
        _ => return None,
    })
}

/// The curved arms' rungs, driven by the
/// kind's margin list ([`identity_data`]): a definitely-nonzero margin
/// means `Distinct`, every datum decided Zero is one carrier, and a
/// datum in band or poisoned is the typed `Undecided` refusal.
fn data_rungs<T: Decide>(
    margins: &[Datum<T>],
    aligned: bool,
    band: Band,
) -> Result<CarrierReading, CarrierEqError> {
    let same = if aligned {
        CarrierRelation::SameOriented
    } else {
        CarrierRelation::SameOpposite
    };
    match identity_data(margins.iter().copied(), band) {
        IdentityData::Distinct => Ok((CarrierRelation::Distinct, None)),
        IdentityData::One(first_zero) => Ok((same, first_zero)),
        IdentityData::Undecided(coincidence) => Err(CarrierEqError::Undecided {
            coincidence,
            // The alignment this traversal was run under.
            relation: same,
        }),
    }
}

/// What a list of identity data reads ([`identity_data`]).
#[derive(Clone, Copy, Debug)]
pub(crate) enum IdentityData {
    /// Some datum decided definitely nonzero: two things.
    Distinct,
    /// Every datum decided Zero: one thing, the data's decided margins
    /// taken jointly (`None` for no data).
    One(Option<geom_core::MarginDiag>),
    /// No datum decided nonzero and one did not decide: the first that
    /// cannot be read, else the first in band.
    Undecided(CoincidenceMeasure),
}

/// **The identity ladder's rule** ([`geom_brep::identity_walk`]), its
/// refusal read as the coincidence it leaves undecided.
pub(crate) fn identity_data<T: Decide>(
    data: impl IntoIterator<Item = (&'static str, Margin<T>)>,
    band: Band,
) -> IdentityData {
    match geom_brep::identity_walk(data, band) {
        geom_brep::IdentityWalk::Distinct => IdentityData::Distinct,
        geom_brep::IdentityWalk::One(joint) => IdentityData::One(joint),
        geom_brep::IdentityWalk::Undecided(diag) => {
            IdentityData::Undecided(CoincidenceMeasure::not_zero(diag))
        }
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
        // (`geom::Surface::nurbs`). A new analytic kind turns this red
        // until the ladder carries it.
        for kind in geom::SurfaceKind::ALL {
            assert_eq!(
                in_inventory(kind),
                !matches!(kind, geom::SurfaceKind::Nurbs | geom::SurfaceKind::Approx),
                "{kind:?}: the ladder describes every analytic kind"
            );
        }
    }

    #[test]
    fn a_cone_is_one_carrier_under_its_reversed_axis() {
        let b = band();
        let arm = 2.0;
        let one = cone([0.0, 1.0, 0.0], [0.0, 1.0, 0.0], 0.5, true);
        let read = |other: &CarrierDesc<f64>| pair_door_reading(&one, other, &at(arm), b);
        for (what, other) in [
            ("itself", one),
            (
                "reversed",
                cone([0.0, 1.0, 0.0], [0.0, -1.0, 0.0], 0.5, true),
            ),
        ] {
            assert!(
                matches!(read(&other), Ok((CarrierRelation::SameOriented, Some(_)))),
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
            matches!(read(&tilted), Ok((CarrierRelation::Distinct, ..))),
            "tilted: {:?}",
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
            Ok((CarrierRelation::SameOriented, Some(margin))) => {
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
                    !diag.margin.is_invalid() && diag.predicate == Some("carrier_sphere_reach"),
                    "the sum in band: {diag:?}"
                );
            }
            other => panic!("the pair door: {other:?}"),
        }
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
                    assert_eq!(diag.predicate, Some("carrier_sphere_radius"), "{radius}");
                }
                other => panic!("a radius of {radius} is unreadable: {other:?}"),
            }
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

    /// **Two torus faces are one carrier exactly where
    /// their pair's pose decides them one surface**
    /// ([`geom_brep::tori_coincide`]), whatever extent the faces are read
    /// over. The poses are the ring of `R = 5`, `r = 0.06` tilted so its
    /// far side swings `0.74ε` (which the pose's gap read tangent beside
    /// the ladder's one carrier), slid `0.6ε` along and across its axis
    /// (which the pose escalated), its tube in the band, and its tube a
    /// thousand escalation widths wider.
    #[test]
    fn the_ladder_reads_two_tori_as_their_pose_does() {
        let b = band();
        let e = b.zero();
        let surface = |c: &CarrierDesc<f64>| match *c {
            CarrierDesc::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                ..
            } => geom::Surface::Torus {
                center,
                axis: axis.normalize(),
                major_radius,
                minor_radius,
                u_ref: axis.normalize().orthonormal_basis().0,
            },
            _ => unreachable!("tori"),
        };
        let ring = torus([0.0; 3], [0.0, 0.0, 1.0], 5.0, 0.06, true);
        let over = [
            at(1.0),
            ConsumedExtent::new(geom_brep::ExtentBall::of_torus(Point3::origin(), 5.0, 0.06)),
        ];
        for (what, other, one) in [
            (
                "tilted",
                torus([0.0; 3], [0.148 * e, 0.0, 1.0], 5.0, 0.06, false),
                Some(true),
            ),
            (
                "slid",
                torus([0.6 * e, 0.0, 0.6 * e], [0.0, 0.0, 1.0], 5.0, 0.06, false),
                Some(true),
            ),
            (
                "in band",
                torus(
                    [0.0; 3],
                    [0.0, 0.0, 1.0],
                    5.0,
                    0.06 + 0.5 * (e + b.escalate()),
                    false,
                ),
                None,
            ),
            (
                "apart",
                torus(
                    [0.0; 3],
                    [0.0, 0.0, 1.0],
                    5.0,
                    0.06 + 1000.0 * b.escalate(),
                    false,
                ),
                Some(false),
            ),
        ] {
            let pose = match geom_brep::TorusTorusPose::decide(&surface(&ring), &surface(&other), b)
                .expect("two tori")
            {
                Err(geom_brep::TorusPoseRefusal::Coincident) => Some(true),
                Err(geom_brep::TorusPoseRefusal::Undecided { .. }) => None,
                Ok(_) => Some(false),
                Err(refusal) => panic!("{what}: the pose refused {refusal:?}"),
            };
            assert_eq!(pose, one, "{what}: the pose");
            for extent in &over {
                let read = match pair_door_reading(&ring, &other, extent, b) {
                    Ok((CarrierRelation::SameOpposite, Some(_))) => Some(true),
                    Ok((CarrierRelation::Distinct, ..)) => Some(false),
                    Err(CarrierEqError::Undecided { .. }) => None,
                    other => panic!("{what}: the ladder read {other:?}"),
                };
                assert_eq!(read, pose, "{what}: the ladder over {:?}", extent.reach);
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
        let ext = ConsumedExtent::new(geom_brep::ExtentBall::new(Point3::new(1.0, 0.0, 0.0), 1e-3));
        assert!(matches!(
            carrier_eq(&ring, &other, &ext, b),
            Err(CarrierEqError::Undecided { .. })
        ));
    }

    /// **A torus off its ring convention has no carrier to compare**:
    /// a spindle (`r > R`) against itself is unreadable at the ladder, as
    /// the pose refuses it, never one carrier on a bound that holds only
    /// for a ring.
    #[test]
    fn a_spindle_torus_is_unreadable_at_the_ladder() {
        let spindle = torus([0.0; 3], [0.0, 0.0, 1.0], 1.0, 1.5, true);
        match carrier_eq(&spindle, &spindle, &at(1.0), band()) {
            Err(CarrierEqError::Undecided {
                coincidence: CoincidenceMeasure::Unreadable(diag),
                ..
            }) => assert_eq!(diag.predicate, Some("ring_torus_convention")),
            other => panic!("the spindle: {other:?}"),
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
    /// answers structurally, in both directions.
    #[test]
    fn torus_against_cylinder_is_a_kind_mismatch() {
        let t = torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 0.06, true);
        let c = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, false);
        assert_eq!(
            carrier_eq(&t, &c, &at(1.0), band()).unwrap(),
            CarrierRelation::Distinct
        );
        assert_eq!(
            carrier_eq(&c, &t, &at(1.0), band()).unwrap(),
            CarrierRelation::Distinct
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
