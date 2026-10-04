//! Review probes (PR 4012, delta lane at dabce2de30). Not for merge.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use crate::shared::tol::{band, eps};
use geom::{NurbsSurface, Surface};
use geom_brep::ssi::{self, SsiDomain};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

fn plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// `z = k·x + h(y)` over `[0, X] × [0, Y]`: degree 1 in u (x), degree 2
/// in v (y) with `hs.len()` = 2m+1 control heights over m C0 quadratic
/// spans (interior knots doubled), y linear in v.
fn c0_wall(k: f64, xm: f64, ym: f64, hs: &[f64]) -> NurbsSurface<f64> {
    let n = hs.len();
    assert!(n % 2 == 1);
    let m = (n - 1) / 2;
    let mut kv = vec![0.0, 0.0, 0.0];
    for i in 1..m {
        let t = i as f64 / m as f64;
        kv.push(t);
        kv.push(t);
    }
    kv.extend([1.0, 1.0, 1.0]);
    let kvv = KnotVector::clamped(kv, 2).unwrap();
    let kvu = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let mut control = Vec::new();
    for i in 0..2 {
        let x = if i == 0 { 0.0 } else { xm };
        for (j, h) in hs.iter().enumerate() {
            let y = ym * j as f64 / (n - 1) as f64;
            control.push(Point3::new(x, y, k * x + h));
        }
    }
    NurbsSurface::new(kvu, kvv, control, vec![1.0; 2 * n]).unwrap()
}

fn declared(
    wall: &NurbsSurface<f64>,
    from: (f64, f64),
    to: (f64, f64),
) -> Result<geom_brep::PlaneNurbsLimbs<f64>, geom_brep::PlaneNurbsRefusal> {
    let carrier = crate::shared::fixture::segment(
        Point3::new(from.0, from.1, 0.0),
        Point3::new(to.0, to.1, 0.0),
    );
    geom_brep::plane_nurbs_limbs::<f64>(&carrier, &plane(), wall, 1.0, band())
}

fn search(wall: &NurbsSurface<f64>, c: (f64, f64), half: f64) -> String {
    let domain = SsiDomain {
        center: Point3::new(c.0, c.1, 0.0),
        half_extent: half,
        extent: 1.0,
        floor_scale: 1.0,
    };
    match ssi::plane_nurbs_ssi(&plane(), wall, domain, band()) {
        Ok(out) => format!(
            "{} branches {:?}; boundary {}",
            out.branches.len(),
            out.branches
                .iter()
                .map(|b| format!("{:?}", b.end))
                .collect::<Vec<_>>(),
            format!("{:?}", out.boundary).chars().take(500).collect::<String>()
        ),
        Err(e) => format!("ERR {e}"),
    }
}

fn show(got: &Result<geom_brep::PlaneNurbsLimbs<f64>, geom_brep::PlaneNurbsRefusal>) -> String {
    match got {
        Ok(_) => "OK".to_string(),
        Err(e) => format!("Err {e:?}").chars().take(240).collect(),
    }
}

/// Alternating `P, −N, P, …, P` over m spans: each span's Bernstein
/// coefficients are `(P, −N, P)`, the curve's minimum `(P − N)/2 > 0`.
fn loose(m: usize, p: f64, n: f64) -> Vec<f64> {
    (0..=2 * m).map(|j| if j % 2 == 0 { p } else { -n }).collect()
}

/// **A loose-hull phantom on a side.** φ along x = 0 is ≥ (P−N)/2 > 0
/// everywhere and the wall rises inward: the exact locus over x ≥ 0 is
/// EMPTY. But each span's Bernstein hull holds −N, so no piece of any
/// stretch is certified one-signed and no piece reads clear.
#[test]
fn d_loose_phantom() {
    let e = eps();
    for (k, m, pk, nk) in [
        (10.0, 256, 0.6, 0.2),
        (10.0, 1024, 0.6, 0.2),
        (10.0, 256, 0.9, 0.5),
        (100.0, 256, 0.6, 0.2),
        (10.0, 64, 0.6, 0.2),
    ] {
        let (p, n) = (pk * e, nk * e);
        let w = c0_wall(k, 1.0, 1.0, &loose(m, p, n));
        let got = declared(&w, (0.0, 0.0), (0.0, 1.0));
        let s = search(&w, (0.5, 0.5), 1.0);
        eprintln!(
            "DPROBE loose_phantom eps={e:e} k={k} m={m} P={pk}e N={nk}e min φ={:.2}e: at rest {} || search {s}",
            (pk - nk) / 2.0,
            show(&got)
        );
    }
}

/// The same geometry as R2's `h ≡ 0.4ε` phantom, but with the side's
/// constant height written over many spans (tight hull): control
/// for reference — must refuse.
#[test]
fn d_tight_phantom_control() {
    let e = eps();
    let w = c0_wall(10.0, 1.0, 1.0, &vec![0.4 * e; 513]);
    eprintln!(
        "DPROBE tight_phantom eps={e:e}: at rest {} || search {}",
        show(&declared(&w, (0.0, 0.0), (0.0, 1.0))),
        search(&w, (0.5, 0.5), 1.0)
    );
}

