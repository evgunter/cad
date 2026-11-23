//! **Values** (FORK-S3M; D10 `[placement.values]`): one freedom a
//! bundle's mates leave, set to a number.
//!
//! A value names no mate. Where a mate fixes a direction, its chart
//! ([`Chart`]) is read once from that mate's two poses and its
//! orientation `O`: the mated direction `d` (the target's, its `Flip`
//! the contact's sense), the copy's mated direction `d_c` in its own
//! coordinates, and the **least turn** `S` carrying `n = O·d_c` onto
//! `d`. The copy's turn is `rot(a, θ)·S·O`, where `θ`, the one spin, is
//! the angle about the target's own direction `a` (its outward normal
//! for a plane: `d` with every flip on the target's side undone,
//! [`flipped`]), whatever the contact's sense, left after the least
//! turn. So no reference axis is chosen on either side, and a flip
//! written on either side of a rest turns its spin the same way. A
//! slide runs along a direction it names ([`Chart::slide_direction`]):
//! the mated direction, a target axis as `S` carries it, or a direction
//! read off a body in the target space. Each is a function of the
//! copy's relative pose alone, so no order of the values is chosen, and
//! nothing about the body the target pose is read off enters the chart.
//!
//! The least turn has no answer at its one pole, `n = −d`, the
//! orientation written backwards; inside the band about it the chart
//! refuses ([`super::MateFault::Backwards`]), naming the orientation to
//! write. Short of the pole `S` is smooth in `d`, so a tilt of the
//! target by a sliver moves a copy by a sliver; its rounding grows as
//! `2u/|n + d|` toward the pole, which the band's conditioning edge
//! holds within ε at `f64`.
//!
//! [`settle`] places a folded bundle's copy at its values: the spin
//! turns the copy about `a` inside the bundle's residual, then the copy
//! stands where the mate seats it — the point of what it leaves nearest
//! the target's origin — plus the sum of its slides, each its value
//! along its direction in what the mate leaves. A carried axis lies
//! there where `n` is a target axis or the mate leaves every
//! translation; otherwise it runs along its part there, and the written
//! slides are admitted only where those parts span what the mate
//! leaves, decided once from the source's geometry
//! ([`super::NoChartWhy::SlidesDoNotSpan`]). A read direction is
//! admitted where it lies in what the mate leaves and the slides stay
//! independent, decided at evaluation
//! ([`super::NoChartWhy::ReadSlidesDoNotSpan`]). On axes that lie in
//! what the mate leaves the sum is the copy's origin measured along
//! them. The values are taken in one order whatever order they are
//! listed in, so the listed order moves no bit. Where no direction is
//! fixed, the spins are a chain the author orders ([`chain`]): about
//! the copy's origin or a read line with no mate ([`unmated_motion`]),
//! about the mated point beside a lone point mate.

use geom_core::linalg::{Affine3, Mat3, Point3, UnitVec3, Vec3};
use geom_core::predicate::{Band, Margin, Sign};
use geom_core::{Angle, RigidMotion};

use super::coset::{Arm, Coset, Subgroup, after_turn};
use super::solve::SolveScalar;
use super::{Coord, MateFault, MateRef, NoChartWhy, Orientation, TargetAxis};
use crate::NodeErrorKind;
use crate::pose::PoseValue;

/// **The chart of the one mate fixing a direction, in the target
/// space.**
#[derive(Debug, Clone, Copy)]
pub(crate) struct Chart<T: geom_core::Real> {
    /// The mated direction: the target's, its `Flip` the contact's
    /// sense.
    d: UnitVec3<T>,
    /// **The direction the one spin turns about**: the target's own,
    /// `d` with every flip along the target's pose undone, so the spin's
    /// sense is the target's whichever side of the mate spells the
    /// contact's.
    about: UnitVec3<T>,
    /// The least turn `S`, minted as the turn it is: its axes are the
    /// target axes as it carries them, `S·x`, `S·y`, `S·z`
    /// ([`Self::carried`]).
    turn: RigidMotion<T>,
    /// Where the orientation puts the copy's mated direction, `O·d_c`.
    n: Vec3<T>,
    /// The target axis `n` is exactly, where it is one.
    aligned: Option<usize>,
    /// The orientation.
    orient: Orientation,
    /// A unit direction across `n`, in the orientation's coordinates:
    /// `S·u` on the target's side and `O⁻¹·u` on the copy's are the two
    /// directions the spin is measured between.
    across: Vec3<T>,
    /// Where the mate seats the copy, read off its two values' own
    /// invariants ([`Seat`]).
    seat: Seat<T>,
    /// The bundle's arm, which the span's verdict is levered over.
    arm: Arm,
    /// The band.
    band: Band,
}

/// **What a mate's two values fix of the copy's translation**, in
/// closed form: `t` the translation of a placed map whose linear part
/// `R` carries the copy's mated direction onto `d`.
#[derive(Debug, Clone, Copy)]
enum Seat<T: geom_core::Real> {
    /// A plane mate: `d · t`, the target's offset along `d` less the
    /// copy's along its own mated direction.
    Depth(T),
    /// An axis mate: `t` across `d` is `foot − R · copy_foot`, the two
    /// lines' points nearest their spaces' origins (each across its
    /// line, so the difference is across `d`).
    Line {
        /// The target line's foot: a point of the line the mate's spin
        /// turns about.
        foot: Point3<T>,
        /// The copy's line's foot, in its own coordinates.
        copy_foot: Point3<T>,
    },
    /// A direction mate fixes no translation.
    Free,
}

/// **Whether `pose` reads the opposite sense of its own direction**: an
/// odd count of flips along the poses it keeps, read plainly or through
/// a role that keeps a sense (a plane's normal, an axis's direction).
/// The one spin turns about the target's own direction, so a flip on
/// either side of a mate moves no spin.
#[must_use]
pub fn flipped<P>(doc: &crate::doc::Doc<P>, pose: crate::pose::RoleRead<crate::VarId>) -> bool {
    use crate::pose::PoseDef;
    let mut odd = false;
    if !pose.keeps_sense() {
        return false;
    }
    let mut at = pose.read;
    while let Some(def) = doc.var(at).and_then(|v| v.def().pose()) {
        odd ^= matches!(def, PoseDef::Flip { .. });
        match def.kept() {
            Some(kept) if kept.keeps_sense() => at = kept.read,
            _ => break,
        }
    }
    odd
}

/// **A bundle's spin about the target's own direction, as a link of the
/// chain** ([`spin_as_chain_turn`]): the target axis the link turns
/// about, the sign the spin reads along it, and the chain `x`, `y`, `z`
/// of the chart's constant turn `S·O`, each a whole number of quarter
/// turns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChainTurn {
    /// The axis of the chain's last link, which the spin turns about.
    pub axis: TargetAxis,
    /// `+1` or `-1`: `rot(about, θ) = rot(axis, sign·θ)`.
    pub sign: i32,
    /// The chain `x`, `y`, `z` of `S·O`, in quarter turns.
    pub quarters: [i32; 3],
}

/// **The link of the chain `chain` that the spin of a bundle charted by
/// `charted` is, exactly, where it is one** (D10 `[placement.values]`).
/// The chart places its copy at `rot(about, θ)·S·O` ([`Chart`]); a chain
/// `x`, `y`, `z` places it at `Rz(γ)·Ry(β)·Rx(α)` ([`chain`]). The two
/// are one rotation about one line, link for link, exactly when:
///
/// - the chain is the three turns about `x`, `y` and `z` in that order,
///   so `z` is its last link, turning the whole copy about the target's
///   own `z`;
/// - `about` lies along `z` exactly ([`super::turn::aligned`]), so
///   `rot(about, θ) = Rz(s·θ)`;
/// - `S·O` is a signed permutation, so its own chain is a whole number of
///   quarter turns about each axis, read exactly; and it does not stand
///   `x` on `±z`, the chain's pole, where `γ` and `α` turn about one line
///   and the chain read gives the whole turn to `α`.
///
/// Then `rot(about, θ)·S·O = Rz(s·θ + q_z·¼)·Ry(q_y·¼)·Rx(q_x·¼)`. `None`
/// for every other chart and chain — no number decides it, and a caller
/// carries no value across a change of freedom the kernel does not state.
#[must_use]
pub fn spin_as_chain_turn<R>(charted: Charted<'_>, chain: &[Coord<R>]) -> Option<ChainTurn> {
    let spins: Vec<TargetAxis> = chain
        .iter()
        .filter_map(|coord| match coord {
            Coord::Spin(axis) => Some(*axis),
            Coord::SpinAbout(_) => Some(None),
            Coord::Slide(_) | Coord::SlideAlong(_) => None,
        })
        .collect::<Option<_>>()?;
    if spins != TargetAxis::ALL {
        return None;
    }
    let (d, copy_d, _) = Chart::<f64>::sides(charted.on, charted.to)?;
    let about = if charted.to_flipped {
        -d.get()
    } else {
        d.get()
    };
    if super::turn::aligned(about) != Some(TargetAxis::Z.index()) {
        return None;
    }
    let sign = if about.z > 0.0 { 1 } else { -1 };
    // `S·O`: the least turn carrying `O·copy_d` onto `d`, after `O`. A
    // signed permutation only where the least turn is none, `O·copy_d`
    // being `d` exactly.
    let n = charted.orient.apply(copy_d.get());
    if [n.x - d.get().x, n.y - d.get().y, n.z - d.get().z] != [0.0; 3] {
        return None;
    }
    let column = |axis: super::SignedAxis| {
        let mut v = [0; 3];
        v[axis.index()] = if axis.negative() { -1 } else { 1 };
        v
    };
    let o = [
        column(charted.orient.x()),
        column(charted.orient.y()),
        column(charted.orient.z()),
    ];
    // The pole: `x` carried onto `±z`.
    if o[0][2] != 0 {
        return None;
    }
    // `S·O = Rz(γ)·Ry(β)·Rx(α)` with `β` off the pole: `β` is `0` or a
    // half turn by where `x` lands, and `γ` the quarter turns from `x`
    // to the column's direction in the `xy` plane.
    let gamma: i32 = match (o[0][0], o[0][1]) {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => -1,
        _ => return None,
    };
    // Rz(−γ)·S·O = Ry(β)·Rx(α) carries `x` onto `x`: β = 0, and α the
    // quarter turns carrying `y` onto that turn's `y` column.
    let unturned = |v: [i32; 3]| -> [i32; 3] {
        // Rz(−γ) by quarter turns, exactly.
        let [x, y, z] = v;
        match gamma.rem_euclid(4) {
            0 => [x, y, z],
            1 => [y, -x, z],
            2 => [-x, -y, z],
            _ => [-y, x, z],
        }
    };
    let y = unturned(o[1]);
    let alpha = match (y[1], y[2]) {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => -1,
        _ => return None,
    };
    Some(ChainTurn {
        axis: TargetAxis::Z,
        sign,
        quarters: [alpha, 0, gamma],
    })
}

