//! A number as text a person reads.
//!
//! Module kind: **vocabulary** (`crates/viewer/README.md`, Module
//! boundaries) — one pure function over an `f64`, the grid it renders
//! on and the two constants that shape it.
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
//! # What "reads back" is measured against
//!
//! [`tolerance`] is the grid, and it is **a difference the kernel can
//! decide**, not a width a box has: an absolute cap one decade below ε
//! met with a relative arm, the finer of the two winning. The same
//! grid `crates/profile/src/path.rs`'s `num` renders a refusal
//! sentence's scalars on, for the same reason — ε is a LENGTH
//! (`docs/DESIGN.md` D4 ¶1), so a purely relative rule crosses it and
//! is coarser above the crossing, and two lengths the kernel certifies
//! as DIFFERENT would then render as one number. Two chrome sentences
//! quoting a document cannot say the same thing about values the
//! document holds apart.
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
//! It is not the exact value wherever a shorter spelling lands inside
//! [`tolerance`] — that is the point of it, and it is what cuts
//! `1024.0000` back to `1024`. What it no longer is is coarse enough
//! for the difference between two values the kernel decides between:
//! the grid is finer than ε everywhere, so a δ the triangle budget
//! chose shows the figures that distinguish it from the next δ rather
//! than four.
//!
//! **It is reached from a commit path and is not one.** An
//! `egui::DragValue` seeds its keyboard edit with the text it last
//! showed and writes the parse back when it loses focus, so what a
//! field renders is what clicking into it and away again USED to
//! commit — which is why `crate::widgets::number_text` exists. That
//! coupling is cut: `crate::widgets::number_field` compares the
//! parsed text against the text its own formatter returned
//! (`crate::props::echoed`) and a field's own render commits
//! nothing, so [`tolerance`] bounds the render alone and the
//! question of what accuracy a RENDER owes is answered here on its
//! own terms. The number a value moves to on purpose is one a user
//! types, never one the chrome echoed at them — now enforced rather
//! than bounded.
//!
//! The one field where the old sentence still holds is a bare
//! `egui::DragValue`, which takes this render from the context and
//! has no parser to take the veto from
//! (`crate::widgets::install_number_formatter`, and
//! `work/vgeom/a-bare-field-still-commits-its-own-render.md`).

use pncad::geom_core::tolerance::DEFAULT_EPS;

/// The absolute arm of [`tolerance`]: one decade below ε.
///
/// **ε is a LENGTH** (`docs/DESIGN.md` D4 ¶1, [`DEFAULT_EPS`]), so a
/// grid stated as a fraction of the value crosses it at one unit and is
/// coarser above the crossing — at a kilometre a relative 10⁻⁹ is a
/// micron, and two lengths the kernel certifies as different render as
/// one number. Capping the grid a decade below ε is what makes a
/// difference the kernel can DECIDE always a difference the chrome
/// SPELLS.
///
/// **It is the compile-time [`DEFAULT_EPS`] and never the run's live
/// `Tolerance::eps()`**, which is the same choice
/// `crates/profile/src/path.rs`'s `num` states at its own site: a live
/// ε would make one rendered value spell three ways across the ε rows
/// CI gates, so the grid is a display choice stated once against the
/// ratified default.
///
/// **The unit it is stated in is the canonical one, and applying it to
/// a WRITTEN number is conservative rather than approximate.** A value
/// reaching [`number`] has been divided by its notation's factor
/// (`crate::props::in_written`), and every length row of
/// `quantity::UNITS` has a factor at most one, so the written number is
/// never smaller than the canonical one and an absolute grid on it
/// resolves a canonical difference at least this fine.
/// `the_grid_is_never_coarser_than_epsilon_in_any_written_unit` reads
/// the table rather than restating it.
const EPS_CAP: f64 = DEFAULT_EPS * 0.1;

