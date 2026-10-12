//! Review probes for PR 4896 (bore ruling, step 3): a feature wholly
//! inside a convex ruled cut-off's sliver goes with it; a crossing
//! feature still refuses. Each row either pins what held or names what
//! broke.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Bounds, Interval, Point2, Point3, Real, Tol, Vec3};
use profile::{ProfileLoop, SketchPlane, test_support::bulge_loop};
use sweep::blend::{BlendError, fillet_edges};
use sweep::test_support::{
    ROD_FILLET, ROD_FLAT, ROD_L, ROD_R, extruded, finished, rod_chord_at, rod_creases,
    rod_section_cut, sketch_from_axes,
};
use topo::{
    AtRestBody, Body, ContactRecords, mass_properties, validate_geometric, validate_pseudomanifold,
};

fn tol() -> Tol {
    Tol::witness()
}

fn volume(body: &Body<f64>) -> f64 {
    mass_properties(body, tol()).expect("props").volume
}

/// The D profile scaled by `s`.
fn d_loop<T: Real>(s: f64) -> ProfileLoop<T> {
    let (c, f) = (rod_chord_at(ROD_FLAT), T::from_f64);
    bulge_loop(vec![
        (Point2::new(f(s * ROD_FLAT), f(s * c.half)), f(c.wall_bulge)),
        (Point2::new(f(s * ROD_FLAT), f(-s * c.half)), f(0.0)),
    ])
}

fn round<T: Real>(x: f64, y: f64, a: f64) -> ProfileLoop<T> {
    let f = T::from_f64;
    bulge_loop(vec![
        (Point2::new(f(x + a), f(y)), f(1.0)),
        (Point2::new(f(x - a), f(y)), f(1.0)),
    ])
}

fn poly(pts: &[(f64, f64)]) -> ProfileLoop<f64> {
    bulge_loop(pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect())
}

fn rod_with(holes: Vec<ProfileLoop<f64>>) -> AtRestBody<f64> {
    let mut loops = vec![d_loop(1.0)];
    loops.extend(holes);
    finished(
        "rod",
        extruded(SketchPlane::xy(), loops, ROD_L, tol()),
        tol(),
    )
}

/// The section a convex band removes at both creases, times the length.
fn two_slivers(s: f64) -> f64 {
    2.0 * rod_section_cut(ROD_R, ROD_FLAT, ROD_FILLET) * ROD_L * s * s * s
}

/// Fillets both creases and, on success, checks tiers 3 / 3′, naming
/// totality, and `ΔV` against `want`. Returns the refusal otherwise.
fn carve(body: &AtRestBody<f64>, r: f64, want: f64, what: &str) -> Result<(), BlendError> {
    let creases = rod_creases(body);
    assert_eq!(creases.len(), 2, "{what}: the two creases");
    let out = fillet_edges(body, &creases, r, tol()).map_err(|e| e.error)?;
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, {e:?}"));
    validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
        .unwrap_or_else(|e| panic!("{what}: tier 3′, {e:?}"));
    sweep::test_support::assert_naming_totality(body, &out, &creases, what);
    let dv = volume(&out.body) - volume(body);
    assert!(
        (dv - want).abs() <= 1e-10 * want.abs().max(1e-6),
        "{what}: ΔV {dv} vs {want} (a wrong body)"
    );
    Ok(())
}

/// **Poses.** The witness bore under a rigid motion about a skew axis
/// still goes with its sliver at the closed form.
#[test]
fn p01_the_witness_under_rigid_motions_goes_at_the_closed_form() {
    let a = 0.002;
    let want = -two_slivers(1.0) + core::f64::consts::PI * a * a * ROD_L;
    let base = rod_with(vec![round(0.297, 0.395, a)]);
    for (k, (axis, angle, shift)) in [
        (Vec3::new(1.0, 2.0, 3.0), 0.7, Vec3::new(3.0, -2.0, 5.0)),
        (Vec3::new(0.0, 1.0, 0.0), 1.9, Vec3::new(-11.0, 0.5, 0.25)),
        (Vec3::new(-0.3, 0.1, 1.0), 2.8, Vec3::new(0.0, 0.0, 0.0)),
    ]
    .into_iter()
    .enumerate()
    {
        let m = Affine3::translation(shift)
            * Affine3::rotation_about_axis(Point3::new(0.1, 0.2, 0.3), axis, angle);
        let moved = topo::transform_rigid(&base, &m, tol()).expect("rigid motion");
        let moved = finished("moved", moved, tol());
        carve(&moved, ROD_FILLET, want, &format!("pose {k}"))
            .unwrap_or_else(|e| panic!("pose {k}: the bore goes, got {e:?}"));
    }
}

