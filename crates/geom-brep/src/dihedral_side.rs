//! **The dihedral side**: which side of one face a second face leaves
//! a shared edge on, read from the two faces' outward normals at a point
//! of the edge, for every surface kind a face carries.
//!
//! # The reading
//!
//! At a point `p` of an edge two faces share, each face has an outward
//! normal, and a face leaves the edge into its own interior along
//! `d = n × t̂`, with `t̂` the edge's direction as that face's loop walks
//! it (outer loops run counterclockwise seen from outside, so the
//! interior lies to the left of the walk). That direction is an
//! [`EdgeWalk`], minted from the edge's curve and the half-edge that
//! walks it. The departing face lies on the side of the other face that
//! `d` points to, which is [`crate::enters_material`] of `d` against the
//! other face's outward normal: the sign of `t̂ · (n_against ×
//! n_departing)`, the dihedral's sine signed by the two faces' material,
//! metered over the folded lever arm in meters (D4 ¶1,
//! [`crate::dihedral`]'s module docs).
//!
//! The sign is a first-order fact. Where the two tangent planes coincide
//! it reads zero, and which side the face lies on is decided one order
//! up, by how the faces bend apart, which this door does not read: it
//! escalates [`SideEscalation::Tangent`] rather than guess, and an
//! in-band reading escalates through the lever's own rungs.
//!
//! # Normals on every kind
//!
//! An analytic face's normal is its implicit gradient at `p`
//! ([`crate::implicit_outward_normal`]); its lever arm is
//! [`crate::curvature_lever_arm`]. A spline face has no implicit form and
//! is given no signed residual (C5, the curve × face clause): its normal
//! is its chart's `S_u × S_v` at a foot of `p` decided on the CARRIER by
//! its distance within the band, and its arm is the radius of its
//! largest principal curvature there.
//!
//! **The foot is the carrier's, not the face's.** This door reads no
//! trim. A carrier that passes within the band of `p` on a sheet the face
//! does not use (one folded back over the face, or another span of a
//! carrier that comes round) can give the foot on that sheet, whose
//! normal is not the face's. Placing the foot in the face is the face's
//! trim's reading (C5: "placed in or out of the face by the face's
//! trim"), so a caller consumes this door's side only once the foot is
//! placed in the face.
//!
//! **The chart's normal is read only where it is defined and
//! continuous.** At an irregular point of the chart (`S_u × S_v` zero:
//! coincident control rows, a pole) the normal has no value, and across a
//! C⁰ line of the chart — an interior knot run of multiplicity at least
//! the degree ([`geom_core::spline::KnotStructure::crease_runs`]), or a
//! closed chart's seam where its two domain ends meet — it has a value on
//! each side, and the jet at a foot on the line reads one of them. A foot
//! within the band of either escalates [`SideEscalation::Crease`]. The
//! irregular point is found from the jet: `‖m‖·‖S_x‖/‖∂_x m‖`, for
//! `m = S_u × S_v` and each direction `x`, is a length in meters along
//! `x` to where `m` vanishes. Where `m` is affine across the gap it is
//! that arc at a pole, where the speed along `x` holds, and twice it at a
//! kink, where the speed itself vanishes; on a regular chart it is about
//! the radius of curvature. A
//! closed seam escalates whether or not it is tangent continuous, since
//! no statement of a seam's continuity exists beside `crease_runs`.
//!
//! **The foot's offset is charged to first order.** The normal at the
//! foot stands for the normal at `p`, a distance `δ` inside the band
//! away, so it turns by about `κ·δ` with `κ` the curvature at the foot,
//! which over an arm of at most `1/κ` is a displacement of about `δ`.
//! That is a first-order statement at the foot: `κ` is read pointwise,
//! not bounded over the gap to `p`, so a curvature rising steeply across
//! that gap is not covered by it. It assumes the normal is defined and
//! continuous between the foot and `p`, which the regularity and crease
//! readings above check at the foot alone.
//!
//! The regularity and crease readings are the door's own, and also
//! [`regular_jet`] and [`chart_normal_at`] for a reader that holds the
//! point's chart parameters already. The foot's whole reading — regular,
//! then decided onto the point, then continuous — is
//! [`chart_point_normal`], which this door reads at its projection foot
//! and the boolean's pierce at the chart point its sweep placed the
//! point at.
//!
//! **A corner of a face on a crease reads the side its sector lies on**
//! ([`chart_normal_beside`]), which the sector walk takes for a vertex on
//! the face's boundary, read off its edges' pcurves. A foot has no side,
//! so on a crease this door refuses; a corner has one: the two edges
//! bounding it leave the vertex into the chart, and each one's pcurve box
//! over its edge's half at the vertex lies past the crease line on one
//! side (`dihedral_side_crease_side`). Where both name one side the normal
//! is that side's one-sided limit, read on the span beside the line
//! ([`geom::NurbsSurface::ders_beside`]). A corner that may hold both
//! sides reads only where the two one-sided normals are decided one
//! (`dihedral_side_crease_fold`), as on a flat chart with a knot crease.

use geom::{NurbsSurface, RunSide, SpanBeside, Surface, SurfaceJet};
use geom_core::k_stats::Magnitude;
use geom_core::{Band, Decide, Indeterminate, Lever, Margin, MarginDiag, Point3, Real, Vec3};

use crate::certify::NurbsLane;

use crate::enters::{EntersMaterial, LeverEscalation, OutwardNormal, enters_material_reported};
use crate::implicit::{curvature_lever_arm, implicit_outward_normal};

/// Which of an edge's two half-edges walks it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeHalf {
    /// The edge's `he_plus`, which walks the carrier forward.
    HePlus,
    /// The edge's `he_minus`, which walks it backward.
    HeMinus,
}

/// The edge's direction at a point, as one of its two half-edges walks
/// it: the carrier's derivative where that half-edge is the edge's
/// `he_plus`, and its negation where it is the other
/// ([`crate::EdgeCurve::walk_tangents`]'s convention). The caller mints
/// it from the half-edge in the face whose side is asked.
#[derive(Clone, Copy, Debug)]
pub struct EdgeWalk<T: Real>(Vec3<T>);

impl<T: Real> EdgeWalk<T> {
    /// The walk of `half` where the carrier's derivative is `derivative`.
    pub(crate) fn of_half(derivative: Vec3<T>, half: EdgeHalf) -> Self {
        Self(match half {
            EdgeHalf::HePlus => derivative,
            EdgeHalf::HeMinus => -derivative,
        })
    }
}

impl<T: Decide> EdgeWalk<T> {
    /// The walk of `curve`'s half `half` at the carrier's parameter `t`.
    #[must_use]
    pub fn along(curve: &crate::EdgeCurve<T>, t: T, half: EdgeHalf) -> Self {
        Self::of_half(curve.carrier().deriv(t), half)
    }
}

/// One face at the shared edge: its carrier and its `Face::sense` bit.
#[derive(Clone, Copy, Debug)]
pub struct SideFace<'a, T: Real> {
    /// The face's carrier.
    pub surface: &'a Surface<T>,
    /// The face's sense bit: its outward normal is its chart normal where
    /// `true` and the negation where `false`.
    pub sense: bool,
}

/// Which of the two faces [`dihedral_side`] reads a refusal names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SideRole {
    /// The face whose side is asked.
    Departing,
    /// The face it is asked against.
    Against,
}

/// Which side of the face it is read against the departing face leaves
/// the edge on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaceSide {
    /// Against the other face's outward normal: into its material.
    Material,
    /// Along its outward normal: away from its material.
    Outside,
}

