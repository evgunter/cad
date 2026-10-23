//! Review probes for the mirror-seam walk (PR 5616): a cell read
//! through a mirror walks to its source's construction only where the
//! reflection fixes it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, len};
use editor_core::{
    CancelToken, EntityKey, EntityKind, EvalOptions, Evaluation, Node, NodeResult, Operand,
    ProfileDoc, RecipeNodeId, StableName, coincide, evaluate,
};
use geom_core::Tol;
use topo::Body;

fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn body(ev: &Evaluation<f64>, id: RecipeNodeId) -> &Body<f64> {
    match ev.nodes.get(&id) {
        Some(NodeResult::Ok(v)) => v.payload.output_body(0).expect("one body"),
        other => panic!("node {id:?} did not build: {other:?}"),
    }
}

fn extruded(loops: Vec<Vec<(f64, f64)>>) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = fixture::on_frame(
        ProfileDoc::empty_derived("review_mirror", Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        loops,
    );
    fixture::insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(0.5),
        },
    )
}

/// The points of the entity `n` names in `b`, sorted.
fn points(b: &Body<f64>, t: &editor_core::NameTable, n: &StableName) -> Vec<[f64; 3]> {
    let mut v: Vec<[f64; 3]> = match fixture::key_of(t, "an entity", n) {
        EntityKey::Vertex(v) => vec![fixture::point(b, v).to_array()],
        EntityKey::Edge(e) => fixture::ends(b, e)
            .iter()
            .map(|&v| fixture::point(b, v).to_array())
            .collect(),
        EntityKey::Face(f) => fixture::face_vertices(b, f)
            .into_iter()
            .map(|v| fixture::point(b, v).to_array())
            .collect(),
        EntityKey::Body => Vec::new(),
    };
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v
}

fn near(a: &[[f64; 3]], b: &[[f64; 3]]) -> bool {
    a.len() == b.len()
        && a.iter()
            .zip(b)
            .all(|(p, q)| p.iter().zip(q).all(|(x, y)| (x - y).abs() < 1e-9))
}

/// Mirror `source` across every planar face of it, plainly and flipped;
/// for each entity of the source, whether its image walks to the
/// source's construction ("fixed") and whether it is geometrically
/// fixed. Panics on a walk-fixed entity the reflection moves (a false
/// proof); returns (walk-fixed, geometric-only) counts.
fn sweep(what: &str, loops: Vec<Vec<(f64, f64)>>) -> (usize, usize, Vec<String>) {
    let (doc, src) = extruded(loops);
    let ev = eval(&doc);
    let b = body(&ev, src);
    let t = fixture::table(&ev, src);
    let names: Vec<StableName> = t.iter().map(|(n, _)| n.clone()).collect();
    let planar: Vec<StableName> = names
        .iter()
        .filter(|n| n.kind == EntityKind::Face)
        .filter(|n| {
            topo::readback::face_carrier_kind(b, fixture::face_of(t, "a face", n))
                .is_ok_and(|k| k == geom::SurfaceKind::Plane)
        })
        .cloned()
        .collect();
    let (mut fixed, mut geometric_only, mut misses) = (0, 0, Vec::new());
    for face in &planar {
        let plain: editor_core::pose::RoleRead<Operand> = editor_core::pose::RoleRead::plain(
            fixture::plane(Operand::select(Operand::output(src, 0), vec![face.clone()])),
        );
        let flipped = editor_core::pose::RoleRead::plain(fixture::flip(plain.read.clone()));
        for plane in [plain, flipped] {
            let (doc, m) = fixture::insert(
                doc.clone(),
                Node::Mirror {
                    body: Operand::output(src, 0).into(),
                    plane,
                },
            );
            let ev = eval(&doc);
            let (sb, st) = (body(&ev, src), fixture::table(&ev, src));
            let (ib, it) = (body(&ev, m), fixture::table(&ev, m));
            let image = doc.output(m, 0).unwrap();
            let source = doc.output(src, 0).unwrap();
            for n in &names {
                let a = coincide::construction(&doc, image, n, Tol::witness()).expect("walks");
                let s = coincide::construction(&doc, source, n, Tol::witness()).expect("walks");
                let walk_fixed = a == s;
                let geo_fixed = near(&points(ib, it, n), &points(sb, st, n));
                if walk_fixed {
                    fixed += 1;
                    assert!(
                        geo_fixed,
                        "{what}: across {face:?}, {n:?} walks fixed but the reflection moves it"
                    );
                } else {
                    assert_eq!(
                        a.maps.first(),
                        Some(&coincide::Map::Mirror(m)),
                        "{what}: an unfixed image is read through the mirror"
                    );
                    if geo_fixed && n.kind != EntityKind::Body {
                        geometric_only += 1;
                        misses.push(format!("plane {:?} cell {:?}", face.path, n.path));
                    }
                }
            }
        }
    }
    (fixed, geometric_only, misses)
}

