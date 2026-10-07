//! **What a non-isosceles turn's overrun is, measured**
//! (`work/band/a-non-isosceles-turn-overruns-past-the-mitre.md`, step 1):
//! a numeric probe, from the geometry alone and independent of the
//! battery, of two requested edges at a trivalent plane–plane vertex
//! whose third edge `L` is unrequested.
//!
//! The vertex is the apex of a convex trihedral cone: `L`, the requested
//! edges `e₁` and `e₂`, the shared face `S` (between `e₁` and `e₂`) and
//! `L`'s two faces `Fᵢ` (between `L` and `eᵢ`). The cone is the material
//! at a convex vertex and the air at a concave one; a band is a wedge of
//! that cone along its edge, cut away on the convex side and filled on
//! the concave side, so the two convexities are one computation on the
//! cone, and the setback depends on the cone alone
//! (`the_setback_reads_the_cone_alone_at_either_convexity`).
//!
//! Each case is measured three ways that share no code past the cone:
//! the closed forms, the trimlines and band surfaces intersected
//! directly, and a grid over each of `L`'s faces counting the points the
//! union of the two wedges takes beyond each face's own band.
//!
//! Run: `cargo nextest run -p sweep --run-ignored only -E
//! 'test(band_turn_overrun_probe)' --no-capture`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::Point3;
use topo::Body;

use crate::common::operands::{leaning_turn, parallelepiped};

type V = [f64; 3];

