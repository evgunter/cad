//! **A ruled face whose straight side holds a point**: a cylinder or
//! cone face with a ruling carried by more than one edge, which
//! `tessellate::lane_of` sends to the chart lane
//! (`walk::splits_a_straight_side`), where it is walked
//! (`trimmed::walked`, `ruled_walk`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Vec3};
use profile::ProfileLoop;
use profile::RawLoop as _;
use sweep::{Revolution, revolve};
use topo::Body;
use topo::construction::Construct;

use crate::common::{axis_y, validated};
use crate::reach_probes::{Got, geometry_enclosing, got_enclosing, tol};

/// The profile `pts` revolved about `y` through `turn`.
fn turned(pts: &[(f64, f64)], turn: Revolution<f64>) -> Body<f64> {
    let lp = ProfileLoop::polygon(pts.iter().map(|&(x, y)| Point2::new(x, y)));
    revolve(&validated(vec![lp]), axis_y(), turn, tol())
        .unwrap()
        .body
        .into_body()
}

/// The body's line edges, in arena order.
fn rulings(body: &Body<f64>) -> Vec<topo::EdgeKey> {
    body.edges()
        .filter(|(_, e)| {
            matches!(
                body.get_curve_geom(e.curve)
                    .and_then(|g| g.certified())
                    .map(|c| c.carrier()),
                Some(geom::Curve3::Line(..))
            )
        })
        .map(|(ek, _)| ek)
        .collect()
}

/// `body` with each of `edges` split at `fracs` of its parameter range,
/// placed by an oblique rigid map (an axis-aligned one keeps the pieces'
/// azimuths bitwise equal).
fn split_and_placed(body: &Body<f64>, edges: &[topo::EdgeKey], fracs: &[f64]) -> Body<f64> {
    let mut b = body.clone();
    for &ek in edges {
        let (t0, t1) = b
            .get_curve_geom(b.get_edge(ek).unwrap().curve)
            .unwrap()
            .certified()
            .unwrap()
            .params();
        for &f in fracs {
            b.split_edge(ek, t0 + (t1 - t0) * f, tol()).unwrap();
        }
    }
    let axis = Vec3::new(0.317_8_f64, 0.941_2, -0.110_9);
    let placement = Affine3::from_parts(
        Affine3::rotation_about_axis(Point3::origin(), axis * (1.0 / axis.norm()), 1.0 / 3.0)
            .linear,
        Vec3::new(0.117, -0.339_1, 5.001_7),
    );
    topo::transform_rigid(&b, &geom_core::test_support::rigid(&placement, tol())).unwrap()
}

/// **A notched wall with a split ruling stays the rectangle lane's to
/// refuse.** The pipe of `reach_probes`' notched-wall row with the bar
/// half in its wall (`A ∪ B`): its wall's loop is no iso-rectangle, and
/// the shape door refuses it (`props_rim_level`). Each line edge split
/// at its middle in turn, a ruling of the notch's sides among them: the
/// route sees a straight side that holds a point
/// (`walk::splits_a_straight_side`), and the door it then asks keeps the
/// face where the door refuses it, so every cell answers as the unsplit
/// body does. Sent to the chart lane on the route alone, the notched wall
/// would mesh there.
#[test]
fn a_notched_wall_with_a_split_ruling_refuses_at_the_shape_door() {
    use crate::reach_probes::{Got, got, op_of, rod};
    use sweep::test_support::{brick, finished};
    let pipe = finished("the pipe", rod(1.0, -2.0, 4.0), tol());
    let bar = finished(
        "the bar",
        brick::<f64>((0.5, 1.1), (-0.3, 0.3), (-0.3, 0.3), tol()),
        tol(),
    );
    let body = op_of(&pipe, &bar, "A ∪ B");
    let want = Got::NotIsoRectangle(geom::SurfaceKind::Cylinder);
    assert_eq!(got(&body, 5e-3), want, "the unsplit notched wall");
    let lines = rulings(&body);
    assert!(lines.len() >= 2, "the notch's sides are rulings");
    for (k, &ek) in lines.iter().enumerate() {
        let split = split_and_placed(&body, &[ek], &[0.5]);
        for delta in [5e-3, 1e-3] {
            assert_eq!(
                got(&split, delta),
                want,
                "line edge {k} split at its middle, δ {delta}"
            );
        }
    }
}

/// The traversals of every loop of face `fk`.
fn traversals(body: &Body<f64>, fk: topo::FaceKey) -> usize {
    body.face(fk)
        .expect("a live face")
        .loops()
        .map(
            |(lk, _, _)| match body.get_loop(lk).expect("a live loop").boundary {
                topo::LoopBoundary::Cycle { first } => {
                    body.loop_cycle(first).expect("a cycle").len()
                }
                _ => 0,
            },
        )
        .sum()
}

/// Whether `solid` is a finished body at this run's ε, whose volume
/// [`geometry_enclosing`] reads: not where its mass properties escalate
/// (the 0.1° draft wall at `10⁻¹²`, `props_cone_circle_off_axis`).
fn volume_read(solid: &Body<f64>) -> bool {
    <f64 as topo::AtRestPolicy>::finish(solid.clone(), tol()).is_ok()
}

