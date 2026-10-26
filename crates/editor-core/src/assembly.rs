//! **The assembly at-rest gate** (ASSEMBLY-DESIGN A5; ASM-R2b D-3).
//!
//! An assembly's validity at rest is the kernel's tier-3′ census over
//! the gathered product, run once by the gather
//! ([`crate::product::product_recorded`]). A mate places a copy and
//! declares nothing: every touch the census finds between two copies,
//! mated or not, is a row of the product's own decision and a record
//! citing it ([`topo::record_at_rest`]), which the
//! `unproven-coincidence` check reads (D1 (iii), D10). The door proves a
//! mated pair's row by its mate (`coincide`'s first rung). A touch inside
//! one copy is that copy's defect, unless it is a part's own finding
//! found again inside the part's copy, which is the part's
//! (`seam_verdict`).
//!
//! # What this module does NOT do
//!
//! It runs no predicate of its own. Every geometric decision here is
//! the kernel's, called as-is: this layer reads the census's findings,
//! reports the overlaps between copies, and refuses what is left. There
//! is no second opinion about whether two faces touch.
//!
//! The kernel's finding passes straight through — stated, never
//! swallowed, never re-labelled. A part's own census findings are the
//! part's: the census over a document instantiating it finds them again
//! inside the part's copy, and the part's carried rows report them in
//! that re-finding's place, once (`checks::at_rest::less_carried`).

use std::sync::Arc;

use geom_core::Decide;
use topo::{ContactRecords, ValidationError};

use crate::doc::Doc;
use crate::eval::Evaluation;
use crate::names::NameTable;
use crate::node::RecipeNodeId;
use crate::product::{Product, ProductError, product_recorded};
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
    /// The at-rest refusals of the documents below, each crossing whole.
    pub at_rest: Vec<CarriedAtRestFinding>,
    /// The pairs of solids those findings are about, keyed in the same
    /// arena as the records, so a census over this body that finds
    /// them again inside the copy can tell they were answered
    /// ([`crate::Product::census_pairs`]).
    pub census_pairs: Vec<(topo::SolidKey, topo::SolidKey)>,
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
    pub finding: crate::checks::at_rest::InterferenceFinding,
}

/// **One finding of an at-rest refusal a document BELOW this one
/// raised** (a touch its census could not record, a pair it could not
/// decide, a pair it could not look at), the refusal's other findings
/// riding with it: the part's own verdict, carried verbatim and raised
/// by the outermost gate ([`AssemblyError::CarriedAtRest`]).
#[derive(Debug, Clone, PartialEq)]
pub struct CarriedAtRestFinding {
    /// How it reached this document.
    pub route: Route,
    /// The pin of `route.of` the finding is of: two pins of one document
    /// are two part rows, and two findings.
    pub pin: crate::ident::ContentPin,
    /// The part's own finding, keyed in its own product.
    pub finding: AtRestFinding,
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
            "{} is not at rest by its own census: {}",
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

/// A validated assembly: the gathered product, its names, and the
/// record set the at-rest gate certified.
#[derive(Debug)]
pub struct Assembly<T: Decide> {
    /// The gathered product body.
    pub body: topo::Body<T>,
    /// Its stable names.
    pub names: NameTable,
    /// The certified record set: the records each copy carried, and
    /// those the census recorded for the touches between copies, each
    /// citing its row ([`crate::Product::coincidences`]).
    pub contacts: ContactRecords,
    /// Every overlap between two copies' material in the world, in the
    /// census's order (D10):
    /// reported, quiet or loud, and never a refusal
    /// ([`crate::checks::at_rest`]).
    pub interference: Vec<crate::checks::at_rest::InterferenceFinding>,
    /// Every overlap a document BELOW this one reported between two of
    /// its own copies, once per part row, in gather order: the part's
    /// findings, quiet or loud by the part's own assertions, and never
    /// a refusal here either.
    pub carried_interference: Vec<CarriedInterference>,
}

// One at-rest finding: the kernel's own, FORWARDED verbatim through its
// `Display`, never restated. The kernel's tier-3′ messages end in their
// own recourse, so no document-layer sentence is appended.
impl crate::spoken::Say for AtRestFinding {
    fn say(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        _by: crate::spoken::Speaker<'_>,
    ) -> core::fmt::Result {
        write!(f, "{}", self.error)
    }
}

/// The finding where no document is at hand.
impl core::fmt::Display for AtRestFinding {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::spoken::Say::say(self, f, crate::spoken::Speaker::TAG)
    }
}