/// The relative arm of [`tolerance`], as a fraction of the value.
///
/// **Not a taste: it is the four-figure scientific form's own worst
/// case.** Four significant figures can misread the value they render
/// by half a unit in the fourth — 5·10⁻⁴ of it — so a spelling is
/// preferred exactly while it is no less truthful than the form that
/// would replace it. That derivation is unchanged and was never the
/// defect; what was missing beside it is [`EPS_CAP`], and a rule with
/// only this arm was coarser than ε above 2·10⁻⁷ of a display unit.
///
/// **It governs only BELOW the crossing, and ε does not bind there.**
/// The arms cross at `EPS_CAP / REL_TOLERANCE`; below it this arm is
/// the finer of the two by construction, so it never decides whether
/// the render can separate two values the kernel separates — the cap
/// has already guaranteed that. What it decides is the figures a value
/// too small for the cap to resolve is spelled to, and **four is the
/// number ε justifies there**: a length below the crossing is below ε
/// itself in every notation the chrome writes one in, so figures past
/// the fourth would be precision no probe established — the defect
/// [`number`] exists to cut, arriving by the other door.
///
/// **A FLOOR under the tolerance is the mirror defect** and is
/// precisely what a `min` cannot become: it would round every value
/// below the floor to the floor's own zero, which is the one thing
/// [`number`] refuses first.
///
/// That is also why it is a property of this module rather than
/// something a caller passes: the tolerance and the fallback spelling
/// are two halves of one choice, and a caller free to name the first
/// would be free to name one the second cannot honour.
pub const REL_TOLERANCE: f64 = 5.0e-4;

/// How far [`number`]'s text may read from the value it renders.
///
/// **The finer of [`EPS_CAP`] and [`REL_TOLERANCE`] of the value**,
/// which is a `min` and not a `max`: each arm is the coarser one
/// somewhere, and the coarser arm is never the one that may govern.
/// Above `EPS_CAP / REL_TOLERANCE` — 2·10⁻⁷ of a display unit — the cap
/// governs and the render separates everything ε does; below it the
/// relative arm governs, and everything there is smaller than ε
/// anyway.
///
/// `tolerance(0.0)` is `0.0` — zero has no figures to keep and only its
/// exact spelling reads back as it, which is `0`.
///
/// # This is the SECOND spelling of one rule, and it is the deletable
/// one
///
/// `crates/profile/src/path.rs`'s `num` computes the same grid for the
/// scalars a kernel refusal sentence quotes. **Where that rule lives
/// once a second crate wants it is
/// `work/props/patherror-display-renders-float-noise.md`'s question**,
/// not this module's, and this function is what a consolidation
/// deletes: one function, value in and tolerance out, called from
/// [`reads_back`] alone, so nothing here spreads the two arms across
/// the search that uses them.
///
/// **The three places the two rules deliberately differ, marked here so
/// a lane merging them can tell a decision from drift:**
///
/// - **Notation.** `num` follows the `Debug` form's own choice — fixed
///   where `{:?}` is fixed, exponential where it is exponential — and
///   only ever shortens that spelling's mantissa. [`number`] chooses:
///   the shortest decimal that reads back, and [`scientific`] when no
///   decimal does. The reason is the caller. `num` renders a payload
///   inside a sentence about geometry a person did not choose the
///   magnitude of; this renders a value in a notation the person chose,
///   and a length they wrote in millimetres wants to read back in
///   millimetres.
/// - **The relative arm.** Different on purpose: `num`'s is `1e-9` and
///   this one is [`REL_TOLERANCE`], 5·10⁻⁴. `num` has no character
///   bound and renders a payload at a magnitude nobody chose, so it can
///   afford ten figures below the crossing; this arm is the width of
///   its own fallback notation, and below the crossing a length is
///   below ε in every notation the chrome writes one in, where a fifth
///   figure would be precision no probe established. **Both are
///   legibility choices under the same cap** — neither is derived from
///   ε, because the cap has already answered ε — so a consolidation
///   takes the arm as a parameter or takes the coarser one knowingly.
///   It must not take one by accident.
/// - **The character bound.** `num` has none — a refusal sentence is as
///   wide as it needs to be. [`number`] has [`MAX_CHARS`], because its
///   texts go in boxes and a clipped render reads as a different value.
///   That bound is what makes the two searches different lengths of
///   loop and is the one difference a merge cannot simply drop.
fn tolerance(value: f64) -> f64 {
    EPS_CAP.min(value.abs() * REL_TOLERANCE)
}

