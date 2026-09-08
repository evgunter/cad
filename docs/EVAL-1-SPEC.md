# EVAL-1 — the affine lift has one home: `embed_affine` retires into `Affine3::map`, and the fallible walk is put to PROPS (spec)

**Program:** EVAL (`work/eval/plan.md`, unit 1). **Items, one unit:**
`work/eval/affine-lift-has-a-second-home-in-anchor-embed-affine.md` and
`work/eval/D368.md`.
**Track:** E — the CIW/CHROME posture (`work/eval/plan.md` §Review posture):
one implementer lane, one style review against
`docs/prompts/reviewer-style-lane.md` with explicit claims to falsify, a fix
pass, record-at-merge. No A/B draw; no correctness arm (the unit moves no
value a document evaluates to — see §Bit-identity, which is the claim that
says so and is checked, not assumed).
**Branch:** `eval/1-affine-lift`. **Difficulty:** S.

## The claim

**The per-coordinate walk over an `Affine3` has one home, the kernel's
`Affine3::map` (`crates/geom-core/src/linalg/affine.rs:47`), and
`editor-core` does not carry a second one for the infallible direction.**
Today it does:

- `crates/editor-core/src/eval/anchor.rs:248` `map_affine` — the twelve
  components through one fallible `f: Fn(A) -> Result<B, E>`, columns and
  translation kept in place; the `Vec3` walk inside it (`:252`, the closure
  `v`) is the hand-lifted `Vec3` `D368` names, now living here rather than at
  the `:238` the row cites.
- `anchor.rs:268` `embed_affine` — `map_affine` with `T::from_f64` and an
  `Infallible` error: `Affine3::map(T::from_f64)` by another name. Its doc
  calls itself "ONE HOME for a per-coordinate `from_f64` walk that had grown
  three", which was true of `editor-core` and is false of the workspace since
  PR 1977 gave `SketchPlane::map` (`crates/profile/src/lib.rs:486`) the same
  walk over the type that carries a frame.

Callers on main today (line numbers as of `063bf1dc8`):

| site | direction | becomes |
|---|---|---|
| `eval/wire.rs:1226` (`ProfileLift::Pinned`) — `SketchPlane::new(anchor::embed_affine::<T>(&placement.placement))` | `f64 → T` | `placement.map(T::from_f64)` — `placement` is already a `SketchPlane<f64>`; this is the non-tour consumer PR 1977's door was minted for |
| `eval/anchor.rs:281` (`embed_profile`) — `embed_affine::<T>(&p.plane.placement)` then `SketchPlane::new(placement)` | `f64 → T` | `p.plane.map(T::from_f64)` |
| `eval/wire.rs:878` (`pinned_plane`) — `map_affine(&plane.placement, \|x\| x.pinned_f64().ok_or(()))` | `T → f64`, fallible | **stays on `map_affine`** this unit (below) |

## What lands

1. **`embed_affine` is deleted.** Both callers go through `SketchPlane::map`.
   Grep-prove the identifier absent from the tree at the merge base and again
   at the merge.
2. **`map_affine` stays, private, with one caller**, and its doc is rewritten
   to state the invariant rather than the history: it is the fallible
   direction of a walk whose infallible direction is `Affine3::map`, kept
   here only until the kernel offers a fallible one. No "had grown three", no
   retired-payload archaeology (`docs/prompts/implementer-discipline.md` §4).
3. **The `try_map` question is put to PROPS by note, not decided here.**
   `anchor.rs`'s own doc argues the walk is written once because a transposed
   `c1`/`c2` is invisible in review; that is an argument for the kernel owning
   the fallible walk too (`Affine3::try_map<U, E>(self, f: impl Fn(T) ->
   Result<U, E>) -> Result<Affine3<U>, E>`, with `map` its infallible
   specialisation). The geom-core vector doors are PROPS' (`work/eval/
   program.md` keep_out); the lane writes the note in its PR body and the
   orchestrator carries it to PROPS' board. When PROPS mints `try_map`,
   `map_affine` goes in the same PR that adopts it — file that as a
   `parked` row on EVAL's slate in this unit, `blocked_on` the PROPS item,
   at the moment the note is written (`work/README.md`: disclosing a residue
   is not scheduling it).
4. **`D368` closes by construction**: the hand-lifted `Vec3` the row names is
   the `v` closure inside `map_affine`, and the `from_f64` instance of it is
   what `embed_affine` was. Say so in `D368`'s `## Closed` section, with the
   line the row cited and where the walk actually was at close.

## Bit-identity

`Affine3::map` and `map_affine` visit the same twelve components through the
same `f`; `T::from_f64` on each is one call either way. **Claim: every
evaluated document is bit-identical before and after** — the corpus goldens,
the verdict-log goldens and the tour frames do not move. The lane states this
as a claim and the review falsifies it: if any golden moves, the unit is not
what this spec says it is, and the lane stops and reports rather than
re-baselining (`memories/output-stability-as-justification.md` — identity
may choose between equivalent spellings, and here it is the CHECK that the
two spellings are equivalent, not the justification for keeping either).

## Sweep

The class is "a hand-written per-coordinate lift beside a `map` door". Sweep
`crates/editor-core/src/` for the SHAPE, not the symbol:

```
rg -n 'from_f64\(' crates/editor-core/src --glob '!**/tests/**'
rg -n '::new\(\s*T::from_f64|::new\(\s*S::from_f64' crates/editor-core/src
```

Put the hit list and each hit's disposition in the PR body, one line per hit.
Two hits are known and are **not this unit**, said here so the sweep does not
re-derive them:

- `anchor.rs:291` `embed_profile`'s `Point2::new(T::from_f64(..),
  T::from_f64(..))` per vertex — a profile-vertex lift, which has no `map`
  door on `ProfileVertex`/`ProfileLoop`/`Profile` today; that is `D385`'s
  shape (`work/tcost/D385.md`, "map_scalar, deciding per file") and the door
  is `crates/profile`'s, S-BOOL's glob. Report; do not build the door.
- anything under `crates/editor-core/src/eval/measure.rs`, `analysis.rs`,
  `product.rs` — M10's and contended ground; report only.

State what the pattern cannot match (a lift spelled through a local helper,
or through `Real::from_f64` by a different receiver name).

## Review

One style lane, `docs/prompts/reviewer-style-lane.md` by path. Claims to
falsify, in order:

1. `embed_affine` is absent from the tree, and no caller re-grew a local
   `from_f64` walk over an `Affine3` to replace it (Q1: grep the copy).
2. Bit-identity: build the corpus at the merge base and at the head and diff
   the committed goldens and the verdict-log goldens; any byte that moved is
   a MAJOR.
3. The `try_map` residue has a FILE on EVAL's slate, `parked` on a PROPS
   item that exists, not a sentence in the PR body (Q6).
4. `map_affine`'s rewritten doc states an invariant that is true of the tree
   (Q2, Q5): one caller, fallible direction, kernel owns the other.
5. The sweep's hit list is real: re-run it shaped differently and compare.

Emphasis for this lane: the fix reproducing the defect it closes (a unit that
unifies two homes can mint a third — `SketchPlane::new(placement.map(..))`
where `placement` is already the plane is the shape to look for), and a
disclosed blind spot read as a discharge.

## Records at merge

`work/eval/log.md` entry; both items `closed` with `pr:`; the `parked` row
for `map_affine`; the PROPS note carried by the orchestrator; this spec
deleted per `docs/DOC-LEDGER.md`.
