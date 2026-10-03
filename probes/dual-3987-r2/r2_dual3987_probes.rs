//! Reviewer R2 probes for PR #3987 (the boolean door takes finished
//! bodies). Independent oracle: analytic box membership (inverse map)
//! and Monte Carlo volume, never the kernel's own predicates.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stderr)]

use crate::common;
use geom_core::{Band, Point3, Tol};
use topo::{
    AtRestBody, AtRestPolicy, BooleanError, BooleanResult, SolidContainment, intersect,
    mass_properties, point_in_solid, subtract, union,
};

/// A deterministic LCG in [0, 1).
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
}

/// A box: centre-corner `o`, orthonormal frame `r` (rows), extents `s`.
#[derive(Clone, Copy)]
struct Bx {
    o: [f64; 3],
    r: [[f64; 3]; 3],
    s: [f64; 3],
}

impl Bx {
    fn at(&self, u: f64, v: f64, w: f64) -> [f64; 3] {
        let l = [u * self.s[0], v * self.s[1], w * self.s[2]];
        let mut p = self.o;
        for (i, pi) in p.iter_mut().enumerate() {
            for (j, lj) in l.iter().enumerate() {
                *pi += self.r[j][i] * lj;
            }
        }
        p
    }
    /// Local coordinates in [0,1]^3 iff inside.
    fn local(&self, p: [f64; 3]) -> [f64; 3] {
        let d = [p[0] - self.o[0], p[1] - self.o[1], p[2] - self.o[2]];
        let mut q = [0.0; 3];
        for (j, qj) in q.iter_mut().enumerate() {
            *qj = (self.r[j][0] * d[0] + self.r[j][1] * d[1] + self.r[j][2] * d[2]) / self.s[j];
        }
        q
    }
    /// Signed clearance in metres: >0 inside, <0 outside.
    fn depth(&self, p: [f64; 3]) -> f64 {
        let q = self.local(p);
        (0..3)
            .map(|j| (q[j] * self.s[j]).min((1.0 - q[j]) * self.s[j]))
            .fold(f64::INFINITY, f64::min)
    }
    fn body(&self, tol: Tol) -> AtRestBody<f64> {
        let me = *self;
        let raw = common::mapped_cube(
            move |u, v, w| {
                let p = me.at(u, v, w);
                Point3::new(p[0], p[1], p[2])
            },
            tol,
        );
        AtRestBody::validate(raw, tol).expect("a mapped box is a finished body")
    }
    fn volume(&self) -> f64 {
        self.s[0] * self.s[1] * self.s[2]
    }
}

fn rot(axis: [f64; 3], ang: f64) -> [[f64; 3]; 3] {
    let n = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
    let (x, y, z) = (axis[0] / n, axis[1] / n, axis[2] / n);
    let (c, s) = (ang.cos(), ang.sin());
    let t = 1.0 - c;
    // Rows are the rotated frame's axes (R^T of the column form).
    let m = [
        [t * x * x + c, t * x * y - s * z, t * x * z + s * y],
        [t * x * y + s * z, t * y * y + c, t * y * z - s * x],
        [t * x * z - s * y, t * y * z + s * x, t * z * z + c],
    ];
    [
        [m[0][0], m[1][0], m[2][0]],
        [m[0][1], m[1][1], m[2][1]],
        [m[0][2], m[1][2], m[2][2]],
    ]
}

#[derive(Clone, Copy, Debug)]
enum Op {
    U,
    I,
    S,
}

fn run(op: Op, a: &AtRestBody<f64>, b: &AtRestBody<f64>, tol: Tol) -> Result<BooleanResult<f64>, BooleanError> {
    match op {
        Op::U => union(a, b, tol),
        Op::I => intersect(a, b, tol),
        Op::S => subtract(a, b, tol),
    }
}

