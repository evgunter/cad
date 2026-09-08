//! **One declaration per closed vocabulary, and every list projected
//! from it.**
//!
//! **Nine** enums in this crate are *closed vocabularies*: a fixed set
//! of choices the chrome offers, which something has to be able to
//! walk in order — a radio row, a combo's options, a suite's sweep.
//! Each of them used to carry a hand-written `const ALL` beside the
//! enum, and that second copy of the membership was free to fall
//! behind the first: **adding a variant compiled**, the radio row
//! silently lost a button, and every sweep keyed on the list quietly
//! narrowed.
//!
//! [`vocabulary!`] removes the second copy. The macro takes ONE list
//! of variants and expands it into the enum AND its `ALL`, so there is
//! no way to add a variant without adding it to the list, and no way
//! to add it to the list without adding it to the enum: they are the
//! same tokens. That is stronger than a compile error — it is the
//! [`crate::tools`] argument one level down, a state that cannot be
//! written down rather than one a check has to catch.
//!
//! **The words ride that same list where they are table data.**
//! Where something walks a vocabulary's table and reads every entry's
//! word — a combo drawing an option per row, a picker drawing a
//! button per mode — the word belongs to the declaration and the
//! table carries it; the accessor a single value's word is asked
//! through is projected from that same list rather than written
//! beside it as a second ordered reading. Where nothing walks the
//! table for words, they are not table data and a match beside the
//! enum is the whole of it (`crates/viewer/README.md`, **Closed
//! vocabularies are declared once**).
//!
//! **The order is the declaration's.** These lists are read in order
//! by menus and radio rows, and several of them say so in their own
//! docs, so the projection preserves it: what a reader sees on screen
//! is the order they see in the file. Where a form wants a different
//! order from the one the type grew in, the ENUM is written in the
//! form's order and says why — one order, stated once.
//!
//! **The shape is the repo's**, not a new one:
//! `crates/profile/src/path/program.rs`'s `arc_modes!` and
//! `transition_table!` are the same construction ("ONE declaration,
//! THREE projections") on the kernel side, and this is the viewer's
//! two-projection case. No derive crate, and therefore no new
//! dependency in a crate whose default-feature graph is deliberately
//! the kernel's.
//!
//! # What it does not cover
//!
//! A table mirroring an enum declared in ANOTHER crate
//! (`forms::BOOLEAN_OPS`, `forms::MATE_PRIMITIVES` — plain code spans
//! and not links, because `forms` is behind the `app` feature and a
//! link to it does not resolve in a default-feature build) cannot use
//! this, because the declaration it would have to be projected from is
//! not here. That is the neighbouring MIRROR question and has its own
//! tracker item; it is not a hole in this one.
//!
//! A DELIBERATELY PARTIAL list is not a vocabulary either
//! ([`crate::frame::SUBJECTS_WITH_AN_EXPIRY_ISSUER`] names two of five
//! `Subject`s on purpose, and each tool's own seat list names its own
//! seats). Those stay hand-written, because completeness is exactly
//! what they do not claim.
//!
//! # What this costs: rustfmt stops at the invocation
//!
//! **`rustfmt` does not reach inside a `macro_rules!` invocation in
//! item position**, so the nine enums declared through
//! [`vocabulary!`] — every variant and every variant doc of
//! `PathVerb` (17), `Seat` (9), `ToolKind` (7), `ArcMode` (6) and five
//! more — are no longer mechanically formatted. Indentation in these
//! blocks is kept by hand.
//!
//! Demonstrated, not assumed: a variant re-indented to column 21
//! inside `blend.rs`'s block leaves `cargo fmt --check` at exit 0,
//! while the same mis-indent on the `BlendError` enum ten lines below
//! it is caught. It is the invocation that stops rustfmt and not the
//! body: the same test with the body rewritten so it parses as a
//! single well-formed item (the `ALL` declaration moved onto the enum
//! as an attribute), and again with the invocation delimited by `()`
//! instead of `{}`, is still not reached.
//!
//! This is a real loss and it is the price of the mechanism, not an
//! oversight — tracked as
//! `work/view/vocabulary-macro-bodies-are-outside-rustfmt.md`.
//!
//! # What the macro cannot express
//!
//! **Fieldless variants only.** A variant with a payload
//! (`Chamfer(u32)`) matches neither arm, and the error says
//! `no rules expected '(' … note: while trying to match '='`, which
//! reads as "you forgot a label" when the answer is that a vocabulary
//! is a set of names and a variant carrying a value is not one.
//!
//! **No explicit discriminants.** The labelled arm spends `= …` on the
//! variant's word, so `Mate = 3` and `#[repr(u8)]` numbering are not
//! available to these nine enums without un-converting them. **This is
//! a one-way door** and is the reason to state it here: an enum that
//! later needs a wire number has to leave the macro to get one.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