/// **Whether a mate's side, as authored, reads the opposite sense of its
/// own direction**: [`flipped`] over an operand — a pose variable read
/// as [`flipped`] reads it, a pose definition walked the same way.
#[must_use]
pub fn side_flipped<P>(
    doc: &crate::doc::Doc<P>,
    side: &crate::pose::RoleRead<crate::Operand>,
) -> bool {
    use crate::pose::PoseDef;
    if !side.keeps_sense() {
        return false;
    }
    match &side.read {
        crate::Operand::Var(var) => flipped(doc, crate::pose::RoleRead::plain(*var)),
        crate::Operand::Pose(def) => {
            let here = matches!(**def, PoseDef::Flip { .. });
            let kept = def.kept().is_some_and(|read| side_flipped(doc, read));
            here != kept
        }
        _ => false,
    }
}

/// **Why a chart refused** ([`Chart::charted`]): the least turn at or
/// about its pole, which its caller names ([`MateFault::Backwards`] at a
/// placement's evaluation, a freedom unread at a gesture's read-back), or
/// a refusal the chart states itself.
#[derive(Debug)]
enum ChartRefusal {
    /// The least turn's pole verdict or conditioning refused.
    Pole,
    /// The chart's own refusal.
    Kind(Box<NodeErrorKind>),
}

/// **Why [`Chart::read`] refused**, before its caller names whose chart
/// it is: a placement's mate ([`MateFault::Backwards`]) or a face's chart
/// ([`crate::pose::PoseConstruction::FaceChartBackwards`]).
#[derive(Debug)]
pub(crate) enum ChartFault {
    /// The orientation is written backwards: the mated direction lies
    /// within the band about the least turn's pole.
    Backwards {
        /// The orientation nearest the one written that puts the copy's
        /// mated direction nearest the mated one.
        write: Orientation,
    },
    /// The chart's own refusal.
    Kind(Box<NodeErrorKind>),
}

impl<T: SolveScalar> Chart<T> {
    /// **The chart of `mate`, equating `on` with `to`**, both read in
    /// the target space, under its orientation. `None` for a point or a
    /// frame mate, which fixes no one direction.
    ///
    /// # Errors
    ///
    /// [`MateFault::Backwards`] in the band about the least turn's pole,
    /// or a direction the band cannot mint.
    #[allow(clippy::too_many_arguments)] // the mate's two poses and its seat, the arm, and the run
    pub(crate) fn of(
        mate: MateRef,
        on: &PoseValue<T>,
        to: &PoseValue<T>,
        to_flipped: bool,
        orient: Orientation,
        arm: Arm,
        band: Band,
        tol: geom_core::Tol,
    ) -> Result<Option<Self>, NodeErrorKind> {
        Self::read(on, to, to_flipped, orient, arm, band, tol).map_err(|fault| match fault {
            ChartFault::Backwards { write } => {
                NodeErrorKind::Mate(Box::new(MateFault::Backwards {
                    mate,
                    orient,
                    write,
                }))
            }
            ChartFault::Kind(kind) => *kind,
        })
    }

    /// **The chart of a mate equating `on` with `to`**, as [`Self::of`]
    /// reads it, its refusal naming no mate: what a placement and a
    /// face's chart ([`FaceChart`]) both read.
    ///
    /// # Errors
    ///
    /// [`ChartFault::Backwards`] in the band about the least turn's pole,
    /// naming the orientation to write; [`ChartFault::Kind`] a direction
    /// the band cannot mint.
    pub(crate) fn read(
        on: &PoseValue<T>,
        to: &PoseValue<T>,
        to_flipped: bool,
        orient: Orientation,
        arm: Arm,
        band: Band,
        tol: geom_core::Tol,
    ) -> Result<Option<Self>, ChartFault> {
        Self::read_as(on, to, to_flipped, orient, arm, band, Some(tol))
    }

    /// **[`Self::read`] as a selection**: its least turn's pole verdict
    /// alone, under [`FACE_CHART_SELECTION_POLE`], with no conditioning
    /// ([`RigidMotion::least_turn_selected`]): a read made before the
    /// extent it is consumed over exists, whose conditioning the carriage
    /// that consumes it decides (the geom-core README, "Float rounding").
    /// The same turn, every bit.
    ///
    /// # Errors
    ///
    /// As [`Self::read`].
    pub(crate) fn selected(
        on: &PoseValue<T>,
        to: &PoseValue<T>,
        orient: Orientation,
        arm: Arm,
        band: Band,
    ) -> Result<Option<Self>, ChartFault> {
        Self::read_as(on, to, false, orient, arm, band, None)
    }

    /// [`Self::read`], its conditioning decided against `tol`'s rounding
    /// reach, or none ([`Self::selected`]).
    fn read_as(
        on: &PoseValue<T>,
        to: &PoseValue<T>,
        to_flipped: bool,
        orient: Orientation,
        arm: Arm,
        band: Band,
        tol: Option<geom_core::Tol>,
    ) -> Result<Option<Self>, ChartFault> {
        let Some((d, copy_d, seat)) = Self::sides(on, to) else {
            return Ok(None);
        };
        // Unsourced lever: the mate bundle's `Arm`, the copy's reach and its written
        // slides plus the poses' distances from their origins, an `f64` sum no extent
        // read spells. The least turn levers its pole and its conditioning at an arm
        // in `T`.
        let lever = geom_core::Lever::unsourced(T::from_computed(arm.get()));
        Self::charted(d, copy_d, seat, to_flipped, orient, arm, lever, band, tol)
            .map(Some)
            .map_err(|refusal| match refusal {
                ChartRefusal::Pole => {
                    let q = |v: Vec3<T>| Vec3::new(v.x.quoted(), v.y.quoted(), v.z.quoted());
                    ChartFault::Backwards {
                        write: orient.toward(q(copy_d.get()), q(d.get())),
                    }
                }
                ChartRefusal::Kind(kind) => ChartFault::Kind(kind),
            })
    }

    /// What a mate fixing a direction reads of its two poses
    /// ([`mated_direction`]), as the chart holds it.
    fn sides(on: &PoseValue<T>, to: &PoseValue<T>) -> Option<(UnitVec3<T>, UnitVec3<T>, Seat<T>)> {
        let mated = mated_direction(on, to)?;
        let seat = match mated.hold {
            MatedHold::Plane {
                copy_offset,
                offset,
            } => Seat::Depth(offset - copy_offset),
            MatedHold::Line { copy_foot, foot } => Seat::Line { foot, copy_foot },
            MatedHold::Free => Seat::Free,
        };
        Some((mated.dir, mated.copy_dir, seat))
    }

    /// **The chart of the two directions**: the least turn carrying where
    /// `orient` puts the copy's mated direction onto `d`, decided over
    /// `lever` (its pole verdict and, against `tol`, its conditioning,
    /// [`RigidMotion::least_turn`]; with no `tol` its pole verdict alone,
    /// [`RigidMotion::least_turn_selected`]), refusing
    /// [`ChartRefusal::Pole`] where either refuses.
    #[allow(clippy::too_many_arguments)]
    fn charted(
        d: UnitVec3<T>,
        copy_d: UnitVec3<T>,
        seat: Seat<T>,
        to_flipped: bool,
        orient: Orientation,
        arm: Arm,
        lever: geom_core::Lever<T>,
        band: Band,
        tol: Option<geom_core::Tol>,
    ) -> Result<Self, ChartRefusal> {
        let degenerate = || {
            ChartRefusal::Kind(Box::new(NodeErrorKind::PoseDegenerate {
                construction: crate::pose::PoseConstruction::Chart,
            }))
        };
        let unit = |v: Vec3<T>| {
            UnitVec3::new(v, topo::query::POSE_UNIT_NORM, band).map_err(|_| degenerate())
        };
        let from = copy_d
            .signed_permutation(orient.signed_permutation())
            .unwrap_or_else(|| unreachable!("an orientation is a permutation of the axes"));
        let n = from.get();
        let turn = {
            // The pole verdict and the conditioning, over the lever: the
            // one decided least-turn mint (`[placement.values]`); or, as a
            // selection, the pole verdict alone.
            let about = geom_core::linalg::Point3::origin();
            let least = match tol {
                Some(tol) => {
                    RigidMotion::least_turn(from, d, about, lever, "mate_chart_pole", band, tol)
                }
                None => RigidMotion::least_turn_selected(
                    from,
                    d,
                    about,
                    lever,
                    FACE_CHART_SELECTION_POLE,
                    band,
                ),
            }
            .map_err(|e| match e {
                geom_core::TurnError::NonFinite(_) => degenerate(),
                _ => ChartRefusal::Pole,
            })?;
            // A pair one direction at every point the scalar stands for
            // turns by the identity's own axes, exactly: nothing to
            // normalize, so no check is spent on them.
            let (f, t) = (from.get(), d.get());
            let identity = [f.x - t.x, f.y - t.y, f.z - t.z]
                .into_iter()
                .all(SolveScalar::zero_everywhere);
            if identity || T::spins_by_frames() {
                least
            } else {
                // Each carried axis is a turn of a unit vector, so unit over
                // the reals at every point a box spans: a frame lane keeps
                // each as it is, which a normalization would widen by its own
                // norm's width, and decides each direction as the value lane
                // does.
                least
                    .renormalized(topo::query::POSE_UNIT_NORM, band)
                    .map_err(|_| degenerate())?
            }
        };
        let aligned = super::turn::aligned(n);
        let across = match aligned {
            Some(_) => super::turn::across(n),
            None => unit(super::turn::across(n))?.get(),
        };
        Ok(Self {
            d,
            about: if to_flipped { -d } else { d },
            turn,
            n,
            aligned,
            orient,
            across,
            seat,
            arm,
            band,
        })
    }

    /// A point of the mate's own line, for an axis mate: the line its
    /// spin turns about.
    fn line(&self) -> Option<Point3<T>> {
        match self.seat {
            Seat::Line { foot, .. } => Some(foot),
            Seat::Depth(_) | Seat::Free => None,
        }
    }

