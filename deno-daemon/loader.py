# blender -b -Y -P loader.py
import sys
import bpy # type: ignore
import base64
import json
import os
import socket
#import psycopg2

sock_path = os.environ["STATUS_SOCKET"]
sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)

def decode_input_data(data):
    base64_bytes = data.encode("ascii")
    sample_string_bytes = base64.b64decode(base64_bytes)
    sample_string = sample_string_bytes.decode("ascii")
    j = json.loads(sample_string)
    #print(j)
    return j

def import_texture(path):
    tex = bpy.data.textures.new(name="custom_texture", type="IMAGE")
    img = bpy.data.images.load(path)
    tex.image = img
    return tex

def set_property(key, value):
    print("Setting " + key + " to " + str(value) + "\n")
    bpy.data.objects["Plane"][key] = value

def type_to_tex_index(s: str):
    if s == "gabor":
        return 0
    elif s == "voronoi":
        return 1
    elif s == "wave":
        return 2
    elif s == "magic":
        return 3
    elif s == "noise":
        return 4
    elif s == "unoise":
        return 5
    elif s == "textured":
        return 6

def r_init(scene):
    global data
    set_property("texture_index", type_to_tex_index(data["texture"]["type"]))
    set_property("repetition", data["polar"]["repetition"])
    set_property("scaling", data["polar"]["scaling"])
    set_property("rotation", data["polar"]["rotation"])
    set_property("pingpong", data["polar"]["pingpong"])
    if data["frames"]["type"] == "animated":
        set_property("_frames_start", data["frames"]["start"])
        set_property("_frames_max", data["frames"]["end"])
        
    for key in data["composite"].keys():
        set_property(key, data["composite"][key])
        
    if type_to_tex_index(data["texture"]["type"]) == 6:
        bpy.data.node_groups["background"].nodes["texture"].image = bpy.data.images.load(data["texture"]["file_path"])
    else:
        for key in data["texture"].keys():
            set_property(key, data["texture"][key])
            
    # save_blend_file(scene)
    # bpy.ops.wm.save_as_mainfile(filepath=data["output_directory"] + "/" + data["id"] + "/project.blend")
    
def post_render(scene):
    global data
    global sock
    status = json.dumps({'id': data["id"] , 'frame': scene.frame_current})
    sock.sendall((status + "\n").encode("utf-8"))
    #back_channel.write(status + "\n")
    #back_channel.flush()
    
    #sock.close()

if __name__ == "__main__":
    print("main function")
    bpy.app.debug_wm = True
    
    bpy.app.handlers.render_init.clear()
    bpy.app.handlers.render_init.append(r_init)
    bpy.app.handlers.render_post.clear()
    bpy.app.handlers.render_post.append(post_render)

    sock.connect(sock_path)

#back_channel = open('/dev/fd/' + writer_fd, 'w')
    data = decode_input_data(sys.argv[-1])