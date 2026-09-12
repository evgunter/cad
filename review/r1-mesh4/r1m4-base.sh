#!/usr/bin/env bash
# Base side of R1's two-build gate: check out the merge base's
# crates/mesh (the ONLY paths the PR touches) into the lane, run the
# same suites, restore the frozen head, verify byte-identity.
set -euo pipefail
LANE=/root/lanes/mesh-4r1
BASE=ba0a90a0811e30c1a34021fb51bd1031c2cc53e9
cd "$LANE"
git checkout "$BASE" -- crates/mesh
echo "base checkout: $(git diff --stat HEAD -- crates/mesh | tail -1)"
/tmp/claude-0/r1m4/r1m4-runs.sh "$LANE" /tmp/claude-0/r1m4/raw.base
/tmp/claude-0/r1m4/r1m4-extract.sh /tmp/claude-0/r1m4/raw.base /tmp/claude-0/r1m4/digest-base.txt
git checkout HEAD -- crates/mesh
git diff --quiet HEAD -- crates/ && echo "RESTORED-CLEAN (byte-identical to frozen head)"
echo "== digests =="
md5sum /tmp/claude-0/r1m4/digest-base.txt /tmp/claude-0/r1m4/digest-head.txt
diff /tmp/claude-0/r1m4/digest-base.txt /tmp/claude-0/r1m4/digest-head.txt > /tmp/claude-0/r1m4/digest.diff \
  && echo "DIGESTS IDENTICAL" || { echo "DIGESTS DIFFER:"; head -40 /tmp/claude-0/r1m4/digest.diff; }
