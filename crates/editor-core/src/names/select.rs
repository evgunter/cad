//! **Structural selectors** (LIB-U7, scoped by LB7): a small query
//! value over the SHAPE of a [`StableName`] — its kind, its minting
//! node, and the pattern of [`RoleSeg`]s on its role path.
//!
//! # What this is for
//!
//! Before this module a document author who wanted "the cap rim of
//! the top face" or "every seam edge where a cap meets a band" had
//! two options: author the names by hand (the `die_composed` corpus
//! document did exactly that, fourteen of them), or write a Rust
//! filter over the evaluated `topo::Body` — which yields ARENA KEYS,
//! which the recipe layer refuses by G1's boundary rule. A selector
//! closes the gap in the layer where names already live.
//!
//! # It is a MATERIALIZER, never a live query
//!
//! [`select`] takes an [`crate::eval::Evaluation`] and
//! hands back `Vec<StableName>` **as of that evaluation**, exactly
//! like [`all_edges`](super::all_edges). A selector value is never
//! stored in a recipe: [`crate::Node::Fillet`]'s payload freezes
//! (`node.rs`'s payload docs — no `All` variant, `Rebind` is the only
//! growth site), and a live query stored in the document is precisely
//! the silent-growth failure the freeze exists to prevent. These
//! types deliberately carry **no serde derives** — there is nowhere
//! in the persisted document for them to go.
//!
//! # The PATTERNS stay structural; geometry is a second STAGE
//!
//! [`Selector`] and [`NamePat`] speak role paths and nothing else, and
//! that is permanent: matching on the name value alone is what makes a
//! pattern reusable anywhere a name exists (appearance, resolve
//! diagnostics), and it needs no body to do it. Carrier kind,
//! adjacent-surface pairs and position are different in kind — they
//! interrogate the ENTITY a name resolves to — so they are a FILTER at
//! the materializer, not a new pattern leaf: [`select_where`] with a
//! conjunction of [`GeomPred`] atoms (SELECT-DESIGN §§1-2, ratified
//! #286; the LB7 deferral this discharges). Convexity stays reserved
//! and unbuilt (GS-Q2 — see [`GeomPred`]).
//!
//! # Shaped as data, not closures
//!
//! Every pattern here is an inspectable value. A binding layer (or a
//! GUI's selection-filter panel) can construct, serialize on its own
//! terms, print, or diff one; a `Fn(&StableName) -> bool` could do
//! none of that.

use geom_core::{Band, Decide, Tol};

use crate::eval::Evaluation;
use crate::expr::VarEnv;
use crate::node::RecipeNodeId;

use super::geompred::{self, GeomPred, SelectRefusal};
use super::interrogate;
use super::role::{
    CapEnd, EntityKind, MeridianEnd, Qualifier, RimSupport, RoleSeg, RunLine, StableName,
    name_free_seg,
};
use super::table::EntityRef;

/// The op group a [`RoleSeg`] belongs to — the enum's own documented
/// grouping, made addressable so a pattern can say "anything the
/// boolean emitter minted" without naming eleven variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OpGroup {
    /// Shared across body-producing ops ([`RoleSeg::OutputBody`], and
    /// [`RoleSeg::From`], an entity carried in through a read).
    Shared,
    /// Extrude — and the loft, a swept solid of the same shape, whose
    /// caps, rims and cap vertices are the extrude's roles and whose
    /// walls and seams pair one piece per section.
    Extrude,
    /// Revolve (the M2 band/pole/seam taxonomy).
    Revolve,
    /// Booleans.
    Boolean,
    /// Split.
    Split,
    /// Fillet (M6-5's composition-surgery vocabulary) — and the
    /// CHAMFER's, which reuses these roles deliberately: the shapes
    /// are the same (a band face off a source edge, a corner patch off
    /// a source vertex), and a `StableName` carries the minting node,
    /// which is what tells the two apart (RECIPE-DOORS D3).
    ///
    /// The group's NAME under-describes what it groups, and stays:
    /// fenced by the ratified verb-vocabulary decision
    /// (`crates/sweep/README.md`, settled ground).
    Fillet,
    /// Repetition: a family's member (D10 `[repetition]`).
    Repetition,
    /// Shell (the hollowing verb's cavity, rim and hole-rim roles).
    /// Its outer wall speaks as [`SegTag::From`], which groups under
    /// [`OpGroup::Shared`]: the tag names the SHAPE (an entity carried
    /// in through a read), and the minting node says which op.
    Shell,
}

macro_rules! seg_tags {
    ($($name:ident),* $(,)?) => {
        /// Which [`RoleSeg`] variant a segment is: the fieldless mirror of
        /// the role enum.
        ///
        /// The mirror is hand-written and its [`SegTag::of`] match is
        /// EXHAUSTIVE with no wildcard arm, so adding a `RoleSeg` variant
        /// fails to compile here rather than silently falling through to "no
        /// tag" — fail-loud, at the site that must grow. The mirror is
        /// declared through one macro so that [`SegTag::ALL`] is projected
        /// from the same list as the variants and cannot fall behind one.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        #[allow(
            missing_docs,
            reason = "each variant mirrors the documented `RoleSeg` variant of the same name"
        )]
        pub enum SegTag {
            $($name),*
        }

        impl SegTag {
            /// Every tag, in declaration order — the row set a census
            /// over the segment vocabulary iterates (the content key's
            /// `seg_content_tags_are_injective`), enumerated from the
            /// same declaration as the variants.
            pub const ALL: &'static [SegTag] = &[$(SegTag::$name),*];
        }
    };
}

