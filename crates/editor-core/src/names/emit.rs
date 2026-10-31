//! Eager per-op name-table emission (N4; spec D4): mechanical, linear
//! passes over each op's output, driven ONLY by kernel birth data and
//! mint-time wiring facts (`Extruded`/`Revolved` maps, `SplitNaming`,
//! `BooleanNaming`, D5 provenance) plus combinatorial adjacency
//! between EMITTED anchors — never geometric matching. Where geometry
//! must discriminate (N2 fragment qualifiers), it enters as margined
//! predicate VERDICTS through `k_stats` ([`super::discriminate`]).
//!
//! Every emitter finishes with the TOTALITY check: every live
//! boundary entity of every output body must be named (a gap is a
//! typed [`NamingError`], surfacing kernel-emission holes loudly),
//! and injectivity is enforced by [`NameTable`] at insertion.

use std::collections::BTreeMap;
use std::sync::Arc;

use geom_core::{BandError, Indeterminate};
use topo::{
    Body, EdgeKey, FaceKey, HalfEdgeKey, LoopKey, SettleError, SplitLineageCycle, VertexKey,
};

use super::role::{EntityKind, StableName};
use super::table::{EntityKey, EntityRef, NameTable, SeatingRefusal};
use crate::node::RecipeNodeId;

/// Typed failure of name emission (spec D4's loud assertions, as
/// in-band errors — this crate has no panic paths).
///
/// **Five kinds of thing live here, and the reader is told which.**
///
/// 1. An emission BUG — a mint-time fact inconsistent with the result
///    body. [`Self::Duplicate`], [`Self::EntityNamed`],
///    [`Self::KindMismatch`], [`Self::ShortTie`], [`Self::Unnamed`],
///    [`Self::MissingUpstream`], [`Self::Emission`],
///    [`Self::SplitLineage`], [`Self::Settle`].
/// 2. An honest in-band escalation: [`Self::Escalated`].
/// 3. An ambient tolerance no discriminator can be built from:
///    [`Self::Band`], whose own doc draws the line the category below
///    stands on — *nothing about the result body is wrong here*.
/// 4. A MISSING RULE: [`Self::UnsupportedSeamVertex`],
///    [`Self::UnsupportedCrossingSenses`], [`Self::UnsupportedSeamSides`],
///    [`Self::UnsupportedJoin`], [`Self::UnsupportedUnionName`],
///    [`Self::UnsupportedTurnedCell`],
///    [`Self::SeamVertexPartners`], [`Self::SharedRim`],
///    [`Self::ChordOffRim`], [`Self::MemberEdgeTied`],
///    [`Self::ConventionalVertex`], [`Self::ClosedCarrierUnread`],
///    [`Self::CarrierUnplaced`] and [`Self::PartnerUnread`],
///    reached from recipes nothing is wrong with,
///    where the emitter has no rule for a construction the recipe
///    produced. They read as a missing rule and not as a bug report,
///    because telling an author to file a kernel bug over their own
///    legal document fails in the expensive direction. A variant that
///    several constructions reach carries a shape naming which.
/// 5. A PRECISION LIMIT the model controls: [`Self::WayInZeroBand`],
///    [`Self::JoinedPieceOffRoot`] and [`Self::BandVerdictsDisagree`], a
///    verdict the tolerance band cannot give on a sound result. Each
///    ends in the model's lever alone.
///
/// **Three of the five categories are readable off the opening
/// words.** Three framings are written once each —
/// [`EMISSION_FRAMING`], [`UNRULED_FRAMING`] and [`LIMIT_FRAMING`] —
/// and each opens every refusal of its category. Categories 2 and 3
/// have one variant each, with a sentence of its own, and borrow no
/// framing. So a reader tells a kernel bug from a MISSING RULE from a
/// precision limit from everything else by the clause a refusal opens
/// with: the first clause of this type's `Display`, which the
/// node-level refusal (`NodeErrorKind::Naming`) renders after its own
/// `name emission failed: `.
///
/// What is machine-checked, measured rather than asserted — by adding
/// a probe variant and watching what each guard did.
/// `display_tests`' `every_variant_names_its_subject` maps every
/// variant to its framing through an exhaustive match, so a new variant
/// must CHOOSE a category to compile, and asserts each sample OPENS
/// with that framing and does not speak the other's. Its coverage
/// check compares sampled indices against `0..rows.len()`, which
/// catches a row deleted from the middle and **stays green for a
/// variant appended past the end** — measured. `crates/editor-core/tests/display_contract.rs`'s
/// `NAMING_ERROR` census closes exactly that: the `f6_variants!` ident
/// list writes the wildcard-free `match` AND the roster, so the probe
/// stops that file compiling, the only fix is to add the ident, adding
/// the ident adds it to the roster, and the set difference then reds
/// until it has a rendered case. No step of that chain is green —
/// measured too.
#[derive(Debug)]
pub enum NamingError {
    /// A would-be duplicate name outside the tie path (the
    /// no-silent-aliasing guarantee N1 rests on).
    Duplicate {
        /// The colliding name.
        name: Box<StableName>,
    },
    /// A row seated on an entity that already holds a name
    /// ([`super::table::EntityNamed`]).
    EntityNamed {
        /// The name the row was to be seated under.
        name: Box<StableName>,
        /// The name the entity already holds.
        held: Box<StableName>,
    },
    /// A row seated on an entity of another kind than its name's
    /// ([`super::table::KindMismatch`]).
    KindMismatch {
        /// The name the row was to be seated under.
        name: Box<StableName>,
        /// The offered entity's kind.
        found: EntityKind,
    },
    /// A tie seated with fewer than two distinct candidates
    /// ([`super::table::ShortTie`]).
    ShortTie {
        /// The name the tie was to be seated under.
        name: Box<StableName>,
        /// How many distinct entities it was offered: 0 or 1.
        candidates: usize,
    },
    /// A live boundary entity the emission failed to cover — a
    /// kernel-emission gap surfaced (spec D4: never guessed around).
    Unnamed {
        /// The uncovered entity's kind.
        kind: EntityKind,
        /// The output-body index it lives in.
        body: u32,
    },
    /// An upstream input's name table lacks an entity the emission
    /// needed (wiring bug: upstream tables are total by this same
    /// machinery).
    MissingUpstream {
        /// The upstream node whose table missed.
        node: RecipeNodeId,
    },
    /// A mint-time emission fact was inconsistent with the result
    /// body (kernel bug, loudly).
    Emission {
        /// What was inconsistent.
        what: &'static str,
    },
    /// An edge's split lineage cycles, as `topo::Body::split_root`
    /// refuses it where an emitter chases through it (the emitters' own
    /// walks over the copies panic instead, `names::chase`) — the same category of kernel bug as
    /// [`Self::Emission`], carrying the one thing the repair needs
    /// that a sentence cannot supply: WHICH edge.
    ///
    /// **Unguardable from this crate, and the reason is WRITER
    /// ACCESS.** A lineage chase advances only on
    /// `Body::edge_provenance`, which is `pub(crate)` to `topo`: `Body::split_edge` records the parent on
    /// a child it has just minted, so a chain is strictly decreasing in
    /// age in the arena that wrote it; a graft forwards it injectively
    /// (each source key to its own result key, live or dead on
    /// arrival), which maps an acyclic chain to an acyclic one. No
    /// caller can close it.
    SplitLineage(SplitLineageCycle),
    /// A Boolean's naming rows settle a result vertex on no live cell
    /// (`topo::BooleanNaming::settler`) — the same category of kernel
    /// bug as [`Self::Emission`], carrying the vertex the chase was asked
    /// about and how it failed.
    Settle(SettleError),
    /// A boolean's or union's seam VERTEX left half-decided by the
    /// edges around it: the incident edges' parents fit none of the
    /// shapes the vertex pass names, and nothing else (an operand
    /// vertex, a contact row) names it. Which shape it met is
    /// [`SeamVertexShape`].
    ///
    /// **Not an emission inconsistency**, and the test is the same one
    /// [`Self::Band`] applies: nothing about the result body is wrong
    /// here. A declared union reached it — a member stacked on a face
    /// that a second declared contact had merged put such a vertex on
    /// the merged wall — and what such an author needs is that the
    /// emitter has no rule for this shape, not an instruction to file a
    /// kernel bug. Maximal edges join that witness's vertex away, and no
    /// document reaches it now; nothing proves the other shapes
    /// unreachable, so they take the same ending.
    ///
    /// Carries the vertex — a key in the RESULT body — which is the one
    /// thing a sentence cannot supply and where a rule would apply.
    UnsupportedSeamVertex {
        /// The result-body vertex whose parentage is not determined.
        vertex: VertexKey,
        /// The shape its edges met.
        shape: SeamVertexShape,
    },
    /// A seam vertex whose boolean contact records pair it with SEVERAL
    /// differently named vertices of the other operand.
    ///
    /// The seam-vertex pass names a vertex with no seam structure of its
    /// own by its contact-record partner. One partner — or several rows
    /// naming the same one — decides it. Several distinct partners do
    /// not, and no rule chooses among them, so the pass refuses rather
    /// than take whichever row the reduction happened to write first.
    /// A sibling word of [`Self::UnsupportedSeamVertex`]: the same vertex
    /// pass, a different structure to read (the contact rows, not the
    /// incident edge roles).
    SeamVertexPartners {
        /// The result-body vertex.
        vertex: VertexKey,
        /// The distinct upstream names the contact rows pair it with,
        /// in name order.
        candidates: Vec<StableName>,
    },
    /// Two faces a boolean's seam-chord derivation believes meet along
    /// ONE edge of an operand body do not.
    ///
    /// **Not an emission inconsistency, and the premise is what makes
    /// that true.** This caller did not build the body it is asking
    /// about: it descends two result faces to operand faces and GUESSES
    /// that the pair carries the chord's rim. Zero shared edges is an
    /// ordinary property of a sound body (most face pairs of any body
    /// share none) and two is an ordinary property of a fragmented one,
    /// so neither answer says the body is corrupt — what they refute is
    /// the guess, which a declared union invalidates the moment a later
    /// member splits a merged face.
    ///
    /// **The walk does not classify**, because it cannot see the
    /// premise: a caller whose body was minted alongside the bundle it
    /// names from (`emit_sweep`'s cap rims) draws the opposite
    /// conclusion from the same answer. The walk (`rim_between`, in this
    /// module) therefore returns a `Rim` and each caller says what it
    /// means. That does not make a wrong classification impossible —
    /// `Emission` takes a `&'static str` and anyone may write one; what
    /// it removes is inheriting one by saying nothing.
    ///
    /// A sibling word of [`Self::UnsupportedSeamVertex`] rather than one
    /// generalised over [`super::table::EntityKey`]: the two name DIFFERENT
    /// structures to go and read — face adjacency in an operand body
    /// here, a vertex's incident edge ROLES there — and their subjects
    /// differ in arity as well as kind, so one word for the class would
    /// hide which structure the missing rule is about.
    ///
    /// The two ways the rim fails to be unique are ONE fact with a
    /// typed discriminant ([`RimShare`]) rather than two words. What
    /// makes that hold is that the classification lives at the CALLER:
    /// this variant is constructed at two sites, both of them arms of
    /// `emit_topo`'s one chord derivation and so both under ONE premise,
    /// and under one premise the question asked ("is the rim unique"),
    /// the subject (this face pair of this operand) and the author's
    /// move are the same whichever way the answer went. A `&'static str` here would be [`Self::Emission`]
    /// one level down.
    SharedRim {
        /// The node whose body was walked — an OPERAND of the boolean,
        /// not its result, so two bare face keys would name nothing a
        /// caller could resolve. [`Self::MissingUpstream`] carries the
        /// same locator for the same reason.
        node: RecipeNodeId,
        /// One face of the pair whose rim was asked for.
        face: FaceKey,
        /// The other — `ShellError::WallClearance`'s spelling for the
        /// same shape, a refusal about a PAIR of faces of one body.
        other: FaceKey,
        /// What the walk found instead of one edge.
        found: RimShare,
    },
    /// A boolean's chord between two faces of one operand, merged or not,
    /// that lies off the one rim each pair of them shares, where one face
    /// of that operand is on both sides: two pieces of it meet along the
    /// chord. Pieces of one face the output stage's merge left apart (a
    /// group it skipped, `topo`'s `finish_output`) meet so in a correct
    /// result, and no rule names a chord within one operand face.
    ChordOffRim {
        /// The result-body edge.
        edge: EdgeKey,
        /// The operand node whose face the chord lies within.
        node: RecipeNodeId,
        /// That face, in the operand's body.
        face: FaceKey,
    },
    /// A **conventional vertex** (`topo::is_conventional_vertex`: its
    /// only edge is one closed edge, at both its ends) that a table
    /// would name. It has no identity of its own (`docs/DESIGN.md`,
    /// maximal edges), so no member- or position-citing name may stand
    /// for it, and the edge-derived name it takes instead is unbuilt
    /// (`work/seamrank/a-conventional-vertex-mints-no-edge-derived-name`).
    /// A construction's own rim self-loop vertex
    /// (`RoleSeg::MeridianVertex` at the seam) keeps its name and is
    /// not refused.
    ConventionalVertex {
        /// The vertex, a key of the output body.
        vertex: topo::VertexKey,
        /// The output-body index it lives in.
        body: u32,
    },
    /// A **closed joined edge on a carrier with no period**: the flush
    /// rule reads a closed edge over its carrier's whole period
    /// (`names/README.md`, "Flush edges"), and a carrier with none (a
    /// spline closed on itself) gives it no span to read. The join
    /// refuses to close one (`topo::BooleanError::JoinCarrierUnsupported`),
    /// so no body reaches here today.
    ClosedCarrierUnread {
        /// The joined edge.
        edge: EdgeKey,
        /// Its carrier's kind.
        carrier: geom::CurveKind,
    },
    /// A **point the flush reading cannot place along a curved edge**:
    /// whether edge `edge` holds a point is read at the point's
    /// parameter on the edge's certified carrier (`names/README.md`,
    /// "Flush edges"), and a carrier of this kind (a spline) gives no
    /// parameter for a point. Answering "not on it" would drop the edge
    /// from the reading, so it refuses.
    CarrierUnplaced {
        /// The edge, in the body the reading placed the point against.
        edge: EdgeKey,
        /// Its carrier's kind.
        carrier: geom::CurveKind,
    },
    /// A **crossing at a vertex whose partner the boolean read nothing
    /// of** ([`topo::UnreadPartner`]): the vertex sits in one pair with
    /// a vertex of the other operand whose neighbourhood no wedge or
    /// corner reading classes its edges against, so the boolean holds no
    /// class for them, and the crossing's senses cannot be read. The
    /// boolean builds such a result soundly; what is missing is a rule
    /// that reads the edges against the partner's neighbourhood
    /// (`work/tang/a-lone-unread-partner-has-no-class-rule`).
    PartnerUnread {
        /// The result-body vertex.
        vertex: VertexKey,
        /// The partner, `unread.partner`: the other operand's vertex, in
        /// that operand's clone keys. A field of its own so the sentence
        /// renders it as one (`pncad-py`'s prose census decides a
        /// rendered binding only at a variant's own field).
        partner: VertexKey,
        /// The boolean's record: the operand vertex whose edges went
        /// unclassed, the partner it met and why that read nothing.
        unread: topo::UnreadPartner,
    },
    /// A union's member edge read where ONE edge is needed, where a tie
    /// stands: the member's own table ties the edge's name (or the
    /// least row on its line) to several edges.
    ///
    /// The union needs one edge to place a finished vertex or edge on
    /// it and to read its two faces (`Flush`); a tie offers several
    /// edges and no rule picks one. Nothing about the result body is wrong, so this
    /// is a missing rule and not an [`Self::Emission`].
    MemberEdgeTied {
        /// The member whose edge it is.
        member: crate::VarId,
        /// The edge, as the member's own table names it.
        edge: Box<StableName>,
    },
    /// A boolean crossing whose senses no rule reads: the edge piece
    /// classes at the vertex fit no one sense per edge name
    /// ([`CrossingSensesShape`]). The boolean's classes are its own
    /// record and a sound result can hold them; what is missing is a rule
    /// reading a vertex that holds several keys, or two tied edges, at
    /// once.
    UnsupportedCrossingSenses {
        /// The result-body vertex.
        vertex: VertexKey,
        /// What the classes there read.
        shape: CrossingSensesShape,
    },
    /// An edge named from the faces beside it, whose faces fit no shape
    /// a seam rule reads ([`SeamSidesShape`]): a wrap edge `Incidence`
    /// reads beside one face, a name that spells two seams, a zipped
    /// seam within one operand, a union seam inside one parent.
    UnsupportedSeamSides {
        /// The result-body edge.
        edge: EdgeKey,
        /// What its faces are.
        shape: SeamSidesShape,
    },
    /// An edge a closing join made whose cover no rule reads
    /// ([`JoinShape`]): a join over pieces that are not all images of
    /// input cells, or a seam beside a merged face the split has no
    /// rule for.
    UnsupportedJoin {
        /// The joined edge, in the door's result body.
        edge: EdgeKey,
        /// What the cover holds.
        shape: JoinShape,
    },
    /// A name a union publishes that its member-keying cannot spell
    /// ([`UnionNameShape`]): the collapse from fold space to member
    /// space does not keep name order or lines apart, and a citation
    /// can reach several parents. The members and the fold are sound.
    UnsupportedUnionName {
        /// The name as the union held it where the shape was met: the
        /// fold row the collapse was rewriting for
        /// [`UnionNameShape::JunctionLinesCollide`], the member-keyed
        /// name being published for the other.
        name: Box<StableName>,
        /// What the name met.
        shape: UnionNameShape,
    },
    /// A seam edge or pole of a sphere the boolean re-charted, kept in the
    /// result where the turn put it (`topo::BooleanNaming::recharted`).
    /// The re-chart turns a closed sphere shell about its centre so its
    /// seams cross the other solid's section circles, and a turned seam
    /// edge or pole descends from no cell of its operand: no name form
    /// (NS2) is true of it. The way through is to cut the sphere along
    /// its own chart instead of turning it
    /// (`work/orbit/closed-sphere-escape-is-re-charted-by-rotation-beside-the-meridian-cut.md`).
    UnsupportedTurnedCell {
        /// The read the turned sphere's operand came in through.
        read: crate::VarId,
        /// The turned cell, in the operand's re-cut keys. The emitter
        /// raises it on an edge or a vertex
        /// (`emit_topo::refuse_turned_cells`); the sentence reads any
        /// kind.
        cell: topo::Cell,
    },
    /// Which way edge `edge` runs along another's carrier decided ZERO:
    /// the two readings compared lie within a zero band of each other. A
    /// certified edge spans at least the escalate band, K zero bands, so
    /// at a small ambiguity factor K the comparison can fall in the zero
    /// band ([`WayShape`] says which comparison). The lever is K.
    WayInZeroBand {
        /// The edge read along the other's carrier, in the body it was
        /// read from.
        edge: EdgeKey,
        /// Which comparison decided zero.
        shape: WayShape,
    },
    /// A split's joined set piece that the cover read as lying along an
    /// operand edge and the way-along reading finds off it: the cover
    /// read the piece over its own length, the way along reads it over
    /// the root's. The lever is the geometry.
    JoinedPieceOffRoot {
        /// The joined piece, in the split's half.
        edge: EdgeKey,
    },
    /// Two band verdicts about one vertex, made over different lengths,
    /// disagree ([`DisagreeShape`]): edges flush only within the band
    /// read as on one another by one decision and off by the other. The
    /// lever is the geometry.
    BandVerdictsDisagree {
        /// The result-body vertex where they disagree.
        vertex: VertexKey,
        /// Which two verdicts.
        shape: DisagreeShape,
    },
    /// The N2 classification band could not be built from the ambient
    /// tolerance, so no discriminator below it can be decided.
    ///
    /// The cause is NOT unique — a validated
    /// [`Tolerance`](geom_core::tolerance::Tolerance) reaches
    /// [`BandError::InvalidValue`] when K·ε overflows to infinity and
    /// [`BandError::Empty`] when K·ε rounds back down onto ε, the two
    /// arms [`Band::linear`](geom_core::Band::linear)'s `# Errors`
    /// states with their conditions (stated there once, and not
    /// restated here).
    /// So the constructor's own diagnostic rides along rather than being
    /// relabelled as an emission inconsistency, which this is not:
    /// nothing about the result body is wrong here.
    Band(BandError),
    /// An N2 discriminator margin escalated in-band (typed, never a
    /// silent pick — spec D3).
    Escalated {
        /// The named predicate.
        predicate: &'static str,
        /// The escalation, unaltered.
        source: Indeterminate,
    },
}

