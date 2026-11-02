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
//! slide is the copy's origin measured from the target space's along a
//! target axis as `S` carries it. Each is a
//! function of the copy's relative pose alone, so no order of the values
//! is chosen, and nothing about the body the target pose is read off
//! enters the chart.
//!
//! The least turn has no answer at its one pole, `n = −d`, the
//! orientation written backwards; inside the band about it the chart
//! refuses ([`super::MateFault::Backwards`]), naming the orientation to
//! write. Short of the pole `S` is smooth in `d`, so a tilt of the
//! target by a sliver moves a copy by a sliver.
//!
//! [`settle`] places a folded bundle's copy at its values: the spin
//! turns the copy about `a` inside the bundle's residual, then the
//! slides set its origin inside what the mates leave of its
//! translation. Where `n` is a target axis, the slide along it is the
//! mate's, and the others run across `d`; otherwise the written slides
//! are admitted only where they span what the mate leaves, decided once
//! from the source's geometry ([`super::NoChartWhy::SlidesDoNotSpan`]).
//! The values are taken in one order whatever order they are listed in,
//! so the listed order moves no bit. Where no direction is fixed, the
//! spins are a chain the author orders ([`chain`]): about the copy's
//! origin with no mate ([`unmated_motion`]), about the mated point
//! beside a lone point mate.

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
    /// The target axes as the least turn carries them: `S·x`, `S·y`,
    /// `S·z`.
    carried: [Vec3<T>; 3],
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

use super::turn::zero;

/// **Whether `pose` reads the opposite sense of its own direction**: an
/// odd count of flips along the poses it keeps, read plainly or through
/// a role that keeps a sense (a plane's normal, an axis's direction).
/// The one spin turns about the target's own direction, so a flip on
/// either side of a mate moves no spin.
#[must_use]
pub fn flipped<P>(doc: &crate::doc::Doc<P>, pose: crate::VarId) -> bool {
    use crate::pose::{PoseDef, PoseRole, RoleRead};
    let mut odd = false;
    let mut at = pose;
    while let Some(def) = doc.var(at).and_then(|v| v.def().pose()) {
        odd ^= matches!(def, PoseDef::Flip { .. });
        match def.kept() {
            Some(RoleRead {
                read,
                role: None | Some(PoseRole::Normal | PoseRole::Direction),
            }) => at = *read,
            _ => break,
        }
    }
    odd
}

