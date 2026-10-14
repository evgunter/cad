//! Review probes for PR 5099 (a bar wholly inside a concave band's fill
//! is absorbed). Each row prints what the kernel did and asserts only
//! the review's contract: a build is a sound body whose volume is the
//! ruling's `S ∪ F` / `S − M`, else a typed refusal.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Bounds, Point2, Point3, Vec3};
use sweep::blend::BlendError;
use sweep::blend::build::{Blended, chamfer_edges, fillet_edges};
use sweep::test_support::{brick, finished, prism_on, sketch_from_axes};
use topo::{
    Body, ContactRecords, EdgeKey, mass_properties, query, validate_geometric,
    validate_pseudomanifold,
};

use crate::band_planar_cut_off::{D, Verb, tol};

const VERBS: [Verb; 2] = [Verb::Fillet, Verb::Chamfer];

fn mid<T: Bounds>(x: T) -> f64 {
    0.5 * (x.lo() + x.hi())
}

fn boolean(union: bool, a: Body<f64>, b: Body<f64>) -> Body<f64> {
    let (a, b) = (finished("A", a, tol()), finished("B", b, tol()));
    if union {
        topo::union(&a, &b, tol())
    } else {
        topo::subtract(&a, &b, tol())
    }
    .unwrap_or_else(|e| panic!("fixture boolean: {e}"))
    .body()
    .expect("a body")
    .body
    .clone()
    .into_body()
}

/// The PR's planar die at unit scale, lean 0: pocket over
/// `[¼,¾]×[¼,¾]` from `z = ½`, floor edge `(¼,¼,½)→(¾,¼,½)`.
fn die() -> Body<f64> {
    let block = brick((-1.0, 2.0), (0.0, 1.0), (0.0, 1.0), tol());
    let pocket = prism_on(
        sketch_from_axes(
            Point3::new(0.0, 0.0, 0.5),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            tol(),
        ),
        [(0.25, 0.25), (0.75, 0.25), (0.75, 0.75), (0.25, 0.75)]
            .into_iter()
            .map(|(x, y)| (Point2::new(x, y), 0.0))
            .collect(),
        1.0,
        tol(),
    );
    boolean(false, block, pocket)
}

/// A round rod along `x` from `x0` to `x1` at `(y, z)`, radius `a`.
fn rod(x0: f64, x1: f64, (y, z): (f64, f64), a: f64) -> Body<f64> {
    prism_on(
        sketch_from_axes(
            Point3::new(x0, y, z),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            tol(),
        ),
        vec![(Point2::new(-a, 0.0), 1.0), (Point2::new(a, 0.0), 1.0)],
        x1 - x0,
        tol(),
    )
}

fn cube(c: (f64, f64, f64), h: f64) -> Body<f64> {
    brick(
        (c.0 - h, c.0 + h),
        (c.1 - h, c.1 + h),
        (c.2 - h, c.2 + h),
        tol(),
    )
}

fn edge(body: &Body<f64>, a: [f64; 3], b: [f64; 3]) -> EdgeKey {
    let near = |p: Point3<f64>, q: [f64; 3]| {
        (p.x - q[0]).abs() + (p.y - q[1]).abs() + (p.z - q[2]).abs() < 1e-9
    };
    let point = |v| {
        *body
            .get_point(body.get_vertex(v).expect("v").point)
            .expect("p")
    };
    query::all_edges(body)
        .into_iter()
        .find(|&e| {
            let he = body.get_edge(e).expect("e").he_plus;
            let (s, t) = (
                point(body.get_half_edge(he).expect("h").start),
                point(body.half_edge_end(he).expect("end")),
            );
            (near(s, a) && near(t, b)) || (near(s, b) && near(t, a))
        })
        .unwrap_or_else(|| panic!("edge {a:?}→{b:?}"))
}

fn floor(body: &Body<f64>) -> EdgeKey {
    edge(body, [0.25, 0.25, 0.5], [0.75, 0.25, 0.5])
}

fn blend(
    verb: Verb,
    body: &Body<f64>,
    edges: &[EdgeKey],
    r: f64,
) -> Result<Blended<f64>, BlendError> {
    let body = finished("operand", body.clone(), tol());
    match verb {
        Verb::Fillet => fillet_edges(&body, edges, r, tol()),
        Verb::Chamfer => chamfer_edges(&body, edges, r, tol()),
    }
    .map_err(|e| e.error)
}

fn vol(body: &Body<f64>) -> f64 {
    mid(mass_properties(body, tol()).expect("props").volume)
}

fn section(verb: Verb, r: f64) -> f64 {
    verb.section_at(r, PI / 2.0)
}

fn rings(body: &Body<f64>) -> usize {
    body.faces().map(|(_, f)| f.rings.len()).sum()
}

