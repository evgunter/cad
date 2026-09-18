---
id: k-probe-eps-const-read-first-match
kind: issue
title: rundump-guard-selftest.sh reads k_probe_sweep.sh's PLAIN_EPS with a first-match grep
status: open
opened: 2026-09-11
refs: [seal-oracle-toolchain-read-first-match]
---

Found by the whole-tree arm of the pin sweep on PR `ciw/pin-residue`, which
swept for the FIRST-MATCH READER shape and not only for version pins.

`scripts/rundump-guard-selftest.sh:43`:

    extract_const() {
      grep -E "^$1=" "$root"/scripts/k_probe_sweep.sh | head -1
    }

and its one caller five lines below:

    if [ -z "$(extract_const PLAIN_EPS)" ]; then
      echo "SELFTEST FAILED: no PLAIN_EPS in scripts/k_probe_sweep.sh" >&2

The extraction is deliberate and the comment above it says why — "a constant
copied here would let the two drift" — so this is the same repair this repo
made for the pin population, one value over. What it carries is the same
residual shape: `^PLAIN_EPS=` matches at column 0 anywhere in the script, and
`head -1` takes whichever comes first in the file rather than whichever is in
scope. Today `k_probe_sweep.sh` sets `PLAIN_EPS` exactly once, so the read is
correct by coincidence of there being one candidate — the state the five
`nightly.yml` sites and `seal-oracle.sh` were both in before they were given
anchored readers.

**The guard beside it is the `test -n` guard `ci-pin.py`'s header dissects**:
`[ -z … ]` catches an ABSENT constant and nothing catches a WRONG one. A
conditional reassignment (`[ -n "$CAD_PLAIN_EPS" ] && PLAIN_EPS=…`) added
later would be picked up or not by position, and the selftest would go on
reporting green against the wrong ε.

**Not urgent, and the fix is small**: refuse on a count other than one, in the
shape `seal-oracle.sh`'s `toolchain_pin` now uses (count the matches, name the
file and the count on stderr, return nonzero) rather than a second general
reader. `scripts/ci-pin.py` does not answer this — it is anchored to a
workflow's `env:` block and deliberately narrow.
