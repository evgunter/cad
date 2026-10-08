//! **`Expr` holds no float** — INTENT-LITERALS PR D's rows of the
//! spec's §8 test plan (`docs/INTENT-LITERALS-SPEC.md`): 13 (exact
//! constants), 14's D half (a snapshot holding a float literal is
//! unreadable; the reduced-ratio refusal is `m4_pr6_refusal`'s), the
//! half of `equal-literals-lower-to-one-identity-token` inside formulas
//! (two typed values in a definition are two variables), and a row per
//! guard the authored `Quantity` leaf and the constant leaves add.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::{body_of, failures};
use crate::fixture::{axis_in_plane, insert, len, on_frame, on_frame_keeping, prism_edges, square};
use editor_core::{
    CancelToken, Dimension, DimensionError, Distribution, DistributionRefusal, DocEdit, DocumentId,
    EvalError, EvalOptions, Evaluation, Expr, ExtrudeSide, Formula, FreeVar, LowerFault, Node,
    ParamValue, PersistError, ProfileDoc, ProfileProgram, Ratio, RecipeNodeId, SlotId, VarDecl,
    VarEnv, VarId, VarName, VarNameReason, apply, eval, evaluate, load, parse_formula, save,
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
            profile: profile.into(),
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
    let var = doc
        .slot(blend, SlotId::Radius)
        .expect("a blend reads its radius");
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
    let doc = ProfileDoc::empty(
        DocumentId::derive("intent-literals-d-ratio"),
        Tol::witness(),
    );
    let doc = declare(
        &doc,
        "w",
        VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.25)),
    );
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
    assert!(
        enclosure.lo() < enclosure.hi(),
        "an enclosure, not a point: {enclosure:?}"
    );
    // 1/10 lies strictly between the double `0.1` (above it) and the
    // double below that, so an enclosure holds it exactly when its
    // lower end is below `0.1` and its upper end at least `0.1`.
    assert!(
        enclosure.lo() < 0.1 && 0.1 <= enclosure.hi(),
        "it encloses 1/10, which lies below the double 0.1: {enclosure:?}"
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
    assert!(
        parsed.bit_eq(&Formula::from(&right)),
        "the text spells the same tree"
    );
}

/// A revolve of a square beside its axis by `angle`: the revolve's id.
fn revolved(doc: ProfileDoc, angle: Formula) -> (ProfileDoc, RecipeNodeId) {
    let (doc, plane, profile) = on_frame_keeping(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(2.0, 0.0, 0.5)],
    );
    let (doc, axis) = insert(doc, axis_in_plane(plane, (0.0, 0.0), (0.0, 1.0)));
    insert(
        doc,
        Node::Revolve {
            profile: profile.into(),
            axis: axis.into(),
            angle,
        },
    )
}

/// The symbolic tier's theorems a session of `run` adds by deciding,
/// past the ones its evaluations add (each evaluation's finiteness
/// check is a decision of its own), and the sign it decides.
fn theorems(
    run: impl Fn() -> geom_core::Sym<Interval>,
) -> (
    Option<Result<geom_core::predicate::Sign, geom_core::predicate::Indeterminate>>,
    u64,
) {
    use geom_core::predicate::{Band, Decide};
    use geom_core::sym::with_session_rules;
    use geom_core::{SymBudget, SymRules};
    let band = Band::linear(Tol::witness()).unwrap();
    let budget = SymBudget {
        max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
        max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
    };
    let session = |decide: bool| {
        with_session_rules(budget, SymRules::shipped(), || {
            let difference = run();
            decide.then(|| difference.sign_within(band).map(|d| d.sign))
        })
    };
    let (_, before) = session(false);
    let (sign, after) = session(true);
    (sign, after.symbolic_zero - before.symbolic_zero)
}

