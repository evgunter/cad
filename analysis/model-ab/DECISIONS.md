# Analysis decisions log

Questions that came up while building the A/B Bayesian analysis. Evan
stepped away partway through, so from "Q5" onward these were resolved by
my judgment and recorded here rather than asked. Each entry states the
question, the resolution, and the reasoning — so any of them can be
overturned cheaply by re-running with a different setting.

## Ruled by Evan (in chat, before stepping away)

**Q1 — Row scope.** *Which rows form the primary comparison?*
→ **v2 only.** Rows 11 onward (blocked randomization). Rows 1–10
(protocol v1, independent fair coin, landed 7 fable / 3 opus) are
excluded entirely — not even shown as a sensitivity panel. Evan chose
"v2 only" over the alternative of pooling.

**Q2 — MAJOR scoring.** *Raw counts, or severity-recoded?*
→ **Both.** Raw MAJ/MIN/NOTE counts are the pre-registered metric and
carry the headline model; a second model uses a hand-coded severity
classification, with the full coding table published so every call is
auditable.

**Q3 — Wall-clock.** *Active-only, or total including gaps?*
→ **Both, side by side.** Evan's words: "probably 1 but i worry that
it's not actually reliably recorded what was a gap or not, so maybe 2?"
So: model active/impl-phase hours AND total-stated wall separately, and
say explicitly where the two disagree.

**Q4 — No-blinded-lane rows.** *(CI infra rows 16/41/42; the
orchestrator-review and Evan-eyeball rows MONTAGE, MV2, GUARD.)*
→ **Excluded from every quality model, retained for cost models**, and
flagged as such in the report.

**Q4b — Design-axis proxies.** Evan picked "build both proxies" (unit
character coding + fix-pass convergence) and added: "probably delegate a
lot of the labelling to subagents to spare your context."
→ Done, and additionally **the labellers were blinded**: they read a
generated extract (`blinded-rows.md`) with the arm column dropped and
every residual model name redacted to `[MODEL]`, and were instructed not
to open the source log. Verified zero name leaks before dispatch. This
was not requested; it seemed clearly right, since the severity and
character codings are exactly the judgments an unblinded labeller could
bias.

## Ruled by me (Evan away)

**Q5 — Which token figure is "cost"?** The parser found that many rows
give a bare lump sum (`~880k`) rather than a phase breakdown
(`~437k impl + ~467k fix`), and mixing the two would compare an
implementation-only number against an implementation+fix+review number.
→ **Primary = `tok_impl` only** (implementation phase, explicitly
tagged): n = 24 on v2 rows and, by luck, **exactly balanced 12 opus /
12 fable**. A secondary "total recorded spend" outcome sums whatever
phases a row records; it is reported separately and labelled as
conflating phases. I did not impute anything.

**Q6 — Which wall figure?** Same problem. → **Primary = `h_impl`**
(n = 19, 10 fable / 9 opus). **Secondary = `h_any`** (`h_impl` when
present, else the row's single overall figure; n = 26) with the
`contaminated` flag entered as a covariate, which is the "both, side by
side" Evan asked for in Q3. Only 4 v2 rows have a wall figure explicitly
including a crash/outage/limit gap, so the contamination correction is
small and I say so rather than leaning on it.

**Q7 — Difficulty covariate.** The log's own pre-logged S/M/L is the
pre-registered stratifier and is used everywhere. The blinded labeller
also produced an independent `scope_size` (1–5) read from row text
without seeing the S/M/L label; that is used only as a robustness check,
never as the headline. Rationale: substituting my own difficulty scale
for the pre-registered one would be a researcher-degrees-of-freedom
move.

**Q8 — Missing data.** Complete-case per model, never imputed. Every
model reports its own n and which rows dropped out. Rows 36, 38, 40 lack
rubric scores; rows 36, 38 lack silent-deviation counts; most rows lack
cost figures. This is the log's known data debt, and shrinking n is the
honest response to it.

**Q9 — Sampler.** No numpy/scipy/pymc on this machine and no pip, so the
models are fit with a hand-written adaptive random-walk Metropolis
sampler (`mcmc.py`). Every reported model carries split-R-hat and ESS;
anything with R-hat > 1.01 or ESS < 400 is re-run longer or flagged in
the report. For the simplest models I also cross-check against a
closed-form conjugate answer to confirm the sampler is not lying.

**Q11 — What counts as a "consequential" MAJOR?** The blinded severity
coder classified 40 MAJORs into 8 categories and used `unclear` on 9 of
them (mostly rows whose findings cell records a count with no prose).
→ **Consequential = `defect_in_scope`** (a real code defect on the
unit's own acceptance target). Because 9 rows are genuinely
undetermined, the model is fit **twice**: once with `unclear` excluded
(lower bound on the defect rate) and once with `unclear` counted as
`defect_in_scope` (upper bound). Both are reported. If the two bounds
disagree about the sign of the arm effect, the honest answer is that the
recode cannot settle it — and the report says so rather than picking the
flattering bound.

**Q12 — "Silent" is overloaded in the log.** The severity coder caught
that row 20's "silent corrupt STL" means the corruption was undetected
at runtime, not that the implementer hid a deviation. → The protocol's
metric is the *deviation* sense, so the `silent` column in
`core-rows.csv` follows the log's own `silent devs` cell, and the
per-MAJOR `silent` flag from the coder is used only descriptively. Noted
because conflating the two would inflate the metric the protocol weights
worst.

**Q13 — Charts.** No `node` on this box, so the bundled palette validator
could not be run. Rather than eyeball it, `palette_check.py`
reimplements the gate in Python: OKLab ΔE between the two series under
normal vision and simulated protanopia / deuteranopia / tritanopia
(Machado 2009 matrices, severity 1.0), plus WCAG contrast against each
chart surface. The two series (blue = fable, orange = opus) pass every
gate in both light and dark mode with wide margins (worst CVD ΔE 24.7 vs
a target of 8; worst contrast 3.12:1 vs a floor of 3). No plotting
library exists here either, so charts are hand-generated inline SVG.

**Q10 — Priors.** Weakly informative and centred on "no difference":
arm coefficients get Normal(0, 0.8) on the log-rate scale (a 95% prior
range of roughly 0.2x–5x, wide relative to any plausible model gap) and
Normal(0, 2) rating points for Gaussian outcomes. A prior-sensitivity
panel refits the headline with half and double that width. Centring the
prior on zero is deliberate: with n this small the prior is doing real
work, and the honest default is scepticism of a difference, not of its
absence.
