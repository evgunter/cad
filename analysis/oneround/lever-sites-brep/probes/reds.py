#!/usr/bin/env python3
"""The red tests of a differential row with the first line of each panic. Usage: reds.py <run log>"""
import re, sys
L = open(sys.argv[1], errors='replace').read().split('\n')
seen = {}
i = 0
while i < len(L):
    m = re.match(r'\s+(FAIL|TIMEOUT|SIGABRT|SIGSEGV)\s+\[[^\]]*\]\s+\(\s*\d+/\d+\)\s+(\S+ \S+)', L[i])
    if m and m.group(2) not in seen:
        msg = ''
        for j in range(i + 1, min(i + 400, len(L))):
            if 'panicked at' in L[j]:
                msg = (L[j].strip() + ' | ' + ' '.join(x.strip() for x in L[j + 1:j + 4]))[:int(sys.argv[2]) if len(sys.argv) > 2 else 380]
                break
            if re.match(r'\s+(PASS|FAIL)\s+\[', L[j]): break
        seen[m.group(2)] = msg
    i += 1
for k in sorted(seen): print('==', k, '\n    ', seen[k])
