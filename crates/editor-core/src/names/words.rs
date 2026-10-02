//! **A name's leaf role, in words** — what a person reads to tell one
//! face from another.
//!
//! The path as a structure is the machine channel; a person reads its
//! leaf in words. The leaf is the entity the author made, reached by
//! looking through every segment that carries an operand's entity on
//! ([`origin`]'s `Carried`): a boolean's `FromA`, a union's
//! `FromMember`, a fillet's `FromTarget` say nothing a reader needs, so
//! the words are the role underneath ("the end cap", "the side wall
//! over the leg of loop 0 step 2").
//!
//! Where the carry or a fragment qualifier is what tells two faces of
//! one leaf apart, it is said around the leaf: "the piece of the end
//! cap bordering the side wall over …", "piece 2 of 3 of …", "the
//! piece above the split of …", "instance 1's copy of …".
//!
//! A role that cites other names — a blend face over its source edge,
//! a seam between two faces, a cavity twin — says each cited name one
//! level down, and a name cited from there is said by its kind alone
//! ("a face"), so the sentence is bounded however deep the derivation
//! runs. The one level is what tells two blend faces, two seams, two
//! shell walls apart: they differ in what they were made over.
//!
//! A profile step is said as the profile pane numbers it, `loop L step
//! S` from zero, wherever the speaker's document holds the step; by
//! its tag (`the profile step <tag>`) where no document is at hand.

use core::fmt;

use crate::names::attribute::{CarriedAs, SegOrigin, origin};
use crate::names::role::{
    CapEnd, EntityKind, MeridianEnd, PieceRun, ProfileEdgeRef, ProfileVertexRef, Qualifier,
    RimSupport, RoleSeg, SectionCircle, SplitHalf, StableName, fragment_tail_start,
};
use crate::spoken::{Said, Say, Speaker};

/// **A name's leaf role, in words** ([module docs](self)): `the end
/// cap`, `the piece above the split of the side wall over the leg of
/// loop 0 step 2`. Article-led, so a sentence takes it as a noun
/// phrase.
///
/// Its `Display` says each profile step by its tag; said by a
/// [`Speaker`] holding the document ([`Said`]), as the profile pane
/// numbers it. The name's own `Display` carries it beside the kind and
/// the minting node.
#[derive(Clone, Copy)]
pub struct LeafRole<'a>(pub &'a StableName);

/// **`name`'s leaf role, in words** ([`LeafRole`]).
#[must_use]
pub fn leaf_role(name: &StableName) -> LeafRole<'_> {
    LeafRole(name)
}

impl Say for LeafRole<'_> {
    fn say(&self, f: &mut fmt::Formatter<'_>, by: Speaker<'_>) -> fmt::Result {
        f.write_str(&phrase(self.0, by, 0))
    }
}

impl fmt::Display for LeafRole<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Said(self, Speaker::TAG).fmt(f)
    }
}

/// **The name whose role a [`LeafRole`] says**: `name` with every
/// carrying segment looked through ([`origin`]) and every fragment
/// qualifier dropped. Its node is the one that minted the entity
/// ([`super::attribute`]'s answer, where that walk finds one).
#[must_use]
pub fn role_leaf(name: &StableName) -> &StableName {
    walk(name).leaf
}

/// How deep a cited name is said: the leaf at 0, a name its role cites
/// at 1, and from there by kind alone.
const CITED_IN_FULL: u8 = 1;

/// One level of a name the walk looked through and must say: a fragment
/// qualifier, or a carry that is not whole.
enum Wrap<'n> {
    Piece(&'n Qualifier),
    Carried(CarriedAs),
}

/// A name, its carrying segments and qualifiers looked through.
struct Walk<'n> {
    /// What was looked through, outermost first.
    wraps: Vec<Wrap<'n>>,
    /// The name the walk stopped at.
    leaf: &'n StableName,
}

fn walk(name: &StableName) -> Walk<'_> {
    let mut wraps = Vec::new();
    let mut at = name;
    loop {
        let tail = fragment_tail_start(&at.path);
        // `[parent, Fragment(q1), Fragment(q2)]` is the q2 piece of the
        // q1 piece: the last qualifier is the outermost.
        wraps.extend(at.path[tail..].iter().rev().filter_map(|seg| match seg {
            RoleSeg::Fragment(q) => Some(Wrap::Piece(q)),
            _ => None,
        }));
        let [seg] = &at.path[..tail] else {
            return Walk { wraps, leaf: at };
        };
        match origin(seg) {
            SegOrigin::Carried(of, CarriedAs::Whole) => at = of,
            SegOrigin::Carried(of, carry) => {
                wraps.push(Wrap::Carried(carry));
                at = of;
            }
            SegOrigin::Minted | SegOrigin::Unclassified => return Walk { wraps, leaf: at },
        }
    }
}