    /// **The copy's translation along carried axis `a` that the mate
    /// fixes**, in closed form from its two values ([`Seat`]) and the
    /// placed map's linear part `r`, where `n` is a target axis: `None`
    /// along a freedom the mate leaves.
    fn seated(&self, r: &Mat3<T>, a: usize) -> Option<T> {
        let j = self.aligned?;
        match self.seat {
            // `S·e_j` is `d` in `n`'s sense.
            Seat::Depth(depth) if a == j => Some([self.n.x, self.n.y, self.n.z][j] * depth),
            Seat::Line { foot, copy_foot } if a != j => {
                let across = (foot - Point3::origin()) - *r * (copy_foot - Point3::origin());
                Some(across.dot(self.carried(a)))
            }
            Seat::Depth(_) | Seat::Line { .. } | Seat::Free => None,
        }
    }

    /// The spin of the copy at relative pose `m`: the angle about the
    /// target's own direction from the least turn's image of the
    /// reference direction to the copy's, as `m` carries it.
    fn spin(&self, linear: &Mat3<T>) -> T {
        let to = self.carried_of(self.across);
        let copy = *linear * self.orient.unapply(self.across);
        T::solve_atan2(to.cross(copy).dot(self.about.get()), to.dot(copy))
    }

    /// `S·e_a`: the target axis `a` as the least turn carries it.
    fn carried(&self, a: usize) -> Vec3<T> {
        match a {
            0 => self.turn.u(),
            1 => self.turn.v(),
            _ => self.turn.w(),
        }
        .get()
    }

    /// `S·v`, from the carried axes.
    fn carried_of(&self, v: Vec3<T>) -> Vec3<T> {
        self.carried(0) * v.x + self.carried(1) * v.y + self.carried(2) * v.z
    }

    /// **`S·O`, the constant turn**: the copy's axes carried onto the
    /// target's by the orientation and the least turn — the least turn's
    /// own axes, signed and permuted as the orientation puts them
    /// ([`RigidMotion::with_signed_axes`]), so nothing is decided.
    fn least(&self) -> RigidMotion<T> {
        let signed = |a: super::SignedAxis| (a.index(), a.negative());
        self.turn
            .with_signed_axes(signed(self.orient.x()), signed(self.orient.y()))
            .unwrap_or_else(|| unreachable!("an orientation's x and y are two axes"))
    }

    /// The turn of every map whose spin is `by`: `rot(a, by)·S·O`, `a`
    /// the target's own direction, each axis of [`Self::least`] spun
    /// ([`RigidMotion::spun_about`]).
    fn turned(&self, by: Angle<T>) -> Result<RigidMotion<T>, NodeErrorKind> {
        self.least()
            .spun_about(self.about, by)
            .map_err(NodeErrorKind::Motion)
    }

    /// **The copy's map at spin `by` and the translation `shift` before
    /// [`Self::least`]**: the shift, then the turn by `by` about the
    /// target's own direction, each composed as a motion, so nothing
    /// re-derives its rigidity from its entries and nothing is decided.
    fn head_at(&self, by: Angle<T>, shift: Vec3<T>) -> Option<RigidMotion<T>> {
        let turn = RigidMotion::rotation_by(self.about, by).ok()?;
        RigidMotion::translation(shift).ok()?.compose(turn).ok()
    }

    /// Whether the mated direction is an axis of the target space
    /// exactly.
    fn on_axes(&self) -> bool {
        super::solve::on_an_axis(self.d.get()).is_some()
    }

    /// The slide of the copy at relative pose `m` along `axis`: its
    /// origin measured from the target space's.
    fn slide(m: &RigidMotion<T>, axis: Vec3<T>) -> T {
        (m.origin() - Point3::origin()).dot(axis)
    }

    /// **Whether the written slides span what the mate leaves**: the
    /// component of `n` along `along` is decided nonzero, levered over
    /// the arm — exactly, with no verdict, where `n` is a target axis.
    fn spans(&self, along: usize) -> bool {
        if let Some(j) = self.aligned {
            return j == along;
        }
        let comp = [self.n.x, self.n.y, self.n.z][along];
        matches!(
            geom_core::k_stats::decide(
                "mate_chart_slides_span",
                Margin::levered_at(comp, self.arm.lever()),
                self.band,
            ),
            Ok(Sign::Positive | Sign::Negative)
        )
    }
}

impl<T: SolveScalar> Chart<T> {
    /// **`v` within what the mate leaves of the copy's translation**: its
    /// part across `d` beside a plane mate, along `d` beside an axis
    /// mate, all of it beside a direction mate.
    fn part_left(&self, v: Vec3<T>) -> Vec3<T> {
        let d = self.d.get();
        match self.seat {
            Seat::Depth(_) => v - d * d.dot(v),
            Seat::Line { .. } => d * d.dot(v),
            Seat::Free => v,
        }
    }

    /// **The unit direction a slide `coord` runs along, in what the mate
    /// leaves** (D10 `[placement.values]`), `line` the line it reads:
    ///
    /// - the mated direction, the target's own (`about`);
    /// - a target axis as the least turn carries it, which lies in what
    ///   the mate leaves where the source's mated direction is one of its
    ///   axes (or the mate leaves every translation), and otherwise runs
    ///   along its part there, a direction decided once from the source's
    ///   geometry ([`Self::spans`]);
    /// - a direction read off a body, admitted where it lies in what the
    ///   mate leaves, decided at evaluation, since it moves with its body.
    ///
    /// # Errors
    ///
    /// [`NoChartWhy::ReadSlidesDoNotSpan`] for a read direction across
    /// what the mate leaves, or one the band cannot mint.
    pub(crate) fn slide_direction<R>(
        &self,
        coord: &Coord<R>,
        line: Option<crate::pose::AxisLine<T>>,
    ) -> Result<Heading<T>, NoChartWhy> {
        let unit = |v: Vec3<T>, why: NoChartWhy| {
            UnitVec3::new(v, topo::query::POSE_UNIT_NORM, self.band)
                .map(|u| Heading {
                    dir: u.get(),
                    frame: false,
                })
                .map_err(|_| why)
        };
        match coord {
            Coord::Slide(None) => Ok(Heading {
                dir: self.about.get(),
                frame: false,
            }),
            Coord::Slide(Some(axis)) => {
                let carried = self.carried(axis.index());
                if self.aligned.is_some() || matches!(self.seat, Seat::Free) {
                    Ok(Heading {
                        dir: carried,
                        frame: true,
                    })
                } else {
                    unit(
                        self.part_left(carried),
                        NoChartWhy::SlidesDoNotSpan {
                            along: None,
                            write: super::turn::slides_left_along(self.left(), self.n),
                        },
                    )
                }
            }
            Coord::SlideAlong(_) => {
                let Some(line) = line else {
                    unreachable!("a slide along a read direction is evaluated with its line")
                };
                let u = line.dir().get();
                let d = self.d.get();
                let across = match self.seat {
                    Seat::Depth(_) => Some(d.dot(u)),
                    Seat::Line { .. } => {
                        let c = d.cross(u);
                        Some(c.norm())
                    }
                    Seat::Free => None,
                };
                if let Some(across) = across {
                    let within = geom_core::k_stats::decide(
                        super::MATE_READ_SLIDE_WITHIN,
                        Margin::levered_at(across, self.arm.lever()),
                        self.band,
                    );
                    if within != Ok(Sign::Zero) {
                        return Err(NoChartWhy::ReadSlidesDoNotSpan);
                    }
                }
                unit(self.part_left(u), NoChartWhy::ReadSlidesDoNotSpan)
            }
            Coord::Spin(_) | Coord::SpinAbout(_) => unreachable!("a spin runs along no direction"),
        }
    }

    /// How many translations the mate leaves.
    fn left(&self) -> usize {
        match self.seat {
            Seat::Depth(_) => 2,
            Seat::Line { .. } => 1,
            Seat::Free => 3,
        }
    }

    /// **Whether `heads` are independent within what the mate leaves**:
    /// their determinant there decided nonzero, levered over the arm.
    fn independent(&self, heads: &[Heading<T>]) -> bool {
        independent(heads, self.d.get(), self.arm, self.band)
    }
}

/// **A slide's direction, as the settle runs it**: a unit direction in
/// what the mates leave, and whether it is one of one orthonormal frame's
/// axes there (the chart's carried axes, or the target space's own with
/// no mate), where the slides' coordinates are their directions' dot
/// products and no system is solved. A read direction is never one.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Heading<T: geom_core::Real> {
    /// The direction.
    dir: Vec3<T>,
    /// Whether it is an axis of the chart's own frame.
    frame: bool,
}

/// **Whether `heads` are independent**: as many as the space they span
/// (a line, a plane across `d`, or every direction), their determinant
/// there decided nonzero, levered over `arm`.
fn independent<T: SolveScalar>(heads: &[Heading<T>], d: Vec3<T>, arm: Arm, band: Band) -> bool {
    let det = match heads {
        [] | [_] => return true,
        [a, b] => a.dir.cross(b.dir).dot(d),
        [a, b, c] => a.dir.dot(b.dir.cross(c.dir)),
        _ => return false,
    };
    matches!(
        geom_core::k_stats::decide(
            super::MATE_SLIDES_INDEPENDENT,
            Margin::levered_at(det, arm.lever()),
            band,
        ),
        Ok(Sign::Positive | Sign::Negative)
    )
}

/// **The coordinates of `t` along `heads`**, its part in the space they
/// span: their dot products where each is an axis of one orthonormal
/// frame, else the solution of their Gram system. `None` where that
/// system is singular.
fn coordinates<T: SolveScalar>(heads: &[Heading<T>], t: Vec3<T>) -> Option<Vec<T>> {
    let dots: Vec<T> = heads.iter().map(|h| h.dir.dot(t)).collect();
    if heads.iter().all(|h| h.frame) || heads.len() < 2 {
        return Some(dots);
    }
    let g = |i: usize, j: usize| heads[i].dir.dot(heads[j].dir);
    let zero = |x: T| x.exactly_zero();
    match heads.len() {
        2 => {
            let (a, b, c) = (g(0, 0), g(0, 1), g(1, 1));
            let det = a * c - b.powi(2);
            if zero(det) {
                return None;
            }
            Some(vec![
                (c * dots[0] - b * dots[1]) / det,
                (a * dots[1] - b * dots[0]) / det,
            ])
        }
        3 => {
            let m = [
                [g(0, 0), g(0, 1), g(0, 2)],
                [g(1, 0), g(1, 1), g(1, 2)],
                [g(2, 0), g(2, 1), g(2, 2)],
            ];
            let det3 = |m: [[T; 3]; 3]| {
                m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
                    - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
                    + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
            };
            let det = det3(m);
            if zero(det) {
                return None;
            }
            Some(
                (0..3)
                    .map(|k| {
                        let mut mk = m;
                        for (row, dot) in mk.iter_mut().zip(&dots) {
                            row[k] = *dot;
                        }
                        det3(mk) / det
                    })
                    .collect(),
            )
        }
        _ => None,
    }
}

