//! **A union's holdings over a curved run only several members open**:
//! a plate, a boss sunk through its top whose wall is two half-cylinders,
//! and bars over the boss. A bar across the boss cuts the wall the plate
//! leaves into pieces, each holding area of both half-cylinders, which
//! no fold step glued by itself; a second bar over both seams cuts each
//! piece again, so each holds one half-cylinder (`topo::Holdings`,
//! `topo::overlap`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use geom_core::{Point2, Tol};
use sweep::test_support::{brick, cylinder_of_arcs_at, finished};
use topo::test_support::holdings::{rejoin_oracle, union_in};
use topo::test_support::meeting::orders;
use topo::{AtRestBody, Body, FaceKey, Surface};

fn tol() -> Tol {
    Tol::witness()
}

fn curved(body: &Body<f64>) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| !matches!(body.get_surface(f.surface), Some(Surface::Plane { .. })))
        .map(|(k, _)| k)
        .collect()
}

/// The plate, the two-arc boss sunk through its top, and the bar across
/// it over neither seam.
fn plate_boss_bar() -> Vec<AtRestBody<f64>> {
    let t = tol();
    vec![
        finished(
            "plate",
            brick::<f64>((0.0, 3.0), (0.0, 3.0), (0.0, 1.0), t),
            t,
        ),
        finished(
            "boss",
            cylinder_of_arcs_at(2, 0.5, Point2::new(1.5, 1.5), 0.5, 1.0, t),
            t,
        ),
        finished(
            "bar",
            brick::<f64>((1.4, 1.6), (0.5, 2.5), (0.8, 2.0), t),
            t,
        ),
    ]
}

/// **Every member order reads one holdings table**, in which each piece
/// of the boss's wall holds both half-cylinders; the rejoin of each
/// member that builds reads the same (the plate's refuses: it lies on
/// the finished boundary and never crosses it).
#[test]
fn a_bar_over_a_sunk_boss_holds_both_wall_halves_in_every_order() {
    let t = tol();
    let bodies = plate_boss_bar();
    let walls: BTreeSet<(usize, FaceKey)> =
        curved(&bodies[1]).into_iter().map(|k| (1, k)).collect();
    assert_eq!(walls.len(), 2, "the boss's wall is two half-cylinders");
    let mut tables = BTreeSet::new();
    for order in orders(bodies.len()) {
        let (result, table, _) = union_in(&bodies, &order, t);
        let pieces = curved(&result);
        assert_eq!(
            pieces.len(),
            2,
            "order {order:?}: the bar cuts the wall in two"
        );
        for f in &pieces {
            assert_eq!(
                table[f], walls,
                "order {order:?}: wall piece {f:?} holds both half-cylinders"
            );
        }
        let built = rejoin_oracle(&bodies, &order, &result, &table, (false, t));
        assert!(
            built >= 2,
            "order {order:?}: the boss's and the bar's rejoins build"
        );
        let mut rows: Vec<_> = table.into_values().collect();
        rows.sort();
        tables.insert(rows);
    }
    assert_eq!(tables.len(), 1, "one holdings table in every member order");
}

/// **A second bar over both seams leaves each wall piece holding one
/// half-cylinder, in every member order**: where the fold glues the
/// halves before a bar cuts them, each piece of the glued wall is read
/// against each half (`topo::overlap`), and holds only the half it lies
/// on.
#[test]
fn two_bars_over_a_sunk_boss_leave_each_wall_piece_one_half_in_every_order() {
    let t = tol();
    let mut bodies = plate_boss_bar();
    bodies.push(finished(
        "bar over the seams",
        brick::<f64>((0.5, 2.5), (1.4, 1.6), (0.8, 1.9), t),
        t,
    ));
    let walls: BTreeSet<(usize, FaceKey)> =
        curved(&bodies[1]).into_iter().map(|k| (1, k)).collect();
    let mut tables = BTreeSet::new();
    for order in orders(bodies.len()) {
        let (result, table, _) = union_in(&bodies, &order, t);
        let pieces = curved(&result);
        assert_eq!(
            pieces.len(),
            4,
            "order {order:?}: the bars cut the wall in four"
        );
        let mut per_half = [0, 0];
        for f in &pieces {
            let held = &table[f];
            assert!(
                held.len() == 1 && held.is_subset(&walls),
                "order {order:?}: wall piece {f:?} holds one half-cylinder: {held:?}"
            );
            let half = walls.iter().position(|w| held.contains(w)).unwrap();
            per_half[half] += 1;
        }
        assert_eq!(per_half, [2, 2], "order {order:?}: two pieces on each half");
        rejoin_oracle(&bodies, &order, &result, &table, (false, t));
        let mut rows: Vec<_> = table.into_values().collect();
        rows.sort();
        tables.insert(rows);
    }
    assert_eq!(tables.len(), 1, "one holdings table in every member order");
}

