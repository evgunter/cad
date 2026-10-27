//! **The wedge-end doors that need a curved body** (D1 tier 3): the
//! split's knife-edge refusal and its counter-rows, the curved
//! boolean's refusals of an undeclared kiss, and the blend and shell
//! consumers of a declared cusp body. A cusp or slit is legal at rest
//! iff jet-determinate, so tier 3 no longer refuses one nobody
//! declared: the op that could mint it owns that refusal. The audit
//! that places every op's door, including those pinned elsewhere, is
//! `work/gather/every-op-that-can-mint-a-wedge-end-refuses-an-undeclared-one.md`.
//!
//! The split and boolean doors are `topo`'s; their rows live here
//! because they need a real curved body, which `sweep` builds. The
//! profile's doors are pinned in `profile`'s own suites
//! (`declared_tangency.rs`, `path_property.rs`, `rejections.rs`).
//!
//! The consumer rows at the end hold a DECLARED cusp body — one that
//! reaches a consumer legally — to the other half of the clause:
//! a consumer with no wedge-0/2π answer refuses typed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{Open, Profile, ProfileLoop, Start};
use sweep::ExtrudeSide;
use sweep::blend::{BlendError, chamfer_edges, fillet_edges};
use sweep::test_support::{brick, finished, sketch_at};
use sweep::{Extruded, Extrusion, extrude};
use topo::construction::Construct;
use topo::{Body, BooleanErrorKind, ContactMark, EdgeKey, KnifeEdgeSite, ShellError};

fn tol() -> Tol {
    Tol::witness()
}

/// A full circle as two semicircular arcs, vertices at `(cx ± r, cy)`.
fn circle(cx: f64, cy: f64, r: f64) -> ProfileLoop<f64> {
    bulge_loop(vec![
        (Point2::new(cx + r, cy), 1.0),
        (Point2::new(cx - r, cy), 1.0),
    ])
}

fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> ProfileLoop<f64> {
    bulge_loop(vec![
        (Point2::new(x0, y0), 0.0),
        (Point2::new(x1, y0), 0.0),
        (Point2::new(x1, y1), 0.0),
        (Point2::new(x0, y1), 0.0),
    ])
}

