//! **A pose value** (D10, Variables): the evaluated value of a pose
//! variable — a frame known up to its kind's symmetry, as geometry
//! VALUES, never kernel entities or recipe references.
//!
//! Every pose a reader reads reaches it here: an operation's pose
//! output ([`crate::VarDef::Output`]: a datum's, a revolve's axis) or a
//! pose definition ([`crate::VarDef::Pose`]) bound at its reader by
//! `eval_pose` (`eval/wire/pose.rs`). Its normals and axis directions are
//! [`UnitVec3`], unit by construction, so an unnormalized pose has no
//! spelling.
//!
//! Each value carries its kind's symmetry ([`PoseSymmetry`]): the
//! [`Subgroup`] the mates fold is the one a pose is known up to, read
//! here once ([`crate::VarKind::symmetry`] names its family).

use geom_core::linalg::{Point3, RigidMotion, UnitVec3};
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
pub const SEL_PLANE_DISTANCE: &str = "sel_plane_distance";

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
    /// An axis: known up to slide and spin along itself, so it holds
    /// its line alone ([`AxisLine`]), and no reader sees a slide along
    /// it.
    Axis(AxisLine<T>),
    /// The plane `p · normal = offset`: known up to in-plane motion, so
    /// it holds those invariants alone, and no reader sees a point in
    /// the plane ([`Self::plane_through`] reads one off).
    Plane {
        /// The normal.
        normal: UnitVec3<T>,
        /// The signed distance of the plane from its space's origin
        /// along `normal` (Length).
        offset: T,
    },
    /// **A frame**: origin plus a right-handed orthonormal pair of
    /// in-plane directions, so the surface and the spin about its
    /// normal are pinned. The payload is the frame witness: `u` and `v`
    /// are unit and orthogonal as a property of the type, and
    /// `w = u × v` is the normal.
    Frame(RigidMotion<T>),
}

/// **An axis's line**: its direction and its foot, the line's point
/// nearest its space's origin. The fields are private, so every foot is
/// the one [`Self::through`] projects, and a construction that keeps
/// the line ([`Self::reversed`]) goes through this type.
#[derive(Debug, Clone, Copy)]
pub struct AxisLine<T: Real> {
    foot: Point3<T>,
    dir: UnitVec3<T>,
}

impl<T: Real> AxisLine<T> {
    /// **The line through `point` along `dir`**: its foot is `point`
    /// with its component along `dir` taken off.
    #[must_use]
    pub fn through(point: Point3<T>, dir: UnitVec3<T>) -> Self {
        let d = dir.get();
        Self {
            foot: point - d * (point - Point3::origin()).dot(d),
            dir,
        }
    }

    /// The line's point nearest its space's origin.
    #[must_use]
    pub fn foot(&self) -> Point3<T> {
        self.foot
    }

    /// The line's direction.
    #[must_use]
    pub fn dir(&self) -> UnitVec3<T> {
        self.dir
    }

    /// The same line in the opposite sense: its foot is kept.
    #[must_use]
    pub fn reversed(self) -> Self {
        Self {
            foot: self.foot,
            dir: -self.dir,
        }
    }
}

impl<T: Real> PoseValue<T> {
    /// **The axis through `point` along `dir`** ([`AxisLine::through`]).
    #[must_use]
    pub fn axis_through(point: Point3<T>, dir: UnitVec3<T>) -> Self {
        Self::Axis(AxisLine::through(point, dir))
    }

    /// **The plane through `point` with `normal`**: its offset the
    /// point's along the normal.
    #[must_use]
    pub fn plane_through(point: Point3<T>, normal: UnitVec3<T>) -> Self {
        Self::Plane {
            normal,
            offset: (point - Point3::origin()).dot(normal.get()),
        }
    }

    /// The direction a direction or an axis runs along; `None` for any
    /// other kind.
    #[must_use]
    pub(crate) fn heading(&self) -> Option<UnitVec3<T>> {
        match self {
            Self::Direction { dir } => Some(*dir),
            Self::Axis(line) => Some(line.dir),
            Self::Point { .. } | Self::Plane { .. } | Self::Frame(_) => None,
        }
    }

    /// **A plane's point nearest its space's origin**: `offset · normal`.
    #[must_use]
    pub fn plane_foot(normal: UnitVec3<T>, offset: T) -> Point3<T> {
        Point3::origin() + normal.get() * offset
    }