/// **The line a value reads**, read in the target space: an axis's
/// line, or a direction's through the origin.
pub(crate) fn heading_of<T: geom_core::Real>(pose: &PoseValue<T>) -> crate::pose::AxisLine<T> {
    match pose {
        PoseValue::Axis(line) => *line,
        PoseValue::Direction { dir } => crate::pose::AxisLine::through(Point3::origin(), *dir),
        PoseValue::Point { .. } | PoseValue::Plane { .. } | PoseValue::Frame(_) => {
            unreachable!("a value's seat admits a direction or an axis, which the doors check")
        }
    }
}

/// **What a mate fixing one direction reads of its two poses**
/// ([`mated_direction`]): the mated direction (the target's, its flip
/// the contact's sense), the copy's in its own coordinates, and what
/// the target holds of the copy's mated point.
#[derive(Debug, Clone, Copy)]
pub struct MatedDirection<T: geom_core::Real> {
    /// The target's direction.
    pub dir: UnitVec3<T>,
    /// The copy's direction, in its own coordinates.
    pub copy_dir: UnitVec3<T>,
    /// What the target holds of the copy's mated point.
    pub hold: MatedHold<T>,
}

/// **What a mate fixing one direction holds of the copy's mated point**.
#[derive(Debug, Clone, Copy)]
pub enum MatedHold<T: geom_core::Real> {
    /// Two planes: the copy's plane at `copy_offset` along its normal,
    /// held on the target's at `offset` along its own.
    Plane {
        /// The copy's plane's offset, in its own coordinates.
        copy_offset: T,
        /// The target's plane's offset.
        offset: T,
    },
    /// Two axes: the copy's line through `copy_foot`, held on the
    /// target's through `foot`, each its point nearest its origin.
    Line {
        /// The copy's line's foot, in its own coordinates.
        copy_foot: Point3<T>,
        /// The target's line's foot.
        foot: Point3<T>,
    },
    /// Two directions: no point is held.
    Free,
}

/// **What a mate equating `on` with `to` fixes**, where it fixes one
/// direction (two planes, two axes, two directions): the one reading
/// of a mate's poses the value chart ([`read_back`]) and a gesture's
/// snap both take. `None` for a point or a frame mate.
#[must_use]
pub fn mated_direction<T: geom_core::Real>(
    on: &PoseValue<T>,
    to: &PoseValue<T>,
) -> Option<MatedDirection<T>> {
    let (dir, copy_dir, hold) = match (on, to) {
        (
            PoseValue::Plane {
                normal: o,
                offset: c_on,
            },
            PoseValue::Plane {
                normal: t,
                offset: c_to,
            },
        ) => (
            *t,
            *o,
            MatedHold::Plane {
                copy_offset: *c_on,
                offset: *c_to,
            },
        ),
        (PoseValue::Direction { dir: o }, PoseValue::Direction { dir: t }) => {
            (*t, *o, MatedHold::Free)
        }
        (PoseValue::Axis(copy), PoseValue::Axis(target)) => (
            target.dir(),
            copy.dir(),
            MatedHold::Line {
                copy_foot: copy.foot(),
                foot: target.foot(),
            },
        ),
        _ => return None,
    };
    Some(MatedDirection {
        dir,
        copy_dir,
        hold,
    })
}

/// **A mate fixing a direction, as the chart reads it** ([`read_back`]):
/// the mate, its two poses read in the target space, whether its `to`
/// reads the opposite sense ([`flipped`]) and its orientation.
#[derive(Debug, Clone, Copy)]
pub struct Charted<'a> {
    /// The copy's pose, in its own coordinates.
    pub on: &'a PoseValue<f64>,
    /// The target's pose, in the target space.
    pub to: &'a PoseValue<f64>,
    /// Whether `to` reads the opposite sense of its own direction.
    pub to_flipped: bool,
    /// The mate's orientation.
    pub orient: Orientation,
}

/// **The number each freedom of `coords` reads where the copy stands
/// at relative pose `at`** (D10 `[placement.values]`), by the one chart
/// a placement is placed by: where `chart` names the mate fixing a
/// direction, the spin about the target's own direction and the slides
/// its chart runs along; with none, the slides along the target space's
/// axes and the chain of spins about them, read where the chain is the
/// three turns about `x`, `y` and `z` in that order. The slides are read
/// together, as the coordinates of where the copy stands, from where
/// the mates seat it, along their directions ([`settle`]). A freedom the
/// chart does not read at `at` reads `None`: a link of a chain in
/// another order, a spin about the target's direction with no mate
/// fixing one, any freedom of a chart refused at its pole, and every
/// slide beside one that reads a direction off a body or spins about a
/// read axis, whose line the caller does not hold. So a gesture that stands a copy somewhere
/// writes the values that place it there through the chart that will
/// place it, never a second spelling of it.
#[must_use]
pub fn read_back<R>(
    chart: Option<Charted<'_>>,
    at: &Affine3<f64>,
    coords: &[Coord<R>],
    band: Band,
    tol: geom_core::Tol,
) -> Vec<Option<f64>> {
    // A read direction, or a chain link about a read line off the origin
    // (whose turn moves the origin), is a line the caller does not hold.
    let reads_a_direction = coords
        .iter()
        .any(|c| matches!(c, Coord::SlideAlong(_) | Coord::SpinAbout(_)));
    let Some(charted) = chart else {
        let chain = chain_read(&at.linear, coords);
        return coords
            .iter()
            .map(|coord| match coord {
                Coord::Slide(Some(axis)) if !reads_a_direction => {
                    Some([at.translation.x, at.translation.y, at.translation.z][axis.index()])
                }
                Coord::Spin(Some(axis)) => chain.map(|turns| turns[axis.index()]),
                Coord::Slide(_)
                | Coord::SlideAlong(_)
                | Coord::Spin(None)
                | Coord::SpinAbout(_) => None,
            })
            .collect();
    };
    // The read is of a pose the caller stands the copy at, with no
    // bundle's arm in hand: the least turn is decided over the session
    // box, which refuses widest (about 4e-4 rad from the pole at the
    // default ε), so a freedom read here is one the placement's
    // evaluation, over its own arm, reads too. That is conservative and
    // costs no gesture: the snap writes the nearest orientation, so no
    // gesture lands that near the pole. Threading the copy's reach in
    // from the caller is
    // `work/onehome/the-gesture-read-back-decides-its-least-turn-over-the-session-box.md`.
    let read = Chart::<f64>::sides(charted.on, charted.to).and_then(|(d, copy_d, seat)| {
        let arm = Arm::of(0.0, 0.0).ok()?;
        Chart::charted(
            d,
            copy_d,
            seat,
            charted.to_flipped,
            charted.orient,
            arm,
            geom_core::Lever::session_box(),
            band,
            Some(tol),
        )
        .ok()
    });
    // The slides, read together along the directions the chart runs
    // them.
    let slid: Option<Vec<f64>> = read.and_then(|chart| {
        if reads_a_direction {
            return None;
        }
        let dirs = coords
            .iter()
            .filter(|c| c.is_slide())
            .map(|c| chart.slide_direction(c, None).ok())
            .collect::<Option<Vec<_>>>()?;
        coordinates(&dirs, at.translation)
    });
    let mut slides = slid.into_iter().flatten();
    coords
        .iter()
        .map(|coord| {
            let chart = read?;
            match coord {
                Coord::Spin(None) => Some(chart.spin(&at.linear)),
                Coord::Slide(_) | Coord::SlideAlong(_) => slides.next(),
                Coord::Spin(Some(_)) | Coord::SpinAbout(_) => None,
            }
        })
        .collect()
}

/// **The chain `x`, `y`, `z` that turns as `r` does**, each about the
/// target space's own axis, the first written the first made ([`chain`]):
/// `r = Rz·Ry·Rx`. `None` where `coords` spells another chain.
fn chain_read<R>(r: &Mat3<f64>, coords: &[Coord<R>]) -> Option<[f64; 3]> {
    let spun: Vec<TargetAxis> = coords
        .iter()
        .filter_map(|coord| match coord {
            Coord::Spin(axis) => Some(*axis),
            Coord::SpinAbout(_) => Some(None),
            Coord::Slide(_) | Coord::SlideAlong(_) => None,
        })
        .collect::<Option<_>>()?;
    if spun != TargetAxis::ALL {
        return None;
    }
    Some(chain_turns(r))
}

/// **The chain `x`, `y`, `z` that turns as `r` does** (D10
/// `[placement.values]`): `r = Rz(γ)·Ry(β)·Rx(α)`, each turn about the
/// target space's own axis, the first written the first made
/// ([`chain`]), answered `[α, β, γ]`. At the chain's pole (`x` carried
/// onto `±z`, `cos β = 0`) only `α ± γ` is fixed, and the whole turn is
/// read as `α` with `γ = 0`. The one reading of a turn as a chain: every
/// gesture that writes a frame's turns as values reads them here.
#[must_use]
pub fn chain_turns(r: &Mat3<f64>) -> [f64; 3] {
    // Column k is `r·e_k`, so the entry at row i, column k is `r.ck[i]`.
    let cos_beta = r.c0.x.hypot(r.c0.y);
    let beta = (-r.c0.z).atan2(cos_beta);
    let (alpha, gamma) = if cos_beta > f64::EPSILON.sqrt() {
        (r.c1.z.atan2(r.c2.z), r.c0.y.atan2(r.c0.x))
    } else {
        ((-r.c2.y).atan2(r.c1.y), 0.0)
    };
    [alpha, beta, gamma]
}

/// **Whom a value's refusal names** ([`settle`]): a placement's
/// constraint ([`MateRef`]), or one of a face's chart's values
/// ([`FaceValue`]). Ordered, so the values of one freedom settle in one
/// order whatever order they are listed in.
pub(crate) trait ValueAt: Copy + Ord {
    /// The refusal of this value's slide, which does not span what the
    /// mate leaves.
    fn no_chart(self, why: NoChartWhy) -> NodeErrorKind;
}

impl ValueAt for MateRef {
    fn no_chart(self, why: NoChartWhy) -> NodeErrorKind {
        NodeErrorKind::Mate(Box::new(MateFault::NoChart { value: self, why }))
    }
}

