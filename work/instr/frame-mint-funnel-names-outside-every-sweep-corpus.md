---
id: frame-mint-funnel-names-outside-every-sweep-corpus
kind: issue
title: Two frame-mint funnel names, one of them production, reach no k-lint sweep corpus
status: open
opened: 2026-09-15
---

## What

Disclosed by FRAME-WITNESS (`docs/FRAME-WITNESS-SPEC.md`, PR 2675) and
scheduled here at its fix pass, on the review finding that a disclosed
gap owes a schedule.

`geom_core::OrthoFrame`'s mints take the K funnel name as a PARAMETER,
so every layer that mints a frame names its own decisions. `docs/K-REPORT.md`'s
roster-change section tables the names that now reach the funnel, and
two of them reach no corpus `scripts/k_probe_sweep.sh` runs:

- **`sketch_plane_frame_norm`** — `crates/pncad-py/src/py/doc.rs`, the
  Python `SketchPlane.from_frame` door. This is PRODUCTION code on the
  binding's value road, and it now decides TWO lengths per call where
  it decided none before: the authored `u`'s own length, and the
  authored `v`'s residual perpendicular to `u`. Both go through
  `OrthoFrame::gram_schmidt` under `Tol::witness()`'s band. No K sweep
  sees either, so the distribution of a shipped door's margins is
  invisible.
- **`fixture_frame_axis`** — `crates/sweep/src/test_support.rs`, the
  fixtures' frame mint. Reached only if a rostered probe module builds
  a fixture plane off non-world axes; none does today.

`tour_frame_axis` is the contrast that makes the gap visible: the
demo-scenes leg runs the scenes that mint it, so a demo's frame
decisions ARE in the distribution while a binding's are not.

## Why it is INSTR's

The roster and the sweep's corpus are this program's ground
(`docs/K-REPORT.md`'s "inventory method, restated" — the roster is
hand-maintained and nothing mechanical catches an omission). The
question is not whether the binding should decide (it should, and
does); it is whether a name the roster carries is one the sweep can
ever read.

## The shapes a fix could take

1. **Bring the binding into the corpus** — a Probe-lane leg over the
   Python guide's own scenes, or a rostered Rust probe module that
   calls the same door under the same site.
2. **Say in the roster that a name is code-reachable and
   behaviourally unreached**, as the `chart_boundary` paragraph
   already does for its own hole, and keep the sweep's coverage claim
   honest rather than widening it.
3. **Retire `fixture_frame_axis` from the roster** if test-support
   names are not roster material at all — which is a question about
   the roster's membership rule, not about this unit.

## Not this

`work/props/exact-frame-mints-cover-three-of-the-world-frames.md` is a
different friction with an overlapping cast: there the complaint is
that a caller has to INVENT a name for an exact frame that needs no
decision. Here the names are earned and the corpus cannot see them.

## The question for Ev (2026-09-16), as re-framed after his first reply

**The first version of this section asked the wrong question, and his
reply is why.** It asked whether a test-support name is roster
material, and divided the three names on *"reaches the sweep's
corpus?"*. Ev: *"isn't everything that reaches the funnel dependent on
our particular demos and tests?"*

**It is, entirely.** Every sample in the distribution comes from
running our demos and tests — as true of `props_quad_converged` and
every other rostered row as of these three. The corpus is the only
thing that produces margins. So that column cannot be the membership
axis: it divides the corpus, not the names, and makes the roster a
property of what we ran last.

### What does divide them, and it is not corpus-relative

The tree states it at both sites. `crates/pncad-py/src/py/doc.rs`, on
the constant itself: the name is *"the binding's own seat, because the
pair reaches it **from a user's Python call** and not from a datum or
from the evaluation layer."* And `crates/sweep/src/lib.rs`:

```rust
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub mod test_support;
```

| name | who can ask for the decision | in a default build? |
|---|---|---|
| `sketch_plane_frame_norm` | a user's Python call | **yes** — ships on the binding's value road |
| `tour_frame_axis` | the tour's authored planes | no, but the demo goes through the public doors as a user would |
| `fixture_frame_axis` | our test harness only | **no** — gated and `doc(hidden)` |

`fixture_frame_axis` is not a name the corpus happens not to reach. It
is a name **nothing outside a test build can reach at all** — a
different kind of fact from `sketch_plane_frame_norm`, where a shipped
door decides two lengths per call and no sweep watches.

### The question, restated

