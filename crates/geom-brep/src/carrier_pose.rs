//! **The pose two face carriers stand in, and which way their materials
//! face** ([`carrier_pose()`]): two planes decided parallel, two round
//! carriers of one kind apart or nested (two cylinders' axes decided
//! parallel), or a plane and a round carrier with the centre or axis on
//! one side of it (a cylinder's axis decided parallel to the plane).
//!
//! It is one reading of the pair, and both of its answers come off it:
//! the configuration a signed gap is read in, and the facing
//! ([`CarrierPose::facing`]), whether each face's material faces the
//! other's. A reader that needs the facing of two carriers asks it of the
//! pose that decided them parallel, never of a point of one face: two
//! parallel rods side by side face each other at the bearing between
//! them and nowhere else, and the pose says so once for the pair.

use geom_core::{Band, Decide, Indeterminate, Lever, Margin, Point3, Real, Sign, Vec3};

use crate::PairSide;
use crate::dihedral::decide;

/// A face's carrier as the pose reads it: its kind's datum and the side
/// its material is on.
#[derive(Clone, Copy, Debug)]
pub enum Posed<T: Real> {
    /// A plane, by a point on it and the face's OUTWARD unit normal.
    Plane {
        /// A point on the plane.
        origin: Point3<T>,
        /// The outward unit normal, the face's sense folded in.
        outward: Vec3<T>,
    },
    /// A sphere, by its centre and radius.
    Sphere {
        /// The centre.
        center: Point3<T>,
        /// The radius.
        radius: T,
        /// The material is inside: a ball, not a socket.
        convex: bool,
    },
    /// A cylinder, by its axis and radius.
    Cylinder {
        /// The axis, its direction unit.
        axis: geom::Line<T>,
        /// The radius.
        radius: T,
        /// The material is inside: a pin, not a bore.
        convex: bool,
    },
}

/// Which way two faces' materials face each other in their pose.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Facing {
    /// Each face faces the other's material: two plates face to face, a
    /// pin in a bore, two rods side by side.
    Opposed,
    /// Both face one way: a flush pair, two rods nested, a rod in a
    /// bore's wall.
    Aligned,
}

/// How two round carriers of one kind lie, by their centres or axes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nest {
    /// The smaller one's centre or axis outside the larger.
    Apart,
    /// Nested, the second inside the first.
    SecondInside,
    /// Nested, the first inside the second.
    FirstInside,
}

/// **The pose two carriers were decided in** ([`carrier_pose()`]), with
/// what each configuration's gap is read from.
#[derive(Clone, Copy, Debug)]
pub enum CarrierPose<T: Real> {
    /// Two planes decided parallel: their outward normals, in the order
    /// given, and the arm the parallelism was decided over.
    Planes {
        /// The two outward normals.
        outward: [Vec3<T>; 2],
        /// The arm.
        arm: Lever<T>,
    },
    /// Two spheres, or two cylinders whose axes are decided parallel, `d`
    /// apart at their centres or axes.
    Rounds {
        /// The centres' or the axes' separation.
        d: T,
        /// How they lie.
        nest: Nest,
        /// Each one's `convex` bit, in the order given.
        convex: [bool; 2],
    },
    /// A plane and a sphere, or a cylinder whose axis is decided parallel
    /// to it: the centre or axis at `h` along the plane's outward normal,
    /// decided on its outward side where `beyond`.
    PlaneRound {
        /// Which of the two given is the plane.
        plane: PairSide,
        /// The centre's or the axis's offset along the outward normal.
        h: T,
        /// `h` decided positive.
        beyond: bool,
        /// The round face's `convex` bit.
        convex: bool,
    },
}

/// Why two carriers have no pose here.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CarrierPoseRefusal {
    /// A pair of kinds the pose has no reading for: a sphere and a
    /// cylinder.
    Unpaired,
    /// The carriers are decidedly not parallel, as the named decision
    /// read them.
    NotParallel(&'static str),
    /// A round carrier's centre or axis lies on the other carrier, or on
    /// the larger of two: no side to read the configuration by.
    OnTheCarrier,
    /// A decision of the pose escalated.
    Escalated(Indeterminate),
}

/// The K name of the parallelism of two planes' normals.
pub const POSE_PLANES_PARALLEL: &str = "bool_plane_parallel";

/// The K name of the parallelism of two cylinders' axes.
pub const POSE_AXES_PARALLEL: &str = "carrier_cyl_axis_parallel";

