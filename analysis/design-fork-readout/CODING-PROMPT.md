# Blinded coding of design-fork rows

You are coding rows of a process log. Each row is one design question
that was weighed by two independent designers, labelled **A** and **B**,
and then decided by the project owner, **Ev**. The labels are random per
row; you do not know, and must not try to infer, who A and B are. Do
not open `docs/DESIGN-FORK-LOG.md`, any `analysis/` branch, or any file
other than the one extract you are given; everything you need is in it.

Each row has: `problem`; `A` and `B` (each designer's FIRST report: the
recommendation, a line of argument, a confidence word, "framing
rejected y/n", "ratified text challenged y/n"); `agree` (whether the
first reports agreed); `reconciliation` (rounds in which the designers
were shown each other's reports and moved); `to_ev` (what the PR put to
Ev); `ev` (Ev's decision); `match` (the logger's own note on which first
recommendation Ev's decision took).

Write a CSV with exactly this header, one line per row in the extract,
in the extract's order, values quoted when they contain commas:

```
key,decided,agree_first,logged_match,coder_match,ev_lean,ev_vs_to_ev,conf_A_word,conf_A,conf_B_word,conf_B,frame_A,frame_B,ratified_A,ratified_B,rounds,movement,extra_designers,notes
```

- `key`: the row's key (e.g. `L27`).
- `decided`: `y` if `ev` records a decision on the design question;
  `n` if it is blank, "—", "n/a", "(not asked)", pending, or withdrawn.
- `agree_first`: on the recommended FINAL STATE in the first reports,
  not the wording: `full`, `partial` (agree on some sub-questions or the
  core, differ on others), or `none`.
- `logged_match`: the `match` cell transcribed into one of `A`, `B`,
  `both`, `neither`, `mixed`, `na`. `both` = both first reports
  recommended what Ev chose; `mixed` = Ev took A on some parts and B on
  others; `neither` = Ev chose something neither first report
  recommended; `na` = undecided or the cell is empty. If the cell
  qualifies itself ("A's first; both after reconciliation"), code what
  it says about the FIRST reports.
- `coder_match`: YOUR OWN judgment, same categories, made by comparing
  Ev's decision (`ev`) against `A` and `B` directly. Do not just copy
  `logged_match`; when you differ from it, say why in `notes`.
- `ev_lean`: `A`, `B`, or `even`: whose first recommendation Ev's
  decision is closer to overall (for `both` or a balanced `mixed` /
  `neither`, `even`); `na` if undecided.
- `ev_vs_to_ev`: `accepted` (Ev approved what the PR carried, possibly
  choosing among options it offered), `amended` (approved with a
  material change), `rejected` (decided against the carried
  recommendation), `na`.
- `conf_X_word`: the confidence word as written (e.g. `likely`,
  `confident`, `high`), or `unstated`. `conf_X`: ordinal `3` (high,
  confident, certain, very likely, strong), `2` (likely, probable,
  moderate), `1` (lean, uncertain, low, tentative, maybe), blank if
  unstated.
- `frame_X`, `ratified_X`: `y`, `n`, or `u` (unstated) — "framing
  rejected" and "ratified text challenged" as the row states them.
- `rounds`: the number of reconciliation rounds (0 if none).
- `movement`: in reconciliation, who moved toward the other's position:
  `A` (A moved toward B), `B`, `both`, `neither` (rounds ran, nobody
  moved), `none` (no reconciliation).
- `extra_designers`: `y` if reconciliation dispatched designers beyond
  the original pair, else `n`.
- `notes`: short; anything ambiguous.

Code every row; never skip. When a cell is genuinely ambiguous, pick
the best code and explain in `notes`. Write the CSV to the output path
you are given and reply with only the row count and a one-line summary
of any systematic ambiguity.
