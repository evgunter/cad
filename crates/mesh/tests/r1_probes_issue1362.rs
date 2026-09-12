//! R1 review probes for issue 1362 / PR 1389 (MESH-1).
//!
//! Independent re-derivation: nothing here calls `walk::loop_area` (it
//! is private, and a reviewer re-deriving the fold is the point). Both
//! spellings are written out locally, so the drift table is reproduced
//! against the reviewer's own arithmetic rather than against the
//! delivery's helper.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Vec3};
use profile::RawLoop;

const TAU: f64 = core::f64::consts::TAU;

/// The PRE-FIX spelling, verbatim from `walk.rs:1067` at the merge
/// base: fold anchored at the world origin.
fn area_origin(pts: &[Point3<f64>]) -> Vec3<f64> {
    let mut area = Vec3::new(0.0, 0.0, 0.0);
    for (i, p) in pts.iter().enumerate() {
        let q = pts[(i + 1) % pts.len()];
        area = area + (*p - Point3::origin()).cross(q - Point3::origin());
    }
    area
}

/// The POST-FIX spelling, re-derived: bbox-centre anchor.
fn area_local(pts: &[Point3<f64>]) -> Vec3<f64> {
    let Some(&first) = pts.first() else {
        return Vec3::new(0.0, 0.0, 0.0);
    };
    let (lo, hi) = pts
        .iter()
        .fold((first, first), |(lo, hi), &p| (lo.min(p), hi.max(p)));
    let o = lo + (hi - lo) * 0.5;
    let mut area = Vec3::new(0.0, 0.0, 0.0);
    for (i, p) in pts.iter().enumerate() {
        let q = pts[(i + 1) % pts.len()];
        area = area + (*p - o).cross(q - o);
    }
    area
}

/// A third, INDEPENDENT anchor: the loop's first point (the spelling
/// `planar::chart_frame` already uses). Over ℝ it must agree with both
/// of the above; in f64 it is a second local anchor, so agreement with
/// `area_local` is evidence the fix is not merely a different wrong
/// answer.
fn area_first(pts: &[Point3<f64>]) -> Vec3<f64> {
    let Some(&o) = pts.first() else {
        return Vec3::new(0.0, 0.0, 0.0);
    };
    let mut area = Vec3::new(0.0, 0.0, 0.0);
    for (i, p) in pts.iter().enumerate() {
        let q = pts[(i + 1) % pts.len()];
        area = area + (*p - o).cross(q - o);
    }
    area
}

/// `z_chart(Sphere)`'s frame, re-derived: axis `+z`, `u_ref = +x`,
/// `v_ref = axis × u_ref = +y`.
fn mid_az(area: Vec3<f64>) -> f64 {
    area.y.atan2(area.x)
}

/// `walk::unwrap_near`, verbatim.
fn unwrap_near(raw: f64, prev: f64) -> f64 {
    raw + TAU * ((prev - raw) / TAU).round()
}

/// The rimless pole-to-pole band's point cycle, re-derived from the
/// PR's stated geometry: south pole, up the meridian at `a0`, north
/// pole, down at `a1`.
fn band_cycle(r: f64, c: Point3<f64>, a0: f64, a1: f64, n: usize) -> Vec<Point3<f64>> {
    let on = |a: f64, t: f64| {
        Point3::new(
            c.x + r * a.cos() * t.sin(),
            c.y + r * a.sin() * t.sin(),
            c.z + r * t.cos(),
        )
    };
    let step = core::f64::consts::PI / (n as f64);
    let mut pts = vec![Point3::new(c.x, c.y, c.z - r)];
    pts.extend((1..n).map(|k| on(a0, core::f64::consts::PI - (k as f64) * step)));
    pts.push(Point3::new(c.x, c.y, c.z + r));
    pts.extend((1..n).map(|k| on(a1, (k as f64) * step)));
    pts
}

fn placed_band(d: f64) -> Vec<Point3<f64>> {
    band_cycle(
        1.3e-3,
        Point3::new(1.3 * d, -2.7 * d, 0.9 * d),
        0.37,
        2.27,
        8,
    )
}

