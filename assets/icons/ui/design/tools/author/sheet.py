#!/usr/bin/env python3
"""view.py out.html size id1 id2 ... | @stand_in | @done  -- big sheet, light+dark side by side"""
import sys, os, tomllib
D=os.environ.get('DD',os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
out, size = sys.argv[1], int(sys.argv[2]); ids = sys.argv[3:]
pal = tomllib.load(open(D+'/palette.toml','rb')); roles=pal['roles']
st = tomllib.load(open(D+'/status.toml','rb'))['stand_in']
import glob
allm = sorted(os.path.basename(f)[:-4] for f in glob.glob(D+'/svg/model/*.svg'))
if ids==['@stand_in']: ids=st
elif ids==['@done']: ids=[m for m in allm if m not in st]
elif ids==['@all']: ids=allm
def themed(svg, theme):
    for r in roles.values(): svg = svg.replace(r['light'], '\0'+r[theme])
    return svg.replace('\0','').replace('currentColor', roles['line'][theme])
h='<meta charset=utf-8><style>body{margin:0;font:11px sans-serif}.row{display:flex;flex-wrap:wrap}.c{width:%dpx;padding:6px;text-align:center}.l{background:#fff;color:#222}.d{background:#1e2530;color:#ddd}.cell{display:inline-block;width:%d;vertical-align:top}</style><div class=row>'%(size*2+20,size)
for i in ids:
    p=None
    for fam in (('small','model','flat','workspace') if os.environ.get('FAM')=='small' else ('model','small','flat','workspace')):
        q=f'{D}/svg/{fam}/{i}.svg'
        if os.path.exists(q): p=q;break
    if not p: continue
    s=open(p).read().replace('<svg ','<svg width=%d height=%d '%(size,size),1)
    def sv(n,theme): return themed(open(p).read().replace('<svg ','<svg width=%d height=%d '%(n,n),1),theme)
    h+='<div style="margin:4px;text-align:center"><div style="display:flex;align-items:flex-start"><div class=l style="padding:2px">%s</div><div style="display:flex;flex-direction:column"><div class=d style="padding:2px">%s</div><div class=l style="padding:2px;display:flex;gap:4px">%s%s</div></div></div>%s</div>'%(sv(size,'light'),sv(size//2,'dark'),sv(24,'light'),sv(16,'light'),i)
open(out,'w').write(h)
