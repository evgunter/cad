---
id: one-suite-owns-another-suites-certified-fixture-group
kind: issue
title: review_arceval's certified operands are m5_s12's, by cross-suite import rather than a shared home
status: parked
opened: 2026-09-20
blocked_on: [two-rules-disagree-on-when-a-fixture-leaves-a-suite]
---


## Finding

- **Where**: `crates/sweep/tests/m5_s12_curved_ops_interval.rs`'s
  `certified` module, and `crates/sweep/tests/review_arceval_r1_probes.rs`.
- **Importance**: low-medium
- **Confidence**: sure; read at `b29fe8bd1`
- **Raised by**: the `dup/one-line-fixture-wrappers` lane, 2026-09-20,
  correcting its own disposition of `m5_s12`'s `plate` — which it had
  filed as a singleton and which has two consuming suites

`review_arceval_r1_probes` reads

```rust
use crate::m5_s12_curved_ops_interval::certified::{
    RECUT_MAPPED_ENCLOSURE_HI, ball, plate, recut_ball,
};
```

Four shared items — a box fixture, a ball builder, a recut builder and
a certified constant — living in one suite and consumed by two. The
`pub(crate)` is deliberate and its reason is stated at the site: *"the
two rows say they use the same plate, and this is what makes that so
rather than saying it."* **That reason is about SHARING, and it is
satisfied by any single home**; what it does not decide is which home.

`crates/sweep/tests/common/mod.rs`'s routing rule seats an item at the
narrowest home all of its consumers can reach, which for these four is
`tests/common`, not a suite. Leaving them where they are makes
`m5_s12_curved_ops_interval` the owner of `review_arceval`'s fixtures —
the shape `crates/sweep/src/test_support.rs`'s own header gives as the
reason a shared home exists at all: hosting fixtures outside the
consuming modules *"keeps neither module the owner of the other's
fixture."*

## Why the fold that found it did not take it

`dup/one-line-fixture-wrappers` moved six box fixtures to
`crates/sweep/tests/common/operands.rs`. `plate` is a box and would go
there by the same rule; the other three are not this program's class
and would not. Moving one of four splits a group the site's rustdoc
binds together and leaves `review_arceval` importing from two places
with the ownership shape intact. Whoever takes this moves the group or
argues it should stay, and answers one question the split raises: the
constant `RECUT_MAPPED_ENCLOSURE_HI` is a certified MEASUREMENT, not a
fixture, and `common/oracles.rs`'s own rule about which per-suite
spellings may come to a shared home is the one that governs it.

Note the interaction: whether `tests/common` is even the right target
is the subject of
`work/dup/two-rules-disagree-on-when-a-fixture-leaves-a-suite.md`,
which is `needs_ev`. This row should not be worked before that one
answers.

## Why it sits on S-DUP's slate

`scripts/work.py territory` puts both files on S-TCOST's and S-TINT's
ground. The finding is about where a shared fixture lives, which is
this program's subject and neither of theirs, and S-DUP claims no
territory by design (`plan.md`). One row rather than two.
