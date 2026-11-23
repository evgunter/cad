//! **A ruled face whose straight side holds a point**: a cylinder or
//! cone face with a ruling carried by more than one edge, which the
//! route sends to the chart lane (`tessellate::route_of`'s
//! `walk::splits_a_straight_side` fact), where it is walked (`ruled_walk`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Vec3};
use profile::ProfileLoop;
use profile::RawLoop as _;
use sweep::{Revolution, revolve};
use topo::Body;
use topo::construction::Construct;

use crate::common::{axis_y, validated};
use crate::reach_probes::{Got, geometry_but_the_volume, geometry_enclosing, got_enclosing, tol};

/// The profile `pts` revolved about `y` through `turn`.
pub(super) fn turned(pts: &[(f64, f64)], turn: Revolution<f64>) -> Body<f64> {
    let lp = ProfileLoop::polygon(pts.iter().map(|&(x, y)| Point2::new(x, y)));
    revolve(&validated(vec![lp]), axis_y(), turn, tol())
        .unwrap()
        .body
        .into_body()
}

/// The body's line edges, in arena order.
pub(super) fn rulings(body: &Body<f64>) -> Vec<topo::EdgeKey> {
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
pub(super) fn split_and_placed(
    body: &Body<f64>,
    edges: &[topo::EdgeKey],
    fracs: &[f64],
) -> Body<f64> {
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

/// **A notched wall with a split ruling answers as the unsplit wall
/// does.** The pipe of `reach_probes`' notched-wall row with the bar half
/// in its wall (`A ∪ B`): its wall's loop is no iso-rectangle, the doors
/// decide so (`props_rim_level`), and the route sends it to the chart
/// lane on that answer, which meshes it. Each line edge split at its
/// middle in turn, a ruling of the notch's sides among them: the split
/// side is a fact the route reads only of a rectangle
/// (`tessellate::route_of`'s `walk::splits_a_straight_side`
/// fact), so the doors' answer still routes the face and every cell
/// answers as the unsplit body does.
#[test]
fn a_notched_wall_with_a_split_ruling_answers_as_the_unsplit_wall() {
    use crate::reach_probes::{got, op_of, rod};
    use sweep::test_support::{brick, finished};
    let pipe = finished("the pipe", rod(1.0, -2.0, 4.0), tol());
    let bar = finished(
        "the bar",
        brick::<f64>((0.5, 1.1), (-0.3, 0.3), (-0.3, 0.3), tol()),
        tol(),
    );
    let body = op_of(&pipe, &bar, "A ∪ B");
    let want = Got::Meshed;
    assert_eq!(got(&body, 5e-3), want, "the unsplit notched wall");
    let lines = rulings(&body);
    assert!(lines.len() >= 2, "the notch's sides are rulings");
    for (k, &ek) in lines.iter().enumerate() {
        let split = split_and_placed(&body, &[ek], &[0.5]);
        for delta in [5e-3, 1e-3] {
            assert_eq!(
                got_enclosing(&split, &body, delta),
                want,
                "line edge {k} split at its middle, δ {delta}"
            );
        }
    }
}

/// The points of face `fk`'s boundary polygon in `mesh`: each traversal
/// of an edge of its loops, at that edge's chord count.
fn boundary_points(body: &Body<f64>, mesh: &mesh::Mesh, fk: topo::FaceKey) -> usize {
    let chords = |ek: topo::EdgeKey| {
        mesh.boundaries
            .iter()
            .find(|b| b.edge == ek)
            .expect("every edge has its polyline")
            .points
            .len()
            - 1
    };
    body.face(fk)
        .expect("a live face")
        .loops()
        .map(
            |(lk, _, _)| match body.get_loop(lk).expect("a live loop").boundary {
                topo::LoopBoundary::Cycle { first } => body
                    .loop_cycle(first)
                    .expect("a cycle")
                    .into_iter()
                    .map(|he| chords(body.get_half_edge(he).expect("a live half-edge").edge))
                    .sum(),
                _ => 0,
            },
        )
        .sum()
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

std::thread_local! {
    /// The cells of this row whose volume [`holds_the_relation`] did not
    /// read ([`volume_read`]); each row that can meet one asserts the count
    /// ([`unread_volumes`]).
    static VOLUME_UNREAD: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
}

/// The cells of this row so far whose volume was not read.
fn unread_volumes() -> usize {
    VOLUME_UNREAD.with(core::cell::Cell::get)
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
    holds_the_relation(what, split, solid, delta, 0);
}

/// The points of `face`'s patch on no edge's polyline: the points the
/// lane laid inside the face and the join's split points. A walked face
/// lays none of its own (`trimmed::walk_face`): every point inside it is
/// a side the join split (`mesh::join`).
fn interior_points(mesh: &mesh::Mesh, face: topo::FaceKey) -> usize {
    let on_edges: std::collections::HashSet<u32> = mesh
        .boundaries
        .iter()
        .flat_map(|b| b.points.iter().copied())
        .collect();
    let mut inside: Vec<u32> = mesh
        .patches
        .iter()
        .filter(|p| p.face == face)
        .flat_map(|p| p.triangles.iter().flatten().copied())
        .filter(|i| !on_edges.contains(i))
        .collect();
    inside.sort_unstable();
    inside.dedup();
    inside.len()
}

/// [`holds_the_count_relation`] where the split ruled faces gain `cuts`
/// points between them where the patches join (`mesh::join`): a side the
/// join's census splits in that face, at a point between its ends
/// ([`interior_points`]), two triangles past the relation per point;
/// every other face holds the relation exactly. So a split the join does
/// not make, and one it makes that the pin does not ask for, are both red.
///
/// A face's points inside it in either body, cut or split where the
/// patches join (`mesh::join`), count two triangles each against the
/// other body's.
fn holds_the_relation(what: &str, split: &Body<f64>, solid: &Body<f64>, delta: f64, cuts: usize) {
    let mesh = mesh::tessellate(split, delta, tol())
        .unwrap_or_else(|e| panic!("{what}, δ {delta}: meshes, got {e:?}"));
    mesh::validate::check_mesh(&mesh).unwrap_or_else(|e| panic!("{what}, δ {delta}: {e:?}"));
    if volume_read(solid) {
        geometry_enclosing(split, solid, &mesh, delta)
            .unwrap_or_else(|e| panic!("{what}, δ {delta}: {e}"));
    } else {
        geometry_but_the_volume(split, &mesh, delta)
            .unwrap_or_else(|e| panic!("{what}, δ {delta}: {e}"));
        VOLUME_UNREAD.with(|n| n.set(n.get() + 1));
    }
    let walked = |face| {
        let kind = crate::reach_probes::kind_of(split, face);
        matches!(kind, geom::SurfaceKind::Cylinder | geom::SurfaceKind::Cone)
            && traversals(split, face) > traversals(solid, face)
    };
    let cut: usize = mesh
        .patches
        .iter()
        .filter(|p| walked(p.face))
        .map(|p| interior_points(&mesh, p.face))
        .sum();
    if std::env::var_os("PRINT_CUTS").is_some() {
        eprintln!("CUTS\t{what}\t{delta}\t{cut}");
        return;
    }
    assert_eq!(cut, cuts, "{what}, δ {delta}: the sides the join split in the walked faces");
    let base = mesh::tessellate(solid, delta, tol())
        .unwrap_or_else(|e| panic!("{what}, δ {delta}: the unsplit body refuses {e:?}"));
    for patch in &mesh.patches {
        let unsplit = base
            .patches
            .iter()
            .find(|p| p.face == patch.face)
            .expect("the face in the unsplit body");
        // The face's boundary points past the unsplit body's: one per
        // traversal of a split edge, and any a chord count moved by.
        let count = |n: usize| i64::try_from(n).expect("a count");
        let gained = count(boundary_points(split, &mesh, patch.face))
            - count(boundary_points(solid, &base, patch.face));
        // A face's lattice is the same in both bodies, so the interior
        // points one holds past the other are its cut or split points.
        let points = |m: &mesh::Mesh| count(interior_points(m, patch.face));
        let owed = count(unsplit.triangles.len()) + gained + 2 * (points(&mesh) - points(&base));
        assert_eq!(
            count(patch.triangles.len()),
            owed,
            "{what}, δ {delta}: a {:?} face that gained {gained} points",
            crate::reach_probes::kind_of(split, patch.face)
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
    assert_eq!(
        unread_volumes(),
        0,
        "the cells whose volume is not read: none"
    );
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
    let (mut meshed, mut refused) = (0usize, Vec::new());
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
            refused.push((
                half_angle,
                format!("{e:?}")
                    .split([' ', '{', '('])
                    .next()
                    .unwrap_or("")
                    .to_string(),
            ));
            continue;
        }
        let split = split_and_placed(&body, &[ruling], &[frac]);
        let solid = split_and_placed(&body, &[], &[]);
        holds_everywhere(&what, &split, &solid, &[0.1, 0.01, 0.001]);
        meshed += 1;
    }
    let (want, refusals): (usize, &[(f64, &str)]) = match eps_row() {
        EpsRow::Fine => (2, &[]),
        EpsRow::Default => (1, &[(5.0, "SplitParamEscalated")]),
        EpsRow::Coarse => (
            0,
            &[
                (20.0, "SplitParamNotInterior"),
                (5.0, "SplitParamNotInterior"),
            ],
        ),
    };
    assert_eq!(meshed, want, "the bodies this ε builds");
    let refused: Vec<(f64, &str)> = refused.iter().map(|(a, e)| (*a, e.as_str())).collect();
    assert_eq!(
        refused, refusals,
        "the splits `split_edge` refuses at this ε, and why"
    );
    assert_eq!(
        unread_volumes(),
        0,
        "the cells whose volume is not read: none"
    );
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
    assert_eq!(
        unread_volumes(),
        if eps_row() == EpsRow::Fine { 6 } else { 0 },
        "the cells whose volume is not read: the draft wall's at 1e-12"
    );
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

/// **No walked triangle has an edge a neighbouring face also makes**, on
/// the tightest bodies with a neighbour holding both of a walked face's
/// sides: a side between the corners of a one-chord face, which its
/// neighbour holds both ends of. No neighbour here makes such a side, so
/// nothing is split where the patches join: each cell passes `check_mesh`
/// (an edge of four triangles fails it), is the body's, and holds the
/// count relation exactly ([`holds_the_count_relation`]). The bored ball
/// ([`a_walked_face_whose_neighbour_holds_both_rims_shares_no_edge`]) is
/// the case of a neighbour holding both rims.
///
/// - a **band**: the cylinder wall's outer face cut along a ruling 0.5
///   rad from its seam ([`banded`]), so a face of one rim chord at δ 0.1
///   and a wide face each hold both of the other's sides;
/// - a **D-shaped prism** of 0.5 rad, whose flat face holds both of its
///   cylinder face's sides (at δ 0.03 and 0.01, two and three chords: at
///   one its caps are two coincident chords, which no face meshes);
/// - a **half cylinder** and a **half cone through the axis**, whose one
///   planar cut face holds both straight sides of the curved face;
/// - a **wedge** turned 0.5 rad, its rims one chord each at δ 0.1;
/// - a **cone through its apex** turned 0.5 rad, its rim one chord at δ 1
///   and 0.1, so one walked triangle holds both of its sides.
///
/// Each line edge split at its middle and at three points, and every line
/// edge at once (both sides of a one-chord face among them), placed
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
        (
            "a cone through its apex turned 0.5 rad",
            turned(&apex_cone(1.0, 30.0), Revolution::Partial(0.5)),
            every,
        ),
    ];
    let mut cells = 0usize;
    for (name, body, deltas) in &bodies {
        let solid = split_and_placed(body, &[], &[]);
        let lines = rulings(body);
        let mut splits: Vec<(String, Vec<topo::EdgeKey>)> = lines
            .iter()
            .enumerate()
            .map(|(k, &ek)| (format!("line edge {k}"), vec![ek]))
            .collect();
        splits.push(("every line edge".to_string(), lines.clone()));
        for (which, edges) in &splits {
            for fracs in [&[0.5][..], &[0.9, 0.5, 0.1][..]] {
                let what = format!("{name}, {which} split at {fracs:?}");
                let split = split_and_placed(body, edges, fracs);
                for &delta in *deltas {
                    holds_the_count_relation(&what, &split, &solid, delta);
                }
                cells += deltas.len();
            }
        }
    }
    assert_eq!(cells, 218, "the cells this row reads");
    assert_eq!(
        unread_volumes(),
        24,
        "the cells whose volume is not read: the band's, which does not finish"
    );
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

/// **A draft cone's seam split within its development's resolution
/// meshes.** On a draft cone two points of one ruling within about `2⁻²⁷`
/// of their slant radius, a length a split reaches ([`draft_cones`]), are
/// one point of the cone's development, where the chart lane's CDT runs.
/// A cone face with a split side and no other trim is walked in its chart
/// (`tessellate::Lane::Walked`), which rounds no slant radius, so the points stay
/// apart.
///
/// The wall, a full turn: its one cone face walks its seam twice. The seam
/// split seven ways ([`run_cells`]); placed obliquely; δ 0.01. Every cell
/// meshes, passes `check_mesh`, and is the body's
/// ([`crate::reach_probes::geometry`]).
///
/// **By ε.** The same at each, on the cones that ε builds
/// ([`draft_cones`]). The grid over more bodies and δ is
/// [`every_close_split_of_a_draft_cones_rulings_meshes`].
#[test]
fn a_draft_cones_seam_split_within_its_developments_resolution_meshes() {
    let cells = run_cells(&[("wall, full", wall, Revolution::Full)], &[0.01]);
    assert_eq!(cells, 7 * draft_cones().len(), "the cells this row reads");
}

/// **The grid of [`a_draft_cones_seam_split_within_its_developments_resolution_meshes`]**:
/// the wall a full turn, a half and a wedge of 0.05; the solid through
/// the axis a full turn and a half (a part turn's cone face ends on two
/// rulings, each shared with a planar cut face); and the wall whose inner
/// face is the cone, reversed, a full turn; at δ 1, 0.1, 0.01 and 0.001.
/// Each face is walked.
///
/// **By ε.** The same at each, on the cones that ε builds
/// ([`draft_cones`]); but the wedge is read at the default ε and `10⁻¹²`
/// alone: at `10⁻⁶` the 0.01° wedge does not revolve.
#[test]
fn every_close_split_of_a_draft_cones_rulings_meshes() {
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

/// **A split beside a corner of a draft cone meshes.** The wall at
/// 26.6°, a wedge of 0.05, each of its two cone rulings split beside its
/// start and beside its end in turn and placed obliquely, δ 0.1, 0.01 and
/// 0.001. The split vertex sits a hair from a corner where the polygon
/// turns between a rim and a ruling; the walk fans it from the corner
/// across the side, and every cell meshes (`Meshed`: `check_mesh`, the
/// winding, sampled deviation and the volume).
///
/// **By ε.** The gap is `10⁻⁸` of the ruling at the default ε and at
/// `10⁻¹²`, `10⁻⁵` at `10⁻⁶`, the smallest split each admits.
#[test]
fn a_split_beside_a_corner_of_a_draft_cone_meshes() {
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

/// **A frustum's seam split within its development's resolution meshes,
/// at ε `10⁻¹²`.** The frustum wall's outer cone face (half-angle 45°)
/// with its seam split beside its start, placed obliquely, at δ 0.1, 0.01
/// and 0.001; the face is walked.
///
/// **By ε.** At `10⁻¹²` the gap is `10⁻⁹` of the seam: the split vertex
/// and the seam's end lie within `2⁻²⁷` of their slant radius, one point
/// of the cone's development, which the walk does not read. At the
/// default ε and `10⁻⁶` the smallest split of this seam (`10⁻⁸` and
/// `10⁻⁵` of it) is wider than that: the row holds that the face meshes
/// with its two points apart.
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

/// **A draft cone the mass properties cannot read still meshes its close
/// splits.** The 0.1° wall, a full turn, its seam split `10⁻⁵` of its
/// length from its start, and twice `10⁻⁵` apart at its middle, placed
/// obliquely, at δ 0.1, 0.01 and 0.001; the face is walked.
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

/// **A one-chord face to the apex split beside both rim ends meshes.** A
/// cone of half-angle 10° and height 100 through its apex, revolved 0.05:
/// a thin wedge whose cone face is the apex, two rulings and one rim
/// chord. Both rulings split beside their rim ends, placed obliquely: the
/// one walked triangle holds both sides, whose points the walk joins to
/// each other (`ruled_walk::Rows::zip_sides`), and the face is three
/// triangles at δ 10, 1 and 0.1.
///
/// **By ε.** The gap is `10⁻⁷` along the ruling at the default ε and at
/// `10⁻¹²`, `10⁻⁵` at `10⁻⁶`; the face is the same three triangles.
#[test]
fn a_one_chord_face_to_the_apex_split_beside_both_rim_ends_meshes() {
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
/// (`tessellate::Lane::Walked`), and hold the count relation
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
    assert_eq!(
        unread_volumes(),
        if eps_row() == EpsRow::Default { 3 } else { 0 },
        "the cells whose volume is not read: the 1000 cone's at the default ε"
    );
}

/// **A cone band shorter than its development resolves meshes.** A wall
/// whose outer face is a cone of half-angle 1° and height `h`, a full
/// turn, its seam split at its middle and placed obliquely. The whole
/// ruling side lies inside one cut of the development's slant radius,
/// where the split vertex and the side's two ends were one point and no
/// run, and the face refused `DevelopedMerge`. It is walked
/// (`tessellate::Lane::Walked`) in its chart, and holds the count relation
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
    assert_eq!(
        unread_volumes(),
        0,
        "the cells whose volume is not read: none"
    );
}

/// **A one-chord face through the apex with both sides split meshes.** A
/// cone through its apex (`apex_cone`) of half-angle 1°, 5°, 30° and 45°,
/// revolved 0.05 and 0.5 rad, every line edge split at `1 − g`, 0.5 and
/// `g` of its range and placed obliquely: at a coarse δ its rim is one
/// chord, so the walk writes one triangle, and both of its sides hold
/// points. The two sides' points are joined to each other in order of
/// distance from the apex (`ruled_walk`, "Both sides on one triangle"),
/// and each body holds the count relation ([`holds_the_count_relation`];
/// the volume where the body finishes).
///
/// **By ε**, each ε on the size and gap it builds: the default, height
/// 1000 at δ 3000, 300, 30 and 3; `10⁻⁶` the same with the splits `10⁻⁴`
/// from the ends (`split_edge` refuses `10⁻⁸` on the 1° cone there);
/// `10⁻¹²`, height 1 at δ 3, 0.3, 0.03 and 0.003 (a cone of 1000 does not
/// revolve there).
#[test]
fn a_one_chord_face_through_the_apex_with_both_sides_split_meshes() {
    let (height, gap, deltas): (f64, f64, &[f64]) = match eps_row() {
        EpsRow::Coarse => (1000.0, 1e-4, &[3000.0, 300.0, 30.0, 3.0]),
        EpsRow::Default => (1000.0, 1e-8, &[3000.0, 300.0, 30.0, 3.0]),
        EpsRow::Fine => (1.0, 1e-8, &[3.0, 0.3, 0.03, 0.003]),
    };
    for alpha in [1.0, 5.0, 30.0, 45.0] {
        for turn in [0.05, 0.5] {
            let body = turned(&apex_cone(height, alpha), Revolution::Partial(turn));
            let lines = rulings(&body);
            let split = split_and_placed(&body, &lines, &[1.0 - gap, 0.5, gap]);
            let solid = split_and_placed(&body, &[], &[]);
            let what = format!("a {alpha}° cone turned {turn}, every line edge split thrice");
            holds_everywhere(&what, &split, &solid, deltas);
        }
    }
    assert_eq!(
        unread_volumes(),
        0,
        "the cells whose volume is not read: none"
    );
}

/// A ball of radius 1.05 bored through along `y` by a coaxial cylinder of
/// radius 1: the profile is the bore's line from `(1, −h)` to `(1, h)` and
/// an arc back, `h = √(1.05² − 1)`, its bulge `sign · tan(θ/4)`,
/// `θ = 2·atan2(h, 1)`: a torus face round the bore for one sign, a
/// sphere face for the other. Either holds both of the bore's rims.
pub(super) fn bored_ball(sign: f64) -> Body<f64> {
    bored_ball_of(1.05, sign)
}

/// [`bored_ball`] of radius `r`, its bore still of radius 1.
fn bored_ball_of(r: f64, sign: f64) -> Body<f64> {
    use profile::test_support::bulge_loop;
    let h = (r * r - 1.0).sqrt();
    let theta = 2.0 * h.atan2(1.0);
    let lp = bulge_loop(vec![
        (Point2::new(1.0, -h), 0.0),
        (Point2::new(1.0, h), sign * (theta / 4.0).tan()),
    ]);
    revolve(&validated(vec![lp]), axis_y(), Revolution::Full, tol())
        .unwrap()
        .body
        .into_body()
}

/// **A walked face whose neighbour holds both of its rims makes no edge
/// of four triangles, and gains a point only where the join splits a side
/// it has inside.** A ball of radius 1.05 bored by a coaxial cylinder of
/// radius 1 ([`bored_ball`]), its bore's seam split at its middle and at
/// five points, placed obliquely, with a sphere face round the bore and
/// with a torus face. The neighbour holds every point of both rims, and
/// where it lays no point between them (δ past the shell's thickness, 0.05)
/// its own triangles join the rims as the walk does; each side both make is
/// split in both patches where they join, and so is each side of a
/// triangle whose corners are all rim points where both faces lay such a
/// triangle (`mesh::join`'s census). Each cell pins the points the bore's
/// patch gained there ([`holds_the_relation`]): 15 to 25 a cell at δ 3 to
/// 0.3 (and 21 at δ 0.1 for sign −1), none at δ 0.04, where the neighbour
/// lays a row between the rims and no triangle of rim points alone. A
/// second, thicker ball (radius 1.3, a shell of 0.3) gains none at any δ:
/// its neighbour never makes the bore's sides. Every cell passes
/// `check_mesh` and is the body's
/// (`work/tesslane/a-strip-against-a-neighbour-holding-both-rims-makes-an-edge-of-four-triangles.md`).
///
/// **By ε.** The same at the default, `10⁻⁶` and `10⁻¹²`.
#[test]
fn a_walked_face_whose_neighbour_holds_both_rims_shares_no_edge() {
    // The points the bore's patch gains where the patches join, per (ball's
    // radius, sign, δ), the same for either split.
    let cuts = |r: f64, sign: f64, delta: f64| match (r == 1.05, sign > 0.0, delta) {
        (true, _, 3.0 | 1.0) | (true, false, 0.3) => 15,
        (true, true, 0.3) => 25,
        (true, false, 0.1) => 21,
        _ => 0,
    };
    let mut cells = 0usize;
    for (r, deltas) in [
        (1.05, &[3.0, 1.0, 0.3, 0.1, 0.04, 0.035, 0.03][..]),
        (1.3, &[3.0, 0.5, 0.25, 0.2, 0.1][..]),
    ] {
        for sign in [1.0, -1.0] {
            let body = bored_ball_of(r, sign);
            let solid = split_and_placed(&body, &[], &[]);
            let [seam] = rulings(&body)[..] else {
                panic!("the bore has one seam");
            };
            for fracs in [&[0.5][..], &[0.9, 0.7, 0.5, 0.3, 0.1][..]] {
                let what =
                    format!("the ball of {r} bored with sign {sign}, its seam split at {fracs:?}");
                let split = split_and_placed(&body, &[seam], fracs);
                for &delta in deltas {
                    holds_the_relation(&what, &split, &solid, delta, cuts(r, sign, delta));
                    cells += 1;
                }
            }
        }
    }
    assert_eq!(cells, 48, "the cells this row reads");
    assert_eq!(
        unread_volumes(),
        0,
        "the cells whose volume is not read: none"
    );
}

/// **A shell thinner than δ meshes, split or not, and encloses its
/// volume.** The ball of radius 1.05 bored at radius 1 ([`bored_ball`]): a
/// shell 0.05 thick, both arc signs, unsplit and with its bore's seam
/// split at its middle, δ 3 to 0.03, so the shell is thinner than δ at
/// every cell but the last, each as built and under two rigid placements.
/// Where the sphere's or torus's patch and the bore's both lay a side
/// between the rims, or both lay a triangle of rim points alone, the join
/// splits it in each (`mesh::join`), and the mesh is a closed manifold by
/// `check_mesh`.
///
/// The unsplit bore's two seam-side edges join the rims' two vertices:
/// chorded to one segment each, they are one segment carried twice, an
/// edge of four triangles the census passes as two chords' and
/// `check_mesh` refuses. They take two chords each (`chords`' coincident-
/// end floor), and every cell passes.
///
/// **The volume, against the body's** (`topo::mass_properties`, about
/// 0.137 for sign +1). Where both faces lay only rim points the two
/// sheets would cover the shell with the same points; a split point of
/// each face's own surface moves each sheet toward its own face. Every
/// cell encloses between [`SHELL_MARGIN`] of the body's volume and all of
/// it. Measured 0.523 to 0.897 at ε default, `10⁻⁶` and `10⁻¹²` alike (the
/// least on the torus sign's unsplit shell at δ 0.3 to 3, in every pose):
/// a relation measured on these cells, not a promise of the mesher's.
#[test]
fn a_shell_thinner_than_delta_meshes_split_or_unsplit() {
    let poses = [
        None,
        Some(Affine3::from_parts(
            Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), 1.1).linear,
            Vec3::new(-3.0, 2.0, 0.5),
        )),
    ];
    let mut cells = 0usize;
    for sign in [1.0, -1.0] {
        let body = bored_ball(sign);
        let [seam] = rulings(&body)[..] else {
            panic!("the bore has one seam");
        };
        // As built, and as `split_and_placed` places it (its oblique pose).
        let mut bodies = vec![
            ("unsplit, placed", split_and_placed(&body, &[], &[])),
            ("split, placed", split_and_placed(&body, &[seam], &[0.5])),
        ];
        for (k, pose) in poses.iter().enumerate() {
            let mut split = body.clone();
            let (t0, t1) = split
                .get_curve_geom(split.get_edge(seam).unwrap().curve)
                .unwrap()
                .certified()
                .unwrap()
                .params();
            split.split_edge(seam, 0.5 * (t0 + t1), tol()).unwrap();
            for (name, b) in [("unsplit", body.clone()), ("split", split.clone())] {
                let b = pose.map_or(b.clone(), |p| {
                    topo::transform_rigid(&b, &geom_core::test_support::rigid(&p, tol())).unwrap()
                });
                bodies.push((if k == 0 { name } else { "posed" }, b));
            }
        }
        for (name, b) in &bodies {
            let truth = topo::mass_properties(
                &topo::test_support::finished("the shell", b.clone(), tol()),
                tol(),
                topo::test_support::pinned_resolution(tol()),
            )
            .unwrap()
            .enclosure
            .volume;
            for delta in [3.0, 1.0, 0.3, 0.1, 0.05, 0.04, 0.03] {
                let what = format!("the bored ball, sign {sign}, {name}, δ {delta}");
                let mesh = mesh::tessellate(b, delta, tol())
                    .unwrap_or_else(|e| panic!("{what}: meshes, got {e:?}"));
                mesh::validate::check_mesh(&mesh)
                    .unwrap_or_else(|e| panic!("{what}: a closed manifold, got {e:?}"));
                let ratio = mesh::validate::signed_volume(&mesh) / truth;
                if std::env::var_os("PRINT_SHELL").is_some() {
                    eprintln!("SHELL\t{what}\t{ratio}");
                }
                assert!(
                    (SHELL_MARGIN..=1.0 + 1e-9).contains(&ratio),
                    "{what}: encloses {ratio} of the body's volume"
                );
                cells += 1;
            }
        }
    }
    assert_eq!(cells, 84, "the cells this row reads");
}

/// The least share of its volume [`a_shell_thinner_than_delta_meshes_split_or_unsplit`]'s
/// shell may enclose on any cell: below the measured least, 0.523.
const SHELL_MARGIN: f64 = 0.45;

/// **A run of three on a cone face that is no rectangle meshes.** The
/// cut wall of [`cut_cone_cells`] (the 1° wall, a half turn, under the
/// tilted slab), every cone ruling split twice beside its end, `g` and
/// `2g` of its range from it, placed obliquely, δ 0.01: each split pair
/// and the ruling's end are one run of three points the development holds
/// as one (`trimmed::Run`), cut apart again along the ruling, and the face
/// meshes and is the body's. Giving every triangle added between the run's
/// points the run's second point reds it: a run of two cannot tell.
///
/// **By ε.** `g` is `10⁻⁷` at the default ε and `10⁻¹²`, where the three
/// points are one in the development; `10⁻⁴` at `10⁻⁶`, the smallest split
/// there, which the development does not merge: the row holds that the
/// face meshes with its points apart.
#[test]
fn a_run_of_three_on_a_cut_cone_face_meshes() {
    use crate::reach_probes::{op_of, posed};
    use sweep::test_support::{brick, finished};
    let gap = match eps_row() {
        EpsRow::Coarse => 1e-4,
        EpsRow::Default | EpsRow::Fine => 1e-7,
    };
    let tilt = Affine3::translation(Vec3::new(0.0, 0.5, 0.0))
        * Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.6, 0.0, 0.8), 0.1);
    let cone = finished(
        "the wall",
        turned(&wall(1.0), Revolution::Partial(core::f64::consts::PI)),
        tol(),
    );
    let slab = posed(
        "the slab",
        brick((-6.0, 6.0), (-6.0, 0.0), (-6.0, 6.0), tol()),
        tilt,
    );
    let cut = op_of(&cone, &slab, "A ∩ B");
    let lines = cone_rulings(&cut);
    let split = split_and_placed(&cut, &lines, &[1.0 - gap, 1.0 - 2.0 * gap]);
    assert_eq!(
        got_enclosing(&split, &split_and_placed(&cut, &[], &[]), 0.01),
        Got::Meshed
    );
}

