//! **Random islands on a sphere sheet wind and re-home as a stereographic
//! oracle says** ([`super::path_island_winding`],
//! [`super::path_ring_side`]). Their runs often reach both sides of the
//! closing chord's plane, the shape the sphere's first reading refused.
//! Each row is a fixed draw of islands — its seed is a fixture
//! identifier, every draw at three frames and three scales (1, 1e-3,
//! 1e3) — not a counterexample search.

use super::*;
use crate::MevSite;
use crate::ring_path::cone_islands::{Frame, band, frames, plane, read_at};
use core::f64::consts::PI;

fn tol() -> Tol {
    Tol::witness()
}

#[derive(Clone, Copy)]
struct Sph {
    f: Frame,
    r: f64,
}

impl Sph {
    fn at(&self, az: f64, z: f64) -> Point3<f64> {
        let s = (1.0 - z * z).sqrt();
        self.f
            .at(self.r * s * az.cos(), self.r * s * az.sin(), self.r * z)
    }
    /// The point at angular distance `rho` from `+x`, bearing `theta`.
    fn polar(&self, rho: f64, theta: f64) -> Point3<f64> {
        self.f.at(
            self.r * rho.cos(),
            self.r * rho.sin() * theta.cos(),
            self.r * rho.sin() * theta.sin(),
        )
    }
    fn surface(&self) -> geom::Surface<f64> {
        geom::Surface::Sphere {
            center: self.f.o,
            radius: self.r,
            axis: self.f.z,
            u_ref: self.f.x,
        }
    }
    /// Stereographic image of `p` from the antipode of `+x`.
    fn proj(&self, p: Point3<f64>) -> (f64, f64) {
        let w = (p - self.f.o) / self.r;
        let (x, y, z) = (w.dot(self.f.x), w.dot(self.f.y), w.dot(self.f.z));
        (y / (1.0 + x), z / (1.0 + x))
    }
}

/// A circle arc of the sphere from `a` through `v` to `b`.
#[derive(Clone)]
struct Arc3 {
    carrier: geom::Curve3<f64>,
    end: f64,
    plane: (Point3<f64>, Vec3<f64>),
}

fn arc3(s: Sph, a: Point3<f64>, v: Point3<f64>, b: Point3<f64>) -> Arc3 {
    let n0 = (v - a).cross(b - a).normalize();
    let tau = 2.0 * PI;
    for n in [n0, -n0] {
        let center = s.f.o + n * n.dot(a - s.f.o);
        let radius = (a - center).norm();
        let u = (a - center) / radius;
        let par = |p: Point3<f64>| {
            let w = p - center;
            w.dot(n.cross(u)).atan2(w.dot(u)).rem_euclid(tau)
        };
        let (tv, tb) = (par(v), par(b));
        if tv < tb {
            return Arc3 {
                carrier: geom::Curve3::Circle {
                    center,
                    axis: n,
                    radius,
                    u_ref: u,
                },
                end: tb,
                plane: (center, n),
            };
        }
    }
    unreachable!()
}

fn spec(body: &mut Body<f64>, surface: SurfaceKey, a: &Arc3) -> EdgeCurveSpec<f64> {
    let s2 = body.add_surface(plane(a.plane.0, a.plane.1));
    EdgeCurveSpec {
        description: geom_brep::EdgeDescriptionSpec::Intersection {
            s1: surface,
            s2,
            witness: a.carrier.mid_point(0.0, a.end),
        },
        carrier: a.carrier.clone(),
        param_start: 0.0,
        param_end: a.end,
    }
}

/// A face of the sphere between the latitudes ±0.97 and the meridians
/// at azimuths ±2.9.
fn sheet(s: Sph, sense: bool) -> (Body<f64>, FaceKey, SurfaceKey) {
    let (lo, hi, t) = (-0.97, 0.97, 2.9);
    let mut body = Body::<f64>::new();
    let start = s.at(-t, lo);
    let seed = body.mvfs(start, sense).unwrap();
    let surface = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: s.surface(),
                sense,
            },
        )
        .unwrap();
    let e = spec(
        &mut body,
        surface,
        &arc3(s, start, s.at(0.0, lo), s.at(t, lo)),
    );
    let first = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            s.at(t, lo),
            e,
            tol(),
        )
        .unwrap();
    let e = spec(
        &mut body,
        surface,
        &arc3(s, s.at(t, lo), s.at(t, 0.0), s.at(t, hi)),
    );
    let up = body
        .mev(
            MevSite::Fan {
                he1: first.he_minus,
                he2: first.he_minus,
            },
            s.at(t, hi),
            e,
            tol(),
        )
        .unwrap();
    let e = spec(
        &mut body,
        surface,
        &arc3(s, s.at(t, hi), s.at(0.0, hi), s.at(-t, hi)),
    );
    let last = body
        .mev(
            MevSite::Fan {
                he1: up.he_minus,
                he2: up.he_minus,
            },
            s.at(-t, hi),
            e,
            tol(),
        )
        .unwrap();
    let e = spec(
        &mut body,
        surface,
        &arc3(s, start, s.at(-t, 0.0), s.at(-t, hi)),
    );
    let face = body
        .mef(
            MefSite::Chords {
                he1: first.he_plus,
                he2: last.he_minus,
            },
            e,
            FaceSurface::Shared {
                key: surface,
                sense,
            },
            tol(),
        )
        .unwrap()
        .face;
    (body, face, surface)
}

