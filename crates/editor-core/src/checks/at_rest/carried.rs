//! **What crosses an instance seam at rest, and the census's rows**
//! (ASSEMBLY A5, *A part's own census findings*): a part's at-rest
//! findings, each carried with the [`Route`] it arrived by and reported
//! once per part row, and the rows the census records between two
//! arrivals ([`AtRestRow`]), which the `unproven-coincidence` lint reads.
//!
//! A part's census findings are the part's: the census over a document
//! instantiating it discards what it finds again inside the part's copy
//! (`product::gather_sources`), and the part's own reading of its census
//! ([`seam_verdict`]) crosses instead.

use std::sync::Arc;

use super::{AtRestEvidence, InterferenceFinding, LocalizeLane};
use crate::doc::Doc;
use crate::node::RecipeNodeId;
use crate::product::Product;
use crate::spoken::HeldNodes;

use geom_core::Tol;

/// How a row of another document's reached THIS document: the
/// instantiating node here, the document the row is of, and the
/// instantiating nodes in between.
///
/// One type for every carrier of that fact — [`CarriedInterference`]
/// and [`CarriedAtRestFinding`] — so a reader who has learned to read a
/// route once reads every one of them, and a route can only be built
/// one way.
///
/// **A node below this document is said as its own document holds
/// it**: each hop here, and the nodes each carrier's `held` keeps of
/// `of`, recorded where that document was in hand, the version its
/// instantiator's pin fixes (DESIGN.md Band 1, "Node labels"). Only
/// `through` is this document's, and the frame that holds this
/// document says it. Equality compares those labels too, deliberately:
/// two rows with equal ids from parts labelled apart (two pins, since
/// a label is in the pin) are unequal, here and in every carrier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Route {
    /// The instantiating node OF THIS DOCUMENT the row came through.
    pub through: RecipeNodeId,
    /// The document the row is of: the file an author opens to act on
    /// it.
    pub of: crate::ident::DocumentId,
    /// The further instantiating nodes between `through` and `of`,
    /// nearest first, each in its own document's id space and as that
    /// document says it: the version its instantiator's pin fixes, so
    /// the label is one the row cannot outlive (DESIGN.md Band 1, "Node
    /// labels"). Empty when `through` instantiates `of` directly; one
    /// entry per intervening sub-assembly, so a row three documents
    /// down names its whole route.
    pub via: Vec<crate::spoken::SpokenNode>,
}

// The route as an author reads it: the instance in this document, said
// by the speaker, then each instance below it, as its own document
// holds it.
impl crate::spoken::Say for Route {
    fn say(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        by: crate::spoken::Speaker<'_>,
    ) -> core::fmt::Result {
        write!(
            f,
            "document {} through {}",
            self.of,
            by.node_as(self.through, "instance")
        )?;
        for hop in &self.via {
            write!(f, " → {}", hop.as_noun("instance"))?;
        }
        Ok(())
    }
}

/// The route where no document is at hand: its first instance by its
/// tag.
impl core::fmt::Display for Route {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::spoken::Say::say(self, f, crate::spoken::Speaker::TAG)
    }
}

/// **One row a part hands to the document instantiating it**: the
/// part's own rows, and those it carried up from its parts, each with
/// the part's nodes it names said as the part holds them. Built where
/// the part's document is in hand (`eval::parts`), so the instance
/// that takes it needs only its own id to route it
/// ([`PartRow::through`]).
#[derive(Debug, Clone)]
pub(crate) struct PartRow<B> {
    /// The document the row is of ([`Route::of`]).
    of: crate::ident::DocumentId,
    /// The instances between the part and `of`, nearest first, each as
    /// its document holds it ([`Route::via`]): empty for the part's own
    /// row.
    via: Vec<crate::spoken::SpokenNode>,
    /// The row itself, in `of`'s ids.
    body: B,
    /// `body`'s nodes, as `of` holds them.
    held: Arc<HeldNodes>,
}