/// `name` said at `depth` ([`CITED_IN_FULL`]).
fn phrase(name: &StableName, by: Speaker<'_>, depth: u8) -> String {
    if depth > CITED_IN_FULL {
        return format!("{} {}", article(name.kind), name.kind.noun());
    }
    let Walk { wraps, leaf } = walk(name);
    let mut np = head(leaf, by, depth);
    let (said, deeper) = wraps.split_at(wraps.len().min(WRAPS_SAID));
    if !deeper.is_empty() {
        np = format!(
            "{} {} derived from {np}",
            article(name.kind),
            name.kind.noun()
        );
    }
    for wrap in said.iter().rev() {
        np = wrapped(wrap, &np, name.kind, by, depth);
    }
    np
}

/// How many looked-through levels are said, outermost first: the
/// levels nearest the name are the ones that tell it from its
/// neighbours, and the rest are said together as "derived from", so a
/// derivation of any depth reads in bounded words.
const WRAPS_SAID: usize = 3;

fn article(kind: EntityKind) -> &'static str {
    match kind {
        EntityKind::Edge => "an",
        EntityKind::Face | EntityKind::Vertex | EntityKind::Body => "a",
    }
}

/// `np` with one looked-through level said around it.
fn wrapped(wrap: &Wrap<'_>, np: &str, kind: EntityKind, by: Speaker<'_>, depth: u8) -> String {
    let cited = |names: &[StableName]| list(names, by, depth + 1);
    match wrap {
        Wrap::Carried(CarriedAs::Whole) => np.to_owned(),
        Wrap::Carried(CarriedAs::Split(side)) => format!("the piece {} of {np}", half(*side)),
        Wrap::Carried(CarriedAs::ToolCopy(side)) => format!("the copy {} of {np}", half(*side)),
        Wrap::Carried(CarriedAs::Instance(i)) => format!("instance {i}'s copy of {np}"),
        Wrap::Carried(CarriedAs::Cut) => format!("the surviving piece of {np}"),
        Wrap::Piece(Qualifier::OrderAlong { rank, of }) => {
            let what = match kind {
                EntityKind::Vertex => "crossing",
                EntityKind::Face | EntityKind::Edge | EntityKind::Body => "piece",
            };
            format!("{what} {} of {of} of {np}", rank.saturating_add(1))
        }
        // A qualifier that cites names says them only where the leaf is
        // said in full: one level down, they would be kind nouns.
        Wrap::Piece(_) if depth >= CITED_IN_FULL => format!("a piece of {np}"),
        Wrap::Piece(Qualifier::Borders(walls)) => {
            format!("the piece of {np} bordering {}", cited(walls))
        }
        Wrap::Piece(Qualifier::Keeps(edges)) => {
            format!("the piece of {np} along {}", cited(edges))
        }
        Wrap::Piece(Qualifier::Ends(ends)) => {
            format!("the piece of {np} between {}", cited(ends))
        }
    }
}

/// Up to two names in full, then a count: a merged face or a band over
/// many edges reads as its first constituent and how many more.
fn list(names: &[StableName], by: Speaker<'_>, depth: u8) -> String {
    match names {
        [] => "nothing".to_owned(),
        [one] => phrase(one, by, depth),
        [a, b] => format!("{} and {}", phrase(a, by, depth), phrase(b, by, depth)),
        [a, rest @ ..] => format!("{} and {} more", phrase(a, by, depth), rest.len()),
    }
}

fn half(side: SplitHalf) -> &'static str {
    match side {
        SplitHalf::Above => "above the split",
        SplitHalf::Below => "below the split",
    }
}

fn cap(e: CapEnd) -> &'static str {
    match e {
        CapEnd::Start => "start",
        CapEnd::End => "end",
    }
}