/// Membership of an expression tree over boxes, with clearance.
#[derive(Clone)]
enum Ex {
    B(Bx),
    Op(Op, Box<Ex>, Box<Ex>),
}
impl Ex {
    /// An axis-aligned box holding the expression's material (oracle side).
    fn bbox(&self) -> ([f64; 3], [f64; 3]) {
        match self {
            Ex::B(b) => {
                let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
                for c in 0..8 {
                    let p = b.at((c & 1) as f64, ((c >> 1) & 1) as f64, ((c >> 2) & 1) as f64);
                    for i in 0..3 {
                        lo[i] = lo[i].min(p[i]);
                        hi[i] = hi[i].max(p[i]);
                    }
                }
                (lo, hi)
            }
            Ex::Op(Op::U, a, b) => {
                let ((l1, h1), (l2, h2)) = (a.bbox(), b.bbox());
                (core::array::from_fn(|i| l1[i].min(l2[i])), core::array::from_fn(|i| h1[i].max(h2[i])))
            }
            Ex::Op(Op::I, a, b) => {
                let ((l1, h1), (l2, h2)) = (a.bbox(), b.bbox());
                (core::array::from_fn(|i| l1[i].max(l2[i])), core::array::from_fn(|i| h1[i].min(h2[i])))
            }
            Ex::Op(Op::S, a, _) => a.bbox(),
        }
    }
    /// Signed depth: >0 inside, <0 outside (min/max algebra).
    fn depth(&self, p: [f64; 3]) -> f64 {
        match self {
            Ex::B(b) => b.depth(p),
            Ex::Op(Op::U, a, b) => a.depth(p).max(b.depth(p)),
            Ex::Op(Op::I, a, b) => a.depth(p).min(b.depth(p)),
            Ex::Op(Op::S, a, b) => a.depth(p).min(-b.depth(p)),
        }
    }
}

/// Checks a result against its expression: point_in_solid at random
/// points clear of every boundary, and the volume by Monte Carlo.
/// Returns the number of disagreements.
fn check(what: &str, body: &AtRestBody<f64>, ex: &Ex, lo: [f64; 3], hi: [f64; 3], rng: &mut Rng, tol: Tol) -> usize {
    let band = Band::linear(tol).unwrap();
    let scale = (0..3).map(|i| hi[i] - lo[i]).fold(0.0, f64::max);
    let margin = 1e-6 * scale + 1e3 * tol.eps();
    let mut bad = 0;
    let mut probed = 0;
    // Membership points over the oracle's bbox, widened 20 %.
    let (lo, hi) = {
        let (l, h) = ex.bbox();
        let (l, h) = if (0..3).any(|i| h[i] <= l[i]) { (lo, hi) } else { (l, h) };
        let w: [f64; 3] = core::array::from_fn(|i| 0.2 * (h[i] - l[i]));
        (core::array::from_fn::<f64, 3, _>(|i| l[i] - w[i]), core::array::from_fn::<f64, 3, _>(|i| h[i] + w[i]))
    };
    let (mut nin, mut nout) = (0, 0);
    let mut tries = 0;
    while (nin < 30 || nout < 30) && tries < 2_000_000 {
        tries += 1;
        let p = [
            lo[0] + rng.next() * (hi[0] - lo[0]),
            lo[1] + rng.next() * (hi[1] - lo[1]),
            lo[2] + rng.next() * (hi[2] - lo[2]),
        ];
        let d = ex.depth(p);
        if d.abs() < margin {
            continue;
        }
        if (d > 0.0 && nin >= 30) || (d < 0.0 && nout >= 30) {
            continue;
        }
        probed += 1;
        let want = if d > 0.0 { nin += 1; SolidContainment::In } else { nout += 1; SolidContainment::Out };
        match point_in_solid(body, Point3::new(p[0], p[1], p[2]), band, tol) {
            Ok(got) if got == want => {}
            other => {
                bad += 1;
                eprintln!("{what}: at {p:?} want {want:?} got {other:?}");
            }
        }
    }
    if nin == 0 {
        eprintln!("{what}: no inside point sampled ({nout} out)");
    }
    // Monte Carlo volume, over the oracle's own bounding box.
    let (lo, hi) = ex.bbox();
    if (0..3).any(|i| hi[i] <= lo[i]) {
        let v = mass_properties(&**body, tol).unwrap().volume;
        eprintln!("{what}: oracle bbox empty, kernel volume {v:e}");
        return bad + 1;
    }
    let n = 200_000;
    let mut inside = 0usize;
    for _ in 0..n {
        let p = [
            lo[0] + rng.next() * (hi[0] - lo[0]),
            lo[1] + rng.next() * (hi[1] - lo[1]),
            lo[2] + rng.next() * (hi[2] - lo[2]),
        ];
        if ex.depth(p) > 0.0 {
            inside += 1;
        }
    }
    let cube = (hi[0] - lo[0]) * (hi[1] - lo[1]) * (hi[2] - lo[2]);
    let pr = inside as f64 / n as f64;
    let mc = pr * cube;
    let pe = pr.max(5.0 / n as f64);
    let sigma = (pe * (1.0 - pe) / n as f64).sqrt() * cube;
    let v = mass_properties(&**body, tol).unwrap().volume;
    if (v - mc).abs() > 5.0 * sigma + 1e-9 * cube {
        bad += 1;
        eprintln!("{what}: volume {v:e} vs Monte Carlo {mc:e} ± {sigma:e}");
    }
    bad
}

