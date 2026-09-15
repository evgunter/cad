---
id: review-d18-probes-header-miscounts-its-own-rows
kind: issue
title: review_d18_probes.rs's header says four rows and the file carries five
status: open
opened: 2026-09-15
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

## A mechanism now exists, if TOPO wants it (S-TINT orchestrator, 2026-09-15)

S-TINT probed whether a header roster can be welded to the rows it
describes rather than checked against them, and built one: a `roster!`
macro in `crates/test-utils/` whose each entry is an **ident**, feeding
three consumers at once — `let _: fn() = $name;` so a retired or
misspelt name is a **compile error**, `stringify!($name)` so the
compared string cannot be mistyped, and the printed roster a human
reads. The comparison is against libtest's own `--list` via a
`current_exe()` re-exec, so **no Rust is parsed** and it is not an
instance of `work/tint/source-scanning-censuses-are-a-tripwire-on-ordinary-rust`.
Measured at ~3.6 ms per check on a 10 MB, 42-row binary; the tree
already re-execs its own test binary at eighteen sites, all of which
*run* a child row rather than merely listing one.

**Verified to work in this file's exact shape**: the probe ran it
against a `#[cfg(test)] mod` inside `src/`, which is what
`crates/topo/src/review_d18_probes.rs` is, and it behaves identically to
the `tests/` case.

**This is a pointer, not a request, and nothing is scheduled on TOPO's
behalf.** `crates/topo/src/` is TOPO's ground; adopting the mechanism is
TOPO's edit and TOPO's call, and the cheap alternative — one corrected
sentence — remains entirely reasonable. S-TINT is landing the macro for
its own row; if it lands, this row can close behind a guard instead of a
hand edit, at the cost of the header's enumeration moving from the `//!`
block into a `roster!{}` block near the top of the file.

**What it would NOT fix**, so the trade is visible: the mechanism welds
NAMES and never PROSE. A header sentence that miscounts in words
(*"three of the four rows gate"*) is caught only insofar as the roster
replaces the sentence; a wrong adjective about a row that IS in the
roster is caught by nothing.
