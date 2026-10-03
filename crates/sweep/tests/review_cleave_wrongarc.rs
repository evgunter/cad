//! **A curved face crossed more than twice is chorded along the arcs
//! inside it** (`work/cleave/split-pairs-curved-face-crossings-across-the-wrong-arc.md`,
//! PR 3718 review): steep splits of cylinders at many seam turns, of
//! tubes (concentric and eccentric bores, `sense: false` walls), of a
//! keyed cylinder made by `topo::subtract`, and planes through a seam's
//! cap corner. Each half passes tiers 1 and 3, its section faces are
//! all counter-clockwise (a clockwise face is the cancelling polygon an
//! outside chord makes), and its volume is the closed form wherever the
//! volume answers. On main's pairing (the book's rule on curved faces)
//! every row goes red on its section faces; the volumes are right on
//! both, by cancellation. A volume refusal (`props_quad_converged`
//! escalations, `QuadratureBudget`) is the props lane's and is printed,
//! not failed.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::bores::turned_cylinder;
use crate::common::cavity::brick;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::test_support::{bored_cylinder, cylinder_of_arcs_at};
use topo::Body;
use topo::splitting::{SplitPart, SplitPlane, split};

fn tol() -> Tol {
    Tol::witness()
}

/// `∫ clamp(a + b·y, 0, h) dy` over `[y1, y2]`, exactly.
fn clamp_lin_integral(a: f64, b: f64, h: f64, y1: f64, y2: f64) -> f64 {
    if y2 <= y1 {
        return 0.0;
    }
    if b.abs() < 1e-300 {
        return a.clamp(0.0, h) * (y2 - y1);
    }
    // Antiderivative of clamp(u, 0, h) in u, divided by b.
    let big = |u: f64| -> f64 {
        if u <= 0.0 {
            0.0
        } else if u <= h {
            0.5 * u * u
        } else {
            0.5 * h * h + h * (u - h)
        }
    };
    (big(a + b * y2) - big(a + b * y1)) / b
}

/// The volume of `{(x, y, z): (x−cx)² + (y−cy)² < r², 0 < z < h,
/// n·(p − o) < 0}` by a 1-D quadrature in x of the exact y integral.
fn disc_below(cx: f64, cy: f64, r: f64, h: f64, plane: &SplitPlane<f64>) -> f64 {
    let n = plane.normal.get();
    let o = plane.origin;
    let m = 20000;
    let mut sum = 0.0;
    for i in 0..m {
        let x = cx - r + 2.0 * r * (i as f64 + 0.5) / m as f64;
        let half = (r * r - (x - cx).powi(2)).max(0.0).sqrt();
        let (y1, y2) = (cy - half, cy + half);
        // n·(p−o) < 0 ⇔ n.z·z < −(n.x(x−o.x) + n.y(y−o.y)) + n.z·o.z
        let len = if n.z.abs() < 1e-14 {
            // Vertical plane: below is a half-space in (x, y) only,
            // `n.x(x−o.x) + n.y(y−o.y) < 0`, exact in y.
            let c = n.x * (x - o.x) - n.y * o.y;
            let inside = if n.y.abs() < 1e-300 {
                if c < 0.0 { y2 - y1 } else { 0.0 }
            } else {
                let yt = -c / n.y;
                if n.y > 0.0 {
                    (yt.min(y2) - y1).max(0.0)
                } else {
                    (y2 - yt.max(y1)).max(0.0)
                }
            };
            inside * h
        } else {
            // z_lim(y) = o.z − (n.x(x−o.x) + n.y(y−o.y))/n.z
            let a = o.z - (n.x * (x - o.x) - n.y * o.y) / n.z;
            let b = -n.y / n.z;
            let under = clamp_lin_integral(a, b, h, y1, y2);
            if n.z > 0.0 {
                under
            } else {
                h * (y2 - y1) - under
            }
        };
        sum += len * 2.0 * r / m as f64;
    }
    sum
}

