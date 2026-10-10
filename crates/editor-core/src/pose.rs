//! **A pose value** (D10, Variables): the evaluated value of a pose
//! variable — a frame known up to its kind's symmetry, as geometry
//! VALUES, never kernel entities or recipe references.
//!
//! Every pose a reader reads reaches it here: an operation's pose
//! output ([`crate::VarDef::Output`]: a datum's, a revolve's axis) or a
//! pose definition ([`crate::VarDef::Pose`]) bound at its reader by
//! `eval_pose` (`eval/pose.rs`). Its normals and axis directions are
//! [`UnitVec3`], unit by construction, so an unnormalized pose has no
//! spelling.
//!
//! Each value carries its kind's symmetry ([`PoseSymmetry`]): the
//! [`Subgroup`] the mates fold is the one a pose is known up to, read
//! here once ([`crate::VarKind::symmetry`] names its family).

use geom_core::linalg::{OrthoFrame, Point3, UnitVec3};
use geom_core::predicate::{Band, Indeterminate, Margin, Sign};
use geom_core::{Decide, Real};

use crate::mate::{PoseSymmetry, Subgroup};

/// **The funnel site name** of the decided position predicate a
/// selector measures against a pose (SELECT-DESIGN §1's `sel_*`
/// prefix). Its comparand is a genuine length (the distance minus the
/// stated value), so it goes through the plain [`Margin::of`] door.
///
/// A K row name reaching the funnel through a const, so it is a roster
/// carrier (`docs/K-REPORT.md`, "The inventory method, restated").
pub const SEL_DATUM_DISTANCE: &str = "sel_datum_distance";

/// **An evaluated pose**, one arm per pose kind (D10's five).
#[derive(Debug, Clone)]
pub enum PoseValue<T: Real> {
    /// A point.
    Point {
        /// Its position.
        position: Point3<T>,
    },
    /// A direction: known up to translation and spin about itself.
    Direction {
        /// The direction.
        dir: UnitVec3<T>,
    },
    /// An axis through `origin` along `dir`: known up to slide and spin
    /// along itself.
    Axis {
        /// A point on the axis.
        origin: Point3<T>,
        /// The direction.
        dir: UnitVec3<T>,
    },
    /// A plane through `origin` with `normal`: known up to in-plane
    /// motion.
    Plane {
        /// A point on the plane.
        origin: Point3<T>,
        /// The normal.
        normal: UnitVec3<T>,
    },
    /// **A frame**: origin plus a right-handed orthonormal pair of
    /// in-plane directions, so the surface and the spin about its
    /// normal are pinned. The payload is the frame witness: `u` and `v`
    /// are unit and orthogonal as a property of the type, and
    /// `w = u × v` is the normal.
    Frame(OrthoFrame<T>),
}

impl<T: Real> PoseValue<T> {
    /// The kind this value is of.
    #[must_use]
    pub fn kind(&self) -> crate::VarKind {
        match self {
            Self::Point { .. } => crate::VarKind::Point,
            Self::Direction { .. } => crate::VarKind::Direction,
            Self::Axis { .. } => crate::VarKind::Axis,
            Self::Plane { .. } => crate::VarKind::Plane,
            Self::Frame(_) => crate::VarKind::Frame,
        }
    }
}

/// A pose is known up to its kind's subgroup: a point up to rotation
/// about it, a direction up to translation and spin about it, an axis
/// up to slide and spin along it, a plane up to in-plane motion, a
/// frame up to nothing.
impl<T: Real> PoseSymmetry<T> for PoseValue<T> {
    fn symmetry(&self) -> Subgroup<T> {
        match *self {
            Self::Point { position } => Subgroup::Spherical { point: position },
            Self::Direction { dir } => Subgroup::Parallel { direction: dir },
            Self::Axis { origin, dir } => Subgroup::Cylindrical {
                point: origin,
                direction: dir,
            },
            Self::Plane { normal, .. } => Subgroup::Planar { normal },
            Self::Frame(_) => Subgroup::Trivial,
        }
    }
}