/// The K name of which side of a carrier a round face's centre or axis
/// lies: of a plane, along its outward normal; of the larger of two round
/// carriers, outside or inside it.
pub const POSE_ROUND_SIDE: &str = "gap_round_side";

/// The K name of which of two nested round carriers is the larger.
pub const POSE_ROUND_LARGER: &str = "gap_round_larger";

/// The K name of a cylinder's axis lying parallel to a plane.
pub const POSE_AXIS_PARALLEL_TO_PLANE: &str = "gap_axis_parallel_to_plane";

/// The K name of which way two parallel planes' outward normals point.
pub const POSE_PLANES_FACING: &str = "carrier_pose_planes_facing";

/// **`carrier_pose`** — the pose `first` and `second` stand in: two
/// planes whose normals are parallel over `arm` ([`POSE_PLANES_PARALLEL`]);
/// two spheres, or two cylinders whose axes are parallel over `arm`
/// ([`POSE_AXES_PARALLEL`]), apart or nested ([`POSE_ROUND_SIDE`], at
/// margin `d − r_large`, and of two nested, [`POSE_ROUND_LARGER`]; of one
/// radius, the convex face is the one inside, and of two of one sense the
/// second); a plane and a sphere, or a cylinder whose axis is parallel to
/// it over `arm` ([`POSE_AXIS_PARALLEL_TO_PLANE`]), with the centre or
/// axis on a side of it ([`POSE_ROUND_SIDE`], at margin `h`). `arm` is an
/// upper bound on the extent of the two faces together: a parallelism is
/// consumed as a separation constant across them.
///
/// # Errors
///
/// [`CarrierPoseRefusal`]: a sphere and a cylinder, carriers decidedly not
/// parallel, a centre or an axis on the other carrier, or an escalation.
pub fn carrier_pose<T: Decide>(
    first: &Posed<T>,
    second: &Posed<T>,
    arm: Lever<T>,
    band: Band,
) -> Result<CarrierPose<T>, CarrierPoseRefusal> {
    use Posed as P;
    match (*first, *second) {
        (P::Plane { outward: a, .. }, P::Plane { outward: b, .. }) => {
            parallel_as(POSE_PLANES_PARALLEL, a, b, arm, band)?;
            Ok(CarrierPose::Planes {
                outward: [a, b],
                arm,
            })
        }
        (
            P::Sphere {
                center: c1,
                radius: r1,
                convex: x1,
            },
            P::Sphere {
                center: c2,
                radius: r2,
                convex: x2,
            },
        ) => rounds((c2 - c1).norm(), [r1, r2], [x1, x2], band),
        (
            P::Cylinder {
                axis: l1,
                radius: r1,
                convex: x1,
            },
            P::Cylinder {
                axis: l2,
                radius: r2,
                convex: x2,
            },
        ) => {
            parallel_as(POSE_AXES_PARALLEL, l1.dir, l2.dir, arm, band)?;
            rounds(l1.distance(l2.origin), [r1, r2], [x1, x2], band)
        }
        (P::Plane { origin, outward }, round @ (P::Sphere { .. } | P::Cylinder { .. })) => {
            plane_round(PairSide::First, origin, outward, &round, arm, band)
        }
        (round @ (P::Sphere { .. } | P::Cylinder { .. }), P::Plane { origin, outward }) => {
            plane_round(PairSide::Second, origin, outward, &round, arm, band)
        }
        (P::Sphere { .. }, P::Cylinder { .. }) | (P::Cylinder { .. }, P::Sphere { .. }) => {
            Err(CarrierPoseRefusal::Unpaired)
        }
    }
}