/// A cone through its apex of half-angle `alpha` and height 100, revolved
/// `turn`, cut near its rim by a plane tilted 0.1 about an oblique axis,
/// the part above it kept (`A ∩ B`): the cone face runs from that
/// section, a conic, up to the apex, so it is the chart lane's by its trim
/// carrier and is triangulated in its development.
fn cut_apex_cone(alpha: f64, turn: Revolution<f64>) -> Body<f64> {
    use crate::reach_probes::{op_of, posed};
    use sweep::test_support::{brick, finished};
    let tilt = Affine3::translation(Vec3::new(0.0, 10.0, 0.0))
        * Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.6, 0.0, 0.8), 0.1);
    let slab = posed(
        "the slab",
        brick((-150.0, 150.0), (0.0, 150.0), (-150.0, 150.0), tol()),
        tilt,
    );
    let cone = finished("the cone", turned(&apex_cone(100.0, alpha), turn), tol());
    op_of(&cone, &slab, "A ∩ B")
}

/// `body` with each of `edges` split `gap` (a length) from its end
/// farther from the axis (`outer`) or nearer it, placed obliquely; `None`
/// where `split_edge` refuses.
fn split_beside(
    body: &Body<f64>,
    edges: &[topo::EdgeKey],
    gap: f64,
    outer: bool,
) -> Option<Body<f64>> {
    let mut s = body.clone();
    for &ek in edges {
        let (a, b) = ends(body, ek);
        let len = (b - a).norm();
        let f = if (radius(b) > radius(a)) == outer {
            1.0 - gap / len
        } else {
            gap / len
        };
        s.split_edge(ek, at(&s, ek, f), tol()).ok()?;
    }
    Some(split_and_placed(&s, &[], &[]))
}

