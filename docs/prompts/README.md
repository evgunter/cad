# `docs/prompts/` — text handed to agents, not prose for humans

Everything in this directory is given to a subagent **by path**, verbatim and
binding. It is not background reading and it is not a design document.

**The rule: point, never paste.** A dispatch says "read
`docs/prompts/<file>.md` in full before you start"; it does not reproduce the
text. Pasting makes one instrument into N hand-synced copies with nothing
detecting the drift, and it makes "which version did this agent actually run?"
unanswerable, because the answer lives in a transcript instead of a commit.

That failure is observed, not hypothetical: the repo-wide implementer
discipline block was embedded here and pasted per dispatch, which produced six
subtly different review instruments in a single wave.

**Because a pointer can be ignored where a paste cannot, every prompt here owes
a read-verification** — something the agent's report must contain that it could
not produce without having read the file. The reviewer lane requires the report
to name the questions it exercised and to carry the confidence vocabulary. A
prompt without such a hook is a request, not an instrument.

| file | given to | dispatcher notes |
|---|---|---|
| `reviewer-style-lane.md` | reviewers, alongside the claims to falsify | `docs/REVIEW-STYLE-DISPATCH.md` |
| `implementer-discipline.md` | every implementer lane | `memories/orchestration-model.md` |

**Scope: repo-general prompts only.** A program whose specs carry their own
standing brief lines keeps them there — `docs/LIB-PYG1-SPEC.md` §0 and the
ASM/TESS/MESH `## Standing brief lines` chains are tied to those programs'
sequencing and are deliberately not folded in here.