/// **A loose-hull gap between two flush stretches.** h = 0 exactly on
/// y ∈ [0, 0.3] and [0.7, 1] (locus is the side there), loose-positive
/// (≥ (P−N)/2) on (0.3, 0.7): the exact locus is two stretches of the
/// side, 0.4 apart. The carrier along the whole side joins them.
#[test]
fn d_loose_gap() {
    let e = eps();
    let m = 1000;
    for (pk, nk) in [(0.6, 0.2), (0.9, 0.5)] {
        let hs: Vec<f64> = (0..=2 * m)
            .map(|j| {
                let y = j as f64 / (2 * m) as f64;
                if !(0.3..=0.7).contains(&y) {
                    0.0
                } else if j % 2 == 0 {
                    pk * e
                } else {
                    -nk * e
                }
            })
            .collect();
        let w = c0_wall(10.0, 1.0, 1.0, &hs);
        eprintln!(
            "DPROBE loose_gap eps={e:e} P={pk}e N={nk}e: at rest whole side {} | first stretch only {} || search {}",
            show(&declared(&w, (0.0, 0.0), (0.0, 1.0))),
            show(&declared(&w, (0.0, 0.0), (0.0, 0.3))),
            search(&w, (0.5, 0.5), 1.0)
        );
    }
}

/// **A flush side whose flush stretch ends mid-side** (h = 0 on
/// [0, ½], then rising to 0.6ε by a tight one-signed quadratic; the
/// wall rises inward, so beyond ½ the side is clear). Carriers along
/// the side to y = 0.5 (exact), 0.6, 1.0 (overrun), and past the domain.
#[test]
fn d_flush_end_overrun() {
    let e = eps();
    let m = 64;
    let hs: Vec<f64> = (0..=2 * m)
        .map(|j| {
            let y = j as f64 / (2 * m) as f64;
            if y <= 0.5 { 0.0 } else { 0.6 * e * ((y - 0.5) / 0.5).powi(2) }
        })
        .collect();
    let w = c0_wall(10.0, 1.0, 1.0, &hs);
    for to in [0.5, 0.5 + 1e-3, 0.6, 1.0] {
        eprintln!(
            "DPROBE flush_end eps={e:e} carrier y 0..{to}: {}",
            show(&declared(&w, (0.0, 0.0), (0.0, to)))
        );
    }
    // fully flush side, carrier past the domain's end
    let w2 = c0_wall(10.0, 1.0, 1.0, &vec![0.0; 129]);
    for to in [1.0, 1.0 + 1e-3, 1.2] {
        eprintln!(
            "DPROBE flush_past_domain eps={e:e} carrier y 0..{to}: {}",
            show(&declared(&w2, (0.0, 0.0), (0.0, to)))
        );
    }
    eprintln!("DPROBE flush_end search: {}", search(&w, (0.5, 0.5), 1.0));
}

