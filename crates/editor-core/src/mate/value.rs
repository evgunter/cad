//! **Values** (FORK-S3M; D10 `[placement.values]`, row 190): one
//! freedom a bundle's mates leave, set to a number, charted on the
//! target space's axes.
//!
//! A value names no mate. Where a mate fixes a direction, its chart
//! ([`Chart`]) is read once from that mate's two poses: the mated
//! direction `d` (the target's, its flip normalised to the copy's
//! side), the target space's axes swung onto `d` by the shortest turn
//! ([`super::reference_about`]) and the copy's own axes swung onto its
//! side of the mate the same way. A value is then a function of the
//! copy's relative pose alone — `Spin(Z)` the angle about `d` from the
//! target space's swung `x` to the copy's, a slide the copy's origin
//! measured from the target space's along a swung axis — so no order
//! of the values is chosen, and nothing about the body the target
//! pose is read off enters the chart.
//!
//! The shortest turn has no limit at its antipode: it swings `z` onto
//! `d` about `z × d`, which turns as fast as `d` nears `−z`, so there a
//! tilt of `d` by `δ` turns the swung `x` by an angle of order
//! `δ / |d + z|`, up to a half turn, and every slide measured along it
//! with it — a copy hung under a face whose normal is near `−z` can
//! move far for a tilt of its target by a sliver. Inside the band about
//! the pole the chart refuses.
//!
//! [`settle`] places a folded bundle's copy at its values: each spin
//! turns the copy about `d` inside the bundle's residual, then each
//! slide moves it along its own axis until its measure there is the
//! value. The bundle rule has admitted each value's freedom as one the
//! mates leave, so nothing here asks whether the residual holds it. The
//! values are taken in one order whatever order they are listed in
//! (spins, then slides along `x`, `y`, `z`), so the listed order moves
//! no bit. Where no direction is fixed, the spins are a chain the
//! author orders ([`chain`]): about the copy's origin with no mate
//! ([`unmated_motion`]), about the mated point beside a lone point
//! mate.

use geom_core::linalg::{Affine3, Mat3, Point3, UnitVec3, Vec3};
use geom_core::predicate::Band;
use geom_core::{Angle, RigidMotion};

use super::coset::{Arm, Coset, Subgroup, after_turn};
use super::solve::SolveScalar;
use super::{Coord, MateRef, TargetAxis};
use crate::NodeErrorKind;
use crate::pose::PoseValue;

/// **The chart of the one mate fixing a direction, in the target
/// space** (row 190).
#[derive(Debug, Clone, Copy)]
pub(crate) struct Chart<T: geom_core::Real> {
    /// The mated direction: the target's, its flip normalised to the
    /// copy's side.
    d: UnitVec3<T>,
    /// The target space's swung `x`, unit.
    x: Vec3<T>,
    /// The target space's swung `y`, `d × x`, unit.
    y: Vec3<T>,
    /// The copy's swung `x`, in the copy's own coordinates.
    copy_x: Vec3<T>,
    /// The copy's mated direction, in the copy's own coordinates.
    copy_d: Vec3<T>,
    /// A point of the mate's own line, for an axis mate: the line its
    /// spin turns about.
    line: Option<Point3<T>>,
}

