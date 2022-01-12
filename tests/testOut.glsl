#version 450
layout(location = 0) vec3 vert_position;
#ifdef VERTEX_NORMAL
layout(location = 3) vec3 vert_normal;
#endif
#ifdef VERTEX_COLOR
layout(location = 5) vec4 vert_color;
#endif
layout(set = 0, binding = 0) uniform FrameUniforms {
  mat4 cameraVP;
  mat4 cameraView;
  mat4 cameraP;
  vec4 cameraPos;
} frameUniforms;
layout(set = 1, binding = 0) objectUniforms {
  mat4 transform;
}
mat4 getCameraView() {
  return frameUniforms.cameraView;
}
mat4 getCameraViewProject() {
  return frameUniforms.cameraVP;
}
int testCoreFn() {
    return 114514;
}

const float cPI = 3.1415927;
float pow5(float n) {
    testCoreFn();
    return n * n * n * n * n;
}

struct CustomType {
    vec4 colorIntensity;
};