/// **Scales.** The witness scaled by `s` goes at `s³` times the closed
/// form.
#[test]
fn p02_the_witness_at_scales_goes_at_the_closed_form() {
    for s in [0.01, 0.2, 7.0, 150.0] {
        let a = 0.002 * s;
        let body = finished(
            "scaled",
            extruded(
                SketchPlane::xy(),
                vec![d_loop(s), round(0.297 * s, 0.395 * s, a)],
                ROD_L * s,
                tol(),
            ),
            tol(),
        );
        let want = -two_slivers(s) + core::f64::consts::PI * a * a * ROD_L * s;
        match carve(&body, ROD_FILLET * s, want, &format!("scale {s}")) {
            Ok(()) => {}
            Err(e) => eprintln!("scale {s}: refused {e:?}"),
        }
    }
}

/// A cylinder of radius `a` along `p → q` (sketch coordinates at
/// `z = 0` and `z = L`), running past both caps.
fn tilted_cylinder(p: (f64, f64), q: (f64, f64), a: f64) -> AtRestBody<f64> {
    let d = Vec3::new(q.0 - p.0, q.1 - p.1, ROD_L);
    let n = d.norm();
    let dh = d / n;
    let over = 0.05;
    let o = Point3::new(p.0, p.1, 0.0) - dh * (over / dh.z);
    let u = Vec3::new(1.0, 0.0, 0.0);
    let u = (u - dh * u.dot(dh)).normalize();
    let v = dh.cross(u);
    let plane = sketch_from_axes(o, u, v, tol());
    finished(
        "tilted cylinder",
        extruded(
            plane,
            vec![round(0.0, 0.0, a)],
            n + 2.0 * over / dh.z,
            tol(),
        ),
        tol(),
    )
}

fn tilted_bored(p: (f64, f64), q: (f64, f64), a: f64) -> Option<AtRestBody<f64>> {
    let rod = rod_with(Vec::new());
    let cyl = tilted_cylinder(p, q, a);
    match topo::subtract(&rod, &cyl, tol()) {
        Ok(r) => r.body().map(|b| b.body.clone()),
        Err(e) => {
            eprintln!("subtract refused: {e:?}");
            None
        }
    }
}

/// **A tilted bore wholly inside the sliver** along its whole length:
/// it goes, or refuses; never a wrong body.
#[test]
fn p03_a_tilted_bore_wholly_inside_goes_or_refuses_never_wrong() {
    let plain = volume(&rod_with(Vec::new()));
    for (what, p, q) in [
        ("long diagonal", (0.2989, 0.3854), (0.2699, 0.4199)),
        ("short tilt", (0.2985, 0.3880), (0.2945, 0.3970)),
    ] {
        let Some(body) = tilted_bored(p, q, 0.0005) else {
            panic!("{what}: the tilted subtract does not build");
        };
        let want = -two_slivers(1.0) + (plain - volume(&body));
        match carve(&body, ROD_FILLET, want, what) {
            Ok(()) => eprintln!("{what}: consumed at the closed form"),
            Err(e) => eprintln!("{what}: refused {e:?}"),
        }
    }
}

/// **A tilted bore whose ends lie in the sliver and whose middle runs
/// through the kept ball**: it crosses the band, so it must refuse.
#[test]
fn p04_a_tilted_bore_inside_at_both_caps_but_crossing_between_refuses() {
    let Some(body) = tilted_bored((0.2993, 0.3620), (0.2647, 0.4234), 0.0003) else {
        panic!("the tilted subtract does not build");
    };
    let creases = rod_creases(&body);
    let got = fillet_edges(&body, &creases, ROD_FILLET, tol());
    match got {
        Ok(out) => panic!(
            "a bore crossing the kept ball between the caps was consumed: ΔV {}",
            volume(&out.body) - volume(&body)
        ),
        Err(e) => eprintln!("tilted crossing: refused {:?}", e.error),
    }
}

