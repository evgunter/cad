//! **Two cylinders on the section value** (C5): the pose decided once on
//! closed-form margins, read over the operation's region, and the
//! section's components over the pair's ball cut by that region (C3's
//! domain rule).
//!
//! Neither side is bounded, so the pose reads the region the caller
//! hands it: parallelism is `‖a₁ × a₂‖` levered across it
//! ([`crate::cylinder_axes_parallel`] at the region's ball), and the
//! axis rows are read at the feet of its centre. Parallel axes are then
//! read by their distance `d` against zero (one surface), `r₁ + r₂` and
//! `|r₁ − r₂|`; crossing axes by the gap `e` along `a₁ × a₂` against the
//! same two levels. The inner level decided `Zero` is a node pose: the
//! bisector ellipses where the radii decide equal and the axes meet,
//! the pinch otherwise.

use geom::{Curve3, Surface};
use geom_core::{Band, Decide, Decided, Margin, MarginDiag, Point3, Real, Sign, Vec3};

use super::{
    ClassCarrier, Coincident, ComponentEnds, ComponentShape, Components, PairPose, PairSection,
    SectionComponent, SectionDecision, TANGENT_COMPONENT, ZeroVerdict,
};
use crate::ExtentBall;
use crate::extent::Reach;
use crate::intersect::{
    SectionError, SectionRadius, cylinder_axes_parallel_reported, decide_across_reported,
    parallel_axes_at, parallel_cylinder_gap_reported, parallel_swing, tilted_section_major,
};

/// The note the pinch's lobes carry: their arcs end at the node, and no
/// march that starts at a node is built.
pub const PINCH: &str = "the thinner wall touches the fatter one from inside, a node its two \
                         lobes end at, and no march that starts at a node is built";

/// **The questions the cylinder pair's pose decides**, each on one
/// margin in metres.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CcDecision {
    /// `cc_cylinder_radius`: each radius definitely positive.
    Radius,
    /// `cc_axes_parallel`: `‖a₁ × a₂‖` levered across the region.
    Parallel,
    /// `cc_coaxial`: parallel, the axes' distance `d` against zero.
    Coaxial,
    /// `cc_parallel_gap`: parallel, `r₁ + r₂ − d`.
    ParallelGap,
    /// `cc_parallel_nest`: parallel, `d − |r₁ − r₂|`.
    ParallelNest,
    /// `cc_crossing_gap`: crossing, `e − (r₁ + r₂)`.
    CrossingGap,
    /// `cc_crossing_nest`: crossing, `e − |r₁ − r₂|`.
    CrossingNest,
    /// `cc_radius_equality`: at the inner level, `r₁ − r₂`.
    RadiusEquality,
    /// `cc_axes_coplanar`: at the inner level, the signed gap.
    AxesMeet,
    /// `cc_thinner`: two loops, `r₁ − r₂`, which wall they encircle.
    Thinner,
}

impl CcDecision {
    /// The decision's K-funnel name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Radius => "cc_cylinder_radius",
            Self::Parallel => "cc_axes_parallel",
            Self::Coaxial => "cc_coaxial",
            Self::ParallelGap => "cc_parallel_gap",
            Self::ParallelNest => "cc_parallel_nest",
            Self::CrossingGap => "cc_crossing_gap",
            Self::CrossingNest => "cc_crossing_nest",
            Self::RadiusEquality => "cc_radius_equality",
            Self::AxesMeet => "cc_axes_coplanar",
            Self::Thinner => "cc_thinner",
        }
    }
}

/// **The node pose at the inner level.**
#[derive(Clone, Copy, Debug)]
pub enum CcNode {
    /// Equal radii, meeting axes: the two bisector-plane ellipses,
    /// crossing at two nodes.
    Bisector {
        /// `cc_radius_equality`'s margin, a coincidence (D10).
        radii: MarginDiag,
        /// `cc_axes_coplanar`'s margin, a coincidence (D10).
        meet: MarginDiag,
    },
    /// The thinner wall inside the fatter one, touching it at one node.
    Pinch,
}

