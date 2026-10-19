//! **Values** (FORK-S3M; rulings C-1 and C-2): one freedom a mate
//! leaves, set to a number, charted on the two bodies' own frames.
//!
//! A mate's chart ([`Chart`]) is read once from its two poses and the
//! target body's frame: the mated direction `d` (the target's, its
//! flip normalised to the copy's side), each body's frame swung onto
//! `d` by the shortest turn ([`super::reference_about`]), and the
//! target body's origin. A value is then a function of the copy's
//! relative pose alone — a spin the angle about `d` from the target's
//! swung `x` to the copy's, a slide the copy's origin measured from
//! the target's along the target's swung axis — so no order of the
//! values is chosen.
//!
//! [`settle`] places a folded bundle's copy at its values: each spin
//! turns the copy about `d` inside the bundle's residual, then each
//! slide moves it along its own axis until its measure there is the
//! value. The bundle rule has
//! admitted each value's freedom as one the rest of the bundle leaves,
//! so nothing here asks whether the residual holds it. The values are
//! taken in one order whatever order they are listed in (spins, then
//! slides along `x`, `y`, `z`, each by its mate's id), so the listed
//! order moves no bit.

use std::collections::BTreeMap;

use geom_core::RigidMotion;
use geom_core::linalg::{Affine3, Mat3, Point3, UnitVec3, Vec3};
use geom_core::predicate::Band;

use super::coset::{Arm, Coset, Subgroup, after_turn};
use super::solve::SolveScalar;
use super::{Coord, MateFault, MateRef, SlideAxis};
use crate::NodeErrorKind;
use crate::node::ConstraintId;
use crate::pose::PoseValue;

/// **One mate's chart, in the target space** (ruling C-1).
#[derive(Debug, Clone, Copy)]
pub(crate) struct Chart<T: geom_core::Real> {
    /// The mated direction: the target's, its flip normalised to the
    /// copy's side.
    d: UnitVec3<T>,
    /// The target body's swung `x`, unit.
    x: Vec3<T>,
    /// The target body's swung `y`, `d × x`, unit.
    y: Vec3<T>,
    /// The target body's origin.
    origin: Point3<T>,
    /// The copy's swung `x`, in the copy's own coordinates.
    copy_x: Vec3<T>,
    /// The copy's mated direction, in the copy's own coordinates.
    copy_d: Vec3<T>,
    /// A point of the mate's own line, for an axis mate: the line its
    /// spin turns about.
    line: Option<Point3<T>>,
    /// The target body's own frame in the target space.
    frame: RigidMotion<T>,
    /// The mated direction in the target body's own coordinates.
    own_d: UnitVec3<T>,
    /// The target body's swung `x` in its own coordinates.
    own_x: Vec3<T>,
}

