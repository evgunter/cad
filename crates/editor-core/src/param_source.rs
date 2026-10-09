//! **The lowered spelling of an expression, as a content key reads
//! it.**
//!
//! [`feed_content_key`] writes a slot expression into a content key as
//! a **canonical prefix encoding of the expression tree**: one tag byte
//! per AST node, operands in [`Expr::child`] order, constants exactly (a
//! ratio as its reduced numerator and denominator, an integer, `turn`),
//! a defined variable as its definition expanded, and a free variable
//! as its minted id (a name is not identity, VR8). It is injective —
//! every distinct expression has a distinct byte string — so two keys
//! that agree on it agree on the expression, not on a claim about it.
//!
//! **Matching [`Expr::bit_eq`]**: a stored expression holds no float,
//! so a written value enters as the id of the variable it was minted
//! as, never as its bits (VR8: two typed `5 mm` encode distinct).
//!
//! # Which expressions a key feeds
//!
//! Only those whose declared flow reaches a stored field: a verb's own
//! SLOT ([`flow_bearing`], read off that verb's declaration), and an
//! expression the OPERAND holds that a downstream verb lands in a field
//! it mints ([`operand_flow_bearing`], asked of the whole vocabulary — a
//! profile's carrier radius is nobody's slot). Every other expression
//! reaches the value through its number alone.

use verbs::{FlowSource, ScalarParam};

use crate::eval::KeyHasher;
use crate::expr::{Expr, ExprKind};
use crate::var::VarId;

// The tag alphabet. Fixed arity per tag is what makes the prefix
// encoding injective: a reader knows how many operands to expect from
// the tag alone, so no separators and no lengths are needed above the
// leaves. The census in `tests` reads every constant below by name and
// refuses two with one value, and the decoder there parses the encoding
// back with its own arity table, so a tag that wrote a different number
// of operands than it claims cannot round-trip.
const T_VAR: u8 = 0x04;
const T_RATIO: u8 = 0x05;
const T_INTEGER: u8 = 0x06;
const T_TURN: u8 = 0x07;
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

/// **A document's definitions, as the encoding reads them**: the
/// defining expression of a defined variable, `None` for a free one or
/// one the table does not hold.
pub(crate) type Definitions<'r, 'e> = &'r dyn Fn(VarId) -> Option<&'e Expr>;

/// The definitions of `doc`'s variable table.
pub(crate) fn definitions_of<'a, P>(
    doc: &'a crate::doc::Doc<P>,
) -> impl Fn(VarId) -> Option<&'a Expr> + 'a {
    |var| doc.var(var).and_then(|v| v.def().defined())
}

