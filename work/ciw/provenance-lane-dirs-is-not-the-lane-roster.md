---
id: provenance-lane-dirs-is-not-the-lane-roster
kind: issue
title: check_render_provenance.py's LANE_DIRS calls itself the lanes and is the PNG trees
status: open
opened: 2026-09-11
---


`demos/check_render_provenance.py:107` is

```python
LANE_DIRS = ("renders", "renders-freecad", "renders-wild", "renders-gui")
```

under a comment that calls it "the committed render trees, one per
montage lane (render.sh / render-wild.sh)". `render.yml` declares six
lanes and re-baselines six directories; two of them — `renders-uv` and
`renders-mc` — are not here.

**The omission is correct and the sentence above it is not.** Both
missing trees hold a single SVG (`montage-uv.svg`, `plate-density.svg`);
PNG `tEXt` provenance chunks are the subject of this file and an SVG has
none. So the tuple is the PNG lanes, not the lanes, and it is complete
for what it actually checks. What is wrong is the claim that it is one
per lane, which is the sentence a reader checks it against.

**Why it is not folded into the lane-parity guard.**
`scripts/check-render-lane-parity.py` compares lane FACTS render.yml
declares — the roster, artifact names, committed directories, the poll's
job regex. "This lane draws PNGs" is not one of them: render.yml says it
only in prose, exactly as it says which lanes are byte-reproducible
(`verify-lane-set-is-a-property-nothing-declares`, the same shape one
file over). A guard over this tuple would have to invent that
declaration, and the two wants the same key.

**What closing it looks like:** at minimum, correct the comment to say
what the tuple is — the PNG lanes — so the next reader does not check it
against the wrong population. If render.yml grows a declared per-lane
property for the reproducible set, this one keys on the same mechanism
and becomes a claim rather than a tuple.