/// **Polygonal holes** (planar walls, line seams) wholly inside the
/// sliver go at the closed form.
#[test]
fn p05_polygonal_holes_inside_go_at_the_closed_form() {
    let h = 0.0012;
    let (x, y) = (0.297, 0.395);
    let square = [
        (x - h, y - h),
        (x + h, y - h),
        (x + h, y + h),
        (x - h, y + h),
    ];
    let tri = [(0.2985, 0.3935), (0.2985, 0.3975), (0.2955, 0.3965)];
    for (what, pts) in [("square", &square[..]), ("triangle", &tri[..])] {
        let area = {
            let mut s = 0.0;
            for i in 0..pts.len() {
                let (p, q) = (pts[i], pts[(i + 1) % pts.len()]);
                s += p.0 * q.1 - q.0 * p.1;
            }
            (0.5 * s).abs()
        };
        let body = rod_with(vec![poly(pts)]);
        carve(&body, ROD_FILLET, -two_slivers(1.0) + area * ROD_L, what)
            .unwrap_or_else(|e| panic!("{what}: goes, got {e:?}"));
    }
}

/// **At and around tangency to the arc and the flat**: a bore that
/// touches or crosses the boundary refuses; one inside either goes at
/// the closed form or refuses — never a wrong body.
#[test]
fn p06_tangent_and_escalate_width_offsets_never_build_wrong() {
    let a = 0.002;
    let cx = ROD_FLAT - ROD_FILLET;
    let c = (cx, ((ROD_R - ROD_FILLET).powi(2) - cx * cx).sqrt());
    let v = (ROD_FLAT, (ROD_R * ROD_R - ROD_FLAT * ROD_FLAT).sqrt());
    let (dx, dy) = (v.0 - c.0, v.1 - c.1);
    let n = dx.hypot(dy);
    let want = -two_slivers(1.0) + core::f64::consts::PI * a * a * ROD_L;
    for off in [-1e-9, -1e-12, -1e-14, 0.0, 1e-14, 1e-12, 1e-9, 1e-7] {
        let d = ROD_FILLET + a + off;
        let (x, y) = (c.0 + d * dx / n, c.1 + d * dy / n);
        let body = rod_with(vec![round(x, y, a)]);
        let r = carve(&body, ROD_FILLET, want, &format!("arc off {off}"));
        eprintln!("arc offset {off:e}: {r:?}");
        if off <= 0.0 {
            assert!(
                r.is_err(),
                "arc offset {off:e}: a touching/crossing bore built"
            );
        }
    }
    for off in [2e-8, 1e-7, 1e-6] {
        let body = rod_with(vec![round(ROD_FLAT - a - off, 0.395, a)]);
        let r = carve(&body, ROD_FILLET, want, &format!("flat off {off}"));
        eprintln!("flat offset {off:e}: {r:?}");
    }
}

/// **A slot from the sliver into the kept ball** (one feature, crossing
/// the arc) refuses as a crossing, not as wholly inside and not built.
#[test]
fn p07_a_slot_from_the_sliver_into_kept_material_refuses_as_a_crossing() {
    let slot = poly(&[
        (0.2935, 0.36),
        (0.2965, 0.36),
        (0.2965, 0.3945),
        (0.2935, 0.3945),
    ]);
    let body = rod_with(vec![slot]);
    let creases = rod_creases(&body);
    let got = fillet_edges(&body, &creases, ROD_FILLET, tol()).map(|_| ());
    let Err(e) = got else {
        panic!("a slot crossing the arc was built")
    };
    assert!(
        !matches!(e.error, BlendError::WhollyInside { .. }),
        "a crossing slot refuses as a crossing or a bound: {:?}",
        e.error
    );
    eprintln!("slot: {:?}", e.error);
}

/// **Several features**: a bore in each crease's sliver and a second
/// in one, all go at the closed form.
#[test]
fn p08_three_features_in_two_slivers_go_at_the_closed_form() {
    let a = 0.0012;
    let body = rod_with(vec![
        round(0.297, 0.395, a),
        round(0.297, -0.395, a),
        round(0.2905, 0.405, 0.001),
    ]);
    let area = core::f64::consts::PI * (2.0 * a * a + 0.001 * 0.001);
    carve(&body, ROD_FILLET, -two_slivers(1.0) + area * ROD_L, "three")
        .unwrap_or_else(|e| panic!("three features go, got {e:?}"));
}

/// **An inside bore beside a bore clear of the band** (the rod's
/// centre): the inside one goes, the clear one stays, at the closed
/// form.
#[test]
fn p09a_an_inside_bore_beside_a_clear_bore_goes_and_the_clear_one_stays() {
    let (a, b) = (0.002, 0.05);
    let want = -two_slivers(1.0) + core::f64::consts::PI * a * a * ROD_L;
    for (k, loops) in [
        vec![round(0.297, 0.395, a), round(0.0, 0.0, b)],
        vec![round(0.0, 0.0, b), round(0.297, 0.395, a)],
    ]
    .into_iter()
    .enumerate()
    {
        carve(&rod_with(loops), ROD_FILLET, want, &format!("order {k}"))
            .unwrap_or_else(|e| panic!("order {k}: builds, got {e:?}"));
    }
}

