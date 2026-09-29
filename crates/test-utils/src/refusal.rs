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
/// or two lowercase tokens (`a-z`, digits, `_`, `-`, and the prime `′`
/// a tier label carries, `tier-3′ census:`) and ending in a colon,
/// minus the labels in `allowed`.
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
                        c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '_' | '-' | '′')
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

/// Whether `text` names an arena key (`FaceKey(3v1)`, `EdgeKey(null)`)
/// or a document by its hex id (`3f9a…c2@81be…`, a `DocRef` or a
/// `DocumentId`'s `Display`).
///
/// A hex id is read by shape ([`hex_ids`]).
#[must_use]
pub fn arena_key(text: &str) -> bool {
    text.contains("Key(") || !hex_ids(text).is_empty()
}

/// **Every word of `text` that reads as a hex id**, by shape: a word
/// of at least [`HEX_ID_MIN`] hex digits holding both a decimal digit
/// and a letter, or one of decimal digits alone whose length is an id's
/// ([`HEX_ID_LENGTHS`]: a hex id that happens to hold no letter) and
/// which is not part of a decimal number.
///
/// An English word has no digit and a decimal number has no letter, so
/// neither reads as the first shape; the second reads a bare integer of
/// exactly an id's length as one, which a refusal has no other reason
/// to print.
#[must_use]
pub fn hex_ids(text: &str) -> Vec<&str> {
    let mut found = Vec::new();
    let mut rest = text;
    let mut before: Option<char> = None;
    while let Some(start) = rest.find(|c: char| c.is_ascii_alphanumeric()) {
        before = rest[..start].chars().next_back().or(before);
        let tail = &rest[start..];
        let len = tail
            .find(|c: char| !c.is_ascii_alphanumeric())
            .unwrap_or(tail.len());
        let (word, after) = tail.split_at(len);
        let hex = word.chars().all(|c| c.is_ascii_hexdigit());
        let digits = word.chars().all(|c| c.is_ascii_digit());
        let mixed = hex && !digits && word.chars().any(|c| c.is_ascii_digit());
        let decimal_part = before == Some('.')
            || (after.starts_with('.') && after[1..].starts_with(|c: char| c.is_ascii_digit()));
        if (mixed && word.len() >= HEX_ID_MIN)
            || (digits && HEX_ID_LENGTHS.contains(&word.len()) && !decimal_part)
        {
            found.push(word);
        }
        before = word.chars().next_back();
        rest = after;
    }
    found
}

/// The shortest run of hex digits [`arena_key`] reads as a document id:
/// a `DocRef`'s pin prefix, the shorter of its two halves.
pub const HEX_ID_MIN: usize = 12;

/// The lengths a hex id is printed at: a `DocRef`'s pin prefix, a
/// `DocumentId`, and a whole pin.
pub const HEX_ID_LENGTHS: [usize; 3] = [HEX_ID_MIN, 32, 64];

/// **A filed row's admission**: the exact span of its rendering that the
/// row filed at `filed` names, which the checks read as admitted and
/// nothing wider — not the row's other text, not another row.
#[derive(Clone, Copy, Debug)]
pub struct Admission<'a> {
    /// The row, by its exact name.
    pub row: &'a str,
    /// The span of the row's text the filed row names.
    pub span: &'a str,
    /// The work item that owns the fix.
    pub filed: &'a str,
}

