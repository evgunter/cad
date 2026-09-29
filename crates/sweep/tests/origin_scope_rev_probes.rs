//! **ORIGIN scope review — execution probes** (PR 3430). Each row tries
//! to make a scoped door (shell, `replace_faces_offset`, point-in-solid)
//! give a wrong answer or an invalid body on a body where one chart is
//! worn across shells or solids. Oracles: closed-form volumes, the
//! pre-move one-solid twin, point classification, tier 3.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_lines,
    clippy::float_cmp
)]

use geom_core::{Point3, Tol, Vec3};
use sweep::test_support::brick;
use topo::{Body, FaceKey, ShellError, SolidContainment, SolidKey};

use crate::shell8_common::{beside_raw, faces_of, tol, volume};

fn plane_faces(body: &Body<f64>, solid: SolidKey, axis: usize, at: f64) -> Vec<FaceKey> {
    faces_of(body, solid)
        .into_iter()
        .filter(|&f| {
            matches!(
                body.get_surface(body.get_face(f).unwrap().surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if ([origin.x, origin.y, origin.z][axis] - at).abs() < 1e-12
                        && [normal.x, normal.y, normal.z][axis].abs() > 0.5
            )
        })
        .collect()
}

fn valid(body: &Body<f64>) {
    assert_eq!(topo::validate_geometric(body, tol()), Ok(()));
}

fn close(got: f64, want: f64, what: &str) {
    assert!(
        (got - want).abs() <= 1e-7,
        "{what}: volume {got} want {want}"
    );
}

fn pis(body: &Body<f64>, solid: SolidKey, q: Point3<f64>) -> SolidContainment {
    let band = geom_core::Band::linear(tol()).unwrap();
    topo::point_in_solid_of(body, solid, q, band, tol()).expect("answers")
}

/// The 6x1x1 slab cut in half; `moved` files the halves as two solids.
fn split_slab(moved: bool) -> Body<f64> {
    let slab = brick((0.0, 6.0), (0.0, 1.0), (0.0, 1.0), Tol::witness());
    let wall = brick((2.5, 3.5), (-1.0, 2.0), (-1.0, 2.0), Tol::witness());
    let Ok(topo::BooleanResult::Body(b)) = topo::subtract(&slab, &wall, tol()) else {
        panic!("no body")
    };
    let mut body = b.body;
    if moved {
        let shells: Vec<topo::ShellKey> = body.shells().map(|(k, _)| k).collect();
        body.move_shells_to_new_solid(&[shells[1]]).unwrap();
    }
    body
}

/// Two unit bricks (x 0..1 and x 3..4), the second one's bottom (z = 1)
/// re-charted onto the first one's top chart with the opposed sense —
/// the PR's own construction.
fn opposed_pair() -> (Body<f64>, SolidKey, SolidKey, FaceKey, FaceKey) {
    let lower = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), Tol::witness());
    let upper = brick((0.0, 1.0), (0.0, 1.0), (1.0, 2.0), Tol::witness());
    let (mut body, up) = beside_raw(&lower, &upper, Vec3::new(3.0, 0.0, 0.0));
    let down = body.solids().map(|(k, _)| k).find(|&k| k != up).unwrap();
    let top = plane_faces(&body, down, 2, 1.0)[0];
    let bottom = plane_faces(&body, up, 2, 1.0)[0];
    let key = body.get_face(top).unwrap().surface;
    let Some(geom::Surface::Plane { normal: n_top, .. }) = body.get_surface(key).cloned() else {
        panic!()
    };
    let bd = body.get_face(bottom).unwrap();
    let Some(geom::Surface::Plane { normal: nb, .. }) = body.get_surface(bd.surface).cloned()
    else {
        panic!()
    };
    let outward = if bd.sense { nb } else { -nb };
    let sense = outward.dot(n_top) > 0.0;
    let rims: Vec<_> = body
        .edges()
        .filter_map(|(e, d)| {
            let face = |he| body.face_of_half_edge(he).unwrap();
            let side = match (face(d.he_plus) == bottom, face(d.he_minus) == bottom) {
                (true, false) => face(d.he_minus),
                (false, true) => face(d.he_plus),
                _ => return None,
            };
            let at = |he| {
                let v = body.get_half_edge(he).unwrap().start;
                *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
            };
            Some((
                e,
                body.get_face(side).unwrap().surface,
                at(d.he_plus),
                at(d.he_minus),
            ))
        })
        .collect();
    body.set_face_surface_and_sense(bottom, topo::FaceSurface::Shared(key), sense)
        .unwrap();
    for (edge, wall, p, q) in rims {
        let len = p.distance(q);
        let spec = geom_brep::EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::Intersection {
                s1: wall,
                s2: key,
                witness: p + (q - p) * 0.5,
            },
            carrier: geom::Curve3::Line {
                origin: p,
                dir: (q - p) / len,
            },
            param_start: 0.0,
            param_end: len,
        };
        body.set_edge_curve(edge, spec, tol()).unwrap();
    }
    valid(&body);
    (body, down, up, top, bottom)
}