/// The distance of `p` from a pose: SIGNED along a plane's or a
/// frame's normal (unit by construction, so the dot product is already
/// a length), UNSIGNED to an axis or a point. A frame answers as the
/// plane it lies in. `None` for a direction, which has no position.
///
/// Arithmetic only: deciding what the distance means is
/// [`datum_distance_sign`]'s.
#[must_use]
pub fn datum_distance<T: Real>(pose: &PoseValue<T>, p: Point3<T>) -> Option<T> {
    Some(match pose {
        PoseValue::Plane { origin, normal } => (p - *origin).dot(normal.get()),
        PoseValue::Axis { origin, dir } => {
            let d = dir.get();
            let v = p - *origin;
            (v - d * v.dot(d)).norm()
        }
        PoseValue::Point { position } => (p - *position).norm(),
        PoseValue::Frame(f) => (p - f.origin()).dot(f.w().get()),
        PoseValue::Direction { .. } => return None,
    })
}

/// DECIDED: which side of the stated `value` the point's
/// [`datum_distance`] lands on, through the [`SEL_DATUM_DISTANCE`]
/// funnel. `None` for a direction.
///
/// # Errors
///
/// The funnel's [`Indeterminate`] when the margin lands strictly
/// inside the ambiguity band.
pub fn datum_distance_sign<T: Decide>(
    pose: &PoseValue<T>,
    p: Point3<T>,
    value: T,
    band: Band,
) -> Option<Result<Sign, Indeterminate>> {
    let d = datum_distance(pose, p)?;
    Some(geom_core::k_stats::decide(
        SEL_DATUM_DISTANCE,
        Margin::of(d - value),
        band,
    ))
}

/// **A pose definition** (D10; FORK-S3P, FORK-1b): how a pose
/// variable is defined, since no pose is free and none is defined from
/// nothing. Each arm states its result kind ([`Self::kind`]), and the
/// doors check each read and scalar at the kind it admits
/// ([`Self::reads`], [`Self::scalars`]).
///
/// Generic over its read form `R` and its scalar form `S`: the stored
/// definition reads variables by id (`PoseDef<VarId, VarId>`); the
/// authored one ([`crate::VarDecl::Pose`], [`crate::Operand::Pose`])
/// reads operands and writes formulas, which the edit door lowers.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum PoseDef<R = crate::VarId, S = crate::VarId> {
    /// **A face read as a plane**: its carrier's plane, with the face's
    /// outward normal (DM1a). A non-planar carrier refuses typed (DM1b).
    Plane {
        /// The face, a `Face` read.
        face: R,
    },
    /// **An axis read off geometry**: a face's carrier axis (a
    /// cylinder's, a cone's or a torus's), or an
    /// edge's line (a straight edge's line, a circular edge's axis).
    Axis {
        /// The face or edge.
        of: R,
    },
    /// **A point read off geometry**: a carrier's centre (a sphere's,
    /// a torus's, a cone's apex, a circle's or an ellipse's), or a
    /// vertex's point.
    Point {
        /// The face, edge or vertex.
        of: R,
    },
    /// **A pose written by coordinates in a frame** the definition
    /// reads, each coordinate a scalar variable. The frame is itself a
    /// definition, so this never starts from nothing.
    InFrame {
        /// The frame, a `Frame` read.
        frame: R,
        /// The coordinates, in the frame's axes; their shape is the
        /// result's kind.
        coords: PoseCoords<S>,
    },
    /// **The frame through an axis and a point** (FORK-1b): origin the
    /// foot of the point on the axis, `z` the axis, `x` towards the
    /// point. A point on the axis refuses typed at evaluation.
    Through {
        /// The axis, an `Axis` read.
        axis: R,
        /// The point, a `Point` read.
        point: R,
    },
    /// **The axis two planes meet in** (FORK-1b), along `a.n × b.n`,
    /// through the line's point nearest `a`'s origin. Parallel planes
    /// refuse typed at evaluation.
    Meet {
        /// The first plane.
        a: R,
        /// The second plane.
        b: R,
    },
    /// **The opposite sense** of a direction, axis, plane or frame (a
    /// frame turned a half-turn about its `x`). A flip of a flip is the
    /// pose itself, so the door refuses it.
    Flip {
        /// The pose flipped.
        pose: R,
    },
    /// **A plane moved along its own normal** by a length.
    Standoff {
        /// The plane.
        plane: R,
        /// How far, along the normal.
        by: S,
    },
    /// **A coarser pose of a finer one**: a frame's xy-plane, z-axis or
    /// origin, or a plane's or an axis's direction.
    Project {
        /// The pose projected.
        of: R,
        /// The kind it is projected to.
        to: crate::VarKind,
    },
}