fn empty_ring(body: &mut Body<f64>, face: FaceKey, p: Point3<f64>) -> LoopKey {
    let outer = body.get_face(face).unwrap().outer;
    let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("outer is a cycle");
    };
    let u = body.half_edge_start_point(first).unwrap();
    let bridge = body
        .mev(
            MevSite::Fan {
                he1: first,
                he2: first,
            },
            p,
            EdgeCurveSpec::line_between(u, p),
            tol(),
        )
        .unwrap();
    body.kemr(bridge.he_plus, bridge.he_minus).unwrap().ring
}

/// An island: corners, and per edge the via point.
#[derive(Clone)]
struct Island {
    corners: Vec<Point3<f64>>,
    vias: Vec<Point3<f64>>,
}

impl Island {
    fn edge(&self, s: Sph, k: usize) -> Arc3 {
        let n = self.corners.len();
        arc3(
            s,
            self.corners[k % n],
            self.vias[k % n],
            self.corners[(k + 1) % n],
        )
    }
    fn reversed(&self) -> Island {
        let n = self.corners.len();
        Island {
            corners: (0..n).map(|k| self.corners[(n - k) % n]).collect(),
            vias: (0..n).map(|k| self.vias[n - 1 - k]).collect(),
        }
    }
    fn polyline(&self, s: Sph) -> Vec<(f64, f64)> {
        let mut out = Vec::new();
        for k in 0..self.corners.len() {
            let a = self.edge(s, k);
            for i in 0..400 {
                out.push(s.proj(a.carrier.eval(a.end * f64::from(i) / 400.0)));
            }
        }
        out
    }
}

fn winding(poly: &[(f64, f64)], (px, py): (f64, f64)) -> f64 {
    let mut total = 0.0;
    for i in 0..poly.len() {
        let (ax, ay) = (poly[i].0 - px, poly[i].1 - py);
        let j = (i + 1) % poly.len();
        let (bx, by) = (poly[j].0 - px, poly[j].1 - py);
        total += (ax * by - ay * bx).atan2(ax * bx + ay * by);
    }
    total / (2.0 * PI)
}

fn simple(poly: &[(f64, f64)]) -> bool {
    let n = poly.len();
    let cross = |o: (f64, f64), a: (f64, f64), b: (f64, f64)| {
        (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
    };
    for i in 0..n {
        for j in i + 2..n {
            if i == 0 && j == n - 1 {
                continue;
            }
            let (a, b, c, d) = (poly[i], poly[(i + 1) % n], poly[j], poly[(j + 1) % n]);
            if (cross(a, b, c) * cross(a, b, d) < 0.0) && (cross(c, d, a) * cross(c, d, b) < 0.0) {
                return false;
            }
        }
    }
    true
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.next()
    }
}

fn random_island(s: Sph, rng: &mut Rng) -> Option<Island> {
    let k = 3 + (rng.next() * 3.0) as usize;
    let mut th: Vec<f64> = (0..k).map(|_| rng.range(0.0, 2.0 * PI)).collect();
    th.sort_by(f64::total_cmp);
    for i in 0..k {
        let gap = (th[(i + 1) % k] - th[i]).rem_euclid(2.0 * PI);
        if gap < 0.35 {
            return None;
        }
    }
    let rho: Vec<f64> = (0..k).map(|_| rng.range(0.12, 0.7)).collect();
    let corners = (0..k).map(|i| s.polar(rho[i], th[i])).collect();
    let vias = (0..k)
        .map(|i| {
            let j = (i + 1) % k;
            let gap = (th[j] - th[i]).rem_euclid(2.0 * PI);
            let mid = th[i] + 0.5 * gap;
            let r = 0.5 * (rho[i] + rho[j]) * rng.range(0.35, 1.6);
            s.polar(r.min(0.95), mid)
        })
        .collect();
    let island = Island { corners, vias };
    let poly = island.polyline(s);
    // Stay well inside the sheet and away from the projection pole.
    for k in 0..island.corners.len() {
        let a = island.edge(s, k);
        for i in 0..=40 {
            let p = a.carrier.eval(a.end * f64::from(i) / 40.0);
            if ((p - s.f.o) / s.r).dot(s.f.x) < 0.25 {
                return None;
            }
        }
    }
    simple(&poly).then_some(island)
}

