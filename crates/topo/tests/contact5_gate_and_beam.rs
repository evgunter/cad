//! **The instance arm decides a meeting pair by the probe and the
//! touch analysis.** The box gate answers containment, shell by shell:
//! a pair is cleared there only when nothing on record says the two
//! boundaries meet and no SHELL of either sits inside the other's
//! reach. A pair that meets — a finding with one entity on each side,
//! or a declared record naming one of each — has every vertex probed
//! against the other's material both ways, and clears only when the
//! findings about it are rests; a crossing of two edges at one point
//! is such a touch, read through the two edges' dihedral wedges.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use geom_core::{Band, Point3, Tol};
use topo::{
    Body, CensusContact, ContactRecords, EntityId, PatchContact, SolidContainment, SolidKey,
    ValidationError, VfContact, VoidContainment, VoidEvidence, insert_void,
    validate_pseudomanifold,
};

fn block(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    common::brick::<f64>(x, y, z, Tol::witness())
}

/// The parallelepiped `p + u·a + v·b + w·c` over the unit cube
/// (`det[a, b, c] > 0`, so an ordinary solid).
fn parallelepiped(p: [f64; 3], a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> Body<f64> {
    common::mapped_cube(
        move |u, v, w| {
            Point3::new(
                p[0] + u * a[0] + v * b[0] + w * c[0],
                p[1] + u * a[1] + v * b[1] + w * c[1],
                p[2] + u * a[2] + v * b[2] + w * c[2],
            )
        },
        Tol::witness(),
    )
}

fn assembly(parts: &[Body<f64>]) -> Body<f64> {
    let mut out = parts[0].clone();
    for part in &parts[1..] {
        topo::graft_disjoint(&mut out, part).unwrap();
    }
    out
}

fn errors_of(body: &Body<f64>) -> Vec<ValidationError> {
    validate_pseudomanifold(body, &ContactRecords::default(), Tol::witness())
        .err()
        .unwrap_or_default()
}

/// The findings arm 2 raises on a solid pair.
fn placement_findings(errors: &[ValidationError]) -> Vec<&ValidationError> {
    errors
        .iter()
        .filter(|e| {
            matches!(
                e,
                ValidationError::CensusUndecidable {
                    a: EntityId::Solid(_),
                    b: EntityId::Solid(_),
                    ..
                } | ValidationError::InstanceInterference { .. }
            )
        })
        .collect()
}

fn crosses(errors: &[ValidationError]) -> usize {
    errors
        .iter()
        .filter(|e| {
            matches!(
                e,
                ValidationError::UndeclaredContact {
                    contact: CensusContact::EdgeEdgeCross { .. },
                    ..
                }
            )
        })
        .count()
}

fn pierces(errors: &[ValidationError]) -> usize {
    errors
        .iter()
        .filter(|e| {
            matches!(
                e,
                ValidationError::UndeclaredContact {
                    contact: CensusContact::EdgeFacePierce { .. },
                    ..
                }
            )
        })
        .count()
}

fn escalations(errors: &[ValidationError]) -> usize {
    errors
        .iter()
        .filter(|e| matches!(e, ValidationError::CensusEscalated { .. }))
        .count()
}

/// Every placement refusal's `what`, cut to its reason: the text before
/// the recourse.
fn refusals(errors: &[ValidationError]) -> Vec<&'static str> {
    placement_findings(errors)
        .into_iter()
        .map(|e| match e {
            ValidationError::CensusUndecidable { what, .. } => {
                what.split(", and this one").next().unwrap_or(what)
            }
            other => panic!("a typed refusal, not a decided interference: {other:?}"),
        })
        .collect()
}

const CROSSING: &str = "another finding reports their boundaries crossing";
const UNEXAMINED: &str = "another finding left their boundaries unchecked";

