//! **A cone against an oblique cylinder or another cone on the section
//! value** (C5): the pose decided once on closed-form margins over the
//! operation's region, and the section's components the ruling sweep
//! along the cone's rulings counts and cuts by that region.
//!
//! The pose reads, in order: each operand's convention (a cone's
//! half-angle strictly inside a right angle, a cylinder's radius
//! positive); for two cones, whether their apexes are one point (their
//! section is their shared rulings, which the value does not build: read
//! where it was read before) and whether their axes are parallel with one
//! aperture (the plane conic, likewise); for a cylinder, whether a
//! generator runs along its axis (refused, [`SectionError::Pinched`]:
//! the clearance has no divisor there); then the **apex margin**, the
//! partner's residual at the ruled cone's apex in metres.
//!
//! - Off the partner, the ruling sweep reads both sheets ([`ruling::sweep`]),
//!   a double root of its discriminant decided a tangency on the rulings'
//!   clearance ceiling.
//! - On it (`Zero`, a coincidence of the apex with the partner), the
//!   cone's own quadratic form read on the partner's tangent plane there
//!   decides the apex: indefinite, a **node** whose lines are the two
//!   rulings in that plane; definite, a **touch**, an isolated point of
//!   the section; degenerate, refused. A `Zero` here is never read as a
//!   tangency. The sweep divides the apex root out ([`ruling::sweep_divided`]).
//!
//! Of two cones, the one ruled is the one whose apex lies on the other,
//! if either does: the other apex must stand off the ruled cone, or its
//! ruling meets the partner doubly there. Both apexes on the other cone
//! refuse.
//!
//! Every component is the general rung's ([`ClassCarrier::General`]),
//! save the points a tangency or a touch lists. The march needs the
//! cone's ℝ³ enclosures, which are not built, so carrying refuses typed
//! ([`super::NoCarrier::ConeUnenclosed`]).

use geom::Surface;
use geom_core::k_stats::decide_reported;
use geom_core::{Band, Decide, Decided, Margin, Point3, Real, Sign, Vec3};

use super::cylinders::{RegionPlacement, placement};
use super::ruling::{self, End, Form, RuledCone};
use super::{
    ClassCarrier, Coincident, ComponentEnds, ComponentShape, Components, NoCarrier, NodeEnd,
    NodeEnds, NodeLine, PairPose, PairSection, PairSide, RaySense, SectionComponent,
    SectionDecision, SectionNode, ZeroVerdict, node_joined,
};
use crate::ExtentBall;
use crate::intersect::{SectionError, SectionRadius};

/// **The questions a cone pair's pose decides.**
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConeDecision {
    /// `kc_cone_aperture_sin`: a cone's `sin α` levered across the
    /// region, definitely positive.
    ApertureSin,
    /// `kc_cone_aperture_cos`: its `cos α`, likewise.
    ApertureCos,
    /// `kc_cylinder_radius`: the cylinder's radius definitely positive.
    Radius,
    /// `kc_generator_along_axis`: `|d·â| − cos α` levered across the
    /// region, a generator along the cylinder's axis at `Zero`.
    GeneratorAlongAxis,
    /// `kc_apexes_apart`: two cones' apexes' distance.
    ApexesApart,
    /// `kc_axes_parallel`: two cones' `‖â₁ × â₂‖` levered across the
    /// region.
    AxesParallel,
    /// `kc_aperture_equality`: parallel axes, `cos²α₁ − cos²α₂` levered.
    ApertureEquality,
    /// `kc_apex_on_partner`: the partner's residual at the ruled cone's
    /// apex, in metres.
    ApexOnPartner,
    /// `kc_partner_apex_on_cone`: the ruled cone's residual at the
    /// partner cone's apex.
    PartnerApexOnCone,
    /// `kc_apex_form`: the determinant of the cone's form on the
    /// partner's tangent plane at the apex, levered.
    ApexForm,
}

