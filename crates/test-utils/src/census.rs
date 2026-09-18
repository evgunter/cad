//! **One declared set against one witnessed set**, for the guards
//! whose whole subject is a list kept in step by hand.
//!
//! A census that compares two sets owes the same two answers wherever
//! it is written — what was declared and never witnessed, what was
//! witnessed and never declared — and a suite that spells the
//! comparison itself is keeping a second copy of the comparator in
//! step by hand, in a file whose subject is a second list kept in step
//! by hand. So it is written once, here, and its callers differ only
//! in the sentence each direction earns.

/// **The one set comparison in this tree**, reported rather than
/// asserted, and printing only the direction that actually failed.
///
/// `declared` is what a roster says the set is; `witnessed` is what the
/// run actually produced. The return is `None` when they agree and a
/// report naming both failing directions when they do not — reported
/// rather than asserted so that a caller walking several sets can name
/// every short one instead of aborting on the first.
///
/// `subject` opens the report; `undeclared_says` and `unwitnessed_says`
/// are the caller's sentences for the two directions, because only the
/// caller knows what its reader should do about either.
#[must_use]
pub fn set_difference(
    declared: &[&str],
    witnessed: &[&str],
    subject: &str,
    undeclared_says: &str,
    unwitnessed_says: &str,
) -> Option<String> {
    let unwitnessed: Vec<&&str> = declared.iter().filter(|d| !witnessed.contains(d)).collect();
    let undeclared: Vec<&&str> = witnessed.iter().filter(|w| !declared.contains(w)).collect();
    if unwitnessed.is_empty() && undeclared.is_empty() {
        return None;
    }
    // Only the failing direction is printed. A clean direction rendered
    // as `[]` beside a real one is noise in a message whose entire
    // purpose is that a reader can act on it without a local repro.
    let mut out = format!("{subject}:");
    if !unwitnessed.is_empty() {
        out.push_str(&format!("\n    {unwitnessed:?} — {unwitnessed_says}"));
    }
    if !undeclared.is_empty() {
        out.push_str(&format!("\n    {undeclared:?} — {undeclared_says}"));
    }
    Some(out)
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::set_difference;

    /// Agreement is `None`, regardless of order or of a witness
    /// produced twice — a set comparison, not a sequence comparison.
    #[test]
    fn agreement_reports_nothing() {
        assert!(set_difference(&["a", "b"], &["b", "a", "a"], "s", "u", "w").is_none());
        assert!(set_difference(&[], &[], "s", "u", "w").is_none());
    }

    /// Each direction names its own members and earns its own
    /// sentence, and a clean direction is absent rather than empty.
    #[test]
    fn each_direction_names_its_members_and_only_when_it_failed() {
        let only_unwitnessed =
            set_difference(&["a", "b"], &["a"], "subject", "undeclared", "unwitnessed")
                .expect("`b` is declared and unwitnessed");
        assert!(only_unwitnessed.contains("\"b\""));
        assert!(only_unwitnessed.contains("unwitnessed"));
        assert!(!only_unwitnessed.contains("undeclared"));

        let only_undeclared =
            set_difference(&["a"], &["a", "c"], "subject", "undeclared", "unwitnessed")
                .expect("`c` is witnessed and undeclared");
        assert!(only_undeclared.contains("\"c\""));
        assert!(only_undeclared.contains("undeclared"));
        assert!(!only_undeclared.contains("unwitnessed"));

        let both = set_difference(&["a"], &["c"], "subject", "undeclared", "unwitnessed")
            .expect("both directions disagree");
        assert!(both.contains("\"a\"") && both.contains("\"c\""));
        assert!(both.starts_with("subject:"));
    }
}
