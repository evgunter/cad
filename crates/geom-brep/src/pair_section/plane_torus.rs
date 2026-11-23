//! **The plane × torus pose** ([`super`]'s module docs): a plane against
//! a ring torus, decided once, and the section built over a tilted pose.
//!
//! # The readings, in order
//!
//! 1. The torus's ring convention.
//! 2. The C5 table's own axis rows ([`crate::plane_torus_section`]),
//!    levered from the torus's centre over its ball: a plane holding the
//!    axis, square to it, or parallel to it short of the inner equator
//!    is the closed form's, and the section value reads it elsewhere.
//!    Every other pose is the general rung's.
//! 3. **The critical gap, in closed form.** With `n` the plane's unit
//!    normal, `a` the torus's axis, `s = ‖n − (n·a)a‖` and `h` the
//!    plane's height over the torus's centre along `n`, the core circle's
//!    heights over the plane range over `σ·s·R − h`, `σ = ±1`, with their
//!    extremes at the two core points `k±` in the meridian half-planes
//!    toward and away from `n`. A common normal at a common point is the
//!    torus's normal there, which lies in that point's meridian plane, so
//!    the plane touches the torus only at the tube circle about `k±`,
//!    where that circle's distance from it is `r`: the rows `r − |h ∓
//!    s·R|` (`pt_near_tube`, `pt_far_tube`), each read across its
//!    rounding. `Zero` on either is a tangency.
//!
//! # The section
//!
//! The rows count the loops (the section certificate's `PlaneCut`, read
//! on this pose): a plane cutting both tube circles of the meridian
//! plane through `n` crosses every meridian, two loops round the axis; a
//! plane passing between them (`s·R − r − |h|` positive,
//! `pt_between_tubes`) cuts two loops round the tube; one cutting a
//! single tube circle cuts one null loop; any other misses the torus.
//! Each loop is the general rung's carrier ([`ClassCarrier::General`]),
//! marched over the torus's ball and matched to the count by its
//! winding about the torus, which the two essential pairs claim and the
//! null loop does not. A plane has no channel, so no loop winds about
//! it. A tangent pose lists one tangent component, a point at the tube
//! circle it touches.

use geom::Surface;
use geom_core::k_stats::decide_across_reported;
use geom_core::{Band, Decide, Indeterminate, MarginDiag, Point3, Real, Sign, rounding_charge};

use super::torus::{Ring, TorusGap, TorusPoseRefusal, TorusQuestion, lever, tangent_component};
use super::{
    ClassCarrier, ComponentShape, Components, PairPose, PairSection, SectionComponent, square_part,
};
use crate::ExtentBall;
use crate::intersect::SectionError;

/// The near-tube row: the plane against the tube circle about `k₊`.
const NEAR: [&str; 2] = ["pt_near_tube", "pt_near_tube_floor"];
/// The far-tube row: the plane against the tube circle about `k₋`.
const FAR: [&str; 2] = ["pt_far_tube", "pt_far_tube_floor"];
/// The between row: the plane passes between the two tube circles.
const BETWEEN: [&str; 2] = ["pt_between_tubes", "pt_between_tubes_floor"];

/// **How the plane cuts the torus**, decided over a transverse gap.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PtCut {
    /// Both tube circles: two loops, each round the axis.
    AroundAxis,
    /// Between them: two loops, each round the tube.
    AroundTube,
    /// One tube circle, about `k₊` where `near`: one null loop.
    Null {
        /// Whether it is the tube circle about `k₊`.
        near: bool,
    },
    /// Neither, and not between: no section.
    Empty,
}

/// **How the plane stands to the torus**, decided.
#[derive(Clone, Copy, Debug)]
pub enum PtStance {
    /// Holding the axis, square to it, or parallel to it short of the
    /// inner equator: the closed form's pose, read elsewhere.
    Closed,
    /// Every other pose, its loops counted over a transverse gap; `None`
    /// over a tangent one.
    Tilted(Option<PtCut>),
}

/// **The plane × torus pose, decided once** (module docs).
#[derive(Clone, Debug)]
pub struct PlaneTorusPose<T: Real> {
    plane: Surface<T>,
    torus: Surface<T>,
    plane_first: bool,
    ring: Ring<T>,
    stance: PtStance,
    gap: TorusGap<T>,
}

