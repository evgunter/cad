---
id: landing-gathers-source-text-row-has-a-gate-now
kind: issue
title: landing_gathers.rs's source-text row over product.rs's gather counter now duplicates a CI gate on the gatedness axis
status: open
opened: 2026-09-06
priority: P4
cost: E
---


## Finding

Filed by the GATES orchestrator from PR 2030's style review (Q1).
`crates/viewer/tests/landing_gathers.rs::every_site_of_the_gather_counter_carries_the_debug_gate`
reads `crates/editor-core/src/product.rs` as text and pins that the
gather counter's sites are under `#[cfg(debug_assertions)]`. Since PR
2030 `scripts/gates/bit-identity-debug-only.sh` scans the same file for
the same question with a real enclosure analysis, so on the gatedness
axis the row is a second reader. What the row pins that the gate does
not: presence (the gate is green on a subject with zero hits), an exact
count of three (today the only mechanical check that `GATHERS` occurs
nowhere else in the file, the gate's KNOWN GAP 3 premise), and exact
attribute-to-statement adjacency (an `#[allow]` between them breaks the
row and not the gate). The decision — retire the row, or narrow it to
the presence-and-count half and cite the gate for gatedness — is a
test-mechanism call on S-TCOST's glob (VIEW's file by charter); the
row's `crates/test-utils/tests/reader_census.rs` ledger line moves
with it. Sequenced after PR 2030 lands.

## Moved to S-TINT (2026-09-11)

Moved by `git mv` from `work/tcost/` at S-TINT's opening. Id, title and
body are unchanged; the directory is the claim (`work/README.md`).

**Why it moved.** S-TCOST's board was re-sorted on 2026-09-11 against the
repository going public on 2026-09-03 (`work/tcost/log.md`, the
2026-09-11 seam). That sort found this row is not a cost lever in either
currency — it neither shortens the gate's critical path nor saves a
billed minute, and it was never argued on one. It reached S-TCOST by the
tracker-wide re-home of 2026-09-04, which routed rows by PATH GLOB
(`crates/*/tests/*`, `crates/test-utils/*`) rather than by question.
This program is the question it was always about: whether the suite
asserts what it claims to assert.

## Re-derived (2026-09-15, lane C)

**VERDICT: PARTIAL** — the duplication is real and both readers are
still in the tree, but **two of the three things this row says the row
pins and the gate does not are now false**: the gate pins presence and
pins a count. Only the adjacency claim survives, so the decision this row
frames should be re-taken on a narrower basis than it was filed on.

**Both readers are live.**
`crates/viewer/tests/landing_gathers.rs::every_site_of_the_gather_counter_carries_the_debug_gate`
still reads `crates/editor-core/src/product.rs` through
`test_utils::source::code_only`, still asserts three literal anchors and
`source.matches("GATHERS").count() == gated.len()`. Its ledger line in
`crates/test-utils/tests/reader_census.rs` is still there
(`crates/viewer/tests/landing_gathers.rs`, `Shared`).
`scripts/gates/bit-identity-debug-only.sh` still carries the subject row
`crates/editor-core/src/product.rs GATHERS|gathers_on_this_thread 4 the
debug-only gather counter`.

**Claim-by-claim, re-derived:**

| the row says the test pins and the gate does not | today |
| --- | --- |
| **presence** — *"the gate is green on a subject with zero hits"* | **false.** Every subject row now pins a use count, and the gate's own selftest covers a missing subject: its summary names *"a missing subject (asked one subject at a time, because the presence check ends at the first one gone)"* as a loud failure case (`plant_subject_gone`). A subject with zero hits reds. |
| **an exact count of three** — *"today the only mechanical check that `GATHERS` occurs nowhere else in the file"* | **false.** The gate re-derives the file's use count every run and compares with `-ne`, in both directions, and its selftest shifts every pin to prove the comparison happens (*"Run in both directions, because `-ne` written as `-lt` would pass a count that ROSE"*). The refusal text says it outright: *"A count that FELL means the gate is holding less than the row claims; one that ROSE means the mechanism grew where nobody re-read the enclosure argument."* |
| **exact attribute-to-statement adjacency** — *"an `#[allow]` between them breaks the row and not the gate"* | **true, still.** The test's three needles are literal strings with embedded newlines (`"#[cfg(debug_assertions)]\n    GATHERS.with("` and the two others), so anything between the attribute and the item breaks the row; the gate reads enclosure by brace depth and by `debug_assert!` statement, which does not care. |

**The two counts are over different spellings, which is worth stating
before anyone calls them redundant.** The test counts `GATHERS` and pins
3; the gate counts `GATHERS|gathers_on_this_thread` and pins 4.
`crates/editor-core/src/product.rs` carries exactly four matching uses
today — `static GATHERS`, `pub fn gathers_on_this_thread`, the
`GATHERS.with(std::cell::Cell::get)` inside it, and
`GATHERS.with(|gathers| …)` in `product_recorded` — so a fourth `GATHERS`
use would raise the gate's count to 5 and red it. The gate's KNOWN GAP 3
(identifier-boundary matching, so `PREGATHERS` is not a use) is the only
place the test's narrower needle could still see something the gate
cannot, and a `GATHERS`-containing longer identifier is not a gather site.

**What this leaves.** The row's framing — *"retire the row, or narrow it
to the presence-and-count half and cite the gate for gatedness"* — no
longer has a presence-and-count half to narrow to. The only residue the
test carries alone is attribute-to-statement adjacency, which is a
formatting pin rather than a claim about the kernel, plus the fact that
the test runs in the per-PR test matrix while the gate runs as a gate
job. Whether that residue is worth a source-text reader in `viewer`'s
test suite is the decision, and it is now closer to "retire" than it was
when filed.

**Not settled here, and it needs a run to settle:** whether the two
readers actually agree at every ε row and feature lane — the test is in
`crates/viewer/tests/` and compiles unconditionally, the gate is a shell
job. Nothing in this re-derivation depends on that, but a unit that
retires the test should confirm the gate job is in the twelve-job gating
set rather than a conditional one before deleting the only other reader.

**Sequencing note.** PR 2030 has landed (the gate exists and carries the
`product.rs` subject row), so the "sequenced after PR 2030 lands" hold in
the `## Finding` is discharged.

**Recommendation (orchestrator's call).** Keep open but re-frame: the
decision is now "retire the row, or keep it for adjacency alone and say
so in its doc". Two of its three stated reasons to keep it are gone.