impl ConeDecision {
    /// The decision's K-funnel name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::ApertureSin => "kc_cone_aperture_sin",
            Self::ApertureCos => "kc_cone_aperture_cos",
            Self::Radius => "kc_cylinder_radius",
            Self::GeneratorAlongAxis => "kc_generator_along_axis",
            Self::ApexesApart => "kc_apexes_apart",
            Self::AxesParallel => "kc_axes_parallel",
            Self::ApertureEquality => "kc_aperture_equality",
            Self::ApexOnPartner => "kc_apex_on_partner",
            Self::PartnerApexOnCone => "kc_partner_apex_on_cone",
            Self::ApexForm => "kc_apex_form",
        }
    }
}

/// **How the ruled cone's apex stands to the partner**, decided.
#[derive(Clone, Copy, Debug)]
pub enum ConeStance<T: Real> {
    /// Off it: the rulings' two sheets.
    Sheets,
    /// On it, the cone's form indefinite on the partner's tangent plane
    /// there: a node at the apex.
    Node {
        /// The partner's unit normal at the apex, outward.
        normal: Vec3<T>,
        /// The two rulings in the tangent plane, unit, each along its
        /// generator's opening direction.
        rulings: [Vec3<T>; 2],
    },
    /// On it, the form definite there: the apex is an isolated point of
    /// the section.
    Touch {
        /// The partner's unit normal at the apex, outward.
        normal: Vec3<T>,
    },
}

/// **A cone pair's pose, decided once** over the region it was asked
/// across ([`Self::decide`]).
#[derive(Clone, Debug)]
pub struct ConeRuledPose<T: Real> {
    first: Surface<T>,
    second: Surface<T>,
    /// The ruled cone is the first operand.
    ruled_first: bool,
    ruled: RuledCone<T>,
    region: ExtentBall<T>,
    apex: Decided,
    stance: ConeStance<T>,
}

/// **What a cone pair's pose answers**: the pose, or a pose whose
/// section the value does not build.
#[derive(Clone, Debug)]
pub enum ConeRead<T: Real> {
    /// The pose.
    Pose(ConeRuledPose<T>),
    /// Two cones through one apex (their shared rulings), or on parallel
    /// axes with one aperture (a plane conic): read where they were read
    /// before.
    Elsewhere,
}

/// A cone's carrier read once: apex, unit axis, `(sin α, cos α)`.
#[derive(Clone, Copy)]
struct Carrier<T: Real> {
    apex: Point3<T>,
    axis: Vec3<T>,
    sc: (T, T),
}

/// `Ok` where the margin decides, the predicate named.
fn read<T: Decide>(
    decision: ConeDecision,
    margin: Margin<T>,
    band: Band,
) -> Result<Decided, SectionError> {
    decide_reported(decision.name(), margin, band).map_err(SectionError::Escalated)
}

/// **A cone's convention**, its half-angle strictly inside a right angle,
/// each clause levered across the region from its apex.
fn carrier<T: Decide>(
    s: &Surface<T>,
    region: ExtentBall<T>,
    band: Band,
) -> Result<Option<Carrier<T>>, SectionError> {
    let &Surface::Cone { frame, half_angle } = s else {
        return Ok(None);
    };
    let apex = frame.origin();
    let lever = region.lever_from(apex);
    let sc = half_angle.sin_cos();
    for (decision, value, at_edge, outside) in [
        (
            ConeDecision::ApertureSin,
            sc.0,
            "the cone's half-angle is zero within the tolerance",
            "the cone's half-angle is negative, outside its convention 0 < α < π/2",
        ),
        (
            ConeDecision::ApertureCos,
            sc.1,
            "the cone's half-angle is a right angle within the tolerance",
            "the cone's half-angle is past a right angle, outside its convention 0 < α < π/2",
        ),
    ] {
        match read(decision, Margin::levered(value, lever), band)?.sign {
            Sign::Positive => {}
            Sign::Zero => return Err(SectionError::ConeAtTheBand { what: at_edge }),
            Sign::Negative => return Err(SectionError::DegenerateOperand { what: outside }),
        }
    }
    Ok(Some(Carrier {
        apex,
        axis: frame.axis(),
        sc,
    }))
}

