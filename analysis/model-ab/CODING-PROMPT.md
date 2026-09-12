# The blinded-coding dispatch prompt

`blind_reviews.py` writes the material and the blank forms; the material's
own header carries the **instrument** (what makes a finding tally). This
file is the other half: the **dispatch prompt** handed to the coder agent.
It was improvised per-run through the third and fourth readouts, which is
how the fourth readout came to be coded unblinded — writing it down is the
fix.

Regenerate material first:

```sh
python3 blind_reviews.py --src ../../docs/MODEL-AB-LOG.md
# optionally --since / --until to code one era at a time
```

Then dispatch one coder per era (or one overall, if the eras are being
pooled deliberately). Substitute the bracketed parts.

---

## Prompt

> Work in `/home/evan/projects/cad/.claude/worktrees/ab-bayes-analysis/analysis/model-ab`.
>
> **READ ONLY `blinded-reviews.md`.** You are the coder in a blinded pass.
> Do **not** open `docs/MODEL-AB-LOG.md`, anything under `keys/`, or any
> other analysis output — each dissolves the blind. If a model name
> (opus / fable / claude-*) appears anywhere in the material, **stop and
> report it**: that is a leak, and the run is void until it is fixed.
>
> The material's header states the coding instrument. It is normative;
> follow it exactly rather than any recollection of how earlier passes
> worked.
>
> Your task: code every MAJOR finding in [pairs / the pairs dated X..Y]
> into the two blank forms beside the material —
> `labels/v6-correspondence-BLANK.csv` and
> `labels/v6-unilateral-adjudication-BLANK.csv`. Copy each to a name
> without `-BLANK` and fill it in. `raiser` takes `A` or `B`, never a
> slot label and never a model.
>
> Three things the instrument depends on, restated because they are where
> coding passes go wrong:
>
> 1. **`unilateral` means the counterpart never mentioned the issue at
>    ANY severity** — not as a MINOR, not as a NOTE. A severity
>    disagreement about a shared finding is `severity_split`, which is a
>    different phenomenon and must not be counted as detection.
> 2. **Use `unclear` freely.** A thin row that does not support a
>    confident call is `unclear`, not a guess. An inflated unilateral
>    count is worse than a sparse one, because the whole readout turns on
>    that number.
> 3. **The rows carry the orchestrators' own verdicts. They are data to
>    audit, not answers to inherit.** Where your reading departs from a
>    row's declared verdict, code what you find and say so in the
>    evidence column — that disagreement is a result, not an error.
>
> Be even-handed. This pass exists to test whether an asymmetry between
> two reviewers is real; a result showing over-flagging, or no asymmetry,
> is exactly as valuable as one showing detection. Do not let the framing
> push you toward validating either side.
>
> Report back: findings coded; the counts per `correspondence` category;
> the unilateral count split by raiser (A vs B — you will not know which
> model either is, and should not try to infer it); how many you marked
> `unclear` and why; any pair you could not code and the reason; and
> **your own model identity**, which the readout records per protocol v6
> item 4.

---

## After coding

`unblind_adjudication.py` joins the coded forms back against the withheld
key. The coder never runs it; whoever holds the key does, and the readout
reports the coder's model alongside the result.
