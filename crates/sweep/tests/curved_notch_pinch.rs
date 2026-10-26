//! The curved notch against the corner pinch: `notch343` with its face
//! from `v` to `(2, 1.15)` bowed into a cylinder arc, against the two-cube
//! corner pinch of `pinch_runs_battery`, every op in both orders. Each
//! line reads the volume against an oracle Richardson-extrapolated over
//! 48 and 96 chords, then tier 2, tier 3′, the certificate, the union
//! with a far brick, the mesh and the vertices per cone at `v`, each
//! refusal named by its variant.
//!
//! `cargo test -p sweep --release --test all curved_notch_pinch --
//! --ignored --nocapture`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Point3;
use topo::{AtRestBody, Body, BooleanError, BooleanResult, mass_properties};

use crate::common::pinch_cones::{
    Op as Cones, Plane, cone_finding, faces_through_two_vertices_at, vertices_at,
};
use crate::join_pierce_runs_sweep::{
    Run, SIDE, convex_volume, cube_at, cube_planes_at, direction, finished, frame, notch343,
    polygon_prism, tag_cones, tol,
};

type Ps = Vec<Vec<([f64; 3], f64)>>;

/// The notch's two pieces with the bowed face's arc cut into `n` chords.
fn curved_pieces(b: f64, n: usize) -> Ps {
    let (p, q) = ((1.0f64, 1.0f64), (2.0f64, 1.15f64));
    let (mx, my) = ((p.0 + q.0) / 2.0, (p.1 + q.1) / 2.0);
    let (dx, dy) = (q.0 - p.0, q.1 - p.1);
    let len = dx.hypot(dy);
    let h = len / 2.0;
    let s = h * b;
    let r = (h * h + s * s) / (2.0 * s);
    let c = (mx - dy / len * (r - s), my + dx / len * (r - s));
    let a0 = (p.1 - c.1).atan2(p.0 - c.0);
    let mut a1 = (q.1 - c.1).atan2(q.0 - c.0);
    while a1 > a0 + std::f64::consts::PI {
        a1 -= std::f64::consts::TAU;
    }
    while a1 < a0 - std::f64::consts::PI {
        a1 += std::f64::consts::TAU;
    }
    let mut upper = vec![(0.0, 1.0), (1.0, 1.0)];
    upper.extend((1..n).map(|t| {
        let a = a0 + (a1 - a0) * t as f64 / n as f64;
        (c.0 + r * a.cos(), c.1 + r * a.sin())
    }));
    upper.extend([(2.0, 1.15), (2.0, 2.0), (0.0, 2.0)]);
    vec![
        polygon_prism(&[(0.0, 0.0), (2.0, 0.0), (2.0, 0.85), (1.0, 1.0), (0.0, 1.0)]),
        polygon_prism(&upper),
    ]
}

/// The notch extruded from a profile whose `v → (2, 1.15)` edge bulges `b`.
fn curved_notch_body(b: f64) -> AtRestBody<f64> {
    use geom_core::Point2;
    let pts = [
        (0.0, 0.0, 0.0),
        (2.0, 0.0, 0.0),
        (2.0, 0.85, 0.0),
        (1.0, 1.0, b),
        (2.0, 1.15, 0.0),
        (2.0, 2.0, 0.0),
        (0.0, 2.0, 0.0),
    ];
    let lp = profile::test_support::bulge_loop(
        pts.iter()
            .map(|&(x, y, g)| (Point2::new(x, y), g))
            .collect(),
    );
    let plane = profile::SketchPlane::xy_at(Point3::new(0.0, 0.0, 0.0));
    let vp = profile::Profile::new(plane, vec![lp])
        .validate(tol())
        .expect("the curved notch validates");
    let depth = sweep::Extrusion::Distance {
        depth: 1.0,
        side: sweep::ExtrudeSide::Along,
    };
    finished(
        "the curved notch",
        sweep::extrude(&vp, depth, tol())
            .expect("the curved notch extrudes")
            .body,
    )
}

/// Unites `bodies` one by one; `None` where a union refuses.
fn pinch(bodies: Vec<Body<f64>>) -> Option<AtRestBody<f64>> {
    let mut parts = bodies.into_iter().map(|b| finished("a cube", b));
    let mut acc = parts.next()?;
    for b in parts {
        acc = topo::union(&acc, &b, tol()).ok()?.body()?.body.clone();
    }
    Some(acc)
}

/// The first word of a refusal's debug text: its variant.
fn variant(e: impl std::fmt::Debug) -> String {
    let s = format!("{e:?}");
    s.split(|c: char| !c.is_alphanumeric())
        .find(|w| !w.is_empty())
        .unwrap_or("")
        .to_owned()
}

