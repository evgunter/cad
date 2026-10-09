//! **The plane×cone germ frame** ([`pair_section_frame_at`]'s cone arm),
//! one row per outcome of the C5 table it reads
//! ([`geom_brep::plane_cone_section`]). The planes are the cone sector
//! spec's poses: B4's axis-normal slab faces and C1's tilted plane cut
//! ellipses or circles, X1's plane through the apex the generator pair,
//! and C2–C4's and B1's faces the conics outside the inventory.
//!
//! The widening cone is REACH's frustum's carrier: the radius runs
//! `0.5 → 1` over `y ∈ [0, 1]`, so the apex is `(0, −1, 0)`, the axis
//! `+y` and `tan α = 1/2`. The full cone's radius runs `1 → 0` over the
//! same span: apex `(0, 1, 0)`, axis `−y`, `α = π/4`.

#![allow(clippy::unwrap_used, clippy::panic)]

use super::BooleanError;
use super::join::{
    FrameError, FrameExtent, frame_extent, frame_reading, frame_refusal, pair_section_frame,
    pair_section_frame_at,
};
use crate::entity::FaceKey;
use geom_brep::{OutsideConic, RadiusEvidence, SectionError};
use geom_core::{Band, Point3, Tol, Vec3};

type Frame = Result<Option<(Point3<f64>, Vec3<f64>)>, FrameError>;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn widening() -> geom::Surface<f64> {
    geom::Surface::Cone {
        apex: Point3::new(0.0, -1.0, 0.0),
        axis: Vec3::new(0.0, 1.0, 0.0),
        half_angle: 0.5_f64.atan(),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn full() -> geom::Surface<f64> {
    geom::Surface::Cone {
        apex: Point3::new(0.0, 1.0, 0.0),
        axis: Vec3::new(0.0, -1.0, 0.0),
        half_angle: core::f64::consts::FRAC_PI_4,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn plane(origin: [f64; 3], normal: Vec3<f64>) -> geom::Surface<f64> {
    let normal = normal.normalize();
    geom::Surface::Plane {
        origin: Point3::new(origin[0], origin[1], origin[2]),
        normal,
        u_ref: normal,
    }
}

/// A point on the widening wall, `y = 0.5` at azimuth 0: where the frame
/// reads a frustum face's boundary centre.
fn on_wall() -> Point3<f64> {
    Point3::new(0.75, 0.5, 0.0)
}

/// The widening frustum face's reach from [`on_wall`]: the far side of
/// its top rim.
fn wall_reach() -> f64 {
    (on_wall() - Point3::new(-1.0, 1.0, 0.0)).norm()
}

/// The frame both ways round, levered at the frustum face's reach
/// ([`FrameExtent::Reach`]). The two readings must agree; the row reads
/// the first.
fn frame(p: &geom::Surface<f64>, cone: &geom::Surface<f64>, at: Point3<f64>) -> Frame {
    let read = |a, b| {
        pair_section_frame_at(
            a,
            b,
            RadiusEvidence::None,
            at,
            FrameExtent::Reach(wall_reach()),
            band(),
        )
    };
    let (first, other) = (read(p, cone), read(cone, p));
    assert_eq!(
        shape(&first),
        shape(&other),
        "the frame's member orders disagree on the outcome's shape"
    );
    first
}

/// An outcome's shape, for comparing readings: the frame's tuple, or the
/// refusal's variant and the predicate or conic it names.
fn shape(frame: &Frame) -> String {
    match frame {
        Ok(Some((c, a))) => format!("frame {c:?} {a:?}"),
        Ok(None) => "straight".into(),
        Err(FrameError::Escalated(d)) => format!("escalated {:?}", d.predicate),
        Err(FrameError::Desync(what)) => format!("desync {what}"),
        Err(FrameError::OutsideInventory { conic, section }) => format!("outside {conic:?} {section}"),
        Err(FrameError::NoArm) => "no arm".into(),
        Err(FrameError::RadiusEscalated { .. }) => "radius escalated".into(),
        Err(FrameError::IntersectingCylinderAxes { .. }) => "pinch".into(),
    }
}

/// The Dandelin construction for a plane `(q, n)` cutting the cone
/// `(A, â, α)` off its apex, as `PlaneConeSection::TiltedEllipse`'s docs
/// state it: centre `A − (δ/K)·(c·â − sin²α·n)`, with `c = â·n`,
/// `δ = (A − q)·n` and `K = c² − sin²α`.
fn dandelin_centre(cone: &geom::Surface<f64>, q: Point3<f64>, n: Vec3<f64>) -> Point3<f64> {
    let &geom::Surface::Cone {
        apex,
        axis,
        half_angle,
        ..
    } = cone
    else {
        panic!("a cone")
    };
    let n = n.normalize();
    let (c, s2) = (axis.dot(n), half_angle.sin().powi(2));
    let delta = (apex - q).dot(n);
    let k = c * c - s2;
    apex - (axis * c - n * s2) * (delta / k)
}

fn assert_conic_frame(got: &Frame, centre: Point3<f64>, normal: Vec3<f64>, what: &str) {
    let Ok(Some((c, a))) = got else {
        panic!("{what}: the conic's frame, got {}", shape(got));
    };
    assert!(
        (*c - centre).norm() < 1e-12,
        "{what}: the frame's centre {c:?} is the conic's {centre:?}"
    );
    assert!(
        a.normalize().cross(normal.normalize()).norm() < 1e-12,
        "{what}: the frame's axis {a:?} is the cutting plane's normal {normal:?}"
    );
}

/// **An ellipse or a circle is the conic's own frame.** B4's slab faces
/// `y = 0.3` and `y = 0.6` cut the widening cone in circles about the
/// axis, of radius `0.5 + 0.5·y`; C1's plane, tilted 10° off
/// axis-normal through `(0, 0.5, 0)`, in the ellipse whose centre is the
/// Dandelin construction's.
#[test]
fn an_ellipse_or_a_circle_is_the_conics_own_frame() {
    let cone = widening();
    for y in [0.3, 0.6] {
        let y_hat = Vec3::new(0.0, 1.0, 0.0);
        let got = frame(&plane([0.0, y, 0.0], y_hat), &cone, on_wall());
        assert_conic_frame(&got, Point3::new(0.0, y, 0.0), y_hat, "B4's slab face");
    }
    let tilt = 10_f64.to_radians();
    let n = Vec3::new(tilt.sin(), tilt.cos(), 0.0);
    let q = Point3::new(0.0, 0.5, 0.0);
    let got = frame(&plane([q.x, q.y, q.z], n), &cone, on_wall());
    assert_conic_frame(&got, dandelin_centre(&cone, q, n), n, "C1's tilted plane");
}

/// **A plane through the apex that dips into the cone is two
/// generators, proved straight.** X1's plane `x = 0` contains the full
/// cone's axis: `Ok(None)`, the straight-chord facing test.
#[test]
fn a_plane_through_the_apex_cuts_two_generators_and_is_straight() {
    let got = frame(
        &plane([0.0, 0.0, 0.0], Vec3::new(1.0, 0.0, 0.0)),
        &full(),
        Point3::new(0.5, 0.5, 0.0),
    );
    assert!(
        matches!(got, Ok(None)),
        "X1: the generator pair, got {}",
        shape(&got)
    );
}

/// **A parabola or a hyperbola refuses by decision, naming the conic.**
/// C2's plane `x = 0.3` and B1's faces `z = 0.2` and `x = 0.6` run along
/// the axis; C4's runs 20° off it; each meets both nappes, a hyperbola.
/// C3's plane is parallel to a generator, a parabola. The plane along the
/// axis is read once more with its stored origin level with the apex,
/// where a lever read off that origin would be zero. Each refusal is
/// [`FrameError::OutsideInventory`] carrying the table's naming, and the
/// Boolean's refusal is [`BooleanError::GermSectionOutsideInventory`]:
/// neither the missing-arm `NoArm` nor a desync.
#[test]
fn a_parabola_or_a_hyperbola_refuses_by_decision_naming_the_conic() {
    let cone = widening();
    let alpha = 0.5_f64.atan();
    let c4 = 20_f64.to_radians();
    let hyperbola = (OutsideConic::Hyperbola, "a hyperbola", "a parabola");
    let parabola = (OutsideConic::Parabola, "a parabola", "a hyperbola");
    let poses = [
        ("C2", [0.3, 0.5, 0.0], Vec3::new(1.0, 0.0, 0.0), hyperbola),
        (
            "C2, origin level with the apex",
            [0.3, -1.0, 0.0],
            Vec3::new(1.0, 0.0, 0.0),
            hyperbola,
        ),
        (
            "B1's z face",
            [0.0, 0.5, 0.2],
            Vec3::new(0.0, 0.0, 1.0),
            hyperbola,
        ),
        (
            "B1's x face",
            [0.6, 0.5, 0.0],
            Vec3::new(1.0, 0.0, 0.0),
            hyperbola,
        ),
        (
            "C4",
            [0.3, 0.5, 0.0],
            Vec3::new(c4.cos(), -c4.sin(), 0.0),
            hyperbola,
        ),
        (
            "C3",
            [0.5, 0.5, 0.0],
            Vec3::new(alpha.cos(), -alpha.sin(), 0.0),
            parabola,
        ),
    ];
    for (label, origin, n, (conic, named, other)) in poses {
        let p = plane(origin, n);
        let got = frame(&p, &cone, on_wall());
        let Err(FrameError::OutsideInventory {
            conic: got_conic,
            section: SectionError::RoutesToGeneralRung { pair, .. },
        }) = &got
        else {
            panic!(
                "{label}: {named} refuses by decision, got {}",
                shape(&got)
            );
        };
        assert_eq!(*pair, "plane×cone", "{label}: the pair the table names");
        assert_eq!(*got_conic, conic, "{label}: the conic the refusal names");
        let refused = frame_refusal(
            got.err().unwrap(),
            (FaceKey::default(), &p),
            (FaceKey::default(), &cone),
        );
        assert!(
            matches!(
                refused,
                BooleanError::GermSectionOutsideInventory {
                    a_kind: geom::SurfaceKind::Plane,
                    b_kind: geom::SurfaceKind::Cone,
                    section: SectionError::RoutesToGeneralRung { .. },
                    ..
                }
            ),
            "{label}: the Boolean's refusal names the conic, got {refused:?}"
        );
        let text = refused.to_string();
        assert!(
            text.contains(named) && !text.contains(other),
            "{label}: the refusal's text names {named} and not {other}: {text}"
        );
    }
}

/// **A plane touching the cone at its apex is not a locus.** A plane
/// through the apex along a generator (`ApexTangentLine`) and the
/// axis-normal plane through it (`ApexPoint`) are touching
/// configurations the reduction should not have paired: the desync the
/// sphere arm's tangent point is.
#[test]
fn a_plane_touching_the_apex_is_the_desync() {
    let cone = widening();
    let alpha = 0.5_f64.atan();
    for (label, n) in [
        (
            "along a generator",
            Vec3::new(alpha.cos(), -alpha.sin(), 0.0),
        ),
        ("axis-normal", Vec3::new(0.0, 1.0, 0.0)),
    ] {
        let got = frame(&plane([0.0, -1.0, 0.0], n), &cone, on_wall());
        assert!(
            matches!(got, Err(FrameError::Desync(what)) if what.contains("not a locus")),
            "{label}: the touching desync, got {}",
            shape(&got)
        );
    }
}

/// **A plane within the band of a generator's direction escalates.**
/// C3's plane turned by `θ` about `z` has the conic-type margin
/// `D = −sin θ`, levered at the extent: an ellipse for `θ > 0`. Turned
/// so the levered margin sits between the band's thresholds, the frame
/// escalates `pn_conic_type` under each extent the dispatch reads; it is
/// never snapped to the ellipse, which it serves once the margin clears
/// the band.
#[test]
fn a_near_parabola_escalates_and_is_never_snapped_to_an_ellipse() {
    let cone = widening();
    let alpha = 0.5_f64.atan();
    let b = band();
    let at = on_wall();
    let lever = wall_reach();
    let turned = |margin: f64| {
        let theta = (margin / lever).asin();
        plane(
            [0.5, 0.5, 0.0],
            Vec3::new((alpha + theta).cos(), -(alpha + theta).sin(), 0.0),
        )
    };
    let read = |p: &geom::Surface<f64>| {
        pair_section_frame_at(
            p,
            &cone,
            RadiusEvidence::None,
            at,
            FrameExtent::Reach(lever),
            b,
        )
    };
    let sliver = (b.zero() * b.escalate()).sqrt();
    let got = read(&turned(sliver));
    assert!(
        matches!(&got, Err(FrameError::Escalated(d)) if d.predicate == Some("pn_conic_type")),
        "in band, the conic type escalates, got {}",
        shape(&got)
    );
    let got = read(&turned(4.0 * b.escalate()));
    assert!(
        matches!(got, Ok(Some(_))),
        "past the band, the ellipse's frame, got {}",
        shape(&got)
    );
}

/// A cone `100 m` long to the frustum it bounds: apex `(0, −100, 0)`,
/// axis `+y`, `tan α = 1/100`, so its wall runs radius `1 → 1.01` over
/// `y ∈ [0, 1]`.
fn far_cone() -> geom::Surface<f64> {
    geom::Surface::Cone {
        apex: Point3::new(0.0, -100.0, 0.0),
        axis: Vec3::new(0.0, 1.0, 0.0),
        half_angle: 0.01_f64.atan(),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// A face of [`far_cone`] whose outer loop is the generator `(1, 0, 0)`
/// to `(1.01, 1, 0)`: it reaches half that generator from its centre,
/// a hundred metres short of the apex.
fn far_generator_wall() -> (crate::Body<f64>, FaceKey) {
    let mut body = crate::Body::<f64>::new();
    let seed = body.mvfs(Point3::new(1.0, 0.0, 0.0), true).unwrap();
    body.set_face_surface(
        seed.face,
        crate::FaceSurface::New {
            surface: far_cone(),
            sense: true,
        },
    )
    .unwrap();
    body.mev_line(
        crate::MevSite::Lone {
            r#loop: seed.r#loop,
        },
        Point3::new(1.01, 1.0, 0.0),
        Tol::witness(),
    )
    .unwrap();
    (body, seed.face)
}

/// **A frustum far from its apex is levered at its own reach.** The plane
/// through `(0.5, 0.5, 0)` parallel to [`far_cone`]'s generator, turned
/// so the conic-type margin levered at the face's reach (half the
/// generator, about `0.5`) sits in the band, reads through the
/// production extent ([`frame_extent`]) in both member orders and
/// escalates `pn_conic_type`. Levered at the reading point's distance
/// from the apex (about `100.5`), or at ten times the reach, the same
/// margin clears the band and the frame serves an ellipse the face's
/// own extent cannot decide.
#[test]
fn a_frustum_far_from_its_apex_is_levered_at_its_own_reach() {
    let (body, face) = far_generator_wall();
    let cone = far_cone();
    let alpha = 0.01_f64.atan();
    let b = band();
    let on = super::rest::face_witnesses(&body, face).unwrap();
    let reading = |a: &geom::Surface<f64>, c: &geom::Surface<f64>| {
        let (on_a, on_b) = if matches!(a, geom::Surface::Plane { .. }) {
            (Vec::new(), on.clone())
        } else {
            (on.clone(), Vec::new())
        };
        let (at, span) = frame_reading(a, c, on_a, on_b).expect("a reading");
        let extent = frame_extent((a, &body, face), (c, &body, face), at, span).unwrap();
        (at, extent)
    };
    let probe = plane([0.5, 0.5, 0.0], Vec3::new(1.0, 0.0, 0.0));
    let (at, extent) = reading(&probe, &cone);
    let FrameExtent::Reach(reach) = extent else {
        panic!("a plane×cone pair is levered at the cone face's reach, got {extent:?}");
    };
    let half = (Point3::new(1.01, 1.0, 0.0) - Point3::new(1.0, 0.0, 0.0)).norm() / 2.0;
    assert!(
        (reach - half).abs() < 1e-12,
        "the face's reach from its centre {at:?} is half its generator, {half}: got {reach}"
    );
    let turned = |margin: f64| {
        let theta = (margin / reach).asin();
        plane(
            [0.5, 0.5, 0.0],
            Vec3::new((alpha + theta).cos(), -(alpha + theta).sin(), 0.0),
        )
    };
    let sliver = (b.zero() * b.escalate()).sqrt();
    for (margin, want) in [(sliver, "pn_conic_type"), (4.0 * b.escalate(), "served")] {
        let p = turned(margin);
        for (label, a, c) in [("plane, cone", &p, &cone), ("cone, plane", &cone, &p)] {
            let (at, extent) = reading(a, c);
            let got = pair_section_frame_at(a, c, RadiusEvidence::None, at, extent, b);
            let verdict = match &got {
                Ok(Some(_)) => "served",
                Err(FrameError::Escalated(d)) => d.predicate.unwrap_or("unnamed"),
                _ => "another outcome",
            };
            assert_eq!(
                verdict,
                want,
                "{label}, margin {margin:e} at the reach: got {}",
                shape(&got)
            );
        }
    }
}

/// **A tilt the classifier decides but the carrier cannot tell from a
/// circle escalates.** A plane tilted off axis-normal by `θ`, with the
/// tilt margin levered at the extent four band-widths clear: the table's
/// classifier reads a tilted ellipse, whose semi-axes differ by
/// `O(θ²)`, far inside the band, and the conic constructor refuses
/// `ellipse_axes_distinct`. That is the pose's undecided circle, so the
/// frame escalates on that predicate rather than reading it as the
/// kernel's desync. The cone is the widening frustum, levered at the
/// face's reach; the cylinder is a unit wall, levered at its radius.
#[test]
fn a_near_circular_tilt_escalates_the_circle_question_not_a_desync() {
    let b = band();
    let tilted = |lever: f64| {
        let theta = (4.0 * b.escalate() / lever).asin();
        Vec3::new(theta.sin(), theta.cos(), 0.0)
    };
    let verdict = |got: &Frame| match got {
        Err(FrameError::Escalated(d)) => d.predicate.unwrap_or("unnamed"),
        _ => "another outcome",
    };
    let cone = widening();
    let p = plane([0.0, 0.5, 0.0], tilted(wall_reach()));
    let got = frame(&p, &cone, on_wall());
    assert_eq!(
        verdict(&got),
        "ellipse_axes_distinct",
        "the cone: got {}",
        shape(&got)
    );
    let wall = geom::Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let p = plane([0.0, 0.5, 0.0], tilted(1.0));
    for (label, a, c) in [("plane, wall", &p, &wall), ("wall, plane", &wall, &p)] {
        let got = pair_section_frame(
            a,
            c,
            RadiusEvidence::None,
            Point3::new(1.0, 0.5, 0.0),
            None,
            b,
        );
        assert_eq!(
            verdict(&got),
            "ellipse_axes_distinct",
            "the cylinder ({label}): got {}",
            shape(&got)
        );
    }
}

/// **A frame handed another pair's extent is a desync.** Only a cone
/// face's reach levers a plane×cone frame, and a cone face's reach levers
/// no cylinder pair: each mismatch refuses loudly rather than levering at
/// a measure of the wrong face.
#[test]
fn a_frame_handed_another_pairs_extent_is_a_desync() {
    let b = band();
    let cone = widening();
    let p = plane([0.0, 0.3, 0.0], Vec3::new(0.0, 1.0, 0.0));
    for extent in [
        FrameExtent::Radii,
        FrameExtent::Span(1.0),
        FrameExtent::Wall {
            below: 1.0,
            above: 1.0,
            across: 1.0,
            round: 1.0,
        },
    ] {
        let got = pair_section_frame_at(&p, &cone, RadiusEvidence::None, on_wall(), extent, b);
        assert!(
            matches!(got, Err(FrameError::Desync(_))),
            "plane×cone under {extent:?}: got {}",
            shape(&got)
        );
    }
    let wall = geom::Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let other = geom::Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(1.0, 0.0, 0.0),
        radius: 0.5,
        u_ref: Vec3::new(0.0, 1.0, 0.0),
    };
    for (label, a, c) in [("plane, wall", &p, &wall), ("wall, wall", &wall, &other)] {
        let got = pair_section_frame_at(
            a,
            c,
            RadiusEvidence::None,
            Point3::new(1.0, 0.3, 0.0),
            FrameExtent::Reach(1.0),
            b,
        );
        assert!(
            matches!(got, Err(FrameError::Desync(_))),
            "{label} under a cone face's reach: got {}",
            shape(&got)
        );
    }
}
