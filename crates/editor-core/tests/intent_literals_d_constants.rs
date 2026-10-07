//! **`Expr` holds no float** — INTENT-LITERALS PR D's rows of the
//! spec's §8 test plan (`docs/INTENT-LITERALS-SPEC.md`): 13 (exact
//! constants), 14's D half (a snapshot holding a float literal is
//! unreadable; the reduced-ratio refusal is `m4_pr6_refusal`'s), the
//! half of `equal-literals-lower-to-one-identity-token` inside formulas
//! (two typed values in a definition are two variables), and a row per
//! guard the authored `Quantity` leaf and the constant leaves add.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::{body_of, failures};
use crate::fixture::{insert, len, on_frame, prism_edges, square};
use editor_core::{
    CancelToken, Dimension, DimensionError, Distribution, DistributionRefusal, DocEdit,
    DocumentId, EvalError, EvalOptions, Evaluation, Expr, ExtrudeSide, Formula, FreeVar,
    LowerFault, Node, ParamValue, PersistError, ProfileDoc, ProfileProgram, Ratio, RecipeNodeId,
    SlotId, VarDecl, VarEnv, VarId, VarName, VarNameReason, apply, eval, evaluate, load,
    parse_formula, save,
};
use geom_core::{Bounds, Interval, Tol};
use topo::{Body, SurfaceField};

fn n(name: &'static str) -> VarName {
    VarName::from_static(name)
}

fn step(doc: &ProfileDoc, edit: DocEdit<ProfileProgram>) -> editor_core::Applied<ProfileProgram> {
    apply(doc, &edit, Tol::witness(), &editor_core::RefusingReach).expect("the edit applies")
}

fn declare(doc: &ProfileDoc, name: &'static str, def: VarDecl) -> ProfileDoc {
    step(doc, DocEdit::DeclareVar { name: n(name), def }).doc
}

fn named(name: &'static str) -> Formula {
    Formula::named(n(name), Dimension::Length)
}

fn mm(value: f64) -> Formula {
    Formula::length_in(value, quantity::MM).unwrap()
}

fn evaluated(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// A unit cube at `cx` with every edge blended by `radius`: the blend's
/// id.
fn filleted(doc: ProfileDoc, cx: f64, radius: Formula) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(cx, 0.0, 0.5)],
    );
    let (doc, cube) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let edges = prism_edges(&doc, cube, 4);
    let (doc, blend) = insert(doc, Node::fillet(cube, radius, edges));
    (doc, blend)
}

/// The radius token a blend's cylinder carries.
fn radius_token(body: &Body<f64>) -> topo::ParamSource {
    let face = topo::query::all_faces(body)
        .into_iter()
        .find(|&f| {
            body.get_face(f)
                .and_then(|fd| body.get_surface(fd.surface))
                .is_some_and(|s| matches!(s, geom::Surface::Cylinder { .. }))
        })
        .expect("a blended cube carries quarter-cylinder blends");
    let surface = body.get_face(face).expect("a live face").surface;
    body.surface_field_source(surface, SurfaceField::CylinderRadius)
        .expect("a document-built blend declares its radius")
        .clone()
}

/// The variables a slot's defined variable reads, in order.
fn definition_reads(doc: &ProfileDoc, blend: RecipeNodeId) -> Vec<VarId> {
    let var = doc.slot(blend, SlotId::Radius).expect("a blend reads its radius");
    let mut reads = Vec::new();
    doc.var(var)
        .and_then(|v| v.def().defined())
        .expect("the radius is a formula")
        .var_reads(&mut reads);
    reads.into_iter().map(|(var, _)| var).collect()
}

fn f64_of(expr: &Expr) -> f64 {
    eval::<f64>(expr, &VarEnv::default()).expect("a constant evaluates")
}

// -------------------------------------------------------------- row 13