/// The cone's rulings against `partner`, its seam a selection square to
/// the axis.
fn ruled<T: Real>(cone: Carrier<T>, partner: Form<T>, region: ExtentBall<T>) -> RuledCone<T> {
    let (n, m) = cone.axis.orthonormal_basis();
    RuledCone {
        apex: cone.apex,
        axis: cone.axis,
        n,
        m,
        sc: cone.sc,
        partner,
        lever: region.lever_from(cone.apex),
    }
}

/// **The partner's unit normal at the ruled cone's apex, outward**: from
/// the cylinder's axis, or away from the partner cone's solid (the
/// gradient of `−q`, `q` the partner's form).
fn normal_at<T: Real>(ruled: &RuledCone<T>) -> Vec3<T> {
    let Form { at, d, .. } = ruled.partner;
    let w = ruled.apex - at;
    match ruled.partner_cos() {
        None => (w - d * w.dot(d)).normalize(),
        Some(cos) => (w * cos.powi(2) - d * d.dot(w)).normalize(),
    }
}

impl<T: Decide> ConeRuledPose<T> {
    /// **Decides the pose of `a` and `b`** across `region` (module docs).
    /// `None` for a pair that is not a cone against a cylinder or a cone.
    ///
    /// # Errors
    ///
    /// [`SectionError::ConeAtTheBand`] or [`SectionError::DegenerateOperand`]
    /// for an operand outside its convention, and
    /// [`SectionError::RadiusEscalated`] for a radius in band;
    /// [`SectionError::Pinched`] for a generator along the cylinder's
    /// axis, both cones' apexes on the other, or a degenerate form at the
    /// apex; [`SectionError::Escalated`] for a question in the band.
    pub fn decide(
        a: &Surface<T>,
        b: &Surface<T>,
        region: ExtentBall<T>,
        band: Band,
    ) -> Option<Result<ConeRead<T>, SectionError>> {
        match (a, b) {
            (Surface::Cone { .. }, Surface::Cylinder { .. } | Surface::Cone { .. })
            | (Surface::Cylinder { .. }, Surface::Cone { .. }) => {}
            _ => return None,
        }
        Some(Self::decide_read(a, b, region, band))
    }

    fn decide_read(
        a: &Surface<T>,
        b: &Surface<T>,
        region: ExtentBall<T>,
        band: Band,
    ) -> Result<ConeRead<T>, SectionError> {
        let (one, two) = (carrier(a, region, band)?, carrier(b, region, band)?);
        let (ruled_first, ruled) = match (one, two) {
            (Some(cone), None) => (true, Self::against_wall(cone, b, region, band)?),
            (None, Some(cone)) => (false, Self::against_wall(cone, a, region, band)?),
            (Some(one), Some(two)) => match Self::against_cone(one, two, region, band)? {
                Some(picked) => picked,
                None => return Ok(ConeRead::Elsewhere),
            },
            (None, None) => unreachable!("`decide` admits only pairs with a cone"),
        };
        let apex = read(
            ConeDecision::ApexOnPartner,
            Margin::of(ruled.partner_residual(ruled.apex)),
            band,
        )?;
        let stance = match apex.sign {
            Sign::Positive | Sign::Negative => ConeStance::Sheets,
            Sign::Zero => Self::apex_stance(&ruled, region, band)?,
        };
        Ok(ConeRead::Pose(Self {
            first: a.clone(),
            second: b.clone(),
            ruled_first,
            ruled,
            region,
            apex,
            stance,
        }))
    }

