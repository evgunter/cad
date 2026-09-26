//! **One declaration per closed vocabulary, and every list projected
//! from it.**
//!
//! **Nine** enums in this crate are *closed vocabularies*: a fixed set
//! of choices the chrome offers, which something has to be able to
//! walk in order — a radio row, a combo's options, a suite's sweep —
//! and one of them, `crate::marks::EdgeLane`, is a renderer's draw
//! order walked the same way.
//! Each of the chrome's used to carry a hand-written `const ALL` beside the
//! enum, and that second copy of the membership was free to fall
//! behind the first: **adding a variant compiled**, the radio row
//! silently lost a button, and every sweep keyed on the list quietly
//! narrowed.
//!
//! [`vocabulary`] removes the second copy. The macro takes ONE list
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
//! table carries it. Where nothing walks the
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
//! `transition_table!` are the same construction (one declaration,
//! every projection expanded from it) on the kernel side, and this is
//! the viewer's two-projection case. No derive crate, and therefore
//! no new dependency in a crate whose default-feature graph is
//! deliberately the kernel's.
//!
//! # What it does not cover
//!
//! A list mirroring an enum declared in ANOTHER crate cannot use this,
//! because the declaration it would have to be projected from is not
//! here. The answer for a list that claims completeness is the same
//! construction one crate over — the kernel publishes its own `ALL`
//! beside its declaration and the form maps over it, which is what the
//! boolean operations do — and the answer for one that claims none
//! ([`crate::forms::MATE_PRIMITIVES`]) is neither: forcing a
//! deliberately partial list would force the wrong thing. What such a
//! list wants is to be TOLD its enum grew, which is the OTHER macro on
//! this page: [`partial_mirror`] classifies every mirrored variant as
//! offered or as deliberately absent, over a match with no wildcard.
//! Either way it is not a hole in this one.
//!
//! A DELIBERATELY PARTIAL mirror is not a vocabulary either
//! ([`crate::frame::SUBJECTS_WITH_AN_EXPIRY_ISSUER`] names two of five
//! `Subject`s on purpose, and each tool's own seat list names its own
//! seats). Those stay hand-written, because completeness is exactly
//! what they do not claim — and a MIRROR among them takes
//! [`partial_mirror`] for the weaker thing that is true of it. A
//! tool's seat list is not one: it specifies that tool rather than
//! tracking `Seat`'s membership, so a new seat no tool asked for is
//! absent from it correctly and there is nothing to be told.
//!
//! # What this costs: rustfmt stops at the invocation
//!
//! **`rustfmt` does not reach inside a `macro_rules!` invocation in
//! item position**, so the eight enums declared through
//! [`vocabulary`] — every variant and every variant doc of `Seat` (9),
//! `ToolKind` (7) and six more — are no longer mechanically
//! formatted. Indentation in these
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
//! available to these eight enums without un-converting them. **This is
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