/// **Two cubes side by side, face to face.** `[0, 2]³` and `[2, 4] ×
/// [0, 2]²` share the face `x = 2` with opposite normals, every edge of
/// it collinear with the other's and every corner coincident. The gate
/// separates both orderings and the boundaries meet, so the touch
/// analysis reads every finding: each is a rest across the plane
/// `x = 2`, and the pair clears.
#[test]
fn two_cubes_side_by_side_clear() {
    let body = assembly(&[
        block((0.0, 2.0), (0.0, 2.0), (0.0, 2.0)),
        block((2.0, 4.0), (0.0, 2.0), (0.0, 2.0)),
    ]);
    let errors = errors_of(&body);
    assert!(!errors.is_empty(), "the undeclared touches stand");
    assert!(placement_findings(&errors).is_empty(), "{errors:?}");
}

/// The pose of the beam across two supports: two blocks `[0, 0.1] ×
/// [0, 0.5] × [0, 1]` and `[3, 3.1] × [0, 0.5] × [0, 1]`, and a beam
/// over `x ∈ [−0.2, 3.3]`, `y ∈ [0.1, 0.15]`, lifted by `lift` from
/// resting on their tops at `z = 1`.
fn beam_across_two_supports(lift: f64) -> Body<f64> {
    let beam = parallelepiped(
        [-0.2, 0.1, 1.0 + lift],
        [3.5, 0.0, 0.0],
        [0.0, 0.05, 0.0],
        [0.0, 0.0, 0.05],
    );
    assembly(&[
        block((0.0, 0.1), (0.0, 0.5), (0.0, 1.0)),
        block((3.0, 3.1), (0.0, 0.5), (0.0, 1.0)),
        beam,
    ])
}

/// **A beam resting across two supports clears.** The beam's two
/// bottom edges cross each support's two top edges in the plane `z = 1`, so the
/// sweeps report edge crosses; each is read at the crossing point
/// through the two edges' wedges, which the plane `z = 1` separates.
/// Together with the edges lying in the other's faces, every finding
/// is a rest.
#[test]
fn a_beam_across_two_supports_clears() {
    let errors = errors_of(&beam_across_two_supports(0.0));
    assert_eq!(crosses(&errors), 8, "{errors:?}");
    assert_eq!(pierces(&errors), 0, "{errors:?}");
    assert!(placement_findings(&errors).is_empty(), "{errors:?}");
}

/// **The same beam sunk a millimetre into its supports refuses.** Its
/// bottom edges now pierce the supports' sides and the supports' top
/// edges pierce its sides, so the boundaries meet in crossings and
/// both beam × support pairs refuse on them.
#[test]
fn a_beam_sunk_into_its_supports_refuses() {
    let errors = errors_of(&beam_across_two_supports(-0.001));
    assert!(pierces(&errors) > 0, "{errors:?}");
    assert_eq!(refusals(&errors), [CROSSING, CROSSING], "{errors:?}");
}

/// **A beam tilted in band about one bottom edge.** The beam pivots on
/// its bottom edge `y = 0.1` so its other bottom edge rises by a height
/// inside the run band: whether that edge's line meets the supports'
/// top edges is too close to call, so the sweep escalates there. The
/// pivot edge's crosses are meetings, so each beam × support pair is
/// probed; the raised corners are in band of the supports' tops and the
/// probe cannot place them, so the findings are read in its place, and
/// they name the escalation: each pair refuses as left unchecked.
#[test]
fn a_beam_tilted_in_band_escalates() {
    let band = Band::linear(Tol::witness()).unwrap();
    let rise = (band.zero() * band.escalate()).sqrt();
    let beam = parallelepiped(
        [-0.2, 0.1, 1.0],
        [3.5, 0.0, 0.0],
        [0.0, 0.05, rise],
        [0.0, 0.0, 0.05],
    );
    let body = assembly(&[
        block((0.0, 0.1), (0.0, 0.5), (0.0, 1.0)),
        block((3.0, 3.1), (0.0, 0.5), (0.0, 1.0)),
        beam,
    ]);
    let errors = errors_of(&body);
    assert!(escalations(&errors) > 0, "{errors:?}");
    assert!(
        crosses(&errors) > 0,
        "the pivot edge still crosses: {errors:?}"
    );
    assert_eq!(refusals(&errors), [UNEXAMINED, UNEXAMINED], "{errors:?}");
}

