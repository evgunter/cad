---
id: pick-rename-left-two-live-sites-naming-the-old-file
kind: issue
title: README:672 and tests/pick_windows.rs:297 still name pick.rs after the rename
status: closed
opened: 2026-09-06
refs: [2083]
closed: 2026-09-06
---


Found by the style review of #2083. `crates/viewer/src/pick.rs` no
longer exists; two live sites in VIEW's own territory still name it,
and neither is disclosed in the PR or in
`renamed-module-leaves-citations-in-two-other-programs`.

## The two sites

**`crates/viewer/README.md:672`** — *"`pick ↔ session` is **a
vocabulary and its driver trading a minted value**"*. Four lines above
it, the same paragraph's ring was rewritten by this PR to
`pickcache → pickindex → session → pickcache` and `evalseam ↔ pick` to
`evalseam ↔ pickcache`. The summary sentence naming the second ring was
left. The PR body lists `:660`, `:664` and `:670` among *"seven more
`pick::`/`pick` prose sites. **Fixed**"*, so the sweep reached the
lines either side of this one.

**`crates/viewer/tests/pick_windows.rs:297`** — an assertion message
reading *"a ZERO-length window is not constructible through this door
and is checked at the structure instead (pick.rs's own unit rows)"*.
Those rows are `pickindex.rs`'s `mod tests` and have been since #2079,
so this was already wrong at the merge base; it now also names a file
that does not exist. It is inside a string literal, which is why the
module-path pattern missed it — but the PR's first pattern is *"the
bare paths `pick.rs`/`pickindex.rs`"*, which matches it directly, so
either that pattern was not run over `crates/` or the hit was not
dispositioned. `docs/prompts/implementer-discipline.md` §5 asks for the
hit list and its disposition, one line per hit; the PR gives the
patterns and their blind spots but no tree hit list.

## Why the disclosed blind spots do not cover these

The PR names two blind spots: a multi-line `fn` header, and a citation
naming a subject without naming a file. Neither describes a backticked
bare module name followed by an arrow, or a filename inside a `\`
-continued string literal. The blind-spot statement is what makes a
negative sweep result a receipt, and this one is narrower than the
sweep actually was.

## Where else to look

Any prose or string that spells the module bare rather than as a Rust
path segment: `\`pick\``, `"pick"`, `pick ↔`, `pick →`, `pick's`, and
the same shapes for `pickindex` after the marks split. A case-sensitive
word-boundary grep for `pick` alone over `crates/viewer/**` and
`docs/**`, hand-triaged, is the instrument; the path-segment pattern is
not.

## Confidence

`sure` on both sites. `likely` that a bare-word sweep turns up more in
`docs/`.

## Closed — both fixed, and the sweep the item prescribes was run (2026-09-06, #2083's fix pass)

Both sites are re-pointed:

- `crates/viewer/README.md:672` now reads *"`pickcache ↔ session` is a
  vocabulary and its driver trading a minted value"*, matching the two
  rings rewritten four lines above it;
- `crates/viewer/tests/pick_windows.rs:297`'s assertion message now
  names *"pickindex.rs's own unit rows"*, which is where those rows
  have been since #2079.

**The item's diagnosis of the sweep is accepted.** The first pattern
was disclosed as *"the bare paths `pick.rs`/`pickindex.rs`"* and would
have matched `pick_windows.rs:297` directly; it was run over `work/`
and `docs/` and NOT over `crates/`, and the PR body did not say so.
That is a narrower sweep than the one described, which is the failure
`implementer-discipline` §5 exists to prevent.

**The instrument the item prescribes was then run**: a case-sensitive
word-boundary grep for `pick` over `crates/viewer/**` and `docs/**`,
hand-triaged. It found two more live sites this item did not name,
both now fixed:

- `crates/viewer/src/session.rs:1104` — *"`pick` is a vocabulary and
  may not name this driver"*, in `index_inputs`'s doc;
- `crates/viewer/src/generation.rs:10` — *"it made `evalseam` and
  `pick` import each other"*.

One hit was triaged as NOT the module and left: `input.rs:338`'s
*"`pick` below is where they are read"* names `InputMap::pick`, the
method fourteen lines under it at `:352`. Two more are history and
stay: `scripts/gates/viewer-module-kinds.sh:74` records the two
exception entries the gate USED to hold (`pick.rs|DocSession|2`), both
retired, and `docs/WORK-TRACKS-2026-09.md:341` is a graduation-time
territory sentence describing a file that has since been split three
ways — a shared doc rather than a slate, left for a taker to judge
whether it is worth re-wording at all.

The full hit list and its disposition are published in the PR body,
which is what §5 asks for and what the first pass omitted.
