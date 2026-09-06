---
id: outstanding-and-progress-are-two-three-state-enums-one-hop-apart
kind: issue
title: session::Outstanding and frame::Progress are parallel three-state enums sharing two variant names, and progress() is nearly their identity
status: open
opened: 2026-09-06
---



After #2055 the crate carries two three-valued enums, one hop apart,
that share two of their three variant names:

    crates/viewer/src/session.rs:448   enum Outstanding { Current, Evaluating, Canceled }
    crates/viewer/src/frame.rs:1414    enum Progress    { Evaluating, Canceled { indexing: bool }, Indexing }

and the function between them is close to their identity:

    crates/viewer/src/frame.rs:1456-1462
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
(`crates/viewer/README.md:345-347`, *"takes the folded value beside
the index seam's `bool`"*). Under the unit's own rule — *"a consumer
is handed that fact and never the pair"* — the index seam is the one
consumer input that is still a raw reading rather than a value, and
`PickCache::indexing()` (`crates/viewer/src/pickcache.rs:373`) is the
door that would mint one. It is not a swap hazard today because there
is only one `bool` left; it is the asymmetry that makes the rule read
as applied to half the signature.
