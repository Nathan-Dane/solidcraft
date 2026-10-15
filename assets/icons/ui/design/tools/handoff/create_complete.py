from pathlib import Path
import runpy,copy,math,json,xml.etree.ElementTree as ET
R=Path(__file__).resolve().parent
m=runpy.run_path(str(R/'create_base.py'))
NS=m['NS']; ET.register_namespace('',NS)
el=m['el'];path=m['path'];poly=m['poly'];circle=m['circle'];line=m['line'];cube=m['cube']
I=m['INK'];G=m['GREY'];B=m['BLUE'];E=2.55;A=4.25
# The existing accepted geometry is preserved, with every line reduced exactly 15%.
for p in [*(R/'icons').glob('*.svg'),*(R/'icons-small').glob('*.svg')]:
 if p.stem not in {'extrude','revolve','fillet','section','body','component','folder','sketch','undo','save','settings','zoom'}:continue
 root=ET.parse(p).getroot()
 for n in root.iter():
  if n.get('stroke-width'):n.set('stroke-width',str(float(n.get('stroke-width'))*.85))
 ET.ElementTree(root).write(p,encoding='unicode')
def p(d,fill='none',stroke=I,w=E,attrs=None):return path(d,fill,stroke,w,attrs)
def l(points,c=I,w=E,attrs=None):return line(points,c,w,attrs)
def rect(x,y,w,h,fill=G[1],stroke=I,sw=E,r=0):
 a={'x':str(x),'y':str(y),'width':str(w),'height':str(h),'fill':fill,'rx':str(r)}
 if stroke:a.update(stroke=stroke,**{'stroke-width':str(sw),'stroke-linejoin':'round'})
 return el('rect',a)
def disk(x,y,r,fill,stroke=I,w=E):
 a={'cx':str(x),'cy':str(y),'r':str(r),'fill':fill}
 if stroke:a.update(stroke=stroke,**{'stroke-width':str(w)})
 return el('circle',a)
def arrow(x,y,dx,dy,c=B[2],w=E):
 n=math.hypot(dx,dy);ux,uy=dx/n,dy/n; bx,by=x+dx-7*ux,y+dy-7*uy
 return [l([(x,y),(bx,by)],c,w),p(poly([(x+dx,y+dy),(bx-4*uy,by+4*ux),(bx+4*uy,by-4*ux)]),c,None)]
def mini(cx,cy,s=1,tones=None):
 # Keep geometric strokes at their final width, independent of the cube's size.
 pts=[(32,6),(8,20),(56,20),(8,46),(32,60),(56,46)]
 return cube(*[(cx+(x-32)*s,cy+(y-33)*s) for x,y in pts],tones or G,E)
