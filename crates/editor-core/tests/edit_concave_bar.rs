//! **A bar a size edit puts wholly inside a concave band's fill vanishes
//! on the blend's verdict** — the concave twin of `edit_planar_carve`. A
//! unit block less a pocket `[¼, ¾]²` from `z = ½` up, a round bar of
//! radius `0.006` along `x` at `(y, z) = (0.262, 0.512)` through the
//! pocket's end walls unioned in, and the pocket's floor edge
//! `(¼, ¼, ½) → (¾, ¼, ½)` filleted or chamfered. The band's fill buries
//! each end wall's sliver and the prism over it between the walls.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;
use editor_core::{
    EntityKey, EntityKind, EvalOptions, LoopProgram, NameRef, Node, ProfileDoc, ProfileProgram,
    RecipeNodeId, RoleSeg, StableName,
};
use fixture::{len, minted, on_frame, table, tol};
use topo::Body;

/// The bar's radius.
const A: f64 = 0.006;

fn volume(body: &Body<f64>) -> f64 {
    topo::test_support::pinned_volume(
        "closed-form mass properties",
        &topo::test_support::finished("the measured body", topo::Body::clone(body), tol()),
        tol(),
    )
    .volume
}

fn extrude(profile: RecipeNodeId, distance: f64) -> editor_core::AuthoredNode {
    Node::Extrude {
        profile: profile.into(),
        distance: len(distance),
    }
}

/// The barred die: the document and its union node.
fn barred_die() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("concave_bar", tol());
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, block) = fixture::insert(doc, extrude(p, 1.0));
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.25, 0.25), (0.75, 0.25), (0.75, 0.75), (0.25, 0.75)]],
    );
    let (doc, pocket) = fixture::insert(doc, extrude(p, 1.0));
    let (doc, pocket) = fixture::slid(doc, block, pocket, [0.0, 0.0, 0.5]);
    let (doc, die) = fixture::insert(
        doc,
        Node::Subtract {
            from: block.into(),
            tool: pocket.into(),
        },
    );
    let (doc, p) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Circle {
                centre: [len(0.262), len(0.512)],
                radius: len(A),
            }],
            ids: Vec::new(),
            on: None,
        }),
    );
    let (doc, bar) = fixture::insert(doc, extrude(p, 0.8));
    // The bar's own xy is the plane `x = 0.1`, its own `z` the block's `x`.
    let (doc, bar) = fixture::against(
        doc,
        block,
        bar,
        &fixture::axes_at([0.1, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]),
    );
    fixture::insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![die.into(), bar.into()]),
        },
    )
}

/// The name, at `node`, of the one entity of `kind` whose key `pick`
/// accepts.
fn named(
    ev: &editor_core::Evaluation<f64>,
    node: RecipeNodeId,
    pick: impl Fn(EntityKey) -> bool,
) -> Vec<StableName> {
    table(ev, node)
        .iter()
        .filter(|(_, e)| matches!(e, editor_core::Entry::Unique(r) if pick(r.key)))
        .map(|(n, _)| n.clone())
        .collect()
}