/// Two square bars turned 45° about their axes and crossed ridge on
/// ridge: the lower bar's top edge runs along `x` at `y = 0, z = 2`, the
/// upper bar's bottom edge along `y` at `x = 0, z = 2 + lift`.
fn crossed_ridges(lift: f64) -> Body<f64> {
    assembly(&[
        parallelepiped(
            [-1.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [0.0, 1.0, 1.0],
            [0.0, -1.0, 1.0],
        ),
        parallelepiped(
            [0.0, -1.0, 2.0 + lift],
            [0.0, 2.0, 0.0],
            [-1.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
        ),
    ])
}

/// **Two ridges crossed edge on edge clear; sunk, they refuse.** No
/// face of either bar lies in the plane the two ridges span, and the
/// crossing is still a touch: the plane `z = 2` through both ridges
/// separates the two wedges. Lowered by a centimetre, the upper ridge
/// passes through the lower bar's slopes and the pair refuses.
#[test]
fn crossed_ridges_clear_resting_and_refuse_sunk() {
    let errors = errors_of(&crossed_ridges(0.0));
    assert_eq!(crosses(&errors), 1, "{errors:?}");
    assert!(placement_findings(&errors).is_empty(), "{errors:?}");
    let errors = errors_of(&crossed_ridges(-0.01));
    assert!(pierces(&errors) > 0, "{errors:?}");
    assert_eq!(refusals(&errors), [CROSSING], "{errors:?}");
}

/// Whether two closed intervals overlap in a positive length.
fn overlaps(a: (f64, f64), b: (f64, f64)) -> bool {
    a.1.min(b.1) - a.0.max(b.0) > 0.0
}

/// **A sweep of axis-aligned brick pairs on a unit grid.** The cube
/// `[0, 2]³` against every brick whose sides run between the grid
/// values `−1, 0, 1, 2, 3` on each axis — a thousand poses, every one
/// with the two boundaries meeting, and with faces coplanar and edges
/// collinear or crossing in plane wherever the grid lines coincide.
/// Two bricks' materials overlap exactly when their intervals overlap
/// in a positive length on all three axes, which is the ground truth.
/// No pose that overlaps may clear. A pose that does not overlap is a
/// rest, and every one of those clears.
#[test]
fn a_grid_of_brick_pairs_clears_exactly_the_rests() {
    let grid = [-1.0, 0.0, 1.0, 2.0, 3.0];
    let spans: Vec<(f64, f64)> = grid
        .iter()
        .enumerate()
        .flat_map(|(i, &lo)| grid[i + 1..].iter().map(move |&hi| (lo, hi)))
        .collect();
    let a = (0.0, 2.0);
    let (mut wrong_clears, mut false_refusals, mut overlapping, mut rests) =
        (Vec::new(), Vec::new(), 0, 0);
    for &x in &spans {
        for &y in &spans {
            for &z in &spans {
                let body = assembly(&[block(a, a, a), block(x, y, z)]);
                let errors = errors_of(&body);
                let refused = !placement_findings(&errors).is_empty();
                let overlap = overlaps(a, x) && overlaps(a, y) && overlaps(a, z);
                if overlap {
                    overlapping += 1;
                    if !refused {
                        wrong_clears.push((x, y, z));
                    }
                } else {
                    rests += 1;
                    if refused {
                        false_refusals.push(((x, y, z), refusals(&errors)));
                    }
                }
            }
        }
    }
    println!(
        "grid sweep: {overlapping} overlapping, {rests} rests; wrong clears {}, false refusals {}",
        wrong_clears.len(),
        false_refusals.len()
    );
    assert_eq!(overlapping + rests, 1000);
    assert!(wrong_clears.is_empty(), "{wrong_clears:?}");
    assert!(false_refusals.is_empty(), "{false_refusals:?}");
}

/// The one `InstanceInterference` a body carries: `(outer, inner)`.
fn the_interference(errors: &[ValidationError]) -> (SolidKey, SolidKey) {
    let placements = placement_findings(errors);
    match placements[..] {
        [ValidationError::InstanceInterference { outer, inner, .. }] => (*outer, *inner),
        _ => panic!("one decided interference: {errors:?}"),
    }
}

/// **A two-lump solid with one lump inside the other instance.** A
/// union of two disjoint cubes is ONE solid with two outer shells.
/// Lump 1, `[0, 1]³`, lies strictly inside `B = [−1, 10] × [−1, 2]²`;
/// lump 2 sits at `x ∈ [10, 11]`, face to face with B, or far off at
/// `x ∈ [20, 21]`. The solid's whole hull sticks out of B's box either
/// way, but lump 1's shell does not, so the pair is probed and lump 1's
/// corners are inside B, whether or not anything meets and whether or
/// not the meetings are declared.
#[test]
fn a_lump_inside_the_other_instance_is_found_whatever_its_sibling_does() {
    let tol = Tol::witness();
    for (lump2, meets) in [((10.0, 11.0), true), ((20.0, 21.0), false)] {
        let unit = (0.0, 1.0);
        let union =
            topo::boolean::union(&block(unit, unit, unit), &block(lump2, unit, unit), tol).unwrap();
        let a = union.body().unwrap().body.clone();
        assert_eq!((a.solids().count(), a.shells().count()), (1, 2));
        let body = assembly(&[a, block((-1.0, 10.0), (-1.0, 2.0), (-1.0, 2.0))]);
        let lumps = body.solids().next().unwrap().0;
        let errors = errors_of(&body);
        assert_eq!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::UndeclaredContact { .. })),
            meets,
            "{errors:?}"
        );
        assert_eq!(the_interference(&errors).1, lumps, "{errors:?}");
        // Declared: lump 2's corners resting on B, every touch recorded.
        let records = ContactRecords {
            b_on_a: errors
                .iter()
                .filter_map(|e| match e {
                    ValidationError::UndeclaredContact {
                        contact: CensusContact::VertexOnFace { vertex, face },
                        ..
                    } => Some(VfContact {
                        vertex: *vertex,
                        face: *face,
                    }),
                    _ => None,
                })
                .collect(),
            ..ContactRecords::default()
        };
        let errors = validate_pseudomanifold(&body, &records, tol).unwrap_err();
        assert_eq!(the_interference(&errors).1, lumps, "{errors:?}");
    }
}

