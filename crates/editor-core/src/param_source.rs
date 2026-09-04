//! **The single spelling of lowered expression identity**, and the
//! attach-at-mint pass that puts it on the fields a verb's parameter
//! reached.
//!
//! # The token
//!
//! [`lower`] turns a slot expression into the opaque
//! [`topo::ParamSource`] the kernel compares. The lowering is a
//! **canonical prefix encoding of the expression tree** under the
//! identity of the parameter table it was lowered against: a scope
//! prefix ([`ParamScope`]), then one tag byte per AST node, operands in
//! [`Expr::child`] order, literals as their `f64` BITS, parameters as
//! their names. It is injective — every distinct (scope, expression)
//! pair has a distinct byte string — so token equality IS expression
//! equality within one parameter table rather than a claim about it,
//! and [`invert`] reads a token back to a slot address holding it.
//!
//! **Bit-semantic, matching [`Expr::bit_eq`]**: `0.0` and `-0.0` are
//! different expressions here, and the display unit a literal was
//! authored in is not part of identity at all (D7).
//!
//! # The scope
//!
//! A parameter name is scoped to the document that declares it. Two
//! documents that both call their blend radius `r` hold two
//! parameters, and the two meet inside ONE evaluation whenever a part
//! is instantiated — the referenced document's product is placed with
//! `transform_rigid`, which carries these records verbatim because a
//! rigid map cannot change a radius. So the token names the TABLE as
//! well as the expression:
//!
//! - the document under evaluation lowers under [`ParamScope::Root`]
//!   — its stable [`DocumentId`], and no version: within one
//!   evaluation there is exactly one current document, and the id is
//!   a fact about the document rather than about any edit of it, so a
//!   memo-served body's token still names the table its re-run
//!   siblings lower against;
//! - a referenced document lowers under [`ParamScope::Part`] — the
//!   [`DocRef`] it was reached through, id AND pin: a host may
//!   instantiate one document at two pins, and those are two versions
//!   of one table whose `r` need not agree, so the version is part of
//!   the identity exactly where versions can coexist. Two instances of
//!   one `DocRef` are one table and still declare.
//!
//! Both are functions of the recipe (D9): the root's id is the
//! document's own, the part's reference is a node of the host. A
//! document opened standalone and the same document instantiated
//! therefore lower to different tokens, which is the asymmetry stated
//! rather than hidden: the two never meet in one evaluation, and
//! [`invert`] answers for the root scope only.
//!
//! **Why a function of the expression and not an index into a table.**
//! A counter handing out ids in evaluation order would be a fact about
//! the RUN, and the memo serves bodies built by an earlier run: a node
//! whose content key is unchanged keeps its geometry, records and all,
//! while its siblings re-run. Two tokens from two different runs' tables
//! would then meet inside one body, and index `0` of the old table would
//! compare EQUAL to index `0` of the new one while naming different
//! expressions — a false `Declared`, which is the one failure this
//! channel must not have. The `GeomSource` records avoid it the same
//! way, by being functions of stable recipe identity (a node id) rather
//! than of an evaluation counter, and this is that discipline at
//! expression granularity. Determinism (D9) is then immediate: the same
//! recipe lowers to the same bytes on every run, in any node order,
//! under any scheduling.
//!
//! # The memo
//!
//! A memo-served body carries the tokens it was minted with, so the
//! token has to be part of what the content key certifies: a slot
//! whose expression changed to another spelling of the SAME value
//! would otherwise hit the memo and hand back a body whose field rows
//! name an expression the document no longer holds. [`feed_content_key`]
//! writes a flow-bearing slot's lowered expression into the key beside
//! its value — only the slots whose declared flow reaches a stored
//! field, because only those put an expression's identity into the
//! value; every other slot reaches the value through its number alone.
//!
//! # The attach
//!
//! [`attach_blend`] walks the verb's DECLARED parameter→field flow
//! (`verbs::ParamFlow`) and stamps the token on exactly the fields it
//! names, found through the operation's own birth record. Nothing here
//! knows what a fillet is: it knows a flow, a record, and the rule that
//! a role family's carriers are the faces that family's rows name.
//!
//! The kernel never mints, composes or interprets one of these — see
//! `topo::param_source`.

