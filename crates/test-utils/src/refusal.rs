//! **The shape a refusal the viewer shows must have**, checked on the
//! rendered sentence: the word budget, no stage prefix, no `Debug`
//! struct, no arena key, exactly one recourse.
//!
//! The standard is stated once, in
//! `work/chrome/error-and-check-text-overflows-its-region.md` ("The
//! standard a refusal is rewritten to"); the budget there is 75 words,
//! counted on the text exactly as the viewer draws it.
//!
//! **Why the prefix check is structural.** A list of the prefixes a
//! rewrite removed sees only those; a prefix still on screen is by
//! construction not on it. So [`stage_prefixes`] reads the SHAPE a
//! stage prefix has instead: a clause ending in a colon that is a bare
//! label rather than a sentence (`section:`, `shell classification:`,
//! `A/B lockstep invariant violated:`, `predicate 'x' indeterminate:`).
//! A sentence's clause carries a word only a sentence has — an article,
//! a determiner, an auxiliary, a negation, or the wrappers' own verbs
//! (`the Boolean op refused:`, `node 5 failed:`, `There is no way
//! through:`) — and a label carries none. The few labels a surface
//! legitimately opens with are named by the caller.

/// The word budget for a refusal the viewer shows.
pub const BUDGET: usize = 75;

/// The words whose presence makes a clause a sentence rather than a
/// label: articles, determiners, pronouns, auxiliaries, negations, and
/// the three verbs every wrapper states its refusal with. Matched on a
/// token lowercased and stripped of surrounding punctuation.
const SENTENCE_WORDS: &[&str] = &[
    "a",
    "an",
    "the",
    "this",
    "that",
    "these",
    "those",
    "its",
    "their",
    "it",
    "they",
    "there",
    "no",
    "not",
    "every",
    "each",
    "any",
    "is",
    "are",
    "was",
    "were",
    "be",
    "been",
    "has",
    "have",
    "had",
    "does",
    "do",
    "did",
    "can",
    "cannot",
    "could",
    "would",
    "will",
    "should",
    "may",
    "might",
    "must",
    "which",
    "whose",
    "whether",
    "so",
    "failed",
    "refused",
    "escalated",
];

/// The recourse phrases the kernel's shared vocabulary states WITHOUT a
/// `Recourse:` label (`geom_core::COINCIDENCE_RECOURSE`,
/// `SPLIT_PLANE_RECOURSE` and `NO_DECLARATION_RECOURSE`, longest
/// first, since the last is a suffix of the first). This crate has no
/// dependencies, so the text is restated here; a caller that has
/// `geom_core` holds the two equal.
pub const BARE_RECOURSES: &[&str] = &[
    "declare the coincidence, move the geometry, or lower the tolerance",
    "move the split plane or the geometry, or lower the tolerance",
    "move the geometry, or lower the tolerance",
];

/// `head` with every parenthesised group that closes inside it replaced
/// by a space, so a parenthetical never opens a clause.
fn without_closed_groups(head: &str) -> String {
    let mut out = String::with_capacity(head.len());
    let mut open: Vec<usize> = Vec::new();
    for c in head.chars() {
        match c {
            '(' => {
                open.push(out.len());
                out.push(c);
            }
            ')' => match open.pop() {
                Some(at) => {
                    out.truncate(at);
                    out.push(' ');
                }
                None => out.push(c),
            },
            _ => out.push(c),
        }
    }
    out
}

