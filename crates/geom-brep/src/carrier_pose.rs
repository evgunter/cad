//! **The pose two face carriers stand in, and which way their materials
//! face** ([`carrier_pose()`]): two planes decided parallel, two round
//! carriers of one kind apart or nested (two cylinders' axes decided
//! parallel), or a plane and a round carrier with the centre or axis on
//! one side of it (a cylinder's axis decided parallel to the plane).
//!
//! It is one reading of the pair, and both of its answers come off it:
//! the configuration a signed gap is read in, carried whole by each arm,
//! and the facing, whether each face's material faces the other's. A
//! reader that needs the facing of two carriers asks it of the pose that
//! decided them parallel, never of a point of one face. Of a round pair
//! and of a plane and a round face the facing follows from the
//! configuration with no further decision; of two planes it is their
//! outward normals' pairing ([`crate::dihedral::normals_alignment`]).
//!
//! Every decision here is the pose's own, under its own K name. The
//! parallelism of two directions ([`directions_parallel`]) is also what a
//! measure of two planes' or two axes' distance reads.

use geom_core::{Band, Decide, Indeterminate, Lever, Margin, Point3, Real, Sign, Vec3};

use crate::dihedral::decide;
use crate::{MaterialPairing, OutwardNormal, PairSide};

/// A face's carrier as the pose reads it: its kind's datum and the side
/// its material is on.
#[derive(Clone, Copy, Debug)]
pub enum Posed<T: Real> {
    /// A plane, by a point on it and the face's outward normal.
    Plane {
        /// A point on the plane.
        origin: Point3<T>,
        /// The outward unit normal, the face's sense folded in.
        outward: OutwardNormal<T>,
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

/// **Two planes decided parallel**, in the order given.
#[derive(Clone, Copy, Debug)]
pub struct PlanePair<T: Real> {
    /// A point on each plane.
    pub origin: [Point3<T>; 2],
    /// Each face's outward normal.
    pub outward: [OutwardNormal<T>; 2],
    /// The arm the parallelism was decided over.
    pub arm: Lever<T>,
}

impl<T: Decide> PlanePair<T> {
    /// **Which way the two faces' materials face**: the pairing of their
    /// outward normals over the arm their parallelism was decided on
    /// ([`POSE_PLANES_FACING`]), the one decision a pose's facing can
    /// refuse.
    ///
    /// # Errors
    ///
    /// As [`crate::AlignmentReading::pairing`]: the cosine in band,
    /// poisoned, or decided zero.
    pub fn facing(&self, band: Band) -> Result<MaterialPairing, Indeterminate> {
        let [a, b] = self.outward;
        crate::dihedral::normals_alignment(POSE_PLANES_FACING, a, b, self.arm, band)?.pairing()
    }
}

/// **Two round carriers of one kind**: two spheres, or two cylinders whose
/// axes are decided parallel, in the order given.
#[derive(Clone, Copy, Debug)]
pub struct RoundPair<T: Real> {
    /// Their centres' or axes' separation.
    pub d: T,
    /// How they lie.
    pub nest: Nest,
    /// Each one's radius.
    pub radius: [T; 2],
    /// Each one's `convex` bit.
    pub convex: [bool; 2],
}

impl<T: Real> RoundPair<T> {
    /// **Which way the two faces' materials face**: apart, they face each
    /// other where both are convex or both hollow; nested, where one is
    /// each.
    #[must_use]
    pub fn facing(&self) -> MaterialPairing {
        let [a, b] = self.convex;
        if (self.nest == Nest::Apart) == (a == b) {
            MaterialPairing::Opposed
        } else {
            MaterialPairing::Aligned
        }
    }
}

/// **A plane and a round carrier**: a sphere, or a cylinder whose axis is
/// decided parallel to the plane.
#[derive(Clone, Copy, Debug)]
pub struct PlaneRound<T: Real> {
    /// Which of the two given is the plane.
    pub plane: PairSide,
    /// The centre's or the axis's offset along the plane's outward
    /// normal.
    pub h: T,
    /// `h` decided positive.
    pub beyond: bool,
    /// The round carrier's radius.
    pub radius: T,
    /// The round face's `convex` bit.
    pub convex: bool,
}

impl<T: Real> PlaneRound<T> {
    /// **Which way the two faces' materials face**: each other where the
    /// centre or axis lies beyond the plane of a convex face, or behind it
    /// of a hollow one.
    #[must_use]
    pub fn facing(&self) -> MaterialPairing {
        if self.beyond == self.convex {
            MaterialPairing::Opposed
        } else {
            MaterialPairing::Aligned
        }
    }
}

/// **The pose two carriers were decided in** ([`carrier_pose()`]), each
/// arm carrying what its gap and its facing are read from.
#[derive(Clone, Copy, Debug)]
pub enum CarrierPose<T: Real> {
    /// Two planes decided parallel.
    Planes(PlanePair<T>),
    /// Two round carriers of one kind.
    Rounds(RoundPair<T>),
    /// A plane and a round carrier.
    PlaneRound(PlaneRound<T>),
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

/// The K name of two planes' normals decided parallel.
pub const POSE_PLANES_PARALLEL: &str = "carrier_pose_planes_parallel";

/// The K name of two axes decided parallel.
pub const POSE_AXES_PARALLEL: &str = "carrier_pose_axes_parallel";

/// The K name of which side of a carrier a round face's centre or axis
/// lies: of a plane, along its outward normal; of the larger of two round
/// carriers, outside or inside it.
pub const POSE_ROUND_SIDE: &str = "carrier_pose_round_side";

/// The K name of which of two nested round carriers is the larger.
pub const POSE_ROUND_LARGER: &str = "carrier_pose_round_larger";

/// The K name of a cylinder's axis lying parallel to a plane.
pub const POSE_AXIS_ALONG_PLANE: &str = "carrier_pose_axis_along_plane";

/// The K name of which way two parallel planes' outward normals point.
pub const POSE_PLANES_FACING: &str = "carrier_pose_planes_facing";

/// **What each of the pose's decisions asks**, as a clause with no colon
/// or dash of its own, for a refusal that quotes one; `None` for a name
/// the pose does not decide.
#[must_use]
pub fn decision_words(predicate: &str) -> Option<&'static str> {
    Some(match predicate {
        POSE_PLANES_PARALLEL => "whether the two planes are parallel",
        POSE_AXES_PARALLEL => "whether the two axes are parallel",
        POSE_ROUND_SIDE => {
            "which side of the other face's carrier a round face's centre or axis lies"
        }
        POSE_ROUND_LARGER => "which of two nested round faces is the larger",
        POSE_AXIS_ALONG_PLANE => "whether a cylinder's axis runs parallel to a plane",
        POSE_PLANES_FACING => "whether two parallel planes face each other or one way",
        _ => return None,
    })
}

/// What two directions are, for [`directions_parallel`]: its K name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Directions {
    /// Two planes' normals ([`POSE_PLANES_PARALLEL`]).
    PlaneNormals,
    /// Two axes ([`POSE_AXES_PARALLEL`]).
    Axes,
}

impl Directions {
    /// The K name the parallelism is decided under.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::PlaneNormals => POSE_PLANES_PARALLEL,
            Self::Axes => POSE_AXES_PARALLEL,
        }
    }
}

