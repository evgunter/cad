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
//! save the points a tangency or a touch lists. A crossing tangency is a
//! node (C5), its arcs ending at it, one node-joined set. No march
//! carries the pair yet (limb 2 has no composite for the cone at the
//! march's search, and the pair has no march door), so carrying refuses
//! typed ([`super::NoCarrier::ConeUnmarched`]).

use geom::Surface;
use geom_core::k_stats::decide_reported;
use geom_core::{Band, Decide, Decided, Margin, Point3, Real, Sign, Vec3};

use super::cylinders::RegionPlacement;
use super::ruling::{self, End, Form, RuledCone, Swept};
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
    /// The cone's own seam, squared to its axis.
    seam: Vec3<T>,
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
    let square = frame.square();
    Ok(Some(Carrier {
        apex,
        axis: square.axis,
        seam: square.u_ref,
        sc,
    }))
}

/// The cone's rulings against `partner`, the ruling angle from the
/// cone's own seam.
fn ruled<T: Real>(cone: Carrier<T>, partner: Form<T>, region: ExtentBall<T>) -> RuledCone<T> {
    RuledCone {
        apex: cone.apex,
        axis: cone.axis,
        n: cone.seam,
        m: cone.axis.cross(cone.seam),
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
        let (ruled_first, (ruled, apex)) = match (one, two) {
            (Some(cone), None) => (true, Self::against_wall(cone, b, region, band)?),
            (None, Some(cone)) => (false, Self::against_wall(cone, a, region, band)?),
            (Some(one), Some(two)) => match Self::against_cone(one, two, region, band)? {
                Some((first, ruled, apex)) => (first, (ruled, apex)),
                None => return Ok(ConeRead::Elsewhere),
            },
            (None, None) => unreachable!("`decide` admits only pairs with a cone"),
        };
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

    /// **The cone's rulings against the cylinder `wall`**, and the apex
    /// margin. The wall's axis is read from the apex's foot on it, a point
    /// the geometry fixes, so the read does not move with where the wall's
    /// frame stands along its axis. The foot itself is computed from the
    /// frame's origin, and carries that origin's rounding off an oblique
    /// axis, of the size of its distance from the apex: that slack is
    /// charged wherever `u` enters ([`Form::with_slack`]).
    fn against_wall(
        cone: Carrier<T>,
        wall: &Surface<T>,
        region: ExtentBall<T>,
        band: Band,
    ) -> Result<(RuledCone<T>, Decided), SectionError> {
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
        let o = frame.origin();
        let axis = geom::Line {
            origin: o + d * d.dot(cone.apex - o),
            dir: d,
        };
        let slack =
            geom_core::rounding_charge((o - Point3::origin()).norm() + (cone.apex - o).norm());
        let ruled = ruled(cone, Form::cylinder(axis, radius).with_slack(slack), region);
        let apex = read(
            ConeDecision::ApexOnPartner,
            Margin::of(ruled.partner_residual(ruled.apex)),
            band,
        )?;
        Ok((ruled, apex))
    }

    /// **Two cones' ruled side**: `None` where the value does not build
    /// the pose ([`ConeRead::Elsewhere`]); else which is ruled, the first
    /// unless only the second's apex lies on the other.
    fn against_cone(
        one: Carrier<T>,
        two: Carrier<T>,
        region: ExtentBall<T>,
        band: Band,
    ) -> Result<Option<(bool, RuledCone<T>, Decided)>, SectionError> {
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
            (_, Sign::Zero) => (false, on_one, second_on),
            _ => (true, on_two, first_on),
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
        // A unit pair spanning the plane: the cone's axis or its seam
        // projected onto it, whichever stands longer there (they are
        // square, so one does), a selection the form's determinant and
        // null lines do not depend on.
        let onto = |x: Vec3<T>| x - normal * normal.dot(x);
        let (pa, pn) = (onto(ruled.axis), onto(ruled.n));
        let pick = pa.norm() - pn.norm();
        let e1 = Vec3::new(
            pick.select_le_zero(pn.x, pa.x),
            pick.select_le_zero(pn.y, pa.y),
            pick.select_le_zero(pn.z, pa.z),
        )
        .normalize();
        let e2 = normal.cross(e1);
        let (a, cos) = (ruled.axis, ruled.sc.1);
        let form = |x: Vec3<T>, y: Vec3<T>| a.dot(x) * a.dot(y) - cos.powi(2) * x.dot(y);
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
                // `Q(cos θ e₁ + sin θ e₂) = h + b cos 2θ + s₂ sin 2θ`, `h`
                // the trace's half, `b = (Q₁₁ − Q₂₂)/2`, `s₂ = Q₁₂`; its
                // zeros are `2θ = ψ ± acos(−h/R)`, `R = √(b² + s₂²)`,
                // `ψ` the angle of `(b, s₂)`. Indefinite, `R > |h|`, so
                // nothing divides by a diagonal entry that may vanish.
                let half = T::from_f64(0.5);
                let (h, b, s2) = ((q11 + q22) * half, (q11 - q22) * half, q12);
                let r = (b.powi(2) + s2.powi(2)).sqrt();
                let psi = (b / r)
                    .max(T::zero() - T::one())
                    .min(T::one())
                    .acos()
                    .copysign(s2);
                let open = ((T::zero() - h) / r)
                    .max(T::zero() - T::one())
                    .min(T::one())
                    .acos();
                let line = |theta: T| {
                    let (sn, cs) = theta.sin_cos();
                    (e1 * cs + e2 * sn).turned_toward(a)
                };
                Ok(ConeStance::Node {
                    normal,
                    rulings: [line((psi + open) * half), line((psi - open) * half)],
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
            (Some(r), _) => geom::Line { origin: at, dir: d }.distance(p) - r,
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
    let mut listed = Vec::new();
    let held = |p: Point3<T>| {
        super::cylinders::placement(super::cylinders::Occupant::point(p), region, band)
            != RegionPlacement::Outside
    };
    let point = |at: Point3<T>, why: NoCarrier| {
        SectionComponent::closed_form(
            ComponentShape::Point,
            [false, false],
            Some(at),
            ClassCarrier::Unminted(why),
        )
    };
    let (arcs, found) = match pose.stance {
        ConeStance::Sheets => {
            let swept = ruling::sweep(&pose.ruled, region, band)?;
            for t in &swept.touches {
                decided.push(ZeroVerdict {
                    decision: SectionDecision::Tangency(ruling::TANGENCY),
                    margin: t.margin,
                });
                if !t.crossing && held(t.at) {
                    listed.push(point(t.at, NoCarrier::Tangent));
                }
            }
            (swept.components, Found::Crossings)
        }
        ConeStance::Node { .. } | ConeStance::Touch { .. } => {
            decided.push(ZeroVerdict {
                decision: SectionDecision::Coincidence(Coincident::ConeApex),
                margin: pose.apex.margin,
            });
            let divided = ruling::sweep_divided(&pose.ruled, region, band)?;
            let node = matches!(pose.stance, ConeStance::Node { .. });
            let passes = if node && held(apex) { 2 } else { 0 };
            if divided.nodes.len() != passes {
                return Err(SectionError::Unaccounted {
                    cause: format!(
                        "the divided sweep read {} passes of the sheet through the apex where the \
                         apex's form and the region give {passes}",
                        divided.nodes.len()
                    ),
                });
            }
            if !node && held(apex) {
                listed.push(point(apex, NoCarrier::ApexTouch));
            }
            let found = match pose.stance {
                ConeStance::Node { normal, rulings } => Found::Apex { normal, rulings },
                // A touch's sheet passes no node: the count above holds it
                // to none.
                ConeStance::Touch { .. } | ConeStance::Sheets => Found::Crossings,
            };
            (divided.arcs, found)
        }
    };
    let nodes = Nodes::read(&pose, &arcs, found, band);
    for arc in &arcs {
        let Some([from, to]) = arc.ends else {
            listed.push(SectionComponent::closed_form(
                ComponentShape::Loop,
                pose.order([arc.encircles, false]),
                Some(arc.witness),
                ClassCarrier::General,
            ));
            continue;
        };
        let end = |e: End<T>, start: bool| match e {
            End::Node(pass) => nodes.end(&pose, pass, start, arc.witness, band).map(Some),
            End::Cut(_) => Ok(None),
        };
        listed.push(SectionComponent {
            cut_ends: arc.cut_ends(),
            ..SectionComponent::closed_form(
                ComponentShape::Open {
                    ends: ComponentEnds {
                        cut: matches!(from, End::Cut(_)) || matches!(to, End::Cut(_)),
                        nodes: NodeEnds {
                            start: end(from, true)?,
                            end: end(to, false)?,
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
    let nodes = nodes.listed;
    let node_sets = node_joined(&listed, nodes.len(), [false, false]);
    let partner = if pose.ruled_first {
        &pose.second
    } else {
        &pose.first
    };
    let partner_arm = match partner {
        Surface::Cylinder { .. } => crate::uniform_curvature_arm(partner),
        _ => region.lever_from(pose.ruled.partner.at),
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

/// What the sweep's passes through nodes pass: crossing tangencies, or
/// the apex with its two rulings in the partner's tangent plane (each
/// along its generator's opening direction).
#[derive(Clone, Copy)]
enum Found<T: Real> {
    Crossings,
    Apex {
        normal: Vec3<T>,
        rulings: [Vec3<T>; 2],
    },
}

/// **The nodes the sweep's arcs end at** (C5, [`SectionNode`]), each with
/// what an arc end there reads: the raw lines its rays run along (before
/// the second is turned counterclockwise from the first about the
/// normal), and whether that turn reversed the second.
struct Nodes<T: Real> {
    listed: Vec<SectionNode<T>>,
    /// Per node: the pass angle it was found at (a crossing's; the apex's
    /// arcs name their passes by the ruling, so none), its raw lines, and
    /// whether `lines[1]` is the second reversed.
    keys: Vec<(Option<f64>, [Vec3<T>; 2], bool)>,
    found: Found<T>,
}

impl<T: Decide> Nodes<T> {
    /// The nodes `arcs` end at: one per crossing angle they pass, or the
    /// apex where they pass it.
    fn read(pose: &ConeRuledPose<T>, arcs: &[Swept<T>], found: Found<T>, band: Band) -> Self {
        let mut out = Self {
            listed: Vec::new(),
            keys: Vec::new(),
            found,
        };
        let passes = arcs
            .iter()
            .flat_map(|a| a.ends.into_iter().flatten())
            .filter_map(|e| match e {
                End::Node(pass) => Some(pass.phi),
                End::Cut(_) => None,
            });
        match found {
            Found::Crossings => {
                for phi in passes {
                    if out.keys.iter().any(|k| k.0 == Some(phi)) {
                        continue;
                    }
                    let x = pose.ruled.crossing(phi);
                    let [a, b, c] = x.form;
                    let (flip, l1) = turned(x.lines, x.normal, band);
                    // The sector from `lines[0]` to `lines[1]` holds the
                    // image of `(1, m+) + (±1, ±m−)`; the form's sign on it
                    // is the partner's residual on the cone there, and the
                    // cone lies on the normal's side where that residual
                    // grows outward.
                    let sign = if flip { T::zero() - T::one() } else { T::one() };
                    let (cphi, ct) = (T::one() + sign, x.slopes[0] + sign * x.slopes[1]);
                    let h = a * cphi.powi(2) + T::from_f64(2.0) * b * cphi * ct + c * ct.powi(2);
                    // NO-DECISION: a label for the sectors, which no reader of a cone pair decides on.
                    let above = above_where((h * x.outward).sign_within(band), pose.ruled_side());
                    out.listed.push(SectionNode {
                        point: x.point,
                        normal: x.normal,
                        lines: [x.lines[0], l1],
                        above,
                    });
                    out.keys.push((Some(phi), x.lines, flip));
                }
            }
            Found::Apex { normal, rulings } => {
                if passes.count() > 0 {
                    let (flip, l1) = turned(rulings, normal, band);
                    let (a, c) = (pose.ruled.axis, pose.ruled.sc.1);
                    let w = rulings[0] + l1;
                    let inside = a.dot(w).powi(2) - c.powi(2) * w.norm_squared();
                    // NO-DECISION: a label for the sectors, which no reader of a cone pair decides on.
                    let above = above_where(inside.sign_within(band), pose.ruled_side());
                    out.listed.push(SectionNode {
                        point: pose.ruled.apex,
                        normal,
                        lines: [rulings[0], l1],
                        above,
                    });
                    out.keys.push((None, rulings, flip));
                }
            }
        }
        out
    }

    /// **An arc's end at the node it passes at `pass`**, `start` the end
    /// its walk leaves from: the line its ray runs along and the sense.
    /// At a crossing, the sheet leaves toward the side `e` of the angle
    /// (the walk's direction past a start, against it before an end)
    /// along the line of `m` of sign `σe` ([`ruling::Crossing`]); at the
    /// apex, the ruling the pass is at, read by azimuth against the two
    /// closed-form rulings, toward the nappe the arc lies on (`t` keeps
    /// its sign between the arc's ends: it changes only through the apex
    /// or past every bound, where the region cuts it).
    fn end(
        &self,
        pose: &ConeRuledPose<T>,
        pass: ruling::Pass,
        start: bool,
        witness: Point3<T>,
        band: Band,
    ) -> Result<NodeEnd, SectionError> {
        let unread = |what: &'static str| SectionError::Unaccounted {
            cause: what.to_owned(),
        };
        let (node, k, along) = match self.found {
            Found::Crossings => {
                let node = self
                    .keys
                    .iter()
                    .position(|key| key.0 == Some(pass.phi))
                    .ok_or_else(|| unread("an arc passes a crossing no node lists"))?;
                let e_forward = start != pass.backward;
                let sigma = pass.sheet == ruling::Sheet::Plus;
                // `σe > 0` takes `m+`, the first raw line.
                let k = usize::from(sigma != e_forward);
                (node, k, e_forward)
            }
            Found::Apex { .. } => {
                let (_, rulings, _) = self
                    .keys
                    .first()
                    .ok_or_else(|| unread("an arc passes the apex no node lists"))?;
                let r = &pose.ruled;
                let v = r.dir(&ruling::Cell::at(pass.phi)).mid() - Point3::origin();
                let azimuth = |l: Vec3<T>| {
                    let (x, y) = (v - r.axis * r.axis.dot(v), l - r.axis * r.axis.dot(l));
                    x.dot(y) / (x.norm() * y.norm())
                };
                // NO-DECISION: the nearer of two rulings by azimuth, a selection between them.
                let second = (azimuth(rulings[1]) - azimuth(rulings[0])).sign_within(band);
                let k = usize::from(matches!(
                    second,
                    Ok(Decided {
                        sign: Sign::Positive,
                        ..
                    })
                ));
                let nappe = (witness - r.apex).dot(r.axis);
                // NO-DECISION: the nappe of a point the sweep decided on the arc, off the apex.
                let up = match nappe.sign_within(band) {
                    Ok(Decided {
                        sign: Sign::Positive,
                        ..
                    }) => true,
                    Ok(Decided {
                        sign: Sign::Negative,
                        ..
                    }) => false,
                    _ => return Err(unread("an arc's witness reads on neither nappe")),
                };
                (0, k, up)
            }
        };
        let flip = self.keys[node].2;
        // The ray is `±raw[k]`, `+` where `along`; `lines[1]` is `raw[1]`
        // reversed where `flip`.
        let reversed = k == 1 && flip;
        Ok(NodeEnd {
            node,
            line: if k == 0 {
                NodeLine::First
            } else {
                NodeLine::Second
            },
            sense: if along != reversed {
                RaySense::Along
            } else {
                RaySense::Against
            },
        })
    }
}

/// `lines[1]` turned counterclockwise from `lines[0]` about `normal` by
/// less than a half turn: whether that reversed it, and the line.
fn turned<T: Decide>(lines: [Vec3<T>; 2], normal: Vec3<T>, band: Band) -> (bool, Vec3<T>) {
    // NO-DECISION: an orientation of two lines the pose read apart, a selection.
    let turn = lines[0].cross(lines[1]).dot(normal).sign_within(band);
    let flip = matches!(
        turn,
        Ok(Decided {
            sign: Sign::Negative,
            ..
        })
    );
    (flip, if flip { -lines[1] } else { lines[1] })
}

/// **`above` at a node** ([`SectionNode::above`]): `cone` where the
/// reading on sector 0 is positive, the partner otherwise.
fn above_where(read: Result<Decided, geom_core::Indeterminate>, cone: PairSide) -> PairSide {
    match read {
        Ok(Decided {
            sign: Sign::Positive,
            ..
        }) => cone,
        _ => cone.other(),
    }
}
