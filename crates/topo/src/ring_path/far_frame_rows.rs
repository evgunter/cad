//! **A path in a frame far from the origin reads the oracle's parity or
//! nothing**: the islands' rows moved to coordinates of about `1e6`,
//! where a coordinate is stored to about `1e-10`, a hundred times the
//! zero band at ε `1e-12`. Points that should lie on one plane or one
//! circle disagree there by more than the band, so a margin read between
//! them is decided only past their [`resolution`]. And **each reading
//! that charges it stands down on a margin inside it**, which the same
//! margin about the origin decides.

use super::cone_islands::{Frame, band, cone, frames, lune, on_nappe, sector};
use super::torus_islands::{
    Island, RADII, island_spans, on_torus, rectangle, spiric_islands, torus,
};
use super::*;
use core::f64::consts::{FRAC_PI_2, FRAC_PI_6, PI};

/// The cone rows' turned frame, moved to `(8, −3, 5)·1e5` at scale `1e3`.
fn far() -> Frame {
    Frame {
        o: Point3::new(8e5, -3e5, 5e5),
        s: 1e3,
        ..frames()[1]
    }
}

/// Paths asked, decided, decided odd, and each wrong one's description.
#[derive(Default)]
struct Tally {
    asked: usize,
    decided: usize,
    odd: usize,
    wrong: Vec<String>,
}

impl Tally {
    /// Every path `quadric` offers from `a` to `b`, read against `arcs`
    /// and the island's oracle. An escalation says nothing, as an
    /// undecided path does.
    fn read(
        &mut self,
        quadric: &Quadric<f64>,
        (a, b): (Point3<f64>, Point3<f64>),
        arcs: &[LoopArc<f64>],
        inside: &dyn Fn(Point3<f64>) -> bool,
        name: &str,
    ) {
        let want = inside(a) != inside(b);
        let Ok(paths) = quadric.paths((a, b), band()) else {
            return;
        };
        for path in paths {
            self.asked += 1;
            if let Ok(Some(got)) = path_parity(&path, arcs, None, band()) {
                self.decided += 1;
                self.odd += usize::from(got);
                if got != want {
                    self.wrong.push(format!("{name}: {a:?} → {b:?}"));
                }
            }
        }
    }

    fn assert(&self, what: &str, decided_percent: usize) {
        assert!(
            self.wrong.is_empty(),
            "{what}: {} of {} decided paths read wrong, the first {:?}",
            self.wrong.len(),
            self.decided,
            &self.wrong[..self.wrong.len().min(3)]
        );
        assert!(
            self.decided * 100 >= self.asked * decided_percent && self.odd * 20 > self.decided,
            "{what}: {} of {} paths decided, {} odd",
            self.decided,
            self.asked,
            self.odd
        );
    }
}

/// The cone rows' grid (`a_cone_path_crosses_an_island_by_the_oracle_parity`)
/// with the lune's top height 2.2 added, whose parallels lie in that
/// arc's plane, against the lune, the lune by the apex and the sector,
/// both nappes, in the far frame.
#[test]
fn a_cone_path_in_a_far_frame_reads_the_oracle_parity_or_nothing() {
    let f = far();
    let mut tally = Tally::default();
    for mirror in [false, true] {
        let cone = cone(f, mirror);
        let quadric = Quadric::of(&cone).unwrap();
        let mut points = Vec::new();
        for h in [0.05, 0.17, 0.6, 1.3, 1.6, 2.1, 2.2, 2.9] {
            for k in 0..16 {
                points.push(on_nappe(f, h, -PI + (f64::from(k) + 0.5) * PI / 8.0));
            }
        }
        for (name, island) in [
            ("lune", lune(f, &cone, 1.0)),
            ("lune by the apex", lune(f, &cone, 0.1)),
            ("sector", sector(f, &cone)),
        ] {
            let arcs: Vec<_> = island
                .edges
                .iter()
                .map(|e| LoopArc::of(&e.carrier, e.params).unwrap())
                .collect();
            for (i, &a) in points.iter().enumerate() {
                for &b in &points[i + 1..] {
                    tally.read(&quadric, (a, b), &arcs, &island.inside, name);
                }
            }
        }
    }
    tally.assert("cone", 90);
}