seg_tags! {
    // Shared
    OutputBody,
    From,
    // Extrude
    Cap,
    Lateral,
    RimEdge,
    LateralEdge,
    CapVertex,
    // Loft
    LoftWall,
    LoftSeam,
    LoftSlabWall,
    LoftSlabSeam,
    LoftSectionEdge,
    LoftSectionVertex,
    // Revolve
    Band,
    BandRim,
    BandRimPi,
    BandPi,
    Meridian,
    MeridianVertex,
    RevolveCap,
    Pole,
    AxisEdge,
    // Boolean
    Seam,
    Crossing,
    EdgeCrossing,
    Merged,
    Fragment,
    // Split
    SplitBody,
    SectionFace,
    SectionEdge,
    SplitFragment,
    CrossingVertex,
    OnToolVertex,
    // Fillet
    BlendFace,
    CornerFace,
    TrimEdge,
    FootVertex,
    EndArc,
    Mitre,
    TurnFoot,
    BandFace,
    BandTrim,
    BandFoot,
    BandCross,
    BandCut,
    BandSlit,
    // Shell
    Inner,
    Rim,
    HoleRim,
    // Pattern
    Member,
}

/// The end/side discriminator a role segment can carry — the closed,
/// float-free tags of the role vocabulary (cap end, meridian end,
/// split half, rim support). One type so a pattern can constrain
/// "which side" uniformly; the `From` impls let a caller write
/// `.side(CapEnd::End)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Side {
    /// An extrude/revolve cap end.
    Cap(CapEnd),
    /// A revolve meridian end.
    Meridian(MeridianEnd),
    /// Which support of a rim blend.
    Rim(RimSupport),
}

impl From<CapEnd> for Side {
    fn from(v: CapEnd) -> Self {
        Self::Cap(v)
    }
}
impl From<MeridianEnd> for Side {
    fn from(v: MeridianEnd) -> Self {
        Self::Meridian(v)
    }
}
impl From<RimSupport> for Side {
    fn from(v: RimSupport) -> Self {
        Self::Rim(v)
    }
}

impl SegTag {
    /// The tag of a segment (exhaustive by construction — module
    /// docs).
    pub fn of(seg: &RoleSeg) -> Self {
        match seg {
            RoleSeg::OutputBody => Self::OutputBody,
            RoleSeg::Cap(..) => Self::Cap,
            RoleSeg::Lateral(..) => Self::Lateral,
            RoleSeg::RimEdge(..) => Self::RimEdge,
            RoleSeg::LateralEdge(..) => Self::LateralEdge,
            RoleSeg::CapVertex(..) => Self::CapVertex,
            RoleSeg::LoftWall(..) => Self::LoftWall,
            RoleSeg::LoftSeam(..) => Self::LoftSeam,
            RoleSeg::LoftSlabWall(..) => Self::LoftSlabWall,
            RoleSeg::LoftSlabSeam(..) => Self::LoftSlabSeam,
            RoleSeg::LoftSectionEdge(..) => Self::LoftSectionEdge,
            RoleSeg::LoftSectionVertex(..) => Self::LoftSectionVertex,
            RoleSeg::Band(..) => Self::Band,
            RoleSeg::BandRim(..) => Self::BandRim,
            RoleSeg::BandRimPi(..) => Self::BandRimPi,
            RoleSeg::BandPi(..) => Self::BandPi,
            RoleSeg::Meridian(..) => Self::Meridian,
            RoleSeg::MeridianVertex(..) => Self::MeridianVertex,
            RoleSeg::RevolveCap(..) => Self::RevolveCap,
            RoleSeg::Pole(..) => Self::Pole,
            RoleSeg::AxisEdge(..) => Self::AxisEdge,
            RoleSeg::From { .. } => Self::From,
            RoleSeg::Seam { .. } => Self::Seam,
            RoleSeg::Crossing { .. } => Self::Crossing,
            RoleSeg::EdgeCrossing { .. } => Self::EdgeCrossing,
            RoleSeg::Merged(..) => Self::Merged,
            RoleSeg::Fragment(..) => Self::Fragment,
            RoleSeg::SplitBody => Self::SplitBody,
            RoleSeg::SectionFace => Self::SectionFace,
            RoleSeg::SectionEdge { .. } => Self::SectionEdge,
            RoleSeg::SplitFragment { .. } => Self::SplitFragment,
            RoleSeg::CrossingVertex { .. } => Self::CrossingVertex,
            RoleSeg::OnToolVertex { .. } => Self::OnToolVertex,
            RoleSeg::BlendFace(..) => Self::BlendFace,
            RoleSeg::CornerFace(..) => Self::CornerFace,
            RoleSeg::TrimEdge { .. } => Self::TrimEdge,
            RoleSeg::FootVertex { .. } => Self::FootVertex,
            RoleSeg::EndArc { .. } => Self::EndArc,
            RoleSeg::Mitre { .. } => Self::Mitre,
            RoleSeg::TurnFoot { .. } => Self::TurnFoot,
            RoleSeg::BandFace(..) => Self::BandFace,
            RoleSeg::BandTrim { .. } => Self::BandTrim,
            RoleSeg::BandFoot(..) => Self::BandFoot,
            RoleSeg::BandCross { .. } => Self::BandCross,
            RoleSeg::BandCut(..) => Self::BandCut,
            RoleSeg::BandSlit { .. } => Self::BandSlit,
            RoleSeg::Inner(..) => Self::Inner,
            RoleSeg::Rim(..) => Self::Rim,
            RoleSeg::HoleRim { .. } => Self::HoleRim,
            RoleSeg::Member { .. } => Self::Member,
        }
    }