/// Why [`dihedral_side`] read no side.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SideEscalation {
    /// A spline face is the empty placeholder, which has no chart.
    Placeholder(SideRole),
    /// A spline face, at a scalar with no certified NURBS lane: the foot
    /// is a bracket read such a scalar may not mint.
    NoLane(SideRole),
    /// The projection of the point onto a spline face did not converge.
    FootInconclusive {
        /// The spline face.
        role: SideRole,
        /// The last distance the projection read, in meters.
        last_distance: f64,
    },
    /// The point's foot on a spline face's carrier lies in the band:
    /// whether the point is on that carrier is a size this tolerance
    /// cannot decide.
    FootUndecided {
        /// The spline face.
        role: SideRole,
        /// The foot distance's escalation.
        diag: Indeterminate,
    },
    /// The point's foot on a spline face's carrier, a regular point of
    /// it, is definitely apart from the point: the point is not on that
    /// carrier, so it is no point of an edge the face bounds.
    OffFace {
        /// The spline face.
        role: SideRole,
        /// The foot distance, decided past the band.
        margin: MarginDiag,
    },
    /// The point's foot on a spline face lies within the band of a point
    /// where the chart's normal is not defined or a line across which it
    /// is not continuous (module docs): an irregular point, a C⁰ knot
    /// run, or a closed chart's seam. No normal is read.
    Crease {
        /// The spline face.
        role: SideRole,
        /// The foot's distance from the point or line, as decided: zero
        /// or in band.
        reading: CreaseReading,
    },
    /// The arm, or the signed sine metered over it, did not decide.
    Lever(LeverEscalation),
    /// The tangent planes coincide at the point, so the departure has no
    /// first-order side: the side is the second order's to read.
    Tangent(MarginDiag),
}

/// **The side of `against` that `departing` leaves the shared edge on**
/// at `p` (module docs). `walk` is the edge's direction at `p` as
/// `departing`'s own half-edge walks it; `reach` is the faces' extent
/// from the edge, the arm's outer fold, as [`crate::classify_dihedral`]
/// takes it; `lane` is the scalar's certified NURBS lane, which a spline
/// face's foot is read through.
///
/// INVARIANT, enforced by the caller: `p` lies on the edge, so on both
/// faces. The door checks only that a spline face's CARRIER passes
/// through `p` ([`SideEscalation::OffFace`]), at a foot it does not place
/// in the face (module docs): a side read off a spline face is the
/// face's only once its caller has placed that foot in the face. An
/// analytic face's gradient is read at `p` as given.
///
/// # Errors
///
/// [`SideEscalation`]: a spline face's foot not decided onto its carrier,
/// or lying where its normal is not defined or not continuous, the arm or
/// the reading undecided, or a tangent dihedral.
pub fn dihedral_side<T: Decide>(
    departing: SideFace<'_, T>,
    against: SideFace<'_, T>,
    p: Point3<T>,
    walk: EdgeWalk<T>,
    reach: Lever<T>,
    lane: Option<NurbsLane<T>>,
    band: Band,
) -> Result<(FaceSide, MarginDiag), SideEscalation> {
    let normal = |face, role| normal_at(face, role, p, reach, lane, band);
    let (n_dep, arm_dep) = normal(departing, SideRole::Departing)?;
    let (n_against, arm_against) = normal(against, SideRole::Against)?;
    let arm = reach.min(arm_dep).min(arm_against);
    let departure = n_dep.vec().cross(walk.0.normalize());
    match enters_material_reported(departure, n_against, arm, band) {
        Ok((EntersMaterial::Enters, margin)) => Ok((FaceSide::Material, margin)),
        Ok((EntersMaterial::Exits, margin)) => Ok((FaceSide::Outside, margin)),
        Ok((EntersMaterial::Tangent, margin)) => Err(SideEscalation::Tangent(margin)),
        Err(e) => Err(SideEscalation::Lever(e)),
    }
}

/// A face's outward normal at `p` and its curvature arm (module docs).
fn normal_at<T: Decide>(
    face: SideFace<'_, T>,
    role: SideRole,
    p: Point3<T>,
    reach: Lever<T>,
    lane: Option<NurbsLane<T>>,
    band: Band,
) -> Result<(OutwardNormal<T>, Lever<T>), SideEscalation> {
    match face.surface.spline_chart() {
        None => Ok((
            implicit_outward_normal(face.surface, face.sense, p),
            curvature_lever_arm(face.surface, p),
        )),
        Some(chart) => {
            let lane = lane.ok_or(SideEscalation::NoLane(role))?;
            spline_normal(chart, face.sense, role, p, reach, lane, band)
        }
    }
}

/// A spline face's outward normal at the foot of `p`, and the radius of
/// its largest principal curvature there folded with `reach`.
fn spline_normal<T: Decide>(
    chart: &NurbsSurface<T>,
    sense: bool,
    role: SideRole,
    p: Point3<T>,
    reach: Lever<T>,
    lane: NurbsLane<T>,
    band: Band,
) -> Result<(OutwardNormal<T>, Lever<T>), SideEscalation> {
    if chart.is_placeholder() {
        return Err(SideEscalation::Placeholder(role));
    }
    let foot = lane
        .surface_foot(chart, p)
        .map_err(|e| SideEscalation::FootInconclusive {
            role,
            last_distance: e.last_distance,
        })?;
    let uv = (T::from_computed(foot.u), T::from_computed(foot.v));
    chart_point_normal(chart, sense, uv, p, reach, "dihedral_side_foot", band).map_err(|refusal| {
        match refusal {
            ChartPointRefusal::Irregular(reading) | ChartPointRefusal::Crease(reading) => {
                SideEscalation::Crease { role, reading }
            }
            ChartPointRefusal::Apart(margin) => SideEscalation::OffFace { role, margin },
            ChartPointRefusal::Undecided(diag) => SideEscalation::FootUndecided { role, diag },
        }
    })
}

/// Why [`chart_point_normal`] read no normal at a chart point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChartPointRefusal {
    /// The chart point lies within the band of an irregular point of the
    /// chart, where `S_u × S_v` vanishes ([`regular_jet`]).
    Irregular(CreaseReading),
    /// The chart's point there is definitely apart from the point.
    Apart(MarginDiag),
    /// Whether the chart's point there is the point is in band.
    Undecided(Indeterminate),
    /// The chart point lies within the band of a line the chart's normal
    /// is not continuous across ([`chart_normal_at`]).
    Crease(CreaseReading),
}

/// **A spline chart's outward normal at the chart point `(u, v)` of a
/// point `p`**, and the radius of its largest principal curvature there
/// folded with `reach`: the one reading of a chart point for every
/// reader that holds one, a projection foot ([`dihedral_side`]) or a
/// chart point the caller placed on the face's own sheet.
///
/// The jet is read where the chart is regular ([`regular_jet`]) first: a
/// chart point where the chart's speed vanishes is a stationary point of
/// the distance, not a nearest one, so its distance decides nothing until
/// the point is regular. Then `S(u, v)` is decided onto `p` (`name`, a
/// magnitude), and the normal read where it is continuous
/// ([`chart_normal_at`]), with `sense` folded in there.
///
/// # Errors
///
/// [`ChartPointRefusal`], as its variants say.
pub fn chart_point_normal<T: Decide>(
    chart: &NurbsSurface<T>,
    sense: bool,
    (u, v): (T, T),
    p: Point3<T>,
    reach: Lever<T>,
    name: &'static str,
    band: Band,
) -> Result<(OutwardNormal<T>, Lever<T>), ChartPointRefusal> {
    let jet =
        regular_jet(chart, (u, v), reach.length(), band).map_err(ChartPointRefusal::Irregular)?;
    match crate::dihedral::decide_magnitude_reported(name, Margin::of((jet.point - p).norm()), band)
    {
        Ok((Magnitude::Zero, _)) => {}
        Ok((Magnitude::Positive, margin)) => return Err(ChartPointRefusal::Apart(margin)),
        Err(diag) => return Err(ChartPointRefusal::Undecided(diag)),
    }
    chart_normal_at(chart, sense, &jet, (u, v), reach, band).map_err(ChartPointRefusal::Crease)
}

/// How near a point of a spline chart lies to where the chart's normal
/// is not defined or not continuous (module docs): zero, or in band.
pub type CreaseReading = Result<MarginDiag, Indeterminate>;

