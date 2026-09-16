//! Review probes for EDIT-PICK3 (lane `pick3-r1`, protocol v6 dual).
//!
//! Each row here falsifies, or fails to falsify, one claim of the unit
//! `pick-door-answers-a-t-interval` by execution.

use bvh::{Aabb, Bvh, Ray};
use editor_core::resolve::{TSpan, crossing, ray_triangle};
use geom_core::{Point3, Vec3};

/// The fixture family the early-out rows use: a triangle whose plane
/// the ray all but contains, entered EXACTLY at the corner the ray
/// crosses, so the box entry and the hit coincide and the whole of the
/// candidate's interval reaches back before its own box.
///
/// `zeta = 2^-20`, `xi = k*zeta*EPSILON`; `a = corner`,
/// `e1 = scale*(1, 0, zeta + xi)`, `e2 = scale*(0, 1, 0)`,
/// `d = (1, 1, zeta)`, the ray's origin `(-L, -L, -L*zeta)`. Then
/// `u = v = 0` exactly and the hit is the corner itself.
fn corner_tangent(k: f64, scale: f64, at: f64) -> [Point3<f64>; 3] {
    let zeta = 2f64.powi(-20);
    let xi = k * zeta * f64::EPSILON;
    let corner = Point3::new(at, at, at * zeta);
    [
        corner,
        corner + Vec3::new(scale, 0.0, scale * (zeta + xi)),
        corner + Vec3::new(0.0, scale, 0.0),
    ]
}

fn tree_over(tris: &[[Point3<f64>; 3]]) -> Bvh {
    let boxes: Vec<Aabb> = tris
        .iter()
        .map(|c| Aabb::from_points(*c).expect("three points box"))
        .collect();
    Bvh::build(&boxes)
}

/// The door's certified order over a set of spans: drop every span
/// some other span PRECEDES, then narrowest, then position. A verbatim
/// restatement of `pick_face`'s rule, used only to say what the door
/// WOULD answer had the traversal offered it every candidate.
fn winner(spans: &[TSpan]) -> Option<usize> {
    let lowest_hi = spans.iter().map(|s| s.t_hi).fold(f64::INFINITY, f64::min);
    (0..spans.len())
        .filter(|&i| spans[i].t_lo <= lowest_hi)
        .reduce(|b, i| if spans[i].width() < spans[b].width() { i } else { b })
}

/// **The early-out can prune a candidate of the certified tie, and
/// that candidate can be the one the door's own order picks.**
///
/// `pick_face`'s contract says the early-out costs only a near-tie "at
/// ULPs". It does not: the break fires on `t_hi(best) < t_enter(cand)`,
/// while membership of the certified tie is `t_lo(cand) <= t_hi(best)`.
/// A candidate whose interval is wide enough to reach back below the
/// running bound, but whose BOX is entered after that bound, is in the
/// tie by the door's own rule and is never tested. Here the pruned
/// candidate is also the NARROWER of the two, so the tie-break would
/// have taken it — the two answers are different faces, not different
/// ULPs.
#[test]
fn the_early_out_prunes_a_candidate_of_the_certified_tie() {
    let zeta = 2f64.powi(-20);
    let dir = Vec3::new(1.0, 1.0, zeta);
    // A wide candidate at t = 4, and a narrower one whose box the ray
    // does not enter until t = 4.5 — past the wide one's upper end,
    // but not past the narrow one's own lower end.
    let tris = [
        corner_tangent(154.0, 1.0, 4.0),
        corner_tangent(238.0, 1.0, 4.5),
    ];
    let origin = Point3::new(0.0, 0.0, 0.0);
    let ray = Ray { origin, dir };
    let spans: Vec<TSpan> = tris
        .iter()
        .enumerate()
        .map(|(i, t)| {
            ray_triangle(&ray, t)
                .unwrap_or_else(|| panic!("candidate {i} admitted? {:?}", crossing(&ray, t)))
        })
        .collect();
    eprintln!("wide   {:?} width {}", spans[0], spans[0].width());
    eprintln!("narrow {:?} width {}", spans[1], spans[1].width());
    let tree = tree_over(&tris);
    let cands: Vec<(usize, f64)> = tree
        .ray(&ray)
        .into_iter()
        .map(|c| (c.item, c.t_enter))
        .collect();
    eprintln!("candidates (item, t_enter) = {cands:?}");
    assert_eq!(cands[0].0, 0, "the tree offers the wide candidate first");
    let entry1 = cands
        .iter()
        .find(|c| c.0 == 1)
        .expect("the narrow candidate is a candidate")
        .1;
    assert!(
        spans[0].t_hi < entry1,
        "the premise: the door's early-out fires ({} < {entry1})",
        spans[0].t_hi
    );
    assert!(
        spans[1].t_lo <= spans[0].t_hi,
        "and yet the narrow candidate is IN the certified tie ({} <= {})",
        spans[1].t_lo,
        spans[0].t_hi
    );
    assert!(
        spans[1].width() < spans[0].width(),
        "and it is the narrower of the two, so the tie-break takes it"
    );
    assert_eq!(
        winner(&spans),
        Some(1),
        "the door's order over both candidates answers the narrow one"
    );
    // The traversal, exactly as `pick_face` runs it.
    let mut bound = f64::INFINITY;
    let mut walked: Vec<usize> = Vec::new();
    for cand in tree.ray(&ray) {
        if bound < cand.t_enter {
            break;
        }
        if let Some(span) = ray_triangle(&ray, &tris[cand.item]) {
            bound = bound.min(span.t_hi);
            walked.push(cand.item);
        }
    }
    assert_eq!(
        walked,
        vec![0, 1],
        "RED: the early-out pruned a member of the certified tie; the door answers {:?} \
         where its own order answers Some(1)",
        walked.first()
    );
}