fn meridian(e: MeridianEnd) -> &'static str {
    match e {
        MeridianEnd::Start => "start",
        MeridianEnd::End => "end",
        MeridianEnd::Seam => "seam",
        MeridianEnd::Pi => "half-turn",
    }
}

/// A profile piece: its role, and the step that drew it as the profile
/// pane numbers it — or, on a kernel-built section, which circle.
fn piece(e: &ProfileEdgeRef, by: Speaker<'_>) -> String {
    match e {
        ProfileEdgeRef::Piece { step, role } => match by.step(*step) {
            Some(at) => format!("the {role} of {at}"),
            None => format!("the {role} of the profile step {step}"),
        },
        ProfileEdgeRef::Section { circle, role } => format!(
            "the {role} of the {} circle",
            match circle {
                SectionCircle::Outer => "outer",
                SectionCircle::Bore => "bore",
            }
        ),
    }
}

fn run(r: &PieceRun, by: Speaker<'_>) -> String {
    r.pieces()
        .iter()
        .map(|e| piece(e, by))
        .collect::<Vec<_>>()
        .join(", then ")
}

fn vertex(v: &ProfileVertexRef, by: Speaker<'_>) -> String {
    let edge = match *v {
        ProfileVertexRef::Piece { step, role } => ProfileEdgeRef::Piece { step, role },
        ProfileVertexRef::Section { circle, role } => ProfileEdgeRef::Section { circle, role },
    };
    format!("the start of {}", piece(&edge, by))
}

