#!/usr/bin/env bash
# Temporary investigation helper (NOT for merge): block until a PR's checks settle.
pr="${1:?usage: bt-wait-pr.sh <pr-number>}"
while true; do
  done_yet=$(gh pr checks "$pr" -R evgunter/cad --json state \
    --jq 'all(.[]; .state != "PENDING" and .state != "QUEUED" and .state != "IN_PROGRESS")' 2>/dev/null)
  [ "$done_yet" = "true" ] && break
  sleep 60
done
echo "PR $pr checks settled"
gh pr checks "$pr" -R evgunter/cad 2>&1 | head -40