impl<T: SolveScalar> Chart<T> {
    /// **The chart of a mate equating `on` with `to`**, both read in
    /// the target space, `to`'s sense flipped when `flipped`. `None` for
    /// a point or a frame mate, which fixes no one direction.
    ///
    /// Beside each side's swing (one [`super::reference_about`]
    /// verdict each), its three directions are minted as a pose's are
    /// (`POSE_UNIT_NORM`), and every refusal is the chart's.
    ///
    /// # Errors
    ///
    /// The swing's own refusal ([`super::reference_about`]), or a
    /// direction the band cannot mint.
    pub(crate) fn of(
        on: &PoseValue<T>,
        to: &PoseValue<T>,
        flipped: bool,
        arm: Arm,
        band: Band,
    ) -> Result<Option<Self>, NodeErrorKind> {
        let (to_dir, on_dir, line) = match (on, to) {
            (PoseValue::Plane { normal: o, .. }, PoseValue::Plane { normal: t, .. })
            | (PoseValue::Direction { dir: o }, PoseValue::Direction { dir: t }) => (*t, *o, None),
            (PoseValue::Axis { dir: o, .. }, PoseValue::Axis { foot, dir: t, .. }) => {
                (*t, *o, Some(*foot))
            }
            _ => return Ok(None),
        };
        let (d, copy_d) = if flipped {
            (-to_dir, -on_dir)
        } else {
            (to_dir, on_dir)
        };
        let unit = |v: Vec3<T>| {
            UnitVec3::new(v, topo::query::POSE_UNIT_NORM, band).map_err(|_| {
                NodeErrorKind::PoseDegenerate {
                    construction: crate::pose::PoseConstruction::Chart,
                }
            })
        };
        // The swung `x` is a turn of a unit vector and `y` the cross of
        // two orthonormal ones, so each is unit over the reals at every
        // point a box spans: a frame lane keeps each as it is, which a
        // normalization would widen by its own norm's width, and decides
        // each direction as the value lane does.
        let swung = super::reference_about(d, arm.get(), band)?;
        let x = unit(swung)?;
        let x = if T::spins_by_frames() { swung } else { x.get() };
        let across = d.get().cross(x);
        let y = unit(across)?;
        let y = if T::spins_by_frames() {
            across
        } else {
            y.get()
        };
        Ok(Some(Self {
            d,
            x,
            y,
            copy_x: super::reference_about(copy_d, arm.get(), band)?,
            copy_d: copy_d.get(),
            line,
        }))
    }

    /// The spin of the copy at relative pose `m`: the angle about `d`
    /// from the target space's swung `x` to the copy's, as `m` carries
    /// it.
    fn spin(&self, m: &Affine3<T>) -> T {
        let xc = m.linear * self.copy_x;
        let x = self.x;
        T::solve_atan2(x.cross(xc).dot(self.d.get()), x.dot(xc))
    }

    /// The turn carrying the copy's chart frame onto the target space's
    /// turned `by` about `d`: the linear part of every map whose spin is
    /// `by`, read off the two charts alone.
    fn turned(&self, by: Angle<T>) -> Mat3<T> {
        let (sin, cos) = by.sin_cos();
        let (x, y, d) = (self.x, self.y, self.d.get());
        let to_x = x * cos + y * sin;
        let to = Mat3::from_cols(to_x, d.cross(to_x), d);
        to * self.from().transpose()
    }

    /// The copy's chart frame, in its own coordinates.
    fn from(&self) -> Mat3<T> {
        Mat3::from_cols(self.copy_x, self.copy_d.cross(self.copy_x), self.copy_d)
    }

    /// **The constant turn carrying the copy's chart frame onto the
    /// target space's swung one.**
    fn onto(&self) -> Affine3<T> {
        let to = Mat3::from_cols(self.x, self.y, self.d.get());
        Affine3::from_parts(to * self.from().transpose(), Vec3::zero())
    }

    /// **The copy's map at spin `by` and the translation `shift` before
    /// [`Self::onto`]**: the shift, then the turn by `by` about the
    /// mated direction, each composed as a motion, so nothing re-derives
    /// its rigidity from its entries and nothing is decided.
    fn head_at(&self, by: Angle<T>, shift: Vec3<T>) -> Option<RigidMotion<T>> {
        let turn = RigidMotion::rotation_by(self.d, by).ok()?;
        RigidMotion::translation(shift).ok()?.compose(turn).ok()
    }

    /// Whether the mated direction is an axis of the target space
    /// exactly.
    fn on_axes(&self) -> bool {
        super::solve::on_an_axis(self.d.get()).is_some()
    }

    /// The axis a slide runs along.
    fn axis(&self, along: TargetAxis) -> Vec3<T> {
        match along {
            TargetAxis::X => self.x,
            TargetAxis::Y => self.y,
            TargetAxis::Z => self.d.get(),
        }
    }