/// **The one sentence every emission-bug refusal opens with**, written
/// once, and true of each of them: every category-1 variant of
/// [`NamingError`] reports an invariant the emitter relies on — its own
/// injectivity and totality, an upstream table's totality, a mint-time
/// fact's consistency with the body, an acyclic lineage — found broken.
/// Each speaks it followed by what it caught —
/// [`NamingError::Duplicate`] the name minted twice,
/// [`NamingError::EntityNamed`] the entity offered a second name, or
/// offered twice,
/// [`NamingError::KindMismatch`] the name seated on another kind,
/// [`NamingError::ShortTie`] the tie offered short,
/// [`NamingError::Unnamed`] the entity left uncovered,
/// [`NamingError::MissingUpstream`] the upstream table that missed,
/// [`NamingError::Emission`] a fact, [`NamingError::SplitLineage`] the
/// record it caught, [`NamingError::Settle`] the vertex — and a
/// reworded copy would let refusals of one category read as several.
const EMISSION_FRAMING: &str =
    "name emission found a kernel bug — an invariant it relies on does not hold";

/// **The one sentence every missing-rule refusal opens with**, written
/// once, and deliberately NOT a reworded copy of [`EMISSION_FRAMING`]:
/// the two say opposite things about whose fault the failure is, and an
/// author who reads the wrong one goes and files a kernel bug against a
/// recipe that is fine.
const UNRULED_FRAMING: &str =
    "no naming rule covers this construction; the recipe and its result are sound";

/// **The one sentence every precision-limit refusal opens with**: the
/// result may be sound, and the band the tolerance gives could not
/// decide what its name rests on. Neither a bug report nor a missing
/// rule, so a reworded copy of either would send the reader the wrong
/// way. Each such refusal ends in its own lever alone
/// ([`geom_core::lever_recourse`]).
const LIMIT_FRAMING: &str = "naming reached a precision limit of the tolerance band";

