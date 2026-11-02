//! **A cup a size edit puts wholly inside a band's region vanishes on
//! the blend's verdict** — the one-cycle twin of `edit_planar_carve` and
//! `edit_concave_bar`. Two documents: the unit block bored blind from
//! its end face `x = 0` to `x = ½` by a round bore of radius `0.006` at
//! `(y, z) = (0.988, 0.988)`, its edge `(0, 1, 1) → (1, 1, 1)` blended
//! (convex); and the unit block less a pocket `[¼, ¾]²` from `z = ½` up,
//! a round pin of the same radius at `(y, z) = (0.262, 0.512)` standing
//! from the end wall `x = ¼` to `x = ½`, the pocket's floor edge
//! `(¼, ¼, ½) → (¾, ¼, ½)` blended (concave).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;
use editor_core::{
    EntityKey, EntityKind, EvalOptions, LoopProgram, NameRef, Node, ProfileDoc, ProfileProgram,
    RecipeNodeId, RoleSeg, StableName,
};
use fixture::{len, minted, on_frame, table, tol};
use topo::Body;

/// The cup's radius.
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

/// A round profile of radius [`A`] at `(y, z)` on the plane `x = x0`,
/// extruded `length` along `x`, placed against `target`: the placement.
fn rod(
    doc: ProfileDoc,
    target: RecipeNodeId,
    x0: f64,
    (y, z): (f64, f64),
    length: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Circle {
                centre: [len(y), len(z)],
                radius: len(A),
            }],
            ids: Vec::new(),
            on: None,
        }),
    );
    let (doc, rod) = fixture::insert(doc, extrude(p, length));
    fixture::against(
        doc,
        target,
        rod,
        &fixture::axes_at([x0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]),
    )
}

/// The unit block.
fn block(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    fixture::insert(doc, extrude(p, 1.0))
}

/// The bored block: the document and its subtract node.
fn bored_block() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("blind_bore", tol());
    let (doc, block) = block(doc);
    let (doc, bore) = rod(doc, block, -0.1, (0.988, 0.988), 0.6);
    fixture::insert(
        doc,
        Node::Subtract {
            from: block.into(),
            tool: bore.into(),
        },
    )
}

/// The pinned die: the document and its union node.
fn pinned_die() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("pin", tol());
    let (doc, block) = block(doc);
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
    let (doc, pin) = rod(doc, block, 0.1, (0.262, 0.512), 0.4);
    fixture::insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![die.into(), pin.into()]),
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

/// What one side of the row reads: the document and its operand node,
/// the blended edge's ends, the cup's axis `(y, z)` and `x` range, the
/// band's length and the cup's length inside it, and the sign of `ΔV`.
struct Side {
    what: &'static str,
    doc: ProfileDoc,
    operand: RecipeNodeId,
    edge: ([f64; 3], [f64; 3]),
    axis: (f64, f64),
    xs: (f64, f64),
    band: f64,
    cup: f64,
    sign: f64,
}

