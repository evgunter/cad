//! Review probes for PR 4209 (band-dual-4209-r2): the isosceles mitre.
//! Each row prints what it measured; the asserts are the claims.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, Point3, Vec3};
use sweep::blend::BlendError;
use sweep::blend::battery::TURN_OVERRUN;
use sweep::test_support::{block, prism_on, realized, sketch_from_axes};
use topo::boolean::BooleanOp;
use topo::{Body, EdgeKey, validate_geometric};

use crate::band_planar_cut_off::{D, Verb, edge, the_box, tol, volume_enclosure};
use crate::common::cavity::{brick, cut};

/// Run `verb` on `edges`; `Ok(Some(dv))` built tier-3 valid, `Ok(None)` refused,
/// `Err` built but tier 3 rejects.
fn outcome(body: &Body<f64>, edges: &[EdgeKey], verb: Verb) -> Result<Option<f64>, String> {
    match verb.run(body, edges) {
        Err(e) => {
            eprintln!("    refused: {e}");
            Ok(None)
        }
        Ok(out) => {
            if let Err(e) = validate_geometric(&out.body, tol()) {
                return Err(format!("built, tier 3 rejects: {e:?}"));
            }
            let dv = volume_enclosure(body).0 - volume_enclosure(&out.body).0;
            Ok(Some(dv))
        }
    }
}

/// The ΔV a request makes on the plain body: a feature the bands leave alone
/// leaves ΔV where it is, so a built body whose ΔV moved touched it.
fn sweep_feature(
    what: &str,
    plain: &Body<f64>,
    featured: impl Fn(f64) -> Body<f64>,
    edges: impl Fn(&Body<f64>) -> Vec<EdgeKey>,
    params: &[f64],
) -> Vec<String> {
    let mut bad = Vec::new();
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let base = match outcome(plain, &edges(plain), verb) {
            Ok(Some(dv)) => dv,
            other => panic!("{what} {verb:?}: plain body builds, got {other:?}"),
        };
        for &p in params {
            let body = featured(p);
            let r = outcome(&body, &edges(&body), verb);
            eprintln!("{what} {verb:?} p={p}: {r:?} (plain ΔV {base})");
            match r {
                Ok(Some(dv)) if (dv - base).abs() > 1e-9 => {
                    bad.push(format!("{what} {verb:?} p={p}: built at ΔV {dv} ≠ {base}"))
                }
                Err(e) => bad.push(format!("{what} {verb:?} p={p}: {e}")),
                _ => {}
            }
        }
    }
    bad
}

fn turn_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    vec![
        edge(body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]),
        edge(body, [0.0, 0.0, 1.0], [0.0, 1.5, 1.0]),
    ]
}

/// **A pocket in L's face** (the front wall `y = 0`) beside the vertical
/// edge, slid up through the foot (`z = 0.9`) into the strip and the other
/// band's cut. A built body must leave ΔV at the plain body's.
#[test]
fn probe_pocket_on_the_third_edges_face_through_the_foot() {
    let plain = the_box();
    let bad = sweep_feature(
        "front-wall pocket",
        &plain,
        |zc| {
            cut(
                "pocket",
                &the_box(),
                &brick(
                    Point3::new(0.02, -0.5, zc - 0.015),
                    Point3::new(0.06, 0.03, zc + 0.015),
                ),
            )
        },
        turn_edges,
        &[0.80, 0.86, 0.88, 0.90, 0.92, 0.95, 0.97],
    );
    assert!(bad.is_empty(), "{bad:#?}");
}