/// **How the two axes stand**, decided.
#[derive(Clone, Copy, Debug)]
pub enum CcStance<T: Real> {
    /// Parallel axes: the section is rulings, or nothing.
    Parallel {
        /// The region centre's foot on the first axis.
        foot: Point3<T>,
        /// From `foot` to the second axis, square to it.
        across: Vec3<T>,
        /// `cc_parallel_gap`.
        gap: Decided,
        /// `cc_parallel_nest`, where the gap is positive.
        nest: Option<Decided>,
    },
    /// Crossing axes.
    Crossing {
        /// The common perpendicular's foot on the first axis.
        foot: Point3<T>,
        /// `unit(a₁ × a₂)`.
        normal: Vec3<T>,
        /// `‖a₁ × a₂‖`.
        sin: T,
        /// The signed gap along `normal` from the first axis to the
        /// second.
        gap: T,
        /// `cc_crossing_gap`.
        reach: Decided,
        /// `cc_crossing_nest`, where the walls cross.
        nest: Option<Decided>,
        /// The node pose, where the nest is `Zero`.
        node: Option<CcNode>,
        /// `cc_thinner`, where there are two loops.
        thinner: Option<Sign>,
    },
}

/// **The cylinder pair's pose, decided once** over the region it was
/// asked across ([`Self::decide`]).
#[derive(Clone, Debug)]
pub struct CylinderCylinderPose<T: Real> {
    first: Surface<T>,
    second: Surface<T>,
    lines: [(Point3<T>, Vec3<T>); 2],
    radii: [T; 2],
    region: ExtentBall<T>,
    stance: CcStance<T>,
    held: bool,
}

/// `Ok` where the margin decides, the predicate named.
fn read<T: Decide>(decision: CcDecision, value: T, band: Band) -> Result<Decided, SectionError> {
    geom_core::k_stats::decide_reported(decision.name(), Margin::of(value), band)
        .map_err(SectionError::Escalated)
}

impl<T: Decide> CylinderCylinderPose<T> {
    /// **Decides the pose of `a` and `b`**, two cylinders, across
    /// `reach` (module docs): the operation's region
    /// ([`Reach::Ball`]), or the consumed extent a pose question names
    /// ([`crate::route_pose`]). In order:
    ///
    /// 1. `cc_cylinder_radius`, each radius definitely positive.
    /// 2. `cc_axes_parallel`, levered across `reach`.
    /// 3. Parallel: `cc_coaxial`, `d` read across the region with its
    ///    swing beside it, `Zero` one surface; then `cc_parallel_gap`
    ///    and, where it is positive, `cc_parallel_nest`, each read
    ///    across the same swing.
    /// 4. Crossing: `cc_crossing_gap`, then where the walls cross
    ///    `cc_crossing_nest`; at its `Zero` the node pose, by
    ///    `cc_radius_equality` and `cc_axes_coplanar`; at two loops
    ///    `cc_thinner`.
    ///
    /// `None` for a pair that is not two cylinders.
    ///
    /// # Errors
    ///
    /// [`SectionError::RadiusEscalated`] or
    /// [`SectionError::DegenerateOperand`] for a radius not definitely
    /// positive; [`SectionError::CoincidentSurfaces`] for one surface;
    /// [`SectionError::Escalated`] for a question in the band.
    pub fn decide(
        a: &Surface<T>,
        b: &Surface<T>,
        reach: &Reach<T>,
        band: Band,
    ) -> Option<Result<Self, SectionError>> {
        let (
            &Surface::Cylinder {
                origin: o1,
                axis: a1,
                radius: r1,
                ..
            },
            &Surface::Cylinder {
                origin: o2,
                axis: a2,
                radius: r2,
                ..
            },
        ) = (a, b)
        else {
            return None;
        };
        let lines = [(o1, a1), (o2, a2)];
        let region = reach.ball();
        Some(
            Self::decide_stance(lines, [r1, r2], reach, band).map(|(stance, held)| Self {
                first: a.clone(),
                second: b.clone(),
                lines,
                radii: [r1, r2],
                region,
                stance,
                held,
            }),
        )
    }