/// **A cup's faces, carried through a blend of size `0.005`, go with
/// one of `0.1` and vanish on a clearance flip**, under a fillet and
/// under a chamfer, on both sides. At `0.005` the band is clear of the
/// cup; at `0.1` the cup lies wholly inside the band's region and the
/// carve takes it, at `ΔV = ∓(A·L − π·a²·ℓ)`. Every face of the cup — its
/// two half-walls and its floor or end — resolved at the blend node
/// before the edit and resolves `Vanished` after it, its cause a
/// `Positive → Negative` flip of a clearance verdict.
#[test]
fn a_cup_a_size_edit_puts_inside_a_bands_region_vanishes_on_the_blends_verdict() {
    let (bored, sub) = bored_block();
    let (pinned, union) = pinned_die();
    let sides = [
        Side {
            what: "a blind bore",
            doc: bored,
            operand: sub,
            edge: ([0.0, 1.0, 1.0], [1.0, 1.0, 1.0]),
            axis: (0.988, 0.988),
            xs: (0.0, 0.5),
            band: 1.0,
            cup: 0.5,
            sign: -1.0,
        },
        Side {
            what: "a pin",
            doc: pinned,
            operand: union,
            edge: ([0.25, 0.25, 0.5], [0.75, 0.25, 0.5]),
            axis: (0.262, 0.512),
            xs: (0.25, 0.5),
            band: 0.5,
            cup: 0.25,
            sign: 1.0,
        },
    ];
    for side in sides {
        let Side {
            what, doc, operand, ..
        } = side;
        let ev0 = fixture::run(&doc, &EvalOptions::default());
        let body = corpus::body_of(&ev0, operand);
        let point = |v| {
            *body
                .get_point(body.get_vertex(v).expect("a vertex").point)
                .expect("a point")
        };
        let near = |p: geom_core::Point3<f64>, q: [f64; 3]| {
            (p.x - q[0]).abs() < 1e-12 && (p.y - q[1]).abs() < 1e-12 && (p.z - q[2]).abs() < 1e-12
        };
        let edge = named(&ev0, operand, |k| match k {
            EntityKey::Edge(e) => {
                let he = body.get_edge(e).expect("an edge").he_plus;
                let s = point(body.get_half_edge(he).expect("a half").start);
                let t = point(body.half_edge_end(he).expect("an end"));
                (near(s, side.edge.0) && near(t, side.edge.1))
                    || (near(s, side.edge.1) && near(t, side.edge.0))
            }
            _ => false,
        });
        assert_eq!(edge.len(), 1, "{what}: the blended edge");
        let in_cup = |p: geom_core::Point3<f64>| {
            (p.y - side.axis.0).hypot(p.z - side.axis.1) < A + 1e-9
                && p.x > side.xs.0 - 1e-9
                && p.x < side.xs.1 + 1e-9
        };
        let cup = named(&ev0, operand, |k| match k {
            EntityKey::Face(f) => {
                let face = body.get_face(f).expect("a face");
                core::iter::once(face.outer)
                    .chain(face.rings.iter().copied())
                    .all(|lp| match body.get_loop(lp).expect("a loop").boundary {
                        topo::LoopBoundary::Empty { vertex } => in_cup(point(vertex)),
                        topo::LoopBoundary::Cycle { first } => body
                            .loop_cycle(first)
                            .expect("a cycle")
                            .into_iter()
                            .all(|he| in_cup(point(body.get_half_edge(he).expect("he").start))),
                    })
            }
            _ => false,
        });
        assert_eq!(
            cup.len(),
            3,
            "{what}: the cup's two half-walls and its floor or end"
        );
        for chamfer in [false, true] {
            let verb = if chamfer { "chamfer" } else { "fillet" };
            let node = if chamfer {
                Node::chamfer(operand, len(0.005), edge.clone())
            } else {
                Node::fillet(operand, len(0.005), edge.clone())
            };
            let (doc, blend) = fixture::insert(doc.clone(), node);
            let carried: Vec<StableName> = cup
                .iter()
                .map(|n| {
                    minted(
                        EntityKind::Face,
                        blend,
                        RoleSeg::From {
                            read: fixture::out(&doc, operand),
                            of: NameRef::new(n.clone()),
                        },
                    )
                })
                .collect();
            let ev1 = fixture::run(&doc, &EvalOptions::default());
            for n in &carried {
                assert!(
                    table(&ev1, blend).lookup(n).is_some(),
                    "{what} ({verb}): at 0.005 the blend carries the cup's face through"
                );
            }
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
            let out = corpus::body_of(&ev2, blend);
            topo::validate_geometric(&out, tol())
                .unwrap_or_else(|e| panic!("{what} ({verb}): tier 3, {e:?}"));
            topo::validate_pseudomanifold(&out, &topo::ContactRecords::default(), tol())
                .unwrap_or_else(|e| panic!("{what} ({verb}): tier 3′, {e:?}"));
            let pi = core::f64::consts::PI;
            let dv = volume(&out) - volume(&(corpus::body_of(&ev2, operand)));
            let section = if chamfer {
                r * r / 2.0
            } else {
                (1.0 - pi / 4.0) * r * r
            };
            let want = side.sign * (section * side.band - pi * A * A * side.cup);
            assert!(
                (dv - want).abs() < 1e-12,
                "{what} ({verb}): ΔV {dv} vs {want}"
            );
            for n in &carried {
                assert!(
                    table(&ev2, blend).lookup(n).is_none(),
                    "{what} ({verb}): at 0.1 the carve takes the cup's face"
                );
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
                    n,
                )
                .expect("a body read");
                let editor_core::Resolution::Failed(f) = res else {
                    panic!("{what} ({verb}): the taken face's name resolves Failed, got {res:?}");
                };
                let editor_core::ResolveError::Vanished { diagnosis, .. } = f.error else {
                    panic!(
                        "{what} ({verb}): the taken face's name vanishes, got {:?}",
                        f.error
                    );
                };
                assert!(
                    matches!(
                        diagnosis,
                        editor_core::Diagnosis::PredicateFlip {
                            predicate: "fillet3_face_clearance" | "fillet3_ring_clearance",
                            from: geom_core::Sign::Positive,
                            to: geom_core::Sign::Negative,
                        }
                    ),
                    "{what} ({verb}): the face vanishes on a clearance flip, got {diagnosis:?}"
                );
            }
        }
    }
}