/// Every walk-fixed image entity is geometrically fixed, across every
/// planar face, over profiles authored either way round, with a hole,
/// a near-collinear vertex, and a value-only coplanar prong; and under
/// a jitter that makes no two walls parallel, so nothing is fixed by
/// value.
#[test]
fn review_every_walk_fixed_image_is_geometrically_fixed() {
    let l = vec![
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (0.0, 2.0),
    ];
    let mut l_cw = l.clone();
    l_cw.reverse();
    let c = vec![
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 3.0),
        (0.0, 3.0),
        (0.0, 2.0),
        (1.0, 2.0),
        (1.0, 1.0),
        (0.0, 1.0),
    ];
    let jitter = |p: &[(f64, f64)]| -> Vec<(f64, f64)> {
        p.iter()
            .enumerate()
            .map(|(k, &(x, y))| {
                let k = k as f64;
                (
                    x + 0.07 * (1.3 * k + 0.4).sin(),
                    y + 0.05 * (0.9 * k + 1.1).cos(),
                )
            })
            .collect()
    };
    let hole = vec![
        vec![(0.0, 0.0), (3.0, 0.0), (3.0, 3.0), (0.0, 3.0)],
        vec![(1.0, 1.0), (1.0, 2.0), (2.0, 2.0), (2.0, 1.0)],
    ];
    let nearer = vec![(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0), (1e-7, 1.0)];
    let near_collinear = vec![(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0), (1e-3, 1.0)];
    for (what, loops) in [
        ("L", vec![l.clone()]),
        ("L clockwise", vec![l_cw]),
        ("C", vec![c.clone()]),
        ("L jittered", vec![jitter(&l)]),
        ("C jittered", vec![jitter(&c)]),
        ("square with a hole", hole),
        ("near-collinear", vec![near_collinear]),
        ("nearer-collinear", vec![nearer]),
    ] {
        let (fixed, geo_only, misses) = sweep(what, loops);
        println!("{what}: walk-fixed {fixed}, fixed by value only {geo_only}: {misses:?}");
        assert!(fixed > 0, "{what}: the sweep fixed something");
    }
}

/// **Re-derives the golden moves** (claim 6): the corpus plate mirrored
/// across its `−x` wall read plainly (this PR's fixture) and across its
/// `+x` wall flipped and slid by 2 (main's fixture) unions to the same
/// body, point for point, bit for bit.
#[test]
fn review_the_plain_and_the_slid_seam_plane_build_one_body() {
    use editor_core::pose::{PoseDef, PoseRole, RoleRead};
    let corpus = crate::corpus::symmetric_plate::document();
    let doc = &corpus.doc;
    let union = corpus.result.expect("the union");
    let Some(Node::Union {
        members: editor_core::Bodies::Spelled(members),
    }) = doc.node(union)
    else {
        panic!("the union")
    };
    let plate = doc.operation_of(members[0].read).expect("the plate");
    let ev = eval(doc);
    let (b, t) = (body(&ev, plate), fixture::table(&ev, plate));
    let wall_at = |x: f64| -> StableName {
        let found: Vec<StableName> = t
            .iter()
            .filter(|(n, _)| n.kind == EntityKind::Face)
            .filter(|(n, _)| {
                let pts = points(b, t, n);
                pts.len() == 4 && pts.iter().all(|p| p[0] == x)
            })
            .map(|(n, _)| n.clone())
            .collect();
        let [one] = found.as_slice() else {
            panic!("one wall at x = {x}: {found:?}")
        };
        one.clone()
    };
    let read = |def| RoleRead::plain(Operand::Pose(Box::new(def)));
    let face = |n: StableName| Operand::select(Operand::output(plate, 0), vec![n]);
    let plain = read(PoseDef::Plane {
        face: face(wall_at(0.0)),
    });
    let flipped = read(PoseDef::Flip {
        pose: read(PoseDef::Plane {
            face: face(wall_at(2.0)),
        }),
    });
    let slid = read(PoseDef::Slide {
        along: RoleRead::through(flipped.read.clone(), PoseRole::Normal),
        pose: flipped,
        by: editor_core::Formula::literal(2.0, editor_core::Dimension::Length).unwrap(),
    });
    let built = |plane: RoleRead<Operand>| -> Vec<[f64; 3]> {
        let (d, m) = fixture::insert(
            doc.clone(),
            Node::Mirror {
                body: Operand::output(plate, 0).into(),
                plane,
            },
        );
        let (d, u) = fixture::insert(
            d,
            Node::Union {
                members: editor_core::Bodies::Spelled(vec![
                    Operand::output(plate, 0).into(),
                    Operand::output(m, 0).into(),
                ]),
            },
        );
        let ev = eval(&d);
        let mut v: Vec<[f64; 3]> = body(&ev, u).points().map(|(_, p)| p.to_array()).collect();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        v
    };
    let (a, s) = (built(plain), built(slid));
    assert!(!a.is_empty());
    assert_eq!(
        a.iter().map(|p| p.map(f64::to_bits)).collect::<Vec<_>>(),
        s.iter().map(|p| p.map(f64::to_bits)).collect::<Vec<_>>(),
        "one body, bit for bit"
    );
}

