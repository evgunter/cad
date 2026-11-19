//! **A relation the file's band reads `Zero` is made exact** (`step_import`'s
//! `snap`, D7 step 3): a native body is written, one of its surfaces is
//! moved inside the file's declared ε_in, and the import makes the
//! relation the file states exact again by the catalogue's construction,
//! reporting the move; a move the file band does not read `Zero`, and a
//! class the catalogue does not solve, refuse.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol, Vec2};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop, SketchPlane};
use step_import::{
    ImportOptions, InterpretationRefusal, NormalizationKind, SnapRelation, StepImport,
    StepImportError,
};
use sweep::{ExtrudeSide, Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::AtRestBody;

fn tol() -> Tol {
    Tol::witness()
}

/// The ε_in every case is read at: 10 µm, a band of (10 µm, 100 µm).
const EPS_IN: f64 = 1e-5;

fn write(body: &AtRestBody<f64>) -> String {
    step_export::step_string(body, &step_export::StepOptions::default(), tol())
        .expect("a finished body exports")
}

fn import(text: &str) -> Result<StepImport, StepImportError> {
    let options = ImportOptions {
        eps_in: Some(EPS_IN),
        ..ImportOptions::default()
    };
    step_import::import_step(text, &options, tol())
}

/// The snaps an import reports: (surface, relation, move).
fn snaps(imported: &StepImport) -> Vec<(u64, SnapRelation, f64)> {
    imported
        .normalizations()
        .iter()
        .filter_map(|n| match n.kind {
            NormalizationKind::Snapped {
                surface,
                relation,
                moved_by,
            } => Some((surface, relation, moved_by)),
            _ => None,
        })
        .collect()
}

/// Metres per unit of the written file's lengths.
fn unit(text: &str) -> f64 {
    assert!(
        text.contains("SI_UNIT($, .METRE.)"),
        "the writer states metres"
    );
    1.0
}

/// The ids of the records of `keyword` in `text`, in file order (the
/// writer states one per face).
fn records(text: &str, keyword: &str) -> Vec<u64> {
    let hits: Vec<u64> = text
        .lines()
        .filter(|l| l.contains(&format!("= {keyword}(")))
        .map(|l| {
            l.trim_start_matches('#')
                .split(' ')
                .next()
                .unwrap()
                .parse()
                .unwrap()
        })
        .collect();
    assert!(!hits.is_empty(), "a {keyword} record");
    hits
}

/// `text` with the last numeric argument of record `#id` moved by
/// `by_m` metres.
fn nudge_last(text: &str, id: u64, by_m: f64) -> String {
    let head = format!("#{id} = ");
    let line = text.lines().find(|l| l.starts_with(&head)).unwrap();
    let close = line.rfind(')').unwrap();
    let comma = line[..close].rfind(',').unwrap();
    let value: f64 = line[comma + 1..close].trim().parse().unwrap();
    let moved = format!(
        "{}{:?}{}",
        &line[..=comma],
        value + by_m / unit(text),
        &line[close..]
    );
    text.replacen(line, &moved, 1)
}

/// `text` with coordinate `k` (0 for `x`) of the `CARTESIAN_POINT` `#id`
/// moved by `by_m` metres.
fn nudge_point(text: &str, id: u64, k: usize, by_m: f64) -> String {
    let head = format!("#{id} = CARTESIAN_POINT('', (");
    let line = text.lines().find(|l| l.starts_with(&head)).unwrap();
    let close = line.rfind("))").unwrap();
    let mut xyz: Vec<f64> = line[head.len()..close]
        .split(',')
        .map(|c| c.trim().parse().unwrap())
        .collect();
    xyz[k] += by_m / unit(text);
    let moved = format!(
        "{head}{:?}, {:?}, {:?}{}",
        xyz[0],
        xyz[1],
        xyz[2],
        &line[close..]
    );
    text.replacen(line, &moved, 1)
}

/// The point record of the placement `#placement`.
fn placement_point(text: &str, placement: u64) -> u64 {
    let head = format!("#{placement} = AXIS2_PLACEMENT_3D('', #");
    let line = text.lines().find(|l| l.starts_with(&head)).unwrap();
    line[head.len()..]
        .split(',')
        .next()
        .unwrap()
        .parse()
        .unwrap()
}

/// The placement record a surface record `#surface` is placed by.
fn surface_placement(text: &str, surface: u64) -> u64 {
    let head = format!("#{surface} = ");
    let line = text.lines().find(|l| l.starts_with(&head)).unwrap();
    let at = line.find("'', #").unwrap() + 5;
    line[at..]
        .split(|c: char| !c.is_ascii_digit())
        .next()
        .unwrap()
        .parse()
        .unwrap()
}

fn prism(loops: Vec<ProfileLoop<f64>>, h: f64) -> AtRestBody<f64> {
    let profile = Profile::new(SketchPlane::xy(), loops)
        .validate(tol())
        .expect("the profile validates");
    let extrusion = Extrusion::Distance {
        depth: h,
        side: ExtrudeSide::Along,
    };
    extrude(&profile, extrusion, tol())
        .expect("the profile extrudes")
        .body
}

/// A unit square prism with one corner rounded at radius 1 (2 × 2 × 1):
/// its one cylinder tangent to two perpendicular planes.
fn filleted() -> String {
    let quarter = (std::f64::consts::PI / 8.0).tan();
    write(&prism(
        vec![bulge_loop(vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.0),
            (Point2::new(2.0, 1.0), quarter),
            (Point2::new(1.0, 2.0), 0.0),
            (Point2::new(0.0, 2.0), 0.0),
        ])],
        1.0,
    ))
}

