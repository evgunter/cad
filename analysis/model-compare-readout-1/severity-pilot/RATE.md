# Brief: rate the severity of code-review findings

You are a senior reviewer on a Rust B-rep CAD kernel. The project's standards: correctness first ("fail loud": a wrong answer is far worse than a refusal; a valid body that newly refuses is a regression), tests must pin the real contract, and docs/PR bodies must be honest about what changed.

Your packet (`packet.json`, path given below) holds several pull requests ("units"). For each unit you get a neutral description of what the PR does, then a list of findings about it, each with an opaque id. The findings were collected from review and rewritten into one neutral template; their order is random and means nothing. Judge each finding from its facts. Assume the facts as stated are correct.

## For each unit

1. **Grade** every finding on this scale:
   - **MAJOR** — must be fixed (or explicitly ruled on by the project owner) before this PR merges: a wrong result, a regression of something that worked, a false claim in the PR that would mislead a reader about what merged, or a gap that leaves the PR's central claim unverified.
   - **MINOR** — worth fixing in this PR or filing as tracked work, but the merge could proceed: a missing test for a real behaviour, an inaccurate doc, a narrow edge case, a cost not disclosed.
   - **NOTE** — worth knowing; reasonable to take no action in this PR.
   - **STYLE** — code or prose form only (naming, duplication, structure, wording) with no behaviour or claim at stake.
2. **Act?** — `yes` if you would ask the author to spend time on it in this PR (fix it, pin it, or file it), `no` if you would let it go.
3. **Rank** all of the unit's findings from most to least severe, as one complete ordering (ties not allowed).

Grade each finding on its own merits; there is no quota per grade.

## Output

Write `ratings.json` (path given below):

```json
{
  "<unit name>": {
    "grades": {"<id>": "MAJOR|MINOR|NOTE|STYLE", ...},
    "act": {"<id>": "yes|no", ...},
    "rank": ["<id most severe>", ..., "<id least severe>"],
    "one_line": {"<id>": "a reason in at most 15 words", ...}
  }
}
```

Every id of every unit must appear in `grades`, `act`, `rank` and `one_line`. Validate the file parses and is complete (write and run a short check). Read only the packet; do not open other files, the repository or git. Your final message: how many findings you graded per unit, and the count of each grade.
