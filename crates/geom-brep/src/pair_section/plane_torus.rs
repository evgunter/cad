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
//!    s·R|` (`pt_near_tube`, `pt_far_tube`), distances the band reads
//!    as they stand. `Zero` on either is a tangency.
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
//! it. A tangent pose lists one tangent component: a point where it
//! touches the tube's outer half, an extreme of the height, and a loop,
//! classification only, at a saddle on the inner half.

use geom::Surface;
use geom_core::k_stats::decide_reported;
use geom_core::{Band, Decide, Indeterminate, Margin, MarginDiag, Point3, Real, Sign};

use super::torus::{Ring, TorusGap, TorusPoseRefusal, TorusQuestion, lever, tangent_component};
use super::{
    ClassCarrier, ComponentShape, Components, PairPose, PairSection, SectionComponent, square_part,
};
use crate::ExtentBall;
use crate::intersect::{PlaneTorusSection, SectionError};

/// The near-tube row: the plane against the tube circle about `k₊`.
const NEAR: &str = "pt_near_tube";
/// The far-tube row: the plane against the tube circle about `k₋`.
const FAR: &str = "pt_far_tube";
/// The between row: the plane passes between the two tube circles.
const BETWEEN: &str = "pt_between_tubes";

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
    /// The closed form's reading, where it classified the pose.
    closed: Option<Result<PlaneTorusSection<T>, SectionError>>,
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
        let site = crate::intersect::SectionPlane::site(plane)?;
        let ring = Ring::of(torus, band)?;
        Some((|| {
            let ring = ring?;
            let read = asked(plane, torus, ring, band);
            let closed = match &read {
                Err(SectionError::RoutesToGeneralRung { .. }) => false,
                Err(SectionError::Escalated(diag)) => {
                    return Err(TorusPoseRefusal::Undecided {
                        question: TorusQuestion::Symmetry,
                        diag: *diag,
                    });
                }
                Err(SectionError::DegenerateTorus(t)) => {
                    return Err(TorusPoseRefusal::Torus(*t));
                }
                // The closed form classified the pose; a spiric it does
                // not mint is its own reader's to refuse.
                Ok(_) | Err(SectionError::Spiric(_)) => true,
                // No plane × torus reading raises these; the closed
                // form's reader meets whatever it does raise.
                Err(
                    SectionError::WrongLane { .. }
                    | SectionError::RadiusEscalated { .. }
                    | SectionError::NoRegion { .. }
                    | SectionError::DegenerateOperand { .. }
                    | SectionError::ConeAtTheBand { .. }
                    | SectionError::BeyondOperandExtent { .. }
                    | SectionError::CoincidentSurfaces
                    | SectionError::Carrier(_)
                    | SectionError::Pinched { .. }
                    | SectionError::Unordered { .. }
                    | SectionError::NoMarch { .. }
                    | SectionError::Unaccounted { .. },
                ) => true,
            };
            let (stance, gap) = if closed {
                (PtStance::Closed, TorusGap::Transverse)
            } else {
                tilted(ring, site, band)?
            };
            Ok(Self {
                plane: plane.clone(),
                torus: torus.clone(),
                plane_first,
                ring,
                stance,
                gap,
                closed: closed.then_some(read),
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

    /// **The closed form's section of a closed pose**, as the pose read
    /// it at its lever: every reader of the closed form takes this, so
    /// no pose reads closed to one reader and tilted to another. A tilted
    /// pose routes to the general rung, the section value's.
    ///
    /// # Errors
    ///
    /// The closed form's own refusal of a pose it classified, or
    /// [`SectionError::RoutesToGeneralRung`] for a tilted pose.
    pub fn closed_form(&self) -> Result<&PlaneTorusSection<T>, SectionError> {
        match &self.closed {
            Some(Ok(section)) => Ok(section),
            Some(Err(e)) => Err(e.clone()),
            None => Err(SectionError::RoutesToGeneralRung {
                pair: "plane×torus",
                why: TILTED,
            }),
        }
    }

    /// **The section value's reading of the pose**: a closed pose is read
    /// elsewhere, by [`Self::closed_form`]; a tilted one is built.
    ///
    /// # Errors
    ///
    /// The tilted section's refusal.
    pub fn read(self) -> Result<super::SectionRead<T>, SectionError> {
        match self.stance {
            PtStance::Closed => {
                let (a, b) = self.operands();
                Ok(super::SectionRead::Elsewhere(crate::route(
                    a.kind(),
                    b.kind(),
                )))
            }
            PtStance::Tilted(_) => plane_torus(self).map(super::SectionRead::Built),
        }
    }
}

/// Why a tilted pose is not the closed form's.
const TILTED: &str = "a plane tilted against the torus's axis cuts a spiric quartic, which the \
                      section value marches (crate::PlaneTorusPose)";

/// **The closed form's question, asked where the section value asks
/// it**: the axis rows levered from the torus's centre over its ball
/// (module docs, the readings).
fn asked<T: Decide>(
    plane: &impl crate::intersect::SectionPlane<T>,
    torus: &Surface<T>,
    ring: Ring<T>,
    band: Band,
) -> Result<PlaneTorusSection<T>, SectionError> {
    crate::intersect::plane_torus_section(plane, torus, lever(ring.center(), ring.ball()), band)
}

/// **The closed form's section of a plane against a torus, read as the
/// section value reads it** ([`PlaneTorusPose::closed_form`]), for a
/// reader holding a plane's site rather than its surface: the same rows
/// at the same lever, so it answers as the pose does.
///
/// # Errors
///
/// As [`crate::plane_torus_section`], and the torus's ring guard as the
/// pose refuses it.
pub fn plane_torus_closed_form<T: Decide>(
    plane: &impl crate::intersect::SectionPlane<T>,
    torus: &Surface<T>,
    band: Band,
) -> Result<PlaneTorusSection<T>, SectionError> {
    match Ring::of(torus, band) {
        Some(ring) => asked(plane, torus, ring?, band),
        None => Err(SectionError::WrongLane {
            expected: "plane×torus",
        }),
    }
}

/// **A plane against a ring torus in the meridian half-plane `Π`
/// through the plane's normal**, the one reading every plane × torus
/// reader of the tube rows takes (module docs, the critical gap): the
/// count's rows, the null loop's witness and the touch point. In `Π`,
/// with `x` along `ê`, the unit part of the normal `n` square to the
/// axis, and `y` along the axis, the tube circle `T_σ` about the core
/// point `k_σ` is centred `(σR, 0)` with radius `r`, and the plane is the
/// line `s·x + (n·a)·y = h`, whose unit normal is `(s, n·a)`; `d_σ = h −
/// σsR` is its offset from `T_σ`'s centre and `(σR + d_σ·s, d_σ·(n·a))`
/// its foot.
#[derive(Clone, Copy, Debug)]
pub struct PlaneTorusMeridian<T: Real> {
    center: Point3<T>,
    axis: geom_core::Vec3<T>,
    across: geom_core::Vec3<T>,
    radii: [T; 2],
    h: T,
    s: T,
    na: T,
}

/// **Where a plane meets a ring torus at a tube row's `Zero` or in its
/// band** ([`PlaneTorusMeridian::cut`]).
#[derive(Clone, Copy, Debug)]
pub enum PtPinch {
    /// A row decided `Zero`: a tangency on the tube circle about `k₊`
    /// where `near` is `Some(true)`, about `k₋` where `Some(false)`, and
    /// along the plane's pass between them where `None`.
    Zero {
        /// The row.
        row: &'static str,
        /// Its `Zero` margin.
        margin: MarginDiag,
        /// Which tube circle.
        near: Option<bool>,
    },
    /// A row in the band.
    Undecided(Indeterminate),
}

impl<T: Decide> PlaneTorusMeridian<T> {
    /// The torus about the unit `axis` through `center` with radii
    /// `[R, r]`, against the plane through `q` with normal `n` (any
    /// length).
    #[must_use]
    pub fn new(
        (center, axis, radii): (Point3<T>, geom_core::Vec3<T>, [T; 2]),
        (q, n): (Point3<T>, geom_core::Vec3<T>),
    ) -> Self {
        let n = n.normalize();
        let (perp, s) = square_part(n, axis);
        Self {
            center,
            axis,
            across: perp / s,
            radii,
            h: n.dot(q - center),
            s,
            na: n.dot(axis),
        }
    }

    /// **The count, or where it pinches** (module docs, the section):
    /// `pt_near_tube`, `pt_far_tube`, then `pt_between_tubes` where the
    /// plane cuts neither tube circle.
    ///
    /// # Errors
    ///
    /// [`PtPinch`]: a row decided `Zero`, or one in the band.
    pub fn cut(&self, band: Band) -> Result<PtCut, PtPinch> {
        let ([big_r, r], h, s) = (self.radii, self.h, self.s);
        let read = |row: &'static str, datum: T, near: Option<bool>| match decide_reported(
            row,
            Margin::of(datum),
            band,
        ) {
            Ok(d) if d.sign == Sign::Zero => Err(PtPinch::Zero {
                row,
                margin: d.margin,
                near,
            }),
            Ok(d) => Ok(d.sign == Sign::Positive),
            Err(diag) => Err(PtPinch::Undecided(diag)),
        };
        let near = read(NEAR, r - (h - s * big_r).abs(), Some(true))?;
        let far = read(FAR, r - (h + s * big_r).abs(), Some(false))?;
        Ok(match (near, far) {
            (true, true) => PtCut::AroundAxis,
            (true, false) => PtCut::Null { near: true },
            (false, true) => PtCut::Null { near: false },
            (false, false) => match read(BETWEEN, s * big_r - r - h.abs(), None)? {
                true => PtCut::AroundTube,
                false => PtCut::Empty,
            },
        })
    }

    fn sigma(near: bool) -> T {
        if near { T::one() } else { T::zero() - T::one() }
    }

    fn in_pi(&self, x: T, y: T) -> Option<Point3<T>> {
        let p = self.center + self.across * x + self.axis * y;
        [p.x, p.y, p.z]
            .into_iter()
            .all(geom_core::is_finite_length)
            .then_some(p)
    }

    /// **A point of the null loop** on `T_σ` (`σ = +1` where `near`): the
    /// point `√(r² − d_σ²)` from the foot along the plane's line in `Π`,
    /// on both surfaces. `None` where it is not finite.
    #[must_use]
    pub fn null_witness(&self, near: bool) -> Option<Point3<T>> {
        let ([big_r, r], sigma) = (self.radii, Self::sigma(near));
        let d = self.h - sigma * self.s * big_r;
        let half = ((r - d.abs()) * (r + d.abs())).sqrt();
        self.in_pi(
            sigma * big_r + d * self.s - self.na * half,
            d * self.na + self.s * half,
        )
    }

    /// **The touch on `T_σ`, where it is a point of the section**: the
    /// foot, where the height `n·(x − c)` has an extreme there. Off a
    /// plane normal to the axis that height has the forced critical
    /// points alone, `±(sR + r)`, its extremes, and `±(sR − r)`, its
    /// saddles; at the tangency `|d_σ| = r`, so the foot is an extreme
    /// where `σ·d_σ` is positive (`pt_touch_extreme`). At a saddle the
    /// section is a node, which no point stands for: `None`, as where the
    /// reading is in band or not finite. A plane normal to the axis is
    /// tangent along a parallel, which the caller reads first.
    #[must_use]
    pub fn touch(&self, near: bool, band: Band) -> Option<Point3<T>> {
        let ([big_r, _], sigma) = (self.radii, Self::sigma(near));
        let d = self.h - sigma * self.s * big_r;
        let extreme = matches!(
            geom_core::k_stats::decide("pt_touch_extreme", Margin::of(sigma * d), band),
            Ok(Sign::Positive)
        );
        if !extreme {
            return None;
        }
        self.in_pi(sigma * big_r + d * self.s, d * self.na)
    }
}

/// The tilted pose's gap and count (module docs), the plane through `q`
/// with normal `n`.
fn tilted<T: Decide>(
    ring: Ring<T>,
    site: (Point3<T>, geom_core::Vec3<T>),
    band: Band,
) -> Result<(PtStance, TorusGap<T>), TorusPoseRefusal> {
    let meridian = PlaneTorusMeridian::new((ring.center(), ring.axis(), ring.radii()), site);
    match meridian.cut(band) {
        Ok(cut) => Ok((PtStance::Tilted(Some(cut)), TorusGap::Transverse)),
        Err(PtPinch::Zero { row, margin, near }) => Ok((
            PtStance::Tilted(None),
            TorusGap::Tangent {
                row,
                margin,
                at: near.and_then(|near| meridian.touch(near, band)),
            },
        )),
        Err(PtPinch::Undecided(diag)) => Err(TorusPoseRefusal::Undecided {
            question: TorusQuestion::Gap,
            diag,
        }),
    }
}

/// The pose's meridian reading.
fn meridian_of<T: Decide>(pose: &PlaneTorusPose<T>) -> Option<PlaneTorusMeridian<T>> {
    let ring = pose.ring;
    crate::intersect::SectionPlane::site(&pose.plane)
        .map(|site| PlaneTorusMeridian::new((ring.center(), ring.axis(), ring.radii()), site))
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
            let witness = meridian_of(&pose).and_then(|m| m.null_witness(near));
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