/// A fillet whose radius the file states 2 µm long reads tangent to both
/// its planes at the file's band and apart at the run's; the snap keeps
/// the planes and moves the axis square to itself, √2 · 2 µm, to where it
/// stands its radius from each. Reported once, on the cylinder.
#[test]
fn a_fillet_off_its_planes_moves_its_axis_to_both() {
    let text = filleted();
    // The file as written moves nothing.
    assert!(snaps_of(&text).is_empty(), "an exact fillet is not moved");
    let [cylinder] = records(&text, "CYLINDRICAL_SURFACE")[..] else {
        panic!("one cylinder record")
    };
    let moved = nudge_last(&text, cylinder, 2e-6);
    let imported = import(&moved).unwrap_or_else(|e| panic!("imports: {}", e.render()));
    let snaps = snaps(&imported);
    assert_eq!(snaps.len(), 1, "one move: {snaps:?}");
    let (surface, relation, moved_by) = snaps[0];
    assert_eq!((surface, relation), (cylinder, SnapRelation::Tangent));
    assert!(
        (moved_by - 2e-6 * std::f64::consts::SQRT_2).abs() < 1e-9,
        "the axis moves √2 · 2 µm: {moved_by:e}"
    );
    let StepImport::Solid { body, .. } = &imported else {
        panic!("a solid")
    };
    assert_eq!(
        topo::validate_geometric(body, tol()),
        Ok(()),
        "tier-3 valid"
    );
    // The file as written moves nothing.
    assert!(snaps_of(&text).is_empty(), "an exact fillet is not moved");
}

fn snaps_of(text: &str) -> Vec<(u64, SnapRelation, f64)> {
    snaps(&import(text).unwrap_or_else(|e| panic!("imports: {}", e.render())))
}

/// A fillet 8 µm long reads tangent at the file's band (its gap is inside
/// the band's 10 µm zero), but the construction moves its axis √2 · 8 µm,
/// which the file's band does not read `Zero`: the file's ambiguity, and
/// nothing is moved.
#[test]
fn a_move_the_file_band_does_not_read_zero_refuses() {
    let text = filleted();
    let [cylinder] = records(&text, "CYLINDRICAL_SURFACE")[..] else {
        panic!("one cylinder record")
    };
    match import(&nudge_last(&text, cylinder, 8e-6)) {
        Err(StepImportError::Interpretation {
            id,
            refusal: InterpretationRefusal::Ambiguous { reading, gap, .. },
            ..
        }) => {
            assert_eq!((id, reading), (cylinder, "step_snap_move"));
            assert!(
                (gap - 8e-6 * std::f64::consts::SQRT_2).abs() < 1e-9,
                "the move read is the axis's: {gap:e}"
            );
        }
        other => panic!("the move refuses as the file's ambiguity: {other:?}"),
    }
}