    /// The cone's rulings against the cylinder `wall`.
    fn against_wall(
        cone: Carrier<T>,
        wall: &Surface<T>,
        region: ExtentBall<T>,
        band: Band,
    ) -> Result<RuledCone<T>, SectionError> {
        let &Surface::Cylinder { frame, radius } = wall else {
            unreachable!("`decide` pairs a cone with a cylinder or a cone")
        };
        match decide_reported(ConeDecision::Radius.name(), Margin::of(radius), band) {
            Ok(Decided {
                sign: Sign::Positive,
                ..
            }) => {}
            Ok(_) => {
                return Err(SectionError::DegenerateOperand {
                    what: "a cylinder's radius is not definitely positive",
                });
            }
            Err(diag) => {
                return Err(SectionError::RadiusEscalated {
                    radius: SectionRadius::Cylinder,
                    diag,
                });
            }
        }
        let d = frame.axis();
        let along = d.dot(cone.axis).abs() - cone.sc.1;
        let lever = region.twice_lever_from(cone.apex);
        if read(
            ConeDecision::GeneratorAlongAxis,
            Margin::levered(along, lever),
            band,
        )?
        .sign
            == Sign::Zero
        {
            return Err(SectionError::Pinched {
                predicate: ConeDecision::GeneratorAlongAxis.name(),
            });
        }
        let axis = geom::Line {
            origin: frame.origin(),
            dir: d,
        };
        Ok(ruled(cone, Form::cylinder(axis, radius), region))
    }

    /// **Two cones' ruled side**: `None` where the value does not build
    /// the pose ([`ConeRead::Elsewhere`]); else which is ruled, the first
    /// unless only the second's apex lies on the other.
    fn against_cone(
        one: Carrier<T>,
        two: Carrier<T>,
        region: ExtentBall<T>,
        band: Band,
    ) -> Result<Option<(bool, RuledCone<T>)>, SectionError> {
        let apart = (one.apex - two.apex).norm();
        if read(ConeDecision::ApexesApart, Margin::of(apart), band)?.sign == Sign::Zero {
            return Ok(None);
        }
        let lever = region
            .twice_lever_from(one.apex)
            .max(region.twice_lever_from(two.apex));
        let tilt = one.axis.cross(two.axis).norm();
        if read(
            ConeDecision::AxesParallel,
            Margin::levered(tilt, lever),
            band,
        )?
        .sign
            == Sign::Zero
        {
            let gap = one.sc.1.powi(2) - two.sc.1.powi(2);
            if read(
                ConeDecision::ApertureEquality,
                Margin::levered(gap, lever),
                band,
            )?
            .sign
                == Sign::Zero
            {
                return Ok(None);
            }
        }
        let form = |c: Carrier<T>| {
            Form::cone(
                geom::Line {
                    origin: c.apex,
                    dir: c.axis,
                },
                c.sc.1,
            )
        };
        let (on_two, on_one) = (ruled(one, form(two), region), ruled(two, form(one), region));
        let first_on = read(
            ConeDecision::ApexOnPartner,
            Margin::of(on_two.partner_residual(one.apex)),
            band,
        )?;
        let second_on = read(
            ConeDecision::PartnerApexOnCone,
            Margin::of(on_one.partner_residual(two.apex)),
            band,
        )?;
        Ok(Some(match (first_on.sign, second_on.sign) {
            (Sign::Zero, Sign::Zero) => {
                return Err(SectionError::Pinched {
                    predicate: ConeDecision::PartnerApexOnCone.name(),
                });
            }
            (_, Sign::Zero) => (false, on_one),
            _ => (true, on_two),
        }))
    }