    /// Which op minted segments with this tag.
    pub fn group(self) -> OpGroup {
        match self {
            Self::OutputBody | Self::From => OpGroup::Shared,
            Self::Cap
            | Self::Lateral
            | Self::RimEdge
            | Self::LateralEdge
            | Self::CapVertex
            | Self::LoftWall
            | Self::LoftSeam
            | Self::LoftSlabWall
            | Self::LoftSlabSeam
            | Self::LoftSectionEdge
            | Self::LoftSectionVertex => OpGroup::Extrude,
            Self::Band
            | Self::BandRim
            | Self::BandRimPi
            | Self::BandPi
            | Self::Meridian
            | Self::MeridianVertex
            | Self::RevolveCap
            | Self::Pole
            | Self::AxisEdge => OpGroup::Revolve,
            Self::Seam | Self::Crossing | Self::EdgeCrossing | Self::Merged | Self::Fragment => {
                OpGroup::Boolean
            }
            Self::SplitBody
            | Self::SectionFace
            | Self::SectionEdge
            | Self::SplitFragment
            | Self::CrossingVertex
            | Self::OnToolVertex => OpGroup::Split,
            Self::BlendFace
            | Self::CornerFace
            | Self::TrimEdge
            | Self::FootVertex
            | Self::EndArc
            | Self::Mitre
            | Self::TurnFoot
            | Self::BandFace
            | Self::BandTrim
            | Self::BandFoot
            | Self::BandCross
            | Self::BandCut
            | Self::BandSlit => OpGroup::Fillet,
            Self::Inner | Self::Rim | Self::HoleRim => OpGroup::Shell,
            Self::Member => OpGroup::Repetition,
        }
    }
}

/// The end/side tag a segment carries, if any. The match is
/// EXHAUSTIVE on purpose (the `walk_names` rule): a future
/// [`RoleSeg`] or [`Qualifier`] variant carrying an end, half or rim
/// support must be classified here or the compile breaks.
fn side_of(seg: &RoleSeg) -> Option<Side> {
    match seg {
        RoleSeg::Cap(e) | RoleSeg::RimEdge(e, _) | RoleSeg::CapVertex(e, _) => Some(Side::Cap(*e)),
        RoleSeg::Meridian(m, _) | RoleSeg::MeridianVertex(m, _) | RoleSeg::RevolveCap(m) => {
            Some(Side::Meridian(*m))
        }
        RoleSeg::BandTrim { support, .. } => Some(Side::Rim(*support)),
        // Side-free segments (kept explicit — see the doc note). A
        // split's half is the variable's, not the name's (N1, the scope).
        RoleSeg::SplitBody
        | RoleSeg::SectionFace
        | RoleSeg::SectionEdge { .. }
        | RoleSeg::SplitFragment { .. }
        | RoleSeg::CrossingVertex { .. }
        | RoleSeg::OnToolVertex { .. }
        | RoleSeg::OutputBody
        | RoleSeg::Lateral(_)
        | RoleSeg::LateralEdge(_)
        | RoleSeg::LoftWall(_)
        | RoleSeg::LoftSeam(_)
        | RoleSeg::LoftSlabWall(..)
        | RoleSeg::LoftSlabSeam(..)
        | RoleSeg::LoftSectionEdge(..)
        | RoleSeg::LoftSectionVertex(..)
        | RoleSeg::Band(_)
        | RoleSeg::BandRim(_)
        | RoleSeg::BandRimPi(_)
        | RoleSeg::BandPi(_)
        | RoleSeg::Pole(_)
        | RoleSeg::AxisEdge(_)
        | RoleSeg::From { .. }
        | RoleSeg::Seam { .. }
        | RoleSeg::Crossing { .. }
        | RoleSeg::EdgeCrossing { .. }
        | RoleSeg::Merged(_)
        | RoleSeg::Fragment(Qualifier::Borders(_) | Qualifier::Keeps(_) | Qualifier::Ends(_))
        | RoleSeg::BlendFace(_)
        | RoleSeg::CornerFace(_)
        | RoleSeg::TrimEdge { .. }
        | RoleSeg::FootVertex { .. }
        | RoleSeg::EndArc { .. }
        | RoleSeg::Mitre { .. }
        | RoleSeg::TurnFoot { .. }
        | RoleSeg::BandFace(_)
        | RoleSeg::BandFoot(_)
        | RoleSeg::BandCross { .. }
        | RoleSeg::BandCut(_)
        | RoleSeg::BandSlit { .. }
        | RoleSeg::Inner(_)
        | RoleSeg::Rim(_)
        | RoleSeg::HoleRim { .. }
        | RoleSeg::Member { .. } => None,
    }
}