    /// The kind this value is of.
    #[must_use]
    pub fn kind(&self) -> crate::VarKind {
        match self {
            Self::Point { .. } => crate::VarKind::Point,
            Self::Direction { .. } => crate::VarKind::Direction,
            Self::Axis(_) => crate::VarKind::Axis,
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
            Self::Axis(line) => Subgroup::Cylindrical {
                point: line.foot,
                direction: line.dir,
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
/// [`pose_distance_sign`]'s.
#[must_use]
pub fn pose_distance<T: Real>(pose: &PoseValue<T>, p: Point3<T>) -> Option<T> {
    Some(match pose {
        PoseValue::Plane { normal, offset } => (p - Point3::origin()).dot(normal.get()) - *offset,
        PoseValue::Axis(line) => {
            let d = line.dir.get();
            let v = p - line.foot;
            (v - d * v.dot(d)).norm()
        }
        PoseValue::Point { position } => (p - *position).norm(),
        PoseValue::Frame(f) => (p - f.origin()).dot(f.w().get()),
        PoseValue::Direction { .. } => return None,
    })
}

/// DECIDED: which side of the stated `value` the point's
/// [`pose_distance`] lands on, through the [`SEL_PLANE_DISTANCE`]
/// funnel. `None` for a direction.
///
/// # Errors
///
/// The funnel's [`Indeterminate`] when the margin lands strictly
/// inside the ambiguity band.
pub fn pose_distance_sign<T: Decide>(
    pose: &PoseValue<T>,
    p: Point3<T>,
    value: T,
    band: Band,
) -> Option<Result<Sign, Indeterminate>> {
    let d = pose_distance(pose, p)?;
    Some(geom_core::k_stats::decide(
        SEL_PLANE_DISTANCE,
        Margin::of(d - value),
        band,
    ))
}

/// **A pose definition** (D10; FORK-S3P, FORK-1b, FORK-MOVE): how a
/// pose variable is defined, since no pose is free and none is defined
/// from nothing. Each arm states its result kind ([`Self::kind`]), and
/// the doors check each read and scalar at the kind it admits
/// ([`Self::reads`], [`Self::scalars`]).
///
/// Every number a definition holds is a `Length` along one direction or
/// an `Angle` about one axis ([`Self::Slide`], [`Self::Spin`]): no slot
/// holds a component of a direction or of a frame's axis, so a zero
/// direction, an axis that is not orthogonal and a normalisation cannot
/// be written.
///
/// Generic over its read form `R` and its scalar form `S`: the stored
/// definition reads variables by id (`PoseDef<VarId, VarId>`); the
/// authored one ([`crate::Operand::Pose`], defined at a seat)
/// reads operands and writes formulas, which the edit door lowers.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(bound(
    serialize = "R: serde::Serialize, S: serde::Serialize",
    deserialize = "R: serde::Deserialize<'de>, S: serde::Deserialize<'de>"
))]
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
    /// **A pose slid** along a direction by a length: a point, an axis,
    /// a plane or a frame moved, its kind kept. A direction has no
    /// position to slide, so its slide is no variant. A chain is
    /// nesting, in the order the author wrote it.
    Slide {
        /// The pose slid.
        pose: RoleRead<R>,
        /// The direction it slides along, a `Direction` read, or an
        /// `Axis` read for its direction.
        along: RoleRead<R>,
        /// How far (Length).
        by: S,
    },
    /// **A pose spun** about an axis by an angle, right-handed: any
    /// pose turned about the line, its kind kept.
    Spin {
        /// The pose spun.
        pose: RoleRead<R>,
        /// The axis it spins about, an `Axis` read.
        about: RoleRead<R>,
        /// How far (Angle).
        by: S,
    },
    /// **The frame through an axis and a point** (FORK-1b): origin the
    /// foot of the point on the axis, `z` the axis, `x` towards the
    /// point. A point on the axis refuses typed at evaluation.
    Through {
        /// The axis, an `Axis` read.
        axis: RoleRead<R>,
        /// The point, a `Point` read.
        point: RoleRead<R>,
    },
    /// **The axis two planes meet in** (FORK-1b), along `a.n × b.n`,
    /// through the line's point nearest the origin. Parallel planes
    /// refuse typed at evaluation.
    Meet {
        /// The first plane.
        a: RoleRead<R>,
        /// The second plane.
        b: RoleRead<R>,
    },
    /// **The opposite sense** of a direction, axis, plane or frame (a
    /// frame turned a half-turn about its `x`). A flip of a flip is the
    /// pose itself: the edit door normalises one whose inner flip reads
    /// plainly to that pose, and refuses one whose inner flip reads
    /// through a role (the pose read through the role is the one
    /// spelling), so a document never stores one
    /// ([`PoseFault::DoubleFlip`] at the load door).
    Flip {
        /// The pose flipped.
        pose: RoleRead<R>,
    },
    /// **A pose of a copied shape as the copy carries it** (FORK-S3M):
    /// `pose`, read off a shape a placement copies, moved by the motion
    /// that placement's bundle solved to, keyed by the placement whose
    /// copies `copy` reads. Its kind is its pose's.
    Carried {
        /// The placement's copies, a `Body` or `Bodies` read of a
        /// placement's output.
        copy: R,
        /// The pose carried.
        pose: RoleRead<R>,
    },
}

