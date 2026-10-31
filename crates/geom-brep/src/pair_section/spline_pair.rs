//! **Two spline walls: the pair read whole.** The section of two NURBS
//! walls (an approximating surface read as its fit) is C3's whole
//! reading, [`crate::ssi::Section::read_all`], over the product of the
//! two knot rectangles: every branch with its certificate, every region
//! of the rows and corners the caller handed in as known, and the
//! accounting that they are all of them. No pose is decided before it:
//! the pair's tangency is its certificate's (C2), the march's
//! transversality and the boundary pass's.
//!
//! Each piece of the section is a component:
//!
//! - a closed branch is a [`ComponentShape::Loop`], with its witness;
//! - an open branch, which runs between two crossings of the
//!   rectangles, a `Side` region (a handed-in row) and a `Corner` region
//!   (a handed-in corner where the section leaves the domain) are
//!   [`ComponentShape::Open`], reaching the sides of each chart they end
//!   on ([`ChartReach`]); a branch that ends anywhere else is refused,
//!   never read as reaching.
//!
//! **A branch's witness** is the first wall's point at a root of the
//! section the Krawczyk test isolates on a slice through the branch's
//! fitted middle, inside the limb-3 tube box that holds it, and its pad
//! the enclosure's reach ([`crate::ssi::pair::branch_witness`]): limb 3
//! proves the section in the box one arc, the branch's, so the root is a
//! point of the branch. A branch no slice isolates a root of carries no
//! witness.
//!
//! The fitted carriers are not kept: no reader carries a spline pair's
//! section ([`UNCARRIED`]).
//!
//! A refusal maps by name: a tangency the reading reports (the march's
//! transversality, a tangency along a handed-in row) and an unresolved
//! face of the boundary pass (the section touching or running along a
//! side, or through a corner, that nothing handed in covers) are a pinch
//! named by its decision, an escalation is the value's escalation, and
//! any other refusal, an accounting that did not close among them, is
//! [`SectionError::Unaccounted`], carrying its cause.

use geom::{NurbsSurface, Surface};
use geom_core::{Band, Decide, Point3, Real};

use super::{
    ChartReach, ClassCarrier, ComponentEnds, ComponentShape, Components, PairPose, PairSection,
    SectionComponent, SsiLane,
};
use crate::ExtentBall;
use crate::Pcurve;
use crate::intersect::SectionError;
use crate::ssi::{BoundaryChart, BranchEnd, PairCorner, PairRegion, PairRow, SsiError};

/// **A row the caller hands in as known**: each wall's image of one
/// curve over one parameter span, the first wall's then the second's, in
/// the order the pose is asked of ([`crate::ssi::PairRow`], which admits
/// only a row on a side of each chart).
#[derive(Clone, Debug)]
pub struct KnownRow<T: Real> {
    /// The first wall's image, then the second's.
    pub images: [Pcurve<T>; 2],
    /// The span both images are stated over.
    pub span: (T, T),
}

/// **Two spline walls**, in the order the section was asked of, with the
/// rows and corners the caller handed in as known.
#[derive(Clone, Debug)]
pub struct SplinePairPose<T: Real> {
    a: Surface<T>,
    b: Surface<T>,
    nets: [NurbsSurface<T>; 2],
    rows: Vec<KnownRow<T>>,
    corners: Vec<PairCorner>,
}

impl<T: Real> SplinePairPose<T> {
    /// The pose of `a` and `b`, each a NURBS or an approximating
    /// surface; `None` for any other pair.
    #[must_use]
    pub fn of(a: &Surface<T>, b: &Surface<T>) -> Option<Self> {
        Some(Self {
            nets: [a.spline_chart()?.clone(), b.spline_chart()?.clone()],
            a: a.clone(),
            b: b.clone(),
            rows: Vec::new(),
            corners: Vec::new(),
        })
    }

    /// The pose with `rows` and `corners` handed in as known
    /// ([`crate::ssi::Section::with_known`]), each in the order the pose
    /// was asked of.
    #[must_use]
    pub fn with_known(mut self, rows: Vec<KnownRow<T>>, corners: Vec<PairCorner>) -> Self {
        self.rows = rows;
        self.corners = corners;
        self
    }

    /// The two surfaces in the order the pose was asked of.
    pub fn operands(&self) -> (&Surface<T>, &Surface<T>) {
        (&self.a, &self.b)
    }

    /// The two nets, in the order the pose was asked of.
    pub fn nets(&self) -> &[NurbsSurface<T>; 2] {
        &self.nets
    }

    /// **A ball holding the whole section**: each wall lies in its net's
    /// convex hull (its weights are positive), so the section in the ball
    /// about the first net's control points.
    pub fn ball(&self) -> ExtentBall<T> {
        let control = self.nets[0].control();
        let Some(&first) = control.first() else {
            return ExtentBall::new(Point3::origin(), T::zero());
        };
        let (lo, hi) = control.iter().fold((first, first), |(lo, hi), &p| {
            (
                Point3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z)),
                Point3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z)),
            )
        });
        let center = lo + (hi - lo) * T::from_f64(0.5);
        let radius = control
            .iter()
            .fold(T::zero(), |r, &p| r.max((p - center).norm()));
        ExtentBall::new(center, radius)
    }

    /// The pair's section with these components, nothing decided Zero.
    fn section(self, components: Components<T>) -> PairSection<T> {
        let lever = self.ball().radius();
        PairSection {
            pose: PairPose::SplinePair(Box::new(self)),
            decided: Vec::new(),
            components,
            nodes: Vec::new(),
            lever,
        }
    }
}

