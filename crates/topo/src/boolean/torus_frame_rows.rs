//! **The plane×torus germ frame** ([`pair_section_frame_at`]'s torus
//! arm), one row per outcome of the C5 table it reads
//! ([`geom_brep::plane_torus_section`]), and the loop a germ's site
//! picks. The torus is the donut of the plane × torus frame spec,
//! `R = 2`, `r = 0.5` about `y` through the origin; the planes are its
//! poses: M1 (`x = 0`, through the axis), P1 (`y = 0.2`, square to it),
//! S1 and B1's sides (`x = 0.3`, `x = ±0.1`, parallel to it short of
//! the inner equator), and the general poses.

#![allow(clippy::unwrap_used, clippy::panic)]

use super::join::{
    FrameError, FrameExtent, SectionLoop, SectionLoops, frame_refusal, germs_face_each_other,
    pair_section_frame_at,
};
use super::{BooleanError, HalfGerm, Locus, SpiricLoops};
use crate::entity::{FaceKey, HalfEdgeKey};
use geom_core::{Band, Point3, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

const BIG_R: f64 = 2.0;
const SMALL_R: f64 = 0.5;

fn donut() -> geom::Surface<f64> {
    geom::Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_y(),
        major_radius: BIG_R,
        minor_radius: SMALL_R,
        u_ref: Vec3::unit_x(),
    }
}

fn plane(origin: [f64; 3], normal: Vec3<f64>) -> geom::Surface<f64> {
    let normal = normal.normalize();
    geom::Surface::Plane {
        origin: Point3::new(origin[0], origin[1], origin[2]),
        normal,
        u_ref: normal.cross(Vec3::unit_y()).normalize(),
    }
}

/// A torus face's reach, from a point on the tube: its far side.
const REACH: f64 = BIG_R + BIG_R + SMALL_R;

type Loops = Result<Option<SectionLoops<f64>>, FrameError>;

/// The frame both ways round, levered at `lever`. The two readings must
/// agree; the row reads the first.
fn frame_at(p: &geom::Surface<f64>, lever: f64) -> Loops {
    let read = |a, b| {
        pair_section_frame_at(
            a,
            b,
            Point3::new(BIG_R, 0.0, 0.0),
            FrameExtent::Reach(lever),
            band(),
        )
    };
    let torus = donut();
    let (first, other) = (read(p, &torus), read(&torus, p));
    assert_eq!(
        shape(&first),
        shape(&other),
        "the frame's member orders disagree"
    );
    first
}

fn frame(p: &geom::Surface<f64>) -> Loops {
    frame_at(p, REACH)
}

/// An outcome's shape, for comparing readings.
fn shape(got: &Loops) -> String {
    match got {
        Ok(Some(loops)) => format!("loops {loops:?}"),
        Ok(None) => "straight".into(),
        Err(FrameError::Escalated(d)) => format!("escalated {:?}", d.predicate),
        Err(FrameError::Desync(what)) => format!("desync {what}"),
        Err(FrameError::Spiric { loops, section }) => format!("spiric {loops:?} {section}"),
        Err(FrameError::NoArm) => "no arm".into(),
        Err(FrameError::OutsideInventory { .. }) => "outside the inventory".into(),
        Err(FrameError::RadiusEscalated { .. }) => "radius escalated".into(),
        Err(FrameError::IntersectingCylinderAxes { .. }) => "pinch".into(),
    }
}

fn served(got: Loops, what: &str) -> SectionLoops<f64> {
    match got {
        Ok(Some(loops)) => loops,
        other => panic!("{what}: the frame serves the pose, got {}", shape(&other)),
    }
}

/// The loop the site `p` picks, and its index.
fn pick(loops: &SectionLoops<f64>, p: Point3<f64>) -> (usize, SectionLoop<f64>) {
    loops
        .at(p, band())
        .unwrap_or_else(|e| panic!("{p:?} picks a loop: {e:?}"))
}

/// Two frames are one when their centres agree and their axes are
/// parallel, within the band.
fn assert_frame(got: SectionLoop<f64>, centre: Point3<f64>, axis: Vec3<f64>, what: &str) {
    let eps = band().escalate();
    assert!(
        (got.center - centre).norm() <= eps,
        "{what}: centre {:?}, want {centre:?}",
        got.center
    );
    assert!(
        got.axis.normalize().cross(axis.normalize()).norm() <= eps,
        "{what}: axis {:?}, want {axis:?}",
        got.axis
    );
}

