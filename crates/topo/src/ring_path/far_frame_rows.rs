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
    Island, IslandEdge, RADII, island_spans, meridian, on_torus, parallel, rectangle,
    spiric_islands, torus,
};
use super::*;
use core::f64::consts::{FRAC_PI_2, FRAC_PI_6, PI};
use geom_core::Tol;
use geom_core::tolerance::{EPS_MIN, SESSION_BOX_EXTENT};

/// The cone rows' turned frame, moved to `(8, −3, 5)·1e5` at scale `1e3`.
fn far() -> Frame {
    Frame {
        o: Point3::new(8e5, -3e5, 5e5),
        s: 1e3,
        ..frames()[1]
    }
}

/// The far frame's axes about the origin at unit scale: where the
/// islands' oracles are read, on coordinates the far frame's rounding has
/// not touched.
fn local() -> Frame {
    Frame {
        o: Point3::origin(),
        s: 1.0,
        ..far()
    }
}

/// How far, in local units, a grid point is kept from an island's
/// boundary: a million times the far frame's rounding of a local unit.
const OFF_BOUNDARY: f64 = 1e-6;

/// Whether the local point `p` is inside by `inside`, or `None` within
/// [`OFF_BOUNDARY`] of the boundary: where a step that far along a local
/// axis changes the answer.
fn settled(inside: &dyn Fn(Point3<f64>) -> bool, p: Point3<f64>) -> Option<bool> {
    let here = inside(p);
    let steps = [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()]
        .into_iter()
        .flat_map(|e| [e, -e])
        .map(|e| e * OFF_BOUNDARY);
    steps
        .map(|e| inside(p + e))
        .all(|x| x == here)
        .then_some(here)
}

/// Paths asked, decided, decided odd, each wrong one's description, and
/// the point–island pairs set aside on the island's boundary.
#[derive(Default)]
struct Tally {
    asked: usize,
    decided: usize,
    odd: usize,
    wrong: Vec<String>,
    kept: usize,
    on_boundary: usize,
}

impl Tally {
    /// Each point's side of an island by `inside`, read at its local
    /// twin; `None` on the boundary, which is counted.
    fn sides(
        &mut self,
        inside: &dyn Fn(Point3<f64>) -> bool,
        locals: &[Point3<f64>],
    ) -> Vec<Option<bool>> {
        let sides: Vec<_> = locals.iter().map(|&p| settled(inside, p)).collect();
        let off = sides.iter().filter(|s| s.is_some()).count();
        self.kept += off;
        self.on_boundary += sides.len() - off;
        sides
    }