/// **One of a face's chart's values** ([`FaceChart`]): its spin, or
/// one of its two slides.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum FaceValue {
    /// The spin about the face's outward normal.
    Spin,
    /// A slide, along the target axis it runs along.
    Slide(TargetAxis),
}

impl ValueAt for FaceValue {
    fn no_chart(self, why: NoChartWhy) -> NodeErrorKind {
        unreachable!(
            "a face's chart slides along the two target axes its orientation leaves, which span \
             the face's plane: {self:?} refused {why:?}"
        )
    }
}

/// **One value, read**: its address, its freedom and its number at
/// the evaluation's scalar.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ReadValue<T: geom_core::Real, A = MateRef> {
    /// The value's address.
    pub(crate) value: A,
    /// Its freedom.
    pub(crate) coord: Coord,
    /// Its number.
    pub(crate) by: T,
    /// The fraction of a turn a spin's number is written as, where it
    /// is one ([`crate::expr::VarEnv::turns`]).
    pub(crate) turns: Option<crate::expr::Ratio>,
    /// The line the direction or axis it reads stands on, in the target
    /// space, where it reads one ([`heading_of`]).
    pub(crate) line: Option<crate::pose::AxisLine<T>>,
}

impl<T: geom_core::Real, A> ReadValue<T, A> {
    /// A spin's number as an angle, with the form it was written in.
    fn angle(&self) -> Angle<T> {
        crate::expr::angle_of(self.by, self.turns)
    }
}

/// **A chain of spins** (D10 `[placement.values]`), in the order the
/// bundle lists them, the first written the first made: each about one
/// of the target space's own axes through the origin, or about an
/// `Axis` read in the target space — about its line where `on_lines`,
/// about its direction through the origin beside a lone point mate,
/// whose chain turns about the mated point. Composed as motions, so
/// nothing is decided; `None` for a chain of no link.
///
/// # Errors
///
/// [`NodeErrorKind::Motion`]: an angle that is no bounded number.
fn chain<T: SolveScalar>(
    values: &[ReadValue<T>],
    on_lines: bool,
) -> Result<Option<RigidMotion<T>>, NodeErrorKind> {
    let motion = NodeErrorKind::Motion;
    let mut turned: Option<RigidMotion<T>> = None;
    for v in values {
        let spin = match v.coord {
            Coord::Spin(Some(axis)) => {
                RigidMotion::rotation_by(axis.unit(), v.angle()).map_err(motion)?
            }
            Coord::SpinAbout(_) => {
                let Some(line) = v.line else {
                    unreachable!("a spin about a read axis is evaluated with its line")
                };
                let turn = RigidMotion::rotation_by(line.dir(), v.angle()).map_err(motion)?;
                if on_lines {
                    let foot = line.foot() - Point3::origin();
                    RigidMotion::translation(foot)
                        .and_then(|into| into.compose(turn))
                        .and_then(|turned| {
                            turned.compose(RigidMotion::translation(Vec3::zero() - foot)?)
                        })
                        .map_err(motion)?
                } else {
                    turn
                }
            }
            Coord::Spin(None) | Coord::Slide(_) | Coord::SlideAlong(_) => continue,
        };
        turned = Some(match turned {
            Some(inner) => spin.compose(inner).map_err(motion)?,
            None => spin,
        });
    }
    Ok(turned)
}

/// **A bundle with no mate placed by its values** (D10
/// `[placement.values]`): the chain of spins ([`chain`]), each turning
/// the copy about its own origin or about the line it reads, then the
/// sum of the slides, each its value along its direction: a target
/// space's axis or a direction read off a body, so on the target's own
/// axes the copy's origin is its slides. Composed as motions. The bundle
/// rule has admitted the values as the six freedoms, each once; where a
/// slide reads a direction, the three are admitted where they are
/// independent, decided here over `lever`.
///
/// # Errors
///
/// [`MateFault::NoChart`] ([`NoChartWhy::ReadSlidesDoNotSpan`]) for
/// slides along read directions that are not independent;
/// [`NodeErrorKind::Motion`]: an angle or a translation that is no
/// bounded number.
pub(crate) fn unmated_motion<T: SolveScalar>(
    values: &[ReadValue<T>],
    lever: Option<(Arm, Band)>,
) -> Result<RigidMotion<T>, NodeErrorKind> {
    let mut shift = Vec3::zero();
    let mut heads = Vec::new();
    for v in values {
        match v.coord {
            Coord::Slide(Some(TargetAxis::X)) => shift.x = v.by,
            Coord::Slide(Some(TargetAxis::Y)) => shift.y = v.by,
            Coord::Slide(Some(TargetAxis::Z)) => shift.z = v.by,
            Coord::Slide(None) => {
                unreachable!("the bundle rule admits no mated slide with no mate")
            }
            Coord::SlideAlong(_) | Coord::Spin(_) | Coord::SpinAbout(_) => {}
        }
        if v.coord.is_slide() {
            heads.push(match (v.coord, v.line) {
                (Coord::SlideAlong(_), Some(line)) => Heading {
                    dir: line.dir().get(),
                    frame: false,
                },
                (Coord::Slide(Some(axis)), _) => Heading {
                    dir: axis.unit().get(),
                    frame: true,
                },
                _ => unreachable!("a slide along a read direction is evaluated with its line"),
            });
        }
    }
    let reads: Vec<&ReadValue<T>> = values
        .iter()
        .filter(|v| matches!(v.coord, Coord::SlideAlong(_)))
        .collect();
    if let Some(first) = reads.first() {
        let Some((arm, band)) = lever else {
            unreachable!("a bundle reading a direction is folded with its lever")
        };
        if !independent(&heads, Vec3::unit_z(), arm, band) {
            return Err(NodeErrorKind::Mate(Box::new(MateFault::NoChart {
                value: first.value,
                why: NoChartWhy::ReadSlidesDoNotSpan,
            })));
        }
        for v in reads {
            if let Some(line) = v.line {
                shift = shift + line.dir().get() * v.by;
            }
        }
    }
    let slid = RigidMotion::translation(shift).map_err(NodeErrorKind::Motion)?;
    match chain(values, true)? {
        Some(turned) => slid.compose(turned).map_err(NodeErrorKind::Motion),
        None => Ok(slid),
    }
}

/// **A lone point mate's copy placed by its chain** (D10
/// `[placement.values]`): the copy's mated point `on` carried onto the
/// target's `to`, turned about it by the chain of spins in the order
/// written ([`chain`]), a read axis's turn about its direction through
/// the mated point, so a chain of zeros leaves the copy's own axes the
/// target space's. Composed as motions.
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
    let turned = match chain(values, false)? {
        Some(turned) => into.compose(turned).map_err(motion)?,
        None => into,
    };
    turned.compose(out).map_err(motion)
}

