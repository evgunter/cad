# The style lane — reviewer brief

**Status: the standing second half of every review brief, alongside the
claims to falsify** (`memories/orchestration-model.md`: *"assign reviewers
explicit claims to falsify"*). Ratified by Evan 2026-08-18; recorded as
Protocol v5 in `docs/MODEL-AB-LOG.md`, which is the normative source for
the review protocol.

**Dispatchers: paste §2 and §3 into the review brief verbatim.** §1 and §4
are for you, not for the reviewer.

---

## 1. Why this exists (dispatcher context)

`docs/SMELL-SCAN-2026-08.md` §C established, from twenty structural scans and
seven postmortem passes over the merged history, that this project's reviews
are **exceptionally strong at soundness and structurally blind**. The same
reviews that ran 8000-matrix SVD differentials, re-derived a meters conversion
by hand, and found a certificate excluding true 2π by ~1111 widths produced
**zero** findings on: a mode switch on `is_empty()`, a two-ε signature, a file
holding four quadrature engines, three parallel CDT pipelines, a second surface
enum, or a body-wide accessor in the wrong crate.

The cause is not effort. It is that the protocol is **claims-driven**:
reviewers falsify the claims they are handed, and they do it well. A code-free
module, a 449-line accumulated header, and a duplicate type name across a
façade **assert nothing**, so nothing points a reviewer at them.

**The failure mode to avoid when using this document is turning it into a
checklist.** Reviewers here answer the questions they are given. Ten crisp
yes/no items will produce ten crisp ticks and no judgement. Every question
below is phrased to require taste, and §2's stance exists to make "I'm not
sure, but this looks off" a *complete and welcome* review finding — which the
adversarial-falsification lane, with its high confidence bar, actively
discourages.

---

## 2. The stance (paste this)

You are also reviewing for **things that look off**. This is a different job
from falsifying the claims, with a different standard, and the difference is
deliberate:

- **You do not need to be sure.** "This might be a problem, I couldn't tell"
  is a complete finding. Report it with your confidence attached. The
  falsification lane needs certainty; this lane does not, and a finding you
  suppressed because you couldn't prove it is the most expensive kind of miss.
- **You do not need to propose a fix.** Naming what looks wrong is the whole
  deliverable. Do not spend effort designing an alternative, and do not
  suppress a finding because you can't think of one.
- **A textual justification is not a defence.** This codebase argues for its
  own designs at length and in good faith. Read the code on its merits. If
  something is justified at unusual length, treat that as **mild evidence it
  is worth flagging**, not as evidence it is fine.
- **Your taste is evidence.** "This just isn't how I'd do it" is a legitimate
  finding, even unaccompanied by an argument. Say it plainly and let the fix
  pass adjudicate.
- **Nothing here blocks the merge** unless it independently rises to a MAJOR
  on the correctness lane. Style findings are recorded, not gating — so
  raising one costs the author nothing but attention, and you should raise
  more rather than fewer.

---

## 3. The questions (paste this)

Not a checklist. These are directions to point your attention; the useful
answer to most of them, most of the time, is nothing.

### Q1. Does anything here play an almost-but-not-quite parallel role?

The highest-yield question by a wide margin. Two types, functions, or modules
doing nearly the same job but not quite: near-duplicate logic, two spellings of
one concept, a second version that never fully replaced the first, parallel
enums that have drifted, several tolerance vocabularies, two ways to say the
same error.

**Grep for the copy before concluding there isn't one.** Every duplication
this project has accumulated is *self-declared in prose at the copy site* —
`verbatim`, `re-derived`, `ported from`, `mirror of`, `one dimension down`,
`the twin of` — and nothing in CI, review, or the logs has ever read that
prose. `rg -n 'verbatim|re-derived|ported from|mirror of' crates/*/src` over
the touched area costs seconds.

If you find one, ask **which direction the dependency runs** and whether the
shared thing has a home. A core hosted inside one of its two consumers is the
shape that drifts.

### Q2. Is a comment doing work the code should be doing?

Specifically:

- Is the justification longer and more convoluted than the code it defends?
- Does the comment **postdate** the code it explains? (`git blame` the comment
  and the code separately.) A justification written after the fact is a
  rationalization until shown otherwise.
- Does it assert an invariant nothing enforces — "the caller must", "callers
  should say which in their own docs", "kept in step by hand"?
- Is it **still true**? Comments that instruct other code to rely on them are
  the dangerous ones; see Q4.

### Q3. Can this test fail?

Not "does it pass" — **can it go red**. Two shapes recur:

- **A premise that excludes the failing mode.** A row named
  `..._refuses_typed_even_though_branches_were_found` cannot exercise the
  branch-free case, which is the one that breaks.
- **An assertion monotone in the wrong direction.** `pad > 0` plus containment
  gets *easier* as the enclosure degrades. Every "never silent" or "certified
  enclosure" claim needs a row that goes red when the guarantee **degrades**,
  not merely when it is violated at one chosen fixture.

### Q4. Did this change invalidate a premise something else cites?

When a PR removes a gate, relaxes a precondition, or splits a state that was
previously conflated, **grep for who cited the old premise** — including in
prose. The project's existing invalidation discipline is symbol-scoped: specs
routinely demand "grep-proven absent" for an *identifier*, and no convention
covers a *sentence*.

Two sub-cases, and they need different handling:

- The doc rotted while the code stayed right → fix the sentence.
- **The code drifted from something that was meant to hold** → that is a latent
  defect wearing a documentation costume, and deleting the sentence would
  erase the only record of the intended invariant. Say which one you think it
  is.

Also: if this PR establishes an invariant by fixing a bug, **sweep the sibling
implementations in this same PR**. An invariant discovered by a bugfix
otherwise protects only the code that already knew.

### Q5. What does this promise that it doesn't do?

Read the crate/module doc-comment against what the module actually contains,
not against the diff. Names, doc headers, and `lib.rs` claims are written once
and rarely re-read — and a claim that was true when written is the most common
kind of false one.

### Q6. Is each disclosed deviation an improvement?

Per Protocol v5: if a deviation is *better* than the spec's letter, nothing
further is owed. Anything else — a shortcut, a narrowing, a placeholder, a
fence artifact, a "can move there later" — owes a **concretely scheduled
followup**: an issue
number, or a named unit in a plan. "Recorded as a pickup" and "deferred" are
not schedules. Flag any deviation that is disclosed but unscheduled.

### Q7. Is this how you would have done it?

If not, say so, even if you cannot articulate why and even if the existing
approach works. Awkward control flow, an invariant held by convention where a
type would do, a data structure that makes the natural query hard, a knob that
is never varied, an abstraction with one implementor — all count.

### Q8. Have you looked at the whole file?

Nothing in this project's process ever reads a whole file, namespace, or
crate — specs, diffs, reviews and log rows are all per-unit. Accumulation is
therefore invisible by construction: a 449-line header grew one titled section
per unit, and no single diff was unreasonable. Once per review, open the
largest file you touched and read it end to end.

---

## 4. What to do with the findings (dispatcher)

**Output**: a `## Style` section in the review, separate from MAJOR/MINOR/NOTE.
Each finding: one or more `file:line`, two-to-five sentences, and an explicit
confidence (`sure` / `likely` / `unsure`). No proposed fixes.

**Severity**: style findings do **not** affect the mergeable verdict. If
something also breaks a claim, it belongs on the correctness lane and keeps its
MAJOR/MINOR there.

**The class-not-instance rule.** The project's standing failure is that
findings are recorded as instances and never as classes with a sweep
obligation, so the fifth instance of a defect gets found by the same accident
that found the first. When a style finding is plausibly one of several:

- say so, and name where else you would look;
- the fix pass either sweeps or records why not;
- if the fix pass sweeps only the reported instance, that is a **half-fix** and
  should be labelled one. (Precedent: a reviewer forced one home for
  `tangent_certificate_lane` and the same fix pass shipped two divergent sample
  schedules; a perf scan corrected `face_box`'s stale NURBS premise while the
  identical stale premise for the planar arm sat fifteen lines from the text it
  quoted.)

**Calibration.** Expect findings counts to rise, and the docs column to widen
downward. Per Protocol v5 that is the instrument changing, not implementation
quality. A style lane producing nothing on most PRs is under-calibrated, not
clean — though that expectation is inferred from the scan's hit rate on merged
code, not measured on single diffs, so revisit it after a few rows.

**What this lane must not become.** A second amnesty channel. §C2/§C7 found
that disclosure currently functions as immunity — a disclosed deviation scores
as a *positive* on the "silent devs" column with no counter-metric asking
whether it was acceptable. Q6 exists to close that; do not let a `## Style`
section become the place where known problems go to be recorded and forgotten.