/// Row 13's symbolic half: two revolves each written `turn/4` store,
/// through the edit door's lowering, a definition holding the constant,
/// and their difference is Zero as a theorem — a constant is a node of
/// the form, the same node twice. Two revolves each written `90 deg`
/// with a tolerance hold two variables, which bind as two parameters
/// (§11: a free variable is an analysis axis exactly when it carries a
/// tolerance; untoleranced, each binds its exact nominal and the
/// difference would decide Zero too), and their difference proves
/// nothing. Breaks if the door lowers `turn/4` to a variable, if `turn`
/// evaluates to a float the form cannot see is the same on both sides,
/// or if two written values bind as one parameter.
#[test]
fn a_quarter_turn_less_a_quarter_turn_is_zero_by_theorem() {
    use geom_core::predicate::Sign;
    use geom_core::{ParamSymbol, Sym};
    type S = Sym<Interval>;
    let doc = ProfileDoc::empty(DocumentId::derive("intent-literals-d-sym"), Tol::witness());
    let quarter = || Formula::div(Formula::turn(), Formula::ratio(4, 1).unwrap()).unwrap();
    let (doc, a) = revolved(doc, quarter());
    let (doc, b) = revolved(doc, quarter());
    let read = |node| doc.slot(node, SlotId::RevolveAngle).unwrap();
    let stored = |node| {
        doc.var(read(node))
            .and_then(|v| v.def().defined())
            .expect("a formula is a defined variable")
            .clone()
    };
    let (ea, eb) = (stored(a), stored(b));
    let mut leaves = Vec::new();
    ea.var_reads(&mut leaves);
    assert!(
        leaves.is_empty(),
        "the definition reads no variable: {ea:?}"
    );
    let (sign, added) = theorems(|| {
        let env = VarEnv::<S>::default();
        eval::<S>(&ea, &env).unwrap() - eval::<S>(&eb, &env).unwrap()
    });
    assert_eq!(sign, Some(Ok(Sign::Zero)));
    assert_eq!(added, 1, "a theorem");

    let right = || {
        Formula::angle_in(90.0, quantity::DEG)
            .unwrap()
            .with_distribution(Some(Distribution::Normal { sigma: 1e-3 }))
            .unwrap()
    };
    let (doc, c) = revolved(doc, right());
    let (doc, d) = revolved(doc, right());
    let (u, v) = (
        doc.slot(c, SlotId::RevolveAngle).unwrap(),
        doc.slot(d, SlotId::RevolveAngle).unwrap(),
    );
    assert!(doc.is_typed_value(u) && doc.is_typed_value(v) && u != v);
    let (_, added) = theorems(|| {
        let half_pi = core::f64::consts::FRAC_PI_2;
        let mut env = VarEnv::<S>::default();
        for var in [u, v] {
            let value = S::param_over(
                ParamSymbol::new(var.0.digest()),
                Interval::from_bounds(half_pi - 1e-3, half_pi + 1e-3),
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
        eval::<S>(&Expr::sub(read(u), read(v)).unwrap(), &env).unwrap()
    });
    assert_eq!(added, 0, "no theorem");
}

/// The symbolic tier holds a non-dyadic constant exactly: `1/10 · 3 −
/// 3/10` is about `5.6e-17` in `f64` and decides Zero as a theorem in
/// `Sym`. Breaks if a ratio folds into the form as its rounded double
/// (`Rat::of_f64(0.1)`), whose triple is not `3/10`.
#[test]
fn a_non_dyadic_constant_is_exact_in_the_symbolic_tier() {
    type S = geom_core::Sym<Interval>;
    let ratio = |num, den| Expr::ratio(num, den).unwrap();
    let difference =
        Expr::sub(Expr::mul(ratio(1, 10), ratio(3, 1)).unwrap(), ratio(3, 10)).unwrap();
    assert_ne!(f64_of(&difference), 0.0, "the doubles do not cancel");
    let (sign, added) = theorems(|| eval::<S>(&difference, &VarEnv::default()).unwrap());
    assert_eq!(sign, Some(Ok(geom_core::predicate::Sign::Zero)));
    assert!(added >= 1, "a theorem, not a float that happens to vanish");
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
    let doc = ProfileDoc::empty(
        DocumentId::derive("intent-literals-d-typed"),
        Tol::witness(),
    );
    let doc = declare(
        &doc,
        "w",
        VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.0625)),
    );
    let plus = |b: Formula| Formula::add(named("w"), b).unwrap();
    let (doc, a) = filleted(doc, 0.0, plus(mm(5.0)));
    let (doc, b) = filleted(doc, 4.0, plus(mm(5.0)));
    let (fa, fb) = (definition_reads(&doc, a), definition_reads(&doc, b));
    assert_eq!(fa.len(), 2, "w and a's own 5 mm");
    assert_eq!(fa[0], fb[0], "both read w");
    assert_ne!(fa[1], fb[1], "two writings, two variables");
    let doc = declare(
        &doc,
        "v",
        VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.005)),
    );
    let (doc, c) = filleted(doc, 8.0, plus(named("v")));
    let (doc, d) = filleted(doc, 12.0, plus(named("v")));
    let ev = evaluated(&doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let token = |blend| radius_token(body_of(&ev, blend));
    assert_ne!(token(a), token(b));
    assert_eq!(
        token(c),
        token(d),
        "sharing is said by reading one variable"
    );

    let twice = Formula::add(mm(5.0), mm(5.0)).unwrap();
    let (doc, e) = filleted(doc, 16.0, twice);
    let reads = definition_reads(&doc, e);
    assert_eq!(reads.len(), 2);
    assert_ne!(
        reads[0], reads[1],
        "one formula, two typed values, two variables"
    );
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
    let doc = ProfileDoc::empty(
        DocumentId::derive("intent-literals-d-order"),
        Tol::witness(),
    );
    let doc = declare(
        &doc,
        "w",
        VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.0625)),
    );
    let three = Formula::add(Formula::add(mm(1.0), named("w")).unwrap(), mm(2.0)).unwrap();
    let (doc, blend) = filleted(doc, 0.0, three);
    let formula = doc.slot(blend, SlotId::Radius).unwrap();
    let reads = definition_reads(&doc, blend);
    let w = doc.var_named("w").unwrap();
    // A variable's mint ordinal is its place in the mint order.
    let at = |var: editor_core::VarId| var.0.ordinal();
    assert_eq!(reads[1], w);
    assert!(at(reads[0]) < at(reads[2]), "pre-order");
    assert!(
        at(reads[2]) < at(formula),
        "the quantities before the definition"
    );
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