/// Each section face of `half` as `(sense, ring count)`, sorted.
fn sections(half: &Body<f64>, plane: &SplitPlane<f64>) -> Vec<(bool, usize)> {
    let mut v: Vec<_> = half
        .faces()
        .filter(|(_, f)| {
            matches!(
                half.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if normal.cross(plane.normal.get()).norm() < 1e-12
                        && (*origin - plane.origin).dot(plane.normal.get()).abs() < 1e-12
            )
        })
        .map(|(_, f)| (f.sense, f.rings.len()))
        .collect();
    v.sort_unstable();
    v
}

/// One split, read: `Ok((below, above))` each `(sections, volume)` or a
/// string naming what refused or failed.
type Half = (Vec<(bool, usize)>, Result<f64, String>);
fn read_split(body: &Body<f64>, plane: &SplitPlane<f64>) -> Result<[Half; 2], String> {
    let r = split(body, plane, tol()).map_err(|e| format!("split refused: {e:?}"))?;
    let mut out = Vec::new();
    for (side, part) in [("below", r.below), ("above", r.above)] {
        let SplitPart::Body(half) = part else {
            return Err(format!("{side}: no material"));
        };
        if let Err(e) = topo::validate(&half) {
            return Err(format!("{side}: tier 1 {e:?}"));
        }
        if let Err(e) = topo::validate_geometric(&half, tol()) {
            return Err(format!("{side}: tier 3 {e:?}"));
        }
        let v = topo::mass_properties(&half, tol())
            .map(|m| m.volume)
            .map_err(|e| format!("{e:?}"));
        out.push((sections(&half, plane), v));
    }
    Ok([out.remove(0), out.remove(0)])
}

fn plane_at(o: [f64; 3], t: f64, phi: f64, flip: bool) -> SplitPlane<f64> {
    let s = if flip { -1.0 } else { 1.0 };
    topo::test_support::split_plane(
        Point3::new(o[0], o[1], o[2]),
        Vec3::new(t.sin() * phi.cos(), t.sin() * phi.sin(), t.cos()) * s,
        tol(),
    )
}

fn turned(body: &Body<f64>, turn: f64) -> Body<f64> {
    let map = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), turn);
    topo::transform_rigid(body, &map, tol()).unwrap()
}

/// Tally of one sweep.
#[derive(Default, Debug)]
struct Tally {
    ok: usize,
    skipped: usize,
    wrong: Vec<String>,
    refused: Vec<String>,
    vol_refused: Vec<String>,
}

impl Tally {
    fn report(&self, what: &str) {
        eprintln!(
            "TALLY {what}: ok {}, skipped {}, wrong {}, refused {}, volume refused {}",
            self.ok,
            self.skipped,
            self.wrong.len(),
            self.refused.len(),
            self.vol_refused.len()
        );
        for w in &self.wrong {
            eprintln!("  WRONG {w}");
        }
        for r in &self.refused {
            eprintln!("  REFUSED {r}");
        }
        for r in &self.vol_refused {
            eprintln!("  VOLUME REFUSED {r}");
        }
    }

    /// Every split answered with halves at rest and every reading was
    /// the truth.
    fn assert_clean(&self, what: &str) {
        self.report(what);
        assert!(
            self.wrong.is_empty() && self.refused.is_empty(),
            "{what}: {} wrong, {} refused (above)",
            self.wrong.len(),
            self.refused.len()
        );
        assert!(self.ok > 0, "{what}: nothing was read");
    }
}

/// Judge one split against `want_sections` (per half; `None` = any all
/// counter-clockwise) and the closed-form below volume.
fn judge(
    t: &mut Tally,
    what: String,
    body: &Body<f64>,
    plane: &SplitPlane<f64>,
    want_below: f64,
    total: f64,
    want_sections: Option<[Vec<(bool, usize)>; 2]>,
) {
    match read_split(body, plane) {
        // A plane only touching the body (through its extreme corner)
        // is a fixture artefact, not a refusal.
        Err(e) if e.ends_with("no material") => t.skipped += 1,
        Err(e) => t.refused.push(format!("{what}: {e}")),
        Ok([(sb, vb), (sa, va)]) => {
            let mut bad = Vec::new();
            match &want_sections {
                Some([wb, wa]) => {
                    if &sb != wb || &sa != wa {
                        bad.push(format!("sections {sb:?} / {sa:?}"));
                    }
                }
                None => {
                    if sb.iter().chain(&sa).any(|&(ccw, _)| !ccw) || sb.is_empty() || sa.is_empty()
                    {
                        bad.push(format!("sections {sb:?} / {sa:?}"));
                    }
                }
            }
            match (vb, va) {
                (Ok(vb), Ok(va)) => {
                    if (vb - want_below).abs() > 1e-4 || (va - (total - want_below)).abs() > 1e-4 {
                        bad.push(format!(
                            "volumes {vb:.6} / {va:.6} against {want_below:.6} / {:.6}",
                            total - want_below
                        ));
                    }
                }
                (vb, va) => t
                    .vol_refused
                    .push(format!("{what}: volume {vb:?} / {va:?}")),
            }
            if bad.is_empty() {
                t.ok += 1;
            } else {
                t.wrong.push(format!("{what}: {}", bad.join("; ")));
            }
        }
    }
}