/// **The coordinates of an [`PoseDef::InFrame`]**, one shape per pose
/// kind, each component a scalar slot in the frame's axes: lengths for
/// a position, scalars for a direction (normalized at evaluation, where
/// a degenerate one refuses).
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum PoseCoords<S = crate::VarId> {
    /// A point at `position`.
    Point {
        /// Its position (Length).
        position: [S; 3],
    },
    /// A direction along `direction`.
    Direction {
        /// The direction (Scalar).
        direction: [S; 3],
    },
    /// An axis through `origin` along `direction`.
    Axis {
        /// A point on it (Length).
        origin: [S; 3],
        /// Its direction (Scalar).
        direction: [S; 3],
    },
    /// A plane through `origin` with normal `normal`.
    Plane {
        /// A point on it (Length).
        origin: [S; 3],
        /// Its normal (Scalar).
        normal: [S; 3],
    },
    /// A frame at `origin` spanned by `u` and `v`, orthonormalized at
    /// evaluation.
    Frame {
        /// Its origin (Length).
        origin: [S; 3],
        /// Its x direction (Scalar).
        u: [S; 3],
        /// Its y direction (Scalar), orthogonalized against `u`.
        v: [S; 3],
    },
}

/// **Which read of a pose definition** an error or a walk addresses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum PoseRead {
    /// [`PoseDef::Plane`]'s face.
    Face,
    /// The geometry a read off geometry reads, or what a projection
    /// projects.
    Of,
    /// [`PoseDef::InFrame`]'s frame.
    Frame,
    /// [`PoseDef::Through`]'s axis.
    Axis,
    /// [`PoseDef::Through`]'s point.
    Point,
    /// [`PoseDef::Meet`]'s first plane.
    A,
    /// [`PoseDef::Meet`]'s second plane.
    B,
    /// What [`PoseDef::Flip`] flips.
    Pose,
    /// [`PoseDef::Standoff`]'s plane.
    Plane,
}

impl PoseRead {
    /// The read as a reader says it.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Face => "face",
            Self::Of => "source",
            Self::Frame => "frame",
            Self::Axis => "axis",
            Self::Point => "point",
            Self::A => "first plane",
            Self::B => "second plane",
            Self::Pose => "pose",
            Self::Plane => "plane",
        }
    }
}

/// **Which scalar of a pose definition**: a coordinate of an
/// [`PoseDef::InFrame`] or a [`PoseDef::Standoff`]'s length.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum PoseSlot {
    /// A position's or an origin's component (Length).
    Origin(crate::node::Axis3),
    /// A direction's component (Scalar).
    Direction(crate::node::Axis3),
    /// A plane's normal component (Scalar).
    Normal(crate::node::Axis3),
    /// A frame's x direction component (Scalar).
    U(crate::node::Axis3),
    /// A frame's y direction component (Scalar).
    V(crate::node::Axis3),
    /// A standoff's length.
    By,
}

impl PoseSlot {
    /// The dimension the slot is read at.
    #[must_use]
    pub fn dimension(self) -> crate::expr::Dimension {
        use crate::expr::Dimension;
        match self {
            Self::Origin(_) | Self::By => Dimension::Length,
            Self::Direction(_) | Self::Normal(_) | Self::U(_) | Self::V(_) => Dimension::Scalar,
        }
    }