fn encode(expr: &Expr, defs: Definitions<'_, '_>, out: &mut Vec<u8>) {
    // EXHAUSTIVE over the AST vocabulary with no wildcard arm (D3): a
    // new expression node cannot reach a key as an unlabelled
    // byte, it breaks this match first.
    match expr.kind() {
        // A constant's dimension is its tag's: a ratio is a scalar, an
        // integer a count, `turn` an angle.
        ExprKind::Ratio(r) => {
            out.push(T_RATIO);
            out.extend_from_slice(&r.num().to_be_bytes());
            out.extend_from_slice(&r.den().to_be_bytes());
        }
        ExprKind::Integer(n) => {
            out.push(T_INTEGER);
            out.extend_from_slice(&n.to_be_bytes());
        }
        ExprKind::Turn => out.push(T_TURN),
        // A defined variable is its definition, expanded (VR8): a
        // reader of `h := 2·w` and a slot spelling `2·w` lower equal.
        // The doors bound each VARIABLE's expansion
        // (`DEFINITION_NODE_BOUND`), not a slot's: a slot reading one
        // `k` times writes up to `k` times that, linear in what was
        // written, never the exponential a diamond of definitions is.
        ExprKind::Var(var) if let Some(definition) = defs(*var) => encode(definition, defs, out),
        // A free variable's identity alone: its kind is fixed (VR3), so
        // the reader's cached dimension says nothing the id does not,
        // and a name is not identity (VR8: a rename moves no byte).
        ExprKind::Var(var) => {
            out.push(T_VAR);
            out.extend_from_slice(&var.0.ordinal().to_be_bytes());
            out.extend_from_slice(&var.0.digest().to_be_bytes());
        }
        ExprKind::Leaf(own) => match *own {},
        ExprKind::Add(a, b) => binary(T_ADD, a, b, defs, out),
        ExprKind::Sub(a, b) => binary(T_SUB, a, b, defs, out),
        ExprKind::Mul(a, b) => binary(T_MUL, a, b, defs, out),
        ExprKind::Div(a, b) => binary(T_DIV, a, b, defs, out),
        ExprKind::Atan2(a, b) => binary(T_ATAN2, a, b, defs, out),
        ExprKind::Min(a, b) => binary(T_MIN, a, b, defs, out),
        ExprKind::Max(a, b) => binary(T_MAX, a, b, defs, out),
        ExprKind::Neg(a) => unary(T_NEG, a, defs, out),
        ExprKind::Sin(a) => unary(T_SIN, a, defs, out),
        ExprKind::Cos(a) => unary(T_COS, a, defs, out),
        ExprKind::Tan(a) => unary(T_TAN, a, defs, out),
        ExprKind::CountToScalar(a) => unary(T_COUNT_TO_SCALAR, a, defs, out),
    }
}

fn binary(tag: u8, a: &Expr, b: &Expr, defs: Definitions<'_, '_>, out: &mut Vec<u8>) {
    out.push(tag);
    encode(a, defs, out);
    encode(b, defs, out);
}

fn unary(tag: u8, a: &Expr, defs: Definitions<'_, '_>, out: &mut Vec<u8>) {
    out.push(tag);
    encode(a, defs, out);
}

/// [`feed_content_key`] of a slot reading `var`.
pub(crate) fn feed_var(h: &mut KeyHasher, defs: Definitions<'_, '_>, var: VarId) {
    feed_content_key(h, defs, &slot_reader(var));
}

/// A lone reader of `var`, as a slot's identity reads it: the encoding
/// writes no reader's dimension, so the one it is built at is moot.
fn slot_reader(var: VarId) -> Expr {
    Expr::var(var, crate::expr::Dimension::Scalar)
}

/// **The expression half of a slot's identity, written into a content
/// key.** Scope-free on purpose: the key is compared against a prior
/// evaluation of the same document, so the table is a constant of the
/// comparison and the expression is the input that can move.
pub(crate) fn feed_content_key(h: &mut KeyHasher, defs: Definitions<'_, '_>, expr: &Expr) {
    let mut bytes = Vec::new();
    encode(expr, defs, &mut bytes);
    h.write_bytes(&bytes);
}

/// **Whether a VERB SCALAR's expression reaches the value at all** —
/// the rule for which of a verb node's own slots [`feed_content_key`]
/// applies to. A parameter whose declared flow names no field (the
/// chamfer's setback) puts nothing but its number into the body, so
/// its spelling is not an input to the value and re-spelling it is
/// rightly a memo hit.
///
/// This answers for slots a verb owns and for nothing else. An
/// expression an OPERAND holds — a profile's carrier radius — is no
/// verb's parameter and has no `ScalarParam` to ask about;
/// [`operand_flow_bearing`] is that question.
pub(crate) fn flow_bearing(param: ScalarParam) -> bool {
    param
        .verb()
        .param_flow()
        .iter()
        .any(|row| row.source == FlowSource::Param(param) && !row.fields.is_empty())
}