impl<T: Decide> CarrierPose<T> {
    /// **Which way the two faces' materials face each other** in the pose.
    /// Two round carriers apart face each other where both are convex or
    /// both hollow, and nested where one is each; a plane and a round face
    /// where the centre or axis lies on the outward side of the plane of
    /// a convex face, or behind it of a hollow one. Two parallel planes
    /// face each other where their outward normals point opposite ways,
    /// read as the sign of their cosine over the arm their parallelism
    /// was decided on ([`POSE_PLANES_FACING`]).
    ///
    /// # Errors
    ///
    /// Two planes' cosine in band, poisoned or decided zero over an arm
    /// too short to read a side over.
    pub fn facing(&self, band: Band) -> Result<Facing, Indeterminate> {
        Ok(match *self {
            Self::Planes {
                outward: [a, b],
                arm,
            } => {
                match crate::dihedral::pairing_of_normals(POSE_PLANES_FACING, a, b, arm, band).0? {
                    crate::MaterialPairing::Aligned => Facing::Aligned,
                    crate::MaterialPairing::Opposed => Facing::Opposed,
                }
            }
            Self::Rounds {
                nest,
                convex: [a, b],
                ..
            } => {
                if (nest == Nest::Apart) == (a == b) {
                    Facing::Opposed
                } else {
                    Facing::Aligned
                }
            }
            Self::PlaneRound { beyond, convex, .. } => {
                if beyond == convex {
                    Facing::Opposed
                } else {
                    Facing::Aligned
                }
            }
        })
    }
}

/// Two directions parallel over `arm`, decided as `name`: the sine
/// between them levered at the arm, decided zero.
fn parallel_as<T: Decide>(
    name: &'static str,
    a: Vec3<T>,
    b: Vec3<T>,
    arm: Lever<T>,
    band: Band,
) -> Result<(), CarrierPoseRefusal> {
    match decide(name, Margin::levered(a.cross(b).norm(), arm), band) {
        Ok(Sign::Zero) => Ok(()),
        Ok(Sign::Positive | Sign::Negative) => Err(CarrierPoseRefusal::NotParallel(name)),
        Err(source) => Err(CarrierPoseRefusal::Escalated(source)),
    }
}

/// Whether a centre or an axis at signed offset `margin` from a carrier
/// lies beyond it ([`POSE_ROUND_SIDE`]): on it is no side.
fn beyond<T: Decide>(margin: T, band: Band) -> Result<bool, CarrierPoseRefusal> {
    match decide(POSE_ROUND_SIDE, Margin::of(margin), band) {
        Ok(Sign::Positive) => Ok(true),
        Ok(Sign::Negative) => Ok(false),
        Ok(Sign::Zero) => Err(CarrierPoseRefusal::OnTheCarrier),
        Err(source) => Err(CarrierPoseRefusal::Escalated(source)),
    }
}

/// Two round carriers `d` apart: apart where the smaller one's centre or
/// axis lies outside the larger, else nested, the smaller inside.
fn rounds<T: Decide>(
    d: T,
    [r1, r2]: [T; 2],
    [x1, x2]: [bool; 2],
    band: Band,
) -> Result<CarrierPose<T>, CarrierPoseRefusal> {
    let nest = if beyond(d - r1.max(r2), band)? {
        Nest::Apart
    } else {
        let second_inside = match decide(POSE_ROUND_LARGER, Margin::of(r1 - r2), band) {
            Ok(Sign::Positive) => true,
            Ok(Sign::Negative) => false,
            Ok(Sign::Zero) => x2 || !x1,
            Err(source) => return Err(CarrierPoseRefusal::Escalated(source)),
        };
        if second_inside {
            Nest::SecondInside
        } else {
            Nest::FirstInside
        }
    };
    Ok(CarrierPose::Rounds {
        d,
        nest,
        convex: [x1, x2],
    })
}

/// A plane and a round carrier: a cylinder's axis parallel to the plane,
/// and the centre or axis on a side of it.
fn plane_round<T: Decide>(
    plane: PairSide,
    origin: Point3<T>,
    outward: Vec3<T>,
    round: &Posed<T>,
    arm: Lever<T>,
    band: Band,
) -> Result<CarrierPose<T>, CarrierPoseRefusal> {
    let (centre, convex) = match *round {
        Posed::Sphere { center, convex, .. } => (center, convex),
        Posed::Cylinder { axis, convex, .. } => {
            match decide(
                POSE_AXIS_PARALLEL_TO_PLANE,
                Margin::levered(axis.dir.dot(outward), arm),
                band,
            ) {
                Ok(Sign::Zero) => {}
                Ok(_) => return Err(CarrierPoseRefusal::NotParallel(POSE_AXIS_PARALLEL_TO_PLANE)),
                Err(source) => return Err(CarrierPoseRefusal::Escalated(source)),
            }
            (axis.origin, convex)
        }
        Posed::Plane { .. } => unreachable!("carrier_pose matched a round carrier"),
    };
    let h = outward.dot(centre - origin);
    Ok(CarrierPose::PlaneRound {
        plane,
        h,
        beyond: beyond(h, band)?,
        convex,
    })
}
