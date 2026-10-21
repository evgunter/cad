//! Reviewer probes, PR 5556 (door reads sphere seam regions).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::seam_region_common::flipped;
use geom::Surface;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::test_support::{ball_poled, ball_poled_y, ball_poled_z, brick, dome, domed_cavity, finished, lantern, waisted};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{Body, FaceKey};
use sweep::test_support::at_rest;
type AB = topo::AtRestBody<f64>;

fn cyl(cx: f64, cy: f64, r: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(cx, cy), r, tol).unwrap();
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, z0));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance { depth: z1 - z0, side: ExtrudeSide::Along }, tol)
        .unwrap()
        .body
}

fn turned(b: &Body<f64>, axis: Vec3<f64>, angle: f64) -> Body<f64> {
    topo::transform_rigid(
        b,
        &geom_core::test_support::rigid(
            &Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), axis, angle),
            Tol::witness(),
        ),
        Tol::witness(),
    )
    .unwrap()
}

fn g<E>(r: Result<topo::BooleanResult<f64>, E>) -> Option<AB> {
    r.ok().and_then(|r| r.body().map(|b| b.body.clone()))
}

fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

fn short(s: String) -> String {
    s.chars().take(140).collect()
}

fn door(b: &Body<f64>) -> String {
    match topo::mass_properties(b, Tol::witness()) {
        Ok(m) => format!("OK v={:.9e} a={:.9e}", m.volume, m.surface_area),
        Err(e) => short(format!("ERR {e:?}")),
    }
}