/// **The jet of `chart` at `(u, v)`, where its normal is defined**: the
/// regularity reading of the module docs. `reach` is the extent the
/// caller reads the face over, which a direction along which
/// `S_u × S_v` does not change reads as the distance to an irregular
/// point.
///
/// # Errors
///
/// The [`CreaseReading`] of an irregular point (`S_u × S_v` vanishing:
/// coincident control rows, a pole) within the band of `(u, v)`.
pub fn regular_jet<T: Decide>(
    chart: &NurbsSurface<T>,
    (u, v): (T, T),
    reach: T,
    band: Band,
) -> Result<SurfaceJet<T>, CreaseReading> {
    let jet = chart.ders(u, v);
    match near_as("dihedral_side_regular", irregular_reach(&jet, reach), band) {
        Some(reading) => Err(reading),
        None => Ok(jet),
    }
}

/// **A spline face's outward normal at a regular jet of its chart** at
/// `(u, v)` ([`regular_jet`]), and the radius of its largest principal
/// curvature there folded with `reach`. The point is the face's own: the
/// caller decides it is (a foot decided onto the carrier and placed in
/// the face, or a boundary point read off its pcurve).
///
/// # Errors
///
/// The [`CreaseReading`] of a line the normal is not continuous across
/// (an interior knot run of multiplicity at least the degree, or a
/// closed chart's seam) within the band of the jet's point: the face has
/// a normal on each side of it, and no one of them is the normal there.
pub fn chart_normal_at<T: Decide>(
    chart: &NurbsSurface<T>,
    sense: bool,
    jet: &SurfaceJet<T>,
    (u, v): (T, T),
    reach: Lever<T>,
    band: Band,
) -> Result<(OutwardNormal<T>, Lever<T>), CreaseReading> {
    if let Some(reading) = on_a_crease(chart, jet.point, (u, v), band) {
        return Err(reading);
    }
    let kappa = crate::shape_operator::max_principal_curvature(jet);
    let (arm, _) = crate::ssi::point_arm(kappa, reach);
    Ok((
        OutwardNormal::from_chart(jet.du.cross(jet.dv).normalize(), sense),
        arm,
    ))
}

/// A length the foot lies within: `Some` with its reading where the band
/// does not decide it positive (zero or in band), `None` past the band.
fn near_as<T: Decide>(name: &'static str, distance: T, band: Band) -> Option<CreaseReading> {
    match crate::dihedral::decide_magnitude_reported(name, Margin::of(distance), band) {
        Ok((Magnitude::Positive, _)) => None,
        Ok((Magnitude::Zero, margin)) => Some(Ok(margin)),
        Err(diag) => Some(Err(diag)),
    }
}

/// How far from the jet's point the chart's normal `m = S_u × S_v`
/// vanishes, in meters, up to the factor two the module docs name: the
/// least over the two directions of `‖m‖·‖S_x‖/‖∂_x m‖`, folded with
/// `reach` so a direction along which `m` does not change reads the
/// faces' extent. Poison where `m` and its change both vanish.
fn irregular_reach<T: Decide>(jet: &SurfaceJet<T>, reach: T) -> T {
    let m = jet.du.cross(jet.dv);
    let along = |speed: T, dm: Vec3<T>| {
        let lever = m.norm() * speed;
        lever / dm.norm().max(lever / reach)
    };
    let dm_u = jet.duu.cross(jet.dv) + jet.du.cross(jet.duv);
    let dm_v = jet.duv.cross(jet.dv) + jet.du.cross(jet.dvv);
    along(jet.du.norm(), dm_u).min(along(jet.dv.norm(), dm_v))
}

/// The first line of `chart` its normal is not continuous across that the
/// foot `at`, at chart parameters `(u, v)`, lies within the band of
/// (module docs), as [`creases_near`] lists them.
fn on_a_crease<T: Decide>(
    chart: &NurbsSurface<T>,
    at: Point3<T>,
    (u, v): (T, T),
    band: Band,
) -> Option<CreaseReading> {
    creases_near(chart, at, (u, v), band)
        .into_iter()
        .next()
        .map(|c| c.reading)
}

/// A parameter direction of a chart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ChartDir {
    /// `u`: a line of it is an iso line `u = c`, across which `u` changes.
    U,
    /// `v`.
    V,
}

impl ChartDir {
    /// `(of_u, of_v)` read in this direction.
    fn pick<X>(self, (of_u, of_v): (X, X)) -> X {
        match self {
            Self::U => of_u,
            Self::V => of_v,
        }
    }

    /// The [`geom::NurbsSurface::ders_beside`] argument naming `side` of
    /// breakpoint run `run` in this direction and none in the other.
    fn beside(self, run: usize, side: RunSide) -> (SpanBeside, SpanBeside) {
        match self {
            Self::U => (Some((run, side)), None),
            Self::V => (None, Some((run, side))),
        }
    }
}

/// A line of a chart its normal is not continuous across, near a point
/// ([`creases_near`]).
#[derive(Clone, Copy, Debug)]
struct CreaseLine<T: Decide> {
    /// The direction its parameter is of.
    dir: ChartDir,
    /// The breakpoint run it lies on: an interior crease run, or for a
    /// closed chart's seam the first run (its other copy the last).
    run: usize,
    /// Its parameter value.
    at: T,
    /// For a seam, the domain's last run and its value.
    seam_end: Option<(usize, T)>,
    /// The point's distance from it, as decided: zero, or in band.
    reading: CreaseReading,
}

/// Every line of `chart` its normal is not continuous across that the
/// foot `at`, at chart parameters `(u, v)`, lies within the band of
/// (module docs), `u`'s first: per direction, each interior crease run,
/// measured from the foot along the iso line through it, then a closed
/// seam, both domain ends within the band of the foot. An open chart's
/// own boundary, where a rim point's foot sits, is one end alone and no
/// line.
fn creases_near<T: Decide>(
    chart: &NurbsSurface<T>,
    at: Point3<T>,
    (u, v): (T, T),
    band: Band,
) -> Vec<CreaseLine<T>> {
    let near = |q: Point3<T>| near_as("dihedral_side_crease", (q - at).norm(), band);
    let mut lines = Vec::new();
    for (knots, dir) in [
        (chart.knot_values_u(), ChartDir::U),
        (chart.knot_values_v(), ChartDir::V),
    ] {
        let at_line = |c: T| match dir {
            ChartDir::U => chart.ders(c, v).point,
            ChartDir::V => chart.ders(u, c).point,
        };
        let runs: Vec<T> = knots.breakpoints().collect();
        for r in knots.structure().crease_runs() {
            if let Some(reading) = near(at_line(runs[r])) {
                lines.push(CreaseLine {
                    dir,
                    run: r,
                    at: runs[r],
                    seam_end: None,
                    reading,
                });
            }
        }
        let last = runs.len() - 1;
        if let (Some(reading), Some(_)) = (near(at_line(runs[0])), near(at_line(runs[last]))) {
            lines.push(CreaseLine {
                dir,
                run: 0,
                at: runs[0],
                seam_end: Some((last, runs[last])),
                reading,
            });
        }
    }
    lines
}