    /// The slide of the copy at relative pose `m` along `axis`: its
    /// origin measured from the target space's.
    fn slide(m: &Affine3<T>, axis: Vec3<T>) -> T {
        m.translation.dot(axis)
    }
}

/// **One value, read**: its address, its freedom and its number at
/// the evaluation's scalar.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ReadValue<T> {
    /// The value's address.
    pub(crate) value: MateRef,
    /// Its freedom.
    pub(crate) coord: Coord,
    /// Its number.
    pub(crate) by: T,
    /// The fraction of a turn a spin's number is written as, where it
    /// is one ([`crate::expr::VarEnv::turns`]).
    pub(crate) turns: Option<crate::expr::Ratio>,
}

impl<T: geom_core::Real> ReadValue<T> {
    /// A spin's number as an angle, with the form it was written in.
    fn angle(&self) -> Angle<T> {
        crate::expr::angle_of(self.by, self.turns)
    }
}

/// **A settled map as the motions its composition made** (a frame
/// lane's one spin and its slides, [`settle`]): `head`, a motion, then
/// the constant turn `onto` carrying the copy's chart frame onto the
/// target's, held as a map until a placement asks for it as a motion
/// ([`Self::motion`]), so its rigidity is decided by the one door the
/// value lane asks of its placed map ([`super::solve::admitted`]), and
/// only there.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Composed<T: geom_core::Real> {
    /// The shift, then the spin about the mated direction
    /// ([`Chart::head_at`]).
    head: RigidMotion<T>,
    /// The constant turn.
    onto: Affine3<T>,
}

impl<T: SolveScalar> Composed<T> {
    /// The map the composition places by.
    fn map(&self) -> Affine3<T> {
        self.head.to_affine() * self.onto
    }

    /// **The placed map as a motion**: the constant turn admitted by
    /// the door every placed map passes ([`super::solve::admitted`]),
    /// then composed after the head.
    ///
    /// # Errors
    ///
    /// The door's refusal, or a composed origin that overflows.
    pub(crate) fn motion(&self, band: Band) -> Result<RigidMotion<T>, NodeErrorKind> {
        let onto = super::solve::admitted(&self.onto, band)?;
        self.head.compose(onto).map_err(NodeErrorKind::Motion)
    }
}

/// **A chain of spins about the target space's own axes** (D10
/// `[placement.values]`), each turning about the origin, in the order
/// the bundle lists them: the first written is the first made. Composed
/// as motions, so nothing is decided; `None` for a chain of no link.
///
/// # Errors
///
/// [`NodeErrorKind::Motion`]: an angle that is no bounded number.
fn chain<T: SolveScalar>(values: &[ReadValue<T>]) -> Result<Option<RigidMotion<T>>, NodeErrorKind> {
    let mut turned: Option<RigidMotion<T>> = None;
    for v in values {
        if let Coord::Spin(axis) = v.coord {
            let spin =
                RigidMotion::rotation_by(axis.unit(), v.angle()).map_err(NodeErrorKind::Motion)?;
            turned = Some(match turned {
                Some(inner) => spin.compose(inner).map_err(NodeErrorKind::Motion)?,
                None => spin,
            });
        }
    }
    Ok(turned)
}

/// **A bundle with no mate placed by its values** (D10
/// `[placement.values]`): the chain of spins about the target space's
/// own axes ([`chain`]), each turning the copy about its own origin,
/// then the slides along those axes, which set the copy's origin.
/// Composed as motions, so nothing is decided. The bundle rule has
/// admitted the values as the six freedoms, each once.
///
/// # Errors
///
/// [`NodeErrorKind::Motion`]: an angle or a translation that is no
/// bounded number.
pub(crate) fn unmated_motion<T: SolveScalar>(
    values: &[ReadValue<T>],
) -> Result<RigidMotion<T>, NodeErrorKind> {
    let mut shift = Vec3::zero();
    for v in values {
        match v.coord {
            Coord::Slide(TargetAxis::X) => shift.x = v.by,
            Coord::Slide(TargetAxis::Y) => shift.y = v.by,
            Coord::Slide(TargetAxis::Z) => shift.z = v.by,
            Coord::Spin(_) => {}
        }
    }
    let slid = RigidMotion::translation(shift).map_err(NodeErrorKind::Motion)?;
    match chain(values)? {
        Some(turned) => slid.compose(turned).map_err(NodeErrorKind::Motion),
        None => Ok(slid),
    }
}

