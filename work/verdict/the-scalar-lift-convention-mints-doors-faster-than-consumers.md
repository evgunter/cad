---
id: the-scalar-lift-convention-mints-doors-faster-than-consumers
kind: issue
title: Measured: of the eight map_scalar rungs scalar_lift.rs's convention names, one has a production consumer outside its crate; both top rungs have no non-test consumer anywhere
status: open
opened: 2026-09-12
priority: P1
cost: D
---


## Finding

Filed by WIRE's `placement-prose` lane (PR body names the branch), out of
fence and on PROPS's slate because `crates/geom/src/*` and
`crates/geom-core/src/*` are PROPS's territory. The evidence and the full
per-rung table live on
`work/wire/frame-linear-generic-door-has-no-consumers.md` under
*"Measured (2026-09-12)"* — this row exists so the `geom`-side decision is
scheduled rather than left in a PR body, and so it survives WIRE closing.

`crates/geom/src/scalar_lift.rs`'s module docs state the convention:
*"One name, `map_scalar` on every geometry type and `map` on every leaf;
a reader looking for 'where does this crate lift X' finds it on X."*
That is a real argument for minting a door an in-tree site does not want.
The measurement is what it costs today.

Instrument, run once per rung and reverted: demote `pub` to `pub(crate)`,
`cargo check --workspace --all-targets`, read the `dead_code` warning and
the `E0624` list. An `E0624` naming a `tests/` file is a test-only
consumer; one naming another crate's `src/` is a production consumer.

Of the **eight** `map_scalar` rungs the module names:

- `NurbsSurface::map_scalar` — **the only one** with a production
  consumer outside its crate (`crates/sweep/src/loft.rs:328`).
- `Curve3::map_scalar` and `Surface::map_scalar`, the two **top** rungs,
  the ones the module is named for — `warning: method 'map_scalar' is
  never used` on the lib target, and every `E0624` inside `geom`'s own
  `tests/`. No non-test consumer anywhere.
- `NurbsCurve2`/`NurbsCurve3::map_scalar` — dead *because* `Curve3`'s is:
  its one lib caller is the rung above.
- `SurfaceDescription::map_scalar` and `ApproxSurface::map_scalar` —
  clean at `pub(crate)`: in-crate callers only, nothing outside `geom`
  names them, not even a test. Their `pub` is unearned.
- `Profile::map_scalar` / `ProfileLoop::map_scalar` (`crates/profile`,
  BOOL's ground, already filed on
  `work/wire/frame-linear-generic-door-has-no-consumers.md`) — test-only
  outside their crate.

Of the six leaf `map` doors it names:

- `Vec2::map` — `warning: method 'map' is never used` and **no errors at
  all**: zero consumers workspace-wide, tests included.
- `Mat3::map` — **live**, and outside-the-crate is the only thing the
  instrument measured. `Affine3::map` calls it in `geom-core` itself
  (`linalg/affine.rs`, `self.linear.map(&f)`), and `Affine3::map` is
  live. Its one consumer outside `geom-core` was
  `crates/editor-core/src/placement.rs`'s `Frame::linear`, deleted by PR
  2487; the in-crate one is untouched, and `Frame::affine` still reaches
  it through `Affine3::map`. (This bullet said *"exactly one consumer
  workspace-wide … if `Frame::linear` is deleted, `Mat3::map` has none"*
  until 2026-09-12. That was a level-B result read as a workspace-wide
  claim; see the retraction at the foot of this row.)
- `Vec3::map` — **live**. 15 call sites inside `scalar_lift.rs`'s own
  dead ladder, plus two in `geom-core` that level B could not see:
  `linalg/mat.rs`'s `Mat3::map` and `linalg/affine.rs`'s `Affine3::map`.
  (This bullet said the 15 were **all** of them until 2026-09-12.)
- `Point2::map`, `Point3::map`, `Affine3::map` — live.

## The decision this asks for

Not "delete these". The convention may well be right, and a library that
lifts `Curve3` but not `Surface` would be worse than one that lifts both
ahead of demand. What the measurement blocks is settling a door's fate by
*citing* the convention, because as practised the convention is mostly
unconsumed — seven of eight `map_scalar` rungs and two of six leaf `map`
doors have no production consumer outside their crate. Ratifying "the
convention owes the door" ratifies that, and it should be decided on
purpose.