/// **The count relation**: `split` (`solid` with points on its line
/// edges, both placed alike) meshes at `delta`, passes `check_mesh`, is
/// the body's ([`geometry_enclosing`]), and each of its faces has the
/// triangles of the same face of `solid` plus one for each traversal its
/// loops gained. A face that holds no interior point has `n − 2`
/// triangles over its `n` boundary points, and a split adds a point to
/// each traversal of its edge, so that is what a walked face and its
/// planar neighbours owe; it fails where a face is chorded otherwise than
/// the unsplit one (the chart lane's headroom on a walked face's rims) or
/// gains interior points (a lattice, a refinement).
fn holds_the_count_relation(what: &str, split: &Body<f64>, solid: &Body<f64>, delta: f64) {
    let mesh = mesh::tessellate(split, delta, tol())
        .unwrap_or_else(|e| panic!("{what}, δ {delta}: meshes, got {e:?}"));
    mesh::validate::check_mesh(&mesh).unwrap_or_else(|e| panic!("{what}, δ {delta}: {e:?}"));
    if volume_read(solid) {
        geometry_enclosing(split, solid, &mesh, delta)
            .unwrap_or_else(|e| panic!("{what}, δ {delta}: {e}"));
    } else {
        mesh::validate::check_winding(&mesh).unwrap_or_else(|e| panic!("{what}, δ {delta}: {e:?}"));
    }
    let base = mesh::tessellate(solid, delta, tol()).unwrap();
    for patch in &mesh.patches {
        let unsplit = base
            .patches
            .iter()
            .find(|p| p.face == patch.face)
            .expect("the face in the unsplit body");
        let gained = traversals(split, patch.face) - traversals(solid, patch.face);
        assert_eq!(
            patch.triangles.len(),
            unsplit.triangles.len() + gained,
            "{what}, δ {delta}: a {:?} face that gained {gained} points",
            crate::reach_probes::kind_of(split, patch.face),
        );
    }
}

