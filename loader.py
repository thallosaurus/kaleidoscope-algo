# blender -b -Y -P loader.py
import sys
import bpy

data = sys.argv[-1]

def init(scene):
    print("Render")

bpy.app.handlers.render_init.clear()
bpy.app.handlers.render_init.append(init)