/// **A partial mirror, told when the mirrored enum grows.**
///
/// Takes the mirrored enum, what the chrome OFFERS, and a ROSTER that
/// classifies every variant of that enum as offered or as deliberately
/// absent with the reason it is absent. Nothing here forces the
/// offering to be COMPLETE — completeness is exactly what a
/// deliberately partial mirror does not claim, and a mechanism that
/// forced it would be forcing the wrong thing. What it forces is that
/// every variant is one or the other, so a variant added to the
/// mirrored enum is neither until someone writes one of the two, and
/// the build says so.
///
/// # The half every shape has
///
/// `every_variant_is_offered_or_named_absent` is a match over the
/// mirrored enum with one arm per roster entry — both sections — and
/// no wildcard. A variant added to the enum has no arm, so `E0004`
/// reds here and names it. The only way to silence it is to put that
/// variant in one section or the other, which is the decision this
/// instrument exists to force.
///
/// **The absent section is not a second hand-written list.** It is the
/// other half of the one roster that match holds against the enum, so
/// it cannot fall behind what it mirrors any more than the offered
/// half can. The reason is required by the GRAMMAR rather than
/// asserted over: an author cannot write an absence without saying
/// what makes it deliberate.
///
/// # Three shapes, because the crate has three
///
/// What the offering IS decides the rest, and the three differ in
/// whether a seat of it can drift from the roster:
///
/// - **`labelled <list>`** — a hand-written `[(Variant, &str); N]`.
/// - **`bare <list>`** — a hand-written `[Variant; N]`. The two list
///   shapes are [`vocabulary`]'s two, which is why they carry its
///   words: a list is one or the other and a seat reads `list[n].0` or
///   `list[n]` accordingly.
/// - **`onto <Choice>`** — a separate FIELDLESS enum whose variants
///   are the offering, each offered entry naming its counterpart in it
///   (`Variant => Counterpart`).
///
/// **The two list shapes get a seat half**: one `assert!` per offered
/// entry, in a `const` block, saying the list holds that variant at
/// that seat. Classifying a new variant as offered therefore asserts
/// `list[n]` for a seat the old list does not have — a const-eval
/// error, out of bounds, until the list itself grows. Classifying it
/// as absent asserts nothing further, which is the whole point: the
/// list stays its own length and the roster says why. A trailing count
/// check closes the third direction: an entry added to the list with
/// no roster classification leaves the list longer than the offered
/// section, and an entry moved from offered to absent leaves it
/// shorter. **The offered section is therefore written in the LIST's
/// order**, which is the order the chrome draws.
///
/// **`onto` has no seat half, and that is not a weaker arm.** Its
/// offering is an enum, and a `vocabulary!` enum's `ALL` is projected
/// from the declaration rather than written beside it, so naming
/// `Choice::X` as a counterpart already says the chrome offers it —
/// there is no second copy of the membership for a seat assertion to
/// hold. What it expands to is the exhaustive half alone, with the
/// counterpart named in each offered arm so the roster IS the mapping
/// rather than a bare list of names. Order is inert in this arm, so
/// the roster is written in the mirrored enum's own order.
///
/// # What `onto` cannot express
///
/// **A fieldless counterpart only.** The arm names `Choice::X` as a
/// VALUE, so a counterpart carrying a payload does not compile. That
/// is the same restriction [`vocabulary`] states, and it holds this
/// arm to the case it argues: a form's choice enum. A partial mirror
/// whose offering is a payload-carrying enum
/// (`session::author::PatternRuleSpec` over `PatternKind`) is not
/// served by this arm.
macro_rules! partial_mirror {
    // The exhaustive half, which is the same match whatever the
    // offering is.
    (@exhaustive $ty:ident,
        [ $($ov:ident $({ $($op:tt)* })?),* ],
        [ $($av:ident $({ $($ap:tt)* })?),* ]
    ) => {
        #[allow(dead_code)]
        fn every_variant_is_offered_or_named_absent(value: $ty) {
            match value {
                $($ty::$ov $({ $($op)* })? => (),)*
                $($ty::$av $({ $($ap)* })? => (),)*
            }
        }
    };

    // LABELLED list: a seat is `(Variant, &str)`.
    (
        $ty:ident, labelled $list:expr,
        offered [ $($ov:ident $({ $($op:tt)* })?),+ $(,)? ],
        absent [ $($av:ident $({ $($ap:tt)* })? => $why:literal),* $(,)? ] $(,)?
    ) => {
        const _: () = {
            crate::vocab::partial_mirror!(@exhaustive $ty,
                [ $($ov $({ $($op)* })?),+ ],
                [ $($av $({ $($ap)* })?),* ]
            );
            let mut seat = 0;
            $(
                assert!(
                    matches!($list[seat].0, $ty::$ov $({ $($op)* })?),
                    "the offering has drifted from its roster: this seat \
                     does not offer the variant the roster puts here"
                );
                seat += 1;
            )+
            assert!(
                seat == $list.len(),
                "the offering holds a variant its roster does not classify"
            );
        };
    };

    // BARE list: a seat is the variant itself.
    (
        $ty:ident, bare $list:expr,
        offered [ $($ov:ident $({ $($op:tt)* })?),+ $(,)? ],
        absent [ $($av:ident $({ $($ap:tt)* })? => $why:literal),* $(,)? ] $(,)?
    ) => {
        const _: () = {
            crate::vocab::partial_mirror!(@exhaustive $ty,
                [ $($ov $({ $($op)* })?),+ ],
                [ $($av $({ $($ap)* })?),* ]
            );
            let mut seat = 0;
            $(
                assert!(
                    matches!($list[seat], $ty::$ov $({ $($op)* })?),
                    "the offering has drifted from its roster: this seat \
                     does not offer the variant the roster puts here"
                );
                seat += 1;
            )+
            assert!(
                seat == $list.len(),
                "the offering holds a variant its roster does not classify"
            );
        };
    };

    // ONTO a fieldless choice enum: the offering is projected from its
    // own declaration, so there is no seat to hold and the roster is
    // the mapping.
    (
        $ty:ident, onto $cty:ident,
        offered [ $($ov:ident $({ $($op:tt)* })? => $cv:ident),+ $(,)? ],
        absent [ $($av:ident $({ $($ap:tt)* })? => $why:literal),* $(,)? ] $(,)?
    ) => {
        const _: () = {
            #[allow(dead_code)]
            fn every_variant_is_offered_or_named_absent(value: $ty) -> Option<$cty> {
                match value {
                    $($ty::$ov $({ $($op)* })? => Some($cty::$cv),)+
                    $($ty::$av $({ $($ap)* })? => None,)*
                }
            }
        };
    };
}

pub(crate) use partial_mirror;
