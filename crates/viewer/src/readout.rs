//! A number as text a person reads.
//!
//! Module kind: **vocabulary** (`crates/viewer/README.md`, Module
//! boundaries) — one pure function over an `f64` and the two constants
//! that shape it.
//!
//! # The rule
//!
//! **A rendered number reads back as the value it renders.** A fixed
//! precision does not: `{:.3}` over millimetres carries a length under
//! half a micrometre as `0.000`, and `{:.1}` carries a camera distance
//! under 50 µm the same way — a number the value cannot be, printed
//! with the confidence of one it can. The same spec over-renders at the
//! other end, where `1024.0000` claims four figures a probe never
//! established.
//!
//! [`number`] answers both with one rule: the shortest decimal spelling
//! that reads back as this value, and a scientific one when no decimal
//! spelling does. Nothing here is a threshold, so there is no magnitude
//! to go stale against a format.
//!
//! # Why this is a module and not four format strings
//!
//! Four places in the chrome render a length: the δ badge and the
//! budget's status line ([`crate::frame::delta_badge`],
//! [`crate::scene::FittedDelta::wording`]), the range probe's sentence
//! ([`crate::bounds::Bounds::wording`]) and the view pane's camera
//! readout. Two of them hold a [`crate::scene::DisplayTolerance`] and
//! two hold a bare `f64` in some display unit, so the rule cannot be a
//! method on that type without being written twice — once as the
//! method and once, by hand, wherever the type is absent. It is one
//! function over the value, and
//! [`crate::scene::DisplayTolerance::render_mm`] is the δ-facing door
//! onto it: the millimetre conversion, and nothing else.
//!
//! # What it is not
//!
//! It is not exact, and it is not a commit path. Four significant
//! figures is what the character bound buys, and a δ the triangle
//! budget chose is `constant / TRIANGLE_BUDGET` — seventeen. The number
//! a value moves to is one a user types, never one the chrome echoed at
//! them.

/// How far [`number`]'s text may read from the value it renders, as a
/// fraction of that value.
///
/// **Not a taste: it is the scientific form's own worst case.** Four
/// significant figures can misread the value they render by half a unit
/// in the fourth — 5·10⁻⁴ of it — so a decimal spelling is preferred
/// exactly while it is no less truthful than the form that would
/// replace it.
///
/// That is also why it is a property of this module rather than
/// something a caller passes: the tolerance and the fallback spelling
/// are two halves of one choice, and a caller free to name the first
/// would be free to name one the second cannot honour.
pub const REL_TOLERANCE: f64 = 5.0e-4;

/// The longest decimal spelling [`number`] will search, in characters.
///
/// **The scientific arm's own worst case over a positive value**, which
/// is `f64`'s smallest subnormal: a four-figure mantissa, `e`, a sign
/// and three exponent digits — `4.941e-324`. The decimal arm is held to
/// the same bound, so ten characters can show every positive value this
/// renders and a negative one spends one more on its sign.
///
/// Like [`REL_TOLERANCE`] this is the rule's own number and not a field
/// width. It is what ENDS the search: a subnormal needs three hundred
/// decimals to spell and the scientific arm is there to carry it, so
/// the loop has to stop somewhere, and it stops at the width past which
/// a decimal spelling could not be shown anyway. A box narrower than
/// this clips, and a clipped render reads as a different value — that
/// is the box's number to meet, not this one's to lower.
pub const MAX_CHARS: usize = 10;

/// `value` as text a person reads.
///
/// **The shortest decimal spelling that reads back as this value, and a
/// scientific one when no decimal spelling does.** A length wants to
/// read as a decimal and does wherever it can (`0.05`, `0.0016`,
/// `0.0003746`); below that a decimal spelling either misreads the
/// value or does not fit, and the scientific form carries it
/// (`1.000e-9`).
///
/// **The choice is made on the property, not on a magnitude.** A
/// spelling is used when it fits [`MAX_CHARS`] and reads back within
/// [`REL_TOLERANCE`] of this value. So a value that is not zero never
/// renders as zero — a text reading `0.000` is a hundred percent away
/// from the value it claims to be, which is the one thing this bound
/// refuses first — and a value that IS zero renders as `0`, since zero
/// reads back as itself.
///
/// **A non-finite value has no reading and gets the fallback.** No
/// spelling of `NaN` reads back as `NaN` — nothing does — so the search
/// exhausts and the scientific arm prints `NaN` or `inf`. Nothing in
/// the chrome hands this one; the behaviour is stated because it is
/// what the rule produces rather than a case it handles.
pub fn number(value: f64) -> String {
    // Decimal counts past the character bound cannot fit whatever they
    // spell, so the bound is what ends the search; the range only has
    // to reach past the last count that could.
    (0..=MAX_CHARS)
        .map(|decimals| format!("{value:.decimals$}"))
        .find(|spelling| reads_back_as(spelling, value))
        .unwrap_or_else(|| format!("{value:.3e}"))
}

