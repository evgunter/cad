//! **INTENT stage 4 PR H — a mate-placed face is structural**
//! (`docs/INTENT-STAGE4-SPEC.md` §9; D10 `[placement.placement]`: "a
//! placed copy's frame … is the construction its bundle states, which
//! the coincidence door replays like any other").

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::result_large_err
)]

use crate::fixture::{self, ang, len};
use editor_core::pose::{PoseDef, PoseRole, RoleRead};
use editor_core::{
    BodyRead, ContactClass, Coord, DocEdit, EvalOptions, ExtrudeSide, Formula, Mate, MateRef, Node,
    Operand, ProfileDoc, RecipeNodeId, RefusingReach, SlideAxis, StableName, Target, Value,
};
use geom_core::Tol;

/// A box `size` on a side at `origin`, extruded from a plane parallel
/// to the world xy plane.
fn a_box(doc: ProfileDoc, origin: [f64; 3], size: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = fixture::on_frame(
        doc,
        origin,
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (size, 0.0), (size, size), (0.0, size)]],
    );
    fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(size),
            side: ExtrudeSide::Along,
        },
    )
}

/// The face of `node`'s value whose outward normal is `n`.
fn face_toward(doc: &ProfileDoc, node: RecipeNodeId, n: [f64; 3]) -> StableName {
    let ev = fixture::run(doc, &EvalOptions::default());
    editor_core::all_faces(&ev, node)
        .into_iter()
        .find(|name| {
            let at = editor_core::face_frame(&ev, node, name).expect("a face's frame");
            let s = if at.sense { 1.0 } else { -1.0 };
            let normal = at.axis * s;
            (normal - geom_core::linalg::Vec3::new(n[0], n[1], n[2])).norm() < 1e-9
        })
        .unwrap_or_else(|| panic!("no face faces {n:?}"))
}

fn pose(def: PoseDef<Operand, Formula>) -> Operand {
    Operand::Pose(Box::new(def))
}

fn plane_of(face: Operand) -> Operand {
    pose(PoseDef::Plane { face })
}

fn flipped(p: Operand) -> Operand {
    pose(PoseDef::Flip {
        pose: RoleRead::plain(p),
    })
}

fn placed(doc: ProfileDoc, body: RecipeNodeId) -> (ProfileDoc, RecipeNodeId) {
    fixture::insert(
        doc,
        Node::place(vec![BodyRead::plain(Operand::output(body, 0))]),
    )
}

