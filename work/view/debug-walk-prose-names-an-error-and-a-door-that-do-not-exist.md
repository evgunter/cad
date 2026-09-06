---
id: debug-walk-prose-names-an-error-and-a-door-that-do-not-exist
kind: issue
title: the new Debug prose calls Derived::none an unbound-pattern error when it is E0063, and says resolver is dumped by asking the session when no door hands it back
status: open
opened: 2026-09-06
refs: [2093]
---



Found by the style review of #2093. Both sentences below are load-
bearing: they are the argument for the mechanism and for one of its
five omissions, and each is checkable and wrong.

## `Derived::none` does not raise the error the walks say it does

`crates/viewer/src/session.rs:302-306`:

> A field added to [`Derived`] is an unbound-pattern error here,
> exactly as it is in [`Derived::none`]

and `crates/viewer/README.md:432-435`:

> a new field is an unbound-pattern error in the rendering exactly as
> it is in `Derived::none`

Adding a witness field to `Derived` and reading the compiler gives:

    session.rs:318:13: error[E0027]: pattern does not mention field ...
    session.rs:294:9:  error[E0063]: missing field ... in initializer
                                     of `session::Derived`

`Derived::none` (`session.rs:291-299`) is a struct **literal**, so its
error is E0063, missing-field-in-initializer — not an unbound pattern.
The same file's older prose gets this right without naming a code
(`README.md:373-377`: *"`Derived::none` stops compiling until someone
writes its cleared value"*), which is what makes the new sentence's
precision the thing that broke it. The property being claimed does
hold at both sites; only the name of the error is wrong, in the two
places a reader would go to learn what the mechanism is.

## `resolver` is not dumped by asking the session

`session.rs:1947-1951` and `README.md:441-444` justify the five `_`
arms:

> `tol`, `display` and `resolver` are values the session OWNS rather
> than knows — each with its own `Debug`, dumped by asking it, not by
> inlining it here.

`DocSession::tol` (`session.rs:584`) and `DocSession::display`
(`:725`) are those doors. **There is no door for `resolver`.** The
only accessor is `DocSession::resolve_dir` (`session.rs:736-738`),
which hands back `Option<&Path>` — the resolver's directory, not the
resolver. `Option<Arc<DirResolver>>` cannot be asked for from outside
`session.rs`, so the sentence covers two of its three subjects and the
third is simply dropped.

`DirResolver` does derive `Debug` (`crates/viewer/src/docio.rs:58`),
so nothing here is a compile problem — the reason given for the
omission is just not a reason that applies to it.