/// **The meridian loop is the germ's** (M1, `x = 0`): the plane holds
/// the axis and cuts the two meridian circles about `(0, 0, ±2)`. A site
/// on each takes that circle's own frame, its centre and the plane's
/// normal. A dispatch that served every germ the first loop's frame
/// hands the `z < 0` site the other circle's centre.
#[test]
fn a_germ_on_a_meridian_circle_takes_that_circles_frame() {
    let loops = served(frame(&plane([0.0, 0.0, 0.0], Vec3::unit_x())), "M1");
    let mut seen = Vec::new();
    for sign in [1.0, -1.0] {
        let centre = Point3::new(0.0, 0.0, 2.0 * sign);
        for site in [
            centre + Vec3::new(0.0, SMALL_R, 0.0),
            centre + Vec3::new(0.0, 0.0, SMALL_R * sign),
            centre + Vec3::new(0.0, -SMALL_R, 0.0),
        ] {
            let (index, lp) = pick(&loops, site);
            assert_frame(lp, centre, Vec3::unit_x(), &format!("M1, site {site:?}"));
            assert!(
                lp.ball.is_none(),
                "M1: a circle's centre is a centre of symmetry"
            );
            seen.push((sign, index));
        }
    }
    assert!(
        seen.iter()
            .all(|&(s, i)| seen.iter().all(|&(t, j)| (s == t) == (i == j))),
        "M1: one loop per circle: {seen:?}"
    );
}

fn germ(dir: Vec3<f64>) -> HalfGerm<f64> {
    HalfGerm {
        he: HalfEdgeKey::default(),
        a_face: FaceKey::default(),
        b_face: FaceKey::default(),
        a_locus: Locus::InFace(FaceKey::default()),
        b_locus: Locus::InFace(FaceKey::default()),
        dir,
    }
}

/// **Germs on different loops never face** (M1's face pair, which holds
/// arcs of both meridian circles): a germ at the top of the `z = 2`
/// circle and one at the top of the `z = −2` circle, turning opposite
/// ways each about its own circle, would face on one circle. Across the
/// two they do not, and that is a verdict, not a desync. The same two
/// directions on one circle do face.
#[test]
fn germs_on_different_meridian_circles_never_face() {
    let loops = served(frame(&plane([0.0, 0.0, 0.0], Vec3::unit_x())), "M1");
    let top = |z: f64| Point3::new(0.0, SMALL_R, z);
    let bottom = |z: f64| Point3::new(0.0, -SMALL_R, z);
    let (g1, g2) = (
        germ(Vec3::new(0.0, 0.0, 1.0)),
        germ(Vec3::new(0.0, 0.0, 1.0)),
    );
    let face = |p1, p2| germs_face_each_other(Some(&loops), &g1, &g2, p1, p2, band());
    assert!(
        matches!(face(top(2.0), bottom(2.0)), Ok(true)),
        "one circle: the top and the bottom turn opposite ways and face"
    );
    assert!(
        matches!(face(top(2.0), bottom(-2.0)), Ok(false)),
        "two circles: they never face, got {:?}",
        face(top(2.0), bottom(-2.0))
    );
}

/// **The concentric circles share one frame, and each is its own
/// loop** (P1, `y = 0.2`): both circles turn about `(0, 0.2, 0)` and the
/// axis; a site outside the ring's core circle picks the outer, one
/// inside it the inner.
#[test]
fn a_plane_square_to_the_axis_frames_both_circles_on_the_axis() {
    let h = 0.2;
    let loops = served(frame(&plane([0.0, h, 0.0], Vec3::unit_y())), "P1");
    let w = (SMALL_R * SMALL_R - h * h).sqrt();
    let centre = Point3::new(0.0, h, 0.0);
    let (outer, inner) = (BIG_R + w, BIG_R - w);
    let sites = [
        (Point3::new(outer, h, 0.0), 0),
        (Point3::new(0.0, h, -outer), 0),
        (Point3::new(inner, h, 0.0), 1),
        (Point3::new(-inner, h, 0.0), 1),
    ];
    for (site, want) in sites {
        let (index, lp) = pick(&loops, site);
        assert_eq!(index, want, "P1, site {site:?}: its circle");
        assert_frame(lp, centre, Vec3::unit_y(), &format!("P1, site {site:?}"));
    }
}

