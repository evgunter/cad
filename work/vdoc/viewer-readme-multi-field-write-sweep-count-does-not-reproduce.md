---
id: viewer-readme-multi-field-write-sweep-count-does-not-reproduce
kind: issue
title: the viewer README's 23-hit multi-field-write sweep reads 24 under a re-take of its own rule
status: open
opened: 2026-09-14
priority: P4
cost: E
---


## The claim

`crates/viewer/README.md`, *What was swept for the writing hat, and
what it could not see*: "Every `fn` under `src/` naming two or more
distinct `self.<field>` assignments, `.clear()`s or `.take()`s, each
hit read against its struct's declaration: **23 hits, and none is a
census**." The same reading is recorded on the closed
`field-censuses-inside-view-survived-the-debug-sweep` (§ *The writing
hat*).

## What a re-take reads

A mechanical re-take of that rule — for each `fn` under
`crates/viewer/src`, the distinct `self.<field>` names appearing in an
`=`, a `.clear()` or a `.take()`, kept when there are two or more —
reads **24** on `origin/main` at `3f7b291b47`, before any change of
this lane's. The 24:

    app.rs sync_scene, set_delta, perform_batch, toolbar_ui, ui
    blend.rs load_all_edges
    bounds.rs observe
    evalseam.rs submit ×2, dispatch ×2, poll ×2, drop ×2
    frame.rs step
    pane/viewport.rs viewport_ui
    pickcache.rs sync, land
    session.rs open, save, new_document, clear_for_new_document,
               request_eval

## Why it is worth a row rather than a silent correction

**The difference is one hit and the instrument is approximate**, so the
honest answer is not "change 23 to 24". Either the README's sweep
excluded a site this re-take includes — `clear_for_new_document` is
called out in the prose as the case the rule matches, so it may have
been counted outside the 23 — or a site has been added since, or this
re-take has one false positive. Nothing in the repo re-takes the
reading, so it cannot tell.

**What the number is load-bearing for** is the sentence after it:
*none is a census*. That claim is about the population, not its size,
and it survives either way — every one of the 24 above is bookkeeping.
So this is a defect in a receipt, not in the code it describes.

## What a taker owes

Re-take the reading with a stated instrument, put the instrument beside
the number the way `crates/viewer/GUI-DESIGN.md`'s doc-link population
does (*"a reading of the tree rather than a property of it"*, with the
command that takes it), and reconcile the two copies — the README and
the closed item's § *The writing hat* — or retire the count and keep
only the claim.

## Found by

The lane that moved the display budget's fit onto its own worker, which
added a third `Drop` to `evalseam` and so had to touch the phrase *the
two `Drop`s in `evalseam`* in the same paragraph. That phrase was
corrected; the count was not, because it could not be verified.

## A third reading, and a population that has since moved

The lane that folded the three threaded seams onto one coalescing
handle (`work/view/the-two-seams-are-hand-maintained-twins`) had to
touch this paragraph again, for the same reason the fit lane did: its
diff deletes the sites the prose names. It re-took the rule with its
own instrument — for each `fn` under `crates/viewer/src`, the distinct
`self.<field>` names in an `=`, a `.clear()` or a `.take()`, kept at two
or more, `fn` extent by brace depth — and read **28** on `origin/main`
at `d71bb6a785` and **21** on its own head.

So there are now three readings of one rule: 23 (the README), 24, and
28, all over trees within a few days of each other. **That settles the
first question this row asks**: the difference is not one hit and it is
not drift, because two instruments disagree by four on the SAME tree —
the enumeration rule as stated does not determine a number, and which
`fn` a `self.<field>` write belongs to is the ambiguity (a closure
inside a `fn`, an `impl` block's `Drop` body, a macro expansion). A
number nobody can re-derive is not a receipt, and the honest repair is
the last one this row offers: **retire the count and keep the claim**,
which is what carries the weight.

The -7 is all `evalseam`: eleven hits there became four, because
`submit`/`dispatch`/`poll`/`drop` × three seams became one
`Coalescing`. The README's example phrase was corrected in the same PR;
the count was not, for this row's original reason.