/// A cylinder capped by a hemisphere on a flat base, revolved about `z`:
/// the sphere coaxial with the cylinder along their tangent circle.
fn capped() -> String {
    let quarter = (std::f64::consts::PI / 8.0).tan();
    let outline = bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), 0.0),
        (Point2::new(1.0, 1.0), quarter),
        (Point2::new(0.0, 2.0), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![outline])
        .validate(tol())
        .expect("the profile validates");
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    write(
        &revolve(&profile, axis, Revolution::Full, tol())
            .expect("the profile revolves")
            .body,
    )
}

/// The cap's centre stated 3 µm off the cylinder's axis reads coaxial at
/// the file's band and apart at the run's: the later of the two records
/// moves onto the earlier one's axis, 3 µm, reported.
#[test]
fn a_cap_off_its_cylinders_axis_is_made_coaxial() {
    let text = capped();
    assert!(snaps_of(&text).is_empty(), "the native capsule is exact");
    // The writer states each face's surface as its own record: every
    // record of the sphere moves, and the import makes them one again.
    let spheres = records(&text, "SPHERICAL_SURFACE");
    let moved = spheres.iter().fold(text.clone(), |t, &s| {
        let point = placement_point(&t, surface_placement(&t, s));
        nudge_point(&t, point, 0, 3e-6)
    });
    let (sphere, cylinder) = (spheres[0], records(&text, "CYLINDRICAL_SURFACE")[0]);
    let imported = import(&moved).unwrap_or_else(|e| panic!("imports: {}", e.render()));
    let snaps = snaps(&imported);
    assert_eq!(snaps.len(), 1, "one move: {snaps:?}");
    let (surface, relation, moved_by) = snaps[0];
    assert_eq!(relation, SnapRelation::Coaxial);
    assert_eq!(surface, sphere.max(cylinder), "the later record moves");
    assert!((moved_by - 3e-6).abs() < 1e-9, "a 3 µm move: {moved_by:e}");
    let StepImport::Solid { body, .. } = &imported else {
        panic!("a solid")
    };
    assert_eq!(
        topo::validate_geometric(body, tol()),
        Ok(()),
        "tier-3 valid"
    );
}

/// The axis direction record of the placement `#placement`.
fn placement_axis(text: &str, placement: u64) -> u64 {
    let head = format!("#{placement} = AXIS2_PLACEMENT_3D('', #");
    let line = text.lines().find(|l| l.starts_with(&head)).unwrap();
    line[head.len()..]
        .split(", #")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap()
}

/// `text` with ratio `k` (0 for `x`) of the `DIRECTION` `#id` moved by
/// `by`.
fn nudge_direction(text: &str, id: u64, k: usize, by: f64) -> String {
    let head = format!("#{id} = DIRECTION('', (");
    let line = text.lines().find(|l| l.starts_with(&head)).unwrap();
    let close = line.rfind("))").unwrap();
    let mut xyz: Vec<f64> = line[head.len()..close]
        .split(',')
        .map(|c| c.trim().parse().unwrap())
        .collect();
    xyz[k] += by;
    let moved = format!(
        "{head}{:?}, {:?}, {:?}{}",
        xyz[0],
        xyz[1],
        xyz[2],
        &line[close..]
    );
    text.replacen(line, &moved, 1)
}

/// The capsule's base plane stated 2 µrad off square to the cylinder's
/// axis reads axis-normal at the file's band and apart at the run's: the
/// plane, the later record, turns onto the axis about where the axis
/// meets it, reported.
#[test]
fn a_base_off_square_to_its_cylinder_is_made_axis_normal() {
    let text = capped();
    let [plane] = records(&text, "PLANE")[..] else {
        panic!("one plane record")
    };
    assert!(
        records(&text, "CYLINDRICAL_SURFACE")[0] < plane,
        "the plane is the later record (the cylinder's are one, the earliest kept)"
    );
    let axis = placement_axis(&text, surface_placement(&text, plane));
    let imported = import(&nudge_direction(&text, axis, 0, 2e-6))
        .unwrap_or_else(|e| panic!("imports: {}", e.render()));
    let snaps = snaps(&imported);
    assert_eq!(snaps.len(), 1, "one move: {snaps:?}");
    let (surface, relation, moved_by) = snaps[0];
    assert_eq!((surface, relation), (plane, SnapRelation::AxisNormal));
    assert!(
        moved_by > 1e-6 && moved_by < EPS_IN,
        "a turn of 2 µrad levered at the faces' reach: {moved_by:e}"
    );
    let StepImport::Solid { body, .. } = &imported else {
        panic!("a solid")
    };
    assert_eq!(
        topo::validate_geometric(body, tol()),
        Ok(()),
        "tier-3 valid"
    );
}