/// A point of the spiric oval cut by the plane `x = d` on the `σ` side
/// of the axis's trace, at the torus's minor angle `v`, and its tangent
/// there (`ŵ = ŷ × x̂ = −ẑ`, so the `σ` side runs along `−σ ẑ`).
fn on_oval(d: f64, sigma: f64, v: f64) -> (Point3<f64>, Vec3<f64>) {
    let rho = BIG_R + SMALL_R * v.cos();
    let t = (rho * rho - d * d).sqrt();
    let y = SMALL_R * v.sin();
    let (drho, dy) = (-SMALL_R * v.sin(), SMALL_R * v.cos());
    let dt = rho * drho / t;
    let w = Vec3::new(0.0, 0.0, -sigma);
    (Point3::new(d, y, 0.0) + w * t, w * dt + Vec3::unit_y() * dy)
}

/// The turn of `tangent` at `p` about `(centre, axis)`.
fn turn(centre: Point3<f64>, axis: Vec3<f64>, p: Point3<f64>, tangent: Vec3<f64>) -> f64 {
    axis.dot((p - centre).cross(tangent))
}

/// **The spiric ovals turn about where the core circle crosses the
/// plane** (S1, `x = 0.3`, and B1's sides, `x = ±0.1`): the frame is
/// `(k_σ, n̂)`, `k_σ = c + n̂d + σŵ√(R² − d²)`, and every point sampled
/// along each oval in closed form turns one strict way about it. The
/// plane's foot `c + n̂d` lies outside both ovals, so about it the sense
/// changes along each: a frame centred there is no frame. Each oval's
/// ball holds every sampled point.
#[test]
fn a_spiric_oval_turns_about_the_core_circles_crossing() {
    let b = band();
    for d in [0.3, 0.1, -0.1] {
        let loops = served(frame(&plane([d, 0.0, 0.0], Vec3::unit_x())), "S1");
        for sigma in [1.0, -1.0] {
            let what = format!("x = {d}, σ = {sigma}");
            let k = Point3::new(d, 0.0, -sigma * (BIG_R * BIG_R - d * d).sqrt());
            let foot = Point3::new(d, 0.0, 0.0);
            let samples: Vec<_> = (0..64)
                .map(|i| on_oval(d, sigma, f64::from(i) * core::f64::consts::TAU / 64.0))
                .collect();
            let (index, lp) = pick(&loops, samples[0].0);
            assert_frame(lp, k, Vec3::unit_x(), &what);
            let ball = lp
                .ball
                .unwrap_or_else(|| panic!("{what}: an oval's centre is no centre of symmetry"));
            let mut senses = Vec::new();
            let mut about_foot = Vec::new();
            for &(p, tangent) in &samples {
                assert_eq!(pick(&loops, p).0, index, "{what}: one loop along the oval");
                assert!(
                    (p - ball.center()).norm() <= ball.radius() + b.escalate(),
                    "{what}: {p:?} lies in the oval's ball"
                );
                let sense = turn(lp.center, lp.axis, p, tangent.normalize());
                assert!(
                    sense.abs() > b.escalate(),
                    "{what}: a definite sense at {p:?}, got {sense}"
                );
                senses.push(sense.signum());
                about_foot.push(turn(foot, lp.axis, p, tangent.normalize()).signum());
            }
            assert!(
                senses.iter().all(|&s| s == senses[0]),
                "{what}: one sense along the oval: {senses:?}"
            );
            assert!(
                about_foot.iter().any(|&s| s != about_foot[0]),
                "{what}: about the plane's foot the sense changes"
            );
        }
    }
}

/// **The loop margin decides definite, or the site is on neither loop**:
/// a site on the axis's trace in the plane, which no point of the
/// section comes near, is a desync, never a loop. On M1 that is the
/// axis itself; on S1 the plane's foot.
#[test]
fn a_site_on_the_axis_trace_is_a_desync_not_a_loop() {
    for (pose, p, site) in [
        (
            "M1",
            plane([0.0, 0.0, 0.0], Vec3::unit_x()),
            Point3::new(0.0, 0.3, 0.0),
        ),
        (
            "S1",
            plane([0.3, 0.0, 0.0], Vec3::unit_x()),
            Point3::new(0.3, -0.2, 0.0),
        ),
    ] {
        let loops = served(frame(&p), pose);
        assert!(
            matches!(loops.at(site, band()), Err(BooleanError::JoinDesync { .. })),
            "{pose}: got {:?}",
            loops.at(site, band())
        );
    }
}

