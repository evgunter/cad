# PCURVE — exit walk

STATUS: **PROPOSED — awaiting Evan's ratification.** A program is closed
when its exit walk is ratified; that walk is then its done-state of
record. Nothing here is a claim about work not yet merged.

The plan's criterion rows are quoted VERBATIM from
`docs/PCURVE-PLAN.md`'s "Exit shape (proposed)" and answered one at a
time. Honesty rows follow — the things a reader would be misled by if
they were left out.

## Criterion rows

**1. "`EdgeGeometry` has ONE conventional form"** — MET. `EdgeGeometry`
is gone entirely: 163 sites in `src/`, 152 across 53 test files, 16
multi-variant match groups, 22 deref sites paid by borrow. The
collapsed vocabulary is `EdgeDescription` with one
`Chart(ChartCurve{surface, pcurve, seam})` arm plus a fenced
`Scaffold`. Verified by grep at zero occurrences and by a reviewer's
independent sweep. **Deviation, argued and accepted**: a second type
`EdgeDescriptionSpec` exists, because forcing acceptance's literal "one
type" would move the mint out of the door and relocate
`ChartImageUnavailable` past the interval checks — a verdict change on
every degenerate-span row. The collapsed TAXONOMY (acceptance's
substance) holds; the literal count does not.

**2. "the exact classes survive as certification lanes with `General`
as the honest floor"** — MET. `Pcurve::General` reaches the Fitted
grade through `certify_general`, and both #498 sub-classes certify
through it unmodified. The exact classes still win where they apply:
the closed-form schedule runs first, and P-2 measured that a partial
column — which cannot take the exact class — takes `General` and
certifies at envelope `3.36e-14 m` at all three ε.

**3. "`MappedCurve` is an authority record read by tier 3's
prefer-intrinsic rules, retained as a description only for transient
scaffolding"** — MET, with the fence criterion CORRECTED before
implementation. U2 ratified "pre-body"; measurement showed
`MappedCurve` reaches REST through `describe_minted_edges` and six
fillet strut sites, so pre-body never fenced it. Evan ratified
**TRANSIENCE** as the criterion. `ValidationError::ScaffoldAtRest`
enforces it, and `EdgeAuthority::{Derived, Declared}` is stored per
edge and **metered** — `CertCheck::MappedSource` on the declared
payload, added at P-1b's fix pass after a reviewer demonstrated a
declaration placed hundreds of units off the body certifying clean.

**4. "adoption still reproduces native descriptions bitwise and D9
bit-replay holds"** — MET, and untouched by construction: `adopt.rs`
and the replay row are outside every file the migration edited, which
was demonstrated rather than asserted.

**5. "#498 executes its own retirement text"** — **PARTIALLY MET, and
this is the walk's main honesty row.** #498 has two sub-classes and
they had entirely different costs:
- **Interior isos: DONE.** An interior-column `Intersection` edge mints
  a certified `General` cache at an interior knot the fixed
  four-candidate schedule could never reach.
- **Diagonal loci: NOT THIS PROGRAM'S, and the plan was wrong about
  why.** The plan said these "carry a typed permanent refusal". They
  never reach the deriver at all — edge certification refuses them
  first at `hull_sup ~= 2.6e-4 m`, five to six orders past every ε in
  the corpus, because of `PXN_IMAGE_DEGREE = 1` in another crate at
  another lifecycle stage. Split out; the blocker is #264's.
- **The body does not validate at rest**, and not because of the
  `Intersection` edge: a rim (`16v1`) needs a de Boor collapse
  extractor the kernel deliberately banked (#1195), and has no fitted
  fallback because a `Chart` description names one surface where the
  fitted grade needs an operand pair.

**6. "every new row ε-row three-outcome honest"** — MET. Every new row
was run at 1e-6, 1e-9 and 1e-12. Two rows were caught passing
VACUOUSLY before they could ship — `m5_pr9_sector2` (its walk selected
on a retired variant: 0 counted against 2 asserted) and
`m8_4_intersection_iso`'s pin (returned early at 1e-12 where the
fixture did not attach). Both are now definite at every draw.

**7. "hosted CI green on every merge"** — MET, and the standard rose
mid-program. Both units merged green; P-1b and P-2 both used a NAMED
configuration (`CI-Config: lane=both`, verified as
`CONFIG_SOURCE=lane:commit-trailer` in the run's own output) rather
than trusting the sampler, using a mechanism (#1136) that landed
because this program's units surfaced the need.

**8. "the walk convention applies at exit"** — this document.

## Honesty rows

**H1. The plan was falsified by its own substrates TWICE, and the specs
by their implementers FOUR TIMES.** P-1's substrate found the stated
binding constraint (adoption's bitwise reproduction) was not the real
one (residual-meter incommensurability). P-2's substrate found the
diagonal refusal unreachable. P-2's implementer corrected its
orchestrator on the consumer-site attribution, a probe artefact, the
number of refusing rims, and the draft-gate false green. **The
orchestrator's specs were wrong three times in one unit** — an
unreachable acceptance criterion, an understated ripple, and a
six-site list whose count was right and attribution wrong — all from
transcribing a substrate's prose into binding instructions instead of
re-deriving it.

**H2. A verification gate this orchestrator designed was structurally
blind.** The f64 bitwise row demanded as the proof of the #1157 fix
could not catch that the fix was partial, because `s*n.z` and `|n.z|`
are IDENTICAL at f64 and differ only under `Interval` — exactly where
the defect lived. A reviewer found it at `n.z = [0,1]`.

**H3. Six defects were found that no one was looking for**, four of
them outside this program's code: the fillet strut secant question
(#1116, re-scoped after its stated cause was measured false), the
coplanar-split defect (#1152, pre-existing, byte-identical on main),
the `orthonormal_basis` poison (#1157, `geom-core`, every vertical
plane), the CI lane pin (#1122, now fixed by #1136), the fail-fast
truncation (#1128, now S-QA's), and the draft-gate false green (#1204,
now S-QA's QA-2). The class question behind #1157 is #1143, which M10
owns and which is now governed by ratified DL6.

**H4. `main` was found broken by an innocent branch** — `pncad-py`
would not compile at `f7016118`, and it had shipped unnoticed because
the intervening commits ran tiers that never compiled it. Reported to
S-QA as a live instance of their charter.

**H5. Two units, both A/B-scored, both with a contamination flag.** The
block draw stood on main from P-1a's spec until its redaction
(#1118/#1119), so each unit's implementer arm was derivable by
arithmetic in that window; reviewers were not dispatched in it, but git
history retains the text. Evan ruled the pairs COUNT. Block PCURVE-1 is
CONSUMED (P-2 took slot 4), so its record may merge to main.

**H6. What this program did NOT do.** It did not touch the diagonal
sub-class, the de Boor extractor, `fillet`'s geometry (#1116 was
re-scoped off exactly that misattribution), or the six measurement
consumers that now refuse TYPED on an affected face (#1179). It did not
retire `MappedCurve` — that was never the goal; it became an authority
record.

## Slate disposition

| item | state |
|---|---|
| P-1a | MERGED (#1073) — representation, meter, authority record |
| P-1b | MERGED (#1107) — consumers, transience fence, deletions |
| P-2 | MERGED (#1177) — #498's interior-iso home |
| P-3 | REMOVED before any code; its premise was measured false |
| diagonal half | SPLIT OUT, blocked on #264 |

## Open, named, not this program's

#264 (`PXN_IMAGE_DEGREE`), #1195 (de Boor collapse extractor), #1179
(the six typed measurement refusals), #1152 (coplanar split), #1143
(M10's enclosure contract), #1128/#1204 (S-QA).
