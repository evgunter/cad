//! **A ruled face whose straight side holds a point**: a cylinder or
//! cone face with a ruling carried by more than one edge, which
//! `tessellate::lane_of` sends to the chart lane
//! (`walk::splits_a_straight_side`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Vec3};
use profile::ProfileLoop;
use profile::RawLoop as _;
use sweep::{Revolution, revolve};
use topo::Body;
use topo::construction::Construct;

use crate::common::{axis_y, validated};
use crate::reach_probes::{Got, got_enclosing, tol};

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

/// **A ruled face whose straight side holds a point meshes**, on the
/// chart lane. Two kinds of body, revolved about `y`:
///
/// - **one-face full turns**, whose cylinder or cone face walks its seam
///   twice: a cylinder wall, a frustum wall (the review's), a slender
///   wall (outer half-angle ≈ 6°) and a needle wall, each face outer and
///   reversed inner;
/// - **faces through the axis**, which a full revolve builds as two half
///   faces: cones through their apex at 45°, 80° and 5°, and a frustum;
///   and a half cylinder wall.
///
/// Each line edge is split at one point, at two and at five, and every
/// line edge at once at one point; each body is placed obliquely; at δ
/// 0.1, 0.01 and 0.001. Every cell meshes, passes `check_mesh`, and is
/// the body's ([`crate::reach_probes::geometry`]: every winding resolved
/// and outward, every triangle sampled within δ of its surface, and the
/// volume the unsplit body's to the chordal deficit).
///
/// But one, pinned as today's answer: the slender wall's outer seam split
/// anywhere, at δ 0.1, where its planar top annulus (width 0.05) refuses
/// `Triangulation`. A planar neighbour narrower than δ, beside any routed
/// face, refuses where the base meshed it: its rims' chords have a
/// sagitta past its width, so whether its loops cross depends on the
/// rims' chord counts, and the routed face chords the rim it shares for
/// the chart lane. The base refuses the same faces the same way with a
/// rim split, and none refuses at δ at most its width
/// (`work/tesslane/a-thin-arc-bounded-face-refuses-as-corrupt-geometry-at-a-coarse-delta.md`).
///
/// **By ε.** The same at the default, `10⁻⁶` and `10⁻¹²`.
#[test]
fn a_ruled_face_with_a_split_ruling_meshes() {
    let tau = core::f64::consts::TAU;
    let cone = |h: f64| vec![(0.0, 0.0), (f64::tan(h.to_radians()), 0.0), (0.0, 1.0)];
    let wall = [(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)];
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
        (
            "frustum",
            turned(
                &[(0.0, 0.0), (2.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
                Revolution::Full,
            ),
        ),
        (
            "half cylinder wall",
            turned(&wall, Revolution::Partial(tau / 2.0)),
        ),
    ];
    let splits: [&[f64]; 3] = [&[0.5], &[0.312_9, 0.156_45], &[0.9, 0.7, 0.5, 0.3, 0.1]];
    let mut cells = 0usize;
    for (name, body) in &bodies {
        let lines = rulings(body);
        let solid = split_and_placed(body, &[], &[]);
        let mut split: Vec<(String, Option<usize>, Body<f64>)> = Vec::new();
        for fracs in splits {
            for (k, &ek) in lines.iter().enumerate() {
                split.push((
                    format!("{name}, line edge {k} split at {fracs:?}"),
                    Some(k),
                    split_and_placed(body, &[ek], fracs),
                ));
            }
        }
        split.push((
            format!("{name}, every line edge split at 0.5"),
            None,
            split_and_placed(body, &lines, &[0.5]),
        ));
        for (what, k, b) in &split {
            for delta in [0.1, 0.01, 0.001] {
                // The slender wall's line edge 0 is its outer cone face's
                // seam.
                let want = if *name == "slender wall" && *k == Some(0) && delta == 0.1 {
                    Got::Triangulation(geom::SurfaceKind::Plane)
                } else {
                    Got::Meshed
                };
                assert_eq!(got_enclosing(b, &solid, delta), want, "{what}, δ {delta}");
                cells += 1;
            }
        }
    }
    assert_eq!(cells, 243, "the cells this row reads");
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
    use crate::reach_probes::{got, op_of, rod};
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

/// **The chord pass chords a routed face for the lane that meshes it**:
/// a one-face full-turn wall with its seam split at its middle, placed
/// obliquely, at δ 0.01, meshes to these triangle counts per patch (the
/// wall, then its neighbours). The chord pass reads the route (it asks
/// `lane_of`), so the wall's edges, and with them its neighbours' shared
/// boundary, are chorded at the chart lane's target. Chorded as the
/// rectangle lane would chord them, every count here falls (the cylinder
/// wall's to 296, 55, 64, 55): what the chord pass's reading of the route
/// decides is density, and these counts are what pins it.
#[test]
fn a_routed_face_is_chorded_for_the_chart_lane() {
    type Wall<'a> = (&'a str, &'a [(f64, f64)], [usize; 4]);
    let walls: [Wall<'_>; 2] = [
        (
            "cylinder wall",
            &[(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)],
            [314, 64, 64, 64],
        ),
        (
            "frustum wall",
            &[(1.0, 0.0), (3.0, 0.0), (2.0, 1.0), (1.5, 1.0)],
            [366, 72, 72, 76],
        ),
    ];
    for (name, profile, want) in walls {
        let body = turned(profile, Revolution::Full);
        let seam = rulings(&body)[..1].to_vec();
        let mesh = mesh::tessellate(&split_and_placed(&body, &seam, &[0.5]), 0.01, tol()).unwrap();
        let counts: Vec<usize> = mesh.patches.iter().map(|p| p.triangles.len()).collect();
        assert_eq!(counts, want, "{name}: triangles per patch");
    }
}

/// **A ruling split a hair from its end folds a triangle: this row pins a
/// gap** (`work/tesslane/a-ruling-split-a-hair-from-its-end-folds-a-triangle.md`).
/// A cone of half-angle 20° through its apex, revolved `2π − 0.3`, its
/// line edge 1 split `3·10⁻⁷` of its range from its start and placed
/// obliquely: at δ 0.01 the chart lane's cone face refuses
/// `FoldedTriangle`. Not every face of the class meshes; the base
/// answered `UnresolvedWinding` here.
///
/// **By ε.** The same at `10⁻¹²`; at `10⁻⁶` `split_edge` reads the split
/// as no split, and the row says so and stops.
#[test]
fn a_ruling_split_a_hair_from_its_end_folds_a_triangle() {
    let body = turned(
        &[(0.0, 0.0), (f64::tan(20f64.to_radians()), 0.0), (0.0, 1.0)],
        Revolution::Partial(core::f64::consts::TAU - 0.3),
    );
    let ruling = rulings(&body)[1];
    let mut unplaced = body.clone();
    let (t0, t1) = unplaced
        .get_curve_geom(unplaced.get_edge(ruling).unwrap().curve)
        .unwrap()
        .certified()
        .unwrap()
        .params();
    if let Err(e) = unplaced.split_edge(ruling, t0 + (t1 - t0) * 3e-7, tol()) {
        assert!(
            tol().eps() != geom_core::tolerance::DEFAULT_EPS,
            "the split is interior at the default ε: {e:?}"
        );
        eprintln!(
            "SKIPPED at ε {:e}: `split_edge` reads a split 3e-7 from the end as no split ({e:?})",
            tol().eps()
        );
        return;
    }
    let split = split_and_placed(&body, &[ruling], &[3e-7]);
    let solid = split_and_placed(&body, &[], &[]);
    assert_eq!(
        got_enclosing(&split, &solid, 0.01),
        Got::Folded(geom::SurfaceKind::Cone)
    );
}

/// The body's cone rulings, on a body revolved about `y`: its line edges
/// whose two ends differ both in radius and in height (a cylinder's
/// ruling keeps its radius, a cap's radial edge its height).
fn cone_rulings(body: &Body<f64>) -> Vec<topo::EdgeKey> {
    rulings(body)
        .into_iter()
        .filter(|&ek| {
            let c = body
                .get_curve_geom(body.get_edge(ek).unwrap().curve)
                .unwrap()
                .certified()
                .unwrap();
            let (t0, t1) = c.params();
            let (a, b) = (c.carrier().eval(t0), c.carrier().eval(t1));
            (a.x.hypot(a.z) - b.x.hypot(b.z)).abs() > 1e-12 && (a.y - b.y).abs() > 1e-12
        })
        .collect()
}

/// Whether `split_edge` admits each of `fracs` on `ek`, at this run's ε.
fn admits(body: &Body<f64>, ek: topo::EdgeKey, fracs: &[f64]) -> bool {
    let mut b = body.clone();
    let (t0, t1) = b
        .get_curve_geom(b.get_edge(ek).unwrap().curve)
        .unwrap()
        .certified()
        .unwrap()
        .params();
    fracs
        .iter()
        .all(|&f| b.split_edge(ek, t0 + (t1 - t0) * f, tol()).is_ok())
}

/// The wall `(1, 0), (2 + tan α, 0), (2, 1), (1, 1)` or, `through` the
/// axis, the solid `(0, 0), (2 + tan α, 0), (2, 1), (0, 1)`, revolved
/// about `y` through `turn`: its outer face is a cone of half-angle `α`
/// degrees, of slant radius `2 / sin α` at its small end. `None` where
/// this run's ε does not revolve it.
fn draft_cone(alpha: f64, through: bool, turn: Revolution<f64>) -> Option<Body<f64>> {
    let t = alpha.to_radians().tan();
    let inner = if through { 0.0 } else { 1.0 };
    let pts = [(inner, 0.0), (2.0 + t, 0.0), (2.0, 1.0), (inner, 1.0)];
    let lp = ProfileLoop::polygon(pts.iter().map(|&(x, y)| Point2::new(x, y)));
    revolve(&validated(vec![lp]), axis_y(), turn, tol())
        .ok()
        .map(|r| r.body.into_body())
}

/// The cells of [`a_run_of_ruling_points_the_development_merges_meshes`]
/// and of its grid: each draft cone of `bodies` (through the axis or not,
/// its turn), at each `(α, gap)` this run's ε builds, each of its cone
/// rulings split five ways, placed obliquely, at each of `deltas`. Every
/// cell is asserted `Meshed`; answers how many there were.
fn run_cells(bodies: &[(&str, bool, Revolution<f64>)], deltas: &[f64]) -> usize {
    let mut cells = 0usize;
    for (alpha, gap) in [(0.01, 1e-5), (1.0, 1e-7)] {
        for &(name, through, turn) in bodies {
            let Some(body) = draft_cone(alpha, through, turn) else {
                eprintln!(
                    "SKIPPED at ε {:e}: the {alpha}° {name} does not revolve",
                    tol().eps()
                );
                continue;
            };
            let solid = split_and_placed(&body, &[], &[]);
            let splits: [(&str, &[f64]); 5] = [
                ("a split beside its start", &[gap]),
                ("a split beside its end", &[1.0 - gap]),
                ("two splits beside each other", &[0.5 + gap, 0.5]),
                (
                    "three splits beside each other",
                    &[0.5 + 2.0 * gap, 0.5 + gap, 0.5],
                ),
                ("a split beside each end", &[1.0 - gap, gap]),
            ];
            for (k, ek) in cone_rulings(&body).into_iter().enumerate() {
                for (what, fracs) in splits {
                    if !admits(&body, ek, fracs) {
                        eprintln!(
                            "SKIPPED at ε {:e}: `split_edge` reads a split {gap:e} from another \
                             as no split",
                            tol().eps()
                        );
                        continue;
                    }
                    let split = split_and_placed(&body, &[ek], fracs);
                    for &delta in deltas {
                        assert_eq!(
                            got_enclosing(&split, &solid, delta),
                            Got::Meshed,
                            "the {alpha}° {name}, cone ruling {k}, {what} ({gap:e}), δ {delta}"
                        );
                        cells += 1;
                    }
                }
            }
        }
    }
    cells
}

/// The cells [`run_cells`] reads of `per_family` at this run's ε, each
/// family being one `(α, gap)`: both at the default ε; at `10⁻⁶` the
/// 0.01° cone's alone (`split_edge` reads a split `10⁻⁷` from another as
/// no split); at `10⁻¹²` the 1° cone's alone (the 0.01° cone does not
/// revolve there).
fn families_at_this_eps(per_family: usize) -> usize {
    if tol().eps() == geom_core::tolerance::DEFAULT_EPS {
        2 * per_family
    } else {
        per_family
    }
}

/// **A run of ruling points the development merges meshes.** The chart
/// lane triangulates a cone face in its development, which cuts a slant
/// radius to 27 significant bits: two points of one ruling within about
/// `2⁻²⁷` of their slant radius are one point there. On a draft cone that
/// is a length a split reaches (`2 / sin α · 2⁻²⁷`: `8.5·10⁻⁵` at 0.01°,
/// `8.5·10⁻⁷` at 1°). The lane holds such a run of consecutive ruling
/// points as one vertex of its triangulation and tells them apart again
/// when it emits (`trimmed::Run`).
///
/// The wall, a full turn: its one cone face walks its seam twice, so each
/// run is met on each of the seam's two rays. The seam split beside its
/// start, beside its end (the run then holds a corner of the polygon),
/// twice and three times at its middle, and beside both ends at once;
/// placed obliquely; δ 0.01. Every cell meshes, passes `check_mesh`, and
/// is the body's ([`crate::reach_probes::geometry`]).
///
/// **By ε.** At the default both cones; at `10⁻⁶` and `10⁻¹²` the one
/// that ε builds and splits ([`families_at_this_eps`]). The grid over the
/// wall and the solid through the axis, full and half turns, and three δ
/// is [`every_run_of_a_draft_cones_rulings_meshes`].
#[test]
fn a_run_of_ruling_points_the_development_merges_meshes() {
    let cells = run_cells(&[("wall, full", false, Revolution::Full)], &[0.01]);
    assert_eq!(cells, families_at_this_eps(5), "the cells this row reads");
}

/// **The grid of [`a_run_of_ruling_points_the_development_merges_meshes`]**:
/// the wall and the solid through the axis, each a full turn and a half
/// (a half turn's cone face ends on two rulings, each shared with a planar
/// cut face), at δ 0.1, 0.01 and 0.001.
#[test]
fn every_run_of_a_draft_cones_rulings_meshes() {
    let half = Revolution::Partial(core::f64::consts::PI);
    let cells = run_cells(
        &[
            ("wall, full", false, Revolution::Full),
            ("wall, half", false, half),
            ("solid, full", true, Revolution::Full),
            ("solid, half", true, half),
        ],
        &[0.1, 0.01, 0.001],
    );
    assert_eq!(cells, families_at_this_eps(105), "the cells this row reads");
}

/// **A run on an ordinary frustum, at ε `10⁻¹²`.** The frustum wall's
/// outer cone face (half-angle 45°) with its seam split `10⁻⁹` of its
/// range from its start, placed obliquely: the split vertex and the seam's
/// end lie within `2⁻²⁷` of their slant radius, and the face meshes at
/// every δ as [`a_run_of_ruling_points_the_development_merges_meshes`]
/// says.
///
/// **By ε.** At `10⁻¹²` as above. At the default ε and `10⁻⁶`
/// `split_edge` reads a split `10⁻⁹` from this body's seam end as no
/// split, so this body cannot be built, and the row asserts that instead.
#[test]
fn a_frustum_seam_split_within_the_developments_resolution_meshes() {
    let body = turned(
        &[(1.0, 0.0), (3.0, 0.0), (2.0, 1.0), (1.5, 1.0)],
        Revolution::Full,
    );
    let seam = rulings(&body)[0];
    if tol().eps() > 1e-12 {
        assert!(
            !admits(&body, seam, &[1e-9]),
            "this body at ε {:e}: a split 1e-9 from its seam end is no split",
            tol().eps()
        );
        return;
    }
    let split = split_and_placed(&body, &[seam], &[1e-9]);
    let solid = split_and_placed(&body, &[], &[]);
    for delta in [0.1, 0.01, 0.001] {
        assert_eq!(
            got_enclosing(&split, &solid, delta),
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
    let body = draft_cone(0.1, false, Revolution::Full).expect("the 0.1° wall revolves");
    let [ruling] = cone_rulings(&body)[..] else {
        panic!("the wall's cone face has one seam");
    };
    let solid = split_and_placed(&body, &[], &[]);
    for (what, fracs) in [
        ("an end split", &[1e-5][..]),
        ("two splits mid-ruling", &[0.5 + 1e-5, 0.5][..]),
    ] {
        let split = split_and_placed(&body, &[ruling], fracs);
        for delta in [0.1, 0.01, 0.001] {
            if tol().eps() < geom_core::tolerance::DEFAULT_EPS {
                let m = mesh::tessellate(&split, delta, tol())
                    .unwrap_or_else(|e| panic!("{what}, δ {delta}: {e:?}"));
                mesh::validate::check_mesh(&m)
                    .unwrap_or_else(|e| panic!("{what}, δ {delta}: {e:?}"));
                mesh::validate::check_winding(&m)
                    .unwrap_or_else(|e| panic!("{what}, δ {delta}: {e:?}"));
            } else {
                assert_eq!(
                    got_enclosing(&split, &solid, delta),
                    Got::Meshed,
                    "{what}, δ {delta}"
                );
            }
        }
    }
}

/// **Two runs beside the apex, anchored by it, mesh.** A cone of
/// half-angle 45° through its apex, revolved 0.05: a thin wedge whose
/// cone face is the apex, two rulings and one rim chord. Both rulings
/// split `10⁻¹⁰` of their range from their rim ends, placed obliquely:
/// each split vertex and its ruling's end are a run whose neighbour along
/// the ruling is the apex, and the development's one triangle at the apex
/// has both runs for corners. The lane cuts it along both, and the face
/// is three triangles at every δ.
///
/// **By ε.** At `10⁻¹²` as above; at the default and `10⁻⁶`
/// `split_edge` reads the split as no split, and the row says so and
/// stops.
#[test]
fn two_runs_anchored_by_the_apex_mesh() {
    let body = turned(
        &[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)],
        Revolution::Partial(0.05),
    );
    let lines = cone_rulings(&body);
    assert_eq!(lines.len(), 2, "the wedge's cone face ends on two rulings");
    let fracs = [1.0 - 1e-10];
    if !lines.iter().all(|&ek| admits(&body, ek, &fracs)) {
        assert!(tol().eps() > 1e-12, "the split is interior at ε 1e-12");
        eprintln!(
            "SKIPPED at ε {:e}: `split_edge` reads a split 1e-10 from the end as no split",
            tol().eps()
        );
        return;
    }
    let split = split_and_placed(&body, &lines, &fracs);
    let solid = split_and_placed(&body, &[], &[]);
    for delta in [0.1, 0.01, 0.001] {
        assert_eq!(
            got_enclosing(&split, &solid, delta),
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

/// **A run whose triangle on its ruling side is a needle leaves triangles
/// f64 cannot wind: this row pins a gap**
/// (`work/tesslane/a-run-anchored-on-a-needle-triangle-makes-triangles-f64-cannot-wind.md`).
/// A cone of half-angle 45° through its apex, a full turn (two half
/// faces), one ruling split `10⁻¹⁰` of its range from its rim end, placed
/// obliquely. The split vertex and the ruling's end are a run, and its
/// neighbour along the ruling is the apex, a whole ruling away. At δ 0.1
/// and 0.01 the face meshes. At δ 0.001 the lattice's rows stand close
/// about the apex, the triangle on the side from the run to the apex takes
/// a node there for its third corner, and the triangle added between the
/// run's two points and that node is `1.4·10⁻¹⁰` wide and about `10⁻⁴` of
/// its length high: under the winding rule's allowance, and the face
/// refuses `UnresolvedWinding`.
///
/// **By ε.** At `10⁻¹²` as above; at the default and `10⁻⁶`
/// `split_edge` reads the split as no split, and the row says so and
/// stops.
#[test]
fn a_run_anchored_on_a_needle_triangle_refuses_unresolved() {
    let body = turned(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)], Revolution::Full);
    let ruling = cone_rulings(&body)[0];
    let fracs = [1.0 - 1e-10];
    if !admits(&body, ruling, &fracs) {
        assert!(tol().eps() > 1e-12, "the split is interior at ε 1e-12");
        eprintln!(
            "SKIPPED at ε {:e}: `split_edge` reads a split 1e-10 from the end as no split",
            tol().eps()
        );
        return;
    }
    let split = split_and_placed(&body, &[ruling], &fracs);
    let solid = split_and_placed(&body, &[], &[]);
    for (delta, want) in [
        (0.1, Got::Meshed),
        (0.01, Got::Meshed),
        (0.001, Got::Unresolved(geom::SurfaceKind::Cone)),
    ] {
        assert_eq!(got_enclosing(&split, &solid, delta), want, "δ {delta}");
    }
}

/// **A merge that is no run still refuses.** A cone band shorter than its
/// development resolves: the wall `(1, 0), (2 + h·tan 1°, 0), (2, h),
/// (1, h)` with `h = 10⁻⁷`, a full turn, its seam split at its middle and
/// placed obliquely. The whole ruling side, its two ends and the split
/// vertex, lies inside one cut of the slant radius (about `8.5·10⁻⁷`
/// here): three consecutive points at one point of the development whose
/// neighbours either way are on the rims, so none lies along the ruling
/// for the lane to cut a triangle beside. That is no run, and the face
/// refuses `SphereTrimShape { DevelopedMerge }` at every δ.
///
/// **By ε.** At `10⁻¹²` as above; at the default and `10⁻⁶` the band
/// is not built (its profile is a sliver beside ε, to the profile's
/// validation or to the revolve), and the row says so and stops.
#[test]
fn a_cone_band_shorter_than_the_developments_resolution_refuses() {
    use mesh::TessellateError as E;
    use mesh::types::SphereTrimCause;
    let h = 1e-7;
    let pts = [
        (1.0, 0.0),
        (2.0 + h * 1f64.to_radians().tan(), 0.0),
        (2.0, h),
        (1.0, h),
    ];
    let lp = ProfileLoop::polygon(pts.iter().map(|&(x, y)| Point2::new(x, y)));
    let built = profile::Profile::new(profile::SketchPlane::xy(), vec![lp])
        .validate(tol())
        .ok()
        .and_then(|profile| revolve(&profile, axis_y(), Revolution::Full, tol()).ok());
    let Some(revolved) = built else {
        assert!(tol().eps() > 1e-12, "the band revolves at ε 1e-12");
        eprintln!("SKIPPED at ε {:e}: the band is not built", tol().eps());
        return;
    };
    let body = revolved.body.into_body();
    let [seam] = cone_rulings(&body)[..] else {
        panic!("the band's cone face has one seam");
    };
    let split = split_and_placed(&body, &[seam], &[0.5]);
    for delta in [0.1, 0.01, 0.001] {
        assert!(
            matches!(
                mesh::tessellate(&split, delta, tol()),
                Err(E::SphereTrimShape {
                    cause: SphereTrimCause::DevelopedMerge,
                    ..
                })
            ),
            "δ {delta}"
        );
    }
}