/// Opposed senses across solids, then OPEN on the shared chart: one
/// side, the other side, both. Before the PR each refused
/// `ChartSenseMixed`; the rim must open only the named solid's face.
#[test]
fn rev_opposed_pair_opens_on_the_shared_chart_per_solid() {
    let t = 0.1;
    let (body, down, up, top, bottom) = opposed_pair();
    let closed = 1.0 - 0.8 * 0.8 * 0.8;
    let open = 1.0 - 0.8 * 0.8 * 0.9;
    for (open_faces, want) in [
        (vec![top], open + closed),
        (vec![bottom], closed + open),
        (vec![top, bottom], 2.0 * open),
        (vec![bottom, top], 2.0 * open),
    ] {
        let s = topo::shell_open(&body, t, &open_faces, tol())
            .unwrap_or_else(|e| panic!("{open_faces:?}: {e}"));
        println!(
            "[rev] opposed pair opened at {open_faces:?}: {}",
            volume(&s.body)
        );
        valid(&s.body);
        close(volume(&s.body), want, "opposed pair opened");
        assert_eq!(s.body.solids().count(), 2);
    }
    let _ = (down, up);
}

/// Same-sense sharing across solids (the split slab), OPEN on a shared
/// chart for one solid only, then for both; against the pre-move
/// one-solid twin, which must still refuse the partial designation.
#[test]
fn rev_split_slab_opens_one_solids_top_and_matches_the_one_solid_twin() {
    let t = 0.05;
    let closed = 2.5 - 2.4 * 0.9 * 0.9;
    let open = 2.5 - 2.4 * 0.9 * 0.95;

    // The pre-move one-solid twin (two OUTER shells) is refused
    // upstream of any chart grouping.
    let one = split_slab(false);
    let s1 = one.solids().next().unwrap().0;
    let tops_one = plane_faces(&one, s1, 2, 1.0);
    assert!(matches!(
        topo::shell_open(&one, t, &tops_one[..1], tol()),
        Err(ShellError::OperandOuterShells { .. })
    ));

    let two = split_slab(true);
    let solids: Vec<SolidKey> = two.solids().map(|(k, _)| k).collect();
    let a = plane_faces(&two, solids[0], 2, 1.0);
    let b = plane_faces(&two, solids[1], 2, 1.0);
    let bb = plane_faces(&two, solids[1], 2, 0.0);
    for (faces, want) in [
        (a.clone(), open + closed),
        (b.clone(), open + closed),
        ([a.clone(), b.clone()].concat(), 2.0 * open),
        ([b.clone(), a.clone()].concat(), 2.0 * open),
        ([a.clone(), bb.clone()].concat(), 2.0 * open),
    ] {
        let s =
            topo::shell_open(&two, t, &faces, tol()).unwrap_or_else(|e| panic!("{faces:?}: {e}"));
        valid(&s.body);
        close(volume(&s.body), want, "split slab opened");
        assert_eq!(s.body.solids().count(), 2);
        // Oracle: every thin solid holds its own wall point and not
        // its cavity centre.
        let wall_a = Point3::new(1.0, 0.025, 0.5);
        let wall_b = Point3::new(5.0, 0.025, 0.5);
        let hollow_a = Point3::new(1.0, 0.5, 0.5);
        let hits: Vec<_> = s
            .body
            .solids()
            .map(|(k, _)| {
                (
                    pis(&s.body, k, wall_a),
                    pis(&s.body, k, wall_b),
                    pis(&s.body, k, hollow_a),
                )
            })
            .collect();
        println!("[rev] split slab {faces:?}: {hits:?}");
        use SolidContainment::{In, Out};
        assert!(
            hits.contains(&(In, Out, Out)) && hits.contains(&(Out, In, Out)),
            "{hits:?}"
        );
    }
}

