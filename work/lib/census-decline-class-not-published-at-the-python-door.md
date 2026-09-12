---
id: census-decline-class-not-published-at-the-python-door
kind: issue
title: the census decline's carried cause is not published at the Python door — CUR3's row, the six rotten prelude sentences, and the equality question come with it
status: open
opened: 2026-09-11
---


(FIX orchestrator) Split out of PR 2354
(`work/fix/census-flattens-the-typed-chart-region-declines`) on that
PR's style review, which recommended the LIB half come out. Filed here
rather than carried, because everything below is LIB's ground and LIB
has no open PR (checked 2026-09-11).

## What landed in `topo`, and what did not cross

PR 2354 stopped `crates/topo/src/census.rs` flattening twelve typed
`ChartRegionError` refusals onto one `ValidationError::CensusUnsupported`.
That variant now carries `cause: CensusUnsupportedCause` —
`ChartRegion(ChartRegionError)`, `ContactLane(&'static str)`,
`FaceUnboundable` — and its `Display` renders the refusing lane's own
sentence.

**The class stops at the Rust door.** A Python consumer reading a
validation finding still gets prose and cannot branch on which lane
declined. That is the same shape `boolean-kind-not-published-at-the-python-door`
closed one crate over, and it is filed the same way.

## Why it was pulled out rather than carried

Not scope for its own sake. The carry was written and reverted because
it **publishes two new prelude names without the guard that makes the
rule it invokes mechanical**:

- `crates/pncad/tests/all.rs:529`,
  `carried_refusal_payloads_are_matchable_through_the_prelude`, is the
  CUR3 property row — *a refusal the prelude names must be MATCHABLE
  THROUGH the prelude*. The reverted half added `CensusUnsupportedCause`
  and `ChartRegionError` to the prelude and touched no file under
  `crates/pncad/tests/`. The rule the carry was justified by got two
  subjects and no row.
- The premise those files state in prose is **already false**, in six
  places, and PR 2354's own new doc (`crates/topo/src/validate.rs:415-420`)
  is what proves it: *"an entity subject is one carrier outside the
  certifiable inventory"* at `crates/pncad/src/prelude.rs:487`,
  `crates/pncad/tests/all.rs:382`, `crates/pncad-py/src/tags.rs:2286`,
  `crates/pncad-py/src/py/value.rs:420`, `crates/pncad-py/pncad.pyi:233`
  and `crates/pncad-py/tests/test_validate.py:456`. `FaceUnboundable`
  is an `Entity(Face)` subject where nothing about the face's carrier
  refused, so the inventory sentence names the wrong thing. Doc-rotted
  rather than code-drifted — the raise predates that PR — but the PR is
  what proves it rotten, and four of the six sit in the files the
  reverted half edited.

A LIB-owned PR has to do the row and the six sentences anyway, so the
carry belongs here whole.

## Four findings that come with it, from the same review

1. **`chart_escalated` would be a pinned wire word no door can emit.**
   `ChartRegionError::Escalated` was mapped to a tag and pinned in
   `TAG_INVENTORY` as a value "Python was promised", while the census
   routes escalations to `CensusEscalated` — so no `decline_kind` can
   ever carry it. Spelling it keeps the compile fence a wildcard would
   lose, which is the right trade; the inventory is the artifact that
   reads as a contract and would gain a dead entry with nothing marking
   it dead.
2. **The tag would be thirteen words for one lane and one for the
   other.** The chart lane delegates to its full arm vocabulary;
   every contact-lane decline collapses to `"contact_lane"`, discarding
   eight distinct `what` strings
   (`crates/topo/src/boolean/contact_verify.rs:106,157,236,239,245,249,295,301`).
   The flatness was argued from *"which LANE refused is not a fact a
   caller acts on"* — but the asymmetry is not lane-versus-arm: a
   caller could distinguish a stopped witness search from a thin
   overlap and could not distinguish a missing Tangent locus from an
   unresolvable face.
3. **The prelude would publish the `Err` half of a `Result` whose `Ok`
   half and door stay behind the façade.** `ChartOverlap`,
   `chart_region_overlap` and `declared_pair_overlap`
   (`crates/topo/src/lib.rs:315-316`) are not prelude names, so a
   consumer could match the refusal only as someone else's nested
   payload and could never call the function that produces it.
4. **Python-visible finding equality moves, and the "no answer moves"
   framing does not cover it.** `Finding`
   (`crates/pncad-py/src/validation.rs:40`) derives `PartialEq`/`Eq`
   and `ValidationFinding`'s docstring promises *"two that say the same
   thing compare equal and hash equal"*. Two declines on the same pair
   from different chart arms were one value and would become two. No
   kernel verdict moves — PR 2354's central claim is about
   `AssemblyError` and holds — but what a Python caller **counts**
   changes, and no test or caller that sets or dedups findings was
   found either way. Establish that before publishing.