/// **A folded bundle's copy placed at its values**: the relative pose
/// with each value's freedom set, inside what the mates leave, on the
/// chart of the one mate fixing a direction. The bundle rule has
/// admitted every value's freedom (`super::solve::overconstrained`) and
/// as many slides as the mates leave translations, so each is held by
/// the residual by construction: a spin turns about the residual's own
/// line, or about the target's own direction through the copy's origin
/// where every line along it will do.
///
/// **The slides.** The copy stands where the mate and the spin seat it,
/// its translation's part in what the mate leaves taken off, plus each
/// slide's value along its direction there ([`Chart::slide_direction`]):
/// `t' = t + Σ u_i (v_i − w_i)`, `w` the coordinates of `t` along the
/// `u_i` (their dot products where they are the chart's own orthonormal
/// axes, the Gram system's solution otherwise, [`coordinates`]).
///
/// # Errors
///
/// [`MateFault::NoChart`] where the written slides do not span what the
/// mate leaves ([`Chart::spans`]), or a read direction does not lie in
/// it.
pub(crate) fn settle<T: SolveScalar, A: ValueAt>(
    coset: Coset<T>,
    values: &[ReadValue<T, A>],
    chart: Option<Chart<T>>,
) -> Result<RigidMotion<T>, NodeErrorKind> {
    let motion = NodeErrorKind::Motion;
    let mut m = coset.representative;
    let mut left = coset.subgroup;
    if values.is_empty() {
        return Ok(m);
    }
    let Some(chart) = chart else {
        unreachable!("the bundle rule admits a value beside a mate fixing a direction alone")
    };
    // One order whatever order the values are listed in: the spin, then
    // slides along x, y and z, along the mated direction, along read
    // directions, then by address.
    let mut ordered: Vec<&ReadValue<T, A>> = values.iter().collect();
    ordered.sort_by_key(|v| {
        let rank = match v.coord {
            Coord::Spin(_) | Coord::SpinAbout(_) => 0,
            Coord::Slide(Some(a)) => 1 + a.index(),
            Coord::Slide(None) => 4,
            Coord::SlideAlong(_) => 5,
        };
        (rank, v.value)
    });
    let (spins, slides): (Vec<_>, Vec<_>) = ordered.into_iter().partition(|v| !v.coord.is_slide());
    // The rule admits the axisless spin alone here, held loud: the chart
    // turns about the target's own direction.
    assert!(
        spins
            .iter()
            .all(|v: &&ReadValue<T, A>| v.coord == Coord::SPIN),
        "a mate's chart is spun about the target's own direction alone"
    );
    // Slides along carried target axes alone are admitted where they span
    // what the mate leaves, decided once from the source's geometry.
    // Slides along carried target axes, each its axis and its value: the
    // slide along the mated direction is the carried axis `S` puts on it
    // where `n` is a target axis, `about = ±S·e_j` exactly, its value
    // signed so, which keeps a frame lane on the chart's closed form.
    let carried: Option<Vec<(usize, T)>> = slides
        .iter()
        .map(|v| match v.coord {
            Coord::Slide(Some(a)) => Some((a.index(), v.by)),
            Coord::Slide(None) => {
                let j = chart.aligned?;
                // `n_j` is ±1 exactly, and `about` is `d` or `−d`
                // exactly: their signs read the same at every scalar.
                let along_d = [chart.n.x, chart.n.y, chart.n.z][j].quoted() > 0.0;
                let same = along_d == (chart.about.get().dot(chart.d.get()).quoted() > 0.0);
                Some((j, if same { v.by } else { T::zero() - v.by }))
            }
            _ => None,
        })
        .collect::<Option<Vec<_>>>()
        .filter(|pairs| {
            pairs
                .iter()
                .enumerate()
                .all(|(k, (a, _))| pairs[..k].iter().all(|(b, _)| b != a))
        });
    if let Some(pairs) = &carried {
        let written: Vec<usize> = pairs.iter().map(|&(a, _)| a).collect();
        let spanned = match written.as_slice() {
            [] | [_, _, _] => true,
            [a] => chart.spans(*a),
            [a, b] => chart.spans(3 - a - b),
            _ => unreachable!("the bundle rule admits as many slides as the mates leave"),
        };
        if !spanned {
            return Err(slides[0].value.no_chart(NoChartWhy::SlidesDoNotSpan {
                along: chart.aligned.map(TargetAxis::of_index),
                write: super::turn::slides_left_along(written.len(), chart.n),
            }));
        }
    }
    let spun = spins.clone();
    // The spin first: the slides after it re-measure the copy's origin.
    for v in spins {
        let measured = chart.spin(&m.linear());
        // A turn about the copy's own origin keeps its translation as it
        // is: composed as a turn about that point, a box lane would
        // widen it by the turn's own width levered over the origin's
        // distance, twice.
        let turned = match residual_turn_point(left).or(chart.line()) {
            Some(at) => turn_to(at, chart.about, v, measured)?
                .compose(m)
                .map_err(motion)?,
            None => turn_to(Point3::origin(), chart.about, v, measured)?
                .turn_alone()
                .compose(m.turn_alone())
                .and_then(|turn| turn.standing_at(m.origin()))
                .map_err(motion)?,
        };
        m = if T::spins_by_frames() {
            chart
                .turned(v.angle())?
                .standing_at(turned.origin())
                .map_err(motion)?
        } else {
            turned
        };
        left = after_turn(left).unwrap_or(left);
    }
    // On a frame lane, where `n` is a target axis and the slides run
    // along carried axes, the translation is set from the chart, as the
    // turn is: its component along each slide's axis is the value, along
    // the axis the mate fixes it is the mate's closed form
    // ([`Chart::seated`]), and along any other it is the copy's measure
    // there, so no difference of the translation with itself widens it.
    // A measure is re-read only on a chart whose mated direction is an
    // axis exactly: re-reading a component along a direction the box
    // widens would spread the translation's width across the others,
    // which the differences below leave in place.
    if let Some(pairs) = &carried {
        let set = |a: usize| {
            pairs
                .iter()
                .find(|&&(w, _)| w == a)
                .map(|&(_, by)| by)
                .or_else(|| chart.seated(&m.linear(), a))
        };
        if T::spins_by_frames()
            && !slides.is_empty()
            && chart.aligned.is_some()
            && (chart.on_axes() || (0..3).all(|a| set(a).is_some()))
        {
            let at = |a: usize| set(a).unwrap_or_else(|| Chart::slide(&m, chart.carried(a)));
            let shift = (0..3).fold(Vec3::zero(), |t, a| t + chart.carried(a) * at(a));
            // A copy turned by the spin is composed as the motion it is:
            // the shift, then the spin, then the constant turn.
            if let [spin] = spun.as_slice()
                && let Some(head) = chart.head_at(spin.angle(), shift)
            {
                return head.compose(chart.least()).map_err(motion);
            }
            return m
                .standing_at(Point3::new(shift.x, shift.y, shift.z))
                .map_err(motion);
        }
    }
    if slides.is_empty() {
        return Ok(m);
    }
    // The slides' directions in what the mate leaves, and where the
    // copy's origin stands along them: the copy stands where the mate
    // seats it plus the sum of its slides, each its value along its
    // direction ([`Chart::slide_direction`]).
    let refuse = |value: A, why: NoChartWhy| value.no_chart(why);
    let heads = slides
        .iter()
        .map(|v| {
            chart
                .slide_direction(&v.coord, v.line)
                .map_err(|why| refuse(v.value, why))
        })
        .collect::<Result<Vec<_>, _>>()?;
    // Carried axes alone were admitted by their span above, decided once
    // from the source's geometry (their parts in what the mate leaves
    // are independent exactly where they span it); any other set is
    // decided here, at evaluation, since a read moves with its body.
    if carried.is_none() && !chart.independent(&heads) {
        let why = if slides
            .iter()
            .any(|v| matches!(v.coord, Coord::SlideAlong(_)))
        {
            NoChartWhy::ReadSlidesDoNotSpan
        } else {
            NoChartWhy::SlidesDoNotSpan {
                along: chart.aligned.map(TargetAxis::of_index),
                write: super::turn::slides_left_along(slides.len(), chart.n),
            }
        };
        return Err(refuse(slides[0].value, why));
    }
    let Some(measured) = coordinates(&heads, m.origin() - Point3::origin()) else {
        return Err(refuse(slides[0].value, NoChartWhy::ReadSlidesDoNotSpan));
    };
    let shift = heads
        .iter()
        .zip(&slides)
        .zip(measured)
        .fold(Vec3::zero(), |t, ((h, v), at)| t + h.dir * (v.by - at));
    // The translation composed onto the copy as a motion: the product the
    // slides have always been placed by, identity part and all.
    RigidMotion::translation(shift)
        .and_then(|slid| slid.compose(m))
        .map_err(motion)
}

/// **The two target axes a face's chart slides along under `orient`**
/// ([`crate::pose::PoseDef::FaceChart`]): those the plane mate leaves
/// across where the orientation puts the copy's `z`, in `x`, `y`, `z`
/// order, as a sketch's plane placement writes them.
#[must_use]
pub fn face_chart_slides(orient: Orientation) -> [TargetAxis; 2] {
    let along = super::turn::slides_left_along(
        crate::SubgroupFamily::Planar.slides(),
        orient.apply(Vec3::<f64>::unit_z()),
    );
    let mut axes = along.axes();
    match (axes.next(), axes.next(), axes.next()) {
        (Some(a), Some(b), None) => [a, b],
        _ => unreachable!("a plane mate leaves two slides across a target axis"),
    }
}

/// The site a face's chart's least turn is decided under where it is
/// selected before the extent it is consumed over exists
/// ([`Chart::selected`]): its pole verdict over the session box.
pub(crate) const FACE_CHART_SELECTION_POLE: &str = "face_chart_selection_pole";

/// **A face's chart, undecided** ([`crate::pose::PoseDef::FaceChart`];
/// D10 `[placement]`): what the motion a plane placement onto `to`, the
/// face's plane in its space, places its copy at is computed from, under
/// `orient`, its spin (its number and the fraction of a turn it is
/// written as) and its slides (along [`face_chart_slides`]). The copy's
/// mated pose is its own `xy` facing its own `z`, a start cap flipped to
/// look up its `z` as DM1's placement mates one, so the frame's `z` is
/// the face's outward normal.
///
/// It decides nothing alone: what stands in it is carried over its own
/// reach, as a placement's copy is. The first carriage mints its motion
/// ([`Self::mint`]); every later one decides its verdict alone
/// ([`MintedChart::carries`]).
#[derive(Debug, Clone)]
pub(crate) struct FaceChart<T: geom_core::Real> {
    to: PoseValue<T>,
    orient: Orientation,
    spin: (T, Option<crate::expr::Ratio>),
    slides: [T; 2],
}

/// **A face's chart, minted at its first carriage**: its inputs and the
/// one motion they place a copy at. No later carriage mints another.
#[derive(Debug, Clone)]
pub(crate) struct MintedChart<T: geom_core::Real> {
    chart: FaceChart<T>,
    motion: std::sync::Arc<RigidMotion<T>>,
}

impl<T: SolveScalar> FaceChart<T> {
    /// The chart onto `to` under `orient`, `spin` and `slides`.
    pub(crate) fn new(
        to: PoseValue<T>,
        orient: Orientation,
        spin: (T, Option<crate::expr::Ratio>),
        slides: [T; 2],
    ) -> Self {
        Self {
            to: super::solve::tightened(to),
            orient,
            spin,
            slides,
        }
    }

    /// The copy's mated plane, its own `xy`.
    fn on() -> PoseValue<T> {
        PoseValue::Plane {
            normal: UnitVec3::exact_z(),
            offset: T::zero(),
        }
    }

    /// The chart's values, as a placement's fold reads its own.
    fn values(&self) -> [ReadValue<T, FaceValue>; 3] {
        let [a, b] = face_chart_slides(self.orient);
        let value = |value, coord, by, turns| ReadValue {
            value,
            coord,
            by,
            turns,
            line: None,
        };
        [
            value(FaceValue::Spin, Coord::SPIN, self.spin.0, self.spin.1),
            value(FaceValue::Slide(a), Coord::slide(a), self.slides[0], None),
            value(FaceValue::Slide(b), Coord::slide(b), self.slides[1], None),
        ]
    }

    /// **The chart, decided over what reaches `reach`** from its origin:
    /// the placement's arm ([`super::solve::arm_of`], over `reach`, the
    /// slides and the face's distance from the origin) and its chart
    /// ([`Chart::read`]), whose least turn's pole verdict and conditioning
    /// are every decision the chart's arm enters.
    fn decided(
        &self,
        reach: Result<f64, super::LeverRefusal>,
        band: Band,
        tol: geom_core::Tol,
    ) -> Result<Chart<T>, NodeErrorKind> {
        use crate::pose::PoseConstruction;
        let refuse = |construction| NodeErrorKind::PoseDegenerate { construction };
        let unleverable = |refusal| {
            refuse(PoseConstruction::FaceChartUnleverable {
                refusal: Box::new(refusal),
            })
        };
        let on = Self::on();
        let arm = super::solve::arm_of(
            reach.map_err(unleverable)?,
            [(&on, &self.to)],
            &self.values(),
        )
        .map_err(unleverable)?;
        match Chart::read(&on, &self.to, false, self.orient, arm, band, tol) {
            Ok(Some(chart)) => Ok(chart),
            Ok(None) => unreachable!("a plane mate fixes a direction"),
            Err(ChartFault::Backwards { write }) => {
                Err(refuse(PoseConstruction::FaceChartBackwards {
                    orient: self.orient,
                    write,
                }))
            }
            Err(ChartFault::Kind(kind)) => Err(*kind),
        }
    }