fn add_value(doc: ProfileDoc, at: MateRef, coord: Coord, by: Formula) -> ProfileDoc {
    doc.apply(
        &DocEdit::AddValue {
            placement: at.placement,
            value: Box::new(Value {
                mate: at.id,
                coord,
                by,
            }),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &RefusingReach,
    )
    .expect("the plane mate leaves the freedom")
    .doc
}

/// **A stack**: a block standing where its body stands, and a smaller
/// box resting on its top by a plane mate, slid and spun by values.
fn stack(slide: [f64; 2], spin: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId, MateRef) {
    let doc = ProfileDoc::empty_derived("s4h_stack", Tol::witness());
    let (doc, block) = a_box(doc, [0.0, 0.0, 0.0], 1.0);
    let (doc, top) = a_box(doc, [5.0, 0.0, 0.0], 0.5);
    let (doc, under) = placed(doc, block);
    let doc = fixture::pinned(doc, under, &EvalOptions::default());
    let (doc, over) = placed(doc, top);
    let block_top = face_toward(&doc, under, [0.0, 0.0, 1.0]);
    let bottom = face_toward(&doc, top, [0.0, 0.0, -1.0]);
    let (doc, mate) = fixture::constrain_at(
        doc,
        over,
        Mate {
            on: plane_of(Operand::select(Operand::output(top, 0), vec![bottom])),
            to: Target::Pose(flipped(plane_of(Operand::select(
                Operand::output(under, 0),
                vec![block_top],
            )))),
            class: ContactClass::Rest,
        },
    );
    let doc = add_value(doc, mate, Coord::Slide(SlideAxis::X), len(slide[0]));
    let doc = add_value(doc, mate, Coord::Slide(SlideAxis::Y), len(slide[1]));
    let doc = add_value(doc, mate, Coord::Spin, ang(spin));
    (doc, under, over, mate)
}

#[test]
fn measure_a_pinned_stack_at_sym() {
    let doc = ProfileDoc::empty_derived("s4h_pinned", Tol::witness());
    let (doc, block) = a_box(doc, [0.0, 0.0, 0.0], 1.0);
    let (doc, top) = a_box(doc, [0.2, 0.2, 1.0], 0.5);
    let (doc, under) = placed(doc, block);
    let doc = fixture::pinned(doc, under, &EvalOptions::default());
    let (doc, over) = placed(doc, top);
    let doc = fixture::pinned(doc, over, &EvalOptions::default());
    let ev = fixture::run(&doc, &EvalOptions::default());
    let report = editor_core::run_checks(
        &doc,
        &ev,
        &editor_core::ChecksConfig::default(),
        Tol::witness(),
    )
    .expect("the checks run");
    eprintln!("pinned: {report}");
    probe(&doc, under, over);
}

#[test]
fn measure_a_stack_at_sym() {
    let _ = PoseRole::Xy;
    for (slide, spin) in [([0.0, 0.0], 0.0), ([0.1, 0.2], 0.0), ([0.1, 0.2], 0.3)] {
        let (doc, _under, _over, mate) = stack(slide, spin);
        let ev = fixture::run(&doc, &EvalOptions::default());
        let report = editor_core::run_checks(
            &doc,
            &ev,
            &editor_core::ChecksConfig::default(),
            Tol::witness(),
        )
        .expect("the checks run");
        eprintln!("slide {slide:?} spin {spin}: mate {mate:?}\n{report}");
        probe(&doc, _under, _over);
    }
}

fn probe(doc: &ProfileDoc, under: RecipeNodeId, over: RecipeNodeId) {
    use editor_core::{EntityKey, EntityRef, Entry};
    use geom_core::sym::deferred;
    let top_name = face_toward(doc, under, [0.0, 0.0, 1.0]);
    let bottom_name = face_toward(doc, over, [0.0, 0.0, -1.0]);
    let under_v = doc.output(under, 0).unwrap();
    let over_v = doc.output(over, 0).unwrap();
    let symbols = EvalOptions {
        symbols: true,
        ..EvalOptions::default()
    };
    let terms: usize = std::env::var("S4H_TERMS")
        .ok()
        .and_then(|t| t.parse().ok())
        .unwrap_or(256);
    let (out, counts) = geom_core::sym::deferred::with_deferred_session(
        geom_core::SymBudget {
            max_terms: terms,
            max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
        },
        geom_core::SymRules::shipped(),
        editor_core::drive::DEFAULT_SYM_RETRY,
        || {
            let o = &symbols;
            let ev: editor_core::Evaluation<geom_core::Sym<f64>> = editor_core::evaluate(
                doc,
                None,
                &editor_core::CancelToken::new(),
                o,
                Tol::witness(),
            );
            let face = |v, name: &StableName| {
                let Some(Entry::Unique(EntityRef {
                    body,
                    key: EntityKey::Face(f),
                })) = ev.table(v).and_then(|t| t.lookup(name))
                else {
                    panic!(
                        "no face {name:?} at {v:?}: {:?}",
                        ev.result(doc.operation_of(v).unwrap())
                    );
                };
                (ev.read_body(v, *body).unwrap().unwrap().clone(), *f)
            };
            let (a, fa) = face(under_v, &top_name);
            let (b, fb) = face(over_v, &bottom_name);
            let band = geom_core::Band::linear(Tol::witness()).unwrap();
            match topo::boolean::carrier_pair_decided(&a, fa, &b, fb, band) {
                Ok(Ok((rel, Some(m)))) => format!(
                    "{rel:?}: {:?} {:?}\n{}",
                    deferred::discharge_or_why(m),
                    deferred::dag_size(m),
                    deferred::render(m).join("\n---\n")
                ),
                other => format!("{other:?}"),
            }
        },
    );
    eprintln!("probe: {out} ({counts:?})");
}

/// Debug: the carrier verdict on an at-rest row's two cells at `Sym`.
pub(crate) fn render_row(
    doc: &ProfileDoc,
    resolver: Option<std::sync::Arc<dyn editor_core::PartResolver>>,
    row: &editor_core::NamedCoincidence,
) {
    use editor_core::{EntityKey, EntityRef, Entry, NamedCell};
    use geom_core::sym::deferred;
    let symbols = EvalOptions {
        symbols: true,
        resolver,
        ..EvalOptions::default()
    };
    let terms: usize = std::env::var("S4H_TERMS")
        .ok()
        .and_then(|t| t.parse().ok())
        .unwrap_or(256);
    let (out, counts) = geom_core::sym::deferred::with_deferred_session(
        geom_core::SymBudget {
            max_terms: terms,
            max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
        },
        geom_core::SymRules::shipped(),
        editor_core::drive::DEFAULT_SYM_RETRY,
        || {
            let ev: editor_core::Evaluation<geom_core::Sym<f64>> = editor_core::evaluate(
                doc,
                None,
                &editor_core::CancelToken::new(),
                &symbols,
                Tol::witness(),
            );
            let face = |cell: &NamedCell| {
                let NamedCell::Entity { input, name } = cell else {
                    panic!("{cell:?}")
                };
                let Some(Entry::Unique(EntityRef {
                    body,
                    key: EntityKey::Face(f),
                })) = ev.table(*input).and_then(|t| t.lookup(name))
                else {
                    panic!("no face {name:?}");
                };
                (ev.read_body(*input, *body).unwrap().unwrap().clone(), *f)
            };
            let (a, fa) = face(&row.cells[0]);
            let (b, fb) = face(&row.cells[1]);
            let band = geom_core::Band::linear(Tol::witness()).unwrap();
            use geom_core::Decide;
            if let (
                Some(topo::boolean::CarrierDesc::Plane { origin: oa, normal: na }),
                Some(topo::boolean::CarrierDesc::Plane { origin: ob, normal: nb }),
            ) = (topo::boolean::face_carrier(&a, fa), topo::boolean::face_carrier(&b, fb))
            {
                let check = |what: &str, x: geom_core::Sym<f64>| {
                    let d = x.sign_within(band).expect("decides");
                    eprintln!(
                        "  {what}: {:?} {:?} {:?} {:?}",
                        x.value,
                        d.sign,
                        deferred::discharge_or_why(d.margin),
                        deferred::dag_size(d.margin)
                    );
                    if std::env::var("S4H_RENDER").is_ok() {
                        eprintln!("    {}", deferred::render(d.margin).join(" | "));
                    }
                };
                for (k, (body, label)) in [(&a, "copy"), (&b, "target")].into_iter().enumerate() {
                    for f in body.faces().map(|(f, _)| f) {
                        if let Some(topo::boolean::CarrierDesc::Plane { normal, .. }) =
                            topo::boolean::face_carrier(body, f)
                        {
                            check(
                                &format!("{k} {label} face {f:?} n.n-1"),
                                normal.dot(normal) - <geom_core::Sym<f64> as geom_core::Real>::one(),
                            );
                        }
                    }
                }
                check("nx", na.x + nb.x);
                check("ny", na.y + nb.y);
                check("nz", na.z + nb.z);
                check("na.na-1", na.dot(na) - <geom_core::Sym<f64> as geom_core::Real>::one());
                check("nb.nb-1", nb.dot(nb) - <geom_core::Sym<f64> as geom_core::Real>::one());
                check("offset", (ob - oa).dot(na));
            }
            match topo::boolean::carrier_pair_decided(&a, fa, &b, fb, band) {
                Ok(Ok((rel, Some(m)))) => format!(
                    "{rel:?}: {:?} {:?}",
                    deferred::discharge_or_why(m),
                    deferred::dag_size(m),
                ),
                other => format!("{other:?}"),
            }
        },
    );
    eprintln!("render: {out} ({counts:?})");
}