impl<T: Decide> PlaneTorusPose<T> {
    /// **Decides the pose of `a` and `b`**, a plane and a torus in
    /// either order (module docs, the readings).
    ///
    /// # Errors
    ///
    /// [`TorusPoseRefusal`]; another pair is `None`.
    pub fn decide(
        a: &Surface<T>,
        b: &Surface<T>,
        band: Band,
    ) -> Option<Result<Self, TorusPoseRefusal>> {
        let (plane, torus, plane_first) = match (a, b) {
            (Surface::Plane { .. }, Surface::Torus { .. }) => (a, b, true),
            (Surface::Torus { .. }, Surface::Plane { .. }) => (b, a, false),
            _ => return None,
        };
        let &Surface::Plane { frame } = plane else {
            return None;
        };
        let ring = Ring::of(torus, band)?;
        Some((|| {
            let ring = ring?;
            let closed = match crate::intersect::plane_torus_section(
                plane,
                torus,
                lever(ring.center(), ring.ball()),
                band,
            ) {
                Err(SectionError::RoutesToGeneralRung { .. }) => false,
                Err(SectionError::Escalated(diag)) => {
                    return Err(TorusPoseRefusal::Undecided {
                        question: TorusQuestion::Symmetry,
                        diag,
                    });
                }
                Err(SectionError::DegenerateTorus(t)) => return Err(TorusPoseRefusal::Torus(t)),
                // The closed form classified the pose; what it mints is
                // its own reader's to refuse.
                Ok(_) | Err(_) => true,
            };
            let (stance, gap) = if closed {
                (PtStance::Closed, TorusGap::Transverse)
            } else {
                tilted(ring, (frame.origin(), frame.axis()), band)?
            };
            Ok(Self {
                plane: plane.clone(),
                torus: torus.clone(),
                plane_first,
                ring,
                stance,
                gap,
            })
        })())
    }

    /// The two surfaces in the order the pose was asked of.
    pub fn operands(&self) -> (&Surface<T>, &Surface<T>) {
        if self.plane_first {
            (&self.plane, &self.torus)
        } else {
            (&self.torus, &self.plane)
        }
    }

    /// The plane, as given.
    pub fn plane(&self) -> &Surface<T> {
        &self.plane
    }

    /// The torus, as given.
    pub fn torus(&self) -> &Surface<T> {
        &self.torus
    }

    /// The torus, guarded.
    pub fn ring(&self) -> Ring<T> {
        self.ring
    }

    /// The torus's ball, which holds the section.
    pub fn ball(&self) -> ExtentBall<T> {
        self.ring.ball()
    }

    /// How the plane stands.
    pub fn stance(&self) -> PtStance {
        self.stance
    }

    /// The critical gap.
    pub fn gap(&self) -> TorusGap<T> {
        self.gap
    }

    /// The tangency the pose decided, if any.
    pub fn tangency(&self) -> Option<super::ZeroVerdict> {
        self.gap.tangency()
    }
}

/// A gap row's reading, or the pose's refusal on it.
fn row<T: Decide>(
    names: [&'static str; 2],
    datum: T,
    slack: T,
    band: Band,
) -> Result<(Sign, MarginDiag), TorusPoseRefusal> {
    decide_across_reported(names, datum, slack, band).map_err(|diag: Indeterminate| {
        TorusPoseRefusal::Undecided {
            question: TorusQuestion::Gap,
            diag,
        }
    })
}

/// The tilted pose's gap and count (module docs), the plane through `q`
/// with normal `n`.
fn tilted<T: Decide>(
    ring: Ring<T>,
    (q, n): (Point3<T>, geom_core::Vec3<T>),
    band: Band,
) -> Result<(PtStance, TorusGap<T>), TorusPoseRefusal> {
    let n = n.normalize();
    let (c, [big_r, r]) = (ring.center(), ring.radii());
    let (perp, s) = square_part(n, ring.axis());
    let h = n.dot(q - c);
    let slack = rounding_charge(ring.size() + (q - Point3::origin()).norm());
    let tangent = |row: &'static str, margin, sigma: Option<T>| {
        // The touch is on the tube circle about the core point toward
        // `σ·m`, `r` from it along the plane's normal.
        let at = sigma.and_then(|sigma| {
            let k = c + perp / s * (sigma * big_r);
            let above = n.dot(k - q);
            let p = k - n * (above.select_le_zero(T::zero() - r, r));
            [p.x, p.y, p.z]
                .into_iter()
                .all(geom_core::is_finite_length)
                .then_some(p)
        });
        Ok((
            PtStance::Tilted(None),
            TorusGap::Tangent { row, margin, at },
        ))
    };
    let near = row(NEAR, r - (h - s * big_r).abs(), slack, band)?;
    if near.0 == Sign::Zero {
        return tangent(NEAR[0], near.1, Some(T::one()));
    }
    let far = row(FAR, r - (h + s * big_r).abs(), slack, band)?;
    if far.0 == Sign::Zero {
        return tangent(FAR[0], far.1, Some(T::zero() - T::one()));
    }
    let cut = match (near.0 == Sign::Positive, far.0 == Sign::Positive) {
        (true, true) => PtCut::AroundAxis,
        (true, false) => PtCut::Null { near: true },
        (false, true) => PtCut::Null { near: false },
        (false, false) => {
            let between = row(BETWEEN, s * big_r - r - h.abs(), slack, band)?;
            match between.0 {
                Sign::Positive => PtCut::AroundTube,
                Sign::Negative => PtCut::Empty,
                Sign::Zero => return tangent(BETWEEN[0], between.1, None),
            }
        }
    };
    Ok((PtStance::Tilted(Some(cut)), TorusGap::Transverse))
}

