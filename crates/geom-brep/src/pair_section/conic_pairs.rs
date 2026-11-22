//! **Plane × sphere, sphere × sphere and plane × cylinder on the section
//! value**: the pairs whose section is a conic or a pair of lines in
//! closed form, each pose read once through the pair's own door
//! ([`crate::plane_sphere_section`], [`crate::sphere_sphere_section`],
//! [`crate::plane_cylinder_section`]) and its components listed from it.
//!
//! A circle or a tilted ellipse is one loop, the whole of its closed
//! curve ([`ClassCarrier::Closed`]). A sphere side bounds its pair, so
//! the two sphere pairs are read over a sphere's own ball and never the
//! region. A plane has no bounded side, so plane × cylinder is read over
//! the operation's region alone (C3): its tilt row is levered across the
//! region ([`Reach::Ball`]), a loop clear of the region is not listed,
//! and its rulings are cut by the region as two parallel walls' are
//! ([`super::cylinders`]), each a line over the span the region holds.
//! A tangency is classification data: the touch point, or the tangent
//! ruling cut by the region, with no carrier ([`NoCarrier::Tangent`]),
//! and the row that decided it `Zero` among the section's verdicts.

use geom::{Curve3, Surface};
use geom_core::{Band, Decide, Decided, MarginDiag, Real};

use super::cylinders::{RegionPlacement, placement, region_span};
use super::{
    ClassCarrier, ComponentEnds, ComponentShape, Components, NoCarrier, PairPose, PairSection,
    SectionComponent, SectionDecision, ZeroVerdict,
};
use crate::ExtentBall;
use crate::extent::Reach;
use crate::intersect::{
    PlaneCylinderSection, PlaneSphereSection, SectionError, SphereSphereSection,
    plane_cylinder_classified, plane_sphere_section_read, sphere_sphere_section_read,
};

/// **The tangency a pose's row decided**, where it read `Zero`.
fn tangency(row: &'static str, margin: MarginDiag) -> ZeroVerdict {
    ZeroVerdict {
        decision: SectionDecision::Tangency(row),
        margin,
    }
}

/// `[first, second]` in the order the pose was asked of, `first` the
/// plane's (or the first sphere's).
fn ordered(plane_first: bool, [on_plane, on_wall]: [bool; 2]) -> [bool; 2] {
    if plane_first {
        [on_plane, on_wall]
    } else {
        [on_wall, on_plane]
    }
}

/// A closed conic, one loop of the section.
fn ring<T: Decide>(curve: Curve3<T>, winds: [bool; 2]) -> SectionComponent<T> {
    let witness = curve.eval(T::zero());
    SectionComponent::closed_form(
        ComponentShape::Loop,
        winds,
        Some(witness),
        ClassCarrier::Closed(curve),
    )
}

/// A tangent point, classification data.
fn touch<T: Real>(at: geom_core::Point3<T>) -> SectionComponent<T> {
    SectionComponent::closed_form(
        ComponentShape::Point,
        [false, false],
        Some(at),
        ClassCarrier::Unminted(NoCarrier::Tangent),
    )
}

/// **A plane and a sphere, their pose decided** by `ps_center_gap`.
#[derive(Clone, Debug)]
pub struct PlaneSpherePose<T: Real> {
    plane: Surface<T>,
    sphere: Surface<T>,
    plane_first: bool,
    section: PlaneSphereSection<T>,
    gap: Decided,
}

impl<T: Decide> PlaneSpherePose<T> {
    /// **Decides the pose of `a` and `b`**, a plane and a sphere in
    /// either order ([`crate::plane_sphere_section`]). `None` for any
    /// other pair.
    ///
    /// # Errors
    ///
    /// The door's: the centre gap in band.
    pub fn decide(a: &Surface<T>, b: &Surface<T>, band: Band) -> Option<Result<Self, SectionError>> {
        let (plane, sphere, plane_first) = match (a, b) {
            (Surface::Plane { .. }, Surface::Sphere { .. }) => (a, b, true),
            (Surface::Sphere { .. }, Surface::Plane { .. }) => (b, a, false),
            _ => return None,
        };
        Some(
            plane_sphere_section_read(plane, sphere, band).map(|(section, gap)| Self {
                plane: plane.clone(),
                sphere: sphere.clone(),
                plane_first,
                section,
                gap,
            }),
        )
    }