/// A deterministic 64-bit xorshift, so the search is replayable.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    /// A uniform integer in `[-half, half]`.
    fn int(&mut self, half: i64) -> f64 {
        (self.next() % (2 * half as u64 + 1)) as i64 as f64 - half as f64
    }
    fn vec(&mut self, half: i64) -> Vec3<f64> {
        Vec3::new(self.int(half), self.int(half), self.int(half))
    }
}

/// **`t_span`'s interval is an enclosure of the EXACT crossing's
/// parameter** — searched over crossings whose exact answer is known
/// by construction.
///
/// Every input is a dyadic rational small enough that `a + u*e1 +
/// v*e2` and `hit - t*d` are computed without a rounding, so the ray
/// `(o, d)` meets the closed triangle at exactly the barycentrics
/// `(u, v)` and exactly the parameter `t` that built it. The claim
/// under test is `pick.rs`'s: "`[t_lo, t_hi]` encloses the parameter
/// of the true crossing whenever that crossing is a point of the
/// closed triangle, taking the corners and the ray as exact". The
/// direction `d` is drawn as `alpha*e1 + beta*e2 + w` so a large
/// share of the draws are near-tangent, which is where the bound is
/// load-bearing; `spread` puts the triangle far from the origin
/// against a small triangle.
#[test]
fn the_interval_encloses_an_exactly_known_crossing() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    let mut admitted = 0u64;
    let mut worst = 0.0f64;
    let mut worst_tight = 0.0f64;
    let mut escapes: Vec<String> = Vec::new();
    for case in 0..400_000u64 {
        let spread = if case % 3 == 0 { 1 << 18 } else { 8 };
        let a = Point3::new(0.0, 0.0, 0.0) + rng.vec(spread);
        let e1 = rng.vec(64);
        let e2 = rng.vec(64);
        let alpha = rng.int(32);
        let beta = rng.int(32);
        let d = e1 * alpha + e2 * beta + rng.vec(2);
        // Barycentrics on a 1/1024 lattice with u + v <= 1, and a
        // parameter on the same lattice: every product below is exact.
        let u = (rng.next() % 1025) as f64 / 1024.0;
        let v = (rng.next() % (1025 - (u * 1024.0) as u64)) as f64 / 1024.0;
        let t = ((rng.next() % 1024) as f64 + 1.0) / 1024.0 * 8.0;
        let tri = [a, a + e1, a + e2];
        let hit = a + e1 * u + e2 * v;
        let origin = hit - d * t;
        // The construction is only a witness if it was exact.
        let back = origin + d * t - hit;
        if back.x != 0.0 || back.y != 0.0 || back.z != 0.0 {
            continue;
        }
        let ray = Ray { origin, dir: d };
        let Some(span) = ray_triangle(&ray, &tri) else {
            continue;
        };
        admitted += 1;
        let half = 0.5 * span.width();
        if half > 0.0 {
            worst_tight = worst_tight.max((t - span.t).abs() / half);
        }
        if !(span.t_lo <= t && t <= span.t_hi) {
            let miss = (t - span.t_hi).max(span.t_lo - t);
            worst = worst.max(miss / span.width().max(f64::MIN_POSITIVE));
            if escapes.len() < 8 {
                escapes.push(format!(
                    "case {case}: exact t = {t}, span {span:?}, miss {miss:e}, \
                     tri {tri:?}, ray o {origin:?} d {d:?}, u {u} v {v}"
                ));
            }
        }
    }
    eprintln!(
        "admitted {admitted} exactly-known crossings; worst escape/width {worst:e}; \
         the exact t reached {worst_tight:.6} of the certified half-width at its furthest"
    );
    assert!(
        escapes.is_empty(),
        "the certified interval missed the exact crossing on {} draws:\n{}",
        escapes.len(),
        escapes.join("\n")
    );
    assert!(admitted > 10_000, "the search admitted too few: {admitted}");
}

