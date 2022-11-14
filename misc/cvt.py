from gltflib import GLTF

if __name__ == "__main__":
    gltf = GLTF.load('../assets/pieces.glb')
    glb_resource = gltf.get_glb_resource()
    gltf.convert_to_file_resource(glb_resource, 'BoxTextured.bin')
    gltf.export('BoxTextured.gltf')