/// Every sphere-carrying body this reviewer could build.
pub(crate) fn corpus() -> Vec<(String, AB)> {
    let tol = Tol::witness();
    let mut out: Vec<(String, AB)> = Vec::new();
    let mut push = |name: String, b: Option<AB>| {
        if let Some(b) = b {
            out.push((name, b));
        } else {
            println!("SKIP {name}");
        }
    };
    let bool3 = |a: &AB, b: &AB| -> Vec<(&'static str, Option<AB>)> {
        vec![
            ("u", g(topo::boolean::union(a, b, tol))),
            ("i", g(topo::boolean::intersect(a, b, tol))),
            ("a-b", g(topo::boolean::subtract(a, b, tol))),
            ("b-a", g(topo::boolean::subtract(b, a, tol))),
        ]
    };
    for &r in &[1e-3, 0.37, 1.0, 250.0] {
        for (pi, pole) in [v(0.0, 0.0, 1.0), v(0.3, -0.5, 0.8), v(1.0, 0.0, 0.0)].iter().enumerate() {
            let p = *pole / pole.norm();
            let c = v(0.7 * r + 3.0, -2.0, 1.5);
            push(format!("ball r={r} pole{pi}"), Some(finished("b", ball_poled(r, c, p, tol), tol)));
        }
    }
    push("ball_y".into(), Some(finished("b", ball_poled_y(1.0, v(0.2, 0.0, 0.0), tol), tol)));
    push("ball_z".into(), Some(finished("b", ball_poled_z(1.0, v(0.0, 0.3, -1.0), tol), tol)));
    push("dome".into(), Some(at_rest(&dome(1.0, tol), tol)));
    push("lantern".into(), Some(at_rest(&lantern(tol), tol)));
    push("domed_cavity".into(), Some(at_rest(&domed_cavity(tol), tol)));
    push("waisted".into(), Some(at_rest(&waisted(tol), tol)));
    let unit = finished("a", ball_poled(1.0, v(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), tol), tol);
    let tilt = finished("a", ball_poled(1.0, v(0.1, 0.0, 0.0), v(0.6, 0.0, 0.8), tol), tol);
    let small_in = finished("s", ball_poled(0.4, v(0.1, 0.2, -0.1), v(0.0, 1.0, 0.0), tol), tol);
    let small_in_z = finished("s", ball_poled(0.4, v(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), tol), tol);
    let off = finished("o", ball_poled(0.8, v(0.9, 0.3, 0.2), v(0.0, 0.6, 0.8), tol), tol);
    let far = finished("f", ball_poled(0.5, v(5.0, 0.0, 0.0), v(0.0, 0.0, 1.0), tol), tol);
    let bigbrick = finished("k", brick((-2.0, 2.0), (-2.0, 2.0), (-2.0, 2.0), tol), tol);
    let halfbrick = finished("k", brick((-2.0, 2.0), (-2.0, 2.0), (0.2, 2.0), tol), tol);
    let tallbar = finished("k", brick((-0.3, 0.3), (-0.2, 0.2), (-2.0, 2.0), tol), tol);
    let rod = at_rest(&cyl(0.1, 0.0, 0.3, -2.0, 2.0), tol);
    let rodx = at_rest(&turned(&cyl(0.0, 0.1, 0.25, -2.0, 2.0), v(1.0, 0.0, 0.0), 0.7), tol);
    let stub = at_rest(&cyl(0.5, 0.2, 0.4, 0.3, 2.0), tol);
    let eqbrick = finished("k", brick((-2.0, 2.0), (-2.0, 2.0), (0.0, 2.0), tol), tol);
    for (an, a) in [("unit", &unit), ("tilt", &tilt)] {
        for (bn, b) in [
            ("small_in", &small_in),
            ("small_in_z", &small_in_z),
            ("off", &off),
            ("far", &far),
            ("bigbrick", &bigbrick),
            ("halfbrick", &halfbrick),
            ("tallbar", &tallbar),
            ("rod", &rod),
            ("rodx", &rodx),
            ("stub", &stub),
            ("eqbrick", &eqbrick),
        ] {
            for (op, r) in bool3(a, b) {
                push(format!("{an} {op} {bn}"), r);
            }
        }
    }
    // A hollow ball with a rod through it, a void in a brick next to a ball.
    if let Some(hollow) = g(topo::boolean::subtract(&unit, &small_in, tol)) {
        push("hollow - rod".into(), g(topo::boolean::subtract(&hollow, &rod, tol)));
        push("hollow u far".into(), g(topo::boolean::union(&hollow, &far, tol)));
    }
    if let Some(void) = g(topo::boolean::subtract(&bigbrick, &small_in, tol)) {
        push("void - unit?".into(), g(topo::boolean::subtract(&void, &off, tol)));
        push("void u far".into(), g(topo::boolean::union(&void, &far, tol)));
    }
    if let Some(two) = g(topo::boolean::union(&unit, &far, tol)) {
        push("two - rod".into(), g(topo::boolean::subtract(&two, &rod, tol)));
    }
    let mut rng = test_utils::fuzz::start("review_5556");
    for i in 0..12 {
        let rb = rng.range(0.4, 1.2);
        let dx = rng.range(-1.5, 1.5);
        let dy = rng.range(-1.5, 1.5);
        let dz = rng.range(-1.5, 1.5);
        let p = v(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), rng.range(0.1, 1.0));
        let b = finished("b", ball_poled(rb, v(dx, dy, dz), p / p.norm(), tol), tol);
        for (op, r) in bool3(&tilt, &b) {
            push(format!("fuzz{i} {op}"), r);
        }
    }
    out
}

fn sphere_faces(body: &Body<f64>) -> Vec<(FaceKey, topo::SurfaceKey)> {
    body.faces()
        .filter(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Sphere { .. })))
        .map(|(k, f)| (k, f.surface))
        .collect()
}

/// Differential dump: run in both trees, diff.
#[test]
#[ignore = "reviewer diagnostic: prints, diff against the merge base"]
fn review_5556_door_dump() {
    let tol = Tol::witness();
    println!("EPS {}", tol.eps());
    for (name, b) in corpus() {
        let t3 = topo::validate_geometric(&b, tol).is_ok();
        println!("ROW {name} | t3={t3} | {}", door(&b));
    }
}