/// `step-export`'s filleted die: each edge fillet a cylinder tangent to
/// two faces of the cube and coaxial with the corner spheres it meets.
fn filleted_die() -> String {
    std::fs::read_to_string(format!(
        "{}/../step-export/tests/fixtures/filleted_die.step",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap()
}

/// A fillet 2 µm long reads tangent to its two faces at the file's band
/// and apart at the run's, and its construction would move its axis; the
/// corner spheres it is coaxial with would then be off it. A class the
/// catalogue does not solve refuses not-yet-supported, naming its lane,
/// and moves nothing.
#[test]
fn a_fillet_with_a_coaxial_partner_refuses_as_unsolved() {
    let text = filleted_die();
    assert!(snaps_of(&text).is_empty(), "the native die is exact");
    let cylinder = records(&text, "CYLINDRICAL_SURFACE")[0];
    match import(&nudge_last(&text, cylinder, 2e-6)) {
        Err(e @ StepImportError::NotYetSupported { .. }) => {
            let StepImportError::NotYetSupported { what, lane, .. } = &e else {
                unreachable!()
            };
            assert_eq!(
                what,
                "a cylinder whose tangency moves it while it is coaxial with another surface"
            );
            assert!(
                lane.contains("a-relation-the-snap-catalogue-does-not-make-exact-refuses"),
                "the lane is the filed row: {lane}"
            );
        }
        other => panic!("an unsolved class refuses: {other:?}"),
    }
}

/// A stadium prism: two half-cylinders of radius 1 between the planes
/// `y = 0` and `y = 2`, a slot's two ends.
fn stadium() -> String {
    write(&prism(
        vec![bulge_loop(vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 1.0),
            (Point2::new(2.0, 2.0), 0.0),
            (Point2::new(0.0, 2.0), 1.0),
        ])],
        1.0,
    ))
}

/// A slot end stated 2 µm wide in its radius and 1 µm off its mid-plane
/// reads tangent to both its parallel planes at the file's band and apart
/// at the run's: the planes stay, the radius becomes half their spacing
/// and the axis moves to the mid-plane, 3 µm in all, reported.
#[test]
fn a_slot_end_between_parallel_planes_takes_half_their_spacing() {
    let text = stadium();
    assert!(snaps_of(&text).is_empty(), "the native stadium is exact");
    let cylinder = records(&text, "CYLINDRICAL_SURFACE")[0];
    let point = placement_point(&text, surface_placement(&text, cylinder));
    let moved = nudge_point(&nudge_last(&text, cylinder, 2e-6), point, 1, 1e-6);
    let imported = import(&moved).unwrap_or_else(|e| panic!("imports: {}", e.render()));
    let snaps = snaps(&imported);
    assert_eq!(snaps.len(), 1, "one move: {snaps:?}");
    let (surface, relation, moved_by) = snaps[0];
    assert_eq!((surface, relation), (cylinder, SnapRelation::Tangent));
    assert!(
        (moved_by - 3e-6).abs() < 1e-9,
        "radius 2 µm and axis 1 µm: {moved_by:e}"
    );
    let StepImport::Solid { body, .. } = &imported else {
        panic!("a solid")
    };
    assert_eq!(
        topo::validate_geometric(body, tol()),
        Ok(()),
        "tier-3 valid"
    );
    let radii: Vec<f64> = body
        .faces()
        .filter_map(|(_, f)| match body.get_surface(f.surface) {
            Some(geom::Surface::Cylinder { radius, .. }) => Some(*radius),
            _ => None,
        })
        .collect();
    assert!(
        radii.iter().all(|r| (r - 1.0).abs() < 1e-12),
        "every slot end is half the spacing: {radii:?}"
    );
}