/// **Twisted, conical and rational walls, a phantom and a near-flush
/// side.** Biquadratic/bilinear nets.
#[test]
fn d_twisted_conical_rational() {
    use crate::shared::fixture::segment;
    let e = eps();
    // Twisted: z = x + x·y + c over [0,1]², bilinear.
    let k1 = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    for (name, c) in [("flush", 0.0), ("phantom +0.4e", 0.4 * e), ("below -0.4e", -0.4 * e)] {
        let control = vec![
            Point3::new(0.0, 0.0, c),
            Point3::new(0.0, 1.0, c),
            Point3::new(1.0, 0.0, 1.0 + c),
            Point3::new(1.0, 1.0, 2.0 + c),
        ];
        let w = NurbsSurface::new(k1(), k1(), control, vec![1.0; 4]).unwrap();
        eprintln!(
            "DPROBE twisted {name} eps={e:e}: at rest {} || search {}",
            show(&declared(&w, (0.0, 0.0), (0.0, 1.0))),
            search(&w, (0.5, 0.5), 1.0)
        );
    }
    // Twisted with the across slope changing sign along the side:
    // z = x·(y − ½) + c, c = 0.4ε.
    let control = vec![
        Point3::new(0.0, 0.0, 0.4 * e),
        Point3::new(0.0, 1.0, 0.4 * e),
        Point3::new(1.0, 0.0, -0.5 + 0.4 * e),
        Point3::new(1.0, 1.0, 0.5 + 0.4 * e),
    ];
    let w = NurbsSurface::new(k1(), k1(), control, vec![1.0; 4]).unwrap();
    eprintln!(
        "DPROBE twisted_signflip eps={e:e}: at rest {} || search {}",
        show(&declared(&w, (0.0, 0.0), (0.0, 1.0))),
        search(&w, (0.5, 0.5), 1.0)
    );
    // Conical (rational) frustum, plane y = d: d = 0 flush, d = −0.4ε
    // phantom (wall at y ≥ 0 strictly above), d = +0.4ε locus just inside.
    for r1 in [1.1, 2.0] {
        let control = vec![
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(r1, 0.0, 1.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(r1, r1, 1.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(0.0, r1, 1.0),
        ];
        let wq = core::f64::consts::FRAC_1_SQRT_2;
        let kv2 = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let kv1 = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let wall = NurbsSurface::new(kv2, kv1, control, vec![1.0, 1.0, wq, wq, 1.0, 1.0]).unwrap();
        for d in [0.0, -0.4 * e, 0.4 * e] {
            let pl = Surface::Plane {
                origin: Point3::new(0.0, d, 0.0),
                normal: Vec3::new(0.0, 1.0, 0.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            };
            let carrier = segment(Point3::new(1.0, d, 0.0), Point3::new(r1, d, 1.0));
            let got = geom_brep::plane_nurbs_limbs::<f64>(&carrier, &pl, &wall, 1.0, band());
            let dom = SsiDomain {
                center: Point3::new(0.5, 0.5, 0.5),
                half_extent: 2.5,
                extent: 1.0,
                floor_scale: 1.0,
            };
            let s = match ssi::plane_nurbs_ssi(&pl, &wall, dom, band()) {
                Ok(out) => format!(
                    "{} branches; boundary {}",
                    out.branches.len(),
                    format!("{:?}", out.boundary).chars().take(300).collect::<String>()
                ),
                Err(err) => format!("ERR {err}"),
            };
            eprintln!(
                "DPROBE frustum r1={r1} plane y={:.1}e eps={e:e}: at rest {} || search {s}",
                d / e,
                show(&got)
            );
        }
    }
}

/// Scan of the loose-hull phantom's amplitude against limb 2.
#[test]
fn d_loose_scan() {
    let e = eps();
    for m in [64usize, 128] {
        for (pk, nk) in [(0.3, 0.25), (0.15, 0.1), (0.6, 0.5), (0.08, 0.06), (0.9, 0.85)] {
            for k in [10.0, 1.0] {
                let w = c0_wall(k, 1.0, 1.0, &loose(m, pk * e, nk * e));
                let got = declared(&w, (0.0, 0.0), (0.0, 1.0));
                eprintln!(
                    "DPROBE scan eps={e:e} k={k} m={m} P={pk}e N={nk}e min φ={:.3}e: at rest {}",
                    (pk - nk) / 2.0,
                    show(&got)
                );
            }
        }
    }
}

/// **The cover's verdict behind the `beyond_reach` screen.** `z = k·x +
/// h(y)`, h ≤ 0 over m C0 spans with Bernstein (0, −a, 0): zero at every
/// knot i/m, dipping to −a/2 between, so the locus wanders a/(2k) from
/// the side and back m times. `beyond_reach` samples a window's stretch
/// at its ends and middle only: where those land on knots it passes, and
/// only `side_cover` refuses. Run under REV_MUT=EQA to remove the cover.
#[test]
fn d_cover_behind_screen() {
    let e = eps();
    for m in [4usize, 8, 16, 32] {
        for (k, ak) in [(1e-3, 0.9), (1e-2, 0.9), (1e-3, 0.3)] {
            let hs: Vec<f64> = (0..=2 * m).map(|j| if j % 2 == 0 { 0.0 } else { -ak * e }).collect();
            let w = c0_wall(k, 1.0, 1.0, &hs);
            eprintln!(
                "DPROBE cover_screen eps={e:e} mut={:?} m={m} k={k} a={ak}e wander={:.0}e: at rest {}",
                std::env::var("REV_MUT").ok(),
                ak / 2.0 / k,
                show(&declared(&w, (0.0, 0.0), (0.0, 1.0)))
            );
        }
    }
}

/// Every cause's rendering at both doors, with word counts.
#[test]
fn d_endings() {
    use geom_brep::recourse::Reading;
    use geom_brep::ssi::{OneArcDoor, OneArcRefusal};
    use geom_core::{Band, Indeterminate, MarginDiag};
    let band = Band::new(1e-9, 1e-8).unwrap();
    let und = OneArcRefusal::Undecided(Indeterminate {
        margin: MarginDiag::INVALID,
        terminal_sliver: false,
        band,
        predicate: Some("ssi_tube_one_arc"),
    });
    for c in [
        OneArcRefusal::Count { solutions: 0 },
        OneArcRefusal::Count { solutions: 4 },
        OneArcRefusal::Unlinked,
        OneArcRefusal::Short,
        und,
    ] {
        eprintln!("DEND cause {c:?}\n  display: {c}");
        for door in [OneArcDoor::Search, OneArcDoor::AtRest] {
            for r in [Reading::Build, Reading::AtRest, Reading::Adopt] {
                let s = c.ending(door, r);
                eprintln!("  {door:?}/{r:?} [{}w]: {s}", s.split_whitespace().count());
            }
        }
        let p = geom_brep::PlaneNurbsRefusal::TubeNotOneArc { rungs: 20, cause: c };
        let full = format!("{p} {}", p.ending(Reading::AtRest).unwrap_or_default());
        eprintln!("  at-rest full [{}w]: {full}", full.split_whitespace().count());
    }
}