/// Row 13: a constant is its reduced value — `2/4` is `1/2`, bit for
/// bit and token for token — a decimal is the rational it spells, its
/// `f64` is the decimal's own parse and its enclosure contains it, a
/// constant past the bound refuses, and a right angle is a quarter
/// turn to the bit. Breaks if the constructor stops reducing (the
/// tokens differ), if a ratio evaluates as a point interval (`lo ==
/// hi`), or if `turn` is anything but `T::tau()`.
#[test]
fn a_constant_is_its_exact_value() {
    let (half, two_quarters) = (Expr::ratio(1, 2).unwrap(), Expr::ratio(2, 4).unwrap());
    assert!(two_quarters.bit_eq(&half), "2/4 is 1/2");
    let doc = ProfileDoc::empty(DocumentId::derive("intent-literals-d-ratio"), Tol::witness());
    let doc = declare(&doc, "w", VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.25)));
    let times = |num, den| Formula::mul(named("w"), Formula::ratio(num, den).unwrap()).unwrap();
    let (doc, a) = filleted(doc, 0.0, times(1, 2));
    let (doc, b) = filleted(doc, 4.0, times(2, 4));
    let (doc, c) = filleted(doc, 8.0, times(1, 3));
    let ev = evaluated(&doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let token = |blend| radius_token(body_of(&ev, blend));
    assert_eq!(token(a), token(b), "equal constants lower to one token");
    assert_ne!(token(a), token(c));

    let tenth = parse_formula("0.1", &Default::default()).unwrap();
    assert_eq!(tenth.as_ratio(), Ratio::new(1, 10).ok(), "0.1 is 1/10");
    let tenth = Expr::try_from(&tenth).expect("a constant is already stored");
    assert_eq!(f64_of(&tenth).to_bits(), 0.1f64.to_bits());
    let enclosure = eval::<Interval>(&tenth, &VarEnv::default()).unwrap();
    assert!(enclosure.lo() < enclosure.hi(), "an enclosure, not a point: {enclosure:?}");
    assert!(
        enclosure.lo() <= 0.1 && 0.1 <= enclosure.hi(),
        "it encloses 1/10: {enclosure:?}"
    );

    assert!(matches!(
        Ratio::new(1 << 54, 1),
        Err(DimensionError::ConstantOutOfRange { .. })
    ));

    let right = Expr::div(Expr::turn(), Expr::ratio(4, 1).unwrap()).unwrap();
    assert_eq!(right.dim(), Dimension::Angle);
    assert_eq!(
        f64_of(&right).to_bits(),
        core::f64::consts::FRAC_PI_2.to_bits()
    );
    let parsed = parse_formula("turn/4", &Default::default()).unwrap();
    assert!(parsed.bit_eq(&Formula::from(&right)), "the text spells the same tree");
}

/// Row 13's symbolic half: `turn/4 − turn/4` is Zero as a theorem —
/// a constant is a node of the form, the same node twice — where two
/// toleranced written right angles are two parameters, and their
/// difference proves nothing. Breaks if `turn` evaluates to a float
/// constant the form cannot see is the same on both sides, or if two
/// written values bind as one parameter.
#[test]
fn a_quarter_turn_less_a_quarter_turn_is_zero_by_theorem() {
    use geom_core::predicate::{Band, Decide, Sign};
    use geom_core::sym::with_session_rules;
    use geom_core::{ParamSymbol, Sym, SymBudget, SymRules};
    type S = Sym<Interval>;
    let quarter = || Expr::div(Expr::turn(), Expr::ratio(4, 1).unwrap()).unwrap();
    let band = Band::linear(Tol::witness()).unwrap();
    let budget = SymBudget {
        max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
        max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
    };

    // The theorems the decision adds, past the evaluations' own (each
    // evaluation's finiteness check is a decision of its own).
    let session = |decide: bool| {
        with_session_rules(budget, SymRules::shipped(), || {
            let env = VarEnv::<S>::default();
            let a: S = eval(&quarter(), &env).unwrap();
            let b: S = eval(&quarter(), &env).unwrap();
            decide.then(|| (a - b).sign_within(band).map(|d| d.sign))
        })
    };
    let (_, before) = session(false);
    let (sign, after) = session(true);
    assert_eq!(sign, Some(Ok(Sign::Zero)));
    assert_eq!(
        after.symbolic_zero - before.symbolic_zero,
        1,
        "a theorem: {after:?}"
    );

    // Two written `90 deg`, each toleranced, are two variables: two
    // symbols, which the form cannot cancel.
    let (u, v) = (VarId(1), VarId(2));
    let session = |decide: bool| {
        with_session_rules(budget, SymRules::shipped(), || {
            let right = core::f64::consts::FRAC_PI_2;
            let mut env = VarEnv::<S>::default();
            for var in [u, v] {
                let value = S::param_over(
                    ParamSymbol::new(var.0),
                    Interval::from_bounds(right - 1e-3, right + 1e-3),
                    -1e-3,
                    1e-3,
                );
                env.bindings.insert(
                    var,
                    ParamValue::Continuous {
                        dim: Dimension::Angle,
                        value,
                    },
                );
            }
            let read = |var| Expr::var(var, Dimension::Angle);
            let difference = eval::<S>(&Expr::sub(read(u), read(v)).unwrap(), &env).unwrap();
            if decide {
                let _ = difference.sign_within(band);
            }
        })
    };
    let ((), before) = session(false);
    let ((), after) = session(true);
    assert_eq!(
        after.symbolic_zero, before.symbolic_zero,
        "no theorem: {after:?}"
    );
}

