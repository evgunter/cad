//! **The shape a refusal the viewer shows must have**, checked on the
//! rendered sentence: the word budget, no stage prefix, no `Debug`
//! struct, no arena key, one recourse marker.
//!
//! The standard is stated once, in
//! `work/chrome/error-and-check-text-overflows-its-region.md` ("The
//! standard a refusal is rewritten to"); the budget there is 75 words,
//! counted on the text exactly as the viewer draws it.
//!
//! **Why the prefix check is structural.** A list of the prefixes a
//! rewrite removed sees only those; a prefix still on screen is by
//! construction not on it. So [`stage_prefixes`] reads the SHAPE a
//! stage prefix has instead: a clause that opens with one or two
//! lowercase words and then a colon (`section:`, `fit_offset:`,
//! `shell classification:`). An English clause does not open that way
//! — a sentence's own words precede its colon (`the Boolean op
//! refused:`), and the recourse label is capitalised (`Recourse:`) — so
//! the few labels that legitimately do are named by the caller.

/// The word budget for a refusal the viewer shows.
pub const BUDGET: usize = 75;

/// Every stage prefix in `text`, structurally: each clause — the text's
/// start, or what follows `": "`, `"— "`, `"; "` or `"("` — made of one
/// or two lowercase tokens (`a-z`, digits, `_`, `-`) and ending in a
/// colon, minus the labels in `allowed`.
#[must_use]
pub fn stage_prefixes(text: &str, allowed: &[&str]) -> Vec<String> {
    let mut found = Vec::new();
    for (colon, _) in text.match_indices(':') {
        let after = &text[colon + 1..];
        if !(after.is_empty() || after.starts_with(' ')) {
            continue;
        }
        let head = &text[..colon];
        let start = [": ", "— ", "; ", "("]
            .iter()
            .filter_map(|b| head.rfind(b).map(|i| i + b.len()))
            .max()
            .unwrap_or(0);
        let clause = head[start..].trim();
        let tokens: Vec<&str> = clause.split(' ').collect();
        let stage_like = !clause.is_empty()
            && tokens.len() <= 2
            && tokens.iter().all(|t| {
                let mut chars = t.chars();
                chars.next().is_some_and(|c| c.is_ascii_lowercase())
                    && chars.all(|c| {
                        c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-'
                    })
            });
        if stage_like && !allowed.contains(&clause) {
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

/// How many recourse markers `text` carries: each `Recourse:`, and each
/// `There is no way through` (the marker a refusal with no recourse
/// states instead). One message states one recourse, so a chain that
/// renders two — a wrapper's and its payload's — reads as a menu.
#[must_use]
pub fn recourse_markers(text: &str) -> usize {
    text.matches("Recourse:").count() + text.matches("There is no way through").count()
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
    let markers = recourse_markers(text);
    if markers > 1 {
        out.push(format!(
            "{name} states {markers} recourses, not one: {text}"
        ));
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
}
