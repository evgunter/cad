# Extraction brief: model-compare rows → JSON

Context: `/home/user/cad` is a Rust CAD kernel repo. It runs a process experiment
comparing two models, **Opus 5.5** and **Fable 5.2**, as IMPLEMENTERS and as
REVIEWERS of code units. Its protocol is `docs/MODEL-COMPARE-PROTOCOL.md`; read it
first (especially rules 2, 3, 6, 8 and 12). One row per unit is in
`docs/model-compare-log/rows/PR-<n>.md`. Rows are free-form markdown with a
YAML-ish frontmatter that is NOT consistent between rows. Read each row IN FULL.

Your job is faithful extraction, not analysis. Do not infer values a row does not
state: write `null` for anything missing, and put the reason in `notes`. Never
estimate tokens or times. Don't edit anything in the repo.

Output: ONE JSON file (a list of objects, one per row) at the path you are given.
Validate it parses with `python3 -c "import json;json.load(open(P))"`.

## Per-row object

```jsonc
{
  "pr": 6854,
  "protocol": "e1c428efbb",          // as written
  "program": "restread",
  "letter": "M",                      // H/M/L/S
  "tier": "single" | "dual",
  "review_mode": "single" | "dual-sequential" | "dual-concurrent",
  "full_review": true|false,          // the row says 'FULL' review (vs a narrower/style review)
  "style_review": true|false,         // the row says the review was a STYLE review

  "implementer": {
    "model": "Opus 5.5",              // exact string as stated; "unknown" if so
    "drawn": true|false,              // was the implementer arm drawn by a byte? (false if "not drawn"/"undrawn")
    "byte": 28 | null,
    "counted": true|false,            // is the implementer half fair/counted for the implementer question? false if the row says excluded / unfair / not drawn / outage / re-dispatch on other model / rule 8 or rule 12
    "exclusion_reason": null | "...",
    "time_ms": 25698540 | null,       // implementation phase wall clock, SUM of all implementation-phase entries (continuations included)
    "time_ms_parts": [25698540, 2342896],
    "tokens_total": 964459 | null,    // SUM of implementation-phase token figures, when given as subagent_tokens / a single total
    "tokens_output": null | int,      // when a breakdown table is given: output tokens (sum over impl phases)
    "cache_read": null | int,
    "cost_usd": null | float,         // sum over impl phases where given
    "tokens_note": "subagent_tokens; continuations as given"
  },

  "fix_pass": {
    "ran": true|false,
    "findings_taken": int|null,       // count of findings/items taken, as best stated
    "files": int|null, "lines_added": int|null, "lines_removed": int|null,
    "time_ms": int|null, "tokens_total": int|null, "tokens_output": int|null, "cost_usd": float|null
  },

  "ci_first_head": "green" | "red-own" | "red-main-only" | "unknown" | "not-run",
  "ci_note": "short",

  "spec_dev_reported": int|null,
  "spec_dev_silent": int|null,

  "rubric": {"idiom": 3, "tests": 4, "docs": 4},   // numeric 1-5; if two reviewers gave separate rubrics, give the row's consolidated one here and the per-review one inside reviews[]
  "rubric_source": "reviewer" | "orchestrator" | "consolidated" | "...",

  "reviews": [
    {
      "slot": "single" | "first" | "second" | "R1" | "R2",
      "model": "Fable 5.2",
      "drawn": true|false,            // was this reviewer's model (or slot order) drawn by a byte
      "byte": 213 | null,
      "counted": true|false,          // fair/counted for the reviewer question (false if excluded under rule 8/12, not drawn, re-dispatched on other model, truncated, etc.)
      "exclusion_reason": null | "...",
      "head": "a97ceb92..." | null,
      "verdict": "APPROVE" | "APPROVE-WITH-FIXES" | "NOT-MERGEABLE-AS-IS" | "REJECT" | null,
      "major": 0, "minor": 4, "note": 3, "style": int|null,   // style = count of style items if stated, else null
      "majors": [ {"text": "one line", "demonstrated": true|false} ],
      "demonstrated_findings": int|null,  // how many of its findings (any severity) are marked DEMONSTRATED
      "taken": int|null, "argued_down": int|null, "duplicate": int|null,  // adjudication counts of THIS review's findings
      "executed": true|false,         // did it execute anything (probes/mutants/tests/battery) vs read only
      "mutants_run": int|null, "mutants_survived": int|null,
      "executed_summary": "short",
      "time_ms": int|null,
      "tokens_total": int|null, "tokens_output": int|null, "cache_read": int|null, "cost_usd": float|null,
      "unique_vs_other": "for second reviews or concurrent pairs: what this review raised that the other did not, one line; else null"
    }
  ],

  "pair": null | {                    // only for dual-concurrent rows
    "bilateral": int|null,
    "unilateral_r1": int|null, "unilateral_r2": int|null,
    "unilateral_r1_major": int|null, "unilateral_r2_major": int|null,
    "severity_divergences": "short",
    "tally_candidates": "short",
    "blind_coder_model": "..." | null
  },

  "dispatch_to_merge_note": "short, as stated",
  "orchestrator_model": "Opus 5.5" | "Fable 5.2" | "unknown",
  "blinding_exposure": "short or null",
  "relaxations": "short or null",
  "later_escapes": ["each appended later-escape line, short: defect, arms, whether review raised it"],
  "notes": "anything unusual that affects counting; be specific"
}
```

If a row covers two reviews of the same slot (e.g. a re-review after the fix
pass by the same arm), list it as an extra element with slot e.g. "first-rereview"
and say so in notes. Units on the orchestrator's read are not in the population;
if a row says a half was not drawn or is excluded, mark counted=false.

Return in your final message: the output path, the number of rows extracted, and
a list of any rows you found ambiguous with one line each.
