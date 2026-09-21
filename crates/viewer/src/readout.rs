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
//! # Why this is a module and not a format string per site
//!
//! **The population is every render of a number this crate writes**,
//! which is not a list anyone can hold: the sweep that produces it is
//! *every `{…}` in the crate whose argument is a quantity*, and it
//! reaches both the sentences the chrome composes and the fields a
//! person edits. The read-only half holds a
//! [`crate::scene::DisplayTolerance`] in two places and a bare `f64` in
//! a display unit in the others, so the rule cannot be a method on that
//! type without being written twice — once as the method and once, by
//! hand, wherever the type is absent. It is one function over the
//! value, [`crate::scene::DisplayTolerance::render_mm`] is the δ-facing
//! door onto it (the millimetre conversion, and nothing else), and
//! `crate::widgets::number_text` is the fields' door.
//!
//! # What it is not
//!
//! It is not exact wherever the character bound is what ends the
//! search: four significant figures is what ten characters buy, and a δ
//! the triangle budget chose is `constant / TRIANGLE_BUDGET` —
//! seventeen. The one band where it IS exact is the top of `f64`, and
//! that is the bound giving way rather than the rule: four figures round
//! out of the type there, and a text that reads as infinity names no
//! value at all.
//!
//! **It is reached from a commit path and is not one.** An
//! `egui::DragValue` seeds its keyboard edit with the text it last
//! showed and writes the parse back when it loses focus, so what a
//! field renders is what clicking into it and away again commits —
//! which is why `crate::widgets::number_text` exists and why
//! [`REL_TOLERANCE`] bounds that commit as well as that render. The
//! number a value moves to on purpose is one a user types, never one
//! the chrome echoed at them.

/// How far [`number`]'s text may read from the value it renders, as a
/// fraction of that value.
///
/// **Not a taste: it is the four-figure scientific form's own worst
/// case.** Four significant figures can misread the value they render
/// by half a unit in the fourth — 5·10⁻⁴ of it — so a decimal spelling
/// is preferred exactly while it is no less truthful than the form that
/// would replace it. It is the same test that chooses between the two
/// scientific spellings in [`scientific`], which is what keeps the four
/// figures wherever they are honest.
///
/// That is also why it is a property of this module rather than
/// something a caller passes: the tolerance and the fallback spelling
/// are two halves of one choice, and a caller free to name the first
/// would be free to name one the second cannot honour.
pub const REL_TOLERANCE: f64 = 5.0e-4;

/// The longest decimal spelling [`number`] will search, in characters.
///
/// **The four-figure scientific arm's own worst case over a positive
/// value**, which is `f64`'s smallest subnormal: a four-figure mantissa,
/// `e`, a sign and three exponent digits — `4.941e-324`. The decimal arm
/// is held to the same bound, so ten characters show every render but
/// one: the band at the top of `f64`, where four figures round out of
/// the type and [`scientific`] spells the value exactly instead —
/// twenty-two characters, twenty-three with a sign.
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
///
/// **The rule holds at the top of the type, and [`MAX_CHARS`] is what
/// gives way there.** Four significant figures round, and from
/// `1.7975000000000001e308` up they round out of `f64` — `1.798e308` is
/// not a spelling of any finite number, it reads back as infinity. So
/// that band, and only that band, is rendered by the exact scientific
/// spelling at twenty-two characters ([`scientific`]).
/// `the_top_of_the_type_is_spelled_exactly` is the row, and
/// `every_value_reads_back_as_itself` no longer stops short of it.
///
/// **The bound ends the SEARCH; it does not license a text that names
/// another value.** That is [`reads_back`]'s own rule — a wide text that
/// names this value is not improved by a narrow one that does not — and
/// a last resort exempt from it would be the one place the module's rule
/// is false, at the one magnitude no caller can sanity-check. What it
/// costs is one widget: `crate::pane::view`'s `FIELD_WIDTH` is sized for
/// [`MAX_CHARS`], so a δ in that band is shown clipped. Clipped is not
/// the same failure as misread here, because the field is a draft the
/// user commits: it holds the whole text and hands the whole text back,
/// where `1.798e308` handed back infinity.
pub fn number(value: f64) -> String {
    // Decimal counts past the character bound cannot fit whatever they
    // spell, so the bound is what ends the search; the range only has
    // to reach past the last count that could.
    (0..=MAX_CHARS)
        .map(|decimals| format!("{value:.decimals$}"))
        .find(|spelling| spelling.chars().count() <= MAX_CHARS && reads_back(spelling, value))
        .unwrap_or_else(|| scientific(value))
}