/// A definition's written quantity is in the edit's mint log and in no
/// other: the document before the edit — what undo keeps — never minted
/// it, the same edit redone mints it at the same id (D9), and retiring
/// it leaves its id in the log, so it is never minted again (VR7).
/// Breaks if a quantity mints off the chain, or its retirement drops it
/// from the log.
#[test]
fn a_definitions_quantity_is_in_its_edits_mint_log() {
    let doc = ProfileDoc::empty(DocumentId::derive("intent-literals-d-undo"), Tol::witness());
    let before = declare(
        &doc,
        "w",
        VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.0625)),
    );
    let formula = || Formula::add(named("w"), mm(1.0)).unwrap();
    let (after, blend) = filleted(before.clone(), 0.0, formula());
    let quantity = definition_reads(&after, blend)[1];
    assert!(after.is_typed_value(quantity));
    assert!(after.has_minted_var(quantity));
    assert!(
        !before.has_minted_var(quantity) && before.var(quantity).is_none(),
        "the document undo keeps never minted it"
    );
    let (redone, _) = filleted(before, 0.0, formula());
    assert_eq!(
        definition_reads(&redone, blend)[1],
        quantity,
        "redone, the same id"
    );
    let retired = step(
        &after,
        DocEdit::SetParam {
            node: blend,
            slot: SlotId::Radius,
            expr: named("w"),
            fresh: Vec::new(),
        },
    )
    .doc;
    assert!(retired.var(quantity).is_none());
    assert!(
        retired.has_minted_var(quantity),
        "the log keeps a retired id"
    );
}

/// A declared definition's own variable mints before its quantities, so
/// the id a refusal of it speaks is the id it is minted at. Breaks if a
/// quantity's mint is drawn first: the declare then holds another id
/// than the one predicted.
#[test]
fn a_declared_definition_mints_its_own_id_first() {
    let doc = ProfileDoc::empty(
        DocumentId::derive("intent-literals-d-declare"),
        Tol::witness(),
    );
    let doc = declare(
        &doc,
        "w",
        VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.0625)),
    );
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
    let doc = declare(
        &doc,
        "w",
        VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.0625)),
    );
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