    /// The two surfaces in the order the pose was asked of.
    pub fn operands(&self) -> (&Surface<T>, &Surface<T>) {
        if self.plane_first {
            (&self.plane, &self.sphere)
        } else {
            (&self.sphere, &self.plane)
        }
    }

    /// The sphere's ball, which holds the section.
    pub fn ball(&self) -> ExtentBall<T> {
        ExtentBall::of_covered(self.sphere.cover().as_ref().ok())
    }

    /// The section as the door classified it.
    pub fn section(&self) -> &PlaneSphereSection<T> {
        &self.section
    }
}

/// **Two spheres, their pose decided** by the door's carrier rows.
#[derive(Clone, Debug)]
pub struct SphereSpherePose<T: Real> {
    first: Surface<T>,
    second: Surface<T>,
    section: SphereSphereSection<T>,
    row: (&'static str, Decided),
}

impl<T: Decide> SphereSpherePose<T> {
    /// **Decides the pose of `a` and `b`**, two spheres
    /// ([`crate::sphere_sphere_section`]). `None` for any other pair.
    ///
    /// # Errors
    ///
    /// The door's: a row in band, and two spheres decided one
    /// ([`SectionError::CoincidentSurfaces`]).
    pub fn decide(a: &Surface<T>, b: &Surface<T>, band: Band) -> Option<Result<Self, SectionError>> {
        let (Surface::Sphere { .. }, Surface::Sphere { .. }) = (a, b) else {
            return None;
        };
        Some(
            sphere_sphere_section_read(a, b, band).map(|(section, row)| Self {
                first: a.clone(),
                second: b.clone(),
                section,
                row,
            }),
        )
    }

    /// The two surfaces in the order the pose was asked of.
    pub fn operands(&self) -> (&Surface<T>, &Surface<T>) {
        (&self.first, &self.second)
    }

    /// The first sphere's ball, which holds the section, as the second's
    /// does.
    pub fn ball(&self) -> ExtentBall<T> {
        ExtentBall::of_covered(self.first.cover().as_ref().ok())
    }

    /// The section as the door classified it.
    pub fn section(&self) -> &SphereSphereSection<T> {
        &self.section
    }
}

/// **A plane and a cylinder, their pose decided across the operation's
/// region** ([`crate::plane_cylinder_section`] at [`Reach::Ball`]).
#[derive(Clone, Debug)]
pub struct PlaneCylinderPose<T: Real> {
    plane: Surface<T>,
    cylinder: Surface<T>,
    plane_first: bool,
    region: ExtentBall<T>,
    section: PlaneCylinderSection<T>,
    tangent: Option<MarginDiag>,
}

impl<T: Decide> PlaneCylinderPose<T> {
    /// **Decides the pose of `a` and `b`**, a plane and a cylinder in
    /// either order, across `region`. `None` for any other pair.
    ///
    /// # Errors
    ///
    /// The door's: a row in band, a radius not definitely positive, an
    /// ellipse the constructor refuses.
    pub fn decide(
        a: &Surface<T>,
        b: &Surface<T>,
        region: ExtentBall<T>,
        band: Band,
    ) -> Option<Result<Self, SectionError>> {
        let (plane, cylinder, plane_first) = match (a, b) {
            (Surface::Plane { .. }, Surface::Cylinder { .. }) => (a, b, true),
            (Surface::Cylinder { .. }, Surface::Plane { .. }) => (b, a, false),
            _ => return None,
        };
        Some(
            plane_cylinder_classified(plane, cylinder, &Reach::Ball(region), band).map(
                |(section, _, tangent)| Self {
                    plane: plane.clone(),
                    cylinder: cylinder.clone(),
                    plane_first,
                    region,
                    section,
                    tangent,
                },
            ),
        )
    }

    /// The two surfaces in the order the pose was asked of.
    pub fn operands(&self) -> (&Surface<T>, &Surface<T>) {
        if self.plane_first {
            (&self.plane, &self.cylinder)
        } else {
            (&self.cylinder, &self.plane)
        }
    }

    /// **The region the pose was read across**, which holds every
    /// section point the operation can use.
    pub fn region(&self) -> ExtentBall<T> {
        self.region
    }