/// One run read through every check, one line.
fn reading(
    r: &Result<BooleanResult<f64>, BooleanError>,
    want: f64,
    cones: (&[Vec<Plane>], &[Vec<Plane>], Cones),
) -> String {
    let bb = match r {
        Err(e) => return format!("ERR {}", variant(e)),
        Ok(r) => match r.body() {
            None if want.abs() < 1e-9 => return "EMPTY ok".into(),
            None => return format!("EMPTY WRONG want={want}"),
            Some(bb) => bb,
        },
    };
    let ok = |b: bool| if b { "ok" } else { "BAD" };
    let vol = topo::mass_properties(
        &bb.body,
        tol(),
        topo::test_support::pinned_resolution(tol()),
    )
    .map(|m| m.enclosure.volume);
    let v = vol.as_ref().map_or_else(variant, |&v| {
        if (v - want).abs() < 1e-7 {
            "ok".to_owned()
        } else {
            format!("BAD({v:.9} want {want:.9})")
        }
    });
    let t2 = ok(topo::validate_closed(&bb.body).is_ok());
    let cert = ok(topo::validate_geometric_certificate(&bb.body, tol()).is_ok());
    let t3 = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()).map_or_else(
        |es| {
            let mut v: Vec<String> = es.iter().map(variant).collect();
            v.dedup();
            v.join("+")
        },
        |()| "ok".into(),
    );
    let far = finished(
        "the far brick",
        sweep::test_support::brick((50.0, 51.0), (50.0, 51.0), (50.0, 51.0), tol()),
    );
    let reuse = topo::union(&bb.body, &far, tol()).map_or_else(
        |e| format!("{e:?}").chars().take(60).collect::<String>(),
        |_| "ok".into(),
    );
    let mesh = mesh::tessellate(&bb.body, 0.05, tol())
        .map_err(|e| match e {
            mesh::TessellateError::Triangulation { face } => {
                let fc = bb.body.get_face(face).unwrap();
                format!(
                    "Triangulation({} rings={})",
                    variant(bb.body.get_surface(fc.surface).unwrap()),
                    fc.rings.len()
                )
            }
            e => variant(e),
        })
        .and_then(|m| mesh::validate::check_mesh(&m).map_err(|_| "check".to_owned()))
        .map_or_else(|e| e, |()| "ok".into());
    let cones = cone_finding(&bb.body, notch343().v, cones)
        .map_or_else(|| "ok".into(), |f| format!("BAD({f})"));
    format!("BODY vol={v} t2={t2} cert={cert} t3p={t3} reuse={reuse} mesh={mesh} cones={cones}")
}

/// The curved notch's bulge: 2.5° of turn at either end of the arc.
fn bulge() -> f64 {
    2.5f64.to_radians().tan()
}

/// The pinch at grid direction `(i, j)` and turn `k`, and its two cubes'
/// half-spaces; `None` where uniting the cubes refuses.
fn corner_pinch(i: u32, j: u32, k: u32) -> Option<(AtRestBody<f64>, Ps)> {
    let v = notch343().v;
    let f = frame(direction(i, j), f64::from(k) * 1.05 + 0.1);
    let y = pinch(vec![cube_at(v, f, [0.0; 3]), cube_at(v, f, [-SIDE; 3])])?;
    Some((
        y,
        vec![
            cube_planes_at(v, f, [0.0; 3]),
            cube_planes_at(v, f, [-SIDE; 3]),
        ],
    ))
}

/// Every op in both orders between the notch `x` and the pinch at
/// `(i, j, k)`: `(tag, result, oracle volume)`, the tag `"{order} {op}"`.
fn runs(x: &AtRestBody<f64>, (i, j, k): (u32, u32, u32)) -> Option<Vec<Run>> {
    let (y, yp) = corner_pinch(i, j, k)?;
    let b = bulge();
    let vol = |ps: &Ps| ps.iter().map(|p| convex_volume(p)).sum::<f64>();
    let (p48, p96) = (curved_pieces(b, 48), curved_pieces(b, 96));
    let va = (4.0 * vol(&p96) - vol(&p48)) / 3.0;
    let clip = |ps: &Ps| -> f64 {
        ps.iter()
            .flat_map(|p| {
                yp.iter().map(move |q| {
                    let mut all = p.clone();
                    all.extend_from_slice(q);
                    convex_volume(&all)
                })
            })
            .sum()
    };
    let common = (4.0 * clip(&p96) - clip(&p48)) / 3.0;
    let vy = 2.0 * SIDE.powi(3);
    let mut out = Vec::new();
    for (order, l, r, vl) in [("ab", x, &y, va), ("ba", &y, x, vy)] {
        out.push((
            format!("{order} U"),
            topo::union(l, r, tol()),
            va + vy - common,
        ));
        out.push((format!("{order} I"), topo::intersect(l, r, tol()), common));
        out.push((
            format!("{order} S"),
            topo::subtract(l, r, tol()),
            vl - common,
        ));
    }
    Some(out)
}

