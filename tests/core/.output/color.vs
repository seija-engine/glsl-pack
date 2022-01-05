////////////生成的代码////////////////////
layout(location = 0) vec3 vert_position;

#if defined(VERTEX_COLOR)
  layout(location = 1) vec4 vert_color;
#endif

//Transform3D
mat4 getTransformMatrix() {
   return objectUniform.transMat;
}

//CameraBackend
mat4 getCameraView() {
   return frameUniform.cameraView;
}

//CameraBackend
mat4 getCameraProj() {
   return frameUniform.cameraProj;
}

//CameraBackend
mat4 getCameraPosition() {
   return frameUniform.cameraPos;
}

void material_vertex(VSOutput output) {
  output.color = output.color * materialProp.color;
}

struct MaterialProp {
  vec4 color;
}

layout(location = 0) out vec4 out_Color;
///////////////////////////////////////
struct VSOutput {
  vec4 vertColor;
  vec3 position;
}

VSOutput vs_main() {
    mat4 mvp = getTransformMatrix() * (getCameraView() * getCameraProj());
    gl_Position = mvp * vert_position;
    VSOutput output;
    output.position = gl_Position;
    material_vertex(output);
    #if defined(VERTEX_COLOR)
      output.color = output.color * vert_color;
    #endif
}


struct MaterialPixel {
  vec4 baseColor;
}

void fs_main(VSOutput vsParams) {
    MaterialPixel pixel;
    pixel.color = pixel.color * materialProp.color;
    material_frag(pixel,vsParams);
    out_Color = pixel.color;
}