impl AtRestFinding {
    /// **The finding as the frame holding the assembled document says
    /// it.**
    #[must_use]
    pub fn spoken<P: crate::ProfilePayload>(&self, doc: &crate::doc::Doc<P>) -> String {
        crate::spoken::spoken_by(self, doc)
    }
}

/// One at-rest refusal: the kernel's finding.
#[derive(Debug, Clone, PartialEq)]
pub struct AtRestFinding {
    /// The kernel's own finding, verbatim: a finding about the
    /// product's own structure or geometry, an overlap or a touch no
    /// record backs inside one copy, a touch between copies the census
    /// could not clear (kept as a finding, not a row:
    /// `topo::record_at_rest`), or a pair the census could not look at.
    pub error: ValidationError,
}

/// Why the assembly gate refused.
#[derive(Debug)]
pub enum AssemblyError {
    /// The gather itself refused.
    Product(Box<ProductError>),
    /// **A `new_space()`'s copies did not gather** (D10, Spaces): the
    /// copies placed into it check inside it, and its gather refused,
    /// so it is not at rest there.
    Space {
        /// The space.
        space: crate::VarId,
        /// The space's gather refusal.
        refusal: Box<ProductError>,
    },
    /// The at-rest refusals of documents BELOW this one: a touch its
    /// census could not record, a pair it could not decide or a pair it
    /// could not look at. Each part's refusal crosses whole, in its own
    /// ids.
    ///
    /// Each is the part's finding, reported once, as the part's: the
    /// census over this product re-finds it inside the part's copy, and
    /// that re-finding is not a second finding of this document's
    /// ([`crate::checks::at_rest`]). Raised before this document's own
    /// refusals: the inner document is the file the author has to open.
    ///
    /// **Every finding travels**, in gather order, each with the route
    /// it arrived by. The door builds this arm only from a non-empty
    /// list; the type does not enforce that.
    CarriedAtRest {
        /// Every carried finding.
        findings: Vec<CarriedAtRestFinding>,
    },
    /// The kernel's tier-3′ door refused the assembled product with
    /// its records (A5). Every finding travels, in the kernel's own
    /// deterministic sweep order.
    AtRest {
        /// Every finding.
        findings: Vec<AtRestFinding>,
    },
}

impl crate::spoken::Say for AssemblyError {
    fn say(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        by: crate::spoken::Speaker<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::Product(e) => write!(f, "{}", crate::spoken::Said(&**e, by)),
            Self::Space { space, refusal } => write!(
                f,
                "the copies placed into {} do not gather: {}",
                by.space(*space),
                crate::spoken::Said(&**refusal, by)
            ),
            // Each row in the ids of the document below that found it,
            // but for its route's first instance, which is this
            // document's.
            Self::CarriedAtRest { findings } => {
                write!(
                    f,
                    "assembly: {} at-rest finding(s) refuse documents below this \
                     one, so this assembly is not at rest over them — open those \
                     documents and repair them there",
                    findings.len()
                )?;
                crate::finding::render_lines(f, findings.iter().map(|r| crate::spoken::Said(r, by)))
            }
            Self::AtRest { findings } => {
                write!(f, "{} finding(s) against this assembly:", findings.len())?;
                crate::finding::render_lines(f, findings.iter().map(|r| crate::spoken::Said(r, by)))
            }
        }
    }
}

/// The sentence where no document is at hand: this document's nodes
/// by their tags, and each carried row's as its own `Display` says it.
impl core::fmt::Display for AssemblyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::spoken::Say::say(self, f, crate::spoken::Speaker::TAG)
    }
}