/// Which side of `line` the box `[lo, hi]` (its parameter range across
/// the line) lies on, if one (`dihedral_side_crease_side`): for an
/// interior line, the side the box reaches past the line on, and not the
/// other; for a seam, the domain end the box touches (beside the first
/// run, above it, or the last, below it), and not the other. Each reach
/// is a signed parameter difference metered at the chart's speed across
/// the line on that side's span at `(u, v)`; a reach decided positive is
/// past, one decided zero or negative is not.
///
/// # Errors
///
/// A reach the band does not decide, as its [`CreaseReading`].
fn side_of_line<T: Decide>(
    chart: &NurbsSurface<T>,
    (u, v): (T, T),
    line: &CreaseLine<T>,
    (lo, hi): (T, T),
    band: Band,
) -> Result<Option<(usize, RunSide)>, CreaseReading> {
    let past = |m: T, (run, side): (usize, RunSide)| {
        let jet = chart.ders_beside(u, v, line.dir.beside(run, side));
        let speed = line.dir.pick((jet.du, jet.dv)).norm();
        crate::dihedral::decide(
            "dihedral_side_crease_side",
            Margin::metered(m, geom_core::InfSpeed::new(speed)),
            band,
        )
        .map(|sign| sign == geom_core::Sign::Positive)
        .map_err(Err)
    };
    let above = (line.run, RunSide::Above);
    Ok(match line.seam_end {
        None => {
            let up = past(hi - line.at, above)?;
            let down = past(line.at - lo, (line.run, RunSide::Below))?;
            match (up, down) {
                (true, false) => Some(above),
                (false, true) => Some((line.run, RunSide::Below)),
                _ => None,
            }
        }
        Some((last, end)) => {
            let off_first = past(lo - line.at, above)?;
            let off_last = past(end - hi, (last, RunSide::Below))?;
            match (off_first, off_last) {
                (false, true) => Some(above),
                (true, false) => Some((last, RunSide::Below)),
                _ => None,
            }
        }
    })
}

/// The outward normal and arm of `chart`'s regular jet `jet`, or the
/// reading of the irregular point it lies within the band of
/// ([`regular_jet`]'s).
fn normal_of_jet<T: Decide>(
    jet: &SurfaceJet<T>,
    sense: bool,
    reach: Lever<T>,
    band: Band,
) -> Result<(OutwardNormal<T>, Lever<T>), CreaseReading> {
    if let Some(reading) = near_as(
        "dihedral_side_regular",
        irregular_reach(jet, reach.length()),
        band,
    ) {
        return Err(reading);
    }
    let kappa = crate::shape_operator::max_principal_curvature(jet);
    let (arm, _) = crate::ssi::point_arm(kappa, reach);
    Ok((
        OutwardNormal::from_chart(jet.du.cross(jet.dv).normalize(), sense),
        arm,
    ))
}