/// **The role a finer pose is read through** at a coarser read (D10:
/// "a finer value is read through its projection, and the role it is
/// read through is part of the read"): a frame's x, y or z axis, its
/// origin or one of its coordinate planes; a plane's normal; an axis's
/// direction.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum PoseRole {
    /// A frame's x axis: through its origin along `u`.
    X,
    /// A frame's y axis: through its origin along `v`.
    Y,
    /// A frame's z axis: through its origin along its normal.
    Z,
    /// A frame's origin.
    Origin,
    /// A frame's xy plane: through its origin, normal its z.
    Xy,
    /// A frame's yz plane: through its origin, normal its x.
    Yz,
    /// A frame's zx plane: through its origin, normal its y.
    Zx,
    /// A plane's normal.
    Normal,
    /// An axis's direction.
    Direction,
}

impl PoseRole {
    /// Every role, in the order a reader lists them.
    pub const ALL: [Self; 9] = [
        Self::X,
        Self::Y,
        Self::Z,
        Self::Origin,
        Self::Xy,
        Self::Yz,
        Self::Zx,
        Self::Normal,
        Self::Direction,
    ];

    /// The kind of pose that offers this role.
    #[must_use]
    pub fn of(self) -> crate::VarKind {
        use crate::VarKind as K;
        match self {
            Self::X | Self::Y | Self::Z | Self::Origin | Self::Xy | Self::Yz | Self::Zx => K::Frame,
            Self::Normal => K::Plane,
            Self::Direction => K::Axis,
        }
    }

    /// The kind a read through this role reads.
    #[must_use]
    pub fn kind(self) -> crate::VarKind {
        use crate::VarKind as K;
        match self {
            Self::X | Self::Y | Self::Z => K::Axis,
            Self::Origin => K::Point,
            Self::Xy | Self::Yz | Self::Zx => K::Plane,
            Self::Normal | Self::Direction => K::Direction,
        }
    }

    /// The role as a reader says it: `x axis`, `xy plane`.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::X => "x axis",
            Self::Y => "y axis",
            Self::Z => "z axis",
            Self::Origin => "origin",
            Self::Xy => "xy plane",
            Self::Yz => "yz plane",
            Self::Zx => "zx plane",
            Self::Normal => "normal",
            Self::Direction => "direction",
        }
    }

    /// **`value` read through this role**, `None` where its kind offers
    /// no such role (a read the door refuses).
    #[must_use]
    pub fn read<T: Real>(self, value: &PoseValue<T>) -> Option<PoseValue<T>> {
        Some(match (value, self) {
            (PoseValue::Frame(f), Self::X | Self::Y | Self::Z) => {
                PoseValue::axis_through(f.origin(), Self::frame_axis(f, self))
            }
            (PoseValue::Frame(f), Self::Origin) => PoseValue::Point {
                position: f.origin(),
            },
            (PoseValue::Frame(f), Self::Xy | Self::Yz | Self::Zx) => PoseValue::plane_through(
                f.origin(),
                Self::frame_axis(
                    f,
                    match self {
                        Self::Xy => Self::Z,
                        Self::Yz => Self::X,
                        _ => Self::Y,
                    },
                ),
            ),
            (PoseValue::Plane { normal: dir, .. }, Self::Normal) => {
                PoseValue::Direction { dir: *dir }
            }
            (PoseValue::Axis(line), Self::Direction) => PoseValue::Direction { dir: line.dir },
            _ => return None,
        })
    }

    /// A frame's axis direction for an axis role.
    fn frame_axis<T: Real>(f: &RigidMotion<T>, axis: Self) -> UnitVec3<T> {
        match axis {
            Self::X => f.u(),
            Self::Y => f.v(),
            _ => f.w(),
        }
    }
}