impl AssemblyError {
    /// **The refusal as the frame holding the assembled document says it**:
    /// this document's nodes as `doc` holds them now
    /// ([`crate::Doc::spoken`]). A row carried up from a document below is
    /// spelled in that document's ids, so it is said as that document
    /// holds it, but for the first instance of its route, which is this
    /// document's.
    #[must_use]
    pub fn spoken<P: crate::ProfilePayload>(&self, doc: &crate::doc::Doc<P>) -> String {
        crate::spoken::spoken_by(self, doc)
    }
}

impl core::error::Error for AssemblyError {}

/// **The A5 gate, assembled** (D-3): take the document's product,
/// whose gather already ran the tier-3′ census over it and recorded
/// every touch between copies, and read the census's verdict.
///
/// **Gather, then [`assemble_gathered`], and nothing else.** The gate
/// itself lives at that door; this one exists for a caller with no
/// product in hand and adds only the gather and
/// [`AssemblyError::Product`], its own arm.
///
/// This is what "an assembly is valid at rest" MEANS: not a stronger
/// check bolted onto the product gather, but the same tier-3′ door the
/// boolean pipeline's results go through. The door is reached
/// through the SCALAR'S at-rest policy ([`topo::AtRestPolicy`],
/// `docs/DUAL-DESIGN.md` DL3): certifying scalars run
/// [`topo::validate_pseudomanifold`]'s verdict, reading tier 3's half of
/// it off the verdict the gather kept on the product's body
/// ([`topo::AtRestBody::validate_pseudomanifold`]), so the local battery
/// runs once per aggregate, and the census, which the gather ran once
/// and kept ([`crate::Product::census`]), is read rather than rerun; at a
/// dual the gate is structurally absent, and its success arm says so
/// ([`topo::AtRestOutcome::NotRunAtThisScalar`]).
///
/// **The pairing obligation (DL3), stated at this door**: at a
/// non-certifying scalar an `Ok(Assembly)` here is NOT a
/// certification — a document the `f64` census refuses assembles
/// green at `Dual64`, because nothing gated it. A dual evaluation is
/// sound only BESIDE a certifying-scalar run of the same recipe,
/// whose bit-identical value channel this same door validated; that
/// paired run is the validation of record. Nothing in the type
/// system enforces the pairing here; the E4 report enforces it where
/// a dual's numbers are read, binding each pass to the validated `f64`
/// evaluation by build identity (`stackup`'s module docs).
///
/// # Errors
///
/// [`AssemblyError`]: the gather's own refusals — including
/// [`crate::ProductError::EvaluationOfAnotherDocument`], which is the
/// ONE path this gate reads an evaluation through, so a mispaired
/// (document, evaluation) never reaches a predicate here — and the
/// kernel's tier-3′ findings.
pub fn assemble<P: crate::ProfilePayload, T: crate::checks::at_rest::LocalizeLane>(
    doc: &Doc<P>,
    evaluation: &Evaluation<T>,
    tol: Tol,
) -> Result<Assembly<T>, AssemblyError> {
    match product_recorded(doc, evaluation, tol) {
        Ok(product) => assemble_gathered(product, tol),
        // A world with no body is no verdict on the spaces beside it: a
        // document whose copies are all in `new_space()`s still checks
        // each inside itself before the gather's refusal is raised. A
        // world that refused for a fault of its own is answered first,
        // as its verdict is when it gathers.
        Err(refusal) => {
            if refusal.kind() == crate::ProductErrorKind::EmptyProduct {
                gate_spaces(
                    crate::product::space_gathers(doc, evaluation, tol),
                    tol,
                    &mut Reported {
                        interference: &mut Vec::new(),
                        carried: Vec::new(),
                    },
                )?;
            }
            Err(AssemblyError::Product(Box::new(refusal)))
        }
    }
}

/// What the gate reports beside its verdict, gathered across the world
/// and every space it checks: the interference it finds, and each part
/// row's carried findings once ([`Assembly::carried_interference`]).
struct Reported<'a> {
    interference: &'a mut Vec<crate::checks::at_rest::InterferenceFinding>,
    carried: Vec<CarriedInterference>,
}