/// **A point of the null loop**: where the plane cuts the tube circle
/// `T_σ` about `k_σ` it cuts, in the meridian half-plane `Π` through it.
/// With `x` along `ê`, the unit part of `n` square to the axis, and `y`
/// along the axis, `T_σ` is centred `(σR, 0)` with radius `r`, and the
/// plane is the line `s·x + (n·a)·y = h`, whose unit normal is
/// `(s, n·a)`: `d = h − σsR` is its offset from the centre, `(σR + ds,
/// d·(n·a))` its foot, and the point `√(r² − d²)` from the foot along the
/// line lies on both surfaces. `None` where it is not finite.
fn null_witness<T: Decide>(pose: &PlaneTorusPose<T>, near: bool) -> Option<Point3<T>> {
    let &Surface::Plane { frame } = &pose.plane else {
        return None;
    };
    let ring = pose.ring;
    let (c, a, [big_r, r]) = (ring.center(), ring.axis(), ring.radii());
    let n = frame.axis().normalize();
    let (perp, s) = square_part(n, a);
    let (h, na) = (n.dot(frame.origin() - c), n.dot(a));
    let sigma = if near { T::one() } else { T::zero() - T::one() };
    let d = h - sigma * s * big_r;
    let half = ((r - d.abs()) * (r + d.abs())).sqrt();
    let (x, y) = (sigma * big_r + d * s - na * half, d * na + s * half);
    let p = c + perp / s * x + a * y;
    [p.x, p.y, p.z]
        .into_iter()
        .all(geom_core::is_finite_length)
        .then_some(p)
}

/// **The plane × torus section over its tilted pose** (module docs, the
/// section); a closed pose is read elsewhere and never reaches it.
pub(super) fn plane_torus<T: Decide>(
    pose: PlaneTorusPose<T>,
) -> Result<PairSection<T>, SectionError> {
    let torus_first = !pose.plane_first;
    let order = |torus: bool| {
        if torus_first {
            [torus, false]
        } else {
            [false, torus]
        }
    };
    let loops = |count: usize, winds: bool| {
        Components::Counted(
            (0..count)
                .map(|_| {
                    SectionComponent::closed_form(
                        ComponentShape::Loop,
                        order(winds),
                        None,
                        ClassCarrier::General,
                    )
                })
                .collect(),
        )
    };
    let components = match pose.stance {
        PtStance::Closed => {
            return Err(SectionError::WrongLane {
                expected: "a plane tilted against the torus's axis",
            });
        }
        PtStance::Tilted(Some(PtCut::AroundAxis | PtCut::AroundTube)) => loops(2, true),
        PtStance::Tilted(Some(PtCut::Null { near })) => {
            let witness = null_witness(&pose, near);
            Components::Counted(vec![SectionComponent::closed_form(
                ComponentShape::Loop,
                [false, false],
                witness,
                ClassCarrier::General,
            )])
        }
        PtStance::Tilted(Some(PtCut::Empty)) => Components::Counted(Vec::new()),
        PtStance::Tilted(None) => {
            let touch = match pose.gap {
                TorusGap::Tangent { at, .. } => at,
                TorusGap::Transverse => None,
            };
            Components::Counted(vec![tangent_component(touch, [false, false])])
        }
    };
    Ok(PairSection {
        lever: crate::uniform_curvature_arm(&pose.torus),
        decided: pose.tangency().into_iter().collect(),
        pose: PairPose::PlaneTorus(pose),
        components,
        nodes: Vec::new(),
        node_sets: Vec::new(),
    })
}
