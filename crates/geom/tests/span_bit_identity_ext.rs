//! **Bit identity, the extended corpus** — adopted verbatim in
//! substance from the review lane that re-derived it independently at
//! the merge base: rational curves and surfaces with interior
//! multiplicities (double and degree-high), degree 1 and 4, the
//! `Interval` lane (multi-span `SpanSet` hulls straddling knots), the
//! `Dual` lane, the whole-domain hull doors, and `ders_basis_funs` rows
//! with `k > p`.
//!
//! 11,151 `f64` values by their bits. The expectation is not
//! self-generated: `DIGEST` was measured by building this corpus at the
//! merge base against the retired `(kv, span)` spellings, and it is
//! unchanged here.
//!
//! **Interval-gated**, because half its rows are `Interval` rows: it
//! runs in the six `interval` test jobs. The default lane's bit
//! identity is `span_bit_identity.rs`, whose 1001 rows carry no feature
//! gate.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom::{NurbsCurve3, NurbsSurface};
use geom_core::spline::{CoeffWindow, KnotVector, RationalWindow, Span, basis};
use geom_core::{Bounds, Dual64, Interval, Point3, Real, Vec3};

// The doors, one name each. The review lane's build put its
// base-vs-head spelling seam here; what the seam proved is now the
// `DIGEST` below, and these stay because they keep the corpus body
// readable.
fn bf<T: Real>(s: Span<'_>, t: T) -> Vec<T> {
    basis::basis_funs(s, t)
}
fn dbf<T: Real>(s: Span<'_>, t: T, n: usize) -> Vec<Vec<T>> {
    basis::ders_basis_funs(s, t, n)
}
fn sh(w: CoeffWindow<'_, f64>) -> geom_core::RingInterval {
    w.hull()
}
fn shr(w: RationalWindow<'_, f64>) -> geom_core::RingInterval {
    w.hull_rational()
}
fn dsh(w: CoeffWindow<'_, f64>) -> geom_core::RingInterval {
    w.derivative_hull()
}
fn snb(w: CoeffWindow<'_, f64>) -> f64 {
    w.sup_norm_bound()
}

type Rows = Vec<(String, u64)>;
fn kv(k: &[f64], p: usize) -> KnotVector {
    KnotVector::clamped(k.to_vec(), p).unwrap()
}
fn curve(k: KnotVector, rational: bool) -> NurbsCurve3<f64> {
    let n = k.control_count();
    let control: Vec<Point3<f64>> = (0..n)
        .map(|i| {
            Point3::new(
                i as f64 * 0.7 - 1.3,
                ((i * 3) % 5) as f64 * 0.45,
                ((i * 7) % 4) as f64 * -0.6,
            )
        })
        .collect();
    let weights: Vec<f64> = (0..n)
        .map(|i| {
            if rational {
                0.4 + ((i * 2) % 5) as f64 * 0.3
            } else {
                1.0
            }
        })
        .collect();
    NurbsCurve3::new(k, control, weights).unwrap()
}
fn lift<T: Real>(c: &NurbsCurve3<f64>) -> NurbsCurve3<T> {
    NurbsCurve3::new(
        c.knots().clone(),
        c.control()
            .iter()
            .map(|p| Point3::new(T::from_f64(p.x), T::from_f64(p.y), T::from_f64(p.z)))
            .collect(),
        c.weights().to_vec(),
    )
    .unwrap()
}
fn lift_s<T: Real>(s: &NurbsSurface<f64>) -> NurbsSurface<T> {
    NurbsSurface::new(
        s.knots_u().clone(),
        s.knots_v().clone(),
        s.control()
            .iter()
            .map(|p| Point3::new(T::from_f64(p.x), T::from_f64(p.y), T::from_f64(p.z)))
            .collect(),
        s.weights().to_vec(),
    )
    .unwrap()
}
fn pf(o: &mut Rows, t: &str, p: Point3<f64>) {
    o.push((format!("{t}.x"), p.x.to_bits()));
    o.push((format!("{t}.y"), p.y.to_bits()));
    o.push((format!("{t}.z"), p.z.to_bits()));
}
fn vf(o: &mut Rows, t: &str, p: Vec3<f64>) {
    o.push((format!("{t}.x"), p.x.to_bits()));
    o.push((format!("{t}.y"), p.y.to_bits()));
    o.push((format!("{t}.z"), p.z.to_bits()));
}
fn pi(o: &mut Rows, t: &str, p: Point3<Interval>) {
    for (n, c) in [("x", p.x), ("y", p.y), ("z", p.z)] {
        o.push((format!("{t}.{n}.lo"), c.lo().to_bits()));
        o.push((format!("{t}.{n}.hi"), c.hi().to_bits()));
    }
}
fn vi(o: &mut Rows, t: &str, p: Vec3<Interval>) {
    for (n, c) in [("x", p.x), ("y", p.y), ("z", p.z)] {
        o.push((format!("{t}.{n}.lo"), c.lo().to_bits()));
        o.push((format!("{t}.{n}.hi"), c.hi().to_bits()));
    }
}
fn pd(o: &mut Rows, t: &str, p: Point3<Dual64>) {
    for (n, c) in [("x", p.x), ("y", p.y), ("z", p.z)] {
        o.push((format!("{t}.{n}.v"), c.value.to_bits()));
        o.push((format!("{t}.{n}.d"), c.deriv.to_bits()));
    }
}
fn vd(o: &mut Rows, t: &str, p: Vec3<Dual64>) {
    for (n, c) in [("x", p.x), ("y", p.y), ("z", p.z)] {
        o.push((format!("{t}.{n}.v"), c.value.to_bits()));
        o.push((format!("{t}.{n}.d"), c.deriv.to_bits()));
    }
}
fn ri(o: &mut Rows, t: &str, r: geom_core::RingInterval) {
    o.push((format!("{t}.lo"), r.lo().to_bits()));
    o.push((format!("{t}.hi"), r.hi().to_bits()));
}