use geom::Surface;
use geom_core::Real;
use sweep::blend::naming::BlendNaming;
use topo::{Body, FaceKey, ParamAttachError, ParamSource, SurfaceField};
use verbs::{FieldRole, ParamFlow, RoleFamily, ScalarParam};

use crate::eval::KeyHasher;
use crate::expr::{Dimension, Expr, ExprKind};
use crate::ident::{DocRef, DocumentId};

// The tag alphabet. Fixed arity per tag is what makes the prefix
// encoding injective: a reader knows how many operands to expect from
// the tag alone, so no separators and no lengths are needed above the
// leaves. The census in `tests` reads every constant below by name and
// refuses two with one value, and the decoder there parses the encoding
// back with its own arity table, so a tag that wrote a different number
// of operands than it claims cannot round-trip.
const T_LITERAL: u8 = 0x01;
const T_COUNT_LITERAL: u8 = 0x02;
const T_PARAM: u8 = 0x03;
const T_ADD: u8 = 0x10;
const T_SUB: u8 = 0x11;
const T_NEG: u8 = 0x12;
const T_MUL: u8 = 0x13;
const T_DIV: u8 = 0x14;
const T_SIN: u8 = 0x15;
const T_COS: u8 = 0x16;
const T_TAN: u8 = 0x17;
const T_ATAN2: u8 = 0x18;
const T_MIN: u8 = 0x19;
const T_MAX: u8 = 0x1a;
const T_COUNT_TO_SCALAR: u8 = 0x1b;
// The two scope tags open every token; they share the alphabet so the
// census sees them beside the node tags.
const T_SCOPE_ROOT: u8 = 0x20;
const T_SCOPE_PART: u8 = 0x21;

/// **The parameter table a token was lowered against.**
///
/// Module docs, "The scope": the root document by its stable id, a
/// referenced document by the reference (id and pin) it was reached
/// through.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ParamScope {
    /// The document under evaluation.
    Root(DocumentId),
    /// A referenced document, at the version the reference pins.
    Part(DocRef),
}

impl ParamScope {
    /// The scope of an evaluation reached through `chain` — the descent
    /// chain the part cache holds, empty at the top-level call and
    /// ending in this document's own reference below it.
    pub(crate) fn of(doc: DocumentId, chain: &[DocRef]) -> Self {
        match chain.last() {
            Some(doc_ref) => Self::Part(*doc_ref),
            None => Self::Root(doc),
        }
    }

    fn encode(self, out: &mut Vec<u8>) {
        match self {
            Self::Root(id) => {
                out.push(T_SCOPE_ROOT);
                out.extend_from_slice(&id.0.to_be_bytes());
            }
            Self::Part(doc_ref) => {
                out.push(T_SCOPE_PART);
                out.extend_from_slice(&doc_ref.id.0.to_be_bytes());
                out.extend_from_slice(&doc_ref.pin.0);
            }
        }
    }
}

/// The dimension byte carried by the two leaves that are not
/// determined by their own tag.
fn dim_code(dim: Dimension) -> u8 {
    match dim {
        Dimension::Length => 0,
        Dimension::Angle => 1,
        Dimension::Count => 2,
        Dimension::Scalar => 3,
    }
}