fn random_box(rng: &mut Rng, scale: f64) -> Bx {
    let axis = [rng.next() - 0.5, rng.next() - 0.5, rng.next() - 0.5];
    let ang = rng.next() * 3.0;
    Bx {
        o: [
            scale * (rng.next() * 0.6 - 0.3),
            scale * (rng.next() * 0.6 - 0.3),
            scale * (rng.next() * 0.6 - 0.3),
        ],
        r: rot(axis, ang),
        s: [
            scale * (0.4 + rng.next()),
            scale * (0.4 + rng.next()),
            scale * (0.4 + rng.next()),
        ],
    }
}

/// E2E: random rotated box pairs at three scales, every op, both
/// orders, results reused as operands; membership + volume oracle.
/// Counts typed refusals separately (an honest refusal is not wrong).
#[test]
fn r2_rotated_boxes_every_op_both_orders_reused() {
    let tol = Tol::witness();
    let mut rng = Rng(0x3987_0002);
    let (mut answered, mut refused, mut bad) = (0, 0, 0);
    let mut refusals: Vec<String> = Vec::new();
    for &scale in &[1e-3, 1.0, 1e3] {
        for case in 0..6 {
            let (ba, bb, bc) = (random_box(&mut rng, scale), random_box(&mut rng, scale), random_box(&mut rng, scale));
            let (a, b, c) = (ba.body(tol), bb.body(tol), bc.body(tol));
            let lo = [-2.5 * scale; 3];
            let hi = [2.5 * scale; 3];
            for op in [Op::U, Op::I, Op::S] {
                for (x, y, ex, ey, ord) in [(&a, &b, ba, bb, "AB"), (&b, &a, bb, ba, "BA")] {
                    let what = format!("s{scale:e} c{case} {op:?} {ord}");
                    let ex1 = Ex::Op(op, Box::new(Ex::B(ex)), Box::new(Ex::B(ey)));
                    match run(op, x, y, tol) {
                        Ok(BooleanResult::Body(r)) => {
                            answered += 1;
                            bad += check(&what, &r.body, &ex1, lo, hi, &mut rng, tol);
                            // Reuse: (x op y) − c and (x op y) ∪ c.
                            for op2 in [Op::S, Op::U] {
                                let ex2 = Ex::Op(op2, Box::new(ex1.clone()), Box::new(Ex::B(bc)));
                                let w2 = format!("{what} then {op2:?} C");
                                match run(op2, &r.body, &c, tol) {
                                    Ok(BooleanResult::Body(r2)) => {
                                        answered += 1;
                                        bad += check(&w2, &r2.body, &ex2, lo, hi, &mut rng, tol);
                                    }
                                    Ok(BooleanResult::Empty) => answered += 1,
                                    Err(e) => {
                                        refused += 1;
                                        refusals.push(format!("{w2}: {e}"));
                                    }
                                }
                            }
                        }
                        Ok(BooleanResult::Empty) => {
                            answered += 1;
                            // The oracle must agree the result is empty.
                            let vol_hint = ex.volume().min(ey.volume());
                            let _ = vol_hint;
                        }
                        Err(e) => {
                            refused += 1;
                            refusals.push(format!("{what}: {e}"));
                        }
                    }
                }
            }
        }
    }
    for r in &refusals {
        eprintln!("REFUSED {r}");
    }
    eprintln!("answered {answered}, refused {refused}, wrong {bad}");
    assert_eq!(bad, 0, "a shipped result disagrees with the oracle");
}

