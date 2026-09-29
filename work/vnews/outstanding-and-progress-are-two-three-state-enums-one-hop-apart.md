---
id: outstanding-and-progress-are-two-three-state-enums-one-hop-apart
kind: issue
title: session::Outstanding and frame::Progress are parallel three-state enums sharing two variant names, and progress() is nearly their identity
status: closed
opened: 2026-09-06
priority: P1
cost: M
branch: vnews/progress-names-its-own-states
closed: 2026-09-28
---



After #2055 the crate carries two three-valued enums, one hop apart,
that share two of their three variant names:

    crates/viewer/src/session.rs:547   enum Outstanding { Current, Evaluating, Canceled }
    crates/viewer/src/frame.rs:1579    enum Progress    { Evaluating, Canceled { indexing: bool }, Indexing }

and the function between them is close to their identity:

    crates/viewer/src/frame.rs:1616-1623
    Outstanding::Evaluating => Some(Progress::Evaluating),
    Outstanding::Canceled   => Some(Progress::Canceled { indexing }),
    Outstanding::Current if indexing => Some(Progress::Indexing),
    Outstanding::Current    => None,

Two of the four arms are a rename carrying a payload. `frame.rs`
imports `Outstanding` (`frame.rs:178`), so both `Evaluating` and both
`Canceled` are in scope in one file, and the line
`Outstanding::Canceled => Some(Progress::Canceled { indexing })` is
where a reader has to hold the distinction. Nothing warns if the two
drift: adding a fourth `Outstanding` variant makes the `match`
non-exhaustive, but renaming `Progress::Canceled`, or giving
`Outstanding` a payload the chrome then has to re-derive, is silent.

This is the style lane's parallel-role question and it is not a claim
that the two should be one type — `Progress` folds in a second seam
and `Outstanding` deliberately does not know about the pick cache.
What is worth deciding is whether the two should **share a name at
all**: the unit's argument is that a value is safer than a pair
because it cannot be transposed, and two enums with the same variant
spellings in one file's scope reintroduce exactly the confusion at
the reading level that the types removed at the calling level.

Related and one step further: `frame::progress` still takes the index
seam as a bare positional `bool`, which the README ratifies
(`crates/viewer/README.md:349-350`, *"takes the folded value beside
the index seam's `bool`"*). Under the unit's own rule — *"a consumer
is handed that fact and never the pair"* — the index seam is the one
consumer input that is still a raw reading rather than a value, and
`PickCache::indexing()` (`crates/viewer/src/pickcache.rs:413`) is the
door that would mint one. It is not a swap hazard today because there
is only one `bool` left; it is the asymmetry that makes the rule read
as applied to half the signature.

## Adjudicated 2026-09-20 (`vnews/frame-cluster-order`)

### The premise holds; every citation has moved

| the body says | the tree today |
|---|---|
| `session.rs:547` — `enum Outstanding` | `crates/viewer/src/session.rs:602` |
| `frame.rs:1579` — `enum Progress` | `crates/viewer/src/frame.rs:1989` |
| `frame.rs:1616-1623` — the four arms | `crates/viewer/src/frame.rs:2027-2033` |
| `frame.rs:178` — the `Outstanding` import | `crates/viewer/src/frame.rs:210` |
| `README.md:349-350` — the bare-`bool` clause | `crates/viewer/README.md:368` |
| `pickcache.rs:413` — `PickCache::indexing()` | `crates/viewer/src/pickcache.rs:487` |

The substance is unchanged: `Outstanding` is still
`{ Current, Evaluating, Canceled }` with no payload, `Progress` is
still `{ Evaluating, Canceled { indexing: bool }, Indexing }`, both
`Evaluating` and both `Canceled` are in one file's scope, and
`progress` is still four arms of which two are a rename carrying a
payload.

### This row does NOT collapse into `ranked-and-unranked-verdicts-are-one-type`

`work/vnews/plan.md`'s §Order group 3 says the two rows are *"one
conversation"* because *"the second's fork decides what the first
collapses INTO"*. Checked against the tree, **there is no collapse
relation between them, in either direction.** They are disjoint type
families on the two channels `crates/viewer/README.md` distinguishes:

- `Outstanding`/`Progress` answer *what work is outstanding* and feed
  the **badge** channel — the toolbar spinner
  (`crates/viewer/src/frame.rs:1979-1987`, the *"one state, not a badge
  per seam"* argument).
- `StatusUpdate`/the proposed ranked verdict answer *what the line
  says* and feed the **status-line** channel.

No value of either family is convertible to, or a special case of, the
other. What the two rows genuinely share is a SHAPE — two parallel
vocabularies one hop apart, with the same collapse-or-name-the-
difference fork — and a shape is not a dependency. They are
independently decidable, and the only thing that orders them is that
both edit `crates/viewer/src/frame.rs`, which is serialized.

### No Ev gate

`crates/viewer/GUI-DESIGN.md` carries no clause about either enum;
`docs/DESIGN.md:33` calls `crates/viewer/README.md` the implementation
record the program maintains itself, and `README.md:368` is the only
ratifying sentence the row cites. The fork is this program's. The
general finding lives once, in
`work/vnews/rank-one-discards-the-frames-other-news`'s adjudication
section.

## Closed 2026-09-28 (`vnews/progress-names-its-own-states`)

**The names stay shared, and `frame::Progress`'s doc now says why.**
`Progress::Evaluating` holds exactly when `Outstanding::Evaluating`
does and `Progress::Canceled` exactly when `Outstanding::Canceled`
does, so the shared spelling names one fact twice. That is the
opposite of `frame::RankedVerdict`'s case, where a shared `Show` meant
a candidate in one type and the winner in the other and the spelling
was the defect; here a second spelling would be the defect, telling a
reader two states differ where they do not. `Progress` is not
`Outstanding` beside the index seam either: evaluation outranks
indexing, so six pairs are five states. The type is unchanged.

**The index seam's `bool` became a value, `pickcache::IndexSeam`, with
one door, `PickCache::index_seam`, and that fixed a live disagreement
the row did not name.** The `bool` was derived twice: the toolbar
folded `picks.indexing() || fit.busy()` inline in `app.rs`, and
`ViewerBehavior::indexing` read `picks.indexing()` alone. While the
fit that prices a newly opened document's δ runs, `PickCache::sync` is
handed no δ and holds no attempt, so the toolbar spun `indexing…`
while a click in the viewport got `NotIndexed::Absent` — *none is
being built*. Both consumers (`frame::progress`, `pickcache::unindexed`)
now take the `IndexSeam`, which nothing but the door that reads both
records returns, so filling either from the cache's record alone no
longer type-checks. The README sentence the row cited
(`crates/viewer/README.md`, The session's vocabularies) was agent text
(`e574dfa00`), not an Ev ratification, and is re-worded with the
change.

Pinned by `tests/frame_policy.rs`'s progress table (six points) and
`the_index_seam_is_building_through_the_fit_its_first_build_waits_on`,
and by `app.rs`'s `the_toolbar_draws_each_progress_state_once`, which
plants each `Outstanding` and index state in a headless app and reads
the words and recourse the toolbar paints.