/// **A ruled face whose straight side holds points meshes at the
/// triangles of the face without them, plus one per point**, and so do
/// its neighbours ([`holds_the_count_relation`]). Bodies revolved about
/// `y`:
///
/// - **one-face full turns**, whose cylinder or cone face walks its seam
///   twice, so a point on the seam is on both of its sides: a cylinder
///   wall, a frustum wall, a slender wall (outer half-angle ≈ 6°, its top
///   annulus 0.05 wide, narrower than δ 0.1) and a needle wall, each
///   face outer and reversed inner;
/// - **faces through the axis**, which a full revolve builds as two half
///   faces: cones through their apex at 45°, 80°, 5° and 0.1°, and a
///   frustum; a half cylinder wall and a half frustum, whose planar cut
///   face holds both of the curved face's sides.
///
/// Each line edge is split at one point, at two and at five, and every
/// line edge at once at one point; each body is placed obliquely; at δ
/// 0.1, 0.01 and 0.001. The same at the default ε, `10⁻⁶` and `10⁻¹²`.
#[test]
fn a_split_side_adds_one_triangle_per_point() {
    let tau = core::f64::consts::TAU;
    let cone = |h: f64| vec![(0.0, 0.0), (f64::tan(h.to_radians()), 0.0), (0.0, 1.0)];
    let wall = [(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)];
    let frustum = [(0.0, 0.0), (2.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let bodies: Vec<(&str, Body<f64>)> = vec![
        ("cylinder wall", turned(&wall, Revolution::Full)),
        (
            "frustum wall",
            turned(
                &[(1.0, 0.0), (3.0, 0.0), (2.0, 1.0), (1.5, 1.0)],
                Revolution::Full,
            ),
        ),
        (
            "slender wall",
            turned(
                &[(1.0, 0.0), (1.2, 0.0), (1.1, 1.0), (1.05, 1.0)],
                Revolution::Full,
            ),
        ),
        (
            "needle wall",
            turned(
                &[(0.1, 0.0), (0.15, 0.0), (0.12, 3.0), (0.11, 3.0)],
                Revolution::Full,
            ),
        ),
        ("cone at 45°", turned(&cone(45.0), Revolution::Full)),
        ("cone at 80°", turned(&cone(80.0), Revolution::Full)),
        ("cone at 5°", turned(&cone(5.0), Revolution::Full)),
        ("cone at 0.1°", turned(&cone(0.1), Revolution::Full)),
        ("frustum", turned(&frustum, Revolution::Full)),
        (
            "half cylinder wall",
            turned(&wall, Revolution::Partial(tau / 2.0)),
        ),
        (
            "half frustum",
            turned(&frustum, Revolution::Partial(tau / 2.0)),
        ),
    ];
    let splits: [&[f64]; 3] = [&[0.5], &[0.312_9, 0.156_45], &[0.9, 0.7, 0.5, 0.3, 0.1]];
    let mut cells = 0usize;
    for (name, body) in &bodies {
        let lines = rulings(body);
        let solid = split_and_placed(body, &[], &[]);
        let mut split: Vec<(String, Body<f64>)> = Vec::new();
        for fracs in splits {
            for (k, &ek) in lines.iter().enumerate() {
                split.push((
                    format!("{name}, line edge {k} split at {fracs:?}"),
                    split_and_placed(body, &[ek], fracs),
                ));
            }
        }
        split.push((
            format!("{name}, every line edge split at 0.5"),
            split_and_placed(body, &lines, &[0.5]),
        ));
        for (what, b) in &split {
            for delta in [0.1, 0.01, 0.001] {
                holds_the_count_relation(what, b, &solid, delta);
                cells += 1;
            }
        }
    }
    assert_eq!(cells, 330, "the cells this row reads");
}

/// **A ruling split a hair from its end meshes.** Cones through their
/// apex, revolved `2π − 0.3`, line edge 1 split a hair from its start and
/// placed obliquely, at δ 0.1, 0.01 and 0.001, hold the count relation
/// ([`holds_the_count_relation`]): half-angle 20° split `3·10⁻⁷` of its
/// range from its start, and 5° split `10⁻⁹`. The split's point
/// and the ruling's end are two corners of one thin triangle the walk
/// fans from the rim point beside them.
///
/// **By ε.** `split_edge` builds the 20° cone's split at the default ε
/// and `10⁻¹²`, the 5° cone's at `10⁻¹²` only, and the row counts the
/// bodies each ε builds. (A 1° cone's `10⁻⁹` split builds at none of the
/// three: its certification escalates at `10⁻¹²`.)
#[test]
fn a_ruling_split_a_hair_from_its_end_meshes() {
    let mut meshed = 0usize;
    for (half_angle, frac) in [(20.0f64, 3e-7), (5.0, 1e-9)] {
        let body = turned(
            &[(0.0, 0.0), (half_angle.to_radians().tan(), 0.0), (0.0, 1.0)],
            Revolution::Partial(core::f64::consts::TAU - 0.3),
        );
        let ruling = rulings(&body)[1];
        let what = format!("a {half_angle}° cone's ruling split {frac:e} from its start");
        if let Err(e) = body
            .clone()
            .split_edge(ruling, at(&body, ruling, frac), tol())
        {
            eprintln!("{what}: no split at ε {:e}: {e:?}", tol().eps());
            continue;
        }
        let split = split_and_placed(&body, &[ruling], &[frac]);
        let solid = split_and_placed(&body, &[], &[]);
        holds_everywhere(&what, &split, &solid, &[0.1, 0.01, 0.001]);
        meshed += 1;
    }
    let want = match tol().eps() {
        e if e <= 1e-12 => 2,
        e if e <= geom_core::tolerance::DEFAULT_EPS => 1,
        _ => 0,
    };
    assert_eq!(meshed, want, "the bodies this ε builds");
}

/// The parameter `frac` of the way along edge `ek`'s range.
fn at(body: &Body<f64>, ek: topo::EdgeKey, frac: f64) -> f64 {
    let (t0, t1) = body
        .get_curve_geom(body.get_edge(ek).unwrap().curve)
        .unwrap()
        .certified()
        .unwrap()
        .params();
    t0 + (t1 - t0) * frac
}

/// **Two points of one ruling closer than 2⁻²⁷ of a cone's slant radius
/// mesh.** The chart lane walks such a face in its chart, so nothing
/// rounds its slant radii (the cone arm's development cuts them to 27
/// bits, where the two points would be one). Each body is placed
/// obliquely, at the δ given, and holds the count relation
/// ([`holds_the_count_relation`]):
///
/// - the frustum wall's outer cone face, its seam split `10⁻⁹` of its
///   range from its start;
/// - a wall whose outer face is a cone of half-angle 0.1° (slant radius
///   about 1150 at its rim), its seam split `10⁻⁵` of its length from its
///   start, and split twice `10⁻⁵` apart at its middle;
/// - a cone of half-angle 2° and height 100 through its apex, each of its
///   two rulings split `3·10⁻⁸` from its rim end, at δ 0.1 and 0.01.
///
/// **By ε.** At the default ε every body but the frustum, whose split
/// `split_edge` reads as no split, and the row asserts that. At `10⁻⁶`
/// the draft wall: the 2° cone's split is under ε too. At `10⁻¹²` the
/// frustum and the draft wall: the 2° cone is outside the session box
/// there, and its volume is not read.
#[test]
fn a_ruling_split_within_two_to_the_minus_27_of_its_slant_meshes() {
    let deltas = [0.1, 0.01, 0.001];
    let frustum = turned(
        &[(1.0, 0.0), (3.0, 0.0), (2.0, 1.0), (1.5, 1.0)],
        Revolution::Full,
    );
    let seam = rulings(&frustum)[0];
    match frustum
        .clone()
        .split_edge(seam, at(&frustum, seam, 1e-9), tol())
    {
        Ok(_) => holds_everywhere(
            "the frustum wall's seam split 1e-9 from its start",
            &split_and_placed(&frustum, &[seam], &[1e-9]),
            &split_and_placed(&frustum, &[], &[]),
            &deltas,
        ),
        Err(e) => assert!(
            tol().eps() > 1e-12,
            "the frustum's split 1e-9 from its seam end builds at ε {:e}: {e:?}",
            tol().eps()
        ),
    }
    let t = 0.1f64.to_radians().tan();
    let draft = turned(
        &[(1.0, 0.0), (2.0 + t, 0.0), (2.0, 1.0), (1.0, 1.0)],
        Revolution::Full,
    );
    // The cone face's seam: the line edge whose ends differ in radius.
    let ruling = rulings(&draft)
        .into_iter()
        .find(|&ek| {
            let (a, b) = ends(&draft, ek);
            (radius(a) - radius(b)).abs() > 1e-12
        })
        .expect("the cone face's seam");
    let solid = split_and_placed(&draft, &[], &[]);
    for (what, fracs) in [
        (
            "the draft wall's seam split 1e-5 from its start",
            &[1e-5][..],
        ),
        (
            "the draft wall's seam split twice 1e-5 apart",
            &[0.5 + 1e-5, 0.5][..],
        ),
    ] {
        holds_everywhere(
            what,
            &split_and_placed(&draft, &[ruling], fracs),
            &solid,
            &deltas,
        );
    }
    let thin = turned(
        &[
            (0.0, 0.0),
            (100.0 * 2f64.to_radians().tan(), 0.0),
            (0.0, 100.0),
        ],
        Revolution::Full,
    );
    if tol().eps() < 1e-9 {
        eprintln!(
            "ε {:e}: the 2° cone of height 100 is outside the session box, its volume unread",
            tol().eps()
        );
        return;
    }
    let lines = rulings(&thin);
    let mut split = thin.clone();
    for &ek in &lines {
        let (a, b) = ends(&split, ek);
        let length = (b - a).norm();
        let frac = if radius(b) > radius(a) {
            1.0 - 3e-8 / length
        } else {
            3e-8 / length
        };
        let s = at(&split, ek, frac);
        if let Err(e) = split.split_edge(ek, s, tol()) {
            assert!(
                tol().eps() > 3e-8,
                "a split 3e-8 from the rim end builds at ε {:e}: {e:?}",
                tol().eps()
            );
            return;
        }
    }
    holds_everywhere(
        "a 2° cone of height 100, each ruling split 3e-8 from its rim end",
        &split_and_placed(&split, &[], &[]),
        &split_and_placed(&thin, &[], &[]),
        &[0.1, 0.01],
    );
}

/// The ends of line edge `ek`, at its range's ends.
fn ends(body: &Body<f64>, ek: topo::EdgeKey) -> (Point3<f64>, Point3<f64>) {
    let c = body
        .get_curve_geom(body.get_edge(ek).unwrap().curve)
        .unwrap()
        .certified()
        .unwrap();
    let (t0, t1) = c.params();
    (c.carrier().eval(t0), c.carrier().eval(t1))
}

/// A point's distance from the `y` axis, which every body here is
/// revolved about.
fn radius(p: Point3<f64>) -> f64 {
    (p.x * p.x + p.z * p.z).sqrt()
}

/// [`holds_the_count_relation`] at each of `deltas`.
fn holds_everywhere(what: &str, split: &Body<f64>, solid: &Body<f64>, deltas: &[f64]) {
    for &delta in deltas {
        holds_the_count_relation(what, split, solid, delta);
    }
}

/// A D-shaped prism: an arc of `angle` radians on a chord of length 1 in
/// the `xy` plane, closed by the chord, extruded 1 along `z`.
fn d_prism(angle: f64) -> Body<f64> {
    use profile::test_support::bulge_loop;
    use profile::{Profile, SketchPlane};
    use sweep::{ExtrudeSide, Extrusion, extrude};
    let lp = bulge_loop(vec![
        (Point2::new(0.0, 0.0), (angle / 4.0).tan()),
        (Point2::new(1.0, 0.0), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy_at(Point3::origin()), vec![lp])
        .validate(tol())
        .unwrap();
    let depth = 1.0;
    let side = ExtrudeSide::Along;
    extrude(&profile, Extrusion::Distance { depth, side }, tol())
        .unwrap()
        .body
        .into_body()
}

/// The cylinder wall's outer face cut along a second ruling `angle`
/// round from its seam (`mef`): two cylinder faces, a narrow one and a
/// wide one, each holding both of the other's straight sides.
fn banded(angle: f64) -> Body<f64> {
    let mut body = turned(
        &[(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)],
        Revolution::Full,
    );
    let (face, _) = body
        .faces()
        .find(|(fk, _)| {
            matches!(body.face_surface(*fk), Some(geom::Surface::Cylinder { radius, .. }) if *radius > 1.5)
        })
        .expect("the outer wall");
    let rims: Vec<topo::EdgeKey> = body
        .edges()
        .filter(|(ek, _)| {
            !is_line(&body, *ek)
                && body
                    .half_edges()
                    .any(|(h, he)| he.edge == *ek && body.face_of_half_edge(h) == Some(face))
        })
        .map(|(ek, _)| ek)
        .collect();
    assert_eq!(rims.len(), 2, "the outer wall's two rims");
    let mut stations = Vec::new();
    for rim in rims {
        let t = at(&body, rim, 0.0) + angle;
        stations.push(body.split_edge(rim, t, tol()).unwrap().vertex);
    }
    let leaving = |v: topo::VertexKey| {
        body.half_edges()
            .find(|(h, he)| he.start == v && body.face_of_half_edge(*h) == Some(face))
            .unwrap()
            .0
    };
    let (he1, he2) = (leaving(stations[0]), leaving(stations[1]));
    body.mef_chord(topo::construction::MefSite::Chords { he1, he2 }, tol())
        .unwrap();
    body
}

/// Whether `ek` is carried by a line.
fn is_line(body: &Body<f64>, ek: topo::EdgeKey) -> bool {
    rulings(body).contains(&ek)
}

/// **No walked triangle has an edge a neighbouring face also makes.** A
/// walked triangle's sides are boundary segments, edges between the two
/// rows, and fan edges from a side's point to the corner opposite, each
/// spanning at most one rim chord. Two points of the polygon that are
/// not neighbours on it and that one other face holds both of lie on this
/// face's two different sides (or a side and the corner across it), the
/// face's whole span apart, so the walk joins them only where its rims
/// are one chord each (`ruled_walk`, "No edge of four triangles"). The
/// tightest bodies for that, each holding the count relation exactly
/// ([`holds_the_count_relation`]: a split triangle would add two):
///
/// - a **band**: the cylinder wall's outer face cut along a ruling 0.5
///   rad from its seam ([`banded`]), so a face of one rim chord at δ 0.1
///   and a wide face each hold both of the other's sides;
/// - a **D-shaped prism** of 0.5 rad, whose flat face holds both of its
///   cylinder face's sides (at δ 0.03 and 0.01, two and three chords: at
///   one its caps are two coincident chords, which no face meshes);
/// - a **half cylinder** and a **half cone through the axis**, whose one
///   planar cut face holds both straight sides of the curved face;
/// - a **wedge** turned 0.5 rad, its rims one chord each at δ 0.1.
///
/// Each line edge split at its middle and at five points, placed
/// obliquely, at δ 1, 0.1 and 0.01 unless given.
///
/// **By ε.** The same at the default, `10⁻⁶` and `10⁻¹²`.
#[test]
fn no_walked_triangle_has_an_edge_a_neighbour_makes() {
    let half = core::f64::consts::PI;
    let every: &[f64] = &[1.0, 0.1, 0.01];
    let bodies: Vec<(&str, Body<f64>, &[f64])> = vec![
        ("a band of 0.5 rad", banded(0.5), every),
        ("a D-shaped prism of 0.5 rad", d_prism(0.5), &[0.03, 0.01]),
        (
            "a half cylinder",
            turned(
                &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
                Revolution::Partial(half),
            ),
            every,
        ),
        (
            "a half cone",
            turned(
                &[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)],
                Revolution::Partial(half),
            ),
            every,
        ),
        (
            "a wedge of 0.5 rad",
            turned(
                &[(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)],
                Revolution::Partial(0.5),
            ),
            every,
        ),
    ];
    let mut cells = 0usize;
    for (name, body, deltas) in &bodies {
        let solid = split_and_placed(body, &[], &[]);
        for (k, &ek) in rulings(body).iter().enumerate() {
            for fracs in [&[0.5][..], &[0.9, 0.7, 0.5, 0.3, 0.1][..]] {
                let what = format!("{name}, line edge {k} split at {fracs:?}");
                let split = split_and_placed(body, &[ek], fracs);
                holds_everywhere(&what, &split, &solid, deltas);
                cells += deltas.len();
            }
        }
    }
    assert_eq!(cells, 154, "the cells this row reads");
}

/// Whether line edge `ek` of a body revolved about `y` is a cone's
/// ruling: its two ends differ both in radius and in height (a cylinder's
/// ruling keeps its radius, a cap's radial edge its height).
fn on_a_cone(body: &Body<f64>, ek: topo::EdgeKey) -> bool {
    let c = body
        .get_curve_geom(body.get_edge(ek).unwrap().curve)
        .unwrap()
        .certified()
        .unwrap();
    let (t0, t1) = c.params();
    let (a, b) = (c.carrier().eval(t0), c.carrier().eval(t1));
    (a.x.hypot(a.z) - b.x.hypot(b.z)).abs() > 1e-12 && (a.y - b.y).abs() > 1e-12
}

/// The body's cone rulings ([`on_a_cone`]), in arena order.
fn cone_rulings(body: &Body<f64>) -> Vec<topo::EdgeKey> {
    rulings(body)
        .into_iter()
        .filter(|&ek| on_a_cone(body, ek))
        .collect()
}

/// Which of the suite's three ε rows this run is.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EpsRow {
    /// `10⁻⁶`.
    Coarse,
    /// The default, `10⁻⁹`.
    Default,
    /// `10⁻¹²`.
    Fine,
}

fn eps_row() -> EpsRow {
    let eps = tol().eps();
    if eps > geom_core::tolerance::DEFAULT_EPS {
        EpsRow::Coarse
    } else if eps < geom_core::tolerance::DEFAULT_EPS {
        EpsRow::Fine
    } else {
        EpsRow::Default
    }
}

/// A cone through its apex, `(0, 0), (h·tan α, 0), (0, h)`, `alpha` in
/// degrees: its ruling runs from the rim to the apex.
fn apex_cone(h: f64, alpha: f64) -> [(f64, f64); 3] {
    [(0.0, 0.0), (h * alpha.to_radians().tan(), 0.0), (0.0, h)]
}

/// **The draft cones the run rows read at this run's ε**, each a
/// half-angle in degrees with two gaps along a ruling of length about 1
/// that the cone's development merges there (a slant radius of
/// `2 / sin α` cut to 27 bits: one bin is `1.2·10⁻⁴` at 0.01° and
/// `9.5·10⁻⁷` at 1°) and `split_edge` admits: the gap of a run of two or
/// three, and the finer one of a run of five.
///
/// - the default ε: 0.01° and 1°;
/// - `10⁻⁶`: 0.01° alone, where a split is a split from `10⁻⁵`, which 1°
///   does not merge;
/// - `10⁻¹²`: 1° alone; the 0.01° cone does not revolve there.
fn draft_cones() -> &'static [(f64, f64, f64)] {
    match eps_row() {
        EpsRow::Coarse => &[(0.01, 1e-5, 1e-5)],
        EpsRow::Default => &[(0.01, 1e-5, 2.5e-6), (1.0, 1e-7, 2.5e-8)],
        EpsRow::Fine => &[(1.0, 1e-7, 2.5e-8)],
    }
}

/// A body of the run rows: its profile at half-angle `alpha` degrees.
type DraftBody<'a> = (&'a str, fn(f64) -> [(f64, f64); 4], Revolution<f64>);