/// A segment's sub-NAME arguments: a SEQUENCE, each argument in a slot
/// of its own, then a SET, whose members are spelled in name order and
/// fill no slot (N0 NS1).
struct Args<'a> {
    /// The slotted arguments, in declaration order.
    seq: Vec<&'a StableName>,
    /// The set's members.
    set: Vec<&'a StableName>,
}

/// A segment's sub-NAME [`Args`]. The sets are [`RoleSeg::Seam`]'s two
/// sides, [`RoleSeg::Merged`]'s constituents, [`RoleSeg::EdgeCrossing`]'s
/// lines, [`RoleSeg::BandFace`]'s chain, and the `band` chain of
/// [`RoleSeg::BandCross`] and [`RoleSeg::BandSlit`] after their slotted
/// `edge`; every other argument has a slot. [`RoleSeg::Fragment`]'s
/// [`Qualifier`] and a crossing's [`Sense`](super::role::Sense) carry
/// verdicts rather than role arguments and contribute none: a pattern
/// has no axis for a sense, so a `Through`'s things are no more an
/// argument than `Enters` is. The match is EXHAUSTIVE on purpose (the
/// `walk_names` rule), field by field: a future [`RoleSeg`] or
/// [`Qualifier`] variant or field embedding names must be classified
/// here or the compile breaks — or,
/// if it embeds no name, added to [`crate::names::name_free_seg`], which
/// is the one place that answer is written for every match that shares
/// it.
///
/// **Only NAMES.** [`RoleSeg::From`] contributes its `of` and not its
/// `read`, exactly as [`RoleSeg::Member`] contributes its `of` and not
/// its `at`: a bare [`crate::VarId`] is not a name and a walk over names
/// cannot see it. The consumers that need the read — the content key,
/// the re-map, and `derivation_nodes` — reach it through
/// [`crate::names::read_edge`], which is where "which segments carry a
/// bare id" is answered.
fn name_args(seg: &RoleSeg) -> Args<'_> {
    fn seq(seq: Vec<&StableName>) -> Args<'_> {
        Args {
            seq,
            set: Vec::new(),
        }
    }
    fn set(set: Vec<&StableName>) -> Args<'_> {
        Args {
            seq: Vec::new(),
            set,
        }
    }
    match seg {
        RoleSeg::From { of: n, read: _ }
        | RoleSeg::BlendFace(n)
        | RoleSeg::CornerFace(n)
        | RoleSeg::Mitre { vertex: n }
        | RoleSeg::TurnFoot { vertex: n }
        | RoleSeg::BandFoot(n)
        | RoleSeg::BandCut(n)
        | RoleSeg::Inner(n)
        | RoleSeg::Rim(n)
        | RoleSeg::HoleRim { of: n, hole: _ }
        | RoleSeg::SectionEdge { face: n }
        | RoleSeg::SplitFragment { parent: n }
        | RoleSeg::CrossingVertex { edge: n, sense: _ }
        | RoleSeg::OnToolVertex { of: n }
        | RoleSeg::BandTrim {
            edge: n,
            support: _,
        }
        | RoleSeg::Member { of: n, at: _ } => seq(vec![n]),
        RoleSeg::Crossing {
            edge,
            face,
            sense: _,
        } => seq(vec![edge, face]),
        RoleSeg::TrimEdge { edge, support } => seq(vec![edge, support]),
        RoleSeg::FootVertex { vertex, support } => seq(vec![vertex, support]),
        RoleSeg::EndArc { vertex, edge } => seq(vec![vertex, edge]),
        RoleSeg::Seam { a, b } => set(vec![a, b]),
        RoleSeg::EdgeCrossing(run) => set(run
            .iter()
            .map(|RunLine { line, sense: _ }| &**line)
            .collect()),
        RoleSeg::Merged(members) | RoleSeg::BandFace(members) => set(members.iter().collect()),
        RoleSeg::BandCross { edge, band } | RoleSeg::BandSlit { edge, band } => Args {
            seq: vec![edge],
            set: band.iter().collect(),
        },
        // A qualifier, not a role argument (see the doc note).
        RoleSeg::Fragment(Qualifier::Borders(_) | Qualifier::Keeps(_) | Qualifier::Ends(_)) => {
            seq(Vec::new())
        }
        name_free_seg!() => seq(Vec::new()),
    }
}

/// Which segment variant(s) a [`SegPat`] accepts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum TagPat {
    /// Any segment.
    #[default]
    Any,
    /// Any segment minted by this op.
    Group(OpGroup),
    /// Exactly this variant (its arguments constrained separately).
    Is(SegTag),
}

/// A pattern over ONE role segment: which variant, which side, and
/// what its sub-name arguments look like. The three axes are the only
/// STRUCTURE a segment has.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SegPat {
    /// Which variant(s) match.
    pub tag: TagPat,
    /// If set, the segment's end/side tag must be exactly this (a
    /// segment carrying no side tag never matches a set `side`).
    pub side: Option<Side>,
    /// Constraints on the segment's sub-name arguments. The patterns
    /// fill the segment's slotted arguments first, slot for slot and as
    /// a PREFIX: an empty vec constrains nothing, and on a `Crossing`
    /// `[p]` constrains the edge only. Each pattern past the slots
    /// matches a DISTINCT member of the segment's set, in any order: on
    /// a `Seam`, `[p, q]` matches whichever way round the two sides
    /// fit, and `[p]` matches a seam where either side does. The sets
    /// are a seam's sides, `Merged` constituents, an `EdgeCrossing`'s
    /// lines and a band's chain (`BandFace`, and `BandCross` and
    /// `BandSlit` after their edge). More patterns than arguments never
    /// match.
    pub args: Vec<NamePat>,
}