/// **A declared rest with a point dipping into the wall.** A prism over
/// `(0, 0), (1, 0), (1, 2), (0, 2), (−0.1, 1)` stands against the wall
/// `x = 0` of a block `x ≤ 0`: its four corners at `x = 0` rest on the
/// wall, and its point `(−0.1, 1)` dips into the block. The four
/// corners are declared, so the pair carries no finding at all; the
/// declaration is a meeting, the pair is probed, and the dipped corner
/// is inside the block.
#[test]
fn a_declared_rest_with_a_dipping_point_is_probed() {
    let wall = block((-5.0, 0.0), (-5.0, 5.0), (-5.0, 5.0));
    let prof = [(0.0, 0.0), (1.0, 0.0), (1.0, 2.0), (0.0, 2.0), (-0.1, 1.0)];
    let part = common::prism_z::<f64>(&prof, 0.0, 1.0, Tol::witness()).body;
    let body = assembly(&[wall, part]);
    let records = ContactRecords {
        b_on_a: errors_of(&body)
            .iter()
            .filter_map(|e| match e {
                ValidationError::UndeclaredContact {
                    contact: CensusContact::VertexOnFace { vertex, face },
                    ..
                } => Some(VfContact {
                    vertex: *vertex,
                    face: *face,
                }),
                _ => None,
            })
            .collect(),
        ..ContactRecords::default()
    };
    assert_eq!(records.b_on_a.len(), 4);
    let errors = validate_pseudomanifold(&body, &records, Tol::witness()).unwrap_err();
    let solids: Vec<SolidKey> = body.solids().map(|(k, _)| k).collect();
    assert_eq!(
        the_interference(&errors),
        (solids[0], solids[1]),
        "{errors:?}"
    );
}