// --------------------------------------------- the half inside formulas

/// `equal-literals-lower-to-one-identity-token`, inside formulas: two
/// blends each written `w + 5 mm` read two `5 mm` variables, so their
/// expansions differ in a variable's id and their radius tokens
/// differ; the same formula reading one shared `v` lowers equal. And
/// one formula writing `5 mm` twice mints two variables (VR6). Breaks
/// if the lowering dedups written quantities by value, or keeps a
/// float in the definition (the tokens then agree on its bits).
#[test]
fn two_typed_values_in_formulas_are_two_variables() {
    let doc = ProfileDoc::empty(DocumentId::derive("intent-literals-d-typed"), Tol::witness());
    let doc = declare(&doc, "w", VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.0625)));
    let plus = |b: Formula| Formula::add(named("w"), b).unwrap();
    let (doc, a) = filleted(doc, 0.0, plus(mm(5.0)));
    let (doc, b) = filleted(doc, 4.0, plus(mm(5.0)));
    let (fa, fb) = (definition_reads(&doc, a), definition_reads(&doc, b));
    assert_eq!(fa.len(), 2, "w and a's own 5 mm");
    assert_eq!(fa[0], fb[0], "both read w");
    assert_ne!(fa[1], fb[1], "two writings, two variables");
    let doc = declare(&doc, "v", VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.005)));
    let (doc, c) = filleted(doc, 8.0, plus(named("v")));
    let (doc, d) = filleted(doc, 12.0, plus(named("v")));
    let ev = evaluated(&doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let token = |blend| radius_token(body_of(&ev, blend));
    assert_ne!(token(a), token(b));
    assert_eq!(token(c), token(d), "sharing is said by reading one variable");

    let twice = Formula::add(mm(5.0), mm(5.0)).unwrap();
    let (doc, e) = filleted(doc, 16.0, twice);
    let reads = definition_reads(&doc, e);
    assert_eq!(reads.len(), 2);
    assert_ne!(reads[0], reads[1], "one formula, two typed values, two variables");
    for var in reads {
        assert!(doc.is_typed_value(var), "each an anonymous free variable");
    }
}

/// A definition's written quantities mint in pre-order before the
/// variable they define, and are retired with it: replacing the slot
/// reports the definition, then each quantity. Breaks if the door mints
/// the definition first (the record's order) or leaves a quantity read
/// by nothing.
#[test]
fn a_definitions_quantities_mint_first_and_retire_after_it() {
    let doc = ProfileDoc::empty(DocumentId::derive("intent-literals-d-order"), Tol::witness());
    let doc = declare(&doc, "w", VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.0625)));
    let three = Formula::add(Formula::add(mm(1.0), named("w")).unwrap(), mm(2.0)).unwrap();
    let (doc, blend) = filleted(doc, 0.0, three);
    let formula = doc.slot(blend, SlotId::Radius).unwrap();
    let reads = definition_reads(&doc, blend);
    let w = doc.var_named("w").unwrap();
    let order = doc.var_order();
    let at = |var| order.iter().position(|&v| v == var).unwrap();
    assert_eq!(reads[1], w);
    assert!(at(reads[0]) < at(reads[2]), "pre-order");
    assert!(at(reads[2]) < at(formula), "the quantities before the definition");
    let applied = step(
        &doc,
        DocEdit::SetParam {
            node: blend,
            slot: SlotId::Radius,
            expr: named("w"),
            fresh: Vec::new(),
        },
    );
    let retired: Vec<VarId> = applied
        .maintenance
        .iter()
        .filter_map(|m| match m {
            editor_core::Maintenance::AnonymousVarRemoved { var, .. } => Some(var.id()),
            _ => None,
        })
        .collect();
    assert_eq!(retired, vec![formula, reads[0], reads[2]]);
}