    /// Every path `quadric` offers from `a` to `b`, read against `arcs`
    /// and the ends' sides. An escalation says nothing, as an undecided
    /// path does; an end on the boundary is not read.
    fn read(
        &mut self,
        quadric: &Quadric<f64>,
        (a, b): (Point3<f64>, Point3<f64>),
        arcs: &[LoopArc<f64>],
        sides: (Option<bool>, Option<bool>),
        name: &str,
    ) {
        let (Some(sa), Some(sb)) = sides else {
            return;
        };
        let want = sa != sb;
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
            self.on_boundary * 20 < self.kept,
            "{what}: {} point–island pairs on a boundary against {} kept",
            self.on_boundary,
            self.kept
        );
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
/// both nappes, in the far frame. Each island's side of a point is read
/// at the point's local twin ([`local`]), and the sector's four points
/// on its top arc are set aside ([`settled`]).
#[test]
fn a_cone_path_in_a_far_frame_reads_the_oracle_parity_or_nothing() {
    let (f, l) = (far(), local());
    let mut tally = Tally::default();
    let chart: Vec<_> = [0.05, 0.17, 0.6, 1.3, 1.6, 2.1, 2.2, 2.9]
        .into_iter()
        .flat_map(|h| (0..16).map(move |k| (h, -PI + (f64::from(k) + 0.5) * PI / 8.0)))
        .collect();
    let points: Vec<_> = chart.iter().map(|&(h, t)| on_nappe(f, h, t)).collect();
    let locals: Vec<_> = chart.iter().map(|&(h, t)| on_nappe(l, h, t)).collect();
    for mirror in [false, true] {
        let (cone, twin) = (cone(f, mirror), cone(l, mirror));
        let quadric = Quadric::of(&cone).unwrap();
        for (name, island, oracle) in [
            ("lune", lune(f, &cone, 1.0), lune(l, &twin, 1.0)),
            ("lune by the apex", lune(f, &cone, 0.1), lune(l, &twin, 0.1)),
            ("sector", sector(f, &cone), sector(l, &twin)),
        ] {
            let arcs: Vec<_> = island
                .edges
                .iter()
                .map(|e| LoopArc::of(&e.carrier, e.params).unwrap())
                .collect();
            let sides = tally.sides(&oracle.inside, &locals);
            for (i, &a) in points.iter().enumerate() {
                for (j, &b) in points.iter().enumerate().skip(i + 1) {
                    tally.read(&quadric, (a, b), &arcs, (sides[i], sides[j]), name);
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

/// Chart points on the planes of the arcs of a torus island bounded by
/// spiric ovals: on the ovals' planes `x = d` and on the parallels'
/// cones, away from the island's corners (`corners`, local).
fn on_oval_planes(corners: &[Point3<f64>]) -> Vec<(f64, f64)> {
    let (big, small) = RADII;
    let mut chart = Vec::new();
    for d in [-0.7, 0.1, 0.2, 0.3, 0.4, 0.9, 1.0] {
        for v in [-2.6, -1.1, -0.2, 0.15, 1.2, 2.0, 2.9] {
            let c = d / (big + small * f64::cos(v));
            if c.abs() < 1.0 {
                chart.extend([(c.acos(), v), (-c.acos(), v)]);
            }
        }
    }
    for v in [-0.7, -0.6, -0.5, 0.4, 0.8, 0.9, 1.8, 2.75] {
        for k in 0..6 {
            chart.push((-PI + (f64::from(k) + 0.27) * PI / 3.0, v));
        }
    }
    chart.retain(|&(u, v)| {
        let p = on_torus(local(), u, v);
        corners.iter().all(|&c| (p - c).norm() > 1e-3)
    });
    chart
}

/// Every torus island, from points on its arcs' carriers and planes
/// (`a_torus_path_from_an_arcs_carrier_reads_the_oracle_parity`'s, and
/// the ovals' planes), to every fifth point of the whole torus's grid
/// and to each other, both ways, the axis either way, in the far frame.
/// Each meridian or parallel through such a point lies in an arc's
/// plane. Each island's side of a point is read at the point's local
/// twin ([`local`]), and the points on an island's arcs are set aside
/// ([`settled`]).
#[test]
fn a_torus_path_in_a_far_frame_reads_the_oracle_parity_or_nothing() {
    let (f, l) = (far(), local());
    let grid: Vec<_> = (0..12)
        .flat_map(|k| {
            (0..8).map(move |j| {
                (
                    -PI + (f64::from(k) + 0.31) * PI / 6.0,
                    -PI + (f64::from(j) + 0.43) * PI / 4.0,
                )
            })
        })
        .step_by(5)
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
                let carriers = super::tests::on_carriers(us, vs);
                (name, rectangle(f, us, vs), rectangle(l, us, vs), carriers)
            });
        let ovals = spiric_islands(f).into_iter().zip(spiric_islands(l)).map(
            |((name, island), (_, oracle))| {
                let carriers = on_oval_planes(&oracle.corners);
                (name, island, oracle, carriers)
            },
        );
        for (name, island, oracle, carriers) in rectangles.chain(ovals) {
            let arcs = torus_arcs(&quadric, &island);
            let chart: Vec<_> = carriers.iter().chain(&grid).copied().collect();
            let points: Vec<_> = chart.iter().map(|&(u, v)| on_torus(f, u, v)).collect();
            let locals: Vec<_> = chart.iter().map(|&(u, v)| on_torus(l, u, v)).collect();
            let sides = tally.sides(&oracle.inside, &locals);
            let n = carriers.len();
            for i in 0..n {
                for j in (n..chart.len()).chain(i + 1..n) {
                    for (x, y) in [(i, j), (j, i)] {
                        let ends = (points[x], points[y]);
                        tally.read(&quadric, ends, &arcs, (sides[x], sides[y]), name);
                    }
                }
            }
        }
    }
    tally.assert("torus", 60);
}

/// **Each charged reading stands down on a margin of three ulps of a far
/// coordinate**, which the same margin about the origin decides: the
/// gap of a circle against a plane three ulps beyond it
/// (**`split_ring_path_meets_plane`**), a root three ulps outside a
/// piece's span, on a unit circle and on one of radius 1/64
/// (**`split_ring_path_in_span`**, whose charge in the parameter is
/// `slack / arm`), a segment three ulps off a conic's plane by its end
/// (**`split_ring_path_segment_side`**), a crossing three ulps on its
/// arc's own loop (**`split_ring_path_torus_loop`**), a parallel three
/// ulps off the crest cylinder (**`split_ring_path_crest_graze`**), a
/// lead from three ulps off it (**`split_ring_path_crest_side`**), a
/// segment at a unit lever six ulps of sine off a ruling arc 1 long, and
/// 192 off one 1/64 long (**`split_ring_path_rulings_apart`**, charged
/// each direction's rounding: four ulps over the lever and four over
/// the arc's length), and the paths
/// from an end three ulps off the cone's apex plane
/// (**`split_ring_path_nappe`**), off its axis
/// (**`split_ring_path_off_apex`**), three ulps of slant past the other's
/// parallel (**`split_ring_path_rise`**), off a torus's core
/// (**`split_ring_path_off_core`**) and three ulps of sine off the other
/// on a sphere (**`split_ring_path_span`**). The far coordinates'
/// resolution is four ulps. Half of it would leave one ulp of each
/// margin, which clears the band's escalation threshold, so each case is
/// red against the mutants "no resolution charged" at its own reading
/// and "[`resolution`] halved"; the short circle is red against `slack`
/// charged in the parameter unlevered, and the short ruling arc against
/// its direction's rounding uncharged.
#[test]
fn each_charged_reading_stands_down_inside_far_points_resolution() {
    let esc = band().escalate();
    let k = (2.0 * esc).log2().ceil() + 52.0;
    let far = 2f64.powf(k);
    let ulp = far * f64::EPSILON;
    let m = 3.0 * ulp;
    assert!(ulp > esc, "one ulp clears the band");
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
                        at(-m - 1.0, 0.0, 0.0),
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
                        (FRAC_PI_2 + m, PI),
                    ))],
                    LoopArc::Conic(conic(None)),
                ),
            ),
            (
                "in_span, radius 1/64",
                read(
                    vec![Piece::Arc(circle(
                        at(0.0, 0.0, 0.0),
                        z,
                        x,
                        1.0 / 64.0,
                        (FRAC_PI_2 + 64.0 * m, PI),
                    ))],
                    LoopArc::Conic(conic(None)),
                ),
            ),
            (
                "segment_side",
                read(
                    vec![segment(at(m, 1.0, 0.0), at(1.0, 1.0, 0.0))],
                    LoopArc::Conic(conic(None)),
                ),
            ),
            (
                "torus_loop",
                read(
                    vec![Piece::Arc(circle(at(0.0, 0.0, 0.0), z, x, 1.0, (0.0, PI)))],
                    LoopArc::Conic(conic(Some(ArcLoop {
                        split: LoopSplit::Side {
                            origin: at(0.0, 1.0 - m, 0.0),
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
                        1.0 + m,
                        (0.0, 1.0),
                    ))],
                    crest.clone(),
                ),
            ),
            (
                "crest_side",
                read(
                    vec![segment(at(1.0 + m, 0.0, 0.0), at(1.5, 0.0, 0.0))],
                    crest,
                ),
            ),
            (
                "rulings_apart",
                read(
                    vec![segment(at(1.0, 0.0, 0.0), at(2.0, 0.0, 0.0))],
                    LoopArc::Line {
                        origin: at(0.0, 0.0, 0.0),
                        dir: Vec3::new(1.0, 2.0 * m, 0.0),
                        span: (0.0, 1.0),
                    },
                ),
            ),
            (
                "rulings_apart, a ruling arc 1/64 long",
                read(
                    vec![segment(at(1.0, 0.0, 0.0), at(2.0, 0.0, 0.0))],
                    LoopArc::Line {
                        origin: at(0.0, 0.0, 0.0),
                        dir: Vec3::new(1.0, 64.0 * m, 0.0),
                        span: (0.0, 1.0 / 64.0),
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
        let e = m / 2f64.sqrt();
        let counts = [
            (
                "nappe",
                cone.paths((at(1.0, 0.0, m), at(1.0, 0.5, 1.0)), band()),
                2,
            ),
            (
                "off_apex",
                cone.paths((at(m, 0.0, 1.0), at(1.0, 0.5, 1.0)), band()),
                2,
            ),
            (
                "rise",
                cone.paths((at(1.0, 0.0, 1.0), at(0.0, 1.0 + e, 1.0 + e)), band()),
                2,
            ),
            (
                "off_core",
                torus.paths((at(2.0 + m, 0.0, 0.0), at(0.0, 2.5, 0.0)), band()),
                2,
            ),
            (
                "span",
                sphere.paths((at(1.0, 0.0, 0.0), at(1.0, m, 0.0)), band()),
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

/// The kind [`Quadric::torus_circle_arc`] reads a circle as: a
/// meridian, a parallel on its side of `R`, a crest on its side of the
/// equator plane, `None` or an escalation.
fn circle_kind(read: Result<Option<LoopArc<f64>>, Indeterminate>) -> String {
    match read {
        Err(_) => "escalated".into(),
        Ok(None) => "none".into(),
        Ok(Some(LoopArc::Crest { own, .. })) => format!("crest {}", own.positive),
        Ok(Some(LoopArc::Conic(Conic { own: Some(own), .. }))) => match own.split {
            LoopSplit::Side { .. } => "meridian".into(),
            LoopSplit::Radius { .. } => format!("parallel {}", own.positive),
            LoopSplit::Bisector { .. } => "bisector".into(),
        },
        Ok(Some(_)) => "another arc".into(),
    }
}

/// The section of `torus(f, mirror)` by the plane of `edge`, stored at
/// its centre moved `shift` local units along the plane, through
/// [`geom_brep::plane_torus_section`] at `band`: the section's kind and,
/// for the circle of it nearest `edge`'s carrier, the kind the ring lane
/// reads.
fn minted_kind(
    f: Frame,
    mirror: bool,
    edge: &IslandEdge,
    shift: f64,
    band: Band,
) -> (&'static str, String) {
    use geom_brep::PlaneTorusSection as S;
    let surface = torus(f, mirror);
    let quadric = Quadric::of(&surface).unwrap();
    let (centre, normal) = edge.plane;
    let u_ref = normal.cross(f.x + f.y * 0.5).normalize();
    let plane = geom::Surface::Plane {
        origin: centre + u_ref * (shift * f.s),
        normal,
        u_ref,
    };
    let extent = 2.0 * (RADII.0 + RADII.1) * f.s;
    let (section, circles) = match geom_brep::plane_torus_section(&plane, &surface, extent, band) {
        Ok(S::MeridianCircles { c1, c2 }) => ("meridians", vec![c1, c2]),
        Ok(S::ConcentricCircles { c1, c2 }) => ("concentric", vec![c1, c2]),
        Ok(S::TangentCircle(c)) => ("tangent", vec![c]),
        Ok(S::SpiricOvals { .. }) => ("ovals", vec![]),
        Ok(S::Empty) => ("empty", vec![]),
        Err(_) => ("refused", vec![]),
    };
    let geom::Curve3::Circle {
        center: k,
        radius: rho,
        ..
    } = edge.carrier
    else {
        unreachable!("a meridian or a parallel")
    };
    let off = |c: &geom::Curve3<f64>| match c {
        geom::Curve3::Circle { center, radius, .. } => (*center - k).norm() + (radius - rho).abs(),
        _ => f64::INFINITY,
    };
    let kind = circles
        .into_iter()
        .min_by(|a, b| off(a).total_cmp(&off(b)))
        .map_or_else(String::new, |c| {
            circle_kind(quadric.torus_circle_arc(&c, edge.params, band))
        });
    (section, kind)
}

/// How far, in local units, [`minted_kind`] stores a plane's origin from
/// its circle's centre when it does not store it there.
const SHIFT: f64 = 3.7;

/// **A torus circle the plane×torus lane mints anywhere in the session
/// box reads the section and the kind it reads at the origin**: the
/// meridians, the parallels either side of `R` and the crests of a grid
/// round the torus, each cut by its plane stored at the circle's centre
/// or off it, in the three frames' axes moved out to the box's edge at
/// scales 1 and `1e2`, at the run's ε and at [`EPS_MIN`]. The lane reads
/// its gaps along the plane's normal on the difference of the plane's
/// stored origin and the torus's centre, so the far frame's rounding
/// enters them as a few ulps of the box, under [`EPS_MIN`]. It mints a
/// circle's axis as the plane's normal or the torus's axis and its radius
/// as `r`, `R` or `R ± w` with `w` decided off zero, none of them a
/// difference of far points, so the four readings of
/// [`Quadric::torus_circle_arc`] see what they see at the origin.
#[test]
fn a_torus_circle_the_section_lane_mints_in_the_box_reads_its_kind_at_the_origin() {
    let us: Vec<f64> = (0..12)
        .map(|k| -PI + (f64::from(k) + 0.31) * PI / 6.0)
        .collect();
    let vs: Vec<f64> = (0..16)
        .map(|k| -PI + (f64::from(k) + 0.43) * PI / 8.0)
        .chain([FRAC_PI_2, -FRAC_PI_2, 0.0, PI])
        .collect();
    let bands = [
        band(),
        Band::linear_at(Tol::witness(), EPS_MIN).expect("the floor is an admissible ε"),
    ];
    let mut compared = std::collections::BTreeMap::<String, usize>::new();
    let mut other_section = Vec::new();
    let mut wrong = Vec::new();
    for band in bands {
        for axes in frames() {
            for s in [1.0, 1e2] {
                // Every stored point lies within the torus's outer radius
                // and the plane's shift of the frame's origin.
                let edge_of_box = SESSION_BOX_EXTENT - (RADII.0 + RADII.1 + SHIFT) * s;
                let origins: Vec<_> = [1e2, 3e2, edge_of_box]
                    .into_iter()
                    .flat_map(|m| {
                        [
                            (1.0, 0.0, 0.0),
                            (0.8, -0.3, 0.5),
                            (-0.2, 0.9, 0.4),
                            (0.0, 0.0, -1.0),
                        ]
                        .map(|(x, y, z)| Point3::origin() + Vec3::new(x, y, z).normalize() * m)
                    })
                    .collect();
                for mirror in [false, true] {
                    let home = Frame {
                        o: Point3::origin(),
                        s,
                        ..axes
                    };
                    let edges = us
                        .iter()
                        .map(|&u| -> Box<dyn Fn(Frame) -> IslandEdge> {
                            Box::new(move |f| meridian(f, u, (0.1, 0.5)))
                        })
                        .chain(vs.iter().map(|&v| -> Box<dyn Fn(Frame) -> IslandEdge> {
                            Box::new(move |f| parallel(f, v, (0.1, 0.5)))
                        }));
                    for edge in edges {
                        for shift in [0.0, SHIFT] {
                            let want = minted_kind(home, mirror, &edge(home), shift, band);
                            for &o in &origins {
                                let f = Frame { o, ..home };
                                let got = minted_kind(f, mirror, &edge(f), shift, band);
                                let at = format!("at {o:?}, scale {s}, {band:?}");
                                if got.0 != want.0 {
                                    other_section.push(format!("{} → {} {at}", want.0, got.0));
                                } else if got.1 != want.1 {
                                    wrong.push(format!("{want:?} reads {got:?} {at}"));
                                } else {
                                    *compared.entry(got.1).or_default() += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let mut by_class = std::collections::BTreeMap::<&str, usize>::new();
    for o in &other_section {
        *by_class
            .entry(o.split(" at ").next().unwrap_or(o))
            .or_default() += 1;
    }
    assert!(
        other_section.is_empty(),
        "{} sections in the box read another section ({by_class:?}), the first {:?}",
        other_section.len(),
        &other_section[..other_section.len().min(3)]
    );
    assert!(
        wrong.is_empty(),
        "{} circles in the box read another kind, the first {:?}",
        wrong.len(),
        &wrong[..wrong.len().min(3)]
    );
    let kinds = [
        "meridian",
        "parallel true",
        "parallel false",
        "crest true",
        "crest false",
    ];
    assert!(
        kinds
            .iter()
            .all(|k| compared.get(*k).is_some_and(|&n| n >= 100))
            && compared.len() == kinds.len(),
        "each kind read on a hundred circles in the box or more: {compared:?}"
    );
}