/// A distribution is a written value's: a lone constant carrying one
/// becomes the written value it equals, and carrying none stays the
/// constant; an integer takes none, and a formula is refused. Breaks if
/// a toleranced number stays a constant (the distribution is then
/// dropped), clearing a distribution turns a constant into a variable,
/// or a formula takes one.
#[test]
fn only_a_written_value_carries_a_distribution() {
    let spread = Some(Distribution::Normal { sigma: 0.01 });
    let number = Formula::ratio(1, 2).unwrap();
    assert_eq!(
        number.clone().with_distribution(None),
        Ok(number.clone()),
        "a constant carrying no spread is the constant"
    );
    let carried = number.with_distribution(spread).unwrap();
    let quantity = carried
        .as_quantity()
        .expect("a toleranced number is a value");
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
        named("w")
            .unresolvable()
            .map(|f| matches!(f, LowerFault::Name(_))),
        Some(true)
    );
    assert_eq!(mm(5.0).unresolvable(), None);
}

/// A bare number inside a formula is the exact constant its decimal
/// spells, and refuses `ConstantOutOfRange` where no constant in range
/// does — it never becomes a hidden variable (VR6); alone, such a
/// decimal is the written value it reads. `Formula::scalar` is always a
/// written value. Breaks if the text door falls back to a variable or
/// to a rounded constant inside a formula (`0.1000000000000000055…`
/// would read as 1/10), or `Formula::scalar` returns a constant.
#[test]
fn a_bare_number_inside_a_formula_is_exact_or_refused() {
    let params = [(n("w"), Dimension::Length)].into();
    let parsed = |text: &str| parse_formula(text, &params);
    assert_eq!(
        parsed("w * 0.25"),
        Ok(Formula::mul(named("w"), Formula::ratio(1, 4).unwrap()).unwrap())
    );
    for text in [
        "w * 1e-20",
        "w * 0.30000000000000004",
        "w * 0.1000000000000000055511151231257827",
        "w * 1e300",
    ] {
        assert!(
            matches!(
                parsed(text),
                Err(editor_core::ParseError::Dimension {
                    error: DimensionError::ConstantOutOfRange { .. },
                    ..
                })
            ),
            "{text}: {:?}",
            parsed(text)
        );
    }
    let alone = parsed("0.30000000000000004").unwrap();
    assert_eq!(
        alone.as_quantity().map(|q| q.value().to_bits()),
        Some((0.1f64 + 0.2).to_bits()),
        "a decimal alone is a value"
    );
    assert_eq!(parsed("0.5"), Ok(Formula::ratio(1, 2).unwrap()));
    assert!(Formula::scalar(0.25).unwrap().as_quantity().is_some());
    assert_eq!(
        Formula::scalar(-0.0)
            .unwrap()
            .literal_value()
            .map(f64::to_bits),
        Some((-0.0f64).to_bits()),
        "a written value keeps its bits"
    );
}

