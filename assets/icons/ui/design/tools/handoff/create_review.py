from pathlib import Path
import xml.etree.ElementTree as ET
R=Path(__file__).resolve().parent;NS='http://www.w3.org/2000/svg';ET.register_namespace('',NS)
def el(t,a=None,text=None):n=ET.Element('{'+NS+'}'+t,a or {});n.text=text;return n
rows=[('Tools',[('sweep','Sweep'),('fillet','Fillet reference'),('chamfer','Chamfer'),('combine','Combine'),('scale','Scale'),('params','Parameters'),('plane','Offset plane'),('plane_angle','Plane at angle'),('measure','Measure')]),('Navigation',[('orbit','Orbit'),('spin','Continuous orbit'),('pan','Pan'),('zoom','Zoom'),('fit','Fit'),('lookat','Look at'),('home','Home'),('perspective','Perspective'),('style','Visual style'),('nav_settings','Settings'),('moon','Dark mode')])]
b=el('svg',{'width':'1210','height':'360','viewBox':'0 0 1210 360'});b.append(el('rect',{'width':'1210','height':'360','fill':'white'}))
def icon(board,key,label,x,y,sub='icons',size=80):
 n=ET.parse(R/sub/(key+'.svg')).getroot();n.attrib.update(x=str(x),y=str(y),width=str(size),height=str(size));board.append(n)
 board.append(el('text',{'x':str(x+size/2),'y':str(y+size+20),'font-family':'Arial','font-size':'11','text-anchor':'middle','fill':'#34465b'},label))
for j,(title,items) in enumerate(rows):
 y=20+j*174;b.append(el('text',{'x':'16','y':str(y),'font-family':'Arial','font-size':'15','font-weight':'600','fill':'#34465b'},title))
 for i,(key,label) in enumerate(items):icon(b,key,label,16+i*108,y+12,'icons' if j==0 else 'icons-flat')
ET.ElementTree(b).write(R/'revised-icons.svg',encoding='unicode')
b=el('svg',{'width':'448','height':'124','viewBox':'0 0 448 124'});b.append(el('rect',{'width':'448','height':'124','fill':'white'}))
for i,(key,label,sub) in enumerate([('params','Parameters','icons'),('scale','Scale','icons'),('sweep','Sweep','icons'),('lookat','Look at','icons-flat')]):icon(b,key,label,16+i*112,8,sub)
ET.ElementTree(b).write(R/'latest-corrections.svg',encoding='unicode')