/// The decimal places [`number`] will search.
///
/// **Where a decimal spelling reaches [`EPS_CAP`]**: rounding to `d`
/// places misreads the value by at most half a unit in the last, so the
/// first `d` whose half-unit is inside the cap is the last one the cap
/// can ask for. Derived from the cap rather than chosen, so a `d` that
/// went stale against a moved ε is not a state this constant has.
///
/// **Past it, more places only serve [`REL_TOLERANCE`]** — the arm that
/// governs below the crossing, where a decimal spelling is a run of
/// leading zeros and [`scientific`] is the notation that reads. So the
/// decimal arm stops where the cap stops, and the small end is the
/// scientific arm's as it has always been.
const DECIMALS: usize = decimals_reaching(EPS_CAP);

/// [`DECIMALS`], as arithmetic rather than as a number to keep in step.
const fn decimals_reaching(cap: f64) -> usize {
    let mut places = 0;
    let mut half_unit = 0.5;
    while half_unit > cap {
        half_unit /= 10.0;
        places += 1;
    }
    places
}

/// The longest spelling [`number`] returns, in characters.
///
/// **[`scientific`]'s own worst case over a positive value**, which is
/// the top of `f64`: seventeen significant figures, `e` and three
/// exponent digits — `1.7976931348623157e308`, twenty-two characters.
/// Nothing wider is reachable. Where the exponent needs three digits at
/// the small end, [`tolerance`]'s relative arm governs and eleven
/// significant figures are the most it can ask for; where seventeen are
/// asked for, [`EPS_CAP`] governs and the exponent is at most `e308`.
///
/// The decimal arm is held to the same bound, which is what keeps a
/// huge magnitude out of a three-hundred-digit spelling that reads back
/// perfectly well. So this is the width of EVERY render, with no band
/// excepted: a box that meets it shows every text this module returns.
/// A box narrower than this clips, and a clipped render reads as a
/// different value — that is the box's number to meet, not this one's
/// to lower.
pub const MAX_CHARS: usize = 22;

/// `value` as text a person reads.
///
/// **The shortest decimal spelling that reads back as this value, and a
/// scientific one when no decimal spelling does.** A length wants to
/// read as a decimal and does wherever it can (`0.05`, `0.0016`,
/// `0.0003746`); below that a decimal spelling either misreads the
/// value or is a run of leading zeros past [`DECIMALS`], and the
/// scientific form carries it (`1e-12`).
///
/// **The choice is made on the property, not on a magnitude.** A
/// spelling is used when it fits [`MAX_CHARS`] and reads back within
/// [`tolerance`] of this value. So a value that is not zero never
/// renders as zero — a text reading `0.000` is a hundred percent away
/// from the value it claims to be, and no tolerance this function can
/// hold admits that, since both arms are proportional to the value or
/// finer — and a value that IS zero renders as `0`, since zero reads
/// back as itself.
///
/// **A non-finite value has no reading and gets the fallback.** No
/// spelling of `NaN` reads back as `NaN` — nothing does — so the search
/// exhausts and the scientific arm prints `NaN` or `inf`. The
/// behaviour is stated because it is what the rule produces rather
/// than a case it handles.
///
/// **Whether the chrome can hand one is the callers' question, and it
/// is answered at each of them rather than here.** The sweep is every
/// call to this function under `crates/viewer/src`, read for what its
/// argument's producer guarantees, and there are three:
/// [`crate::scene::DisplayTolerance::render_mm`], whose door refuses a
/// δ whose millimetre value is not an `f64`;
/// [`crate::props::written_text`], which asks
/// [`crate::props::written`] whether the notation can name the value
/// and says so when it cannot; and `crate::widgets::number_text`'s
/// fallback, whose argument is whatever the widget was bound to. The
/// first two cannot reach here with a non-finite value. The third can,
/// and `number_text` owns that.
///
/// **The top of the type is no longer an exception to the width.**
/// From `1.7975000000000001e308` up, four significant figures round
/// out of `f64` — `1.798e308` is not a spelling of any finite number,
/// it reads back as infinity — so the band is rendered exactly, and
/// that exact spelling is what [`MAX_CHARS`] is the width of.
/// `the_top_of_the_type_is_spelled_exactly` is the row.
pub fn number(value: f64) -> String {
    (0..=DECIMALS)
        .map(|decimals| format!("{value:.decimals$}"))
        .find(|spelling| spelling.chars().count() <= MAX_CHARS && reads_back(spelling, value))
        .unwrap_or_else(|| scientific(value))
}