/// **Claim 2, reproduced.** The PR's direction-drift table, both
/// spellings measured in one run against the reviewer's own fold.
/// Printed so the digits can be compared column by column with the PR
/// body; asserted so the row can go red.
#[test]
fn r1_reproduces_the_direction_drift_table() {
    let ref_origin = mid_az(area_origin(&placed_band(0.0)));
    let ref_local = mid_az(area_local(&placed_band(0.0)));
    println!("  d        origin-anchored   loop-anchored    first-point      budget");
    for d in [1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8] {
        let pts = placed_band(d);
        let (o, l, f) = (
            (mid_az(area_origin(&pts)) - ref_origin).abs(),
            (mid_az(area_local(&pts)) - ref_local).abs(),
            (mid_az(area_first(&pts)) - ref_local).abs(),
        );
        let budget = 1e-11 * d;
        println!("{d:8e}  {o:15e}  {l:15e}  {f:15e}  {budget:e}");
        assert!(
            l < budget,
            "loop-anchored fold missed its own budget at d = {d:e}: {l:e}"
        );
        assert!(
            o > l,
            "origin-anchored fold was not worse at d = {d:e}: {o:e} vs {l:e}"
        );
    }
    // The headline digits the PR body quotes, independently taken.
    let d6 = (mid_az(area_origin(&placed_band(1e6))) - ref_origin).abs();
    assert!(
        (d6 - 3.392).abs() < 5e-3,
        "PR body's 1e6 origin-anchored drift 3.392 rad not reproduced: {d6:e}"
    );
    let d2 = (mid_az(area_origin(&placed_band(1e2))) - ref_origin).abs();
    assert!(
        (d2 - 1.570e-6).abs() < 1e-9,
        "PR body's 1e2 origin-anchored drift 1.570e-6 rad not reproduced: {d2:e}"
    );
}

/// **Claim 2's consequence, reproduced.** The whole-2π branch flip the
/// PR reports at `d = 1e6`: the column, not the azimuth.
#[test]
fn r1_reproduces_the_whole_two_pi_branch_flip() {
    let ref_origin = mid_az(area_origin(&placed_band(0.0)));
    let mut flips = Vec::new();
    for d in [1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8] {
        let far = mid_az(area_origin(&placed_band(d)));
        for u_raw in [0.37, 2.27] {
            let k = ((unwrap_near(u_raw, far) - unwrap_near(u_raw, ref_origin)) / TAU).round();
            if k != 0.0 {
                flips.push((d, u_raw, k));
            }
        }
    }
    println!("origin-anchored branch flips: {flips:?}");
    assert!(
        flips.iter().any(|&(d, _, k)| d == 1e6 && k.abs() == 1.0),
        "the PR's 1e6 whole-2π flip did not reproduce; flips seen: {flips:?}"
    );
    // And the fix has none of them, at any row.
    let ref_local = mid_az(area_local(&placed_band(0.0)));
    for d in [1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8] {
        let far = mid_az(area_local(&placed_band(d)));
        for u_raw in [0.37, 2.27] {
            let k = ((unwrap_near(u_raw, far) - unwrap_near(u_raw, ref_local)) / TAU).round();
            assert_eq!(k, 0.0, "loop-anchored fold flipped a branch at d = {d:e}");
        }
    }
}