    /// **The chart's motion, selected before what it carries exists**: a
    /// trace reads its faces through the turn before the profile's loops
    /// resolve, so the turn is a selection (the geom-core README, "Float
    /// rounding"): its pole verdict alone ([`Chart::selected`]), over the
    /// session box's reach about the frame's origin through the
    /// placement's arm ([`super::solve::arm_of`]), and its conditioning
    /// left to the carriages that consume it ([`MintedChart::carries`]).
    /// The origin stands `|o| = √(offset² + |s|²)` from the space's (the
    /// face's plane's foot, and the slides across it), which needs no
    /// turn. The same motion [`Self::mint`] makes, every bit.
    ///
    /// # Errors
    ///
    /// As [`Self::mint`], less the conditioning's.
    pub(crate) fn select(self, band: Band) -> Result<MintedChart<T>, NodeErrorKind> {
        use crate::pose::PoseConstruction;
        let refuse = |construction| NodeErrorKind::PoseDegenerate { construction };
        let unleverable = |refusal| {
            refuse(PoseConstruction::FaceChartUnleverable {
                refusal: Box::new(refusal),
            })
        };
        let on = Self::on();
        let foot = super::solve::pose_origin(&self.to) - Point3::origin();
        let origin = (foot.norm_squared()
            + self.slides[0] * self.slides[0]
            + self.slides[1] * self.slides[1])
            .sqrt()
            .upper();
        let extent = geom_core::tolerance::SESSION_BOX_EXTENT;
        let reach = 3f64.sqrt() * extent + origin;
        let arm =
            super::solve::arm_of(reach, [(&on, &self.to)], &self.values()).map_err(unleverable)?;
        let chart = match Chart::selected(&on, &self.to, self.orient, arm, band) {
            Ok(Some(chart)) => chart,
            Ok(None) => unreachable!("a plane mate fixes a direction"),
            Err(ChartFault::Backwards { write }) => {
                return Err(refuse(PoseConstruction::FaceChartBackwards {
                    orient: self.orient,
                    write,
                }));
            }
            Err(ChartFault::Kind(kind)) => return Err(*kind),
        };
        self.settled(chart)
    }

    /// The coset and the values settled on `chart`, the motion.
    fn settled(self, chart: Chart<T>) -> Result<MintedChart<T>, NodeErrorKind> {
        let on = Self::on();
        let Some(coset) =
            super::solve::fold_cosets([(&on, &self.to)]).map_err(NodeErrorKind::Motion)?
        else {
            unreachable!("one mate meets the unconstrained coset")
        };
        let motion = settle(coset, &self.values(), Some(chart))?;
        Ok(MintedChart {
            chart: self,
            motion: std::sync::Arc::new(motion),
        })
    }

    /// **The chart's motion, minted at its first carriage**, of what
    /// reaches `reach`: decided as [`MintedChart::carries`] decides, then the
    /// mate's coset ([`super::solve::fold_cosets`]) and the values settled
    /// on it ([`settle`]), the calls a placement's fold makes.
    ///
    /// # Errors
    ///
    /// [`crate::pose::PoseConstruction::FaceChartBackwards`] within the
    /// band about the least turn's pole, naming the orientation to write;
    /// [`crate::pose::PoseConstruction::FaceChartUnleverable`] for a
    /// carried body with no reach, or an arm past the format's reach; a
    /// direction the band cannot mint; a motion's overflow.
    pub(crate) fn mint(
        self,
        reach: Result<f64, super::LeverRefusal>,
        band: Band,
        tol: geom_core::Tol,
    ) -> Result<MintedChart<T>, NodeErrorKind> {
        let chart = self.decided(reach, band, tol)?;
        self.settled(chart)
    }
}

impl<T: geom_core::Real> MintedChart<T> {
    /// The motion its first carriage minted.
    pub(crate) fn motion(&self) -> &std::sync::Arc<RigidMotion<T>> {
        &self.motion
    }
}

impl<T: SolveScalar> MintedChart<T> {
    /// **Whether the chart carries what reaches `reach`** from its origin:
    /// its verdict over that reach alone. The settle reads no arm on a
    /// face's chart (its slides run along carried target axes, exactly),
    /// so the chart's own decisions are all a carriage owes.
    ///
    /// # Errors
    ///
    /// As [`FaceChart::mint`], less a motion's.
    pub(crate) fn carries(
        &self,
        reach: Result<f64, super::LeverRefusal>,
        band: Band,
        tol: geom_core::Tol,
    ) -> Result<(), NodeErrorKind> {
        self.chart.decided(reach, band, tol).map(drop)
    }
}