impl core::fmt::Display for PoseRole {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.label())
    }
}

/// **A read of a pose, through the role it is read through where it
/// reads a finer pose at a coarser seat** (D10; VR4). A plain read
/// carries no role, and is serialized as the bare read; a read through
/// a role as `{"read": …, "role": …}`.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoleRead<R> {
    /// The variable read.
    pub read: R,
    /// The role it is read through, `None` for the pose itself.
    pub role: Option<PoseRole>,
}

impl<R> RoleRead<R> {
    /// A plain read of `read`.
    #[must_use]
    pub fn plain(read: impl Into<R>) -> Self {
        Self {
            read: read.into(),
            role: None,
        }
    }

    /// `read` read through `role`.
    #[must_use]
    pub fn through(read: impl Into<R>, role: PoseRole) -> Self {
        Self {
            read: read.into(),
            role: Some(role),
        }
    }

    /// This read in another form, its role kept.
    ///
    /// # Errors
    ///
    /// `f`'s.
    pub fn try_map<R2, E>(&self, f: impl FnOnce(&R) -> Result<R2, E>) -> Result<RoleRead<R2>, E> {
        Ok(RoleRead {
            read: f(&self.read)?,
            role: self.role,
        })
    }

    /// **What the read's variable may be, at a seat admitting
    /// `admits`**: the seat's own kinds read plainly, or through a role,
    /// the kind that offers it.
    #[must_use]
    pub fn admits(&self, admits: crate::SlotKind) -> crate::SlotKind {
        self.role
            .map_or(admits, |role| crate::SlotKind::Is(role.of()))
    }
}

impl<R: core::fmt::Debug> core::fmt::Debug for RoleRead<R> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // A plain read dumps as the bare read, as it serializes.
        let Self { read, role } = self;
        match role {
            None => read.fmt(f),
            Some(role) => f
                .debug_struct("RoleRead")
                .field("read", read)
                .field("role", role)
                .finish(),
        }
    }
}

impl From<crate::VarId> for RoleRead<crate::VarId> {
    fn from(read: crate::VarId) -> Self {
        Self::plain(read)
    }
}

impl From<crate::Operand> for RoleRead<crate::Operand> {
    fn from(read: crate::Operand) -> Self {
        Self::plain(read)
    }
}

impl From<crate::VarId> for RoleRead<crate::Operand> {
    fn from(var: crate::VarId) -> Self {
        Self::plain(var)
    }
}

impl From<crate::RecipeNodeId> for RoleRead<crate::Operand> {
    fn from(node: crate::RecipeNodeId) -> Self {
        Self::plain(node)
    }
}

impl<R: serde::Serialize> serde::Serialize for RoleRead<R> {
    fn serialize<Z: serde::Serializer>(&self, ser: Z) -> Result<Z::Ok, Z::Error> {
        use serde::ser::SerializeStruct as _;
        let Some(role) = self.role else {
            return self.read.serialize(ser);
        };
        let mut st = ser.serialize_struct("RoleRead", 2)?;
        st.serialize_field("read", &self.read)?;
        st.serialize_field("role", &role)?;
        st.end()
    }
}