/// **Claim 1's algebra, executed.** The mod-`n` wrap makes the fold
/// anchor-independent unconditionally — it is an identity in the index
/// set, not a premise about the geometry. So a cycle that is NOT a
/// geometrically sensible closed loop (a self-crossing scribble, a
/// repeated point, a two-point degenerate) must still telescope.
/// Checked against a high-precision reference: the same fold over
/// exactly representable small integer coordinates, where every
/// product is exact in f64 and the two anchors must agree BITWISE.
#[test]
fn r1_the_wrap_makes_the_fold_anchor_independent_for_any_point_list() {
    // Small integers: products fit in f64's exact range, and a
    // half-integer bbox centre keeps every subtraction exact too.
    let cases: Vec<Vec<Point3<f64>>> = vec![
        // a self-crossing scribble — not a simple loop at all
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(4.0, 4.0, 0.0),
            Point3::new(4.0, 0.0, 2.0),
            Point3::new(0.0, 4.0, 2.0),
        ],
        // a repeated point (zero-length edge)
        vec![
            Point3::new(2.0, 0.0, 0.0),
            Point3::new(2.0, 0.0, 0.0),
            Point3::new(0.0, 6.0, 0.0),
            Point3::new(0.0, 0.0, 6.0),
        ],
        // two points: a degenerate "cycle" traversed out and back
        vec![Point3::new(1.0, 2.0, 3.0), Point3::new(-3.0, 5.0, 7.0)],
        // one point
        vec![Point3::new(9.0, -4.0, 6.0)],
    ];
    for (i, pts) in cases.iter().enumerate() {
        let (o, l, f) = (area_origin(pts), area_local(pts), area_first(pts));
        assert_eq!(
            (o.x, o.y, o.z),
            (l.x, l.y, l.z),
            "case {i}: bbox-centre anchor moved the value on exact data"
        );
        assert_eq!(
            (o.x, o.y, o.z),
            (f.x, f.y, f.z),
            "case {i}: first-point anchor moved the value on exact data"
        );
    }
}

/// **Claim 1's second half, attacked by search.** "Conditioning only:
/// the direction for well-conditioned inputs is unchanged." The
/// consumer is `unwrap_near`, whose `round` is a knife edge when a
/// meridian sits exactly π from `mid_az` — there, an ulp of anchor
/// difference flips the COLUMN by 2π at any placement, near-origin
/// included. This row searches for such an input over ordinary band
/// geometry and reports how close the search got.
#[test]
fn r1_hunts_a_near_origin_input_where_the_anchor_flips_a_column() {
    // Deterministic LCG — no fuzz harness, so the digits are the same
    // on every machine.
    let mut s: u64 = 0x1362_1389;
    let mut next = || {
        s = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((s >> 11) as f64) / ((1u64 << 53) as f64)
    };
    let mut worst_margin = f64::INFINITY;
    let mut worst = (0.0, 0.0, 0.0);
    let mut disagreements = 0u32;
    for _ in 0..200_000 {
        let a0 = next() * TAU - core::f64::consts::PI;
        let width = next() * TAU; // full range of lune widths, 0..2π
        let a1 = a0 + width;
        let r = 10f64.powf(next() * 6.0 - 3.0); // 1 mm .. 1 km
        // ORDINARY placement: within a few body-radii of the origin.
        let c = Point3::new(
            (next() * 2.0 - 1.0) * 10.0 * r,
            (next() * 2.0 - 1.0) * 10.0 * r,
            (next() * 2.0 - 1.0) * 10.0 * r,
        );
        let n = 2 + (next() * 30.0) as usize;
        let pts = band_cycle(r, c, a0, a1, n);
        let (mo, ml) = (mid_az(area_origin(&pts)), mid_az(area_local(&pts)));
        for u_raw in [a0, a1] {
            // distance of (prev - raw)/TAU from a .5 tie, in [0, .5]
            let k = (ml - u_raw) / TAU;
            let margin = (k - k.round()).abs();
            if margin < worst_margin {
                worst_margin = margin;
                worst = (a0, a1, r);
            }
            if unwrap_near(u_raw, mo) != unwrap_near(u_raw, ml) {
                disagreements += 1;
                println!(
                    "COLUMN DISAGREEMENT at ordinary placement: a0 {a0} a1 {a1} r {r:e} \
                     c {c:?} n {n} -> origin {} local {}",
                    unwrap_near(u_raw, mo),
                    unwrap_near(u_raw, ml)
                );
            }
        }
    }
    println!(
        "200k ordinary-placement bands: {disagreements} column disagreements; \
         closest approach to the unwrap_near knife edge = {worst_margin:e} \
         (0.5 is the tie), worst geometry a0/a1/r = {worst:?}"
    );
    assert_eq!(
        disagreements, 0,
        "the anchor change flipped a column at an ordinary placement"
    );
}