impl SegPat {
    /// The wildcard segment pattern.
    #[must_use]
    pub fn any() -> Self {
        Self::default()
    }

    /// Segments of this variant.
    #[must_use]
    pub fn tag(tag: SegTag) -> Self {
        Self {
            tag: TagPat::Is(tag),
            ..Self::default()
        }
    }

    /// Segments minted by this op.
    #[must_use]
    pub fn group(group: OpGroup) -> Self {
        Self {
            tag: TagPat::Group(group),
            ..Self::default()
        }
    }

    /// Constrains the end/side tag (`.side(CapEnd::End)`).
    #[must_use]
    pub fn side(mut self, side: impl Into<Side>) -> Self {
        self.side = Some(side.into());
        self
    }

    /// Constrains the sub-name arguments: the slots as a prefix, then
    /// distinct members of the set ([`SegPat::args`]).
    #[must_use]
    pub fn of(mut self, args: impl IntoIterator<Item = NamePat>) -> Self {
        self.args = args.into_iter().collect();
        self
    }

    /// Whether `seg` matches.
    #[must_use]
    pub fn matches(&self, seg: &RoleSeg) -> bool {
        let mut needs = Vec::new();
        self.level_matches(seg, &mut needs) && needs.iter().all(|need| need.holds(&all_match))
    }

    /// Whether `seg` matches this pattern's own axes, what its argument
    /// patterns ask of the segment's names set aside in `needs`.
    fn level_matches<'a>(&'a self, seg: &'a RoleSeg, needs: &mut Vec<Need<'a>>) -> bool {
        let tag = SegTag::of(seg);
        let tag_ok = match self.tag {
            TagPat::Any => true,
            TagPat::Group(g) => tag.group() == g,
            TagPat::Is(t) => tag == t,
        };
        if !tag_ok || (self.side.is_some() && self.side != side_of(seg)) {
            return false;
        }
        let Args { seq, set } = name_args(seg);
        if self.args.len() > seq.len() + set.len() {
            return false;
        }
        let (slotted, members) = self.args.split_at(self.args.len().min(seq.len()));
        needs.extend(slotted.iter().zip(seq).map(Need::One));
        if !members.is_empty() {
            needs.push(Need::Distinct(members, set));
        }
        true
    }
}

/// One question the matcher answers: whether this pattern matches this
/// name, every level of both.
type Ask<'a> = (&'a NamePat, &'a StableName);

/// What a level that matched still needs of the levels below it.
enum Need<'a> {
    /// This pattern matches this name.
    One(Ask<'a>),
    /// Each of these patterns matches a distinct one of these members.
    Distinct(&'a [NamePat], Vec<&'a StableName>),
    /// Each of these segment patterns matches a distinct one of these
    /// segments: a seam junction's run of lines, a set (N0) whose order
    /// is only its name order (`canonical::is_junction`).
    Segments(&'a [SegPat], &'a [RoleSeg]),
}

impl<'a> Need<'a> {
    /// The questions whose answers decide this need.
    fn asks(&self) -> Vec<Ask<'a>> {
        match self {
            Self::One(ask) => vec![*ask],
            Self::Distinct(pats, members) => pats
                .iter()
                .flat_map(|pat| members.iter().map(move |&member| (pat, member)))
                .collect(),
            Self::Segments(pats, segs) => pats
                .iter()
                .flat_map(|pat| segs.iter().map(move |seg| (pat, seg)))
                .flat_map(|(pat, seg)| {
                    let mut needs = Vec::new();
                    if pat.level_matches(seg, &mut needs) {
                        needs.iter().flat_map(Need::asks).collect()
                    } else {
                        Vec::new()
                    }
                })
                .collect(),
        }
    }

    /// Whether this need holds, given the answers to its [`Need::asks`].
    fn holds(&self, answer: &dyn Fn(Ask<'a>) -> bool) -> bool {
        match self {
            Self::One(ask) => answer(*ask),
            Self::Distinct(pats, members) => {
                let fits: Vec<Vec<bool>> = pats
                    .iter()
                    .map(|pat| {
                        members
                            .iter()
                            .map(|&member| answer((pat, member)))
                            .collect()
                    })
                    .collect();
                each_fits_a_distinct_member(&fits)
            }
            Self::Segments(pats, segs) => {
                let fits: Vec<Vec<bool>> = pats
                    .iter()
                    .map(|pat| {
                        segs.iter()
                            .map(|seg| {
                                let mut needs = Vec::new();
                                pat.level_matches(seg, &mut needs)
                                    && needs.iter().all(|need| need.holds(answer))
                            })
                            .collect()
                    })
                    .collect();
                each_fits_a_distinct_member(&fits)
            }
        }
    }
}