impl<'de, R: serde::Deserialize<'de>> serde::Deserialize<'de> for RoleRead<R> {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        use crate::operand::{Prepended, Spelled};
        use serde::de::value::MapAccessDeserializer;
        use serde::de::{Error as _, MapAccess, Visitor};

        /// Told apart by the first key, in one forward pass: a map whose
        /// first key is `read` or `role` is the read through its role,
        /// and anything else is the bare read, handed on whole.
        struct Either<R>(core::marker::PhantomData<R>);

        impl<'de, R: serde::Deserialize<'de>> Visitor<'de> for Either<R> {
            type Value = RoleRead<R>;
            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("a read, or a read through its role (`{\"read\": …, \"role\": …}`)")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                R::deserialize(Spelled::<E>(v, core::marker::PhantomData)).map(RoleRead::plain)
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let first = map.next_key::<String>()?;
                if !matches!(first.as_deref(), Some("read" | "role")) {
                    let rest = Prepended { first, rest: map };
                    return R::deserialize(MapAccessDeserializer::new(rest)).map(RoleRead::plain);
                }
                let (mut read, mut role) = (None, None);
                let mut next = first;
                while let Some(k) = next {
                    match k.as_str() {
                        "read" if read.is_none() => read = Some(map.next_value::<R>()?),
                        "role" if role.is_none() => role = Some(map.next_value::<PoseRole>()?),
                        "read" | "role" => return Err(A::Error::duplicate_field("read or role")),
                        other => return Err(A::Error::unknown_field(other, &["read", "role"])),
                    }
                    next = map.next_key::<String>()?;
                }
                let read = read.ok_or_else(|| A::Error::missing_field("read"))?;
                let role = role.ok_or_else(|| A::Error::missing_field("role"))?;
                Ok(RoleRead::through(read, role))
            }
        }

        de.deserialize_any(Either(core::marker::PhantomData))
    }
}

/// **Which read of a pose definition** an error or a walk addresses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum PoseRead {
    /// [`PoseDef::Plane`]'s face.
    Face,
    /// The geometry a read off geometry reads.
    Of,
    /// [`PoseDef::Through`]'s axis.
    Axis,
    /// [`PoseDef::Through`]'s point.
    Point,
    /// [`PoseDef::Meet`]'s first plane.
    A,
    /// [`PoseDef::Meet`]'s second plane.
    B,
    /// What a move, a flip or a carried pose moves.
    Pose,
    /// [`PoseDef::Slide`]'s direction.
    Along,
    /// [`PoseDef::Spin`]'s axis.
    About,
    /// The copies a [`PoseDef::Carried`] pose is carried by.
    Copy,
}

impl PoseRead {
    /// The read as a reader says it.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Face => "face",
            Self::Of => "source",
            Self::Axis => "axis",
            Self::Point => "point",
            Self::A => "first plane",
            Self::B => "second plane",
            Self::Pose => "pose",
            Self::Along => "direction",
            Self::About => "axis",
            Self::Copy => "copy",
        }
    }
}

/// **Which scalar of a pose definition**: a slide's length or a spin's
/// angle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum PoseSlot {
    /// [`PoseDef::Slide`]'s length.
    Slide,
    /// [`PoseDef::Spin`]'s angle.
    Spin,
}

impl PoseSlot {
    /// The dimension the slot is read at.
    #[must_use]
    pub fn dimension(self) -> crate::expr::Dimension {
        use crate::expr::Dimension;
        match self {
            Self::Slide => Dimension::Length,
            Self::Spin => Dimension::Angle,
        }
    }

    /// The slot as a reader says it.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Slide => "slide",
            Self::Spin => "spin",
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
    /// A point, an axis, a plane or a frame: what has a position to
    /// slide.
    Slid,
    /// A direction or an axis: what a slide is along.
    Heading,
    /// Any pose: what a copy carries and a spin turns.
    Any,
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
            Self::Slid => &[K::Point, K::Axis, K::Plane, K::Frame],
            Self::Heading => &[K::Direction, K::Axis],
            Self::Any => &[K::Point, K::Direction, K::Axis, K::Plane, K::Frame],
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
    /// A flip of a flip: the pose itself, said a second way. The edit
    /// door normalises one; a file holding one refuses at load.
    DoubleFlip,
    /// A read through a role its pose's kind does not offer, or whose
    /// projection the read does not admit.
    Role {
        /// The read.
        read: PoseRead,
        /// The role it reads through.
        role: PoseRole,
        /// The kind of the pose it reads.
        found: crate::VarKind,
        /// What the read admits.
        expected: crate::SlotKind,
    },
    /// A read or a scalar the document's mint log does not hold (a
    /// file's fault, never a door's).
    Unminted {
        /// The variable read.
        var: crate::VarId,
    },
    /// A scalar of another dimension than its slot's.
    ScalarKind {
        /// The slot.
        slot: PoseSlot,
        /// The kind it holds.
        found: crate::VarKind,
    },
    /// A move, a flip or a carried pose stored at another kind than the
    /// pose it reads.
    Kind {
        /// The kind stored.
        stored: crate::VarKind,
        /// The kind its read defines.
        defined: crate::VarKind,
    },
    /// The definition reaches itself through the poses it reads.
    Cycle,
    /// A carried pose's copy is no placement's output: only a
    /// placement's copies carry a pose.
    NotACopy {
        /// The variable read as the copy.
        var: crate::VarId,
    },
}