/// `v` turned by the angle whose sine and cosine are given about the
/// unit direction `d`.
fn rotate<T: geom_core::Real>(d: Vec3<T>, v: Vec3<T>, sin: T, cos: T) -> Vec3<T> {
    let along = d * d.dot(v);
    v * cos + d.cross(v) * sin + along * (T::one() - cos)
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
    pub(crate) fn of(
        mate: MateRef,
        on: &PoseValue<T>,
        to: &PoseValue<T>,
        to_flipped: bool,
        orient: Orientation,
        arm: Arm,
        band: Band,
    ) -> Result<Option<Self>, NodeErrorKind> {
        let (d, copy_d, seat) = match (on, to) {
            (
                PoseValue::Plane {
                    normal: o,
                    offset: c_on,
                },
                PoseValue::Plane {
                    normal: t,
                    offset: c_to,
                },
            ) => (*t, *o, Seat::Depth(*c_to - *c_on)),
            (PoseValue::Direction { dir: o }, PoseValue::Direction { dir: t }) => {
                (*t, *o, Seat::Free)
            }
            (PoseValue::Axis(copy), PoseValue::Axis(target)) => (
                target.dir(),
                copy.dir(),
                Seat::Line {
                    foot: target.foot(),
                    copy_foot: copy.foot(),
                },
            ),
            _ => return Ok(None),
        };
        let unit = |v: Vec3<T>| {
            UnitVec3::new(v, topo::query::POSE_UNIT_NORM, band).map_err(|_| {
                NodeErrorKind::PoseDegenerate {
                    construction: crate::pose::PoseConstruction::Chart,
                }
            })
        };
        let n = orient.apply(copy_d.get());
        let dv = d.get();
        let one = T::one();
        let least = super::turn::LeastTurn::between(n, dv);
        // The chord from the pole: |n + d| = √(2(1 + c)), levered over
        // the arm.
        let chord = ((one + one) * least.one_plus_c()).sqrt();
        let backwards = || {
            let q = |v: Vec3<T>| Vec3::new(v.x.quoted(), v.y.quoted(), v.z.quoted());
            NodeErrorKind::Mate(Box::new(MateFault::Backwards {
                mate,
                orient,
                write: orient.toward(q(copy_d.get()), q(dv)),
            }))
        };
        let sign = geom_core::k_stats::decide(
            "mate_chart_pole",
            Margin::levered(chord, T::from_computed(arm.get())),
            band,
        )
        .map_err(|_| backwards())?;
        if sign == Sign::Zero {
            return Err(backwards());
        }
        let axes = [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()];
        // Where `n` is `d` exactly the least turn is the identity, and the
        // carried axes are the target's own, exactly, at every scalar.
        let carried = if [n.x - dv.x, n.y - dv.y, n.z - dv.z].into_iter().all(zero) {
            axes
        } else {
            let mut carried = axes.map(|e| least.apply(e));
            // Each carried axis is a turn of a unit vector, so unit over
            // the reals at every point a box spans: a frame lane keeps each
            // as it is, which a normalization would widen by its own norm's
            // width, and decides each direction as the value lane does.
            for axis in &mut carried {
                let minted = unit(*axis)?.get();
                if !T::spins_by_frames() {
                    *axis = minted;
                }
            }
            carried
        };
        let aligned = super::turn::aligned(n);
        let across = match aligned {
            Some(_) => super::turn::across(n),
            None => unit(super::turn::across(n))?.get(),
        };
        Ok(Some(Self {
            d,
            about: if to_flipped { -d } else { d },
            carried,
            n,
            aligned,
            orient,
            across,
            seat,
            arm,
            band,
        }))
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
                Some(across.dot(self.carried[a]))
            }
            Seat::Depth(_) | Seat::Line { .. } | Seat::Free => None,
        }
    }

    /// The spin of the copy at relative pose `m`: the angle about the
    /// target's own direction from the least turn's image of the
    /// reference direction to the copy's, as `m` carries it.
    fn spin(&self, m: &Affine3<T>) -> T {
        let to = self.carried_of(self.across);
        let copy = m.linear * self.orient.unapply(self.across);
        T::solve_atan2(to.cross(copy).dot(self.about.get()), to.dot(copy))
    }

    /// `S·v`, from the carried axes.
    fn carried_of(&self, v: Vec3<T>) -> Vec3<T> {
        self.carried[0] * v.x + self.carried[1] * v.y + self.carried[2] * v.z
    }

    /// `S·O`: the copy's axes carried onto the target's by the
    /// orientation and the least turn, column by column.
    fn least(&self) -> Mat3<T> {
        let col = |k: usize| {
            let a = match k {
                0 => self.orient.x(),
                1 => self.orient.y(),
                _ => self.orient.z(),
            };
            let v = self.carried[a.index()];
            if a.negative() { -v } else { v }
        };
        Mat3::from_cols(col(0), col(1), col(2))
    }

    /// The turn of every map whose spin is `by`: `rot(a, by)·S·O`, `a`
    /// the target's own direction.
    fn turned(&self, by: Angle<T>) -> Mat3<T> {
        let (sin, cos) = by.sin_cos();
        let d = self.about.get();
        let l = self.least();
        Mat3::from_cols(
            rotate(d, l.c0, sin, cos),
            rotate(d, l.c1, sin, cos),
            rotate(d, l.c2, sin, cos),
        )
    }

    /// **The constant turn `S·O`**, as a map.
    fn onto(&self) -> Affine3<T> {
        Affine3::from_parts(self.least(), Vec3::zero())
    }

    /// **The copy's map at spin `by` and the translation `shift` before
    /// [`Self::onto`]**: the shift, then the turn by `by` about the
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
    fn slide(m: &Affine3<T>, axis: Vec3<T>) -> T {
        m.translation.dot(axis)
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
                Margin::levered(comp, T::from_computed(self.arm.get())),
                self.band,
            ),
            Ok(Sign::Positive | Sign::Negative)
        )
    }
}

