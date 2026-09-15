---
id: startup-notices-join-on-a-mark-a-prefs-notice-contains
kind: issue
title: startup_notices joins the preferences notices with LIST_SEPARATOR and three Notice sentences contain it
status: open
opened: 2026-09-15
refs: [withdrawal-causes-join-on-a-mark-a-fault-may-contain, startup-notices-need-holding-to-badge]
---



Found by the sweep of `withdrawal-causes-join-on-a-mark-a-fault-may-
contain`, whose population was *every place a rendered fault reaches a
joined list*. That item's own defect is latent; **this one is not.**

## The join, and the sentences it joins

`frame::startup_notices` (`crates/viewer/src/frame.rs`) joins the
preferences file's startup notices with `frame::LIST_SEPARATOR`
(`"; "`). It takes `&[String]` and says so in its own doc — *"Not
type-pinned: the notices arrive already rendered, from three sources
with three types"*.

Three of the four `prefs::Notice` arms write a `"; "` inside one
sentence (`crates/viewer/src/prefs.rs`, `impl Display for Notice`):

- `WrongType` — *"preferences: `{key}` should be {expected}; ignored"*
- `UnknownTheme` — *"preferences: no theme called `{name}`; using
  `{}`"*
- `UnknownPreset` — *"preferences: no input preset called `{name}`;
  using the default"*

So a line carrying two notices reads as one item more than it has, and
a line carrying two of these reads as two more.

## Reachable by editing a file, not by a widened error set

`ViewerApp::new` (`crates/viewer/src/app.rs`, the `store.load()` block
feeding `status: frame::startup_notices(&notices)`) collects
`Prefs::from_toml`'s notices and then extends them with
`resolve_theme`'s and `resolve_keys`'s. A preferences file naming a
theme that is not in the registry AND an input preset that is not
gives exactly two, both of them semicolon-carrying, with no error path
involved: the file parses, both names resolve to `None`, and both arms
report. `prefs.rs`'s own module docs describe that as the ordinary
recovery.

## What the withdrawal fix does not buy it

`withdrawal-causes-…` narrowed `Withdrawn.cause` so the type of the
things a `Withdrawal` joins is the guarantee. That shape is not
available here: the elements arrive as `String`, from three types, and
the door's doc records that as a deliberate choice. The fix has to
decide between pinning the door to a type it can make a claim about,
giving this join the two-half treatment `Message::new`/`Message::joined`
gave the notice boundary, and re-wording the three `Notice` arms — the
third being a claim about three sentences rather than about a type, and
`prefs.rs` being where it would land.

`startup-notices-need-holding-to-badge` is the same door's other open
question (the notices are rendered once and dropped) and is not this;
the two would be answered together if the door gets a type.

## Home

VIEW's: `crates/viewer/src/frame.rs`, `crates/viewer/src/prefs.rs`.