/// **A general pose refuses by name, with the loops the certificate
/// reads**: one null oval for a plane parallel to the axis past the
/// inner equator (S3, `x = 1.8`, and the donut corner bar's end square
/// `z = −1.5068`), two loops round the tube for the oblique planes O1
/// and O2, two round the axis for a plane 3° off axis-normal. The
/// Boolean's refusal is [`BooleanError::GermSectionSpiric`], carrying
/// both, never the missing arm's `GermFrameUnsupported`.
#[test]
fn a_general_pose_refuses_by_name_with_its_loops() {
    let tilt = |a: f64| {
        let a = a.to_radians();
        Vec3::new(a.sin(), a.cos(), 0.0)
    };
    let rho = BIG_R - (SMALL_R * SMALL_R - 0.01_f64).sqrt();
    let end = -(rho * rho - 0.01).sqrt();
    for (pose, p, want) in [
        (
            "S3",
            plane([1.8, 0.0, 0.0], Vec3::unit_x()),
            SpiricLoops::Null,
        ),
        (
            "the corner bar's end square",
            plane([0.0, 0.0, end], Vec3::unit_z()),
            SpiricLoops::Null,
        ),
        (
            "O1",
            plane([0.0, 0.1, 0.0], tilt(20.0)),
            SpiricLoops::AroundTube,
        ),
        (
            "O2",
            plane([0.0, 0.0, 0.0], tilt(70.0)),
            SpiricLoops::AroundTube,
        ),
        (
            "3° off axis-normal",
            plane([0.0, 0.2, 0.0], tilt(3.0)),
            SpiricLoops::AroundAxis,
        ),
    ] {
        let got = frame(&p);
        let Err(e @ FrameError::Spiric { loops, .. }) = got else {
            panic!("{pose}: want the spiric refusal, got {}", shape(&got));
        };
        assert_eq!(loops, want, "{pose}");
        let torus = donut();
        assert!(
            matches!(
                frame_refusal(e, (FaceKey::default(), &p), (FaceKey::default(), &torus)),
                BooleanError::GermSectionSpiric {
                    a_kind: geom::SurfaceKind::Plane,
                    b_kind: geom::SurfaceKind::Torus,
                    loops,
                    section: geom_brep::SectionError::RoutesToGeneralRung { .. },
                    ..
                } if loops == want
            ),
            "{pose}: the Boolean's refusal names the spiric"
        );
    }
}

/// **Touching under a germ is the desync**: the plane `y = 0.5` touches
/// the tube along its top circle, and `y = 0.7` misses it; neither is a
/// locus a minted germ could lie on.
#[test]
fn a_tangent_circle_or_no_section_is_the_desync() {
    for y in [SMALL_R, 0.7] {
        let got = frame(&plane([0.0, y, 0.0], Vec3::unit_y()));
        assert!(
            matches!(got, Err(FrameError::Desync(_))),
            "y = {y}: got {}",
            shape(&got)
        );
    }
}

/// **The lever is the torus face's reach**: a plane through the centre
/// whose axis tilt, levered at the reach, lies in the band escalates
/// `pt_axis_in_plane`; it is never served as the meridian pair or the
/// ovals. At ten times the reach the same tilt is definite, and the pose
/// is general. Any other extent is a desync.
#[test]
fn the_axis_tilt_is_levered_at_the_torus_faces_reach() {
    let b = band();
    let sine = 2.0 * b.zero() / REACH;
    let n = Vec3::new((1.0 - sine * sine).sqrt(), sine, 0.0);
    let p = plane([0.0, 0.0, 0.0], n);
    let got = frame(&p);
    assert!(
        matches!(&got, Err(FrameError::Escalated(d)) if d.predicate == Some("pt_axis_in_plane")),
        "in band at the reach: got {}",
        shape(&got)
    );
    let got = frame_at(&p, 10.0 * REACH);
    assert!(
        matches!(
            got,
            Err(FrameError::Spiric {
                loops: SpiricLoops::AroundTube,
                ..
            })
        ),
        "definite at ten times the reach: got {}",
        shape(&got)
    );
    let torus = donut();
    for extent in [FrameExtent::Radii, FrameExtent::Span(REACH)] {
        let got = pair_section_frame_at(&p, &torus, Point3::origin(), extent, b);
        assert!(
            matches!(got, Err(FrameError::Desync(_))),
            "plane×torus under {extent:?}: got {}",
            shape(&got)
        );
    }
}
