//! JOIN-2 review lane r2: probes of the tangent-germ locus ranking and
//! of the zip's seam realization. Each probe prints one
//! `common::differential::outcome` line per (pose, order, op), so the
//! same binary built on main and on the PR head can be diffed. The
//! volume oracle is closed form from the outlines (shoelace plus the
//! bulge arcs' circular segments), independent of the kernel.
//!
//! Run: `cargo nextest run --release -p sweep -E 'test(/join2_r2/)'
//! --run-ignored all --no-capture`.

use super::common::differential::outcome;
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::RawLoop;
use sweep::test_support::{corners, extruded, prism_on, sketch_at, sketch_from_axes};
use topo::{Body, BooleanDeclarations, BooleanError, BooleanOp};

fn tol() -> Tol {
    Tol::witness()
}

/// The closed-form area of a bulge loop (CCW, positive bulge = an arc
/// bulging out of the loop).
fn bulge_area(v: &[((f64, f64), f64)]) -> f64 {
    let n = v.len();
    let mut s = 0.0;
    for i in 0..n {
        let ((ax, ay), b) = v[i];
        let ((bx, by), _) = v[(i + 1) % n];
        s += (ax * by - bx * ay) / 2.0;
        if b != 0.0 {
            let c = ((bx - ax).powi(2) + (by - ay).powi(2)).sqrt();
            let th = 4.0 * b.abs().atan();
            let r = c / (2.0 * (th / 2.0).sin());
            s += b.signum() * r * r / 2.0 * (th - th.sin());
        }
    }
    s
}

fn plate(v: &[((f64, f64), f64)], z0: f64) -> Body<f64> {
    let n = v.len();
    let verts = v
        .iter()
        .map(|&((x, y), b)| (Point2::new(x, y), b))
        .collect();
    // A fillet arc's two ends are declared-tangent joints.
    let mut joints: Vec<usize> = Vec::new();
    for (k, &(_, b)) in v.iter().enumerate() {
        if b != 0.0 {
            for j in [k, (k + 1) % n] {
                if !joints.contains(&j) {
                    joints.push(j);
                }
            }
        }
    }
    joints.sort_unstable();
    let lp = profile::test_support::bulge_loop(verts).with_tangent_joints(joints);
    extruded(sketch_at(z0), vec![lp], 1.0, tol())
}

fn sharp(v: &[(f64, f64)]) -> Vec<((f64, f64), f64)> {
    v.iter().map(|&p| (p, 0.0)).collect()
}

/// The flush detector's findings, every one declared (Rest and
/// continuations alike).
fn declared(a: &Body<f64>, b: &Body<f64>) -> Result<BooleanDeclarations, String> {
    let found = topo::flush::find_flush_candidates(a, b, tol()).map_err(|e| format!("{e:?}"))?;
    Ok(topo::flush::declare_all(&found))
}

/// Every op in both orders, one line each, plus the join's own refusal
/// for the union (to tell a zip-built body from a join-built one).
fn battery(label: &str, a: &Body<f64>, b: &Body<f64>, va: f64, vb: f64, disjoint: bool) {
    for (order, x, y, vx, vy) in [("ab", a, b, va, vb), ("ba", b, a, vb, va)] {
        let d = match declared(x, y) {
            Ok(d) => d,
            Err(e) => {
                println!("R2P {label} {order} DECL-ERR {e}");
                continue;
            }
        };
        let d2 = declared(y, x).unwrap_or_default();
        assert!(disjoint, "only stacked poses here");
        let join = topo::test_support::boolean_join_refusal(BooleanOp::Union, x, y, &d, tol());
        let path = match &join {
            Ok(None) => "join".to_string(),
            Ok(Some(e)) => format!("zip-or-refuse[{}]", short(e)),
            Err(e) => format!("red-err[{}]", short(e)),
        };
        println!(
            "R2P {label} {order} union {} | {path}",
            outcome(topo::union_with(x, y, &d, tol()), vx + vy, tol())
        );
        println!(
            "R2P {label} {order} sub {}",
            outcome(topo::subtract_with(x, y, &d, tol()), vx, tol())
        );
        println!(
            "R2P {label} {order} rsub {}",
            outcome(topo::subtract_with(y, x, &d2, tol()), vy, tol())
        );
        println!(
            "R2P {label} {order} meet {}",
            outcome(topo::intersect_with(x, y, &d, tol()), 0.0, tol())
        );
    }
}

