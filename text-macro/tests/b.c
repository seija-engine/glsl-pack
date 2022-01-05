#ifdef HAS_LIGHT
  float has_light = PI;
#endif

#ifdef HAS_NORMAL
#define HAS_LIGHT
  int has_normal = 666;
#endif

#ifdef HAS_TEXTURE
  #define HAS_NORMAL
  int has_texture = 666;
#endif