    /// **The apex on the partner** (C5): the cone's form
    /// `Q(x) = (â·x)² − cos²α|x|²` on the partner's tangent plane at the
    /// apex, in an orthonormal pair `e₁, e₂` of it. Its determinant
    /// `Q₁₁Q₂₂ − Q₁₂²` negative is a node, its null lines the rulings in
    /// the plane; positive a touch (the form is never positive definite
    /// there: the plane meets `â⊥`); `Zero` is a contact of higher order,
    /// refused.
    fn apex_stance(
        ruled: &RuledCone<T>,
        region: ExtentBall<T>,
        band: Band,
    ) -> Result<ConeStance<T>, SectionError> {
        let normal = normal_at(ruled);
        let (e1, e2) = normal.orthonormal_basis();
        let (a, c) = (ruled.axis, ruled.sc.1);
        let form = |x: Vec3<T>, y: Vec3<T>| a.dot(x) * a.dot(y) - c.powi(2) * x.dot(y);
        let (q11, q12, q22) = (form(e1, e1), form(e1, e2), form(e2, e2));
        let det = q11 * q22 - q12.powi(2);
        let decided = read(
            ConeDecision::ApexForm,
            Margin::levered(det, region.lever_from(ruled.apex)),
            band,
        )?;
        match decided.sign {
            Sign::Positive => Ok(ConeStance::Touch { normal }),
            Sign::Zero => Err(SectionError::Pinched {
                predicate: ConeDecision::ApexForm.name(),
            }),
            Sign::Negative => {
                // `Q₁₁x² + 2Q₁₂xy + Q₂₂y² = 0`, solved for the ratio along
                // the larger diagonal entry's axis, a selection.
                let root = (T::zero() - det).sqrt();
                let first = (q11.abs() - q22.abs()).select_le_zero(T::zero(), T::one());
                let line = |sign: T| {
                    let along_first = (T::zero() - q12 + sign * root) / q11;
                    let along_second = (T::zero() - q12 + sign * root) / q22;
                    let v1 = e1 * along_first + e2;
                    let v2 = e1 + e2 * along_second;
                    let v = Vec3::new(
                        first.select_le_zero(v2.x, v1.x),
                        first.select_le_zero(v2.y, v1.y),
                        first.select_le_zero(v2.z, v1.z),
                    );
                    v.normalize().turned_toward(a)
                };
                Ok(ConeStance::Node {
                    normal,
                    rulings: [line(T::one()), line(T::zero() - T::one())],
                })
            }
        }
    }

    /// The two surfaces in the order the pose was asked of.
    pub fn operands(&self) -> (&Surface<T>, &Surface<T>) {
        (&self.first, &self.second)
    }

    /// **The region the pose was read across**: the ball the sweep cuts
    /// the section by and the march ends at.
    pub fn region(&self) -> ExtentBall<T> {
        self.region
    }

    /// How the ruled cone's apex stands to the partner.
    pub fn stance(&self) -> ConeStance<T> {
        self.stance
    }

    /// The apex margin's decision.
    pub fn apex(&self) -> Decided {
        self.apex
    }

    /// The side the ruled cone is, in the order the pose was asked of.
    pub fn ruled_side(&self) -> PairSide {
        if self.ruled_first {
            PairSide::First
        } else {
            PairSide::Second
        }
    }

    /// `[on the ruled cone, on the partner]` in the order asked of.
    fn order<X>(&self, [on_ruled, on_partner]: [X; 2]) -> [X; 2] {
        if self.ruled_first {
            [on_ruled, on_partner]
        } else {
            [on_partner, on_ruled]
        }
    }
}

impl<T: Real> RuledCone<T> {
    /// The partner cone's `cos α`, `None` for a cylinder.
    fn partner_cos(&self) -> Option<T> {
        self.partner.cone_cos()
    }

    /// **The partner's residual at `p`, in metres**: its distance from
    /// the cylinder's axis less the radius, or its signed distance from
    /// the partner's double cone ([`crate::cone_elevation`]).
    fn partner_residual(&self, p: Point3<T>) -> T {
        let Form { at, d, .. } = self.partner;
        match (self.partner.radius(), self.partner_cos()) {
            (Some(r), _) => (p - at).cross(d).norm() - r,
            (None, Some(cos)) => {
                let sin = (T::one() - cos.powi(2)).max(T::zero()).sqrt();
                let (h, rho) = ((p - at).dot(d), ((p - at) - d * (p - at).dot(d)).norm());
                crate::implicit::cone_offset(rho, h.abs(), sin, cos)
            }
            (None, None) => unreachable!("a cone's partner is a cylinder or a cone"),
        }
    }
}