fn short(e: &BooleanError) -> String {
    format!("{e:?}").chars().take(60).collect()
}

/// `tan(π/8)`: the bulge of a quarter-circle fillet.
fn q() -> f64 {
    (core::f64::consts::PI / 8.0).tan()
}

/// The W × H rect with its south-east corner rounded by `r` (convex).
fn rounded_se(w: f64, h: f64, r: f64) -> Vec<((f64, f64), f64)> {
    vec![
        ((0.0, 0.0), 0.0),
        ((w - r, 0.0), q()),
        ((w, r), 0.0),
        ((w, h), 0.0),
        ((0.0, h), 0.0),
    ]
}

/// **Claim 1, convex: a sharp rect over (and under) a rect rounded at
/// one corner**, the sharp one's east edge swept across the fillet:
/// short of it, on its tangent point, through the arc, on its far end,
/// past it. Radii toward 0 and toward the plate size.
#[test]
#[ignore = "review probe battery; prints lines"]
fn join2_r2_convex_fillet_sweep() {
    let (w, h) = (6.0, 4.0);
    for r in [0.02, 0.25, 1.0, 2.5, 3.9] {
        let lower = rounded_se(w, h, r);
        let vl = bulge_area(&lower);
        for f in [-0.5, 0.0, 0.3, 0.7, 1.0, 1.5] {
            let x1 = w - r + f * r;
            for (y0, y1) in [(0.0, h), (0.0, h - 0.5), (-0.5, h)] {
                let up = sharp(&[(0.0, y0), (x1, y0), (x1, y1), (0.0, y1)]);
                let vu = bulge_area(&up);
                for (stack, lo, hi, vlo, vhi) in [
                    ("rounded-below", &lower, &up, vl, vu),
                    ("rounded-above", &up, &lower, vu, vl),
                ] {
                    let (a, b) = (plate(lo, 0.0), plate(hi, 1.0));
                    battery(
                        &format!("convex r={r} f={f} y=({y0},{y1}) {stack}"),
                        &a,
                        &b,
                        vlo,
                        vhi,
                        true,
                    );
                }
            }
        }
    }
}

/// The 6 × 6 L less its 3 × 3 north-east notch, the notch's concave
/// corner rounded by `r`.
fn ell_concave(r: f64) -> Vec<((f64, f64), f64)> {
    vec![
        ((0.0, 0.0), 0.0),
        ((6.0, 0.0), 0.0),
        ((6.0, 3.0), 0.0),
        ((3.0 + r, 3.0), -q()),
        ((3.0, 3.0 + r), 0.0),
        ((3.0, 6.0), 0.0),
        ((0.0, 6.0), 0.0),
    ]
}

/// **Claim 1, concave**: a sharp L whose notch corner sits at `(c, 3)`
/// over (and under) the L whose notch corner is filleted.
#[test]
#[ignore = "review probe battery; prints lines"]
fn join2_r2_concave_fillet_sweep() {
    for r in [0.02, 0.5, 1.0, 2.0, 2.9] {
        let rounded = ell_concave(r);
        let vr = bulge_area(&rounded);
        for c in [2.5, 3.0, 3.0 + 0.5 * r, 3.0 + r, 3.0 + r + 0.25] {
            if c >= 6.0 {
                continue;
            }
            let up = sharp(&[
                (0.0, 0.0),
                (6.0, 0.0),
                (6.0, 3.0),
                (c, 3.0),
                (c, 6.0),
                (0.0, 6.0),
            ]);
            let vu = bulge_area(&up);
            for (stack, lo, hi, vlo, vhi) in [
                ("rounded-below", &rounded, &up, vr, vu),
                ("rounded-above", &up, &rounded, vu, vr),
            ] {
                let (a, b) = (plate(lo, 0.0), plate(hi, 1.0));
                battery(
                    &format!("concave r={r} c={c} {stack}"),
                    &a,
                    &b,
                    vlo,
                    vhi,
                    true,
                );
            }
        }
    }
}