/// steep cuts of the unit cylinder (2-arc and 3-arc walls) at many
/// seam angles, tilts, offsets and plane azimuths.
#[test]
fn steep_cuts_of_the_cylinder_at_many_seam_turns_chord_inside_each_wall_face() {
    use core::f64::consts::PI;
    let h = 2.5;
    let mut t = Tally::default();
    let total = PI * h;
    // Seam turns main's pairing reads wrong (measured), plus both sides
    // of the ±π wrap.
    let turns = [
        -PI / 2.0 + 0.013,
        1.1911,
        PI / 2.0 + 0.05,
        -1.9505,
        PI - 1e-3,
        -PI + 1e-3,
        0.0,
    ];
    for &turn in &turns {
        for (arcs, body) in [
            (2, turned_cylinder(turn, h)),
            (
                3,
                turned(
                    &cylinder_of_arcs_at(3, 1.0, Point2::new(0.0, 0.0), 0.0, h, tol()),
                    turn,
                ),
            ),
        ] {
            for tilt in [1.1, 1.4] {
                for (o, phi) in [([0.0, 0.0, 1.25], 0.0), ([-0.3, 0.2, 1.6], -2.3)] {
                    for flip in [false, true] {
                        let plane = plane_at(o, tilt, phi, flip);
                        let want = disc_below(0.0, 0.0, 1.0, h, &plane);
                        judge(
                            &mut t,
                            format!(
                                "{arcs}-arc turn {turn:.4} tilt {tilt} at {o:?} phi {phi} flip {flip}"
                            ),
                            &body,
                            &plane,
                            want,
                            total,
                            Some([vec![(true, 0)], vec![(true, 0)]]),
                        );
                    }
                }
            }
        }
    }
    t.assert_clean("p1 cylinder");
}

/// tubes — concentric and eccentric bores, turned through many
/// seam angles, cut steeply. Bore faces have `sense: false`.
#[test]
fn steep_cuts_of_tubes_chord_inside_each_bore_face() {
    use core::f64::consts::PI;
    let mut t = Tally::default();
    for (a, d, outer_phi) in [(0.4, 0.0, 0.0), (0.4, 0.3, 0.5), (0.25, -0.5, 1.2)] {
        let base = bored_cylinder(a, d, outer_phi, tol());
        let total = PI * (1.0 - a * a);
        for turn in [-1.5498, 1.0682, 1.5918, -2.5970] {
            let body = turned(&base, turn);
            let (bc, bs) = (d * turn.cos(), d * turn.sin());
            for tilt in [1.0, 1.2] {
                for (o, phi) in [([0.0, 0.0, 0.5], 0.0)] {
                    for flip in [false, true] {
                        let plane = plane_at(o, tilt, phi, flip);
                        let want = disc_below(0.0, 0.0, 1.0, 1.0, &plane)
                            - disc_below(bc, bs, a, 1.0, &plane);
                        judge(
                            &mut t,
                            format!(
                                "tube a {a} d {d} turn {turn:.4} tilt {tilt} at {o:?} phi {phi} flip {flip}"
                            ),
                            &body,
                            &plane,
                            want,
                            total,
                            None,
                        );
                    }
                }
            }
        }
    }
    t.assert_clean("p2 tube");
}