/// Parity over flips (singles, same-surface pairs, all-on-surface),
/// classifying every (tier 3 refuses, door measures) row by what tier
/// 3 said.
#[test]
#[ignore = "reviewer diagnostic: prints, diff against the merge base"]
fn review_5556_flip_parity() {
    let tol = Tol::witness();
    let mut bad = Vec::new();
    let mut le0 = 0;
    let mut n = 0;
    for (name, b) in corpus() {
        if topo::validate_geometric(&b, tol).is_err() {
            continue;
        }
        let sph = sphere_faces(&b);
        let mut sets: Vec<Vec<FaceKey>> = sph.iter().map(|(k, _)| vec![*k]).collect();
        for (i, (a, sa)) in sph.iter().enumerate() {
            for (c, sc) in &sph[i + 1..] {
                if sa == sc {
                    sets.push(vec![*a, *c]);
                }
            }
        }
        let mut surfs: Vec<_> = sph.iter().map(|x| x.1).collect();
        surfs.dedup();
        for s in surfs {
            sets.push(sph.iter().filter(|x| x.1 == s).map(|x| x.0).collect());
        }
        for set in sets {
            n += 1;
            let f = flipped(&b, &set);
            let t3 = topo::validate_geometric(&f, tol);
            let d = topo::mass_properties(&f, tol);
            match (&t3, &d) {
                (Ok(()), Err(e)) => bad.push(format!("{name} {set:?}: t3 ok, door {e:?}")),
                (Err(errs), Ok(m)) if m.volume > 0.0 => {
                    bad.push(short(format!("{name} {set:?}: t3 {errs:?}, door {}", m.volume)))
                }
                (Err(errs), Ok(m)) => {
                    le0 += 1;
                    println!("LE0 {name} {set:?} v={} t3={}", m.volume, short(format!("{errs:?}")));
                }
                (Ok(()), Ok(_)) => println!("BOTHOK {name} {set:?}"),
                _ => {}
            }
        }
    }
    println!("{n} flips, {le0} door<=0, {} bad", bad.len());
    for b in &bad {
        println!("BAD {b}");
    }
    assert!(bad.is_empty());
}

/// Interval lane: a ball and a two-ball union at the certified scalar,
/// every single flip of a sphere face, door vs the f64 door's kind.
#[test]
#[ignore = "reviewer diagnostic: prints, diff against the merge base"]
fn review_5556_interval_lane() {
    use geom_core::{Interval, Real};
    use sweep::test_support::ball_poled_z_at;
    let tol = Tol::witness();
    let iv = Interval::from_f64;
    let a = finished("a", ball_poled_z_at(iv(1.0), Vec3::new(iv(0.0), iv(0.0), iv(0.0)), tol), tol);
    let b = finished("b", ball_poled_z_at(iv(0.8), Vec3::new(iv(0.9), iv(0.3), iv(0.2)), tol), tol);
    let mut bodies: Vec<(&str, topo::AtRestBody<Interval>)> = vec![("ball", a.clone())];
    for (n, r) in [
        ("u", topo::boolean::union(&a, &b, tol)),
        ("i", topo::boolean::intersect(&a, &b, tol)),
        ("a-b", topo::boolean::subtract(&a, &b, tol)),
    ] {
        match r.ok().and_then(|r| r.body().map(|x| x.body.clone())) {
            Some(x) => bodies.push((n, x)),
            None => println!("ISKIP {n}"),
        }
    }
    for (n, body) in &bodies {
        let d = topo::mass_properties(&**body, tol);
        println!("IROW {n} | {}", short(format!("{:?}", d.as_ref().map(|m| m.volume))));
        let faces: Vec<FaceKey> = body
            .faces()
            .filter(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Sphere { .. })))
            .map(|(k, _)| k)
            .collect();
        for f in faces {
            let fl = body.flipped_face_sense_for_tests(f).unwrap();
            let d = topo::mass_properties(&fl, tol);
            let t3 = topo::validate_geometric(&fl, tol);
            println!(
                "IFLIP {n} {f:?} t3_ok={} | {}",
                t3.is_ok(),
                short(format!("{:?}", d.map(|m| m.volume)))
            );
        }
    }
}

