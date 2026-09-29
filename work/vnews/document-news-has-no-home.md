---
id: document-news-has-no-home
kind: issue
title: What a tool has to say is one door, but the panes reach it three different ways
status: closed
opened: 2026-09-05
priority: P1
cost: M
branch: vnews/tool-news-is-a-value
closed: 2026-09-28
---


## What this is

`frame::tool_news` is the door for what a tool has to say —
`Subject::Document`, twelve call sites. It is the one door in that
family a type does not pin (its sites render through
`tools::ToolKind::says`, `tools::ToolNotice` and the typed forms
vocabulary, and arrive as text), and the residue is that the sites
reach it three different ways:

- `crates/viewer/src/pane/create.rs` — ten sites, each
  `frame::tool_news(ToolKind::X.says(&error))` or similar.
- `crates/viewer/src/app.rs` — two sites, inside `.map()` closures over
  `tools::ToolNotice` and the seat vocabulary's declined picks.

A door that takes a `String` cannot tell a tool's words from anyone
else's, so nothing stops a future site handing it a camera refusal.

## The shape of an answer

`tools::ToolNotice` and `ToolKind::says`'s output become one typed
value with a `Display` — the shape `frame::Withdrawal` took in the same
unit and the shape `prefs::Notice` already has — and `tool_news` takes
that type instead of a `String`. Then the door is type-pinned like its
six siblings and every site hands over a value rather than text.
(**This sentence said *"the twelve sites"*.** It was twelve when the
row was filed and is fourteen today — the enumeration and the two sites
the row's census never saw are in the adjudication section below. The
number is removed rather than updated, for the reason
`work/vnews/hand-maintained-counts-in-frame-rs-prose-have-no-guard`
gives: nothing reds when the next one lands.)

`ToolKind::says` returning `String` (`crates/viewer/src/tools.rs:118`)
is the thing in the way: it is a wording function over `impl Display`,
so its output has no type of its own.

## Adjudicated 2026-09-20 (`vnews/frame-cluster-order`)

### The population is fourteen, not twelve, and the row's census omits a file

Re-derived by the row's own subject — every production call of
`frame::tool_news` under `crates/viewer/src`:

| file | sites | lines |
|---|---|---|
| `crates/viewer/src/pane/create.rs` | 10 | `:203`, `:208`, `:438`, `:641`, `:645`, `:678`, `:1014`, `:1047`, `:1054`, `:1097` |
| `crates/viewer/src/app.rs` | 2 | `:895`, `:1903` |
| `crates/viewer/src/pane/profile.rs` | **2** | `:110`, `:120` |

**Fourteen.** The row's twelve was correct when it was filed: the
`pane/profile.rs` pair arrived with `49897008d` on **2026-09-19**, a
day before this adjudication and two weeks after this row. Four
further calls are in `crates/viewer/tests/frame_policy.rs` (`:227`,
`:228`, `:622`, `:916`) and are not production sites.

**A stale count in the source follows from the same move.**
`crates/viewer/src/frame.rs:1345` still says *"what [`tool_news`] buys
for its twelve sites by having one door"*. It is one line, it is in
this program's file, and it does not need this row's design work — it
rides the `frame.rs` prose pass rather than waiting here.

### `crates/viewer/src/pane/profile.rs` is claimed by no dispatching program

Changing `tool_news`'s parameter type edits every site above, and
`pane/profile.rs` is one of the eleven `crates/viewer/src` files
`work/view/viewer-src-files-no-successor-claims` names as claimed by no
re-scope successor. VIEW's glob still covers it and VIEW is
`NOT DISPATCHING`.

**This is the same blocker `work/vnews/plan.md` already applies to
`environmental-facts-answer-usable-as-a-bool-with-the-reason-elsewhere`**
(*"it also reaches `platform.rs` and `prefs.rs`, which are in no
program's territory … so it is not dispatchable here until that is
sorted"*). The row is left `open` rather than `parked`, matching that
precedent, and the fact is recorded here so the next dispatch does not
re-derive it. `crates/viewer/src/tools.rs` — where `ToolKind::says`
lives — is claimed here AND by VSEAM (`work/vseam/program.md:11`), and
`crates/viewer/src/app.rs` is VSEAM's: both are announced crossings,
not blockers.

### Other citations re-derived

- `ToolKind::says` is `crates/viewer/src/tools.rs:101`, not `:118`.
- `frame::tool_news` is declared at `crates/viewer/src/frame.rs:1616`.
- `frame::Withdrawal`, the shape the row wants `tool_news` to take, is
  `crates/viewer/src/frame.rs:991` (`superseded`) and its family above.

### No Ev gate

Nothing about the door is in `crates/viewer/GUI-DESIGN.md`; the
vocabulary is stated in `frame.rs`'s doc comments and in
`crates/viewer/README.md`, which `docs/DESIGN.md:33` calls the
implementation record the program maintains itself. The general
finding lives once, in
`work/vnews/rank-one-discards-the-frames-other-news`'s adjudication
section.

## Closed

`frame::tool_news` takes `&tools::PanelRefusal`, a value built only by
`ToolKind::refuses` and `FormKind::refuses` (`crates/viewer/src/tools.rs`),
with a `Display` composed by the same private function `ToolKind::says`
uses, so a panel's name has one spelling. The door answers
`Retold::Again` itself: the value is by type a refusal at a panel's
button, and the same click says it again — the caller no longer
answers it. A `compile_fail,E0308` doctest on `tool_news` shows a
`String` does not build.

**The census had moved again.** At the merge base the population was
eleven, not fourteen: `app.rs`'s two sites had gone to
`frame::tool_notice(&ToolNotice)`, a typed door that reads `Retold`
from the event's arm. That door is also why the row's shape — one type
for `ToolNotice` and `says`' output — was not taken: `ToolNotice`
already has its own door, and merging the two would put arms that
answer `Never` behind a door that answers `Again`. The eleven were
three spellings: `ToolKind::X.says(&error)` (five, `pane/create.rs`),
`format!("<form>: {error}")` or a literal (four, `pane/create.rs`'s
add-datum, add-profile and extrude forms) and the same in
`pane/profile.rs` (two, edit and revert). They are now one:
`frame::tool_news(&<Kind>::<X>.refuses(&why))`, with the forms named
by `tools::FormKind`.

One test site handed the door text that was not a tool's —
`frame_policy.rs`' `a_joined_line_splits_back_into_the_notices_it_was_made_from`
put a `DisplayFault` and an `AdmissionFault` through it — and now
builds those with `Message::new`, which is what it meant.

The sibling left: `frame::startup_notices(&[String])`, filed as
`work/vnews/startup-notices-door-takes-text`.