impl<T: SolveScalar> Chart<T> {
    /// **The chart of a mate equating `on` with `to`**, `to` read off a
    /// body whose own frame is `target` in the target space, its sense
    /// flipped when `flipped`. `None` for a point or a frame mate, which
    /// has no chart.
    ///
    /// Beside each body's swing (one [`super::reference_about`] verdict
    /// each), its three directions are minted as a datum's are
    /// (`DATUM_UNIT_NORM`), and every refusal is the chart's.
    ///
    /// # Errors
    ///
    /// The swing's own refusal ([`super::reference_about`]), or a
    /// direction the band cannot mint.
    pub(crate) fn of(
        on: &PoseValue<T>,
        to: &PoseValue<T>,
        own: Option<&Vec3<T>>,
        flipped: bool,
        frame: &RigidMotion<T>,
        arm: Arm,
        band: Band,
    ) -> Result<Option<Self>, NodeErrorKind> {
        let target = &frame.to_affine();
        let (to_dir, on_dir, line) = match (on, to) {
            (PoseValue::Plane { normal: o, .. }, PoseValue::Plane { normal: t, .. })
            | (PoseValue::Direction { dir: o }, PoseValue::Direction { dir: t }) => (*t, *o, None),
            (PoseValue::Axis { dir: o, .. }, PoseValue::Axis { origin, dir: t }) => {
                (*t, *o, Some(*origin))
            }
            _ => return Ok(None),
        };
        let (d, copy_d) = if flipped {
            (-to_dir, -on_dir)
        } else {
            (to_dir, on_dir)
        };
        let unit = |v: Vec3<T>| {
            UnitVec3::new(v, topo::query::DATUM_UNIT_NORM, band).map_err(|_| {
                NodeErrorKind::PoseDegenerate {
                    construction: crate::pose::PoseConstruction::Chart,
                }
            })
        };
        // The mated direction in the target body's own coordinates: on a
        // frame lane, read there where the target is read off a copy its
        // placement moved, which taking it back through the body's frame
        // would widen by the frame's width twice; else taken back.
        // The taken-back direction is decided unit on every lane, so a
        // frame lane decides what the value lane does while keeping its
        // own direction.
        let back = unit(target.linear.transpose() * d.get())?;
        let own_dir = own
            .filter(|_| T::spins_by_frames())
            .and_then(|n| super::solve::on_an_axis(*n));
        let own_d = match own_dir {
            Some(n) if flipped => -n,
            Some(n) => n,
            None => back,
        };
        // The swung `x` is the target's turn of a unit vector and `y` the
        // cross of two orthonormal ones, so each is unit over the reals
        // at every point a box spans: a frame lane keeps the image as it
        // is, which a normalization would widen by its own norm's width,
        // and decides each direction as the value lane does.
        let own_x = super::reference_about(own_d, arm.get(), band)?;
        let swung = target.linear * own_x;
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
            origin: Point3::origin() + target.translation,
            copy_x: super::reference_about(copy_d, arm.get(), band)?,
            copy_d: copy_d.get(),
            line,
            frame: *frame,
            own_d,
            own_x,
        }))
    }

    /// The spin of the copy at relative pose `m`: the angle about `d`
    /// from the target's swung `x` to the copy's, as `m` carries it.
    fn spin(&self, m: &Affine3<T>) -> T {
        let xc = m.linear * self.copy_x;
        let x = self.x;
        T::solve_atan2(x.cross(xc).dot(self.d.get()), x.dot(xc))
    }

    /// The turn carrying the copy's chart frame onto the target's
    /// turned `by` about `d`: the linear part of every map whose spin
    /// is `by`, read off the two charts alone.
    fn turned(&self, by: T) -> Mat3<T> {
        let (sin, cos) = by.sin_cos();
        let (x, y, d) = (self.x, self.y, self.d.get());
        let to_x = x * cos + y * sin;
        let to = Mat3::from_cols(to_x, d.cross(to_x), d);
        let from = Mat3::from_cols(self.copy_x, self.copy_d.cross(self.copy_x), self.copy_d);
        to * from.transpose()
    }

    /// **The constant turn carrying the copy's chart frame onto the
    /// target's**, both in their bodies' own coordinates.
    fn onto(&self) -> Affine3<T> {
        let own_y = self.own_d.get().cross(self.own_x);
        let to = Mat3::from_cols(self.own_x, own_y, self.own_d.get());
        let from = Mat3::from_cols(self.copy_x, self.copy_d.cross(self.copy_x), self.copy_d);
        Affine3::from_parts(to * from.transpose(), Vec3::zero())
    }

    /// **The copy's map at spin `by` and the translation `shift` (in
    /// the target body's own coordinates) before [`Self::onto`]**: the
    /// target body's frame, then the shift, then the turn by `by` about
    /// the mated direction, each composed as a motion, so nothing
    /// re-derives its rigidity from its entries and nothing is decided.
    fn head_at(&self, by: T, shift: Vec3<T>) -> Option<RigidMotion<T>> {
        let turn = RigidMotion::rotation(self.own_d, by).ok()?;
        self.frame
            .compose(RigidMotion::translation(shift).ok()?)
            .ok()?
            .compose(turn)
            .ok()
    }

    /// Whether the mated direction is an axis exactly, in the target
    /// space and in the target body's own coordinates.
    fn on_axes(&self) -> bool {
        super::solve::on_an_axis(self.d.get()).is_some()
            && super::solve::on_an_axis(self.own_d.get()).is_some()
    }

    /// The axis a slide runs along, in the target body's own
    /// coordinates.
    fn own_axis(&self, along: SlideAxis) -> Vec3<T> {
        match along {
            SlideAxis::X => self.own_x,
            SlideAxis::Y => self.own_d.get().cross(self.own_x),
            SlideAxis::Z => self.own_d.get(),
        }
    }

    /// The axis a slide runs along.
    fn axis(&self, along: SlideAxis) -> Vec3<T> {
        match along {
            SlideAxis::X => self.x,
            SlideAxis::Y => self.y,
            SlideAxis::Z => self.d.get(),
        }
    }

    /// **The value a spin takes at relative pose `m`** — what a gesture
    /// writes to put the copy at `m`.
    pub(crate) fn spin_at(&self, m: &Affine3<T>) -> T {
        self.spin(m)
    }

    /// **The value a slide along `along` takes at relative pose `m`**.
    pub(crate) fn slide_at(&self, m: &Affine3<T>, along: SlideAxis) -> T {
        self.slide(m, self.axis(along))
    }

    /// The slide of the copy at relative pose `m` along `axis`: its
    /// origin measured from the target's.
    fn slide(&self, m: &Affine3<T>, axis: Vec3<T>) -> T {
        ((Point3::origin() + m.translation) - self.origin).dot(axis)
    }
}