**What is the roster a record OF?**

- **(a) Decisions the kernel can be asked to make.** The `cfg` gate
  decides: `fixture_frame_axis` leaves, `sketch_plane_frame_norm` stays
  and its missing coverage is the finding, and the roster is a claim
  about the product that does not move when we change what we run.
- **(b) Decisions our corpus did make.** Sampling is the only
  criterion; `fixture_frame_axis` and `sketch_plane_frame_norm` are
  both out today and return when something exercises them — and the
  roster's size becomes a fact about the test suite, which is what Ev's
  question exposes.

**This program recommends (a)**: under (b) the roster is re-derivable
only by running everything, and a reader cannot tell *"we do not decide
this"* from *"we did not run it this week"*. Ev's call.

### The inconsistency this exposes, which is bigger than the one name

`docs/K-REPORT.md` currently practises neither cleanly. The
`chart_bound_*` paragraph — *"the roster's code half reaches them and
its behavioural half does not"* — is **(a)** with a coverage column.
The stated rule in "the inventory method, restated" — in scope *"if it
reaches the funnel from anywhere **the sweep can execute**"* — is
**(b)**. Whichever way the ruling goes, one of those two sentences is
wrong and has to follow it.

### Not blocked

Unit 16 is late in lane C, and the coverage-honesty half for
`sketch_plane_frame_norm` lands under either answer.

## RULED (Ev, 2026-09-16): (a) — the roster records decisions the kernel can be ASKED to make

*"ok, (a) makes sense then!"* — PR #2733, after the re-framing above.

**What the ruling settles, and what it hands unit 16.**

1. **The `cfg` gate decides membership.** `fixture_frame_axis` is minted
   in `crates/sweep/src/test_support.rs`, whose module declaration is
   `#[cfg(any(test, feature = "test-support"))] #[doc(hidden)]` — no
   default build can reach it, so nobody outside our own test harness
   can ask for that decision. **It leaves the roster**, with a line
   saying why, in the shape the row's third option described.
2. **`sketch_plane_frame_norm` stays, and its gap is the finding.** It
   ships on the Python binding's value road and decides two lengths per
   call — the authored `u`'s own length and the authored `v`'s residual
   perpendicular to `u`, both through `OrthoFrame::gram_schmidt` under
   `Tol::witness()`'s band — and no corpus watches. Under (a) that is
   exactly what the roster is for: a decision the kernel can be asked to
   make, with the coverage column saying honestly that nothing samples
   it. The fix is the honest note or a corpus that reaches the binding,
   **not** removing the name.
3. **`tour_frame_axis` is unaffected.** It is rostered and sampled
   either way.

**The ruling's real scope is larger than the two names, and this is the
part unit 16 must not skip.** `docs/K-REPORT.md` states a membership
rule that is now wrong:

> *"A predicate name is in scope if it reaches the `geom_core::k_stats`
> funnel … from anywhere **the sweep can execute**, however it is
> spelled at the call site."*

That is reading (b), which the ruling rejects. Reading (a) is what the
`chart_bound_*` paragraph already practises — *"the roster's code half
reaches them and its behavioural half does not"* — so the document's
two statements now disagree in a way the ruling decides. **Unit 16
rewrites the stated rule to (a)** and keeps the coverage column as the
behavioural half. `docs/K-REPORT.md` is `Reference` in
`docs/DESIGN.md`'s companion table, so that edit is not a second design
conversation; it is this ruling landing.

**Why (a), in one line, so a later reader does not reopen it**: under
(b) the roster is re-derivable only by running everything, and a reader
cannot tell *"we do not decide this"* from *"we did not run it this
week"*.

`needs_ev` cleared. Unit 16 is unblocked and stays where it is in lane C.

## Added 2026-09-20 (MSOLVE-8, PR 2896)

Two more production names of the same shape, both in the mate solve,
which `scripts/k_probe_sweep.sh`'s corpus does not run:
`mate_axes_parallel` (`crates/editor-core/src/mate/coset.rs`, now the
`UnitVec3::new` mint of the levered cross product rather than a bare
`decide`) and `mate_coset_inverse` (`crates/editor-core/src/mate/solve.rs`,
the transported direction's re-mint). The K-REPORT roster-change
paragraph for MSOLVE-8 tables them; two test-owned names
(`fixture_mate_axis`, `pncad_py_test_normal`) follow
`fixture_frame_axis` and reach no corpus by construction.