/// Declares a refusal's shape enum from one table: each shape, its doc
/// and the clause its refusal renders, so the enum's `ALL` holds every
/// shape by construction and no shape is added without its sentence.
macro_rules! shapes {
    (
        $(#[$doc:meta])*
        $name:ident {
            $($(#[$vdoc:meta])* $variant:ident => $text:expr,)*
        }
    ) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {
            $($(#[$vdoc])* $variant,)*
        }

        impl $name {
            /// Every shape, once.
            pub const ALL: &[Self] = &[$(Self::$variant),*];
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(match self {
                    $(Self::$variant => $text,)*
                })
            }
        }
    };
}

shapes! {
    /// Which shape a seam vertex's incident edges met
    /// ([`NamingError::UnsupportedSeamVertex`]).
    SeamVertexShape {
        /// A seam between two faces of one operand: it lies along the
        /// other operand's edge that filled the gap, and has no A and B
        /// side to cross.
        OneOperandSeam => "lies on a seam between two faces of one operand, which has no side of \
                           each operand to cross",
        /// One edge line meeting several faces across it and no seam.
        /// With planar faces no manifold result reaches this; curved
        /// faces are outside that argument.
        LineAcrossSeveralFaces => "lies on one edge line meeting several faces across it and no \
                                   seam",
        /// Any other parentage: one operand's edge with no single face
        /// of the other across it, its mirror, or several edges of each.
        NoEdgePair => "meets edges whose parents fit no shape a naming rule reads, so its \
                       parentage is half-decided",
    }
}

shapes! {
    /// What a crossing's piece classes read
    /// ([`NamingError::UnsupportedCrossingSenses`]).
    CrossingSensesShape {
        /// One side of a crossed edge holds pieces on both sides of the
        /// other operand: a vertex holding several keys classes one
        /// edge's pieces against two wedges.
        PiecesBothSides => "holds, on one side of a crossed edge, pieces on both sides of the \
                            other operand",
        /// Edges sharing one name read different senses: two tied edges
        /// at one vertex.
        NameReadsTwoSenses => "holds edges of one name that read different senses",
    }
}

shapes! {
    /// What the faces beside an edge named from them are
    /// ([`NamingError::UnsupportedSeamSides`]).
    SeamSidesShape {
        /// Not two faces: `Incidence` holds a wrap edge's one face once,
        /// and a wrap edge pierced at a point and joined back after the
        /// boolean is one way in.
        NotTwoFaces => "does not lie between two faces as the result's incidence reads it, and a \
                        seam is named from two",
        /// Its seam name is spelled alike by a seam between faces of one
        /// operand and one between faces of both: tied names can do that.
        NameSpellsTwoSeams => "takes a seam name that also spells a seam between faces of a \
                               different number of operands",
        /// A seam the join zipped between faces of one operand, not both
        /// merged: along an operand rim beside one merged face.
        WithinOneOperand => "is a zipped seam between faces of one operand that are not both \
                             merged",
        /// A union seam whose two faces descend from one parent: a pair
        /// can merge A's face with B's where the fold skips the same
        /// curved group.
        InsideOneParent => "is a union seam whose two faces descend from one parent",
    }
}

shapes! {
    /// What a joined edge's cover holds ([`NamingError::UnsupportedJoin`]).
    JoinShape {
        /// A split joined a section chord to a piece of an operand edge.
        ChordWithPiece => "joins a section chord to a piece of an operand edge, which no input \
                           cell reads",
        /// A split joined a section chord beside a merged operand face to
        /// whole operand edges: the seam rule reads the case where the
        /// merged face holds the section, not this mirror.
        ChordBesideMergedFace => "joins a section chord beside a merged operand face to whole \
                                  operand edges",
        /// A door joined an edge it minted outright (an end arc, a mitre,
        /// a slit, a remnant), which is the image of no input edge.
        Outright => "takes an edge its door minted outright, which no input cell reads",
    }
}

shapes! {
    /// What a name a union publishes met
    /// ([`NamingError::UnsupportedUnionName`]).
    UnionNameShape {
        /// It cites member faces of several parents: a tied member face
        /// is keyed to each candidate's parent, and those can differ.
        SeveralParents => "cites faces of several parents, so it names no one face",
        /// Two lines of a seam junction, met at different steps, collapse
        /// to one member-space line.
        JunctionLinesCollide => "is a seam junction two of whose lines collapse to one \
                                 member-space line",
    }
}

shapes! {
    /// Which way-along comparison decided zero
    /// ([`NamingError::WayInZeroBand`]).
    WayShape {
        /// On another edge's carrier, read at three stations: the two
        /// stretches compared, a quarter of the edge's length apart, span
        /// at least K/4 zero bands, so at K ≤ 4 they can decide zero.
        Carrier => "lies on another edge's carrier and runs neither way along it, the stretches \
                    compared a quarter of its length apart falling in the zero band",
        /// On another segment's line: each end may lie up to a zero band
        /// off the line, so a certified segment's projection can.
        Line => "lies on another segment's line and runs neither way along it, its projection on \
                 the line falling in the zero band",
    }
}

shapes! {
    /// Which two band verdicts disagree at a vertex
    /// ([`NamingError::BandVerdictsDisagree`]).
    DisagreeShape {
        /// The cover read the members over the joined edge's length; the
        /// vertex is placed over each member's.
        VertexOffJoinedSet => "is a vertex of a joined edge and lies on none of the operand \
                               edges its set holds",
        /// Two independent pairwise judgements of edges flush only within
        /// the band.
        FlushSensesDisagree => "is where two edges flush on one line, of union members or of \
                                a boolean's operands, disagree on its sense",
        /// A piece shorter than the band puts both its neighbours'
        /// classes at one point.
        PiecesAtOneVertex => "holds two pieces of one member line, each classified there",
        /// The cover's verdict and an end point's are two band decisions.
        JoinedEdgeOffMembers => "is on a union's joined edge that lies on none of the member \
                                 edges it is named for",
    }
}

impl DisagreeShape {
    /// The geometry move that takes this shape's two verdicts out of the
    /// band, about the entities its sentence names.
    const fn lever(self) -> &'static str {
        match self {
            Self::VertexOffJoinedSet => concat!(
                geom_core::coincidence_move_arm!(),
                " so the vertex lies clearly on or clearly off each edge of the set"
            ),
            Self::FlushSensesDisagree => concat!(
                geom_core::coincidence_move_arm!(),
                " so the edges on one line at the vertex are clearly flush or clearly apart"
            ),
            Self::PiecesAtOneVertex => concat!(
                geom_core::coincidence_move_arm!(),
                " so each piece of the member line at the vertex is longer than the band"
            ),
            Self::JoinedEdgeOffMembers => concat!(
                geom_core::coincidence_move_arm!(),
                " so the joined edge at the vertex lies clearly on or clearly off each member \
                 edge it is named for"
            ),
        }
    }
}

/// The lever of [`NamingError::WayInZeroBand`]: a larger ambiguity
/// factor K widens the escalate band against the zero band, so a
/// certified edge spans more zero bands.
const AMBIGUITY_FACTOR_LEVER: &str = "raise the tolerance's ambiguity factor K";

/// The lever of [`NamingError::JoinedPieceOffRoot`].
const JOINED_PIECE_LEVER: &str = concat!(
    geom_core::coincidence_move_arm!(),
    " so the joined piece lies clearly on or clearly off that operand edge"
);

/// What a shared-rim derivation found instead of the one edge it asked
/// for — the fact a free-text sentence would have hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RimShare {
    /// The two faces are not adjacent at all.
    NotAdjacent,
    /// They meet along more than one edge, so no single edge is the rim.
    Several,
}

impl core::fmt::Display for RimShare {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotAdjacent => write!(f, "no edge"),
            Self::Several => write!(f, "more than one edge"),
        }
    }
}

// #380: the refusal must NAME its subject. Every variant above is
// diagnosed precisely at the emitter — which name collided, which
// entity went uncovered, which upstream table missed, which
// consistency fact broke — and until this impl existed all of that
// died at the `EvalError::Naming` boundary, whose `Display` said only
// "name emission failed". Callers (Python's typed exception text
// included) had to BISECT scenes to relocate a wall the emitter had
// already located. Unlike the kernel-refusal payloads that ride the
// op variants unaltered (`NodeErrorKind`'s Display note, D2), this is
// editor-core's OWN error: rendering it IS the op's vocabulary, and
// there is no other path by which it reaches a human.
//
// It is raised only inside evaluation and memoized with the node's
// refusal, so it holds ids: each node and each name's minting node is
// said by the speaker of the frame that hands the refusal out.
impl crate::spoken::Say for NamingError {
    #[allow(clippy::too_many_lines)] // one arm per variant, each short
    fn say(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        by: crate::spoken::Speaker<'_>,
    ) -> core::fmt::Result {
        match self {
            // The role path rides along here, alone among the crate's
            // name renderings: a duplicate mint is a kernel bug report,
            // and its reader replays the mint from the path's structure.
            Self::Duplicate { name } => write!(
                f,
                "{EMISSION_FRAMING}: {} (role path {:?}) was minted twice — names \
                 alias silently only over the kernel's dead body",
                by.name(name),
                name.path
            ),
            // A held name that IS the offered one is a list that
            // offered one entity twice, and says so.
            Self::EntityNamed { name, held } if name == held => write!(
                f,
                "{EMISSION_FRAMING}: {} was offered the same {} twice — a name's \
                 candidates are distinct entities",
                by.name(name),
                name.kind.noun()
            ),
            Self::EntityNamed { name, held } => write!(
                f,
                "{EMISSION_FRAMING}: {} was seated on {} {} that {} already names — an \
                 entity holds one name",
                by.name(name),
                name.kind.article(),
                name.kind.noun(),
                by.name(held)
            ),
            Self::KindMismatch { name, found } => write!(
                f,
                "{EMISSION_FRAMING}: {} names {} {} and was seated on {} {} — a \
                 name denotes only entities of its own kind",
                by.name(name),
                name.kind.article(),
                name.kind.noun(),
                found.article(),
                found.noun()
            ),
            Self::ShortTie { name, candidates } => write!(
                f,
                "{EMISSION_FRAMING}: {} was offered as a tie of {} and refused — a tie names \
                 two or more entities",
                by.name(name),
                match candidates {
                    0 => "no candidates",
                    1 => "one candidate",
                    _ => "too few candidates",
                }
            ),
            Self::Unnamed { kind, body } => write!(
                f,
                "{EMISSION_FRAMING}: a live {} of output body {body} was left unnamed",
                kind.noun()
            ),
            Self::MissingUpstream { node } => write!(
                f,
                "{EMISSION_FRAMING}: the name table of upstream {} lacks an entity the \
                 emission needed",
                by.node(*node)
            ),
            Self::Emission { what } => write!(f, "{EMISSION_FRAMING}: {what}"),
            // The category IS an emission inconsistency, so the framing
            // is the same one; what the caught record adds is the locator.
            Self::SplitLineage(cycle) => write!(f, "{EMISSION_FRAMING}: {cycle}"),
            Self::Settle(e) => write!(f, "{EMISSION_FRAMING}: {e}"),
            // The framing is the missing-rule one, not the emission
            // one: what this refusal reports is the absence of a rule,
            // and the vertex is where a rule would be applied.
            Self::UnsupportedSeamVertex { vertex, shape } => {
                write!(f, "{UNRULED_FRAMING}: seam vertex {vertex:?} {shape}")
            }
            Self::UnsupportedCrossingSenses { vertex, shape } => write!(
                f,
                "{UNRULED_FRAMING}: crossing vertex {vertex:?} {shape}, so no rule reads its \
                 sense"
            ),
            Self::UnsupportedSeamSides { edge, shape } => {
                write!(f, "{UNRULED_FRAMING}: edge {edge:?} {shape}")
            }
            Self::UnsupportedJoin { edge, shape } => {
                write!(f, "{UNRULED_FRAMING}: joined edge {edge:?} {shape}")
            }
            Self::UnsupportedUnionName { name, shape } => {
                write!(f, "{UNRULED_FRAMING}: union name {} {shape}", by.name(name))
            }
            Self::UnsupportedTurnedCell { read, cell } => write!(
                f,
                "{UNRULED_FRAMING}: the Boolean turned a sphere of {} about its centre to find \
                 where it meets the other solid, and the result keeps {} {cell:?} where the \
                 turn put it, which descends from no cell of {}",
                by.read(*read),
                match cell {
                    topo::Cell::Vertex(_) => "its pole",
                    topo::Cell::Edge(_) => "a piece of its seam,",
                    topo::Cell::Face(_) => "its face",
                },
                by.read(*read),
            ),
            Self::WayInZeroBand { edge, shape } => {
                write!(f, "{LIMIT_FRAMING}: edge {edge:?} {shape}")
            }
            Self::JoinedPieceOffRoot { edge } => write!(
                f,
                "{LIMIT_FRAMING}: a split's joined piece {edge:?} lies off an operand edge its \
                 cover holds"
            ),
            Self::BandVerdictsDisagree { vertex, shape } => {
                write!(f, "{LIMIT_FRAMING}: vertex {vertex:?} {shape}")
            }
            // The node rides in the sentence, not only in the payload:
            // the two keys are an OPERAND body's, so a reader handed
            // them without it has nothing to resolve them against.
            Self::SharedRim {
                node,
                face,
                other,
                found,
            } => write!(
                f,
                "{UNRULED_FRAMING}: faces {face:?} and {other:?} of operand {}'s body \
                 share {found} where a seam chord's rim, derived from adjacency alone, needs \
                 exactly one",
                by.node(*node)
            ),
            Self::ChordOffRim { edge, node, face } => write!(
                f,
                "{UNRULED_FRAMING}: seam chord {edge:?} lies between two pieces of face {face:?} \
                 of operand {}'s body, off every rim the faces beside it share, and no rule \
                 names a chord within one operand face",
                by.node(*node)
            ),
            Self::SeamVertexPartners { vertex, candidates } => write!(
                f,
                "{UNRULED_FRAMING}: seam vertex {vertex:?} coincides, in the boolean's contact \
                 records, with {} differently named vertices of the other operand ({}), and \
                 no rule chooses which one names it",
                candidates.len(),
                candidates
                    .iter()
                    .map(|name| by.name(name).to_string())
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
            Self::ClosedCarrierUnread { edge, carrier } => write!(
                f,
                "{UNRULED_FRAMING}: joined edge {edge:?} is closed on its {} carrier, which has \
                 no period to read the operand edges it lies along over",
                carrier.name()
            ),
            Self::CarrierUnplaced { edge, carrier } => write!(
                f,
                "{UNRULED_FRAMING}: edge {edge:?} runs on a {} carrier, which gives no \
                 parameter for a point, so whether the edge holds a point it is read against \
                 cannot be said",
                carrier.name()
            ),
            Self::PartnerUnread {
                vertex,
                partner,
                unread,
            } => write!(
                f,
                "{UNRULED_FRAMING}: operand {}'s vertex {vertex:?} meets the other operand's \
                 vertex {partner:?}, which lies {} and which the boolean read nothing of, so it \
                 classed none of the vertex's edges and the crossing has no senses to name it \
                 by (work/tang/a-lone-unread-partner-has-no-class-rule)",
                match unread.operand {
                    topo::Operand::A => "A",
                    topo::Operand::B => "B",
                },
                match unread.neighbourhood {
                    topo::UnreadNeighbourhood::OneFace => "inside one smooth face",
                    topo::UnreadNeighbourhood::NoBisector => {
                        "on an edge whose two faces hold no bisector"
                    }
                    topo::UnreadNeighbourhood::ConeUnread => {
                        "at a polygon cone, some edge of which no reference decides"
                    }
                }
            ),
            Self::ConventionalVertex { vertex, body } => write!(
                f,
                "{UNRULED_FRAMING}: vertex {vertex:?} of output body {body} is a closed \
                 edge's conventional vertex, which has no identity of its own, and the name it \
                 takes from its edge is not built yet \
                 (work/seamrank/a-conventional-vertex-mints-no-edge-derived-name)"
            ),
            Self::MemberEdgeTied { member, edge } => write!(
                f,
                "{UNRULED_FRAMING}: member {}'s edge ({}) is read where one edge is needed, \
                 to place a vertex on it or read its faces, but the member ties that name to \
                 several edges",
                by.read(*member),
                by.name(edge)
            ),
            Self::Band(error) => write!(
                f,
                "the naming band could not be built from the ambient tolerance, so no \
                 name can be decided: {error}"
            ),
            Self::Escalated { predicate, source } => {
                let what = crate::decision::words(predicate).unwrap_or(geom_core::UNNAMED_DECISION);
                write!(
                    f,
                    "{}",
                    source.undecided(
                        format_args!("{what}, which a name rests on,"),
                        crate::decision::ending(predicate, source)
                    )
                )
            }
        }?;
        match self.ending() {
            Some(ending) => write!(f, ". {ending}"),
            None => Ok(()),
        }
    }
}

/// **Whose a naming refusal's failure is**: the kernel's, or a missing
/// rule's. Each category opens with its own framing and ends with its
/// own ending.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Category {
    /// An invariant the emitter relies on was found broken.
    Emission,
    /// A well-formed recipe reached a construction no rule names.
    Unruled,
    /// A verdict the band cannot give on a result that may be sound;
    /// each variant names its own lever ([`NamingError::lever`]).
    Limit,
}