/// **Two runs anchored by the apex in one triangle mesh, on a cone face
/// that is no rectangle.** A cone of half-angle 10°, a wedge of 0.05, cut
/// near its rim with its apex kept ([`cut_apex_cone`]): the face runs
/// from one section chord up to the apex. Every line edge on a cone face
/// split `10⁻⁷` beside its outer end, placed obliquely: each split vertex and its
/// ruling's end are a run whose neighbour along the ruling is the apex,
/// and the development's one triangle at the apex has both for corners;
/// the lane cuts it along both (`trimmed::cut_along_runs`), and every cell
/// meshes and is the body's at δ 10, 1 and 0.1. Leaving the fan's corner
/// on the second run's point unmoved reds it.
///
/// **By ε.** The gap is `10⁻⁷` at the default ε and `10⁻¹²`, `10⁻⁴` at
/// `10⁻⁶`, which the development does not merge: the row holds that the
/// face meshes with its points apart.
#[test]
fn two_runs_anchored_by_the_apex_on_a_cut_cone_face_mesh() {
    let gap = match eps_row() {
        EpsRow::Coarse => 1e-4,
        EpsRow::Default | EpsRow::Fine => 1e-7,
    };
    let cut = cut_apex_cone(10.0, Revolution::Partial(0.05));
    let lines = cone_rulings(&cut);

    let split = split_beside(&cut, &lines, gap, true).expect("the splits build");
    let whole = split_and_placed(&cut, &[], &[]);
    for delta in [10.0, 1.0, 0.1] {
        assert_eq!(
            got_enclosing(&split, &whole, delta),
            Got::Meshed,
            "δ {delta}"
        );
    }
}