/// Either a typed refusal (printed), or a sound body whose ΔV is `added`.
/// Returns whether it built.
fn sound_or_typed(
    what: &str,
    verb: Verb,
    body: &Body<f64>,
    edges: &[EdgeKey],
    added: Option<f64>,
) -> bool {
    match blend(verb, body, edges, D) {
        Err(e) => {
            eprintln!("PROBE {what} ({verb:?}): refused {e:?}");
            assert!(
                !matches!(e, BlendError::BodyNotIntact { .. }),
                "{what} ({verb:?}): an untyped refusal {e:?}"
            );
            false
        }
        Ok(out) => {
            let c0 = topo::readback::euler_counts(body);
            let c1 = topo::readback::euler_counts(&out.body);
            let dv = vol(&out.body) - vol(body);
            eprintln!(
                "PROBE {what} ({verb:?}): BUILT shells {}→{} genus {:?}→{:?} rings {}→{} ΔV {dv} (expected {added:?})",
                c0.s,
                c1.s,
                c0.genus(),
                c1.genus(),
                rings(body),
                rings(&out.body)
            );
            validate_geometric(&out.body, tol())
                .unwrap_or_else(|e| panic!("{what} ({verb:?}): tier 3 {e:?}"));
            validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
                .unwrap_or_else(|e| panic!("{what} ({verb:?}): tier 3′ {e:?}"));
            if let Some(added) = added {
                assert!(
                    (dv - added).abs() < 1e-8,
                    "{what} ({verb:?}): ΔV {dv} vs S∪F's {added}"
                );
            }
            true
        }
    }
}

const BAR: (f64, f64) = (0.012, 0.006);

/// **A blind hole drilled into one end wall, its mouth inside the wall's
/// sliver.** Its faces lie beyond the cap plane, in the wall: outside
/// the prism. Under `S ∪ F` the fill caps its mouth and it becomes a
/// closed void (two shells, ΔV = +A·L). On main the concave cap ring
/// refused `WhollyInside`; on the head it is a consumable `CapRing` no
/// consumed face uses.
#[test]
fn probe_blind_hole_into_an_end_wall_inside_the_sliver() {
    let (d, a) = BAR;
    let body = boolean(false, die(), rod(0.1, 0.3, (0.25 + d, 0.5 + d), a));
    for verb in VERBS {
        let added = section(verb, D) * 0.5;
        if sound_or_typed("blind hole", verb, &body, &[floor(&body)], Some(added)) {
            let out = blend(verb, &body, &[floor(&body)], D).unwrap();
            let c1 = topo::readback::euler_counts(&out.body);
            assert_eq!(
                c1.s, 2,
                "blind hole ({verb:?}): S ∪ F leaves the hole a closed void"
            );
        }
    }
}

/// The same hole in both end walls.
#[test]
fn probe_blind_holes_into_both_end_walls() {
    let (d, a) = BAR;
    let body = boolean(false, die(), rod(0.1, 0.3, (0.25 + d, 0.5 + d), a));
    let body = boolean(false, body, rod(0.7, 0.9, (0.25 + d, 0.5 + d), a));
    for verb in VERBS {
        let added = section(verb, D) * 0.5;
        if sound_or_typed("two blind holes", verb, &body, &[floor(&body)], Some(added)) {
            let out = blend(verb, &body, &[floor(&body)], D).unwrap();
            assert_eq!(topo::readback::euler_counts(&out.body).s, 3, "two voids");
        }
    }
}

/// **The convex mirror, for comparison (main's gate too)**: a pin out of a
/// box's end face, its footprint inside the convex sliver. `S − M` leaves
/// the pin floating, disconnected — no single-shell body is right.
#[test]
fn probe_convex_pin_out_of_an_end_face_inside_the_sliver() {
    let (d, a) = BAR;
    let body = boolean(
        true,
        brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()),
        rod(-0.2, 0.05, (1.0 - d, 1.0 - d), a),
    );
    let e = edge(&body, [0.0, 1.0, 1.0], [1.0, 1.0, 1.0]);
    for verb in VERBS {
        if sound_or_typed("convex pin out", verb, &body, &[e], None) {
            let out = blend(verb, &body, &[e], D).unwrap();
            let c1 = topo::readback::euler_counts(&out.body);
            panic!(
                "convex pin out ({verb:?}): built {} shells; S − M disconnects the pin",
                c1.s
            );
        }
    }
}