/// Curved (the axial door, `ChartsTogether` lift): a ball cut by two
/// slabs into a top cap, a zone and a bottom cap, all wearing the
/// ball's one sphere key, filed as three solids. The zone takes TWO
/// rims (the second lift runs after the first rim's retirements) beside
/// a cap's single rim; every lift's zero-distance moves include the
/// shared sphere chart, which must be the lift solid's own wearer only.
/// Oracle: additivity over solids and the caps' symmetry.
#[test]
fn rev_three_piece_ball_opens_many_rims_additively() {
    let t = 0.05;
    let ball = sweep::test_support::ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol());
    let s1 = brick((-2.0, 2.0), (-2.0, 2.0), (0.3, 0.5), Tol::witness());
    let s2 = brick((-2.0, 2.0), (-2.0, 2.0), (-0.5, -0.3), Tol::witness());
    let Ok(topo::BooleanResult::Body(b)) = topo::subtract(&ball, &s1, tol()) else {
        panic!("first cut")
    };
    let body = match topo::subtract(&b.body, &s2, tol()) {
        Ok(topo::BooleanResult::Body(b)) => b.body,
        other => panic!("second cut: {:?}", other.err()),
    };
    valid(&body);
    let mut body = body;
    let shells: Vec<topo::ShellKey> = body.shells().map(|(k, _)| k).collect();
    println!(
        "[rev] three-piece ball: solids={} shells={}",
        body.solids().count(),
        shells.len()
    );
    assert_eq!(shells.len(), 3);
    for &sh in &shells[1..] {
        body.move_shells_to_new_solid(&[sh]).unwrap();
    }
    valid(&body);
    let solids: Vec<SolidKey> = body.solids().map(|(k, _)| k).collect();
    let at = |z: f64| -> Vec<FaceKey> {
        solids
            .iter()
            .flat_map(|&s| plane_faces(&body, s, 2, z))
            .collect()
    };
    let (top_cap, zone_up, zone_down, bot_cap) = (at(0.5), at(0.3), at(-0.3), at(-0.5));
    assert_eq!(
        (top_cap.len(), zone_up.len(), zone_down.len(), bot_cap.len()),
        (1, 1, 1, 1)
    );
    let sphere_wearers: Vec<_> = body
        .faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Sphere { .. })
            )
        })
        .map(|(_, f)| f.surface)
        .collect();
    println!(
        "[rev] sphere wearers {} distinct keys {:?}",
        sphere_wearers.len(),
        {
            let mut k = sphere_wearers.clone();
            k.dedup();
            k.len()
        }
    );
    let vol_of = |faces: &[FaceKey]| -> f64 {
        let s =
            topo::shell_open(&body, t, faces, tol()).unwrap_or_else(|e| panic!("{faces:?}: {e}"));
        valid(&s.body);
        assert_eq!(s.body.solids().count(), 3);
        volume(&s.body)
    };
    let closed = topo::shell(&body, t, tol()).unwrap();
    valid(&closed.body);
    let c = volume(&closed.body);
    // Per-solid closed volumes are not equal; read each opening as a
    // delta against the closed body.
    let d_top = vol_of(&top_cap) - c;
    let d_bot = vol_of(&bot_cap) - c;
    close(d_top, d_bot, "cap symmetry");
    let d_zone2 = vol_of(&[zone_up[0], zone_down[0]]) - c;
    let d_zone2r = vol_of(&[zone_down[0], zone_up[0]]) - c;
    close(d_zone2, d_zone2r, "zone two rims, either order");
    let d_up = vol_of(&zone_up) - c;
    let d_down = vol_of(&zone_down) - c;
    close(d_up, d_down, "zone symmetry");
    let mixed = vol_of(&[zone_up[0], top_cap[0], zone_down[0]]) - c;
    let mixed_r = vol_of(&[top_cap[0], zone_down[0], zone_up[0]]) - c;
    let all = vol_of(&[bot_cap[0], zone_up[0], top_cap[0], zone_down[0]]) - c;
    println!("[rev] d_top={d_top} d_zone2={d_zone2} d_up={d_up} mixed={mixed} all={all}");
    close(mixed, d_zone2 + d_top, "additivity");
    close(mixed_r, d_zone2 + d_top, "additivity, reversed");
    close(all, d_zone2 + 2.0 * d_top, "all open");
    assert!(
        d_top < 0.0 && d_zone2 < d_up && d_up < 0.0,
        "openings remove material"
    );
}