/// **An inside bore with a bore in the kept ball just short of the
/// arc** (neither inside nor certified clear) refuses the same in
/// either profile order, and not as wholly inside.
#[test]
fn p09b_an_inside_bore_and_a_bounded_bore_refuse_the_same_in_either_order() {
    let cx = ROD_FLAT - ROD_FILLET;
    let c = (cx, ((ROD_R - ROD_FILLET).powi(2) - cx * cx).sqrt());
    let v = (ROD_FLAT, (ROD_R * ROD_R - ROD_FLAT * ROD_FLAT).sqrt());
    let (dx, dy) = (v.0 - c.0, v.1 - c.1);
    let n = dx.hypot(dy);
    let a = 0.002;
    let d = ROD_FILLET - a - 0.0005;
    let near = (c.0 + d * dx / n - 0.004, c.1 + d * dy / n);
    let mut errs = Vec::new();
    for loops in [
        vec![round(0.297, 0.395, a), round(near.0, near.1, a)],
        vec![round(near.0, near.1, a), round(0.297, 0.395, a)],
    ] {
        let body = rod_with(loops);
        let want = -two_slivers(1.0) + core::f64::consts::PI * a * a * ROD_L;
        match carve(&body, ROD_FILLET, want, "inside + near") {
            Ok(()) => {
                eprintln!("built at the closed form");
                errs.push("built".to_string());
            }
            Err(e) => {
                assert!(!matches!(e, BlendError::WhollyInside { .. }), "{e:?}");
                eprintln!("refused {e:?}");
                errs.push(format!("{:?}", core::mem::discriminant(&e)));
            }
        }
    }
    assert_eq!(errs[0], errs[1], "the outcome is order-independent");
}

/// **At Interval**: the witness at tiers 3 and 3′, and its volume at
/// the closed form (the lane's row checks only a census).
#[test]
fn p10_the_witness_at_interval_is_valid_and_at_the_closed_form() {
    let body = finished(
        "bored",
        extruded(
            SketchPlane::xy(),
            vec![d_loop(1.0), round::<Interval>(0.297, 0.395, 0.002)],
            Interval::from_f64(ROD_L),
            tol(),
        ),
        tol(),
    );
    let creases = rod_creases(&body);
    let out = fillet_edges(&body, &creases, Interval::from_f64(ROD_FILLET), tol())
        .unwrap_or_else(|e| panic!("goes at Interval, got {e}"));
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("tier 3, {e:?}"));
    validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
        .unwrap_or_else(|e| panic!("tier 3′, {e:?}"));
    sweep::test_support::assert_naming_totality(&body, &out, &creases, "interval");
    let v = |b: &Body<Interval>| mass_properties(b, tol()).expect("props").volume;
    let dv = v(&out.body) - v(&body);
    let want = -two_slivers(1.0) + core::f64::consts::PI * 0.002 * 0.002 * ROD_L;
    assert!(
        dv.lo() - 1e-9 <= want && want <= dv.hi() + 1e-9,
        "ΔV {dv:?} vs {want}"
    );
}

/// The rod (with `holes`) cut off at a plane through `(0, 0, 1)` tilted
/// `angle` about `axis` (in the xy plane): its top cap is not normal to
/// the spine.
fn tilted_cap_rod(holes: Vec<ProfileLoop<f64>>, axis: Vec3<f64>, angle: f64) -> AtRestBody<f64> {
    let mut loops = vec![d_loop(1.0)];
    loops.extend(holes);
    let rod = finished(
        "long rod",
        extruded(SketchPlane::xy(), loops, 1.6, tol()),
        tol(),
    );
    let cutter = finished(
        "cutter",
        sweep::test_support::brick((-2.0, 2.0), (-2.0, 2.0), (1.0, 4.0), tol()),
        tol(),
    );
    let m = Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 1.0), axis, angle);
    let cutter = finished(
        "tilted cutter",
        topo::transform_rigid(&cutter, &m, tol()).expect("rigid"),
        tol(),
    );
    let cut = topo::subtract(&rod, &cutter, tol()).expect("the cut builds");
    cut.body().expect("a body").body.clone()
}