/// Two mirrors of one body across one plane, and a mirror of a mirror
/// across the same plane: an off-plane cell's images are two
/// constructions (each mirror is its own link), and the double image is
/// not walked back to its source; an on-plane cell is its source in all.
#[test]
fn review_two_mirrors_across_one_plane_are_two_links() {
    let (doc, src) = extruded(vec![vec![
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (0.0, 2.0),
    ]]);
    let ev = eval(&doc);
    let (b, t) = (body(&ev, src), fixture::table(&ev, src));
    let names: Vec<StableName> = t.iter().map(|(n, _)| n.clone()).collect();
    let wall = names
        .iter()
        .find(|n| {
            n.kind == EntityKind::Face && {
                let p = points(b, t, n);
                p.len() == 4 && p.iter().all(|q| q[0] == 0.0)
            }
        })
        .unwrap()
        .clone();
    let plane = || {
        editor_core::pose::RoleRead::plain(fixture::plane(Operand::select(
            Operand::output(src, 0),
            vec![wall.clone()],
        )))
    };
    let (doc, m1) = fixture::insert(
        doc,
        Node::Mirror {
            body: Operand::output(src, 0).into(),
            plane: plane(),
        },
    );
    let (doc, m2) = fixture::insert(
        doc,
        Node::Mirror {
            body: Operand::output(src, 0).into(),
            plane: plane(),
        },
    );
    let (doc, mm) = fixture::insert(
        doc,
        Node::Mirror {
            body: Operand::output(m1, 0).into(),
            plane: plane(),
        },
    );
    let walk = |node, n: &StableName| {
        coincide::construction(&doc, doc.output(node, 0).unwrap(), n, Tol::witness()).unwrap()
    };
    let cap = names
        .iter()
        .find(|n| {
            matches!(
                n.path.as_slice(),
                [editor_core::RoleSeg::Cap(editor_core::CapEnd::End)]
            )
        })
        .unwrap();
    assert_eq!(walk(m1, cap).maps, [coincide::Map::Mirror(m1)]);
    assert_ne!(walk(m1, cap), walk(m2, cap), "two mirrors, two links");
    assert_eq!(
        walk(mm, cap).maps,
        [coincide::Map::Mirror(mm), coincide::Map::Mirror(m1)],
        "a double image is not walked back to its source"
    );
    for node in [m1, m2, mm] {
        assert_eq!(
            walk(node, &wall),
            walk(src, &wall),
            "the plane's face is fixed"
        );
    }
}

/// **A copy's cell on the copy of the plane's face is not on the plane**:
/// a copy of the plate slid 5 along `x` into the plate's own space,
/// mirrored across the plane of the plate's `−x` wall. The copy's `−x`
/// wall and every edge and vertex on it are named as the plate's are,
/// but stand 5 off the plane, so each image is read through the mirror
/// and is no cell of the copy. `lies_on`'s "one chain of maps" guard is
/// the only thing that tells these apart.
#[test]
fn review_a_slid_copys_wall_is_not_on_the_plates_plane() {
    let (doc, src) = extruded(vec![vec![
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (0.0, 2.0),
    ]]);
    let ev = eval(&doc);
    let (b, t) = (body(&ev, src), fixture::table(&ev, src));
    let wall = t
        .iter()
        .map(|(n, _)| n.clone())
        .find(|n| {
            n.kind == EntityKind::Face && {
                let p = points(b, t, n);
                p.len() == 4 && p.iter().all(|q| q[0] == 0.0)
            }
        })
        .unwrap();
    let (doc, copy) = fixture::slid(doc, src, src, [5.0, 0.0, 0.0]);
    let plane = editor_core::pose::RoleRead::plain(fixture::plane(Operand::select(
        Operand::output(src, 0),
        vec![wall.clone()],
    )));
    let (doc, m) = fixture::insert(
        doc,
        Node::Mirror {
            body: Operand::output(copy, 0).into(),
            plane,
        },
    );
    let ev = eval(&doc);
    let (cb, ct) = (body(&ev, copy), fixture::table(&ev, copy));
    let (ib, it) = (body(&ev, m), fixture::table(&ev, m));
    let (copy_read, image_read) = (doc.output(copy, 0).unwrap(), doc.output(m, 0).unwrap());
    let mut checked = 0;
    for (n, _) in ct.iter() {
        if n.kind == EntityKind::Body {
            continue;
        }
        let a = coincide::construction(&doc, image_read, n, Tol::witness()).expect("walks");
        let s = coincide::construction(&doc, copy_read, n, Tol::witness()).expect("walks");
        assert!(!near(&points(ib, it, n), &points(cb, ct, n)), "{n:?} moves");
        assert_ne!(a, s, "{n:?}: the image is not the copy's cell");
        assert_eq!(a.maps.first(), Some(&coincide::Map::Mirror(m)), "{n:?}");
        checked += 1;
    }
    assert!(checked > 30, "{checked}");
}