/// **Claim 1, the residue reached.** The lower plate's convex fillet
/// (r = 1 at (3, 0)) is followed by a notch open to the south, so the
/// upper plate's straight bottom edge, tangent to the fillet at
/// (2, 0), re-meets the lower one at the notch's far corner (4, 0)
/// (a vertex pair: `Boundary`); the upper plate's own notch, open to
/// the north, has its floor y = d cross the arc (`Boundary`). Both far
/// ends are recorded alike, so both germs stay `OnEdge`. Must refuse,
/// or build right.
#[test]
#[ignore = "review probe battery; prints lines"]
fn join2_r2_like_far_end_touches() {
    for d in [0.3, 0.5, 0.8] {
        for (n0, n1) in [(2.5, 3.5), (2.2, 3.5), (2.5, 4.5)] {
            let lower = vec![
                ((0.0, 0.0), 0.0),
                ((2.0, 0.0), q()),
                ((3.0, 1.0), 0.0),
                ((3.0, 3.0), 0.0),
                ((4.0, 3.0), 0.0),
                ((4.0, 0.0), 0.0),
                ((6.0, 0.0), 0.0),
                ((6.0, 4.0), 0.0),
                ((0.0, 4.0), 0.0),
            ];
            let upper = sharp(&[
                (0.0, 0.0),
                (6.0, 0.0),
                (6.0, 4.0),
                (n1, 4.0),
                (n1, d),
                (n0, d),
                (n0, 4.0),
                (0.0, 4.0),
            ]);
            let (vl, vu) = (bulge_area(&lower), bulge_area(&upper));
            for (stack, lo, hi, vlo, vhi) in [
                ("rounded-below", &lower, &upper, vl, vu),
                ("rounded-above", &upper, &lower, vu, vl),
            ] {
                let (a, b) = (plate(lo, 0.0), plate(hi, 1.0));
                battery(
                    &format!("tie d={d} notch=({n0},{n1}) {stack}"),
                    &a,
                    &b,
                    vlo,
                    vhi,
                    true,
                );
            }
        }
    }
}