/// **A declared seat with a keel through the top.** A part's two
/// underside faces lie on a block's top face `y = 1`, declared as two
/// face-pair records (the Rest mate's declaration), and between them a
/// V keel hangs half a metre into the block along the part's whole
/// length. The records back every event the seat makes, so no finding
/// stands; the records are meetings, the pair is probed, and the keel's
/// corners are inside the block. The flat part on the same records
/// still validates.
#[test]
fn a_declared_seat_with_a_keel_is_probed() {
    for keel in [true, false] {
        let base: common::Prism<f64> = common::prism_z(
            &[(-1.0, -1.0), (4.0, -1.0), (4.0, 1.0), (-1.0, 1.0)],
            -1.0,
            4.0,
            Tol::witness(),
        );
        let prof: Vec<(f64, f64)> = if keel {
            vec![
                (0.0, 1.0),
                (1.0, 1.0),
                (1.5, 0.5),
                (2.0, 1.0),
                (3.0, 1.0),
                (3.0, 1.5),
                (0.0, 1.5),
            ]
        } else {
            vec![
                (0.0, 1.0),
                (1.0, 1.0),
                (2.0, 1.0),
                (3.0, 1.0),
                (3.0, 1.5),
                (0.0, 1.5),
            ]
        };
        let part: common::Prism<f64> = common::prism_z(&prof, 0.0, 3.0, Tol::witness());
        let mut body = base.body;
        let top = base.side_faces[2];
        let keys = topo::graft_disjoint_all_keyed(&mut body, &part.body).unwrap();
        let under = |i: usize| keys.face(part.side_faces[i]).unwrap();
        let records = ContactRecords {
            patches: vec![
                PatchContact {
                    face_a: under(0),
                    face_b: top,
                },
                PatchContact {
                    face_a: under(if keel { 3 } else { 2 }),
                    face_b: top,
                },
            ],
            ..ContactRecords::default()
        };
        let errors = validate_pseudomanifold(&body, &records, Tol::witness())
            .err()
            .unwrap_or_default();
        let placements = placement_findings(&errors);
        if keel {
            assert!(
                matches!(
                    placements[..],
                    [ValidationError::InstanceInterference { .. }]
                ),
                "{errors:?}"
            );
        } else {
            assert!(placements.is_empty(), "{errors:?}");
        }
    }
}

/// **One solid's boundary crossing itself blocks its pairs.** Solid A
/// is two slabs crossed in a plus, `[0, 3] × [1, 2] × [0, 1]` and
/// `[1, 2] × [0, 3] × [0, 1]`, as two shells of ONE solid: their edges
/// cross each other in the planes `z = 0` and `z = 1`. A block B rests
/// on A's first slab end, face to face. Every finding between A and B
/// is a rest, but A's own edge crosses say its boundary crosses itself,
/// and no placement can be read against such a material.
#[test]
fn a_solid_crossing_itself_blocks_its_pair() {
    let mut body = block((0.0, 3.0), (1.0, 2.0), (0.0, 1.0));
    let a = body.solids().next().unwrap().0;
    topo::graft_disjoint_all_onto_keyed(
        &mut body,
        &[a],
        &block((1.0, 2.0), (0.0, 3.0), (0.0, 1.0)),
    )
    .unwrap();
    topo::graft_disjoint(&mut body, &block((-1.0, 0.0), (1.0, 2.0), (0.0, 1.0))).unwrap();
    let errors = errors_of(&body);
    assert!(crosses(&errors) > 0, "{errors:?}");
    assert_eq!(refusals(&errors), [CROSSING], "{errors:?}");
}