/// **The section of a cone pair's pose** (module docs).
///
/// # Errors
///
/// [`SectionError::Escalated`] where the sweep escalates, and
/// [`SectionError::Unaccounted`] where its readings disagree or the
/// passes through the apex are not the stance's.
pub(super) fn cone_ruled<T: Decide>(
    pose: ConeRuledPose<T>,
    band: Band,
) -> Result<PairSection<T>, SectionError> {
    let region = pose.region;
    let apex = pose.ruled.apex;
    let mut decided = Vec::new();
    let mut nodes = Vec::new();
    let mut listed = Vec::new();
    let held =
        |p: Point3<T>| placement(ExtentBall::point(p), region, band) != RegionPlacement::Outside;
    let point = |at: Point3<T>, why: NoCarrier| {
        SectionComponent::closed_form(
            ComponentShape::Point,
            [false, false],
            Some(at),
            ClassCarrier::Unminted(why),
        )
    };
    match pose.stance {
        ConeStance::Sheets => {
            let swept = ruling::sweep(&pose.ruled, region, band)?;
            for c in swept.components {
                listed.push(SectionComponent {
                    cut_ends: c.cut_ends,
                    ..SectionComponent::closed_form(
                        match c.cut_ends {
                            Some(_) => ComponentShape::Open {
                                ends: ComponentEnds::cut(),
                            },
                            None => ComponentShape::Loop,
                        },
                        pose.order([c.encircles, false]),
                        Some(c.witness),
                        if c.tangent {
                            ClassCarrier::Unminted(NoCarrier::Tangent)
                        } else {
                            ClassCarrier::General
                        },
                    )
                });
            }
            for t in swept.touches {
                decided.push(ZeroVerdict {
                    decision: SectionDecision::Tangency(ruling::TANGENCY),
                    margin: t.margin,
                });
                if !t.crossing && held(t.at) {
                    listed.push(point(t.at, NoCarrier::Tangent));
                }
            }
        }
        ConeStance::Node { .. } | ConeStance::Touch { .. } => {
            decided.push(ZeroVerdict {
                decision: SectionDecision::Coincidence(Coincident::ConeApex),
                margin: pose.apex.margin,
            });
            let divided = ruling::sweep_divided(&pose.ruled, region, band)?;
            let lines = match pose.stance {
                ConeStance::Node { normal, rulings } => Some((normal, rulings)),
                ConeStance::Touch { .. } | ConeStance::Sheets => None,
            };
            let passes = if lines.is_some() && held(apex) { 2 } else { 0 };
            if divided.nodes.len() != passes {
                return Err(SectionError::Unaccounted {
                    cause: format!(
                        "the divided sweep read {} passes of the sheet through the apex where the \
                         apex's form and the region give {passes}",
                        divided.nodes.len()
                    ),
                });
            }
            let node = lines
                .filter(|_| passes == 2)
                .map(|(normal, rulings)| apex_node(&pose, normal, rulings, band));
            for arc in &divided.arcs {
                let Some([from, to]) = arc.ends else {
                    listed.push(SectionComponent::closed_form(
                        ComponentShape::Loop,
                        pose.order([true, false]),
                        Some(arc.witness),
                        ClassCarrier::General,
                    ));
                    continue;
                };
                let end = |e: End<T>| match (e, &node) {
                    (End::Node(phi), Some((node, rulings))) => {
                        Some(node_end(&pose, node, *rulings, phi, arc.witness, band))
                    }
                    _ => None,
                };
                let cuts = match (from, to) {
                    (End::Cut(p), End::Cut(q)) => Some([p, q]),
                    _ => None,
                };
                listed.push(SectionComponent {
                    cut_ends: cuts,
                    ..SectionComponent::closed_form(
                        ComponentShape::Open {
                            ends: ComponentEnds {
                                cut: matches!(from, End::Cut(_)) || matches!(to, End::Cut(_)),
                                nodes: NodeEnds {
                                    start: end(from).transpose()?,
                                    end: end(to).transpose()?,
                                },
                                ..ComponentEnds::default()
                            },
                        },
                        [false, false],
                        Some(arc.witness),
                        ClassCarrier::General,
                    )
                });
            }
            if let Some((node, _)) = node {
                nodes.push(node);
            }
            if matches!(pose.stance, ConeStance::Touch { .. }) && held(apex) {
                listed.push(point(apex, NoCarrier::ApexTouch));
            }
        }
    }
    let node_sets = node_joined(&listed, nodes.len(), [false, false]);
    let partner_arm = match pose.ruled.partner.radius() {
        Some(r) => geom_core::Lever::curvature_radius(r),
        None => region.lever_from(pose.ruled.partner.at),
    };
    let lever = pose.ruled.lever.min(partner_arm);
    let pose = match (&pose.first, &pose.second) {
        (Surface::Cone { .. }, Surface::Cone { .. }) => PairPose::ConeCone(pose),
        _ => PairPose::ConeCylinder(pose),
    };
    Ok(PairSection {
        lever,
        nodes,
        node_sets,
        decided,
        components: Components::Counted(listed),
        pose,
    })
}