    /// The section as the door classified it.
    pub fn section(&self) -> &PlaneCylinderSection<T> {
        &self.section
    }
}

/// The plane × sphere arm over its decided pose.
pub(super) fn plane_sphere<T: Decide>(pose: PlaneSpherePose<T>) -> PairSection<T> {
    let (components, decided) = match &pose.section {
        PlaneSphereSection::Circle(c) => (vec![ring(c.clone(), [false, false])], None),
        &PlaneSphereSection::TangentPoint(at) => (
            vec![touch(at)],
            Some(tangency("ps_center_gap", pose.gap.margin)),
        ),
        PlaneSphereSection::Empty => (Vec::new(), None),
    };
    PairSection {
        lever: crate::uniform_curvature_arm(&pose.sphere),
        pose: PairPose::PlaneSphere(pose),
        decided: decided.into_iter().collect(),
        components: Components::Counted(components),
        nodes: Vec::new(),
        node_sets: Vec::new(),
    }
}

/// The sphere × sphere arm over its decided pose.
pub(super) fn sphere_sphere<T: Decide>(pose: SphereSpherePose<T>) -> PairSection<T> {
    let (row, read) = pose.row;
    let (components, decided) = match &pose.section {
        SphereSphereSection::Circle(c) => (vec![ring(c.clone(), [false, false])], None),
        &SphereSphereSection::TangentPoint(at) => {
            (vec![touch(at)], Some(tangency(row, read.margin)))
        }
        SphereSphereSection::Empty => (Vec::new(), None),
    };
    PairSection {
        lever: crate::uniform_curvature_arm(&pose.first)
            .min(crate::uniform_curvature_arm(&pose.second)),
        pose: PairPose::SphereSphere(pose),
        decided: decided.into_iter().collect(),
        components: Components::Counted(components),
        nodes: Vec::new(),
        node_sets: Vec::new(),
    }
}

/// **The plane × cylinder arm over its decided pose**, listed over the
/// region: a loop clear of it is not listed, and each ruling is the
/// span of its line the region holds, where it reaches it.
pub(super) fn plane_cylinder<T: Decide>(pose: PlaneCylinderPose<T>, band: Band) -> PairSection<T> {
    let cut = ComponentShape::Open {
        ends: ComponentEnds::cut(),
    };
    let ruling = |line: &Curve3<T>, carrier: &dyn Fn(Curve3<T>, (T, T)) -> ClassCarrier<T>| {
        let &Curve3::Line(l) = line else {
            unreachable!("the plane × cylinder door's rulings are lines")
        };
        region_span(pose.region, l, band).map(|span| {
            let witness = line.eval(geom::mid_param(span.0, span.1));
            SectionComponent::closed_form(
                cut,
                [false, false],
                Some(witness),
                carrier(line.clone(), span),
            )
        })
    };
    let spanned = |curve, span| ClassCarrier::Span { curve, span };
    let tangent = |_, _| ClassCarrier::Unminted(NoCarrier::Tangent);
    let loop_of = |c: &Curve3<T>| {
        let ball = ExtentBall::of_covered(c.cover().as_ref().ok());
        (placement(ball, pose.region, band) != RegionPlacement::Outside)
            .then(|| ring(c.clone(), ordered(pose.plane_first, [false, true])))
    };
    let components = match &pose.section {
        PlaneCylinderSection::TiltedEllipse(c) | PlaneCylinderSection::Rim(c) => {
            loop_of(c).into_iter().collect()
        }
        PlaneCylinderSection::ParallelLines { l1, l2 } => [l1, l2]
            .into_iter()
            .filter_map(|l| ruling(l, &spanned))
            .collect(),
        PlaneCylinderSection::TangentLine(l) => ruling(l, &tangent).into_iter().collect(),
        PlaneCylinderSection::Empty => Vec::new(),
    };
    let decided = pose
        .tangent
        .map(|margin| tangency("pc_parallel_gap", margin));
    PairSection {
        lever: crate::uniform_curvature_arm(&pose.cylinder),
        pose: PairPose::PlaneCylinder(pose),
        decided: decided.into_iter().collect(),
        components: Components::Counted(components),
        nodes: Vec::new(),
        node_sets: Vec::new(),
    }
}