/// The role the walk stopped at, in words. Exhaustive over
/// [`RoleSeg`], so a new segment is given words here or the compile
/// breaks.
fn head(leaf: &StableName, by: Speaker<'_>, depth: u8) -> String {
    let path = &leaf.path[..fragment_tail_start(&leaf.path)];
    let seg = match path {
        [] => return "the entity with no role".to_owned(),
        [seg] => seg,
        // A seam junction: the run of the seams that meet there.
        many if many.iter().all(|s| matches!(s, RoleSeg::Seam { .. })) => {
            return format!("the junction of {} seams", many.len());
        }
        _ => return "the entity of an unread role".to_owned(),
    };
    let c = |n: &StableName| phrase(n, by, depth + 1);
    let kind = leaf.kind.noun();
    match seg {
        RoleSeg::OutputBody => "the output body".to_owned(),
        RoleSeg::Cap(e) => format!("the {} cap", cap(*e)),
        RoleSeg::Lateral(r) => format!("the side wall over {}", run(r, by)),
        RoleSeg::RimEdge(c, e) => format!("the {} rim edge over {}", cap(*c), piece(e, by)),
        RoleSeg::LateralEdge(v) => format!("the lateral edge over {}", vertex(v, by)),
        RoleSeg::CapVertex(c, v) => format!("the {} cap vertex over {}", cap(*c), vertex(v, by)),
        RoleSeg::LoftWall(pieces) => format!(
            "the loft wall over {}",
            pieces
                .iter()
                .map(|e| piece(e, by))
                .collect::<Vec<_>>()
                .join(", then ")
        ),
        RoleSeg::LoftSeam(vertices) => format!(
            "the loft seam over {}",
            vertices
                .iter()
                .map(|v| vertex(v, by))
                .collect::<Vec<_>>()
                .join(", then ")
        ),
        RoleSeg::Band(r) => format!("the band face over {}", run(r, by)),
        RoleSeg::BandRim(v) => format!("the band rim over {}", vertex(v, by)),
        RoleSeg::BandRimPi(v) => format!("the second band rim over {}", vertex(v, by)),
        RoleSeg::BandPi(r) => format!("the second band face over {}", run(r, by)),
        RoleSeg::Meridian(m, r) => {
            format!("the {} meridian edge over {}", meridian(*m), run(r, by))
        }
        RoleSeg::MeridianVertex(m, v) => {
            format!(
                "the {} meridian vertex over {}",
                meridian(*m),
                vertex(v, by)
            )
        }
        RoleSeg::RevolveCap(m) => format!("the {} wedge cap", meridian(*m)),
        RoleSeg::Pole(v) => format!("the pole over {}", vertex(v, by)),
        RoleSeg::AxisEdge(e) => format!("the axis edge over {}", piece(e, by)),
        RoleSeg::Seam { a, b } => format!("the seam {kind} of {} and {}", c(a), c(b)),
        RoleSeg::Merged(set) => format!("the merged face of {}", list(set, by, depth + 1)),
        RoleSeg::SplitBody(side) => format!("the body {}", half(*side)),
        RoleSeg::SectionFace { side, section } => {
            format!("section face {} {}", section.saturating_add(1), half(*side))
        }
        RoleSeg::SectionEdge { side, face } => {
            format!("the section edge across {} {}", c(face), half(*side))
        }
        RoleSeg::CrossingVertex { side, edge } => {
            format!("the split's crossing of {} {}", c(edge), half(*side))
        }
        RoleSeg::BlendFace(edge) => format!("the blend face over {}", c(edge)),
        RoleSeg::CornerFace(v) => format!("the corner face at {}", c(v)),
        RoleSeg::TrimEdge { edge, support } => format!(
            "the trim edge on {} of the blend over {}",
            c(support),
            c(edge)
        ),
        RoleSeg::FootVertex { vertex, support } => {
            format!("the blend foot on {} at {}", c(support), c(vertex))
        }
        RoleSeg::EndArc { vertex, edge } => {
            format!("the end arc at {} of the blend over {}", c(vertex), c(edge))
        }
        RoleSeg::BandFace(edges) => format!("the blend band over {}", list(edges, by, depth + 1)),
        RoleSeg::BandTrim { edge, support } => format!(
            "the {}-side trim edge of the blend band over {}",
            match support {
                RimSupport::Host => "host",
                RimSupport::Mate => "mate",
            },
            c(edge)
        ),
        RoleSeg::BandFoot(v) => format!("the blend band's foot at {}", c(v)),
        RoleSeg::BandCross { edge, .. } => format!("the blend band's crossing of {}", c(edge)),
        RoleSeg::BandSlit { edge, .. } => format!("the blend band's slit along {}", c(edge)),
        RoleSeg::Inner(of) => format!("the cavity twin of {}", c(of)),
        RoleSeg::Rim(of) => format!("the shell rim of {}", c(of)),
        RoleSeg::HoleRim { of, hole } => format!("the rim of hole {hole} in {}", c(of)),
        // The part's own steps and nodes are another document's ids,
        // so the part-local name is said by tag.
        RoleSeg::InPart { of } => {
            format!("{} of the part", phrase(of, Speaker::TAG, depth + 1))
        }
        // The walk looks through these and never stops at one.
        RoleSeg::FromA(of)
        | RoleSeg::FromB(of)
        | RoleSeg::FromMember { of, .. }
        | RoleSeg::FromTarget(of)
        | RoleSeg::SplitFragment { parent: of, .. }
        | RoleSeg::OnToolVertex { of, .. }
        | RoleSeg::Instance { of, .. }
        | RoleSeg::BandCut(of) => c(of),
        RoleSeg::Fragment(_) => format!("a piece of {} {kind}", article(leaf.kind)),
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use crate::names::role::NameRef;
    use crate::node::{RecipeNodeId, StepId};
    use profile::PieceRole;

    const EXTRUDE: RecipeNodeId = RecipeNodeId(1 << 16);
    const OTHER: RecipeNodeId = RecipeNodeId(2 << 16);
    const OP: RecipeNodeId = RecipeNodeId(3 << 16);

    fn name(kind: EntityKind, node: RecipeNodeId, path: Vec<RoleSeg>) -> StableName {
        StableName { kind, node, path }
    }

    fn leg(step: u64) -> ProfileEdgeRef {
        ProfileEdgeRef::Piece {
            step: StepId(step << 16),
            role: PieceRole::Leg,
        }
    }

    fn cap(end: CapEnd) -> StableName {
        name(EntityKind::Face, EXTRUDE, vec![RoleSeg::Cap(end)])
    }

    fn wall(step: u64) -> StableName {
        name(
            EntityKind::Face,
            EXTRUDE,
            vec![RoleSeg::Lateral(leg(step).into())],
        )
    }

    fn rim(step: u64) -> StableName {
        name(
            EntityKind::Edge,
            EXTRUDE,
            vec![RoleSeg::RimEdge(CapEnd::End, leg(step))],
        )
    }

    fn said(n: &StableName) -> String {
        leaf_role(n).to_string()
    }

    /// The carrying wrappers are looked through to the role the author
    /// made, and the name's own sentence adds the node that made it.
    #[test]
    fn a_carried_face_is_said_by_its_leaf() {
        let carried = name(
            EntityKind::Face,
            OP,
            vec![RoleSeg::FromA(NameRef::new(name(
                EntityKind::Face,
                OTHER,
                vec![RoleSeg::FromTarget(NameRef::new(cap(CapEnd::End)))],
            )))],
        );
        assert_eq!(said(&carried), "the end cap");
        assert_eq!(role_leaf(&carried).node, EXTRUDE);
        assert_eq!(
            carried.to_string(),
            "face name minted by node 000000000003 (the end cap, minted by node 000000000001)"
        );
    }

    /// The pieces of one cut face differ in what they border, or in
    /// their rank, and a split's two pieces in their side: each is said.
    #[test]
    fn the_pieces_of_one_face_are_told_apart() {
        let piece = |q| {
            name(
                EntityKind::Face,
                OP,
                vec![
                    RoleSeg::FromA(NameRef::new(cap(CapEnd::End))),
                    RoleSeg::Fragment(q),
                ],
            )
        };
        assert_eq!(
            said(&piece(Qualifier::Borders(vec![wall(1)]))),
            "the piece of the end cap bordering the side wall over the leg of the profile step \
             000000000001"
        );
        assert_eq!(
            said(&piece(Qualifier::OrderAlong { rank: 1, of: 3 })),
            "piece 2 of 3 of the end cap"
        );
        let half = |side| {
            name(
                EntityKind::Face,
                OP,
                vec![RoleSeg::SplitFragment {
                    side,
                    parent: NameRef::new(wall(1)),
                }],
            )
        };
        assert_eq!(
            said(&half(SplitHalf::Above)),
            "the piece above the split of the side wall over the leg of the profile step \
             000000000001"
        );
        assert_ne!(said(&half(SplitHalf::Above)), said(&half(SplitHalf::Below)));
    }

    /// A blend face, a shell's cavity twin and a split's section edge
    /// are said over what they were made against, one level down, which
    /// is what tells two of them apart.
    #[test]
    fn blend_shell_and_split_faces_are_said_over_their_source() {
        let blend = |step| {
            name(
                EntityKind::Face,
                OP,
                vec![RoleSeg::BlendFace(NameRef::new(rim(step)))],
            )
        };
        assert_eq!(
            said(&blend(1)),
            "the blend face over the end rim edge over the leg of the profile step 000000000001"
        );
        assert_ne!(said(&blend(1)), said(&blend(2)));
        let twin = name(
            EntityKind::Face,
            OP,
            vec![RoleSeg::Inner(NameRef::new(cap(CapEnd::Start)))],
        );
        assert_eq!(said(&twin), "the cavity twin of the start cap");
        let section = name(
            EntityKind::Edge,
            OP,
            vec![RoleSeg::SectionEdge {
                side: SplitHalf::Below,
                face: NameRef::new(wall(2)),
            }],
        );
        assert_eq!(
            said(&section),
            "the section edge across the side wall over the leg of the profile step \
             000000000002 below the split"
        );
    }

    /// A name cited two levels down is said by its kind alone, so a
    /// blend over a seam reads in bounded words.
    #[test]
    fn a_name_cited_past_one_level_is_said_by_its_kind() {
        let seam = name(
            EntityKind::Edge,
            OTHER,
            vec![RoleSeg::Seam {
                a: NameRef::new(cap(CapEnd::End)),
                b: NameRef::new(wall(1)),
            }],
        );
        let blend = name(
            EntityKind::Face,
            OP,
            vec![RoleSeg::BlendFace(NameRef::new(seam.clone()))],
        );
        assert_eq!(
            said(&seam),
            "the seam edge of the end cap and the side wall over the leg of the profile step \
             000000000001"
        );
        assert_eq!(
            said(&blend),
            "the blend face over the seam edge of a face and a face"
        );
    }

    /// A derivation of any depth reads in bounded words: past three
    /// levels, the rest are said together.
    #[test]
    fn a_deep_derivation_is_said_in_bounded_words() {
        let deep = (0..50).fold(cap(CapEnd::End), |n, i| {
            name(
                EntityKind::Face,
                OP,
                vec![RoleSeg::Instance {
                    i,
                    of: NameRef::new(n),
                }],
            )
        });
        assert_eq!(
            said(&deep),
            "instance 49's copy of instance 48's copy of instance 47's copy of a face derived \
             from the end cap"
        );
    }
}