/// **Why a spline pair's component has no carrier to carry**: the
/// section is read whole for the section certificate alone.
pub(super) const UNCARRIED: &str = "a spline pair's section is read whole for the section \
    certificate alone, which carries none of its components";

/// The note a region's component carries.
const REGION: &str = "a Side or Corner region of a handed-in row or corner: the section there \
                      is the row, or leaves the domain at the corner, and no carrier is minted \
                      for it";

/// **The pair's classification**: the march door's arm, with the rows
/// and corners handed in.
///
/// # Errors
///
/// [`SectionError::WrongLane`] for a pair that is not two spline walls,
/// [`SectionError::NoMarch`] with no `lane`, and the whole reading's
/// refusal, mapped (module docs).
pub(super) fn classified<T: Decide>(
    a: &Surface<T>,
    b: &Surface<T>,
    (rows, corners): (Vec<KnownRow<T>>, Vec<PairCorner>),
    lane: Option<SsiLane<T>>,
    band: Band,
) -> Result<PairSection<T>, SectionError> {
    let pose = SplinePairPose::of(a, b)
        .ok_or(SectionError::WrongLane {
            expected: "two NURBS or approximating surfaces",
        })?
        .with_known(rows, corners);
    let lane = lane.ok_or(SectionError::NoMarch { scalar: T::NAME })?;
    (lane.pair)(pose, band)
}

/// **The section read whole** (module docs).
///
/// # Errors
///
/// The whole reading's refusal, mapped, and [`SectionError::Unaccounted`]
/// for a branch that ends off both rectangles.
pub(super) fn section(
    pose: SplinePairPose<f64>,
    band: Band,
) -> Result<PairSection<f64>, SectionError> {
    let ball = pose.ball();
    let other = SplinePairPose {
        nets: [pose.nets[1].clone(), pose.nets[0].clone()],
        a: pose.b.clone(),
        b: pose.a.clone(),
        rows: Vec::new(),
        corners: Vec::new(),
    }
    .ball();
    let reach = ball.center().distance(other.center()) + ball.radius() + other.radius();
    let domain = super::domain_about(ball.center(), reach, 2.0 * reach);
    let [na, nb] = &pose.nets;
    // A row that is not a row on a side of each chart is not admitted:
    // the section along it is then the boundary pass's to read.
    let rows = pose
        .rows
        .iter()
        .filter_map(|r| {
            let [ia, ib] = r.images.clone();
            PairRow::new((na, ia), (nb, ib), r.span)
        })
        .collect();
    let reading = crate::ssi::Section::of(na, nb, domain, band)
        .map_err(refused)?
        .with_known(rows, pose.corners.clone())
        .read_all()
        .map_err(refused)?;
    listed(pose, reading)
}

/// The whole reading's refusal as the section value's (module docs).
fn refused(e: SsiError) -> SectionError {
    if let Some(predicate) = e.tangency() {
        return SectionError::Pinched { predicate };
    }
    match e {
        SsiError::PairBoundaryUnresolved { .. } => SectionError::Pinched {
            predicate: "ssi_pair_boundary",
        },
        SsiError::Escalated { cause, .. } | SsiError::CertificateEscalated { cause, .. } => {
            SectionError::Escalated(cause)
        }
        other => SectionError::Unaccounted {
            cause: other.to_string(),
        },
    }
}

/// **The whole reading as the section's components** (module docs).
///
/// # Errors
///
/// [`SectionError::Unaccounted`] for a branch that ends off both
/// rectangles.
fn listed(
    pose: SplinePairPose<f64>,
    reading: crate::ssi::PairReading,
) -> Result<PairSection<f64>, SectionError> {
    let open = |rims: [ChartReach; 2]| ComponentShape::Open {
        ends: ComponentEnds {
            rims,
            ..ComponentEnds::default()
        },
    };
    let on = |p: crate::ssi::BoundaryPoint| {
        let side = ChartReach::side(p.side);
        match p.chart {
            BoundaryChart::First => [side, ChartReach::default()],
            BoundaryChart::Second => [ChartReach::default(), side],
        }
    };
    let out = reading.outcome;
    let mut components = Vec::with_capacity(out.branches.len() + reading.regions.len());
    for b in &out.branches {
        let shape = match b.end {
            BranchEnd::Closed => ComponentShape::Loop,
            BranchEnd::Rim { from, to } => {
                let [fa, fb] = on(from);
                let [ta, tb] = on(to);
                open([fa.or(ta), fb.or(tb)])
            }
            BranchEnd::Cut => {
                return Err(SectionError::Unaccounted {
                    cause: "a branch of the spline pair's reading ended off both rectangles, \
                            where no side of either chart holds it"
                        .to_owned(),
                });
            }
        };
        let witness = crate::ssi::pair::branch_witness((&pose.nets[0], &pose.nets[1]), b);
        components.push(SectionComponent {
            shape,
            winds: [false, false],
            witness: witness.map(|(q, _)| q),
            pad: witness.map_or(0.0, |(_, pad)| pad),
            carrier: ClassCarrier::Unminted(UNCARRIED),
        });
    }
    components.extend(reading.regions.iter().map(|region| {
        let rims = match *region {
            PairRegion::Side { sides } => sides.map(ChartReach::side),
            PairRegion::Corner { corners } => corners.map(ChartReach::corner),
        };
        SectionComponent {
            shape: open(rims),
            winds: [false, false],
            witness: None,
            pad: 0.0,
            carrier: ClassCarrier::Unminted(REGION),
        }
    }));
    Ok(pose.section(Components::Counted(components)))
}