/// The torus islands' arcs, as the ring lane reads them.
fn torus_arcs(quadric: &Quadric<f64>, island: &Island) -> Vec<LoopArc<f64>> {
    island
        .edges
        .iter()
        .map(|e| {
            quadric
                .torus_spiric_arc(&e.carrier, e.params)
                .or_else(|| {
                    quadric
                        .torus_circle_arc(&e.carrier, e.params, band())
                        .unwrap()
                })
                .expect("a meridian, a parallel or an oval")
        })
        .collect()
}

/// Points on the planes of the arcs of a torus island bounded by spiric
/// ovals: on the ovals' planes `x = d` and on the parallels' cones,
/// away from the island's corners.
fn on_oval_planes(f: Frame, island: &Island) -> Vec<Point3<f64>> {
    let (big, small) = RADII;
    let mut points = Vec::new();
    for d in [-0.7, 0.1, 0.2, 0.3, 0.4, 0.9, 1.0] {
        for v in [-2.6, -1.1, -0.2, 0.15, 1.2, 2.0, 2.9] {
            let c = d / (big + small * f64::cos(v));
            if c.abs() < 1.0 {
                points.extend([c.acos(), -c.acos()].map(|u| on_torus(f, u, v)));
            }
        }
    }
    for v in [-0.7, -0.6, -0.5, 0.4, 0.8, 0.9, 1.8, 2.75] {
        for k in 0..6 {
            points.push(on_torus(f, -PI + (f64::from(k) + 0.27) * PI / 3.0, v));
        }
    }
    points.retain(|&p| island.corners.iter().all(|&c| (p - c).norm() > 1e-3 * f.s));
    points
}

/// Every torus island, from points on its arcs' carriers and planes
/// (`a_torus_path_from_an_arcs_carrier_reads_the_oracle_parity`'s, and
/// the ovals' planes), to every fifth point of the whole torus's grid
/// and to each other, both ways, the axis either way, in the far frame.
/// Each meridian or parallel through such a point lies in an arc's
/// plane.
#[test]
fn a_torus_path_in_a_far_frame_reads_the_oracle_parity_or_nothing() {
    let f = far();
    let grid: Vec<_> = (0..12)
        .flat_map(|k| {
            (0..8).map(move |j| {
                on_torus(
                    f,
                    -PI + (f64::from(k) + 0.31) * PI / 6.0,
                    -PI + (f64::from(j) + 0.43) * PI / 4.0,
                )
            })
        })
        .collect();
    let mut tally = Tally::default();
    for mirror in [false, true] {
        let surface = torus(f, mirror);
        let quadric = Quadric::of(&surface).unwrap();
        let rectangles = island_spans()
            .into_iter()
            // `crest_islands`' rectangles.
            .chain([
                ("up to the top", (-0.5, 0.6), (0.2, FRAC_PI_2)),
                ("down to the bottom", (-0.5, 0.6), (-FRAC_PI_2, -0.2)),
                ("from the top inward", (-0.3, 0.8), (FRAC_PI_2, 2.5)),
                ("the outer side", (-1.0, 0.7), (-FRAC_PI_2, FRAC_PI_2)),
            ])
            .map(|(name, us, vs)| {
                let carriers: Vec<_> = super::tests::on_carriers(us, vs)
                    .into_iter()
                    .map(|(u, v)| on_torus(f, u, v))
                    .collect();
                (name, rectangle(f, us, vs), carriers)
            });
        let ovals = spiric_islands(f).into_iter().map(|(name, island)| {
            let carriers = on_oval_planes(f, &island);
            (name, island, carriers)
        });
        for (name, island, carriers) in rectangles.chain(ovals) {
            let arcs = torus_arcs(&quadric, &island);
            for (i, &a) in carriers.iter().enumerate() {
                for &b in grid.iter().step_by(5).chain(&carriers[i + 1..]) {
                    for ends in [(a, b), (b, a)] {
                        tally.read(&quadric, ends, &arcs, &island.inside, name);
                    }
                }
            }
        }
    }
    tally.assert("torus", 60);
}