impl PoseFault {
    /// The edit that writes this pose as a door stores it, where the
    /// fault names one of its own; `None` where the read's kinds are
    /// the way through.
    pub(crate) fn own_recourse(&self) -> Option<&'static str> {
        match self {
            Self::DoubleFlip => Some(
                "read the unflipped pose instead, through the role the inner flip reads it \
                 through if it reads one",
            ),
            _ => None,
        }
    }
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
            Self::DoubleFlip => f.write_str("it flips a flip, which is the unflipped pose"),
            Self::Unminted { var } => {
                write!(
                    f,
                    "it reads {var}, which the document's mint log does not hold"
                )
            }
            Self::ScalarKind { slot, found } => write!(
                f,
                "its {} holds {} {found}, where {} {} is read",
                slot.label(),
                crate::sentence::article(&found.to_string()),
                crate::sentence::article(&slot.dimension().to_string()),
                slot.dimension()
            ),
            Self::Kind { stored, defined } => write!(
                f,
                "it is stored as {} {stored} and reads {} {defined}",
                crate::sentence::article(&stored.to_string()),
                crate::sentence::article(&defined.to_string())
            ),
            Self::Cycle => f.write_str("it reaches itself through the poses it reads"),
            Self::NotACopy { var } => write!(
                f,
                "it carries its pose by {var}, which is no placement's copies"
            ),
            Self::Role {
                read,
                role,
                found,
                expected,
            } if *found != role.of() => write!(
                f,
                "its {} reads {} {found} through its {role}, which only {} {} offers (a frame \
                 its axes, origin and planes; a plane its normal; an axis its direction); \
                 {expected} is read there",
                read.label(),
                crate::sentence::article(&found.to_string()),
                crate::sentence::article(&role.of().to_string()),
                role.of()
            ),
            Self::Role {
                read,
                role,
                found,
                expected,
            } => write!(
                f,
                "its {} reads {} {found}'s {role}, which is {} {}, where {expected} is read",
                read.label(),
                crate::sentence::article(&found.to_string()),
                crate::sentence::article(&role.kind().to_string()),
                role.kind()
            ),
        }
    }
}

impl<R, S> PoseDef<R, S> {
    /// **The kind this definition defines**, `None` for a move, a flip
    /// and a carried pose, whose kind is their pose read's.
    #[must_use]
    pub fn kind(&self) -> Option<crate::VarKind> {
        use crate::VarKind as K;
        Some(match self {
            Self::Plane { .. } => K::Plane,
            Self::Axis { .. } | Self::Meet { .. } => K::Axis,
            Self::Point { .. } => K::Point,
            Self::Through { .. } => K::Frame,
            Self::Slide { .. } | Self::Spin { .. } | Self::Flip { .. } | Self::Carried { .. } => {
                return None;
            }
        })
    }

    /// **The read whose kind this definition's is**, for a move, a flip
    /// and a carried pose; `None` for an arm that states its kind.
    #[must_use]
    pub fn kept(&self) -> Option<&RoleRead<R>> {
        match self {
            Self::Slide { pose, .. }
            | Self::Spin { pose, .. }
            | Self::Flip { pose }
            | Self::Carried { pose, .. } => Some(pose),
            Self::Plane { .. }
            | Self::Axis { .. }
            | Self::Point { .. }
            | Self::Through { .. }
            | Self::Meet { .. } => None,
        }
    }