/// A U channel extruded along +y over `y ∈ [0, len]`: arms `[0, 1]`
/// and `[3, 4]` in x rising to z = 1 over a base `z ∈ [0, 0.5]`, so
/// its top is two faces.
fn channel(len: f64) -> (Body<f64>, f64) {
    let pts = [
        (0.0, 0.0),
        (4.0, 0.0),
        (4.0, 1.0),
        (3.0, 1.0),
        (3.0, 0.5),
        (1.0, 0.5),
        (1.0, 1.0),
        (0.0, 1.0),
    ];
    // u = +x, v = +z: the sketch normal is u × v = −y, so the profile
    // sits at y = len and extrudes back to y = 0.
    let plane = sketch_from_axes(
        Point3::new(0.0, len, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        tol(),
    );
    let body = prism_on(plane, corners(&pts), len, tol());
    (body, 3.0 * len)
}

/// **Claim 3: one face, two contact components.** A plate's bottom
/// face rests on both arms of the channel: one arm's top is an island
/// inside the face (four pierce-ring vertices, never joined to the
/// face's boundary), the other arm's top crosses the face's edge, so
/// its chords divide the face. `mef` leaves the island's loop on the
/// old face, whichever fragment it lies in. Each pose mirrors which arm
/// is the island.
#[test]
#[ignore = "review probe battery; prints lines"]
fn join2_r2_island_and_crossing_in_one_face() {
    let (ch, vch) = channel(4.0);
    for (name, x0, x1, y0, y1) in [
        ("island-west", -1.0, 3.5, -1.0, 5.0),
        ("island-east", 0.5, 5.0, -1.0, 5.0),
        ("island-west-wide", -1.0, 3.5, -1.0, 6.0),
        ("both-cross", -1.0, 3.5, 1.0, 5.0),
    ] {
        let p = sharp(&[(x0, y0), (x1, y0), (x1, y1), (x0, y1)]);
        let vp = bulge_area(&p);
        let plate = plate(&p, 1.0);
        battery(&format!("channel {name}"), &ch, &plate, vch, vp, true);
    }
}

/// **Claim 3 through the zip**: the island-and-crossing face of
/// [`join2_r2_island_and_crossing_in_one_face`], with the plate's
/// north-east corner rounded so its fillet is tangent to the crossing
/// arm's end edge mid-span (the tangency the join's surgery refuses, so
/// the declared-REST zip builds the union). The island arm lies inside
/// the same bottom face.
#[test]
#[ignore = "review probe battery; prints lines"]
fn join2_r2_island_through_the_zip() {
    let (ch, vch) = channel(4.0);
    for (name, r, xe, step) in [
        ("r=1 xe=4.5", 1.0, 4.5, 2.0),
        ("r=0.75 xe=4.25", 0.75, 4.25, 2.0),
        ("r=1 xe=4.5 step=2.5", 1.0, 4.5, 2.5),
        ("r=0.5 xe=4.2", 0.5, 4.2, 2.0),
    ] {
        let p = vec![
            ((-1.0, -1.0), 0.0),
            ((xe, -1.0), 0.0),
            ((xe, 4.0 - r), q()),
            ((xe - r, 4.0), 0.0),
            ((step, 4.0), 0.0),
            ((step, 5.0), 0.0),
            ((-1.0, 5.0), 0.0),
        ];
        let vp = bulge_area(&p);
        let up = plate(&p, 1.0);
        battery(&format!("zip-island {name}"), &ch, &up, vch, vp, true);
        // Mirrored in x about 2 (the island arm becomes the east one).
        let m: Vec<((f64, f64), f64)> = p
            .iter()
            .rev()
            .enumerate()
            .map(|(i, _)| {
                // reversed order keeps CCW after the mirror; bulges move
                // to the preceding vertex.
                let n = p.len();
                let k = n - 1 - i;
                let ((x, y), _) = p[k];
                let b = p[(k + n - 1) % n].1;
                ((4.0 - x, y), b)
            })
            .collect();
        let vm = bulge_area(&m);
        let mplate = plate(&m, 1.0);
        battery(
            &format!("zip-island-mirror {name}"),
            &ch,
            &mplate,
            vch,
            vm,
            true,
        );
    }
}

/// **Claim 3, the ring-first order**: both arms cross the plate's
/// north edge (no island), the crossing arm's corner tangent to the
/// plate's fillet. Each arm's south end is a span between two
/// pierce-ring vertices, which a chord can join only after one of them
/// is on the face's boundary.
#[test]
#[ignore = "review probe battery; prints lines"]
fn join2_r2_ring_order_through_the_zip() {
    let (ch, vch) = channel(4.0);
    for (name, r, xe, ys) in [
        ("r=1 xe=4.5", 1.0, 4.5, -1.0),
        ("r=0.5 xe=4.2", 0.5, 4.2, -1.0),
        ("r=1 xe=4.5 south-flush", 1.0, 4.5, 0.0),
    ] {
        let p = vec![
            ((-1.0, ys), 0.0),
            ((xe, ys), 0.0),
            ((xe, 4.0 - r), q()),
            ((xe - r, 4.0), 0.0),
            ((-1.0, 4.0), 0.0),
        ];
        let vp = bulge_area(&p);
        let up = plate(&p, 1.0);
        battery(&format!("ring-order {name}"), &ch, &up, vch, vp, true);
    }
}

/// One union that must build sound, its join's matching having paired
/// every germ (the join's surgery may still refuse, and the zip build).
fn builds_paired(label: &str, x: &Body<f64>, y: &Body<f64>, want: f64) {
    let d = declared(x, y);
    assert!(d.is_ok(), "{label}: declarations: {d:?}");
    let d = d.unwrap_or_default();
    let join = topo::test_support::boolean_join_refusal(BooleanOp::Union, x, y, &d, tol());
    assert!(
        !matches!(
            join,
            Ok(Some(BooleanError::Join(
                topo::SplitJoinError::UnpairedLooseEnds { .. }
            )))
        ),
        "{label}: the join pairs every germ, got {join:?}"
    );
    let line = outcome(topo::union_with(x, y, &d, tol()), want, tol());
    assert!(line.starts_with("OK SOUND"), "{label}: {line}");
}

/// **Two tangent edges recorded alike are told apart by the partner
/// face's trim.** The upper plate's straight bottom edge leaves the
/// lower plate's fillet tangent at (2, 0) and ends on the notch corner
/// (4, 0); the arc ends where the upper's notch floor crosses it. Both
/// far ends are vertex pairs, and the arc is the edge inside the upper
/// plate's face. Each pose builds in both stackings and both orders.
#[test]
fn a_like_far_ends_tie_is_decided_by_the_partner_faces_trim() {
    let lower = vec![
        ((0.0, 0.0), 0.0),
        ((2.0, 0.0), q()),
        ((3.0, 1.0), 0.0),
        ((3.0, 3.0), 0.0),
        ((4.0, 3.0), 0.0),
        ((4.0, 0.0), 0.0),
        ((6.0, 0.0), 0.0),
        ((6.0, 4.0), 0.0),
        ((0.0, 4.0), 0.0),
    ];
    for (d, n0, n1) in [(0.3, 2.5, 3.5), (0.8, 2.5, 3.5)] {
        let upper = sharp(&[
            (0.0, 0.0),
            (6.0, 0.0),
            (6.0, 4.0),
            (n1, 4.0),
            (n1, d),
            (n0, d),
            (n0, 4.0),
            (0.0, 4.0),
        ]);
        let want = bulge_area(&lower) + bulge_area(&upper);
        for (stack, lo, hi) in [("below", &lower, &upper), ("above", &upper, &lower)] {
            let (a, b) = (plate(lo, 0.0), plate(hi, 1.0));
            let label = format!("tie d={d} rounded {stack}");
            builds_paired(&format!("{label} ab"), &a, &b, want);
            builds_paired(&format!("{label} ba"), &b, &a, want);
        }
    }
}

/// **Segments between pierce-ring vertices only refuse typed.** The
/// zip's pose of [`join2_r2_island_through_the_zip`]: the channel's west
/// arm top is an island inside the plate's bottom face, so every one of
/// its segments runs between two ring vertices, and no realized segment
/// ever reaches one. The zip refuses the island in both orders at its
/// own frontier.
#[test]
fn an_island_of_ring_vertex_segments_refuses_at_the_zip_frontier() {
    let (ch, _) = channel(4.0);
    let (r, xe, step) = (1.0, 4.5, 2.0);
    let p = vec![
        ((-1.0, -1.0), 0.0),
        ((xe, -1.0), 0.0),
        ((xe, 4.0 - r), q()),
        ((xe - r, 4.0), 0.0),
        ((step, 4.0), 0.0),
        ((step, 5.0), 0.0),
        ((-1.0, 5.0), 0.0),
    ];
    let up = plate(&p, 1.0);
    for (order, x, y) in [("ab", &ch, &up), ("ba", &up, &ch)] {
        let d = declared(x, y).unwrap_or_else(|e| panic!("{order}: declarations: {e}"));
        let got = topo::union_with(x, y, &d, tol());
        assert!(
            matches!(
                got,
                Err(BooleanError::RestZipUnsupported {
                    what: topo::RestZipFrontier::SegmentsBetweenIsolatedPierces
                })
            ),
            "{order}: {got:?}"
        );
    }
}

/// **A span between two pierce-ring vertices waits for one of them to
/// join the face's boundary.** Both channel arms cross the plate's
/// north edge, the plate's fillet tangent to one arm's end; each arm's
/// south end is a span between two ring vertices, realized after the
/// arm's spans from the boundary.
#[test]
fn a_channel_whose_arm_ends_join_two_ring_vertices_builds_sound() {
    let (ch, vch) = channel(4.0);
    let (r, xe) = (1.0, 4.5);
    let p = vec![
        ((-1.0, -1.0), 0.0),
        ((xe, -1.0), 0.0),
        ((xe, 4.0 - r), q()),
        ((xe - r, 4.0), 0.0),
        ((-1.0, 4.0), 0.0),
    ];
    let up = plate(&p, 1.0);
    let want = vch + bulge_area(&p);
    builds_paired("channel ab", &ch, &up, want);
    builds_paired("channel ba", &up, &ch, want);
}