/// `value` in scientific notation: the shortest mantissa that reads
/// back, and the exact spelling where none does.
///
/// **The same search [`number`] runs, in the other notation.** A
/// mantissa of seventeen significant figures names every `f64`
/// exactly, so the loop always finds one for a finite value and the
/// bound past it is reached only by a value with no reading at all.
///
/// **A non-finite value takes that fallback and is unchanged by it.**
/// Nothing reads back as `NaN`, so every precision fails and `{:e}` is
/// asked for — and it writes `NaN` and `inf` exactly as a rounded
/// spelling did. The arm is reached, not the behaviour.
fn scientific(value: f64) -> String {
    (0..=17)
        .map(|figures| format!("{value:.figures$e}"))
        .find(|spelling| reads_back(spelling, value))
        .unwrap_or_else(|| format!("{value:e}"))
}

/// Whether `spelling` reads within [`tolerance`] of `value`.
///
/// The tolerance is a distance, so the test says the same thing on both
/// sides of zero: a rendered number is judged by how far it reads from
/// the value, and a sign is not a distance.
///
/// **Width is no part of reading back, which is why it is no part of
/// this.** [`number`] is bounded by [`MAX_CHARS`] because it searches
/// every precision and something has to end the search; a caller
/// judging a spelling SOMEONE ELSE chose is asking only whether that
/// text names this value, and a wide text that does is not improved by
/// replacing it with a narrow one that does not.
///
/// **This is the module's truth test and the only door onto
/// [`tolerance`]**, which is why it is public where the grid is not: a
/// caller asking whether a text names a value is asking this, and a
/// caller reaching for the number instead would be spelling the
/// comparison a second time.
pub fn reads_back(spelling: &str, value: f64) -> bool {
    spelling
        .parse::<f64>()
        .is_ok_and(|read| (read - value).abs() <= tolerance(value))
}