impl<B: crate::spoken::Say> PartRow<B> {
    /// `doc`'s own row, reached through no instance of `doc`'s, its
    /// nodes as `doc` holds them: the ones the body's own sentence
    /// names, the sentence every carrier says it by.
    pub(crate) fn own<P: crate::program::ProfilePayload>(
        doc: &Doc<P>,
        of: crate::ident::DocumentId,
        body: B,
    ) -> Self {
        let held = Arc::new(crate::spoken::held_by(&body, doc));
        Self {
            of,
            via: Vec::new(),
            body,
            held,
        }
    }
}

impl<B> PartRow<B> {
    /// `of`'s own row whose body names no node a sentence says: an
    /// interference finding, whose copies are `of`'s variables and its
    /// faces `of`'s names.
    pub(crate) fn unspoken(of: crate::ident::DocumentId, body: B) -> Self {
        Self {
            of,
            via: Vec::new(),
            body,
            held: Arc::default(),
        }
    }

    /// A row `doc` carried up from its parts by `route`: its first
    /// instance is `doc`'s, said as `doc` holds it, and is the next
    /// hop of the document instantiating `doc`.
    pub(crate) fn below<P>(doc: &Doc<P>, route: Route, body: B, held: Arc<HeldNodes>) -> Self {
        Self {
            of: route.of,
            via: core::iter::once(doc.spoken(route.through))
                .chain(route.via)
                .collect(),
            body,
            held,
        }
    }
}

impl<B: Clone> PartRow<B> {
    /// The row as the document holding the instance `through` of the
    /// part reads it: its route, its body, and the body's nodes. The
    /// row's `of` does not move: which document authored it is not a
    /// function of who is looking.
    pub(crate) fn through(&self, through: RecipeNodeId) -> (Route, B, Arc<HeldNodes>) {
        (
            Route {
                through,
                of: self.of,
                via: self.via.clone(),
            },
            self.body.clone(),
            Arc::clone(&self.held),
        )
    }
}

/// What one instantiated value carries up from the documents below it:
/// their at-rest findings, each already tagged with the route it
/// arrived by.
///
/// Bundled because the rows travel together and are filled at ONE site
/// — the instantiate op — and are empty on every other op.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CarriedFindings {
    /// The interference the documents below reported between their own
    /// copies, each the part's finding, judged against the part's own
    /// assertions.
    pub interference: Vec<CarriedInterference>,
    /// What the documents below could not look at.
    pub at_rest: Vec<CarriedAtRestFinding>,
}

/// **An overlap a document BELOW this one reported between two of its
/// own copies**, arriving across the instantiation seam: reported, never
/// a refusal (D10), and quiet or loud as the part's own assertions
/// judged it.
///
/// The finding is the part's own, carried verbatim: its copies are
/// placements of `route.of`, its faces named as that document names
/// them.
#[derive(Debug, Clone, PartialEq)]
pub struct CarriedInterference {
    /// How it reached this document.
    pub route: Route,
    /// The pin of `route.of` the finding is of: two pins of one document
    /// are two part rows, and two findings.
    pub pin: crate::ident::ContentPin,
    /// The part's own finding, in `route.of`'s ids.
    pub finding: InterferenceFinding,
}

/// **One pair a document BELOW this one could not look at** (a touch
/// its census could not record, a pair it could not decide, a pair it
/// had no lane for): the part's own could-not-look finding, carried
/// verbatim, loud and never refusing.
#[derive(Debug, Clone, PartialEq)]
pub struct CarriedAtRestFinding {
    /// How it reached this document.
    pub route: Route,
    /// The pin of `route.of` the finding is of: two pins of one document
    /// are two part rows, and two findings.
    pub pin: crate::ident::ContentPin,
    /// The part's own finding, keyed in its own product.
    pub finding: AtRestEvidence,
    /// The finding's nodes, as `route.of` holds them.
    pub held: Arc<HeldNodes>,
}