/// a boolean (the cylinder less a slab that notches its wall, so
/// a wall face holds a window), then steep and flat splits.
#[test]
fn a_keyed_cylinder_from_subtract_splits_into_counter_clockwise_sections() {
    let mut t = Tally::default();
    let h = 2.5;
    // Full-height keyways: no box edge pierces the curved wall (a
    // box edge through the wall refuses `CurvedSectorSideUnsupported`).
    let slabs = [
        ((0.3, 2.0), (-0.5, 0.5), (-1.0, 3.5)),
        ((-2.0, -0.5), (-0.3, 0.6), (-1.0, 3.5)),
        ((-0.25, 0.25), (0.4, 2.0), (-1.0, 3.5)),
    ];
    for turn in [0.0, -2.0] {
        let cyl = turned_cylinder(turn, h);
        for (sx, sy, sz) in slabs {
            let slab = brick(Point3::new(sx.0, sy.0, sz.0), Point3::new(sx.1, sy.1, sz.1));
            let body = match topo::subtract(&cyl, &slab, tol()) {
                Ok(r) => match r.body() {
                    Some(b) => b.body.clone(),
                    None => continue,
                },
                Err(e) => {
                    eprintln!("P3 slab {sx:?} {sy:?} {sz:?} turn {turn:.3}: boolean refused {e:?}");
                    continue;
                }
            };
            let total = topo::mass_properties(&body, tol()).unwrap().volume;
            for tilt in [0.0, 1.0, 1.2, 1.4] {
                for (o, phi) in [([0.0, 0.0, 1.25], 0.0), ([0.0, 0.3, 1.25], 1.3)] {
                    for flip in [false, true] {
                        let plane = plane_at(o, tilt, phi, flip);
                        let want = disc_below(0.0, 0.0, 1.0, h, &plane)
                            - box_disc_below(&plane, sx, sy, sz);
                        judge(
                            &mut t,
                            format!(
                                "cyl−slab {sx:?}{sy:?}{sz:?} turn {turn:.3} tilt {tilt} at {o:?} phi {phi} flip {flip}"
                            ),
                            &body,
                            &plane,
                            want,
                            total,
                            None,
                        );
                    }
                }
            }
        }
    }
    t.assert_clean("p3 cyl-slab");
}

/// The volume of (unit disc × [0, ∞)) ∩ box below `plane`, by a 1-D
/// midpoint rule in x with the exact y integral.
fn box_disc_below(plane: &SplitPlane<f64>, sx: (f64, f64), sy: (f64, f64), sz: (f64, f64)) -> f64 {
    let (n, o) = (plane.normal.get(), plane.origin);
    let m = 4000;
    let (x0, x1) = (sx.0.max(-1.0), sx.1.min(1.0));
    let sz = (sz.0.max(0.0), sz.1.min(2.5));
    let dz = sz.1 - sz.0;
    let mut s = 0.0;
    for i in 0..m {
        let x = x0 + (x1 - x0) * (i as f64 + 0.5) / m as f64;
        let r = (1.0 - x * x).max(0.0).sqrt();
        let (ymin, ymax) = (sy.0.max(-r), sy.1.min(r));
        if ymax <= ymin {
            continue;
        }
        let a = o.z - (n.x * (x - o.x) - n.y * o.y) / n.z - sz.0;
        let b = -n.y / n.z;
        let under = clamp_lin_integral(a, b, dz, ymin, ymax);
        let len = if n.z > 0.0 {
            under
        } else {
            dz * (ymax - ymin) - under
        };
        s += len * (x1 - x0) / m as f64;
    }
    s
}

/// planes through a seam's cap corner, with the seams at many
/// azimuths — including on the section ellipse's major vertices, where
/// the eccentric anomaly wraps at ±π.
#[test]
fn planes_through_a_seam_cap_corner_chord_inside_each_wall_face() {
    use core::f64::consts::PI;
    let h = 2.5;
    let total = PI * h;
    let mut t = Tally::default();
    for turn in [0.0, PI, PI / 2.0, PI / 2.0 + 0.05, 1.0] {
        let body = turned_cylinder(turn, h);
        for z in [0.0, h] {
            for corner in [turn, turn + PI] {
                for tilt in [1.1, 1.4] {
                    for phi in [0.0, 0.5] {
                        for flip in [false, true] {
                            let o = [corner.cos(), corner.sin(), z];
                            let plane = plane_at(o, tilt, phi, flip);
                            let want = disc_below(0.0, 0.0, 1.0, h, &plane);
                            judge(
                                &mut t,
                                format!(
                                    "corner turn {turn:.3} at az {corner:.3} z {z} tilt {tilt} phi {phi} flip {flip}"
                                ),
                                &body,
                                &plane,
                                want,
                                total,
                                Some([vec![(true, 0)], vec![(true, 0)]]),
                            );
                        }
                    }
                }
            }
        }
    }
    t.assert_clean("p5 corners");
}