    /// The slot as a reader says it: `origin x`, `by`.
    #[must_use]
    pub fn label(self) -> String {
        let axis = |a: crate::node::Axis3| match a {
            crate::node::Axis3::X => "x",
            crate::node::Axis3::Y => "y",
            crate::node::Axis3::Z => "z",
        };
        match self {
            Self::Origin(a) => format!("origin {}", axis(a)),
            Self::Direction(a) => format!("direction {}", axis(a)),
            Self::Normal(a) => format!("normal {}", axis(a)),
            Self::U(a) => format!("u {}", axis(a)),
            Self::V(a) => format!("v {}", axis(a)),
            Self::By => "by".to_owned(),
        }
    }
}

/// **The kinds a pose read admits beyond one** ([`crate::SlotKind::Pose`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum PoseAdmits {
    /// A face or an edge: what an axis is read off.
    Carrier,
    /// A face, an edge or a vertex: what a point is read off.
    Centre,
    /// A direction, an axis, a plane or a frame: what has a sense.
    Sensed,
    /// A frame, a plane or an axis: what has a coarser projection.
    Projected,
}

impl PoseAdmits {
    /// The kinds admitted, in the order a reader lists them.
    #[must_use]
    pub fn kinds(self) -> &'static [crate::VarKind] {
        use crate::VarKind as K;
        match self {
            Self::Carrier => &[K::Face, K::Edge],
            Self::Centre => &[K::Face, K::Edge, K::Vertex],
            Self::Sensed => &[K::Direction, K::Axis, K::Plane, K::Frame],
            Self::Projected => &[K::Frame, K::Plane, K::Axis],
        }
    }
}

/// **What is wrong with a pose definition's shape**, decided at the
/// doors from the kinds it reads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PoseFault {
    /// A read of a kind the read does not admit.
    ReadKind {
        /// The read.
        read: PoseRead,
        /// The kind it reads.
        found: crate::VarKind,
        /// What it admits.
        expected: crate::SlotKind,
    },
    /// A flip of a flip: the pose itself, said a second way.
    DoubleFlip,
    /// A projection the source kind has no arm to: a frame projects to
    /// a plane, an axis or a point, a plane or an axis to a direction.
    Projection {
        /// The source's kind.
        from: crate::VarKind,
        /// The kind asked for.
        to: crate::VarKind,
    },
}

impl core::fmt::Display for PoseFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ReadKind {
                read,
                found,
                expected,
            } => write!(
                f,
                "its {} reads {} {found}, where {expected} is read",
                read.label(),
                crate::sentence::article(&found.to_string())
            ),
            Self::DoubleFlip => f.write_str(
                "it flips a flip, which is the unflipped pose: read that pose instead",
            ),
            Self::Projection { from, to } => write!(
                f,
                "{} {from} has no {to} to project to (a frame projects to its plane, axis or \
                 origin; a plane or an axis to its direction)",
                crate::sentence::article(&from.to_string())
            ),
        }
    }
}

/// The projections a pose kind has (D10: a finer value is read through
/// its projection).
#[must_use]
pub fn projects(from: crate::VarKind, to: crate::VarKind) -> bool {
    use crate::VarKind as K;
    matches!(
        (from, to),
        (K::Frame, K::Plane | K::Axis | K::Point) | (K::Plane | K::Axis, K::Direction)
    )
}

impl<R, S> PoseDef<R, S> {
    /// **The kind this definition defines**, `None` for a flip, whose
    /// kind is its read's.
    #[must_use]
    pub fn kind(&self) -> Option<crate::VarKind> {
        use crate::VarKind as K;
        Some(match self {
            Self::Plane { .. } | Self::Standoff { .. } => K::Plane,
            Self::Axis { .. } | Self::Meet { .. } => K::Axis,
            Self::Point { .. } => K::Point,
            Self::Through { .. } => K::Frame,
            Self::InFrame { coords, .. } => coords.kind(),
            Self::Project { to, .. } => *to,
            Self::Flip { .. } => return None,
        })
    }