    /// **Its reads, each with the role it reads through and the kind it
    /// admits**, in field order.
    #[must_use]
    pub fn reads(&self) -> Vec<(PoseRead, &R, Option<PoseRole>, crate::SlotKind)> {
        use crate::SlotKind::{Is, Pose};
        use crate::VarKind as K;
        fn at<R>(
            read: PoseRead,
            r: &RoleRead<R>,
            admits: crate::SlotKind,
        ) -> (PoseRead, &R, Option<PoseRole>, crate::SlotKind) {
            (read, &r.read, r.role, admits)
        }
        match self {
            Self::Plane { face } => vec![(PoseRead::Face, face, None, Is(K::Face))],
            Self::Axis { of } => vec![(PoseRead::Of, of, None, Pose(PoseAdmits::Carrier))],
            Self::Point { of } => vec![(PoseRead::Of, of, None, Pose(PoseAdmits::Centre))],
            Self::Slide { pose, along, .. } => vec![
                at(PoseRead::Pose, pose, Pose(PoseAdmits::Slid)),
                at(PoseRead::Along, along, Pose(PoseAdmits::Heading)),
            ],
            Self::Spin { pose, about, .. } => vec![
                at(PoseRead::Pose, pose, Pose(PoseAdmits::Any)),
                at(PoseRead::About, about, Is(K::Axis)),
            ],
            Self::Through { axis, point } => vec![
                at(PoseRead::Axis, axis, Is(K::Axis)),
                at(PoseRead::Point, point, Is(K::Point)),
            ],
            Self::Meet { a, b } => vec![
                at(PoseRead::A, a, Is(K::Plane)),
                at(PoseRead::B, b, Is(K::Plane)),
            ],
            Self::Flip { pose } => vec![at(PoseRead::Pose, pose, Pose(PoseAdmits::Sensed))],
            Self::Carried { copy, pose } => vec![
                (PoseRead::Copy, copy, None, crate::SlotKind::Placeable),
                at(PoseRead::Pose, pose, Pose(PoseAdmits::Any)),
            ],
        }
    }

    /// **Its scalars, each with its slot**, in field order.
    #[must_use]
    pub fn scalars(&self) -> Vec<(PoseSlot, &S)> {
        match self {
            Self::Slide { by, .. } => vec![(PoseSlot::Slide, by)],
            Self::Spin { by, .. } => vec![(PoseSlot::Spin, by)],
            Self::Plane { .. }
            | Self::Axis { .. }
            | Self::Point { .. }
            | Self::Through { .. }
            | Self::Meet { .. }
            | Self::Flip { .. }
            | Self::Carried { .. } => Vec::new(),
        }
    }

    /// **This definition in another form**: every read rewritten by
    /// `read` (handed its role in the definition), every scalar by
    /// `scalar` (handed its slot), in field order; each read's
    /// projection role is kept.
    ///
    /// # Errors
    ///
    /// The first refusal of either.
    pub fn try_map<R2, S2, E>(
        &self,
        read: &mut impl FnMut(PoseRead, &R) -> Result<R2, E>,
        scalar: &mut impl FnMut(PoseSlot, &S) -> Result<S2, E>,
    ) -> Result<PoseDef<R2, S2>, E> {
        fn at<R, R2, E>(
            read: &mut impl FnMut(PoseRead, &R) -> Result<R2, E>,
            which: PoseRead,
            r: &RoleRead<R>,
        ) -> Result<RoleRead<R2>, E> {
            r.try_map(|r| read(which, r))
        }
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
            Self::Slide { pose, along, by } => PoseDef::Slide {
                pose: at(read, PoseRead::Pose, pose)?,
                along: at(read, PoseRead::Along, along)?,
                by: scalar(PoseSlot::Slide, by)?,
            },
            Self::Spin { pose, about, by } => PoseDef::Spin {
                pose: at(read, PoseRead::Pose, pose)?,
                about: at(read, PoseRead::About, about)?,
                by: scalar(PoseSlot::Spin, by)?,
            },
            Self::Through { axis, point } => PoseDef::Through {
                axis: at(read, PoseRead::Axis, axis)?,
                point: at(read, PoseRead::Point, point)?,
            },
            Self::Meet { a, b } => PoseDef::Meet {
                a: at(read, PoseRead::A, a)?,
                b: at(read, PoseRead::B, b)?,
            },
            Self::Flip { pose } => PoseDef::Flip {
                pose: at(read, PoseRead::Pose, pose)?,
            },
            Self::Carried { copy, pose } => PoseDef::Carried {
                copy: read(PoseRead::Copy, copy)?,
                pose: at(read, PoseRead::Pose, pose)?,
            },
        })
    }
}