/// **[`problems`] with `admissions` applied**: each admitted span of
/// this row is read as one uppercase placeholder word per word it holds,
/// so every other shape in the text is still checked, and an admission
/// whose span the text no longer holds is itself a problem — the list
/// cannot outlive what it admits.
#[must_use]
pub fn problems_admitting(
    name: &str,
    text: &str,
    allowed: &[&str],
    keyed: bool,
    admissions: &[Admission<'_>],
) -> Vec<String> {
    let mut out = Vec::new();
    let mut masked = text.to_owned();
    for admission in admissions.iter().filter(|a| a.row == name) {
        if !masked.contains(admission.span) {
            out.push(format!(
                "{name} no longer needs its admission of {:?} ({}): {text}",
                admission.span, admission.filed
            ));
        }
        let placeholder = vec!["ADMITTED"; admission.span.split_whitespace().count().max(1)];
        masked = masked.replace(admission.span, &placeholder.join(" "));
    }
    out.extend(problems(name, &masked, allowed, keyed));
    out
}

/// **Every admission that names no row in `names`**, as one problem
/// each: a roster whose row went away takes its admission with it.
#[must_use]
pub fn unclaimed_admissions<'a>(
    admissions: &[Admission<'_>],
    names: impl IntoIterator<Item = &'a str>,
) -> Vec<String> {
    let names: std::collections::BTreeSet<&str> = names.into_iter().collect();
    admissions
        .iter()
        .filter(|a| !names.contains(a.row))
        .map(|a| {
            format!(
                "the admission for {} names no row on the roster ({})",
                a.row, a.filed
            )
        })
        .collect()
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
        assert_eq!(
            stage_prefixes("tier-3′ census: an edge crossing a face", &[]),
            vec!["tier-3′ census:"]
        );
        assert!(debug_struct(
            "refused `budget` (Depth { max_cell_depth: 20 })"
        ));
        assert!(!debug_struct("a set {1, 2}"));
        assert!(arena_key("face FaceKey(null) is gone"));
        assert!(arena_key(
            "instantiating 3f9a0c41d2e87b6a5f10c9d8e7b6a5f1@81be0c2d4f6a: gone"
        ));
        assert!(arena_key(
            "the document 3f9a0c41d2e87b6a5f10c9d8e7b6a5f1 is gone"
        ));
        assert!(!arena_key("the offset is 0.30000000000000004 mm"));
        assert!(!arena_key("a deadbeef-like word, and 12345678901 items"));
        // A pin prefix whose twelve hex digits are all decimal.
        assert!(arena_key("the part pinned at 951583145512 is gone"));
        assert!(!arena_key("the offset is 0.300000000000 mm"));
        assert!(!arena_key("the offset is 123456789012.5 mm"));
        assert!(!arena_key("i64::MAX is 9223372036854775807"));
        assert_eq!(
            recourse_markers("x. Recourse: a. There is no way through yet"),
            2
        );
    }

    /// An admission admits its row's exact span and nothing wider, and
    /// reds once the row no longer holds it.
    #[test]
    fn an_admission_admits_its_span_and_nothing_else() {
        const ID: &str = "3f9a0c41d2e87b6a5f10c9d8e7b6a5f1@81be0c2d4f6a";
        let admissions = [Admission {
            row: "Part/ReferenceCycle",
            span: ID,
            filed: "work/edit/x.md",
        }];
        let check =
            |name: &str, text: &str| problems_admitting(name, text, &[], false, &admissions);
        let admitted = format!("the loop returns to {ID}. Recourse: break it");
        assert!(check("Part/ReferenceCycle", &admitted).is_empty());
        for (what, text) in [
            ("an arena key", format!("{admitted} at FaceKey(3v1)")),
            ("a Debug struct", format!("{admitted} (DocRef {{ id: 1 }})")),
            (
                "another id",
                format!("{admitted} and 11c1eee0e02516b19e263d060a3c9f80"),
            ),
        ] {
            assert!(
                !check("Part/ReferenceCycle", &text).is_empty(),
                "{what} on an admitted row is still red"
            );
        }
        assert!(
            !check("Part/NoResolver", &admitted).is_empty(),
            "another row is not admitted"
        );
        assert!(
            !check(
                "Part/ReferenceCycle",
                "the loop returns. Recourse: break it"
            )
            .is_empty(),
            "an admission its row no longer needs is red"
        );
        assert_eq!(
            unclaimed_admissions(&admissions, ["Part/NoResolver"]).len(),
            1,
            "an admission naming no roster row is red"
        );
    }
}