/// **A mate fixing a direction, as the chart reads it** ([`read_back`]):
/// the mate, its two poses read in the target space, whether its `to`
/// reads the opposite sense ([`flipped`]) and its orientation.
#[derive(Debug, Clone, Copy)]
pub struct Charted<'a> {
    /// The mate, named in a refusal.
    pub mate: MateRef,
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
/// along the target axes its least turn carries; with none, the slides
/// along the target space's axes and the chain of spins about them,
/// read where the chain is the three turns about `x`, `y` and `z` in
/// that order. A freedom the chart does not read at `at` — a link of a
/// chain in another order, a spin about the target's direction with no
/// mate fixing one, any freedom of a chart refused at its pole — reads
/// `None`. So a gesture that stands a copy somewhere writes the values
/// that place it there through the chart that will place it, never a
/// second spelling of it.
#[must_use]
pub fn read_back(
    chart: Option<Charted<'_>>,
    at: &Affine3<f64>,
    coords: &[Coord],
    band: Band,
) -> Vec<Option<f64>> {
    let Some(charted) = chart else {
        let chain = chain_read(&at.linear, coords);
        return coords
            .iter()
            .map(|coord| match *coord {
                Coord::Slide(axis) => Some([at.translation.x, at.translation.y, at.translation.z][axis.index()]),
                Coord::Spin(Some(axis)) => chain.map(|turns| turns[axis.index()]),
                Coord::Spin(None) => None,
            })
            .collect();
    };
    let arm = Arm::of(0.0, 0.0).ok();
    let read = arm.and_then(|arm| {
        Chart::<f64>::of(
            charted.mate,
            charted.on,
            charted.to,
            charted.to_flipped,
            charted.orient,
            arm,
            band,
        )
        .ok()
        .flatten()
    });
    coords
        .iter()
        .map(|coord| {
            let chart = read?;
            match *coord {
                Coord::Spin(None) => Some(chart.spin(at)),
                Coord::Slide(axis) => Some(Chart::slide(at, chart.carried[axis.index()])),
                Coord::Spin(Some(_)) => None,
            }
        })
        .collect()
}

