// https://github.com/three-rs/three/blob/master/data/shaders/basic_ps.glsl
// https://suhr.github.io/gsgt/ => tutorial will help me out in the long run
// https://learnopengl.com/Advanced-OpenGL/Blending => tutorial will help me out in the long run
#version 150 core

in vec2 v_TexCoord;
in vec4 v_Color;
out vec4 Target0;

uniform sampler2D t_Map;

void main() {
    Target0 = v_Color * texture(t_Map, v_TexCoord);
}