---
id: one-suite-owns-another-suites-certified-fixture-group
kind: issue
title: review_arceval's certified operands are m5_s12's, by cross-suite import rather than a shared home
status: closed
opened: 2026-09-20
closed: 2026-09-28
priority: P4
cost: E
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

## Closed (2026-09-28)

Ev ruled that `tests/common`'s narrowest-home rule governs (see
`two-rules-disagree-on-when-a-fixture-leaves-a-suite`), so this is an
ordinary fold. At the merge base the cross-suite import named
**three** items, not the four quoted above: `certified::ball` had
already gone (`the-interval-ball-fixture-is-homed-in-a-row-that-never-varies-it`),
leaving `plate`, `recut_ball` and `RECUT_MAPPED_ENCLOSURE_HI`.

All three moved together to a new `crates/sweep/tests/common/sphere_recut.rs`,
and both `m5_s12_curved_ops_interval` and `review_arceval_r1_probes`
import them from there. `m5_s12`'s `certified` module is private again.

- **The group stays whole.** The constant is a measurement of this
  exact pair of bodies, so moving the plate to `common/operands.rs` by
  shape would have split what the site's reason binds. `operands.rs`'s
  not-absorbed list names the plate, and the new module's own list
  names `operands` and `oracles`.
- **The constant, under `oracles.rs`'s rule.** That rule lets a
  spelling come to a shared home when it cannot disagree with the
  other, and this constant has one spelling that both rows read. It
  does not go to `oracles` itself, which holds truths derived WITHOUT
  the kernel. This constant is the opposite: a measurement of the
  kernel.
- **The site's reason is kept.** Its sentence, *"the two rows say they
  use the same plate, and this is what makes that so rather than
  saying it"*, is now the module header's, and the shared home
  satisfies it.
- `common/mod.rs`'s routing list, `review_arceval_r1_probes`'s header
  and `m5_s13_pips_interval`'s pointer to the constant name the new
  home.

**Plants**, on `common/sphere_recut.rs`, scoped to the two consuming
suites (6 rows; baseline 6/6 at both ε). Each plant was restored by
copying the file's bytes back, then checked clean against `HEAD`.

| plant | ε | red |
| --- | --- | --- |
| constant `…965_9e-12` → `…967e-12` | default | 0: the constant only selects the arm above 1e-12 |
| same | 1e-12 | 2: `m5_s12` sphere row, E2 (both on the bit-exact `hi ==`) |
| ball centre `z` 0.5 → 0.55 | default | 1: `m5_s12` sphere row (volume enclosure) |
| same | 1e-12 | 2: `m5_s12` sphere row, E2 (`hi` moved to `1.1177e-12`) |
| `plate()` → `panic!` | default | 2: `m5_s12` blind hole, sphere row (E2 returns before building) |
| same | 1e-12 | 3: those two and E2 |

