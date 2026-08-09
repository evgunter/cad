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
//! [`select`] takes an [`Evaluation`](crate::eval::Evaluation) and
//! hands back `Vec<StableName>` **as of that evaluation**, exactly
//! like [`all_edges`](super::all_edges). A selector value is never
//! stored in a recipe: [`crate::Node::Fillet`]'s payload freezes
//! (`node.rs`'s payload docs — no `All` variant, `Rebind` is the only
//! growth site), and a live query stored in the document is precisely
//! the silent-growth failure the freeze exists to prevent. These
//! types deliberately carry **no serde derives** — there is nowhere
//! in the persisted document for them to go.
//!
//! # STRUCTURAL only (the LB7 fence)
//!
//! The vocabulary here speaks role paths and nothing else. Carrier
//! kind, adjacent-surface pairs, convexity and position are NOT
//! expressible and must not become expressible here: a geometric
//! selector predicate is a DECIDED predicate and inherits
//! `docs/DESIGN.md`'s margins-and-recorded-verdicts discipline
//! (§ "selection/tie-break/ordering"), which this module has no
//! machinery for. Those predicates are deferred to a designed
//! follow-up (LIB-LOG LB7; GUI-DESIGN GQ7).
//!
//! # Shaped as data, not closures
//!
//! Every pattern here is an inspectable value. A binding layer (or a
//! GUI's selection-filter panel) can construct, serialize on its own
//! terms, print, or diff one; a `Fn(&StableName) -> bool` could do
//! none of that.

use geom_core::Decide;

use crate::eval::{Evaluation, NodeResult};
use crate::node::RecipeNodeId;

use super::role::{CapEnd, EntityKind, MeridianEnd, RimSupport, RoleSeg, SplitHalf, StableName};

/// The op group a [`RoleSeg`] belongs to — the enum's own documented
/// grouping, made addressable so a pattern can say "anything the
/// boolean emitter minted" without naming eleven variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OpGroup {
    /// Shared across body-producing ops ([`RoleSeg::OutputBody`]).
    Shared,
    /// Extrude.
    Extrude,
    /// Revolve (the M2 band/pole/seam taxonomy).
    Revolve,
    /// Booleans.
    Boolean,
    /// Split.
    Split,
    /// Fillet (M6-5's composition-surgery vocabulary).
    Fillet,
    /// Pattern.
    Pattern,
}

/// Which [`RoleSeg`] variant a segment is: the fieldless mirror of
/// the role enum.
///
/// The mirror is hand-written and its [`SegTag::of`] match is
/// EXHAUSTIVE with no wildcard arm, so adding a `RoleSeg` variant
/// fails to compile here rather than silently falling through to "no
/// tag" — fail-loud, at the site that must grow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[allow(
    missing_docs,
    reason = "each variant mirrors the documented `RoleSeg` variant of the same name"
)]
pub enum SegTag {
    // Shared
    OutputBody,
    // Extrude
    Cap,
    Lateral,
    RimEdge,
    LateralEdge,
    CapVertex,
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
    FromA,
    FromB,
    Seam,
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
    FromTarget,
    BlendFace,
    CornerFace,
    TrimEdge,
    FootVertex,
    CornerArc,
    BandFace,
    BandTrim,
    BandFoot,
    BandCross,
    BandCut,
    BandSlit,
    // Pattern
    Instance,
}

