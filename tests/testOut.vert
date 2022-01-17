#version 450
layout(location = 0) in vec3 vert_position;
#ifdef VERTEX_NORMAL
layout(location = 3) in vec3 vert_normal;
#endif
#ifdef VERTEX_COLOR
layout(location = 5) in vec4 vert_color;
#endif
layout(set = 0, binding = 0) uniform FrameUniforms {
  mat4 cameraVP;
  mat4 cameraView;
  mat4 cameraP;
  vec4 cameraPos;
} frameUniforms;
layout(set = 1, binding = 0) uniform ObjectUniforms {
  mat4 transform;
} objectUniforms;
mat4 getCameraView() {
  return frameUniforms.cameraView;
}
mat4 getCameraViewProject() {
  return frameUniforms.cameraVP;
}

layout(location = 0) out VSOutput _output;

void main() {
    VSOutput output2;
    int a = 112;
    if (a > 5) {
        _output = output2;
        return ;
    } else if (a < 2) {
        _output = output2;
        return ;
    } else {
        _output = output2;
        return ;
    }
    slot_vs(output2);
    while (true) {
        _output = output2;
        return ;
    }
    for (int i = 0; i < 10; i++) {
        _output = output2;
        return ;
    }
    _output = output2;
    return ;
}