/// `value` in scientific notation: four significant figures where that
/// reads back, and the exact spelling where it does not.
///
/// **The four-figure form is the one [`MAX_CHARS`] is the width of**, and
/// it carries every value a decimal spelling cannot — its worst case is
/// [`REL_TOLERANCE`] by construction, so it reads back everywhere its
/// rounding stays inside `f64`. The exact arm is for the one place that
/// rounding leaves the type: the band up to `f64::MAX`, where four
/// figures round to `1.798e308` and that text is infinity.
///
/// **A non-finite value takes the exact arm and is unchanged by it.**
/// Nothing reads back as `NaN`, so the test fails and the exact spelling
/// is asked for — and `{:e}` writes `NaN` and `inf` exactly as `{:.3e}`
/// did. The arm is reached, not the behaviour.
fn scientific(value: f64) -> String {
    let rounded = format!("{value:.3e}");
    if reads_back(&rounded, value) {
        rounded
    } else {
        format!("{value:e}")
    }
}

/// Whether `spelling` reads within [`REL_TOLERANCE`] of `value`.
///
/// The tolerance is relative to the MAGNITUDE, so the test says the
/// same thing on both sides of zero: a rendered number is judged by how
/// far it reads from the value, and a sign is not a distance.
///
/// **Width is no part of reading back, which is why it is no part of
/// this.** [`number`] is bounded by [`MAX_CHARS`] because it searches
/// every precision and something has to end the search; a caller
/// judging a spelling SOMEONE ELSE chose is asking only whether that
/// text names this value, and a wide text that does is not improved by
/// replacing it with a narrow one that does not.
pub(crate) fn reads_back(spelling: &str, value: f64) -> bool {
    spelling
        .parse::<f64>()
        .is_ok_and(|read| reads_as(read, value))
}

/// Whether a number `read` back off a render names `value` — the
/// numeric half of [`reads_back`], for a caller holding the number
/// rather than the text.
///
/// [`crate::props::typed_edit`] is that caller: a field hands its
/// parse back as an `f64`, and asking whether that number is the one
/// the field was already showing is the same question [`reads_back`]
/// asks of the text, over the same bound. Spelled here so the bound is
/// applied in one place; a second comparison against
/// [`REL_TOLERANCE`] would be free to disagree with the render it is
/// judging.
pub(crate) fn reads_as(read: f64, value: f64) -> bool {
    (read - value).abs() <= REL_TOLERANCE * value.abs()
}