/// The shell door and a face subset: a hollow ball whose outer region
/// has one member flipped; asking for the void shell alone measures it,
/// asking for the outer shell refuses.
#[test]
#[ignore = "reviewer diagnostic: prints, diff against the merge base"]
fn review_5556_shell_door() {
    let tol = Tol::witness();
    let unit = finished("a", ball_poled(1.0, v(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), tol), tol);
    let small = finished("s", ball_poled(0.4, v(0.1, 0.2, -0.1), v(0.0, 1.0, 0.0), tol), tol);
    let hollow = g(topo::boolean::subtract(&unit, &small, tol)).unwrap();
    for (k, s) in hollow.shells() {
        println!("SHELL {k:?} faces {:?}", s.faces);
    }
    for (fk, f) in hollow.faces() {
        if matches!(hollow.get_surface(f.surface), Some(Surface::Sphere { .. })) {
            let fl = hollow.flipped_face_sense_for_tests(fk).unwrap();
            let shells: Vec<topo::ShellKey> = fl.shells().map(|(k, _)| k).collect();
            for sk in &shells {
                let r = topo::classify_shells_of(&fl, &[*sk], tol);
                println!(
                    "SHELLDOOR flip {fk:?} shell {sk:?} -> {}",
                    short(format!("{:?}", r.map(|v| v.iter().map(|c| c.volume).collect::<Vec<_>>())))
                );
            }
            println!("DOOR flip {fk:?} -> {}", door(&fl));
        }
    }
}

/// Cost: the door over the corpus, and booleans of balls (the gate's
/// sign walks), wall-clock.
#[test]
#[ignore = "reviewer diagnostic: prints, diff against the merge base"]
fn review_5556_cost() {
    let tol = Tol::witness();
    let corpus = corpus();
    let unit = finished("a", ball_poled(1.0, v(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), tol), tol);
    let off = finished("o", ball_poled(0.8, v(0.9, 0.3, 0.2), v(0.0, 0.6, 0.8), tol), tol);
    for rep in 0..3 {
        let t = std::time::Instant::now();
        for _ in 0..20 {
            for (_, b) in &corpus {
                let _ = std::hint::black_box(topo::mass_properties(&**b, tol));
            }
        }
        let door = t.elapsed().as_secs_f64() / 20.0;
        let t = std::time::Instant::now();
        for _ in 0..5 {
            let _ = std::hint::black_box(topo::boolean::union(&unit, &off, tol));
            let _ = std::hint::black_box(topo::boolean::subtract(&unit, &off, tol));
        }
        let boo = t.elapsed().as_secs_f64() / 5.0;
        println!("COST rep{rep} door/corpus {:.3} ms ({} bodies), boolean pair {:.3} ms", door * 1e3, corpus.len(), boo * 1e3);
    }
}

