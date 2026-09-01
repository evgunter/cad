//! R1 review probe: the stage-2 completeness argument, exercised on a
//! VERBATIM port of `chart_region`'s private decomposition schedule
//! (`NominalSeg` / `event_abscissae` / `midpoint` / the slab-midline
//! loop, copied from the frozen review head). The port is pure `f64`
//! and self-contained, so it is the same computation the rung runs;
//! what it lets this probe do is (a) drive it on polygon pairs that
//! would be expensive to build as real bodies, and (b) compare its
//! offered candidates against a brute-force ground truth.
//!
//! Property under test — the PR's completeness claim: if
//! `P = A ∩ B` has interior, some offered cell centre lies strictly
//! inside it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// ---------------------------------------------------------------
// PORT (verbatim from crates/topo/src/chart_region.rs @ 11947309b)
// ---------------------------------------------------------------

struct NominalSeg {
    p: [f64; 2],
    q: [f64; 2],
}

impl NominalSeg {
    fn at_abscissa(&self, x: f64) -> Option<f64> {
        let run = self.q[0] - self.p[0];
        if run == 0.0 {
            return None;
        }
        let t = (x - self.p[0]) / run;
        if !(t > 0.0 && t < 1.0) {
            return None;
        }
        let y = self.p[1] + t * (self.q[1] - self.p[1]);
        y.is_finite().then_some(y)
    }

    fn meeting_abscissa(&self, other: &Self) -> Option<f64> {
        let r = [self.q[0] - self.p[0], self.q[1] - self.p[1]];
        let s = [other.q[0] - other.p[0], other.q[1] - other.p[1]];
        let denom = r[0] * s[1] - r[1] * s[0];
        if denom == 0.0 || !denom.is_finite() {
            return None;
        }
        let d = [other.p[0] - self.p[0], other.p[1] - self.p[1]];
        let t = (d[0] * s[1] - d[1] * s[0]) / denom;
        let u = (d[0] * r[1] - d[1] * r[0]) / denom;
        if !(0.0..=1.0).contains(&t) || !(0.0..=1.0).contains(&u) {
            return None;
        }
        let x = self.p[0] + t * r[0];
        x.is_finite().then_some(x)
    }
}

fn event_abscissae(segments: &[NominalSeg]) -> Vec<f64> {
    let mut xs = Vec::with_capacity(2 * segments.len());
    for s in segments {
        xs.push(s.p[0]);
        xs.push(s.q[0]);
    }
    for i in 0..segments.len() {
        for j in (i + 1)..segments.len() {
            if let Some(x) = segments[i].meeting_abscissa(&segments[j]) {
                xs.push(x);
            }
        }
    }
    xs.sort_by(f64::total_cmp);
    xs.dedup();
    xs
}

fn midpoint(a: f64, b: f64) -> f64 {
    a + 0.5 * (b - a)
}

const SEGMENTS: usize = 128;
const CELLS: usize = 4096;

/// The stage-2 loop, verbatim, with the probe made a collector so the
/// whole offered candidate stream is visible.
fn offered(segments: &[NominalSeg]) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    if segments.len() < 2 || segments.len() > SEGMENTS {
        return out;
    }
    let mut spent = 0usize;
    let abscissae = event_abscissae(segments);
    for slab in abscissae.windows(2) {
        let x = midpoint(slab[0], slab[1]);
        if !(x > slab[0] && x < slab[1]) {
            continue;
        }
        let mut ys: Vec<f64> = segments.iter().filter_map(|s| s.at_abscissa(x)).collect();
        ys.sort_by(f64::total_cmp);
        for cell in ys.windows(2) {
            let y = midpoint(cell[0], cell[1]);
            if !(y > cell[0] && y < cell[1]) {
                continue;
            }
            spent += 1;
            if spent > CELLS {
                return out;
            }
            out.push((x, y));
        }
    }
    out
}

// ---------------------------------------------------------------
// Harness
// ---------------------------------------------------------------

type Poly = Vec<[f64; 2]>;