/// Every stage prefix in `text`, structurally: each clause — the text's
/// start, or what follows `": "`, `"— "`, `"; "`, `". "`, `", "`, a line
/// break or an unclosed `"("` — that ends in a colon and carries none of
/// the words a sentence has ([`SENTENCE_WORDS`]), minus the labels in
/// `allowed` and the recourse label itself.
#[must_use]
pub fn stage_prefixes(text: &str, allowed: &[&str]) -> Vec<String> {
    let mut found = Vec::new();
    for (colon, _) in text.match_indices(':') {
        let after = &text[colon + 1..];
        if !(after.is_empty() || after.starts_with(' ') || after.starts_with('\n')) {
            continue;
        }
        let head = without_closed_groups(&text[..colon]);
        let start = [": ", "— ", "; ", ". ", ", ", "(", "\n"]
            .iter()
            .filter_map(|b| head.rfind(b).map(|i| i + b.len()))
            .max()
            .unwrap_or(0);
        let clause = head[start..].trim();
        if clause.is_empty() || clause == "Recourse" || allowed.contains(&clause) {
            continue;
        }
        let sentence = clause.split_whitespace().any(|t| {
            let word = t
                .trim_matches(|c: char| !c.is_alphanumeric() && c != '\'')
                .to_lowercase();
            SENTENCE_WORDS.contains(&word.as_str())
        });
        if !sentence {
            found.push(format!("{clause}:"));
        }
    }
    found
}

/// Whether `text` renders a `Debug` struct: an identifier, then
/// `" { "`, then a field name and a colon (`Depth { max_cell_depth: 20 }`).
#[must_use]
pub fn debug_struct(text: &str) -> bool {
    text.match_indices(" { ").any(|(i, _)| {
        let before_ok = text[..i]
            .chars()
            .next_back()
            .is_some_and(|c| c.is_alphanumeric() || c == '_');
        let field: String = text[i + 3..]
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        before_ok && !field.is_empty() && text[i + 3 + field.len()..].starts_with(':')
    })
}

/// Whether `text` names an arena key (`FaceKey(3v1)`, `EdgeKey(null)`).
#[must_use]
pub fn arena_key(text: &str) -> bool {
    text.contains("Key(")
}

/// How many recourses `text` states: each `Recourse:`, each "there is no
/// way through" in either case (the marker a refusal with no recourse
/// states instead), and each [`BARE_RECOURSES`] phrase stated in a
/// sentence no `Recourse:` opened. One message states one recourse, so
/// a chain that renders two — a wrapper's and its payload's — reads as
/// a menu, and one that renders none has dropped the part the standard
/// never drops.
#[must_use]
pub fn recourse_markers(text: &str) -> usize {
    let lower = text.to_lowercase();
    let labelled = text.matches("Recourse:").count();
    let dead_ends = lower.matches("there is no way through").count();
    let mut taken: Vec<std::ops::Range<usize>> = Vec::new();
    let mut bare = 0;
    for phrase in BARE_RECOURSES {
        for (at, _) in lower.match_indices(phrase) {
            let span = at..at + phrase.len();
            if taken
                .iter()
                .any(|t| t.start < span.end && span.start < t.end)
            {
                continue;
            }
            taken.push(span);
            let sentence_start = lower[..at].rfind(". ").map_or(0, |i| i + 2);
            if !lower[sentence_start..at].contains("recourse:") {
                bare += 1;
            }
        }
    }
    labelled + dead_ends + bare
}

