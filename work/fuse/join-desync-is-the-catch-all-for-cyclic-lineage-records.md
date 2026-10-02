---
id: join-desync-is-the-catch-all-for-cyclic-lineage-records
kind: issue
title: BooleanError::JoinDesync, named and displayed as an A/B lockstep break, has become the catch-all for cyclic lineage records (fragment, split, absorption), beside dedicated cycle variants elsewhere
status: open
opened: 2026-10-02
priority: P3
cost: M
refs: [descendant-chase-spends-its-budget-into-a-dropped-contact-record, 3874]
---


## Finding (review of PR 3874, 2026-10-02, style lane, likely)

`BooleanError::JoinDesync` (`crates/topo/src/boolean/mod.rs`) is named
and displayed as an A/B lockstep invariant ("A/B lockstep invariant
violated: …"). It now carries three corrupt-lineage cycles that have
nothing A/B in them:

- `discard.rs`: "a face's fragment lineage is cyclic";
- the split-lineage arm;
- `Descendants::live_face` (PR 3874): "a face's absorption rows are cyclic".

The same concept has dedicated variants elsewhere:
`Body::split_root`'s `SplitLineageCycle`, and editor-core's
`FragmentLineage` and `SplitLineage`. So a corrupt-lineage cycle is
spelled two ways, and a user reading the boolean's message is told about
a lockstep break that did not happen.

What is owed: one typed corrupt-lineage refusal in `BooleanError`,
naming which lineage cycled. Every `JoinDesync { what: "...cyclic" }`
site moves to it, and `crates/pncad-py`'s `boolean_error_tag` and stubs
follow. Sweep every `JoinDesync` site for others that are not lockstep
breaks.
