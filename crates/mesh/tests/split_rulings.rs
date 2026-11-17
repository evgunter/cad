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

/// **Two points of one ruling closer than the development resolves
/// refuse: this row pins a gap, at ε `10⁻¹²`**
/// (`work/tesslane/a-ruling-split-within-the-developments-resolution-merges-two-points.md`;
/// its default-ε form is
/// [`a_draft_cone_ruling_split_within_the_developments_resolution_refuses`]).
/// The frustum wall's outer cone face with its seam split `10⁻⁹` of its
/// range from its start, placed obliquely. The face is the chart lane's,
/// which triangulates a cone face in its development, whose slant radii
/// are cut to 27 significant bits: the split vertex and the seam's end
/// lie closer than `2⁻²⁷` of their slant radius, develop to one point,
/// and the face refuses
/// `SphereTrimShape { DevelopedMerge }` at every δ.
///
/// **By ε.** At `10⁻¹²` as above. At the default ε and `10⁻⁶`
/// `split_edge` reads a split `10⁻⁹` from this body's seam end as no
/// split, so this body cannot be built, and the row asserts that instead.
/// That is this body's answer and not the class's: on a slender cone the
/// development resolves a coarser length, which a split reaches at the
/// default ε
/// ([`a_draft_cone_ruling_split_within_the_developments_resolution_refuses`]).
#[test]
fn a_ruling_split_within_the_developments_resolution_refuses() {
    use mesh::TessellateError as E;
    use mesh::types::SphereTrimCause;
    let body = turned(
        &[(1.0, 0.0), (3.0, 0.0), (2.0, 1.0), (1.5, 1.0)],
        Revolution::Full,
    );
    let seam = rulings(&body)[0];
    let mut unplaced = body.clone();
    let (t0, t1) = unplaced
        .get_curve_geom(unplaced.get_edge(seam).unwrap().curve)
        .unwrap()
        .certified()
        .unwrap()
        .params();
    let built = unplaced.split_edge(seam, t0 + (t1 - t0) * 1e-9, tol());
    if tol().eps() > 1e-12 {
        assert!(
            built.is_err(),
            "this body at ε {:e}: a split 1e-9 from its seam end is no split (a slender \
             cone reaches the merge at this ε: the draft-cone pin)",
            tol().eps()
        );
        return;
    }
    built.unwrap();
    let split = split_and_placed(&body, &[seam], &[1e-9]);
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

/// **On a draft-angle cone the development's resolution is a length a
/// split reaches at the default ε: this row pins a gap**
/// (`work/tesslane/a-ruling-split-within-the-developments-resolution-merges-two-points.md`).
/// A wall whose outer face is a cone of half-angle 0.1° (profile
/// `(1, 0), (2 + tan 0.1°, 0), (2, 1), (1, 1)`, a full turn), its seam
/// ruling split `10⁻⁵` of its length from its start, and split twice
/// `10⁻⁵` apart at its middle, placed obliquely. The development cuts a
/// slant radius (`r / sin α`, about 1150 here) to 27 significant bits,
/// which resolves no finer than `2⁻²⁷·r / sin α` ≈ `8.5·10⁻⁶`
/// along the ruling: the two points develop to one, and the face refuses
/// `SphereTrimShape { DevelopedMerge }` at every δ.
///
/// **By ε.** The same at the default, `10⁻⁶` and `10⁻¹²`: `split_edge`
/// admits a split `10⁻⁵` from an end at each.
#[test]
fn a_draft_cone_ruling_split_within_the_developments_resolution_refuses() {
    use mesh::TessellateError as E;
    use mesh::types::SphereTrimCause;
    let t = 0.1f64.to_radians().tan();
    let body = turned(
        &[(1.0, 0.0), (2.0 + t, 0.0), (2.0, 1.0), (1.0, 1.0)],
        Revolution::Full,
    );
    // The cone face's seam: the line edge whose ends differ in radius.
    let ruling = rulings(&body)
        .into_iter()
        .find(|&ek| {
            let c = body
                .get_curve_geom(body.get_edge(ek).unwrap().curve)
                .unwrap()
                .certified()
                .unwrap();
            let (t0, t1) = c.params();
            let (a, b) = (c.carrier().eval(t0), c.carrier().eval(t1));
            ((a.x * a.x + a.z * a.z).sqrt() - (b.x * b.x + b.z * b.z).sqrt()).abs() > 1e-12
        })
        .expect("the cone face's seam");
    for (what, fracs) in [
        ("an end split", &[1e-5][..]),
        ("two splits mid-ruling", &[0.5 + 1e-5, 0.5][..]),
    ] {
        let split = split_and_placed(&body, &[ruling], fracs);
        for delta in [0.1, 0.01, 0.001] {
            assert!(
                matches!(
                    mesh::tessellate(&split, delta, tol()),
                    Err(E::SphereTrimShape {
                        cause: SphereTrimCause::DevelopedMerge,
                        ..
                    })
                ),
                "{what}, δ {delta}"
            );
        }
    }
}