/// A fillet whose side plane `x = 2` the file tilts 2 µrad toward the
/// fillet's axis reads tangent at the file's band, the tilt levered at
/// the faces' reach inside it, and apart at the run's. The tangent
/// construction translates the axis and keeps the plane, so no
/// translation makes that tangency exact: the class re-read at the run's
/// band is not, and refuses as unsolved, moving nothing.
#[test]
fn a_tangency_its_construction_leaves_apart_refuses_as_unsolved() {
    let text = filleted();
    let side = records(&text, "PLANE")
        .into_iter()
        .find(|&p| {
            let axis = placement_axis(&text, surface_placement(&text, p));
            text.contains(&format!("#{axis} = DIRECTION('', (1.0, 0.0, 0.0));"))
        })
        .expect("the side plane x = 2");
    let axis = placement_axis(&text, surface_placement(&text, side));
    match import(&nudge_direction(&text, axis, 2, 2e-6)) {
        Err(StepImportError::NotYetSupported { what, lane, .. }) => {
            assert!(
                what.contains("tangent relation its class's constructions leave"),
                "the tangency left apart is named: {what}"
            );
            assert!(
                lane.contains("a-relation-the-snap-catalogue-does-not-make-exact-refuses"),
                "the lane is the filed row: {lane}"
            );
        }
        other => panic!("an unsolved class refuses: {other:?}"),
    }
}

/// A coin: a disc of radius 50 mm and thickness `h`, revolved about `y`.
fn coin(h: f64) -> String {
    let outline = bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(0.05, 0.0), 0.0),
        (Point2::new(0.05, h), 0.0),
        (Point2::new(0.0, h), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![outline])
        .validate(tol())
        .expect("the profile validates");
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    write(
        &revolve(&profile, axis, Revolution::Full, tol())
            .expect("the profile revolves")
            .body,
    )
}

/// **A coin's cap tilted, read over the cap's reach.** A disc 50 mm in
/// radius and 0.5 or 1 mm thick: its rim edges are whole circles with one
/// vertex each, so the faces' vertices span no more than the thickness,
/// while the cap's rim moves by the tilt times the radius. The later cap
/// tilted 1e-4 moves its rim 5 µm, inside the band's zero: it turns onto
/// the axis, its move the rim's. Tilted 4e-4 (20 µm, in the band) the
/// relation is not read and the build refuses the rim as the file's
/// ambiguity; tilted 4e-3 (200 µm) the rim lies past the band.
#[test]
fn a_coins_tilted_cap_is_read_over_its_rim() {
    let radius = 0.05;
    for h in [5e-4, 1e-3] {
        let text = coin(h);
        let cap = *records(&text, "PLANE").last().expect("two caps");
        assert!(
            records(&text, "CYLINDRICAL_SURFACE")[0] < cap,
            "the tilted cap is the later record"
        );
        let axis = placement_axis(&text, surface_placement(&text, cap));
        let tilted = |by: f64| import(&nudge_direction(&text, axis, 0, by));
        let snapped = tilted(1e-4).unwrap_or_else(|e| panic!("h {h}: imports: {}", e.render()));
        let moves = snaps(&snapped);
        assert_eq!(moves.len(), 1, "h {h}: one move: {moves:?}");
        let (surface, relation, moved_by) = moves[0];
        assert_eq!(
            (surface, relation),
            (cap, SnapRelation::AxisNormal),
            "h {h}"
        );
        let rim = 1e-4 * radius;
        assert!(
            moved_by >= rim * (1.0 - 1e-6) && moved_by <= rim * 1.01,
            "h {h}: the move is the rim's, {rim:e}: {moved_by:e}"
        );
        for (by, in_band) in [(4e-4, true), (4e-3, false)] {
            match tilted(by) {
                Err(StepImportError::Interpretation { refusal, .. }) => match (refusal, in_band) {
                    (
                        InterpretationRefusal::Ambiguous {
                            reading, gap: miss, ..
                        },
                        true,
                    )
                    | (InterpretationRefusal::EvidenceMiss { reading, miss }, false) => {
                        assert_eq!(reading, "step_edge_on_description", "h {h} tilt {by}");
                        assert!(
                            miss >= by * radius * (1.0 - 1e-2) && miss <= 2.0 * by * radius * 1.01,
                            "h {h} tilt {by}: the rim's miss, one to two of its move: {miss:e}"
                        );
                    }
                    (other, _) => panic!("h {h} tilt {by}: the rim's reading, got {other:?}"),
                },
                other => panic!("h {h} tilt {by}: refuses as the file's, got {other:?}"),
            }
        }
    }
}