/// **The node at the apex** (C5, [`SectionNode`]): the partner's normal,
/// the two rulings in its tangent plane as the lines, the second turned
/// counterclockwise from the first about the normal by less than a half
/// turn, and `above` the side the residuals' signs name in sector 0:
/// the cone where the partner runs inside the cone's solid there (the
/// cone's form positive on the sector's bisector), else the partner, as a
/// smooth surface's residual on its partner names its side.
fn apex_node<T: Decide>(
    pose: &ConeRuledPose<T>,
    normal: Vec3<T>,
    [l0, l1]: [Vec3<T>; 2],
    band: Band,
) -> (SectionNode<T>, [Vec3<T>; 2]) {
    let l1 = l1 * l0.cross(l1).dot(normal).unit_sign();
    let (a, c) = (pose.ruled.axis, pose.ruled.sc.1);
    let w = l0 + l1;
    let inside = a.dot(w).powi(2) - c.powi(2) * w.norm_squared();
    let cone = pose.ruled_side();
    let above = match inside.sign_within(band) {
        // NO-DECISION: a label for the sectors, which no reader decides
        // on.
        Ok(Decided {
            sign: Sign::Positive,
            ..
        }) => cone,
        _ => cone.other(),
    };
    (
        SectionNode {
            point: pose.ruled.apex,
            normal,
            lines: [l0, l1],
            above,
        },
        [l0, l1],
    )
}

/// **An arc's end at the apex node**, from the pass at the ruling angle
/// `phi`: the node line its ruling lies along (the nearer of the two),
/// and the sense the arc leaves along, the nappe its witness lies on
/// (`t`'s sign is the arc's between its ends: it changes only through the
/// apex or past every bound, where the region cuts it).
fn node_end<T: Decide>(
    pose: &ConeRuledPose<T>,
    node: &SectionNode<T>,
    _rulings: [Vec3<T>; 2],
    phi: f64,
    witness: Point3<T>,
    band: Band,
) -> Result<NodeEnd, SectionError> {
    let v = pose.ruled.dir(&ruling::Cell::at(phi)).mid() - Point3::origin();
    let along = |l: Vec3<T>| v.dot(l).abs();
    let line = match (along(node.lines[0]) - along(node.lines[1])).sign_within(band) {
        // NO-DECISION: which of two lines a ruling found near one of them
        // is nearer, a selection.
        Ok(Decided {
            sign: Sign::Negative,
            ..
        }) => NodeLine::Second,
        _ => NodeLine::First,
    };
    let l = match line {
        NodeLine::First => node.lines[0],
        NodeLine::Second => node.lines[1],
    };
    let nappe = (witness - pose.ruled.apex).dot(pose.ruled.axis);
    let ray = v * nappe.unit_sign();
    let sense = match ray.dot(l).sign_within(band) {
        // NO-DECISION: the sense of a ruling read along a line it lies on
        // to the sweep's precision, a selection.
        Ok(Decided {
            sign: Sign::Positive,
            ..
        }) => RaySense::Along,
        Ok(Decided {
            sign: Sign::Negative,
            ..
        }) => RaySense::Against,
        _ => {
            return Err(SectionError::Unaccounted {
                cause: "an arc's ray at the apex read along neither sense of its node line"
                    .to_owned(),
            });
        }
    };
    Ok(NodeEnd {
        node: 0,
        line,
        sense,
    })
}