/// Where face `f` of an `n`-arc boss of radius `r` about `c` starts
/// round its axis, in whole degrees, counterclockwise: each wall face
/// spans `360 / n` from a corner, the one whose span's middle, at height
/// `z`, the face holds.
fn starts_at(body: &Body<f64>, f: FaceKey, n: u32, (c, r, z): (Point2<f64>, f64, f64)) -> u32 {
    let mut at: Vec<u32> = body
        .vertex_points()
        .filter(|&(v, _)| body.faces_of_vertex(v).unwrap().contains(&f))
        .map(|(_, p)| {
            let deg = (p.y - c.y).atan2(p.x - c.x).to_degrees().rem_euclid(360.0);
            u32::try_from(deg.round() as i64).unwrap() % 360
        })
        .collect();
    at.sort_unstable();
    at.dedup();
    let band = geom_core::Band::linear(tol()).unwrap();
    let holds_middle = |s: u32| {
        let mid = f64::from(s + 180 / n).to_radians();
        let p = geom_core::Point3::new(c.x + r * mid.cos(), c.y + r * mid.sin(), z);
        topo::curved_face_containment(body, f, p, band).unwrap() == Some(topo::FaceContainment::In)
    };
    let starts: Vec<u32> = at.into_iter().filter(|&s| holds_middle(s)).collect();
    let [s] = starts[..] else {
        panic!("a wall face holds the middle of one span from its corners: {starts:?}");
    };
    s
}

/// **`overlap` on a cylinder**: each half of a two-arc boss's wall
/// against each third of a three-arc boss's wall on the same cylinder,
/// shifted up half its height, meets it in area exactly where their
/// spans round the axis overlap; two that share only a seam ruling
/// touch from either side of it, and share no area.
#[test]
fn overlap_reads_wall_pieces_of_one_cylinder_by_their_spans() {
    let t = tol();
    let band = geom_core::Band::linear(t).unwrap();
    let c = Point2::new(0.0, 0.0);
    let halves = cylinder_of_arcs_at(2, 0.5, c, 0.0, 1.0, t);
    let thirds = cylinder_of_arcs_at(3, 0.5, c, 0.5, 1.0, t);
    let mut seen = [0, 0];
    for h in curved(&halves) {
        let sh = starts_at(&halves, h, 2, (c, 0.5, 0.5));
        for k in curved(&thirds) {
            let sk = starts_at(&thirds, k, 3, (c, 0.5, 1.0));
            let meets = (0..360).any(|d| {
                let x = f64::from(d) + 0.5;
                let within = |s: u32, w: f64| (x - f64::from(s)).rem_euclid(360.0) < w;
                within(sh, 180.0) && within(sk, 120.0)
            });
            let got = topo::overlap((&halves, h), (&thirds, k), band).unwrap();
            assert_eq!(
                got, meets,
                "the half from {sh} degrees and the third from {sk} meet in area: {meets}"
            );
            seen[usize::from(meets)] += 1;
        }
    }
    assert_eq!(
        seen,
        [2, 4],
        "two pairs touch along a seam alone, four overlap"
    );
}

/// **Every op of a slab and two balls a re-chart turns reads every
/// holding**: the balls' sphere faces are turned about their centres
/// before the op runs (`topo::BooleanNaming::recharted`), so each
/// finished face and each lost region of a turned face is read against
/// the input shell's faces its region meets, a subtract's turned faces
/// in the sense the result holds them.
#[test]
fn every_op_over_re_charted_balls_reads_every_holding() {
    let t = tol();
    let ball = |x: f64, y: f64| {
        finished(
            "the pip ball",
            sweep::test_support::ball_poled_y(0.5, geom_core::Vec3::new(x, y, 1.2), t),
            t,
        )
    };
    let pair = topo::union(&ball(2.0, 1.2), &ball(2.0, 2.8), t)
        .expect("disjoint balls assemble")
        .body()
        .expect("a body")
        .body
        .clone();
    let slab = finished("the slab", crate::common::operands::slab::<f64>(), t);
    let mut turned = 0;
    for op in [
        topo::BooleanOp::Subtract,
        topo::BooleanOp::Union,
        topo::BooleanOp::Intersect,
    ] {
        let Ok(topo::BooleanResult::Body(out)) = topo::boolean_op_with(
            op,
            &slab,
            &pair,
            &topo::BooleanInputs::default(),
            topo::SweepStrategy::Realized,
            t,
        ) else {
            panic!("{op:?}: the op builds");
        };
        turned += usize::from(!out.naming.recharted[1].is_empty());
        let rows = out
            .holdings
            .faces()
            .map(|(_, h)| h)
            .chain(out.holdings.lost_rows())
            .chain(out.holdings.uncut_rows());
        for held in rows {
            assert!(
                held.faces().is_some_and(|f| !f.is_empty()),
                "{op:?}: a holding reads: {held:?}"
            );
        }
    }
    assert_eq!(turned, 3, "every op turns the balls' shells");
}