    /// **Its reads, each with the kind it admits**, in field order.
    #[must_use]
    pub fn reads(&self) -> Vec<(PoseRead, &R, crate::SlotKind)> {
        use crate::SlotKind::{Is, Pose};
        use crate::VarKind as K;
        match self {
            Self::Plane { face } => vec![(PoseRead::Face, face, Is(K::Face))],
            Self::Axis { of } => vec![(PoseRead::Of, of, Pose(PoseAdmits::Carrier))],
            Self::Point { of } => vec![(PoseRead::Of, of, Pose(PoseAdmits::Centre))],
            Self::InFrame { frame, .. } => vec![(PoseRead::Frame, frame, Is(K::Frame))],
            Self::Through { axis, point } => vec![
                (PoseRead::Axis, axis, Is(K::Axis)),
                (PoseRead::Point, point, Is(K::Point)),
            ],
            Self::Meet { a, b } => vec![
                (PoseRead::A, a, Is(K::Plane)),
                (PoseRead::B, b, Is(K::Plane)),
            ],
            Self::Flip { pose } => vec![(PoseRead::Pose, pose, Pose(PoseAdmits::Sensed))],
            Self::Standoff { plane, .. } => vec![(PoseRead::Plane, plane, Is(K::Plane))],
            Self::Project { of, .. } => vec![(PoseRead::Of, of, Pose(PoseAdmits::Projected))],
        }
    }

    /// **Its scalars, each with its slot**, in field order.
    #[must_use]
    pub fn scalars(&self) -> Vec<(PoseSlot, &S)> {
        match self {
            Self::InFrame { coords, .. } => coords.slots(),
            Self::Standoff { by, .. } => vec![(PoseSlot::By, by)],
            Self::Plane { .. }
            | Self::Axis { .. }
            | Self::Point { .. }
            | Self::Through { .. }
            | Self::Meet { .. }
            | Self::Flip { .. }
            | Self::Project { .. } => Vec::new(),
        }
    }

    /// **This definition in another form**: every read rewritten by
    /// `read` (handed its role), every scalar by `scalar` (handed its
    /// slot), in field order.
    ///
    /// # Errors
    ///
    /// The first refusal of either.
    pub fn try_map<R2, S2, E>(
        &self,
        read: &mut impl FnMut(PoseRead, &R) -> Result<R2, E>,
        scalar: &mut impl FnMut(PoseSlot, &S) -> Result<S2, E>,
    ) -> Result<PoseDef<R2, S2>, E> {
        Ok(match self {
            Self::Plane { face } => PoseDef::Plane {
                face: read(PoseRead::Face, face)?,
            },
            Self::Axis { of } => PoseDef::Axis {
                of: read(PoseRead::Of, of)?,
            },
            Self::Point { of } => PoseDef::Point {
                of: read(PoseRead::Of, of)?,
            },
            Self::InFrame { frame, coords } => PoseDef::InFrame {
                frame: read(PoseRead::Frame, frame)?,
                coords: coords.try_map(scalar)?,
            },
            Self::Through { axis, point } => PoseDef::Through {
                axis: read(PoseRead::Axis, axis)?,
                point: read(PoseRead::Point, point)?,
            },
            Self::Meet { a, b } => PoseDef::Meet {
                a: read(PoseRead::A, a)?,
                b: read(PoseRead::B, b)?,
            },
            Self::Flip { pose } => PoseDef::Flip {
                pose: read(PoseRead::Pose, pose)?,
            },
            Self::Standoff { plane, by } => PoseDef::Standoff {
                plane: read(PoseRead::Plane, plane)?,
                by: scalar(PoseSlot::By, by)?,
            },
            Self::Project { of, to } => PoseDef::Project {
                of: read(PoseRead::Of, of)?,
                to: *to,
            },
        })
    }
}

impl<S> PoseCoords<S> {
    /// The kind these coordinates write.
    #[must_use]
    pub fn kind(&self) -> crate::VarKind {
        use crate::VarKind as K;
        match self {
            Self::Point { .. } => K::Point,
            Self::Direction { .. } => K::Direction,
            Self::Axis { .. } => K::Axis,
            Self::Plane { .. } => K::Plane,
            Self::Frame { .. } => K::Frame,
        }
    }