/// The end/side discriminator a role segment can carry — the closed,
/// float-free tags of the role vocabulary (cap end, meridian end,
/// split half, rim support). One type so a pattern can constrain
/// "which side" uniformly; the `From` impls let a caller write
/// `.side(CapEnd::Top)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Side {
    /// An extrude/revolve cap end.
    Cap(CapEnd),
    /// A revolve meridian end.
    Meridian(MeridianEnd),
    /// A split output half.
    Split(SplitHalf),
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
impl From<SplitHalf> for Side {
    fn from(v: SplitHalf) -> Self {
        Self::Split(v)
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
            RoleSeg::Band(..) => Self::Band,
            RoleSeg::BandRim(..) => Self::BandRim,
            RoleSeg::BandRimPi(..) => Self::BandRimPi,
            RoleSeg::BandPi(..) => Self::BandPi,
            RoleSeg::Meridian(..) => Self::Meridian,
            RoleSeg::MeridianVertex(..) => Self::MeridianVertex,
            RoleSeg::RevolveCap(..) => Self::RevolveCap,
            RoleSeg::Pole(..) => Self::Pole,
            RoleSeg::AxisEdge(..) => Self::AxisEdge,
            RoleSeg::FromA(..) => Self::FromA,
            RoleSeg::FromB(..) => Self::FromB,
            RoleSeg::Seam { .. } => Self::Seam,
            RoleSeg::Merged(..) => Self::Merged,
            RoleSeg::Fragment(..) => Self::Fragment,
            RoleSeg::SplitBody(..) => Self::SplitBody,
            RoleSeg::SectionFace { .. } => Self::SectionFace,
            RoleSeg::SectionEdge { .. } => Self::SectionEdge,
            RoleSeg::SplitFragment { .. } => Self::SplitFragment,
            RoleSeg::CrossingVertex { .. } => Self::CrossingVertex,
            RoleSeg::OnToolVertex { .. } => Self::OnToolVertex,
            RoleSeg::FromTarget(..) => Self::FromTarget,
            RoleSeg::BlendFace(..) => Self::BlendFace,
            RoleSeg::CornerFace(..) => Self::CornerFace,
            RoleSeg::TrimEdge { .. } => Self::TrimEdge,
            RoleSeg::FootVertex { .. } => Self::FootVertex,
            RoleSeg::CornerArc { .. } => Self::CornerArc,
            RoleSeg::BandFace(..) => Self::BandFace,
            RoleSeg::BandTrim { .. } => Self::BandTrim,
            RoleSeg::BandFoot(..) => Self::BandFoot,
            RoleSeg::BandCross(..) => Self::BandCross,
            RoleSeg::BandCut(..) => Self::BandCut,
            RoleSeg::BandSlit(..) => Self::BandSlit,
            RoleSeg::Instance { .. } => Self::Instance,
        }
    }

    /// Which op minted segments with this tag.
    pub fn group(self) -> OpGroup {
        match self {
            Self::OutputBody => OpGroup::Shared,
            Self::Cap | Self::Lateral | Self::RimEdge | Self::LateralEdge | Self::CapVertex => {
                OpGroup::Extrude
            }
            Self::Band
            | Self::BandRim
            | Self::BandRimPi
            | Self::BandPi
            | Self::Meridian
            | Self::MeridianVertex
            | Self::RevolveCap
            | Self::Pole
            | Self::AxisEdge => OpGroup::Revolve,
            Self::FromA | Self::FromB | Self::Seam | Self::Merged | Self::Fragment => {
                OpGroup::Boolean
            }
            Self::SplitBody
            | Self::SectionFace
            | Self::SectionEdge
            | Self::SplitFragment
            | Self::CrossingVertex
            | Self::OnToolVertex => OpGroup::Split,
            Self::FromTarget
            | Self::BlendFace
            | Self::CornerFace
            | Self::TrimEdge
            | Self::FootVertex
            | Self::CornerArc
            | Self::BandFace
            | Self::BandTrim
            | Self::BandFoot
            | Self::BandCross
            | Self::BandCut
            | Self::BandSlit => OpGroup::Fillet,
            Self::Instance => OpGroup::Pattern,
        }
    }
}

/// The end/side tag a segment carries, if any.
fn side_of(seg: &RoleSeg) -> Option<Side> {
    match seg {
        RoleSeg::Cap(e) | RoleSeg::RimEdge(e, _) | RoleSeg::CapVertex(e, _) => Some(Side::Cap(*e)),
        RoleSeg::Meridian(m, _) | RoleSeg::MeridianVertex(m, _) | RoleSeg::RevolveCap(m) => {
            Some(Side::Meridian(*m))
        }
        RoleSeg::SplitBody(s)
        | RoleSeg::SectionFace { side: s, .. }
        | RoleSeg::SectionEdge { side: s, .. }
        | RoleSeg::SplitFragment { side: s, .. }
        | RoleSeg::CrossingVertex { side: s, .. }
        | RoleSeg::OnToolVertex { side: s, .. } => Some(Side::Split(*s)),
        RoleSeg::BandTrim { support, .. } => Some(Side::Rim(*support)),
        _ => None,
    }
}