fn add(a: V, b: V) -> V {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn sub(a: V, b: V) -> V {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn mul(a: V, k: f64) -> V {
    [a[0] * k, a[1] * k, a[2] * k]
}
fn dot(a: V, b: V) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: V, b: V) -> V {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn norm(a: V) -> f64 {
    dot(a, a).sqrt()
}
fn unit(a: V) -> V {
    mul(a, 1.0 / norm(a))
}
fn angle(a: V, b: V) -> f64 {
    norm(cross(a, b)).atan2(dot(a, b))
}
/// `a` less its component along the unit `n`, normalised.
fn across(a: V, n: V) -> V {
    unit(sub(a, mul(n, dot(a, n))))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Verb {
    Chamfer,
    Fillet,
}

/// The cone at the vertex (the origin): unit edge directions and the
/// inward unit normals of its three faces.
#[derive(Clone, Copy, Debug)]
struct Cone {
    l: V,
    e: [V; 2],
    n_s: V,
    n: [V; 2],
}

impl Cone {
    fn of(l: V, e1: V, e2: V) -> Self {
        let (l, e1, e2) = (unit(l), unit(e1), unit(e2));
        let inward = |a: V, b: V, toward: V| {
            let n = unit(cross(a, b));
            if dot(n, toward) < 0.0 {
                mul(n, -1.0)
            } else {
                n
            }
        };
        Cone {
            l,
            e: [e1, e2],
            n_s: inward(e1, e2, l),
            n: [inward(l, e1, e2), inward(l, e2, e1)],
        }
    }

    /// The cone of face angles `φ₁`, `φ₂` (each `eᵢ` against `L`) and
    /// dihedral `γ` at `L`.
    fn angles(phi: [f64; 2], gamma: f64) -> Self {
        let l = [0.0, 0.0, 1.0];
        let u = [[1.0, 0.0, 0.0], [gamma.cos(), gamma.sin(), 0.0]];
        let e = |i: usize| add(mul(l, phi[i].cos()), mul(u[i], phi[i].sin()));
        Cone::of(l, e(0), e(1))
    }

    /// The dihedral inside the cone at `eᵢ`, between `S` and `Fᵢ`.
    fn beta(&self, i: usize) -> f64 {
        PI - angle(self.n_s, self.n[i])
    }
    fn phi(&self, i: usize) -> f64 {
        angle(self.e[i], self.l)
    }
    fn theta_s(&self) -> f64 {
        angle(self.e[0], self.e[1])
    }
    fn gamma(&self) -> f64 {
        PI - angle(self.n[0], self.n[1])
    }
    /// `L`'s height over `S` per unit length: `sin φᵢ · sin βᵢ`, either `i`.
    fn h(&self) -> f64 {
        dot(self.l, self.n_s)
    }
    fn inside(&self, x: V, eps: f64) -> bool {
        dot(x, self.n_s) >= -eps && dot(x, self.n[0]) >= -eps && dot(x, self.n[1]) >= -eps
    }
}

/// One requested edge's band: its setback (equal on both supports, for
/// either verb), the in-plane units from the edge toward each trimline,
/// and its surface — the chamfer's plane, or the fillet's axis.
#[derive(Clone, Copy, Debug)]
struct Band {
    e: V,
    s: f64,
    w_s: V,
    w_f: V,
    /// Chamfer: the strip plane's unit normal, towards the vertex.
    plane: V,
    /// Fillet: a point of the axis (along `e`) and the radius.
    axis: V,
    r: f64,
}

fn band(c: &Cone, i: usize, verb: Verb, size: f64) -> Band {
    let e = c.e[i];
    let w_s = across(c.n[i], c.n_s);
    let w_f = across(c.n_s, c.n[i]);
    let beta = c.beta(i);
    let s = match verb {
        Verb::Chamfer => size,
        Verb::Fillet => size / (beta / 2.0).tan(),
    };
    let (ts, tf) = (mul(w_s, s), mul(w_f, s));
    let mut plane = unit(cross(e, sub(tf, ts)));
    if dot(sub([0.0; 3], ts), plane) < 0.0 {
        plane = mul(plane, -1.0);
    }
    Band {
        e,
        s,
        w_s,
        w_f,
        plane,
        axis: add(ts, mul(c.n_s, size)),
        r: size,
    }
}

impl Band {
    fn axis_distance(&self, x: V) -> f64 {
        let d = sub(x, self.axis);
        norm(sub(d, mul(self.e, dot(d, self.e))))
    }
    /// Whether `x` (in the closed cone) lies in the wedge this band takes.
    fn takes(&self, verb: Verb, x: V) -> bool {
        match verb {
            Verb::Chamfer => dot(sub(x, mul(self.w_s, self.s)), self.plane) > 0.0,
            Verb::Fillet => {
                dot(x, self.w_s) < self.s
                    && dot(x, self.w_f) < self.s
                    && self.axis_distance(x) > self.r
            }
        }
    }
}

/// The meet of the line `o + λ d` with the plane `{x · n = k}`.
fn meet(o: V, d: V, n: V, k: f64) -> V {
    add(o, mul(d, (k - dot(o, n)) / dot(d, n)))
}

/// One turn, measured.
#[derive(Clone, Debug)]
struct Turn {
    verb: Verb,
    /// Each band's trimline on its face of `L`, met with `L`, as
    /// distances along `L`: direct, and closed form.
    feet: [f64; 2],
    feet_closed: [f64; 2],
    /// The band whose foot is further, and the face the overrun lies on
    /// (the other band's support of `L`); `None` when the feet coincide.
    far: Option<usize>,
    gap: f64,
    /// The trimlines' crossing on `S` and the mitre's end on each `Fⱼ`.
    crossing: V,
    q: [V; 2],
    /// The face the mitre reaches first, read from the ends alone.
    mitre_lands: Option<usize>,
    /// The overrun region on the near face: closed form (chamfer) or
    /// boundary integral (fillet), and the grid's count.
    area: f64,
    area_grid: f64,
    /// The grid's resolution: what its counts may be off by.
    grid_tol: f64,
    /// What the union takes beyond its own band on the FAR face (the
    /// grid's count; zero when the overrun is one face's alone).
    far_face_extra: f64,
    /// The overrun region's angle at the far foot (between `L` and the
    /// overrun curve) and at the mitre's end (between the near band's
    /// trimline and the overrun curve).
    angle_at_foot: f64,
    angle_at_mitre: f64,
    /// The turn the band's boundary makes at the mitre's end, from the
    /// mitre onto the overrun curve (zero: tangent).
    mitre_kink: f64,
    /// Fillet: the overrun ellipse's semi-axes.
    ellipse: Option<(f64, f64)>,
}

fn measure(c: &Cone, verb: Verb, size: f64, grid: usize) -> Turn {
    let b = [band(c, 0, verb, size), band(c, 1, verb, size)];
    let foot = |i: usize| b[i].s / dot(c.l, b[i].w_f);
    let feet = [foot(0), foot(1)];
    let feet_closed = [0, 1].map(|i| match verb {
        Verb::Chamfer => size / c.phi(i).sin(),
        Verb::Fillet => size * (1.0 + c.beta(i).cos()) / c.h(),
    });
    let gap = (feet[0] - feet[1]).abs();
    let far = (gap > 1e-12 * size).then(|| usize::from(feet[1] > feet[0]));
    // The crossing: x = a e₁ + b e₂ at setback from each edge on S.
    let crossing = add(
        mul(c.e[0], b[1].s / dot(c.e[0], b[1].w_s)),
        mul(c.e[1], b[0].s / dot(c.e[1], b[0].w_s)),
    );
    // The mitre's end on each Fⱼ: on Fⱼ's trimline of band j, and on the
    // mitre (the chamfer planes' line; the fillet's bisector plane
    // through the axes' crossing, normal e₁ − e₂).
    let q = [0, 1].map(|j| {
        let (o, d) = (mul(b[j].w_f, b[j].s), b[j].e);
        match verb {
            Verb::Chamfer => {
                let k = 1 - j;
                meet(o, d, b[k].plane, dot(mul(b[k].w_s, b[k].s), b[k].plane))
            }
            Verb::Fillet => {
                // The axes project onto the trimlines on S, so they
                // cross over the trimlines' crossing.
                let apex = add(crossing, mul(c.n_s, size));
                let n = sub(c.e[0], c.e[1]);
                let x = meet(o, d, n, dot(apex, n));
                let k = 1 - j;
                assert!(
                    (b[k].axis_distance(x) - size).abs() <= 1e-9 * size,
                    "the fillet mitre's end on F{} lies on both cylinders",
                    j + 1
                );
                x
            }
        }
    });
    // Which of L's faces the mitre reaches first, walking it from the
    // crossing: the chamfer's line into the cone, to the nearer of the
    // two face planes. The fillet's ellipse touches each Fⱼ (tangent to
    // cylinder j) at its end on Fⱼ, crossing neither: it lands on F₂
    // first when that end lies inside band 1's arc, short of F₁.
    let mitre_lands = if gap <= 1e-12 * size {
        None
    } else {
        match verb {
            Verb::Chamfer => {
                let mut m = cross(b[0].plane, b[1].plane);
                if dot(m, c.n_s) < 0.0 {
                    m = mul(m, -1.0);
                }
                let mu = [0, 1].map(|j| -dot(crossing, c.n[j]) / dot(m, c.n[j]));
                let ahead = |j: usize| mu[j] > 0.0;
                match (ahead(0), ahead(1)) {
                    (true, true) => Some(usize::from(mu[1] < mu[0])),
                    (true, false) => Some(0),
                    (false, true) => Some(1),
                    (false, false) => None,
                }
            }
            Verb::Fillet => {
                let (r0, r1) = (mul(c.n_s, -1.0), mul(c.n[0], -1.0));
                let radial = across(sub(q[1], b[0].axis), c.e[0]);
                let short_of_f1 =
                    dot(radial, across(r1, r0)) > 0.0 && angle(r0, radial) < angle(r0, r1);
                Some(usize::from(short_of_f1))
            }
        }
    };
    let mut turn = Turn {
        verb,
        feet,
        feet_closed,
        far,
        gap,
        crossing,
        q,
        mitre_lands,
        area: 0.0,
        area_grid: 0.0,
        grid_tol: 0.0,
        far_face_extra: 0.0,
        angle_at_foot: 0.0,
        angle_at_mitre: 0.0,
        mitre_kink: 0.0,
        ellipse: None,
    };
    let Some(k) = far else {
        return turn;
    };
    let j = 1 - k;
    let (p_near, p_far, qj) = (mul(c.l, feet[j]), mul(c.l, feet[k]), q[j]);
    // The overrun curve, far foot to mitre end, on Fⱼ.
    let curve: Vec<V> = match verb {
        Verb::Chamfer => vec![p_far, qj],
        Verb::Fillet => {
            let bk = b[k];
            let radial = |x: V| across(sub(x, bk.axis), bk.e);
            let (r0, r1) = (radial(p_far), radial(qj));
            let span = angle(r0, r1);
            let ortho = across(r1, r0);
            (0..=400)
                .map(|t| {
                    let a = span * f64::from(t) / 400.0;
                    let rho = add(mul(r0, a.cos()), mul(ortho, a.sin()));
                    let o = add(bk.axis, mul(rho, bk.r));
                    meet(o, bk.e, c.n[j], 0.0)
                })
                .collect()
        }
    };
    // In-face coordinates on Fⱼ: along L, and across it toward eⱼ.
    let u = across(c.e[j], c.l);
    let flat = |x: V| (dot(x, c.l), dot(x, u));
    let mut ring: Vec<(f64, f64)> = vec![flat(p_near)];
    ring.extend(curve.iter().map(|&x| flat(x)));
    turn.area = ring
        .iter()
        .zip(ring.iter().cycle().skip(1))
        .map(|(a, b)| a.0 * b.1 - b.0 * a.1)
        .sum::<f64>()
        .abs()
        / 2.0;
    if verb == Verb::Chamfer {
        let closed = 0.5 * gap * gap * c.phi(0).sin() * c.phi(1).sin() / c.theta_s().sin();
        assert!(
            (closed - turn.area).abs() <= 1e-9 * size * size,
            "the chamfer overrun's closed-form area {closed} against its triangle {}",
            turn.area
        );
    }
    let toward = |a: V, b: V| unit(sub(b, a));
    turn.angle_at_foot = angle(mul(c.l, -1.0), toward(curve[0], curve[1]));
    let n = curve.len();
    turn.angle_at_mitre = angle(toward(qj, p_near), toward(curve[n - 1], curve[n - 2]));
    // The mitre's direction of travel as it arrives at qⱼ.
    let arriving = match verb {
        Verb::Chamfer => toward(crossing, qj),
        Verb::Fillet => {
            let bk = b[k];
            let r0 = across(sub(crossing, bk.axis), bk.e);
            let rq = across(sub(qj, bk.axis), bk.e);
            let (span, ortho) = (angle(r0, rq), across(rq, r0));
            let a = span * (1.0 - 1e-6);
            let rho = add(mul(r0, a.cos()), mul(ortho, a.sin()));
            let n = sub(c.e[0], c.e[1]);
            let apex = add(crossing, mul(c.n_s, size));
            toward(
                meet(add(bk.axis, mul(rho, size)), bk.e, n, dot(apex, n)),
                qj,
            )
        }
    };
    turn.mitre_kink = angle(arriving, toward(curve[n - 1], curve[n - 2]));
    if verb == Verb::Fillet {
        turn.ellipse = Some((size, size / dot(c.e[k], c.n[j]).abs()));
    }
    // The grid: what the union takes on each face beyond that face's own
    // band, over a box enclosing every point the probe names.
    let reach = [p_far, p_near, qj, q[k], crossing]
        .iter()
        .chain(curve.iter())
        .map(|&x| norm(x))
        .fold(0.0, f64::max)
        * 1.5;
    let extra = |face: usize, own: usize| {
        let u = across(c.e[face], c.l);
        let cell = 2.0 * reach / grid as f64;
        let mut count = 0usize;
        for ia in 0..grid {
            for ib in 0..grid {
                let (a, bb) = (
                    -reach + (ia as f64 + 0.5) * cell,
                    (ib as f64 + 0.5) * cell * 0.5,
                );
                let x = add(mul(c.l, a), mul(u, bb));
                if !c.inside(x, 1e-12 * size) {
                    continue;
                }
                let taken = b[0].takes(verb, x) || b[1].takes(verb, x);
                if taken && !b[own].takes(verb, x) {
                    count += 1;
                }
            }
        }
        count as f64 * cell * cell * 0.5
    };
    turn.grid_tol = 6.0 * reach * (2.0 * reach / grid as f64);
    turn.area_grid = extra(j, j);
    turn.far_face_extra = extra(k, k);
    turn
}

fn deg(x: f64) -> f64 {
    x * 180.0 / PI
}

fn row(name: &str, c: &Cone, t: &Turn, size: f64) -> String {
    let face = |i: Option<usize>| i.map_or("—".to_string(), |i| format!("F{}", i + 1));
    format!(
        "{name:<34} {:>7} φ=({:5.1},{:5.1}) β=({:5.1},{:5.1}) γ={:5.1} θS={:5.1} | \
         feet/d=({:.4},{:.4}) gap/d={:.4} far=e{} onto={} mitre→{} | \
         area/d²={:.5} grid={:.5} farface={:.1e} ∠foot={:5.1} ∠mitre={:5.1} kink={:5.1}{}",
        format!("{:?}", t.verb),
        deg(c.phi(0)),
        deg(c.phi(1)),
        deg(c.beta(0)),
        deg(c.beta(1)),
        deg(c.gamma()),
        deg(c.theta_s()),
        t.feet[0] / size,
        t.feet[1] / size,
        t.gap / size,
        t.far.map_or("—".to_string(), |k| (k + 1).to_string()),
        face(t.far.map(|k| 1 - k)),
        face(t.mitre_lands),
        t.area / size / size,
        t.area_grid / size / size,
        t.far_face_extra / size / size,
        deg(t.angle_at_foot),
        deg(t.angle_at_mitre),
        deg(t.mitre_kink),
        t.ellipse.map_or(String::new(), |(a, b)| format!(
            " ellipse=({:.3},{:.3})r",
            a / size,
            b / size
        )),
    )
}

/// The checks every measured row owes, labelled by the row.
fn check(name: &str, c: &Cone, t: &Turn, size: f64) {
    for i in 0..2 {
        assert!(
            (t.feet[i] - t.feet_closed[i]).abs() <= 1e-9 * size,
            "{name}: foot {i} direct {} against closed form {}",
            t.feet[i],
            t.feet_closed[i]
        );
    }
    // Who reaches further, in closed form: the chamfer the larger sin β,
    // the fillet the smaller β.
    if let Some(k) = t.far {
        let j = 1 - k;
        match t.verb {
            Verb::Chamfer => assert!(c.beta(k).sin() > c.beta(j).sin(), "{name}: chamfer far"),
            Verb::Fillet => assert!(c.beta(k) < c.beta(j), "{name}: fillet far"),
        }
        assert_eq!(
            t.mitre_lands,
            Some(j),
            "{name}: the mitre lands on the near band's face"
        );
        let tol = 0.02 * t.area + t.grid_tol;
        assert!(
            (t.area - t.area_grid).abs() <= tol,
            "{name}: overrun area {} against the grid's {}",
            t.area,
            t.area_grid
        );
        assert!(
            t.far_face_extra <= tol,
            "{name}: the far face loses {} beyond its band",
            t.far_face_extra
        );
        if t.verb == Verb::Fillet {
            assert!(
                t.angle_at_foot < 0.05,
                "{name}: the fillet overrun leaves the far foot along L ({})",
                deg(t.angle_at_foot)
            );
        }
    } else {
        // Coincident feet: the mitre lands on L, at the common foot.
        for j in 0..2 {
            assert!(
                norm(sub(t.q[j], mul(c.l, t.feet[0]))) <= 1e-9 * size,
                "{name}: the mitre's end on F{} is the common foot",
                j + 1
            );
        }
    }
    assert!(c.inside(t.crossing, 1e-12), "{name}: the crossing is on S");
}

/// The corner of `body` at `at` as a cone, its edge toward `l_to` taken
/// as `L`.
fn cone_at(body: &Body<f64>, at: Point3<f64>, l_to: Point3<f64>) -> Cone {
    let point = |v| {
        *body
            .get_point(body.get_vertex(v).expect("a vertex").point)
            .expect("a point")
    };
    let close = |x: Point3<f64>, y: Point3<f64>| (x - y).norm() < 1e-9;
    let mut out: Vec<V> = topo::query::all_edges(body)
        .into_iter()
        .filter_map(|e| {
            let he = body.get_edge(e).expect("an edge").he_plus;
            let p = point(body.get_half_edge(he).expect("a half").start);
            let q = point(body.half_edge_end(he).expect("an end"));
            let far = if close(p, at) {
                q
            } else if close(q, at) {
                p
            } else {
                return None;
            };
            Some([far.x - at.x, far.y - at.y, far.z - at.z])
        })
        .collect();
    assert_eq!(out.len(), 3, "a trivalent corner at {at:?}");
    let to = unit([l_to.x - at.x, l_to.y - at.y, l_to.z - at.z]);
    let li = out
        .iter()
        .position(|&d| dot(unit(d), to) > 1.0 - 1e-9)
        .expect("L among the corner's edges");
    let l = out.remove(li);
    Cone::of(l, out[0], out[1])
}

fn witnesses() -> Vec<(String, Cone)> {
    let s2 = core::f64::consts::FRAC_1_SQRT_2;
    let mut w = vec![
        // The bracket's section face x + y = 2.75 on the x-leg, its cap
        // z = 0.75: e₁ the cap chord, e₂ the section edge, L along the
        // side wall's top edge. At y = 0 the section edge's dihedral is
        // 45°, at y = 1 it is 135°.
        (
            "bracket y=0 (β₂ 45°)".to_string(),
            Cone::of([-1.0, 0.0, 0.0], [-s2, s2, 0.0], [0.0, 0.0, -1.0]),
        ),
        (
            "bracket y=1 (β₂ 135°)".to_string(),
            Cone::of([-1.0, 0.0, 0.0], [s2, -s2, 0.0], [0.0, 0.0, -1.0]),
        ),
    ];
    let s = 0.3;
    let sheared = parallelepiped(s);
    w.push((
        "sheared box s=0.3 (supplementary)".to_string(),
        cone_at(
            &sheared,
            Point3::new(2.0 + s, s, 1.0),
            Point3::new(2.0, 0.0, 0.0),
        ),
    ));
    w.push((
        "sheared box s=0.3 (isosceles)".to_string(),
        cone_at(&sheared, Point3::new(s, s, 1.0), Point3::new(0.0, 0.0, 0.0)),
    ));
    for lean in [0.5, -0.5] {
        let (body, _) = leaning_turn(lean);
        w.push((
            format!("leaning prism s={lean}"),
            cone_at(
                &body,
                Point3::new(lean, 0.0, 1.0),
                Point3::new(0.0, 0.0, 0.0),
            ),
        ));
    }
    w
}

/// **The three witnesses**, both verbs: the bracket's section-face
/// corners, the sheared box's supplementary (and isosceles) corner, and
/// the leaning prism, each corner read off its body where the tests can
/// build it.
#[test]
#[ignore = "a probe: prints its table; run command in the module docs"]
fn the_overrun_at_the_witnesses() {
    let d = 0.1;
    for (name, c) in witnesses() {
        for verb in [Verb::Chamfer, Verb::Fillet] {
            let t = measure(&c, verb, d, 800);
            println!("{}", row(&name, &c, &t, d));
            check(&name, &c, &t, d);
        }
    }
}

/// **The sweep**: face angles `(φ₁, φ₂)` over `(20°…160°)²` in 10°
/// steps at five dihedrals at `L`, both verbs. One line per case, then a
/// regime count per dihedral.
#[test]
#[ignore = "a probe: prints its table; run command in the module docs"]
fn the_overrun_over_the_face_angle_sweep() {
    let d = 1.0;
    for gamma in [30.0_f64, 60.0, 90.0, 120.0, 150.0] {
        let mut census = std::collections::BTreeMap::<String, usize>::new();
        let mut worst = (0.0_f64, String::new());
        for a in (2..=16).map(|k| f64::from(k) * 10.0) {
            for b in (2..=16).map(|k| f64::from(k) * 10.0) {
                let c = Cone::angles([a.to_radians(), b.to_radians()], gamma.to_radians());
                for verb in [Verb::Chamfer, Verb::Fillet] {
                    let t = measure(&c, verb, d, 160);
                    let name = format!("γ={gamma} φ=({a},{b})");
                    println!("{}", row(&name, &c, &t, d));
                    check(&name, &c, &t, d);
                    let regime = match (t.far, verb) {
                        (None, _) if (a - b).abs() < 1e-9 => "isosceles",
                        (None, _) => "coincident feet, not isosceles",
                        (Some(_), _) => "overrun",
                    };
                    *census.entry(format!("{verb:?} {regime}")).or_default() += 1;
                    if verb == Verb::Chamfer && t.far.is_some() {
                        let rel = t.gap / d;
                        if rel > worst.0 {
                            worst = (rel, name.clone());
                        }
                    }
                }
            }
        }
        println!(
            "γ={gamma}: {census:?}; widest chamfer gap/d {:.3} at {}",
            worst.0, worst.1
        );
    }
}

/// **The setback reads the cone alone**: the kernel's spelling of a
/// fillet's setback, `r·√((1 − d)/(1 + d))` over the supports' OUTWARD
/// normals of the material, is the cone's `r·cot(β/2)` at a convex edge
/// (the cone is the material, outward normals the cone's inward ones
/// negated) and at a concave one (the cone is the air, outward normals
/// of the material the cone's inward ones), so the two convexities'
/// rows are one row.
#[test]
#[ignore = "a probe: prints its table; run command in the module docs"]
fn the_setback_reads_the_cone_alone_at_either_convexity() {
    for (a, b, g) in [
        (40.0_f64, 110.0_f64, 70.0_f64),
        (135.0, 90.0, 90.0),
        (25.0, 150.0, 140.0),
    ] {
        let c = Cone::angles([a.to_radians(), b.to_radians()], g.to_radians());
        for i in 0..2 {
            let cone = 1.0 / (c.beta(i) / 2.0).tan();
            for (convexity, sign) in [("convex", -1.0), ("concave", 1.0)] {
                let (oa, ob) = (mul(c.n_s, sign), mul(c.n[i], sign));
                let dd = dot(oa, ob);
                let kernel = ((1.0 - dd) / (1.0 + dd)).sqrt();
                assert!(
                    (kernel - cone).abs() < 1e-12,
                    "{convexity} φ=({a},{b}) γ={g} edge {i}: {kernel} against {cone}"
                );
            }
        }
    }
}