/// A declared definition's own variable mints before its quantities, so
/// the id a refusal of it speaks is the id it is minted at. Breaks if a
/// quantity's mint is drawn first: the declare then holds another id
/// than the one predicted.
#[test]
fn a_declared_definition_mints_its_own_id_first() {
    let doc = ProfileDoc::empty(DocumentId::derive("intent-literals-d-declare"), Tol::witness());
    let doc = declare(&doc, "w", VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.0625)));
    let def = VarDecl::defined(Formula::add(named("w"), mm(1.0)).unwrap());
    let predicted = doc.spoken_declare(&n("h"), &def).id();
    let doc = declare(&doc, "h", def);
    assert_eq!(doc.var_named("h"), Some(predicted));
}

// -------------------------------------------------------------- row 14

/// Row 14's D half: a snapshot whose expression holds a float literal
/// is unreadable — the variant is gone — and the refusal names it.
#[test]
fn a_snapshot_holding_a_float_literal_is_unreadable() {
    let doc = ProfileDoc::empty(DocumentId::derive("intent-literals-d-load"), Tol::witness());
    let doc = declare(&doc, "w", VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.0625)));
    let doc = declare(
        &doc,
        "h",
        VarDecl::defined(Formula::mul(named("w"), Formula::ratio(2, 1).unwrap()).unwrap()),
    );
    let h = doc.var_named("h").unwrap();
    let text = save(&doc, &[], Tol::witness()).unwrap();
    let (header, body) = text.split_once('\n').unwrap();
    let mut body: serde_json::Value = serde_json::from_str(body).unwrap();
    body["snapshot"]["vars"][h.0.to_string()]["def"] = serde_json::json!({
        "Defined": { "Literal": { "value": 0.125, "dim": "Length", "unit": "m" } }
    });
    let tampered = format!("{header}\n{}", serde_json::to_string(&body).unwrap());
    match load(&tampered, Tol::witness()) {
        Err(PersistError::Unreadable { detail, .. }) => {
            assert!(detail.contains("Literal"), "{detail}");
        }
        other => panic!("a float literal is unreadable, got {other:?}"),
    }
    load(&text, Tol::witness()).expect("the untampered document loads");
}

// ------------------------------------------- the guards this PR adds

/// `turn` is a keyword: no variable is named it, and the refusal says
/// why. Breaks if the name door asks the parser for a reference and
/// reads the keyword's constant as padding.
#[test]
fn turn_is_no_variable_name() {
    let fault = VarName::new("turn").expect_err("a keyword is no name");
    assert_eq!(fault.reason, VarNameReason::Keyword);
    assert!(VarName::new("turns").is_ok());
}

/// A distribution is a written value's: a lone dimensionless number
/// carrying one becomes the written value it equals, an integer takes
/// none, and a formula is refused. Breaks if a toleranced number stays
/// a constant (the distribution is then dropped) or a formula takes one.
#[test]
fn only_a_written_value_carries_a_distribution() {
    let spread = Some(Distribution::Normal { sigma: 0.01 });
    let number = Formula::number(0.5).unwrap();
    assert!(number.as_ratio().is_some(), "0.5 is a constant until it carries a spread");
    let carried = number.with_distribution(spread).unwrap();
    let quantity = carried.as_quantity().expect("a toleranced number is a value");
    assert_eq!(quantity.value(), 0.5);
    assert_eq!(quantity.distribution(), spread.as_ref());
    assert_eq!(
        Formula::count(3).with_distribution(spread),
        Err(DistributionRefusal::CountHasNoAnnotation)
    );
    assert_eq!(
        Formula::add(mm(1.0), mm(2.0))
            .unwrap()
            .with_distribution(spread),
        Err(DistributionRefusal::NotAWrittenValue)
    );
}

