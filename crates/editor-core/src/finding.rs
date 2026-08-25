//! **The document layer's finding sink** (DISCIPLINES-DESIGN DS8;
//! #981 part 1): one composition and one list rendering for the
//! layer's finding surfaces — the checks report and refusal, the
//! undeclared-contact refusal, and the assembly at-rest gate — so a
//! new finding kind plugs into a shared shape instead of hand-rolling
//! another renderer.
//!
//! The sink unifies the *rendering around* the findings, not the
//! findings: payloads stay per-site, evidence types stay where their
//! subject lives, and `Display` remains the one rendering surface
//! (the bindings' `kind` tags stay the machine channel). What must
//! NOT grow here, and why:
//!
//! - **no severity and no check identity** — those are report
//!   plumbing ([`crate::checks`]), not part of what a finding says;
//! - **no recourse enum** — the menus are genuinely different
//!   vocabularies per site (`topo`'s `CONTACT_RECOURSE` deliberately
//!   has no "lower the tolerance" arm; see its doc comment), and a
//!   shared enum would invite the generic tail this module exists to
//!   forbid;
//! - **no refusal-to-run types** — [`crate::ChecksError`] and its
//!   kin mean the analysis could not RUN, which is not a finding.

use core::fmt;

/// One user-facing finding: a subject, one story, and at most one
/// recourse, composed by [`compose`] and listed by [`render_list`].
///
/// `story` may forward a payload's own `Display` — forwarding is the
/// rule wherever the payload has one (the one-vocabulary discipline:
/// a site that re-states a payload it holds invents a second
/// vocabulary for a refusal that already has one).
///
/// `recourse` returns the finding's ONE recourse (the two-tolerance
/// principle's "one message, one recourse"), or `""` when the story
/// already ends in it — a forwarded kernel `Display` that carries its
/// own menu, or pinned prose whose recourse predates the sink. An
/// empty recourse is "already told", never "none": the composed
/// message still renders exactly one, and the sink never appends a
/// generic tail on top of a story that has one.
pub(crate) trait Finding {
    /// Writes what the finding is ABOUT — the attribution a user can
    /// act on (a check and its `(root, output)` subject, a mate's
    /// declaration, a refusing op). No trailing punctuation;
    /// [`compose`] supplies the joint.
    fn subject(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    /// Writes what was found — forwarding the payload's own `Display`
    /// where one exists.
    fn story(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    /// The one recourse, or `""` when the story already carries it.
    fn recourse(&self) -> &str;
}

/// The one composition: `subject: story — recourse` (the recourse
/// joint is omitted when [`Finding::recourse`] answers `""`, because
/// the story then ends in the site's own recourse).
pub(crate) fn compose<F: Finding + ?Sized>(f: &mut fmt::Formatter<'_>, finding: &F) -> fmt::Result {
    finding.subject(f)?;
    f.write_str(": ")?;
    finding.story(f)?;
    let recourse = finding.recourse();
    if !recourse.is_empty() {
        f.write_str(" — ")?;
        f.write_str(recourse)?;
    }
    Ok(())
}

/// The one list rendering: each finding composed on its own indented
/// line under a header the CALLER has already written (headers are
/// per-report prose — a count, a severity, a gate's name — not part
/// of any finding).
pub(crate) fn render_list<'a, F, I>(f: &mut fmt::Formatter<'_>, findings: I) -> fmt::Result
where
    F: Finding + 'a,
    I: IntoIterator<Item = &'a F>,
{
    for finding in findings {
        f.write_str("\n  ")?;
        compose(f, finding)?;
    }
    Ok(())
}