/// Every island, from every corner, in both directions and at both
/// senses, winds as the oracle's left-of-run membership says.
#[test]
fn a_random_sphere_island_winds_as_its_stereographic_oracle_does() {
    let mut rng = Rng(0x0123456789abcdef);
    let (mut seen, mut m2, mut asked) = ([0usize; 2], 0usize, 0usize);
    let mut failures = Vec::new();
    let mut built = 0;
    let (target, mut stood) = (36, 0);
    while built < target {
        let f = frames()[built % 3];
        let r = [1.0, 1e-3, 1e3][(built / 3) % 3];
        let s = Sph { f, r };
        let Some(island) = random_island(s, &mut rng) else {
            continue;
        };
        built += 1;
        if !read_at(0.1 * r) || 1e-13 * r > band().zero() {
            stood += 1;
            continue;
        }
        let poly = island.polyline(s);
        for back in [false, true] {
            let island = if back {
                island.reversed()
            } else {
                island.clone()
            };
            let n = island.corners.len();
            for c in 0..n {
                // The run leaves corner `c`; the chord is edge c-1.
                let closing = island.edge(s, c + n - 1);
                let (o, nn) = closing.plane;
                let mut sides = [false; 2];
                for k in 0..n - 1 {
                    let a = island.edge(s, c + k);
                    for i in 1..200 {
                        let d = nn.dot(a.carrier.eval(a.end * f64::from(i) / 200.0) - o) / r;
                        if d > 1e-6 {
                            sides[0] = true;
                        }
                        if d < -1e-6 {
                            sides[1] = true;
                        }
                    }
                }
                let both = sides[0] && sides[1];
                for sense in [true, false] {
                    let first = island.edge(s, c);
                    let m = first.carrier.mid_point(0.0, first.end);
                    let travel = first.carrier.deriv(0.5 * first.end);
                    let radial = (m - f.o) / r;
                    let outward = geom_brep::OutwardNormal::from_chart(radial, sense).vec();
                    let left = outward.cross(travel).normalize();
                    let x = m + left * (1e-4 * r);
                    let x = f.o + (x - f.o).normalize() * r;
                    let want = if winding(&poly, s.proj(x)).abs() > 0.5 {
                        Sign::Positive
                    } else {
                        Sign::Negative
                    };
                    let (mut body, face, key) = sheet(s, sense);
                    let ring = empty_ring(&mut body, face, island.corners[c]);
                    let mut halves: Vec<crate::MevCreated> = Vec::new();
                    for k in 0..n - 1 {
                        let a = island.edge(s, c + k);
                        let e = spec(&mut body, key, &a);
                        let site = match halves.last() {
                            None => MevSite::Lone { r#loop: ring },
                            Some(h) => MevSite::Fan {
                                he1: h.he_minus,
                                he2: h.he_minus,
                            },
                        };
                        halves.push(
                            body.mev(site, island.corners[(c + k + 1) % n], e, tol())
                                .unwrap(),
                        );
                    }
                    let run = (halves[0].he_plus, halves[n - 2].he_plus);
                    let cs = spec(&mut body, key, &closing);
                    asked += 1;
                    m2 += usize::from(both);
                    let got = path_island_winding(&body, face, run, Some(&cs), band());
                    match got {
                        Ok(Ok(g)) if g == want => {}
                        other => failures.push(format!(
                            "r {r:e}, back {back}, corner {c}, sense {sense}, both sides {both}: want {want:?}, got {other:?}"
                        )),
                    }
                    seen[usize::from(want == Sign::Positive)] += 1;
                }
            }
        }
    }
    if stood > 0 {
        test_utils::vacuity::stood_down(
            "a sphere island at scale 1e-3 or 1e3",
            "its smallest feature is within ten thousand coincidence widths at this ε, or \
             its arcs, built in f64, certify their ends only to about 1e-13 of its size",
        );
    }
    assert!(
        failures.is_empty() && seen[0] > 0 && seen[1] > 0 && m2 > 0,
        "asked {asked}, a run on both sides of the chord's plane {m2}, windings {seen:?}: {:?}",
        &failures[..failures.len().min(10)]
    );
}

/// After the chord's `mef`, a bystander ring inside the island moves and
/// one outside stays.
#[test]
fn a_bystander_of_a_random_sphere_island_is_re_homed_as_its_oracle_says() {
    let mut rng = Rng(0xfedcba9876543210);
    let (mut asked, mut failures) = (0usize, Vec::new());
    let mut built = 0;
    let (target, mut stood) = (18, 0);
    while built < target {
        let f = frames()[built % 3];
        let r = [1.0, 1e-3, 1e3][(built / 3) % 3];
        let s = Sph { f, r };
        let Some(island) = random_island(s, &mut rng) else {
            continue;
        };
        let poly = island.polyline(s);
        // Bystanders: a grid point inside, one outside (still on the sheet,
        // off the boundary).
        let mut pin = None;
        let mut pout = None;
        for i in 0..30 {
            for j in 0..30 {
                let p = s.polar(0.03 * f64::from(i), 2.0 * PI * f64::from(j) / 30.0 + 0.01);
                let w = winding(&poly, s.proj(p));
                let clear = poly.iter().all(|q| {
                    let pp = s.proj(p);
                    ((q.0 - pp.0).powi(2) + (q.1 - pp.1).powi(2)).sqrt() > 0.02
                });
                if !clear {
                    continue;
                }
                if w.abs() > 0.5 && pin.is_none() {
                    pin = Some(p);
                }
                if w.abs() < 0.5 && pout.is_none() && i > 3 {
                    pout = Some(p);
                }
            }
        }
        let (Some(pin), Some(pout)) = (pin, pout) else {
            continue;
        };
        built += 1;
        if !read_at(0.1 * r) || 1e-13 * r > band().zero() {
            stood += 1;
            continue;
        }
        let n = island.corners.len();
        for c in 0..n {
            let (mut body, face, key) = sheet(s, true);
            let ring_in = empty_ring(&mut body, face, pin);
            let ring_out = empty_ring(&mut body, face, pout);
            let rring = empty_ring(&mut body, face, island.corners[c]);
            let mut halves: Vec<crate::MevCreated> = Vec::new();
            for k in 0..n - 1 {
                let a = island.edge(s, c + k);
                let e = spec(&mut body, key, &a);
                let site = match halves.last() {
                    None => MevSite::Lone { r#loop: rring },
                    Some(h) => MevSite::Fan {
                        he1: h.he_minus,
                        he2: h.he_minus,
                    },
                };
                halves.push(
                    body.mev(site, island.corners[(c + k + 1) % n], e, tol())
                        .unwrap(),
                );
            }
            let run = (halves[0].he_plus, halves[n - 2].he_plus);
            let after = body.get_half_edge(run.1).unwrap().next;
            let site = MefSite::Chords {
                he1: run.0,
                he2: after,
            };
            let back = island.reversed();
            // The chord from corner c back to corner c-1 is the reversed
            // island's edge from its corner (n - c) % n.
            let chord = back.edge(s, (n - c) % n);
            let cs = spec(&mut body, key, &chord);
            let made = body
                .mef(site, cs, FaceSurface::Shared { key, sense: true }, tol())
                .unwrap();
            let remainder = body.get_half_edge(after).unwrap().parent_loop;
            let mut joiner = ChordJoiner::new(band());
            asked += 1;
            match joiner.rehome_rings(&mut body, face, made.face, remainder) {
                Err(e) => failures.push(format!("r {r:e} corner {c}: {e:?}")),
                Ok(_) => {
                    let face_of = |ring| body.get_loop(ring).unwrap().face;
                    if face_of(ring_in) != made.face || face_of(ring_out) != face {
                        failures.push(format!(
                            "r {r:e} corner {c}: in on new {}, out on old {}",
                            face_of(ring_in) == made.face,
                            face_of(ring_out) == face
                        ));
                    }
                }
            }
        }
    }
    if stood > 0 {
        test_utils::vacuity::stood_down(
            "a sphere island at scale 1e-3 or 1e3",
            "its smallest feature is within ten thousand coincidence widths at this ε, or \
             its arcs, built in f64, certify their ends only to about 1e-13 of its size",
        );
    }
    assert!(
        failures.is_empty() && asked > 0,
        "asked {asked}: {:?}",
        &failures[..failures.len().min(10)]
    );
}
