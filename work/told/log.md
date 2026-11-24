# TOLD log

Newest entries at the bottom; the tail is the program's live status.

- 2026-11-23 — (GREEN, U17, PR 7676) `test_utils::source::funnel_sites` no longer counts a `decide` call whose first argument is a closure as a predicate-name site. So `quantity`'s `Exact::decide` (`crates/quantity/src/exact.rs`) no longer makes `predicate_name_coverage` ask `quantity` for a roster. No change to `quantity`.