/// **Whether two unit directions are parallel over `arm`**: the sine
/// between them levered at the arm, decided zero, under the K name of
/// what they are. `arm` is an upper bound on the extent of the two faces
/// together, so a parallelism is consumed as a separation constant across
/// them. The one reading of parallelism the pose and the measures of a
/// distance between planes or axes share.
///
/// # Errors
///
/// The decision's [`Indeterminate`]: in band or poisoned.
pub fn directions_parallel<T: Decide>(
    what: Directions,
    a: Vec3<T>,
    b: Vec3<T>,
    arm: Lever<T>,
    band: Band,
) -> Result<bool, Indeterminate> {
    Ok(decide(what.name(), Margin::levered(a.cross(b).norm(), arm), band)? == Sign::Zero)
}

/// **`carrier_pose`** — the pose `first` and `second` stand in: two
/// planes whose normals are parallel over `arm`; two spheres, or two
/// cylinders whose axes are parallel over `arm`, apart or nested
/// ([`POSE_ROUND_SIDE`], at margin `d − r_large`, and of two nested,
/// [`POSE_ROUND_LARGER`]; of one radius, the convex face is the one
/// inside, and of two of one sense the second); a plane and a sphere, or
/// a cylinder whose axis is parallel to it over `arm`
/// ([`POSE_AXIS_ALONG_PLANE`]), with the centre or axis on a side of it
/// ([`POSE_ROUND_SIDE`], at margin `h`).
///
/// # Errors
///
/// [`CarrierPoseRefusal`]: a sphere and a cylinder, carriers decidedly
/// not parallel, a centre or an axis on the other carrier, or an
/// escalation.
pub fn carrier_pose<T: Decide>(
    first: &Posed<T>,
    second: &Posed<T>,
    arm: Lever<T>,
    band: Band,
) -> Result<CarrierPose<T>, CarrierPoseRefusal> {
    use Posed as P;
    match (*first, *second) {
        (
            P::Plane {
                origin: o1,
                outward: a,
            },
            P::Plane {
                origin: o2,
                outward: b,
            },
        ) => {
            parallel(Directions::PlaneNormals, a.vec(), b.vec(), arm, band)?;
            Ok(CarrierPose::Planes(PlanePair {
                origin: [o1, o2],
                outward: [a, b],
                arm,
            }))
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
            parallel(Directions::Axes, l1.dir, l2.dir, arm, band)?;
            rounds(l1.distance(l2.origin), [r1, r2], [x1, x2], band)
        }
        (P::Sphere { .. }, P::Cylinder { .. }) | (P::Cylinder { .. }, P::Sphere { .. }) => {
            Err(CarrierPoseRefusal::Unpaired)
        }
        (
            P::Plane { origin, outward },
            P::Sphere {
                center,
                radius,
                convex,
            },
        ) => plane_round(
            PairSide::First,
            (origin, outward.vec()),
            (center, radius, convex, None),
            arm,
            band,
        ),
        (
            P::Sphere {
                center,
                radius,
                convex,
            },
            P::Plane { origin, outward },
        ) => plane_round(
            PairSide::Second,
            (origin, outward.vec()),
            (center, radius, convex, None),
            arm,
            band,
        ),
        (
            P::Plane { origin, outward },
            P::Cylinder {
                axis,
                radius,
                convex,
            },
        ) => plane_round(
            PairSide::First,
            (origin, outward.vec()),
            (axis.origin, radius, convex, Some(axis.dir)),
            arm,
            band,
        ),
        (
            P::Cylinder {
                axis,
                radius,
                convex,
            },
            P::Plane { origin, outward },
        ) => plane_round(
            PairSide::Second,
            (origin, outward.vec()),
            (axis.origin, radius, convex, Some(axis.dir)),
            arm,
            band,
        ),
    }
}