impl Category {
    /// The sentence a refusal of this category opens with: each arm's
    /// `Display` writes it by hand, and the framing test holds the two
    /// equal.
    #[cfg(test)]
    pub(crate) fn framing(self) -> &'static str {
        match self {
            Self::Emission => EMISSION_FRAMING,
            Self::Unruled => UNRULED_FRAMING,
            Self::Limit => LIMIT_FRAMING,
        }
    }
}

impl NamingError {
    /// **The one place a variant's category is written down**, and it is
    /// a match, so a variant added without choosing one does not
    /// compile. `None` is a variant that speaks its own sentence and
    /// states its own recourse (the band, an escalation).
    pub(crate) fn category(&self) -> Option<Category> {
        match self {
            Self::Duplicate { .. }
            | Self::EntityNamed { .. }
            | Self::KindMismatch { .. }
            | Self::ShortTie { .. }
            | Self::Unnamed { .. }
            | Self::MissingUpstream { .. }
            | Self::Emission { .. }
            | Self::SplitLineage(_)
            | Self::Settle(_) => Some(Category::Emission),
            Self::UnsupportedSeamVertex { .. }
            | Self::SeamVertexPartners { .. }
            | Self::SharedRim { .. }
            | Self::ChordOffRim { .. }
            | Self::ConventionalVertex { .. }
            | Self::ClosedCarrierUnread { .. }
            | Self::CarrierUnplaced { .. }
            | Self::PartnerUnread { .. }
            | Self::MemberEdgeTied { .. }
            | Self::UnsupportedCrossingSenses { .. }
            | Self::UnsupportedSeamSides { .. }
            | Self::UnsupportedJoin { .. }
            | Self::UnsupportedUnionName { .. }
            | Self::UnsupportedTurnedCell { .. } => Some(Category::Unruled),
            Self::WayInZeroBand { .. }
            | Self::JoinedPieceOffRoot { .. }
            | Self::BandVerdictsDisagree { .. } => Some(Category::Limit),
            Self::Band(_) | Self::Escalated { .. } => None,
        }
    }

    /// The sentence a refusal ends with, by its category: the defect
    /// report, the not-yet ending, or a precision limit's own lever
    /// alone ([`geom_core::lever_recourse`]). `None` where the variant
    /// states its own.
    pub(crate) fn ending(&self) -> Option<String> {
        Some(match self.category()? {
            Category::Emission => geom_core::KERNEL_DEFECT_ENDING.to_owned(),
            Category::Unruled => geom_core::NOT_YET_ENDING.to_owned(),
            Category::Limit => geom_core::lever_recourse(self.lever(), None),
        })
    }

    /// The model's lever for a precision-limit variant.
    fn lever(&self) -> &'static str {
        match self {
            Self::WayInZeroBand { .. } => AMBIGUITY_FACTOR_LEVER,
            Self::JoinedPieceOffRoot { .. } => JOINED_PIECE_LEVER,
            Self::BandVerdictsDisagree { shape, .. } => shape.lever(),
            other => unreachable!(
                "{other:?} has no lever: `NamingError::category` puts exactly the three \
                 precision-limit variants in `Category::Limit`, and only those ask for one"
            ),
        }
    }
}

/// The refusal where no document is at hand: each node by its tag.
impl core::fmt::Display for NamingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::spoken::Say::say(self, f, crate::spoken::Speaker::TAG)
    }
}

// `Band::linear(tol)?` rather than a closure at the band door: one
// total conversion, so there is no site at which the caught
// `BandError` could be dropped again.
impl From<BandError> for NamingError {
    fn from(e: BandError) -> Self {
        Self::Band(e)
    }
}

// `body.split_root(..)?` rather than a closure at the chase site: a
// `map_err` closure is one keystroke from `map_err(|_| ..)`, and the
// `EdgeKey` this carries is the only locator a cycling lineage has.
impl From<SplitLineageCycle> for NamingError {
    fn from(e: SplitLineageCycle) -> Self {
        Self::SplitLineage(e)
    }
}

impl From<SettleError> for NamingError {
    fn from(e: SettleError) -> Self {
        Self::Settle(e)
    }
}

impl From<SeatingRefusal> for NamingError {
    fn from(e: SeatingRefusal) -> Self {
        match e {
            SeatingRefusal::Duplicate(d) => Self::Duplicate { name: d.name },
            SeatingRefusal::EntityNamed(n) => Self::EntityNamed {
                name: n.name,
                held: n.held,
            },
            SeatingRefusal::KindMismatch(k) => Self::KindMismatch {
                name: k.name,
                found: k.found,
            },
            SeatingRefusal::ShortTie(t) => Self::ShortTie {
                name: t.name,
                candidates: t.candidates,
            },
        }
    }
}

/// A name at `node` with a single role segment.
pub(crate) fn name1(kind: EntityKind, node: RecipeNodeId, seg: super::role::RoleSeg) -> StableName {
    StableName {
        kind,
        node,
        path: vec![seg],
    }
}

/// An entity in output body `body`.
pub(crate) fn ent(body: u32, key: EntityKey) -> EntityRef {
    EntityRef { body, key }
}

/// The empty table (nodes with no output bodies: profile, measure,
/// assertion, empty boolean).
pub(crate) fn empty() -> Arc<NameTable> {
    Arc::new(NameTable::new())
}

/// **An output-body index, as the table carries it.** [`EntityRef::body`]
/// is a `u32`, so a body count past that has no row to land in — the
/// one bound every multi-body value shares, which is why an index past
/// it is this layer's refusal wherever it is met (the evaluator's
/// placers and `Node::Part`, the emitters, the mate walk's `Part`
/// agreement) and not a number silently narrowed.
pub(crate) fn output_body(index: usize) -> Result<u32, NamingError> {
    u32::try_from(index).map_err(|_| NamingError::Emission {
        what: "an output-body index exceeds the table's u32 row width",
    })
}

/// **A count or index of what one body holds, narrowed to the `u32`
/// the name vocabulary and the table store.** A saturating or
/// truncating cast would hand two different values one stored number,
/// so a name or row would silently alias another. Every caller's `n`
/// is bounded below `u32::MAX` by what it counts — distinct entities of
/// one kind in one body, which a `topo` arena (a `SlotMap`, refusing
/// its `u32::MAX`-th slot) caps, or the few pieces a door builds — and
/// `what` names that subject and its bound, so past it is a kernel bug
/// and panics with both.
#[track_caller]
pub(crate) fn to_u32(n: usize, what: &'static str) -> u32 {
    let Ok(narrow) = u32::try_from(n) else {
        unreachable!("{what}: {n} does not fit the u32 a name stores")
    };
    narrow
}

/// **A family, named** (D10 `[repetition]`): every name of each
/// member's table wrapped `Member { at, of }`, its rows at the member's
/// own output body, the members in index order. The wrapping is
/// uniform: it wraps every name of a member, body rows included, so a
/// name of one member is the same selector on every member but its
/// `at`.
pub(crate) fn name_family<T: geom_core::Real>(
    node: RecipeNodeId,
    members: &[(super::role::MemberAt, &NameTable)],
    bodies: &[&Body<T>],
) -> Result<Arc<NameTable>, NamingError> {
    let mut t = NameTable::new();
    for (at_body, (at, member)) in members.iter().enumerate() {
        // A member's table read WHOLE seals here, as one read an entity
        // at a time seals in `upstream_name`.
        member.seal_order();
        let body = output_body(at_body)?;
        let place = |e: &EntityRef| -> Result<EntityRef, NamingError> {
            if e.body != 0 {
                return Err(NamingError::Emission {
                    what: "a family's member names a body the member does not have",
                });
            }
            Ok(ent(body, e.key))
        };
        for (name, entry) in member.iter_refs() {
            let wrapped = StableName {
                kind: name.kind,
                node,
                path: vec![super::role::RoleSeg::Member {
                    at: at.clone(),
                    of: name.clone(),
                }],
            };
            match entry {
                super::table::Entry::Unique(e) => t.insert(wrapped, place(e)?)?,
                super::table::Entry::Tied(es) => {
                    let rows = es.iter().map(place).collect::<Result<Vec<_>, _>>()?;
                    t.insert_tied(wrapped, rows)?;
                }
            }
        }
    }
    for (i, body) in bodies.iter().enumerate() {
        check_total(&t, body, output_body(i)?)?;
    }
    Ok(Arc::new(t))
}

/// **A copy, named** (D10 `[placement.placement]`): a placement's or
/// an instance's copy keeps every name of its source, so a placement
/// adds no name segment. The copy's own body row is minted here, at the
/// copying node, since the copy is that node's output; a family's rows,
/// body rows included, pass through whole.
///
/// Keys hold verbatim, because a copy is its source's body under its
/// frame.
pub(crate) fn name_copy<T: geom_core::Real>(
    node: RecipeNodeId,
    source: &NameTable,
    placed: &[&Body<T>],
) -> Result<Arc<NameTable>, NamingError> {
    source.seal_order();
    let one = placed.len() == 1;
    let mut t = NameTable::new();
    if one {
        t.insert(
            name1(EntityKind::Body, node, super::role::RoleSeg::OutputBody),
            ent(0, EntityKey::Body),
        )?;
    }
    for (name, entry) in source.iter_refs() {
        if one && name.kind == EntityKind::Body {
            continue;
        }
        match entry {
            super::table::Entry::Unique(e) => t.insert((**name).clone(), *e)?,
            super::table::Entry::Tied(es) => t.insert_tied((**name).clone(), es.clone())?,
        }
    }
    for (ix, body) in placed.iter().enumerate() {
        check_total(&t, body, output_body(ix)?)?;
    }
    Ok(Arc::new(t))
}