fn encode(expr: &Expr, out: &mut Vec<u8>) {
    // EXHAUSTIVE over the AST vocabulary with no wildcard arm (D3): a
    // new expression node cannot reach the kernel as an unlabelled
    // token, it breaks this match first.
    match expr.kind() {
        ExprKind::Literal(lit) => {
            out.push(T_LITERAL);
            out.extend_from_slice(&lit.value.to_bits().to_be_bytes());
            out.push(dim_code(expr.dim()));
        }
        ExprKind::CountLiteral(n) => {
            out.push(T_COUNT_LITERAL);
            out.extend_from_slice(&n.to_be_bytes());
        }
        ExprKind::Param(name) => {
            out.push(T_PARAM);
            let bytes = name.0.as_bytes();
            // A length prefix, because a name is the one payload with
            // no fixed width. The width is `u32`, saturating: a name
            // beyond four gigabytes is not a name any document can
            // hold, and if one ever arrived the prefix would name the
            // first `u32::MAX` bytes and the slice would carry exactly
            // those — still a prefix code, still self-delimiting,
            // never a panic.
            let len = u32::try_from(bytes.len()).unwrap_or(u32::MAX);
            out.extend_from_slice(&len.to_be_bytes());
            out.extend_from_slice(&bytes[..len as usize]);
            out.push(dim_code(expr.dim()));
        }
        ExprKind::Add(a, b) => binary(T_ADD, a, b, out),
        ExprKind::Sub(a, b) => binary(T_SUB, a, b, out),
        ExprKind::Mul(a, b) => binary(T_MUL, a, b, out),
        ExprKind::Div(a, b) => binary(T_DIV, a, b, out),
        ExprKind::Atan2(a, b) => binary(T_ATAN2, a, b, out),
        ExprKind::Min(a, b) => binary(T_MIN, a, b, out),
        ExprKind::Max(a, b) => binary(T_MAX, a, b, out),
        ExprKind::Neg(a) => unary(T_NEG, a, out),
        ExprKind::Sin(a) => unary(T_SIN, a, out),
        ExprKind::Cos(a) => unary(T_COS, a, out),
        ExprKind::Tan(a) => unary(T_TAN, a, out),
        ExprKind::CountToScalar(a) => unary(T_COUNT_TO_SCALAR, a, out),
    }
}

fn binary(tag: u8, a: &Expr, b: &Expr, out: &mut Vec<u8>) {
    out.push(tag);
    encode(a, out);
    encode(b, out);
}

fn unary(tag: u8, a: &Expr, out: &mut Vec<u8>) {
    out.push(tag);
    encode(a, out);
}

/// **The lowered identity of one slot expression under one table.**
///
/// Two slots anywhere in the document lower to the same token exactly
/// when they hold the same expression — which is what makes "both walls
/// offset by the same declared `t`" equal by construction (both are
/// `r − t`) while `r` and `r − t` differ. Two documents' slots never
/// do, whatever they spell: the scope prefix differs.
pub(crate) fn lower(scope: ParamScope, expr: &Expr) -> ParamSource {
    let mut bytes = Vec::new();
    scope.encode(&mut bytes);
    encode(expr, &mut bytes);
    ParamSource::from_lowered(&bytes)
}

/// **The expression half of a slot's identity, written into a content
/// key.** Scope-free on purpose: the key is compared against a prior
/// evaluation of the same document, so the table is a constant of the
/// comparison and the expression is the input that can move.
pub(crate) fn feed_content_key(h: &mut KeyHasher, expr: &Expr) {
    let mut bytes = Vec::new();
    encode(expr, &mut bytes);
    h.write_bytes(&bytes);
}

/// **Whether a scalar parameter's expression reaches the value at all**
/// — the rule for which slots [`feed_content_key`] applies to. A
/// parameter whose declared flow names no field (the chamfer's setback)
/// puts nothing but its number into the body, so its spelling is not
/// an input to the value and re-spelling it is rightly a memo hit.
pub(crate) fn flow_bearing(param: ScalarParam) -> bool {
    param
        .verb()
        .param_flow()
        .iter()
        .any(|row| row.param == param && !row.fields.is_empty())
}

