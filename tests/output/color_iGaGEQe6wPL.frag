#version 450
layout(location = 0) out vec4 _outColor;
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
struct VSOutput {
    vec3 color;
    vec3 normal;
    vec2 uv;
};
layout(location = 0) in VSOutput _input;

void main() {
    VSOutput vo = _input;
    vec4 color = vec4(1);
    if (vo.uv.x > 0) {
        _outColor = vec4(1);
        return ;
    }
    _outColor = color;
    return ;
}