/// Naming: which face the door names for a single flip, against tier
/// 3's CurvedSenseInverted, with each face's own encoding.
#[test]
fn review_5556_naming() {
    use crate::seam_region_common::{seam_region, unencoded};
    let tol = Tol::witness();
    let mut wrong = 0;
    let mut bounded_misnamed = 0;
    for (name, b) in corpus() {
        if topo::validate_geometric(&b, tol).is_err() {
            continue;
        }
        for (f, _) in sphere_faces(&b) {
            let fl = flipped(&b, &[f]);
            let d = topo::mass_properties(&fl, tol);
            let errs = topo::validate_geometric(&fl, tol).err().unwrap_or_default();
            let t3_named: Vec<FaceKey> = errs
                .iter()
                .filter_map(|e| match e {
                    topo::ValidationError::CurvedSenseInverted { face } => Some(*face),
                    _ => None,
                })
                .collect();
            if let Err(topo::MassPropsError::Face { face, source }) = &d
                && *face != f
            {
                wrong += 1;
                let (members, bounded) = seam_region(&b, f);
                if bounded && t3_named.contains(&f) {
                    bounded_misnamed += 1;
                }
                println!(
                    "MISNAME {name}: flipped {f:?} (unenc={}) door names {face:?} (unenc={}) {source:?}; region {members:?} bounded={bounded}; t3 names {t3_named:?}",
                    unencoded(&b, f, tol),
                    unencoded(&b, *face, tol),
                );
            }
        }
    }
    println!("{wrong} misnamed");
    // Red at 54a36b68: on a BOUNDED region (`unit u off` and kin) tier 3
    // names the flipped face and the door names its correct neighbour.
    assert_eq!(bounded_misnamed, 0, "the door names a correct neighbour on a bounded seam region");
}

/// An exterior meridian on a BOUNDED region (a union of two balls):
/// an over-cover whose region the pole read can encode.
#[test]
fn review_5556_bounded_overcover() {
    use crate::seam_region_common::split_outside;
    let tol = Tol::witness();
    let unit = finished("a", ball_poled(1.0, v(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), tol), tol);
    let off = finished("o", ball_poled(0.8, v(0.9, 0.3, 0.2), v(0.0, 0.6, 0.8), tol), tol);
    for (name, r) in [
        ("u off", topo::boolean::union(&unit, &off, tol)),
        ("a-b off", topo::boolean::subtract(&unit, &off, tol)),
    ] {
        let mut b = Body::clone(&r.unwrap().body().unwrap().body);
        for d in [v(0.0, 1.0, 0.0), v(1.0, 0.0, 0.0), v(0.0, 0.0, 1.0), v(-0.6, 0.0, 0.8)] {
            let mut c = b.clone();
            if split_outside(&mut c, d, tol) {
                let t3 = topo::validate_geometric(&c, tol);
                println!("OVER {name} {d:?}: t3={} | door {}", short(format!("{t3:?}")), door(&c));
                // Kills the mutant that skips the census where the pole
                // read encodes a side (the door then measures 9.72).
                assert!(t3.is_err());
                assert!(matches!(
                    topo::mass_properties(&c, tol),
                    Err(topo::MassPropsError::Face {
                        source: geom_brep::props::PropsError::SeamRegionOvercovered { .. },
                        ..
                    })
                ));
                b = c;
                break;
            }
        }
    }
}

/// Corruption: one entity of a two-ball union removed at a time; does
/// the door refuse typed, or panic? (Diff against the merge base.)
#[test]
#[ignore = "reviewer diagnostic: diff against the merge base"]
fn review_5556_corrupt_bodies() {
    use topo::EntityId;
    let tol = Tol::witness();
    let unit = finished("a", ball_poled(1.0, v(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), tol), tol);
    let off = finished("o", ball_poled(0.8, v(0.9, 0.3, 0.2), v(0.0, 0.6, 0.8), tol), tol);
    let b = g(topo::boolean::union(&unit, &off, tol)).unwrap();
    let mut ids: Vec<EntityId> = Vec::new();
    ids.extend(b.faces().map(|(k, _)| EntityId::Face(k)));
    for (_, f) in b.faces() {
        ids.push(EntityId::Loop(f.outer));
    }
    ids.extend(b.edges().map(|(k, _)| EntityId::Edge(k)));
    ids.extend(b.half_edges().map(|(k, _)| EntityId::HalfEdge(k)));
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|info| {
        println!("PANICAT {}", info.location().map(|l| l.to_string()).unwrap_or_default());
    }));
    for id in ids {
        let c = b.with_entity_removed_for_tests(id).unwrap();
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| door(&c)));
        println!("CORRUPT {id:?} -> {}", match r { Ok(s) => s, Err(_) => "PANIC".into() });
    }
    std::panic::set_hook(prev);
}
