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

/// A pair `[the plane's, the wall's]` in the order the pose was asked of.
fn ordered<X>(plane_first: bool, [on_plane, on_wall]: [X; 2]) -> [X; 2] {
    if plane_first {
        [on_plane, on_wall]
    } else {
        [on_wall, on_plane]
    }
}

/// A closed conic, one loop of the section, witnessed where `witnessed`
/// at its parameter zero.
fn ring<T: Decide>(curve: Curve3<T>, winds: [bool; 2], witnessed: bool) -> SectionComponent<T> {
    let witness = witnessed.then(|| curve.eval(T::zero()));
    SectionComponent::closed_form(
        ComponentShape::Loop,
        winds,
        witness,
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
    row: (&'static str, Decided),
}

impl<T: Decide> PlaneSpherePose<T> {
    /// **Decides the pose of `a` and `b`**, a plane and a sphere in
    /// either order ([`crate::plane_sphere_section`]). `None` for any
    /// other pair.
    ///
    /// # Errors
    ///
    /// The door's: the centre gap in band.
    pub fn decide(
        a: &Surface<T>,
        b: &Surface<T>,
        band: Band,
    ) -> Option<Result<Self, SectionError>> {
        let (plane, sphere, plane_first) = match (a, b) {
            (Surface::Plane { .. }, Surface::Sphere { .. }) => (a, b, true),
            (Surface::Sphere { .. }, Surface::Plane { .. }) => (b, a, false),
            _ => return None,
        };
        Some(
            plane_sphere_section_read(plane, sphere, band).map(|(section, row)| Self {
                plane: plane.clone(),
                sphere: sphere.clone(),
                plane_first,
                section,
                row,
            }),
        )
    }

    /// The two surfaces in the order the pose was asked of.
    pub fn operands(&self) -> (&Surface<T>, &Surface<T>) {
        let [a, b] = ordered(self.plane_first, [&self.plane, &self.sphere]);
        (a, b)
    }

    /// The sphere's ball, which holds the section.
    pub fn ball(&self) -> ExtentBall<T> {
        super::side_ball(&self.sphere)
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
    pub fn decide(
        a: &Surface<T>,
        b: &Surface<T>,
        band: Band,
    ) -> Option<Result<Self, SectionError>> {
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
        super::side_ball(&self.first)
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
    tangent: Option<(&'static str, MarginDiag)>,
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
        let [a, b] = ordered(self.plane_first, [&self.plane, &self.cylinder]);
        (a, b)
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
        PlaneSphereSection::Circle(c) => (vec![ring(c.clone(), [false, false], true)], None),
        &PlaneSphereSection::TangentPoint(at) => (
            vec![touch(at)],
            Some(tangency(pose.row.0, pose.row.1.margin)),
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
        SphereSphereSection::Circle(c) => (vec![ring(c.clone(), [false, false], true)], None),
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
/// span of its line the region holds, where it reaches it. A loop winds
/// about the wall and names no witness: it is cleared on the wall's
/// face by its winding (W2), never by a point read in a face's trim
/// (`work/inside/a-seamless-bands-trim-reads-a-point-inside-it-out.md`).
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
        (placement(
            super::cylinders::Occupant::ball(super::cylinders::closed_form_ball(c)),
            pose.region,
            band,
        ) != RegionPlacement::Outside)
            .then(|| ring(c.clone(), ordered(pose.plane_first, [false, true]), false))
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
    let decided = pose.tangent.map(|(row, margin)| tangency(row, margin));
    PairSection {
        lever: crate::uniform_curvature_arm(&pose.cylinder),
        pose: PairPose::PlaneCylinder(pose),
        decided: decided.into_iter().collect(),
        components: Components::Counted(components),
        nodes: Vec::new(),
        node_sets: Vec::new(),
    }
}

#[cfg(test)]
#[allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]
mod rows {
    use geom::{Curve3, Frame, Surface};
    use geom_core::{Band, NoLane, Point3, Tol, Vec3};

    use super::super::{
        ClassCarrier, ComponentShape, PairSection, SectionComponent, SectionRead, pair_section,
    };
    use crate::ExtentBall;
    use crate::intersect::SectionError;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn plane(at: Point3<f64>, n: Vec3<f64>) -> Surface<f64> {
        let n = n.normalize();
        let (u, _) = n.orthonormal_basis();
        Surface::Plane {
            frame: Frame::new(at, n, u),
        }
    }

    fn sphere(c: Point3<f64>, radius: f64) -> Surface<f64> {
        Surface::Sphere {
            frame: Frame::new(c, Vec3::unit_z(), Vec3::unit_x()),
            radius,
        }
    }

    fn wall() -> Surface<f64> {
        Surface::Cylinder {
            frame: Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
            radius: 1.0,
        }
    }

    fn read(
        a: &Surface<f64>,
        b: &Surface<f64>,
        region: Option<ExtentBall<f64>>,
    ) -> Result<PairSection<f64>, SectionError> {
        match pair_section(a, b, region, None::<NoLane>, band())? {
            SectionRead::Built(s) => Ok(s),
            SectionRead::Elsewhere(route) => panic!("{route:?}"),
        }
    }

    fn listed(s: &PairSection<f64>) -> &[SectionComponent<f64>] {
        s.counted().expect("a closed form counts its components")
    }

    /// The tangency the section decided, by its row.
    fn tangent_row(s: &PairSection<f64>) -> Option<&'static str> {
        s.tangency().map(|z| z.decision.name())
    }

    /// **A crossing is one closed loop, a tangency its touch point, and
    /// clear is nothing**, plane × sphere in both orders and sphere ×
    /// sphere outside and inside: the loop's carrier is the circle whole,
    /// its witness on both surfaces; the touch has no carrier and its row
    /// is the decided tangency.
    #[test]
    fn a_plane_or_a_sphere_meets_a_sphere_in_a_loop_a_touch_or_nothing() {
        let ball = sphere(Point3::new(0.1, -0.2, 0.3), 1.0);
        let k = Vec3::new(0.3, -0.4, 0.5).normalize();
        let c = Point3::new(0.1, -0.2, 0.3);
        let cases = [
            ("plane crossing", plane(c + k * 0.6, k), Some(None)),
            (
                "plane touching",
                plane(c + k, k),
                Some(Some("ps_center_gap")),
            ),
            ("plane clear", plane(c + k * 1.5, k), None),
            ("sphere crossing", sphere(c + k * 1.2, 0.5), Some(None)),
            (
                "sphere touching outside",
                sphere(c + k * 1.5, 0.5),
                Some(Some("ss_carrier_external")),
            ),
            (
                "sphere touching inside",
                sphere(c + k * 0.5, 0.5),
                Some(Some("ss_carrier_internal")),
            ),
            ("sphere clear", sphere(c + k * 3.0, 0.5), None),
            ("sphere nested", sphere(c + k * 0.1, 0.5), None),
        ];
        for (label, other, want) in cases {
            for (order, a, b) in [("first", &ball, &other), ("second", &other, &ball)] {
                let at = format!("{label}, the ball {order}");
                let s = read(a, b, None).unwrap_or_else(|e| panic!("{at}: {e:?}"));
                let (oa, ob) = s.pose().operands();
                assert_eq!(
                    (format!("{oa:?}"), format!("{ob:?}")),
                    (format!("{a:?}"), format!("{b:?}")),
                    "{at}: the pose's operands are in the asked order"
                );
                match (listed(&s), want) {
                    ([], None) => assert_eq!(tangent_row(&s), None, "{at}"),
                    ([loop_], Some(None)) => {
                        assert_eq!(loop_.shape, ComponentShape::Loop, "{at}");
                        assert!(
                            matches!(loop_.carrier, ClassCarrier::Closed(Curve3::Circle(_))),
                            "{at}: {:?}",
                            loop_.carrier
                        );
                        let w = loop_.witness.expect("a closed form's witness");
                        for s in [a, b] {
                            let r = crate::implicit_residual(s, w);
                            assert!(r.abs() < 1e-12, "{at}: the witness is {r} off {s:?}");
                        }
                        assert_eq!(tangent_row(&s), None, "{at}");
                    }
                    ([touch], Some(Some(row))) => {
                        assert_eq!(touch.shape, ComponentShape::Point, "{at}");
                        assert!(
                            matches!(touch.carrier, ClassCarrier::Unminted(_)),
                            "{at}: a touch mints no carrier"
                        );
                        assert_eq!(tangent_row(&s), Some(row), "{at}");
                    }
                    (got, _) => panic!("{at}: {got:?}"),
                }
            }
        }
    }

    /// **A plane cuts a cylinder over the region alone**: a tilted plane
    /// is one ellipse that winds about the cylinder, listed while the
    /// region reaches it and not once the region stands clear; a plane
    /// holding the axis is two rulings, each the span of its line inside
    /// the region; a plane touching the wall along a ruling is that
    /// ruling cut by the region, with no carrier and the gap row its
    /// tangency; and with no region the pair refuses.
    #[test]
    fn a_plane_cuts_a_cylinder_over_the_region_alone() {
        let near = ExtentBall::fixture(Point3::new(0.0, 0.0, 0.5), 3.0);
        let tilted = plane(Point3::new(0.0, 0.0, 0.5), Vec3::new(0.4, 0.1, 1.0));
        for (order, a, b, winds) in [
            ("plane first", &tilted, &wall(), [false, true]),
            ("wall first", &wall(), &tilted, [true, false]),
        ] {
            let s = read(a, b, Some(near)).unwrap();
            let [ellipse] = listed(&s) else {
                panic!("{order}: {:?}", listed(&s))
            };
            assert!(
                matches!(ellipse.carrier, ClassCarrier::Closed(Curve3::Ellipse(_))),
                "{order}: {:?}",
                ellipse.carrier
            );
            assert_eq!(ellipse.winds, winds, "{order}: it winds about the cylinder");
            let far = ExtentBall::fixture(Point3::new(0.0, 0.0, 40.0), 3.0);
            assert!(
                listed(&read(a, b, Some(far)).unwrap()).is_empty(),
                "{order}: a region clear of the ellipse lists nothing"
            );
        }
        let across = ExtentBall::fixture(Point3::new(2.0, 0.0, 0.5), 1.5);
        assert_eq!(
            listed(&read(&tilted, &wall(), Some(across)).unwrap()).len(),
            1,
            "a region across the ellipse's ball lists it"
        );
        let holding = plane(Point3::new(0.5, 0.0, 0.0), Vec3::unit_x());
        let s = read(&holding, &wall(), Some(near)).unwrap();
        let rulings = listed(&s);
        assert_eq!(rulings.len(), 2, "{rulings:?}");
        for r in rulings {
            let ClassCarrier::Span { curve, span } = &r.carrier else {
                panic!("a ruling is a span: {:?}", r.carrier)
            };
            assert!(
                matches!(r.shape, ComponentShape::Open { ends } if ends.cut),
                "a ruling is cut by the region"
            );
            for t in [span.0, span.1] {
                let reach = (curve.eval(t) - near.center()).norm();
                assert!(
                    (reach - near.radius()).abs() < 1e-9,
                    "the span ends on the region's boundary: {reach}"
                );
            }
        }
        let touching = plane(Point3::new(1.0, 0.0, 0.0), Vec3::unit_x());
        let s = read(&wall(), &touching, Some(near)).unwrap();
        let [line] = listed(&s) else {
            panic!("{:?}", listed(&s))
        };
        assert!(matches!(line.carrier, ClassCarrier::Unminted(_)));
        assert_eq!(tangent_row(&s), Some("pc_parallel_gap"));
        assert!(matches!(
            read(&tilted, &wall(), None),
            Err(SectionError::NoRegion { .. })
        ));
    }

    /// **A closed-form section carries with no march**: every component
    /// of a crossing plane × sphere and plane × cylinder section carries
    /// as classified, with no lane.
    #[test]
    fn a_conic_pair_carries_with_no_lane() {
        let near = ExtentBall::fixture(Point3::new(0.0, 0.0, 0.5), 3.0);
        for (label, a, b) in [
            (
                "plane × sphere",
                plane(Point3::new(0.0, 0.0, 0.3), Vec3::unit_z()),
                sphere(Point3::origin(), 1.0),
            ),
            (
                "plane × cylinder",
                plane(Point3::new(0.0, 0.0, 0.5), Vec3::new(0.4, 0.1, 1.0)),
                wall(),
            ),
            (
                "rulings",
                plane(Point3::new(0.5, 0.0, 0.0), Vec3::unit_x()),
                wall(),
            ),
        ] {
            let s = read(&a, &b, Some(near)).unwrap();
            let carried = s
                .carry(None::<NoLane>, band())
                .unwrap_or_else(|e| panic!("{label}: {e:?}"));
            assert_eq!(
                carried.components().len(),
                listed(&s).len(),
                "{label}: every component carries"
            );
        }
    }

    /// **The tilt is levered across the region, not the radius**: a
    /// plane whose normal leans off square to the unit wall's axis by
    /// `c = 3ε/10` reads `c` times the lever, in band across a region of
    /// radius 10 about the axis (`3ε` to `6ε`), where levered at the
    /// radius it would read `0.3ε` and decide the axis in the plane.
    #[test]
    fn the_tilt_is_levered_across_the_region() {
        let eps = Tol::witness().eps();
        let c = 3.0 * eps / 10.0;
        let leaning = plane(
            Point3::new(0.5, 0.0, 0.0),
            Vec3::new((1.0 - c * c).sqrt(), 0.0, c),
        );
        let region = ExtentBall::fixture(Point3::origin(), 10.0);
        match read(&leaning, &wall(), Some(region)) {
            Err(SectionError::Escalated(diag)) => {
                assert_eq!(diag.predicate, Some("pc_axis_plane_parallel"));
            }
            other => panic!("the tilt reads in band across the region: {other:?}"),
        }
    }
}