/// The square frustum of `band_planar_mitre::a_frustum_top_rim_mitres_at_leaning_walls`.
fn frustum(s: f64, lean_y: f64) -> Body<f64> {
    let trapezoid = |origin: Point3<f64>, u: Vec3<f64>, (z0, z1): (f64, f64), a: f64, b: f64| {
        let plane = sketch_from_axes(origin, u, Vec3::new(0.0, 0.0, 1.0), tol());
        prism_on(
            plane,
            vec![
                (Point2::new(a * z0, z0), 0.0),
                (Point2::new(2.0 - b * z0, z0), 0.0),
                (Point2::new(2.0 - b * z1, z1), 0.0),
                (Point2::new(a * z1, z1), 0.0),
            ],
            3.0,
            tol(),
        )
    };
    let along_x = trapezoid(
        Point3::new(0.0, 0.5, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        (0.0, 1.0),
        s,
        s,
    );
    let along_y = trapezoid(
        Point3::new(2.5, 0.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        (-0.5, 1.5),
        lean_y,
        lean_y,
    );
    realized(BooleanOp::Intersect, &along_x, &along_y, tol())
}

/// **A void beside an OBLIQUE turn**: the frustum (`s = 0.3`, dihedrals
/// `90° + atan 0.3`), a sealed void under its corner `(0.3, −0.3, 1)`
/// raised through the bands. No row pins a feature at a leaning turn.
#[test]
fn probe_void_beside_an_oblique_turn() {
    let s = 0.3;
    let plain = frustum(s, s);
    let edges = |b: &Body<f64>| {
        vec![
            edge(b, [s, -s, 1.0], [2.0 - s, -s, 1.0]),
            edge(b, [s, -s, 1.0], [s, -(2.0 - s), 1.0]),
        ]
    };
    let bad = sweep_feature(
        "frustum void",
        &plain,
        |top| {
            cut(
                "void",
                &frustum(s, s),
                &brick(Point3::new(0.33, -0.6, 0.5), Point3::new(0.6, -0.33, top)),
            )
        },
        edges,
        &[0.85, 0.9, 0.92, 0.94, 0.96, 0.98],
    );
    assert!(bad.is_empty(), "{bad:#?}");
}

/// **A box rim's corner void, slid diagonally** (convex, right trihedron),
/// a column under the mitre's plane at several offsets.
#[test]
fn probe_void_column_under_the_mitre() {
    let plain = the_box();
    let mut bad = Vec::new();
    for off in [0.02, 0.04, 0.07] {
        bad.extend(sweep_feature(
            &format!("void column off={off}"),
            &plain,
            |top| {
                cut(
                    "void",
                    &the_box(),
                    &brick(
                        Point3::new(off, off, 0.5),
                        Point3::new(off + 0.03, off + 0.03, top),
                    ),
                )
            },
            turn_edges,
            &[0.9, 0.93, 0.96, 0.98],
        ));
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// **A diagonally sheared box** (each wall leaning by `s`, the lean along
/// `(1, −1)`): two of its four top corners are isosceles, and at the other
/// two the face angles are `β` and `π − β`. Requesting the whole top rim:
/// what does a user meet?
#[test]
fn probe_sheared_box_rim() {
    let s = 0.3;
    // Lean +x along x and −y along y: the along_y trapezoid with
    // `a = b = −s` on `u = −y` leans its walls the same way.
    let body = {
        let par = |origin: Point3<f64>, u: Vec3<f64>, (z0, z1): (f64, f64), a: f64| {
            let plane = sketch_from_axes(origin, u, Vec3::new(0.0, 0.0, 1.0), tol());
            prism_on(
                plane,
                vec![
                    (Point2::new(a * z0, z0), 0.0),
                    (Point2::new(2.0 + a * z0, z0), 0.0),
                    (Point2::new(2.0 + a * z1, z1), 0.0),
                    (Point2::new(a * z1, z1), 0.0),
                ],
                4.0,
                tol(),
            )
        };
        let along_x = par(
            Point3::new(0.0, 1.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            (0.0, 1.0),
            s,
        );
        let along_y = par(
            Point3::new(3.0, 0.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            (-0.5, 1.5),
            s,
        );
        realized(BooleanOp::Intersect, &along_x, &along_y, tol())
    };
    validate_geometric(&body, tol()).expect("the sheared box");
    // Top corners: x ∈ {s, 2+s}, y ∈ {−s, −(2+s)}.
    let c = |x: f64, y: f64| [x, y, 1.0];
    let (x0, x1, y0, y1) = (s, 2.0 + s, -s, -(2.0 + s));
    let rim = [
        edge(&body, c(x0, y0), c(x1, y0)),
        edge(&body, c(x1, y0), c(x1, y1)),
        edge(&body, c(x1, y1), c(x0, y1)),
        edge(&body, c(x0, y1), c(x0, y0)),
    ];
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let r = verb
            .run(&body, &rim)
            .map(|o| o.naming.unwrap().mitres.len());
        eprintln!("sheared rim {verb:?}: {r:?}");
        // Each corner pair.
        for (i, pair) in [[0, 1], [1, 2], [2, 3], [3, 0]].iter().enumerate() {
            let es = [rim[pair[0]], rim[pair[1]]];
            match outcome(&body, &es, verb) {
                Ok(o) => eprintln!("  corner {i} {verb:?}: {o:?}"),
                Err(e) => panic!("corner {i} {verb:?}: {e}"),
            }
        }
        match verb.run(&body, &rim) {
            Err(BlendError::UnsupportedRunOut { detail, .. }) => assert_eq!(detail, TURN_OVERRUN),
            other => eprintln!("  rim: {:?}", other.map(|_| ())),
        }
    }
}

/// **L with a foot at both ends**: a box of height `h`, its top two edges
/// and bottom two edges at the corner over `x = y = 0`; the vertical edge
/// carries a turn foot at each end, `d` in from each. At `h ≥ 2d` the
/// feet are apart; below they cross and must refuse.
#[test]
fn probe_third_edge_with_turn_feet_at_both_ends() {
    for h in [0.5, 0.25, 0.21, 0.2, 0.19, 0.15] {
        let body = block(2.0, 1.5, h, tol());
        let es = vec![
            edge(&body, [0.0, 0.0, h], [2.0, 0.0, h]),
            edge(&body, [0.0, 0.0, h], [0.0, 1.5, h]),
            edge(&body, [0.0, 0.0, 0.0], [2.0, 0.0, 0.0]),
            edge(&body, [0.0, 0.0, 0.0], [0.0, 1.5, 0.0]),
        ];
        for verb in [Verb::Chamfer, Verb::Fillet] {
            let r = outcome(&body, &es, verb);
            eprintln!("h={h} {verb:?}: {r:?}");
            if h < 2.0 * D {
                assert!(
                    matches!(r, Ok(None)),
                    "h={h} {verb:?}: crossing feet must refuse"
                );
            } else {
                assert!(!matches!(r, Err(_)), "h={h} {verb:?}: {r:?}");
            }
        }
        // And a cut-off at L's far end: the top turn plus one bottom edge.
        let es = vec![es[0], es[1], es[2]];
        for verb in [Verb::Chamfer, Verb::Fillet] {
            let r = outcome(&body, &es, verb);
            eprintln!("h={h} turn+cut-off {verb:?}: {r:?}");
            assert!(!matches!(r, Err(_)), "h={h} {verb:?}: {r:?}");
        }
    }
}

/// **Independent overlap check**: the chamfered top rim of the box,
/// sampled point membership against the analytic solid (box less the
/// four half-spaces beyond the chamfer planes), and tier 3′.
#[test]
fn probe_rim_chamfer_point_membership_and_pseudomanifold() {
    let body = the_box();
    let rim = [
        edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]),
        edge(&body, [2.0, 0.0, 1.0], [2.0, 1.5, 1.0]),
        edge(&body, [2.0, 1.5, 1.0], [0.0, 1.5, 1.0]),
        edge(&body, [0.0, 1.5, 1.0], [0.0, 0.0, 1.0]),
    ];
    let out = Verb::Chamfer.run(&body, &rim).expect("builds");
    topo::validate_pseudomanifold(&out.body, &topo::boolean::ContactRecords::default(), tol())
        .expect("tier 3′");
    let inside = |p: Point3<f64>| {
        let d = D;
        let box_in = p.x > 0.0 && p.x < 2.0 && p.y > 0.0 && p.y < 1.5 && p.z > 0.0 && p.z < 1.0;
        let depth = 1.0 - p.z;
        box_in
            && p.y + depth > d
            && (1.5 - p.y) + depth > d
            && p.x + depth > d
            && (2.0 - p.x) + depth > d
    };
    let band = Band::linear(tol()).unwrap();
    let mut wrong = Vec::new();
    let mut n = 0;
    let mut seed: u64 = 0x9e37_79b9_7f4a_7c15;
    let mut rnd = || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed >> 11) as f64 / (1u64 << 53) as f64
    };
    for _ in 0..3000 {
        // Concentrate near the corners and the rim.
        let (cx, cy) = if rnd() < 0.5 { (0.0, 0.0) } else { (2.0, 1.5) };
        let p = Point3::new(
            cx + (rnd() - 0.5) * 0.4,
            cy + (rnd() - 0.5) * 0.4,
            0.8 + rnd() * 0.25,
        );
        let want = inside(p);
        match topo::point_in_solid(&out.body, p, band, tol()) {
            Ok(c) => {
                n += 1;
                let got = format!("{c:?}");
                let got_in = got.contains("Inside");
                let got_out = got.contains("Outside");
                if (want && got_out) || (!want && got_in) {
                    wrong.push((p, want, got));
                }
            }
            Err(_) => {}
        }
    }
    eprintln!("membership: {n} answered, {} wrong", wrong.len());
    assert!(wrong.is_empty(), "{:#?}", &wrong[..wrong.len().min(10)]);
}

/// The diagonally sheared box of [`probe_sheared_box_rim`].
fn sheared(s: f64) -> Body<f64> {
    let par = |origin: Point3<f64>, u: Vec3<f64>, (z0, z1): (f64, f64)| {
        let plane = sketch_from_axes(origin, u, Vec3::new(0.0, 0.0, 1.0), tol());
        prism_on(
            plane,
            vec![
                (Point2::new(s * z0, z0), 0.0),
                (Point2::new(2.0 + s * z0, z0), 0.0),
                (Point2::new(2.0 + s * z1, z1), 0.0),
                (Point2::new(s * z1, z1), 0.0),
            ],
            4.0,
            tol(),
        )
    };
    let along_x = par(
        Point3::new(0.0, 1.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        (0.0, 1.0),
    );
    let along_y = par(
        Point3::new(3.0, 0.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        (-0.5, 1.5),
    );
    realized(BooleanOp::Intersect, &along_x, &along_y, tol())
}

/// **Independent overlap check at OBLIQUE turns**: the sheared box's two
/// isosceles corners (dihedrals 73.3° and 106.7°) chamfered, sampled
/// against the analytic solid — the parallelepiped less, per requested
/// edge, the side of its chamfer plane (through the two lines `d` in from
/// the edge across each face) holding the edge.
#[test]
fn probe_oblique_turn_chamfer_point_membership() {
    let s = 0.3;
    let body = sheared(s);
    let lean = Vec3::new(s, -s, 1.0);
    let solid = |p: Point3<f64>| {
        let (a, b) = (p.x - s * p.z, p.y + s * p.z);
        a > 0.0 && a < 2.0 && b > -2.0 && b < 0.0 && p.z > 0.0 && p.z < 1.0
    };
    let band = Band::linear(tol()).unwrap();
    let (x0, x1, y0, y1) = (s, 2.0 + s, -s, -(2.0 + s));
    for (v, other_x, other_y) in [([x1, y1], x0, y0), ([x0, y0], x1, y1)] {
        let vp = Point3::new(v[0], v[1], 1.0);
        let ex = Point3::new(other_x, v[1], 1.0);
        let ey = Point3::new(v[0], other_y, 1.0);
        let es = [
            edge(&body, [v[0], v[1], 1.0], [other_x, v[1], 1.0]),
            edge(&body, [v[0], v[1], 1.0], [v[0], other_y, 1.0]),
        ];
        let out = Verb::Chamfer
            .run(&body, &es)
            .expect("the isosceles corner builds");
        validate_geometric(&out.body, tol()).expect("tier 3");
        topo::validate_pseudomanifold(&out.body, &topo::boolean::ContactRecords::default(), tol())
            .expect("tier 3′");
        // Each chamfer plane: edge direction e, the other top edge t, L down.
        let down = -lean;
        let planes: Vec<(Point3<f64>, Vec3<f64>)> = [(ex - vp, ey - vp), (ey - vp, ex - vp)]
            .iter()
            .map(|&(e, t)| {
                let e = e.normalize();
                let perp = |f: Vec3<f64>| (f - e * f.dot(e)).normalize();
                let p0 = vp + perp(t) * D;
                let p1 = vp + perp(down) * D;
                let n = e.cross(p1 - p0);
                // Orient so the kept side is positive (the vertex is removed).
                let n = if n.dot(vp - p0) > 0.0 { -n } else { n };
                (p0, n)
            })
            .collect();
        let inside = |p: Point3<f64>| solid(p) && planes.iter().all(|(o, n)| n.dot(p - *o) > 0.0);
        let mut seed: u64 = 0x2545_f491_4f6c_dd1d;
        let mut rnd = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            (seed >> 11) as f64 / (1u64 << 53) as f64
        };
        let (mut n, mut wrong) = (0, Vec::new());
        for _ in 0..3000 {
            let p = Point3::new(
                vp.x + (rnd() - 0.5) * 0.5,
                vp.y + (rnd() - 0.5) * 0.5,
                0.75 + rnd() * 0.3,
            );
            if let Ok(c) = topo::point_in_solid(&out.body, p, band, tol()) {
                n += 1;
                let got = format!("{c:?}");
                let want = inside(p);
                if (want && got.contains("Outside")) || (!want && got.contains("Inside")) {
                    wrong.push((p, want, got));
                }
            }
        }
        eprintln!("oblique corner {v:?}: {n} answered, {} wrong", wrong.len());
        assert!(wrong.is_empty(), "{:#?}", &wrong[..wrong.len().min(8)]);
    }
}

/// **A lean inside the Zero band builds** (the midpoint foot), both verbs,
/// tier 3 and Euler: `leaning_turn` at a tenth of the zero band.
#[test]
fn probe_lean_inside_the_zero_band_builds() {
    let band = Band::linear(tol()).expect("the run's band");
    for frac in [0.1, 0.5, 0.9] {
        let s = band.zero() * frac / 2.0;
        let (body, turn) = crate::common::operands::leaning_turn(s);
        for verb in [Verb::Chamfer, Verb::Fillet] {
            let r = outcome(&body, &turn, verb);
            eprintln!("lean s={s:e} {verb:?}: {r:?}");
            assert!(matches!(r, Ok(Some(_))), "s={s:e} {verb:?}: {r:?}");
        }
    }
}