#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]
    #![allow(clippy::panic)]

    use super::{DECIMALS, EPS_CAP, MAX_CHARS, REL_TOLERANCE, number, tolerance};
    use pncad::geom_core::tolerance::DEFAULT_EPS;

    /// What the rule claims, checked as a property rather than a table:
    /// a render reads back within the render's own grid — and therefore
    /// a value that is not zero never renders as zero, which is the
    /// whole defect this module closes.
    ///
    /// The character bound is asserted here too, because there is no
    /// longer a band it does not cover: [`MAX_CHARS`] is the width of
    /// every text [`number`] returns, sign aside.
    fn reads_back(value: f64) {
        let text = number(value);
        let read: f64 = text
            .parse()
            .unwrap_or_else(|error| panic!("{value} renders as {text}, not a number: {error}"));
        assert!(
            (read - value).abs() <= tolerance(value),
            "{value} renders as {text}, further from it than the render's own grid"
        );
        assert!(
            value == 0.0 || read != 0.0,
            "{value} renders as {text}, which reads as a zero it is not"
        );
        assert!(
            text.trim_start_matches('-').chars().count() <= MAX_CHARS,
            "{value} renders as {text}, past the {MAX_CHARS} character bound"
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
        // And both ends of the type, which a geometric grid does not
        // reach.
        for end in [5.0e-324, f64::MIN_POSITIVE, f64::MAX] {
            reads_back(end);
            reads_back(-end);
        }
    }

    /// **The grid separates what the kernel separates**, which is the
    /// property the cap exists for and the one a relative rule loses
    /// above its crossing.
    ///
    /// Two lengths one ε apart, at magnitudes where a relative 10⁻⁹
    /// would have spelled them the same number: a kilometre, where ε is
    /// 10⁻¹² of the value, and the metre scale where the row was filed.
    /// The row goes red the moment the cap stops governing there.
    #[test]
    fn two_values_the_kernel_decides_between_render_as_two_numbers() {
        for metres in [1.0e3_f64, 1.0, 1.0e-1] {
            for written in [metres, metres * 1.0e3] {
                let apart = written + DEFAULT_EPS;
                assert!(
                    apart != written,
                    "{written} plus ε is a different f64, or this row proves nothing"
                );
                assert_ne!(
                    number(written),
                    number(apart),
                    "{written} and {apart} are further apart than ε and render as one number"
                );
            }
        }
    }

    /// **The row's own worked example**, which is what a person sees
    /// differently: a field holding 1000.001 mm showed `1000.0`, four
    /// significant figures being all the old grid asked for.
    #[test]
    fn a_millimetre_value_keeps_the_micrometre_it_holds() {
        assert_eq!(number(1000.001), "1000.001");
        assert_eq!(number(-1000.001), "-1000.001");
        assert_eq!(number(1_024.000_1), "1024.0001");
    }

    /// **The grid is never coarser than ε in any notation the chrome
    /// writes a length in**, read off `quantity::UNITS` rather than
    /// restated: every length row's factor is at most one, so a written
    /// number is at least its canonical metres and an absolute grid on
    /// the written number resolves a canonical difference at least as
    /// fine.
    #[test]
    fn the_grid_is_never_coarser_than_epsilon_in_any_written_unit() {
        let lengths: Vec<_> = quantity::UNITS
            .iter()
            .filter(|unit| unit.as_length().is_some())
            .collect();
        assert_eq!(lengths.len(), 4, "the length rows of the closed table");
        for unit in lengths {
            assert!(
                unit.factor() <= 1.0,
                "{} has factor {}, above one, so a written number can be \
                 SMALLER than its metres and this grid stops being conservative",
                unit.symbol(),
                unit.factor()
            );
            // The canonical difference the cap resolves, written in this
            // unit: one decade below ε, divided by a factor at most one.
            let canonical = EPS_CAP * unit.factor();
            assert!(
                canonical <= DEFAULT_EPS,
                "{}'s grid resolves {canonical} m, coarser than ε",
                unit.symbol()
            );
        }
    }

    /// **[`DECIMALS`] is where a decimal spelling reaches [`EPS_CAP`]**,
    /// and one place fewer does not. Derived in `decimals_reaching`, so
    /// this row goes red if that derivation stops answering the cap it
    /// is handed — including if `DEFAULT_EPS` moves.
    #[test]
    fn the_decimal_arm_reaches_the_cap_and_stops() {
        let half_unit = |places: i32| 0.5 * 10.0_f64.powi(-places);
        assert!(
            half_unit(i32::try_from(DECIMALS).expect("a small count")) <= EPS_CAP,
            "{DECIMALS} places do not reach {EPS_CAP}"
        );
        assert!(
            half_unit(i32::try_from(DECIMALS).expect("a small count") - 1) > EPS_CAP,
            "{DECIMALS} places are one more than the cap asks for"
        );
    }

    /// **The two arms cross where the `min` says they do**, and each is
    /// the governing one on its own side — the claim that makes the
    /// tolerance a `min` rather than either arm alone.
    #[test]
    fn each_arm_governs_on_its_own_side_of_the_crossing() {
        let crossing = EPS_CAP / REL_TOLERANCE;
        assert!((tolerance(crossing) - EPS_CAP).abs() <= EPS_CAP * 1.0e-12);
        assert_eq!(
            tolerance(crossing * 1.0e3),
            EPS_CAP,
            "the cap governs above"
        );
        let below = crossing * 1.0e-3;
        assert_eq!(
            tolerance(below),
            below * REL_TOLERANCE,
            "the relative arm governs below"
        );
        assert!(
            tolerance(below) < EPS_CAP,
            "and it is the FINER of the two there, which is what a min picks"
        );
        assert_eq!(tolerance(0.0), 0.0, "zero keeps only its exact spelling");
        assert_eq!(tolerance(-1.0), tolerance(1.0), "a sign is not a distance");
    }

    /// **The top of the type is spelled exactly**, and that spelling is
    /// what [`MAX_CHARS`] is the width of: four significant figures
    /// round out of `f64` from `1.7975000000000001e308` up, so the only
    /// text that names a value there is the exact one.
    ///
    /// **It is a band and not a value.** Every `f64` from that bound to
    /// `f64::MAX` rounds to `1.798e308`, which is infinity — about
    /// 9.7·10¹¹ of them, each sign. The row samples across it.
    #[test]
    fn the_top_of_the_type_is_spelled_exactly() {
        assert_eq!(number(f64::MAX), "1.7976931348623157e308");
        assert_eq!(number(f64::MAX).chars().count(), MAX_CHARS);
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

        let low = 1.797_500_000_000_000_1e308_f64;
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

    /// **No render passes the character bound, anywhere in the type.**
    /// The bound used to have one band excepted; it has none, and that
    /// is what a box sized for it can now rely on.
    ///
    /// The sweep also counts the notations, so a change that quietly
    /// sent every value to one arm would go red rather than pass.
    #[test]
    fn no_render_passes_the_character_bound() {
        let (mut decimal, mut scientific) = (0_u32, 0_u32);
        let mut value = 1.0e-320_f64;
        while value < 1.0e300 {
            for signed in [value, -value] {
                let text = number(signed);
                assert!(
                    text.trim_start_matches('-').chars().count() <= MAX_CHARS,
                    "{signed} renders as {text}, past the {MAX_CHARS} character bound"
                );
                if text.contains('e') {
                    scientific += 1;
                } else {
                    decimal += 1;
                }
            }
            reads_back(value);
            value *= 1.05;
        }
        assert!(decimal > 100, "only {decimal} values took the decimal arm");
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

    /// **A non-finite value has no reading, so the search exhausts and
    /// the scientific arm names it.** Nothing parses back equal to
    /// `NaN` and `inf - inf` is `NaN`, so every comparison in the
    /// search is false — the arm is REACHED rather than special-cased,
    /// which is what this row pins.
    #[test]
    fn a_non_finite_value_falls_to_the_scientific_arm() {
        assert_eq!(number(f64::NAN), "NaN");
        assert_eq!(number(f64::INFINITY), "inf");
        assert_eq!(number(f64::NEG_INFINITY), "-inf");
        assert!(
            !super::reads_back("inf", f64::INFINITY),
            "an infinity does not read back as itself, which is why the search exhausts"
        );
    }

    /// A sign is not a distance: the grid is stated as a distance from
    /// the value, so a negative value is rendered by the same rule.
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
        assert_eq!(number(0.0016), "0.0016");
        assert_eq!(number(0.0003746), "0.0003746");
    }

    /// Below what [`DECIMALS`] places of decimal can spell, the
    /// scientific arm carries it — and the boundary is the BOUND, not a
    /// magnitude anybody wrote down.
    #[test]
    fn a_value_no_decimal_spelling_reaches_goes_scientific() {
        assert_eq!(number(1.0e-11), "1e-11");
        assert_eq!(number(5.0e-324), "5e-324");
        // And just above it, where a decimal spelling still fits.
        assert_eq!(number(1.0e-10), "0.0000000001");
        assert_eq!(number(4.0e-7), "0.0000004");
    }
}