fn rows() -> Rows {
    let mut o = Rows::new();
    let corpus: Vec<(&str, KnotVector, bool)> = vec![
        (
            "c0cubic",
            kv(&[0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0], 3),
            true,
        ),
        (
            "quart",
            kv(
                &[
                    0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 1.5, 2.0, 2.0, 2.0, 2.0, 2.0,
                ],
                4,
            ),
            true,
        ),
        ("lin", kv(&[0.0, 0.0, 1.0, 2.0, 3.0, 3.0], 1), false),
        (
            "quadm",
            kv(&[0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2),
            true,
        ),
    ];
    for (name, k, rational) in corpus {
        let c = curve(k.clone(), rational);
        let cd: NurbsCurve3<Dual64> = lift(&c);
        let ci: NurbsCurve3<Interval> = lift(&c);
        let n = k.control_count();
        let coeffs: Vec<f64> = (0..n).map(|i| ((i * 3) % 7) as f64 * 0.5 - 1.25).collect();
        let w: Vec<f64> = c.weights().to_vec();
        let pair = k
            .with_coeffs(&coeffs)
            .expect("minted against its own vector");
        let rpair = k
            .with_rational_coeffs(&coeffs, &w)
            .expect("minted against its own vector");
        // parameters: every knot value, every span midpoint, both domain ends
        let mut ts: Vec<f64> = k.knots().to_vec();
        for i in k.first_span()..=k.last_span() {
            ts.push(0.5 * (k.knots()[i] + k.knots()[i + 1]));
        }
        ts.dedup();
        for (ti, t) in ts.iter().enumerate() {
            let t = *t;
            pf(&mut o, &format!("{name}.eval@{ti}"), c.eval(t));
            vf(&mut o, &format!("{name}.deriv@{ti}"), c.deriv(t));
            vf(&mut o, &format!("{name}.deriv2@{ti}"), c.deriv2(t));
            let d = Dual64::variable(t);
            pd(&mut o, &format!("{name}.dual.eval@{ti}"), cd.eval(d));
            vd(&mut o, &format!("{name}.dual.deriv@{ti}"), cd.deriv(d));
            let iv = Interval::from_bounds(t - 0.05, t + 0.05);
            pi(&mut o, &format!("{name}.iv.eval@{ti}"), ci.eval(iv));
            vi(&mut o, &format!("{name}.iv.deriv@{ti}"), ci.deriv(iv));
            vi(&mut o, &format!("{name}.iv.deriv2@{ti}"), ci.deriv2(iv));
        }
        for index in k.first_span()..=k.last_span() {
            let Some(win) = c.span(index) else { continue };
            let span = win.span();
            for (pi_, t) in [
                k.knots()[index],
                0.5 * (k.knots()[index] + k.knots()[index + 1]),
            ]
            .into_iter()
            .enumerate()
            {
                let tag = format!("{name}.s{index}p{pi_}");
                pf(&mut o, &format!("{tag}.eval_in_span"), win.eval_in_span(t));
                let (p, d, dd) = win.ders_in_span(t);
                pf(&mut o, &format!("{tag}.ders.p"), p);
                vf(&mut o, &format!("{tag}.ders.d"), d);
                vf(&mut o, &format!("{tag}.ders.dd"), dd);
                let (p1, d1) = win.ders1_in_span(t);
                pf(&mut o, &format!("{tag}.ders1.p"), p1);
                vf(&mut o, &format!("{tag}.ders1.d"), d1);
                vf(
                    &mut o,
                    &format!("{tag}.deriv_in_span"),
                    win.deriv_in_span(t),
                );
                vf(
                    &mut o,
                    &format!("{tag}.deriv2_in_span"),
                    win.deriv2_in_span(t),
                );
                // the same span doors at Dual and Interval scalars
                let wind = cd
                    .span(index)
                    .expect("same knot vector, same nonempty spans");
                pd(
                    &mut o,
                    &format!("{tag}.dual.eval_in_span"),
                    wind.eval_in_span(Dual64::variable(t)),
                );
                let (pdd, ddd, _) = wind.ders_in_span(Dual64::variable(t));
                pd(&mut o, &format!("{tag}.dual.ders.p"), pdd);
                vd(&mut o, &format!("{tag}.dual.ders.d"), ddd);
                let iv = Interval::from_bounds(t, t + 0.01);
                let wini = ci
                    .span(index)
                    .expect("same knot vector, same nonempty spans");
                pi(
                    &mut o,
                    &format!("{tag}.iv.eval_in_span"),
                    wini.eval_in_span(iv),
                );
                let (pii, dii, ddii) = wini.ders_in_span(iv);
                pi(&mut o, &format!("{tag}.iv.ders.p"), pii);
                vi(&mut o, &format!("{tag}.iv.ders.d"), dii);
                vi(&mut o, &format!("{tag}.iv.ders.dd"), ddii);
                for (j, b) in bf::<f64>(span, t).iter().enumerate() {
                    o.push((format!("{tag}.basis.{j}"), b.to_bits()));
                }
                for (r, row) in dbf::<f64>(span, t, 3).iter().enumerate() {
                    for (j, b) in row.iter().enumerate() {
                        o.push((format!("{tag}.dbasis.{r}.{j}"), b.to_bits()));
                    }
                }
                for (j, b) in bf::<Dual64>(span, Dual64::variable(t)).iter().enumerate() {
                    o.push((format!("{tag}.basis.dual.{j}.v"), b.value.to_bits()));
                    o.push((format!("{tag}.basis.dual.{j}.d"), b.deriv.to_bits()));
                }
                for (j, b) in bf::<Interval>(span, iv).iter().enumerate() {
                    o.push((format!("{tag}.basis.iv.{j}.lo"), b.lo().to_bits()));
                    o.push((format!("{tag}.basis.iv.{j}.hi"), b.hi().to_bits()));
                }
            }
            let tag = format!("{name}.s{index}");
            let cw = pair.span(index).expect("the curve's span is the vector's");
            let rw = rpair.span(index).expect("the same span");
            ri(&mut o, &format!("{tag}.span_hull"), sh(cw));
            ri(&mut o, &format!("{tag}.span_hull_rational"), shr(rw));
            ri(&mut o, &format!("{tag}.derivative_span_hull"), dsh(cw));
            o.push((format!("{tag}.sup_norm_bound_span"), snb(cw).to_bits()));
        }
        ri(&mut o, &format!("{name}.domain_hull"), pair.domain_hull());
        ri(
            &mut o,
            &format!("{name}.domain_hull_rational"),
            rpair.domain_hull_rational(),
        );
        ri(
            &mut o,
            &format!("{name}.derivative_domain_hull"),
            pair.derivative_domain_hull(),
        );
        o.push((
            format!("{name}.sup_norm_bound"),
            pair.sup_norm_bound().to_bits(),
        ));
        o.push((
            format!("{name}.sup_norm_bound_rational"),
            rpair.sup_norm_bound_rational().to_bits(),
        ));
    }
    // ---- surfaces ----
    let surfs: Vec<(&str, KnotVector, KnotVector)> = vec![
        (
            "s_mult",
            kv(&[0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0], 3),
            kv(&[0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 2.0, 2.0], 2),
        ),
        (
            "s_mixed",
            kv(&[0.0, 0.0, 1.0, 2.0, 3.0, 3.0], 1),
            kv(
                &[0.0, 0.0, 0.0, 0.0, 0.0, 0.3, 0.3, 1.0, 1.0, 1.0, 1.0, 1.0],
                4,
            ),
        ),
    ];
    for (name, ku, kvv) in surfs {
        let (nu, nv) = (ku.control_count(), kvv.control_count());
        let control: Vec<Point3<f64>> = (0..nu * nv)
            .map(|f| {
                let (iu, iv) = (f / nv, f % nv);
                Point3::new(
                    iu as f64 * 1.1 + iv as f64 * 0.05,
                    iv as f64 * 0.9 - iu as f64 * 0.02,
                    (((iu * 5 + iv * 3) % 7) as f64) * 0.4 - 0.8,
                )
            })
            .collect();
        let weights: Vec<f64> = (0..nu * nv)
            .map(|f| 0.6 + ((f * 3) % 5) as f64 * 0.2)
            .collect();
        let s = NurbsSurface::new(ku.clone(), kvv.clone(), control, weights).unwrap();
        let sd: NurbsSurface<Dual64> = lift_s(&s);
        let si: NurbsSurface<Interval> = lift_s(&s);
        let mut us: Vec<f64> = ku.knots().to_vec();
        for i in ku.first_span()..=ku.last_span() {
            us.push(0.5 * (ku.knots()[i] + ku.knots()[i + 1]));
        }
        us.dedup();
        let mut vs: Vec<f64> = kvv.knots().to_vec();
        for i in kvv.first_span()..=kvv.last_span() {
            vs.push(0.5 * (kvv.knots()[i] + kvv.knots()[i + 1]));
        }
        vs.dedup();
        for (ui, u) in us.iter().enumerate() {
            for (vi_, v) in vs.iter().enumerate() {
                let (u, v) = (*u, *v);
                let tag = format!("{name}.u{ui}v{vi_}");
                pf(&mut o, &format!("{tag}.eval"), s.eval(u, v));
                let j = s.ders(u, v);
                pf(&mut o, &format!("{tag}.ders.p"), j.point);
                vf(&mut o, &format!("{tag}.ders.du"), j.du);
                vf(&mut o, &format!("{tag}.ders.dv"), j.dv);
                vf(&mut o, &format!("{tag}.ders.duu"), j.duu);
                vf(&mut o, &format!("{tag}.ders.duv"), j.duv);
                vf(&mut o, &format!("{tag}.ders.dvv"), j.dvv);
                let j3 = s.ders3(u, v);
                vf(&mut o, &format!("{tag}.ders3.duuu"), j3.duuu);
                vf(&mut o, &format!("{tag}.ders3.duuv"), j3.duuv);
                vf(&mut o, &format!("{tag}.ders3.duvv"), j3.duvv);
                vf(&mut o, &format!("{tag}.ders3.dvvv"), j3.dvvv);
                let jd = sd.ders(Dual64::variable(u), Dual64::new(v, 0.5));
                pd(&mut o, &format!("{tag}.dual.ders.p"), jd.point);
                vd(&mut o, &format!("{tag}.dual.ders.duv"), jd.duv);
                let (iu, iv) = (
                    Interval::from_bounds(u - 0.04, u + 0.04),
                    Interval::from_bounds(v - 0.04, v + 0.04),
                );
                pi(&mut o, &format!("{tag}.iv.eval"), si.eval(iu, iv));
                let ji = si.ders(iu, iv);
                vi(&mut o, &format!("{tag}.iv.ders.du"), ji.du);
                vi(&mut o, &format!("{tag}.iv.ders.dvv"), ji.dvv);
                let j3i = si.ders3(iu, iv);
                vi(&mut o, &format!("{tag}.iv.ders3.duvv"), j3i.duvv);
                let win = s.window_at(u, v);
                pf(&mut o, &format!("{tag}.win.eval"), win.eval_in_span(u, v));
                let jw = win.ders_in_span(u, v);
                pf(&mut o, &format!("{tag}.win.ders.p"), jw.point);
                vf(&mut o, &format!("{tag}.win.ders.duv"), jw.duv);
                let j3w = win.ders3_in_span(u, v);
                vf(&mut o, &format!("{tag}.win.ders3.duuv"), j3w.duuv);
                vf(&mut o, &format!("{tag}.win.ders3.dvvv"), j3w.dvvv);
                let wind = sd.window_at(u, v);
                pd(
                    &mut o,
                    &format!("{tag}.win.dual.eval"),
                    wind.eval_in_span(Dual64::variable(u), Dual64::new(v, 0.5)),
                );
                let wini = si.window_at(u, v);
                pi(
                    &mut o,
                    &format!("{tag}.win.iv.eval"),
                    wini.eval_in_span(
                        Interval::from_bounds(u, u + 0.01),
                        Interval::from_bounds(v, v + 0.01),
                    ),
                );
                let jwi = wini.ders_in_span(
                    Interval::from_bounds(u, u + 0.01),
                    Interval::from_bounds(v, v + 0.01),
                );
                vi(&mut o, &format!("{tag}.win.iv.ders.du"), jwi.du);
            }
        }
        for su in ku.first_span()..=ku.last_span() {
            for sv in kvv.first_span()..=kvv.last_span() {
                let Some(win) = s.window(su, sv) else {
                    continue;
                };
                let (u, v) = (
                    0.5 * (ku.knots()[su] + ku.knots()[su + 1]),
                    0.5 * (kvv.knots()[sv] + kvv.knots()[sv + 1]),
                );
                let tag = format!("{name}.cell{su}{sv}");
                pf(&mut o, &format!("{tag}.eval"), win.eval_in_span(u, v));
                let j3w = win.ders3_in_span(u, v);
                vf(&mut o, &format!("{tag}.ders3.duuu"), j3w.duuu);
                vf(&mut o, &format!("{tag}.jet.dv"), j3w.jet.dv);
            }
        }
    }
    o
}

fn digest(rows: &[(String, u64)]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for (name, bits) in rows {
        for byte in format!("{name} {bits:#018x}\n").bytes() {
            h ^= u64::from(byte);
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    h
}

/// The row count, pinned so a corpus that stops covering a door
/// cannot leave the digest green by producing a shorter stream.
const ROW_COUNT: usize = 11_151;

/// FNV-1a 64 over `"{label} {bits:#018x}\n"` for every row in order,
/// measured on the retired `(kv, span)` spellings at the merge base.
const DIGEST: u64 = 0x606f_ae2d_7244_63e4;

#[test]
fn the_extended_corpus_is_bit_identical_to_the_retired_spellings() {
    let r = rows();
    assert_eq!(
        r.len(),
        ROW_COUNT,
        "the corpus changed shape — the digest is a claim about a \
         DIFFERENT stream and proves nothing until it is re-measured"
    );
    assert_eq!(
        digest(&r),
        DIGEST,
        "a value in the extended corpus moved; run with --nocapture to print every row"
    );
    if std::env::var_os("CAD_PRINT_ROWS").is_some() {
        for (n, b) in &r {
            println!("ROW {n} {b:#018x}");
        }
    }
}
