//! MATE-7a R1 review probes (blinded lane) — independent re-measurement
//! of PR #1477's load-bearing claims, above all the HEADLINE deviation:
//! lily wall 1 did NOT retire, on a measurement claim about the lily
//! scene's own geometry. Each probe either confirms that measurement
//! from scratch (rebuilding the scene's stem and arch from the turtle
//! math in `demos/tour/src/lily.rs`, outside the demo crate) or
//! attacks a claim the PR's own rows cannot already witness.
//!
//! Review-lane only; not part of the PR under review.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol, Vec3};
use sweep::test_support::{finished, tube_frame};
use sweep::{TubeWindow, tube_along_arc};
use topo::{AtRestBody, Body, BooleanError, FaceKey};

const STEM_TUBE: f64 = 0.060;
const ARCH_TUBE: f64 = 0.052;
const STEM_RING: f64 = 5.0;
const ARCH_RING: f64 = 1.1;

fn deg(d: f64) -> f64 {
    d.to_radians()
}

/// The lily's lower stem arc, rebuilt from the scene's own turtle
/// numbers: root at the origin heading +z (in the xz-plane), left turn
/// of 22 degrees on a 5 m ring — so the ring centre is (-5, 0, 0), the
/// start radial +x, the axis -y (`tube_arc`'s left-turn sense).
/// The stem, or `None` at ε = 1e-12, where its torus wall is past limb
/// 2's reach: the kernel's limit
/// (`work/offset/the-torus-composite-bounds-a-thin-torus-far-above-its-carriers-rounding.md`).
pub(crate) fn stem() -> Option<AtRestBody<f64>> {
    let stem = tube_along_arc(
        tube_frame(
            Point3::new(-STEM_RING, 0.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Tol::witness(),
        ),
        STEM_RING,
        TubeWindow::Arc {
            t0: 0.0,
            t1: deg(22.0),
        },
        STEM_TUBE,
        Tol::witness(),
    );
    let stem =
        test_utils::kernel_limit::pinned(stem, Tol::witness().eps(), &[1e-12], "the stem")?.body;
    Some(finished("the stem", stem, Tol::witness()))
}

/// The fork point (the stem's end / the arch's start), its tangent,
/// the arch's ring centre and start radial — the turtle math of
/// `lily.rs` (`Turtle::arc`), reproduced independently.
pub(crate) struct ArchFrame {
    pub(crate) fork: Point3<f64>,
    pub(crate) center: Point3<f64>,
    pub(crate) radial: Vec3<f64>,
    /// The arch's END point (the far cap's centre).
    pub(crate) far: Point3<f64>,
}

pub(crate) fn arch_frame() -> ArchFrame {
    let a = deg(22.0);
    // at_fork (xz): p = stem centre + 5·rot((1,0), 22°); t = rot((0,1), 22°).
    let fork2 = (-STEM_RING + STEM_RING * a.cos(), STEM_RING * a.sin());
    let t2 = (-a.sin(), a.cos());
    // Left turn: n = (-t.z, t.x); centre = fork + 1.1 n; radial = -n.
    let n2 = (-t2.1, t2.0);
    let c2 = (fork2.0 + ARCH_RING * n2.0, fork2.1 + ARCH_RING * n2.1);
    let r2 = (-n2.0, -n2.1);
    // The arch's end: centre + 1.1·rot(radial, 170°).
    let turn = deg(170.0);
    let adv = (
        r2.0 * turn.cos() - r2.1 * turn.sin(),
        r2.0 * turn.sin() + r2.1 * turn.cos(),
    );
    ArchFrame {
        fork: Point3::new(fork2.0, 0.0, fork2.1),
        center: Point3::new(c2.0, 0.0, c2.1),
        radial: Vec3::new(r2.0, 0.0, r2.1),
        far: Point3::new(c2.0 + ARCH_RING * adv.0, 0.0, c2.1 + ARCH_RING * adv.1),
    }
}

/// The lily's arch, continuing G1 from the stem's end: 170 degrees on
/// a 1.1 m ring, tube 0.052.
pub(crate) fn arch() -> AtRestBody<f64> {
    let f = arch_frame();
    let arch = tube_along_arc(
        tube_frame(
            f.center,
            Vec3::new(0.0, -1.0, 0.0),
            f.radial,
            Tol::witness(),
        ),
        ARCH_RING,
        TubeWindow::Arc {
            t0: 0.0,
            t1: deg(170.0),
        },
        ARCH_TUBE,
        Tol::witness(),
    )
    .expect("arch builds")
    .body;
    finished("the arch", arch, Tol::witness())
}

fn plane_faces(body: &Body<f64>) -> Vec<(FaceKey, Point3<f64>, Vec3<f64>)> {
    body.faces()
        .filter_map(|(k, f)| match body.get_surface(f.surface) {
            Some(&geom::Surface::Plane { origin, normal, .. }) => Some((k, origin, normal)),
            _ => None,
        })
        .collect()
}

/// The coplanar cross cap pairs at the fork (stem end cap against
/// arch start cap), counted by POSITION: the weld's whole flush
/// contact.
pub(crate) fn weld_cap_pairs(stem: &Body<f64>, arch: &Body<f64>) -> usize {
    let fork = arch_frame().fork;
    let mut pairs = 0;
    for &(_, oa, _) in &plane_faces(stem) {
        for &(_, ob, _) in &plane_faces(arch) {
            if (oa - fork).norm() < 1e-9 && (ob - fork).norm() < 1e-9 {
                pairs += 1;
            }
        }
    }
    pairs
}

/// Exact distance from a point to the stem's 22-degree tube-centre
/// arc (closest point on an arc is the azimuth projection when it
/// lands in the window, else the nearer endpoint).
fn dist_to_stem_center_arc(p: Point3<f64>) -> f64 {
    let c = Point3::new(-STEM_RING, 0.0, 0.0);
    let rel = p - c;
    let theta = rel.z.atan2(rel.x).clamp(0.0, deg(22.0));
    let on = Point3::new(
        c.x + STEM_RING * theta.cos(),
        0.0,
        c.z + STEM_RING * theta.sin(),
    );
    (p - on).norm()
}

/// P1 — the HEADLINE measurement, re-taken from scratch.
///
/// PR #1477's deviation 1 measured the wall-1 refusal as the stem's
/// tube wall against the arch's FAR cap, 2.08 m from any contact,
/// overlapping only because the wall's box was the WHOLE RING. What
/// this row re-derives now is: (a) wall 1 builds, both orders: the
/// section certificate's parallel-tori arm reads the two tube walls'
/// section as two loops, mirror images across the weld plane, each
/// outside one face's window, so the interior-loop guard clears them
/// and the union is one stem of the two tubes' volumes; ∩ is empty and
/// each difference gives back its first operand; (b) the far cap
/// stands ~2 m clear of the stem wall's exact locus; (c) it is no
/// longer inside the wall's box either; (d) near the weld the walls
/// stand the 8 mm annular gap apart (tube 0.060 vs 0.052). No `if let`
/// skip-hazard: the far cap MUST exist.
#[test]
fn p1_wall1_glues_the_stem_into_one_body() {
    let Some(s) = stem() else { return };
    let a = arch();
    assert_eq!(
        weld_cap_pairs(&s, &a),
        1,
        "exactly one coplanar cross cap pair at the fork"
    );

    // (a) Pappus: each tube is its disc swept along its arc.
    let tube = |r: f64, ring: f64, turn: f64| core::f64::consts::PI * r * r * ring * deg(turn);
    let (vs, va) = (
        tube(STEM_TUBE, STEM_RING, 22.0),
        tube(ARCH_TUBE, ARCH_RING, 170.0),
    );
    let volume = |what: &str, r: Result<topo::BooleanResult<f64>, BooleanError>| {
        let r = r.unwrap_or_else(|e| panic!("{what} builds: {e:?}"));
        r.body().map(|b| {
            assert_eq!(b.body.shells().count(), 1, "{what}: one shell");
            topo::test_support::pinned_volume(what, &b.body, Tol::witness()).volume
        })
    };
    for (what, r, want) in [
        (
            "stem ∪ arch",
            topo::union(&s, &a, Tol::witness()),
            Some(vs + va),
        ),
        (
            "arch ∪ stem",
            topo::union(&a, &s, Tol::witness()),
            Some(vs + va),
        ),
        ("stem ∩ arch", topo::intersect(&s, &a, Tol::witness()), None),
        (
            "stem ∖ arch",
            topo::subtract(&s, &a, Tol::witness()),
            Some(vs),
        ),
        (
            "arch ∖ stem",
            topo::subtract(&a, &s, Tol::witness()),
            Some(va),
        ),
    ] {
        let got = volume(what, r);
        match (got, want) {
            (Some(got), Some(want)) => assert!(
                (got - want).abs() < 1e-9 * want,
                "{what}: volume {got}, closed form {want}"
            ),
            (None, None) => {}
            _ => panic!("{what}: body {got:?}, wanted {want:?}"),
        }
    }
    let frame = arch_frame();
    let far_cap = plane_faces(&a)
        .into_iter()
        .find(|&(_, o, _)| (o - Point3::new(0.0, 0.0, 0.0)).norm() > 2.0)
        .expect("the arch has a cap more than 2 m from the world origin");
    assert!(
        (far_cap.1 - frame.far).norm() < 1e-9,
        "that cap sits at the turtle math's arch end point"
    );

    // (b) The far cap's loci never approach — every point of its disc
    // is more than 1.9 m from the stem's windowed tube wall. Lower bound: distance
    // to the tube-CENTRE arc minus the tube radius, over a dense disc
    // sampling.
    let (e1, e2) = {
        let n = (frame.far - frame.center).cross(Vec3::new(0.0, 1.0, 0.0));
        // The cap plane contains ±y and the in-plane direction
        // perpendicular to the end tangent; a basis from the world's
        // own frame is enough for sampling.
        let n = n.normalize();
        (Vec3::new(0.0, 1.0, 0.0), n)
    };
    let mut min_lb = f64::INFINITY;
    for ir in 0..=8 {
        let r = ARCH_TUBE * f64::from(ir) / 8.0;
        for ip in 0..48 {
            let psi = core::f64::consts::TAU * f64::from(ip) / 48.0;
            let p = frame.far + (e1 * psi.cos() + e2 * psi.sin()) * r;
            min_lb = min_lb.min(dist_to_stem_center_arc(p) - STEM_TUBE);
        }
    }
    assert!(
        min_lb > 1.9,
        "the far cap must stand ~2 m clear of the stem wall's exact locus; \
         measured lower bound {min_lb:.3} m"
    );

    // (c) The box that used to reach the far cap, and the one that
    // does not. The far cap's CENTRE still sits inside the WHOLE-RING
    // box the wall used to get; it stands more than 1.5 m clear of the
    // window box the wall gets now, in x.
    let whole = (
        (-2.0 * STEM_RING - STEM_TUBE, STEM_TUBE),
        (-STEM_TUBE, STEM_TUBE),
        (-STEM_RING - STEM_TUBE, STEM_RING + STEM_TUBE),
    );
    let q = frame.far;
    assert!(
        q.x > whole.0.0
            && q.x < whole.0.1
            && q.y > whole.1.0
            && q.y < whole.1.1
            && q.z > whole.2.0
            && q.z < whole.2.1,
        "the far cap centre lies inside the stem wall's whole-torus box"
    );
    let tight_min_x = -STEM_RING + STEM_RING * deg(22.0).cos() - STEM_TUBE;
    assert!(
        q.x < tight_min_x - 1.5,
        "the window box of the 22° arc (min x {tight_min_x:.3}) clears the far cap \
         (x {:.3}) by more than 1.5 m",
        q.x
    );

    // (d) Near the weld plane the two tube walls stand ~8 mm apart:
    // their minor radii differ (0.060 vs 0.052) and their cross-sections
    // there are concentric. Sampled on both walls within 1° of the weld.
    let assert_minor = |body: &Body<f64>, want: f64| {
        for (_, f) in body.faces() {
            if let Some(&geom::Surface::Torus { minor_radius, .. }) = body.get_surface(f.surface) {
                assert!((minor_radius - want).abs() < 1e-12, "minor radius {want}");
            }
        }
    };
    assert_minor(&s, STEM_TUBE);
    assert_minor(&a, ARCH_TUBE);
    let stem_pts: Vec<Point3<f64>> = {
        let mut v = Vec::new();
        for it in 0..=30 {
            let theta = deg(21.0) + deg(1.0) * f64::from(it) / 30.0;
            let cs = Point3::new(
                -STEM_RING + STEM_RING * theta.cos(),
                0.0,
                STEM_RING * theta.sin(),
            );
            let radial = Vec3::new(theta.cos(), 0.0, theta.sin());
            for ip in 0..180 {
                let phi = core::f64::consts::TAU * f64::from(ip) / 180.0;
                v.push(
                    cs + (radial * phi.cos() + Vec3::new(0.0, 1.0, 0.0) * phi.sin()) * STEM_TUBE,
                );
            }
        }
        v
    };
    let arch_pts: Vec<Point3<f64>> = {
        let mut v = Vec::new();
        for it in 0..=30 {
            let theta = deg(1.0) * f64::from(it) / 30.0;
            let (rc, rs) = (theta.cos(), theta.sin());
            // Rotate the arch's start radial by theta about -y (the
            // same sense the window advances).
            let r0 = frame.radial;
            let radial = Vec3::new(r0.x * rc - r0.z * rs, 0.0, r0.x * rs + r0.z * rc);
            let cs = frame.center + radial * ARCH_RING;
            for ip in 0..180 {
                let phi = core::f64::consts::TAU * f64::from(ip) / 180.0;
                v.push(
                    cs + (radial * phi.cos() + Vec3::new(0.0, 1.0, 0.0) * phi.sin()) * ARCH_TUBE,
                );
            }
        }
        v
    };
    let mut min_wall = f64::INFINITY;
    for p in &stem_pts {
        for q in &arch_pts {
            min_wall = min_wall.min((*p - *q).norm());
        }
    }
    assert!(
        (0.006..0.010).contains(&min_wall),
        "near the weld the tube walls' nearest approach is the 8 mm annular gap, \
         measured {min_wall:.4} m"
    );
}

/// **The same two tori with a stem window that reaches over a loop
/// refuse at their germs.** Run to 30° instead of 22°, the stem's tube
/// wall spans the loop on the arch's side of the weld plane, and its
/// seams cross the arch's tube, so the pair carries events, and the
/// reduction's germ mint, carrying the two tube walls' section to read a
/// site's foot, refuses it (`GermSectionUnbuilt`, torus × torus: the
/// march's own stop on the lily's tori, `CellBudget` at the compiled ε
/// and the tube's straddle at 1e-6,
/// [`the_lilys_tori_exhaust_the_doors_cell_budget`]) before the
/// interior-loop guard is asked. What this row pins is that refusal: it
/// goes red when the op starts building this pair, or refuses it
/// somewhere else. It cannot see the certificate. The certificate-level degrade row is
/// `section_cert_parallel_tori_rows`
/// `the_lilys_pair_clears_only_on_windows_that_miss_its_loops`.
#[test]
fn a_stem_window_reaching_over_a_loop_does_not_glue() {
    let long = tube_along_arc(
        tube_frame(
            Point3::new(-STEM_RING, 0.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Tol::witness(),
        ),
        STEM_RING,
        TubeWindow::Arc {
            t0: 0.0,
            t1: deg(30.0),
        },
        STEM_TUBE,
        Tol::witness(),
    );
    let Some(long) =
        test_utils::kernel_limit::pinned(long, Tol::witness().eps(), &[1e-12], "the long stem")
    else {
        return;
    };
    let (long, a) = (finished("the long stem", long.body, Tol::witness()), arch());
    for (what, r) in [
        ("long ∪ arch", topo::union(&long, &a, Tol::witness())),
        ("arch ∪ long", topo::union(&a, &long, Tol::witness())),
    ] {
        // The door's own stop on the pair: its cell budget at the
        // compiled ε, its tube at 1e-6, as the door row below pins it.
        let coarse = Tol::witness().eps() == 1e-6;
        assert!(
            match &r {
                Err(BooleanError::GermSectionUnbuilt {
                    a_kind: geom::SurfaceKind::Torus,
                    b_kind: geom::SurfaceKind::Torus,
                    why: topo::SectionUnbuilt::March(march),
                    ..
                }) => {
                    if coarse {
                        matches!(march, geom_brep::SsiError::TubeStraddles { .. })
                    } else {
                        matches!(march, geom_brep::SsiError::CellBudget { .. })
                    }
                }
                _ => false,
            },
            "{what}: the two tube walls refuse at their germs: {:?}",
            r.err()
        );
    }
}

/// **An arch started inside the stem glues to the same volume.** Started
/// 2° or 5° back from the fork, the arch's tube runs inside the stem's
/// there (the stem-side loop starts about 7.8° back), so the union is
/// still the two tubes' volume. Mutant: a certificate that clears a pair
/// whose overlap the op then counts twice.
#[test]
fn an_arch_started_inside_the_stem_glues_to_the_same_volume() {
    let f = arch_frame();
    let want = core::f64::consts::PI
        * (STEM_TUBE.powi(2) * STEM_RING * deg(22.0) + ARCH_TUBE.powi(2) * ARCH_RING * deg(170.0));
    let Some(s) = stem() else { return };
    for back in [2.0, 5.0] {
        let early = tube_along_arc(
            tube_frame(
                f.center,
                Vec3::new(0.0, -1.0, 0.0),
                f.radial,
                Tol::witness(),
            ),
            ARCH_RING,
            TubeWindow::Arc {
                t0: deg(-back),
                t1: deg(170.0),
            },
            ARCH_TUBE,
            Tol::witness(),
        )
        .expect("the early arch builds")
        .body;
        let a = finished("the early arch", early, Tol::witness());
        for (what, r) in [
            ("stem ∪ early", topo::union(&s, &a, Tol::witness())),
            ("early ∪ stem", topo::union(&a, &s, Tol::witness())),
        ] {
            let b = r.unwrap_or_else(|e| panic!("{what}: {e:?}"));
            let b = b.body().expect("a body");
            assert_eq!(b.body.shells().count(), 1, "{what}: one shell");
            let v = topo::test_support::pinned_volume(what, &b.body, Tol::witness()).volume;
            assert!(
                (v - want).abs() < 1e-9 * want,
                "{what} from {back}° back: {v} vs {want}"
            );
        }
    }
}

/// **The lily's stem and arch tori exhaust the door's cell budget.** The
/// two tube walls' whole tori (axes parallel and apart): the pose's gap
/// is transverse and the value lists the parallel tori's components,
/// but where the arch leaves the fork its core runs tangent to the
/// stem's and its tube 8 mm inside the stem's, a near tube-circle
/// tangency, and the accounting over the smaller torus's ball refuses
/// `CellBudget` at the compiled ε, on the tube at 1e-6
/// (`work/sectrung/the-torus-doors-tube-straddles-transverse-poses-at-a-coarse-band.md`)
/// and on the thin stem's limb 2 at 1e-12 (the kernel limit [`stem`]
/// names),
/// as a cylinder × sphere pose near its node does
/// (`work/sectrung/a-cylinder-sphere-march-near-its-node-costs-seconds-and-refuses-by-budget.md`).
/// This row pins that stop: it goes red the day the door carries the
/// lily's pair.
#[test]
fn the_lilys_tori_exhaust_the_doors_cell_budget() {
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let ring = |center, big, small| geom::Surface::Torus {
        center,
        axis: Vec3::new(0.0, -1.0, 0.0),
        major_radius: big,
        minor_radius: small,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let stem = ring(Point3::new(-STEM_RING, 0.0, 0.0), STEM_RING, STEM_TUBE);
    let arch = ring(arch_frame().center, ARCH_RING, ARCH_TUBE);
    let geom_brep::SectionRead::Built(section) =
        geom_brep::pair_section(&stem, &arch, band).expect("the pose decides")
    else {
        panic!("the value builds two tori");
    };
    assert!(
        section.tangency().is_none() && section.counted().is_some_and(|c| !c.is_empty()),
        "a transverse gap, its components listed"
    );
    let got = geom_brep::torus_torus_ssi(&stem, &arch, section.pose().domain(), band);
    let eps = Tol::witness().eps();
    assert!(
        match &got {
            Err(geom_brep::SsiError::TubeStraddles { .. }) => eps == 1e-6,
            Err(geom_brep::SsiError::RefinementExhausted { .. }) => eps == 1e-12,
            Err(geom_brep::SsiError::CellBudget { .. }) => eps != 1e-6 && eps != 1e-12,
            _ => false,
        },
        "the door's stop on the lily's pair: {:?}",
        got.map(|o| o.branches.len())
    );
}