/// **A `⊔`'s one body, named** (D10 `[operations.operators]`): every
/// name of each member, keyed by the member's read
/// ([`super::role::RoleSeg::From`], as a union keys its members),
/// re-keyed onto the grafted body through that member's graft bridges
/// (`members[m].2[i]` is the bridge of the member's body `i`). The
/// body row is the node's own; a member's body rows have nothing left
/// to point at.
pub(crate) fn name_assemble<T: geom_core::Real>(
    node: RecipeNodeId,
    members: &[(crate::VarId, Arc<NameTable>, Vec<topo::GraftKeys>)],
    fused: &Body<T>,
) -> Result<Arc<NameTable>, NamingError> {
    let mut t = NameTable::new();
    t.insert(
        name1(EntityKind::Body, node, super::role::RoleSeg::OutputBody),
        ent(0, EntityKey::Body),
    )?;
    let mut by_name: BTreeMap<StableName, Vec<EntityRef>> = BTreeMap::new();
    for (read, table, bridges) in members {
        table.seal_order();
        for (name, entry) in table.iter_refs() {
            if name.kind == EntityKind::Body {
                continue;
            }
            let rows: Vec<EntityRef> = match entry {
                super::table::Entry::Unique(e) => vec![*e],
                super::table::Entry::Tied(es) => es.clone(),
            };
            let moved = rows
                .into_iter()
                .map(|e| {
                    let keys = bridges.get(e.body as usize).ok_or(NamingError::Emission {
                        what: "a member's table names a body the member does not have",
                    })?;
                    let key = match e.key {
                        EntityKey::Body => None,
                        EntityKey::Face(f) => keys.face(f).map(EntityKey::Face),
                        EntityKey::Edge(e) => keys.edge(e).map(EntityKey::Edge),
                        EntityKey::Vertex(v) => keys.vertex(v).map(EntityKey::Vertex),
                    };
                    key.map(|key| ent(0, key)).ok_or(NamingError::Emission {
                        what: "a member's graft bridge is not total over its entities",
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            let keyed = StableName {
                kind: name.kind,
                node,
                path: vec![super::role::RoleSeg::From {
                    read: *read,
                    of: name.clone(),
                }],
            };
            by_name.entry(keyed).or_default().extend(moved);
        }
    }
    // A read assembled twice keys its two pieces' entities alike: each
    // such name is a tie of both, whose candidates keep their identity.
    for (keyed, moved) in by_name {
        match moved.as_slice() {
            [one] => t.insert(keyed, *one)?,
            _ => t.insert_tied(keyed, moved)?,
        }
    }
    check_total(&t, fused, 0)?;
    Ok(Arc::new(t))
}

/// Per-body topology walk products the emitters share: half-edge
/// incidence (vertex → edges, edge → faces) built in ONE deterministic
/// arena-order pass — combinatorial wiring facts, not inspection.
pub(crate) struct Incidence {
    /// Vertex → incident edges (sorted, deduplicated).
    pub vertex_edges: BTreeMap<VertexKey, Vec<EdgeKey>>,
    /// Edge → the (≤ 2) faces of its halves' loops.
    pub edge_faces: BTreeMap<EdgeKey, Vec<FaceKey>>,
}

impl Incidence {
    /// Builds the incidence maps for one body.
    ///
    /// # Panics
    ///
    /// For a half-edge whose loop is dead, which a tier-1-valid body
    /// never holds.
    #[track_caller]
    pub(crate) fn of<T: geom_core::Real>(body: &Body<T>) -> Self {
        let mut vertex_edges: BTreeMap<VertexKey, Vec<EdgeKey>> = BTreeMap::new();
        let mut edge_faces: BTreeMap<EdgeKey, Vec<FaceKey>> = BTreeMap::new();
        for (k, he) in body.half_edges() {
            vertex_edges.entry(he.start).or_default().push(he.edge);
            let Some(l) = body.get_loop(he.parent_loop) else {
                unreachable!(
                    "incidence walk: dangling loop {:?} of half-edge {k:?}; {TIER1}",
                    he.parent_loop
                )
            };
            edge_faces.entry(he.edge).or_default().push(l.face);
        }
        for v in vertex_edges.values_mut() {
            v.sort_unstable();
            v.dedup();
        }
        for f in edge_faces.values_mut() {
            f.sort_unstable();
            f.dedup();
        }
        Self {
            vertex_edges,
            edge_faces,
        }
    }
}

/// Why a link a record of the body holds resolves: the doors hand the
/// emitters tier-1-valid bodies.
const TIER1: &str = "a tier-1-valid body keeps every key it links live and every half-edge mated";

/// Why a key a caller passes resolves: every caller reads it, in the
/// same call, off a live record of the same body — its arena
/// iteration, a link of a live entity, or a row of the table emitted
/// for it, which names only that body's live entities. A live-key
/// brand would carry this as a type (`topo`'s `Live`), but it would
/// spread into every emitter signature naming a key, which do not
/// otherwise care, so row 0 is "no" here.
const LIVE_ARGUMENT: &str = "every caller reads the key off a live record of this body in the \
                             same call";

/// What the rim walk found. A caller MUST write down which of the two
/// it means, because the two mean different things to different callers.
pub(crate) enum Rim {
    /// The one edge the pair shares.
    One(EdgeKey),
    /// They do not share exactly one, and which way.
    NotOne(RimShare),
}

/// The one edge face `f` and face `g` share, or which way they do not
/// have one.
///
/// **This walk reports; it does not classify.** Whether "not exactly
/// one" is a kernel bug depends entirely on the CALLER's premise, and
/// the walk cannot see it: `emit_sweep` asks about a body minted
/// alongside the key bundle it is naming from, where a wall meets each
/// cap along one rim by construction and any other answer is a
/// contradicted bundle; `emit_topo` asks an OPERAND body whether two
/// descended faces carry a seam chord's rim, which is a guess a later
/// split can legitimately refute.
///
/// The return is a [`Rim`], so a caller must **write** its
/// classification: the cardinality arrives as a value to be matched,
/// and no caller inherits a verdict by saying nothing.
///
/// What the walk does NOT report is structural corruption of `body`
/// (a dead key, an unmated half-edge): that is about the body rather
/// than anybody's premise, and the doors keep every body an emitter
/// reads tier-1-valid, so it panics as a kernel bug under every caller.
///
/// Named `rim_between` rather than `shared_rim`: `topo`'s
/// `boolean::rim_wedge::shared_rim` answers the same question one layer
/// down, and one name for two functions in two crates costs every
/// future reader a `grep` they cannot disambiguate.
#[track_caller]
pub(crate) fn rim_between<T: geom_core::Real>(body: &Body<T>, f: FaceKey, g: FaceKey) -> Rim {
    match rims_between(body, f, g).as_slice() {
        [] => Rim::NotOne(RimShare::NotAdjacent),
        [e] => Rim::One(*e),
        _ => Rim::NotOne(RimShare::Several),
    }
}

/// EVERY edge face `f` and face `g` share, each once, in `f`'s
/// half-edge order — the walk [`rim_between`] classifies, for a caller
/// whose premise lets it choose among several by something else (the
/// boolean pass's chord, [`crate::names::emit_topo`]). Structural
/// corruption panics exactly as [`rim_between`] documents.
#[track_caller]
pub(crate) fn rims_between<T: geom_core::Real>(
    body: &Body<T>,
    f: FaceKey,
    g: FaceKey,
) -> Vec<EdgeKey> {
    let mut found: Vec<EdgeKey> = Vec::new();
    for he in face_half_edges(body, f) {
        let Some(mate) = body.mate(he) else {
            unreachable!("rim walk: unmated half-edge {he:?} of face {f:?}; {TIER1}")
        };
        let Some(mate_he) = body.get_half_edge(mate) else {
            unreachable!(
                "rim walk: dangling mate {mate:?} of half-edge {he:?} of face {f:?}; {TIER1}"
            )
        };
        let Some(other) = body.get_loop(mate_he.parent_loop) else {
            unreachable!(
                "rim walk: dangling loop {:?} of mate {mate:?} of face {f:?}; {TIER1}",
                mate_he.parent_loop
            )
        };
        if other.face == g && !found.contains(&mate_he.edge) {
            found.push(mate_he.edge);
        }
    }
    found
}

/// A vertex's point in `body` — the one reader of it in this module's
/// emitters.
///
/// # Panics
///
/// For a dead vertex ([`LIVE_ARGUMENT`]: `v` is the caller's key) or a
/// vertex without a point ([`TIER1`]: a link the vertex holds).
#[track_caller]
pub(crate) fn vertex_point<T: geom_core::Real>(
    body: &Body<T>,
    v: VertexKey,
) -> geom_core::Point3<T> {
    let Some(vd) = body.get_vertex(v) else {
        unreachable!("vertex point: dangling vertex {v:?}; {LIVE_ARGUMENT}")
    };
    let Some(p) = body.get_point(vd.point) else {
        unreachable!(
            "vertex point: dead point {:?} of vertex {v:?}; {TIER1}",
            vd.point
        )
    };
    *p
}

/// All half-edges of a face (outer loop + rings), deterministic
/// order.
///
/// # Panics
///
/// For a dead face ([`LIVE_ARGUMENT`]: `f` is the caller's key), or
/// where [`loop_half_edges`] panics on one of its loops ([`TIER1`]).
#[track_caller]
pub(crate) fn face_half_edges<T: geom_core::Real>(body: &Body<T>, f: FaceKey) -> Vec<HalfEdgeKey> {
    let Some(face) = body.get_face(f) else {
        unreachable!("face walk: dangling face {f:?}; {LIVE_ARGUMENT}")
    };
    let mut out = Vec::new();
    for l in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
        out.extend(loop_half_edges(body, l));
    }
    out
}

/// The half-edges of loop `l`, in cycle order (none for an empty loop).
///
/// # Panics
///
/// For a dead loop, or a cycle that does not close ([`TIER1`]): every
/// caller reads `l` off a live face of `body` ([`face_half_edges`], the
/// plane pieces' outline walk), a link that face holds.
#[track_caller]
pub(crate) fn loop_half_edges<T: geom_core::Real>(body: &Body<T>, l: LoopKey) -> Vec<HalfEdgeKey> {
    let Some(lp) = body.get_loop(l) else {
        unreachable!("face walk: dangling loop {l:?}; {TIER1}")
    };
    let topo::LoopBoundary::Cycle { first } = lp.boundary else {
        return Vec::new();
    };
    let Some(cycle) = body.loop_cycle(first) else {
        unreachable!("face walk: unwalkable loop {l:?} from {first:?}; {TIER1}")
    };
    cycle
}

/// The two endpoint vertices of an edge.
///
/// # Panics
///
/// For a dead edge ([`LIVE_ARGUMENT`]: `e` is the caller's key) or a
/// broken link from its `he_plus` ([`TIER1`]).
#[track_caller]
pub(crate) fn edge_ends<T: geom_core::Real>(body: &Body<T>, e: EdgeKey) -> (VertexKey, VertexKey) {
    let Some(edge) = body.get_edge(e) else {
        unreachable!("edge ends: dangling edge {e:?}; {LIVE_ARGUMENT}")
    };
    let Some(plus) = body.get_half_edge(edge.he_plus) else {
        unreachable!(
            "edge ends: dangling he_plus {:?} of edge {e:?}; {TIER1}",
            edge.he_plus
        )
    };
    let Some(end) = body.half_edge_end(edge.he_plus) else {
        unreachable!(
            "edge ends: he_plus has no end: {:?} of edge {e:?}; {TIER1}",
            edge.he_plus
        )
    };
    (plus.start, end)
}

/// TOTALITY (spec D4): every live face/edge/vertex of `body` (output
/// body index `ix`) is covered by the table, plus the body row
/// itself.
pub(crate) fn check_total<T: geom_core::Real>(
    table: &NameTable,
    body: &Body<T>,
    ix: u32,
) -> Result<(), NamingError> {
    let miss = |kind| NamingError::Unnamed { kind, body: ix };
    if table.name_of(&ent(ix, EntityKey::Body)).is_none() {
        return Err(miss(EntityKind::Body));
    }
    for (f, _) in body.faces() {
        if table.name_of(&ent(ix, EntityKey::Face(f))).is_none() {
            return Err(miss(EntityKind::Face));
        }
    }
    for (e, _) in body.edges() {
        if table.name_of(&ent(ix, EntityKey::Edge(e))).is_none() {
            return Err(miss(EntityKind::Edge));
        }
    }
    for (v, _) in body.vertices() {
        let Some(name) = table.name_of(&ent(ix, EntityKey::Vertex(v))) else {
            return Err(miss(EntityKind::Vertex));
        };
        // A conventional vertex has no identity of its own: no rule may
        // name it for a member or a position. A construction's own rim
        // self-loop vertex is that construction's, and keeps its name.
        let own_rim = matches!(
            super::words::role_leaf(name).path.as_slice(),
            [super::role::RoleSeg::MeridianVertex(
                super::role::MeridianEnd::Seam,
                _
            )]
        );
        if topo::is_conventional_vertex(body, v) && !own_rim {
            return Err(NamingError::ConventionalVertex {
                vertex: v,
                body: ix,
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod display_tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use geom_core::{Band, MarginDiag};
    use topo::construction::Construct;

    use super::*;
    use crate::eval::NodeErrorKind;
    use crate::names::role::RoleSeg;

    fn escalation() -> Indeterminate {
        Indeterminate {
            margin: MarginDiag::INVALID,
            band: Band::new(1e-9, 1e-6).unwrap(),
            predicate: Some("side_of_plane"),
            terminal_sliver: false,
        }
    }

    /// **Two distinct edge keys, out of a real arena.** `EdgeKey` is a
    /// slotmap key and nothing in this crate mints one by hand, so the
    /// arena is the only source; nothing below depends on their VALUES,
    /// only on their being distinct and rendering distinctly.
    fn two_edges() -> (EdgeKey, EdgeKey) {
        // Two solids in ONE arena: keys are per-body, so two bodies
        // would hand out the same index twice.
        let mut body = topo::Body::<f64>::new();
        let mut mint = |x: f64| {
            let born = body
                .mvfs(geom_core::Point3::new(x, 0.0, 0.0), true)
                .expect("mvfs births a solid, shell, face and lone vertex");
            body.mev_line(
                topo::construction::MevSite::Lone {
                    r#loop: born.r#loop,
                },
                geom_core::Point3::new(x + 1.0, 0.0, 0.0),
                geom_core::Tol::witness(),
            )
            .expect("mev on an empty loop grows it by one edge")
            .edge
        };
        let (a, b) = (mint(0.0), mint(10.0));
        assert_ne!(a, b, "two DISTINCT keys, or the rows below prove nothing");
        (a, b)
    }

    /// **Two distinct face keys, out of a real arena.** `FaceKey` is a
    /// slotmap key and nothing in this crate mints one by hand, so the
    /// arena is the only source; two solids in ONE arena is what makes
    /// the keys different, and nothing below depends on their VALUES.
    fn two_faces() -> (FaceKey, FaceKey) {
        let mut body = topo::Body::<f64>::new();
        let mut mint = |x: f64| {
            body.mvfs(geom_core::Point3::new(x, 0.0, 0.0), true)
                .expect("mvfs births a solid, shell, face and lone vertex")
                .face
        };
        let (a, b) = (mint(0.0), mint(10.0));
        assert_ne!(a, b, "two DISTINCT keys, or the rows below prove nothing");
        (a, b)
    }

    /// Two DISTINCT vertex keys, minted the same way.
    fn two_vertices() -> (VertexKey, VertexKey) {
        let mut body = topo::Body::<f64>::new();
        let mut mint = |x: f64| {
            body.mvfs(geom_core::Point3::new(x, 0.0, 0.0), true)
                .expect("mvfs births a solid, shell, face and lone vertex")
                .vertex
        };
        let (a, b) = (mint(0.0), mint(10.0));
        assert_ne!(a, b, "two DISTINCT keys, or the rows below prove nothing");
        (a, b)
    }

    /// **The cycling edge survives the conversion and the node
    /// boundary — and it is THIS edge, not a constant.** The node-level
    /// prose is the only route by which an emitter refusal reaches a
    /// human (Python's typed exception message is exactly this string),
    /// so the locator is pinned there rather than at `NamingError`'s own
    /// `Display`. Two different keys must give two different sentences:
    /// a refusal that reads the same for both has no locator, which is
    /// the defect `map_err(|_| ..)` used to have here.
    #[test]
    fn the_cycling_edge_reaches_the_node_level_prose() {
        let (a, b) = two_edges();
        let carried =
            |edge| NodeErrorKind::Naming(NamingError::from(SplitLineageCycle { edge })).to_string();

        let sa = carried(a);
        assert!(
            sa.contains(&format!("{a:?}")),
            "the cycling edge is the only locator this failure has: {sa}"
        );
        let sb = carried(b);
        assert!(sb.contains(&format!("{b:?}")), "{sb}");
        assert_ne!(
            sa, sb,
            "a refusal that reads the same for two edges has no locator"
        );
        assert!(
            !sa.contains(&format!("{b:?}")),
            "the refusal names the edge it caught, not another: {sa}"
        );

        // The category stays honest: a corrupt birth record IS an
        // emission inconsistency, so the framing sentence is the one
        // `Emission` speaks and the node boundary's own word survives it.
        assert!(sa.contains(EMISSION_FRAMING), "category kept: {sa}");
        assert!(
            sa.contains("name emission failed"),
            "node category kept: {sa}"
        );

        // `crate::py::typed_err` (pncad-py) asserts `reads_as_prose` on
        // every raise, live under release, and its fingerprint is the
        // field brace. A payload rendered through a derived `Debug` is
        // how that assertion gets broken, and this refusal carries one.
        for s in [&sa, &sb] {
            assert!(
                !s.contains(" { "),
                "a braced payload panics the Python binding at the arm \
                 meant to refuse gracefully: {s}"
            );
        }
    }

    /// Every variant names WHICH entity/node/fact it refused on.
    ///
    /// **The enumeration is the compiler's claim, not prose**: the
    /// `sampled` match below is exhaustive, so a new variant has no arm
    /// until someone writes one, and the arm names the row that samples
    /// it, so a row that goes missing reds the set comparison.
    #[test]
    fn every_variant_names_its_subject() {
        let name = StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId::new(0, test_utils::refusal::tagged(7)),
            path: vec![RoleSeg::Cap(super::super::role::CapEnd::End)],
        };
        // Locators the rows below sample by value, so a refusal that
        // renders a constant where its subject belongs fails here.
        let vtx = two_vertices().0;
        let vtx_shown = format!("{vtx:?}");
        // Two DIFFERENTLY named partners, as the refusal's premise has
        // them: the same cap vertex of two different operands' sweeps.
        let partner = |node| StableName {
            kind: EntityKind::Vertex,
            node: RecipeNodeId::new(0, test_utils::refusal::tagged(node)),
            path: vec![RoleSeg::CapVertex(
                super::super::role::CapEnd::End,
                super::super::role::ProfileVertexRef::Piece {
                    step: crate::node::StepId::new(0, test_utils::refusal::tagged(0)),
                    role: crate::names::PieceRole::Leg,
                },
            )],
        };
        let pair = two_faces();
        assert_ne!(
            pair.0, pair.1,
            "the pair row proves nothing on one key twice"
        );
        let (face0, face1) = (format!("{:?}", pair.0), format!("{:?}", pair.1));
        let edges = two_edges();
        let (edge0, edge1) = (format!("{:?}", edges.0), format!("{:?}", edges.1));
        let rows: Vec<(NamingError, Vec<&str>)> = vec![
            (
                NamingError::Duplicate {
                    name: Box::new(name.clone()),
                },
                vec!["the end cap", "7", "Cap"],
            ),
            (
                NamingError::Unnamed {
                    kind: EntityKind::Edge,
                    body: 3,
                },
                vec!["edge", "3"],
            ),
            (
                NamingError::MissingUpstream {
                    node: RecipeNodeId::new(0, test_utils::refusal::tagged(11)),
                },
                vec!["00000000000b"],
            ),
            (
                // A refusal that still EXISTS (ties propagate, so there
                // is no tied-upstream refusal to sample): a sample
                // payload that greps to nothing would outlive its own
                // subject.
                NamingError::Emission {
                    what: "a chart cut survives in the result",
                },
                vec!["a chart cut survives in the result"],
            ),
            (
                NamingError::Escalated {
                    predicate: crate::names::discriminate::ON_MEMBER_EDGE,
                    source: escalation(),
                },
                vec!["a point's place along an edge"],
            ),
            (
                NamingError::SplitLineage(SplitLineageCycle {
                    edge: two_edges().0,
                }),
                vec!["split lineage of edge"],
            ),
            (
                NamingError::UnsupportedSeamVertex {
                    vertex: vtx,
                    shape: SeamVertexShape::LineAcrossSeveralFaces,
                },
                vec![vtx_shown.as_str(), "several faces across it"],
            ),
            (
                NamingError::UnsupportedCrossingSenses {
                    vertex: vtx,
                    shape: CrossingSensesShape::NameReadsTwoSenses,
                },
                vec![vtx_shown.as_str(), "read different senses"],
            ),
            (
                NamingError::UnsupportedSeamSides {
                    edge: edges.0,
                    shape: SeamSidesShape::InsideOneParent,
                },
                vec![edge0.as_str(), "descend from one parent"],
            ),
            (
                NamingError::UnsupportedJoin {
                    edge: edges.1,
                    shape: JoinShape::Outright,
                },
                vec![edge1.as_str(), "minted outright"],
            ),
            (
                NamingError::UnsupportedUnionName {
                    name: Box::new(name.clone()),
                    shape: UnionNameShape::SeveralParents,
                },
                vec!["the end cap", "faces of several parents"],
            ),
            (
                NamingError::WayInZeroBand {
                    edge: edges.0,
                    shape: WayShape::Carrier,
                },
                vec![edge0.as_str(), "runs neither way", "ambiguity factor K"],
            ),
            (
                NamingError::JoinedPieceOffRoot { edge: edges.1 },
                vec![
                    edge1.as_str(),
                    "lies off an operand edge",
                    "that operand edge",
                ],
            ),
            (
                NamingError::UnsupportedTurnedCell {
                    read: crate::VarId::new(0, test_utils::refusal::tagged(38)),
                    cell: topo::Cell::Vertex(vtx),
                },
                vec!["turned a sphere", "its pole", "descends from no cell"],
            ),
            (
                NamingError::BandVerdictsDisagree {
                    vertex: vtx,
                    shape: DisagreeShape::FlushSensesDisagree,
                },
                vec![
                    vtx_shown.as_str(),
                    "disagree on its sense",
                    "move the geometry",
                ],
            ),
            (
                // ONE arena: two `two_faces()` calls hand out keys from
                // two bodies, which are not guaranteed distinct.
                NamingError::SharedRim {
                    node: RecipeNodeId::new(0, test_utils::refusal::tagged(23)),
                    face: pair.0,
                    other: pair.1,
                    found: RimShare::Several,
                },
                vec![
                    face0.as_str(),
                    face1.as_str(),
                    "000000000017",
                    "more than one edge",
                ],
            ),
            (
                NamingError::SeamVertexPartners {
                    vertex: vtx,
                    candidates: vec![partner(3), partner(4)],
                },
                vec![
                    vtx_shown.as_str(),
                    "2 differently named vertices",
                    "of node 000000000003",
                    "of node 000000000004",
                ],
            ),
            (
                NamingError::ClosedCarrierUnread {
                    edge: EdgeKey::default(),
                    carrier: geom::CurveKind::Nurbs,
                },
                vec!["closed on its", "no period"],
            ),
            (
                NamingError::CarrierUnplaced {
                    edge: EdgeKey::default(),
                    carrier: geom::CurveKind::Nurbs,
                },
                vec!["gives no parameter for a point", "holds a point"],
            ),
            (
                NamingError::PartnerUnread {
                    vertex: two_vertices().0,
                    partner: two_vertices().1,
                    unread: topo::UnreadPartner {
                        operand: topo::Operand::B,
                        vertex: two_vertices().0,
                        partner: two_vertices().1,
                        neighbourhood: topo::UnreadNeighbourhood::OneFace,
                    },
                },
                vec![
                    "inside one smooth face",
                    "classed none of the vertex's edges",
                    "work/tang/a-lone-unread-partner-has-no-class-rule",
                ],
            ),
            (
                NamingError::Settle(topo::SettleError::Dead {
                    vertex: two_vertices().0,
                    reached: topo::Cell::Vertex(two_vertices().1),
                }),
                vec!["a vertex of a Boolean's result settles on a cell"],
            ),
            (
                NamingError::ConventionalVertex {
                    vertex: topo::VertexKey::default(),
                    body: 3,
                },
                vec![
                    "conventional vertex",
                    "output body 3",
                    "a-conventional-vertex",
                ],
            ),
            (
                NamingError::MemberEdgeTied {
                    member: crate::VarId::new(0, test_utils::refusal::tagged(37)),
                    edge: Box::new(StableName {
                        kind: EntityKind::Edge,
                        node: RecipeNodeId::new(0, test_utils::refusal::tagged(37)),
                        path: vec![RoleSeg::LateralEdge(
                            super::super::role::ProfileVertexRef::Piece {
                                step: crate::node::StepId::new(0, test_utils::refusal::tagged(2)),
                                role: crate::names::PieceRole::Leg,
                            },
                        )],
                    }),
                },
                vec![
                    "member read 000000000025",
                    "of node 000000000025)",
                    "ties that name to several edges",
                ],
            ),
            (
                // The band's subject is the pair of thresholds that
                // could not separate: which end of the axis the ambient
                // tolerance landed on is what tells the reader whether
                // to raise eps or lower K.
                NamingError::Band(BandError::Empty {
                    zero: 5e-324,
                    escalate: 5e-324,
                }),
                vec!["5e-324"],
            ),
            (
                NamingError::KindMismatch {
                    name: Box::new(name.clone()),
                    found: EntityKind::Edge,
                },
                vec!["the end cap", "names a face", "seated on an edge"],
            ),
            (
                NamingError::ShortTie {
                    name: Box::new(name.clone()),
                    candidates: 1,
                },
                vec!["the end cap", "a tie of one candidate"],
            ),
            (
                NamingError::EntityNamed {
                    name: Box::new(name.clone()),
                    held: Box::new(StableName {
                        path: vec![RoleSeg::Cap(super::super::role::CapEnd::Start)],
                        ..name.clone()
                    }),
                },
                vec![
                    "the end cap",
                    "seated on a face",
                    "the start cap",
                    "already names",
                ],
            ),
            (
                NamingError::ChordOffRim {
                    edge: two_edges().0,
                    node: RecipeNodeId::new(0, test_utils::refusal::tagged(29)),
                    face: pair.0,
                },
                vec![
                    face0.as_str(),
                    "00000000001d",
                    "two pieces of face",
                    "within one operand face",
                ],
            ),
        ];
        // The category is `NamingError::category`'s, which also chooses
        // the ending. Which framing each arm's `Display` interpolates is
        // written by hand per arm, so this ties the second to the first;
        // which word `naming_error_tag` returns is across a crate
        // boundary and still hand-held.
        //
        // `None` is a variant that speaks its OWN sentence, and it is
        // not a free pass: the loop below asserts such a variant
        // contains NEITHER framing nor either ending, so a bespoke
        // sentence that drifted into one of them reds.
        let expected_framing = |err: &NamingError| err.category().map(Category::framing);
        let sampled = |err: &NamingError| -> usize {
            match err {
                NamingError::Duplicate { .. } => 0,
                NamingError::Unnamed { .. } => 1,
                NamingError::MissingUpstream { .. } => 2,
                NamingError::Emission { .. } => 3,
                NamingError::Escalated { .. } => 4,
                NamingError::SplitLineage(_) => 5,
                NamingError::UnsupportedSeamVertex { .. } => 6,
                NamingError::SharedRim { .. } => 7,
                NamingError::SeamVertexPartners { .. } => 8,
                NamingError::Band(_) => 9,
                NamingError::MemberEdgeTied { .. } => 10,
                NamingError::ConventionalVertex { .. } => 11,
                NamingError::ClosedCarrierUnread { .. } => 12,
                NamingError::Settle(_) => 13,
                NamingError::CarrierUnplaced { .. } => 14,
                NamingError::PartnerUnread { .. } => 15,
                NamingError::KindMismatch { .. } => 16,
                NamingError::ShortTie { .. } => 17,
                NamingError::EntityNamed { .. } => 18,
                NamingError::ChordOffRim { .. } => 19,
                NamingError::UnsupportedCrossingSenses { .. } => 20,
                NamingError::UnsupportedSeamSides { .. } => 21,
                NamingError::UnsupportedJoin { .. } => 22,
                NamingError::UnsupportedUnionName { .. } => 23,
                NamingError::WayInZeroBand { .. } => 24,
                NamingError::BandVerdictsDisagree { .. } => 25,
                NamingError::JoinedPieceOffRoot { .. } => 26,
                NamingError::UnsupportedTurnedCell { .. } => 27,
            }
        };
        let covered: std::collections::BTreeSet<usize> =
            rows.iter().map(|(e, _)| sampled(e)).collect();
        assert_eq!(
            covered,
            (0..rows.len()).collect::<std::collections::BTreeSet<_>>(),
            "a row was deleted from the middle of the table. An appended \
             VARIANT with no row is invisible here — that is welded in \
             tests/display_contract.rs by set difference, not by this \
             contiguity check."
        );
        // Every shape of every variant that carries one, beside the one
        // row per variant above: each shape's sentence, and the lever a
        // precision-limit shape ends in, is its own text, so each is
        // checked on its own.
        let mut every_shape: Vec<(NamingError, String)> = Vec::new();
        for &shape in SeamVertexShape::ALL {
            let err = NamingError::UnsupportedSeamVertex { vertex: vtx, shape };
            every_shape.push((err, shape.to_string()));
        }
        for &shape in CrossingSensesShape::ALL {
            let err = NamingError::UnsupportedCrossingSenses { vertex: vtx, shape };
            every_shape.push((err, shape.to_string()));
        }
        for &shape in SeamSidesShape::ALL {
            let err = NamingError::UnsupportedSeamSides {
                edge: edges.0,
                shape,
            };
            every_shape.push((err, shape.to_string()));
        }
        for &shape in JoinShape::ALL {
            let err = NamingError::UnsupportedJoin {
                edge: edges.0,
                shape,
            };
            every_shape.push((err, shape.to_string()));
        }
        for &shape in UnionNameShape::ALL {
            let err = NamingError::UnsupportedUnionName {
                name: Box::new(name.clone()),
                shape,
            };
            every_shape.push((err, shape.to_string()));
        }
        for &shape in WayShape::ALL {
            let err = NamingError::WayInZeroBand {
                edge: edges.0,
                shape,
            };
            every_shape.push((err, shape.to_string()));
        }
        for &shape in DisagreeShape::ALL {
            let err = NamingError::BandVerdictsDisagree { vertex: vtx, shape };
            every_shape.push((err, format!("{shape}. Recourse: {}", shape.lever())));
        }
        for (err, _) in &every_shape {
            let shown = err.to_string();
            assert!(
                shown.split_whitespace().count() <= test_utils::refusal::BUDGET,
                "{err:?} renders over the word budget: {shown}"
            );
            assert_eq!(
                test_utils::refusal::recourse_markers(&shown),
                1,
                "{err:?} states one recourse: {shown}"
            );
        }
        let rows = rows
            .into_iter()
            .map(|(err, wanted)| (err, wanted.into_iter().map(str::to_owned).collect()))
            .chain(every_shape.into_iter().map(|(err, text)| (err, vec![text])));
        for (err, wanted) in rows {
            let shown = err.to_string();
            for w in wanted {
                assert!(
                    shown.contains(w.as_str()),
                    "{err:?} rendered without {w:?}: {shown}"
                );
            }
            // The category the variant is in must be the category its
            // sentence puts it in. A not-a-bug refusal that opens with
            // the emission framing sends its author to file a kernel
            // bug against a recipe that is fine, and nothing else in
            // this file would notice.
            let mine = expected_framing(&err);
            if let Some(f) = mine {
                assert!(
                    shown.starts_with(&format!("{f}: ")),
                    "{err:?} does not open with its framing: {shown}"
                );
            }
            for f in [EMISSION_FRAMING, UNRULED_FRAMING, LIMIT_FRAMING] {
                assert!(
                    mine == Some(f) || !shown.contains(f),
                    "{err:?} speaks another category's framing: {shown}"
                );
            }
            // A missing rule is not a kernel bug, so its sentence ends
            // in "not yet" and never in the defect report; an emission
            // bug ends in the report; a precision limit ends in its
            // lever alone, and in neither.
            let ends = err.ending();
            if let Some(ending) = &ends {
                assert!(
                    shown.ends_with(ending.as_str()),
                    "{err:?} does not end in its category's ending: {shown}"
                );
            }
            let others: &[&str] = match ends {
                Some(_) => &[
                    geom_core::KERNEL_DEFECT_ENDING,
                    geom_core::NOT_YET_ENDING,
                    geom_core::KERNEL_LIMIT_LAST_RESORT,
                ],
                None => &[geom_core::KERNEL_DEFECT_ENDING],
            };
            for never in others {
                assert!(
                    ends.as_ref().is_some_and(|e| e.contains(never)) || !shown.contains(never),
                    "{err:?} ends in another category's ending: {shown}"
                );
            }
            // `crate::py::typed_err` (pncad-py) asserts `reads_as_prose`
            // on every raise, live under release, and its fingerprint is
            // the field brace. A payload rendered through a derived
            // `Debug` is how that assertion gets broken, and several of
            // these refusals carry one. Asserted here for EVERY variant
            // rather than hand-spelled per row, which is how the rewrite
            // of one row dropped it.
            assert!(
                !shown.contains(" { "),
                "{err:?} renders a braced payload — that panics the Python \
                 binding at the arm meant to refuse gracefully: {shown}"
            );
            // The defect a `contains(CONST)` cannot see: a literal that
            // a line continuation padded. Every sentence here is prose
            // a human reads, and prose has no double space in it.
            assert!(
                !shown.contains("  "),
                "{err:?} renders a padded run — a line continuation baked \
                 whitespace into a literal: {shown}"
            );
        }
    }

    /// **The framings, pinned as WORDS.**
    ///
    /// Every other assertion about them is `shown.contains(FRAMING)` —
    /// the constant compared against itself, which passes whatever the
    /// constant holds. `UNRULED_FRAMING` shipped once with two runs of
    /// 32 spaces in it, through three such assertions and a review,
    /// because nothing anywhere said what the sentence is.
    #[test]
    fn the_framings_are_the_sentences_they_are_written_as() {
        assert_eq!(
            EMISSION_FRAMING,
            "name emission found a kernel bug — an invariant it relies on does not hold"
        );
        assert_eq!(
            UNRULED_FRAMING,
            "no naming rule covers this construction; the recipe and its result are sound"
        );
    }

    /// **The walk reports the cardinality; it does not classify it.**
    ///
    /// This is the mechanism [`NamingError::SharedRim`]'s doc argues
    /// for, pinned. A unit cube that `topo::validate_closed` accepts
    /// answers "no shared edge" for its two caps — most face pairs of
    /// most bodies share none — and the walk returns that as a FACT, a
    /// [`Rim`] the caller has to match. A caller
    /// therefore cannot inherit a classification it did not choose:
    /// `emit_sweep` calls the same answer a contradicted key bundle and
    /// `emit_topo` calls it a refuted guess, and both are right because
    /// each is speaking about its own premise.
    ///
    /// The control is the pair that DOES share one rim: without it this
    /// row would pass if the walk had stopped finding rims at all. The
    /// corruption the walk panics on is driven hop by hop in
    /// `walk_tests`.
    #[test]
    fn the_rim_walk_reports_cardinality_as_a_fact() {
        let cube = super::walk_tests::cube();

        let wall = cube.side_faces()[0][0];
        let Rim::One(rim) = rim_between(&cube.body, cube.top, wall) else {
            panic!("a cap and a wall of a box share exactly one rim");
        };
        assert!(
            cube.body.get_edge(rim).is_some(),
            "the control must return a LIVE edge, or the row below passes \
             because the walk finds nothing at all"
        );

        match rim_between(&cube.body, cube.top, cube.bottom) {
            Rim::NotOne(RimShare::NotAdjacent) => {}
            Rim::One(_) => panic!("opposite caps of a box share no rim"),
            Rim::NotOne(other) => panic!("wanted NotAdjacent, got {other:?}"),
        }
    }

    /// The seam-vertex refusal carries THIS vertex, not a constant, and
    /// opens with the missing-rule framing rather than the bug one.
    #[test]
    fn the_underdetermined_seam_vertex_reaches_the_node_level_prose() {
        let (a, b) = two_vertices();
        let carried = |vertex| {
            NodeErrorKind::Naming(NamingError::UnsupportedSeamVertex {
                vertex,
                shape: SeamVertexShape::NoEdgePair,
            })
            .to_string()
        };
        let (sa, sb) = (carried(a), carried(b));
        assert!(sa.contains(&format!("{a:?}")), "no locator: {sa}");
        assert_ne!(
            sa, sb,
            "a refusal that reads the same for two vertices has no locator"
        );
        assert!(sa.contains(UNRULED_FRAMING), "missing-rule framing: {sa}");
        assert!(
            !sa.contains(EMISSION_FRAMING),
            "a missing rule must not read as a kernel bug: {sa}"
        );
        for s in [&sa, &sb] {
            assert!(
                !s.contains(" { "),
                "a braced payload panics the Python binding at the arm \
                 meant to refuse gracefully: {s}"
            );
        }
    }

    /// The boundary the issue was actually about: wrapping the refusal
    /// in `NodeErrorKind` must not swallow it.
    #[test]
    fn the_eval_boundary_carries_the_diagnostic_through() {
        let shown = NodeErrorKind::Naming(NamingError::Emission {
            what: "split naming across boolean-minted faces",
        })
        .to_string();
        assert!(
            shown.contains("split naming across boolean-minted faces"),
            "the emitter's diagnostic must reach Display consumers: {shown}"
        );
    }
}

/// **Each hop of the hand-written walks refuses in its own words**,
/// driven through a real body with that hop's entity removed
/// (`Body::with_entity_removed_for_tests`, `topo`'s `sweep-testing`
/// failure-injection door) and asserted by the sentence it refuses with
/// (`chord_faces` panics instead, a torn body being a kernel bug, and is
/// asserted by its panic's words).
///
/// The walks: [`rim_between`] and `emit_topo`'s `chord_faces` (half-edge
/// → mate → mate's loop → face), [`face_half_edges`] (face → loop →
/// cycle) and [`edge_ends`] (edge → half-edge → end).
/// `topo::Body::face_of_half_edge` answers the last two hops of the first
/// two walks as one `None`, so a walk folded onto it names one hop where
/// there were two, and the row for the other hop reds.
#[cfg(test)]
mod walk_tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use std::collections::BTreeSet;

    use topo::EntityId;
    use topo::splitting::PlaneSide;

    use super::*;
    use crate::names::emit_topo::chord_faces;

    /// A unit cube that `topo::validate_closed` accepts.
    pub(super) fn cube() -> sweep::Extruded<f64> {
        use geom_core::Point2;
        use profile::RawLoop;
        let plane = profile::SketchPlane::from_frame(geom_core::RigidMotion::axes_xy(
            geom_core::Point3::new(0.0, 0.0, 0.0),
        ));
        let square = profile::ProfileLoop::polygon(
            [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
                .into_iter()
                .map(|(x, y)| Point2::new(x, y)),
        );
        let prof = profile::Profile::new(plane, vec![square])
            .validate(geom_core::Tol::witness())
            .expect("a unit square validates");
        let cube = sweep::extrude(
            &prof,
            sweep::Extrusion::Distance {
                depth: 1.0_f64,
                side: sweep::ExtrudeSide::Along,
            },
            geom_core::Tol::witness(),
        )
        .expect("a unit cube extrudes");
        topo::validate_closed(&cube.body).expect("the cube is a sound closed body");
        cube
    }

    /// The cube's top cap, one wall, and the rim between them: the rim
    /// edge and its half in the wall.
    struct Rim1 {
        cube: sweep::Extruded<f64>,
        top: FaceKey,
        wall: FaceKey,
        mate: HalfEdgeKey,
        edge: EdgeKey,
    }

    fn rim1() -> Rim1 {
        let cube = cube();
        let (top, wall) = (cube.top, cube.side_faces()[0][0]);
        let body = &cube.body;
        let (he, mate) = face_half_edges(body, top)
            .into_iter()
            .map(|he| (he, body.mate(he).unwrap()))
            .find(|&(_, m)| body.face_of_half_edge(m) == Some(wall))
            .expect("a cap and a wall of a box share a rim");
        let edge = body.get_half_edge(he).unwrap().edge;
        Rim1 {
            cube,
            top,
            wall,
            mate,
            edge,
        }
    }

    impl Rim1 {
        fn body(&self) -> &Body<f64> {
            &self.cube.body
        }

        /// The body with `entity` removed and every reference to it
        /// dangling.
        fn without(&self, entity: EntityId) -> Body<f64> {
            self.body()
                .with_entity_removed_for_tests(entity)
                .expect("the removed entity is live")
        }

        /// The wall's loop, which the rim's mate half-edge names.
        fn wall_loop(&self) -> EntityId {
            EntityId::Loop(self.body().get_half_edge(self.mate).unwrap().parent_loop)
        }
    }

    #[test]
    fn the_rim_walk_finds_the_rim_of_a_sound_box() {
        let r = rim1();
        match rim_between(r.body(), r.top, r.wall) {
            Rim::One(e) => assert_eq!(e, r.edge, "the control finds the rim"),
            Rim::NotOne(other) => {
                panic!("a cap and a wall of a sound box share one rim: {other:?}")
            }
        }
    }

    /// The half-edge's edge is gone: it has no mate.
    #[test]
    #[should_panic(expected = "rim walk: unmated half-edge")]
    fn the_rim_walk_panics_on_an_unmated_half_edge() {
        let r = rim1();
        rim_between(&r.without(EntityId::Edge(r.edge)), r.top, r.wall);
    }

    /// The mate is named but gone: the first hop.
    #[test]
    #[should_panic(expected = "rim walk: dangling mate")]
    fn the_rim_walk_panics_on_a_dangling_mate() {
        let r = rim1();
        rim_between(&r.without(EntityId::HalfEdge(r.mate)), r.top, r.wall);
    }

    /// The mate is live and its loop is gone: the second hop.
    #[test]
    #[should_panic(expected = "rim walk: dangling loop")]
    fn the_rim_walk_panics_on_a_dangling_loop() {
        let r = rim1();
        rim_between(&r.without(r.wall_loop()), r.top, r.wall);
    }

    /// The chord walk over the cube's top as a section face, through
    /// `body`.
    fn chord_walk(r: &Rim1, body: &Body<f64>) -> BTreeMap<EdgeKey, FaceKey> {
        let sections = [(r.top, PlaneSide::Above)];
        let keys = BTreeSet::from([r.top]);
        chord_faces(body, &sections, &keys)
    }

    #[test]
    fn the_chord_walk_carries_the_rim_to_the_face_across_it() {
        let r = rim1();
        assert_eq!(
            chord_walk(&r, r.body()).get(&r.edge),
            Some(&r.wall),
            "the control carries the rim edge to the wall across it"
        );
    }

    #[test]
    #[should_panic(expected = "chord mate missing")]
    fn the_chord_walk_panics_on_an_unmated_half_edge() {
        let r = rim1();
        chord_walk(&r, &r.without(EntityId::Edge(r.edge)));
    }

    #[test]
    #[should_panic(expected = "chord mate dangling")]
    fn the_chord_walk_panics_on_a_dangling_mate() {
        let r = rim1();
        chord_walk(&r, &r.without(EntityId::HalfEdge(r.mate)));
    }

    #[test]
    #[should_panic(expected = "chord loop dangling")]
    fn the_chord_walk_panics_on_a_dangling_loop() {
        let r = rim1();
        chord_walk(&r, &r.without(r.wall_loop()));
    }

    #[test]
    fn the_face_walk_walks_the_square_cap() {
        let r = rim1();
        assert_eq!(
            face_half_edges(r.body(), r.top).len(),
            4,
            "the control walks the square cap"
        );
    }

    #[test]
    #[should_panic(expected = "face walk: dangling face")]
    fn the_face_walk_panics_on_a_dangling_face() {
        let r = rim1();
        face_half_edges(&r.without(EntityId::Face(r.top)), r.top);
    }

    #[test]
    #[should_panic(expected = "face walk: dangling loop")]
    fn the_face_walk_panics_on_a_dangling_loop() {
        let r = rim1();
        let outer = r.body().get_face(r.top).unwrap().outer;
        face_half_edges(&r.without(EntityId::Loop(outer)), r.top);
    }

    /// A live loop whose cycle runs through a removed half-edge.
    #[test]
    #[should_panic(expected = "face walk: unwalkable loop")]
    fn the_face_walk_panics_on_an_unwalkable_loop() {
        let r = rim1();
        let hes = face_half_edges(r.body(), r.top);
        face_half_edges(&r.without(EntityId::HalfEdge(hes[1])), r.top);
    }

    #[test]
    fn edge_ends_reads_two_ends_of_a_sound_edge() {
        let r = rim1();
        let (start, end) = edge_ends(r.body(), r.edge);
        assert_ne!(start, end, "the control reads two ends");
    }

    #[test]
    #[should_panic(expected = "edge ends: dangling edge")]
    fn edge_ends_panics_on_a_dangling_edge() {
        let r = rim1();
        edge_ends(&r.without(EntityId::Edge(r.edge)), r.edge);
    }

    #[test]
    #[should_panic(expected = "edge ends: dangling he_plus")]
    fn edge_ends_panics_on_a_dangling_he_plus() {
        let r = rim1();
        let he_plus = r.body().get_edge(r.edge).unwrap().he_plus;
        edge_ends(&r.without(EntityId::HalfEdge(he_plus)), r.edge);
    }

    #[test]
    #[should_panic(expected = "edge ends: he_plus has no end")]
    fn edge_ends_panics_on_a_he_plus_with_no_end() {
        let r = rim1();
        let he_plus = r.body().get_edge(r.edge).unwrap().he_plus;
        let next = r.body().get_half_edge(he_plus).unwrap().next;
        edge_ends(&r.without(EntityId::HalfEdge(next)), r.edge);
    }

    #[test]
    #[should_panic(expected = "incidence walk: dangling loop")]
    fn the_incidence_walk_panics_on_a_dangling_loop() {
        let r = rim1();
        Incidence::of(&r.without(r.wall_loop()));
    }

    /// The rim edge's start, read off the sound cube.
    fn rim_start(r: &Rim1) -> VertexKey {
        edge_ends(r.body(), r.edge).0
    }

    #[test]
    fn vertex_point_reads_a_sound_vertex() {
        let r = rim1();
        let v = rim_start(&r);
        let at = *r
            .body()
            .get_point(r.body().get_vertex(v).unwrap().point)
            .unwrap();
        let p = vertex_point(r.body(), v);
        assert_eq!(
            [p.x, p.y, p.z],
            [at.x, at.y, at.z],
            "the control reads the vertex's point"
        );
    }

    #[test]
    #[should_panic(expected = "vertex point: dangling vertex")]
    fn vertex_point_panics_on_a_dangling_vertex() {
        let r = rim1();
        let v = rim_start(&r);
        vertex_point(&r.without(EntityId::Vertex(v)), v);
    }

    #[test]
    #[should_panic(expected = "vertex point: dead point")]
    fn vertex_point_panics_on_a_vertex_without_a_point() {
        let r = rim1();
        let v = rim_start(&r);
        let point = r.body().get_vertex(v).unwrap().point;
        let torn = r
            .body()
            .with_point_removed_for_tests(point)
            .expect("the vertex's point is live");
        vertex_point(&torn, v);
    }
}

/// **The conventional-vertex guard** (`check_total`): a vertex whose
/// only edge is one closed edge, at both ends, has no identity of its
/// own, so a table naming it refuses typed — unless the name is a
/// construction's own rim self-loop vertex. Hand-assembled: no document
/// reaches a closed join yet (`work/seamrank/a-conventional-vertex-mints-no-edge-derived-name`).
#[cfg(test)]
mod conventional_vertex_tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use crate::names::role::{
        MeridianEnd, ProfileEdgeRef, ProfileVertexRef, RoleSeg, band, meridian_vertex,
    };
    use crate::node::StepId;
    use profile::PieceRole;
    use topo::construction::Construct;

    const NODE: RecipeNodeId = RecipeNodeId::new(0, 7);

    fn at(step: u64) -> ProfileVertexRef {
        ProfileVertexRef::Piece {
            step: StepId::new(0, step),
            role: PieceRole::RunOut,
        }
    }

    /// `body` named whole: each face and edge by a distinct role, each
    /// vertex by `vertex`'s name for its index.
    fn named(body: &Body<f64>, vertex: impl Fn(u64) -> StableName) -> NameTable {
        let mut t = NameTable::new();
        t.insert(
            name1(EntityKind::Body, NODE, RoleSeg::OutputBody),
            ent(0, EntityKey::Body),
        )
        .unwrap();
        for (i, (f, _)) in (0u64..).zip(body.faces()) {
            let piece = ProfileEdgeRef::Piece {
                step: StepId::new(0, i),
                role: PieceRole::RunOut,
            };
            t.insert(band(NODE, piece), ent(0, EntityKey::Face(f)))
                .unwrap();
        }
        for (i, (e, _)) in (0u64..).zip(body.edges()) {
            t.insert(
                name1(EntityKind::Edge, NODE, RoleSeg::BandRim(at(i))),
                ent(0, EntityKey::Edge(e)),
            )
            .unwrap();
        }
        for (i, (v, _)) in (0u64..).zip(body.vertices()) {
            t.insert(vertex(i), ent(0, EntityKey::Vertex(v))).unwrap();
        }
        t
    }

    /// A lone vertex closed into a self-loop: one face split in two by
    /// one closed edge whose only vertex is conventional.
    fn self_loop() -> (Body<f64>, VertexKey) {
        let mut body = Body::<f64>::new();
        let born = body
            .mvfs(geom_core::Point3::new(0.0, 0.0, 0.0), true)
            .expect("mvfs births a solid, shell, face and lone vertex");
        body.mef_chord(
            topo::construction::MefSite::Lone {
                r#loop: born.r#loop,
            },
            geom_core::Tol::witness(),
        )
        .expect("mef on an empty loop closes a self-loop");
        assert!(topo::is_conventional_vertex(&body, born.vertex));
        (body, born.vertex)
    }

    #[test]
    fn a_table_naming_a_conventional_vertex_refuses_typed() {
        let (body, v) = self_loop();
        let start = named(&body, |i| meridian_vertex(MeridianEnd::Start, NODE, at(i)));
        assert!(
            matches!(
                check_total(&start, &body, 0),
                Err(NamingError::ConventionalVertex { vertex, body: 0 }) if vertex == v
            ),
            "a member- or position-citing name for a conventional vertex refuses"
        );
    }

    #[test]
    fn a_constructions_own_rim_self_loop_vertex_keeps_its_name() {
        let (body, _) = self_loop();
        let seam = named(&body, |i| meridian_vertex(MeridianEnd::Seam, NODE, at(i)));
        assert!(check_total(&seam, &body, 0).is_ok());
    }

    #[test]
    fn an_open_edges_vertices_are_not_refused() {
        let mut body = Body::<f64>::new();
        let born = body
            .mvfs(geom_core::Point3::new(0.0, 0.0, 0.0), true)
            .expect("mvfs births a solid, shell, face and lone vertex");
        body.mev_line(
            topo::construction::MevSite::Lone {
                r#loop: born.r#loop,
            },
            geom_core::Point3::new(1.0, 0.0, 0.0),
            geom_core::Tol::witness(),
        )
        .expect("mev on an empty loop grows it by one edge");
        assert!(!topo::is_conventional_vertex(&body, born.vertex));
        let start = named(&body, |i| meridian_vertex(MeridianEnd::Start, NODE, at(i)));
        assert!(check_total(&start, &body, 0).is_ok());
    }
}

#[cfg(test)]
mod to_u32_tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::to_u32;

    /// The last value a `u32` holds narrows exactly.
    #[test]
    fn narrows_to_the_bound() {
        let max = usize::try_from(u32::MAX).expect("a 32-bit or wider usize");
        assert_eq!(to_u32(max, "unused"), u32::MAX);
        assert_eq!(to_u32(0, "unused"), 0);
    }

    /// The first value past the bound panics with the caller's own
    /// sentence and the value, never saturated onto `u32::MAX` where it
    /// would share that value's name.
    #[cfg(target_pointer_width = "64")]
    #[test]
    #[should_panic(expected = "the caller's sentence: 4294967296 does not fit")]
    fn panics_one_past_the_bound() {
        let past = usize::try_from(u64::from(u32::MAX) + 1).expect("a 64-bit usize");
        to_u32(past, "the caller's sentence");
    }
}
