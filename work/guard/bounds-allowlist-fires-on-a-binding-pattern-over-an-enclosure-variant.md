---
id: bounds-allowlist-fires-on-a-binding-pattern-over-an-enclosure-variant
kind: issue
title: bounds-allowlist fires on a struct-pattern binding whose path ends in an Enclosure variant
status: open
opened: 2026-09-30
priority: P4
cost: E
---


Found by AUTH-7's fix pass (PR 3528). `scripts/gates/bounds-allowlist.sh`
failed on `crates/viewer/src/pane/features.rs`, which writes no trait
bound at all. The record it read was a test's match pattern:

```rust
let (reason, door) = match fixture
    .evaluation
    .result(fixture.clearance)
    .and_then(|result| result.value())
    .map(|value| &value.payload)
{
    Some(ValuePayload::MeasureUnavailable {
        reason: reason @ MeasureUnavailableAt::NeedsEnclosure { door, .. },
        ..
    }) => (reason.to_string(), *door),
    other => panic!("…: {other:?}"),
};
```

The variant name `NeedsEnclosure` ends in the gate's `Enclosure` suffix,
and one of the compound-bound alternatives in the gate's regex
(line ~377 of the script) matched this record even though it contains
no `+` in a bound position. AUTH-7 re-spelled the test around it:
the binding came out, and there is now a plain `let` over the one
variant. I did not work out which alternative fired.

The lane's repair was cheap, but the gate refused ordinary code with a
message telling the author to "ratify before allowlisting" a bound that
does not exist. A matcher that keys on the suffix may want to exclude
a path segment ending in `Enclosure` that is followed by `{` or `(`,
which is a variant, not a trait.