    fn decide_stance(
        [l1, l2]: [(Point3<T>, Vec3<T>); 2],
        [r1, r2]: [T; 2],
        reach: &Reach<T>,
        band: Band,
    ) -> Result<(CcStance<T>, bool), SectionError> {
        for r in [r1, r2] {
            match geom_core::k_stats::decide_reported(
                CcDecision::Radius.name(),
                Margin::of(r),
                band,
            ) {
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
        }
        let parallel = cylinder_axes_parallel_reported(reach, l1, l2, band)
            .map_err(SectionError::Escalated)?;
        if parallel.sign == Sign::Zero {
            let at = parallel_axes_at(reach, l1, l2);
            let swing = parallel_swing(reach, l1, l2, at.d);
            let across = |names: [&'static str; 2], value: T| {
                decide_across_reported(names, value, swing, band)
                    .map(|(sign, margin)| Decided { sign, margin })
                    .map_err(SectionError::Escalated)
            };
            if across(["cc_coaxial", "cc_coaxial_floor"], at.d)?.sign == Sign::Zero {
                return Err(SectionError::CoincidentSurfaces);
            }
            let gap = parallel_cylinder_gap_reported((r1, r2), at.d, swing, band)
                .map(|(sign, margin)| Decided { sign, margin })
                .map_err(SectionError::Escalated)?;
            let nest = match gap.sign {
                Sign::Positive => Some(across(
                    ["cc_parallel_nest", "cc_parallel_nest_floor"],
                    at.d - (r1 - r2).abs(),
                )?),
                Sign::Zero | Sign::Negative => None,
            };
            let stance = CcStance::Parallel {
                foot: at.foot1,
                across: at.d_vec,
                gap,
                nest,
            };
            return Ok((stance, false));
        }
        let (a1, a2) = (l1.1, l2.1);
        let cross = a1.cross(a2);
        let sin = cross.norm();
        let normal = cross / sin;
        let w = reach.foot_on(l2.0, a2) - reach.foot_on(l1.0, a1);
        let gap = w.dot(normal);
        // The common perpendicular's foot on the first axis, from the
        // region centre's foot.
        let f1 = reach.foot_on(l1.0, a1);
        let foot = f1 + a1 * (w.cross(a2).dot(cross) / sin.powi(2));
        let e = gap.abs();
        let reach_d = read(CcDecision::CrossingGap, e - (r1 + r2), band)?;
        let nest = match reach_d.sign {
            Sign::Negative => Some(read(CcDecision::CrossingNest, e - (r1 - r2).abs(), band)?),
            Sign::Zero | Sign::Positive => None,
        };
        let node = match nest.map(|n| n.sign) {
            Some(Sign::Zero) => {
                let radii = read(CcDecision::RadiusEquality, r1 - r2, band)?;
                if radii.sign == Sign::Zero {
                    let meet = read(CcDecision::AxesMeet, gap, band)?;
                    match meet.sign {
                        Sign::Zero => Some(CcNode::Bisector {
                            radii: radii.margin,
                            meet: meet.margin,
                        }),
                        // Equal radii put the inner level at the gap
                        // itself, which the nest just read `Zero`.
                        Sign::Positive | Sign::Negative => {
                            return Err(SectionError::Pinched {
                                predicate: CcDecision::AxesMeet.name(),
                            });
                        }
                    }
                } else {
                    Some(CcNode::Pinch)
                }
            }
            _ => None,
        };
        let thinner = match nest.map(|n| n.sign) {
            Some(Sign::Negative) => Some(read(CcDecision::Thinner, r1 - r2, band)?.sign),
            _ => None,
        };
        let stance = CcStance::Crossing {
            foot,
            normal,
            sin,
            gap,
            reach: reach_d,
            nest,
            node,
            thinner,
        };
        let held = region_holds(
            crossing_ball(foot, normal, sin, gap, [r1, r2]),
            reach.ball(),
            band,
        );
        Ok((stance, held))
    }

    /// The two surfaces in the order the pose was asked of.
    pub fn operands(&self) -> (&Surface<T>, &Surface<T>) {
        (&self.first, &self.second)
    }

    /// The radii, in that order, each decided positive.
    pub fn radii(&self) -> [T; 2] {
        self.radii
    }

    /// The axes' unit directions, in that order.
    pub fn axes(&self) -> [Vec3<T>; 2] {
        self.lines.map(|(_, a)| a)
    }

    /// The region the pose was read across.
    pub fn region(&self) -> ExtentBall<T> {
        self.region
    }

    /// How the axes stand.
    pub fn stance(&self) -> CcStance<T> {
        self.stance
    }

    /// **The ball the section is read over**: the pair's own ball
    /// `(r₁ + r₂)/sin θ` across and `min r + |e|/2` along the common
    /// perpendicular about its midpoint, where the region holds it;
    /// otherwise the region, which holds every section point the
    /// operation can use.
    pub fn ball(&self) -> ExtentBall<T> {
        match self.stance {
            CcStance::Crossing {
                foot,
                normal,
                sin,
                gap,
                ..
            } if self.held => crossing_ball(foot, normal, sin, gap, self.radii),
            CcStance::Crossing { .. } | CcStance::Parallel { .. } => self.region,
        }
    }

    /// Whether the region cuts the section, so that a component may end
    /// on the domain's boundary.
    pub fn cut(&self) -> bool {
        !self.held
    }

    /// **The tangency the pose decided**, if any: a tangent ruling, an
    /// outer touch, or a node pose, each decided `Zero`.
    pub fn tangency(&self) -> Option<ZeroVerdict> {
        let zero = |decision: CcDecision, d: Decided| {
            (d.sign == Sign::Zero).then_some(ZeroVerdict {
                decision: SectionDecision::Tangency(decision.name()),
                margin: d.margin,
            })
        };
        match self.stance {
            CcStance::Parallel { gap, nest, .. } => zero(CcDecision::ParallelGap, gap)
                .or_else(|| nest.and_then(|n| zero(CcDecision::ParallelNest, n))),
            CcStance::Crossing { reach, nest, .. } => zero(CcDecision::CrossingGap, reach)
                .or_else(|| nest.and_then(|n| zero(CcDecision::CrossingNest, n))),
        }
    }

    /// **The nodes the pose places in closed form**: the bisector's two,
    /// `p ∓ r·n̂` about the axes' meeting point `p`, or the pinch's one
    /// on the common perpendicular, `r₁` from the first axis on the side
    /// the fatter wall's axis stands for an outer, against it for an
    /// inner wall.
    pub fn nodes(&self) -> Vec<Point3<T>> {
        let CcStance::Crossing {
            foot,
            normal,
            gap,
            node: Some(node),
            ..
        } = self.stance
        else {
            return Vec::new();
        };
        let [r1, r2] = self.radii;
        match node {
            CcNode::Bisector { .. } => {
                let off = normal * r1.abs();
                vec![foot - off, foot + off]
            }
            CcNode::Pinch => vec![foot + normal * r1.copysign(gap * (r1 - r2))],
        }
    }

    /// The point where the walls touch at an outer touch, `r₁` along the
    /// common perpendicular from the first axis toward the second.
    fn touch(&self, foot: Point3<T>, normal: Vec3<T>, gap: T) -> Point3<T> {
        let [r1, r2] = self.radii;
        foot + normal * (gap * r1 / (r1 + r2))
    }
}

/// The pair's own ball for crossing axes (module docs of
/// [`CylinderCylinderPose::ball`]). A point within `r₁` of the first
/// axis and `r₂` of the second stands within `(r₁ + r₂)/sin θ` of the
/// common perpendicular's midpoint across it, and within `min r + |e|/2`
/// along it.
fn crossing_ball<T: Real>(
    foot: Point3<T>,
    normal: Vec3<T>,
    sin: T,
    gap: T,
    [r1, r2]: [T; 2],
) -> ExtentBall<T> {
    let half = T::from_f64(0.5);
    let across = (r1 + r2) / sin;
    let along = r1.min(r2) + gap.abs() * half;
    ExtentBall::new(
        foot + normal * (gap * half),
        (across.powi(2) + along.powi(2)).sqrt(),
    )
}

/// **Whether the region holds the pair's whole ball**, the one place a
/// crossing pose is listed counted or marched: counted where it does,
/// the closed form then counting every component the domain holds, and
/// marched where the region cuts the section. A choice of listing, not
/// a decision about the pair, so it reads the margin's sign without
/// recording it.
fn region_holds<T: Decide>(ball: ExtentBall<T>, region: ExtentBall<T>, band: Band) -> bool {
    let spare = region.radius() - ((ball.center() - region.center()).norm() + ball.radius());
    matches!(
        spare.sign_within(band),
        Ok(Decided {
            sign: Sign::Positive,
            ..
        })
    )
}

/// The parameter span of the line `origin + t·dir` (`dir` unit) inside
/// `region`, empty at the foot of its centre where the line misses it.
fn region_span<T: Real>(region: ExtentBall<T>, origin: Point3<T>, dir: Vec3<T>) -> (T, T) {
    let to = region.center() - origin;
    let t = to.dot(dir);
    let off = to.norm_squared() - t.powi(2);
    let half = (region.radius().powi(2) - off).max(T::zero()).sqrt();
    (t - half, t + half)
}

/// The two bisector-plane ellipses through the axes' meeting point `p`,
/// `e₁` in the `a₁ − a₂`-normal plane and `e₂` in the `a₁ + a₂`-normal
/// one: each the tilted plane cut of the first wall, on the second by the
/// mirror symmetry. The nodes stand at `e₁(±π/2)` and `e₂(∓π/2)`.
///
/// The normals are `a₁ ∓ a₂`, whose halved lengths are the half-angle's
/// sine and cosine read without cancellation. The shorter is made square
/// to the longer: an axis a hair off unit turns `a₁ − a₂` toward
/// `a₁ + a₂` by `u / φ`, and the long ellipse's plane must not tilt by
/// that.
pub(crate) fn bisector_ellipses<T: Decide>(
    p: Point3<T>,
    (a1, a2): (Vec3<T>, Vec3<T>),
    r: T,
    band: Band,
) -> Result<[Curve3<T>; 2], SectionError> {
    let two = T::from_f64(2.0);
    let (d, s) = (a1 - a2, a1 + a2);
    let off_unit = |v: Vec3<T>, w: Vec3<T>| v - w * v.dot(w);
    let (dn, sn) = (d.normalize(), s.normalize());
    let d_shorter = d.norm() - s.norm();
    let pick = |when_le: Vec3<T>, when_gt: Vec3<T>| {
        Vec3::new(
            d_shorter.select_le_zero(when_le.x, when_gt.x),
            d_shorter.select_le_zero(when_le.y, when_gt.y),
            d_shorter.select_le_zero(when_le.z, when_gt.z),
        )
    };
    let (d, s) = (pick(off_unit(d, sn), d), pick(s, off_unit(s, dn)));
    let (n1, n2) = (d.normalize(), s.normalize());
    let (sin_half, cos_half) = (d.norm() / two, s.norm() / two);
    let mk = |n: Vec3<T>, c: T| -> Result<Curve3<T>, SectionError> {
        let u_major = tilted_section_major(n, a1.cross(n));
        Ok(Curve3::ellipse(p, n, r / c, r, u_major, band)?)
    };
    Ok([mk(n1, sin_half)?, mk(n2, cos_half)?])
}

/// **The rulings of parallel walls**, in closed form: the chord of the
/// two cross-section circles, `x` along `û` from the first axis's foot
/// and `±h` across, `x = (d² + r₁² − r₂²)/(2d)` and `h² = r₁² − x²`, `d`
/// the axes' distance square to them. At a tangency the two coincide.
pub(crate) fn parallel_rulings<T: Real>(
    foot: Point3<T>,
    across: Vec3<T>,
    axis: Vec3<T>,
    [r1, r2]: [T; 2],
) -> [Point3<T>; 2] {
    let d = across.norm();
    let u = across / d;
    let x = (d.powi(2) + r1.powi(2) - r2.powi(2)) / (d * T::from_f64(2.0));
    let h = (r1.powi(2) - x.powi(2)).max(T::zero()).sqrt();
    let side = axis.cross(u);
    let at = foot + u * x;
    [at + side * h, at - side * h]
}

/// **The cylinder pair's section over its decided pose** (module docs).
///
/// # Errors
///
/// [`SectionError::Carrier`] where a bisector ellipse refuses.
pub(super) fn cylinder_cylinder<T: Decide>(
    pose: CylinderCylinderPose<T>,
    band: Band,
) -> Result<PairSection<T>, SectionError> {
    let [r1, r2] = pose.radii;
    let axis = pose.lines[0].1;
    let mut decided = Vec::new();
    let component = |shape, winds, witness, carrier| SectionComponent {
        shape,
        winds,
        witness,
        carrier,
    };
    let cut = ComponentShape::Open {
        ends: ComponentEnds::cut(),
    };
    let ruling = |p: Point3<T>| {
        let span = region_span(pose.region, p, axis);
        let line = Curve3::Line {
            origin: p,
            dir: axis,
        };
        let witness = line.eval((span.0 + span.1) * T::from_f64(0.5));
        (line, span, witness)
    };
    let components = match pose.stance {
        CcStance::Parallel {
            foot,
            across,
            gap,
            nest,
        } => {
            let [p, q] = parallel_rulings(foot, across, axis, pose.radii);
            match (gap.sign, nest.map(|n| n.sign)) {
                (Sign::Negative, _) | (Sign::Positive, Some(Sign::Negative)) => {
                    Components::Counted(Vec::new())
                }
                (Sign::Zero, _) | (Sign::Positive, Some(Sign::Zero)) => {
                    let (_, _, witness) = ruling(p + (q - p) * T::from_f64(0.5));
                    Components::Counted(vec![component(
                        cut,
                        [false, false],
                        Some(witness),
                        ClassCarrier::Unminted(TANGENT_COMPONENT),
                    )])
                }
                (Sign::Positive, _) => Components::Counted(
                    [p, q]
                        .into_iter()
                        .map(|p| {
                            let (curve, span, witness) = ruling(p);
                            component(
                                cut,
                                [false, false],
                                Some(witness),
                                ClassCarrier::Span { curve, span },
                            )
                        })
                        .collect(),
                ),
            }
        }
        CcStance::Crossing {
            foot,
            normal,
            gap,
            reach,
            nest,
            node,
            thinner,
            ..
        } => match (reach.sign, nest.map(|n| n.sign)) {
            (Sign::Positive, _) => Components::Counted(Vec::new()),
            (Sign::Zero, _) => Components::Counted(vec![component(
                ComponentShape::Point,
                [false, false],
                Some(pose.touch(foot, normal, gap)),
                ClassCarrier::Unminted(TANGENT_COMPONENT),
            )]),
            (Sign::Negative, Some(Sign::Zero)) => match node {
                Some(CcNode::Bisector { radii, meet }) => {
                    for (c, margin) in [
                        (Coincident::CylinderRadii, radii),
                        (Coincident::CylinderAxesMeet, meet),
                    ] {
                        decided.push(ZeroVerdict {
                            decision: SectionDecision::Coincidence(c),
                            margin,
                        });
                    }
                    let [e1, e2] = bisector_ellipses(foot, (axis, pose.lines[1].1), r1, band)?;
                    let half = T::pi() * T::from_f64(0.5);
                    // `e₁` runs from the second node to the first over
                    // `(−π/2, π/2)`, `e₂` from the first to the second.
                    let arcs = [
                        (&e1, (T::zero() - half, half), [1, 0]),
                        (&e1, (half, half + T::pi()), [0, 1]),
                        (&e2, (T::zero() - half, half), [0, 1]),
                        (&e2, (half, half + T::pi()), [1, 0]),
                    ];
                    Components::Counted(
                        arcs.into_iter()
                            .map(|(curve, span, [from, to])| {
                                component(
                                    ComponentShape::Open {
                                        ends: ComponentEnds::nodes(from, to),
                                    },
                                    [false, false],
                                    Some(curve.eval((span.0 + span.1) * T::from_f64(0.5))),
                                    ClassCarrier::Span {
                                        curve: curve.clone(),
                                        span,
                                    },
                                )
                            })
                            .collect(),
                    )
                }
                Some(CcNode::Pinch) | None => Components::Counted(
                    (0..2)
                        .map(|_| {
                            component(
                                ComponentShape::Open {
                                    ends: ComponentEnds::nodes(0, 0),
                                },
                                [false, false],
                                None,
                                ClassCarrier::Unminted(PINCH),
                            )
                        })
                        .collect(),
                ),
            },
            _ if !pose.held => Components::Marched,
            // One loop, witnessed on the second wall's ruling on the
            // first axis's side, where that ruling enters the first wall.
            (Sign::Negative, Some(Sign::Positive)) => {
                let (o1, a1) = pose.lines[0];
                let (o2, a2) = pose.lines[1];
                let q = o2 - normal * r2.copysign(gap);
                let (av, bv) = (a2.cross(a1), (q - o1).cross(a1));
                let (qa, qb, qc) = (av.dot(av), av.dot(bv), bv.dot(bv) - r1.powi(2));
                let t = ((qb.powi(2) - qa * qc).max(T::zero()).sqrt() - qb) / qa;
                Components::Counted(vec![component(
                    ComponentShape::Loop,
                    [false, false],
                    Some(q + a2 * t),
                    ClassCarrier::General,
                )])
            }
            // Two loops, each encircling the thinner wall.
            _ => {
                let winds = match thinner {
                    Some(Sign::Negative) => [true, false],
                    _ => [false, true],
                };
                Components::Counted(
                    (0..2)
                        .map(|_| {
                            component(ComponentShape::Loop, winds, None, ClassCarrier::General)
                        })
                        .collect(),
                )
            }
        },
    };
    decided.extend(pose.tangency());
    let nodes = pose.nodes();
    Ok(PairSection {
        lever: r1.min(r2),
        nodes,
        pose: PairPose::CylinderCylinder(pose),
        decided,
        components,
    })
}
