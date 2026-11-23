//! Review probes for PR 5981 (`section_cones`): the pinch-corner split
//! swept over plane normals through the pinch, cube tilts and cone
//! counts, each side checked against a closed-form volume, the boolean
//! (`union ∩ box`, which groups a point's cones itself) as the oracle
//! for how many vertices a side keeps at the corner, tiers 3 and 3′
//! against the union's own records carried through the split, and the
//! vertex the split keeps across member order.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use crate::split_through_a_union_pinch::{
    Half, PINCH_AT, WEDGE, box_halves, convex_volume, cross, dot, ell, unit,
};

use common::{finished, split_plane};
use geom_core::{Point3, Tol, Vec3};
use topo::{
    AtRestBody, BooleanBody, SplitPart, split, validate_geometric, validate_pseudomanifold,
};

fn t() -> Tol {
    Tol::witness()
}

fn frame(normal: [f64; 3]) -> [[f64; 3]; 3] {
    let m = unit(normal);
    let u = unit([-m[2] * m[0], -m[2] * m[1], 1.0 - m[2] * m[2]]);
    [m, u, cross(m, u)]
}

/// The side-4 cube whose bottom face is on the plane through the pinch
/// with `normal`.
fn cube_along(normal: [f64; 3]) -> AtRestBody<f64> {
    let [m, u, w] = frame(normal);
    let v = PINCH_AT;
    let body = common::mapped_cube::<f64>(
        move |x, y, z| {
            let (a, b, c) = (4.0 * (x - 0.5), 4.0 * (y - 0.5), 4.0 * z);
            Point3::new(
                v[0] + a * u[0] + b * w[0] + c * m[0],
                v[1] + a * u[1] + b * w[1] + c * m[1],
                v[2] + a * u[2] + b * w[2] + c * m[2],
            )
        },
        t(),
    );
    finished("the cube", body, t())
}

fn cube_halves_along(normal: [f64; 3]) -> Vec<Half> {
    let [m, u, w] = frame(normal);
    let at = |d: [f64; 3]| dot(d, PINCH_AT);
    let neg = |d: [f64; 3]| d.map(|c| -c);
    vec![
        (neg(m), -at(m)),
        (m, at(m) + 4.0),
        (u, at(u) + 2.0),
        (neg(u), 2.0 - at(u)),
        (w, at(w) + 2.0),
        (neg(w), 2.0 - at(w)),
    ]
}

/// A parallelepiped at the pinch on `edges` (right-handed) scaled by `s`.
fn wedge_on(edges: [[f64; 3]; 3], s: f64) -> AtRestBody<f64> {
    let body = common::mapped_cube::<f64>(
        move |x, y, z| {
            let q = [0, 1, 2]
                .map(|i| PINCH_AT[i] + s * (x * edges[0][i] + y * edges[1][i] + z * edges[2][i]));
            Point3::new(q[0], q[1], q[2])
        },
        t(),
    );
    finished("a wedge", body, t())
}

/// The parallelepiped as halfspaces.
fn wedge_halves(e: [[f64; 3]; 3], s: f64) -> Vec<Half> {
    let det = dot(e[0], cross(e[1], e[2]));
    // Rows of the inverse of the edge matrix: dual vectors.
    let duals =
        [cross(e[1], e[2]), cross(e[2], e[0]), cross(e[0], e[1])].map(|d| d.map(|c| c / det));
    let mut out = Vec::new();
    for d in duals {
        // 0 ≤ d·(x − p)/s ≤ 1.
        let at = dot(d, PINCH_AT);
        out.push((d, at + s));
        out.push((d.map(|c| -c), -at));
    }
    out
}

/// In the gap above the L's top and under the cube's bottom.
const WEDGE_A: [[f64; 3]; 3] = [[-1.0, -1.0, 0.3], [-0.2, -1.0, 0.5], [-1.0, -0.2, 0.5]];

