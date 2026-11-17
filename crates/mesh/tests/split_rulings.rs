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
use crate::reach_probes::{geometry_enclosing, tol};

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
    let face = body.get_face(fk).expect("a live face");
    std::iter::once(face.outer)
        .chain(face.rings.iter().copied())
        .map(|lk| match body.get_loop(lk).expect("a live loop").boundary {
            topo::LoopBoundary::Cycle { first } => body.loop_cycle(first).expect("a cycle").len(),
            _ => 0,
        })
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

/// **A ruling split a hair from its end meshes.** A cone of half-angle
/// 20° through its apex, revolved `2π − 0.3`, its line edge 1 split
/// `3·10⁻⁷` of its range from its start and placed obliquely, at δ 0.1,
/// 0.01 and 0.001: the count relation ([`holds_the_count_relation`]).
/// The split's point and the ruling's end are two corners of one thin
/// triangle the walk fans from the rim point beside them.
///
/// **By ε.** The same at `10⁻¹²`; at `10⁻⁶` `split_edge` reads the split
/// as no split, and the row asserts that instead.
#[test]
fn a_ruling_split_a_hair_from_its_end_meshes() {
    let body = turned(
        &[(0.0, 0.0), (f64::tan(20f64.to_radians()), 0.0), (0.0, 1.0)],
        Revolution::Partial(core::f64::consts::TAU - 0.3),
    );
    let ruling = rulings(&body)[1];
    if let Err(e) = body.clone().split_edge(ruling, at(&body, ruling, 3e-7), tol()) {
        assert!(
            tol().eps() != geom_core::tolerance::DEFAULT_EPS,
            "the split is interior at the default ε: {e:?}"
        );
        eprintln!(
            "ε {:e}: `split_edge` reads a split 3e-7 from the end as no split ({e:?})",
            tol().eps()
        );
        return;
    }
    let split = split_and_placed(&body, &[ruling], &[3e-7]);
    let solid = split_and_placed(&body, &[], &[]);
    for delta in [0.1, 0.01, 0.001] {
        holds_the_count_relation("a ruling split 3e-7 from its end", &split, &solid, delta);
    }
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
    match frustum.clone().split_edge(seam, at(&frustum, seam, 1e-9), tol()) {
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
        ("the draft wall's seam split 1e-5 from its start", &[1e-5][..]),
        ("the draft wall's seam split twice 1e-5 apart", &[0.5 + 1e-5, 0.5][..]),
    ] {
        holds_everywhere(what, &split_and_placed(&draft, &[ruling], fracs), &solid, &deltas);
    }
    let thin = turned(
        &[(0.0, 0.0), (100.0 * 2f64.to_radians().tan(), 0.0), (0.0, 100.0)],
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
