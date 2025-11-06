# blender -b -Y -P loader.py
import sys
import bpy # type: ignore
import base64
import json
#import psycopg2

def decode_input_data(data):
    base64_bytes = data.encode("ascii")
    sample_string_bytes = base64.b64decode(base64_bytes)
    sample_string = sample_string_bytes.decode("ascii")
    j = json.loads(sample_string)
    print(j)
    return j

bpy.app.debug_wm = True
writer_fd = "7"
back_channel = open('/dev/fd/' + writer_fd, 'w')
data = decode_input_data(sys.argv[-1])

def import_texture(path):
    tex = bpy.data.textures.new(name="custom_texture", type="IMAGE")
    img = bpy.data.images.load(path)
    tex.image = img
    
    return tex

def set_property(key, value):
    print("Setting " + key + " to " + str(value))
    bpy.data.objects["Plane"][key] = value


def render_init(scene):
    global data
    set_property("texture_index", data["texture_index"])
    set_property("repetition", data["repetition"])
    set_property("scaling", data["scaling"])
    set_property("rotation", data["rotation"])
    set_property("pingpong", data["pingpong"])
    set_property("_frames_start", data["frames"]["_frames_start"])
    set_property("_frames_max", data["frames"]["_frames_max"])
        
    for key in data["composite"].keys():
        set_property(key, data["composite"][key])
        
    if data["texture_index"] == 6:
        bpy.data.node_groups["background"].nodes["texture"].image = bpy.data.images.load(data["texture"]["file_path"])
    else:
        for key in data["texture"].keys():
            set_property(key, data["texture"][key])
            
    # save_blend_file(scene)
    bpy.ops.wm.save_as_mainfile(filepath=data["output_directory"] + "/" + data["id"] + "/project.blend")
    
def post_render(scene):
    global data
    status = json.dumps({'id': data["id"] , 'frame': scene.frame_current})
    back_channel.write(status + "\n")
    back_channel.flush()

bpy.app.handlers.render_init.clear()
bpy.app.handlers.render_init.append(render_init)
bpy.app.handlers.render_post.clear()
bpy.app.handlers.render_post.append(post_render)