    /// Every component with its slot, in field order.
    #[must_use]
    pub fn slots(&self) -> Vec<(PoseSlot, &S)> {
        use crate::node::Axis3;
        fn row<S>(at: fn(Axis3) -> PoseSlot, xs: &[S; 3]) -> Vec<(PoseSlot, &S)> {
            [Axis3::X, Axis3::Y, Axis3::Z]
                .into_iter()
                .zip(xs.iter())
                .map(|(a, x)| (at(a), x))
                .collect()
        }
        match self {
            Self::Point { position } => row(PoseSlot::Origin, position),
            Self::Direction { direction } => row(PoseSlot::Direction, direction),
            Self::Axis { origin, direction } => {
                let mut out = row(PoseSlot::Origin, origin);
                out.extend(row(PoseSlot::Direction, direction));
                out
            }
            Self::Plane { origin, normal } => {
                let mut out = row(PoseSlot::Origin, origin);
                out.extend(row(PoseSlot::Normal, normal));
                out
            }
            Self::Frame { origin, u, v } => {
                let mut out = row(PoseSlot::Origin, origin);
                out.extend(row(PoseSlot::U, u));
                out.extend(row(PoseSlot::V, v));
                out
            }
        }
    }

    /// These coordinates in another scalar form.
    ///
    /// # Errors
    ///
    /// The first refusal of `scalar`.
    pub fn try_map<S2, E>(
        &self,
        scalar: &mut impl FnMut(PoseSlot, &S) -> Result<S2, E>,
    ) -> Result<PoseCoords<S2>, E> {
        use crate::node::Axis3;
        let mut row = |at: fn(Axis3) -> PoseSlot, xs: &[S; 3]| -> Result<[S2; 3], E> {
            Ok([
                scalar(at(Axis3::X), &xs[0])?,
                scalar(at(Axis3::Y), &xs[1])?,
                scalar(at(Axis3::Z), &xs[2])?,
            ])
        };
        Ok(match self {
            Self::Point { position } => PoseCoords::Point {
                position: row(PoseSlot::Origin, position)?,
            },
            Self::Direction { direction } => PoseCoords::Direction {
                direction: row(PoseSlot::Direction, direction)?,
            },
            Self::Axis { origin, direction } => PoseCoords::Axis {
                origin: row(PoseSlot::Origin, origin)?,
                direction: row(PoseSlot::Direction, direction)?,
            },
            Self::Plane { origin, normal } => PoseCoords::Plane {
                origin: row(PoseSlot::Origin, origin)?,
                normal: row(PoseSlot::Normal, normal)?,
            },
            Self::Frame { origin, u, v } => PoseCoords::Frame {
                origin: row(PoseSlot::Origin, origin)?,
                u: row(PoseSlot::U, u)?,
                v: row(PoseSlot::V, v)?,
            },
        })
    }
}

/// **A carrier's kind**, surface or curve: what a pose read off
/// geometry found.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Carrier {
    /// A face's carrier.
    Surface(geom::SurfaceKind),
    /// An edge's carrier.
    Curve(geom::CurveKind),
}

impl core::fmt::Display for Carrier {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            Self::Surface(kind) => kind.name(),
            Self::Curve(kind) => kind.name(),
        };
        write!(f, "{} {name}", crate::sentence::article(name))
    }
}

/// **Why a pose read off geometry found none**
/// ([`crate::NodeErrorKind::PoseRead`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoseReadFault {
    /// A face read as a plane lies on a curved carrier (DM1b).
    NotPlanar {
        /// The carrier the face has.
        carrier: geom::SurfaceKind,
    },
    /// The carrier has no axis: a plane, a sphere, a curve with no line
    /// or centre axis.
    NoAxis {
        /// The carrier found.
        carrier: Carrier,
    },
    /// The carrier has no centre: a plane, a cylinder, a straight edge.
    NoCentre {
        /// The carrier found.
        carrier: Carrier,
    },
    /// The entity resolved to a key its body could not read back.
    Readback {
        /// The kernel's refusal, unaltered.
        error: topo::readback::ReadbackError,
    },
}

