---
id: anchored-exact-text-skip-has-three-homes
kind: issue
title: The anchored exact-text skip with an abstaining subject check is hand-spelled in three gates and lib.sh has no home for it
status: review
opened: 2026-09-06
refs: [unanchored-definition-skip, D211]
branch: gates/anchored-skip
pr: 2064
---


## Finding

Filed by the GATES orchestrator from PR 2029's style review (Q1). Three
gates now spell the same mechanism by hand — "an exact-text skip
anchored at its home path, with a subject check that reds when the
home file is present but the text is not, and abstains when the file is
gone":

- `scripts/gates/bounds-allowlist.sh` — `DEFINITION_HOME`,
  `DEFINITION_HOME_RE`, the plain texts and their hand-escaped `_RE`
  twins, `gate_definition_skip_subject` with `[ -f … ] || return 0`,
  `gate_definition_skip` filtering `^$HOME_RE:[0-9]+:$DECL_RE$` (PR
  2029, copied from the twin below and said so).
- `scripts/gates/no-extra-real-bounds.sh` — `SEALED_HOME_RE`,
  `gate_sealed_skip_subject`, the filter at `:133`,
  `plant_sealed_home_clean` and `plant_sealed_decl_elsewhere`.
- `scripts/gates/viewer-module-kinds.sh:485` — the same anchored skip
  with a stronger subject check (a missing exception file is a red at
  `:458`, not an abstention).

The row `unanchored-definition-skip` was the cost of the first copy
drifting from the second; the PR that closed it copied the shape by
hand and the review found the copy missing one of the twin's fixtures
(the at-home passes-case). A `lib.sh` helper — `gate_exact_skip HOME
TEXT…` building the anchored filter, the subject check and both
planted cases from the plain text once (so no hand-escaped `_RE`
twin can drift) — is the fix shape; the three gates call it.
Sequenced after PR 2029 lands.

**What was not measured**: whether `viewer-module-kinds.sh`'s
stronger subject check is the rule all three want (a missing home as a
red rather than an abstention), which is decided when the helper is
written.

## Landed

`lib.sh` grows the mechanism — `gate_exact_skip` (declare),
`gate_exact_skip_subject`, `gate_exact_skip_filter`, `gate_ere_escape`,
`gate_record_anchor`, four planters and `gate_exact_skip_selftest` —
and the pattern is DERIVED from the plain text: the record shape comes
from `gate_rust_code` itself (the text is handed to the reader as a
file of its own, in the view the gate declares), and the escaping from
`gate_ere_escape`, so there is no hand-escaped `_RE` twin left in the
directory to drift. `bounds-allowlist.sh` and `no-extra-real-bounds.sh`
declare their skip and call the three; their `*_HOME_RE`/`*_DECL_RE`
twins, `gate_definition_skip`, `gate_definition_skip_subject` and
`gate_sealed_skip_subject` are gone.

**The subject-check rule, as landed: a missing home is a RED.** A skip
whose home is gone exempts nothing today and is a ratification the next
file written at that path inherits without argument — the D103 class
this directory already reds on twice (`viewer-module-kinds.sh` on an
exception count with nothing behind it, `bounds-allowlist.sh`'s own
census on a roster entry whose file is not in the tree) — and `lib.sh`
already answers the same question for a gate's subject at
`gate_require_file`. The abstention's own defence covered one case of
three: it is argued from the anchor making a MOVED home loud, and says
nothing about a home DELETED with its text or a home that was never at
that path. The rule is not a parameter and no gate differs; it costs
each caller one line in its clean fixture (`gate_plant_clean` now
plants the skip's home), which makes the skip live in every fixture.

Not an [ev] question: the two rules were not both load-bearing.

`viewer-module-kinds.sh` is NOT converted, and the row's premise is
narrowed by measurement: its exception is `(file, needle, count)` where
the needle is a PATTERN matched anywhere in the record and the
exemption is granted only at an exact site count (both directions red).
That is a different mechanism from an exact-text skip, and calling
`gate_exact_skip_filter` would drop the site-granularity the gate
argues for at `README`'s cited section. What it shares is the anchor,
and it takes that: `gate_record_anchor` replaces its three hand-spelled
`^$SRC/$exfile:[0-9]+:` interpolations and escapes the path, which the
hand-spelled form did not (`forms.rs` as a pattern also matches
`formsXrs`). Its subject check already reds on a missing home, so the
divergence this row names is resolved in its favour.

What did NOT land: nothing else was deferred inside the fence.