/// Whether every pattern can be given a member of its own that it fits,
/// `fits[i][j]` saying whether pattern `i` fits member `j`: a bipartite
/// matching, grown one pattern at a time along augmenting paths. Which
/// member a pattern is given is never an answer — only whether every
/// pattern can be given one.
fn each_fits_a_distinct_member(fits: &[Vec<bool>]) -> bool {
    /// Gives pattern `i` a member, moving the patterns already given
    /// one where that frees one up; `seen` marks the members this
    /// search has tried. Recurses at most once per pattern.
    fn give(i: usize, fits: &[Vec<bool>], seen: &mut [bool], holder: &mut [Option<usize>]) -> bool {
        for (j, &fit) in fits[i].iter().enumerate() {
            if fit && !seen[j] {
                seen[j] = true;
                if holder[j].is_none_or(|k| give(k, fits, seen, holder)) {
                    holder[j] = Some(i);
                    return true;
                }
            }
        }
        false
    }
    let members = fits.first().map_or(0, Vec::len);
    let mut holder = vec![None; members];
    (0..fits.len()).all(|i| give(i, fits, &mut vec![false; members], &mut holder))
}

/// The key an [`Ask`] is answered under: the two values it asks about,
/// by address, so a name two handles share is asked about once — the
/// name by the address `names::nest`'s walks key their answers by.
fn ask_key((pat, name): Ask<'_>) -> (usize, usize) {
    (
        core::ptr::from_ref(pat) as usize,
        super::nest::address(name),
    )
}

/// Whether `root`'s pattern matches its name, as deep as they nest —
/// from this walk's own stack. A level that matches leaves the questions
/// its needs ask on the stack above its own answer, so each answer is
/// taken after the answers it reads; a set need asks every pattern of
/// every member, so one question can come up twice, and is answered
/// once.
fn all_match(root: Ask<'_>) -> bool {
    enum Step<'a> {
        Ask(Ask<'a>),
        Answer(Ask<'a>, Vec<Need<'a>>),
    }
    let mut answers = std::collections::HashMap::new();
    let mut steps = vec![Step::Ask(root)];
    while let Some(step) = steps.pop() {
        match step {
            Step::Ask(ask) => {
                if answers.contains_key(&ask_key(ask)) {
                    continue;
                }
                let mut needs = Vec::new();
                if !ask.0.level_matches(ask.1, &mut needs) {
                    answers.insert(ask_key(ask), false);
                    continue;
                }
                let below: Vec<Ask<'_>> = needs.iter().flat_map(Need::asks).collect();
                steps.push(Step::Answer(ask, needs));
                steps.extend(below.into_iter().map(Step::Ask));
            }
            Step::Answer(ask, needs) => {
                let held = needs
                    .iter()
                    .all(|need| need.holds(&|below| answers[&ask_key(below)]));
                answers.insert(ask_key(ask), held);
            }
        }
    }
    answers[&ask_key(root)]
}

/// A pattern over a whole [`StableName`]: kind, minting node, and the
/// exact shape of the role path.
///
/// A pattern nests one whole pattern per argument it constrains, as
/// deep as its author builds it, so its `Drop`, `Clone`, `PartialEq`,
/// `Debug` and [`NamePat::matches`] are written one level at a time
/// and none recurses on the nesting. `Clone`, `PartialEq` and `Debug`
/// are the walks a [`StableName`] runs (`names::nest`'s `copy_nested`,
/// `eq_nested` and `render_nested`). `Drop` and `matches` are their
/// own: a pattern owns every pattern it holds, where a name shares what
/// it holds through handles and asks each whether it is the last
/// holder, and matching walks a pattern and a name in step, which no
/// walk over a name alone does.
#[derive(Default)]
pub struct NamePat {
    /// If set, the name's entity kind must be exactly this.
    pub kind: Option<EntityKind>,
    /// If set, the name's minting node must be exactly this.
    pub node: Option<RecipeNodeId>,
    /// If set, the role path must have EXACTLY this length and match
    /// segment-for-segment — but for a seam junction's run of lines, a
    /// set, where each segment pattern matches a distinct line in any
    /// order. `None` matches any path — which is what
    /// makes a sub-name pattern like "any face of the A operand"
    /// sayable without spelling that operand's whole derivation.
    pub path: Option<Vec<SegPat>>,
}

impl NamePat {
    /// The wildcard: matches every name.
    #[must_use]
    pub fn any() -> Self {
        Self::default()
    }

    /// Names of this entity kind.
    #[must_use]
    pub fn of_kind(kind: EntityKind) -> Self {
        Self {
            kind: Some(kind),
            node: None,
            path: None,
        }
    }

    /// Constrains the minting node.
    #[must_use]
    pub fn node(mut self, node: RecipeNodeId) -> Self {
        self.node = Some(node);
        self
    }

    /// Constrains the role path (exact length, segment-for-segment; a
    /// seam junction's lines in any order, [`NamePat::path`]'s field).
    #[must_use]
    pub fn path(mut self, path: impl IntoIterator<Item = SegPat>) -> Self {
        self.path = Some(path.into_iter().collect());
        self
    }

    /// Constrains the role path to ONE segment — the common case.
    #[must_use]
    pub fn seg(self, seg: SegPat) -> Self {
        self.path([seg])
    }

    /// Whether `name` matches.
    #[must_use]
    pub fn matches(&self, name: &StableName) -> bool {
        all_match((self, name))
    }