The counter-position is already written down elsewhere in the tree, at
the same altitude and with the same instrument:
`crates/pncad/src/lib.rs` refuses to re-export `bvh` — *"No demo scene,
no export corpus, and no document-layer path names it, and that
measurement is what decides the re-export. Re-export it the day a
consumer needs it."* Two doors, two opposite rules, neither aware of the
other. That is the thing to resolve.

Citations accurate at `e25946743`.

## Neither rule is ratified — so nothing here waits on Ev

Checked over full history (`git rev-parse --is-shallow-repository` is
`false`), from PR 2475's review and re-run on this branch:

- The convention was written by an **agent in a fix pass**: `b61d25ddc`
  (2026-09-02, *"CERT-N1 fix pass: … one home for the poison argument"*,
  PR 1536), as a module doc-comment on `crates/geom/src/scalar_lift.rs`.
  It was never proposed as a decision.
- `docs/DESIGN.md` does not mention `map_scalar` or scalar lifts **at
  all** — zero occurrences.
- The opposite rule came in with the façade skeleton, `b43bb3e29`
  (2026-08-06, *"pncad: façade crate skeleton — module re-exports,
  prelude, f64-first authoring seam"*), likewise as a comment.

So this is not a ratified decision being re-litigated, and CLAUDE.md's
merge rule does not bite: what waits for Ev is *"text that binds future
work"* — a ratified clause in `DESIGN.md`, or in a README its companion
table lists. Neither site is one. **Two unratified conventions point
opposite ways at the same altitude with neither aware of the other**, and
whoever takes this row decides it as ordinary engineering, ratifying the
answer afterwards if it turns out to be worth binding.


## RETRACTED, and what replaces it (2026-09-12, PR 2487)

**A section here claimed `Mat3::map` was left with zero call sites
anywhere by the deletion of `Frame::linear`. That was false.** It is
retracted in full, and the leaf-`map` bullets above are corrected to
match. `crates/geom-core/src/linalg/affine.rs` — `Affine3::map`'s body —
is `Affine3::from_parts(self.linear.map(&f), self.translation.map(&f))`,
where `self.linear` is a `Mat3<T>` and `self.translation` a `Vec3<T>`.
`Affine3::map`'s own doc comment says it in words: *"the linear part
through [`Mat3::map`], the translation through [`Vec3::map`]"*. Both
leaf doors therefore have an in-crate consumer, and `Affine3::map` is
live. A row on this same slate already said so in passing —
`work/props/affine3-try-map-the-fallible-walk-has-no-kernel-door.md`:
*"`Affine3::map` … descends through `Mat3::map` and `Vec3::map`"* — so
the refutation was on PROPS's own slate before the false claim reached
it.

**The error's shape is the useful part, because it is a property of this
row's instrument.** Every verdict in the source table was taken at
level B — `pub` → `pub(crate)` — which answers *"any consumer outside
this crate"*, because `E0624` is a privacy error and a same-crate
caller does not trip it. Two verdicts were then narrated as claims about
the **workspace**, which does not follow. The refuting evidence was
already in the table: every level-B rung with no in-crate consumer
reports a `warning: … is never used` beside its errors, and `Mat3::map`
and `Vec3::map` are the only two rows that report no warning at all.
**Read a level-B row as a claim about the crate boundary and nothing
wider**, and treat a missing dead-code warning as positive evidence of
an in-crate consumer.

The whole table has been re-read for the same substitution and the
row-by-row dispositions are on
`work/wire/frame-linear-generic-door-has-no-consumers.md` under
*"Correction (2026-09-12, PR 2487's review)"*. Two verdicts changed;
`Vec2::map`'s workspace-wide claim was re-checked against the source
rather than assumed and stands.

**What survives for this row's actual question.** Deleting
`Frame::linear` removed `Mat3::map`'s only consumer OUTSIDE `geom-core`,
so the count in *"The decision this asks for"* — two of six leaf `map`
doors with no production consumer outside their crate — is unchanged,
and now holds of `Mat3::map` for a plainer reason than before. Nothing
in `geom-core` was edited by PR 2487; the disposition of these doors is
this row's call.

## Four more members, minted 2026-09-15 (PROPS' affine-try-map, PR 2743)

The convention now names a THIRD spelling, `try_map`, and the lane that
minted it added two members with no consumer outside the ladder itself
— which is exactly the population this row measures, so they are
appended here rather than left for the next census to rediscover.

The full census at that lane's head, by grep over
`crates/geom-core/src`, `crates/geom/src` and `crates/profile/src`:

- **`map` — six leaves plus two geometry types.** `Point2::map`,
  `Point3::map`, `Vec2::map`, `Vec3::map`, `Mat3::map`, `Affine3::map`;
  `ProfileVertex::map` and `SketchPlane::map`.
- **`try_map` — three leaves plus one geometry type.** `Vec3::try_map`,
  `Mat3::try_map`, `Affine3::try_map`; `SketchPlane::try_map`.

Dispositions of the four new members against this row's question:

- `Affine3::try_map` and `SketchPlane::try_map` have a production
  consumer the day they land: `editor-core`'s `pinned_plane`, which is
  what the deleted private `map_affine` was written for. Consumed, not
  speculative.
- **`Vec3::try_map` and `Mat3::try_map` have no consumer outside the
  ladder itself** — `Affine3::try_map` calls `Mat3::try_map` and
  `Vec3::try_map`, and nothing else calls either. They are the same
  shape as this row's `Vec2::map` and `Mat3::map` bullets, with one
  difference worth recording: they are *structurally* required, because
  the fallible walk descends the same three levels the infallible one
  does and the alternative is `Affine3::try_map` spelling twelve
  components itself — the copy the whole unit exists to remove. So they
  are an in-ladder consumer rather than a convention-only mint, which
  is a distinction this row's instrument (level B, `pub` →
  `pub(crate)`) cannot draw: demoting either would report clean, and
  the answer would still be that the ladder wants them.

