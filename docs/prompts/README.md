# `docs/prompts/` — text handed to agents, not prose for humans

Everything in this directory is given to a subagent **by path**, verbatim and
binding. It is not background reading and it is not a design document.

**The rule: point, never paste.** A dispatch says "read
`docs/prompts/<file>.md` in full before you start"; it does not reproduce the
text.

| file | given to | dispatcher notes |
|---|---|---|
| `reviewer-style-lane.md` | reviewers, alongside the claims to falsify | `docs/REVIEW-STYLE-DISPATCH.md` |
| `implementer-discipline.md` | every implementer lane | `memories/orchestration-model.md` |

**Scope: repo-general prompts only.** A program whose specs carry their own
standing brief lines keeps them there.