/// **A lone point mate's copy placed by its chain** (D10
/// `[placement.values]`): the copy's mated point `on` carried onto the
/// target's `to`, turned about it by the chain of spins about the target
/// space's axes in the order written ([`chain`]), so a chain of zeros
/// leaves the copy's own axes the target space's. Composed as motions.
///
/// # Errors
///
/// [`NodeErrorKind::Motion`]: an angle or a point that is no bounded
/// number.
pub(crate) fn point_motion<T: SolveScalar>(
    on: Point3<T>,
    to: Point3<T>,
    values: &[ReadValue<T>],
) -> Result<RigidMotion<T>, NodeErrorKind> {
    let motion = |e| NodeErrorKind::Motion(e);
    let into = RigidMotion::translation(to - Point3::origin()).map_err(motion)?;
    let out = RigidMotion::translation(Point3::origin() - on).map_err(motion)?;
    let turned = match chain(values)? {
        Some(turned) => into.compose(turned).map_err(motion)?,
        None => into,
    };
    turned.compose(out).map_err(motion)
}

/// **A folded bundle's copy placed at its values**: the relative pose
/// with each value's freedom set, inside what the mates leave, on the
/// chart of the one mate fixing a direction. The bundle rule has
/// admitted every value's freedom (`super::solve::overconstrained`), so
/// each is held by the residual by construction, and nothing here
/// decides whether it is: a spin turns about the residual's own line,
/// or about `d` through the copy's origin where every line along `d`
/// will do; a slide moves the copy along its own axis.
///
/// **Why a slide moves along its axis alone.** The rule admits a slide
/// only beside mates that free every translation along it, so every
/// slide a bundle sets is along the chart's `x`, `y` or `d`, which are
/// perpendicular and lie in the residual: one slide's move changes no
/// other's measure, and no line is left to mint between them.
///
/// # Errors
///
/// A chart degenerate at its values.
pub(crate) fn settle<T: SolveScalar>(
    coset: Coset<T>,
    values: &[ReadValue<T>],
    chart: Option<Chart<T>>,
) -> Result<(Affine3<T>, Option<Composed<T>>), NodeErrorKind> {
    let mut m = coset.representative;
    let mut left = coset.subgroup;
    if values.is_empty() {
        return Ok((m, None));
    }
    let Some(chart) = chart else {
        unreachable!("the bundle rule admits a value beside a mate fixing a direction alone")
    };
    // One order whatever order the values are listed in: spins, then
    // slides along x, y and z, then by id.
    let mut ordered: Vec<&ReadValue<T>> = values.iter().collect();
    ordered.sort_by_key(|v| {
        let rank = match v.coord {
            Coord::Spin(_) => 0,
            Coord::Slide(TargetAxis::X) => 1,
            Coord::Slide(TargetAxis::Y) => 2,
            Coord::Slide(TargetAxis::Z) => 3,
        };
        (rank, v.value.id)
    });
    let (spins, slides): (Vec<_>, Vec<_>) = ordered
        .into_iter()
        .partition(|v| matches!(v.coord, Coord::Spin(_)));
    // The rule admits a spin about the mated direction alone here, held
    // loud: the chart turns about `d`.
    assert!(
        spins
            .iter()
            .all(|v: &&ReadValue<T>| v.coord == Coord::Spin(TargetAxis::Z)),
        "a mate's chart is spun about its mated direction alone"
    );
    let spun = spins.clone();
    // Spins first: every slide after them re-measures the copy's origin.
    for v in spins {
        let measured = chart.spin(&m);
        // A turn about the copy's own origin keeps its translation as it
        // is: composed as a turn about that point, a box lane would
        // widen it by the turn's own width levered over the origin's
        // distance, twice.
        let turned = match residual_turn_point(left).or(chart.line) {
            Some(at) => turn_to(at, chart.d, v, measured) * m,
            None => {
                let turn = turn_to(Point3::origin(), chart.d, v, measured);
                Affine3::from_parts(turn.linear * m.linear, m.translation)
            }
        };
        m = if T::spins_by_frames() {
            Affine3::from_parts(chart.turned(v.angle()), turned.translation)
        } else {
            turned
        };
        left = after_turn(left).unwrap_or(left);
    }
    // On a frame lane the translation is set from the chart, as the turn
    // is: its component along each slide's axis is the value, and along
    // the others it is the copy's measure there, so no difference of the
    // translation with itself widens it. Only on a chart whose mated
    // direction is an axis exactly: re-reading a component along a
    // direction the box widens would spread the translation's width
    // across the others, which the differences below leave in place.
    if T::spins_by_frames() && chart.on_axes() && !slides.is_empty() {
        let along = |a: TargetAxis| {
            slides
                .iter()
                .rev()
                .find(|v| v.coord == Coord::Slide(a))
                .map_or_else(|| Chart::slide(&m, chart.axis(a)), |v| v.by)
        };
        let shift = TargetAxis::ALL
            .into_iter()
            .fold(Vec3::zero(), |t, a| t + chart.axis(a) * along(a));
        // A copy turned by one spin is composed as the motion it is:
        // the shift, then the spin, then the constant turn.
        if let [spin] = spun.as_slice()
            && let Some(head) = chart.head_at(spin.angle(), shift)
        {
            let composed = Composed {
                head,
                onto: chart.onto(),
            };
            return Ok((composed.map(), Some(composed)));
        }
        return Ok((Affine3::from_parts(m.linear, shift), None));
    }
    for v in slides {
        let Coord::Slide(along) = v.coord else {
            unreachable!("the spins were taken above")
        };
        let axis = chart.axis(along);
        let gap = v.by - Chart::slide(&m, axis);
        m = Affine3::from_parts(Mat3::identity(), axis * gap) * m;
    }
    Ok((m, None))
}
/// **The turn about the line through `at` along `d` that takes a copy
/// spun `measured` to `v`'s spin**: by the difference of the two, or,
/// for a spin written as a fraction of a turn, the measured spin undone
/// and then the written turn made, each from its own sine and cosine,
/// so the written one is exact where its form is. A measured spin that
/// is exactly zero has nothing to undo.
fn turn_to<T: SolveScalar>(
    at: Point3<T>,
    d: UnitVec3<T>,
    v: &ReadValue<T>,
    measured: T,
) -> Affine3<T> {
    match v.turns {
        Some(_) if measured.exactly_zero() => {
            Affine3::rotation_about_unit_axis_by(at, d, v.angle())
        }
        Some(_) => {
            Affine3::rotation_about_unit_axis_by(at, d, v.angle())
                * Affine3::rotation_about_unit_axis_by(at, d, Angle::radians(-measured))
        }
        None => Affine3::rotation_about_axis(at, d.get(), v.by - measured),
    }
}

/// **The line a residual's turns about its direction fix**: the
/// residual's own for a line's or a ball's group, none for a group
/// freeing every translation across its direction.
fn residual_turn_point<T: geom_core::Real>(g: Subgroup<T>) -> Option<Point3<T>> {
    match g {
        Subgroup::Cylindrical { point, .. }
        | Subgroup::Revolute { point, .. }
        | Subgroup::Spherical { point } => Some(point),
        Subgroup::Se3
        | Subgroup::Planar { .. }
        | Subgroup::Parallel { .. }
        | Subgroup::Prismatic { .. }
        | Subgroup::Translation
        | Subgroup::PlaneTranslation { .. }
        | Subgroup::Trivial => None,
    }
}
