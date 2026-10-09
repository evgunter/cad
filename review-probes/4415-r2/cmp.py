import sys, re, collections
def load(p):
    d={}
    for line in open(p):
        if line.startswith(' ') or ': ' not in line: continue
        tag,rest=line.rstrip('\n').split(': ',1)
        d[tag]=rest
    return d
b,h=load(sys.argv[1]),load(sys.argv[2])
assert set(b)==set(h), (len(b),len(h), list(set(b)^set(h))[:5])
moved=collections.Counter(); ex={}
for k in b:
    x,y=b[k],h[k]
    if x==y: continue
    xs=re.sub(r' want=\S+','',x); ys=re.sub(r' want=\S+','',y)
    if xs==ys: moved['oracle-only']+=1; continue
    kx='ShellRoleUndecided' in x and x.startswith('ERR ResultInvalid')
    ky=y.startswith('ERR Escalated { decision: ShellRole')
    key=('RI{ShellRoleUndecided}->Esc{ShellRole}' if kx and ky else 'OTHER')
    moved[key]+=1; ex.setdefault(key,[]).append((k,x[:150],y[:150]))
print(len(b),'runs', dict(moved))
for k,v in ex.items():
    if k=='OTHER':
        for e in v[:10]: print(e)