/// **A spline face's outward normal at a corner `(u, v)`, on its
/// sector's side of any crease there.** Where the corner lies within the
/// band of a line the chart's normal is not continuous across
/// ([`chart_normal_at`]'s refusal), the face has a normal on each side of
/// the line, and the corner's sector, between its two bounding edges,
/// lies on one side or straddles the line. `bounds` are the two bounding
/// edges' pcurves' chart boxes, each over the stretch of its edge nearest
/// the corner ([`crate::Pcurve::chart_box`]). Each box's side of each line
/// is decided ([`side_of_line`]): where both name one side of every
/// line, and the corner is decided convex about that side's normal
/// ([`crate::sector_shape::sector_shape`] on `(own, next)`, the corner's
/// counter-clockwise-last and -first bounding chords as that module
/// takes them), the jet is read on the span beside each line there
/// ([`geom::NurbsSurface::ders_beside`]). Otherwise the sector may hold
/// both sides, and it reads only if the one-sided normals on every side
/// it may hold are decided one (`dihedral_side_crease_fold`, their angle
/// levered at `reach`), as on a flat chart. Each jet read is checked
/// regular ([`regular_jet`]'s reading). Off every crease it is
/// [`regular_jet`] then [`chart_normal_at`].
///
/// # Errors
///
/// A [`CreaseReading`]: an irregular point of a jet read; a side reach
/// the band does not decide (the side decision's own); two lines near
/// in one direction (a crease run and a seam, or two runs), whose spans
/// between are not read, as the first's; and a sector that may hold
/// both sides of a line whose two normals differ or are not decided
/// equal, as that line's.
pub fn chart_normal_beside<T: Decide>(
    chart: &NurbsSurface<T>,
    sense: bool,
    (u, v): (T, T),
    bounds: [&crate::ChartWindow<T>; 2],
    (own, next): (Vec3<T>, Vec3<T>),
    reach: Lever<T>,
    band: Band,
) -> Result<(OutwardNormal<T>, Lever<T>), CreaseReading> {
    let at = chart.ders(u, v).point;
    let lines = creases_near(chart, at, (u, v), band);
    let Some(first_line) = lines.first() else {
        let jet = regular_jet(chart, (u, v), reach.length(), band)?;
        return chart_normal_at(chart, sense, &jet, (u, v), reach, band);
    };
    for dir in [ChartDir::U, ChartDir::V] {
        let mut of_dir = lines.iter().filter(|l| l.dir == dir);
        if let (Some(first), Some(_)) = (of_dir.next(), of_dir.next()) {
            return Err(first.reading);
        }
    }
    // Per direction, the sides the sector may hold: the one both bounds
    // name, or both; and the reading a fold across a straddled line
    // refuses with.
    let (mut of_u, mut of_v): (Vec<SpanBeside>, Vec<SpanBeside>) = (vec![None], vec![None]);
    let mut fold = first_line.reading;
    for line in &lines {
        let window =
            |b: &crate::ChartWindow<T>| line.dir.pick(((b.u_min, b.u_max), (b.v_min, b.v_max)));
        let sides = [
            side_of_line(chart, (u, v), line, window(bounds[0]), band)?,
            side_of_line(chart, (u, v), line, window(bounds[1]), band)?,
        ];
        let held = match sides {
            [Some(a), Some(b)] if a == b => vec![Some(a)],
            _ => {
                fold = line.reading;
                let below = line.seam_end.map_or(line.run, |(last, _)| last);
                vec![
                    Some((line.run, RunSide::Above)),
                    Some((below, RunSide::Below)),
                ]
            }
        };
        match line.dir {
            ChartDir::U => of_u = held,
            ChartDir::V => of_v = held,
        }
    }
    // Both bounds on one side of every line: the corner lies on that side
    // where it is convex there, the positive cone of its bounds
    // ([`crate::sector_shape`]'s rung 2, about that side's normal). A
    // reflex or undecided corner may sweep across a line its bounds both
    // lie beside, so it holds every side.
    if let ([bu], [bv]) = (of_u.as_slice(), of_v.as_slice()) {
        let side = normal_of_jet(&chart.ders_beside(u, v, (*bu, *bv)), sense, reach, band)?;
        if let Ok(shape) = crate::sector_shape::sector_shape(own, next, side.0, false, band)
            && shape.bisector.is_none()
        {
            return Ok(side);
        }
        for line in &lines {
            let below = line.seam_end.map_or(line.run, |(last, _)| last);
            let both = vec![
                Some((line.run, RunSide::Above)),
                Some((below, RunSide::Below)),
            ];
            match line.dir {
                ChartDir::U => of_u = both,
                ChartDir::V => of_v = both,
            }
        }
    }
    let mut read: Option<(OutwardNormal<T>, Lever<T>)> = None;
    for &bu in &of_u {
        for &bv in &of_v {
            let side = normal_of_jet(&chart.ders_beside(u, v, (bu, bv)), sense, reach, band)?;
            let Some((first, _)) = read else {
                read = Some(side);
                continue;
            };
            let turn = (side.0.vec() - first.vec()).norm();
            match crate::dihedral::decide_magnitude_reported(
                "dihedral_side_crease_fold",
                Margin::levered(turn, reach),
                band,
            ) {
                Ok((Magnitude::Zero, _)) => {}
                Ok((Magnitude::Positive, _)) => return Err(fold),
                Err(diag) => return Err(Err(diag)),
            }
        }
    }
    read.ok_or(fold)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Tol;
    use geom_core::spline::KnotVector;

    use geom_core::test_support::fixture_lever;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// The band's geometric middle: a length no row's band decides.
    fn in_band() -> f64 {
        let b = band();
        (b.zero() * b.escalate()).sqrt()
    }

    fn lane() -> Option<NurbsLane<f64>> {
        Some(NurbsLane::certified())
    }

    fn face(surface: &Surface<f64>, sense: bool) -> SideFace<'_, f64> {
        SideFace { surface, sense }
    }

    /// The door at `reach`, walking a carrier that runs along `t` by its
    /// `he_plus`.
    fn side_at(
        departing: SideFace<'_, f64>,
        against: SideFace<'_, f64>,
        p: Point3<f64>,
        t: Vec3<f64>,
        reach: f64,
    ) -> Result<FaceSide, SideEscalation> {
        let walk = EdgeWalk::of_half(t, EdgeHalf::HePlus);
        dihedral_side(
            departing,
            against,
            p,
            walk,
            fixture_lever(reach),
            lane(),
            band(),
        )
        .map(|(s, _)| s)
    }

    fn side(
        departing: SideFace<'_, f64>,
        against: SideFace<'_, f64>,
        p: Point3<f64>,
        t: Vec3<f64>,
    ) -> Result<FaceSide, SideEscalation> {
        side_at(departing, against, p, t, 1.0)
    }

    fn plane(origin: Point3<f64>, normal: Vec3<f64>, u_ref: Vec3<f64>) -> Surface<f64> {
        Surface::Plane {
            origin,
            normal: normal.normalize(),
            u_ref,
        }
    }

    /// The plane `z = 0`, its chart normal `+z`.
    fn floor() -> Surface<f64> {
        plane(Point3::new(0.0, 0.0, 0.0), Z, X)
    }

    /// A surface linear along the edge `edge(s)`, `s ∈ [−1, 1]`, with the
    /// rows `edge(s) + profile[k]` on the knots `kv`.
    fn spline(
        edge: impl Fn(f64) -> Point3<f64>,
        kv: KnotVector,
        profile: &[Vec3<f64>],
    ) -> Surface<f64> {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let control = [-1.0, 1.0]
            .into_iter()
            .flat_map(|s| profile.iter().map(|&d| edge(s) + d).collect::<Vec<_>>())
            .collect();
        let n = 2 * profile.len();
        Surface::Nurbs(std::sync::Arc::new(
            NurbsSurface::new(ku, kv, control, vec![1.0; n]).unwrap(),
        ))
    }

    fn quadratic() -> KnotVector {
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap()
    }

    /// A wall rising from the edge, quadratic away from it: it leaves
    /// along `v1` and bends toward `v2 − 2·v1`.
    fn wall(edge: impl Fn(f64) -> Point3<f64>, v1: Vec3<f64>, v2: Vec3<f64>) -> Surface<f64> {
        spline(edge, quadratic(), &[Vec3::new(0.0, 0.0, 0.0), v1, v2])
    }

    /// A ruled surface along the `x` axis through the `(y, z)` profile.
    fn ruled(kv: KnotVector, profile: &[(f64, f64)]) -> Surface<f64> {
        let rows: Vec<Vec3<f64>> = profile.iter().map(|&(y, z)| Vec3::new(0.0, y, z)).collect();
        spline(|s| Point3::new(s, 0.0, 0.0), kv, &rows)
    }

    /// A wall on the floor along the `x` axis, leaving it toward `+y`
    /// with the rise `rise` per unit run and curving up past it.
    fn floor_wall(rise: f64) -> Surface<f64> {
        wall(
            |s| Point3::new(s, 0.0, 0.0),
            Vec3::new(0.0, 0.5, 0.5 * rise),
            Vec3::new(0.0, 1.0, rise + 0.5),
        )
    }

    /// The cylinder `x² + y² = 1`.
    fn cylinder() -> Surface<f64> {
        Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 0.0),
            axis: Z,
            radius: 1.0,
            u_ref: X,
        }
    }

    /// A spline wall on the cylinder's ruling `(1, 0, z)`, leaving it
    /// toward `+y` and out (`flare > 0`) or in.
    fn ruling_wall(flare: f64) -> Surface<f64> {
        wall(
            |s| Point3::new(1.0, 0.0, s),
            Vec3::new(0.5 * flare, 0.5, 0.0),
            Vec3::new(flare, 1.0, 0.0),
        )
    }

    /// A carrier folded along the `x` axis at the C⁰ knot `v = ½`: the
    /// half `v < ½` is `(s, −a, a)`, the half `v > ½` is `(s, a, a)`.
    fn creased() -> Surface<f64> {
        ruled(
            KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap(),
            &[(-1.0, 1.0), (0.0, 0.0), (1.0, 1.0)],
        )
    }

    /// The same fold with no knot crease: degree 2, a simple knot at `½`
    /// and the middle two control rows coincident, so `S_v` vanishes on
    /// the fold and `crease_runs` lists nothing.
    fn kinked() -> Surface<f64> {
        ruled(
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap(),
            &[(-1.0, 1.0), (0.0, 0.0), (0.0, 0.0), (1.0, 1.0)],
        )
    }

    const ON_EDGE: Point3<f64> = Point3::new(0.3, 0.0, 0.0);
    const ON_RULING: Point3<f64> = Point3::new(1.0, 0.0, 0.3);
    const X: Vec3<f64> = Vec3::new(1.0, 0.0, 0.0);
    const Z: Vec3<f64> = Vec3::new(0.0, 0.0, 1.0);

    fn is_crease(got: Result<FaceSide, SideEscalation>) -> bool {
        matches!(got, Err(SideEscalation::Crease { .. }))
    }

    /// A spline wall against a plane, rising and falling: the side is the
    /// wall's departure, and a wall of either sense, walked the way its
    /// own loop walks the edge, reads the same side.
    #[test]
    fn a_spline_wall_reads_its_side_of_a_plane() {
        let floor = floor();
        for (rise, want) in [(1.0, FaceSide::Outside), (-1.0, FaceSide::Material)] {
            let w = floor_wall(rise);
            // Sense true: the chart normal `S_u × S_v` is outward and the
            // interior `+v` lies to its left walking `+x`; sense false
            // walks `−x`.
            for (sense, t) in [(true, X), (false, -X)] {
                assert_eq!(
                    side(face(&w, sense), face(&floor, true), ON_EDGE, t),
                    Ok(want),
                    "rise {rise}, wall sense {sense}"
                );
            }
            let flipped = match want {
                FaceSide::Outside => FaceSide::Material,
                FaceSide::Material => FaceSide::Outside,
            };
            assert_eq!(
                side(face(&w, true), face(&floor, false), ON_EDGE, X),
                Ok(flipped),
                "rise {rise}, floor sense false"
            );
        }
    }

    /// The minus half walks the carrier backward: the same face walked by
    /// the edge's other half reads the other side.
    #[test]
    fn the_minus_half_walks_the_carrier_backward() {
        let (w, floor) = (floor_wall(1.0), floor());
        let read = |half| {
            let walk = EdgeWalk::of_half(X, half);
            dihedral_side(
                face(&w, true),
                face(&floor, true),
                ON_EDGE,
                walk,
                fixture_lever(1.0),
                lane(),
                band(),
            )
            .map(|(s, _)| s)
        };
        assert_eq!(read(EdgeHalf::HePlus), Ok(FaceSide::Outside));
        assert_eq!(read(EdgeHalf::HeMinus), Ok(FaceSide::Material));
    }

    /// A plane against a spline wall: the floor strip on either bank of
    /// the edge, read against the rising wall, whose material lies on its
    /// `+y, −z` side.
    #[test]
    fn a_plane_reads_its_side_of_a_spline_wall() {
        let (floor, w) = (floor(), floor_wall(1.0));
        assert_eq!(
            side(face(&floor, true), face(&w, true), ON_EDGE, X),
            Ok(FaceSide::Material),
            "the strip under the wall"
        );
        assert_eq!(
            side(face(&floor, true), face(&w, true), ON_EDGE, -X),
            Ok(FaceSide::Outside),
            "the strip behind it"
        );
    }

    /// A spline wall against a cylinder across a ruling, both ways out of
    /// it, and the cylinder against the wall on both banks.
    #[test]
    fn a_spline_wall_and_a_cylinder_read_each_others_side() {
        let cyl = cylinder();
        for (flare, want) in [(1.0, FaceSide::Outside), (-1.0, FaceSide::Material)] {
            let w = ruling_wall(flare);
            assert_eq!(
                side(face(&w, true), face(&cyl, true), ON_RULING, Z),
                Ok(want),
                "flare {flare}"
            );
        }
        // The outward wall's material lies on its `+x, −y` side, so the
        // cylinder's sheet leaving the ruling toward `−y` (walking `+z`
        // under its outward `+x`) enters it and the sheet toward `+y`
        // does not.
        let w = ruling_wall(1.0);
        assert_eq!(
            side(face(&cyl, true), face(&w, true), ON_RULING, Z),
            Ok(FaceSide::Material),
            "the cylinder toward −y"
        );
        assert_eq!(
            side(face(&cyl, true), face(&w, true), ON_RULING, -Z),
            Ok(FaceSide::Outside),
            "the cylinder toward +y"
        );
    }

    /// A wall leaving the floor tangent to it has no first-order side,
    /// and one leaving at an angle the band cannot tell from tangent
    /// escalates: neither guesses.
    #[test]
    fn a_tangent_or_in_band_departure_escalates() {
        let floor = floor();
        assert!(
            matches!(
                side(face(&floor_wall(0.0), true), face(&floor, true), ON_EDGE, X),
                Err(SideEscalation::Tangent(_))
            ),
            "a tangent wall"
        );
        // The wall's sine is about its rise and its curvature radius one
        // (`S_v ≈ ŷ`, `S_vv = ẑ` at the edge), so the margin metered over
        // the unit reach is about the rise: placed inside the band.
        let rise = in_band();
        match side(
            face(&floor_wall(rise), true),
            face(&floor, true),
            ON_EDGE,
            X,
        ) {
            Err(SideEscalation::Lever(e)) => {
                assert_eq!(e.rung(), crate::LeverRung::Reading, "the reading escalates");
            }
            other => panic!("an in-band rise {rise} read {other:?}"),
        }
    }

    /// The arm folds the wall's curvature: past its radius of curvature
    /// the faces' reach meters nothing, so a rise in band over that radius
    /// escalates however far the faces run.
    #[test]
    fn the_arm_is_the_walls_radius_of_curvature_past_its_reach() {
        // The wall bends at `κ ≈ 1`, so its arm is one: a rise in band
        // there, over a reach whose own arm would carry it ten times past
        // the band.
        let rise = in_band();
        let reach = 10.0 * band().escalate() / rise;
        let near = floor_wall(rise);
        match side_at(face(&near, true), face(&floor(), true), ON_EDGE, X, reach) {
            Err(SideEscalation::Lever(e)) => {
                assert_eq!(e.rung(), crate::LeverRung::Reading, "the reading escalates");
            }
            other => panic!("rise {rise} over reach {reach} read {other:?}"),
        }
    }

    /// A point whose foot on the spline carrier lies in the band is a size
    /// the tolerance cannot place on it: escalated, not read.
    #[test]
    fn a_foot_in_band_escalates() {
        // Beside the wall's rim, off its chart: the foot is the rim point
        // and its distance is the offset.
        let off = Point3::new(0.3, -in_band(), 0.0);
        assert!(
            matches!(
                side(face(&floor_wall(1.0), true), face(&floor(), true), off, X),
                Err(SideEscalation::FootUndecided {
                    role: SideRole::Departing,
                    ..
                })
            ),
            "an in-band foot"
        );
    }

    /// A point off the spline carrier is no point of its edge: refused,
    /// not read at its foot.
    #[test]
    fn a_point_off_the_spline_face_is_refused() {
        let off = Point3::new(0.3, -0.2, 0.0);
        assert!(
            matches!(
                side(face(&floor_wall(1.0), true), face(&floor(), true), off, X),
                Err(SideEscalation::OffFace {
                    role: SideRole::Departing,
                    ..
                })
            ),
            "the wall's foot is 0.2 m away"
        );
    }

    /// A scalar with no certified NURBS lane reads no spline face, and
    /// names which face it could not read; an analytic pair needs no lane.
    #[test]
    fn a_spline_face_without_a_lane_refuses_typed() {
        let (w, floor) = (floor_wall(-1.0), floor());
        let walk = EdgeWalk::of_half(X, EdgeHalf::HePlus);
        assert_eq!(
            dihedral_side(
                face(&floor, true),
                face(&w, true),
                ON_EDGE,
                walk,
                fixture_lever(1.0),
                None,
                band()
            ),
            Err(SideEscalation::NoLane(SideRole::Against))
        );
        let tilted = plane(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, -1.0, 1.0), X);
        assert_eq!(
            dihedral_side(
                face(&tilted, true),
                face(&floor, true),
                ON_EDGE,
                walk,
                fixture_lever(1.0),
                None,
                band()
            )
            .map(|(s, _)| s),
            Ok(FaceSide::Outside),
            "the plane the rising wall leaves along"
        );
    }

    /// Each half of the creased carrier, as a face whose rim is the
    /// crease, has a normal of its own there that the chart's jet cannot
    /// tell from the other half's: the side escalates, on either walk.
    #[test]
    fn each_half_of_a_creased_carrier_escalates_on_the_crease() {
        let (c, floor) = (creased(), floor());
        for (half, walk) in [("lower", -X), ("upper", X)] {
            let got = side(face(&c, true), face(&floor, true), ON_EDGE, walk);
            assert!(
                matches!(
                    got,
                    Err(SideEscalation::Crease {
                        role: SideRole::Departing,
                        ..
                    })
                ),
                "the {half} half read {got:?}"
            );
        }
    }

    /// Past the band from the crease, a creased carrier reads the span
    /// the foot lies in: the lower half against the plane `x = 0.3`
    /// across its line there.
    #[test]
    fn a_creased_carrier_reads_its_side_off_the_crease() {
        let c = creased();
        let cut = plane(Point3::new(0.3, 0.0, 0.0), X, Vec3::new(0.0, 1.0, 0.0));
        let p = Point3::new(0.3, -0.5, 0.5);
        // The lower half's outward normal is ∝ (0, 1, 1), so walking
        // (0, −1, 1) its interior lies toward +x.
        let t = Vec3::new(0.0, -1.0, 1.0);
        let read = |t| side(face(&c, true), face(&cut, true), p, t);
        assert_eq!(read(t), Ok(FaceSide::Outside), "toward +x");
        assert_eq!(read(-t), Ok(FaceSide::Material), "toward −x");
    }

    /// A C⁰ fold need not be a knot crease: coincident control rows fold
    /// the chart where `S_v` vanishes, and a foot within the band of it
    /// escalates as the knot crease's does.
    #[test]
    fn a_kink_from_coincident_control_rows_escalates_like_a_knot_crease() {
        let floor = floor();
        // On the upper half, a tenth of the band from the fold: within
        // the band of the lower face too, so a point of its rim edge.
        let a = band().zero() / 10.0;
        let p = Point3::new(0.3, a, a);
        let knot = side(face(&creased(), true), face(&floor, true), p, -X);
        let kink = side(face(&kinked(), true), face(&floor, true), p, -X);
        assert!(is_crease(knot), "the knot-run fold read {knot:?}");
        assert!(is_crease(kink), "the coincident-row fold read {kink:?}");
    }

    /// A point on a kinked chart near its fold, where the projection
    /// accepts the zero-speed fold as a foot, is not refused as off the
    /// chart: the foot is irregular, so the distance decides nothing.
    #[test]
    fn a_point_on_a_kinked_chart_is_not_refused_off_it() {
        let on = Point3::new(0.3, 1e-3, 1e-3);
        let got = side(face(&kinked(), true), face(&floor(), true), on, X);
        assert!(is_crease(got), "a point on the chart read {got:?}");
    }

    /// Beside the kink, the regularity reach reads the distance to it:
    /// `S_v` is affine across the gap on the kinked chart's span, so the
    /// reading is exact up to the factor `‖S_vv‖δ²` bears to the arc
    /// `‖S_vv‖δ²/2`.
    #[test]
    fn the_regularity_reach_is_the_distance_to_the_kink() {
        let k = kinked();
        let chart = k.spline_chart().unwrap();
        for dv in [1e-3, 1e-5] {
            let jet = chart.ders(0.65, 0.5 + dv);
            let arc = (jet.point - chart.ders(0.65, 0.5).point).norm();
            let got = irregular_reach(&jet, 1.0);
            assert!(
                (got / (2.0 * arc) - 1.0).abs() < 1e-6,
                "at δ = {dv}: read {got}, the arc is {arc}"
            );
        }
    }

    /// A chart closed in `v` with a C⁰ seam: its two ends meet along the
    /// edge with opposite normals, so a foot on the seam escalates.
    #[test]
    fn a_closed_charts_seam_escalates() {
        let fin = ruled(quadratic(), &[(0.0, 0.0), (1.0, 1.0), (0.0, 0.0)]);
        let got = side(face(&fin, true), face(&floor(), true), ON_EDGE, X);
        assert!(is_crease(got), "the seam read {got:?}");
    }

    /// A chart closed in `v` with a tangent-continuous seam has one
    /// normal there, but no statement of a seam's continuity exists beside
    /// `crease_runs`, so it escalates as a C⁰ seam does: a known
    /// over-escalation, fail-safe, which a seam-continuity reading would
    /// turn into a side.
    #[test]
    fn a_c1_seam_escalates_for_want_of_a_seam_continuity_reading() {
        let loop_ = ruled(
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.25, 0.5, 0.75, 1.0, 1.0, 1.0], 2).unwrap(),
            &[
                (0.0, 0.0),
                (1.0, 0.0),
                (1.0, 1.0),
                (-1.0, 1.0),
                (-1.0, 0.0),
                (0.0, 0.0),
            ],
        );
        let cut = plane(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 1.0), X);
        let got = side(face(&loop_, true), face(&cut, true), ON_EDGE, X);
        assert!(is_crease(got), "the C1 seam read {got:?}");
    }

    /// A chart box `[u0, u1] × [v0, v1]`.
    fn window(((u0, u1), (v0, v1)): ((f64, f64), (f64, f64))) -> crate::ChartWindow<f64> {
        crate::ChartWindow {
            u_min: u0,
            u_max: u1,
            v_min: v0,
            v_max: v1,
        }
    }

    /// [`chart_normal_beside`] on `surface`'s chart at `(u, v)`, a corner
    /// whose two bounding edges leave into the boxes `bounds` along the
    /// chords `(own, next)`, read to a unit vector.
    fn beside_with(
        surface: &Surface<f64>,
        (u, v): (f64, f64),
        bounds: [((f64, f64), (f64, f64)); 2],
        chords: (Vec3<f64>, Vec3<f64>),
    ) -> Result<Vec3<f64>, CreaseReading> {
        let chart = surface.spline_chart().unwrap();
        let at = chart.ders(u, v).point;
        let [a, b] = bounds.map(window);
        let net = geom::cover::Cover::net(chart.control().to_vec());
        let reach = crate::ExtentBall::of_cover_from(net.as_ref(), at).lever();
        chart_normal_beside(chart, true, (u, v), [&a, &b], chords, reach, band())
            .map(|(normal, _)| normal.vec())
    }

    /// A convex corner's chords inside the box `bound`: from the chart
    /// point at `(u, v)` to the box's two corners farthest from it, the
    /// pair ordered counter-clockwise about the chart's normal at the
    /// box's middle.
    fn convex_in(
        surface: &Surface<f64>,
        (u, v): (f64, f64),
        ((u0, u1), (v0, v1)): ((f64, f64), (f64, f64)),
    ) -> (Vec3<f64>, Vec3<f64>) {
        let chart = surface.spline_chart().unwrap();
        let far = |lo: f64, hi: f64, at: f64| {
            if (hi - at).abs() >= (at - lo).abs() {
                hi
            } else {
                lo
            }
        };
        let at = chart.ders(u, v).point;
        let (fu, fv) = (far(u0, u1, u), far(v0, v1, v));
        let (a, b) = if (u1 - u0) >= (v1 - v0) {
            (chart.ders(u0, fv).point - at, chart.ders(u1, fv).point - at)
        } else {
            (chart.ders(fu, v0).point - at, chart.ders(fu, v1).point - at)
        };
        let mid = chart.ders(0.5 * (u0 + u1), 0.5 * (v0 + v1));
        let n = mid.du.cross(mid.dv);
        if b.cross(a).dot(n) > 0.0 {
            (a, b)
        } else {
            (b, a)
        }
    }

    /// [`beside_with`] for a convex corner with one bounding edge into
    /// each of `bounds`, its chords [`convex_in`] the first box.
    fn beside2(
        surface: &Surface<f64>,
        (u, v): (f64, f64),
        bounds: [((f64, f64), (f64, f64)); 2],
    ) -> Result<Vec3<f64>, CreaseReading> {
        beside_with(
            surface,
            (u, v),
            bounds,
            convex_in(surface, (u, v), bounds[0]),
        )
    }

    /// [`beside2`] for a convex corner both of whose bounding edges
    /// leave into the box `bound`.
    fn beside(
        surface: &Surface<f64>,
        (u, v): (f64, f64),
        bound: ((f64, f64), (f64, f64)),
    ) -> Result<Vec3<f64>, CreaseReading> {
        beside2(surface, (u, v), [bound, bound])
    }

    fn unit(x: f64, y: f64, z: f64) -> Vec3<f64> {
        Vec3::new(x, y, z).normalize()
    }

    /// **Each side of a fold reads its own normal.** On the creased
    /// carrier at `(0.3, 0, 0)`, on the crease `v = ½`, a box leaving
    /// below the crease reads the lower half's normal `∝ (0, 1, 1)`, one
    /// leaving above it the upper half's `∝ (0, −1, 1)`, and a box off
    /// the crease in `u` changes nothing (no line of `u` is near).
    #[test]
    fn each_side_of_a_fold_reads_its_own_normal() {
        let c = creased();
        let at = (0.65, 0.5);
        for ((v0, v1), want, side) in [
            ((0.3, 0.5), unit(0.0, 1.0, 1.0), "below"),
            ((0.5, 0.7), unit(0.0, -1.0, 1.0), "above"),
        ] {
            let got =
                beside(&c, at, ((0.6, 0.7), (v0, v1))).unwrap_or_else(|e| panic!("{side}: {e:?}"));
            assert!((got - want).norm() < 1e-12, "{side}: {got:?}");
        }
    }

    /// **A box along the crease, or astride it, refuses in band**: neither
    /// side, or both, is named, so the line's own reading comes back.
    #[test]
    fn a_box_along_or_astride_the_crease_refuses() {
        let c = creased();
        for (box_v, what) in [((0.5, 0.5), "along"), ((0.3, 0.7), "astride")] {
            let got = beside(&c, (0.65, 0.5), ((0.6, 0.7), box_v));
            assert!(got.is_err(), "{what}: {got:?}");
        }
        // Within the band of the crease on the far side still names none.
        let a = band().zero() / 10.0;
        let got = beside(&c, (0.65, 0.5), ((0.6, 0.7), (0.5 - a, 0.5 + a)));
        assert!(got.is_err(), "in band: {got:?}");
    }

    /// **An irregular side refuses**: the fold's upper span has coincident
    /// control rows, so `S_v` vanishes on it, and a box leaving into it
    /// reads no side of the crease there, while the lower side reads; and
    /// a side decided whose one-sided jet is irregular reads no normal.
    #[test]
    fn an_irregular_side_refuses_and_the_regular_one_reads() {
        let c = ruled(
            KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap(),
            &[(-1.0, 1.0), (0.0, 0.0), (0.0, 0.0)],
        );
        let at = (0.65, 0.5);
        let upper = beside(&c, at, ((0.6, 0.7), (0.5, 0.7)));
        assert!(upper.is_err(), "the degenerate side read {upper:?}");
        let lower = beside(&c, at, ((0.6, 0.7), (0.3, 0.5))).unwrap();
        assert!((lower - unit(0.0, 1.0, 1.0)).norm() < 1e-12, "{lower:?}");
        // A named side whose span is irregular: above the crease `S_v`
        // runs along the edge, parallel to `S_u`, so the side is decided
        // (its speed across the line is not zero) and its normal is not.
        let along = spline(
            |s| Point3::new(s, 0.0, 0.0),
            KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap(),
            &[
                Vec3::new(0.0, -1.0, 1.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
            ],
        );
        let upper = beside(&along, at, ((0.6, 0.7), (0.5, 0.7)));
        assert!(upper.is_err(), "the side along the edge read {upper:?}");
        let lower = beside(&along, at, ((0.6, 0.7), (0.3, 0.5))).unwrap();
        assert!((lower - unit(0.0, 1.0, 1.0)).norm() < 1e-12, "{lower:?}");
    }

    /// **A closed chart's seam reads the end the box touches**: the
    /// two-span fin's first span leaves the edge along `(0, 1, 1)` and its
    /// last arrives back along `(0, 1, −1)`, so the two ends read the
    /// normals `∝ (0, −1, 1)` and `∝ (0, 1, 1)`, each on its own span (a
    /// one-span chart's two ends share their span).
    #[test]
    fn a_seam_reads_the_end_the_box_touches() {
        let fin = ruled(
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap(),
            &[(0.0, 0.0), (1.0, 1.0), (-1.0, 1.0), (0.0, 0.0)],
        );
        let first = beside(&fin, (0.65, 0.0), ((0.6, 0.7), (0.0, 0.2))).unwrap();
        let last = beside(&fin, (0.65, 1.0), ((0.6, 0.7), (0.8, 1.0))).unwrap();
        assert!((first - unit(0.0, -1.0, 1.0)).norm() < 1e-12, "{first:?}");
        assert!((last - unit(0.0, 1.0, 1.0)).norm() < 1e-12, "{last:?}");
        let both = beside(&fin, (0.65, 0.0), ((0.6, 0.7), (0.0, 1.0)));
        assert!(both.is_err(), "a box over the whole loop read {both:?}");
    }

    /// **A reach on the far side of the line, within its band, reads as
    /// not past at either side's speed** (review of 6749, M1): the lower
    /// half is a thousand times faster across the line than the upper, so
    /// the vertex a quarter-band above the line, its box running up from
    /// it, measures `at − lo` negative and past the band at the lower
    /// speed. That is "does not reach below", and the upper side reads.
    #[test]
    fn a_reach_a_hair_negative_at_the_other_sides_speed_reads_not_past() {
        let c = ruled(
            KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap(),
            &[(-1000.0, 1000.0), (0.0, 0.0), (1.0, 1.0)],
        );
        let v = 0.5 + band().zero() / 4.0 / 2.83;
        let got = beside(&c, (0.65, v), ((0.6, 0.7), (v, 0.7))).unwrap();
        assert!((got - unit(0.0, -1.0, 1.0)).norm() < 1e-9, "{got:?}");
    }

    /// **A box reaching an in-band distance past the line on one side and
    /// far past it on the other refuses in band**: the side decision's own
    /// reading, which [`geom_core::ladder::Refusal::of_crease`] ranks as a
    /// margin, not a limit.
    #[test]
    fn an_in_band_reach_on_one_side_refuses_in_band() {
        let c = creased();
        // The lower half's speed across the line is 2·√2: four of the
        // band's zero in metres, inside its escalation.
        let a = 4.0 * band().zero() / (2.0 * 2f64.sqrt());
        for bound in [((0.6, 0.7), (0.5 - a, 0.7)), ((0.6, 0.7), (0.3, 0.5 + a))] {
            let got = beside(&c, (0.65, 0.5), bound);
            let Err(reading) = got else {
                panic!("{bound:?} read {got:?}");
            };
            assert!(
                matches!(
                    geom_core::ladder::Refusal::of_crease(reading),
                    geom_core::ladder::Refusal::Margin(_)
                ),
                "{bound:?}: {reading:?}"
            );
        }
    }

    /// **The lower side irregular refuses too**: below the crease the
    /// span's `S_v` runs along the edge, parallel to `S_u`, so a box
    /// leaving below names that side and reads no normal there, while the
    /// upper side reads.
    #[test]
    fn an_irregular_lower_side_refuses_and_the_upper_reads() {
        let c = spline(
            |s| Point3::new(s, 0.0, 0.0),
            KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap(),
            &[
                Vec3::new(-1.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 1.0),
            ],
        );
        let at = (0.65, 0.5);
        let lower = beside(&c, at, ((0.6, 0.7), (0.3, 0.5)));
        assert!(lower.is_err(), "the side along the edge read {lower:?}");
        let upper = beside(&c, at, ((0.6, 0.7), (0.5, 0.7))).unwrap();
        assert!((upper - unit(0.0, -1.0, 1.0)).norm() < 1e-12, "{upper:?}");
    }

    /// **A sector straddling a fold refuses; one straddling a flat crease
    /// reads** (review of 6749, M2): the corner's two bounding edges leave
    /// on opposite sides of the line, so the sector holds both halves. On
    /// the fold their normals differ, and no one normal is the corner's;
    /// on a flat chart with the same knot crease they are one, and it
    /// reads.
    #[test]
    fn a_sector_straddling_a_fold_refuses_and_a_flat_one_reads() {
        let below = ((0.6, 0.7), (0.3, 0.5));
        let above = ((0.6, 0.7), (0.5, 0.7));
        let fold = beside2(&creased(), (0.65, 0.5), [below, above]);
        assert!(
            matches!(fold, Err(Ok(_))),
            "a straddled fold refuses as the line's limit: {fold:?}"
        );
        let flat = ruled(
            KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap(),
            &[(-1.0, 0.0), (0.0, 0.0), (1.0, 0.0)],
        );
        let got = beside2(&flat, (0.65, 0.5), [below, above]).unwrap();
        assert!((got - unit(0.0, 0.0, 1.0)).norm() < 1e-12, "{got:?}");
    }

    /// **Two lines near in one direction refuse**: two crease runs a hair
    /// apart in `v`, both within the band of the corner, leave the span
    /// between them unread, whatever the box names.
    #[test]
    fn two_creases_near_in_one_direction_refuse() {
        let h = band().zero() / 10.0;
        let c = ruled(
            KnotVector::clamped(vec![0.0, 0.0, 0.5, 0.5 + h, 1.0, 1.0], 1).unwrap(),
            &[(-1.0, 1.0), (0.0, 0.0), (0.0, 0.0), (1.0, 1.0)],
        );
        let got = beside(&c, (0.65, 0.5), ((0.6, 0.7), (0.5, 0.7)));
        assert!(got.is_err(), "{got:?}");
    }

    /// **A reflex corner whose bounds both leave on one side refuses**
    /// (review of 6749, M2, ruled (a)): above the fold's crease, a corner
    /// whose two chords leave into the upper half reads that half's normal
    /// where it is convex there, the positive cone of its chords; the
    /// same two chords taken the other way round bound the reflex corner,
    /// which sweeps through the lower half too, and the two halves'
    /// normals differ.
    #[test]
    fn a_reflex_corner_whose_bounds_leave_on_one_side_refuses() {
        let c = creased();
        let above = ((0.6, 0.7), (0.5, 0.7));
        let (a, b) = (Vec3::new(-0.1, 0.1, 0.1), Vec3::new(0.1, 0.1, 0.1));
        let convex = beside_with(&c, (0.65, 0.5), [above, above], (a, b)).unwrap();
        assert!((convex - unit(0.0, -1.0, 1.0)).norm() < 1e-12, "{convex:?}");
        let reflex = beside_with(&c, (0.65, 0.5), [above, above], (b, a));
        assert!(
            matches!(reflex, Err(Ok(_))),
            "the reflex corner read {reflex:?}"
        );
    }
}
