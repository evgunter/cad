//! **The cone × torus pose** ([`super`]'s module docs): a double cone
//! against a ring torus, decided once — the operands guarded, the cone's
//! axis read against the torus's, the apex read against the torus, and
//! the critical gap decided ([`super::gap`]) — and the section built over
//! the decided pose.
//!
//! # The readings, in order
//!
//! 1. The torus's ring convention, then the cone's `0 < α < π/2`
//!    (`kt_aperture_sin`, `kt_aperture_cos`, levered from the apex over
//!    the torus's ball).
//! 2. The axis pose ([`super::torus::axis_pose`]) over the torus's own
//!    ball, as every torus pair reads it. A coaxial cone is the closed
//!    form's ([`crate::cone_torus_section`]), and the section value
//!    reads it elsewhere; a parallel axis and every tilted one are the
//!    general pose, whose components the march lists.
//! 3. **The apex on the torus** (C5's node clause): the apex's distance
//!    from the core circle against `r` (`kt_apex_on_torus`, across its
//!    rounding). Where that is `Zero` the cone is singular on the torus,
//!    and the section there is read from the cone's own quadratic form on
//!    the torus's tangent plane `T` at the apex. With `n` the torus's
//!    normal there and `d` the cone's axis, that form is
//!    `cos²α·|v|² − (v·d)²` on `T`, whose determinant has the sign of
//!    `(d·n)² − sin²α`: `|d·n| − sin α` (`kt_apex_form`, levered from the
//!    apex) positive is a definite form, the plane meeting the cone at
//!    the apex alone, a touch; negative is an indefinite one, the plane
//!    holding two rulings, a node; `Zero` is a plane holding one ruling,
//!    contact of higher order, which refuses. No apex contact is a
//!    tangency.
//! 4. Past a clear apex, the critical gap: the sweep of the core circle
//!    against the double cone as its own spine (`kt_critical_gap`).
//!
//! # The section
//!
//! A tangent pose lists one tangent component. A transverse one has no
//! closed-form count of its components: they are the march's
//! ([`Components::Marched`]), the general rung's, marched over the
//! torus's ball, which holds the whole section. An apex on the torus is
//! classified and not built: the ℝ³ march ends no branch at a node and
//! accounts no isolated point.

use geom::Surface;
use geom_core::k_stats::{decide, decide_across_reported};
use geom_core::{
    Band, Decide, Indeterminate, Margin, MarginDiag, Point3, Real, Sign, Vec2, Vec3,
    rounding_charge,
};

use super::gap::Spine;
use super::torus::{AxisPose, Ring, TorusGap, TorusPoseRefusal, Turn, axis_pose, lever};
use super::{Components, PairPose, PairSection, ZeroVerdict, square_part};
use crate::ExtentBall;
use crate::intersect::SectionError;
use crate::torus_convention::TorusRefusal;

/// An undecided gap beside an apex within this fraction of the tube
/// radius of the torus is read as the apex's question, not the gap's.
const NEAR_APEX_FRACTION: f64 = 64.0;

/// Why an apex whose tangent plane holds a ruling is not built.
pub(crate) const APEX_RULING: &str = "the cone's apex lies on the torus and the torus's tangent plane there holds a ruling: \
     contact of higher order at the apex";