/// `loops` on the xy plane at height `z0`, extruded `h` along `+z`.
fn extruded(loops: Vec<ProfileLoop<f64>>, z0: f64, h: f64) -> Extruded<f64> {
    let profile = Profile::new(sketch_at(z0), loops)
        .validate(tol())
        .expect("the fixture's profile validates");
    extrude(
        &profile,
        Extrusion::Distance {
            depth: h,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .expect("the fixture extrudes")
}

/// The 6 × 6 × 1 plate with a unit round hole on the z axis; the
/// hole's two arcs meet at `(±1, 0)`, so the plane `x = 1` is tangent
/// to the hole wall along the vertical line through a vertex.
fn plate_with_hole() -> Body<f64> {
    extruded(
        vec![rect(-3.0, -3.0, 3.0, 3.0), circle(0.0, 0.0, 1.0)],
        0.0,
        1.0,
    )
    .body
}

/// The lune between the internally tangent circles `(0,1) r 1` and
/// `(0,2) r 2`, `.cusp()` at the kiss — the DECLARED cusp.
fn lune() -> ProfileLoop<f64> {
    Open.at(Point2::new(0.0, 4.0))
        .angle(-std::f64::consts::FRAC_PI_2, tol())
        .unwrap()
        .line(2.0, tol())
        .unwrap()
        .turn(std::f64::consts::FRAC_PI_2, tol())
        .unwrap()
        .tangent_arc_to(Point2::new(0.0, 0.0), tol())
        .unwrap()
        .cusp()
        .tangent_arc_to(Start, tol())
        .unwrap()
        .into()
}

/// The edges check 4 marks `Tangent`, each by its two endpoints.
fn tangent_edges(body: &Body<f64>) -> Vec<(EdgeKey, [Point3<f64>; 2])> {
    let point = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    topo::contact_marks(body, tol())
        .expect("the body is tier-3 valid")
        .iter()
        .filter(|(_, m)| **m == ContactMark::Tangent)
        .map(|(e, _)| {
            let he = body.get_edge(e).unwrap().he_plus;
            let ends = [
                point(body.get_half_edge(he).unwrap().start),
                point(body.half_edge_end(he).unwrap()),
            ];
            (e, ends)
        })
        .collect()
}

fn on_the_kiss(p: &Point3<f64>) -> bool {
    p.x.abs() < 1e-9 && p.y.abs() < 1e-9
}

// ---------------------------------------------------------------------
// The split: no declaration channel, so every minted wedge end refuses.
// ---------------------------------------------------------------------

/// **A split plane tangent to a hole wall refuses at the split.** On
/// the hole's side of `x = 1` the material near the tangent line is two
/// crescents between the cut face and the wall, each vanishing to a
/// knife edge (a doubled cusp): jet-determinate, so tier 3 would pass
/// it, and nothing declared it. The reduction reads the graze and
/// refuses it there. Both orientations of the plane, since the side
/// that carries the crescents is `below` in one and `above` in the
/// other.
#[test]
fn a_split_tangent_to_a_hole_wall_refuses_the_knife_edge_it_would_mint() {
    let body = plate_with_hole();
    let body = sweep::test_support::finished("the body", body, tol());
    for normal in [1.0, -1.0] {
        let plane = topo::test_support::split_plane(
            Point3::new(1.0, 0.0, 0.0),
            Vec3::new(normal, 0.0, 0.0),
            geom_core::Tol::witness(),
        );
        let knife = match topo::split(&body, &plane, tol()) {
            Err(e) => *e
                .knife_edge()
                .unwrap_or_else(|| panic!("normal {normal}: refused {e:?}, not the knife edge")),
            Ok(_) => panic!("normal {normal}: answered"),
        };
        let wall = body.get_surface(body.get_face(knife.wall).unwrap().surface);
        assert_eq!(
            wall.map(geom::Surface::kind),
            Some(geom::SurfaceKind::Cylinder),
            "normal {normal}: the hole's wall"
        );
        let KnifeEdgeSite::Edge(seam) = knife.at else {
            panic!("normal {normal}: read along the seam, got {:?}", knife.at);
        };
        let he = body.get_edge(seam).unwrap().he_plus;
        for v in [
            body.get_half_edge(he).unwrap().start,
            body.half_edge_end(he).unwrap(),
        ] {
            let q = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            assert!(
                (q.x - 1.0).abs() < 1e-12 && q.y.abs() < 1e-12,
                "normal {normal}: the seam at (1, 0), {q:?}"
            );
        }
    }
}

/// The door's two counter-rows. A plane cutting THROUGH the hole meets
/// its wall transversally and cuts clean; a cut across a DECLARED cusp
/// (the lune's strut, at mid-height) cuts clean and keeps the cusp,
/// which is inherited, not minted.
#[test]
fn a_split_through_the_hole_or_across_a_declared_cusp_still_cuts() {
    let through = topo::test_support::split_plane(
        Point3::new(0.5, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        geom_core::Tol::witness(),
    );
    let halves = topo::split(
        &sweep::test_support::finished("the operand", plate_with_hole(), tol()),
        &through,
        tol(),
    )
    .expect("a transverse cut");
    for (side, part) in [("above", &halves.above), ("below", &halves.below)] {
        let body = part.body().expect("material on both sides of x = 0.5");
        assert_eq!(
            topo::validate_geometric(body, tol()),
            Ok(()),
            "{side} is tier-3 valid"
        );
        assert!(
            tangent_edges(body).is_empty(),
            "{side}: no tangent edge on a transverse cut"
        );
    }

    let cusp = extruded(vec![lune()], 0.0, 1.0);
    let operand = sweep::test_support::finished("the fixture", cusp.body.clone(), tol());
    let mid = topo::test_support::split_plane(
        Point3::new(0.0, 0.0, 0.5),
        Vec3::new(0.0, 0.0, 1.0),
        geom_core::Tol::witness(),
    );
    let halves = topo::split(&operand, &mid, tol()).expect("a cut across the strut");
    for (side, part) in [("above", &halves.above), ("below", &halves.below)] {
        let body = part.body().expect("material on both sides of z = 0.5");
        let tangent = tangent_edges(body);
        assert_eq!(tangent.len(), 1, "{side}: the strut's half, {tangent:?}");
        assert!(
            tangent[0].1.iter().all(on_the_kiss),
            "{side}: the tangent edge is the declared strut, {tangent:?}"
        );
    }
}

/// **A π seam at the cut is not a wedge end, and cuts.** The rounded
/// shoulder: a quarter arc centred at the origin runs from the corner
/// `(0,1)` down to `(-1,0)`, so `y = 1` is tangent to its wall along
/// the vertex ruling — the hole-wall row's shape exactly — but here the
/// wall's material and the section's lie on the SAME side (aligned
/// normals): the piece below meets the cut in a smooth seam. Only the
/// material pairing tells the two apart, and this row fails a door
/// that decides by surface kind.
#[test]
fn a_split_tangent_to_a_rounded_shoulder_cuts_at_a_seam() {
    let body = shoulder(1.0);
    let on_the_ruling = |p: &Point3<f64>| p.x.abs() < 1e-9 && (p.y - 1.0).abs() < 1e-9;
    let halves = topo::split(&body, &shoulder_cut(), tol())
        .unwrap_or_else(|e| panic!("a seam at the cut must cut, got {e:?}"));
    let mut seams = 0;
    for (side, part) in [("above", &halves.above), ("below", &halves.below)] {
        let piece = part
            .body()
            .unwrap_or_else(|| panic!("material {side} y = 1"));
        assert_eq!(
            topo::validate_geometric(piece, tol()),
            Ok(()),
            "{side} is tier-3 valid"
        );
        for (edge, ends) in tangent_edges(piece) {
            assert!(
                ends.iter().all(on_the_ruling),
                "{side}: a tangent edge off the ruling, {ends:?}"
            );
            // The section plane against a cylinder is jet-determinate
            // (κ_rel = 1/r), so the must-carry rule demands the
            // intrinsic description over the seam's CURRENT pair.
            let e = piece.get_edge(edge).unwrap();
            let face_surface = |he| {
                let face = piece.face_of_half_edge(he).unwrap();
                piece.get_face(face).unwrap().surface
            };
            let faces =
                geom_brep::SurfacePair::new(face_surface(e.he_plus), face_surface(e.he_minus));
            let description = piece
                .get_curve_geom(e.curve)
                .and_then(|g| g.certified())
                .expect("the seam is described")
                .description();
            let geom_brep::EdgeDescription::TangentIntersection { pair, .. } = *description else {
                panic!("{side}: the seam stores {description:?}, not its intrinsic tangency");
            };
            assert_eq!(
                pair, faces,
                "{side}: the seam's tangency names its two faces"
            );
            seams += 1;
        }
    }
    assert_eq!(seams, 1, "the one seam the cut mints");
}

/// The rounded shoulder of [`a_split_tangent_to_a_rounded_shoulder_cuts_at_a_seam`],
/// extruded `h` along `+z`: the unit quarter arc from `(0,1)` down to
/// `(-1,0)` below the 2 × 2 block, `2 + π/4` of section below `y = 1`
/// and `2` above.
fn shoulder(h: f64) -> topo::AtRestBody<f64> {
    let bulge = (std::f64::consts::PI / 8.0).tan();
    let outline = bulge_loop(vec![
        (Point2::new(-1.0, 0.0), 0.0),
        (Point2::new(2.0, 0.0), 0.0),
        (Point2::new(2.0, 2.0), 0.0),
        (Point2::new(0.0, 2.0), 0.0),
        (Point2::new(0.0, 1.0), bulge),
    ]);
    finished("the shoulder", extruded(vec![outline], 0.0, h).body, tol())
}

/// `y = 1`, tangent to the shoulder's wall along its vertex ruling,
/// normal `−y`. Normal `+y` refuses earlier, at the reduction
/// (`ConsecutiveOnSectors`), for a reason of its own:
/// `work/honesplit/split-shoulder-refuses-one-orientation-at-the-reduction.md`.
fn shoulder_cut() -> topo::SplitPlane<f64> {
    topo::test_support::split_plane(Point3::new(0.0, 1.0, 0.0), Vec3::new(0.0, -1.0, 0.0), tol())
}

/// **The tangent ruling stores what the must-carry rule demands, at
/// every depth.** The shoulder's seam is a ruling of the unit cylinder
/// against its tangent plane, `κ_rel = 1`, and the rule levers the
/// sagitta at the faces' extent (`topo::wedge_reach`): the wall's
/// quarter arc and the section's width reach past the radius, so the
/// arm is the radius and the sagitta `1/2` at every depth. Every depth
/// stores the intrinsic tangency, both pieces tier-3 valid with their
/// closed-form volumes; the depths are the ones that put a sagitta
/// levered at the ruling's own length under, in and over the band
/// (multiples of `√(2ε)`, 2% either side of each boundary).
#[test]
fn a_split_tangent_to_a_thin_shoulder_describes_its_ruling_by_the_rule() {
    let eps = tol().eps();
    let k = tol().k();
    let unit = (2.0 * eps).sqrt();
    let root_k = k.sqrt();
    for c in [
        0.25,
        0.5,
        0.8,
        0.99,
        1.01,
        1.5,
        2.5,
        0.99 * root_k,
        1.01 * root_k,
        4.0,
        8.0,
    ] {
        let h = c * unit;
        let label = format!("h = {c}·√(2ε) = {h:e}");
        let result = topo::split(&shoulder(h), &shoulder_cut(), tol());
        let halves = result.unwrap_or_else(|e| panic!("{label}: the shoulder cuts, got {e:?}"));
        let want = [("above", 2.0 + std::f64::consts::FRAC_PI_4), ("below", 2.0)];
        let mut seams = 0;
        for ((side, area), part) in want.into_iter().zip([&halves.above, &halves.below]) {
            let piece = part
                .body()
                .unwrap_or_else(|| panic!("{label}: material {side}"));
            assert_eq!(
                topo::validate_geometric(piece, tol()),
                Ok(()),
                "{label}: {side} is tier-3 valid"
            );
            let volume =
                topo::test_support::pinned_volume("the measured body", piece, tol()).volume;
            assert!(
                (volume - area * h).abs() <= 1e-9 * area * h,
                "{label}: {side} holds {volume:e}, want {:e}",
                area * h
            );
            let point = |v| *piece.get_point(piece.get_vertex(v).unwrap().point).unwrap();
            let surface_of = |he| {
                let face = piece.face_of_half_edge(he).unwrap();
                piece.get_face(face).unwrap().surface
            };
            let on_the_ruling = |p: Point3<f64>| p.x.abs() <= eps && (p.y - 1.0).abs() <= eps;
            // The seam: the ruling's edge between the wall and the cut.
            for (_, e) in piece.edges() {
                let start = piece.get_half_edge(e.he_plus).unwrap().start;
                let end = piece.half_edge_end(e.he_plus).unwrap();
                let pair = [surface_of(e.he_plus), surface_of(e.he_minus)];
                let kinds = pair.map(|s| piece.get_surface(s).unwrap().kind());
                if !(on_the_ruling(point(start)) && on_the_ruling(point(end)))
                    || !kinds.contains(&geom::SurfaceKind::Cylinder)
                {
                    continue;
                }
                let description = piece
                    .get_curve_geom(e.curve)
                    .and_then(|g| g.certified())
                    .expect("the seam is described")
                    .description();
                assert!(
                    matches!(
                        description,
                        geom_brep::EdgeDescription::TangentIntersection { pair: named, .. }
                            if *named == geom_brep::SurfacePair::new(pair[0], pair[1])
                    ),
                    "{label}: {side}'s seam is the wall's and the section's tangency, got \
                     {description:?}"
                );
                seams += 1;
            }
        }
        assert_eq!(seams, 1, "{label}: the one seam the cut mints");
    }
}

/// **A cut across a tangent seam describes each piece by the rule over
/// the piece.** The `y < 1` piece of the shoulder cut at its seam (the
/// unit cylinder's ruling `x = 0, y = 1`, `κ_rel = 1`, stored as the
/// intrinsic tangency) is cut again by `z = 1 − d` (both normals) and
/// `z = d`, so the seam splits into a long piece and one of length `d`.
/// The rule levers each piece's sagitta at its faces' extent
/// (`topo::wedge_reach`), and the short piece's wall and section still
/// run the shoulder's width, past the radius: both pieces keep the
/// intrinsic tangency, tier-3 valid with their closed-form volumes, at
/// the lengths that put a sagitta levered at the piece's own length
/// under, in and over the band (multiples of `√(2ε)`, 1% either side
/// of each boundary).
#[test]
fn a_cut_across_a_tangent_seam_describes_each_piece_by_the_rule() {
    let eps = tol().eps();
    let k = tol().k();
    let unit = (2.0 * eps).sqrt();
    let root_k = k.sqrt();
    let area = 2.0 + std::f64::consts::FRAC_PI_4;
    let topo::SplitPart::Body(wall) = topo::split(&shoulder(1.0), &shoulder_cut(), tol())
        .expect("the shoulder cuts at its seam")
        .above
    else {
        panic!("material on the wall's side of the seam")
    };
    let wall = wall;
    for c in [
        0.25,
        0.5,
        0.99,
        1.01,
        1.5,
        0.99 * root_k,
        1.01 * root_k,
        4.0,
        8.0,
    ] {
        let d = c * unit;
        for (z, up) in [(1.0 - d, 1.0), (1.0 - d, -1.0), (d, 1.0), (d, -1.0)] {
            let label = format!("d = {c}·√(2ε) = {d:e}, z = {z}, normal {up}·z");
            let plane = topo::test_support::split_plane(
                Point3::new(0.0, 0.0, z),
                Vec3::new(0.0, 0.0, up),
                tol(),
            );
            let result = topo::split(&wall, &plane, tol());
            let halves = result.unwrap_or_else(|e| panic!("{label}: the seam cuts, got {e:?}"));
            // The plane's normal side holds `z > cut` for `+z`.
            let heights = if up > 0.0 { [1.0 - z, z] } else { [z, 1.0 - z] };
            for (height, part) in heights.into_iter().zip([&halves.above, &halves.below]) {
                let piece = part
                    .body()
                    .unwrap_or_else(|| panic!("{label}: both sides hold material"));
                assert_eq!(
                    topo::validate_geometric(piece, tol()),
                    Ok(()),
                    "{label}: the {height:e} piece is tier-3 valid"
                );
                let volume =
                    topo::test_support::pinned_volume("the measured body", piece, tol()).volume;
                assert!(
                    (volume - area * height).abs() <= 1e-9 * area * height,
                    "{label}: the {height:e} piece holds {volume:e}, want {:e}",
                    area * height
                );
                let point = |v| *piece.get_point(piece.get_vertex(v).unwrap().point).unwrap();
                let surface_of = |he| {
                    let face = piece.face_of_half_edge(he).unwrap();
                    piece.get_face(face).unwrap().surface
                };
                let on_the_ruling = |p: Point3<f64>| p.x.abs() <= eps && (p.y - 1.0).abs() <= eps;
                let seams: Vec<_> = piece
                    .edges()
                    .filter(|(_, e)| {
                        let start = piece.get_half_edge(e.he_plus).unwrap().start;
                        let end = piece.half_edge_end(e.he_plus).unwrap();
                        on_the_ruling(point(start))
                            && on_the_ruling(point(end))
                            && [e.he_plus, e.he_minus].into_iter().any(|he| {
                                piece.get_surface(surface_of(he)).unwrap().kind()
                                    == geom::SurfaceKind::Cylinder
                            })
                    })
                    .map(|(_, e)| e.clone())
                    .collect();
                let [seam] = &seams[..] else {
                    panic!(
                        "{label}: the {height:e} piece has one seam, got {}",
                        seams.len()
                    )
                };
                let curve = piece
                    .get_curve_geom(seam.curve)
                    .and_then(|g| g.certified())
                    .expect("the seam is described");
                let faces = [surface_of(seam.he_plus), surface_of(seam.he_minus)];
                assert!(
                    matches!(
                        curve.description(),
                        geom_brep::EdgeDescription::TangentIntersection { pair, .. }
                            if *pair == geom_brep::SurfacePair::new(faces[0], faces[1])
                    ),
                    "{label}: the {height:e} piece keeps the tangency of its faces, got {:?}",
                    curve.description()
                );
            }
        }
    }
}

/// The shoulder seam a piece holds: its description's arm, and the kinds
/// of the surfaces it names, in sorted order.
fn shoulder_seam(piece: &Body<f64>) -> (&'static str, Vec<geom::SurfaceKind>) {
    let eps = tol().eps();
    let on_the_ruling = |p: Point3<f64>| p.x.abs() <= eps && (p.y - 1.0).abs() <= eps;
    let point = |v| *piece.get_point(piece.get_vertex(v).unwrap().point).unwrap();
    let kind = |s| piece.get_surface(s).unwrap().kind();
    let seams: Vec<_> = piece
        .edges()
        .filter(|(_, e)| {
            let start = piece.get_half_edge(e.he_plus).unwrap().start;
            let end = piece.half_edge_end(e.he_plus).unwrap();
            on_the_ruling(point(start)) && on_the_ruling(point(end))
        })
        .collect();
    let [(_, seam)] = seams[..] else {
        panic!("one seam on the ruling, got {}", seams.len())
    };
    let curve = piece
        .get_curve_geom(seam.curve)
        .and_then(|g| g.certified())
        .expect("the seam is described");
    let (arm, mut kinds) = match curve.description() {
        geom_brep::EdgeDescription::OnPair { pair } => ("OnPair", pair.keys().map(kind).to_vec()),
        geom_brep::EdgeDescription::Chart(chart) => ("Chart", vec![kind(chart.surface)]),
        geom_brep::EdgeDescription::Intersection { pair, .. } => {
            ("Intersection", pair.keys().map(kind).to_vec())
        }
        geom_brep::EdgeDescription::TangentIntersection { pair, .. } => {
            ("TangentIntersection", pair.keys().map(kind).to_vec())
        }
        geom_brep::EdgeDescription::Scaffold(_) => ("Scaffold", Vec::new()),
    };
    kinds.sort_by_key(|k| format!("{k:?}"));
    (arm, kinds)
}

/// **One locus, one description, whichever door made it.** The thin
/// shoulder's seam the chord mints (a cut at depth `h = ½√(2ε)`) and
/// the short piece `split_edge` leaves of the deep shoulder's tangent
/// seam (a cut at `z = d`, `d = ½√(2ε)`) are the same geometric locus —
/// a short ruling of the unit cylinder against its tangent plane,
/// between faces that run past the radius — and both store it as the
/// tangency of those two surfaces.
#[test]
fn the_shoulder_seam_stores_one_description_through_the_chord_and_split_edge() {
    let short = 0.5 * (2.0 * tol().eps()).sqrt();
    let chord = topo::split(&shoulder(short), &shoulder_cut(), tol())
        .expect("the thin shoulder cuts at its seam");
    let topo::SplitPart::Body(wall) = topo::split(&shoulder(1.0), &shoulder_cut(), tol())
        .expect("the shoulder cuts at its seam")
        .above
    else {
        panic!("material on the wall's side of the seam")
    };
    let wall = wall;
    let plane =
        topo::test_support::split_plane(Point3::new(0.0, 0.0, short), Vec3::unit_z(), tol());
    let pieces = topo::split(&wall, &plane, tol()).expect("the seam cuts short");
    let by_split_edge = pieces.below.body().expect("the short piece");
    let want = (
        "TangentIntersection",
        vec![geom::SurfaceKind::Cylinder, geom::SurfaceKind::Plane],
    );
    assert_eq!(
        shoulder_seam(chord.above.body().expect("material above")),
        want,
        "the chord's seam is the tangency"
    );
    assert_eq!(
        shoulder_seam(by_split_edge),
        want,
        "split_edge's short piece is the same tangency"
    );
}

/// The cap of `piece` lying in `z = height`, read within ε: the thin
/// shoulder's two caps are only `½√(2ε)` apart.
fn cap_at(piece: &Body<f64>, height: f64) -> topo::FaceKey {
    let caps: Vec<_> = piece
        .faces()
        .filter(|(_, f)| {
            matches!(
                piece.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if normal.z.abs() > 0.5 && (origin.z - height).abs() < tol().eps()
            )
        })
        .map(|(k, _)| k)
        .collect();
    let [cap] = caps[..] else {
        panic!("one cap at z = {height}, got {}", caps.len())
    };
    cap
}

/// **An offset that lengthens the seam keeps the tangency.** The thin
/// shoulder's chord seam (`h = ½√(2ε)`) is the intrinsic tangency, its
/// wedge levered at the faces' extent; it is re-anchored when either
/// cap moves out by `d = c·√(2ε)`, the seam then `h + d` long, and the
/// body is valid at rest with the seam still the tangency.
#[test]
fn an_offset_cap_re_asks_the_shoulder_pair_over_its_new_length() {
    let h = 0.5 * (2.0 * tol().eps()).sqrt();
    let wall = topo::split(&shoulder(h), &shoulder_cut(), tol())
        .expect("the thin shoulder cuts at its seam")
        .above
        .body()
        .expect("material on the wall's side")
        .clone();
    assert_eq!(
        shoulder_seam(&wall).0,
        "TangentIntersection",
        "the short seam is the tangency"
    );
    // Lengthened to `(½ + c)·√(2ε)`, past the band's `√K` at K = 10 at
    // every ε row (an absolute `d` of a millimetre is in band at 1e-6).
    for height in [0.0, h] {
        for d in [4.0, 16.0].map(|c| c * (2.0 * tol().eps()).sqrt()) {
            let label = format!("the cap at z = {height:e} moved out by {d:e}");
            let mut body = topo::Body::clone(&wall);
            topo::replace_faces_offset(&mut body, &[cap_at(&wall, height)], d, tol())
                .unwrap_or_else(|e| panic!("{label}: the offset builds, got {e:?}"));
            assert_eq!(
                topo::validate_geometric(&body, tol()),
                Ok(()),
                "{label}: the body is valid at rest"
            );
            assert_eq!(
                shoulder_seam(&body),
                (
                    "TangentIntersection",
                    vec![geom::SurfaceKind::Cylinder, geom::SurfaceKind::Plane]
                ),
                "{label}: the lengthened seam is the determinate tangency"
            );
        }
    }
}

/// **A join of the seam's pieces keeps the tangency.** The wall piece
/// of a shoulder `7·√(2ε)` deep carries its seam as the intrinsic
/// tangency; `split_edge` cuts two pieces `d = c·√(2ε)` long off its
/// start, each still the tangency (its faces run past the radius, the
/// wedge's arm), and the join takes the seam back to one tangency,
/// valid at rest.
#[test]
fn a_join_of_the_seams_pieces_keeps_the_tangency() {
    let unit = (2.0 * tol().eps()).sqrt();
    let ruling = |piece: &Body<f64>| -> Vec<EdgeKey> {
        let eps = tol().eps();
        let on = |p: Point3<f64>| p.x.abs() <= eps && (p.y - 1.0).abs() <= eps;
        let point = |v| *piece.get_point(piece.get_vertex(v).unwrap().point).unwrap();
        piece
            .edges()
            .filter(|(_, e)| {
                on(point(piece.get_half_edge(e.he_plus).unwrap().start))
                    && on(point(piece.half_edge_end(e.he_plus).unwrap()))
            })
            .map(|(k, _)| k)
            .collect()
    };
    let description = |piece: &Body<f64>, e: EdgeKey| {
        let curve = piece.get_edge(e).unwrap().curve;
        let c = piece.get_curve_geom(curve).unwrap().certified().unwrap();
        (format!("{:?}", c.description()), c.params())
    };
    for c in [0.4, 0.9] {
        let d = c * unit;
        let label = format!("d = {c}·√(2ε)");
        let mut body = topo::split(&shoulder(7.0 * unit), &shoulder_cut(), tol())
            .expect("the shoulder cuts at its seam")
            .above
            .body()
            .expect("material on the wall's side")
            .clone()
            .into_body();
        let [seam] = ruling(&body)[..] else {
            panic!("{label}: one seam")
        };
        let cut = |body: &mut Body<f64>, e: EdgeKey, len: f64| {
            let (_, (t0, t1)) = description(body, e);
            body.split_edge(e, t0 + (t1 - t0) * (d / len), tol())
                .unwrap_or_else(|err| panic!("{label}: the seam splits, got {err:?}"))
                .new_edge
        };
        let rest = cut(&mut body, seam, 7.0 * unit);
        cut(&mut body, rest, 7.0 * unit - d);
        let tangent = |body: &Body<f64>| {
            ruling(body)
                .into_iter()
                .map(|e| description(body, e).0.starts_with("TangentIntersection"))
                .collect::<Vec<_>>()
        };
        assert_eq!(
            tangent(&body),
            [true; 3],
            "{label}: every piece is the tangency"
        );
        let band = geom_core::Band::linear(tol()).unwrap();
        body.join_edges(band, tol())
            .unwrap_or_else(|e| panic!("{label}: the pieces join, got {e:?}"));
        assert_eq!(
            topo::validate_geometric(&body, tol()),
            Ok(()),
            "{label}: valid at rest"
        );
        assert_eq!(tangent(&body), [true], "{label}: one tangency seam");
    }
}

/// **A cut turned about the tangent ruling refuses before the chord's
/// first-order reading.** The section table reads the turned plane
/// tangent while the wall's sag off it stays in the zero band, and the
/// must-carry rule's first order would meter the turn over the ruling;
/// at every turn the rule could leave in band or read a corner, the
/// reduction refuses first, so neither
/// [`topo::SplitJoinError::TangentChordEscalated`] nor
/// [`topo::SplitJoinError::TangentChordRefuted`] is reached, and a cut
/// that answers is tier-3 valid.
#[test]
fn a_cut_turned_about_the_ruling_refuses_before_the_chord() {
    let eps = tol().eps();
    for h in [1.0, 0.5 * (2.0 * eps).sqrt()] {
        let body = shoulder(h);
        for theta in [
            1e-10_f64, 1e-9, 3e-9, 1e-8, 3e-8, 1e-7, 1e-6, 1e-5, 4e-5, 1e-4,
        ] {
            let label = format!("h = {h:e}, θ = {theta:e}");
            let normal = Vec3::new(theta.sin(), -theta.cos(), 0.0);
            let plane = topo::test_support::split_plane(Point3::new(0.0, 1.0, 0.0), normal, tol());
            match topo::split(&body, &plane, tol()) {
                Ok(halves) => {
                    for part in [&halves.above, &halves.below] {
                        if let Some(piece) = part.body() {
                            assert_eq!(
                                topo::validate_geometric(piece, tol()),
                                Ok(()),
                                "{label}: a side that answers is tier-3 valid"
                            );
                        }
                    }
                }
                Err(topo::SplitError::Join(
                    e @ (topo::SplitJoinError::TangentChordEscalated { .. }
                    | topo::SplitJoinError::TangentChordRefuted { .. }),
                )) => panic!("{label}: the chord's first-order arm is reached: {e:?}"),
                Err(_) => {}
            }
        }
    }
}

// ---------------------------------------------------------------------
// The boolean: a tangent operand pair refuses at the op.
// ---------------------------------------------------------------------

/// **The internal kiss the curved boolean could turn into a wedge end
/// refuses typed at the op** (the crescent a subtract would leave): it
/// stops at the crossing layer's frontier; the route is
/// `work/tang/declared-cusps-second-order-wedge-arm.md` item 3.
#[test]
fn a_boolean_that_would_kiss_a_curved_face_refuses_typed_at_the_op() {
    let disc = |what: &str, c: ProfileLoop<f64>, z0: f64, z1: f64| {
        finished(what, extruded(vec![c], z0, z1).body, tol())
    };
    let big = disc("the big disc", circle(0.0, 2.0, 2.0), 0.0, 1.0);
    let small = disc("the small disc", circle(0.0, 1.0, 1.0), -1.0, 3.0);
    match topo::subtract(&big, &small, tol()) {
        Err(e) => assert_eq!(
            e.kind(),
            BooleanErrorKind::CurvedPierceUnsupported,
            "internal kiss, subtract: {e}"
        ),
        Ok(r) => panic!("internal kiss, subtract: expected a refusal, got a result {r:?}"),
    }
}

/// **A plane cutter tangent to a hole wall answers every op.** The
/// cutter's face `x = 1` touches the unit hole of a 6 × 6 × 1 plate
/// along the ruling `x = 1, y = 0`, their outward normals agreeing
/// there: the glue door glues the pair `Tangent`, claiming nothing of
/// the materials, and each op's result is tier-3 valid at its closed
/// form, but the plate less the cutter's. That leaves the ruling in the
/// cut face, the hole's wall tangent to it on both sides: the material
/// parts it into two coincident edges, each bounding a cusp (D1's
/// doubled cusp), whose reading on a line of coincident edges is not
/// built, and it refuses `EdgeInFace` (`Tangent`) rather than build the
/// one edge the face passes through.
#[test]
fn a_plane_tangent_to_a_hole_answers_every_op() {
    let plate = finished("the holed plate", plate_with_hole(), tol());
    let cutter = finished(
        "the cutter",
        brick::<f64>((1.0, 4.0), (-4.0, 4.0), (-1.0, 2.0), tol()),
        tol(),
    );
    let pi = std::f64::consts::PI;
    match topo::subtract(&plate, &cutter, tol()) {
        Err(topo::BooleanError::EdgeInFace {
            gap: topo::EdgeInFaceGap::Tangent,
            ..
        }) => {}
        other => panic!("plate ∖ cutter: want EdgeInFace (Tangent), got {other:?}"),
    }
    for (name, got, want) in [
        (
            "cutter ∖ plate",
            topo::subtract(&cutter, &plate, tol()),
            60.0,
        ),
        (
            "plate ∪ cutter",
            topo::union(&plate, &cutter, tol()),
            96.0 - pi,
        ),
        (
            "plate ∩ cutter",
            topo::intersect(&plate, &cutter, tol()),
            12.0,
        ),
    ] {
        let body = match got {
            Ok(topo::BooleanResult::Body(b)) => b.body,
            other => panic!("{name}: want a body, got {other:?}"),
        };
        let v = topo::test_support::pinned_volume("the measured body", &body, tol()).volume;
        assert!((v - want).abs() < 1e-9, "{name}: volume {v}, want {want}");
        assert_eq!(
            topo::validate_geometric(&body, tol()),
            Ok(()),
            "{name}: tier 3"
        );
    }
}

/// **Two discs kissing externally touch at every op**: unit discs about
/// `(0, 0)` and `(2, 0)`, extruded over `z ∈ [0, 1]`, meet along the
/// ruling `x = 1, y = 0` only. No edge of the result has a wedge end:
/// each disc keeps its own edge on the ruling, ∪ is the two solids with
/// the edge–edge record of those two edges, ∖ returns each disc, and ∩
/// is empty.
#[test]
fn two_discs_kissing_externally_touch_at_every_op() {
    let disc = |what: &str, cx: f64| {
        finished(
            what,
            extruded(vec![circle(cx, 0.0, 1.0)], 0.0, 1.0).body,
            tol(),
        )
    };
    crate::common::touch_union::assert_touch_at_every_op(
        "the external kiss",
        &disc("the left disc", 0.0),
        &disc("the right disc", 2.0),
        [Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 1.0)],
        [Point3::new(0.0, 0.0, 0.5), Point3::new(2.0, 0.0, 0.5)],
    );
}

// ---------------------------------------------------------------------
// The consumers: a declared cusp body refuses typed or comes out sane.
// ---------------------------------------------------------------------

/// **Chamfer and fillet refuse the cusp strut typed** — on the lune's
/// cusp (wedge 0) and on a lune-shaped hole's slit (wedge 2π) alike:
/// the supports meet tangentially, so the blend has no side to sit in.
#[test]
fn chamfer_and_fillet_refuse_a_cusp_or_slit_strut_typed() {
    let cusp = extruded(vec![lune()], 0.0, 1.0);
    let slit = extruded(vec![rect(-1.0, -1.0, 3.0, 5.0), lune()], 0.0, 1.0);
    for (name, built, loop_index) in [("cusp", &cusp, 0), ("slit", &slit, 1)] {
        let strut = built.strut_edges()[loop_index]
            .iter()
            .flatten()
            .copied()
            .find(|&e| tangent_edges(&built.body).iter().any(|(t, _)| *t == e))
            .expect("the strut the cusp joint swept is marked Tangent");
        let operand = sweep::test_support::at_rest(&built.body, tol());
        for (verb, got) in [
            (
                "chamfer",
                chamfer_edges(&operand, &[strut], 0.05, tol()).map(|_| ()),
            ),
            (
                "fillet",
                fillet_edges(&operand, &[strut], 0.05, tol()).map(|_| ()),
            ),
        ] {
            match got {
                Err(refusal) => assert!(
                    matches!(refusal.error, BlendError::TangentialEdge { edge, .. } if edge == strut),
                    "{name} {verb}: expected TangentialEdge on the strut, got {refusal:?}"
                ),
                Ok(()) => panic!("{name} {verb}: blended a wedge-{name} edge"),
            }
        }
    }
}

/// **Shell refuses a cusp or slit body typed.** Neither refusal is a
/// wedge-specific door — an inward offset of the cusp's walls has no
/// place to meet near the kiss, and the slit's two unequal cylinders
/// have no closed-form pose — but both are typed at the op, at a thin
/// wall and a thick one, and no body ships.
#[test]
fn shell_refuses_a_cusp_or_slit_body_typed() {
    let cusp = extruded(vec![lune()], 0.0, 1.0);
    let slit = extruded(vec![rect(-1.0, -1.0, 3.0, 5.0), lune()], 0.0, 1.0);
    for (name, body) in [("cusp", &cusp.body), ("slit", &slit.body)] {
        for thickness in [1e-3, 0.05] {
            match topo::shell(
                &finished("the operand", body.clone(), tol()),
                thickness,
                tol(),
            ) {
                Err(ShellError::Face { .. }) => {}
                Err(other) => panic!("{name} at {thickness}: an unexpected refusal {other}"),
                Ok(_) => panic!("{name} at {thickness}: shelled a wedge-{name} body"),
            }
        }
    }
}

/// **A nested aligned touch answers every op**: a unit disc about
/// `(1, 0)` inside a radius-2 disc about the origin, both over
/// `z ∈ [0, 1]`, their walls tangent along the ruling `x = 2, y = 0`
/// with their outward normals agreeing there. The glue door glues the
/// walls `Tangent`, claiming nothing of the materials, the lump reads
/// the thin wall `In`, and each op is tier-3 valid at its closed form.
#[test]
fn a_nested_aligned_touch_answers_every_op() {
    let disc =
        |what: &str, c: ProfileLoop<f64>| finished(what, extruded(vec![c], 0.0, 1.0).body, tol());
    let fat = disc("the fat disc", circle(0.0, 0.0, 2.0));
    let thin = disc("the thin disc", circle(1.0, 0.0, 1.0));
    let pi = std::f64::consts::PI;
    for (name, got, want) in [
        (
            "fat ∪ thin",
            topo::union(&fat, &thin, tol()),
            Some(4.0 * pi),
        ),
        (
            "fat ∖ thin",
            topo::subtract(&fat, &thin, tol()),
            Some(3.0 * pi),
        ),
        ("thin ∖ fat", topo::subtract(&thin, &fat, tol()), None),
        ("fat ∩ thin", topo::intersect(&fat, &thin, tol()), Some(pi)),
    ] {
        match (got, want) {
            (Ok(topo::BooleanResult::Body(b)), Some(want)) => {
                let v =
                    topo::test_support::pinned_volume("the measured body", &b.body, tol()).volume;
                assert!((v - want).abs() < 1e-9, "{name}: volume {v}, want {want}");
                assert_eq!(
                    topo::validate_geometric(&b.body, tol()),
                    Ok(()),
                    "{name}: tier 3"
                );
            }
            (Ok(topo::BooleanResult::Empty), None) => {}
            (other, want) => panic!("{name}: want {want:?}, got {other:?}"),
        }
    }
}

/// **A thin shoulder's seam is clean at its build ε.** The wall piece
/// of a shoulder `c·√(2ε)` deep (`c ≤ 1`, where a sagitta levered at the
/// ruling's own length would be zero-side) stores its seam as the
/// tangency of faces that run past the radius, and tier 3 reads the
/// body at the ε it was built at.
#[test]
fn a_thin_shoulders_seam_is_clean_at_its_build_eps() {
    let eps = tol().eps();
    for c in [0.25, 0.5, 0.8] {
        let wall = topo::split(&shoulder(c * (2.0 * eps).sqrt()), &shoulder_cut(), tol())
            .expect("the thin shoulder cuts at its seam")
            .above
            .body()
            .expect("material on the wall's side")
            .clone()
            .into_body();
        assert_eq!(
            shoulder_seam(&wall).0,
            "TangentIntersection",
            "c = {c}: the seam is the tangency"
        );
        assert_eq!(
            topo::validate_geometric(&wall, tol()),
            Ok(()),
            "c = {c}: clean at the ε it was built at"
        );
    }
}

/// The shoulder seam a piece holds, by key ([`shoulder_seam`]'s edge).
fn shoulder_seam_edge(piece: &Body<f64>) -> EdgeKey {
    let eps = tol().eps();
    let on_the_ruling = |p: Point3<f64>| p.x.abs() <= eps && (p.y - 1.0).abs() <= eps;
    let point = |v| *piece.get_point(piece.get_vertex(v).unwrap().point).unwrap();
    let seams: Vec<_> = piece
        .edges()
        .filter(|(_, e)| {
            let start = piece.get_half_edge(e.he_plus).unwrap().start;
            let end = piece.half_edge_end(e.he_plus).unwrap();
            on_the_ruling(point(start)) && on_the_ruling(point(end))
        })
        .map(|(k, _)| k)
        .collect();
    let [seam] = seams[..] else {
        panic!("one seam on the ruling, got {}", seams.len())
    };
    seam
}

/// **A conventional form stated over a determinate tangency is a
/// defect** (the third review's witness). The shoulder `3√K·√(2ε)` deep
/// stores its seam as the intrinsic tangency; restated by hand as the
/// pair, or as a derived image in the plane's chart
/// (`Body::set_edge_curve`, which certifies incidence alone), tier 3
/// reads the rule determinate and refuses `TangentNotIntrinsic`: the
/// check keys on the description's form.
#[test]
fn a_conventional_form_stated_over_a_determinate_tangency_is_a_defect() {
    let depth = 3.0 * tol().k().sqrt() * (2.0 * tol().eps()).sqrt();
    let wall = topo::split(&shoulder(depth), &shoulder_cut(), tol())
        .expect("the shoulder cuts at its seam")
        .above
        .body()
        .expect("material on the wall's side")
        .clone();
    assert_eq!(shoulder_seam(&wall).0, "TangentIntersection");
    let seam = shoulder_seam_edge(&wall);
    let curve = wall
        .get_curve_geom(wall.get_edge(seam).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap()
        .clone();
    let pair = curve
        .description()
        .pair()
        .expect("the tangency names its pair");
    let plane = pair
        .keys()
        .into_iter()
        .find(|&k| matches!(wall.get_surface(k), Some(geom::Surface::Plane { .. })))
        .expect("the cut's plane is one side of the seam");
    assert!(!curve.authority().is_declared(), "the seam is derived");
    for (form, spec) in [
        ("OnPair", curve.restated_on_pair(pair)),
        ("Chart", curve.restated_in_chart(plane)),
    ] {
        let mut restated = topo::Body::clone(&wall);
        restated
            .set_edge_curve(seam, spec, tol())
            .unwrap_or_else(|e| panic!("{form}: the restatement certifies by incidence: {e:?}"));
        assert_eq!(
            topo::validate_geometric(&restated, tol()),
            Err(vec![topo::ValidationError::TangentNotIntrinsic {
                edge: seam
            }]),
            "{form}: the hand-stated conventional form is a defect"
        );
    }
}

/// **The transverse arm, alike, and keyed on the form** (ruling 1 of the
/// third review; D2): a brick's edge, two planes at a right angle,
/// restated by hand in a conventional form refuses
/// `TransverseNotIntrinsic` whoever stated it — as their pair, as a
/// derived image in one plane's chart, and as the pair carrying a
/// sketch declaration of the edge.
#[test]
fn a_conventional_form_stated_over_a_corner_is_a_defect() {
    let cube = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol());
    let point = |v| *cube.get_point(cube.get_vertex(v).unwrap().point).unwrap();
    // The top edge on `y = 0`, which a segment of the sketch at `z = 1`
    // declares.
    let (edge, curve) = cube
        .edges()
        .find_map(|(k, e)| {
            let (p0, p1) = (
                point(cube.get_half_edge(e.he_plus)?.start),
                point(cube.half_edge_end(e.he_plus)?),
            );
            let top = (p0.z, p1.z, p0.y, p1.y) == (1.0, 1.0, 0.0, 0.0);
            let c = cube.get_curve_geom(e.curve)?.certified()?;
            (top && c.description().pair().is_some()).then(|| (k, c.clone()))
        })
        .expect("the brick's top edge on y = 0, between two planes");
    assert!(
        !curve.authority().is_declared(),
        "the brick's edge is derived"
    );
    let pair = curve.description().pair().unwrap();
    let mut declared_pair = curve.restated_on_pair(pair);
    let (p0, p1) = (
        declared_pair.carrier.eval(declared_pair.param_start),
        declared_pair.carrier.eval(declared_pair.param_end),
    );
    declared_pair.description = geom_brep::EdgeDescriptionSpec::OnPair {
        pair,
        declared: Some(geom_brep::MappedCurve::whole(
            geom_brep::MappedSource::PlacedSegment {
                segment: geom_brep::SketchSegment::Line {
                    a: Point2::new(p0.x, p0.y),
                    b: Point2::new(p1.x, p1.y),
                },
                place: geom_core::Affine3::translation(Vec3::new(0.0, 0.0, 1.0)),
            },
        )),
    };
    for (form, spec) in [
        ("OnPair", curve.restated_on_pair(pair)),
        ("Chart", curve.restated_in_chart(pair.keys()[0])),
        ("declared OnPair", declared_pair),
    ] {
        let mut restated = cube.clone();
        restated
            .set_edge_curve(edge, spec, tol())
            .unwrap_or_else(|e| panic!("{form}: the restatement certifies: {e:?}"));
        assert_eq!(
            topo::validate_geometric(&restated, tol()),
            Err(vec![topo::ValidationError::TransverseNotIntrinsic { edge }]),
            "{form}: the hand-stated conventional form is a defect"
        );
    }
}

/// **A wrap edge is outside the rule.** A one-segment circle extrudes
/// to one cylinder wall, whose strut is a wrap image in the wall's
/// chart: its two halves bound the one face, so check 4 makes no demand
/// of it, and the drum is clean.
#[test]
fn a_wrap_seam_is_left_alone() {
    let one_segment = profile::RawLoop::new([(
        Point2::new(1.0, 0.0),
        profile::Segment::Arc(geom_core::Arc2 {
            centre: Point2::new(0.0, 0.0),
            radius: 1.0,
            sweep: std::f64::consts::TAU,
        }),
    )]);
    let plate = extruded(vec![one_segment], 0.0, 1.0).body;
    let wraps: Vec<EdgeKey> = plate
        .edges()
        .filter(|(_, e)| {
            plate
                .get_curve_geom(e.curve)
                .and_then(topo::CurveGeom::certified)
                .is_some_and(
                    |c| matches!(c.description(), geom_brep::EdgeDescription::Chart(ch) if ch.wrap),
                )
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(wraps.len(), 1, "the drum's wall has one wrap strut");
    assert_eq!(topo::validate_geometric(&plate, tol()), Ok(()));
}
