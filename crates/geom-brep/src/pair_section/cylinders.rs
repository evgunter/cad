//! **Two cylinders on the section value** (C5): the pose decided once on
//! closed-form margins, read over the operation's region, and the
//! section's components over the pair's ball cut by that region (C3's
//! domain rule).
//!
//! Neither side is bounded, so the pose reads the region the caller
//! hands it: parallelism is `‖a₁ × a₂‖` levered across it
//! ([`crate::cylinder_axes_parallel`] at the region's ball), and the
//! axis rows are read at the feet of its centre. Parallel axes are then
//! read by their distance `d` against `r₁ + r₂` and `|r₁ − r₂|`, the
//! inner level's `Zero` one surface only where the radii are one too and
//! `d` is `Zero`, a tangent ruling inside otherwise; crossing axes by the
//! gap `e` along `a₁ × a₂` against the same two levels, the inner
//! level's `Zero` a node pose: the bisector ellipses where the radii
//! decide equal and the axes meet, the pinch otherwise.

use geom::{Curve3, Surface};
use geom_core::{Band, Decide, Decided, Margin, MarginDiag, Point3, Real, Sign, Vec3};

use super::ruling::{self, RuledCylinders, SweepRefusal};
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

    /// The floor row of a decision read across the region's swing
    /// ([`decide_across_reported`]), which decides its definite side.
    #[must_use]
    pub const fn floor(self) -> &'static str {
        match self {
            Self::Coaxial => "cc_coaxial_floor",
            Self::ParallelGap => "cc_parallel_gap_floor",
            Self::ParallelNest => "cc_parallel_nest_floor",
            Self::Radius
            | Self::Parallel
            | Self::CrossingGap
            | Self::CrossingNest
            | Self::RadiusEquality
            | Self::AxesMeet
            | Self::Thinner => self.name(),
        }
    }
}

impl SectionError {
    /// **Whether this is two parallel walls' gap or nest read in band**
    /// ([`CcDecision::ParallelGap`], [`CcDecision::ParallelNest`]): the
    /// walls meet in rulings, a tangent ruling or nothing, so every
    /// component the section could have is a line.
    #[must_use]
    pub fn parallel_walls_undecided(&self) -> bool {
        let Self::Escalated(diag) = self else {
            return false;
        };
        diag.predicate.is_some_and(|p| {
            [CcDecision::ParallelGap, CcDecision::ParallelNest]
                .into_iter()
                .any(|d| p == d.name() || p == d.floor())
        })
    }
}

/// **Where a ball stands against the region**: wholly inside it, wholly
/// clear of it, or across its boundary (or undecided there, which reads
/// as across: the region is padded, so nothing the operation uses lies
/// at its boundary).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionPlacement {
    /// Wholly inside the region.
    Inside,
    /// Wholly clear of it.
    Outside,
    /// Across its boundary.
    Straddles,
}

