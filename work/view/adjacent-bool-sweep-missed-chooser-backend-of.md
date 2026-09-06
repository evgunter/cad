---
id: adjacent-bool-sweep-missed-chooser-backend-of
kind: issue
title: the two-adjacent-bools sweep on #2055 reports zero other hits and chooser_backend_of is one, in the same file
status: closed
opened: 2026-09-06
closed: 2026-09-06
pr: 2055
branch: view/progress
---



PR #2055 closed `progress-takes-three-positional-bools` and recorded a
sweep receipt: *"Grepped for the shape — a function whose signature
takes two or more adjacent `bool` parameters — across
`crates/viewer/src` … Hits and disposition: `frame::progress` — fixed,
this unit. No other site in the crate takes two adjacent bare
`bool`s."*

That negative result is wrong, and the counterexample is a hundred
lines below the function the unit fixed:

    crates/viewer/src/frame.rs:1551
    pub fn chooser_backend_of(zenity_on_path: bool, session_bus: bool) -> ChooserBackend

It is the same defect in every part, not a resemblance:

- **Two adjacent, differently defined, swappable `bool`s.**
  `zenity_on_path` is "a `zenity` binary sits in some `PATH`
  directory"; `session_bus` is "a D-Bus session-bus address is
  advertised". Nothing relates them.
- **The body is `match (a, b)` on the tuple**, which is the shape the
  old `progress` had (`crates/viewer/src/frame.rs:1552-1557`).
- **A swap type-checks and produces a plausible verdict.** Swapped,
  `(zenity absent, bus present)` returns `ZenityPresent` instead of
  `PortalPossible`, and `(zenity present, bus absent)` returns
  `PortalPossible` instead of `ZenityPresent`. Both are `usable()`, so
  the chrome keeps Open/Save As enabled either way and the only
  visible difference is the one #1097 exists to prevent: which
  readings the app is confident about.
- **The one caller passes them positionally**, from two same-typed
  probe functions declared adjacently:
  `chooser_backend_of(zenity_on_path(), session_bus_hinted())` at
  `crates/viewer/src/frame.rs:1571`.
- **The test row repeats the positional convention**, which is the
  half of the defect the PR describes as the interesting half:
  `crates/viewer/tests/frame_policy.rs:727-742` calls
  `chooser_backend_of(true, false)`, `(true, true)`, `(false, true)`,
  `(false, false)`. A swapped call site and a swapped row would agree
  with each other exactly as `the_chrome_has_one_progress_state…` and
  `app.rs` did.

The finding is the sweep, not only the site. The receipt was offered
as evidence of a class being closed and it recorded a hit list of one
over a shape with two hits in the same file, so the class is open and
the disposition line for the second hit was never written.

## Re-derivation

`rg` cannot see a multi-line signature, so the sweep was re-run by
parsing each `fn` header's parameter list and reporting every adjacent
same-typed pair. Over `crates/viewer/src` the `bool`/`bool` hits are
exactly two: `frame::progress` (fixed by #2055) and
`frame::chooser_backend_of`. A one-line check that finds it without
the parser:

    rg -n 'bool,\s*[a-z_]+: bool' crates/viewer/src

## Outside this program's fence

The same parse over `crates/` finds five more adjacent-`bool`
signatures, reported here rather than filed on other programs'
slates: `crates/sweep/src/blend/naming.rs:122`
(`second_support_is_host(first_planar, second_planar)`),
`crates/topo/src/validate.rs:2867`
(`material_arm_outcome(aligned, opposed, jet_determinate, …,
side_mixed)`), `crates/topo/src/boolean/carrier_eq.rs:374`,
`crates/step-export/tests/orientation_oracle.rs:318`
(`composed_direction(bound_orientation, oriented_edge_flag)` — an
oracle, where a swap that agrees with the code under test is the worst
case), and `crates/sweep/tests/r1_probes_m9_3.rs:106`.

## Closed (2026-09-06)

Both halves taken on #2055.

**The receipt.** Re-run by parsing every `fn` header under
`crates/viewer/src` — the parameter list split at top-level commas,
every adjacent pair with textually identical types reported — which is
the method that sees a multi-line signature. Over `crates/viewer/src`
the `bool`/`bool` adjacencies were exactly the two this item names.
The corrected receipt, its method and its blind spots are in #2055's
body.

**The second instance is fixed rather than scheduled.** `Zenity`
(`OnPath | NotOnPath`) and `SessionBus` (`Advertised | NotAdvertised`)
are named types now, returned by the two probe functions and taken by
`chooser_backend_of`, so the bools are gone from the whole chain and a
transposed call does not type-check; the row at
`crates/viewer/tests/frame_policy.rs` names readings instead of
positions. Two named types rather than the fold `Outstanding` got,
because there is no third party minting these — they are two
independent environment probes and `ChooserBackend` is already the
value that ranks them.

Taken here rather than left standing because the unit then removes the
SHAPE from this crate instead of one instance of it, and because the
site is a hundred lines from the one it fixed: a second lane arriving
later would have re-derived the same argument to change fifteen lines.

`crates/viewer/src/frame.rs` is now free of adjacent same-typed `bool`
parameters, and `crates/viewer/README.md` says so beside the
`Outstanding` rule. The wider class — same-typed adjacencies that are
not `bool` — stays open as
`work/view/adjacent-same-typed-arguments-are-the-same-swap.md`.