/// The curved notch against the corner pinch over the battery's grid at
/// three turns.
#[test]
#[ignore = "measurement probe: prints one line per run"]
fn curved_notch_pinch() {
    let x = curved_notch_body(bulge());
    for i in 0..12 {
        for j in 0..7 {
            for k in [0u32, 2, 4] {
                let Some(runs) = runs(&x, (i, j, k)) else {
                    println!("curved i={i} j={j} k={k}: the pinch refuses");
                    continue;
                };
                let (_, yp) = corner_pinch(i, j, k).expect("the pinch");
                let x96 = curved_pieces(bulge(), 96);
                for (tag, res, want) in runs {
                    let cones = tag_cones(&tag, "ab", (&x96, &yp));
                    println!(
                        "curved i={i} j={j} k={k} {tag}: {}",
                        reading(&res, want, cones)
                    );
                }
            }
        }
    }
}

/// A curved notch meeting a corner pinch at its reflex vertex builds the
/// right body, which the census's cross-solid backstop cannot yet read:
/// it cannot clear two solids touching only at the pinch
/// (`work/restread/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`).
/// A further union with a brick far away builds, the brick added: the
/// closed-form volume cannot side a cylinder wall cut along an ellipse,
/// and a ray aimed at a planar face does. One pose per population of
/// [`curved_notch_pinch`]; when the census lane lands, its pose's pinned
/// finding flips and the pin is re-baselined there. At `0 3 2` and
/// `3 1 0` a section face passes `v` twice, once by the bowed wall's arc
/// and once by a line to the same point, so the discard reads the arc's
/// kept ends off its own copy on the section face's twin
/// (`finish::twin_copy`); red as `JoinDesync` if it reads them off the
/// copies' vertex pairs, which two twin edges join. Its discards still
/// name the stretch by its two ends, and at `3 1 0 ba S` four settle on
/// ends that more than one result edge joins
/// (`work/refs/a-discard-names-a-bordered-stretch-by-its-two-end-vertices.md`):
/// the count flips to 0 when the record carries the edge.
#[test]
fn a_curved_notch_at_a_corner_pinch_builds_bodies_the_census_cannot_read_yet() {
    let x = curved_notch_body(bulge());
    let far = finished(
        "the far brick",
        sweep::test_support::brick((50.0, 51.0), (50.0, 51.0), (50.0, 51.0), tol()),
    );
    // (pose, tag, tier 3' undecidable, stretches on shared ends)
    let pins = [
        ((0, 0, 0), "ab I", true, 0),
        ((0, 0, 0), "ab U", false, 0),
        ((9, 4, 4), "ab S", true, 0),
        ((0, 3, 2), "ab U", false, 0),
        ((3, 1, 0), "ba S", true, 4),
    ];
    for (pose, tag, undecidable, shared) in pins {
        let label = format!("{pose:?} {tag}");
        let (_, res, want) = runs(&x, pose)
            .unwrap_or_else(|| panic!("{label}: the pinch refuses"))
            .into_iter()
            .find(|(t, ..)| t == tag)
            .unwrap();
        let res = res.unwrap_or_else(|e| panic!("{label}: refused {e:?}"));
        let bb = res.body().unwrap_or_else(|| panic!("{label}: empty"));
        let v = mass_properties(
            &bb.body,
            tol(),
            topo::test_support::pinned_resolution(tol()),
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume;
        assert!(
            (v - want).abs() < 1e-7,
            "{label}: volume {v} against the oracle's {want}"
        );
        assert!(topo::validate_closed(&bb.body).is_ok(), "{label}: tier 2");
        assert!(
            topo::validate_geometric_certificate(&bb.body, tol()).is_ok(),
            "{label}: the certificate"
        );
        let findings = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol())
            .err()
            .unwrap_or_default();
        assert!(
            findings
                .iter()
                .all(|e| matches!(e, topo::ValidationError::CensusUndecidable { .. })),
            "{label}: tier 3' has a definite finding: {findings:?}"
        );
        assert_eq!(
            !findings.is_empty(),
            undecidable,
            "{label}: tier 3' undecidable: {findings:?}"
        );
        assert_eq!(
            stretches_on_shared_ends(bb),
            shared,
            "{label}: bordered stretches whose ends several result edges join"
        );
        let reuse = topo::union(&bb.body, &far, tol())
            .unwrap_or_else(|e| panic!("{label}: the further union builds, got {e:?}"));
        let joined = mass_properties(
            &reuse.body().expect("not empty").body,
            tol(),
            topo::test_support::pinned_resolution(tol()),
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume;
        assert!(
            (joined - (want + 1.0)).abs() < 1e-7,
            "{label}: the further union's volume {joined} against {}",
            want + 1.0
        );
    }
}

/// One run of [`curved_notch_pinch`] (`CURVED_POSE="i j k order op"`):
/// the body's solids, its records and tier 3′'s findings, each face
/// named with its solid and surface, and the mesh at four δ, with the
/// corners of a planar face the coarsest refuses.
#[test]
#[ignore = "measurement probe: prints one run's findings"]
fn curved_notch_pinch_one() {
    let pose = std::env::var("CURVED_POSE").expect("CURVED_POSE=\"i j k order op\"");
    let w: Vec<&str> = pose.split(' ').collect();
    let n = |i: usize| w[i].parse::<u32>().unwrap();
    let v = notch343().v;
    let x = curved_notch_body(bulge());
    let tag = format!("{} {}", w[3], w[4]);
    let (_, res, _) = runs(&x, (n(0), n(1), n(2)))
        .expect("the pinch")
        .into_iter()
        .find(|(t, ..)| *t == tag)
        .unwrap();
    let res = match res {
        Ok(r) => r,
        Err(e) => {
            println!("refusal {e:?}");
            return;
        }
    };
    let bb = res.body().expect("a body");
    let body = &bb.body;
    println!("solids {}", body.solids().count());
    for row in bb.contacts.rows() {
        println!("record {row:?}");
    }
    let face = |e: &topo::EntityId| match e {
        topo::EntityId::Face(fk) => {
            let fc = body.get_face(*fk).unwrap();
            let s = body.get_surface(fc.surface).unwrap();
            format!("{fk:?} solid={:?} {}", body.solid_of_face(*fk), variant(s))
        }
        e => format!("{e:?}"),
    };
    if let Err(es) = topo::validate_pseudomanifold(body, &bb.contacts, tol()) {
        for e in &es {
            if let topo::ValidationError::CensusUndecidable { a, b, what, .. } = e {
                println!("undecidable {} | {} | {what}", face(a), face(b));
            } else {
                println!("finding {e:?}");
            }
        }
    }
    let far = finished(
        "the far brick",
        sweep::test_support::brick((50.0, 51.0), (50.0, 51.0), (50.0, 51.0), tol()),
    );
    println!("reuse {:?}", topo::union(body, &far, tol()).err());
    let at_v = vertices_at(body, v);
    println!(
        "vertices at v {} faces through two {}",
        at_v.len(),
        faces_through_two_vertices_at(body, v)
    );
    if let Err(mesh::TessellateError::Triangulation { face }) = mesh::tessellate(body, 0.05, tol())
    {
        let fc = body.get_face(face).unwrap();
        if let topo::LoopBoundary::Cycle { first } = body.get_loop(fc.outer).unwrap().boundary {
            for he in body.loop_cycle(first).unwrap() {
                let h = body.get_half_edge(he).unwrap();
                let p = body.vertex_points().find(|(k, _)| *k == h.start).unwrap().1;
                println!(
                    "  corner {:?}{} [{:.6} {:.6} {:.6}]",
                    h.start,
                    if at_v.contains(&h.start) { " AT V" } else { "" },
                    p.x,
                    p.y,
                    p.z
                );
            }
        }
    }
    for delta in [0.05, 0.02, 5e-3, 1e-3] {
        let m =
            mesh::tessellate(body, delta, tol()).map(|m| mesh::validate::check_mesh(&m).is_ok());
        println!("mesh at {delta}: {:?}", m.map_err(variant));
    }
}

/// How many of `bb`'s discards' bordered stretches settle on an end pair
/// that more than one of the result's edges joins: a stretch the naming
/// (`editor_core`'s `by_ends`) then credits to every edge on the pair.
fn stretches_on_shared_ends(bb: &topo::BooleanBody<f64>) -> usize {
    let body: &Body<f64> = &bb.body;
    // An end settled inside an edge or a face is no edge's end.
    let settler = bb.naming.settler(body);
    let settle = |v| match settler.settle(v).unwrap() {
        topo::Cell::Vertex(k) => Some(k),
        topo::Cell::Edge(_) | topo::Cell::Face(_) => None,
    };
    let mut joining: std::collections::BTreeMap<_, usize> = std::collections::BTreeMap::new();
    for (_, e) in body.edges() {
        let s = body.get_half_edge(e.he_plus).unwrap().start;
        let t = body.half_edge_end(e.he_plus).unwrap();
        *joining.entry((s.min(t), s.max(t))).or_default() += 1;
    }
    bb.naming
        .discards
        .iter()
        .flat_map(|d| d.bordered.iter().chain(&d.bordered_through))
        .filter(|&&(u, w)| {
            let (Some(u), Some(w)) = (settle(u), settle(w)) else {
                return false;
            };
            joining.get(&(u.min(w), u.max(w))).is_some_and(|&n| n > 1)
        })
        .count()
}
