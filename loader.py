# blender -b -Y -P loader.py
import sys
import bpy
import base64
import json

bpy.app.debug_wm = True

def decode_input_data(data):
    base64_bytes = data.encode("ascii")
    sample_string_bytes = base64.b64decode(base64_bytes)
    sample_string = sample_string_bytes.decode("ascii")
    return json.loads(sample_string)

def set_property(key, value):
    print("Setting " + key + " to " + str(value))
    bpy.data.objects["Plane"][key] = value


def init(scene):
    data = decode_input_data(sys.argv[-1])
    set_property("texture_index", data["texture_index"])
    set_property("repetition", data["repetition"])
    set_property("scaling", data["scaling"])
    set_property("rotation", data["rotation"])
    set_property("pingpong", data["pingpong"])
    
    for key in data["texture"].keys():
        set_property(key, data["texture"][key])
    
def post_render(scene):
    print("post render")    
    # this forces blender to quit itself
    #bpy.ops.wm.quit_blender()

bpy.app.handlers.render_init.clear()
bpy.app.handlers.render_init.append(init)
bpy.app.handlers.render_post.clear()
bpy.app.handlers.render_post.append(post_render)