/// The wall `(1, 0), (2 + tan α, 0), (2, 1), (1, 1)`: its outer face is a
/// cone of half-angle `α`.
fn wall(alpha: f64) -> [(f64, f64); 4] {
    let t = alpha.to_radians().tan();
    [(1.0, 0.0), (2.0 + t, 0.0), (2.0, 1.0), (1.0, 1.0)]
}

/// The solid `(0, 0), (2 + tan α, 0), (2, 1), (0, 1)`, through the axis.
fn solid(alpha: f64) -> [(f64, f64); 4] {
    let t = alpha.to_radians().tan();
    [(0.0, 0.0), (2.0 + t, 0.0), (2.0, 1.0), (0.0, 1.0)]
}

/// The wall `(2 + tan α, 0), (3, 0), (3, 1), (2, 1)`: its INNER face is
/// the cone, reversed.
fn bored(alpha: f64) -> [(f64, f64); 4] {
    let t = alpha.to_radians().tan();
    [(2.0 + t, 0.0), (3.0, 0.0), (3.0, 1.0), (2.0, 1.0)]
}

/// The cells of the run rows: each of `bodies` at each of
/// [`draft_cones`], each of its cone rulings split seven ways, placed
/// obliquely, at each of `deltas`. Every cell is asserted `Meshed`;
/// answers how many there were.
///
/// The seven: a split beside the ruling's start; one beside its end (the
/// run then holds a corner of the polygon); two and three beside each
/// other at its middle; one beside each end; five beside each other; and
/// two beside each other and the end, a run of three at a corner.
fn run_cells(bodies: &[DraftBody<'_>], deltas: &[f64]) -> usize {
    let mut cells = 0usize;
    for &(alpha, gap, fine) in draft_cones() {
        for &(name, profile, turn) in bodies {
            let body = turned(&profile(alpha), turn);
            let whole = split_and_placed(&body, &[], &[]);
            let five: Vec<f64> = (0..5).rev().map(|k| 0.5 + f64::from(k) * fine).collect();
            let splits: [(&str, &[f64]); 7] = [
                ("a split beside its start", &[gap]),
                ("a split beside its end", &[1.0 - gap]),
                ("two splits beside each other", &[0.5 + gap, 0.5]),
                (
                    "three beside each other",
                    &[0.5 + 2.0 * gap, 0.5 + gap, 0.5],
                ),
                ("a split beside each end", &[1.0 - gap, gap]),
                ("five beside each other", &five),
                (
                    "two beside each other and the end",
                    &[1.0 - fine, 1.0 - 2.0 * fine],
                ),
            ];
            for (k, ek) in cone_rulings(&body).into_iter().enumerate() {
                for (what, fracs) in splits {
                    let split = split_and_placed(&body, &[ek], fracs);
                    for &delta in deltas {
                        assert_eq!(
                            got_enclosing(&split, &whole, delta),
                            Got::Meshed,
                            "the {alpha}° {name}, cone ruling {k}, {what}, δ {delta}"
                        );
                        cells += 1;
                    }
                }
            }
        }
    }
    cells
}

/// **A run of ruling points the development merges meshes.** The chart
/// lane triangulates a cone face in its development, which cuts a slant
/// radius to 27 significant bits: two points of one ruling within about
/// `2⁻²⁷` of their slant radius are one point there. On a draft cone that
/// is a length a split reaches ([`draft_cones`]). The lane holds such a
/// run of consecutive ruling points as one vertex of its triangulation
/// and tells them apart again when it emits (`trimmed::Run`).
///
/// The wall, a full turn: its one cone face walks its seam twice, so each
/// run is met on each of the seam's two rays. The seam split seven ways
/// ([`run_cells`]); placed obliquely; δ 0.01. Every cell meshes, passes
/// `check_mesh`, and is the body's ([`crate::reach_probes::geometry`]).
///
/// **By ε.** The same at each, on the cones that ε builds
/// ([`draft_cones`]). The grid over more bodies and δ is
/// [`every_run_of_a_draft_cones_rulings_meshes`].
#[test]
fn a_run_of_ruling_points_the_development_merges_meshes() {
    let cells = run_cells(&[("wall, full", wall, Revolution::Full)], &[0.01]);
    assert_eq!(cells, 7 * draft_cones().len(), "the cells this row reads");
}

/// **The grid of [`a_run_of_ruling_points_the_development_merges_meshes`]**:
/// the wall a full turn, a half and a wedge of 0.05; the solid through
/// the axis a full turn and a half (a part turn's cone face ends on two
/// rulings, each shared with a planar cut face); and the wall whose inner
/// face is the cone, reversed, a full turn; at δ 1, 0.1, 0.01 and 0.001.
///
/// **By ε.** The same at each, on the cones that ε builds
/// ([`draft_cones`]); but the wedge is read at the default ε and `10⁻¹²`
/// alone: at `10⁻⁶` the 0.01° wedge does not revolve.
#[test]
fn every_run_of_a_draft_cones_rulings_meshes() {
    let half = Revolution::Partial(core::f64::consts::PI);
    let wedge: DraftBody<'_> = ("wall, a wedge of 0.05", wall, Revolution::Partial(0.05));
    let mut bodies: Vec<DraftBody<'_>> = vec![
        ("wall, full", wall, Revolution::Full),
        ("wall, half", wall, half),
        ("solid, full", solid, Revolution::Full),
        ("solid, half", solid, half),
        ("bored wall, full", bored, Revolution::Full),
    ];
    // Cone rulings: one on a full-turn wall, two on every other body.
    let mut rulings = 1 + 2 + 2 + 2 + 1;
    if eps_row() != EpsRow::Coarse {
        bodies.push(wedge);
        rulings += 2;
    }
    let cells = run_cells(&bodies, &[1.0, 0.1, 0.01, 0.001]);
    assert_eq!(
        cells,
        rulings * 7 * 4 * draft_cones().len(),
        "the cells this row reads"
    );
}