/// Prints admitted crossings as raw bit patterns, for the exact
/// rational check in `/home/user/pick3-r1-scratch/enclosure.py` (this
/// lane's script, not the tree's). **Nothing is asserted here that the
/// script asserts**: this row exists so the exact check reads the REAL
/// numbers the door answered rather than a second spelling of the door
/// in Python. A no-op unless `PICK3_R1_DUMP` is set, so it costs a CI
/// run nothing.
#[test]
fn dump_admitted_spans_for_the_exact_enclosure_check() {
    if std::env::var("PICK3_R1_DUMP").is_err() {
        return;
    }
    let hx = |x: f64| format!("{:016x}", x.to_bits());
    let mut rng = Rng(0xD1B5_4A32_D192_ED03);
    let mut lines = 0u64;
    // (a) random f64 triangles and rays over a spread of exponents,
    // the direction biased into the triangle's own plane; (b) the
    // near-tangent family swept down to the certification floor, where
    // the barycentric bound is the whole of the answer.
    for case in 0..4_000_000u64 {
        let (tri, ray) = if case % 2 == 0 {
            let scale = 2f64.powi((rng.next() % 41) as i32 - 20);
            let off = 2f64.powi((rng.next() % 41) as i32 - 20);
            let a = Point3::new(0.0, 0.0, 0.0) + rng.vec(1 << 20) * (off / 1048576.0);
            let e1 = rng.vec(1 << 20) * (scale / 1048576.0);
            let e2 = rng.vec(1 << 20) * (scale / 1048576.0);
            let alpha = rng.int(64) / 8.0;
            let beta = rng.int(64) / 8.0;
            let w = rng.vec(1 << 10) * (scale / 1048576.0);
            let d = e1 * alpha + e2 * beta + w;
            let u = (rng.next() % 1024) as f64 / 2048.0;
            let v = (rng.next() % 1024) as f64 / 2048.0;
            let hit = a + e1 * u + e2 * v;
            let t = (rng.next() % 4096) as f64 / 512.0 + 0.001;
            let origin = hit - d * t;
            ([a, a + e1, a + e2], Ray { origin, dir: d })
        } else {
            let zeta = 2f64.powi(-20);
            let k = 6.5 + (rng.next() % 200_000) as f64 / 64.0;
            let xi = k * zeta * f64::EPSILON;
            let tri = [
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(1.0, 0.0, zeta + xi),
                Point3::new(0.0, 1.0, 0.0),
            ];
            let shift = rng.int(4096) / 8192.0;
            let origin = Point3::new(-1.0 + shift, -1.0 - shift, 0.5 * xi - zeta);
            (
                tri,
                Ray {
                    origin,
                    dir: Vec3::new(1.0, 1.0, zeta),
                },
            )
        };
        let Some(span) = ray_triangle(&ray, &tri) else {
            continue;
        };
        let f: Vec<String> = [
            tri[0].x, tri[0].y, tri[0].z, tri[1].x, tri[1].y, tri[1].z, tri[2].x, tri[2].y,
            tri[2].z, ray.origin.x, ray.origin.y, ray.origin.z, ray.dir.x, ray.dir.y, ray.dir.z,
            span.t, span.t_lo, span.t_hi,
        ]
        .iter()
        .map(|&x| hx(x))
        .collect();
        println!("SPAN {}", f.join(" "));
        lines += 1;
        if lines >= 300_000 {
            break;
        }
    }
    eprintln!("dumped {lines} admitted crossings");
}