fn revolved(chain: Vec<(Point2<f64>, f64)>) -> String {
    let profile = Profile::new(SketchPlane::xy(), vec![bulge_loop(chain)])
        .validate(tol())
        .expect("the profile validates");
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    write(
        &revolve(&profile, axis, Revolution::Full, tol())
            .expect("the profile revolves")
            .body,
    )
}

/// A cylinder under a cone to its apex on a flat base, revolved about
/// `y`: the cone the earliest record, then the cylinder, then the base.
fn cone_on_cylinder() -> String {
    revolved(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), 0.0),
        (Point2::new(1.0, 1.0), 0.0),
        (Point2::new(0.0, 2.0), 0.0),
    ])
}

/// **Every relation of a class is re-read after its moves, the exact ones
/// too.** The cone's axis stated 2 µrad off the cylinder's reads coaxial
/// at the file's band and apart at the run's, and the cylinder, the later
/// record, turns onto it. Its base, exactly square to it before, is then
/// 2 µrad off: the class's exact axis-normal relation re-reads not exact,
/// and the class refuses as unsolved.
#[test]
fn a_move_that_breaks_an_exact_relation_refuses() {
    let text = cone_on_cylinder();
    assert!(snaps_of(&text).is_empty(), "the native body is exact");
    let cones = records(&text, "CONICAL_SURFACE");
    assert!(
        cones[0] < records(&text, "CYLINDRICAL_SURFACE")[0],
        "the cone is earlier"
    );
    let tilted = cones.iter().fold(text.clone(), |t, &c| {
        let axis = placement_axis(&t, surface_placement(&t, c));
        nudge_direction(&t, axis, 0, 2e-6)
    });
    match import(&tilted) {
        Err(StepImportError::NotYetSupported { what, lane, .. }) => {
            assert!(
                what.starts_with("the axis_normal relation its class's constructions leave ")
                    && what.ends_with(" m from exact"),
                "the exact relation the move broke: {what}"
            );
            assert!(lane.contains("a-relation-the-snap-catalogue-does-not-make-exact-refuses"));
        }
        other => panic!("refuses as unsolved: {other:?}"),
    }
}

/// **A relation whose kept surface another construction moves refuses**,
/// whichever order would have read them. The capsule's cap stated 3 µm off
/// the cylinder's axis moves the cylinder, the later record, onto it; its
/// base tilted 2 µrad would turn onto the cylinder's axis, which that move
/// changes: the solve anchored at each relation's earlier record cannot
/// make both exact, and says so, naming its lane.
#[test]
fn a_construction_reading_a_moved_surface_refuses() {
    let text = capped();
    let mut moved = records(&text, "SPHERICAL_SURFACE")
        .iter()
        .fold(text.clone(), |t, &s| {
            let point = placement_point(&t, surface_placement(&t, s));
            nudge_point(&t, point, 0, 3e-6)
        });
    let [plane] = records(&text, "PLANE")[..] else {
        panic!("one plane record")
    };
    let axis = placement_axis(&moved, surface_placement(&moved, plane));
    moved = nudge_direction(&moved, axis, 0, 2e-6);
    match import(&moved) {
        Err(StepImportError::NotYetSupported { what, lane, .. }) => {
            assert_eq!(
                what,
                "the axis_normal relation whose kept surface another of its class's \
                 constructions moves"
            );
            assert!(lane.contains("a-snap-class-whose-solve-moves-a-surface-twice-refuses"));
        }
        other => panic!("refuses, naming the anchored solve's lane: {other:?}"),
    }
}