/// **A run at a corner of the polygon is anchored along its ruling.** The
/// wall at 26.6°, a wedge of 0.05, each of its two cone rulings split
/// beside its start and beside its end in turn and placed obliquely, δ
/// 0.1, 0.01 and 0.001. The split vertex and the ruling's end are a run,
/// and the end is a corner: on one side the polygon reaches the run along
/// the rim and leaves it along the ruling, on the other the reverse. The
/// lane adds the run's triangle beside the neighbour along the ruling
/// (`trimmed::Anchor`), and every cell meshes. Added beside the neighbour
/// before the run whichever side that is on, the second ruling split
/// beside its end refuses at δ 0.01.
///
/// Inside a ruling side both neighbours are along the ruling and the lane
/// takes the first; either gives a valid mesh, and which is held by
/// `trimmed`'s unit rows alone.
///
/// **By ε.** The gap is `10⁻⁸` of the ruling at the default ε and at
/// `10⁻¹²`. At `10⁻⁶` a split is a split from `10⁻⁵`, which this cone's
/// development does not merge: the row holds that the face meshes with
/// its two points apart.
#[test]
fn a_run_at_a_corner_is_anchored_along_its_ruling() {
    let gap = match eps_row() {
        EpsRow::Coarse => 1e-5,
        EpsRow::Default | EpsRow::Fine => 1e-8,
    };
    let body = turned(&wall(26.6), Revolution::Partial(0.05));
    let whole = split_and_placed(&body, &[], &[]);
    let lines = cone_rulings(&body);
    assert_eq!(lines.len(), 2, "the wedge's cone face ends on two rulings");
    for (k, ek) in lines.into_iter().enumerate() {
        for frac in [gap, 1.0 - gap] {
            let split = split_and_placed(&body, &[ek], &[frac]);
            for delta in [0.1, 0.01, 0.001] {
                assert_eq!(
                    got_enclosing(&split, &whole, delta),
                    Got::Meshed,
                    "cone ruling {k} split at {frac}, δ {delta}"
                );
            }
        }
    }
}

