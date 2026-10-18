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
//! slide moves it, by the least translation the residual still holds,
//! until its measure along its axis is the value. A turn the residual
//! does not hold, or a slide whose axis the residual's translations do
//! not reach, is a freedom the bundle's other constraints already fix
//! ([`MateFault::NotAFreedom`] with no kind), the placement's own
//! refusal until F's door (the named interim C-3). The values are taken
//! in one order whatever order they are listed in (spins, then slides
//! along `x`, `y`, `z`, each by its mate's id), so the listed order
//! moves no bit.

use std::collections::BTreeMap;

use geom_core::linalg::{Affine3, Mat3, Point3, UnitVec3, Vec3};
use geom_core::predicate::{Band, Indeterminate};

use super::coset::{Arm, Coset, Subgroup, after_turn, turn_line};
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
    /// The target body's swung `x`.
    x: UnitVec3<T>,
    /// The target body's swung `y`, `d × x`.
    y: UnitVec3<T>,
    /// The target body's origin.
    origin: Point3<T>,
    /// The copy's swung `x`, in the copy's own coordinates.
    copy_x: Vec3<T>,
    /// A point of the mate's own line, for an axis mate: the line its
    /// spin turns about.
    line: Option<Point3<T>>,
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
        flipped: bool,
        target: &Affine3<T>,
        arm: Arm,
        band: Band,
    ) -> Result<Option<Self>, NodeErrorKind> {
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
        let own_d = unit(target.linear.transpose() * d.get())?;
        let x = unit(target.linear * super::reference_about(own_d, arm.get(), band)?)?;
        let y = unit(d.get().cross(x.get()))?;
        Ok(Some(Self {
            d,
            x,
            y,
            origin: Point3::origin() + target.translation,
            copy_x: super::reference_about(copy_d, arm.get(), band)?,
            line,
        }))
    }

    /// The spin of the copy at relative pose `m`: the angle about `d`
    /// from the target's swung `x` to the copy's, as `m` carries it.
    fn spin(&self, m: &Affine3<T>) -> T {
        let xc = m.linear * self.copy_x;
        let x = self.x.get();
        T::solve_atan2(x.cross(xc).dot(self.d.get()), x.dot(xc))
    }

    /// The axis a slide runs along.
    fn axis(&self, along: SlideAxis) -> UnitVec3<T> {
        match along {
            SlideAxis::X => self.x,
            SlideAxis::Y => self.y,
            SlideAxis::Z => self.d,
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
    fn slide(&self, m: &Affine3<T>, axis: UnitVec3<T>) -> T {
        ((Point3::origin() + m.translation) - self.origin).dot(axis.get())
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

/// **A folded bundle's copy placed at its values**: `Some` the relative
/// pose once its mates and values leave nothing free, `None` for a
/// loose copy.
///
/// # Errors
///
/// [`MateFault::NotAFreedom`] (no kind) for a value whose turn or slide
/// the residual does not hold, [`MateFault::NoChart`] for one whose
/// mate has no chart, and [`MateFault::Indeterminate`] for a verdict
/// in the band.
pub(crate) fn settle<T: SolveScalar>(
    coset: Coset<T>,
    values: &[ReadValue<T>],
    charts: &BTreeMap<ConstraintId, Option<Chart<T>>>,
    arm: Arm,
    band: Band,
) -> Result<Option<Affine3<T>>, Box<MateFault>> {
    let mut m = coset.representative;
    let mut left = coset.subgroup;
    let indeterminate = |value: MateRef| {
        move |diag: Indeterminate| {
            Box::new(MateFault::Indeterminate {
                mate: value,
                diag: Box::new(diag),
            })
        }
    };
    let not_a_freedom = |v: &ReadValue<T>| {
        Box::new(MateFault::NotAFreedom {
            value: v.value,
            coord: v.coord,
            kind: None,
        })
    };
    let chart = |v: &ReadValue<T>| match charts.get(&v.mate) {
        Some(Some(chart)) => Ok(*chart),
        _ => Err(Box::new(MateFault::NoChart {
            value: v.value,
            why: super::NoChartWhy::PointRotation,
        })),
    };
    // One order whatever order the values are listed in: spins, then
    // slides along x, y and z, each by its mate's id, then by its own
    // (two values of one freedom on one mate: the second, which the
    // fold refuses, is the same value whichever is listed first).
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
    // Spins first: every slide after them re-measures the copy's origin.
    for v in spins {
        let chart = chart(v)?;
        let Some(line) = turn_line(left, chart.d, band, arm).map_err(indeterminate(v.value))?
        else {
            return Err(not_a_freedom(v));
        };
        let by = v.by - chart.spin(&m);
        // A turn about the copy's own origin keeps its translation as it
        // is: composed as a turn about that point, a box lane would
        // widen it by the turn's own width levered over the origin's
        // distance, twice.
        m = match line.or(chart.line) {
            Some(at) => Affine3::rotation_about_axis(at, chart.d.get(), by) * m,
            None => {
                let turn = Affine3::rotation_about_axis(Point3::origin(), chart.d.get(), by);
                Affine3::from_parts(turn.linear * m.linear, m.translation)
            }
        };
        // A turn the table holds no rest of (a ball's, less one) leaves
        // a turn free beside it: no value can pin what remains, and a
        // second turn is not charted on it.
        let Some(rest) = after_turn(left) else {
            return Err(not_a_freedom(v));
        };
        left = rest;
    }
    let turns_fixed = matches!(
        left,
        Subgroup::PlaneTranslation { .. }
            | Subgroup::Prismatic { .. }
            | Subgroup::Translation
            | Subgroup::Trivial
    );
    let mut free = Translations::of(left);
    for v in slides {
        let chart = chart(v)?;
        let Coord::Slide(along) = v.coord else {
            unreachable!("the spins were taken above")
        };
        let axis = chart.axis(along);
        let gap = v.by - chart.slide(&m, axis);
        let (shift, rest) = free
            .slide(axis, gap, band, arm)
            .map_err(indeterminate(v.value))?
            .ok_or_else(|| not_a_freedom(v))?;
        m = Affine3::from_parts(Mat3::identity(), shift) * m;
        free = rest;
    }
    Ok((turns_fixed && matches!(free, Translations::None)).then_some(m))
}

/// **The translations a residual still holds**, apart from its turns:
/// every one, those across a normal (a plane of them), those along a
/// direction (a line of them), or none.
#[derive(Debug, Clone, Copy)]
enum Translations<T: geom_core::Real> {
    Every,
    Across(UnitVec3<T>),
    Along(UnitVec3<T>),
    None,
}

impl<T: SolveScalar> Translations<T> {
    fn of(g: Subgroup<T>) -> Self {
        match g {
            Subgroup::Se3 | Subgroup::Parallel { .. } | Subgroup::Translation => Self::Every,
            Subgroup::Planar { normal } | Subgroup::PlaneTranslation { normal } => {
                Self::Across(normal)
            }
            Subgroup::Cylindrical { direction, .. } | Subgroup::Prismatic { direction } => {
                Self::Along(direction)
            }
            Subgroup::Revolute { .. }
            | Subgroup::Spherical { .. }
            | Subgroup::Trivial
            | Subgroup::Empty => Self::None,
        }
    }

    /// **The least translation these hold that moves a measure along
    /// `axis` by `gap`**, and the translations left that keep it: `None`
    /// where none of these reaches `axis` (every one is perpendicular to
    /// it). The one divisor is the share of `axis` these reach, decided
    /// away from zero over `arm`.
    #[allow(clippy::type_complexity)]
    fn slide(
        self,
        axis: UnitVec3<T>,
        gap: T,
        band: Band,
        arm: Arm,
    ) -> Result<Option<(Vec3<T>, Self)>, Indeterminate> {
        let a = axis.get();
        Ok(match self {
            Self::Every => Some((a * gap, Self::Across(axis))),
            Self::Across(n) => {
                // The line the plane keeps once `a`'s measure is fixed:
                // across `a` and `n`, none where they are parallel.
                let Some(left) = super::coset::across(axis, n, band, arm)? else {
                    return Ok(None);
                };
                // `a` with its share along `n` taken out: the plane's
                // nearest translation to `a`, whose own measure along `a`
                // is its squared length.
                let reach = a - n.get() * a.dot(n.get());
                Some((reach * (gap / reach.dot(a)), Self::Along(left)))
            }
            Self::Along(r) => {
                if !super::coset::reaches(axis, r, band, arm)? {
                    return Ok(None);
                }
                Some((r.get() * (gap / r.get().dot(a)), Self::None))
            }
            Self::None => None,
        })
    }
}
