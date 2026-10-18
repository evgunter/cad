//! **Values** (FORK-S3M; rulings C-1 and C-2): one freedom a mate
//! leaves, set to a number, charted on the two bodies' own frames.
//!
//! A mate's chart ([`Chart`]) is read once from its two poses and the
//! target body's frame: the mated direction `d` (the target's, its
//! flip normalised to the copy's side), each body's frame swung onto
//! `d` by the shortest turn ([`super::reference_about`]), and the
//! target body's origin. A swing the band cannot decide refuses only
//! the values that read it: a slide along `d` reads neither, a slide
//! across it the target's, a spin both. A value is then a function of the copy's
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
    /// The target body's swung `x` and `y` (`d × x`), or `None` where
    /// the band could not decide the swing: what a slide across `d`
    /// and a spin read.
    across: Option<(UnitVec3<T>, UnitVec3<T>)>,
    /// The target body's origin.
    origin: Point3<T>,
    /// The copy's swung `x`, in the copy's own coordinates, or `None`
    /// where the band could not decide the swing: what a spin reads.
    copy_x: Option<Vec3<T>>,
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
    /// (`DATUM_UNIT_NORM`), and every refusal is the chart's. A swing
    /// is decided here and refused only where a value reads it
    /// ([`Chart::spin_at`], [`Chart::slide_at`]), so building a chart
    /// refuses nothing.
    pub(crate) fn of(
        on: &PoseValue<T>,
        to: &PoseValue<T>,
        flipped: bool,
        target: &Affine3<T>,
        arm: Arm,
        band: Band,
    ) -> Option<Self> {
        let (to_dir, on_dir, line) = match (on, to) {
            (PoseValue::Plane { normal: o, .. }, PoseValue::Plane { normal: t, .. })
            | (PoseValue::Direction { dir: o }, PoseValue::Direction { dir: t }) => (*t, *o, None),
            (PoseValue::Axis { dir: o, .. }, PoseValue::Axis { origin, dir: t }) => {
                (*t, *o, Some(*origin))
            }
            _ => return None,
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
        let across = (|| {
            let own_d = unit(target.linear.transpose() * d.get())?;
            let x = unit(target.linear * super::reference_about(own_d, arm.get(), band)?)?;
            let y = unit(d.get().cross(x.get()))?;
            Ok::<_, NodeErrorKind>((x, y))
        })()
        .ok();
        Some(Self {
            d,
            across,
            origin: Point3::origin() + target.translation,
            copy_x: super::reference_about(copy_d, arm.get(), band).ok(),
            line,
        })
    }

    /// A swing the band could not decide, as a value that reads it
    /// refuses.
    fn undecided() -> NodeErrorKind {
        NodeErrorKind::PoseDegenerate {
            construction: crate::pose::PoseConstruction::Chart,
        }
    }

    /// The spin of the copy at relative pose `m`: the angle about `d`
    /// from the target's swung `x` to the copy's, as `m` carries it.
    fn spin(&self, m: &Affine3<T>) -> Result<T, NodeErrorKind> {
        let ((x, _), copy_x) = self.across.zip(self.copy_x).ok_or_else(Self::undecided)?;
        let xc = m.linear * copy_x;
        let x = x.get();
        Ok(T::solve_atan2(x.cross(xc).dot(self.d.get()), x.dot(xc)))
    }

    /// The axis a slide runs along: `d` itself, or the target's swung
    /// `x` or `y`.
    fn axis(&self, along: SlideAxis) -> Result<UnitVec3<T>, NodeErrorKind> {
        match along {
            SlideAxis::Z => Ok(self.d),
            SlideAxis::X | SlideAxis::Y => {
                let (x, y) = self.across.ok_or_else(Self::undecided)?;
                Ok(if along == SlideAxis::X { x } else { y })
            }
        }
    }

    /// **The value a spin takes at relative pose `m`** — what a gesture
    /// writes to put the copy at `m`.
    ///
    /// # Errors
    ///
    /// [`NodeErrorKind::PoseDegenerate`] (`Chart`) where either body's
    /// swing is undecided at the band.
    pub(crate) fn spin_at(&self, m: &Affine3<T>) -> Result<T, NodeErrorKind> {
        self.spin(m)
    }

    /// **The value a slide along `along` takes at relative pose `m`**.
    ///
    /// # Errors
    ///
    /// [`NodeErrorKind::PoseDegenerate`] (`Chart`) for a slide across
    /// `d` where the target's swing is undecided at the band.
    pub(crate) fn slide_at(&self, m: &Affine3<T>, along: SlideAxis) -> Result<T, NodeErrorKind> {
        Ok(self.slide(m, self.axis(along)?))
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
/// (`super::solve::own_freedom`), which only a direction mate does,
/// and a direction mate beside any other mate overconstrains it. So
/// every slide a bundle sets is on one mate's chart, along `x`, `y` or
/// `d`, which are perpendicular, and the residual holds each: one
/// slide's move changes no other's measure, and no line is left to
/// mint between them.
///
/// # Errors
///
/// [`MateFault::NoChart`] for a value whose mate has no chart, and
/// [`NodeErrorKind::PoseDegenerate`] (`Chart`) for a value that reads a
/// swing the band cannot decide.
pub(crate) fn settle<T: SolveScalar>(
    coset: Coset<T>,
    values: &[ReadValue<T>],
    charts: &BTreeMap<ConstraintId, Option<Chart<T>>>,
) -> Result<Affine3<T>, NodeErrorKind> {
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
    // Spins first: every slide after them re-measures the copy's origin.
    for v in spins {
        let chart = chart(v)?;
        let by = v.by - chart.spin(&m)?;
        // A turn about the copy's own origin keeps its translation as it
        // is: composed as a turn about that point, a box lane would
        // widen it by the turn's own width levered over the origin's
        // distance, twice.
        m = match residual_turn_point(left).or(chart.line) {
            Some(at) => Affine3::rotation_about_axis(at, chart.d.get(), by) * m,
            None => {
                let turn = Affine3::rotation_about_axis(Point3::origin(), chart.d.get(), by);
                Affine3::from_parts(turn.linear * m.linear, m.translation)
            }
        };
        left = after_turn(left).unwrap_or(left);
    }
    for v in slides {
        let chart = chart(v)?;
        let Coord::Slide(along) = v.coord else {
            unreachable!("the spins were taken above")
        };
        let axis = chart.axis(along)?;
        let gap = v.by - chart.slide(&m, axis);
        m = Affine3::from_parts(Mat3::identity(), axis.get() * gap) * m;
    }
    Ok(m)
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::linalg::{Affine3, UnitVec3, Vec3};
    use geom_core::predicate::Band;

    use super::Chart;
    use crate::NodeErrorKind;
    use crate::mate::SlideAxis;
    use crate::mate::coset::Arm;
    use crate::pose::{PoseConstruction, PoseValue};

    fn band() -> Band {
        Band::linear(geom_core::Tol::witness()).expect("a band")
    }

    fn direction(v: [f64; 3]) -> PoseValue<f64> {
        PoseValue::Direction {
            dir: UnitVec3::new(Vec3::from_array(v), topo::query::DATUM_UNIT_NORM, band())
                .expect("a direction"),
        }
    }

    /// A direction the band cannot tell from `−z`: its swing is
    /// undecided.
    fn in_the_antipode_band() -> [f64; 3] {
        [1.5 * band().zero(), 0.0, -1.0]
    }

    fn undecided(read: Result<f64, NodeErrorKind>) -> bool {
        matches!(
            read,
            Err(NodeErrorKind::PoseDegenerate {
                construction: PoseConstruction::Chart
            })
        )
    }

    /// **A value refuses only the swing it reads**
    /// (`a-lone-slide-refuses-the-chart-it-never-reads`). With the
    /// copy's direction in the antipode band, a slide along `d` and the
    /// slides across it read, and only a spin, which reads the copy's
    /// swing, refuses. With the target's in the band, the slide along
    /// `d` still reads, and every value across it refuses.
    ///
    /// *Breaks if* the chart is built whole again, refusing every value
    /// of a mate whose either swing the band cannot decide.
    #[test]
    fn a_value_refuses_only_the_swing_it_reads() {
        let arm = Arm::of(1.0, 0.0).expect("an arm");
        let at = Affine3::identity();
        let copy_undecided = Chart::of(
            &direction(in_the_antipode_band()),
            &direction([0.0, 0.0, 1.0]),
            false,
            &Affine3::identity(),
            arm,
            band(),
        )
        .expect("a direction mate has a chart");
        for along in [SlideAxis::X, SlideAxis::Y, SlideAxis::Z] {
            assert!(
                copy_undecided.slide_at(&at, along).is_ok(),
                "a slide reads no copy swing: {along:?}"
            );
        }
        assert!(undecided(copy_undecided.spin_at(&at)), "a spin reads it");

        let target_undecided = Chart::of(
            &direction([0.0, 0.0, 1.0]),
            &direction(in_the_antipode_band()),
            false,
            &Affine3::identity(),
            arm,
            band(),
        )
        .expect("a direction mate has a chart");
        assert!(
            target_undecided.slide_at(&at, SlideAxis::Z).is_ok(),
            "a slide along d reads no swing"
        );
        for along in [SlideAxis::X, SlideAxis::Y] {
            assert!(
                undecided(target_undecided.slide_at(&at, along)),
                "a slide across d reads the target's swing: {along:?}"
            );
        }
        assert!(
            undecided(target_undecided.spin_at(&at)),
            "and so does a spin"
        );
    }
}