/// The cells of the cut-face rows: the 1° wall revolved `turn`, under a
/// slab tilted 0.1 about an oblique axis (`A ∩ B`), each ruling of the cut
/// body's that rises along a cone split beside its start and beside its
/// end in turn, placed obliquely, δ 0.01, each asserted `Meshed`.
fn cut_cone_cells(turn: Revolution<f64>) {
    use crate::reach_probes::{op_of, posed};
    use sweep::test_support::{brick, finished};
    let gap = match eps_row() {
        EpsRow::Coarse => 1e-4,
        EpsRow::Default | EpsRow::Fine => 1e-7,
    };
    let tilt = Affine3::translation(Vec3::new(0.0, 0.5, 0.0))
        * Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.6, 0.0, 0.8), 0.1);
    let cone = finished("the wall", turned(&wall(1.0), turn), tol());
    let slab = posed(
        "the slab",
        brick((-6.0, 6.0), (-6.0, 0.0), (-6.0, 6.0), tol()),
        tilt,
    );
    let cut = op_of(&cone, &slab, "A ∩ B");
    let whole = split_and_placed(&cut, &[], &[]);
    let lines = cone_rulings(&cut);
    assert!(!lines.is_empty(), "the cut face keeps a ruling");
    for (k, ek) in lines.into_iter().enumerate() {
        for frac in [gap, 1.0 - gap] {
            let split = split_and_placed(&cut, &[ek], &[frac]);
            assert_eq!(
                got_enclosing(&split, &whole, 0.01),
                Got::Meshed,
                "ruling {k} split at {frac}"
            );
        }
    }
}