/// **The reachability of the knife edge itself**, independent of the
/// anchor: an input CONSTRUCTED to sit on `unwrap_near`'s tie, to show
/// the hazard is real arithmetic and only the geometry keeps it away.
#[test]
fn r1_the_unwrap_near_tie_is_a_real_two_pi_cliff() {
    let pts = band_cycle(1.0, Point3::new(0.0, 0.0, 0.0), 0.0, 1.0, 8);
    let m = mid_az(area_local(&pts));
    // A raw azimuth exactly π from mid_az: the two branches are
    // equidistant, and `round`'s half-away-from-zero rule decides.
    let raw = m - core::f64::consts::PI;
    let a = unwrap_near(raw, m);
    let b = unwrap_near(raw, m - 1e-12);
    println!("tie column {a} vs one-ulp-perturbed {b} (delta {})", a - b);
    assert!(
        (a - b).abs() > 1.0,
        "expected a 2π cliff at the tie; got {a} vs {b}"
    );
}

// ---------------------------------------------------------------------
// END TO END through the public API: body → tessellate → signed volume,
// at a large placement. The PR argues an end-to-end row is not
// available because "the tight-ε bands refuse it at the door". That is
// a claim about the doors, so open them and look.
// ---------------------------------------------------------------------

/// A unit ball whose sketch plane sits at `(d, d, d)`: the two-band
/// sphere is exactly the `no_rim` pole-to-pole shape `loop_area`
/// serves, so this is the real consumer path.
fn placed_ball(d: f64) -> Result<topo::Body<f64>, String> {
    let plane = profile::SketchPlane::from_frame(
        geom_core::Point3::new(d, d, d),
        geom_core::Vec3::new(1.0, 0.0, 0.0),
        geom_core::Vec3::new(0.0, 1.0, 0.0),
    );
    let lp = profile::ProfileLoop::new(vec![
        profile::ProfileVertex::new(geom_core::Point2::new(0.0, -1.0), 1.0),
        profile::ProfileVertex::new(geom_core::Point2::new(0.0, 1.0), 0.0),
    ]);
    let vp = profile::Profile::new(plane, vec![lp])
        .validate(geom_core::Tol::witness())
        .map_err(|e| format!("profile validate: {e:?}"))?;
    sweep::revolve(
        &vp,
        sweep::RevolveAxis {
            origin: geom_core::Point2::new(0.0, 0.0),
            dir: geom_core::Vec2::new(0.0, 1.0),
        },
        sweep::Revolution::Full,
        geom_core::Tol::witness(),
    )
    .map(|r| r.body)
    .map_err(|e| format!("revolve: {e:?}"))
}

/// **The end-to-end path, at seven placements.** Each row either
/// tessellates — and then must be watertight and measure 4π/3 — or
/// refuses TYPED, which is printed. A silently wrong mesh at any
/// placement is the finding; a refusal is the PR's stated wall and is
/// recorded as such rather than assumed.
#[test]
fn r1_the_ball_tessellates_honestly_at_every_placement_the_doors_admit() {
    use mesh::validate::{check_mesh, signed_volume};
    let exact = 4.0 * core::f64::consts::PI / 3.0;
    let mut admitted: Vec<(f64, f64)> = Vec::new();
    let mut refused = Vec::new();
    for d in [0.0, 1e2, 1e4, 1e6, 1e8, 1e10] {
        let body = match placed_ball(d) {
            Ok(b) => b,
            Err(e) => {
                println!("d = {d:e}: typed refusal upstream — {e}");
                refused.push(d);
                continue;
            }
        };
        let m = match mesh::tessellate(&body, 0.02, geom_core::Tol::witness()) {
            Ok(m) => m,
            Err(e) => {
                println!("d = {d:e}: typed refusal at tessellate — {e:?}");
                refused.push(d);
                continue;
            }
        };
        let watertight = check_mesh(&m);
        let v = signed_volume(&m);
        let tris: usize = m.patches.iter().map(|p| p.triangles.len()).sum();
        println!(
            "d = {d:e}: {tris} triangles, watertight {watertight:?}, volume {v} \
             (chordal deficit vs 4π/3 = {:e})",
            ((v - exact) / exact).abs()
        );
        assert_eq!(watertight, Ok(()), "d = {d:e}: mesh not watertight");
        admitted.push((d, v));
    }
    println!("admitted placements {admitted:?}; typed-refused {refused:?}");
    let (_, v0) = *admitted.first().expect("the origin-placed ball must build");
    // The oracle: the SAME body meshed at a distance must measure the
    // same volume. A mis-picked meridian branch meshes the
    // complementary half and moves this by O(1), not by ulps.
    for &(d, v) in &admitted {
        let rel = ((v - v0) / v0).abs();
        println!("  placement {d:e}: volume drift vs origin = {rel:e}");
        assert!(
            rel < 1e-3,
            "d = {d:e}: volume {v} vs {v0} at the origin (relative {rel:e}) — \
             a branch pick, not a chord budget"
        );
    }
}

