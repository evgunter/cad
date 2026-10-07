//! Review probes for PR 4254 (a link's reach skips the faces at its own
//! two vertices, not its chain's). Each probe prints the door's verdict
//! and, when a body builds, its tier-3 and tier-3' verdicts and a
//! sampled point-membership check against an analytic oracle.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, Point3, Tol, Vec3};
use sweep::blend::{BlendRequest, fillet_edges};
use sweep::test_support::band_reach;
use topo::boolean::{SolidContainment, point_in_solid};
use topo::{Body, EdgeKey, validate_geometric};

use crate::common::cavity::{brick, cut, edges_with_corners, prism};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).expect("the witness band")
}

/// A convex 90° straight band, for the membership oracle: the edge from
/// `a` to `b`, the outward normals of its supports.
struct Convex90 {
    a: Point3<f64>,
    b: Point3<f64>,
    na: Vec3<f64>,
    nb: Vec3<f64>,
    /// How far past `b` the band may run (an oblique cut-off's pad):
    /// points there are not judged.
    pad: f64,
}

impl Convex90 {
    /// `Some(true)` removed, `Some(false)` kept, `None` too near a
    /// boundary of the band (or past its window) to judge.
    fn removed(&self, q: Point3<f64>, r: f64) -> Option<bool> {
        let t = self.b - self.a;
        let len = t.norm();
        let s = (q - self.a).dot(t) / len;
        if s < r || s > len - r {
            return None;
        }
        let u = -(q - self.a).dot(self.na);
        let v = -(q - self.a).dot(self.nb);
        if u < -1e-9 || v < -1e-9 {
            return Some(false);
        }
        let d = ((u - r).powi(2) + (v - r).powi(2)).sqrt();
        let inside_sq = u < r && v < r;
        if (d - r).abs() < 1e-3 || (u - r).abs() < 1e-3 || (v - r).abs() < 1e-3 {
            return None;
        }
        Some(inside_sq && d > r)
    }
}

/// The door's verdict, as a line; on a build, tier 3, tier 3' and the
/// membership check over `bands` (convex 90° bands of the request).
fn door(what: &str, body: &Body<f64>, edges: &[EdgeKey], r: f64, bands: &[Convex90]) -> String {
    let meter = match band_reach(
        &BlendRequest {
            body,
            edges: edges.to_vec(),
            size: r,
        },
        band(),
    ) {
        Ok(()) => "meter: pass".to_string(),
        Err(e) => format!("meter: {e:?}"),
    };
    let out = match fillet_edges(body, edges, r, tol()) {
        Err(e) => format!("door: REFUSED {:?}", e.error),
        Ok(f) => {
            let t3 = validate_geometric(&f.body, tol());
            let t3p =
                topo::validate_pseudomanifold(&f.body, &topo::ContactRecords::default(), tol());
            let mut wrong = Vec::new();
            let mut checked = 0;
            let mut cut = 0;
            if !bands.is_empty() {
                let (lo, hi) = bbox(body);
                let n = 18;
                for i in 0..=n {
                    for j in 0..=n {
                        for k in 0..=n {
                            let q = Point3::new(
                                lo.x + (hi.x - lo.x) * (f64::from(i) + 0.37) / f64::from(n + 1),
                                lo.y + (hi.y - lo.y) * (f64::from(j) + 0.41) / f64::from(n + 1),
                                lo.z + (hi.z - lo.z) * (f64::from(k) + 0.43) / f64::from(n + 1),
                            );
                            let Ok(orig) = point_in_solid(body, q, band(), tol()) else {
                                continue;
                            };
                            if matches!(orig, SolidContainment::OnBoundary) {
                                continue;
                            }
                            let mut gone = false;
                            let mut unsure = false;
                            for b in bands {
                                match b.removed(q, r) {
                                    Some(true) => gone = true,
                                    None => {
                                        let s = (q - b.a).dot((b.b - b.a).normalize());
                                        let u = -(q - b.a).dot(b.na);
                                        let v = -(q - b.a).dot(b.nb);
                                        if u < 2.0 * r
                                            && v < 2.0 * r
                                            && s > -2.0 * r
                                            && s < (b.b - b.a).norm() + 2.0 * r + b.pad
                                        {
                                            unsure = true;
                                        }
                                    }
                                    Some(false) => {}
                                }
                            }
                            if unsure {
                                continue;
                            }
                            let want = matches!(orig, SolidContainment::In) && !gone;
                            cut += usize::from(gone && matches!(orig, SolidContainment::In));
                            let got = point_in_solid(&f.body, q, band(), tol());
                            checked += 1;
                            if !matches!(got, Ok(SolidContainment::In) if want)
                                && !matches!(got, Ok(SolidContainment::Out) if !want)
                            {
                                wrong.push((q, got, want));
                            }
                        }
                    }
                }
            }
            format!(
                "door: BUILT tier3={:?} tier3'={} membership: {checked} checked, {cut} cut, {} wrong {:?}",
                t3.map_err(|e| e.len()),
                if t3p.is_ok() {
                    "ok".to_string()
                } else {
                    format!(
                        "{:?}",
                        t3p.err().map(|e| e.into_iter().take(2).collect::<Vec<_>>())
                    )
                },
                wrong.len(),
                wrong.iter().take(3).collect::<Vec<_>>()
            )
        }
    };
    let line = format!("PROBE {what}: {meter} | {out}");
    eprintln!("{line}");
    line
}

