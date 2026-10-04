import sys,re
def load(p):
    d={}
    for l in open(p):
        if ': ' not in l or l.startswith('#'): continue
        k,v=l.rstrip('\n').split(': ',1); d[k]=v
    return d
def cls(v):
    if v.startswith('OK SOUND'): return 'SOUND'
    if v.startswith('OK BAD') or v.startswith('OK-UN') or 'WRONG' in v: return 'BAD'
    if v.startswith('PANIC'): return 'PANIC'
    if v.startswith('EMPTY ok'): return 'SOUND'
    return 'REF'
b,h=load(sys.argv[1]),load(sys.argv[2])
from collections import Counter
c=Counter(); ex={}
for k in h:
    if k not in b: c['new-only']+=1; continue
    t=(cls(b[k]),cls(h[k]))
    if b[k]==h[k]: t=t+('same',)
    c[t]+=1; ex.setdefault(t,[]).append(k)
print(len(b),len(h))
for t,n in sorted(c.items(),key=lambda x:-x[1]): print(t,n, ex.get(t,[''])[0] if isinstance(t,tuple) else '')
