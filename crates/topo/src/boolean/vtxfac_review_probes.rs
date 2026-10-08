//! REVIEW PROBES (PR 4300), scratch branch only.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]
use super::*;

/// Whether chords (a, b) and (c, d) on circle positions cross.
fn cross(a: usize, b: usize, c: usize, d: usize) -> bool {
    let (a, b) = (a.min(b), a.max(b));
    let inside = |x: usize| a < x && x < b;
    inside(c) != inside(d)
}

fn permutations(items: Vec<usize>) -> Vec<Vec<usize>> {
    if items.len() <= 1 {
        return vec![items];
    }
    let mut out = Vec::new();
    for i in 0..items.len() {
        let mut rest = items.clone();
        let x = rest.remove(i);
        for mut p in permutations(rest) {
            p.insert(0, x);
            out.push(p);
        }
    }
    out
}

/// Every closed meander of order k (k runs above, link order
/// s0 e0 s1 e1 ...; above chords (s_i, e_i), below chords (e_i, s_{i+1})
/// non-crossing), germ 0 at clockwise position 0: `ring_corners` must
/// return Some exactly where each above chord's germs are cyclic
/// neighbours, the corners in cyclic pair order from run 0, each with
/// its first germ; and wherever it is Some the facing is uniform.
#[test]
fn review_ring_corners_on_every_meander_k_le_6() {
    for k in 2..=5usize {
        let m = 2 * k;
        let (mut total, mut star, mut mixed) = (0, 0, 0);
        for rest in permutations((1..m).collect()) {
            // order[p] = germ at clockwise position p
            let mut order = vec![0];
            order.extend(rest);
            let mut pos = vec![0; m];
            for (p, &g) in order.iter().enumerate() {
                pos[g] = p;
            }
            let above: Vec<_> = (0..k).map(|i| (pos[2 * i], pos[2 * i + 1])).collect();
            let below: Vec<_> = (0..k)
                .map(|i| (pos[2 * i + 1], pos[(2 * i + 2) % m]))
                .collect();
            let ok = |ch: &[(usize, usize)]| {
                (0..ch.len()).all(|i| {
                    (i + 1..ch.len()).all(|j| !cross(ch[i].0, ch[i].1, ch[j].0, ch[j].1))
                })
            };
            if !(ok(&above) && ok(&below)) {
                continue;
            }
            total += 1;
            let nb = |a: usize, b: usize| (a + 1) % m == b || (b + 1) % m == a;
            let is_star = above.iter().all(|&(a, b)| nb(a, b));
            let got = ring_corners(&order);
            assert_eq!(got.is_some(), is_star, "k={k} order={order:?}");
            if let Some(c) = got {
                star += 1;
                // expected: pairs in cyclic position order, first germ clockwise
                let mut want: Vec<(usize, usize, bool)> = (0..k)
                    .map(|i| {
                        let (a, b) = above[i];
                        let first_is_start = (a + 1) % m == b;
                        let p = if first_is_start { a } else { b };
                        (p, i, first_is_start)
                    })
                    .collect();
                want.sort();
                let r = want.iter().position(|w| w.1 == 0).unwrap();
                want.rotate_left(r);
                let want: Vec<_> = want.iter().map(|&(_, i, f)| (i, f)).collect();
                assert_eq!(c, want, "k={k} order={order:?}");
                if c.iter().any(|x| x.1 != c[0].1) {
                    mixed += 1;
                }
            }
        }
        println!("k={k}: meanders {total}, star {star}, mixed facing {mixed}");
        assert_eq!(mixed, 0, "k={k}: a star with mixed facing");
    }
}

/// Germs on the unit circle in the z = 0 plane at `deg` clockwise about +z.
fn at(deg: f64) -> (Vec3<f64>, f64) {
    let t = -deg.to_radians();
    (Vec3::new(t.cos(), t.sin(), 0.0), 1.0)
}

/// A germ within the zero band of germ 0 (run 0's start) is never
/// compared with germ 0. Comb-shaped k = 2: run 0 wide (30 -> 30 - d),
/// run 1 narrow inside it (200 -> 100): truly [0, 3, 2, 1], every run
/// `Above`. With run 0's end a hair before its start, what does the
/// sort read?
#[test]
fn review_a_germ_in_band_of_germ_zero() {
    let band = Band::linear(Tol::witness()).unwrap();
    let n = Vec3::new(0.0, 0.0, 1.0);
    for d in [1.0, 1e-3, 1e-6, 1e-8, 1e-9, 1e-10, 1e-11, 1e-12, 1e-13] {
        let germs = [at(30.0), at(30.0 - d), at(200.0), at(100.0)];
        let r = ring_order(&germs, n, band);
        let c = r.as_ref().ok().map(|o| ring_corners(o));
        println!("d={d:e}: order {:?} corners {c:?}", r.map_err(|e| e.to_string()));
        // main's k = 2 facing read for run 0: from run 1's start
        let main = super::super::insert::strut_order(germs[2].0, n, (germs[0].0, germs[1].0), 1.0, band);
        println!("        main's run-0 facing read: {:?}", main.map_err(|e| e.to_string()));
    }
}

/// Side-by-side k = 2 (every run `Below`, truly [0, 1, 2, 3]) with run
/// 1's end a hair before run 0's start: an in-band tie at germ 0. A
/// star always exists at k = 2, so `None` here would refuse
/// `PierceRunsEnclose` for a geometry that cannot occur.
#[test]
fn review_side_by_side_tie_at_germ_zero() {
    let band = Band::linear(Tol::witness()).unwrap();
    let n = Vec3::new(0.0, 0.0, 1.0);
    for d in [1e-3, 1e-6, 1e-9, 1e-11] {
        let germs = [at(0.0), at(90.0), at(180.0), at(360.0 - d)];
        let r = ring_order(&germs, n, band).unwrap();
        println!("side-by-side d={d:e}: order {r:?} corners {:?}", ring_corners(&r));
    }
}