fn bbox(body: &Body<f64>) -> (Point3<f64>, Point3<f64>) {
    let mut lo = Point3::new(f64::MAX, f64::MAX, f64::MAX);
    let mut hi = Point3::new(f64::MIN, f64::MIN, f64::MIN);
    for (_, v) in body.vertices() {
        let p = *body.get_point(v.point).expect("a point");
        lo = Point3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z));
        hi = Point3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z));
    }
    (lo, hi)
}

fn near(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

/// A. A convex band whose plane end face is stepped: the end face at
/// `x = 4` only above `z = zs`, the body running on to `x = 4.5` below.
/// At `r = 0.5` the band's end section spans `z ∈ [1.5, 2]`.
#[test]
fn probe_a_convex_stepped_end_face() {
    let p = Point3::new;
    for zs in [1.7, 1.3] {
        let body = cut(
            "step",
            &brick(p(0.0, 0.0, 0.0), p(4.5, 2.0, 2.0)),
            &brick(p(4.0, -1.0, zs), p(5.0, 3.0, 3.0)),
        );
        let edges = edges_with_corners(&body, |q| {
            near(q.y, 0.0) && near(q.z, 2.0) && q.x < 4.0 + 1e-9
        });
        assert_eq!(edges.len(), 1);
        let bands = [Convex90 {
            a: p(0.0, 0.0, 2.0),
            b: p(4.0, 0.0, 2.0),
            na: Vec3::new(0.0, -1.0, 0.0),
            nb: Vec3::new(0.0, 0.0, 1.0),
            pad: 0.0,
        }];
        door(
            &format!("A convex step zs={zs}"),
            &body,
            &edges,
            0.5,
            &bands,
        );
    }
}

/// B. A concave band (a pocket's floor–wall edge, `r = 1`) whose end
/// plate at `x = 4` steps back to `x = 4.5` past `y = ys`: the band's
/// end section spans `y ∈ [0, 1]`.
#[test]
fn probe_b_concave_stepped_end_face() {
    let p = Point3::new;
    for ys in [0.3, 1.2] {
        let tool = prism(
            &[
                Point2::new(0.0, 0.0),
                Point2::new(4.0, 0.0),
                Point2::new(4.0, ys),
                Point2::new(4.5, ys),
                Point2::new(4.5, 3.5),
                Point2::new(0.0, 3.5),
            ],
            0.0,
            3.5,
        );
        let body = cut(
            "pocket",
            &brick(p(-1.0, -1.0, -1.0), p(5.5, 3.0, 3.0)),
            &tool,
        );
        let edges = edges_with_corners(&body, |q| {
            near(q.y, 0.0) && near(q.z, 0.0) && (-0.5..4.25).contains(&q.x)
        });
        assert_eq!(edges.len(), 1);
        door(&format!("B concave step ys={ys}"), &body, &edges, 1.0, &[]);
    }
}

/// C. The same concave band, its end plate notched: a slot through the
/// plate at `y ∈ [0.4, 0.7]`, `z ∈ [0.05, 0.25]` opening into the band's
/// end section (which reaches `z ≈ 0.134` at `y = 0.5`).
#[test]
fn probe_c_concave_notched_end_face() {
    let p = Point3::new;
    for (y0, z0) in [(0.4, 0.05), (0.4, 0.6)] {
        let pocket = cut(
            "pocket",
            &brick(p(-1.0, -1.0, -1.0), p(5.0, 3.0, 3.0)),
            &brick(p(0.0, 0.0, 0.0), p(4.0, 3.5, 3.5)),
        );
        let body = cut(
            "notch",
            &pocket,
            &brick(p(3.5, y0, z0), p(5.5, y0 + 0.3, z0 + 0.2)),
        );
        let edges = edges_with_corners(&body, |q| {
            near(q.y, 0.0) && near(q.z, 0.0) && (-0.5..4.5).contains(&q.x)
        });
        assert_eq!(edges.len(), 1);
        door(
            &format!("C concave notch y0={y0} z0={z0}"),
            &body,
            &edges,
            1.0,
            &[],
        );
    }
}

/// D. A very oblique cut-off: the end face from `(4, 0)` toward
/// `(4 + k, 1)`, so the pad grows as `k`.
#[test]
fn probe_d_oblique_cut_off_pad() {
    let p = Point3::new;
    for k in [1.0, 3.0, 6.0, 12.0] {
        let body = prism(
            &[
                Point2::new(0.0, 0.0),
                Point2::new(4.0, 0.0),
                Point2::new(4.0 + 3.0 * k, 3.0),
                Point2::new(0.0, 3.0),
            ],
            0.0,
            2.0,
        );
        let edges = edges_with_corners(&body, |q| near(q.y, 0.0) && near(q.z, 2.0));
        assert_eq!(edges.len(), 1);
        let bands = [Convex90 {
            a: p(0.0, 0.0, 2.0),
            b: p(4.0, 0.0, 2.0),
            na: Vec3::new(0.0, -1.0, 0.0),
            nb: Vec3::new(0.0, 0.0, 1.0),
            pad: 0.5 * 3.0 * k,
        }];
        door(&format!("D oblique k={k}"), &body, &edges, 0.5, &bands);
    }
}

/// E. A two-link chain on a trapezoid top `(0,0) (4,0) (4,D) (0,d)`:
/// `L0` the front edge, turning at `(0,0)` into `L1` the left edge, cut
/// off at `(0,d)` by the oblique back face. The back face is at `L1`'s
/// end and at no vertex of `L0`; at small `d` it runs through `L0`'s
/// band near the turn.
#[test]
fn probe_e_oblique_far_end_face_near_a_turn() {
    let p = Point3::new;
    for (d, dd) in [
        (0.3, 3.0),
        (0.45, 1.0),
        (0.6, 3.0),
        (0.8, 0.8),
        (1.2, 1.2),
        (0.55, 0.55),
    ] {
        let body = prism(
            &[
                Point2::new(0.0, 0.0),
                Point2::new(4.0, 0.0),
                Point2::new(4.0, dd),
                Point2::new(0.0, d),
            ],
            0.0,
            2.0,
        );
        let front = edges_with_corners(&body, |q| near(q.y, 0.0) && near(q.z, 2.0));
        let left = edges_with_corners(&body, |q| near(q.x, 0.0) && near(q.z, 2.0));
        let edges: Vec<_> = front.into_iter().chain(left).collect();
        assert_eq!(edges.len(), 2);
        let bands = [
            Convex90 {
                a: p(0.0, 0.0, 2.0),
                b: p(4.0, 0.0, 2.0),
                na: Vec3::new(0.0, -1.0, 0.0),
                nb: Vec3::new(0.0, 0.0, 1.0),
                pad: 0.0,
            },
            Convex90 {
                a: p(0.0, 0.0, 2.0),
                b: p(0.0, d, 2.0),
                na: Vec3::new(-1.0, 0.0, 0.0),
                nb: Vec3::new(0.0, 0.0, 1.0),
                pad: 0.0,
            },
        ];
        let bands: &[Convex90] = if d > 1.0 && near(d, dd) { &bands } else { &[] };
        door(
            &format!("E far end face d={d} D={dd}"),
            &body,
            &edges,
            0.5,
            bands,
        );
    }
}

/// F. The meter at `Interval` against `f64` on the PR's trapezoid and on
/// probe E's turn: the verdict's kind and face must agree.
#[test]
fn probe_f_interval_agrees_with_f64() {
    use crate::common::interval::{iv, p2};
    use geom_core::Interval;
    use profile::{ProfileLoop, RawLoop};
    fn edges_at<T: geom_core::Bounds>(
        body: &Body<T>,
        on: impl Fn([f64; 3]) -> bool,
    ) -> Vec<EdgeKey> {
        let mut out: Vec<EdgeKey> = body
            .edges()
            .filter(|(k, _)| {
                let e = body.get_edge(*k).unwrap();
                let h = body.get_half_edge(e.he_plus).unwrap();
                let end = body.half_edge_end(e.he_plus).unwrap();
                let pt = |v| {
                    let q = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
                    [
                        0.5 * (q.x.lo() + q.x.hi()),
                        0.5 * (q.y.lo() + q.y.hi()),
                        0.5 * (q.z.lo() + q.z.hi()),
                    ]
                };
                on(pt(h.start)) && on(pt(end))
            })
            .map(|(k, _)| k)
            .collect();
        out.sort_unstable();
        out
    }
    let verdict = |e: Result<(), sweep::blend::BlendError>| match e {
        Ok(()) => "pass".to_string(),
        Err(sweep::blend::BlendError::FaceClearance { at, bounded, .. }) => {
            format!("FaceClearance {at:?} bounded={bounded}")
        }
        Err(e) => format!("{e:?}").chars().take(60).collect(),
    };
    let polys: Vec<(String, Vec<(f64, f64)>, bool)> = [0.2, 0.45, 0.6]
        .into_iter()
        .map(|w| {
            (
                format!("trapezoid w={w}"),
                vec![(0.0, 0.0), (4.0, 0.0), (4.0, w), (-1.0, w)],
                true,
            )
        })
        .chain([0.3, 0.45, 0.6].into_iter().map(|d| {
            (
                format!("E d={d}"),
                vec![(0.0, 0.0), (4.0, 0.0), (4.0, 3.0), (0.0, d)],
                false,
            )
        }))
        .collect();
    for (what, poly, trapezoid) in polys {
        let w = poly[2].1.max(poly[3].1);
        let pick = move |q: [f64; 3]| {
            let top = (q[2] - 2.0).abs() < 1e-9;
            if trapezoid {
                top && (q[1] - w).abs() > 1e-9 || top && false
            } else {
                top && (q[1].abs() < 1e-9 || q[0].abs() < 1e-9)
            }
        };
        let pick_tr = move |a: [f64; 3]| (a[2] - 2.0).abs() < 1e-9;
        let fb = prism(
            &poly
                .iter()
                .map(|&(x, y)| Point2::new(x, y))
                .collect::<Vec<_>>(),
            0.0,
            2.0,
        );
        let lp = ProfileLoop::polygon(poly.iter().map(|&(x, y)| p2(x, y)));
        let ib: Body<Interval> = sweep::test_support::extruded(
            sweep::test_support::sketch_at(iv(0.0)),
            vec![lp],
            iv(2.0),
            tol(),
        );
        let (fe, ie) = if trapezoid {
            // the three top edges but the back one
            let fe: Vec<_> = edges_at(&fb, pick_tr)
                .into_iter()
                .filter(|e| !edges_at(&fb, |q| pick_tr(q) && (q[1] - w).abs() < 1e-9).contains(e))
                .collect();
            let ie: Vec<_> = edges_at(&ib, pick_tr)
                .into_iter()
                .filter(|e| !edges_at(&ib, |q| pick_tr(q) && (q[1] - w).abs() < 1e-9).contains(e))
                .collect();
            (fe, ie)
        } else {
            (edges_at(&fb, pick), edges_at(&ib, pick))
        };
        let f = verdict(band_reach(
            &BlendRequest {
                body: &fb,
                edges: fe.clone(),
                size: 0.5,
            },
            band(),
        ));
        let i = verdict(band_reach(
            &BlendRequest {
                body: &ib,
                edges: ie.clone(),
                size: iv(0.5),
            },
            band(),
        ));
        eprintln!(
            "PROBE F {what}: f64 {f} ({} edges) | Interval {i} ({} edges)",
            fe.len(),
            ie.len()
        );
    }
}

/// G. Row `blend-reach-never-meters-the-bands-of-one-chain-against-each-other`:
/// one chain over three supports, `L0` (top, front) turning down `L1`
/// (front, right) and into `L2` (right, bottom). `L0` and `L2` share no
/// support; on a low box their bands come together along `L1`.
#[test]
fn probe_g_one_chains_bands_that_share_no_support() {
    let p = Point3::new;
    for h in [0.6, 0.9, 0.95, 0.98, 0.995, 1.0, 1.005, 1.02, 1.2, 2.0] {
        let body = brick(p(0.0, 0.0, 0.0), p(4.0, 3.0, h));
        let l0 = edges_with_corners(&body, |q| near(q.y, 0.0) && near(q.z, h));
        let l1 = edges_with_corners(&body, |q| near(q.x, 4.0) && near(q.y, 0.0));
        let l2 = edges_with_corners(&body, |q| near(q.x, 4.0) && near(q.z, 0.0));
        let edges: Vec<_> = l0.into_iter().chain(l1).chain(l2).collect();
        assert_eq!(edges.len(), 3);
        door(
            &format!("G one chain three supports h={h}"),
            &body,
            &edges,
            0.5,
            &[],
        );
    }
}

/// H. Row `support-screen-skips-adjacent-boundary-features-wherever-they-approach`:
/// the top of a prism whose line `AB` (requested) runs into an arc at
/// `B` that comes back to within `0.45` of `AB` near `x = 3`, inside a
/// `0.5` setback. The screen skips the pair (they share `B`); the arc's
/// wall is the end face at `B`.
#[test]
fn probe_h_an_arc_that_comes_back_toward_the_line_it_shares_a_vertex_with() {
    for bulge in [1.6, 2.5] {
        let built = std::panic::catch_unwind(|| {
            let lp = profile::test_support::bulge_loop(vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(4.0, 0.0), bulge),
                (Point2::new(3.0, 0.45), 0.0),
            ]);
            sweep::test_support::extruded(
                crate::common::cavity::sketch_at(0.0),
                vec![lp],
                2.0,
                tol(),
            )
        });
        let Ok(body) = built else {
            eprintln!("PROBE H bulge={bulge}: profile invalid");
            continue;
        };
        let edges = edges_with_corners(&body, |q| near(q.y, 0.0) && near(q.z, 2.0));
        if edges.len() != 1 {
            eprintln!("PROBE H bulge={bulge}: {} AB edges", edges.len());
            continue;
        }
        door(
            &format!("H arc back toward AB bulge={bulge}"),
            &body,
            &edges,
            0.5,
            &[],
        );
    }
}