/// A block `outer` with the block `void` cut out of it as a cavity: one
/// solid, an outer shell and a void shell.
fn hollow(outer: [(f64, f64); 3], void: [(f64, f64); 3]) -> Body<f64> {
    let mut body = block(outer[0], outer[1], outer[2]);
    let hole = block(void[0], void[1], void[2]);
    let solid = body.solids().next().unwrap().0;
    let evidence = VoidEvidence {
        shells: hole
            .shells()
            .map(|(s, _)| (s, VoidContainment::Probed(SolidContainment::In)))
            .collect(),
    };
    insert_void(&mut body, solid, hole, &evidence).unwrap();
    body
}

/// The Rest mate's declaration of every face-to-face seat in `body`:
/// planar faces of two solids on one plane with opposite outward
/// normals and overlapping extents, one patch record each.
fn rest_patches(body: &Body<f64>) -> Vec<PatchContact> {
    let extent = |f: topo::FaceKey| -> ([f64; 3], [f64; 3]) {
        let mut b = ([f64::MAX; 3], [f64::MIN; 3]);
        for (_, he) in body.half_edges() {
            if body.get_loop(he.parent_loop).unwrap().face != f {
                continue;
            }
            let p = body
                .get_point(body.get_vertex(he.start).unwrap().point)
                .unwrap();
            for (i, c) in p.to_array().into_iter().enumerate() {
                b.0[i] = b.0[i].min(c);
                b.1[i] = b.1[i].max(c);
            }
        }
        b
    };
    let planes: Vec<_> = body
        .faces()
        .filter_map(|(k, f)| match body.get_surface(f.surface) {
            Some(geom::Surface::Plane { origin, normal, .. }) => {
                let s = if f.sense { 1.0 } else { -1.0 };
                let n = [normal.x * s, normal.y * s, normal.z * s];
                let d = n[0] * origin.x + n[1] * origin.y + n[2] * origin.z;
                Some((k, body.get_shell(f.shell).unwrap().solid, n, d))
            }
            _ => None,
        })
        .collect();
    let mut out = Vec::new();
    for (i, &(fa, sa, na, da)) in planes.iter().enumerate() {
        for &(fb, sb, nb, db) in &planes[i + 1..] {
            let dot = na[0] * nb[0] + na[1] * nb[1] + na[2] * nb[2];
            if sa == sb || dot > -1.0 + 1e-9 || (da + db).abs() > 1e-9 {
                continue;
            }
            let (ba, bb) = (extent(fa), extent(fb));
            if (0..3).all(|k| ba.0[k] <= bb.1[k] + 1e-9 && bb.0[k] <= ba.1[k] + 1e-9) {
                out.push(PatchContact {
                    face_a: fa,
                    face_b: fb,
                });
            }
        }
    }
    out
}

fn declared_seats(body: &Body<f64>) -> Result<(), Vec<ValidationError>> {
    let records = ContactRecords {
        patches: rest_patches(body),
        ..ContactRecords::default()
    };
    assert!(!records.patches.is_empty());
    validate_pseudomanifold(body, &records, Tol::witness())
}

/// A U channel along `z`: floor `y ∈ [−1, 0]`, walls `x ∈ [−1, 0]` and
/// `x ∈ [10, 11]` rising to `y = 7`, `z ∈ [−1, 11]`.
fn channel() -> Body<f64> {
    let profile = [
        (-1.0, -1.0),
        (11.0, -1.0),
        (11.0, 7.0),
        (10.0, 7.0),
        (10.0, 0.0),
        (0.0, 0.0),
        (0.0, 7.0),
        (-1.0, 7.0),
    ];
    common::prism_z::<f64>(&profile, -1.0, 11.0, Tol::witness()).body
}

/// An L corner along `z`: walls `x ∈ [−1, 0]` and `y ∈ [−1, 0]`,
/// `z ∈ [−1, 11]`.
fn corner() -> Body<f64> {
    let profile = [
        (-1.0, -1.0),
        (11.0, -1.0),
        (11.0, 0.0),
        (0.0, 0.0),
        (0.0, 11.0),
        (-1.0, 11.0),
    ];
    common::prism_z::<f64>(&profile, -1.0, 11.0, Tol::witness()).body
}