def plane(points,fill=B[0],stroke=I):return p(poly(points),fill,stroke)
def block():return mini(32,34,.92)
def sheet():return [p('M10 42 Q24 16 51 22 L55 42 Q33 37 19 56 Z',G[1])]
def cross(x,y,s,c=I,w=E):return [l([(x-s,y),(x+s,y)],c,w),l([(x,y-s),(x,y+s)],c,w)]
D={}
D['sweep']=[p('M9 52 V37 Q9 15 32 15 H48 V29 H33 Q23 29 23 39 V52 Z',B[1]),p('M9 52 L16 57 L30 57 L23 52 Z',G[1]),p('M48 15 L55 21 V35 L48 29 Z',G[2])]
D['chamfer']=[plane([(9,21),(29,9),(42,9),(55,22),(55,45),(32,59),(9,45)],G[1]),plane([(9,21),(29,9),(42,9),(21,22)],G[0]),plane([(21,22),(42,9),(55,22),(32,35)],B[1]),plane([(32,35),(55,22),(55,45),(32,59)],G[2]),l([(9,21),(21,22),(32,35),(32,59)])]
D['combine']=mini(24,38,.58)+mini(41,25,.58)+[plane([(28,28),(38,22),(47,27),(38,33)],B[1]),l([(38,33),(38,43)],B[2],4)]
D['scale']=mini(25,41,.45)+[p('M12 25 V8 H56 V52 H39',stroke='#8394a5',attrs={'stroke-dasharray':'4 5'})]+arrow(27,37,25,-25)
D['joint']=mini(18,40,.45)+mini(46,24,.45)+[l([(24,35),(42,28)],B[2],9),disk(33,32,6,B[0])]
D['as_built']=mini(18,34,.46)+mini(46,34,.46)+[rect(24,27,16,14,B[1]),disk(32,34,3,'white',None)]
D['params']=[rect(11,8,42,48,G[0],r=4),l([(19,20),(45,20)],'#8e9faf'),l([(19,32),(45,32)],'#8e9faf'),l([(19,44),(45,44)],'#8e9faf'),rect(25,15,7,10,B[1],None,r=2),rect(35,27,7,10,B[1],None,r=2),rect(21,39,7,10,B[1],None,r=2)]
D['config']=[rect(8,10,48,44,G[0],r=3),rect(9.3,11.3,45.4,11,B[1],None),rect(26,23,14,30,B[0],None),l([(8,23),(56,23)]),l([(8,38),(56,38)]),l([(25,23),(25,54)]),l([(40,23),(40,54)])]
D['plane']=[plane([(8,35),(37,18),(57,28),(28,46)],G[1]),plane([(8,21),(37,4),(57,14),(28,32)],B[0]),l([(28,36),(28,43)],B[2],3,{'stroke-dasharray':'2 4'})]+arrow(45,29,0,-16)
D['plane_angle']=[plane([(7,41),(35,25),(57,37),(29,53)],G[1]),plane([(29,53),(35,25),(47,5),(41,33)],B[0]),p('M48 45 Q59 35 51 22',stroke=B[2])]
D['interference']=mini(23,38,.56)+mini(41,26,.56)+[plane([(27,28),(36,23),(44,28),(35,33)],B[1]),plane([(27,28),(35,33),(35,44),(27,39)],B[1]),plane([(35,33),(44,28),(44,39),(35,44)],B[2])]
D['measure']=[p('M8 43 L44 7 L57 20 L21 56 Z',G[0]),l([(14,41),(20,47)]),l([(23,32),(27,36)]),l([(32,23),(38,29)]),l([(41,14),(45,18)]),l([(10,55),(55,10)],B[2],3,{'stroke-dasharray':'3 5'})]
D['fastener']=[plane([(16,12),(32,4),(48,12),(48,23),(32,31),(16,23)],G[1]),plane([(16,12),(32,4),(48,12),(32,20)],G[0]),p('M25 28 V53 L32 60 L39 53 V28',G[1]),l([(25,37),(39,43)]),l([(25,46),(39,52)]),p('M27 12 H37 M32 8 V17',stroke=B[2],w=4)]
D['import']=mini(32,42,.75)+arrow(32,4,0,23)
# Surface/analysis controls: descriptive silhouettes, one blue result area, no decorative shading.
D['zebra']=sheet()+[p('M17 34 Q30 18 53 27',stroke=B[2],w=4),p('M12 43 Q29 28 54 35',stroke=B[2],w=4),p('M20 51 Q35 36 53 42',stroke=B[2],w=4)]
D['draft_analysis']=[plane([(12,51),(24,11),(41,11),(54,51)],G[1]),plane([(32,51),(32,11),(41,11),(54,51)],B[1]),l([(32,5),(32,59)],I,2.04,{'stroke-dasharray':'3 5'})]
D['curvature_map']=sheet()+[p('M31 23 Q43 19 51 22 L54 34 Q43 31 35 38 Z',B[1],None),p('M27 36 Q36 29 40 32 Q38 41 29 46 L19 56 L16 47 Z',B[0],None),p('M10 42 Q24 16 51 22 L55 42 Q33 37 19 56 Z','none',I)]
D['environment_map']=[disk(32,32,24,G[0]),p('M9 26 Q30 38 55 22 L56 33 Q30 48 10 38 Z',B[1],None),p('M15 48 Q34 38 49 47',stroke=I),disk(32,32,24,'none')]
D['accessibility']=[rect(9,42,46,13,G[1]),rect(22,29,20,13,B[1])]+arrow(32,4,0,20)+[l([(13,24),(13,35)],I),l([(51,24),(51,35)],I)]
D['min_radius']=[p('M9 52 Q10 10 32 10 Q54 10 55 52',stroke=I,w=5),p('M21 38 A13 13 0 0 1 43 38',stroke=B[2],w=4),disk(32,43,3,B[2],None),l([(32,43),(39,32)],B[2])]
D['iso_analysis']=sheet()+[p('M14 35 Q28 21 51 27',stroke=B[2]),p('M13 43 Q31 28 53 35',stroke=B[2]),p('M23 24 Q18 36 22 52',stroke=B[2]),p('M37 21 Q31 35 35 45',stroke=B[2]),p('M48 22 Q43 32 47 41',stroke=B[2])]
D['curvature_comb']=[p('M8 49 Q27 9 55 39',stroke=I,w=3.8),l([(14,39),(7,33)],B[2]),l([(21,29),(16,17)],B[2]),l([(30,25),(30,7)],B[2]),l([(40,26),(45,10)],B[2]),l([(49,32),(59,22)],B[2])]
D['center_of_mass']=mini(32,33,.78)+[disk(32,34,12,'white',I),p('M32 22 A12 12 0 0 1 44 34 H32 Z',B[1],None),p('M32 46 A12 12 0 0 1 20 34 H32 Z',B[1],None),l([(16,34),(48,34)],I,1.8),l([(32,18),(32,50)],I,1.8)]
# Flat single-colour controls use broad, rounded silhouettes and open counters.
flat={}
flat['home']=[p('M7 29 L32 8 L57 29 H49 V55 H37 V39 H27 V55 H15 V29 Z',I,None)]
flat['moon']=[p('M46 10 A24 24 0 1 1 15 17 A20 20 0 0 0 46 10 Z',I,None)]
flat['help']=[disk(32,32,23,'none',I,A),p('M24 24 C24 13 43 15 41 26 C40 31 32 31 32 38',stroke=I,w=A),disk(32,47,2.6,I,None)]
flat['report']=[rect(20,18,24,34,I,None,r=10),disk(32,17,7,I,None),l([(12,22),(20,27)],I,A),l([(11,37),(20,37)],I,A),l([(12,51),(21,46)],I,A),l([(52,22),(44,27)],I,A),l([(53,37),(44,37)],I,A),l([(52,51),(43,46)],I,A),l([(32,26),(32,45)],'white',3)]
flat['orbit']=[p('M48 19 A22 22 0 1 0 49 46',stroke=I,w=A),p('M53 8 L55 26 L38 21 Z',I,None),disk(31,33,4,I,None)]
flat['spin']=[p('M50 26 A20 20 0 0 0 14 22',stroke=I,w=A),p('M10 10 L9 28 L27 24 Z',I,None),p('M14 40 A20 20 0 0 0 50 44',stroke=I,w=A),p('M54 56 L55 38 L37 42 Z',I,None)]
flat['pan']=[p('M32 5 L41 17 H35 V29 H47 V23 L59 32 L47 41 V35 H35 V47 H41 L32 59 L23 47 H29 V35 H17 V41 L5 32 L17 23 V29 H29 V17 H23 Z',I,None)]
flat['fit']=[p('M10 24 V10 H24 M40 10 H54 V24 M54 40 V54 H40 M24 54 H10 V40',stroke=I,w=A),rect(24,24,16,16,I,None,r=2)]
flat['lookat']=[rect(10,13,31,39,'none',I,A,3)]+arrow(57,32,-29,0,I,A)
flat['perspective']=[p('M10 17 L54 7 V57 L10 47 Z','none',I,A),p('M22 26 L43 21 V43 L22 39 Z',I,None)]
flat['style']=[p('M32 7 L55 20 V46 L32 59 L9 46 V20 Z',I,None),p('M32 32 L32 51 M15 23 L32 32 L49 23',stroke='white',w=3.6)]
flat['chevron']=[p('M16 24 L32 40 L48 24',stroke=I,w=5)]
flat['folder']=[p('M8 20 Q8 15 13 15 H26 L32 21 H52 Q57 21 57 26 V49 Q57 54 52 54 H13 Q8 54 8 49 Z',I,None)]
exec((R/'revised_geometry.py').read_text(),globals())
# Flat counterparts are deliberately separate from volumetric toolbar/object glyphs.
for key,nodes in D.items():
 for sub in ['icons','icons-small']:
  root=el('svg',{'width':'48','height':'48','viewBox':'0 0 64 64','role':'img'})
  root.append(el('title',text='Technical Accent — '+key.replace('_',' ')))
  for n in nodes:root.append(copy.deepcopy(n))
  ET.ElementTree(root).write(R/sub/(key+'.svg'),encoding='unicode')