/// **Each `new_space()`'s copies check as usual inside it**, and
/// against nothing outside it (D10, Spaces): the gate over each
/// [`crate::SpaceGather`] in turn, a space whose copies denote no body
/// holding nothing to check.
fn gate_spaces<T: crate::checks::at_rest::LocalizeLane>(
    spaces: Vec<crate::SpaceGather<T>>,
    tol: Tol,
    reported: &mut Reported<'_>,
) -> Result<(), AssemblyError> {
    for space in spaces {
        match space.gather {
            Ok(product) => {
                reported.interference.extend(verdict(&product, tol)?);
                once_per_part_row(
                    &mut reported.carried,
                    product.carried_interference.iter().cloned(),
                );
            }
            Err(refusal) if refusal.kind().means_no_body() => {}
            Err(refusal) => {
                return Err(AssemblyError::Space {
                    space: space.space,
                    refusal: Box::new(refusal),
                });
            }
        }
    }
    Ok(())
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

/// **The A5 gate over a product the caller already gathered** — the
/// canonical door, and everything [`assemble`] is once its gather is
/// done.
///
/// [`assemble`] is the wrapper for a caller with no product in hand;
/// this is the one for a caller that has one, so a document is gathered
/// once however many of its consumers need the product. The gate, its
/// verdicts and its refusal order are the same — they
/// are here, and there is one copy of them.
///
/// The product is CONSUMED: its body, names and records become the
/// [`Assembly`] on the success arm. A caller with another consumer of
/// the same product runs that consumer first.
///
/// This door reads no `(doc, evaluation)` pair and so re-states nothing
/// about the DI3 pairing: the gather is where an evaluation of another
/// document is refused, and this door's subject is whatever that gather
/// returned.
///
/// # Errors
///
/// [`AssemblyError`] except [`AssemblyError::Product`], which is the
/// wrapper's arm: the kernel's tier-3′ findings.
pub fn assemble_gathered<T: crate::checks::at_rest::LocalizeLane>(
    product: Product<T>,
    tol: Tol,
) -> Result<Assembly<T>, AssemblyError> {
    let mut interference = verdict(&product, tol)?;
    let Product {
        body,
        names,
        contacts,
        spaces,
        carried_interference,
        ..
    } = product;
    let mut reported = Reported {
        interference: &mut interference,
        carried: carried_interference,
    };
    gate_spaces(spaces, tol, &mut reported)?;
    let carried_interference = reported.carried;
    Ok(Assembly {
        body: body.into_body(),
        names,
        contacts,
        interference,
        carried_interference,
    })
}

/// **What the gate refuses before it reads the census**: the at-rest
/// refusals of the documents below. `None` when the gate goes on to its
/// at-rest stage ([`at_rest_stage`]).
fn ahead_of_at_rest<T: Decide>(product: &Product<T>) -> Option<AssemblyError> {
    // A part's own at-rest refusals, before this document's: the part
    // decided them, and the census over this product re-finds them only
    // inside the part's copy (`less_carried`, in the at-rest stage).
    (!product.carried_at_rest.is_empty()).then(|| AssemblyError::CarriedAtRest {
        findings: product.carried_at_rest.clone(),
    })
}

/// **The gate's at-rest stage**: the overlaps between copies, reported,
/// and every other verdict, which the gate refuses. Read by
/// [`verdict`] and, at the seam, by [`seam_verdict`]: one reading of
/// what the census found, so a part's verdict crosses as its own gate
/// gave it.
fn at_rest_stage<T: crate::checks::at_rest::LocalizeLane>(
    product: &Product<T>,
    tol: Tol,
) -> (
    Vec<crate::checks::at_rest::InterferenceFinding>,
    Vec<AtRestFinding>,
) {
    // The census's findings over the records, as the gather kept them. A
    // contact between copies is not among them: the gather recorded it
    // as a row (D10).
    //
    // What a part's census already reported, re-found here inside its
    // copy, is the part's, carried up and reported as the part's.
    let errors = crate::checks::at_rest::less_carried(product, product.census.clone());
    if errors.is_empty() {
        return (Vec::new(), Vec::new());
    }
    // An overlap between two copies is a finding, never a refusal
    // (D10); what is left is refused as it always was.
    let (interference, errors) = crate::checks::at_rest::partition(product, errors, tol);
    let findings = errors
        .into_iter()
        .map(|error| AtRestFinding { error })
        .collect();
    (interference, findings)
}

/// **The gate's verdict over the gathered world**: the one copy of the
/// gate's checks and their refusal order.
fn verdict<T: crate::checks::at_rest::LocalizeLane>(
    product: &Product<T>,
    tol: Tol,
) -> Result<Vec<crate::checks::at_rest::InterferenceFinding>, AssemblyError> {
    if let Some(refusal) = ahead_of_at_rest(product) {
        return Err(refusal);
    }
    let (interference, findings) = at_rest_stage(product, tol);
    if findings.is_empty() {
        Ok(interference)
    } else {
        Err(AssemblyError::AtRest { findings })
    }
}

/// **A part's own at-rest verdict, as it crosses the seam**
/// ([`mod@crate::eval`]'s part store): what the census over the part's
/// product found, judged as the part's own gate judges it, so the
/// document instantiating the part reports each finding once, as the
/// part's. A refusal crosses whole, because it refuses the
/// instantiating gate before that gate's own verdict
/// ([`AssemblyError::CarriedAtRest`]).
pub(crate) struct SeamVerdict {
    /// The overlaps between the part's own copies.
    pub(crate) interference: Vec<crate::checks::at_rest::InterferenceFinding>,
    /// The part's at-rest refusal, whole; empty where it has none.
    pub(crate) refused: Vec<AtRestFinding>,
    /// The pairs of solids both lists are about, with those the part
    /// carried from below, sorted, in the part's product.
    pub(crate) pairs: Vec<(topo::SolidKey, topo::SolidKey)>,
}

/// The part's at-rest verdict over its gathered `product`
/// ([`SeamVerdict`]): its gate's [`at_rest_stage`], as [`verdict`] reads
/// it, and nothing of it when the gate refuses before that stage
/// ([`ahead_of_at_rest`]).
///
/// One departure from the part's own gate, deliberate: the stage's
/// overlaps cross beside its refusal. The part's gate reports no overlap
/// beside a refusal, but an overlap that did not cross would be found
/// again inside the part's copy and refused as the instantiating
/// document's own.
pub(crate) fn seam_verdict<T: crate::checks::at_rest::LocalizeLane>(
    product: &Product<T>,
    tol: Tol,
) -> SeamVerdict {
    // A part whose gate refuses before its at-rest stage reports nothing
    // of its own census, here as at its own gate: what refuses it crosses
    // on its own rows (the at-rest refusals carried from below), and the
    // census's re-findings of those carried findings are no finding of
    // the part's.
    let (interference, refused) = if ahead_of_at_rest(product).is_some() {
        (Vec::new(), Vec::new())
    } else {
        at_rest_stage(product, tol)
    };
    let mut pairs: Vec<(topo::SolidKey, topo::SolidKey)> = product
        .census_pairs
        .iter()
        .copied()
        .chain(
            interference
                .iter()
                .flat_map(|finding| &finding.evidence)
                .chain(refused.iter().map(|finding| &finding.error))
                .filter_map(|error| crate::checks::at_rest::finding_solids(&product.body, error)),
        )
        .collect();
    pairs.sort_unstable();
    pairs.dedup();
    SeamVerdict {
        interference,
        refused,
        pairs,
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
    pub copies: (
        crate::checks::at_rest::CopyRef,
        crate::checks::at_rest::CopyRef,
    ),
    /// The decision, each cell named as the copy the census read it in
    /// reads it: the copy's placement output, and its name there.
    pub row: crate::coincide::NamedCoincidence,
}