fn segs(polys: &[&Poly]) -> Vec<NominalSeg> {
    let mut out = Vec::new();
    for poly in polys {
        if poly.len() < 3 {
            continue;
        }
        for i in 0..poly.len() {
            out.push(NominalSeg {
                p: poly[i],
                q: poly[(i + 1) % poly.len()],
            });
        }
    }
    out
}

/// Even-odd containment by a ray cast along +x, on the *exact* input
/// (ground truth; the rung's own containment is `contfp`'s).
fn inside(poly: &Poly, x: f64, y: f64) -> bool {
    let mut c = false;
    let n = poly.len();
    for i in 0..n {
        let (a, b) = (poly[i], poly[(i + 1) % n]);
        if (a[1] > y) != (b[1] > y) {
            let t = (y - a[1]) / (b[1] - a[1]);
            if a[0] + t * (b[0] - a[0]) > x {
                c = !c;
            }
        }
    }
    c
}

/// Brute-force: does `A ∩ B` plausibly have interior? Dense grid scan
/// over the shared bounding box.
fn overlap_seen(a: &Poly, b: &Poly, n: usize) -> Option<(f64, f64)> {
    let xs: Vec<f64> = a.iter().chain(b.iter()).map(|p| p[0]).collect();
    let ys: Vec<f64> = a.iter().chain(b.iter()).map(|p| p[1]).collect();
    let (x0, x1) = (
        xs.iter().cloned().fold(f64::INFINITY, f64::min),
        xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
    );
    let (y0, y1) = (
        ys.iter().cloned().fold(f64::INFINITY, f64::min),
        ys.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
    );
    for i in 0..n {
        for j in 0..n {
            let x = x0 + (x1 - x0) * (i as f64 + 0.5) / n as f64;
            let y = y0 + (y1 - y0) * (j as f64 + 0.5) / n as f64;
            if inside(a, x, y) && inside(b, x, y) {
                return Some((x, y));
            }
        }
    }
    None
}

fn check(name: &str, a: &Poly, b: &Poly) {
    let s = segs(&[a, b]);
    let cands = offered(&s);
    let hit = cands
        .iter()
        .any(|&(x, y)| inside(a, x, y) && inside(b, x, y));
    let truth = overlap_seen(a, b, 400);
    println!(
        "{name}: segs={} cands={} schedule_hit={hit} bruteforce={:?}",
        s.len(),
        cands.len(),
        truth.is_some()
    );
    if let Some(p) = truth {
        assert!(
            hit,
            "{name}: brute force found interior at {p:?} but NO offered cell centre \
             is strictly inside both ({} candidates offered)",
            cands.len()
        );
    }
}

fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> Poly {
    vec![[x0, y0], [x1, y0], [x1, y1], [x0, y1]]
}

