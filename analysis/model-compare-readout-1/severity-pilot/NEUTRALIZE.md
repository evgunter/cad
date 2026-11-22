# Brief: rewrite one PR's review findings in a neutral, uniform style

You are given, in one directory:
- `A.md` and `B.md`: two independent code reviews of the same pull request (a Rust CAD kernel), at the same commit.
- `blind-coding.md`: a correspondence coding of those two reviews. Its findings table has one row per UNDERLYING DEFECT (numbered), saying which report raised it and at what label.

Your job is to produce, for every numbered row of the coding's findings table, ONE neutral description of that underlying defect, written so that a later reader can judge how serious it is **from the facts alone**, with no hint of how serious either reviewer thought it was or of who raised it.

## Output

Write a JSON file `neutral.json` in the same directory:

```json
{
  "context": "4-8 sentences: what the PR changes and what it claims, stated neutrally, from the two reports. No findings here.",
  "items": [
    {
      "row": 1,
      "title": "8-14 word neutral title",
      "where": "files / functions / tests involved (short)",
      "what": "2-4 sentences: what is wrong or missing, concretely",
      "evidence": "1-2 sentences: what was shown and how (a probe, a mutant, a test run on base and head, a log, a grep, or reading the code). Keep numbers.",
      "consequence": "1-2 sentences: what it affects and how, in plain terms (a user-visible result, a refused valid body, a misleading doc, a missing test, code form). State scope (how many cases, which inputs) when the reports give it."
    }
  ]
}
```

One item per numbered row, in row order, every row included (also rows coded as style, unlabelled, off-unit or "in passing"). Validate the file parses (`python3 -c "import json;json.load(open('neutral.json'))"`).

## Rules — these matter more than anything else

1. **No severity signal.** Never write MAJOR, MINOR, NOTE, style, blocker, nit, critical, serious, minor, small, trivial, important, "must", "should be fixed before merge", "not blocking", "worth a row", or a reviewer's verdict or recommendation. Drop confidence words (sure / likely / unsure) — if a claim is uncertain in the reports, say what is and is not established as a fact ("the cause was not traced").
2. **No attribution.** Never say which report raised it, whether one or both did, or that reviewers agreed or disagreed. Where both raised it, merge their facts into one description. Where they disagree on a fact, state both readings neutrally ("one measurement found X; another reading of the code suggests Y").
3. **Uniform register.** Plain declarative sentences, about 60–120 words per item in total, no intensifiers ("badly", "seriously", "simply", "just"), no rhetoric, no exclamation. Keep every concrete number, file name and measured count that bears on how much it matters. Do not add facts that are not in the reports.
4. **Do not judge.** You are not rating; do not reorder, merge or drop rows, and do not decide whether a finding is right.

Your final message: the number of items written, and any row you found hard to neutralise (one line each, no severity words).