/// **The token read back, as an address that holds it** — the
/// inversion this side of the line owes an opaque token.
///
/// The kernel cannot answer "which expression is this?" and is not
/// meant to; here the document is in hand, so the answer is a real
/// address: the first slot of the first node whose expression lowers to
/// `token` under this document's ROOT scope, scanned in the document's
/// own deterministic node order ([`Doc::order`](crate::doc::Doc::order)).
///
/// **What the answer is, precisely.** A token is the identity of an
/// expression, not of a slot: every slot holding that expression lowers
/// to it, by design, so "the slot that produced it" is not a question
/// the token can answer and this door does not pretend to — it answers
/// "a slot that holds it", the first in order. The token of a memo-
/// served body is always one the current document holds at that
/// body's own slot ([`feed_content_key`] re-runs a node whose spelling
/// changed), so the first match is at least a slot spelling exactly
/// what the body was minted from.
///
/// A token minted from a slot this document does not hold — a
/// referenced document's, whose scope is that reference rather than
/// this root, or a slot since edited away — has no address and answers
/// `None`.
///
/// The scan is the honest shape for a diagnosis door: a stored
/// token→address table would be a second spelling of the recipe with
/// nothing forcing it to agree, and diagnosis is not on a hot path.
#[must_use]
pub fn invert<P: crate::ProfilePayload>(
    doc: &crate::doc::Doc<P>,
    token: &ParamSource,
) -> Option<crate::expr::ExprPath> {
    let scope = ParamScope::Root(doc.id());
    for &node in doc.order() {
        let Some(n) = doc.node(node) else { continue };
        for slot in n.slots() {
            let Some(expr) = n.expr(slot) else { continue };
            if lower(scope, expr) == *token {
                return Some(crate::expr::ExprPath {
                    node,
                    slot,
                    path: Vec::new(),
                });
            }
        }
    }
    None
}

/// **The stored fields a declared role can name**, in the order a
/// carrier is asked. One role can be two fields because one quantity
/// has two spellings: the rolling radius is a cylinder's `radius` where
/// the spine is straight and a torus's `minor_radius` where it is
/// curved.
///
/// This is the role axis alone. The KIND axis — which field a given
/// carrier stores — is `SurfaceField::belongs_to`, declared once in
/// `topo`, and [`field_of`] is the product of the two; nothing here
/// restates which surface kinds store which field.
fn role_fields(role: FieldRole) -> &'static [SurfaceField] {
    match role {
        FieldRole::BlendCarrierRadius => {
            &[SurfaceField::CylinderRadius, SurfaceField::TorusMinorRadius]
        }
        FieldRole::CornerCarrierRadius => &[SurfaceField::SphereRadius],
        FieldRole::BandCarrierMinorRadius => &[SurfaceField::TorusMinorRadius],
    }
}

/// **Which stored field of THIS carrier a declared field role names**:
/// the role's candidate field that the carrier's kind stores, by
/// `topo`'s own declaration — so a field this returns belongs to the
/// carrier BY CONSTRUCTION, and the attach door's `FieldNotOnKind`
/// refusal has nothing left to catch.
///
/// `None` is a real answer, not a gap: a blend face on a plane (a
/// chamfer's strip, positioned by its setback) or on a spline arm (a
/// control net, not a radius) stores no field the role names, so the
/// channel simply does not reach it and the consuming family reads
/// absence — which refuses typed.
fn field_of<T: Real>(role: FieldRole, carrier: &Surface<T>) -> Option<SurfaceField> {
    role_fields(role)
        .iter()
        .copied()
        .find(|field| field.belongs_to(carrier))
}

/// The faces a role family's rows name in a blend birth record.
fn family_faces(family: RoleFamily, rec: &BlendNaming) -> Vec<FaceKey> {
    match family {
        RoleFamily::Blends => rec.blends.iter().map(|&(f, _)| f).collect(),
        RoleFamily::Corners => rec.corners.iter().map(|&(f, _)| f).collect(),
        RoleFamily::Bands => rec.bands.iter().map(|(f, _)| *f).collect(),
    }
}