/// **A run anchored by the apex on a needle triangle refuses: this row
/// pins a gap**
/// (`work/tesslane/a-run-anchored-on-a-needle-triangle-makes-triangles-f64-cannot-wind.md`),
/// on a cone face the chart lane still triangulates in its development. A
/// cone of half-angle 2°, a full turn, cut near its rim with its apex kept
/// ([`cut_apex_cone`]), its first ruling split a hair beside its section
/// end, placed obliquely: the split vertex and the ruling's end are a run
/// whose neighbour along the ruling is the apex, a whole ruling away, and
/// the triangles added between them are under the winding allowance: the
/// face refuses `UnresolvedWinding`.
///
/// **By ε.** The default: a gap of `10⁻⁸`, δ 10, 1 and 0.1 mesh and δ 0.01
/// refuses. `10⁻¹²`: a gap of `10⁻¹⁰` refuses at δ 10, 1, 0.1 and 0.01.
/// `10⁻⁶`: the smallest split `split_edge` admits, `10⁻⁵`, meshes at every
/// δ, and the row holds that.
#[test]
fn a_run_anchored_by_the_apex_on_a_needle_refuses_unresolved() {
    let refused = Got::Unresolved(geom::SurfaceKind::Cone);
    let (gap, cells): (f64, &[(f64, Got)]) = match eps_row() {
        EpsRow::Default => (
            1e-8,
            &[
                (10.0, Got::Meshed),
                (1.0, Got::Meshed),
                (0.1, Got::Meshed),
                (0.01, refused.clone()),
            ],
        ),
        EpsRow::Fine => (
            1e-10,
            &[
                (10.0, refused.clone()),
                (1.0, refused.clone()),
                (0.1, refused.clone()),
                (0.01, refused.clone()),
            ],
        ),
        EpsRow::Coarse => (
            1e-5,
            &[
                (10.0, Got::Meshed),
                (1.0, Got::Meshed),
                (0.1, Got::Meshed),
                (0.01, Got::Meshed),
            ],
        ),
    };
    let cut = cut_apex_cone(2.0, Revolution::Full);
    let ruling = cone_rulings(&cut)[0];
    let split = split_beside(&cut, &[ruling], gap, true).expect("the split builds");
    let whole = split_and_placed(&cut, &[], &[]);
    for (delta, want) in cells {
        assert_eq!(&got_enclosing(&split, &whole, *delta), want, "δ {delta}");
    }
}