    /// Whether `name` matches this pattern's own level, what its segment
    /// patterns' argument patterns ask set aside in `needs`.
    fn level_matches<'a>(&'a self, name: &'a StableName, needs: &mut Vec<Need<'a>>) -> bool {
        if self.kind.is_some_and(|k| k != name.kind) || self.node.is_some_and(|n| n != name.node) {
            return false;
        }
        match &self.path {
            None => true,
            Some(pats) if pats.len() != name.path.len() => false,
            Some(pats) if super::canonical::is_junction(name) => {
                needs.push(Need::Segments(pats, &name.path));
                true
            }
            Some(pats) => pats
                .iter()
                .zip(&name.path)
                .all(|(p, s)| p.level_matches(s, needs)),
        }
    }

    /// The patterns this one holds, one level down, in declaration
    /// order.
    fn held(&self) -> Vec<&NamePat> {
        self.path.iter().flatten().flat_map(|s| &s.args).collect()
    }
}

impl Drop for NamePat {
    fn drop(&mut self) {
        // Every pattern below this one is moved here before it goes, so
        // each dropped in the loop holds none.
        fn take(pat: &mut NamePat, into: &mut Vec<NamePat>) {
            for seg in pat.path.iter_mut().flatten() {
                into.append(&mut seg.args);
            }
        }
        let mut pats = Vec::new();
        take(self, &mut pats);
        while let Some(mut pat) = pats.pop() {
            take(&mut pat, &mut pats);
        }
    }
}

impl Clone for NamePat {
    fn clone(&self) -> Self {
        super::nest::copy_nested(self, NamePat::held, |pat, copies| {
            let NamePat { kind, node, path } = pat;
            NamePat {
                kind: *kind,
                node: *node,
                path: path.as_ref().map(|segs| {
                    segs.iter()
                        .map(|seg| {
                            let SegPat { tag, side, args } = seg;
                            SegPat {
                                tag: *tag,
                                side: *side,
                                args: copies.take(args.len()).collect(),
                            }
                        })
                        .collect()
                }),
            }
        })
    }
}

impl PartialEq for NamePat {
    fn eq(&self, other: &Self) -> bool {
        use super::nest::{Family, Walk, eq_nested, shallow};
        if shallow(Walk::Eq, Family::Pattern) {
            return true;
        }
        // Every field is bound, so a field added to the pattern is an
        // E0027 here until equality says what it does with it.
        let same = |a: &NamePat, b: &NamePat| {
            let NamePat { kind, node, path } = a;
            let NamePat {
                kind: b_kind,
                node: b_node,
                path: b_path,
            } = b;
            kind == b_kind && node == b_node && path == b_path
        };
        eq_nested(Family::Pattern, self, other, same, NamePat::held)
    }
}

impl Eq for NamePat {}

/// One level of a pattern as the derived impl renders it.
struct PatLevel<'a>(&'a NamePat);

impl core::fmt::Debug for PatLevel<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let NamePat { kind, node, path } = self.0;
        f.debug_struct("NamePat")
            .field("kind", kind)
            .field("node", node)
            .field("path", path)
            .finish()
    }
}

impl core::fmt::Debug for NamePat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use super::nest::{Family, Walk, render_nested, shallow};
        if shallow(Walk::Debug, Family::Pattern) {
            return core::fmt::Write::write_char(f, super::nest::HOLE);
        }
        render_nested(
            self,
            f,
            Family::Pattern,
            |p, alternate| {
                if alternate {
                    format!("{:#?}", PatLevel(p))
                } else {
                    format!("{:?}", PatLevel(p))
                }
            },
            NamePat::held,
        )
    }
}

/// A structural selector: a UNION of name patterns.
///
/// Union is the whole combinator, deliberately: a selection is a SET
/// of names, alternatives are how a real selection is described ("the
/// twelve box edges AND the pip rim"), and every pattern here is
/// already a conjunction of its own fields. Intersection and negation
/// are not offered — they would let a selector describe an empty or
/// surprising set with no site to explain it, and nothing measured in
/// the census needs them. Growth is additive later.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Selector {
    /// The alternatives. An EMPTY selector matches nothing (and the
    /// fillet's insert is where an empty selection refuses — one door
    /// for that refusal, not two).
    pub alts: Vec<NamePat>,
}

impl Selector {
    /// A selector with one alternative.
    #[must_use]
    pub fn of(pat: NamePat) -> Self {
        Self { alts: vec![pat] }
    }

    /// A selector over several alternatives (their union).
    #[must_use]
    pub fn any_of(pats: impl IntoIterator<Item = NamePat>) -> Self {
        Self {
            alts: pats.into_iter().collect(),
        }
    }

    /// Adds an alternative.
    #[must_use]
    pub fn or(mut self, pat: NamePat) -> Self {
        self.alts.push(pat);
        self
    }

    /// Whether any alternative matches.
    #[must_use]
    pub fn matches(&self, name: &StableName) -> bool {
        self.alts.iter().any(|p| p.matches(name))
    }
}

