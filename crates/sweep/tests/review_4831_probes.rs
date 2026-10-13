//! Tangent poses through the claim-free glue door, each run through all
//! four ops. A body must match its closed-form volume, pass tier 3 and
//! answer point membership as the oracle says; each pose then pins which
//! ops answer and which refuse, by kind.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, Point3, Tol};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop};
use sweep::ExtrudeSide;
use sweep::test_support::{brick, finished, sketch_at};
use sweep::{Extrusion, extrude};
use topo::{AtRestBody, Body, SolidContainment};

fn tol() -> Tol {
    Tol::witness()
}

fn circle(cx: f64, cy: f64, r: f64) -> ProfileLoop<f64> {
    bulge_loop(vec![
        (Point2::new(cx + r, cy), 1.0),
        (Point2::new(cx - r, cy), 1.0),
    ])
}

fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> ProfileLoop<f64> {
    bulge_loop(vec![
        (Point2::new(x0, y0), 0.0),
        (Point2::new(x1, y0), 0.0),
        (Point2::new(x1, y1), 0.0),
        (Point2::new(x0, y1), 0.0),
    ])
}

fn ext(what: &str, loops: Vec<ProfileLoop<f64>>, z0: f64, h: f64) -> AtRestBody<f64> {
    let profile = Profile::new(sketch_at(z0), loops).validate(tol()).unwrap();
    let body: Body<f64> = extrude(
        &profile,
        Extrusion::Distance {
            depth: h,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    finished(what, body, tol())
}

type Oracle = fn(Point3<f64>) -> Option<bool>;

/// Runs one op; returns "OK", "REFUSED <kind>" or panics on a wrong body.
fn check(
    pose: &str,
    name: &str,
    got: Result<topo::BooleanResult<f64>, topo::BooleanError>,
    want: Option<f64>,
    inside: impl Fn(Point3<f64>) -> Option<bool>,
    probes: &[Point3<f64>],
) -> String {
    let band = Band::linear(tol()).unwrap();
    let eps = tol().get().eps;
    match got {
        Ok(topo::BooleanResult::Body(b)) => {
            let v = topo::mass_properties(&b.body, tol()).unwrap().volume;
            let want = want.unwrap_or_else(|| panic!("{pose} {name}: body {v}, want Empty"));
            assert!(
                (v - want).abs() < 1e-6,
                "WRONG BODY {pose} {name} eps {eps}: volume {v}, want {want}"
            );
            let t3 = topo::validate_geometric(&b.body, tol());
            assert_eq!(t3, Ok(()), "{pose} {name} eps {eps}: tier 3");
            for &q in probes {
                if let Some(want_in) = inside(q) {
                    let got = topo::point_in_solid(&b.body, q, band, tol()).unwrap();
                    let want_c = if want_in {
                        SolidContainment::In
                    } else {
                        SolidContainment::Out
                    };
                    assert_eq!(got, want_c, "WRONG BODY {pose} {name} eps {eps}: at {q:?}");
                }
            }
            format!("OK body {v:.9}")
        }
        Ok(topo::BooleanResult::Empty) => {
            assert!(
                want.is_none(),
                "WRONG BODY {pose} {name} eps {eps}: Empty, want {want:?}"
            );
            "OK Empty".into()
        }
        Err(e) => format!(
            "REFUSED {:?}: {}",
            e.kind(),
            e.to_string().chars().take(700).collect::<String>()
        ),
    }
}

/// The four ops' outcomes, in the order ∪, A ∖ B, B ∖ A, ∩.
fn four(
    pose: &str,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    in_a: Oracle,
    in_b: Oracle,
    want: [Option<f64>; 4],
    probes: &[Point3<f64>],
) -> Vec<String> {
    let and = |p, q| match (p, q) {
        (Some(x), Some(y)) => Some((x, y)),
        _ => None,
    };
    let eps = tol().get().eps;
    let rows = [
        (
            "A ∪ B",
            topo::union(a, b, tol()),
            &(|q| and(in_a(q), in_b(q)).map(|(x, y)| x || y))
                as &dyn Fn(Point3<f64>) -> Option<bool>,
        ),
        (
            "A ∖ B",
            topo::subtract(a, b, tol()),
            &(|q| and(in_a(q), in_b(q)).map(|(x, y)| x && !y)),
        ),
        (
            "B ∖ A",
            topo::subtract(b, a, tol()),
            &(|q| and(in_a(q), in_b(q)).map(|(x, y)| y && !x)),
        ),
        (
            "A ∩ B",
            topo::intersect(a, b, tol()),
            &(|q| and(in_a(q), in_b(q)).map(|(x, y)| x && y)),
        ),
    ];
    let mut out = Vec::new();
    for (i, (name, got, f)) in rows.into_iter().enumerate() {
        let r = check(pose, name, got, want[i], f, probes);
        eprintln!("{pose}\teps={eps:e}\t{name}\t{r}");
        out.push(r);
    }
    out
}

/// Every op answered.
fn all_answer(pose: &str, got: &[String]) {
    assert!(got.iter().all(|r| r.starts_with("OK")), "{pose}: {got:#?}");
}

fn margin(d: f64) -> Option<bool> {
    (d.abs() > 1e-3).then_some(d < 0.0)
}
fn disc(q: Point3<f64>, cx: f64, cy: f64, r: f64) -> f64 {
    ((q.x - cx).powi(2) + (q.y - cy).powi(2)).sqrt() - r
}
fn slab(q: Point3<f64>, z0: f64, z1: f64) -> f64 {
    (z0 - q.z).max(q.z - z1)
}
fn boxd(q: Point3<f64>, x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> f64 {
    (x.0 - q.x)
        .max(q.x - x.1)
        .max(y.0 - q.y)
        .max(q.y - y.1)
        .max(z.0 - q.z)
        .max(q.z - z.1)
}

fn grid() -> Vec<Point3<f64>> {
    let mut out = Vec::new();
    for i in 0..9 {
        for j in 0..9 {
            for z in [0.25, 0.75, 1.5] {
                out.push(Point3::new(
                    -3.1 + 0.775 * f64::from(i),
                    -3.1 + 0.775 * f64::from(j),
                    z,
                ));
            }
        }
    }
    out.extend([
        Point3::new(1.5, 0.0, 0.5),
        Point3::new(1.9, 0.0, 0.5),
        Point3::new(1.95, 0.2, 0.5),
        Point3::new(0.0, 1.5, 0.5),
        Point3::new(2.1, 0.0, 0.5),
        Point3::new(0.9, 0.0, 0.5),
        Point3::new(1.1, 0.0, 0.75),
    ]);
    out
}

/// Opposed normals, SHARED crescent: a bore (r 1 about (1, 0)) in a
/// 6 × 6 plate, touching a solid disc (r 2 about the origin) from inside,
/// the disc's caps coplanar with the plate's. The pair glues `Tangent`,
/// and every op refuses `Join`: at the line's end vertices the disc's cap
/// sector (a coplanar lump) and its wall sector (a tangent lump) share the
/// cap's circle edge as a bound, the vertex shape of
/// `work/contacthold/a-dipping-round-refuses-at-a-run-boundary-on-its-coplanar-bottom-sector.md`.
/// The same pair with a taller disc answers every op
/// ([`p1_a_bore_inside_a_tall_disc_touching_its_wall`]).
#[test]
fn p1_a_bore_inside_a_disc_touching_its_wall() {
    let pi = std::f64::consts::PI;
    let a = ext("disc r2", vec![circle(0.0, 0.0, 2.0)], 0.0, 1.0);
    let b = ext(
        "bored plate",
        vec![rect(-3.0, -3.0, 3.0, 3.0), circle(1.0, 0.0, 1.0)],
        0.0,
        1.0,
    );
    let got = four(
        "p1 bore-in-disc",
        &a,
        &b,
        |q| margin(disc(q, 0.0, 0.0, 2.0).max(slab(q, 0.0, 1.0))),
        |q| margin(boxd(q, (-3.0, 3.0), (-3.0, 3.0), (0.0, 1.0)).max(-disc(q, 1.0, 0.0, 1.0))),
        [Some(36.0), Some(pi), Some(36.0 - 4.0 * pi), Some(3.0 * pi)],
        &grid(),
    );
    assert!(
        got.iter().all(|r| r.starts_with("REFUSED Join")),
        "p1: the dip row's refusal at every op: {got:#?}"
    );
}

/// [`p1_a_bore_inside_a_disc_touching_its_wall`] with the disc run past
/// the plate both ways (z ∈ [−1, 2]), so no cap is coplanar: the
/// overlapping tangency answers every op.
#[test]
fn p1_a_bore_inside_a_tall_disc_touching_its_wall() {
    let pi = std::f64::consts::PI;
    let a = ext("tall disc r2", vec![circle(0.0, 0.0, 2.0)], -1.0, 3.0);
    let b = ext(
        "bored plate",
        vec![rect(-3.0, -3.0, 3.0, 3.0), circle(1.0, 0.0, 1.0)],
        0.0,
        1.0,
    );
    let got = four(
        "p1 bore-in-tall-disc",
        &a,
        &b,
        |q| margin(disc(q, 0.0, 0.0, 2.0).max(slab(q, -1.0, 2.0))),
        |q| margin(boxd(q, (-3.0, 3.0), (-3.0, 3.0), (0.0, 1.0)).max(-disc(q, 1.0, 0.0, 1.0))),
        [
            Some(36.0 + 8.0 * pi),
            Some(9.0 * pi),
            Some(36.0 - 4.0 * pi),
            Some(3.0 * pi),
        ],
        &grid(),
    );
    all_answer("p1 tall", &got);
}

/// Aligned normals, a bore inside a bigger bore: plate − D(0,0,2) and
/// plate − D(1,0,1). B ∖ A is the lune with a cusp along the line.
#[test]
fn p2_a_bore_inside_a_bigger_bore() {
    let pi = std::f64::consts::PI;
    let a = ext(
        "big bore",
        vec![rect(-3.0, -3.0, 3.0, 3.0), circle(0.0, 0.0, 2.0)],
        0.0,
        1.0,
    );
    let b = ext(
        "small bore",
        vec![rect(-3.0, -3.0, 3.0, 3.0), circle(1.0, 0.0, 1.0)],
        0.0,
        1.0,
    );
    let got = four(
        "p2 bore-in-bore",
        &a,
        &b,
        |q| margin(boxd(q, (-3.0, 3.0), (-3.0, 3.0), (0.0, 1.0)).max(-disc(q, 0.0, 0.0, 2.0))),
        |q| margin(boxd(q, (-3.0, 3.0), (-3.0, 3.0), (0.0, 1.0)).max(-disc(q, 1.0, 0.0, 1.0))),
        [Some(36.0 - pi), None, Some(3.0 * pi), Some(36.0 - 4.0 * pi)],
        &grid(),
    );
    all_answer("p2", &got);
}

/// How far off tangency [`p3_a_nested_touch_just_off_tangent`] moves
/// the thin disc: just outside the band at every ε.
fn off_tangent() -> f64 {
    1e4 * tol().get().eps
}

/// Near-tangent just outside the band: the nested aligned touch with the
/// thin disc moved `d` ([`off_tangent`]) inward (clear, every op at its closed form) and
/// outward (crossing in a thin lens, every body tier-3 clean and the four
/// volumes consistent with the operands'; at ε 1e-12 every outward op
/// refuses `CurvedPierceUnsupported`).
#[test]
fn p3_a_nested_touch_just_off_tangent() {
    let pi = std::f64::consts::PI;
    let d = off_tangent();
    let a = ext("fat", vec![circle(0.0, 0.0, 2.0)], 0.0, 1.0);
    let b = ext("thin", vec![circle(1.0 - d, 0.0, 1.0)], 0.0, 1.0);
    let got = four(
        "p3 nested off inward",
        &a,
        &b,
        |q| margin(disc(q, 0.0, 0.0, 2.0).max(slab(q, 0.0, 1.0))),
        |q| margin(disc(q, 1.0 - off_tangent(), 0.0, 1.0).max(slab(q, 0.0, 1.0))),
        [Some(4.0 * pi), Some(3.0 * pi), None, Some(pi)],
        &grid(),
    );
    all_answer("p3 inward", &got);
    let b = ext("thin", vec![circle(1.0 + d, 0.0, 1.0)], 0.0, 1.0);
    // At ε 1e-12 every op refuses typed: the thin disc's edges cross
    // the fat disc's wall where no closed form reads the pierce.
    let fine = tol().get().eps == 1e-12;
    let volume = |name: &str, got: Result<topo::BooleanResult<f64>, topo::BooleanError>| match got {
        Ok(topo::BooleanResult::Body(r)) => {
            assert_eq!(
                topo::validate_geometric(&r.body, tol()),
                Ok(()),
                "p3 outward {name}: tier 3"
            );
            Some(topo::mass_properties(&r.body, tol()).unwrap().volume)
        }
        Err(e) if fine && e.kind() == topo::BooleanErrorKind::CurvedPierceUnsupported => {
            eprintln!("p3 outward {name}: refused {e}");
            None
        }
        other => panic!("p3 outward {name}: {other:?}"),
    };
    let union = volume("∪", topo::union(&a, &b, tol()));
    let a_b = volume("A ∖ B", topo::subtract(&a, &b, tol()));
    let b_a = volume("B ∖ A", topo::subtract(&b, &a, tol()));
    let meet = volume("∩", topo::intersect(&a, &b, tol()));
    if fine {
        assert!(
            [union, a_b, b_a, meet].iter().all(Option::is_none),
            "p3 outward at ε 1e-12: every op refuses, {union:?} {a_b:?} {b_a:?} {meet:?}"
        );
        return;
    }
    let (Some(union), Some(a_b), Some(b_a), Some(meet)) = (union, a_b, b_a, meet) else {
        unreachable!("an op refused, and only at ε 1e-12 does one")
    };
    for (what, got, want) in [
        ("∪ + ∩ = A + B", union + meet, 5.0 * pi),
        ("A ∖ B + ∩ = A", a_b + meet, 4.0 * pi),
        ("B ∖ A + ∩ = B", b_a + meet, pi),
    ] {
        assert!(
            (got - want).abs() < 1e-9,
            "p3 outward {what}: {got} vs {want}"
        );
    }
    assert!(b_a > 0.0, "p3 outward: the lens is kept");
}

/// A witness on part of the locus: the cutter's face `x = 1` covers only
/// z ∈ [0.5, 1] of the hole's ruling.
#[test]
fn p4_a_plane_tangent_to_part_of_a_hole() {
    let pi = std::f64::consts::PI;
    let a = ext(
        "holed plate",
        vec![rect(-3.0, -3.0, 3.0, 3.0), circle(0.0, 0.0, 1.0)],
        0.0,
        1.0,
    );
    let b = finished(
        "cutter",
        brick::<f64>((1.0, 4.0), (-4.0, 4.0), (0.5, 2.0), tol()),
        tol(),
    );
    let got = four(
        "p4 plane tangent to part of a hole",
        &a,
        &b,
        |q| margin(boxd(q, (-3.0, 3.0), (-3.0, 3.0), (0.0, 1.0)).max(-disc(q, 0.0, 0.0, 1.0))),
        |q| margin(boxd(q, (1.0, 4.0), (-4.0, 4.0), (0.5, 2.0))),
        [Some(66.0 - pi), Some(30.0 - pi), Some(30.0), Some(6.0)],
        &grid(),
    );
    all_answer("p4", &got);
}

/// `wedge_end_doors::a_plane_tangent_to_a_hole_answers_every_op`, read
/// by point membership as well.
#[test]
fn p5_the_plane_tangent_to_a_hole_by_membership() {
    let pi = std::f64::consts::PI;
    let a = ext(
        "holed plate",
        vec![rect(-3.0, -3.0, 3.0, 3.0), circle(0.0, 0.0, 1.0)],
        0.0,
        1.0,
    );
    let b = finished(
        "cutter",
        brick::<f64>((1.0, 4.0), (-4.0, 4.0), (-1.0, 2.0), tol()),
        tol(),
    );
    let got = four(
        "p5 plane tangent to a hole",
        &a,
        &b,
        |q| margin(boxd(q, (-3.0, 3.0), (-3.0, 3.0), (0.0, 1.0)).max(-disc(q, 0.0, 0.0, 1.0))),
        |q| margin(boxd(q, (1.0, 4.0), (-4.0, 4.0), (-1.0, 2.0))),
        [Some(96.0 - pi), Some(24.0 - pi), Some(60.0), Some(12.0)],
        &grid(),
    );
    all_answer("p5", &got);
}

/// `wedge_end_doors::a_nested_aligned_touch_answers_every_op`, read by
/// point membership as well.
#[test]
fn p6_the_nested_aligned_touch_by_membership() {
    let pi = std::f64::consts::PI;
    let a = ext("fat", vec![circle(0.0, 0.0, 2.0)], 0.0, 1.0);
    let b = ext("thin", vec![circle(1.0, 0.0, 1.0)], 0.0, 1.0);
    let got = four(
        "p6 nested aligned touch",
        &a,
        &b,
        |q| margin(disc(q, 0.0, 0.0, 2.0).max(slab(q, 0.0, 1.0))),
        |q| margin(disc(q, 1.0, 0.0, 1.0).max(slab(q, 0.0, 1.0))),
        [Some(4.0 * pi), Some(3.0 * pi), None, Some(pi)],
        &grid(),
    );
    all_answer("p6", &got);
}

/// Opposed normals, VOID crescent, a line through a plane face's
/// interior: a disc (r 2 about the origin) against the face `x = 2` of a
/// slab. The union refuses `TangentSlitArmUnbuilt`; ∖ and ∩ answer.
#[test]
fn p7_a_disc_against_a_slab_face() {
    let pi = std::f64::consts::PI;
    let a = ext("disc r2", vec![circle(0.0, 0.0, 2.0)], 0.0, 1.0);
    let b = finished(
        "slab",
        brick::<f64>((2.0, 4.0), (-3.0, 3.0), (-1.0, 2.0), tol()),
        tol(),
    );
    let got = four(
        "p7 disc against a slab face",
        &a,
        &b,
        |q| margin(disc(q, 0.0, 0.0, 2.0).max(slab(q, 0.0, 1.0))),
        |q| margin(boxd(q, (2.0, 4.0), (-3.0, 3.0), (-1.0, 2.0))),
        [Some(4.0 * pi + 36.0), Some(4.0 * pi), Some(36.0), None],
        &grid(),
    );
    assert!(
        got[0].starts_with("REFUSED TangentSlitArmUnbuilt"),
        "p7 ∪: {}",
        got[0]
    );
    all_answer("p7 ∖ and ∩", &got[1..]);
}

/// The nested aligned touch with the thin disc run past the fat one both
/// ways (z ∈ [−1, 2]): every op answers.
#[test]
fn p8_a_long_thin_disc_nested_in_a_fat_one() {
    let pi = std::f64::consts::PI;
    let a = ext("fat", vec![circle(0.0, 0.0, 2.0)], 0.0, 1.0);
    let b = ext("thin long", vec![circle(1.0, 0.0, 1.0)], -1.0, 3.0);
    let got = four(
        "p8 long thin disc in a fat one",
        &a,
        &b,
        |q| margin(disc(q, 0.0, 0.0, 2.0).max(slab(q, 0.0, 1.0))),
        |q| margin(disc(q, 1.0, 0.0, 1.0).max(slab(q, -1.0, 2.0))),
        [Some(6.0 * pi), Some(3.0 * pi), Some(2.0 * pi), Some(pi)],
        &grid(),
    );
    all_answer("p8", &got);
}

/// A rod (r 1 about (1, 0), z ∈ [−1, 2]) seated in a plate's bore (r 2
/// about the origin), touching its wall along a ruling: every op answers.
#[test]
fn p9_a_rod_seated_in_a_plates_bore() {
    let pi = std::f64::consts::PI;
    let a = ext(
        "bored plate",
        vec![rect(-3.0, -3.0, 3.0, 3.0), circle(0.0, 0.0, 2.0)],
        0.0,
        1.0,
    );
    let b = ext("rod", vec![circle(1.0, 0.0, 1.0)], -1.0, 3.0);
    let got = four(
        "p9 rod in a plate's bore",
        &a,
        &b,
        |q| margin(boxd(q, (-3.0, 3.0), (-3.0, 3.0), (0.0, 1.0)).max(-disc(q, 0.0, 0.0, 2.0))),
        |q| margin(disc(q, 1.0, 0.0, 1.0).max(slab(q, -1.0, 2.0))),
        [Some(36.0 - pi), Some(36.0 - 4.0 * pi), Some(3.0 * pi), None],
        &grid(),
    );
    all_answer("p9", &got);
}

/// The sliver family: a fat disc (r 2, z ∈ [0, h]) nested on a long
/// thin one (r 1 about (1, 0), z ∈ [−1, 1 + 2h]), internally tangent
/// along the ruling through (2, 0), across thicknesses `h` down to where
/// the fat disc's ruling edge is too short for its height off the
/// tangent plane to show. Every op answers at its closed form, tier 3
/// clean, or refuses typed where [`sliver_refusal`] pins it, both
/// operand orders; never `ClassificationInvariant`.
#[test]
fn p10_the_sliver_family() {
    let pi = std::f64::consts::PI;
    let eps = tol().get().eps;
    for h in [1e-2, 1e-3, 1e-4, 3e-5, 1e-5, 3e-6, 1e-6] {
        let pinned = sliver_refusal(eps, h);
        let built = (
            try_ext(vec![circle(0.0, 0.0, 2.0)], 0.0, h),
            try_ext(vec![circle(1.0, 0.0, 1.0)], -1.0, 1.0 + 2.0 * h),
        );
        let (Ok(a), Ok(b)) = built else {
            assert_eq!(
                pinned,
                Some(Sliver::Unbuilt),
                "p10 h={h:e} eps {eps:e}: the extrusion refused {built:?}"
            );
            continue;
        };
        let (va, vb, meet) = (4.0 * pi * h, pi * (1.0 + 2.0 * h), pi * h);
        for (name, got, want) in [
            ("A ∪ B", topo::union(&a, &b, tol()), va + vb - meet),
            ("A ∖ B", topo::subtract(&a, &b, tol()), va - meet),
            ("B ∖ A", topo::subtract(&b, &a, tol()), vb - meet),
            ("A ∩ B", topo::intersect(&a, &b, tol()), meet),
            ("B ∪ A", topo::union(&b, &a, tol()), va + vb - meet),
            ("B ∩ A", topo::intersect(&b, &a, tol()), meet),
        ] {
            let row = format!("p10 h={h:e} {name} eps {eps:e}");
            match got {
                Ok(topo::BooleanResult::Body(r)) => {
                    let v = topo::mass_properties(&r.body, tol()).unwrap().volume;
                    assert!(
                        (v - want).abs() < 1e-9 * want.max(1.0),
                        "WRONG BODY {row}: volume {v} vs {want}"
                    );
                    assert_eq!(
                        topo::validate_geometric(&r.body, tol()),
                        Ok(()),
                        "{row}: tier 3"
                    );
                    assert!(
                        pinned.is_none() || pinned == Some(Sliver::Free),
                        "{row}: answered where {pinned:?} is pinned"
                    );
                }
                Ok(topo::BooleanResult::Empty) => panic!("WRONG BODY {row}: empty"),
                Err(e) => {
                    let kind = e.kind();
                    eprintln!("{row}: refused {kind:?}: {e}");
                    assert_ne!(
                        kind,
                        topo::BooleanErrorKind::ClassificationInvariant,
                        "{row}: {e}"
                    );
                    assert!(
                        matches!(pinned, Some(Sliver::Refuses(k)) if k == kind)
                            || pinned == Some(Sliver::Free),
                        "{row}: refused {kind:?} where {pinned:?} is pinned"
                    );
                }
            }
        }
    }
}

/// What a sliver-family row does at a sampled ε other than answering.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Sliver {
    /// The fat disc's extrusion refuses: `h` lies in the band.
    Unbuilt,
    /// Every op refuses with this kind.
    Refuses(topo::BooleanErrorKind),
    /// An ε the table does not sample: an op answers or refuses typed.
    Free,
}

/// The pinned non-answers of [`p10_the_sliver_family`]. At ε 1e-6 the
/// escalation is a vertex `h` from a face read in band; at 1e-9 and
/// 1e-12, a bisector bound levered at its sector's arm `h` reads its
/// second-order departure in band:
/// `work/contacthold/a-glued-tangencys-bisector-reads-its-curving-at-its-sectors-short-arm.md`.
fn sliver_refusal(eps: f64, h: f64) -> Option<Sliver> {
    use topo::BooleanErrorKind::Escalated;
    if ![1e-6, 1e-9, 1e-12].contains(&eps) {
        return Some(Sliver::Free);
    }
    match (eps, h) {
        (1e-6, 1e-5) | (1e-9, 1e-4) | (1e-12, 3e-6) => Some(Sliver::Refuses(Escalated)),
        (1e-6, _) if h < 1e-5 => Some(Sliver::Unbuilt),
        _ => None,
    }
}

/// [`ext`] that hands back the extrusion's refusal.
fn try_ext(
    loops: Vec<ProfileLoop<f64>>,
    z0: f64,
    h: f64,
) -> Result<AtRestBody<f64>, sweep::ExtrudeError> {
    let profile = Profile::new(sketch_at(z0), loops).validate(tol()).unwrap();
    let body: Body<f64> = extrude(
        &profile,
        Extrusion::Distance {
            depth: h,
            side: ExtrudeSide::Along,
        },
        tol(),
    )?
    .body;
    Ok(finished("sliver", body, tol()))
}

/// Operand order swapped for p4, p8, p9 (the four ops in B, A order).
#[test]
fn p11_the_new_rows_in_the_other_order() {
    let pi = std::f64::consts::PI;
    let hole = ext(
        "holed plate",
        vec![rect(-3.0, -3.0, 3.0, 3.0), circle(0.0, 0.0, 1.0)],
        0.0,
        1.0,
    );
    let cut = finished(
        "cutter",
        brick::<f64>((1.0, 4.0), (-4.0, 4.0), (0.5, 2.0), tol()),
        tol(),
    );
    let got = four(
        "p11 p4 swapped",
        &cut,
        &hole,
        |q| margin(boxd(q, (1.0, 4.0), (-4.0, 4.0), (0.5, 2.0))),
        |q| margin(boxd(q, (-3.0, 3.0), (-3.0, 3.0), (0.0, 1.0)).max(-disc(q, 0.0, 0.0, 1.0))),
        [Some(66.0 - pi), Some(30.0), Some(30.0 - pi), Some(6.0)],
        &grid(),
    );
    all_answer("p11 p4 swapped", &got);
    let fat = ext("fat", vec![circle(0.0, 0.0, 2.0)], 0.0, 1.0);
    let thin = ext("thin long", vec![circle(1.0, 0.0, 1.0)], -1.0, 3.0);
    let got = four(
        "p11 p8 swapped",
        &thin,
        &fat,
        |q| margin(disc(q, 1.0, 0.0, 1.0).max(slab(q, -1.0, 2.0))),
        |q| margin(disc(q, 0.0, 0.0, 2.0).max(slab(q, 0.0, 1.0))),
        [Some(6.0 * pi), Some(2.0 * pi), Some(3.0 * pi), Some(pi)],
        &grid(),
    );
    all_answer("p11 p8 swapped", &got);
    let plate = ext(
        "bored plate",
        vec![rect(-3.0, -3.0, 3.0, 3.0), circle(0.0, 0.0, 2.0)],
        0.0,
        1.0,
    );
    let rod = ext("rod", vec![circle(1.0, 0.0, 1.0)], -1.0, 3.0);
    let got = four(
        "p11 p9 swapped",
        &rod,
        &plate,
        |q| margin(disc(q, 1.0, 0.0, 1.0).max(slab(q, -1.0, 2.0))),
        |q| margin(boxd(q, (-3.0, 3.0), (-3.0, 3.0), (0.0, 1.0)).max(-disc(q, 0.0, 0.0, 2.0))),
        [Some(36.0 - pi), Some(3.0 * pi), Some(36.0 - 4.0 * pi), None],
        &grid(),
    );
    all_answer("p11 p9 swapped", &got);
}

/// A cutter face covering only the middle of the hole's ruling
/// (z ∈ [0.25, 0.75]): two edge–sector sites on one ruling.
#[test]
fn p12_a_plane_tangent_to_the_middle_of_a_hole() {
    let pi = std::f64::consts::PI;
    let a = ext(
        "holed plate",
        vec![rect(-3.0, -3.0, 3.0, 3.0), circle(0.0, 0.0, 1.0)],
        0.0,
        1.0,
    );
    let b = finished(
        "cutter",
        brick::<f64>((1.0, 4.0), (-4.0, 4.0), (0.25, 0.75), tol()),
        tol(),
    );
    // cutter 3·8·0.5 = 12; meet 2·6·0.5 = 6
    let got = four(
        "p12 middle of a hole",
        &a,
        &b,
        |q| margin(boxd(q, (-3.0, 3.0), (-3.0, 3.0), (0.0, 1.0)).max(-disc(q, 0.0, 0.0, 1.0))),
        |q| margin(boxd(q, (1.0, 4.0), (-4.0, 4.0), (0.25, 0.75))),
        [Some(42.0 - pi), Some(30.0 - pi), Some(6.0), Some(6.0)],
        &grid(),
    );
    all_answer("p12", &got);
}

/// Opposite senses, void crescent, cylinder × cylinder outside each
/// other, one run long: ∪ two solids, ∩ empty, ∖ the operand.
#[test]
fn p13_two_discs_touching_outside_one_long() {
    let pi = std::f64::consts::PI;
    let a = ext("disc", vec![circle(0.0, 0.0, 1.0)], 0.0, 1.0);
    let b = ext("long disc", vec![circle(2.0, 0.0, 1.0)], -1.0, 3.0);
    let got = four(
        "p13 outside touch, long",
        &a,
        &b,
        |q| margin(disc(q, 0.0, 0.0, 1.0).max(slab(q, 0.0, 1.0))),
        |q| margin(disc(q, 2.0, 0.0, 1.0).max(slab(q, -1.0, 2.0))),
        [Some(4.0 * pi), Some(pi), Some(3.0 * pi), None],
        &grid(),
    );
    all_answer("p13", &got);
}

/// Opposite senses, void crescent, plane × cylinder with a staggered
/// edge: a block's face x = 1 over z ∈ [0.5, 2] against a unit disc
/// over z ∈ [0, 1]. ∖ and ∩ answer; ∪ refuses the slit it does not build.
#[test]
fn p14_a_block_resting_on_a_disc_wall_staggered() {
    let pi = std::f64::consts::PI;
    let a = ext("disc", vec![circle(0.0, 0.0, 1.0)], 0.0, 1.0);
    let b = finished(
        "block",
        brick::<f64>((1.0, 3.0), (-2.0, 2.0), (0.5, 2.0), tol()),
        tol(),
    );
    let got = four(
        "p14 block on a disc wall, staggered",
        &a,
        &b,
        |q| margin(disc(q, 0.0, 0.0, 1.0).max(slab(q, 0.0, 1.0))),
        |q| margin(boxd(q, (1.0, 3.0), (-2.0, 2.0), (0.5, 2.0))),
        [Some(pi + 12.0), Some(pi), Some(12.0), None],
        &grid(),
    );
    assert!(
        got[0].starts_with("REFUSED TangentSlitArmUnbuilt"),
        "p14 ∪: {got:#?}"
    );
    all_answer("p14 ∖ and ∩", &got[1..]);
}

/// Opposite senses, SHARED crescent, staggered: a bore (r 1 about (1, 0))
/// in a plate over z ∈ [0.5, 1.5] against a solid disc r 2 over [0, 1].
#[test]
fn p15_a_bore_inside_a_disc_staggered() {
    let pi = std::f64::consts::PI;
    let a = ext("disc r2", vec![circle(0.0, 0.0, 2.0)], 0.0, 1.0);
    let b = ext(
        "bored plate",
        vec![rect(-3.0, -3.0, 3.0, 3.0), circle(1.0, 0.0, 1.0)],
        0.5,
        1.0,
    );
    // A 4π; B 36 − π; A∩B over z∈[0.5,1]: 0.5·(4π − π) = 1.5π
    let got = four(
        "p15 bore in disc, staggered",
        &a,
        &b,
        |q| margin(disc(q, 0.0, 0.0, 2.0).max(slab(q, 0.0, 1.0))),
        |q| margin(boxd(q, (-3.0, 3.0), (-3.0, 3.0), (0.5, 1.5)).max(-disc(q, 1.0, 0.0, 1.0))),
        [
            Some(4.0 * pi + 36.0 - pi - 1.5 * pi),
            Some(4.0 * pi - 1.5 * pi),
            Some(36.0 - pi - 1.5 * pi),
            Some(1.5 * pi),
        ],
        &grid(),
    );
    all_answer("p15", &got);
}