/// Whether `spelling` fits [`MAX_CHARS`] and reads within
/// [`REL_TOLERANCE`] of `value`.
///
/// The tolerance is relative to the MAGNITUDE, so the test says the
/// same thing on both sides of zero: a rendered number is judged by how
/// far it reads from the value, and a sign is not a distance.
fn reads_back_as(spelling: &str, value: f64) -> bool {
    spelling.chars().count() <= MAX_CHARS
        && spelling
            .parse::<f64>()
            .is_ok_and(|read| (read - value).abs() <= REL_TOLERANCE * value.abs())
}

#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]

    use super::{MAX_CHARS, REL_TOLERANCE, number};

    /// What the rule claims, checked as a property rather than a table:
    /// a render fits the bound and reads back within the render's own
    /// accuracy — and therefore a value that is not zero never renders
    /// as zero, which is the whole defect this module closes.
    fn reads_back(value: f64) {
        let text = number(value);
        assert!(
            text.trim_start_matches('-').chars().count() <= MAX_CHARS,
            "{value} renders as {text}, past the {MAX_CHARS} character bound"
        );
        let read: f64 = text
            .parse()
            .unwrap_or_else(|error| panic!("{value} renders as {text}, not a number: {error}"));
        assert!(
            (read - value).abs() <= REL_TOLERANCE * value.abs(),
            "{value} renders as {text}, further from it than the render's own accuracy"
        );
        assert!(
            value == 0.0 || read != 0.0,
            "{value} renders as {text}, which reads as a zero it is not"
        );
    }

    /// The whole range a chrome number can come from, both signs: a
    /// decade below the kernel's finest length to well above the
    /// coarsest a camera band reaches.
    #[test]
    fn every_value_reads_back_as_itself() {
        let mut sampled: u32 = 0;
        let mut value = 1.0e-12_f64;
        while value < 1.0e6 {
            reads_back(value);
            reads_back(-value);
            sampled += 1;
            value *= 1.01;
        }
        assert!(sampled > 2_000, "the sweep covered only {sampled} values");
        // And the ends of the type, which a geometric grid does not
        // reach.
        for end in [5.0e-324, f64::MIN_POSITIVE, f64::MAX] {
            reads_back(end);
            reads_back(-end);
        }
    }

    /// **Zero is the value the δ door's own predicate could not have
    /// carried**, and it is why this rule is not `render_mm` with the δ
    /// taken out: a probed bound may BE zero, and zero reads back as
    /// itself.
    #[test]
    fn zero_renders_as_zero() {
        assert_eq!(number(0.0), "0");
        reads_back(0.0);
    }

    /// A sign is not a distance: the accuracy bound is relative to the
    /// magnitude, so a negative value is rendered by the same rule.
    #[test]
    fn a_negative_value_is_rendered_by_the_same_rule() {
        assert_eq!(number(-0.05), "-0.05");
        assert_eq!(number(-4.0e-5), "-0.00004");
    }

    /// **The rule cuts false precision as well as false zeroes.** A
    /// fixed spec pays for its finest case everywhere; the shortest
    /// spelling that reads back pays per value.
    #[test]
    fn a_value_with_a_short_spelling_gets_it() {
        assert_eq!(number(1024.0), "1024");
        assert_eq!(number(0.05), "0.05");
        assert_eq!(number(1500.0), "1500");
    }

    /// Below what ten characters of decimal can spell, the scientific
    /// arm carries it — the arm whose accuracy [`REL_TOLERANCE`] names.
    #[test]
    fn a_value_no_decimal_spelling_reaches_goes_scientific() {
        assert_eq!(number(1.0e-9), "1.000e-9");
        assert_eq!(number(4.0e-9), "4.000e-9");
        // And just above it, where a decimal spelling still fits — the
        // boundary is the BOUND, not a magnitude anybody wrote down.
        assert_eq!(number(4.0e-7), "0.0000004");
        // The widest text the positive arm returns, which is what
        // `MAX_CHARS` is the width of.
        assert_eq!(number(5.0e-324), "4.941e-324");
        assert_eq!(number(5.0e-324).chars().count(), MAX_CHARS);
    }
}