// One carried finding as an author reads it: which document, and what
// its census found, as the part says it.
impl crate::spoken::Say for CarriedAtRestFinding {
    fn say(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        by: crate::spoken::Speaker<'_>,
    ) -> core::fmt::Result {
        write!(
            f,
            "{} reports what its own census could not look at: {}",
            crate::spoken::Said(&self.route, by),
            crate::spoken::Said(&self.finding, crate::spoken::Speaker::held(&self.held))
        )
    }
}

/// The row where no document is at hand: its route's first instance
/// by its tag.
impl core::fmt::Display for CarriedAtRestFinding {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::spoken::Say::say(self, f, crate::spoken::Speaker::TAG)
    }
}

/// **A part's carried findings, each once per part row** (A5, *A part's
/// own census findings*): `rows` appended to `held`, but for a row an
/// earlier list already holds, the same finding of the same pinned
/// document arriving by another route (a part placed twice, or in the world and
/// in another space). Only an earlier list's row is the same row: one
/// list is its part's, and two equal findings in it (escalations, which
/// name no entity) are two findings.
pub(crate) fn once_per_part_row<R: CarriedRow + Clone>(
    held: &mut Vec<R>,
    rows: impl IntoIterator<Item = R>,
) {
    let earlier = held.len();
    for row in rows {
        if !held[..earlier].iter().any(|h| h.same_row(&row)) {
            held.push(row);
        }
    }
}

/// A carried census row: the same row is the same document's same
/// finding, whatever route it came by.
pub(crate) trait CarriedRow {
    /// Whether `other` is this row by another route.
    fn same_row(&self, other: &Self) -> bool;
}

impl CarriedRow for CarriedInterference {
    fn same_row(&self, other: &Self) -> bool {
        (self.route.of, self.pin, &self.finding) == (other.route.of, other.pin, &other.finding)
    }
}

impl CarriedRow for CarriedAtRestFinding {
    fn same_row(&self, other: &Self) -> bool {
        (self.route.of, self.pin, &self.finding) == (other.route.of, other.pin, &other.finding)
    }
}

/// **A carried census finding with the pin of the document it is of**,
/// the body of a part's census row ([`mod@crate::eval`]'s `PartValue`): which
/// version of `of` found it, which a [`Route`] does not say.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Pinned<F> {
    pub(crate) pin: crate::ident::ContentPin,
    pub(crate) finding: F,
}

// Said as the finding is: the pin names no node.
impl<F: crate::spoken::Say> crate::spoken::Say for Pinned<F> {
    fn say(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        by: crate::spoken::Speaker<'_>,
    ) -> core::fmt::Result {
        self.finding.say(f, by)
    }
}

/// **A part's own at-rest reading, as it crosses the seam**
/// ([`mod@crate::eval`]'s part store): what the census over the part's
/// product found, read as the part's own at-rest check reads it
/// ([`super::reading`]), so the document instantiating the part reports
/// each finding once, as the part's. Neither half refuses the host.
pub(crate) struct SeamVerdict {
    /// The overlaps between the part's own copies.
    pub(crate) interference: Vec<InterferenceFinding>,
    /// What the part's census could not look at.
    pub(crate) could_not_look: Vec<AtRestEvidence>,
}

/// The part's at-rest reading over its gathered `product`
/// ([`SeamVerdict`]).
pub(crate) fn seam_verdict<T: LocalizeLane>(product: &Product<T>, tol: Tol) -> SeamVerdict {
    let reading = super::reading(product, tol);
    SeamVerdict {
        interference: reading.interference,
        could_not_look: reading.could_not_look,
    }
}

/// **One at-rest census decision** ([`crate::Product::coincidences`]):
/// a touch between two copies, which the record backing it cites, or a
/// flush pair between two copies, which no record cites
/// (`topo::AtRestPolicy::flush_at_rest`).
#[derive(Clone, Debug, PartialEq)]
pub struct AtRestRow {
    /// The two copies whose pair the census decided, earlier in gather
    /// order first.
    pub copies: (super::CopyRef, super::CopyRef),
    /// The decision, each cell named as the copy the census read it in
    /// reads it: the copy's placement output, and its name there.
    pub row: crate::coincide::NamedCoincidence,
}