/// A segment's sub-NAME arguments, in declaration order (the set-
/// valued variants [`RoleSeg::Merged`] and [`RoleSeg::BandFace`]
/// contribute their members in the canonical name order they are
/// stored in). [`RoleSeg::Fragment`]'s [`Qualifier`](super::Qualifier)
/// carries verdicts rather than a role argument and contributes none.
fn name_args(seg: &RoleSeg) -> Vec<&StableName> {
    match seg {
        RoleSeg::FromA(n)
        | RoleSeg::FromB(n)
        | RoleSeg::FromTarget(n)
        | RoleSeg::BlendFace(n)
        | RoleSeg::CornerFace(n)
        | RoleSeg::BandFoot(n)
        | RoleSeg::BandCross(n)
        | RoleSeg::BandCut(n)
        | RoleSeg::BandSlit(n)
        | RoleSeg::SectionEdge { face: n, .. }
        | RoleSeg::SplitFragment { parent: n, .. }
        | RoleSeg::CrossingVertex { edge: n, .. }
        | RoleSeg::OnToolVertex { of: n, .. }
        | RoleSeg::BandTrim { edge: n, .. }
        | RoleSeg::Instance { of: n, .. } => vec![n],
        RoleSeg::Seam { a, b } => vec![a, b],
        RoleSeg::TrimEdge { edge, support } => vec![edge, support],
        RoleSeg::FootVertex { vertex, support } => vec![vertex, support],
        RoleSeg::CornerArc { vertex, edge } => vec![vertex, edge],
        RoleSeg::Merged(set) | RoleSeg::BandFace(set) => set.iter().collect(),
        _ => Vec::new(),
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
    /// Constraints on the segment's sub-name arguments, positionally
    /// in declaration order. A PREFIX: an empty vec constrains
    /// nothing, `[p]` constrains the first argument only. Longer than
    /// the segment's argument list never matches.
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

    /// Constrains the end/side tag (`.side(CapEnd::Top)`).
    #[must_use]
    pub fn side(mut self, side: impl Into<Side>) -> Self {
        self.side = Some(side.into());
        self
    }

    /// Constrains the sub-name arguments (positional prefix).
    #[must_use]
    pub fn of(mut self, args: impl IntoIterator<Item = NamePat>) -> Self {
        self.args = args.into_iter().collect();
        self
    }

    /// Whether `seg` matches.
    #[must_use]
    pub fn matches(&self, seg: &RoleSeg) -> bool {
        let tag = SegTag::of(seg);
        let tag_ok = match self.tag {
            TagPat::Any => true,
            TagPat::Group(g) => tag.group() == g,
            TagPat::Is(t) => tag == t,
        };
        if !tag_ok || (self.side.is_some() && self.side != side_of(seg)) {
            return false;
        }
        let args = name_args(seg);
        self.args.len() <= args.len()
            && self
                .args
                .iter()
                .zip(args)
                .all(|(pat, name)| pat.matches(name))
    }
}

/// A pattern over a whole [`StableName`]: kind, minting node, and the
/// exact shape of the role path.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NamePat {
    /// If set, the name's entity kind must be exactly this.
    pub kind: Option<EntityKind>,
    /// If set, the name's minting node must be exactly this.
    pub node: Option<RecipeNodeId>,
    /// If set, the role path must have EXACTLY this length and match
    /// segment-for-segment. `None` matches any path — which is what
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
            ..Self::default()
        }
    }

    /// Constrains the minting node.
    #[must_use]
    pub fn node(mut self, node: RecipeNodeId) -> Self {
        self.node = Some(node);
        self
    }

    /// Constrains the role path (exact length, segment-for-segment).
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
        if self.kind.is_some_and(|k| k != name.kind) || self.node.is_some_and(|n| n != name.node) {
            return false;
        }
        match &self.path {
            None => true,
            Some(pats) => {
                pats.len() == name.path.len()
                    && pats.iter().zip(&name.path).all(|(p, s)| p.matches(s))
            }
        }
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
    /// fillet node is where an empty selection refuses — one door for
    /// that refusal, not two).
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
/// no value, no table, or nothing matching — the fillet node is where
/// an empty selection refuses.
pub fn select<T: Decide>(
    ev: &Evaluation<T>,
    node: RecipeNodeId,
    sel: &Selector,
) -> Vec<StableName> {
    let Some(NodeResult::Ok(value)) = ev.nodes.get(&node) else {
        return Vec::new();
    };
    let mut out: Vec<StableName> = value
        .name_table
        .iter()
        .map(|(name, _)| name)
        .filter(|name| sel.matches(name))
        .cloned()
        .collect();
    out.sort();
    out.dedup();
    out
}