/// **The turn about the line through `at` along `d` that takes a copy
/// spun `measured` to `v`'s spin**: by the difference of the two, or,
/// for a spin written as a fraction of a turn, the measured spin undone
/// and then the written turn made, each from its own sine and cosine,
/// so the written one is exact where its form is. A measured spin that
/// is exactly zero has nothing to undo.
fn turn_to<T: SolveScalar, A>(
    at: Point3<T>,
    d: UnitVec3<T>,
    v: &ReadValue<T, A>,
    measured: T,
) -> Result<RigidMotion<T>, NodeErrorKind> {
    match v.turns {
        Some(_) if measured.exactly_zero() => RigidMotion::rotation_about_axis_by(at, d, v.angle()),
        Some(_) => RigidMotion::rotation_about_axis_by(at, d, v.angle()).and_then(|written| {
            written.compose(RigidMotion::rotation_about_axis_by(
                at,
                d,
                Angle::radians(-measured),
            )?)
        }),
        None => RigidMotion::rotation_about_normalized_axis(at, d.get(), v.by - measured),
    }
    .map_err(NodeErrorKind::Motion)
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
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::{Chart, Coord, ReadValue, RigidMotion, Seat, settle};
    use crate::mate::coset::{Arm, Coset, Subgroup};
    use crate::mate::{MateRef, Orientation, TargetAxis};
    use crate::node::{ConstraintId, RecipeNodeId};
    use geom_core::linalg::{Affine3, Point3, UnitVec3, Vec3};
    use geom_core::predicate::Band;

    /// **On axes that lie in what the mate leaves, the sum of the slides
    /// is the copy's origin measured along them, bit for bit**: a rest on
    /// a plane tilted off every target axis, the copy's face on its own
    /// `z`, spun and slid along both carried axes, settles where the
    /// projection reading `t + Σ c_a (v_a − t·c_a)` puts it — every
    /// bit, since a bundle written on target axes before the sum reading
    /// must stand where it stood. *Breaks if* a carried axis in the plane
    /// is run through the Gram system, whose solve moves the low bits.
    #[test]
    fn the_sum_on_a_tilted_planes_carried_axes_is_the_projection_bit_for_bit() {
        let band = Band::linear(geom_core::Tol::witness()).expect("a band");
        let unit = |v: Vec3<f64>| UnitVec3::new(v, topo::query::POSE_UNIT_NORM, band).unwrap();
        let d = unit(Vec3::new(0.3, -0.2, 0.9));
        let chart = Chart::charted(
            d,
            UnitVec3::exact_z(),
            Seat::Depth(0.25),
            false,
            Orientation::IDENTITY,
            Arm::of(0.5, 0.0).unwrap(),
            geom_core::Lever::session_box(),
            band,
            Some(geom_core::Tol::witness()),
        )
        .expect("a chart off the pole");
        let value = |k: u32, coord, by| ReadValue {
            value: MateRef::new(RecipeNodeId::WORLD, ConstraintId(crate::MintId::new(k, 1))),
            coord,
            by,
            turns: None,
            line: None,
        };
        let values = [
            value(1, Coord::SPIN, 0.7),
            value(2, Coord::slide(TargetAxis::X), 0.031),
            value(3, Coord::slide(TargetAxis::Y), -0.0175),
        ];
        // One admitted pose of the rest: the least turn, its depth along
        // `d` and an in-plane offset.
        let seat = d.get() * 0.25 + chart.carried(0) * 0.4;
        let start = chart
            .least()
            .standing_at(Point3::new(seat.x, seat.y, seat.z))
            .unwrap();
        let coset = Coset {
            subgroup: Subgroup::Planar { normal: d },
            representative: start,
        };
        let at = settle(coset, &values, Some(chart))
            .expect("the bundle settles")
            .to_affine();
        // The spin alone, then the projection reading over its pose.
        let spun = settle(coset, &values[..1], Some(chart)).expect("the spin settles");
        let t = spun.to_affine().translation;
        let shift = [(0, 0.031), (1, -0.0175)]
            .into_iter()
            .fold(Vec3::zero(), |s, (a, v)| {
                s + chart.carried(a) * (v - Chart::<f64>::slide(&spun, chart.carried(a)))
            });
        let want = RigidMotion::translation(shift)
            .unwrap()
            .compose(spun)
            .unwrap()
            .to_affine();
        let bits = |m: &Affine3<f64>| {
            let l = m.linear;
            [
                m.translation.x,
                m.translation.y,
                m.translation.z,
                l.c0.x,
                l.c0.y,
                l.c0.z,
                l.c1.x,
                l.c1.y,
                l.c1.z,
                l.c2.x,
                l.c2.y,
                l.c2.z,
            ]
            .map(f64::to_bits)
        };
        assert_eq!(
            bits(&at),
            bits(&want),
            "the settled map, {at:?} against {want:?}"
        );
        assert!(
            (t - at.translation).norm() > 1e-3,
            "the slides moved the copy: the row is not vacuous"
        );
    }

    /// **Beside a mate whose direction is oblique in the source's own
    /// coordinates, a carried slide moves the copy exactly its value, in
    /// what the mate leaves**: a rest of a face whose normal is no axis
    /// of the source, its carried `x` and `y` running along their parts
    /// in the plane. Moving one slide by `δ` moves the copy `δ` along
    /// that part, and the copy stays on the plane. *Breaks if* a carried
    /// axis off the plane is read as the copy's origin measured along
    /// it, which steps `δ` by more than `δ`.
    #[test]
    fn an_oblique_charts_carried_slide_moves_its_value_along_its_part_in_the_plane() {
        let band = Band::linear(geom_core::Tol::witness()).expect("a band");
        let unit = |v: Vec3<f64>| UnitVec3::new(v, topo::query::POSE_UNIT_NORM, band).unwrap();
        let d = unit(Vec3::new(0.0, 0.0, 1.0));
        let copy_d = unit(Vec3::new(1.0, 1.0, 1.0));
        let chart = Chart::charted(
            d,
            copy_d,
            Seat::Depth(0.25),
            false,
            Orientation::IDENTITY,
            Arm::of(0.5, 0.0).unwrap(),
            geom_core::Lever::session_box(),
            band,
            Some(geom_core::Tol::witness()),
        )
        .expect("a chart off the pole");
        assert!(
            chart.aligned.is_none(),
            "the source's mated direction is no axis"
        );
        let seat = d.get() * 0.25;
        let start = chart
            .least()
            .standing_at(Point3::new(seat.x, seat.y, seat.z))
            .unwrap();
        let coset = Coset {
            subgroup: Subgroup::Planar { normal: d },
            representative: start,
        };
        let at = |x: f64, y: f64| {
            let values = [
                ReadValue {
                    value: MateRef::new(
                        RecipeNodeId::WORLD,
                        ConstraintId(crate::MintId::new(1, 1)),
                    ),
                    coord: Coord::slide(TargetAxis::X),
                    by: x,
                    turns: None,
                    line: None,
                },
                ReadValue {
                    value: MateRef::new(
                        RecipeNodeId::WORLD,
                        ConstraintId(crate::MintId::new(2, 1)),
                    ),
                    coord: Coord::slide(TargetAxis::Y),
                    by: y,
                    turns: None,
                    line: None,
                },
            ];
            settle(coset, &values, Some(chart))
                .expect("the slides span")
                .to_affine()
                .translation
        };
        let (zero, x, y) = (at(0.0, 0.0), at(0.02, 0.0), at(0.0, -0.03));
        for (moved, by, label) in [(x - zero, 0.02, "x"), (y - zero, 0.03, "y")] {
            assert!(
                (moved.norm() - by).abs() < 1e-12,
                "the slide along {label} moves the copy {}, not {by}",
                moved.norm()
            );
            assert!(
                moved.dot(d.get()).abs() < 1e-12,
                "the slide along {label} stays on the plane"
            );
        }
        assert!(
            (zero.dot(d.get()) - 0.25).abs() < 1e-12,
            "the copy rests on the plane"
        );
        // Its zero is where the mate seats it: the plane's point nearest
        // the target's origin.
        assert!(
            (zero - d.get() * 0.25).norm() < 1e-12,
            "zero is the seat: {zero:?}"
        );
    }

    /// **At the dual scalar a chart whose mated directions agree only in
    /// value keeps its least turn's derivative**: the copy's direction `x̂`
    /// and the target's `(cos t, sin t, 0)` seeded at `t = 0`. The chart's
    /// least turn is the rotation about `z` by `t`, so its carried `x` axis
    /// moves at `ŷ` and its carried `y` axis at `−x̂`. Main's chart took the
    /// identity there on the value channel alone (`mate::turn::zero`) and
    /// dropped that derivative. *Breaks if* the identity test reads the
    /// value channel alone.
    #[test]
    fn a_dual_chart_one_direction_only_in_value_keeps_its_derivative() {
        use geom_core::Dual;
        let band = Band::linear(geom_core::Tol::witness()).expect("a band");
        let d = |v: f64, dv: f64| Dual {
            value: v,
            deriv: dv,
        };
        let zero = d(0.0, 0.0);
        let unit =
            |v| UnitVec3::new(v, topo::query::POSE_UNIT_NORM, band).expect("a unit direction");
        let copy = unit(Vec3::new(d(1.0, 0.0), zero, zero));
        let target = unit(Vec3::new(d(1.0, 0.0), d(0.0, 1.0), zero));
        let chart = Chart::charted(
            target,
            copy,
            Seat::Free,
            false,
            Orientation::IDENTITY,
            Arm::of(0.5, 0.0).unwrap(),
            geom_core::Lever::session_box(),
            band,
            Some(geom_core::Tol::witness()),
        )
        .expect("a chart off the pole");
        let deriv = |v: Vec3<Dual<f64>>| [v.x.deriv, v.y.deriv, v.z.deriv];
        assert_eq!(deriv(chart.carried(0)), [0.0, 1.0, 0.0], "S'·x̂ is ŷ");
        assert_eq!(deriv(chart.carried(1)), [-1.0, 0.0, 0.0], "S'·ŷ is −x̂");
    }

    /// The band-edge fixture: a target direction `d` oblique to every
    /// axis, and a copy direction `n` that is `−d` tilted by `tau` about
    /// an oblique axis `w` across `d`, so the chord `|n + d|` is
    /// `2·sin(tau/2)`. Returns the chart, or `None` where it refuses,
    /// with `n`, `d` and `w` as `f64`.
    fn band_edge_chart<T: crate::mate::SolveScalar>(
        tau: f64,
        band: Band,
    ) -> (Option<Chart<T>>, [Vec3<f64>; 3]) {
        let norm = |v: Vec3<f64>| v * (1.0 / v.norm());
        let d = norm(Vec3::new(0.3, -0.2, 0.9));
        let w = norm(d.cross(Vec3::new(1.0, 2.0, 3.0)));
        let n = -d * tau.cos() + w.cross(-d) * tau.sin();
        let lift = |v: Vec3<f64>| {
            UnitVec3::new(
                Vec3::new(
                    T::from_computed(v.x),
                    T::from_computed(v.y),
                    T::from_computed(v.z),
                ),
                topo::query::POSE_UNIT_NORM,
                band,
            )
            .expect("a unit direction")
        };
        let arm = 0.5;
        // The fixture's stated arm, as `Chart::of` levers the chart over
        // the bundle's `Arm`.
        let chart = Chart::charted(
            lift(d),
            lift(n),
            Seat::Free,
            false,
            Orientation::IDENTITY,
            Arm::of(arm, 0.0).unwrap(),
            geom_core::test_support::fixture_lever(T::from_computed(arm)),
            band,
            Some(geom_core::Tol::witness()),
        )
        .ok();
        (chart, [n, d, w])
    }

    /// The tilt at which the chart's refusal ends, by bisection on a log
    /// scale: the largest refusing tilt and the smallest building one.
    fn band_edge<T: crate::mate::SolveScalar>(band: Band) -> (f64, f64) {
        let (mut lo, mut hi) = (1e-14_f64, 1e-1_f64);
        assert!(band_edge_chart::<T>(lo, band).0.is_none());
        assert!(band_edge_chart::<T>(hi, band).0.is_some());
        for _ in 0..80 {
            let mid = (lo * hi).sqrt();
            if band_edge_chart::<T>(mid, band).0.is_some() {
                hi = mid
            } else {
                lo = mid
            }
        }
        (lo, hi)
    }

    /// The least turn's image of `v` read off the chart's carried axes,
    /// with `v` lifted into the chart's scalar.
    fn turned<T: crate::mate::SolveScalar>(chart: &Chart<T>, v: Vec3<f64>) -> Vec3<T> {
        let s = chart.least();
        let lift = |x: f64| T::from_computed(x);
        s.u().get() * lift(v.x) + s.v().get() * lift(v.y) + s.w().get() * lift(v.z)
    }

    /// **At `f64` the chart's least turn lands within ε at its band's
    /// edge, at the `2u/chord` rate**: an oblique target and a copy
    /// direction tilted off its pole about an oblique axis, the tilt
    /// bisected to where the refusal ends. Just past it, `S` carries `n`
    /// onto `d` and keeps the turn's axis `w` fixed (the two facts that
    /// make a rotation the least turn), each within ε and within
    /// `2u/|n + d|`, the rate the module doc states. *Breaks if* the
    /// conditioning admits a chord whose turn rounds past ε, or the
    /// spelling near the pole is one whose error does not scale as the
    /// chord does.
    #[test]
    fn the_f64_charts_least_turn_lands_within_eps_at_its_band_edge() {
        let band = Band::linear(geom_core::Tol::witness()).expect("a band");
        let eps = geom_core::Tol::witness().get().eps;
        let (lo, hi) = band_edge::<f64>(band);
        assert!(
            band_edge_chart::<f64>(lo, band).0.is_none(),
            "inside the band it refuses"
        );
        for tau in [hi, hi * 1.2, hi * 10.0, hi * 1e3] {
            let (chart, [n, d, w]) = band_edge_chart::<f64>(tau, band);
            let chart = chart.expect("past the band it charts");
            let chord = (n + d).norm();
            let rate = f64::EPSILON / chord;
            let onto = (turned(&chart, n) - d).norm();
            let axis = (turned(&chart, w) - w).norm();
            for (what, defect) in [("S·n − d", onto), ("S·w − w", axis)] {
                assert!(
                    defect <= eps && defect <= rate,
                    "at tilt {tau:e} (chord {chord:e}) |{what}| = {defect:e}: past ε {eps:e} or the rate {rate:e}"
                );
            }
        }
    }

    /// **At `Interval` the chart's least turn encloses the exact image at
    /// its band's edge, and is wider than ε there**: the fixture of
    /// [`the_f64_charts_least_turn_lands_within_eps_at_its_band_edge`].
    /// `S·n` contains `d` at every tilt past the edge, and its widest
    /// component stays within `32u/|n + d|`, at every ε. At the edge itself,
    /// at the default ε, that width is about 12ε, wider than ε, which
    /// `work/onehome/the-interval-charts-least-turn-is-wider-than-eps-at-its-band-edge.md`
    /// records; this row pins both until that row moves the edge.
    /// *Breaks if* the enclosure loses `d` (unsound), widens past the
    /// rate, or narrows to within ε at the edge (close the row).
    #[test]
    fn the_interval_charts_least_turn_encloses_its_image_at_its_band_edge() {
        use geom_core::Bounds;
        let band = Band::linear(geom_core::Tol::witness()).expect("a band");
        let eps = geom_core::Tol::witness().get().eps;
        let (lo, hi) = band_edge::<geom_core::Interval>(band);
        assert!(
            band_edge_chart::<geom_core::Interval>(lo, band).0.is_none(),
            "inside the band it refuses"
        );
        let width = |tau: f64| {
            let (chart, [n, d, _]) = band_edge_chart::<geom_core::Interval>(tau, band);
            let image = turned(&chart.expect("past the band it charts"), n);
            let chord = (n + d).norm();
            for (c, v) in [(image.x, d.x), (image.y, d.y), (image.z, d.z)] {
                assert!(
                    c.lo() <= v && v <= c.hi(),
                    "at tilt {tau:e} {c:?} loses {v}"
                );
            }
            let wide = [image.x, image.y, image.z]
                .map(|c| c.hi() - c.lo())
                .into_iter()
                .fold(0.0, f64::max);
            let rate = 32.0 * f64::EPSILON / 2.0 / chord;
            assert!(
                wide <= rate,
                "at tilt {tau:e} the width {wide:e} passes the rate {rate:e}"
            );
            wide
        };
        for tau in [hi * 1.2, hi * 10.0, hi * 1e3] {
            width(tau);
        }
        let at_edge = width(hi);
        // The row measured the edge at the default ε; elsewhere the edge
        // sits where another verdict binds, and only the rate is pinned.
        if eps == geom_core::tolerance::DEFAULT_EPS {
            assert!(
                at_edge > eps,
                "the width at the edge, {at_edge:e}, is within ε {eps:e}: close the row this pins"
            );
        }
    }
}