/// [`directions_parallel`] as the pose consumes it: a definite "not
/// parallel" is the pose's refusal.
fn parallel<T: Decide>(
    what: Directions,
    a: Vec3<T>,
    b: Vec3<T>,
    arm: Lever<T>,
    band: Band,
) -> Result<(), CarrierPoseRefusal> {
    match directions_parallel(what, a, b, arm, band) {
        Ok(true) => Ok(()),
        Ok(false) => Err(CarrierPoseRefusal::NotParallel(what.name())),
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
    Ok(CarrierPose::Rounds(RoundPair {
        d,
        nest,
        radius: [r1, r2],
        convex: [x1, x2],
    }))
}

/// A plane and a round carrier, its centre or axis point, radius, side
/// and (a cylinder's) axis direction: the axis parallel to the plane,
/// and the centre or axis on a side of it.
fn plane_round<T: Decide>(
    plane: PairSide,
    (origin, outward): (Point3<T>, Vec3<T>),
    (centre, radius, convex, axis): (Point3<T>, T, bool, Option<Vec3<T>>),
    arm: Lever<T>,
    band: Band,
) -> Result<CarrierPose<T>, CarrierPoseRefusal> {
    if let Some(dir) = axis {
        match decide(
            POSE_AXIS_ALONG_PLANE,
            Margin::levered(dir.dot(outward), arm),
            band,
        ) {
            Ok(Sign::Zero) => {}
            Ok(_) => return Err(CarrierPoseRefusal::NotParallel(POSE_AXIS_ALONG_PLANE)),
            Err(source) => return Err(CarrierPoseRefusal::Escalated(source)),
        }
    }
    let h = outward.dot(centre - origin);
    Ok(CarrierPose::PlaneRound(PlaneRound {
        plane,
        h,
        beyond: beyond(h, band)?,
        radius,
        convex,
    }))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::test_support::fixture_lever;
    use geom_core::{Band, Point3, Tol, Vec3};

    use super::*;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
        Point3::new(x, y, z)
    }

    fn plane(origin: Point3<f64>, n: Vec3<f64>) -> Posed<f64> {
        Posed::Plane {
            origin,
            outward: OutwardNormal::from_chart(n, true),
        }
    }

    fn ball(center: Point3<f64>, radius: f64, convex: bool) -> Posed<f64> {
        Posed::Sphere {
            center,
            radius,
            convex,
        }
    }

    fn rod(origin: Point3<f64>, dir: Vec3<f64>, radius: f64, convex: bool) -> Posed<f64> {
        Posed::Cylinder {
            axis: geom::Line {
                origin,
                dir: dir.normalize(),
            },
            radius,
            convex,
        }
    }

    fn pose(a: &Posed<f64>, b: &Posed<f64>) -> Result<CarrierPose<f64>, CarrierPoseRefusal> {
        carrier_pose(a, b, fixture_lever(2.0), band())
    }

    /// A tilt that puts a direction past the band over the arm of 2: three
    /// times the band's escalation, per metre of arm.
    fn tilt() -> f64 {
        3.0 * band().escalate() / 2.0
    }

    /// **Two planes have a pose only parallel, and their facing is their
    /// outward normals' pairing.** Face to face, opposed; one over the
    /// other facing up, aligned, the gap's origins and normals carried in
    /// the order given; tilted past the band over the arm, no pose, the
    /// refusal naming the planes' parallelism. Red if a pair of planes not
    /// parallel reads a pose, or a facing reads the wrong side.
    #[test]
    fn two_planes_have_a_pose_only_parallel() {
        let z = Vec3::new(0.0, 0.0, 1.0);
        let floor = plane(p(0.0, 0.0, 0.0), z);
        for (what, other, want) in [
            (
                "face to face",
                plane(p(0.0, 0.0, 0.1), -z),
                MaterialPairing::Opposed,
            ),
            (
                "flush",
                plane(p(0.0, 0.0, 0.1), z),
                MaterialPairing::Aligned,
            ),
        ] {
            let Ok(CarrierPose::Planes(planes)) = pose(&floor, &other) else {
                panic!("{what}: two parallel planes");
            };
            assert_eq!(
                (planes.origin[1] - p(0.0, 0.0, 0.0)).z,
                0.1,
                "{what}: the origins in order"
            );
            assert_eq!(planes.facing(band()), Ok(want), "{what}");
        }
        let tilted = plane(p(0.0, 0.0, 0.1), Vec3::new(tilt(), 0.0, 1.0).normalize());
        assert_eq!(
            pose(&floor, &tilted).map(|_| ()),
            Err(CarrierPoseRefusal::NotParallel(POSE_PLANES_PARALLEL)),
            "tilted past the band"
        );
    }

    /// **Two round carriers apart or nested, of one radius by their
    /// sides, and a centre on the other carrier refuses.** Two balls apart;
    /// one inside the other each way round; of one radius and one centre,
    /// the convex face is the one inside and of two of one sense the
    /// second (`x₂ ∨ ¬x₁` for the second inside); a centre on the larger
    /// sphere has no side. Two rods with parallel axes read as two balls
    /// do, and skew past the band have no pose. The facing: apart, opposed
    /// where both are convex or both hollow; nested, where one is each.
    /// Red if the tie reads the hollow face inside a convex one, the
    /// facing turns, or skew rods read a pose.
    #[test]
    fn two_round_carriers_lie_apart_or_nested() {
        use MaterialPairing::{Aligned, Opposed};
        use Nest::{Apart, FirstInside, SecondInside};
        let o = p(0.0, 0.0, 0.0);
        for (what, a, b, nest, facing) in [
            (
                "two balls apart",
                ball(o, 0.5, true),
                ball(p(2.0, 0.0, 0.0), 0.3, true),
                Apart,
                Opposed,
            ),
            (
                "a ball apart from a socket",
                ball(o, 0.5, false),
                ball(p(2.0, 0.0, 0.0), 0.3, true),
                Apart,
                Aligned,
            ),
            (
                "a ball in a socket",
                ball(o, 1.0, false),
                ball(p(0.2, 0.0, 0.0), 0.3, true),
                SecondInside,
                Opposed,
            ),
            (
                "a socket round a ball",
                ball(p(0.2, 0.0, 0.0), 0.3, true),
                ball(o, 1.0, false),
                FirstInside,
                Opposed,
            ),
            (
                "a ball in a ball",
                ball(o, 1.0, true),
                ball(p(0.2, 0.0, 0.0), 0.3, true),
                SecondInside,
                Aligned,
            ),
            (
                "one radius, both convex",
                ball(o, 1.0, true),
                ball(o, 1.0, true),
                SecondInside,
                Aligned,
            ),
            (
                "one radius, both hollow",
                ball(o, 1.0, false),
                ball(o, 1.0, false),
                SecondInside,
                Aligned,
            ),
            (
                "one radius, the convex second",
                ball(o, 1.0, false),
                ball(o, 1.0, true),
                SecondInside,
                Opposed,
            ),
            (
                "one radius, the convex first",
                ball(o, 1.0, true),
                ball(o, 1.0, false),
                FirstInside,
                Opposed,
            ),
        ] {
            let Ok(CarrierPose::Rounds(rounds)) = pose(&a, &b) else {
                panic!("{what}: a round pair");
            };
            assert_eq!((rounds.nest, rounds.facing()), (nest, facing), "{what}");
        }
        assert_eq!(
            pose(&ball(o, 1.0, true), &ball(p(1.0, 0.0, 0.0), 0.3, true)).map(|_| ()),
            Err(CarrierPoseRefusal::OnTheCarrier),
            "a centre on the larger sphere"
        );
        let z = Vec3::new(0.0, 0.0, 1.0);
        let Ok(CarrierPose::Rounds(rods)) =
            pose(&rod(o, z, 0.5, false), &rod(p(0.1, 0.0, 0.0), z, 0.3, true))
        else {
            panic!("a pin in a bore: a round pair");
        };
        assert_eq!(
            (rods.nest, rods.facing(), rods.radius),
            (SecondInside, Opposed, [0.5, 0.3]),
            "a pin in a bore"
        );
        assert_eq!(
            pose(
                &rod(o, z, 0.5, true),
                &rod(p(2.0, 0.0, 0.0), Vec3::new(tilt(), 0.0, 1.0), 0.3, true)
            )
            .map(|_| ()),
            Err(CarrierPoseRefusal::NotParallel(POSE_AXES_PARALLEL)),
            "skew rods"
        );
    }

    /// **A plane and a round face, in either seat, and a cylinder only
    /// along the plane.** A ball over a plate, opposed, with the plate's
    /// seat carried; a socket centred over a plate, aligned; a rod along a
    /// plate, opposed; a rod tilted off the plate's plane past the band, no
    /// pose; a ball centred on the plate, no side; a ball and a rod, no
    /// pose. Red if a seat or a side turns, or a tilted rod reads a pose.
    #[test]
    fn a_plane_and_a_round_face_in_either_seat() {
        use MaterialPairing::{Aligned, Opposed};
        let z = Vec3::new(0.0, 0.0, 1.0);
        let x = Vec3::new(1.0, 0.0, 0.0);
        let plate = plane(p(0.0, 0.0, 0.0), z);
        for (what, round, facing) in [
            (
                "a ball over a plate",
                ball(p(0.0, 0.0, 1.0), 0.5, true),
                Opposed,
            ),
            (
                "a socket over a plate",
                ball(p(0.0, 0.0, 1.0), 0.5, false),
                Aligned,
            ),
            (
                "a rod along a plate",
                rod(p(0.0, 0.0, 1.0), x, 0.5, true),
                Opposed,
            ),
        ] {
            for (seat, a, b) in [
                (PairSide::First, &plate, &round),
                (PairSide::Second, &round, &plate),
            ] {
                let Ok(CarrierPose::PlaneRound(pr)) = pose(a, b) else {
                    panic!("{what}, {seat:?}: a plane and a round face");
                };
                assert_eq!(
                    (pr.plane, pr.beyond, pr.radius, pr.facing()),
                    (seat, true, 0.5, facing),
                    "{what}, {seat:?}"
                );
            }
        }
        assert_eq!(
            pose(
                &plate,
                &rod(p(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, tilt()), 0.5, true)
            )
            .map(|_| ()),
            Err(CarrierPoseRefusal::NotParallel(POSE_AXIS_ALONG_PLANE)),
            "a rod tilted off the plate"
        );
        assert_eq!(
            pose(&plate, &ball(p(0.0, 0.0, 0.0), 0.5, true)).map(|_| ()),
            Err(CarrierPoseRefusal::OnTheCarrier),
            "a ball centred on the plate"
        );
        assert_eq!(
            pose(
                &ball(p(0.0, 0.0, 0.0), 0.5, true),
                &rod(p(3.0, 0.0, 0.0), z, 0.5, true)
            )
            .map(|_| ()),
            Err(CarrierPoseRefusal::Unpaired),
            "a ball and a rod"
        );
    }
}