const PART: [(f64, f64); 3] = [(0.0, 10.0), (0.0, 10.0), (0.0, 10.0)];
const CAVITY: [(f64, f64); 3] = [(4.0, 6.0), (4.0, 6.0), (4.0, 6.0)];

/// **A hollow part seated declared validates.** A part `[0, 10]³` with
/// a cavity `[4, 6]³` sits in a U channel (on its floor, between its
/// walls) or in an L corner, every seat declared as the Rest mate's
/// patch record. The cavity's hull lies inside the channel's and the
/// corner's reach box, but a void shell is not an outer shell, so the
/// gate reads only the part's outer shell; the pair meets only in
/// declared seats, so it is probed and its records are taken on their
/// word. The same seats on a solid part validate too.
#[test]
fn a_hollow_part_seated_declared_validates() {
    for support in [channel(), corner()] {
        for part in [hollow(PART, CAVITY), block(PART[0], PART[1], PART[2])] {
            let body = assembly(&[support.clone(), part]);
            assert_eq!(declared_seats(&body), Ok(()));
        }
    }
}

/// **The gate skips a void shell and probes nothing it need not.** The
/// same hollow part floats in the channel with a gap of 0.1 all round:
/// nothing meets, the cavity's hull is inside the channel's reach and
/// the part's outer shell is not, so the pair clears at the gate. A
/// block floating in the cavity, touching nothing, is inside the part's
/// reach and is probed: every corner of each is outside the other, and
/// the pair clears.
#[test]
fn a_void_shell_does_not_open_the_gate() {
    let gap = hollow(
        [(0.1, 9.9), (0.1, 9.9), (0.1, 9.9)],
        [(4.0, 6.0), (4.0, 6.0), (4.0, 6.0)],
    );
    let body = assembly(&[channel(), gap]);
    assert_eq!(
        validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness()),
        Ok(())
    );
    let body = assembly(&[
        hollow(PART, CAVITY),
        block((4.5, 5.5), (4.5, 5.5), (4.5, 5.5)),
    ]);
    assert_eq!(
        validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness()),
        Ok(())
    );
}

/// **A two-lump solid seated declared.** One lump `[0, 2] × [0, 1] ×
/// [0, 2]` rests on the U channel's floor, declared as a patch record.
/// Its sibling floats far off, or floats between the channel's walls
/// touching nothing — its hull inside the channel's reach, so the pair
/// is probed — and either way the pair validates. With the sibling sunk
/// in the channel's floor at `[4, 6] × [−0.8, −0.2] × [4, 6]` it
/// refuses: its corners are inside the channel.
#[test]
fn a_two_lump_solid_seated_declared() {
    let tol = Tol::witness();
    let seated = [(0.0, 2.0), (0.0, 1.0), (0.0, 2.0)];
    for (sibling, inside) in [
        ([(20.0, 22.0), (0.0, 1.0), (0.0, 2.0)], false),
        ([(4.0, 6.0), (3.0, 5.0), (4.0, 6.0)], false),
        ([(4.0, 6.0), (-0.8, -0.2), (4.0, 6.0)], true),
    ] {
        let lumps = topo::boolean::union(
            &block(seated[0], seated[1], seated[2]),
            &block(sibling[0], sibling[1], sibling[2]),
            tol,
        )
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone();
        assert_eq!(lumps.shells().count(), 2);
        let body = assembly(&[channel(), lumps]);
        let result = declared_seats(&body);
        if inside {
            let errors = result.unwrap_err();
            let solids: Vec<SolidKey> = body.solids().map(|(k, _)| k).collect();
            assert_eq!(
                the_interference(&errors),
                (solids[0], solids[1]),
                "{errors:?}"
            );
        } else {
            assert_eq!(result, Ok(()), "{sibling:?}");
        }
    }
}