/// **The names of `node`'s output matching `sel`, as of THIS
/// evaluation** — the structural selector's materializer.
///
/// Same contract as [`all_edges`](super::all_edges), and the same
/// discipline: the caller STORES the result, and from that moment it
/// behaves like any other authored selection (the growth path is
/// [`crate::DocEdit::Rebind`]). Returns canonical order (sorted,
/// deduped), ready for [`crate::Node::fillet`]. Empty if `node` has
/// no value, no table, or nothing matching — the fillet's insert is
/// where an empty selection refuses.
///
/// **"No value" answers "no names" here, deliberately**: the question
/// is which names the table `at` names holds, and a node with no value
/// holds no table, nor does a node named alone whose outputs carry
/// names apart (a split: read a half, NAMES N1). The empty list is not the last word on it — the node that
/// consumes the stored selection reads `node` as its input, so it is
/// poisoned through it while it has no value, and refuses the empty
/// selection once it has one.
pub fn select<T: Decide>(
    ev: &Evaluation<T>,
    at: impl Into<crate::At>,
    sel: &Selector,
) -> Vec<StableName> {
    // No value, no names: this function's doc.
    let Some(table) = ev.table_at(at) else {
        return Vec::new();
    };
    let mut out: Vec<StableName> = table
        .iter()
        .map(|(name, _)| name)
        .filter(|name| sel.matches(name))
        .cloned()
        .collect();
    out.sort();
    out.dedup();
    out
}

/// **The names of `node`'s output matching `sel` AND every atom of
/// `geom`, as of THIS evaluation** — the structural selector with a
/// geometric filter bolted on at the materializer.
///
/// # The shape, and why it is this shape
///
/// The structural [`Selector`] is UNCHANGED and geometry is a second
/// STAGE, not a new [`NamePat`] field. A `NamePat` matches on the name
/// value alone (`matches(&StableName) -> bool` — pure, no evaluation
/// in sight), which is exactly what makes it reusable anywhere a name
/// exists. A geometric predicate is different in kind: it interrogates
/// the ENTITY the name resolves to. Structural narrows; geometric
/// filters the survivors.
///
/// `geom` is a CONJUNCTION. Union-of-conjunctions stays the whole
/// algebra — call this twice and concatenate for a geometric union,
/// the same additive-growth posture the structural union has. An EMPTY
/// `geom` makes this exactly [`select`], infallibly.
///
/// # Still a MATERIALIZER
///
/// Same contract as [`select`] and [`all_edges`](super::all_edges):
/// answers as of one evaluation, returns canonical order (sorted,
/// deduped) `Vec<StableName>`, and the CALLER stores it. Nothing here
/// is storable in a recipe — [`GeomPred`] carries no serde derives for
/// the same reason [`Selector`] does not. Empty (not an error) if
/// `node` has no value, no table, or nothing matching.
///
/// # `params`
///
/// [`GeomPred::PlaneDistance`] states its value as a [`Formula`](crate::Formula), which
/// cannot be evaluated without the document's parameter bindings
/// (`Doc::var_env`). The design's signature omits this argument; it
/// is added here rather than degrading the value to a bare float,
/// because a selection rule written against a named parameter is the
/// whole point of `Expr` being the value type (SELECT-DESIGN §5).
///
/// # Errors
///
/// [`SelectRefusal`] — the two honesty obligations decided predicates
/// bring (an in-band margin refuses rather than silently including or
/// excluding; a tied name whose candidates DISAGREE cannot be
/// half-selected), plus the static faults of a malformed query. A
/// purely-EXACT `geom` produces none of them on a well-formed name
/// table — the exception being [`SelectRefusal::Unreadable`], which a
/// table entry pointing outside its node's payload would raise for any
/// `geom` at all (an emitter invariant violation, reported not
/// swallowed).
pub fn select_where<T: Decide>(
    ev: &Evaluation<T>,
    at: impl Into<crate::At>,
    sel: &Selector,
    geom: &[GeomPred],
    params: &VarEnv<T>,
    tol: Tol,
) -> Result<Vec<StableName>, SelectRefusal> {
    // No value, no names: `select`'s doc.
    let Some(var) = ev.at_var(at) else {
        return Ok(Vec::new());
    };
    let Some(table) = ev.table(var) else {
        return Ok(Vec::new());
    };
    let atoms = geompred::prepare(ev, var, geom, params, tol)?;
    let band = Band::linear(tol)?;
    let mut out: Vec<StableName> = Vec::new();
    for (name, entry) in table.iter() {
        if !sel.matches(name) {
            continue;
        }
        if atoms.is_empty() {
            out.push(name.clone());
            continue;
        }
        // GS-Q4: a tie is the name table's own fact, so the filter
        // must ask ALL of its candidates — all match ⇒ include (still
        // tied, and referencing it still refuses downstream as
        // `Ambiguous`); none ⇒ exclude; MIXED ⇒ refuse, because a
        // filter cannot half-select a name and silence in either
        // direction lies.
        let candidates: &[EntityRef] = entry.entities();
        let mut matched = 0usize;
        for ent in candidates {
            let unreadable = |error| SelectRefusal::Unreadable {
                name: Box::new(name.clone()),
                error,
            };
            let body = ev
                .read_body(var, ent.body)
                .map_err(|standing| unreadable(interrogate::InterrogateError::Standing(standing)))?
                .map_err(unreadable)?;
            if geompred::candidate_matches(body, ent.key, &atoms, band, name)? {
                matched += 1;
            }
        }
        if matched == candidates.len() {
            out.push(name.clone());
        } else if matched > 0 {
            return Err(SelectRefusal::TiedDisagrees {
                name: Box::new(name.clone()),
                matched,
                candidates: candidates.len(),
            });
        }
    }
    out.sort();
    out.dedup();
    Ok(out)
}