/// a plane parallel to the cylinder's axis (a straight section:
/// two rulings), with the seams turned so one wall face holds all four
/// crossings — the book's rule still pairs these.
#[test]
fn axis_parallel_cuts_left_to_the_book_rule_still_answer() {
    use core::f64::consts::PI;
    let h = 2.5;
    let total = PI * h;
    let mut t = Tally::default();
    for turn in [PI / 2.0 + 0.05, 0.0, 1.0] {
        let body = turned_cylinder(turn, h);
        for c in [0.3, -0.7] {
            for phi in [0.0, 1.3] {
                for flip in [false, true] {
                    let plane = plane_at(
                        [c * f64::cos(phi), c * f64::sin(phi), 1.0],
                        PI / 2.0,
                        phi,
                        flip,
                    );
                    let n = plane.normal.get();
                    let plane = topo::test_support::split_plane(
                        plane.origin,
                        Vec3::new(n.x, n.y, 0.0),
                        tol(),
                    );
                    let want = disc_below(0.0, 0.0, 1.0, h, &plane);
                    judge(
                        &mut t,
                        format!("axis-parallel turn {turn:.3} c {c} phi {phi} flip {flip}"),
                        &body,
                        &plane,
                        want,
                        total,
                        Some([vec![(true, 0)], vec![(true, 0)]]),
                    );
                }
            }
        }
    }
    t.assert_clean("p7 axis-parallel");
}

/// PR 3981 review: the steep-tube pose that refused check 5 at
/// ε = 1e-6, and its neighbours in turn, tilt, height and plane
/// azimuth (one bore radius: 0.38 read identically to 0.4). Every
/// split must answer with halves at rest whose volumes are the closed
/// form: what the clamp turned from a refusal into an answer is
/// checked against the truth, not only admitted.
#[test]
fn steep_tube_neighbours_of_the_check_5_pose_answer_and_are_right() {
    use core::f64::consts::PI;
    let mut t = Tally::default();
    let a = 0.4;
    let base = bored_cylinder(a, 0.0, 0.0, tol());
    let total = PI * (1.0 - a * a);
    for turn in [1.0682, 1.0582, 1.0782, 1.1682, 0.9682, 1.3682] {
        let body = turned(&base, turn);
        for tilt in [1.15, 1.2, 1.25] {
            for (o, phi) in [([0.0, 0.0, 0.5], 0.0), ([0.0, 0.0, 0.47], 0.3)] {
                for flip in [false, true] {
                    let plane = plane_at(o, tilt, phi, flip);
                    let want = disc_below(0.0, 0.0, 1.0, 1.0, &plane)
                        - disc_below(0.0, 0.0, a, 1.0, &plane);
                    judge(
                        &mut t,
                        format!(
                            "tube a {a} turn {turn:.4} tilt {tilt} at {o:?} phi {phi} flip {flip}"
                        ),
                        &body,
                        &plane,
                        want,
                        total,
                        None,
                    );
                }
            }
        }
    }
    // Tier 3's quadrature refusals (`props_quad_converged`, QUAD's
    // `quadrature-convergence-test-escalates-instead-of-refining`) are
    // printed, not failed; a pcurve-certificate refusal is this row's.
    t.report("p2 steep-tube neighbours");
    assert!(t.wrong.is_empty(), "{} wrong (above)", t.wrong.len());
    let ours: Vec<_> = t
        .refused
        .iter()
        .filter(|r| !r.contains("props_quad_converged"))
        .collect();
    assert!(
        ours.is_empty(),
        "refusals outside the quadrature lane: {ours:#?}"
    );
    assert!(t.ok > 50, "only {} splits answered", t.ok);
}