/// **A split beside the apex of a needle cone the band cannot place
/// refuses: this row pins the walk's fallback.** A cone of half-angle
/// 0.001° and height 1 through its apex, a full turn, its line edge 0 split
/// `10⁻⁸` from its apex end, placed obliquely. The route asks whether the
/// face's vertices are the apex (`trimmed::walkable`); the split vertex is
/// within the band of it and the band does not decide, so the route does
/// not walk the face, and the lattice and CDT refuse it
/// `SphereTrimShape { SingularUndecided }` at δ 0.1 and 0.01.
///
/// **By ε.** At the default as above. At `10⁻¹²` the band decides the
/// vertex off the apex, the face is walked, and it meshes. At `10⁻⁶` the
/// cone's profile does not validate (its loop's orientation escalates),
/// and the row holds that.
#[test]
fn a_split_beside_the_apex_of_a_needle_cone_refuses_undecided() {
    use mesh::TessellateError as E;
    use mesh::types::SphereTrimCause;
    let lp = ProfileLoop::polygon(
        apex_cone(1.0, 0.001)
            .iter()
            .map(|&(x, y)| Point2::new(x, y)),
    );
    let profile = profile::Profile::new(profile::SketchPlane::xy(), vec![lp]).validate(tol());
    let Ok(profile) = profile else {
        assert_eq!(
            eps_row(),
            EpsRow::Coarse,
            "the needle cone's profile validates at ε {:e}",
            tol().eps()
        );
        return;
    };
    let body = revolve(&profile, axis_y(), Revolution::Full, tol())
        .unwrap()
        .body
        .into_body();
    let ruling = rulings(&body)[0];
    let split = split_beside(&body, &[ruling], 1e-8, false).expect("the split builds");
    for delta in [0.1, 0.01] {
        let got = mesh::tessellate(&split, delta, tol());
        if eps_row() == EpsRow::Fine {
            let m = got.unwrap_or_else(|e| panic!("δ {delta}: {e:?}"));
            mesh::validate::check_mesh(&m).unwrap_or_else(|e| panic!("δ {delta}: {e:?}"));
        } else {
            assert!(
                matches!(
                    got,
                    Err(E::SphereTrimShape {
                        cause: SphereTrimCause::SingularUndecided,
                        ..
                    })
                ),
                "δ {delta}: {got:?}"
            );
        }
    }
}