/// Structured adversarial cases for the completeness argument.
#[test]
fn r1_stage2_completeness_edge_cases() {
    // Vertical edges everywhere (axis-aligned rectangles).
    check(
        "axis-aligned overlap",
        &rect(0., 0., 1., 1.),
        &rect(0.5, 0.5, 2., 2.),
    );
    // Shared full edge (the flush seat's shape).
    check("shared edge", &rect(0., 0., 1., 1.), &rect(0., 0., 1., 2.));
    // Shared vertical span, overlapping interiors.
    check(
        "shared span",
        &rect(0., 0., 1., 1.),
        &rect(0., 0.25, 1., 0.75),
    );
    // Repeated abscissae: many vertices at the same x.
    check(
        "repeated abscissae",
        &vec![[0., 0.], [0., 1.], [1., 1.], [1., 0.5], [1., 0.]],
        &vec![[0., 0.2], [0., 0.8], [1., 0.8], [1., 0.2]],
    );
    // A region much thinner than the slabs it lives in: one thin
    // horizontal sliver crossing a wide box.
    check(
        "thin horizontal sliver",
        &rect(0., 0., 10., 10.),
        &rect(-1., 4.999, 11., 5.001),
    );
    // A thin VERTICAL sliver: its width is smaller than every slab it
    // is not itself an endpoint of.
    check(
        "thin vertical sliver",
        &rect(0., 0., 10., 10.),
        &rect(4.999, -1., 5.001, 11.),
    );
    // Crossings exactly at slab boundaries (vertices of one polygon on
    // the abscissae of the other).
    check(
        "crossing at slab boundary",
        &vec![[0., 0.], [1., 0.], [1., 1.], [0., 1.]],
        &vec![[0.5, -1.], [1.5, 0.5], [0.5, 2.]],
    );
    // Collinear overlapping boundary segments, offset regions.
    check(
        "collinear overlap",
        &vec![[0., 0.], [2., 0.], [2., 1.], [0., 1.]],
        &vec![[0.5, 0.], [1.5, 0.], [1.5, -1.], [0.5, -1.]],
    ); // touching only: no interior overlap
    check(
        "collinear partial + interior",
        &vec![[0., 0.], [2., 0.], [2., 1.], [0., 1.]],
        &vec![[0.5, 0.], [1.5, 0.], [1.5, 0.5], [0.5, 0.5]],
    );
    // The overhang seat's own trims, in the shelf's chart (the thin
    // triangle H-A-B).
    let cap = vec![
        [0.20, 0.20],
        [0.40, 0.30],
        [0.60, 0.42],
        [0.70, 0.30],
        [0.80, 0.42],
        [0.85, 0.50],
        [0.15, 0.50],
        [0.25, 0.30],
    ];
    check("overhang seat trims", &cap, &rect(0.0, 0.0, 0.9, 0.30));
    // Non-convex against non-convex: two interlocking combs.
    let comb_a = vec![
        [0., 0.],
        [3., 0.],
        [3., 3.],
        [2.5, 3.],
        [2.5, 1.],
        [1.5, 1.],
        [1.5, 3.],
        [1.0, 3.],
        [1.0, 1.],
        [0., 1.],
    ];
    let comb_b = vec![
        [0.2, 0.5],
        [2.8, 0.5],
        [2.8, 2.5],
        [2.6, 2.5],
        [2.6, 0.7],
        [0.2, 0.7],
    ];
    check("interlocking combs", &comb_a, &comb_b);
}

/// Randomized sweep: star-shaped polygons at pseudo-random radii, many
/// pairs, checking the same property. Deterministic (fixed LCG seed).
#[test]
fn r1_stage2_completeness_randomized() {
    let mut seed = 0x2f6e_2b19_u64;
    let mut rnd = move || {
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((seed >> 33) as f64) / ((1u64 << 31) as f64)
    };
    let mut misses = 0;
    for case in 0..300 {
        let star = |cx: f64, cy: f64, n: usize, r: &mut dyn FnMut() -> f64| -> Poly {
            (0..n)
                .map(|i| {
                    let th = core::f64::consts::TAU * (i as f64) / (n as f64);
                    let rad = 0.3 + 0.9 * r();
                    [cx + rad * th.cos(), cy + rad * th.sin()]
                })
                .collect()
        };
        let n_a = 3 + (rnd() * 6.0) as usize;
        let n_b = 3 + (rnd() * 6.0) as usize;
        let a = star(0.0, 0.0, n_a, &mut rnd);
        let b = star(rnd() * 1.6 - 0.8, rnd() * 1.6 - 0.8, n_b, &mut rnd);
        let s = segs(&[&a, &b]);
        let cands = offered(&s);
        let hit = cands
            .iter()
            .any(|&(x, y)| inside(&a, x, y) && inside(&b, x, y));
        if let Some(p) = overlap_seen(&a, &b, 200) {
            if !hit {
                misses += 1;
                println!(
                    "case {case}: MISS — interior at {p:?}, {} candidates",
                    cands.len()
                );
            }
        }
    }
    assert_eq!(misses, 0, "the schedule missed a decidable overlap");
}

