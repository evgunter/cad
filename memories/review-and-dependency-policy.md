---
name: review-and-dependency-policy
description: Evan's rules for reviews (hands-on e2e demos, not just diff reading) and for adding dependencies (fine to install; ~2-week minimum release age)
metadata:
  type: feedback
---

Two standing rules from Evan (2026-07-15), applying to any agent working
on this project:

**Reviews must include end-to-end exercise, when applicable.** Code
review alone is not enough: reviewer agents should walk through a few
real demos — write and run actual programs against the functionality
under review — to check not only in-scope correctness but that the thing
*solves the right problems* and isn't missing something that matters in
practice.

**Why:** a diff can be locally correct while the API is unusable or
misses a practically important case; only driving it end-to-end surfaces
that.

**How to apply:** reviewer prompts should require writing/running small
realistic usage programs (not just the crate's own tests) and reporting
what the exercise revealed about scope/ergonomics, alongside the ranked
findings. For pure-scaffolding diffs with no runtime surface, running
the toolchain/CI commands is the e2e equivalent.

**Dependencies: install freely, with supply-chain sanity.** Installing
tools/crates as needed is fine, as long as it isn't genuinely risky
supply-chain-wise; put roughly a **2-week minimum age** on dependency
versions (avoid brand-new releases). Combine with the existing
crate-landscape vetting in DESIGN.md ([[cad-project-state]]).