/// **Whether an OPERAND-carried scalar reaches a value** — the same
/// rule as [`flow_bearing`], asked of a source no single verb owns.
///
/// A profile's carrier radius is not a parameter of any verb: the
/// profile node holds it, and a sweep downstream lands it in the walls
/// it mints. So the question "does re-spelling it change what a body
/// carries?" cannot be answered from one verb's declaration — it is
/// answered by whether ANY migrated verb declares that source into a
/// field, which is what this reads. The moment no verb does, a
/// profile's radius is a number again and its spelling stops being an
/// input to any key.
pub(crate) fn operand_flow_bearing(source: FlowSource) -> bool {
    verbs::VerbKind::ALL.iter().any(|kind| {
        kind.param_flow()
            .iter()
            .any(|row| row.source == source && !row.fields.is_empty())
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    use crate::expr::Dimension;

    /// The rational constant `num / den`.
    fn ratio(num: i64, den: u64) -> Expr {
        Expr::ratio(num, den).expect("a constant in range")
    }

    /// A table of free variables only: every reader lowers as its id.
    fn free(_: VarId) -> Option<&'static Expr> {
        None
    }

    /// A reader of the length variable `id`.
    fn p(id: u64) -> Expr {
        Expr::var(VarId::new(0, id), Dimension::Length)
    }

    /// The expression's encoding, as the key writes it.
    fn encoded(expr: &Expr) -> Vec<u8> {
        let mut bytes = Vec::new();
        encode(expr, &free, &mut bytes);
        bytes
    }

    /// How many operands follow a tag, or what fixed-width payload a
    /// leaf carries — the decoder's own reading of the alphabet.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum Shape {
        /// A leaf with a fixed payload width in bytes.
        Leaf(usize),
        Unary,
        Binary,
    }

    /// EVERY tag constant, by name, with the shape the encoder gives it.
    const ALPHABET: &[(&str, u8, Shape)] = &[
        ("T_VAR", T_VAR, Shape::Leaf(12)),
        ("T_RATIO", T_RATIO, Shape::Leaf(16)),
        ("T_INTEGER", T_INTEGER, Shape::Leaf(8)),
        ("T_TURN", T_TURN, Shape::Leaf(0)),
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
    ];

    /// **Retired tags**: the bytes earlier encodings gave a leaf the
    /// stored expression no longer has (a float literal and a count
    /// literal), and the two scope tags the kernel's parameter tokens
    /// opened with. Nothing decodes them; they are kept out of reuse so
    /// that no byte means two things across builds a reader might
    /// compare.
    const RETIRED: &[(&str, u8)] = &[
        ("T_LITERAL", 0x01),
        ("T_COUNT_LITERAL", 0x02),
        ("T_SCOPE_ROOT", 0x20),
        ("T_SCOPE_PART", 0x21),
    ];

    /// **Every tag is distinct** — the injectivity argument's first
    /// premise, executed over the constants by NAME so that two
    /// constants sharing a value (`T_SUB = T_ADD`) red here rather than
    /// silently encoding `r + t` and `r - t` alike — and none reuses a
    /// retired byte.
    #[test]
    fn every_tag_is_distinct() {
        for (i, (name, tag, _)) in ALPHABET.iter().enumerate() {
            for (other, tag2, _) in &ALPHABET[..i] {
                assert_ne!(
                    tag, tag2,
                    "{name} and {other} share the tag byte {tag:#04x}"
                );
            }
            for (retired, byte) in RETIRED {
                assert_ne!(tag, byte, "{name} reuses the retired {retired}'s byte");
            }
        }
    }

    /// **The alphabet is the whole encoder**: one row per stored AST
    /// arm. The match is exhaustive, so a new expression
    /// node fails this file until it is visited, and visiting it means
    /// naming its row.
    #[test]
    fn the_alphabet_covers_the_encoder() {
        let arms = match ExprKind::Neg(Box::new(ratio(0, 1))) {
            ExprKind::Ratio(_)
            | ExprKind::Integer(_)
            | ExprKind::Turn
            | ExprKind::Var(_)
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
            | ExprKind::CountToScalar(_) => 16,
            // The stored form adds no leaf of its own.
            ExprKind::Leaf(own) => match own {},
        };
        assert_eq!(
            ALPHABET.len(),
            arms,
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
            Shape::Unary => parse_node(bytes, at + 1),
            Shape::Binary => {
                let mid = parse_node(bytes, at + 1)?;
                parse_node(bytes, mid)
            }
        }
    }

    /// An encoding parses as exactly one expression.
    fn parses_whole(expr: &Expr) -> bool {
        let bytes = encoded(expr);
        parse_node(&bytes, 0) == Some(bytes.len())
    }

    /// A small expression family: every leaf kind, every operator, two
    /// levels deep, plus ids that differ only in high bytes and
    /// constants that differ only in sign or denominator. Built
    /// through the dimension-checked doors, so only well-typed
    /// combinations enter (a length plus an angle is not an expression).
    fn family() -> Vec<Expr> {
        let count = Expr::integer(3);
        let leaves = vec![
            p(1),
            p(2),
            p(3),
            p(1 << 32),
            p(u64::MAX),
            Expr::var(VarId::new(0, 6), Dimension::Angle),
            ratio(0, 1),
            ratio(1, 2),
            ratio(-1, 2),
            ratio(1, 3),
            Expr::turn(),
            count.clone(),
            Expr::integer(-3),
            Expr::count_to_scalar(count).unwrap(),
        ];
        let mut out = leaves.clone();
        for x in &leaves {
            out.extend(Expr::neg(x.clone()).ok());
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
        let angle = Expr::var(VarId::new(0, 7), Dimension::Angle);
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

    /// **Fixed arity per tag, executed**: every encoding in the family
    /// parses back as one expression consuming exactly the bytes
    /// written, by a decoder that reads the arity off the tag alone. An
    /// encoder writing a different operand count than the alphabet
    /// claims for a tag cannot pass this.
    #[test]
    fn every_encoding_parses_back_by_its_tags_alone() {
        for expr in family() {
            assert!(parses_whole(&expr), "{expr:?} does not parse back whole");
        }
    }

    /// **Encoding equality is `bit_eq`, over the whole family**: the
    /// injectivity claim executed pairwise rather than asserted. Both
    /// directions — two expressions equal by bits share an encoding, and
    /// two that differ do not — with ids that differ only in their high
    /// bytes, operand order and the sign of zero all inside the family.
    #[test]
    fn encoding_equality_is_expression_equality() {
        let family = family();
        let encodings: Vec<Vec<u8>> = family.iter().map(encoded).collect();
        for (i, x) in family.iter().enumerate() {
            for (j, y) in family.iter().enumerate().skip(i) {
                assert_eq!(
                    encodings[i] == encodings[j],
                    x.bit_eq(y),
                    "encoding equality disagrees with bit_eq for {x:?} and {y:?}"
                );
            }
        }
    }

    /// **A constant is its value**: `2/4` and `1/2` are one constant
    /// and encode alike, where two typed values are two variables.
    #[test]
    fn equal_constants_encode_alike() {
        let (a, b) = (ratio(2, 4), ratio(1, 2));
        assert!(a.bit_eq(&b));
        assert_eq!(encoded(&a), encoded(&b));
        assert_ne!(encoded(&ratio(1, 2)), encoded(&ratio(1, 3)));
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
                .filter(|row| row.source == FlowSource::Param(param))
                .map(|row| row.fields.len())
                .sum();
            assert_eq!(flow_bearing(param), fields > 0, "{param:?}");
        }
        assert!(flow_bearing(ScalarParam::FilletRadius));
        assert!(!flow_bearing(ScalarParam::ChamferDistance));
        // The sweeps' two extents are the same case as the chamfer's
        // setback and are asserted beside it rather than left to the
        // loop: an extent that started feeding the key would be a memo
        // invalidation nobody asked for.
        assert!(!flow_bearing(ScalarParam::ExtrudeDistance));
        assert!(!flow_bearing(ScalarParam::RevolveAngle));
        // The operand-carried source IS flow-bearing, which is what
        // puts a carrier loop's spelling into the profile's key.
        assert!(operand_flow_bearing(FlowSource::ProfileEdge(
            verbs::EdgeScalar::Radius
        )));
    }
}