/// **A thin D-prism split on every line edge refuses as the same prism
/// split on its caps' chords alone does: a regression against main this
/// unit keeps.** The D-shaped prism of 0.5 rad ([`d_prism`]) at δ 0.1,
/// placed obliquely. Its refusing face is a planar cap, not the walked
/// one: with a point on its straight chord the cap meets points the planar
/// lane cannot wind (`UnresolvedWinding`). Three cells:
///
/// - **unsplit:** meshes, here and on main;
/// - **its caps' chords alone split** (no ruled face has a split side, so
///   nothing is walked): refuses so, here and on main;
/// - **every line edge split**, the cylinder face's rulings too: refuses
///   so here. On main it meshed, only because main's chart lane chorded the
///   ruled face's rim finer (its headroom), which gave the caps an arc they
///   can hold; the walk chords the rim as the unsplit face's is chorded.
///
/// The 0.2 rad prism at δ 0.03 is the same (12 cells over both, six split
/// patterns each, refuse here and mesh on main). The class is the planar
/// lane's (`work/tesslane/a-thin-arc-bounded-face-refuses-as-corrupt-geometry-at-a-coarse-delta.md`).
#[test]
fn a_thin_d_prism_split_on_every_line_edge_refuses_as_its_split_caps_do() {
    use geom::SurfaceKind;
    let body = d_prism(0.5);
    let lines = rulings(&body);
    let caps: Vec<topo::EdgeKey> = lines
        .iter()
        .copied()
        .filter(|&ek| {
            let c = body
                .get_curve_geom(body.get_edge(ek).unwrap().curve)
                .unwrap()
                .certified()
                .unwrap();
            let (t0, t1) = c.params();
            (c.carrier().eval(t0).z - c.carrier().eval(t1).z).abs() < 1e-9
        })
        .collect();
    assert_eq!(caps.len(), 2, "the prism's two cap chords");
    let solid = split_and_placed(&body, &[], &[]);
    assert_eq!(
        got_enclosing(&solid, &solid, 0.1),
        Got::Meshed,
        "the unsplit prism, δ 0.1"
    );
    for (what, edges) in [("its caps' chords", &caps), ("every line edge", &lines)] {
        let split = split_and_placed(&body, edges, &[0.5]);
        assert_eq!(
            got_enclosing(&split, &solid, 0.1),
            Got::Unresolved(SurfaceKind::Plane),
            "the prism split on {what}, δ 0.1"
        );
    }
}