/// A unit-scaled ball of radius `r` on a sketch plane at `(d, d, d)`.
fn placed_ball_r(d: f64, r: f64) -> Result<topo::Body<f64>, String> {
    let plane = profile::SketchPlane::from_frame(
        geom_core::Point3::new(d, d, d),
        geom_core::Vec3::new(1.0, 0.0, 0.0),
        geom_core::Vec3::new(0.0, 1.0, 0.0),
    );
    let lp = profile::ProfileLoop::new(vec![
        profile::ProfileVertex::new(geom_core::Point2::new(0.0, -r), 1.0),
        profile::ProfileVertex::new(geom_core::Point2::new(0.0, r), 0.0),
    ]);
    let vp = profile::Profile::new(plane, vec![lp])
        .validate(geom_core::Tol::witness())
        .map_err(|e| format!("profile validate: {e:?}"))?;
    sweep::revolve(
        &vp,
        sweep::RevolveAxis {
            origin: geom_core::Point2::new(0.0, 0.0),
            dir: geom_core::Vec2::new(0.0, 1.0),
        },
        sweep::Revolution::Full,
        geom_core::Tol::witness(),
    )
    .map(|r| r.body)
    .map_err(|e| format!("revolve: {e:?}"))
}

/// **Is the PR's "no end-to-end row is available" argument right?**
///
/// The PR says the fold's failure needs `D/r ≳ 6.7e7` and that driving
/// that through `revolve` + `tessellate` "would put the fixture where
/// the tight-ε bands refuse it at the door". But the DOOR is an
/// absolute test (coordinate ulp against ε) while the DEFECT is a
/// ratio test (`D/r`). A SMALL body at a moderate distance satisfies
/// both: `r = 1e-4` at `d = 1e4` has `ulp(1e4) ≈ 1.8e-12 ≪ ε` yet
/// `D/r ≈ 1.7e8`. This row walks that corner and reports what happens.
#[test]
fn r1_looks_for_an_end_to_end_fixture_that_clears_the_door_and_fails_the_fold() {
    use mesh::validate::{check_mesh, signed_volume};
    for (r, d) in [
        (1e-2, 1e4),
        (1e-3, 1e4),
        (1e-4, 1e4),
        (1e-4, 1e5),
        (1e-5, 1e4),
        (1e-6, 1e3),
    ] {
        let exact = 4.0 * core::f64::consts::PI * r * r * r / 3.0;
        let ratio = (d * 3f64.sqrt()) / r;
        let build = |dd: f64| -> Result<f64, String> {
            let body = placed_ball_r(dd, r)?;
            let m = mesh::tessellate(&body, r / 20.0, geom_core::Tol::witness())
                .map_err(|e| format!("tessellate: {e:?}"))?;
            if check_mesh(&m) != Ok(()) {
                return Err(format!("not watertight: {:?}", check_mesh(&m)));
            }
            Ok(signed_volume(&m))
        };
        match (build(0.0), build(d)) {
            (Ok(v0), Ok(v)) => {
                let rel = ((v - v0) / v0).abs();
                println!(
                    "r {r:e} d {d:e} (D/r ≈ {ratio:e}): ADMITTED — origin {v0:e}, \
                     placed {v:e}, drift {rel:e}, exact {exact:e}"
                );
                assert!(
                    rel < 1e-3,
                    "r {r:e} d {d:e}: the placed mesh's volume moved by {rel:e} — \
                     an END-TO-END manifestation of the branch pick"
                );
            }
            (_, Err(e)) => println!("r {r:e} d {d:e} (D/r ≈ {ratio:e}): refused — {e}"),
            (Err(e), _) => println!("r {r:e} d {d:e}: origin build refused — {e}"),
        }
    }
}
