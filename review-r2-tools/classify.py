import sys,re,collections
cls=collections.Counter(); ex={}
runs=0
def flush(tag,mints,res):
    global runs
    if tag is None: return
    runs+=1
    out=res.split()[0:2] if res else ['?']
    o=' '.join(out[:2]) if out and out[0]=='OK' else (res.split('{')[0].strip() if res else '?')
    if 'what:' in (res or ''): o+=' '+res.split('what:')[1][:40]
    # group mints by (slot, plan)
    by=collections.defaultdict(dict)
    for m in mints:
        by[(m['slot'],m['plan'])][m['run']]=m
    keys=set()
    for (slot,plan),rs in by.items():
        for k,m in rs.items():
            if m['holder'] is None: continue
            # depth & chain kinds
            chain=[]; h=m['holder']; d=0
            while h is not None and h in rs:
                hm=rs[h]; chain.append('fan' if hm['fan']>0 else 'strut'); d+=1; h=hm['holder']
            held='fan' if m['fan']>0 else 'strut'
            hm=rs[m['holder']]
            if held=='strut':
                pos='first' if m['from']==hm['from'] else ('last' if m['from']==hm['to'] else 'mid')
            else:
                pos='endfirst' if m['to']==hm['from'] else ('endlast' if m['to']==hm['to'] else 'mid')
            keys.add(f"d{d} held={held}@{pos} holders={'/'.join(chain)}")
    if not keys: keys={'no-nest'}
    for kk in keys:
        cls[(kk,o)]+=1
        ex.setdefault((kk,o),tag)
tag=None;mints=[];res=None
for line in open(sys.argv[1],errors='replace'):
    if line.startswith('SXRUN '):
        flush(tag,mints,res); tag=line[6:].strip(); mints=[]; res=None
    elif line.startswith('SXMINT'):
        d=dict(re.findall(r'(\w+)=(\S+)',line.split(' holder=')[0]))
        hm=re.search(r'holder=(None|Some\(\((\d+), (\d+), (\d+), (\d+)\)\))',line)
        mints.append(dict(slot=int(d['slot']),plan=int(d['plan']),run=int(d['run']),fr=0,**{'from':int(d['from'])},to=int(d['to']),fan=int(d['fanlen']),holder=None if hm.group(1)=='None' else int(hm.group(2))))
    elif line.startswith('SXRES '):
        res=line[6:].strip()
flush(tag,mints,res)
print("runs",runs)
for (k,o),c in sorted(cls.items()):
    print(f"{c:7d}  {k:55s} {o}   e.g. {ex[(k,o)]}")
