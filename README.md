# glsl-pack
glsl pack给glsl提供了包的概念，并且提供了以下功能。  

1. 加入了`use`语法。
可以在文件头部直接`use core.math;`来引入其他文件。

2. 支持了预处理的文本宏。  
可以使用 `#ifdef` `#elif` `#else` `#endif`宏。  


3. 可以类似通过如下配置生成最终的glsl文件。  
  生成的最终glsl文件只会引入使用过的变量和函数
```json
{
    "name":"core",

    "shaders": [
        {
            "name":"color",
            "vertex":{
                "POSITION" : "require",
                "COLOR"    : "option",
                "NORMAL"   : "option"
            },
            "backend":["Camera","Transform3D"],
            "slots":["material_vertex","material_frag"],
            "vs":"color.vs_main",
            "fs":"color.fs_main"
        }
    ]
}

```