/// **The split where the patches join is a function of the body and δ:
/// no ε moves it.** DESIGN `[tolerance.chordal]`: an ε flip never moves
/// emitted coordinates with δ held fixed. The bored ball of sign +1
/// ([`bored_ball`]), its seam split at its middle, placed obliquely, at δ
/// 0.3, where its bore's patch gains 25 points, and at 0.04, where it gains
/// none: the census reads shared ids and the split the face's chart and
/// certificate, and the mesh's bytes are the same pinned digest at ε
/// `10⁻⁹`, `10⁻⁶` and `10⁻¹²` alike.
#[test]
fn the_join_splits_the_same_at_every_eps() {
    use crate::route_rows::mesh_digest;
    let body = bored_ball(1.0);
    let [seam] = rulings(&body)[..] else {
        panic!("the bore has one seam");
    };
    let split = split_and_placed(&body, &[seam], &[0.5]);
    let solid = split_and_placed(&body, &[], &[]);
    for (delta, cuts, digest) in [
        (0.3, 25, 0x3d77_e08a_d06d_ff71_u64),
        (0.04, 0, 0x882f_0997_46ad_3e90),
    ] {
        let what = "the ball bored with sign 1, its seam split at [0.5]";
        holds_the_relation(what, &split, &solid, delta, cuts);
        assert_eq!(
            mesh_digest(&split, delta),
            digest,
            "{what}, δ {delta}: the mesh's bytes"
        );
    }
}
