---
id: review-d18-probes-header-miscounts-its-own-rows
kind: issue
title: review_d18_probes.rs's header says four rows and the file carries five
status: open
opened: 2026-09-15
priority: P4
cost: E
---


(S-TINT orchestrator, 2026-09-15) Filed on TOPO's slate because
`crates/topo/src/review_d18_probes.rs` is TOPO's territory
(`work.py territory`). Found by an S-TINT re-derivation lane sweeping
for header rosters that have gone stale; the class is S-TINT's, this
instance is on your ground, and no permission was needed either way
(`work/README.md`: a finding goes straight onto the slate of the program
whose ground it lands on).

## The defect

`crates/topo/src/review_d18_probes.rs`'s `//!` header says:

> *"Three of the four rows gate; the fourth is marked as evidence for
> the review"*

The file carries **five** `#[test]` rows:

- `d18_split_edge_refuses_a_dangling_prev_of_he_minus`
- `d18_split_edge_still_refuses_a_dangling_next_of_he_plus`
- `d18_kef_refuses_a_dangling_prev_of_he`
- `d18_torn_body_fixture_leaves_every_prev_live`
- `d18_no_unreachable_message_can_impersonate_the_postcondition`

So the header's arithmetic is wrong and, more usefully, its
**disposition** is unreadable: a reader cannot tell which row is the
evidence one, because the sentence describes a four-row file. Whoever
added the fifth did not update it, and nothing mechanical says so.

## What S-TINT thinks is worth knowing

This is one instance of a class S-TINT carries several members of —
a file header that enumerates or counts its own rows, kept by hand, with
no keeper (`work/tint/r2-m10-6-header-roster-omits-the-suites-heaviest-row`,
`work/tint/interrogate-ladder-header-claims-every-rung-and-pins-five`,
`work/tint/test-headers-name-fns-that-exist-nowhere`). **If S-TINT ever
lands an executable roster check, this closes for free**, so TOPO may
reasonably choose to sit on it rather than spend an edit. The cheap fix
meanwhile is one sentence, and it is TOPO's call which.

No fix is proposed and nothing is scheduled on TOPO's behalf.

## The roster mechanism now exists (S-TINT TINT-4, 2026-09-15)

The "if S-TINT ever lands an executable roster check" above is no longer
hypothetical. `test_utils::roster!` (`crates/test-utils/src/roster.rs`)
is the weld: a block of the file's own row **idents**, each with a
sentence beside it, compared against libtest's own
`--list --format=terse` of the running binary via a `current_exe()`
re-exec. A retired or misspelt name is `error[E0425]`; a row added
without an entry reds, naming the row. **Adopting it is TOPO's edit** —
`crates/topo/src/review_d18_probes.rs` is out of S-TINT's fence and
nothing is scheduled on TOPO's behalf.

What it would cost here, checked against this file so the estimate is
not a guess:

- The rows are at the top level of `review_d18_probes.rs`, which
  `crates/topo/src/lib.rs` mounts as `#[cfg(test)] mod
  review_d18_probes;`. So the invocation goes at the top of the file,
  `module_path!()` is `topo::review_d18_probes`, and the derived prefix
  is `review_d18_probes::` — the same shape the macro was measured on.
  No `pub` and no visibility change is needed.
- `topo` already dev-depends on `test-utils`.
- The header sentence *"Three of the four rows gate; the fourth is
  marked as evidence for the review"* would move into the block as five
  entries, one per row, each saying which it is.

**What it would NOT fix, and this is the half that matters here.** The
macro welds NAMES and never PROSE. The count in that sentence, and the
claim about which row is the evidence one, are exactly the kind of text
nothing checks — a roster keeps the five names honest and would say
nothing about a sentence that miscounts them. The count would stop
being written down at all, which is why it can no longer be wrong; but
if TOPO keeps a prose disposition beside the names, that disposition is
as unchecked after adoption as before.

**Two things the superseded draft of this note carried and this one
should keep.** The cheap alternative — one corrected sentence, no
mechanism — remains entirely reasonable, and nothing here argues
otherwise; and TINT-4's fix pass made a `#[test]` under a **nested
`mod`** a violation rather than a silent exemption, so a file adopting
`roster!` must have its rows at module level. Checked against this
file: its five rows are all top-level (`review_d18_probes.rs:69, 97,
156, 189, 271`) and there is no nested `mod`, so the constraint does
not bite here — but it is a hard rule now rather than a caveat.