/// `h := w * 0.5±σ`, its number written through `Formula::scalar` with a
/// distribution, survives save and load: the log replays to the
/// document, with the same variables and the spread on the minted one.
/// Breaks if the wire rebuilds a dimensionless quantity as the constant
/// its value spells (the distribution then has no leaf and is dropped).
#[test]
fn a_toleranced_number_in_a_definition_survives_save_and_load() {
    let doc = ProfileDoc::empty(
        DocumentId::derive("intent-literals-d-spread"),
        Tol::witness(),
    );
    let snapshot = declare(
        &doc,
        "w",
        VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.0625)),
    );
    let spread = Distribution::Normal { sigma: 0.01 };
    let half = Formula::scalar(0.5)
        .unwrap()
        .with_distribution(Some(spread))
        .unwrap();
    let edit = DocEdit::DeclareVar {
        name: n("h"),
        def: VarDecl::defined(Formula::mul(named("w"), half).unwrap()),
    };
    let live = step(&snapshot, edit.clone()).doc;
    let text = save(&snapshot, std::slice::from_ref(&edit), Tol::witness()).unwrap();
    let loaded = load(&text, Tol::witness()).unwrap();
    assert_eq!(loaded.edits, vec![edit]);
    let spreads = |doc: &ProfileDoc| {
        doc.var_ids()
            .into_iter()
            .filter_map(|id| match doc.var(id)?.def() {
                editor_core::VarDef::Free(FreeVar::Continuous { distribution, .. }) => {
                    Some(*distribution)
                }
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(loaded.doc.var_ids().len(), live.var_ids().len());
    assert_eq!(spreads(&loaded.doc), spreads(&live));
    assert!(spreads(&live).contains(&Some(spread)));
}

/// A rebuild of a document by re-inserting its nodes as written is told
/// which anonymous variables it would not reproduce: a count a
/// definition reads, which written there reads back as the integer
/// constant — and not a written dimensionless value, which reads back
/// as the value it is. Breaks if the written form's normalisation of a
/// count goes unreported, or a written value is reported as lost.
#[test]
fn a_count_a_definition_reads_does_not_reproduce() {
    let doc = ProfileDoc::empty(
        DocumentId::derive("intent-literals-d-written"),
        Tol::witness(),
    );
    let doc = declare(
        &doc,
        "w",
        VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.0625)),
    );
    let inexact = Formula::scalar(0.1 + 0.2).unwrap();
    let (doc, _) = filleted(doc, 0.0, Formula::mul(named("w"), inexact).unwrap());
    assert!(
        doc.written_would_not_reproduce().is_empty(),
        "a written value is written back as itself"
    );
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(4.0, 0.0, 0.5)],
    );
    let applied = step(
        &doc,
        DocEdit::InsertNode {
            node: Box::new(Node::Extrude {
                profile: profile.into(),
                distance: Formula::mul(
                    named("w"),
                    Formula::count_to_scalar(Formula::fresh(0, Dimension::Count)).unwrap(),
                )
                .unwrap(),
                side: ExtrudeSide::Along,
            }),
            fresh: vec![VarDecl::Free(FreeVar::Count { value: 2 })],
        },
    );
    let [entry] = applied.record.fresh[..] else {
        panic!("one entry minted: {:?}", applied.record.fresh)
    };
    assert_eq!(applied.doc.written_would_not_reproduce(), vec![entry]);
}

// ------------------------------------------------- the text door's shape

/// `w` read at `Length` and a document-free reading of the text against
/// it: the parse, and its value with `w` bound to 1 m.
fn read_with_w(text: &str) -> (Formula, f64) {
    let params = [(n("w"), Dimension::Length)].into();
    let parsed = parse_formula(text, &params).unwrap_or_else(|e| panic!("{text}: {e}"));
    let w = VarId::new(1, 1);
    let lowered = parsed
        .lower(&|name| (name.as_str() == "w").then_some((w, editor_core::VarKind::Length)))
        .unwrap();
    let mut env = VarEnv::<f64>::default();
    env.bindings.insert(
        w,
        ParamValue::Continuous {
            dim: Dimension::Length,
            value: 1.0,
        },
    );
    (parsed, eval::<f64>(&lowered, &env).unwrap())
}

fn constant(num: i64, den: u64) -> Formula {
    Formula::ratio(num, den).unwrap()
}