/// A written quantity does not lower where nothing mints, and a name
/// does not evaluate outside a document: each refuses in the lowering's
/// own words. Breaks if either reads as a value it is not.
#[test]
fn an_authored_leaf_refuses_outside_the_door() {
    assert_eq!(
        Expr::try_from(&mm(5.0)),
        Err(LowerFault::Quantity {
            dim: Dimension::Length
        })
    );
    assert_eq!(eval::<f64>(&mm(5.0), &VarEnv::default()), Ok(0.005));
    let refused = eval::<f64>(&named("w"), &VarEnv::default()).unwrap_err();
    assert!(
        matches!(refused, EvalError::Unlowered(LowerFault::Name(_))),
        "{refused:?}"
    );
    assert_eq!(
        named("w").unresolvable().map(|f| matches!(f, LowerFault::Name(_))),
        Some(true)
    );
    assert_eq!(mm(5.0).unresolvable(), None);
}

/// A bare number at a slot's root is a value, inside a formula a
/// constant, and a number no constant in range spells is a written
/// value wherever it stands. Breaks if `Formula::number` keeps an
/// inexact number as a rounded constant (its bits then move) or makes
/// an exact one a variable inside a formula.
#[test]
fn a_bare_number_is_a_constant_where_it_is_exact() {
    assert!(Formula::number(0.25).unwrap().as_ratio().is_some());
    let inexact = Formula::number(0.1 + 0.2).unwrap();
    assert_eq!(
        inexact.as_quantity().map(|q| q.value().to_bits()),
        Some((0.1f64 + 0.2).to_bits())
    );
    let negative_zero = Formula::number(-0.0).unwrap();
    assert_eq!(
        negative_zero.literal_value().map(f64::to_bits),
        Some((-0.0f64).to_bits()),
        "the sign of a zero is a bit a constant cannot hold"
    );
    let text = editor_core::unparse(&inexact, &|_| None);
    assert!(
        parse_formula(&text, &Default::default()).unwrap().bit_eq(&inexact),
        "{text} reads back as the written value"
    );
}

/// A rebuild of a document by re-inserting its nodes as written is told
/// which anonymous variables it would not reproduce: a bare number a
/// definition reads, which written there reads back as a constant — and
/// not a number no constant spells, which reads back as the value it
/// is. Breaks if the written form's normalisation of a number goes
/// unreported, or an inexact number is reported as lost.
#[test]
fn a_number_a_definition_reads_does_not_reproduce() {
    let doc = ProfileDoc::empty(DocumentId::derive("intent-literals-d-written"), Tol::witness());
    let doc = declare(&doc, "w", VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.0625)));
    let inexact = Formula::scalar(0.1 + 0.2).unwrap();
    let (doc, _) = filleted(doc, 0.0, Formula::mul(named("w"), inexact).unwrap());
    assert!(
        doc.written_would_not_reproduce().is_empty(),
        "an inexact number is written back as itself"
    );
    let scalar = |value| VarDecl::Free(FreeVar::continuous(Dimension::Scalar, value));
    let (doc, profile) = on_frame(doc, [0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![square(4.0, 0.0, 0.5)]);
    let applied = step(
        &doc,
        DocEdit::InsertNode {
            node: Box::new(Node::Extrude {
                profile,
                distance: Formula::mul(named("w"), Formula::fresh(0, Dimension::Scalar)).unwrap(),
                side: ExtrudeSide::Along,
            }),
            fresh: vec![scalar(2.0)],
        },
    );
    let [entry] = applied.record.fresh[..] else {
        panic!("one entry minted: {:?}", applied.record.fresh)
    };
    assert_eq!(applied.doc.written_would_not_reproduce(), vec![entry]);
}