/// The budget guard, at the algorithm level: at 129 segments the
/// schedule offers NOTHING — no partial search, no narrowed one. The
/// same pair at 128 segments offers a full arrangement. (A body-level
/// fixture for this could not be built cheaply: subdividing the seat's
/// cap to >124 edges trips `ScaffoldAtRest` long before the rung, so
/// the guard has no integration-level exercise anywhere in the unit.)
#[test]
fn r1_the_segment_budget_is_all_or_nothing() {
    // A 64-gon against a 64-gon: 128 segments, at the limit.
    let ring = |n: usize, r: f64, cx: f64| -> Poly {
        (0..n)
            .map(|i| {
                let th = core::f64::consts::TAU * (i as f64 + 0.13) / (n as f64);
                [cx + r * th.cos(), r * th.sin()]
            })
            .collect()
    };
    let a64 = ring(64, 1.0, 0.0);
    let b64 = ring(64, 1.0, 0.5);
    let at_limit = offered(&segs(&[&a64, &b64]));
    let a65 = ring(65, 1.0, 0.0);
    let over = offered(&segs(&[&a65, &b64]));
    println!(
        "128 segments -> {} candidates; 129 segments -> {} candidates",
        at_limit.len(),
        over.len()
    );
    assert!(at_limit.len() > 100, "the in-budget pair is searched");
    assert_eq!(
        over.len(),
        0,
        "one segment over the budget searches nothing"
    );
}

/// Holes: `boundary_segments` feeds rings in too, and the doc claims
/// that is what stops two genuine cells being merged across a hole's
/// edge. Ground truth here is even-odd over outer XOR ring.
#[test]
fn r1_stage2_completeness_with_holes() {
    let in_region =
        |outer: &Poly, ring: &Poly, x: f64, y: f64| inside(outer, x, y) && !inside(ring, x, y);
    let outer_a = rect(0., 0., 4., 4.);
    let ring_a = rect(1., 1., 3., 3.);
    let outer_b = rect(0.5, 0.5, 3.5, 3.5);
    let ring_b = rect(1.5, 1.5, 2.5, 2.5);
    let s = segs(&[&outer_a, &ring_a, &outer_b, &ring_b]);
    let cands = offered(&s);
    let hit = cands
        .iter()
        .any(|&(x, y)| in_region(&outer_a, &ring_a, x, y) && in_region(&outer_b, &ring_b, x, y));
    println!(
        "holed pair: segs={} cands={} hit={hit}",
        s.len(),
        cands.len()
    );
    assert!(
        hit,
        "annulus ∩ annulus has interior, no cell centre found it"
    );
    // The hole's own edges must be in X: a cell centre must exist in
    // the corridor between ring_a and ring_b (width 0.5) — check that
    // at least one candidate lands there.
    let corridor = cands
        .iter()
        .any(|&(x, y)| inside(&ring_a, x, y) && !inside(&ring_b, x, y));
    println!("  corridor between the two holes sampled: {corridor}");
}

/// Near-parallel crossings, where `meeting_abscissa`'s `f64` divide is
/// worst-conditioned: does the arrangement still cover the overlap?
#[test]
fn r1_stage2_near_parallel_stress() {
    let mut seed = 0x9e37_79b9_u64;
    let mut rnd = move || {
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((seed >> 33) as f64) / ((1u64 << 31) as f64)
    };
    let mut misses = 0;
    for case in 0..600 {
        // Two long thin near-parallel quads crossing at a tiny angle.
        let eps = 10f64.powf(-3.0 - 6.0 * rnd());
        let w = 10f64.powf(-1.0 - 3.0 * rnd());
        let a = vec![[-5., -w], [5., -w], [5., w], [-5., w]];
        let dy = (rnd() - 0.5) * 4.0 * w;
        let b = vec![
            [-5., dy - w - 5.0 * eps],
            [5., dy - w + 5.0 * eps],
            [5., dy + w + 5.0 * eps],
            [-5., dy + w - 5.0 * eps],
        ];
        let s = segs(&[&a, &b]);
        let cands = offered(&s);
        let hit = cands
            .iter()
            .any(|&(x, y)| inside(&a, x, y) && inside(&b, x, y));
        if let Some(p) = overlap_seen(&a, &b, 300) {
            if !hit {
                misses += 1;
                if misses < 6 {
                    println!(
                        "case {case} (eps={eps:.3e}, w={w:.3e}): MISS at {p:?}, {} candidates",
                        cands.len()
                    );
                }
            }
        }
    }
    println!("near-parallel misses: {misses}/600");
    assert_eq!(misses, 0, "near-parallel stress found a miss");
}