/// **Declare a closed vocabulary**: an enum and the `ALL` that lists
/// every one of its variants, from one list of variants.
///
/// Two shapes, because the crate has two. A BARE vocabulary projects
/// `[Self; N]` and keeps its wording in a `label`/`name` match beside
/// it; a LABELLED one writes each variant's word in the declaration
/// and projects `[(Self, &'static str); N]`, which is what a row that
/// walks the table for its words iterates. Every variant carries a
/// label or none does — a half-labelled list matches neither arm and
/// fails to compile, which is the right answer to "which shape is
/// this".
///
/// A labelled vocabulary whose word is ALSO asked for one value at a
/// time declares `fn <name>;` under its `ALL`, and gets that accessor
/// projected from the same list as a match. **Declaring it is how the
/// vocabulary asks for it**: a projection nobody asked for would be
/// dead code in the five labelled vocabularies that never read one,
/// and an `#[allow(dead_code)]` blanketing all of them would silence
/// the report that an accessor has lost its last reader.
///
/// `N` is counted from the same list, so no count is written down
/// either.
///
/// ```ignore
/// vocabulary! {
///     /// Which end of the thing.
///     #[derive(Clone, Copy, PartialEq, Eq)]
///     pub enum End {
///         /// The near end.
///         Near = "near",
///         /// The far end.
///         Far = "far",
///     }
///
///     /// Both ends with their button labels, in form order.
///     pub const ALL;
///
///     /// This end's word.
///     pub fn label;
/// }
/// ```
macro_rules! vocabulary {
    // The count, from the same tokens the variants come from.
    (@count) => { 0usize };
    (@count $head:ident $($tail:ident)*) => {
        1usize + crate::vocab::vocabulary!(@count $($tail)*)
    };

    // LABELLED, and the word is asked for one value at a time too:
    // the accessor is projected from the same list, so declaring it is
    // how a vocabulary asks for it and no vocabulary carries one it
    // never reads.
    (
        $(#[$emeta:meta])*
        $evis:vis enum $name:ident {
            $(
                $(#[$vmeta:meta])*
                $variant:ident = $label:literal
            ),+ $(,)?
        }

        $(#[$ameta:meta])*
        $avis:vis const $all:ident;

        $(#[$fmeta:meta])*
        $fvis:vis fn $word:ident;
    ) => {
        crate::vocab::vocabulary! {
            $(#[$emeta])*
            $evis enum $name {
                $( $(#[$vmeta])* $variant = $label ),+
            }

            $(#[$ameta])*
            $avis const $all;
        }

        impl $name {
            $(#[$fmeta])*
            ///
            /// A match over the same list the array above is built
            /// from, projected by the crate's `vocabulary!` macro
            /// (`crates/viewer/src/vocab.rs`): the two readings are
            /// the same tokens, and a variant with no word does not
            /// parse.
            $fvis const fn $word(self) -> &'static str {
                match self {
                    $( Self::$variant => $label, )+
                }
            }
        }
    };

    // LABELLED: every variant states the word the chrome shows.
    (
        $(#[$emeta:meta])*
        $evis:vis enum $name:ident {
            $(
                $(#[$vmeta:meta])*
                $variant:ident = $label:literal
            ),+ $(,)?
        }

        $(#[$ameta:meta])*
        $avis:vis const $all:ident;
    ) => {
        $(#[$emeta])*
        $evis enum $name {
            $( $(#[$vmeta])* $variant, )+
        }

        impl $name {
            $(#[$ameta])*
            ///
            /// Projected from this enum's declaration by the crate's
            /// `vocabulary!` macro (`crates/viewer/src/vocab.rs`), so
            /// it is the variant list and not a copy of it: a variant
            /// cannot reach the enum without reaching this array, in
            /// this order.
            $avis const $all: [(Self, &'static str);
                crate::vocab::vocabulary!(@count $($variant)+)] =
                [$( (Self::$variant, $label), )+];
        }
    };

    // BARE: the wording, if there is any, lives in a match beside it.
    (
        $(#[$emeta:meta])*
        $evis:vis enum $name:ident {
            $(
                $(#[$vmeta:meta])*
                $variant:ident
            ),+ $(,)?
        }

        $(#[$ameta:meta])*
        $avis:vis const $all:ident;
    ) => {
        $(#[$emeta])*
        $evis enum $name {
            $( $(#[$vmeta])* $variant, )+
        }

        impl $name {
            $(#[$ameta])*
            ///
            /// Projected from this enum's declaration by the crate's
            /// `vocabulary!` macro (`crates/viewer/src/vocab.rs`), so
            /// it is the variant list and not a copy of it: a variant
            /// cannot reach the enum without reaching this array, in
            /// this order.
            $avis const $all: [Self; crate::vocab::vocabulary!(@count $($variant)+)] =
                [$( Self::$variant, )+];
        }
    };
}

pub(crate) use vocabulary;