/// **Each charged reading stands down on a margin of two ulps of a far
/// coordinate**, which the same margin about the origin decides: the
/// gap of a circle against a plane two ulps beyond it
/// (**`split_ring_path_meets_plane`**), a root two ulps outside a
/// piece's span (**`split_ring_path_in_span`**), a segment two ulps off
/// a conic's plane by its end (**`split_ring_path_segment_side`**), a
/// crossing two ulps on its arc's own loop
/// (**`split_ring_path_torus_loop`**), a parallel two ulps off the crest
/// cylinder (**`split_ring_path_crest_graze`**), a lead from two ulps off
/// it (**`split_ring_path_crest_side`**), a segment two ulps of sine off
/// a ruling at a unit lever (**`split_ring_path_rulings_apart`**), and
/// the paths from an end two ulps off the cone's apex plane
/// (**`split_ring_path_nappe`**), off its axis
/// (**`split_ring_path_off_apex`**), two ulps of slant past the other's
/// parallel (**`split_ring_path_rise`**), off a torus's core
/// (**`split_ring_path_off_core`**) and two ulps of sine off the other on
/// a sphere (**`split_ring_path_span`**). Two ulps clear the band's
/// escalation threshold; the far coordinates' resolution is four. Each
/// case is red against the mutant "no resolution charged" at its own
/// reading.
#[test]
fn each_charged_reading_stands_down_inside_far_points_resolution() {
    let esc = band().escalate();
    let k = (2.0 * esc).log2().ceil() + 51.0;
    let far = 2f64.powf(k);
    let ulp = far * f64::EPSILON;
    assert!(2.0 * ulp > esc, "two ulps clear the band");
    let read = |pieces: Vec<Piece<f64>>, arc: LoopArc<f64>| {
        path_parity(
            &Path {
                pieces,
                arrival: Vec3::unit_x(),
            },
            &[arc],
            None,
            band(),
        )
    };
    let (x, y, z) = (Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z());
    let mut bad = Vec::new();
    for o in [far, 0.0] {
        let at = |x: f64, y: f64, z: f64| Point3::new(o + x, y, z);
        let circle = |centre, axis, u_ref, radius, span| CircleArc {
            centre,
            axis,
            radius,
            u_ref,
            span,
        };
        // The unit circle in the plane `x = 0` about `at(0)`.
        let conic = |own| Conic {
            centre: at(0.0, 0.0, 0.0),
            axis: x,
            u_ref: y,
            a: 1.0,
            b: 1.0,
            span: (-1.0, 1.0),
            own,
        };
        let crest = LoopArc::Crest {
            centre: at(0.0, 0.0, 0.0),
            axis: z,
            u_ref: x,
            radius: 1.0,
            span: (0.0, 1.0),
            core: (at(0.0, 0.0, 0.0), z, 1.0),
            own: ArcLoop {
                split: LoopSplit::Side {
                    origin: at(0.0, 0.0, 0.0),
                    dir: z,
                },
                positive: true,
            },
        };
        let segment = |from, to| Piece::Segment {
            from,
            to,
            ruling: x,
            lever: 1.0,
        };
        let parities = [
            (
                "meets_plane",
                read(
                    vec![Piece::Arc(circle(
                        at(-2.0 * ulp - 1.0, 0.0, 0.0),
                        z,
                        x,
                        1.0,
                        (-1.0, 1.0),
                    ))],
                    LoopArc::Conic(conic(None)),
                ),
            ),
            (
                "in_span",
                read(
                    vec![Piece::Arc(circle(
                        at(0.0, 0.0, 0.0),
                        z,
                        x,
                        1.0,
                        (FRAC_PI_2 + 2.0 * ulp, PI),
                    ))],
                    LoopArc::Conic(conic(None)),
                ),
            ),
            (
                "segment_side",
                read(
                    vec![segment(at(2.0 * ulp, 1.0, 0.0), at(1.0, 1.0, 0.0))],
                    LoopArc::Conic(conic(None)),
                ),
            ),
            (
                "torus_loop",
                read(
                    vec![Piece::Arc(circle(at(0.0, 0.0, 0.0), z, x, 1.0, (0.0, PI)))],
                    LoopArc::Conic(conic(Some(ArcLoop {
                        split: LoopSplit::Side {
                            origin: at(0.0, 1.0 - 2.0 * ulp, 0.0),
                            dir: y,
                        },
                        positive: true,
                    }))),
                ),
            ),
            (
                "crest_graze",
                read(
                    vec![Piece::Arc(circle(
                        at(0.0, 0.0, 0.0),
                        z,
                        x,
                        1.0 + 2.0 * ulp,
                        (0.0, 1.0),
                    ))],
                    crest,
                ),
            ),
            (
                "crest_side",
                read(
                    vec![segment(at(1.0 + 2.0 * ulp, 0.0, 0.0), at(1.5, 0.0, 0.0))],
                    crest,
                ),
            ),
            (
                "rulings_apart",
                read(
                    vec![segment(at(1.0, 0.0, 0.0), at(2.0, 0.0, 0.0))],
                    LoopArc::Line {
                        origin: at(0.0, 0.0, 0.0),
                        dir: Vec3::new(1.0, 2.0 * ulp, 0.0),
                        span: (0.0, 1.0),
                    },
                ),
            ),
        ];
        let cone = Quadric::Cone {
            apex: at(0.0, 0.0, 0.0),
            axis: z,
            half_angle: FRAC_PI_6,
        };
        let torus = Quadric::Torus {
            centre: at(0.0, 0.0, 0.0),
            axis: z,
            major: 2.0,
            minor: 0.8,
        };
        let sphere = Quadric::Sphere {
            centre: at(0.0, 0.0, 0.0),
            radius: 1.0,
        };
        let e = 2.0 * ulp / 2f64.sqrt();
        let counts = [
            (
                "nappe",
                cone.paths((at(1.0, 0.0, 2.0 * ulp), at(1.0, 0.5, 1.0)), band()),
                2,
            ),
            (
                "off_apex",
                cone.paths((at(2.0 * ulp, 0.0, 1.0), at(1.0, 0.5, 1.0)), band()),
                2,
            ),
            (
                "rise",
                cone.paths((at(1.0, 0.0, 1.0), at(0.0, 1.0 + e, 1.0 + e)), band()),
                2,
            ),
            (
                "off_core",
                torus.paths((at(2.0 + 2.0 * ulp, 0.0, 0.0), at(0.0, 2.5, 0.0)), band()),
                2,
            ),
            (
                "span",
                sphere.paths((at(1.0, 0.0, 0.0), at(1.0, 2.0 * ulp, 0.0)), band()),
                1,
            ),
        ];
        for (name, got) in parities {
            let decided = matches!(got, Ok(Some(_)));
            if decided != (o == 0.0) {
                bad.push(format!("{name} about {o}: {got:?}"));
            }
        }
        for (name, got, all) in counts {
            let n = got.map(|p| p.len());
            let held = if o == 0.0 {
                n == Ok(all)
            } else {
                n.is_ok_and(|n| n < all)
            };
            if !held {
                bad.push(format!("{name} about {o}: {n:?} of {all} paths"));
            }
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}