/// **A run on a cone face that is no rectangle meshes.** The 1° wall, a
/// half turn, under a slab tilted about an oblique axis
/// ([`cut_cone_cells`]): the cone face keeps its lower rim and its two
/// rulings, and its upper boundary is the slab's section of the cone, so
/// it is the chart lane's by its trim carrier whether or not a ruling is
/// split. A ruling split beside an end: the split vertex and the
/// ruling's end are a run, and every cell meshes, passes `check_mesh`,
/// and is the body's ([`crate::reach_probes::geometry`]).
///
/// **By ε.** The gap is `10⁻⁷` of the ruling at the default ε and at
/// `10⁻¹²`. At `10⁻⁶` the smallest split of the cut rulings is wider
/// than this cone's development merges (the gap is `10⁻⁴` of the
/// ruling): the row holds that the cut face meshes with its two points
/// apart.
#[test]
fn a_run_on_a_cone_face_cut_by_an_oblique_plane_meshes() {
    cut_cone_cells(Revolution::Partial(core::f64::consts::PI));
}

/// **[`a_run_on_a_cone_face_cut_by_an_oblique_plane_meshes`], a full
/// turn**: one cone face, cut, walking its seam twice, so the run is met
/// on each of the seam's two rays of a polygon that is no rectangle.
///
/// **By ε.** As that row.
#[test]
fn a_run_on_a_cut_cone_faces_seam_meshes() {
    cut_cone_cells(Revolution::Full);
}

/// **A run on an ordinary frustum, at ε `10⁻¹²`.** The frustum wall's
/// outer cone face (half-angle 45°) with its seam split beside its start,
/// placed obliquely, at δ 0.1, 0.01 and 0.001.
///
/// **By ε.** At `10⁻¹²` the gap is `10⁻⁹` of the seam: the split vertex
/// and the seam's end lie within `2⁻²⁷` of their slant radius, a run, and
/// the face meshes as
/// [`a_run_of_ruling_points_the_development_merges_meshes`] says. At the
/// default ε and `10⁻⁶` the smallest split of this seam (`10⁻⁸` and
/// `10⁻⁵` of it) is wider than this cone's development merges: the row
/// holds that the face meshes with its two points apart.
#[test]
fn a_frustum_seam_split_within_the_developments_resolution_meshes() {
    let gap = match eps_row() {
        EpsRow::Coarse => 1e-5,
        EpsRow::Default => 1e-8,
        EpsRow::Fine => 1e-9,
    };
    let body = turned(
        &[(1.0, 0.0), (3.0, 0.0), (2.0, 1.0), (1.5, 1.0)],
        Revolution::Full,
    );
    let seam = rulings(&body)[0];
    let split = split_and_placed(&body, &[seam], &[gap]);
    let whole = split_and_placed(&body, &[], &[]);
    for delta in [0.1, 0.01, 0.001] {
        assert_eq!(
            got_enclosing(&split, &whole, delta),
            Got::Meshed,
            "δ {delta}"
        );
    }
}

/// **A draft cone the mass properties cannot read still meshes its
/// runs.** The 0.1° wall, a full turn, its seam split `10⁻⁵` of its
/// length from its start, and twice `10⁻⁵` apart at its middle, placed
/// obliquely, at δ 0.1, 0.01 and 0.001.
///
/// **By ε.** At the default and `10⁻⁶` every cell is `Meshed`
/// ([`got_enclosing`]). At `10⁻¹²` the unsplit wall is no finished body
/// (its volume is not computable there), so the volume is not read: the
/// mesh passes `check_mesh` and `check_winding`.
#[test]
fn a_draft_cone_ruling_split_within_the_developments_resolution_meshes() {
    let body = turned(&wall(0.1), Revolution::Full);
    let [ruling] = cone_rulings(&body)[..] else {
        panic!("the wall's cone face has one seam");
    };
    let whole = split_and_placed(&body, &[], &[]);
    for (what, fracs) in [
        ("an end split", &[1e-5][..]),
        ("two splits mid-ruling", &[0.5 + 1e-5, 0.5][..]),
    ] {
        let split = split_and_placed(&body, &[ruling], fracs);
        for delta in [0.1, 0.01, 0.001] {
            if eps_row() == EpsRow::Fine {
                let m = mesh::tessellate(&split, delta, tol())
                    .unwrap_or_else(|e| panic!("{what}, δ {delta}: {e:?}"));
                mesh::validate::check_mesh(&m)
                    .unwrap_or_else(|e| panic!("{what}, δ {delta}: {e:?}"));
                mesh::validate::check_winding(&m)
                    .unwrap_or_else(|e| panic!("{what}, δ {delta}: {e:?}"));
            } else {
                assert_eq!(
                    got_enclosing(&split, &whole, delta),
                    Got::Meshed,
                    "{what}, δ {delta}"
                );
            }
        }
    }
}

