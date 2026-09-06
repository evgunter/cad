---
id: S22-row-1
kind: ruling
title: ε ambience — keep the OnceLock, add provenance, and thread ε as a zero-sized witness
status: closed
opened: 2026-08-18
closed: 2026-08-21
refs: [659, 497, 470]
---

## Question

**ε ambience** — *settled 2026-08-19, half of it reversed 2026-08-21*:
keep the `OnceLock`, add provenance (#659), no session object, no
mixed-ε assemblies — all standing. The *no threading* half is
**reversed**: ε is threaded at every call site as a zero-sized `Tol`
witness, which is not the value-parameter design the ruling rejected.
This is `S22`'s first row only — `Band::linear()`'s ambient ε and K;
the row's other halves are closed, and this file exists so the finding
does not read as open.

## Gates

Nothing: the ruling is made and its threading is its own work. The
conditions a taker still honours are in the ruling below — no session
object, no per-model ε, no mixed-ε assemblies, no decision moves, and
the `no-ambient-env` rule's four-part test.

## Ruling

### S22 row 1 DECIDED (2026-08-19): keep the `OnceLock`; ε gets a provenance channel, not a thread

**Ev, 2026-08-19: keep the `OnceLock`. Do not thread ε.** This decides
S22's **first row only** — `Band::linear()`'s ambient ε and K. The other
three rows (the `k_stats` verdict log, `mesh`'s double read + snap,
`same_chart`'s `ptr::eq`) keep their 2026-08-18 verdicts and are
untouched by this.

Two things settle it.

**1. No mixed-ε assemblies.** That removes the only *functional*
motivation for a session object. The one thing the current design
genuinely cannot do is resolve an assembly whose member documents record
different ε — `crates/pncad/src/workspace.rs:433`,
`ResolveFault::EpsilonSeam`. With mixed-ε assemblies ruled out, that
limit is **intended behaviour, not a defect**, and the session design
has nothing left to buy.

**2. Threading is already shipped, twice, and one instance
degenerated.** `crates/profile/src/validate.rs:802` is exactly the
design under consideration — `pub fn validate(&self, tol: Tolerance)`,
band built once at the funnel, `Tolerance` is `Copy`. Measured: **256
call sites in the workspace, and zero pass anything other than
`Tolerance::get()`** — while `profile` still calls `Tolerance::get()`
internally at `path.rs:1292`, `path.rs:1486`, `path/family.rs:242` and
`:310`, so the crate carries *both* mechanisms. It bought a signature
that documents the dependency and no configurability at all. `mesh` is
the second and cleaner instance (one `Tolerance::get()` at
`tessellate.rs:42`, threaded down) and it works because `tessellate` is
a leaf pipeline with one entry point. `topo` is not: ~40
`Band::linear()` sites across 22 files.

Also decisive: **the `OnceLock` is the only thing structurally enforcing
one ε per process.** Threading deletes that enforcement in exchange for
documentation. The postmortem's own defence of the status quo stands
unrebutted — the zero-test-cooperation property is ratified, and it is
the lock that provides it.

**What the finding got right, and what the tree now carries.** The row's
real content was never "ε is ambient"; it was that ε is ambient *and
silent*. A stale `CAD_TOLERANCE_EPS` in a shell changes what
"coincident" means with no output line saying so, which is why **#497**
exists and reads as a mystery. Two things follow:

- **An ε provenance channel** (`crates/geom-core/src/tolerance.rs`).
  `struct Global` gains `EpsilonSource` — *compiled default / env /
  explicit `init` / document* — written by whichever path won the
  `get_or_init`, read back by `Tolerance::eps_source`, and rendered with
  the committed value and any rejected env value by
  `Tolerance::report` / `Tolerance::committed_report`. `pncad::tolerance`
  is the curated door; the demo runs print the line. The distinction the
  channel has to draw is *"an env bootstrap that nothing overrode"* vs
  *"a document stated this"* — `init_document_eps` already outranks env
  by committing first, and the channel now says which one happened.
  `committed_report` is the non-committing door, so reporting cannot
  itself bootstrap ε and turn a later load into a `ToleranceConflict`.
- **The `no-ambient-env` gate's justification**
  (`scripts/gates/no-ambient-env.sh`). Nothing anywhere argued why the
  `NURBS_PROBE` indictment — *"changes shipped behaviour with no
  rebuild, no flag, and no call site to review"* — does not apply
  verbatim to `CAD_TOLERANCE_EPS`, which is what made the allowlist
  entry read as special pleading (and `memories/telemetry-gating.md`,
  where the rule used to live, no longer exists — created by #562,
  deleted in `dd6d1990` / #615). The rule is now stated where it lives:
  an ambient channel escapes the indictment when **(1)** the value is a
  contract-ratified parameter of the model rather than an implementation
  switch, **(2)** it is committed once and immutable, **(3)** the
  committed value and its provenance are *reported*, and **(4)** a more
  authoritative source either wins or refuses. `NURBS_PROBE` had none of
  the four. `CAD_TOLERANCE_EPS` had 1, 2 and 4 and **failed 3** — which
  is exactly what the provenance channel fixes, so the allowlist entry
  is an instance of a stated rule rather than an exemption.

**What this ruling explicitly does NOT do.**

- **No threading.** No `&Tolerance` parameter added to any predicate
  funnel, and `profile`'s and `mesh`'s existing threading is left
  exactly as it is — it is the evidence, not a target. (Whether
  `profile`'s double mechanism should collapse is a separate question
  nobody has asked.)
- **No session/context object.** Its only functional payoff was mixed-ε
  assemblies, which are out of scope by decision.
- **No per-model ε.** D4 ¶1 already rejects it and this changes nothing
  there.
- **No decision moves.** The ranking (document ε outranks an unread
  `CAD_TOLERANCE_EPS`), `ToleranceConflict` at load, `env_init_errors`
  and its loud test, and the evaluate-time ε check (`eval/mod.rs:971`)
  all stay bit-for-bit as they were. Provenance is a channel: no kernel
  predicate reads `EpsilonSource`, and the zero-test-cooperation
  property is untouched.
- **#470 is not decided here.** Ev is re-deciding it separately after
  being shown that the issue defers itself.

**The prose obligation this creates** is a separate, non-self-merging
PR: the purity thesis and D4 ¶1 currently let ε read as an
implementation detail, which is what makes this row look like a
contradiction with `DESIGN.md`'s central commitment. What is actually
true and now ratified — **the model is a pure function of (parameter
vector, ε)**, ε being a declared run parameter with a recorded
provenance, one per process by construction, mixed-ε assemblies out of
scope — belongs in `docs/DESIGN.md`, marked `PROPOSED` pending sign-off
exactly as #628 did for the D2 addendum.

### S22 row 1 REVISED (2026-08-21): threaded after all — as a witness, not a value

**Ev, 2026-08-21: thread ε, at every call site.** This reverses the
*"do not thread ε"* half of the 2026-08-19 ruling above and nothing
else. Everything that ruling settled stands untouched: the `OnceLock`
keeps its place and its enforcement job, no session object, no
per-model ε, no mixed-ε assemblies, and the provenance channel it
commissioned (#659) is unaffected — this change gives `EpsilonSource`
no new readers and moves no decision.

**The reversal turns on a design the ruling did not consider.** Both
sides of the 2026-08-19 argument assumed the threaded parameter would
be a `Tolerance` — the *value*. Both of the ruling's decisive objections
are objections to exactly that, and neither reaches a witness:

- *"The `OnceLock` is the only thing structurally enforcing one ε per
  process; threading deletes that enforcement in exchange for
  documentation."* True of a value parameter. A zero-sized `Tol`
  witness carries evidence instead — the value never leaves the
  `OnceLock`, which stays where it is. Nothing is deleted; enforcement
  is added to.
- *"It bought a signature that documents the dependency and no
  configurability at all"* — `profile`'s 256 call sites, every one
  passing `Tolerance::get()`. That reads as a false promise because
  `tol: Tolerance` *looks* like it could carry something else. `Tol` has
  one inhabitant and cannot, so the signature promises precisely what it
  delivers, and "every call site passes the same thing" stops being
  evidence of a bad trade and becomes the type's stated content.

**The objection that survives is churn**, which was real then and is
being paid now: ~80 `Band::linear()` and 17 `Band::angular_at()` sites
in `src`, their callers up to each operation entry, and ~400 test
sites. What makes it affordable is that it is compiler-driven and
mechanical — the 355 functions that already take a `Band` are where
threading stops, since the band is the derived value — and that no
conflicting work is in flight.

**What it buys that neither 2026-08-19 option could.** The
`no-ambient-env` rule gains an enforceable sibling rather than a
documented convention; the central commitment's ε exception is
*deleted* rather than reworded, which is the prose obligation above
discharged at its root instead of patched; `mesh`'s ε inventory — pinned
as a test by #872 and the subject of #884's open D9 question — becomes
structural, since an ε read that is not in a signature stops compiling;
and `profile`'s double mechanism, the open question this row explicitly
left behind, collapses into one.
