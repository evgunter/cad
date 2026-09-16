---
id: startup-notices-join-on-a-mark-a-prefs-notice-contains
kind: issue
title: startup_notices joins the preferences notices with LIST_SEPARATOR and three Notice sentences contain it
status: closed
opened: 2026-09-15
refs: [withdrawal-causes-join-on-a-mark-a-fault-may-contain, startup-notices-need-holding-to-badge]
closed: 2026-09-16
branch: view/startup-notices
pr: 2710
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

## Closed (`view/startup-notices`, 2026-09-16)

**The startup notices are several notices, not one notice's list** —
so the boundary between them is `NOTICE_SEPARATOR`, and the hold is the
one the outer level already has. `startup_notices` builds one
`Message` per element through `Message::new` and joins them with
`Message::joined`; the door keeps `&[String]`.

Not one of the three the item named, and the reason is that none of
them can hold this level:

- **Pinning the door to a type** gives a closed population to make a
  claim over, which is what `withdrawal-causes-…` bought — but the
  claim is FALSE over this population. Three of `Notice`'s four arms
  write the mark, so the type pin would have to be paired with the
  re-wording, and even then two arms (`UnknownKey`, `WrongType`) echo a
  TOML key out of the user's own file and no type bounds what that
  string holds.
- **A second two-half treatment** needs a mark that is never legitimate
  in band. `withdrawal-causes-…` rejected its own option 2 on that
  ground and the objection is STRONGER here, not weaker: there the
  in-band marks were punctuation four authored sentences were entitled
  to, here the payload is arbitrary user text, so every mark is in
  band. A rewriting door has nowhere to demote to either —
  `Message::new` can rewrite a bullet to a semicolon because there is a
  level below; at the bottom there is not.
- **Re-wording the three arms** is the weakest for the reason the item
  gives, and it does not even reach: it cannot touch the echoed key.

What was actually wrong was the classification. A `Withdrawal`'s causes
are the items a counted preamble introduces; these have no preamble and
nothing counts them, so `LIST_SEPARATOR` was never the right mark for
them.

### What this item claimed, checked

- Three of four `Notice` arms write a `"; "`: **true**
  (`UnknownKey` is the one that does not).
- Reachable with no error path: **true, and reproduced.** A file
  naming a theme and a preset the registries do not hold renders
  ``preferences: no theme called `aurora`; using `dark-neutral`;
  preferences: no input preset called `modal`; using the default`` —
  four `"; "`-delimited pieces for two notices. Red on the base tree at
  `4d3ff671c0`.
- *"the notices arrive already rendered, from three sources with three
  types (`prefs::Notice`, `prefs::PrefsError`, and the theme and preset
  resolutions)"*: **the count is right and the membership is wrong.**
  `resolve_theme` and `resolve_keys` return `Option<Notice>` — the same
  type `from_toml` yields. The unnamed third is `prefs::StoreError`,
  from `store.load()`'s `Err` arm. Corrected at the door and in the
  README.
- *"the two would be answered together if the door gets a type"*
  (`startup-notices-need-holding-to-badge`): **the door did not get a
  type, so they were not.** That row is untouched and still open. The
  bearing is smaller and real: the startup line is now composed exactly
  the way `frame_status` composes one, so holding the notices as a
  `Vec<Message>` and letting the ranking join them is a shorter step
  than it was.
