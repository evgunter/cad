---
id: decoration-seam-header-names-no-pin-for-enclose
kind: issue
title: decoration_seam.rs's header says the ssi::enclose crossing is pinned by no row; enclose.rs's own decoration_seam module pins it
status: open
opened: 2026-09-04
track: W
refs: [D289]
---


## Finding

`crates/geom-core/tests/decoration_seam.rs`'s header enumerates the four
`crates/*/src` sites that reach `RingInterval::from_certified` and cannot
be called from that suite, naming for each the row elsewhere that pins it.
The fourth bullet — `geom_brep::ssi::enclose`'s, which **no row named here
pins** — is stale in substance: `crates/geom-brep/src/ssi/enclose.rs`'s
own `tests::decoration_seam` module pins that crossing with three rows,
`every_ring_crossing_refuses_exactly_where_the_decoration_degrades`,
`no_crossing_may_be_rebounded_to_the_bracket_door` and
`a_violated_radius_poisons_the_pad`, all three green under
`--features interval` (measured 2026-09-04). The bullet should name them
as the other three bullets name theirs.

The sentence is not wrong about what THESE rows pin, which is why the
lane that added the pin could leave it standing; what it costs is the
roster's whole purpose — a reader checking whether the crossing is
covered reads the header and stops.

## Was

Disclosed by `D289`'s landing (Track Q), which found the pin already in
the tree and could not correct the header: `crates/*/tests/` is Track W's
fence, and the file is `tcost`'s territory by glob.

## Claimed by S-TCOST (2026-09-04)

Moved from `work/code-quality/` to `work/tcost/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which claimed
**Track W whole** for S-TCOST. Id, `track: W` and body unchanged.

The letter and the program are the same ground: W's fence is
`crates/*/tests/` (all crates) and `crates/test-utils/`, and S-TCOST's
`paths` are `crates/*/tests/*` and `crates/test-utils/*` — an exact
match. `work/code-quality/plan.md` recorded W's claimant as "ground is
`tcost`'s" without the rows ever moving, and no `smell/w-*` lane has
ever run, so the rows were waiting on an owner rather than on a ruling
or a dependency.

**This claim was made on Ev's direction rather than by the S-TCOST
orchestrator**, which is not how `work/README.md` expects a claim to
happen. If S-TCOST would rather not hold a row, moving it back is a
`git mv` and this section is the record of why it arrived.

The two seams `work/code-quality/plan.md` states for W are unchanged
and travel with the rows: `crates/test-utils/src/source.rs` is the
shared home three tracks' rows land in, and the `UNCONVERTED_TODAY`
ceiling is **re-derived from the table at each landing, never lowered
by a row's own member count**; and a W row whose mechanism reaches into
a crate's `src/` is **filed on the owning track** rather than edited
there.

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