/// **One value, read**: its address, the mate it names, its freedom
/// and its number at the evaluation's scalar.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ReadValue<T> {
    /// The value's address.
    pub(crate) value: MateRef,
    /// The mate it names.
    pub(crate) mate: ConstraintId,
    /// Its freedom.
    pub(crate) coord: Coord,
    /// Its number.
    pub(crate) by: T,
}

/// **A settled map as the motions its composition made** (a frame
/// lane's one spin and its slides, [`settle`]): `head`, a motion, then
/// the constant turn `onto` carrying the copy's chart frame onto the
/// target's, held as a map until a placement asks for it as a motion
/// ([`Self::motion`]), so the decision of its rigidity is made where the
/// value lane decides its placed map's, and only there.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Composed<T: geom_core::Real> {
    /// The target body's frame, the shift and the spin.
    head: RigidMotion<T>,
    /// The constant turn.
    onto: Affine3<T>,
    /// The constant turn held with the identity, where the lane holds
    /// both ([`super::solve::SolveScalar::with_rest`]).
    at_rest: Option<Affine3<T>>,
}

impl<T: SolveScalar> Composed<T> {
    /// The map with `onto` as its constant turn.
    fn map(&self, onto: &Affine3<T>) -> Affine3<T> {
        self.head.to_affine() * *onto
    }

    /// The map with its constant turn held with the identity, where the
    /// lane holds both.
    pub(crate) fn at_rest(&self) -> Option<Affine3<T>> {
        self.at_rest.map(|k| self.map(&k))
    }

    /// **The placed map as a motion**: the constant turn (`at_rest`'s
    /// where `rest`) admitted by the decided door, then composed after
    /// the head. `None` where the door does not admit it.
    pub(crate) fn motion(&self, rest: bool, band: Band) -> Option<RigidMotion<T>> {
        let onto = if rest { self.at_rest? } else { self.onto };
        let onto = RigidMotion::check_rigid(&onto, band).ok()?;
        self.head.compose(onto).ok()
    }
}