/// **Attach-at-mint for a blend verb**: stamp `token` on every stored
/// field the verb's declared flow says `param` reached.
///
/// The flow is the kernel-side declaration
/// (`verbs::VerbKind::param_flow`) and this is its only consumer: the
/// document layer knows the expression, the verb knows where its
/// parameter lands, and the two meet exactly here. A flow with no rows
/// for `param` — the chamfer's setback, which positions planes and is
/// stored in none of them — attaches nothing, which is the declaration
/// being obeyed rather than a case being skipped.
///
/// # Errors
///
/// The kernel's attach door refuses a stale key or a field the carrier
/// does not store. Neither can happen here — every key was read off
/// `body` a moment earlier with no mutation between, and every field
/// came out of `belongs_to` — so a refusal is a broken invariant of
/// this function, surfaced typed rather than discarded: fed-but-dead
/// is the failure class a silent `let _` would reintroduce.
pub(crate) fn attach_blend<T: Real>(
    body: &mut Body<T>,
    flow: &[ParamFlow],
    param: ScalarParam,
    token: &ParamSource,
    rec: &BlendNaming,
) -> Result<(), ParamAttachError> {
    let Some(row) = flow.iter().find(|row| row.param == param) else {
        return Ok(());
    };
    let mut stamps: Vec<(topo::SurfaceKey, SurfaceField)> = Vec::new();
    for &role in row.fields {
        for face in family_faces(role.family(), rec) {
            let Some(surface_key) = body.get_face(face).map(|f| f.surface) else {
                continue;
            };
            let Some(carrier) = body.get_surface(surface_key) else {
                continue;
            };
            if let Some(field) = field_of(role, carrier) {
                stamps.push((surface_key, field));
            }
        }
    }
    for (surface_key, field) in stamps {
        body.set_surface_field_source(surface_key, field, token.clone())?;
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::{Point3, Vec3};

    use super::*;
    use crate::doc::ParamName;

    fn p(name: &str) -> Expr {
        Expr::param(ParamName::new(name), Dimension::Length)
    }

    fn lit(v: f64) -> Expr {
        Expr::literal(v, Dimension::Length).unwrap()
    }

    fn root() -> ParamScope {
        ParamScope::Root(DocumentId::derive("param-source-tests"))
    }

    /// How many operands follow a tag, or what fixed-width payload a
    /// leaf carries — the decoder's own reading of the alphabet.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum Shape {
        /// A leaf with a fixed payload width in bytes.
        Leaf(usize),
        /// The parameter leaf: a length-prefixed name and a dimension.
        Name,
        Unary,
        Binary,
    }

    /// EVERY tag constant, by name, with the shape the encoder gives it.
    const ALPHABET: &[(&str, u8, Shape)] = &[
        ("T_LITERAL", T_LITERAL, Shape::Leaf(9)),
        ("T_COUNT_LITERAL", T_COUNT_LITERAL, Shape::Leaf(8)),
        ("T_PARAM", T_PARAM, Shape::Name),
        ("T_ADD", T_ADD, Shape::Binary),
        ("T_SUB", T_SUB, Shape::Binary),
        ("T_NEG", T_NEG, Shape::Unary),
        ("T_MUL", T_MUL, Shape::Binary),
        ("T_DIV", T_DIV, Shape::Binary),
        ("T_SIN", T_SIN, Shape::Unary),
        ("T_COS", T_COS, Shape::Unary),
        ("T_TAN", T_TAN, Shape::Unary),
        ("T_ATAN2", T_ATAN2, Shape::Binary),
        ("T_MIN", T_MIN, Shape::Binary),
        ("T_MAX", T_MAX, Shape::Binary),
        ("T_COUNT_TO_SCALAR", T_COUNT_TO_SCALAR, Shape::Unary),
        ("T_SCOPE_ROOT", T_SCOPE_ROOT, Shape::Leaf(16)),
        ("T_SCOPE_PART", T_SCOPE_PART, Shape::Leaf(16 + 32)),
    ];

    /// **Every tag is distinct** — the injectivity argument's first
    /// premise, executed over the constants by NAME so that two
    /// constants sharing a value (`T_SUB = T_ADD`) red here rather than
    /// silently encoding `r + t` and `r - t` alike.
    #[test]
    fn every_tag_is_distinct() {
        for (i, (name, tag, _)) in ALPHABET.iter().enumerate() {
            for (other, tag2, _) in &ALPHABET[..i] {
                assert_ne!(
                    tag, tag2,
                    "{name} and {other} share the tag byte {tag:#04x}"
                );
            }
        }
    }

    /// **The alphabet is the whole encoder**: one row per AST arm plus
    /// the two scope tags. The match is exhaustive, so a new expression
    /// node fails this file until it is visited, and visiting it means
    /// naming its row.
    #[test]
    fn the_alphabet_covers_the_encoder() {
        let arms = match ExprKind::Neg(Box::new(lit(0.0))) {
            ExprKind::Literal(_)
            | ExprKind::CountLiteral(_)
            | ExprKind::Param(_)
            | ExprKind::Add(..)
            | ExprKind::Sub(..)
            | ExprKind::Mul(..)
            | ExprKind::Div(..)
            | ExprKind::Atan2(..)
            | ExprKind::Min(..)
            | ExprKind::Max(..)
            | ExprKind::Neg(_)
            | ExprKind::Sin(_)
            | ExprKind::Cos(_)
            | ExprKind::Tan(_)
            | ExprKind::CountToScalar(_) => 15,
        };
        assert_eq!(
            ALPHABET.len(),
            arms + 2,
            "an AST arm has no row in the tag alphabet"
        );
    }

    /// Parses one node at `bytes[at..]` by the decoder's own arity
    /// table, returning where it ended.
    fn parse_node(bytes: &[u8], at: usize) -> Option<usize> {
        let tag = *bytes.get(at)?;
        let (_, _, shape) = ALPHABET.iter().find(|(_, t, _)| *t == tag)?;
        match shape {
            Shape::Leaf(width) => {
                let end = at + 1 + width;
                (end <= bytes.len()).then_some(end)
            }
            Shape::Name => {
                let len_bytes: [u8; 4] = bytes.get(at + 1..at + 5)?.try_into().ok()?;
                let len = u32::from_be_bytes(len_bytes) as usize;
                let end = at + 5 + len + 1;
                (end <= bytes.len()).then_some(end)
            }
            Shape::Unary => parse_node(bytes, at + 1),
            Shape::Binary => {
                let mid = parse_node(bytes, at + 1)?;
                parse_node(bytes, mid)
            }
        }
    }

    /// A token parses as exactly one scope leaf and one expression.
    fn parses_whole(scope: ParamScope, expr: &Expr) -> bool {
        let mut bytes = Vec::new();
        scope.encode(&mut bytes);
        encode(expr, &mut bytes);
        let after_scope = parse_node(&bytes, 0);
        after_scope.and_then(|at| parse_node(&bytes, at)) == Some(bytes.len())
    }

    /// A small expression family: every leaf kind, every operator, two
    /// levels deep, plus the name-boundary and sign-of-zero pairs. Built
    /// through the dimension-checked doors, so only well-typed
    /// combinations enter (a length plus an angle is not an expression).
    fn family() -> Vec<Expr> {
        let count = Expr::count(3);
        let leaves = vec![
            p("a"),
            p("b"),
            p("ab"),
            p("c"),
            p("bc"),
            Expr::param(ParamName::new("a"), Dimension::Angle),
            lit(0.0),
            lit(-0.0),
            lit(1.0),
            count.clone(),
            Expr::count_to_scalar(count).unwrap(),
        ];
        let mut out = leaves.clone();
        for x in &leaves {
            out.push(Expr::neg(x.clone()));
            for y in &leaves {
                out.extend(Expr::add(x.clone(), y.clone()).ok());
                out.extend(Expr::sub(x.clone(), y.clone()).ok());
                out.extend(Expr::min(x.clone(), y.clone()).ok());
                out.extend(Expr::max(x.clone(), y.clone()).ok());
                out.extend(Expr::mul(x.clone(), y.clone()).ok());
                out.extend(Expr::div(x.clone(), y.clone()).ok());
                out.extend(Expr::atan2(x.clone(), y.clone()).ok());
            }
        }
        let angle = Expr::param(ParamName::new("th"), Dimension::Angle);
        out.extend(Expr::sin(angle.clone()).ok());
        out.extend(Expr::cos(angle.clone()).ok());
        out.extend(Expr::tan(angle).ok());
        let snapshot = out.clone();
        for x in &snapshot {
            for y in &leaves[..3] {
                out.extend(Expr::add(x.clone(), y.clone()).ok());
                out.extend(Expr::sub(y.clone(), x.clone()).ok());
            }
        }
        out
    }

    /// **Fixed arity per tag, executed**: every token in the family
    /// parses back as one scope leaf and one expression consuming
    /// exactly the bytes written, by a decoder that reads the arity
    /// off the tag alone. An encoder writing a different operand count
    /// than the alphabet claims for a tag cannot pass this.
    #[test]
    fn every_token_parses_back_by_its_tags_alone() {
        let part = ParamScope::Part(DocRef {
            id: DocumentId::derive("part"),
            pin: crate::ident::ContentPin::of_bytes(b"pin"),
        });
        for expr in family() {
            assert!(
                parses_whole(root(), &expr),
                "{expr:?} does not parse back whole"
            );
            assert!(
                parses_whole(part, &expr),
                "{expr:?} under a part scope does not parse back whole"
            );
        }
    }

    /// **Token equality is `bit_eq`, over the whole family**: the
    /// injectivity claim executed pairwise rather than asserted. Both
    /// directions — two expressions equal by bits share a token, and
    /// two that differ do not — with the name-boundary pair
    /// (`ab + c` vs `a + bc`), operand order, the sign of zero, and a
    /// name's dimension all inside the family.
    #[test]
    fn token_equality_is_expression_equality() {
        let family = family();
        let tokens: Vec<ParamSource> = family.iter().map(|e| lower(root(), e)).collect();
        for (i, x) in family.iter().enumerate() {
            for (j, y) in family.iter().enumerate().skip(i) {
                assert_eq!(
                    tokens[i] == tokens[j],
                    x.bit_eq(y),
                    "token equality disagrees with bit_eq for {x:?} and {y:?}"
                );
            }
        }
    }

    /// The display unit is not identity (D7), matching `bit_eq`.
    #[test]
    fn a_display_unit_is_not_identity() {
        let mm = quantity::unit_by_symbol("mm").unwrap();
        let cm = quantity::unit_by_symbol("cm").unwrap();
        let a = Expr::literal_with_unit(0.125, Dimension::Length, mm).unwrap();
        let b = Expr::literal_with_unit(0.125, Dimension::Length, cm).unwrap();
        assert!(a.bit_eq(&b));
        assert_eq!(lower(root(), &a), lower(root(), &b));
    }

    /// **Two scopes are two tokens for one expression**, and the part
    /// scope tells two pins of one document apart.
    #[test]
    fn a_scope_is_part_of_the_identity() {
        let a = DocumentId::derive("a");
        let b = DocumentId::derive("b");
        let pin1 = crate::ident::ContentPin::of_bytes(b"one");
        let pin2 = crate::ident::ContentPin::of_bytes(b"two");
        let r = p("r");
        assert_ne!(
            lower(ParamScope::Root(a), &r),
            lower(ParamScope::Root(b), &r)
        );
        assert_ne!(
            lower(ParamScope::Root(a), &r),
            lower(ParamScope::Part(DocRef { id: a, pin: pin1 }), &r),
            "a document opened standalone and the same document instantiated are two tables"
        );
        assert_ne!(
            lower(ParamScope::Part(DocRef { id: a, pin: pin1 }), &r),
            lower(ParamScope::Part(DocRef { id: a, pin: pin2 }), &r),
            "two pins of one document are two versions of one table"
        );
        assert_eq!(
            lower(ParamScope::Part(DocRef { id: a, pin: pin1 }), &r),
            lower(ParamScope::Part(DocRef { id: a, pin: pin1 }), &r),
        );
    }

    /// The scope of an evaluation follows the descent chain.
    #[test]
    fn the_scope_follows_the_descent_chain() {
        let host = DocumentId::derive("host");
        let part = DocRef {
            id: DocumentId::derive("part"),
            pin: crate::ident::ContentPin::of_bytes(b"pin"),
        };
        assert_eq!(ParamScope::of(host, &[]), ParamScope::Root(host));
        assert_eq!(ParamScope::of(part.id, &[part]), ParamScope::Part(part));
    }

    /// **Only a flow-bearing parameter's spelling is an input to the
    /// value**: the fillet's radius lands in fields, the chamfer's
    /// setback in none.
    #[test]
    fn the_key_feeds_exactly_the_flow_bearing_parameters() {
        for &param in ScalarParam::ALL {
            let fields: usize = param
                .verb()
                .param_flow()
                .iter()
                .filter(|row| row.param == param)
                .map(|row| row.fields.len())
                .sum();
            assert_eq!(flow_bearing(param), fields > 0, "{param:?}");
        }
        assert!(flow_bearing(ScalarParam::FilletRadius));
        assert!(!flow_bearing(ScalarParam::ChamferDistance));
    }

    /// **One rule, not two spellings**: for every role and every
    /// carrier kind, at most one of the role's candidate fields belongs
    /// to the kind (so `find` is not an order-dependent choice), and
    /// whatever `field_of` answers belongs to the carrier — which is
    /// true by construction and pinned here so a change to either
    /// declaration that broke it is caught at the seam that joins them.
    #[test]
    fn a_role_names_at_most_one_field_of_each_kind_and_it_belongs() {
        let o = Point3::new(0.0, 0.0, 0.0);
        let z = Vec3::new(0.0, 0.0, 1.0);
        let x = Vec3::new(1.0, 0.0, 0.0);
        let kinds: Vec<Surface<f64>> = vec![
            Surface::Plane {
                origin: o,
                normal: z,
                u_ref: x,
            },
            Surface::Cylinder {
                origin: o,
                axis: z,
                radius: 1.0,
                u_ref: x,
            },
            Surface::Sphere {
                center: o,
                radius: 1.0,
                axis: z,
                u_ref: x,
            },
            Surface::Torus {
                center: o,
                axis: z,
                major_radius: 2.0,
                minor_radius: 0.5,
                u_ref: x,
            },
        ];
        for role in [
            FieldRole::BlendCarrierRadius,
            FieldRole::CornerCarrierRadius,
            FieldRole::BandCarrierMinorRadius,
        ] {
            assert!(
                !role_fields(role).is_empty(),
                "{role:?} names no field at all"
            );
            for carrier in &kinds {
                let belonging = role_fields(role)
                    .iter()
                    .filter(|f| f.belongs_to(carrier))
                    .count();
                assert!(
                    belonging <= 1,
                    "{role:?} names two fields of one carrier kind"
                );
                if let Some(field) = field_of(role, carrier) {
                    assert!(field.belongs_to(carrier));
                }
            }
        }
    }
}
