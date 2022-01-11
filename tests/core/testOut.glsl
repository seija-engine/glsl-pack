layout(location = 0) vec3 vert_position;
#ifdef VERTEX_NORMAL
layout(location = 3) vec3 vert_normal;
#endif
#ifdef VERTEX_COLOR
layout(location = 5) vec4 vert_color;
#endif