/// `replace_faces_offset` inside ONE solid across two shells: naming
/// one shell's wearer still refuses (the scope is the solid), naming
/// both moves them; after the move, naming one is served.
#[test]
fn rev_replace_faces_offset_one_solid_two_shells() {
    let mut one = split_slab(false);
    let s = one.solids().next().unwrap().0;
    let tops = plane_faces(&one, s, 2, 1.0);
    let e = topo::replace_faces_offset(&mut one.clone(), &tops[..1], -0.1, tol()).unwrap_err();
    assert!(
        matches!(e, topo::ReplaceFaceError::SharedSurfaceKey { .. }),
        "{e}"
    );
    let before = volume(&one);
    topo::replace_faces_offset(&mut one, &tops, -0.1, tol()).expect("both wearers");
    valid(&one);
    close(
        volume(&one),
        before - 2.0 * 2.5 * 0.1,
        "one solid both tops",
    );

    // After the move, naming both solids' wearers is one call too.
    let mut two = split_slab(true);
    let solids: Vec<SolidKey> = two.solids().map(|(k, _)| k).collect();
    let both = [
        plane_faces(&two, solids[0], 2, 1.0),
        plane_faces(&two, solids[1], 2, 1.0),
    ]
    .concat();
    let before = volume(&two);
    topo::replace_faces_offset(&mut two, &both, -0.1, tol()).expect("both solids' wearers");
    valid(&two);
    close(
        volume(&two),
        before - 2.0 * 2.5 * 0.1,
        "two solids both tops",
    );
    // And the opposed pair: offset the shared chart for the lower only.
    let (mut pair, down, up, top, bottom) = opposed_pair();
    let before = volume(&pair);
    topo::replace_faces_offset(&mut pair, &[top], -0.1, tol()).expect("lower's top alone");
    valid(&pair);
    close(volume(&pair), before - 0.1, "opposed: lower shrinks only");
    assert_eq!(
        pis(&pair, down, Point3::new(0.5, 0.5, 0.95)),
        SolidContainment::Out
    );
    assert_eq!(
        pis(&pair, up, Point3::new(3.5, 0.5, 1.05)),
        SolidContainment::In
    );
    let _ = bottom;
}

/// Point-in-solid on the UNMOVED ball caps: one solid, two shells, one
/// sphere key across them (the old guard refused `of_shell` here, so
/// check 10 was silent); and the whole-body entry after the move.
#[test]
fn rev_point_in_solid_ball_caps_one_solid_and_whole_body() {
    let ball = sweep::test_support::ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol());
    let slab = brick((-2.0, 2.0), (-2.0, 2.0), (-0.2, 0.2), Tol::witness());
    let Ok(topo::BooleanResult::Body(b)) = topo::subtract(&ball, &slab, tol()) else {
        panic!()
    };
    let mut body = b.body;
    valid(&body);
    let s = body.solids().next().unwrap().0;
    use SolidContainment::{In, Out};
    for (q, want) in [
        (Point3::new(0.0, 0.0, 0.6), In),
        (Point3::new(0.0, 0.0, -0.6), In),
        (Point3::new(0.0, 0.0, 0.0), Out),
        (Point3::new(0.0, 0.0, 1.5), Out),
        (Point3::new(0.75, 0.0, 0.75), Out),
        (Point3::new(0.6, 0.0, -0.6), In),
    ] {
        assert_eq!(pis(&body, s, q), want, "one solid at {q:?}");
    }
    let shells: Vec<topo::ShellKey> = body.shells().map(|(k, _)| k).collect();
    body.move_shells_to_new_solid(&[shells[1]]).unwrap();
    valid(&body);
    let band = geom_core::Band::linear(tol()).unwrap();
    for (q, want) in [
        (Point3::new(0.0, 0.0, 0.6), In),
        (Point3::new(0.0, 0.0, -0.6), In),
        (Point3::new(0.0, 0.0, 0.0), Out),
        (Point3::new(0.0, 0.0, 1.5), Out),
    ] {
        assert_eq!(
            topo::point_in_solid(&body, q, band, tol()).unwrap(),
            want,
            "whole body at {q:?}"
        );
    }
    // Shell the two caps (curved, sphere chart across solids).
    match topo::shell(&body, 0.05, tol()) {
        Ok(s) => {
            valid(&s.body);
            println!(
                "[rev] caps shelled: solids={} vol={}",
                s.body.solids().count(),
                volume(&s.body)
            );
        }
        Err(e) => println!("[rev] caps shell refuses: {e}"),
    }
}