/// An unspaced `INT/INT` is one ratio everywhere but as the right
/// operand of `/`, where the slashes divide left to right: `w/2/3`,
/// `w / 2/3` and `turn/2/3` are a sixth, by tree and by value, and
/// `w*2/3` is `w` times the ratio. Breaks if the lexer joins the pair
/// after a `/` (`w/2/3` is then `w ÷ 2/3`, one and a half `w`).
#[test]
fn an_unspaced_pair_after_a_slash_divides_left_to_right() {
    let sixth = |of: Formula| {
        Formula::div(Formula::div(of, constant(2, 1)).unwrap(), constant(3, 1)).unwrap()
    };
    for text in ["w/2/3", "w / 2/3"] {
        let (parsed, value) = read_with_w(text);
        assert!(parsed.bit_eq(&sixth(named("w"))), "{text}: {parsed:?}");
        assert_eq!(value.to_bits(), (1.0f64 / 2.0 / 3.0).to_bits(), "{text}");
    }
    let turns = parse_formula("turn/2/3", &Default::default()).unwrap();
    assert!(turns.bit_eq(&sixth(Formula::turn())), "{turns:?}");
    assert_eq!(
        f64_of(&Expr::try_from(&turns).unwrap()).to_bits(),
        (core::f64::consts::TAU / 2.0 / 3.0).to_bits()
    );
    let (parsed, value) = read_with_w("w*2/3");
    assert!(
        parsed.bit_eq(&Formula::mul(named("w"), constant(2, 3)).unwrap()),
        "{parsed:?}"
    );
    assert_eq!(value.to_bits(), (2.0f64 / 3.0).to_bits());
}

/// A ratio written `p/q` is bracketed as a divisor, so `unparse ∘
/// parse` is the identity over a chain `a / (p/q) / r`, signed
/// constants included. Breaks if `unparse` writes `w / 1/3`, which the
/// parser reads as two divisions.
#[test]
fn a_ratio_divisor_is_written_bracketed() {
    let params = [(n("w"), Dimension::Length)].into();
    let third = Formula::div(named("w"), constant(1, 3)).unwrap();
    assert_eq!(editor_core::unparse(&third, &|_| None), "w / (1/3)");
    let negative = Formula::div(named("w"), constant(-1, 3)).unwrap();
    assert_eq!(editor_core::unparse(&negative, &|_| None), "w / (-1/3)");
    let chain = Formula::div(third, constant(2, 7)).unwrap();
    for formula in [chain, negative] {
        let text = editor_core::unparse(&formula, &|_| None);
        let back = parse_formula(&text, &params).unwrap();
        assert!(back.bit_eq(&formula), "{text}: {back:?}");
        assert_eq!(editor_core::unparse(&back, &|_| None), text);
    }
    let parsed = parse_formula("w / (1/3) / 2", &params).unwrap();
    assert!(
        parsed.bit_eq(
            &Formula::div(
                Formula::div(named("w"), constant(1, 3)).unwrap(),
                constant(2, 1)
            )
            .unwrap()
        )
    );
}

/// Constants coerce to a scalar beside a non-count whichever side they
/// fold on: `2*3*w` and `w*2*3` both read six `w`. Breaks if only a lone
/// integer coerces (`2*3` then folds to a count first and refuses).
#[test]
fn constants_coerce_whichever_side_they_fold_on() {
    let (left, l) = read_with_w("2*3*w");
    let (right, r) = read_with_w("w*2*3");
    assert_eq!((l, r), (6.0, 6.0));
    assert!(
        left.bit_eq(
            &Formula::mul(
                Formula::mul(constant(2, 1), constant(3, 1)).unwrap(),
                named("w")
            )
            .unwrap()
        )
    );
    assert!(
        right.bit_eq(
            &Formula::mul(
                Formula::mul(named("w"), constant(2, 1)).unwrap(),
                constant(3, 1)
            )
            .unwrap()
        )
    );
}

/// An unspaced slash between numbers is a ratio, whose parts are
/// integers: `2/3.5`, `1/3e2` and `2.5/3` refuse saying so, where the
/// spaced division reads. Breaks if the lexer reads the pair as one
/// token and refuses on the `.` as a character outside the alphabet.
#[test]
fn a_ratio_with_a_part_that_is_no_integer_says_so() {
    for text in ["2/3.5", "1/3e2", "2.5/3"] {
        let refused = parse_formula(text, &Default::default()).unwrap_err();
        assert_eq!(
            refused,
            editor_core::ParseError::RatioPartNotInteger {
                pos: 0,
                text: text.to_string(),
            }
        );
        assert!(
            refused.to_string().contains("parts are integers"),
            "{refused}"
        );
    }
    let spaced = parse_formula("2 / 3.5", &Default::default()).unwrap();
    assert_eq!(
        f64_of(&Expr::try_from(&spaced).unwrap()).to_bits(),
        (2.0f64 / 3.5).to_bits()
    );
}