/// **A folded bundle's copy placed at its values**: the relative pose
/// with each value's freedom set, inside what the mates leave. The
/// bundle rule has admitted every value's freedom
/// (`super::solve::overconstrained`), so each is held by the residual
/// by construction, and nothing here decides whether it is: a spin
/// turns about the residual's own line, or about `d` through the
/// copy's origin where every line along `d` will do; a slide moves the
/// copy along its own axis.
///
/// **Why a slide moves along its axis alone.** The rule admits a slide
/// only beside mates that free every translation
/// (`super::solve::own_freedom`), which only a direction mate does. A
/// direction mate beside any other mate repeats an equation and is
/// refused, but for a ball, beside which the slide itself is refused
/// (`NoChartWhy::SlideTurnsABall`). So every slide a bundle sets is on
/// one lone mate's chart, along its `x`, `y` or `d`, which are
/// perpendicular and lie in that mate's residual: one slide's move
/// changes no other's measure, and no line is left to mint between
/// them.
///
/// # Errors
///
/// [`MateFault::NoChart`] for a value whose mate has no chart.
pub(crate) fn settle<T: SolveScalar>(
    coset: Coset<T>,
    values: &[ReadValue<T>],
    charts: &BTreeMap<ConstraintId, Option<Chart<T>>>,
) -> Result<(Affine3<T>, Option<Composed<T>>), NodeErrorKind> {
    let mut m = coset.representative;
    let mut left = coset.subgroup;
    let chart = |v: &ReadValue<T>| match charts.get(&v.mate) {
        Some(Some(chart)) => Ok(*chart),
        _ => Err(NodeErrorKind::Mate(Box::new(MateFault::NoChart {
            value: v.value,
            why: super::NoChartWhy::PointRotation,
        }))),
    };
    // One order whatever order the values are listed in: spins, then
    // slides along x, y and z, each by its mate's id, then by its own.
    let mut ordered: Vec<&ReadValue<T>> = values.iter().collect();
    ordered.sort_by_key(|v| {
        let rank = match v.coord {
            Coord::Spin => 0,
            Coord::Slide(SlideAxis::X) => 1,
            Coord::Slide(SlideAxis::Y) => 2,
            Coord::Slide(SlideAxis::Z) => 3,
        };
        (rank, v.mate, v.value.id)
    });
    let (spins, slides): (Vec<_>, Vec<_>) =
        ordered.into_iter().partition(|v| v.coord == Coord::Spin);
    // The premise each slide's move rests on, held loud: a rule that came
    // to admit slides on two mates would otherwise place the copy off a
    // mate it holds, saying nothing.
    assert!(
        slides
            .windows(2)
            .all(|w: &[&ReadValue<T>]| w[0].mate == w[1].mate),
        "the bundle rule admits slides on one mate's chart alone"
    );
    let spun = spins.clone();
    // Spins first: every slide after them re-measures the copy's origin.
    for v in spins {
        let chart = chart(v)?;
        let by = v.by - chart.spin(&m);
        // A turn about the copy's own origin keeps its translation as it
        // is: composed as a turn about that point, a box lane would
        // widen it by the turn's own width levered over the origin's
        // distance, twice.
        let turned = match residual_turn_point(left).or(chart.line) {
            Some(at) => Affine3::rotation_about_axis(at, chart.d.get(), by) * m,
            None => {
                let turn = Affine3::rotation_about_axis(Point3::origin(), chart.d.get(), by);
                Affine3::from_parts(turn.linear * m.linear, m.translation)
            }
        };
        m = if T::spins_by_frames() {
            Affine3::from_parts(chart.turned(v.by), turned.translation)
        } else {
            turned
        };
        left = after_turn(left).unwrap_or(left);
    }
    // On a frame lane, a copy turned by one spin and slid on that
    // spin's chart is composed as the motion it is: the target body's
    // frame, then its values in the target's own coordinates (each
    // slide's component the value, any other the copy's measure there).
    // Only on a chart whose mated direction is an axis exactly, in the
    // target space and in the body's own: re-reading a component along
    // a direction the box widens would spread the translation's width
    // across the others, which the differences below leave in place.
    if T::spins_by_frames()
        && let [spin] = spun.as_slice()
        && slides.first().is_some_and(|v| v.mate == spin.mate)
        && chart(spin)?.on_axes()
    {
        let chart = chart(spin)?;
        let own = chart.frame.to_affine();
        let rel = own.linear.transpose() * (m.translation - own.translation);
        let along = |a: SlideAxis| {
            slides
                .iter()
                .rev()
                .find(|v| v.coord == Coord::Slide(a))
                .map_or_else(|| rel.dot(chart.own_axis(a)), |v| v.by)
        };
        let shift = [SlideAxis::X, SlideAxis::Y, SlideAxis::Z]
            .into_iter()
            .fold(Vec3::zero(), |t, a| t + chart.own_axis(a) * along(a));
        if let Some(head) = chart.head_at(spin.by, shift) {
            let onto = chart.onto();
            // At rest the map is `onto` alone, so a box holding the rest
            // holds `onto` with the identity.
            let composed = Composed {
                head,
                onto,
                at_rest: T::with_rest(&onto),
            };
            return Ok((composed.map(&onto), Some(composed)));
        }
    }
    // On a frame lane the translation is set from the chart's frame,
    // as the turn is: its component along each slide's axis is the
    // value, and along the others it is the copy's measure there, so
    // no difference of the translation with itself widens it.
    if T::spins_by_frames()
        && let Some(first) = slides.first()
        && chart(first)?.on_axes()
    {
        let chart = chart(first)?;
        let along = |a: SlideAxis| {
            slides
                .iter()
                .rev()
                .find(|v| v.coord == Coord::Slide(a))
                .map_or_else(|| chart.slide(&m, chart.axis(a)), |v| v.by)
        };
        let shift = [SlideAxis::X, SlideAxis::Y, SlideAxis::Z]
            .into_iter()
            .fold(chart.origin - Point3::origin(), |t, a| {
                t + chart.axis(a) * along(a)
            });
        return Ok((Affine3::from_parts(m.linear, shift), None));
    }
    for v in slides {
        let chart = chart(v)?;
        let Coord::Slide(along) = v.coord else {
            unreachable!("the spins were taken above")
        };
        let axis = chart.axis(along);
        let gap = v.by - chart.slide(&m, axis);
        m = Affine3::from_parts(Mat3::identity(), axis * gap) * m;
    }
    Ok((m, None))
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