/// I. Near the boundary: the PR's trapezoid and probe E's turn just past
/// the width at which the far end face touches the link's band
/// (`r = 0.5`). Over-refusal shows as a meter refusal on a body the
/// door builds valid on main.
#[test]
fn probe_i_far_end_face_just_clear_of_the_band() {
    for w in [0.5, 0.501, 0.505, 0.51, 0.52, 0.55] {
        let body = prism(
            &[
                Point2::new(0.0, 0.0),
                Point2::new(4.0, 0.0),
                Point2::new(4.0, w),
                Point2::new(-1.0, w),
            ],
            0.0,
            2.0,
        );
        let top = |q: Point3<f64>| near(q.z, 2.0);
        let back = edges_with_corners(&body, |q| top(q) && near(q.y, w));
        let edges: Vec<_> = edges_with_corners(&body, top)
            .into_iter()
            .filter(|e| !back.contains(e))
            .collect();
        door(&format!("I trapezoid w={w}"), &body, &edges, 0.5, &[]);
    }
    for d in [0.5, 0.501, 0.505, 0.51, 0.52, 0.55] {
        let body = prism(
            &[
                Point2::new(0.0, 0.0),
                Point2::new(4.0, 0.0),
                Point2::new(4.0, 3.0),
                Point2::new(0.0, d),
            ],
            0.0,
            2.0,
        );
        let front = edges_with_corners(&body, |q| near(q.y, 0.0) && near(q.z, 2.0));
        let left = edges_with_corners(&body, |q| near(q.x, 0.0) && near(q.z, 2.0));
        let edges: Vec<_> = front.into_iter().chain(left).collect();
        door(&format!("I oblique far end d={d}"), &body, &edges, 0.5, &[]);
    }
}

/// J. A two-link chain ending at a corner patch: `L1` (top, left) turns
/// into `L0` (top, front), which ends at the front-right corner patch
/// with the front-right and top-right edges. The right face, at `L0`'s
/// far end and a support of the patch's other chains, is now metered
/// against `L1`.
#[test]
fn probe_j_a_chain_ending_at_a_corner_patch() {
    let p = Point3::new;
    for wx in [4.0, 1.6, 1.2, 1.05] {
        let body = brick(p(0.0, 0.0, 0.0), p(wx, 4.0, 2.0));
        let l0 = edges_with_corners(&body, |q| near(q.y, 0.0) && near(q.z, 2.0));
        let l1 = edges_with_corners(&body, |q| near(q.x, 0.0) && near(q.z, 2.0));
        let fr = edges_with_corners(&body, |q| near(q.x, wx) && near(q.y, 0.0));
        let tr = edges_with_corners(&body, |q| near(q.x, wx) && near(q.z, 2.0));
        let edges: Vec<_> = l0.into_iter().chain(l1).chain(fr).chain(tr).collect();
        assert_eq!(edges.len(), 4);
        door(
            &format!("J chain into a corner wx={wx}"),
            &body,
            &edges,
            0.5,
            &[],
        );
    }
}
