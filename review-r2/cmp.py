import sys, re, collections
def load(fs):
    d={}
    for f in fs:
        for line in open(f):
            line=line.rstrip('\n')
            if ': ' not in line: continue
            k,v=line.split(': ',1)
            d[k]=v
    return d
def cat(v, curved=False):
    if v.startswith('PANIC'): return 'PANIC'
    if v.startswith('EMPTY ok'): return 'EMPTY'
    if v.startswith('ERR'):
        m=re.match(r'ERR (\w+(\(\w+)?)',v); return 'REF:'+m.group(1)
    if v.startswith('OK'):
        if curved:
            ok = 't2=true t3p=true cert=true' in v
            mv=re.search(r' v=(\S+) want=(\S+)',v)
            if mv and abs(float(mv.group(1))-float(mv.group(2)))>1e-7: ok=False
            return ('SOUND' if ok else 'BAD')+(' F2V' if 'FACE2V' in v else '')
        return ('SOUND' if 'OK SOUND' in v else 'BAD')+(' F2V' if 'FACE2V' in v else '')
    return 'OTHER'
curved = sys.argv[1]=='cyl'
a=load(sys.argv[2].split(','))
b=load(sys.argv[3].split(','))
print('lines', len(a), len(b))
trans=collections.Counter()
ex={}
for k in a:
    if k not in b: trans[('missing',)]+=1; continue
    ca, cb = cat(a[k],curved), cat(b[k],curved)
    if ca!=cb:
        def grp(c): return 'refusal' if c.startswith('REF') else c
        key=(grp(ca), grp(cb))
        trans[key]+=1
        ex.setdefault(key, []).append(k)
    elif a[k]!=b[k] and not ca.startswith('REF') and ca!='PANIC':
        trans[('same-cat-diff-line',ca)]+=1
        ex.setdefault(('same-cat-diff-line',ca),[]).append(k)
for t,n in sorted(trans.items(), key=lambda x:-x[1]): print(n, t, ex.get(t,[''])[:2])