fn united(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> BooleanBody<f64> {
    match topo::union(a, b, t()) {
        Ok(topo::BooleanResult::Body(b)) => b,
        other => panic!("the union builds: {other:?}"),
    }
}

fn at_corner(b: &AtRestBody<f64>) -> usize {
    b.vertex_points()
        .filter(|(_, p)| [p.x, p.y, p.z] == PINCH_AT)
        .count()
}

/// The box `0 ≤ n·(x − p) ≤ 20`, `|a·(x − p)|, |b·(x − p)| ≤ 10`.
fn side_box(n: [f64; 3]) -> AtRestBody<f64> {
    let n = unit(n);
    let a = unit(if n[2].abs() < 0.9 {
        cross(n, [0.0, 0.0, 1.0])
    } else {
        cross(n, [1.0, 0.0, 0.0])
    });
    let b = cross(n, a);
    let v = PINCH_AT;
    let body = common::mapped_cube::<f64>(
        move |x, y, z| {
            let (s, r, h) = (20.0 * (x - 0.5), 20.0 * (y - 0.5), 20.0 * z);
            Point3::new(
                v[0] + s * a[0] + r * b[0] + h * n[0],
                v[1] + s * a[1] + r * b[1] + h * n[1],
                v[2] + s * a[2] + r * b[2] + h * n[2],
            )
        },
        t(),
    );
    finished("the side box", body, t())
}

/// Fibonacci directions on the sphere.
fn normals(k: usize) -> Vec<[f64; 3]> {
    let g = geom_core::GOLDEN_ANGLE;
    (0..k)
        .map(|i| {
            let z = 1.0 - 2.0 * (i as f64 + 0.5) / k as f64;
            let r = (1.0 - z * z).sqrt();
            let th = g * i as f64;
            // Round to a few digits so the plane data are tame.
            [r * th.cos(), r * th.sin(), z].map(|c| (c * 1000.0).round() / 1000.0)
        })
        .collect()
}

struct Case {
    what: String,
    union: BooleanBody<f64>,
    /// The convex parts, each `(halves, sign)`: volume adds `sign·vol(part ∩ side)`.
    parts: Vec<(Vec<Half>, f64)>,
}

fn cases() -> Vec<Case> {
    let boxes = [
        box_halves([0.0, 0.0, 0.0], [2.0, 1.0, 1.0]),
        box_halves([0.0, 1.0, 0.0], [1.0, 2.0, 1.0]),
    ];
    let mut out = Vec::new();
    for tilt in [[1.0, 1.3, 0.7], [1.0, 1.0, 1.0], [0.6, 1.4, 1.1]] {
        let ch = cube_halves_along(tilt);
        let mut base: Vec<(Vec<Half>, f64)> = Vec::new();
        for b in &boxes {
            base.push((b.clone(), 1.0));
            let mut both = b.clone();
            both.extend_from_slice(&ch);
            base.push((both, -1.0));
        }
        base.push((ch.clone(), 1.0));
        let two = united(&ell(), &cube_along(tilt));
        let tag = format!("tilt {tilt:?}");
        // The wedges only fit the default tilt's gaps for certain;
        // they are checked to lie in the gaps by their volumes.
        if tilt == [1.0, 1.3, 0.7] {
            let wb = wedge_on(WEDGE, 0.3);
            let wa = wedge_on(WEDGE_A, 0.3);
            let three = united(&two.body, &wb);
            let three_a = united(&two.body, &wa);
            let four = united(&three.body, &wa);
            let mut p3 = base.clone();
            p3.push((wedge_halves(WEDGE, 0.3), 1.0));
            let mut p3a = base.clone();
            p3a.push((wedge_halves(WEDGE_A, 0.3), 1.0));
            let mut p4 = p3.clone();
            p4.push((wedge_halves(WEDGE_A, 0.3), 1.0));
            out.push(Case {
                what: format!("{tag}, three (B wedge)"),
                union: three,
                parts: p3,
            });
            out.push(Case {
                what: format!("{tag}, three (A wedge)"),
                union: three_a,
                parts: p3a,
            });
            out.push(Case {
                what: format!("{tag}, four"),
                union: four,
                parts: p4,
            });
        }
        out.push(Case {
            what: format!("{tag}, two"),
            union: two,
            parts: base,
        });
    }
    out
}

fn closed_form(parts: &[(Vec<Half>, f64)], n: [f64; 3], above: bool) -> f64 {
    let s = if above { -1.0 } else { 1.0 };
    let cut: Half = (n.map(|c| s * c), s * dot(n, PINCH_AT));
    parts
        .iter()
        .map(|(h, sign)| {
            let mut all = vec![cut];
            all.extend_from_slice(h);
            sign * convex_volume(&all)
        })
        .sum()
}

/// The sweep. Prints one line per cell that is not clean; panics at the
/// end with the tally if any side is wrong (a refusal is reported, not
/// failed).
#[test]
#[ignore = "review sweep; run by hand"]
fn review_sweep_normals_through_the_pinch() {
    let k: usize = std::env::var("REVIEW_K")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(40);
    let band = geom_core::Band::linear(t()).unwrap();
    let mut wrong = 0;
    let mut refused = 0;
    let mut cells = 0;
    let mut oracle_refused = 0;
    let mut carry_refused = 0;
    let mut fused_sides = 0;
    for case in cases() {
        let cones = at_corner(&case.union.body);
        let ns = if std::env::var_os("REVIEW_PERTURB").is_some() {
            let mut v = Vec::new();
            for base in [
                [1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0],
                [0.0, 1.0, 0.0],
                unit([1.0, 1.3, 0.7]),
                unit([1.0, -1.0, 0.0]),
            ] {
                for d in [0.0, 1e-7, -1e-7, 1e-9, -1e-9, 1e-11, -1e-11] {
                    for s in [1.0, -1.0] {
                        v.push([
                            s * base[0] + d,
                            s * base[1] - 0.5 * d,
                            s * base[2] + 0.3 * d,
                        ]);
                    }
                }
            }
            v
        } else {
            normals(k)
        };
        for n in ns {
            cells += 1;
            let plane = split_plane(
                Point3::new(PINCH_AT[0], PINCH_AT[1], PINCH_AT[2]),
                Vec3::new(n[0], n[1], n[2]),
                t(),
            );
            let r = match split(&case.union.body, &plane, t()) {
                Ok(r) => r,
                Err(e) => {
                    refused += 1;
                    println!("REFUSED {} n={n:?}: {e:?}", case.what);
                    continue;
                }
            };
            let carried = r.carry_contacts(
                &case.union.contacts,
                band,
                Some(topo::RegionLane::certified()),
            );
            let kept: Vec<usize> = [&r.above, &r.below]
                .iter()
                .map(|p| match p {
                    SplitPart::Body(b) => at_corner(b),
                    _ => 0,
                })
                .collect();
            match &carried {
                Err(e) => {
                    carry_refused += 1;
                    println!(
                        "CARRY {} n={n:?} cones {cones} kept {kept:?}: {e:?}",
                        case.what
                    );
                }
                Ok(_) => println!(
                    "CARRYOK {} n={n:?} cones {cones} kept {kept:?} records {}",
                    case.what,
                    case.union.contacts.len()
                ),
            }
            for (above, part) in [(true, &r.above), (false, &r.below)] {
                let what = format!(
                    "{} n={n:?} {}",
                    case.what,
                    if above { "above" } else { "below" }
                );
                let want = closed_form(&case.parts, n, above);
                let SplitPart::Body(b) = part else {
                    if want > 1e-9 {
                        wrong += 1;
                        println!("WRONG {what}: empty, want volume {want}");
                    }
                    continue;
                };
                let mut bad = Vec::new();
                let v = topo::test_support::pinned_volume("side", b, t()).volume;
                let perturbed = std::env::var_os("REVIEW_PERTURB").is_some();
                if !perturbed && (v - want).abs() > 1e-8 * want.max(1.0) {
                    bad.push(format!("volume {v} want {want}"));
                }
                if let Err(e) = validate_geometric(b, t()) {
                    bad.push(format!("tier3 {e:?}"));
                }
                let records = match &carried {
                    Ok(c) => {
                        if above {
                            c.above.clone()
                        } else {
                            c.below.clone()
                        }
                    }
                    Err(_) => topo::ContactRecords::default(),
                };
                if let Err(e) = validate_pseudomanifold(b, &records, t()) {
                    bad.push(format!("tier3' {e:?}"));
                }
                let got = at_corner(b);
                if got < cones && got > 0 {
                    fused_sides += 1;
                }
                let dir = if above { n } else { n.map(|c| -c) };
                match topo::intersect(&case.union.body, &side_box(dir), t()) {
                    Ok(topo::BooleanResult::Body(o)) => {
                        let want_v = at_corner(&o.body);
                        let ov = topo::test_support::pinned_volume("oracle", &o.body, t()).volume;
                        if (v - ov).abs() > 1e-7 * ov.max(1.0) {
                            bad.push(format!("volume {v}, boolean oracle {ov}"));
                        }
                        if want_v != got {
                            bad.push(format!("corner vertices {got}, boolean oracle {want_v}"));
                        }
                    }
                    Ok(topo::BooleanResult::Empty) => bad.push("oracle empty".into()),
                    Err(e) => {
                        oracle_refused += 1;
                        println!("ORACLE-REFUSED {what}: {e:?}");
                    }
                }
                if !bad.is_empty() {
                    wrong += 1;
                    println!("WRONG {what} (cones {cones}, kept {got}): {bad:?}");
                }
            }
        }
    }
    println!(
        "cells {cells}, refused {refused}, wrong sides {wrong}, oracle refused {oracle_refused}, carry refused {carry_refused}, sides with fused cones {fused_sides}"
    );
    assert_eq!(wrong, 0);
}

/// **A fused cone's vertex has an image in the split's lineage.** The
/// three-cone union (the wedge in the notch's gap) split by a plane
/// through the pinch where a side fuses cones: every vertex the
/// union's records cite has a live image on some side (D1), so the
/// records carry. Red on the PR's head: the fusion's `kev` kills a
/// vertex the lineage still states as kept, and the carry refuses it
/// `Unmapped`.
#[test]
fn a_fused_cones_vertex_has_a_live_image_in_the_splits_lineage() {
    let two = united(&ell(), &cube_along([1.0, 1.3, 0.7]));
    let three = united(&two.body, &wedge_on(WEDGE, 0.3));
    let n = [0.271, 0.864, 0.425];
    let plane = split_plane(
        Point3::new(PINCH_AT[0], PINCH_AT[1], PINCH_AT[2]),
        Vec3::new(n[0], n[1], n[2]),
        t(),
    );
    let r = split(&three.body, &plane, t()).expect("the split builds");
    let sides: Vec<&AtRestBody<f64>> = [&r.above, &r.below]
        .into_iter()
        .filter_map(|p| match p {
            SplitPart::Body(b) => Some(b),
            SplitPart::Empty => None,
        })
        .collect();
    let live = |c: topo::Cell| match c {
        topo::Cell::Vertex(v) => sides.iter().any(|b| b.get_vertex(v).is_some()),
        _ => true,
    };
    let lineage = r.lineage();
    let dead: Vec<_> = three
        .body
        .vertex_points()
        .filter(|(_, p)| [p.x, p.y, p.z] == PINCH_AT)
        .map(|(v, _)| (v, lineage.images(topo::Cell::Vertex(v))))
        .filter(|(_, images)| images.iter().any(|&c| !live(c)))
        .collect();
    assert_eq!(dead, vec![], "pinch vertices whose lineage images are dead");
    let band = geom_core::Band::linear(t()).unwrap();
    assert!(
        r.carry_contacts(&three.contacts, band, Some(topo::RegionLane::certified()))
            .is_ok(),
        "the union's records carry through the split"
    );
}

/// **Every lineage image of every operand vertex and edge is live**
/// (review 2 of PR 5981). Over 200 normals through the pinch and every
/// union of [`cases`], each split that builds states, for each vertex
/// and edge of the union, only images some side holds. Faces are left
/// out: their images read dead in every cell, fusion or none (a finding
/// of the review, not this row's subject). Ignored while it also asks
/// for a chained fusion (a fusion whose kept vertex a later one kills,
/// which `Lineage::through_fusions` reads to its survivor): no input
/// here reaches one.
#[test]
#[ignore = "review probe: no input here chains two fusions"]
fn every_operand_cells_split_images_are_live_through_chained_fusions() {
    let mut chains = 0;
    let mut built = 0;
    let mut dead_cells = 0;
    for case in cases() {
        for n in normals(200) {
            let plane = split_plane(
                Point3::new(PINCH_AT[0], PINCH_AT[1], PINCH_AT[2]),
                Vec3::new(n[0], n[1], n[2]),
                t(),
            );
            let Ok(r) = split(&case.union.body, &plane, t()) else {
                continue;
            };
            built += 1;
            if r.naming.vertex_fusions.rows().len() > 2 {
                println!(
                    "MULTI {} n={n:?} fusions {:?}",
                    case.what,
                    r.naming.vertex_fusions.rows()
                );
            }
            let rows = r.naming.vertex_fusions.rows();
            chains += rows
                .iter()
                .enumerate()
                .filter(|&(i, &(_, kept))| rows[i + 1..].iter().any(|&(d, _)| d == kept))
                .count();
            let sides: Vec<&AtRestBody<f64>> = [&r.above, &r.below]
                .into_iter()
                .filter_map(|p| match p {
                    SplitPart::Body(b) => Some(b),
                    SplitPart::Empty => None,
                })
                .collect();
            let live = |c: topo::Cell| {
                sides.iter().any(|b| match c {
                    topo::Cell::Vertex(v) => b.get_vertex(v).is_some(),
                    topo::Cell::Edge(e) => b.get_edge(e).is_some(),
                    topo::Cell::Face(f) => b.get_face(f).is_some(),
                })
            };
            let band = geom_core::Band::linear(t()).unwrap();
            if let Err(e) = r.carry_contacts(
                &case.union.contacts,
                band,
                Some(topo::RegionLane::certified()),
            ) {
                println!("CARRY {} n={n:?} fusions {}: {e:?}", case.what, rows.len());
            }
            let lineage = r.lineage();
            let u = &case.union.body;
            let cells: Vec<topo::Cell> = u
                .vertices()
                .map(|(v, _)| topo::Cell::Vertex(v))
                .chain(u.edges().map(|(e, _)| topo::Cell::Edge(e)))
                .collect();
            let dead: Vec<_> = cells
                .into_iter()
                .filter_map(|c| {
                    let bad: Vec<_> = lineage
                        .images(c)
                        .into_iter()
                        .filter(|&i| !live(i))
                        .collect();
                    (!bad.is_empty()).then_some((c, bad))
                })
                .collect();
            if !dead.is_empty() {
                dead_cells += 1;
                let vertices = dead
                    .iter()
                    .filter(|(c, _)| matches!(c, topo::Cell::Vertex(_)))
                    .count();
                println!(
                    "DEAD {} n={n:?} fusions {} cells {} of them vertices {vertices}",
                    case.what,
                    rows.len(),
                    dead.len()
                );
            }
        }
    }
    println!("built {built}, chained fusions {chains}, cells with dead images {dead_cells}");
    assert_eq!(dead_cells, 0, "cells with dead images");
    assert!(chains > 0, "no input chained two fusions");
}

/// Review 2 trace of a `Parted` carry refusal on the four-cone union
/// (record 3, n = (-0.046, 0.999, 0.025)): which fusions ran, the
/// record's two cells, and which side holds each image. Prints only.
#[test]
#[ignore = "review trace; run by hand"]
fn review_trace_a_four_cone_parted_cell() {
    let two = united(&ell(), &cube_along([1.0, 1.3, 0.7]));
    let three = united(&two.body, &wedge_on(WEDGE, 0.3));
    let four = united(&three.body, &wedge_on(WEDGE_A, 0.3));
    for (n, rec) in [([-0.046, 0.999, 0.025], 3), ([0.988, 0.133, -0.075], 0)] {
        let plane = split_plane(
            Point3::new(PINCH_AT[0], PINCH_AT[1], PINCH_AT[2]),
            Vec3::new(n[0], n[1], n[2]),
            t(),
        );
        let r = split(&four.body, &plane, t()).expect("builds");
        println!("n={n:?} fusions {:?}", r.naming.vertex_fusions.rows());
        let (x, y) = four.contacts.cell_pairs().nth(rec).unwrap();
        let lineage = r.lineage();
        for c in [x, y] {
            let at = |p: &SplitPart<f64>, c: topo::Cell| match (p, c) {
                (SplitPart::Body(b), topo::Cell::Vertex(v)) => b.get_vertex(v).is_some(),
                (SplitPart::Body(b), topo::Cell::Edge(e)) => b.get_edge(e).is_some(),
                (SplitPart::Body(b), topo::Cell::Face(f)) => b.get_face(f).is_some(),
                _ => false,
            };
            let images: Vec<_> = lineage
                .images(c)
                .into_iter()
                .map(|i| (i, at(&r.above, i), at(&r.below, i)))
                .collect();
            let fused = matches!(c, topo::Cell::Vertex(v)
                if r.naming.vertex_fusions.rows().iter().any(|&(d, k)| d == v || k == v));
            println!(
                "  record {rec} cell {c:?} fused {fused}: images (cell, above, below) {images:?}"
            );
        }
    }
}