/// **A surface two constructions would move refuses.** A corner sphere of
/// the filleted die stated 3 µm off its corner reads coaxial with each of
/// the three fillets it meets and apart from each at the run's band; it is
/// the later record of all three, and no one construction moves it onto
/// three axes.
#[test]
fn a_surface_two_constructions_would_move_refuses() {
    let text = filleted_die();
    let sphere = records(&text, "SPHERICAL_SURFACE")[0];
    let point = placement_point(&text, surface_placement(&text, sphere));
    match import(&nudge_point(&text, point, 0, 3e-6)) {
        Err(StepImportError::NotYetSupported { what, lane, .. }) => {
            assert_eq!(
                what,
                "a surface two of its class's constructions would move"
            );
            assert!(lane.contains("a-snap-class-whose-solve-moves-a-surface-twice-refuses"));
        }
        other => panic!("refuses, naming the anchored solve's lane: {other:?}"),
    }
}

/// A prism whose bottom side turns by `turn` radians into its next side
/// through a fillet of radius 1 at `(4, 1)`.
fn turned(turn: f64) -> String {
    let (s, c) = turn.sin_cos();
    let b = Point2::new(4.0 + s, 1.0 - c);
    let end = Point2::new(b.x + 2.0 * c, b.y + 2.0 * s);
    write(&prism(
        vec![bulge_loop(vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(4.0, 0.0), (turn / 4.0).tan()),
            (b, 0.0),
            (end, 0.0),
            (Point2::new(end.x, end.y + 2.0), 0.0),
            (Point2::new(0.0, end.y + 2.0), 0.0),
        ])],
        1.0,
    ))
}

/// **A move the file band reads apart refuses, and nothing moves.** A
/// fillet in a corner of 8° (its sides turning 172°), its radius stated
/// 8 µm long: each tangency's gap reads inside the band's zero (10 µm), but
/// the two offsets meet 8 µm / sin 4° ≈ 115 µm along the bisector, past
/// the band.
#[test]
fn a_move_the_file_band_reads_apart_refuses_as_unsolved() {
    let text = turned(172f64.to_radians());
    assert!(snaps_of(&text).is_empty(), "the native fillet is exact");
    let [cylinder] = records(&text, "CYLINDRICAL_SURFACE")[..] else {
        panic!("one cylinder record")
    };
    match import(&nudge_last(&text, cylinder, 8e-6)) {
        Err(StepImportError::NotYetSupported { id, what, lane, .. }) => {
            assert!(
                what.starts_with("the tangent relation's construction moving its surface ")
                    && what.ends_with(" m, past the file's band"),
                "#{id}: {what}"
            );
            assert!(lane.contains("a-relation-the-snap-catalogue-does-not-make-exact-refuses"));
        }
        other => panic!("refuses as unsolved: {other:?}"),
    }
}

/// **The two-plane solve keeps each plane's offset to its own plane.** A
/// fillet between two sides 60° apart, the bottom side stated 3 µm lower:
/// only that tangency is off, so the axis moves along the other side, the
/// bottom's gap over the sine of the 60° between their normals.
#[test]
fn an_asymmetric_fillet_moves_its_axis_along_the_side_that_holds() {
    let text = turned(60f64.to_radians());
    assert!(snaps_of(&text).is_empty(), "the native fillet is exact");
    let bottom = records(&text, "PLANE")
        .into_iter()
        .find(|&p| {
            let axis = placement_axis(&text, surface_placement(&text, p));
            text.contains(&format!("#{axis} = DIRECTION('', (0.0, -1.0, 0.0));"))
                || text.contains(&format!("#{axis} = DIRECTION('', (-0.0, -1.0, -0.0));"))
        })
        .expect("the bottom side y = 0");
    let point = placement_point(&text, surface_placement(&text, bottom));
    let imported = import(&nudge_point(&text, point, 1, -3e-6))
        .unwrap_or_else(|e| panic!("imports: {}", e.render()));
    let moves = snaps(&imported);
    assert_eq!(moves.len(), 1, "one move: {moves:?}");
    let (_, relation, moved_by) = moves[0];
    assert_eq!(relation, SnapRelation::Tangent);
    let want = 3e-6 / 60f64.to_radians().sin();
    assert!(
        (moved_by - want).abs() < 1e-3 * want,
        "the axis moves g / sin 60° = {want:e}: {moved_by:e}"
    );
    let StepImport::Solid { body, .. } = &imported else {
        panic!("a solid")
    };
    assert_eq!(
        topo::validate_geometric(body, tol()),
        Ok(()),
        "tier-3 valid"
    );
}