/// **Why a decided pose is not marched**, where it is not: the one
/// statement the section value and the march door both read.
#[derive(Clone, Copy, Debug)]
pub enum Unmarched {
    /// Coaxial: the closed form's pose, the march the wrong lane.
    Coaxial(&'static str),
    /// The apex on the torus, a node or a touch.
    Apex(&'static str),
}

/// The apex-on-torus row, read across its rounding.
const APEX_ON_TORUS: [&str; 2] = ["kt_apex_on_torus", "kt_apex_on_torus_floor"];
/// The apex form's row.
const APEX_FORM: &str = "kt_apex_form";

/// **Which question of a cone × torus pose was left undecided.**
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KtQuestion {
    /// The cone's half-angle against its convention.
    Aperture,
    /// The axis pose: a coaxiality, a tilt or an offset.
    Axis,
    /// The apex on the torus, or its form there.
    Apex,
    /// The critical gap.
    Gap,
}

/// **Why a cone × torus pose is not decided.**
#[derive(Clone, Debug, PartialEq)]
pub enum ConeTorusRefusal {
    /// A torus not shown inside its ring convention.
    Torus(TorusRefusal),
    /// The cone's half-angle decided at (`at_band`) or past an end of its
    /// convention `0 < α < π/2`.
    Aperture {
        /// Decided `Zero`, rather than past the end.
        at_band: bool,
        /// Which end, in words.
        what: &'static str,
    },
    /// The apex lies on the torus and the torus's tangent plane there
    /// holds one of the cone's rulings (`kt_apex_form` decided `Zero`):
    /// contact of higher order.
    ApexRuling {
        /// The form's `Zero` margin.
        margin: MarginDiag,
    },
    /// A question in the band or poisoned.
    Undecided {
        /// Which question.
        question: KtQuestion,
        /// Its diagnostics.
        diag: Indeterminate,
    },
}

impl From<ConeTorusRefusal> for SectionError {
    fn from(refusal: ConeTorusRefusal) -> Self {
        match refusal {
            ConeTorusRefusal::Torus(t) => Self::DegenerateTorus(t),
            ConeTorusRefusal::Aperture {
                at_band: true,
                what,
            } => Self::ConeAtTheBand { what },
            ConeTorusRefusal::Aperture {
                at_band: false,
                what,
            } => Self::DegenerateOperand { what },
            ConeTorusRefusal::ApexRuling { .. } => Self::RoutesToGeneralRung {
                pair: "cone×torus",
                why: APEX_RULING,
            },
            ConeTorusRefusal::Undecided { diag, .. } => Self::Escalated(diag),
        }
    }
}

/// **How the cone's apex meets the torus**, where it lies on it.
#[derive(Clone, Copy, Debug)]
pub enum ApexContact {
    /// The form indefinite: two rulings in the tangent plane, a node.
    Node,
    /// The form definite: the apex an isolated point of the section.
    Touch,
}

/// **How the cone stands to the torus**, decided.
#[derive(Clone, Copy, Debug)]
pub enum KtStance<T: Real> {
    /// The cone's axis on the torus's ([`AxisPose::Coaxial`]): the
    /// closed form's pose ([`crate::cone_torus_section`]).
    Coaxial {
        /// The coaxial reading's slack.
        slack: T,
    },
    /// Off the axis, the apex on the torus.
    Apex(ApexContact),
    /// Off the axis, the apex clear of the torus, and the critical gap.
    Apart(TorusGap<T>),
}

/// **The cone × torus pose, decided once** (module docs).
#[derive(Clone, Debug)]
pub struct ConeTorusPose<T: Real> {
    cone: Surface<T>,
    torus: Surface<T>,
    cone_first: bool,
    ring: Ring<T>,
    stance: KtStance<T>,
}

impl<T: Decide> ConeTorusPose<T> {
    /// **Decides the pose of `a` and `b`**, a cone and a torus in either
    /// order (module docs, the readings in order).
    ///
    /// # Errors
    ///
    /// [`ConeTorusRefusal`]; a pair that is not a cone and a torus is
    /// `None`.
    pub fn decide(
        a: &Surface<T>,
        b: &Surface<T>,
        band: Band,
    ) -> Option<Result<Self, ConeTorusRefusal>> {
        let (cone, torus, cone_first, half_angle) = match (a, b) {
            (&Surface::Cone { half_angle, .. }, Surface::Torus { .. }) => (a, b, true, half_angle),
            (Surface::Torus { .. }, &Surface::Cone { half_angle, .. }) => (b, a, false, half_angle),
            _ => return None,
        };
        let cone_axis = cone.axis_line()?;
        let ring = Ring::of(torus, band)?;
        Some((|| {
            let ring = ring.map_err(|refusal| match refusal {
                TorusPoseRefusal::Torus(t) => ConeTorusRefusal::Torus(t),
                other => unreachable!("a ring refuses by its convention alone, not {other:?}"),
            })?;
            let apex = cone_axis.origin;
            let ball = ring.ball();
            let (sin, cos) = half_angle.sin_cos();
            aperture(apex, (sin, cos), ball, band)?;
            let stance = match axis_pose(
                ring.frame().axis_line(),
                cone_axis,
                // The coaxial arm reads the torus centre's station along
                // the cone's axis from the apex: the turn onto the torus
                // axis moves it by that station times `1 − cos` (at most
                // `sin²`), and by the offset's part along the tilted axis.
                // The pivot is the foot of the torus's ball's centre, so
                // both are second order beside the swing.
                |t: &Turn<T>| {
                    (ring.center() - t.at).dot(ring.axis()).abs() * t.tilt.powi(2)
                        + t.offset * t.tilt
                },
                ball,
                band,
            ) {
                AxisPose::Coaxial { slack, .. } => KtStance::Coaxial { slack },
                AxisPose::Undecided(diag) => return Err(undecided(KtQuestion::Axis, diag)),
                AxisPose::Parallel { .. } | AxisPose::Other => {
                    let (contact, apex_off) =
                        apex_on_torus(ring, apex, cone_axis.dir.get(), sin, band)?;
                    match contact {
                        Some(contact) => KtStance::Apex(contact),
                        None => KtStance::Apart(
                            super::torus::swept(
                                ring,
                                Spine::Cone {
                                    axis: cone_axis,
                                    cos,
                                    sin,
                                },
                                [ring.radii()[1], T::zero()],
                                &crate::rows::KT_GAP,
                                band,
                            )
                            .map_err(|refusal| match refusal {
                                // A gap undecided beside an apex barely off
                                // the torus is the near-node's: its two
                                // branches are what the sweep cannot part.
                                TorusPoseRefusal::Undecided { diag, .. } => {
                                    let near = matches!(
                                        decide(
                                            "kt_apex_near",
                                            Margin::of(
                                                ring.radii()[1] / T::from_f64(NEAR_APEX_FRACTION)
                                                    - apex_off,
                                            ),
                                            band,
                                        ),
                                        Ok(Sign::Positive)
                                    );
                                    let question = if near {
                                        KtQuestion::Apex
                                    } else {
                                        KtQuestion::Gap
                                    };
                                    undecided(question, diag)
                                }
                                other => {
                                    unreachable!("the gap's sweep refuses undecided, not {other:?}")
                                }
                            })?,
                        ),
                    }
                }
            };
            Ok(Self {
                cone: cone.clone(),
                torus: torus.clone(),
                cone_first,
                ring,
                stance,
            })
        })())
    }

    /// The two surfaces in the order the pose was asked of.
    pub fn operands(&self) -> (&Surface<T>, &Surface<T>) {
        if self.cone_first {
            (&self.cone, &self.torus)
        } else {
            (&self.torus, &self.cone)
        }
    }

    /// The cone, as given.
    pub fn cone(&self) -> &Surface<T> {
        &self.cone
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

    /// How the cone stands.
    pub fn stance(&self) -> KtStance<T> {
        self.stance
    }

    /// **The parallel a coaxial nappe touches the tube along**, the near
    /// nappe's (`far` false: the one the cone's axis points into) or the
    /// far one's: in the meridian half-plane about the apex, the foot on
    /// the nappe's generator of the tube centre `(R, z)`, swept about the
    /// axis. Read where that nappe's tube row decided `Zero`: the tube
    /// circle then touches the generator there, and lies in `ρ > 0`, so
    /// the foot does too. `None` off the coaxial pose.
    #[must_use]
    pub fn tangent_parallel(&self, far: bool) -> Option<geom::Curve3<T>> {
        let KtStance::Coaxial { .. } = self.stance else {
            return None;
        };
        let &Surface::Cone {
            frame, half_angle, ..
        } = &self.cone
        else {
            return None;
        };
        let ring = self.ring;
        let ([big_r, _], center, a) = (ring.radii(), ring.center(), ring.axis());
        let (apex, k) = (frame.origin(), frame.axis().normalize());
        let (s, co) = half_angle.sin_cos();
        let z = (center - apex).dot(k);
        let nappe = if far { -T::one() } else { T::one() };
        // The generator's direction is (sin α, ±cos α) in (ρ, along k).
        let along = big_r * s + nappe * z * co;
        let (rho, y) = (along * s, along * nappe * co);
        Some(ring.parallel(rho, (apex - center).dot(a) + y * k.dot(a)))
    }

    /// **Why the pose is not marched**, if it is not (module docs, the
    /// section): coaxial, or the apex on the torus.
    pub fn unmarched(&self) -> Option<Unmarched> {
        match self.stance {
            KtStance::Coaxial { .. } => Some(Unmarched::Coaxial(
                "a cone off the torus's axis: the coaxial pose is the closed form's \
                 (cone_torus_section)",
            )),
            KtStance::Apex(ApexContact::Node) => Some(Unmarched::Apex(
                "the cone's apex lies on the torus with its form there indefinite, a node of the \
                 section, and the ℝ³ march ends no branch at a node",
            )),
            KtStance::Apex(ApexContact::Touch) => Some(Unmarched::Apex(
                "the cone's apex touches the torus with its form there definite, an isolated \
                 point of the section, which the ℝ³ march does not account",
            )),
            KtStance::Apart(_) => None,
        }
    }

    /// **The tangency the pose decided**, if any: the critical gap's
    /// `Zero`, classification data at every reader.
    pub fn tangency(&self) -> Option<ZeroVerdict> {
        match self.stance {
            KtStance::Apart(gap) => gap.tangency(),
            KtStance::Coaxial { .. } | KtStance::Apex(_) => None,
        }
    }
}

fn undecided(question: KtQuestion, diag: Indeterminate) -> ConeTorusRefusal {
    ConeTorusRefusal::Undecided { question, diag }
}

/// The cone's convention `0 < α < π/2`, its sine and cosine each levered
/// from the apex over `ball`.
fn aperture<T: Decide>(
    apex: Point3<T>,
    (sin, cos): (T, T),
    ball: ExtentBall<T>,
    band: Band,
) -> Result<(), ConeTorusRefusal> {
    for (name, margin, at_edge, outside) in [
        (
            "kt_aperture_sin",
            sin,
            "the cone's half-angle is zero within the tolerance",
            "the cone's half-angle is negative, outside its convention 0 < α < π/2",
        ),
        (
            "kt_aperture_cos",
            cos,
            "the cone's half-angle is a right angle within the tolerance",
            "the cone's half-angle is past a right angle, outside its convention 0 < α < π/2",
        ),
    ] {
        match decide(name, Margin::levered(margin, lever(apex, ball)), band)
            .map_err(|diag| undecided(KtQuestion::Aperture, diag))?
        {
            Sign::Positive => {}
            Sign::Zero => {
                return Err(ConeTorusRefusal::Aperture {
                    at_band: true,
                    what: at_edge,
                });
            }
            Sign::Negative => {
                return Err(ConeTorusRefusal::Aperture {
                    at_band: false,
                    what: outside,
                });
            }
        }
    }
    Ok(())
}

/// **The apex against the torus** (module docs, reading 3): `None` where
/// it is clear of it, with its distance from the torus.
fn apex_on_torus<T: Decide>(
    ring: Ring<T>,
    apex: Point3<T>,
    axis: Vec3<T>,
    sin: T,
    band: Band,
) -> Result<(Option<ApexContact>, T), ConeTorusRefusal> {
    let [big_r, r] = ring.radii();
    let q = apex - ring.center();
    let (perp, s) = square_part(q, ring.axis());
    let h = q.dot(ring.axis());
    let off = Vec2::new(s - big_r, h).norm() - r;
    let (sign, _) = decide_across_reported(
        APEX_ON_TORUS,
        off,
        rounding_charge(ring.size() + q.norm()),
        band,
    )
    .map_err(|diag| undecided(KtQuestion::Apex, diag))?;
    if sign != Sign::Zero {
        return Ok((None, off.abs()));
    }
    // On the torus, the apex stands `r` from the core point in its
    // meridian half-plane, off the axis since `R > r`.
    let foot = ring.center() + perp * (big_r / s);
    let n = (apex - foot).normalize();
    let form = axis.dot(n).abs() - sin;
    match geom_core::k_stats::decide_reported(
        APEX_FORM,
        Margin::levered(form, lever(apex, ring.ball())),
        band,
    )
    .map_err(|diag| undecided(KtQuestion::Apex, diag))?
    {
        geom_core::Decided {
            sign: Sign::Positive,
            ..
        } => Ok((Some(ApexContact::Touch), T::zero())),
        geom_core::Decided {
            sign: Sign::Negative,
            ..
        } => Ok((Some(ApexContact::Node), T::zero())),
        geom_core::Decided {
            sign: Sign::Zero,
            margin,
        } => Err(ConeTorusRefusal::ApexRuling { margin }),
    }
}

/// The cone × torus section over its decided pose, off the axis.
///
/// # Errors
///
/// [`SectionError::RoutesToGeneralRung`] where the apex lies on the
/// torus: a node or a touch, which no march here ends a branch at or
/// accounts.
pub fn cone_torus<T: Decide>(pose: ConeTorusPose<T>) -> Result<PairSection<T>, SectionError> {
    match pose.unmarched() {
        Some(Unmarched::Coaxial(expected)) => return Err(SectionError::WrongLane { expected }),
        Some(Unmarched::Apex(why)) => {
            return Err(SectionError::RoutesToGeneralRung {
                pair: "cone×torus",
                why,
            });
        }
        None => {}
    }
    let components = match pose.stance {
        KtStance::Coaxial { .. } | KtStance::Apex(_) => {
            unreachable!("`unmarched` answers every pose but an apart one")
        }
        KtStance::Apart(TorusGap::Tangent { .. }) => {
            Components::Counted(vec![super::torus::tangent_component(None, [false, false])])
        }
        KtStance::Apart(TorusGap::Transverse) => Components::Marched,
    };
    Ok(PairSection {
        lever: crate::uniform_curvature_arm(&pose.torus),
        decided: pose.tangency().into_iter().collect(),
        pose: PairPose::ConeTorus(pose),
        components,
        nodes: Vec::new(),
        node_sets: Vec::new(),
    })
}