/// **The parallel pose's section, in closed form**: nothing, the tangent
/// ruling through the point, or the two rulings through the two points,
/// each along the axes.
#[derive(Clone, Copy, Debug)]
pub(crate) enum ParallelSection<T: Real> {
    /// The walls clear each other, or one nests inside the other.
    Empty,
    /// They touch along one ruling, outside or inside.
    Tangent(Point3<T>),
    /// They cross along two rulings.
    Rulings([Point3<T>; 2]),
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
    placed: RegionPlacement,
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
    /// 3. Parallel: `cc_parallel_gap`, read across the region with the
    ///    axes' swing beside it; where it is positive,
    ///    `cc_radius_equality`, then the inner level across the same
    ///    swing: for one radius `cc_coaxial` (`d` against zero), whose
    ///    `Zero` is one surface, and for two `cc_parallel_nest`
    ///    (`d − |r₁ − r₂|`). Only both `Zero` are a coincidence: coaxial
    ///    walls of two radii nest.
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
            Self::decide_stance(lines, [r1, r2], reach, band).map(|(stance, placed)| Self {
                first: a.clone(),
                second: b.clone(),
                lines,
                radii: [r1, r2],
                region,
                stance,
                placed,
            }),
        )
    }

    fn decide_stance(
        [l1, l2]: [(Point3<T>, Vec3<T>); 2],
        [r1, r2]: [T; 2],
        reach: &Reach<T>,
        band: Band,
    ) -> Result<(CcStance<T>, RegionPlacement), SectionError> {
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
            let across = |decision: CcDecision, value: T| {
                decide_across_reported([decision.name(), decision.floor()], value, swing, band)
                    .map(|(sign, margin)| Decided { sign, margin })
                    .map_err(SectionError::Escalated)
            };
            let gap = parallel_cylinder_gap_reported((r1, r2), at.d, swing, band)
                .map(|(sign, margin)| Decided { sign, margin })
                .map_err(SectionError::Escalated)?;
            // The inner level. Walls of one radius read it as their
            // axes' distance against zero, whose `Zero` is one surface;
            // walls of two radii as `d − |r₁ − r₂|`, whose `Zero` is a
            // tangent ruling inside and whose `Negative` nests them.
            let nest = match gap.sign {
                Sign::Positive => {
                    if read(CcDecision::RadiusEquality, r1 - r2, band)?.sign == Sign::Zero {
                        let coaxial = across(CcDecision::Coaxial, at.d)?;
                        if coaxial.sign == Sign::Zero {
                            return Err(SectionError::CoincidentSurfaces);
                        }
                        Some(coaxial)
                    } else {
                        Some(across(CcDecision::ParallelNest, at.d - (r1 - r2).abs())?)
                    }
                }
                Sign::Zero | Sign::Negative => None,
            };
            let stance = CcStance::Parallel {
                foot: at.foot1,
                across: at.d_vec,
                gap,
                nest,
            };
            return Ok((stance, RegionPlacement::Straddles));
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
                        // itself, within the zero band of the nest's
                        // `Zero`: definite only under a band whose
                        // escalation edge stands within twice its zero,
                        // which `Band::new` admits.
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
        let placed = placement(
            crossing_ball(foot, normal, sin, gap, [r1, r2]),
            reach.ball(),
            band,
        );
        Ok((stance, placed))
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

    /// **The ball the march ends at**: the region, which holds every
    /// section point the operation can use. The march's domain is its
    /// cube, and the ruling sweep cuts the section by the ball itself.
    pub fn ball(&self) -> ExtentBall<T> {
        self.region
    }

    /// **The thinner wall's rulings against the other** ([`RuledCylinders`]),
    /// where the axes cross. Which wall is ruled is a selection on
    /// `r₁ − r₂`, not a decision: either wall's sweep counts the section,
    /// the thinner's over fewer branch points.
    pub(crate) fn ruled(&self) -> Option<RuledCylinders<T>> {
        let CcStance::Crossing {
            foot, normal, gap, ..
        } = self.stance
        else {
            return None;
        };
        let [r1, r2] = self.radii;
        let [a1, a2] = self.axes();
        let first = r1 - r2;
        let pick = |when_first: T, when_second: T| first.select_le_zero(when_first, when_second);
        let pick3 =
            |p: Vec3<T>, q: Vec3<T>| Vec3::new(pick(p.x, q.x), pick(p.y, q.y), pick(p.z, q.z));
        let other = foot + normal * gap;
        let foot = Point3::new(
            pick(foot.x, other.x),
            pick(foot.y, other.y),
            pick(foot.z, other.z),
        );
        let (axis, partner) = (pick3(a1, a2), pick3(a2, a1));
        let v = axis.cross(normal);
        Some(RuledCylinders {
            foot,
            n: normal,
            v,
            axis,
            rho: pick(r1, r2),
            r: pick(r2, r1),
            gap: pick(gap, T::zero() - gap),
            cos: axis.dot(partner),
            beta: partner.dot(v),
        })
    }

    /// Where the pair's own ball stands against the region; parallel
    /// walls have none, and the region always cuts their rulings.
    pub fn placed(&self) -> RegionPlacement {
        self.placed
    }

    /// Whether the region cuts the section, so that a component may end
    /// on the domain's boundary.
    pub fn cut(&self) -> bool {
        self.placed != RegionPlacement::Inside
    }

    /// **The parallel pose's section in closed form** ([`parallel_rulings`]),
    /// where the axes are parallel: the one reading of the gap and the
    /// nest ladder, which [`cylinder_cylinder`] lists and
    /// [`crate::cylinder_cylinder_section`] mints.
    pub(crate) fn parallel_section(&self) -> Option<ParallelSection<T>> {
        let CcStance::Parallel {
            foot,
            across,
            gap,
            nest,
        } = self.stance
        else {
            return None;
        };
        let [p, q] = parallel_rulings(foot, across, self.lines[0].1, self.radii);
        Some(match (gap.sign, nest.map(|n| n.sign)) {
            (Sign::Negative, _) | (Sign::Positive, Some(Sign::Negative)) => ParallelSection::Empty,
            (Sign::Zero, _) | (Sign::Positive, Some(Sign::Zero)) => {
                ParallelSection::Tangent(p + (q - p) * T::from_f64(0.5))
            }
            (Sign::Positive, _) => ParallelSection::Rulings([p, q]),
        })
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

/// **Where `ball` stands against `region`** ([`RegionPlacement`]), the one
/// reading every listing of the pair takes: the crossing loops are
/// counted where the region holds the pair's ball, none where it is
/// clear of it, and marched where it cuts it; a closed-form component
/// is dropped where its own ball is clear of the region, and carried
/// whole, flagged cut, where the region cuts it. A choice of listing,
/// not a decision about the pair, so it reads the margins' signs without
/// recording them.
fn placement<T: Decide>(ball: ExtentBall<T>, region: ExtentBall<T>, band: Band) -> RegionPlacement {
    let apart = (ball.center() - region.center()).norm();
    let positive = |m: T| {
        matches!(
            m.sign_within(band),
            Ok(Decided {
                sign: Sign::Positive,
                ..
            })
        )
    };
    if positive(region.radius() - (apart + ball.radius())) {
        RegionPlacement::Inside
    } else if positive(apart - (region.radius() + ball.radius())) {
        RegionPlacement::Outside
    } else {
        RegionPlacement::Straddles
    }
}

/// A closed conic's ball: its centre, out to its larger semi-axis.
fn conic_ball<T: Real>(curve: &Curve3<T>) -> Option<ExtentBall<T>> {
    match *curve {
        Curve3::Ellipse {
            center,
            semi_u,
            semi_v,
            ..
        } => Some(ExtentBall::new(
            center,
            Curve3::semi_axes_by_size(semi_u, semi_v).0,
        )),
        Curve3::Circle { center, radius, .. } => Some(ExtentBall::new(center, radius.abs())),
        Curve3::Line { .. } | Curve3::Spiric { .. } | Curve3::Nurbs(_) => None,
    }
}

/// **The parameter span of the line `origin + t·dir`** (`dir` unit)
/// inside `region`, `None` where the line is clear of it.
fn region_span<T: Decide>(
    region: ExtentBall<T>,
    origin: Point3<T>,
    dir: Vec3<T>,
    band: Band,
) -> Option<(T, T)> {
    let to = region.center() - origin;
    let t = to.dot(dir);
    let off = (to.norm_squared() - t.powi(2)).max(T::zero()).sqrt();
    let foot = ExtentBall::point(origin + dir * t);
    if placement(foot, region, band) == RegionPlacement::Outside {
        return None;
    }
    let half = (region.radius().powi(2) - off.powi(2))
        .max(T::zero())
        .sqrt();
    Some((t - half, t + half))
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

/// **The cylinder pair's section over its decided pose** (module docs),
/// listed over the domain ([`placement`]): a closed form clear of the
/// region is not listed, and one the region cuts is carried whole, its
/// carrier exact past the region, flagged cut; the crossing loops are
/// counted where the region holds the pair's ball, none where it is
/// clear of it, and marched where it cuts it.
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
    let component = SectionComponent::closed_form;
    let cut = ComponentShape::Open {
        ends: ComponentEnds::cut(),
    };
    // A closed form's own ball against the region; inside throughout
    // where the region holds the pair's ball.
    let against = |ball: ExtentBall<T>| match pose.placed {
        RegionPlacement::Inside => RegionPlacement::Inside,
        RegionPlacement::Outside | RegionPlacement::Straddles => placement(ball, pose.region, band),
    };
    // A ruling through `p`, cut to the region, where it reaches it.
    let ruling = |p: Point3<T>| {
        region_span(pose.region, p, axis, band).map(|span| {
            let line = Curve3::Line {
                origin: p,
                dir: axis,
            };
            let witness = line.eval((span.0 + span.1) * T::from_f64(0.5));
            (line, span, witness)
        })
    };
    let components = match pose.stance {
        CcStance::Parallel { .. } => Components::Counted(match pose.parallel_section() {
            Some(ParallelSection::Rulings(points)) => points
                .into_iter()
                .filter_map(ruling)
                .map(|(curve, span, witness)| {
                    component(
                        cut,
                        [false, false],
                        Some(witness),
                        ClassCarrier::Span { curve, span },
                    )
                })
                .collect(),
            Some(ParallelSection::Tangent(p)) => ruling(p)
                .map(|(_, _, witness)| {
                    component(
                        cut,
                        [false, false],
                        Some(witness),
                        ClassCarrier::Unminted(TANGENT_COMPONENT),
                    )
                })
                .into_iter()
                .collect(),
            Some(ParallelSection::Empty) | None => Vec::new(),
        }),
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
            (Sign::Zero, _) => {
                let at = pose.touch(foot, normal, gap);
                Components::Counted(match against(ExtentBall::point(at)) {
                    RegionPlacement::Outside => Vec::new(),
                    RegionPlacement::Inside | RegionPlacement::Straddles => vec![component(
                        ComponentShape::Point,
                        [false, false],
                        Some(at),
                        ClassCarrier::Unminted(TANGENT_COMPONENT),
                    )],
                })
            }
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
                    let mut listed = Vec::new();
                    for (curve, span, [from, to]) in arcs {
                        let placed = conic_ball(curve).map_or(RegionPlacement::Straddles, &against);
                        if placed == RegionPlacement::Outside {
                            continue;
                        }
                        listed.push(component(
                            ComponentShape::Open {
                                ends: ComponentEnds {
                                    cut: placed == RegionPlacement::Straddles,
                                    ..ComponentEnds::nodes(from, to)
                                },
                            },
                            // The two ellipses joined at their nodes,
                            // each a plane section round both axes.
                            [true, true],
                            Some(curve.eval((span.0 + span.1) * T::from_f64(0.5))),
                            ClassCarrier::Span {
                                curve: curve.clone(),
                                span,
                            },
                        ));
                    }
                    Components::Counted(listed)
                }
                Some(CcNode::Pinch) | None => Components::Counted(match pose.placed {
                    RegionPlacement::Outside => Vec::new(),
                    placed @ (RegionPlacement::Inside | RegionPlacement::Straddles) => (0..2)
                        .map(|_| {
                            component(
                                ComponentShape::Open {
                                    ends: ComponentEnds {
                                        cut: placed == RegionPlacement::Straddles,
                                        ..ComponentEnds::nodes(0, 0)
                                    },
                                },
                                [false, false],
                                None,
                                ClassCarrier::Unminted(PINCH),
                            )
                        })
                        .collect(),
                }),
            },
            // One loop or two: the ruling sweep counts them and cuts them
            // by the region, the ball the march ends at.
            _ => Components::Counted(match pose.placed {
                RegionPlacement::Outside => Vec::new(),
                RegionPlacement::Inside | RegionPlacement::Straddles => {
                    let ruled = pose.ruled().unwrap_or_else(|| {
                        unreachable!("a crossing stance's pose reads its ruled side")
                    });
                    let swept = ruling::sweep(&ruled, pose.region, band).map_err(|e| match e {
                        SweepRefusal::Escalated(diag) => SectionError::Escalated(diag),
                        SweepRefusal::Inconsistent(cause) => SectionError::Unaccounted {
                            cause: cause.to_owned(),
                        },
                    })?;
                    debug_assert_eq!(
                        swept.loops,
                        if nest.map(|n| n.sign) == Some(Sign::Positive) {
                            1
                        } else {
                            2
                        },
                        "the ruling sweep's arcs of a positive discriminant are the loops the \
                         pose's nest margin counts: one arc bounded by two branch points where \
                         the walls cross in one loop, the whole turn where the thinner passes \
                         through the fatter"
                    );
                    let encircling = match thinner {
                        Some(Sign::Negative) => [true, false],
                        _ => [false, true],
                    };
                    swept
                        .components
                        .into_iter()
                        .map(|c| SectionComponent {
                            cut_ends: c.cut_ends,
                            ..component(
                                match c.cut_ends {
                                    Some(_) => cut,
                                    None => ComponentShape::Loop,
                                },
                                if c.encircles {
                                    encircling
                                } else {
                                    [false, false]
                                },
                                Some(c.witness),
                                ClassCarrier::General,
                            )
                        })
                        .collect()
                }
            }),
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