/// **A bar with an enclosed cavity** (a void shell wholly inside the
/// bar). `S ∪ F` fills the void: one shell, ΔV = A·L − π(a²−b²)·L_c −
/// π a²(L − L_c)... written as `A·L − π a²·L + π b²·l`.
#[test]
fn probe_bar_with_an_enclosed_cavity() {
    let (d, a) = BAR;
    let b = 0.003;
    let barred = boolean(true, die(), rod(0.1, 0.9, (0.25 + d, 0.5 + d), a));
    let body = boolean(false, barred, rod(0.35, 0.65, (0.25 + d, 0.5 + d), b));
    assert_eq!(
        topo::readback::euler_counts(&body).s,
        2,
        "fixture: a void shell"
    );
    for verb in VERBS {
        let added = (section(verb, D) - PI * a * a) * 0.5 + PI * b * b * 0.3;
        if sound_or_typed("bar with cavity", verb, &body, &[floor(&body)], Some(added)) {
            let out = blend(verb, &body, &[floor(&body)], D).unwrap();
            assert_eq!(
                topo::readback::euler_counts(&out.body).s,
                1,
                "the void is filled"
            );
        }
    }
}

/// **Two bars inside the fill**: both absorbed, four rings fewer.
#[test]
fn probe_two_bars_inside_the_fill() {
    let a = 0.005;
    let body = boolean(true, die(), rod(0.1, 0.9, (0.262, 0.512), a));
    let body = boolean(true, body, rod(0.1, 0.9, (0.262, 0.53), a));
    for verb in VERBS {
        let added = (section(verb, D) - 2.0 * PI * a * a) * 0.5;
        if sound_or_typed("two bars", verb, &body, &[floor(&body)], Some(added)) {
            let out = blend(verb, &body, &[floor(&body)], D).unwrap();
            assert_eq!(rings(&out.body) + 4, rings(&body), "four rings leave");
        }
    }
}

/// **A bar that runs on out through the block's outer faces**: the pocket
/// segment is absorbed, the two stubs stay.
#[test]
fn probe_bar_through_the_walls_and_out_of_the_block() {
    let (d, a) = BAR;
    let body = boolean(true, die(), rod(-1.3, 2.3, (0.25 + d, 0.5 + d), a));
    for verb in VERBS {
        let added = (section(verb, D) - PI * a * a) * 0.5;
        sound_or_typed(
            "bar out through the block",
            verb,
            &body,
            &[floor(&body)],
            Some(added),
        );
    }
}

/// **A bar with a lump mid-length that pokes out through the band
/// surface**: must refuse.
#[test]
fn probe_bar_with_a_lump_through_the_band_surface() {
    let (d, a) = BAR;
    let body = boolean(true, die(), rod(0.1, 0.9, (0.25 + d, 0.5 + d), a));
    let body = boolean(true, body, cube((0.5, 0.25 + 0.03, 0.5 + 0.03), 0.02));
    for verb in VERBS {
        assert!(
            !sound_or_typed("bar with a lump", verb, &body, &[floor(&body)], None),
            "a lump across the band surface is not wholly inside F"
        );
    }
}

/// **A lump wholly inside F on the bar** (a feature of its own, inside
/// the fill): absorbed with the bar, or a typed refusal.
#[test]
fn probe_bar_with_a_lump_inside_the_fill() {
    let (d, a) = BAR;
    let body = boolean(true, die(), rod(0.1, 0.9, (0.25 + d, 0.5 + d), a));
    let h = 0.004;
    let c = (0.5, 0.25 + d + a + h - 0.002, 0.5 + d);
    let body = boolean(true, body, cube(c, h));
    for verb in VERBS {
        sound_or_typed(
            "bar with an inside lump",
            verb,
            &body,
            &[floor(&body)],
            None,
        );
    }
}

/// **The mitre's union**: the floor's three concave edges filleted
/// together, a bar between the end walls in the floor/side fill.
#[test]
fn probe_bar_under_a_three_edge_floor_chain() {
    let (d, a) = BAR;
    let body = boolean(true, die(), rod(0.1, 0.9, (0.25 + d, 0.5 + d), a));
    let es = [
        floor(&body),
        edge(&body, [0.25, 0.25, 0.5], [0.25, 0.75, 0.5]),
        edge(&body, [0.75, 0.25, 0.5], [0.75, 0.75, 0.5]),
    ];
    for verb in VERBS {
        sound_or_typed("three-edge chain", verb, &body, &es, None);
    }
}

/// **A bar crossing the side wall's fill and the floor's** (mixed: the
/// floor edge and the side/floor edge both requested, bar in the floor
/// band's fill only): absorbed or typed.
#[test]
fn probe_bar_with_the_opposite_floor_edge_also_filleted() {
    let (d, a) = BAR;
    let body = boolean(true, die(), rod(0.1, 0.9, (0.25 + d, 0.5 + d), a));
    let es = [
        floor(&body),
        edge(&body, [0.25, 0.75, 0.5], [0.75, 0.75, 0.5]),
    ];
    for verb in VERBS {
        let added = (2.0 * section(verb, D) - PI * a * a) * 0.5;
        sound_or_typed("two parallel floor edges", verb, &body, &es, Some(added));
    }
}
