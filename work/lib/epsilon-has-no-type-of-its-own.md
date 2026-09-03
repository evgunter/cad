---
id: epsilon-has-no-type-of-its-own
kind: issue
title: Epsilon has no type of its own, so StepOptions and step-import restate Tolerance::init's rule by hand (S4's shape)
status: open
opened: 2026-08-20
github: 741
refs: [732, 881, C13]
---

## From GitHub issue 741

Opened 2026-08-20; 2 comments.

Found by #732's style review, out of that unit's scope. **This is a cross-crate public-API change with a design element — its plan goes to Ev before implementation.** Filed as a takeable Track C row, not a parked issue.

## The duplication

`geom_core::Tolerance::init` already enforces the rule that an ε must be **finite and strictly positive**. Three other places restate it by hand on a bare `f64`:

- `step-export/src/lib.rs` — `StepOptions::uncertainty_m: Option<f64>`, validated at write time into `StepExportError::InvalidUncertainty`.
- `step-import/src/entities.rs` — the same bare `f64`, twice.

Same quantity, same units (meters), same validity rule, three hand-synced copies of the check and three separate error spellings. That is **S4** — *one vocabulary, N hand-synced copies* — and not a missing-newtype complaint about `StepOptions` alone.

## The distinction that survives, and is the design question

`Tolerance` is `{eps, k}` — the **run configuration**. What these sites want is **ε alone**, and that concept has no type. So this is not "use `Tolerance` here"; it is "ε deserves a type and does not have one".

Two open parts, deliberately **not** pre-decided:

1. **Where the ε-alone type lives.** `geom-core`, beside `Tolerance`, is the obvious candidate — but `step-import` and `step-export` are the consumers, and putting a type in the kernel core because two exchange crates want it is a claim about where the concept belongs, not a convenience.
2. **Whether `Tolerance::eps` itself becomes that type**, or merely validates into it. The first makes `Tolerance` `{Eps, k}` and moves the invariant into the field; the second leaves `Tolerance` as it is and gives the exchange crates a shared validating door. These have different blast radii and different stories about who owns the invariant.

## Why this is not a patch

It changes public API in `geom-core`, `step-export` and `step-import` at once, and the answer to (1) and (2) is a placement call. A plan first, then a lane.

## Comments

**2026-09-01** — comment:

Coordination note from the mesh side, so this issue's plan is not drafted around a surprise.

## What landed next door, and why it is not on your ground

Issue 881's remaining half — ε's terminal reads get NAMED operations, so `crates/mesh`'s ε inventory becomes methods the compiler keeps honest — is implemented as a **`mesh`-local newtype**, `mesh::sizing::Eps`, `pub(crate)`. It carries four operations (`separates`, `coincident`, `dominates`, `pad`), holds its band in a private field with no accessor, and is minted in exactly one place: `Eps::at(tol)`, the crate's only read of `Tol::eps()`.

**Nothing in `geom-core` moved.** `Tol` and `Tolerance` are untouched; `Tol::eps()` keeps its signature and its other callers. The choice of a mesh-local type over growing the operations on `Tol` was made deliberately for this issue's sake: `Tol` is this issue's configuration surface, its placement question (1) and its ownership question (2) are open and explicitly yours to answer with Ev, and growing cross-crate API on that ground while the plan is undrafted is coordination neither half needs. What `Eps` names is a mesh-local fact — what mesh's own terminal reads DO — which is a different question from where ε's validated type lives.

Checked before implementing: `work/lib/log.md` lists this issue among the plans "drafted when their turns come" and records it as waiting on LIB drafting, not on Ev. Nothing suggested your half was moving. If that reading is stale, say so and the mesh lane will defer.

## The collapse seam, stated so your plan can price it

The newtype's own doc records this, and it is the whole of the coupling:

> **If #741's surface later carries these same four operations, this newtype collapses onto it**: the callers keep their spellings, `Eps` becomes a re-export or is deleted, and the seam that has to move is `Eps::at` alone.

So if your answer to question (2) is that `Tolerance::eps` becomes a type of its own, the mesh side costs one constructor call and an import — the ~30 call sites do not move, because they are already spelled as named operations rather than as band arithmetic. If your answer is a validating door that leaves `Tolerance` as it is, `Eps` simply stays where it is and wraps whatever that door hands out. **Neither answer is foreclosed by this**, which was the point of keeping it mesh-local.

One input for question (1) that the port produced, offered as evidence rather than as a position: the four operations are worth naming because mesh has SIX terminal reads across four kinds, and the kind was carried by prose at each site. `step-import` and `step-export`'s ε reads are validation of a configured value, not band comparisons — so an ε type that validates and an ε type that compares may genuinely be two concerns, and the answer to (1) can be "both, at different layers" without that being a fudge.

---
_Generated by [Claude Code](https://claude.ai/code)_

**2026-09-01** — comment:

Correction to my note above, found by MESH-4's review: the conclusion stands but the reason I gave for most of it was wrong, and the wrong reason would mislead this issue's plan about where the cost sits.

I wrote that "the ~30 call sites do not move, because they are already spelled as named operations rather than as band arithmetic". That is true of **six** of them — the terminal reads, which are `separates` / `coincident` / `dominates` / `pad` calls. The other **twenty-five are CARRIERS**: typed `Eps` parameters, fields and hand-offs (`walk.rs` 14, `curved.rs` 6, `sizing.rs` 2, `tessellate.rs` 2, `trimmed.rs` 1). They do not move for a different and weaker reason — they name a TYPE, so they survive a collapse only insofar as the type keeps its name or gets a `use` alias, not because of anything about how the reads are spelled.

The practical difference for your plan: if `Eps` collapses onto a `geom-core` ε type, the true cost is one constructor (`Eps::at`), one import line per consuming module, and a rename across those twenty-five carriers if the new type is not aliased to the same name — mechanical and compiler-checked either way, but a rename is not nothing, and my note implied it was already absorbed. The six operation call sites are the part that genuinely does not move.

---
_Generated by [Claude Code](https://claude.ai/code)_

## Home

LIB: the code-quality row that carries the implementation (`C13`, Track U) is held under UV-R5 with the rule "LIB drafts the plan, then Ev signs off; implementation lands here after", and `work/lib/log.md` records the issue as waiting on LIB drafting — so the drafting half is this program's.