impl PoseDef<crate::Operand, crate::Formula> {
    /// **The point at `(x, y, z)` in `frame`** (FORK-MOVE): the frame's
    /// origin slid along its x, then its y, then its z axis, each by its
    /// length, as an author would write it nested.
    #[must_use]
    pub fn point_in(frame: &crate::Operand, at: [crate::Formula; 3]) -> Self {
        let [x, y, z] = at;
        let slide = |pose: RoleRead<crate::Operand>, along, by| PoseDef::Slide {
            pose,
            along: RoleRead::through(frame.clone(), along),
            by,
        };
        let nested = |def: Self| RoleRead::plain(crate::Operand::Pose(Box::new(def)));
        let along_x = slide(
            RoleRead::through(frame.clone(), PoseRole::Origin),
            PoseRole::X,
            x,
        );
        let along_y = slide(nested(along_x), PoseRole::Y, y);
        slide(nested(along_y), PoseRole::Z, z)
    }
}

/// **The kind a read through `role` reads, where `found` offers it and
/// `admits` admits it**; `None` otherwise: the one rule both doors
/// check a role read by.
#[must_use]
pub fn role_kind(
    found: crate::VarKind,
    role: PoseRole,
    admits: crate::SlotKind,
) -> Option<crate::VarKind> {
    let kind = role.kind();
    let admitted = match admits {
        crate::SlotKind::Is(is) => is == kind,
        crate::SlotKind::Pose(set) => set.kinds().contains(&kind),
        crate::SlotKind::Placeable | crate::SlotKind::Measured(_) => false,
    };
    (found == role.of() && admitted).then_some(kind)
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
                write!(
                    f,
                    "the entity resolved to a key its body could not read back: {error}"
                )
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
    /// A value's chart: a direction its least turn carries that the
    /// band cannot mint as a unit.
    Chart,
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
            Self::Chart => {
                "a value's chart cannot mint the direction its least turn carries as a unit. \
                 Recourse: read the mate's poses off geometry whose directions are unit"
            }
        })
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]
    use super::{PoseValue, pose_distance, pose_distance_sign};
    use geom_core::linalg::{Point3, UnitVec3, Vec3};
    use geom_core::predicate::{Band, Sign};
    use topo::POSE_UNIT_NORM;

    /// **A pose built from any scale measures a LENGTH**: the same
    /// plane spelled at scale 1e6 answers the same distance.
    #[test]
    fn a_pose_built_at_any_scale_measures_a_length() {
        let band = Band::new(1e-6, 1e-3).expect("a well-ordered band");
        let p = Point3::new(3.0, 4.0, -2.0);
        let at = |v| PoseValue::Plane {
            normal: UnitVec3::new(v, POSE_UNIT_NORM, band).expect("a vector with a length"),
            offset: 0.0,
        };
        assert_eq!(
            pose_distance(&at(Vec3::new(0.0, 0.0, 1e6)), p),
            pose_distance(&at(Vec3::new(0.0, 0.0, 1.0)), p)
        );
    }

    #[test]
    fn the_decided_door_partitions_on_the_band() {
        let band = Band::new(1e-6, 1e-3).expect("a well-ordered band");
        let up = UnitVec3::new(Vec3::new(0.0, 0.0, 1.0), POSE_UNIT_NORM, band).expect("a unit z");
        let origin = Point3::new(0.0, 0.0, 0.0);
        let plane = PoseValue::plane_through(origin, up);
        let axis = PoseValue::axis_through(origin, up);
        let point = PoseValue::Point { position: origin };
        let p = Point3::new(3.0, 4.0, -2.0);
        // SIGNED along a plane's normal, UNSIGNED to an axis or point.
        assert_eq!(pose_distance(&plane, p), Some(-2.0));
        assert_eq!(pose_distance(&axis, p), Some(5.0));
        assert!(pose_distance(&point, p).is_some_and(|d| (d - 29.0_f64.sqrt()).abs() < 1e-12));
        assert_eq!(pose_distance(&PoseValue::Direction { dir: up }, p), None);
        for pose in [&plane, &axis, &point] {
            let d = pose_distance(pose, p).expect("a pose with a position");
            let sign = |value| pose_distance_sign(pose, p, value, band).expect("a position");
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