/// **The chain `x`, `y`, `z` that turns as `r` does**, each about the
/// target space's own axis, the first written the first made ([`chain`]):
/// `r = Rz·Ry·Rx`. `None` where `coords` spells another chain.
fn chain_read(r: &Mat3<f64>, coords: &[Coord]) -> Option<[f64; 3]> {
    let spun: Vec<TargetAxis> = coords
        .iter()
        .filter_map(|coord| match coord {
            Coord::Spin(axis) => Some(*axis),
            Coord::Slide(_) => None,
        })
        .collect::<Option<_>>()?;
    if spun != TargetAxis::ALL {
        return None;
    }
    // Column k is `r·e_k`, so the entry at row i, column k is `r.ck[i]`.
    let x = r.c1.z.atan2(r.c2.z);
    let y = (-r.c0.z).atan2(r.c1.z.hypot(r.c2.z));
    let z = r.c0.y.atan2(r.c0.x);
    Some([x, y, z])
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
    /// The shift, then the spin about the target's own direction
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
        if let Coord::Spin(Some(axis)) = v.coord {
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
/// admitted every value's freedom (`super::solve::overconstrained`) and
/// as many slides as the mates leave translations, so each is held by
/// the residual by construction: a spin turns about the residual's own
/// line, or about the target's own direction through the copy's origin
/// where every line along it will do.
///
/// **The slides.** In the least turn's coordinates (`′`, so `S·v′ = v`)
/// the mate leaves the copy's translation the plane across `n` (or the
/// line along it, or all of it), and a slide along carried axis `a` is
/// the coordinate `t′_a`. So the written slides set their coordinates,
/// and the one left unwritten across a plane (or the line's length) is
/// what keeps the mate: `Δ′_c = −(n_a Δ′_a + n_b Δ′_b) / n_c` for a
/// plane, `Δ′ = n · Δ′_a / n_a` for a line. Where `n` is a target axis
/// the carried axes across it lie in the plane, the one along it is the
/// mate's, and the slides move the copy along their own axes alone.
///
/// # Errors
///
/// [`MateFault::NoChart`] where the written slides do not span what the
/// mate leaves ([`Chart::spans`]).
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
    // One order whatever order the values are listed in: the spin, then
    // slides along x, y and z, then by id.
    let mut ordered: Vec<&ReadValue<T>> = values.iter().collect();
    ordered.sort_by_key(|v| {
        let rank = match v.coord {
            Coord::Spin(_) => 0,
            Coord::Slide(a) => 1 + a.index(),
        };
        (rank, v.value.id)
    });
    let (spins, slides): (Vec<_>, Vec<_>) = ordered
        .into_iter()
        .partition(|v| matches!(v.coord, Coord::Spin(_)));
    // The rule admits the axisless spin alone here, held loud: the chart
    // turns about the target's own direction.
    assert!(
        spins.iter().all(|v: &&ReadValue<T>| v.coord == Coord::SPIN),
        "a mate's chart is spun about the target's own direction alone"
    );
    let along = |v: &ReadValue<T>| match v.coord {
        Coord::Slide(a) => a.index(),
        Coord::Spin(_) => unreachable!("the spins were taken above"),
    };
    // The span, decided once from the source's geometry.
    let written: Vec<usize> = slides.iter().map(|v| along(v)).collect();
    let spanned = match written.as_slice() {
        [] | [_, _, _] => true,
        [a] => chart.spans(*a),
        [a, b] => chart.spans(3 - a - b),
        _ => unreachable!("the bundle rule admits as many slides as the mates leave"),
    };
    if !spanned {
        return Err(NodeErrorKind::Mate(Box::new(MateFault::NoChart {
            value: slides[0].value,
            why: NoChartWhy::SlidesDoNotSpan {
                along: chart.aligned.map(TargetAxis::of_index),
                write: super::turn::slides_left_along(written.len(), chart.n),
            },
        })));
    }
    let spun = spins.clone();
    // The spin first: the slides after it re-measure the copy's origin.
    for v in spins {
        let measured = chart.spin(&m);
        // A turn about the copy's own origin keeps its translation as it
        // is: composed as a turn about that point, a box lane would
        // widen it by the turn's own width levered over the origin's
        // distance, twice.
        let turned = match residual_turn_point(left).or(chart.line()) {
            Some(at) => turn_to(at, chart.about, v, measured) * m,
            None => {
                let turn = turn_to(Point3::origin(), chart.about, v, measured);
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
    // On a frame lane, where `n` is a target axis, the translation is set
    // from the chart, as the turn is: its component along each slide's
    // axis is the value, along the axis the mate fixes it is the mate's
    // closed form ([`Chart::seated`]), and along any other it is the
    // copy's measure there, so no difference of the translation with
    // itself widens it. A measure is re-read only on a chart whose mated
    // direction is an axis exactly: re-reading a component along a
    // direction the box widens would spread the translation's width
    // across the others, which the differences below leave in place.
    let set = |a: usize| {
        slides
            .iter()
            .rev()
            .find(|v| along(v) == a)
            .map(|v| v.by)
            .or_else(|| chart.seated(&m.linear, a))
    };
    if T::spins_by_frames()
        && !slides.is_empty()
        && chart.aligned.is_some()
        && (chart.on_axes() || (0..3).all(|a| set(a).is_some()))
    {
        let at = |a: usize| set(a).unwrap_or_else(|| Chart::slide(&m, chart.carried[a]));
        let shift = (0..3).fold(Vec3::zero(), |t, a| t + chart.carried[a] * at(a));
        // A copy turned by the spin is composed as the motion it is: the
        // shift, then the spin, then the constant turn.
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
    // The written coordinates' gaps, in the least turn's coordinates.
    let gap = |v: &ReadValue<T>| v.by - Chart::slide(&m, chart.carried[along(v)]);
    let mut moved = [None::<T>; 3];
    for v in &slides {
        moved[along(v)] = Some(gap(v));
    }
    let n = [chart.n.x, chart.n.y, chart.n.z];
    match (written.as_slice(), chart.aligned) {
        // Across `n` where it is an axis: the mate's slide stays put.
        ([_, _], Some(_)) | ([_, _, _], _) | ([], _) => {}
        ([a, b], None) => {
            let c = 3 - a - b;
            let (ga, gb) = (
                moved[*a].unwrap_or(T::zero()),
                moved[*b].unwrap_or(T::zero()),
            );
            moved[c] = Some(T::zero() - (n[*a] * ga + n[*b] * gb) / n[c]);
        }
        ([a], _) => {
            let along_n = moved[*a].unwrap_or(T::zero()) / n[*a];
            moved = [0, 1, 2].map(|k| match chart.aligned {
                Some(j) if k != j => None,
                _ => Some(n[k] * along_n),
            });
        }
        _ => unreachable!("the bundle rule admits as many slides as the mates leave"),
    }
    let shift = (0..3).fold(Vec3::zero(), |t, a| match moved[a] {
        Some(by) => t + chart.carried[a] * by,
        None => t,
    });
    if !slides.is_empty() {
        m = Affine3::from_parts(Mat3::identity(), shift) * m;
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