/// Every way `text` falls short of the standard, as one line each.
///
/// `allowed` names the clause labels a caller's surface legitimately
/// opens with; `keyed` says whether this row is one allowed to name an
/// arena key (a kernel bug, whose report needs it).
#[must_use]
pub fn problems(name: &str, text: &str, allowed: &[&str], keyed: bool) -> Vec<String> {
    let mut out = Vec::new();
    let words = text.split_whitespace().count();
    if words > BUDGET {
        out.push(format!(
            "{name} renders {words} words, over {BUDGET}: {text}"
        ));
    }
    for prefix in stage_prefixes(text, allowed) {
        out.push(format!(
            "{name} carries the stage prefix {prefix:?}: {text}"
        ));
    }
    if debug_struct(text) {
        out.push(format!("{name} renders a Debug struct: {text}"));
    }
    if !keyed && arena_key(text) {
        out.push(format!("{name} dumps an arena key: {text}"));
    }
    match recourse_markers(text) {
        0 => out.push(format!("{name} states no recourse: {text}")),
        1 => {}
        markers => out.push(format!(
            "{name} states {markers} recourses, not one: {text}"
        )),
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each check reads the shape, so each goes red on a prefix, struct
    /// or key it was never told about, and stays quiet on prose.
    #[test]
    fn each_check_sees_a_shape_it_was_never_listed() {
        assert_eq!(
            stage_prefixes("the shell op refused: replace_face_offset: gone", &[]),
            vec!["replace_face_offset:"]
        );
        assert_eq!(
            stage_prefixes("the op refused: shell classification: shell 3", &[]),
            vec!["shell classification:"]
        );
        assert_eq!(
            stage_prefixes("section: the surfaces", &[]),
            vec!["section:"]
        );
        assert!(
            stage_prefixes(
                "node 5 failed: the Boolean op refused: the faces meet. Recourse: move it",
                &[]
            )
            .is_empty()
        );
        assert!(stage_prefixes("check separation: root 4", &["check separation"]).is_empty());
        assert_eq!(
            stage_prefixes("tier-3′ census: an edge crossing a face", &[]),
            vec!["tier-3′ census:"]
        );
        assert!(debug_struct(
            "refused `budget` (Depth { max_cell_depth: 20 })"
        ));
        assert!(!debug_struct("a set {1, 2}"));
        assert!(arena_key("face FaceKey(null) is gone"));
        assert_eq!(
            recourse_markers("x. Recourse: a. There is no way through yet"),
            2
        );
    }

    /// The shapes `the-refusal-shape-guard-has-blind-spots` found the
    /// first reading passing over are each red, and the sentences either
    /// side of them stay quiet.
    #[test]
    fn the_prefix_shapes_the_first_reading_missed_are_red() {
        // After a full stop, and after a comma.
        assert_eq!(
            stage_prefixes("the op refused. fit: no samples", &[]),
            vec!["fit:"]
        );
        assert_eq!(
            stage_prefixes("the op refused, knot algebra: a knot", &[]),
            vec!["knot algebra:"]
        );
        // Three words and more.
        assert_eq!(
            stage_prefixes("declared-REST union zip: the loops", &[]),
            vec!["declared-REST union zip:"]
        );
        assert_eq!(
            stage_prefixes(
                "the op refused: path junction classification: predicate 'turn' indeterminate: \
                 margin 1e-9",
                &[]
            ),
            vec![
                "path junction classification:",
                "predicate 'turn' indeterminate:"
            ]
        );
        // Capitalised.
        assert_eq!(
            stage_prefixes("A/B lockstep invariant violated: face 3", &[]),
            vec!["A/B lockstep invariant violated:"]
        );
        // A parenthetical does not open a clause, and a sentence that
        // carries one is still a sentence.
        assert!(
            stage_prefixes(
                "the junction reverses (turn margin 1e-12 m on a 0.5 m arm): a cusp",
                &[]
            )
            .is_empty()
        );
        assert!(
            stage_prefixes("mapped edge EdgeKey(null) failed re-certification: x", &[]).is_empty()
        );
        assert!(stage_prefixes("There is no way through: this is a defect", &[]).is_empty());
    }

    /// A message that loses its recourse is red, and so is one that
    /// states two, whichever spelling each is in.
    #[test]
    fn a_lost_or_doubled_recourse_is_red() {
        let lost = problems("row", "the faces meet", &[], false);
        assert!(
            lost.iter().any(|p| p.starts_with("row states no recourse")),
            "{lost:?}"
        );
        assert_eq!(
            recourse_markers("the extrusion refused; there is no way through this yet"),
            1
        );
        assert_eq!(
            recourse_markers(
                "a near-coincidence; declare the coincidence, move the geometry, or lower the \
                 tolerance"
            ),
            1
        );
        assert_eq!(
            recourse_markers(
                "a near-coincidence. Recourse: declare the coincidence, move the geometry, or \
                 lower the tolerance"
            ),
            1
        );
        assert_eq!(
            recourse_markers(
                "x — move the geometry, or lower the tolerance. Recourse: move the split plane"
            ),
            2
        );
    }
}