#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]
    #![allow(clippy::panic)]

    use super::{MAX_CHARS, REL_TOLERANCE, number};

    /// What the rule claims, checked as a property rather than a table:
    /// a render reads back within the render's own accuracy — and
    /// therefore a value that is not zero never renders as zero, which
    /// is the whole defect this module closes.
    ///
    /// **The character bound is asserted separately**, by
    /// [`fits_and_reads_back`], because it is not part of the rule: it
    /// ends the decimal search, and the band at the top of the type is
    /// spelled exactly and is wider.
    fn reads_back(value: f64) {
        let text = number(value);
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

    /// [`reads_back`], and inside the character bound — which is every
    /// value below the band `the_top_of_the_type_is_spelled_exactly`
    /// covers.
    fn fits_and_reads_back(value: f64) {
        let text = number(value);
        assert!(
            text.trim_start_matches('-').chars().count() <= MAX_CHARS,
            "{value} renders as {text}, past the {MAX_CHARS} character bound"
        );
        reads_back(value);
    }

    /// The whole range a chrome number can come from, both signs: a
    /// decade below the kernel's finest length to well above the
    /// coarsest a camera band reaches.
    #[test]
    fn every_value_reads_back_as_itself() {
        let mut sampled: u32 = 0;
        let mut value = 1.0e-12_f64;
        while value < 1.0e6 {
            fits_and_reads_back(value);
            fits_and_reads_back(-value);
            sampled += 1;
            value *= 1.01;
        }
        assert!(sampled > 2_000, "the sweep covered only {sampled} values");
        // And both ends of the type, which a geometric grid does not
        // reach. The top is in the band the character bound does not
        // cover, so it is asserted by the rule alone and its width has
        // its own row below.
        for end in [5.0e-324, f64::MIN_POSITIVE] {
            fits_and_reads_back(end);
            fits_and_reads_back(-end);
        }
        reads_back(f64::MAX);
        reads_back(-f64::MAX);
    }

    /// **The one band where the render passes [`MAX_CHARS`]**, and why
    /// it does: four significant figures round out of `f64` here, so
    /// the only text that names the value is the exact one.
    ///
    /// **It is a band and not a value.** Every `f64` from
    /// `1.7975000000000001e308` to `f64::MAX` rounds to `1.798e308`,
    /// which is infinity — about 9.7·10¹¹ of them, each sign. The row
    /// samples across it rather than pinning an end, and pins the edge
    /// by stepping one `f64` below it, where four figures still read
    /// back and the render is nine characters again.
    #[test]
    fn the_top_of_the_type_is_spelled_exactly() {
        assert_eq!(number(f64::MAX), "1.7976931348623157e308");
        assert_eq!(number(-f64::MAX), "-1.7976931348623157e308");
        assert_eq!(
            number(f64::MAX).parse::<f64>(),
            Ok(f64::MAX),
            "the exact spelling is the arm because it is the one that reads back"
        );
        assert!(
            "1.798e308".parse::<f64>().is_ok_and(f64::is_infinite),
            "and the four-figure text it replaces names no finite value at all"
        );

        // The edge, from both sides. One `f64` below it the four-figure
        // arm still reads back, so nothing wider is spent there.
        let low = 1.797_500_000_000_000_1e308_f64;
        let below = f64::from_bits(low.to_bits() - 1);
        assert_eq!(number(below), "1.797e308");
        fits_and_reads_back(below);
        assert_eq!(number(1.0e308), "1.000e308");

        // And across the band, both signs. The character bound is not
        // asserted: this is the band it does not cover.
        let (low_bits, top_bits) = (low.to_bits(), f64::MAX.to_bits());
        let stride = (top_bits - low_bits) / 512;
        assert!(stride > 0, "the band is wider than the grid over it");
        for step in 0..=512 {
            let value = f64::from_bits(low_bits + stride * step);
            reads_back(value);
            reads_back(-value);
        }
        reads_back(low);
    }

    /// **Nothing below the band renders differently than it did**, which
    /// is what makes the exact arm a repair rather than a restyle: the
    /// four-figure arm still wins wherever it reads back, and that is
    /// everywhere the millimetre and metre magnitudes of this chrome
    /// live.
    #[test]
    fn the_four_figure_arm_still_carries_everything_below_the_band() {
        let mut value = 1.0e-320_f64;
        let mut scientific: u32 = 0;
        while value < 1.0e300 {
            let text = number(value);
            if text.contains('e') {
                assert!(
                    text.chars().count() <= MAX_CHARS,
                    "{value} renders as {text}, which is the exact arm below the band"
                );
                scientific += 1;
            }
            fits_and_reads_back(value);
            value *= 1.05;
        }
        assert!(
            scientific > 1_000,
            "only {scientific} values took a scientific arm"
        );
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