/// **The bar's wall, carried through a blend of size `0.005`, is
/// absorbed by one of `0.1` and vanishes on a clearance flip**, under a
/// fillet and under a chamfer. At `0.005` the band fills a part of the
/// corner `[0, 0.005]²` (pocket-local), which the bar (from `0.006`) is
/// clear of; at `0.1` it lies wholly inside the fill and the carve takes
/// it, at `ΔV = +(A − π·a²)/2`. The wall's name at the blend node, which
/// resolved before the edit, resolves `Vanished` after it, its cause a
/// `Positive → Negative` flip of one of the two clearance verdicts the
/// absorption is logged under (which one is the population diff's, as
/// `edit_planar_carve` says).
#[test]
fn a_bar_a_size_edit_puts_inside_a_concave_fill_vanishes_on_the_blends_verdict() {
    let (doc, union) = barred_die();
    let ev0 = fixture::run(&doc, &EvalOptions::default());
    let body = corpus::body_of(&ev0, union);
    let point = |v| {
        *body
            .get_point(body.get_vertex(v).expect("a vertex").point)
            .expect("a point")
    };
    let floor = named(&ev0, union, |k| match k {
        EntityKey::Edge(e) => {
            let he = body.get_edge(e).expect("an edge").he_plus;
            let s = point(body.get_half_edge(he).expect("a half").start);
            let t = point(body.half_edge_end(he).expect("an end"));
            let on =
                |p: geom_core::Point3<f64>| (p.y - 0.25).abs() < 1e-12 && (p.z - 0.5).abs() < 1e-12;
            on(s) && on(t) && ((s.x - t.x).abs() - 0.5).abs() < 1e-12
        }
        _ => false,
    });
    assert_eq!(floor.len(), 1, "the pocket's floor edge");
    let walls = named(&ev0, union, |k| match k {
        EntityKey::Face(f) => matches!(
            body.get_face(f).and_then(|f| body.get_surface(f.surface)),
            Some(geom::Surface::Cylinder { radius, .. }) if (radius - A).abs() < 1e-12
        ),
        _ => false,
    });
    assert_eq!(walls.len(), 2, "the bar's two half-walls");
    for chamfer in [false, true] {
        let verb = if chamfer { "chamfer" } else { "fillet" };
        let node = if chamfer {
            Node::chamfer(union, len(0.005), floor.clone())
        } else {
            Node::fillet(union, len(0.005), floor.clone())
        };
        let (doc, blend) = fixture::insert(doc.clone(), node);
        let carried = minted(
            EntityKind::Face,
            blend,
            RoleSeg::From {
                read: fixture::out(&doc, union),
                of: NameRef::new(walls[0].clone()),
            },
        );
        let ev1 = fixture::run(&doc, &EvalOptions::default());
        assert!(
            table(&ev1, blend).lookup(&carried).is_some(),
            "{verb}: at 0.005 the blend carries the bar's wall through"
        );
        let r = 0.1;
        let (doc2, _) = fixture::step(
            doc.clone(),
            editor_core::DocEdit::SetParam {
                node: blend,
                slot: if chamfer {
                    editor_core::SlotId::ChamferDistance
                } else {
                    editor_core::SlotId::Radius
                },
                value: len(r).into(),
                fresh: Vec::new(),
            },
        );
        let ev2 = fixture::run(&doc2, &EvalOptions::default());
        assert!(
            table(&ev2, blend).lookup(&carried).is_none(),
            "{verb}: at 0.1 the carve takes the bar's wall"
        );
        let out = corpus::body_of(&ev2, blend);
        topo::validate_geometric(&out, tol()).unwrap_or_else(|e| panic!("{verb}: tier 3, {e:?}"));
        topo::validate_pseudomanifold(&out, &topo::ContactRecords::default(), tol())
            .unwrap_or_else(|e| panic!("{verb}: tier 3′, {e:?}"));
        let pi = core::f64::consts::PI;
        let dv = volume(&out) - volume(&(corpus::body_of(&ev2, union)));
        let section = if chamfer {
            r * r / 2.0
        } else {
            (1.0 - pi / 4.0) * r * r
        };
        let want = (section - pi * A * A) * 0.5;
        assert!((dv - want).abs() < 1e-12, "{verb}: ΔV {dv} vs {want}");
        let res = editor_core::resolve_with_prior(
            editor_core::RunCtx {
                doc: &doc2,
                eval: &ev2,
            },
            editor_core::RunCtx {
                doc: &doc,
                eval: &ev1,
            },
            doc2.read_of_node(blend).expect("the blend's read"),
            &carried,
        )
        .expect("a body read");
        let editor_core::Resolution::Failed(f) = res else {
            panic!("{verb}: the absorbed wall's name resolves Failed, got {res:?}");
        };
        let editor_core::ResolveError::Vanished { diagnosis, .. } = f.error else {
            panic!(
                "{verb}: the absorbed wall's name vanishes, got {:?}",
                f.error
            );
        };
        // The diagnosis names the node's first flip by predicate name.
        // Under the chamfer the face clearance's readings only gain
        // inside instances (none clear at 0.005 turns inside), which is
        // a divergence, not a flip, so the ring clearance is the flip.
        let predicate = if chamfer {
            "fillet3_ring_clearance"
        } else {
            "fillet3_face_clearance"
        };
        assert_eq!(
            diagnosis,
            editor_core::Diagnosis::PredicateFlip {
                predicate,
                from: geom_core::Sign::Positive,
                to: geom_core::Sign::Negative,
            },
            "{verb}: the wall vanishes on the blend's {predicate} flip"
        );
    }
}