/// **Tilted caps** (`|n·τ| < 1`): the witness bore goes, and the bored
/// rod's carve has the unbored rod's volume (the bore lies wholly in
/// the material removed), at tiers 3 / 3′.
#[test]
fn p11_a_tilted_cap_carves_the_bored_rod_to_the_unbored_rods_volume() {
    for (axis, angle) in [
        (Vec3::new(1.0, 0.0, 0.0), 0.3),
        (Vec3::new(0.0, 1.0, 0.0), -0.35),
        (Vec3::new(1.0, 1.0, 0.0), 0.5),
    ] {
        let what = format!("tilt {angle} about {axis:?}");
        let plain = tilted_cap_rod(Vec::new(), axis, angle);
        let creases = rod_creases(&plain);
        let Ok(p) = fillet_edges(&plain, &creases, ROD_FILLET, tol()) else {
            eprintln!("{what}: the unbored tilted rod refuses; no oracle");
            continue;
        };
        let target = volume(&p.body);
        let a = 0.002;
        let bored = tilted_cap_rod(vec![round(0.297, 0.395, a)], axis, angle);
        let want = target - volume(&bored);
        match carve(&bored, ROD_FILLET, want, &what) {
            Ok(()) => eprintln!("{what}: consumed at the unbored volume"),
            Err(e) => eprintln!("{what}: refused {e:?}"),
        }
    }
}

/// A one-segment circle (one edge per ring) of radius `a` about
/// `(x, y)`, its vertex at azimuth `az`.
fn one_edge(x: f64, y: f64, a: f64, az: f64) -> ProfileLoop<f64> {
    profile::RawLoop::new([(
        Point2::new(x + a * az.cos(), y + a * az.sin()),
        profile::Segment::Arc(geom_core::Arc2 {
            centre: Point2::new(x, y),
            radius: a,
            sweep: core::f64::consts::TAU,
        }),
    )])
}

/// A circle of `n` equal arcs.
fn n_arcs(x: f64, y: f64, a: f64, n: usize) -> ProfileLoop<f64> {
    let step = core::f64::consts::TAU / n as f64;
    profile::RawLoop::new((0..n).map(|i| {
        let t = i as f64 * step + 0.4;
        (
            Point2::new(x + a * t.cos(), y + a * t.sin()),
            profile::Segment::Arc(geom_core::Arc2 {
                centre: Point2::new(x, y),
                radius: a,
                sweep: step,
            }),
        )
    }))
}

/// **The one-edge bore at tier 3′** (the lane's only one-edge row is
/// editor-core's, tier 3 and ΔV), at several seam azimuths, and a
/// three-arc bore: each goes at the closed form.
#[test]
fn p12_one_edge_and_three_arc_bores_go_at_tier_3_prime() {
    let a = 0.002;
    let want = -two_slivers(1.0) + core::f64::consts::PI * a * a * ROD_L;
    for az in [0.0, 1.3, 3.0, 4.9] {
        let body = rod_with(vec![one_edge(0.297, 0.395, a, az)]);
        carve(&body, ROD_FILLET, want, &format!("one edge, az {az}"))
            .unwrap_or_else(|e| panic!("one edge at {az}: goes, got {e:?}"));
    }
    let body = rod_with(vec![n_arcs(0.297, 0.395, a, 3)]);
    carve(&body, ROD_FILLET, want, "three arcs").unwrap_or_else(|e| panic!("three arcs: {e:?}"));
}

/// **What `prism_inside`'s ruling direction becomes for a plane normal
/// to the spine**: `(τ − n(n·τ)).normalize()` with `n = τ` exactly, at
/// `f64` and at Interval, and what the reading's `sure` (`lo ≥
/// escalate`) makes of an excess carrying it.
#[test]
fn p13_a_degenerate_ruling_direction_at_each_scalar() {
    let tau = Vec3::new(0.0, 0.0, 1.0);
    let a = (tau - tau * tau.dot(tau)).normalize();
    let excess = 0.01 - a.cross(tau).norm();
    eprintln!(
        "f64: a = {a:?}, excess = {excess}, sure = {}",
        excess >= 1e-8
    );
    let ti = Vec3::new(
        Interval::from_f64(0.0),
        Interval::from_f64(0.0),
        Interval::from_f64(1.0),
    );
    let ai = (ti - ti * ti.dot(ti)).normalize();
    let ei = Interval::from_f64(0.01) - ai.cross(ti).norm();
    eprintln!(
        "interval: a = {ai:?}, excess = {ei:?}, lo = {}, sure = {}",
        ei.lo(),
        ei.lo() >= 1e-8
    );
}
