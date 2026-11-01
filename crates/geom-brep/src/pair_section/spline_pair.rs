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
//! A refusal maps by name, as the plane × spline lane's does
//! (`pair_section::whole_reading_refused`, the one classification): a
//! tangency the reading reports ([`crate::ssi::SsiError::tangency`]: the
//! march's transversality, a tangency along a handed-in row, an
//! unresolved face of the boundary pass where the section touches or
//! runs along a side, or through a corner, that nothing handed in covers)
//! is a pinch named by its decision; an escalation is the value's
//! escalation; and any other refusal, the pass's incapacity
//! ([`crate::ssi::PairBoundaryCause`]) and an accounting that did not
//! close among them, is [`SectionError::Unaccounted`], carrying its
//! cause. A row the caller hands in that is no row on a side of each
//! chart is refused the same way, naming why
//! ([`crate::ssi::PairRowRefusal`]).

use geom::{NurbsSurface, Surface};
use geom_core::{Band, Decide, Point3, Real};

use super::{
    ChartReach, ClassCarrier, ComponentEnds, ComponentShape, Components, PairPose, PairSection,
    SectionComponent, SsiLane,
};
use crate::ExtentBall;
use crate::Pcurve;
use crate::intersect::SectionError;
use crate::ssi::{BoundaryChart, BranchEnd, PairCorner, PairRegion, PairRow};

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

/// **What a caller hands a spline pair's reading as known**: the rows and
/// the corners the two walls share, each in the order the pose is asked
/// of. The interior-loop guard builds it from the faces' edges.
#[derive(Clone, Debug)]
pub struct KnownPair<T: Real> {
    /// The shared rows.
    pub rows: Vec<KnownRow<T>>,
    /// The shared corners.
    pub corners: Vec<PairCorner>,
}

impl<T: Real> Default for KnownPair<T> {
    fn default() -> Self {
        Self {
            rows: Vec::new(),
            corners: Vec::new(),
        }
    }
}

/// **Two spline walls**, in the order the section was asked of, with the
/// rows and corners the caller handed in as known.
#[derive(Clone, Debug)]
pub struct SplinePairPose<T: Real> {
    a: Surface<T>,
    b: Surface<T>,
    nets: [NurbsSurface<T>; 2],
    known: KnownPair<T>,
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
            known: KnownPair::default(),
        })
    }

    /// The pose with `known` handed in ([`crate::ssi::Section::of_known`]).
    #[must_use]
    pub fn with_known(mut self, known: KnownPair<T>) -> Self {
        self.known = known;
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
    known: KnownPair<T>,
    lane: Option<SsiLane<T>>,
    band: Band,
) -> Result<PairSection<T>, SectionError> {
    let pose = SplinePairPose::of(a, b)
        .ok_or(SectionError::WrongLane {
            expected: "two NURBS or approximating surfaces",
        })?
        .with_known(known);
    let lane = lane.ok_or(SectionError::NoMarch { scalar: T::NAME })?;
    (lane.spline_pair)(pose, band)
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
        known: KnownPair::default(),
    }
    .ball();
    let reach = ball.center().distance(other.center()) + ball.radius() + other.radius();
    let domain = super::domain_about(ball.center(), reach, 2.0 * reach);
    let [na, nb] = &pose.nets;
    // A row the caller hands in that is not a row on a side of each
    // chart is the caller's defect, refused naming it.
    let rows = pose
        .known
        .rows
        .iter()
        .map(|r| {
            let [ia, ib] = r.images.clone();
            PairRow::new((na, ia), (nb, ib), r.span).map_err(|e| SectionError::Unaccounted {
                cause: e.to_string(),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let reading =
        crate::ssi::Section::of_known(na, nb, domain, band, (rows, pose.known.corners.clone()))
            .map_err(super::whole_reading_refused)?
            .read_all()
            .map_err(super::whole_reading_refused)?;
    listed(pose, reading)
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