impl core::fmt::Display for PoseReadFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotPlanar { carrier } => write!(
                f,
                "the face lies on {} carrier, not a plane",
                Carrier::Surface(*carrier)
            ),
            Self::NoAxis { carrier } => write!(f, "{carrier} carrier has no axis"),
            Self::NoCentre { carrier } => write!(f, "{carrier} carrier has no centre"),
            Self::Readback { error } => {
                write!(f, "the entity resolved to a key its body could not read back: {error}")
            }
        }
    }
}

/// **A construction that refuses its degenerate case**
/// ([`crate::NodeErrorKind::PoseDegenerate`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoseConstruction {
    /// [`PoseDef::Through`]: the point lies on the axis.
    Through,
    /// [`PoseDef::Meet`]: the planes are parallel.
    Meet,
}

impl core::fmt::Display for PoseConstruction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Through => {
                "the frame through an axis and a point has no x direction: the point lies on \
                 the axis. Recourse: read a point off the axis"
            }
            Self::Meet => {
                "the planes are parallel, so they meet in no line. Recourse: read two planes \
                 that cross"
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{PoseValue, datum_distance, datum_distance_sign};
    use geom_core::linalg::{Point3, UnitVec3, Vec3};
    use geom_core::predicate::{Band, Sign};
    use topo::DATUM_UNIT_NORM;

    /// **A pose built from any scale measures a LENGTH**: the same
    /// plane spelled at scale 1e6 answers the same distance.
    #[test]
    fn a_pose_built_at_any_scale_measures_a_length() {
        let band = Band::new(1e-6, 1e-3).expect("a well-ordered band");
        let p = Point3::new(3.0, 4.0, -2.0);
        let at = |v| PoseValue::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: UnitVec3::new(v, DATUM_UNIT_NORM, band).expect("a vector with a length"),
        };
        assert_eq!(
            datum_distance(&at(Vec3::new(0.0, 0.0, 1e6)), p),
            datum_distance(&at(Vec3::new(0.0, 0.0, 1.0)), p)
        );
    }

    #[test]
    fn the_decided_door_partitions_on_the_band() {
        let band = Band::new(1e-6, 1e-3).expect("a well-ordered band");
        let up = UnitVec3::new(Vec3::new(0.0, 0.0, 1.0), DATUM_UNIT_NORM, band).expect("a unit z");
        let origin = Point3::new(0.0, 0.0, 0.0);
        let plane = PoseValue::Plane { origin, normal: up };
        let axis = PoseValue::Axis { origin, dir: up };
        let point = PoseValue::Point { position: origin };
        let p = Point3::new(3.0, 4.0, -2.0);
        // SIGNED along a plane's normal, UNSIGNED to an axis or point.
        assert_eq!(datum_distance(&plane, p), Some(-2.0));
        assert_eq!(datum_distance(&axis, p), Some(5.0));
        assert!(datum_distance(&point, p).is_some_and(|d| (d - 29.0_f64.sqrt()).abs() < 1e-12));
        assert_eq!(datum_distance(&PoseValue::Direction { dir: up }, p), None);
        for pose in [&plane, &axis, &point] {
            let d = datum_distance(pose, p).expect("a pose with a position");
            let sign = |value| datum_distance_sign(pose, p, value, band).expect("a position");
            for dv in [0.0, 1e-7, -1e-7] {
                assert_eq!(sign(d - dv), Ok(Sign::Zero), "dv={dv}");
            }
            for dv in [5e-4, -5e-4, 2e-6, -2e-6] {
                assert!(sign(d - dv).is_err(), "dv={dv}");
            }
            for dv in [2e-3, 1.0] {
                assert_eq!(sign(d - dv), Ok(Sign::Positive), "dv={dv}");
                assert_eq!(sign(d + dv), Ok(Sign::Negative), "dv={dv}");
            }
        }
    }
}