That distinction is the thing to carry into the decision: "no consumer
outside the crate" and "no consumer but the rung above" are different
verdicts, and this row's table currently spells them the same way (see
the 2026-09-12 retraction, which is the same instrument limit seen from
the other side).

The convention's own text was corrected at the same time — the
`scalar_lift.rs` module docs now say `try_map` is deliberately partial
and name the four types that have it, instead of implying the third
name is owed on every type. Takeable without Ev: checked again on that
branch, the convention was written by an agent in a fix pass
(`b61d25ddc`) and `docs/DESIGN.md` still has zero occurrences of
`scalar_lift` or `map_scalar`.

## Evidence added (2026-09-24, GATHER's E-row batch, PR 3141)

The two `profile` rungs were re-measured at level B with `--features
interval --keep-going` and are unchanged: both report `never used`, and
every `E0624` is in a test (`profile/tests/` ×12, and one in
`sweep/tests/wire_loft_end_profile_lift.rs`).
`work/gather/frame-linear-generic-door-has-no-consumers.md` now waits on
this row for their disposition.

**The instrument has a feature gap that bears on this row's negative
verdicts.** At default features, `editor-core`'s `AssertionVerdict::map`
read as having zero consumers at level A. It is live behind the
`interval` feature (the leaf replay in `eval/mod.rs`), and PR 3141's
deletion of it was reverted when the interval lane went red. This row's
`never used` verdicts (`Curve3`, `Surface`, `NurbsCurve*` and `Vec2::map`)
were taken without `--features interval`. They should be re-taken with it
before any of them is acted on. Add `--keep-going` as well, because
without it cargo stops at the first failing crate.

## One more member, minted 2026-09-26 (S-DUP, `dup/b6-c`)

`profile::Step::map_scalar` (`crates/profile/src/path/program.rs`) is
new. It is `pub`, and its only consumers are tests: six hand-written
recorded-program embeds in `profile/tests/` and `editor-core/tests/`
now route through it (`work/dup/step-program-embed-has-no-map-door.md`).
Nothing in `src` lifts a `Step`. The two rungs under it,
`Target::map_scalar` and `ArcData::map_scalar`, were minted
`pub(crate)`, because nothing outside the ladder lifts a bare target or
spec. So the new member adds one `pub` door to this row's population,
not three.
