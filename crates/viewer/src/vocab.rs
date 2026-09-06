//! **One declaration per closed vocabulary, and every list projected
//! from it.**
//!
//! A dozen enums in this crate are *closed vocabularies*: a fixed set
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
//! ([`crate::forms::BOOLEAN_OPS`], [`crate::forms::MATE_PRIMITIVES`])
//! cannot use this, because the declaration it would have to be
//! projected from is not here. That is the neighbouring MIRROR
//! question and has its own tracker item; it is not a hole in this
//! one.
//!
//! A DELIBERATELY PARTIAL list is not a vocabulary either
//! ([`crate::frame::SUBJECTS_WITH_AN_EXPIRY_ISSUER`] names two of five
//! `Subject`s on purpose, and each tool's own seat list names its own
//! seats). Those stay hand-written, because completeness is exactly
//! what they do not claim.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

/// **Declare a closed vocabulary**: an enum and the `ALL` that lists
/// every one of its variants, from one list of variants.
///
/// Two shapes, because the crate has two. A BARE vocabulary projects
/// `[Self; N]` and keeps its wording in a `label` match beside it; a
/// LABELLED one writes each variant's word in the declaration and
/// projects `[(Self, &'static str); N]`, which is what a radio row
/// iterates. Every variant carries a label or none does — a
/// half-labelled list matches neither arm and fails to compile, which
/// is the right answer to "which shape is this".
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
/// }
/// ```
macro_rules! vocabulary {
    // The count, from the same tokens the variants come from.
    (@count) => { 0usize };
    (@count $head:ident $($tail:ident)*) => {
        1usize + crate::vocab::vocabulary!(@count $($tail)*)
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
            /// Projected from this enum's declaration by
            /// [`crate::vocab::vocabulary`], so it is the variant list
            /// and not a copy of it: a variant cannot reach the enum
            /// without reaching this array, in this order.
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
            /// Projected from this enum's declaration by
            /// [`crate::vocab::vocabulary`], so it is the variant list
            /// and not a copy of it: a variant cannot reach the enum
            /// without reaching this array, in this order.
            $avis const $all: [Self; crate::vocab::vocabulary!(@count $($variant)+)] =
                [$( Self::$variant, )+];
        }
    };
}

pub(crate) use vocabulary;