(R/'icons-flat').mkdir(exist_ok=True)
for key,nodes in flat.items():
 root=el('svg',{'width':'48','height':'48','viewBox':'0 0 64 64','role':'img'})
 root.append(el('title',text='Flat controls — '+key))
 for n in nodes:root.append(n)
 ET.ElementTree(root).write(R/'icons-flat'/(key+'.svg'),encoding='unicode')
for key in ['undo','save','settings','zoom']:(R/'icons-flat'/(key+'.svg')).write_bytes((R/'icons-small'/(key+'.svg')).read_bytes())
# Sheet of all newly authored toolbar/analysis icons, for optical review.
keys=list(D)
b=el('svg',{'width':'720','height':str(math.ceil(len(keys)/8)*104),'viewBox':f'0 0 720 {math.ceil(len(keys)/8)*104}'})
b.append(rect(0,0,720,math.ceil(len(keys)/8)*104,'white',None))
for j,key in enumerate(keys):
 x=12+(j%8)*90;y=8+(j//8)*104
 n=ET.parse(R/'icons'/(key+'.svg')).getroot();n.attrib.update(x=str(x+9),y=str(y),width='48',height='48');b.append(n)
 b.append(el('text',{'x':str(x+33),'y':str(y+69),'text-anchor':'middle','font-family':'Arial','font-size':'10','fill':I},key.replace('_',' ')))
ET.ElementTree(b).write(R/'new-icons.svg',encoding='unicode')
(R/'palette.json').write_text(json.dumps({'ink':I,'grey':G,'blue':B,'geometry_stroke':E,'action_stroke':A,'navigation_stroke':5.6,'parameters_stroke':4.2,'line_reduction':.15},indent=2))
print('Created',len(D),'additional geometric subjects and',len(flat)+4,'flat control masters.')

zoom=ET.parse(R/'icons-flat/zoom.svg').getroot()
for n in zoom.iter():
 if n.get('stroke-width'):n.set('stroke-width','5.6')
ET.ElementTree(zoom).write(R/'icons-flat/zoom.svg',encoding='unicode')
settings=ET.parse(R/'icons-flat/settings.svg').getroot();settings.set('viewBox','2 2 60 60')
ET.ElementTree(settings).write(R/'icons-flat/nav_settings.svg',encoding='unicode')