/// **Two runs beside the apex, anchored by it, mesh.** A cone of
/// half-angle 10° and height 100 through its apex, revolved 0.05: a thin
/// wedge whose cone face is the apex, two rulings and one rim chord. Both
/// rulings split beside their rim ends, placed obliquely: each split
/// vertex and its ruling's end are a run whose neighbour along the ruling
/// is the apex, and the development's one triangle at the apex has both
/// runs for corners. The lane cuts it along both, and the face is three
/// triangles at δ 10, 1 and 0.1.
///
/// **By ε.** The gap is `10⁻⁷` along the ruling at the default ε and at
/// `10⁻¹²`. At `10⁻⁶` a split is a split from `10⁻⁵`, which this cone's
/// development does not merge: the row holds that the face is the same
/// three triangles with its points apart.
#[test]
fn two_runs_anchored_by_the_apex_mesh() {
    let gap = match eps_row() {
        EpsRow::Coarse => 1e-5,
        EpsRow::Default | EpsRow::Fine => 1e-7,
    };
    let profile = apex_cone(100.0, 10.0);
    let body = turned(&profile, Revolution::Partial(0.05));
    let lines = cone_rulings(&body);
    assert_eq!(lines.len(), 2, "the wedge's cone face ends on two rulings");
    let length = profile[1].0.hypot(100.0);
    let split = split_and_placed(&body, &lines, &[1.0 - gap / length]);
    let whole = split_and_placed(&body, &[], &[]);
    for delta in [10.0, 1.0, 0.1] {
        assert_eq!(
            got_enclosing(&split, &whole, delta),
            Got::Meshed,
            "δ {delta}"
        );
        let m = mesh::tessellate(&split, delta, tol()).unwrap();
        let cone = m
            .patches
            .iter()
            .find(|p| crate::reach_probes::kind_of(&split, p.face) == geom::SurfaceKind::Cone)
            .expect("the cone face's patch");
        assert_eq!(
            cone.triangles.len(),
            3,
            "δ {delta}: the cone face's triangles"
        );
    }
}

/// **A split beside a large cone's rim end, whose development made a run
/// anchored on a needle triangle, meshes.** A cone of half-angle 45°
/// through its apex, a full turn (two half faces), one ruling split beside
/// its rim end, placed obliquely: the faces are walked
/// (`trimmed::walked`), and hold the count relation
/// ([`holds_the_count_relation`]; the volume where the body finishes).
/// Triangulated in the development, the split vertex and the ruling's end
/// were a run whose ruling-side triangle took a lattice node near the
/// apex, leaving triangles f64 cannot wind
/// (`work/tesslane/a-run-anchored-on-a-needle-triangle-makes-triangles-f64-cannot-wind.md`,
/// whose class is still open on a cone face the walk does not take).
///
/// **By ε**, each ε on the cone and gap that reached the refusal there:
/// the default, height 1000 and a gap of `10⁻⁸` along the ruling, at δ
/// 100, 10 and 1; `10⁻¹²`, height 1 and a gap of `10⁻¹⁰` of the ruling, at
/// δ 0.1, 0.01 and 0.001; `10⁻⁶`, height 1 and a gap of `10⁻⁵`, the same δ.
#[test]
fn a_split_beside_a_large_cones_rim_end_meshes() {
    let (height, gap, deltas): (f64, f64, &[f64]) = match eps_row() {
        EpsRow::Coarse => (1.0, 1e-5, &[0.1, 0.01, 0.001]),
        EpsRow::Default => (1000.0, 1e-8 / (1000.0 * 2f64.sqrt()), &[100.0, 10.0, 1.0]),
        EpsRow::Fine => (1.0, 1e-10, &[0.1, 0.01, 0.001]),
    };
    let body = turned(&apex_cone(height, 45.0), Revolution::Full);
    let ruling = cone_rulings(&body)[0];
    let split = split_and_placed(&body, &[ruling], &[1.0 - gap]);
    let whole = split_and_placed(&body, &[], &[]);
    holds_everywhere("a ruling split beside its rim end", &split, &whole, deltas);
}

/// **A cone band shorter than its development resolves meshes.** A wall
/// whose outer face is a cone of half-angle 1° and height `h`, a full
/// turn, its seam split at its middle and placed obliquely. The whole
/// ruling side lies inside one cut of the development's slant radius,
/// where the split vertex and the side's two ends were one point and no
/// run, and the face refused `DevelopedMerge`. It is walked
/// (`trimmed::walked`) in its chart, and holds the count relation
/// ([`holds_the_count_relation`]).
///
/// **By ε**, each ε on the band it builds: the default and `10⁻⁶`, radius
/// 200 (`(100, 0), (200 + h·tan 1°, 0), (200, h), (100, h)`),
/// `h = 5·10⁻⁵`, δ 10 and 1; `10⁻¹²`, radius 2, `h = 10⁻⁷`, δ 0.1, 0.01
/// and 0.001.
#[test]
fn a_cone_band_shorter_than_the_developments_resolution_meshes() {
    let (radius, h, deltas): (f64, f64, &[f64]) = match eps_row() {
        EpsRow::Coarse | EpsRow::Default => (200.0, 5e-5, &[10.0, 1.0]),
        EpsRow::Fine => (2.0, 1e-7, &[0.1, 0.01, 0.001]),
    };
    let inner = radius / 2.0;
    let body = turned(
        &[
            (inner, 0.0),
            (radius + h * 1f64.to_radians().tan(), 0.0),
            (radius, h),
            (inner, h),
        ],
        Revolution::Full,
    );
    let [seam] = cone_rulings(&body)[..] else {
        panic!("the band's cone face has one seam");
    };
    let split = split_and_placed(&body, &[seam], &[0.5]);
    let whole = split_and_placed(&body, &[], &[]);
    holds_everywhere("a band's seam split at its middle", &split, &whole, deltas);
}