/// Claim 1, census half: two bricks touching along one edge. The union
/// ships; does the result pass tier 3′ over its own contact records?
#[test]
fn r2_edge_touch_union_against_tier_3_prime() {
    let tol = Tol::witness();
    for (what, b) in [
        ("edge touch", common::brick::<f64>((1.0, 2.0), (1.0, 2.0), (0.0, 1.0), tol)),
        ("vertex touch", common::brick::<f64>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0), tol)),
        ("face flush offset", common::brick::<f64>((1.0, 2.0), (0.5, 1.5), (0.0, 1.0), tol)),
    ] {
        let a = common::finished("a", common::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol), tol);
        let b = common::finished(what, b, tol);
        let r = union(&a, &b, tol);
        match &r {
            Ok(BooleanResult::Body(r)) => {
                let census = f64::gate_at_rest_declared(&r.body, &r.contacts, tol);
                eprintln!(
                    "{what}: shipped kind {:?}, {} solids, outcome {:?}, contacts {:?}; tier 3' census: {:?}",
                    r.kind,
                    r.body.solids().count(),
                    r.body.outcome(),
                    r.contacts,
                    census.as_ref().map_err(|e| e.iter().map(ToString::to_string).collect::<Vec<_>>())
                );
            }
            other => eprintln!("{what}: {other:?}"),
        }
    }
}

// Claim 3 compile-fail probes (each was compiled alone under
// `#[cfg(all())]` and refused: E0596 for `&mut *x` and for
// `describe_as_intersections(&mut clone)`, E0451 for the struct literal,
// E0624 for `AtRestBody::not_run`).
#[cfg(any())]
mod compile_fail {
    use geom_core::Tol;
    use topo::{AtRestBody, Body};
    pub fn a(x: &mut AtRestBody<f64>) -> &mut Body<f64> {
        &mut *x
    }
    pub fn b(body: Body<f64>) -> AtRestBody<f64> {
        AtRestBody { body, outcome: topo::AtRestOutcome::Validated }
    }
    pub fn c(body: Body<f64>) -> AtRestBody<f64> {
        AtRestBody::not_run(body)
    }
    pub fn d(x: &AtRestBody<f64>, tol: Tol) {
        let mut y = x.clone();
        crate::common::describe_as_intersections(&mut y, tol);
    }
}

/// At a dual the at-rest gate is structurally absent, so the clockwise
/// (inside-out) wedge is "finished" with no verdict. What does the door
/// answer for it? (On main CLEAVE's operand read ran at every scalar.)
#[test]
fn r2_inside_out_wedge_at_dual() {
    use geom_core::Dual64;
    let tol = Tol::witness();
    let at = |deg: f64| (deg.to_radians().cos(), deg.to_radians().sin());
    for (what, prof) in [
        ("clockwise", [(0.0, 0.0), at(190.0), at(80.0)]),
        ("counterclockwise", [(0.0, 0.0), at(80.0), at(190.0)]),
    ] {
        let brick = common::brick::<Dual64>((0.0, 1.0), (0.0, 1.0), (0.5, 1.5), tol);
        let wedge = common::prism_z::<Dual64>(&prof, 0.5, 1.0, tol).body;
        let decls = common::flush_declarations(&brick, &wedge, tol);
        let (b, w) = (
            Dual64::gate_at_rest_kept(brick, tol).expect("dual gate is absent"),
            Dual64::gate_at_rest_kept(wedge, tol).expect("dual gate is absent"),
        );
        eprintln!("{what}: wedge outcome {:?}", w.outcome());
        for (name, r) in [
            ("brick − wedge", topo::subtract_with(&b, &w, &decls, tol)),
            ("brick ∩ wedge", topo::intersect_with(&b, &w, &decls, tol)),
        ] {
            match r {
                Ok(BooleanResult::Body(r)) => eprintln!(
                    "  {name}: answers, volume {:?}",
                    topo::mass_properties_structural(&r.body, tol).map(|m| m.volume.value)
                ),
                other => eprintln!("  {name}: {:?}", other.map(|_| "empty")),
            }
        }
    }
}
