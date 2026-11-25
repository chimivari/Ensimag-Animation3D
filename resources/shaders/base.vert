#version 330 core
layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 normal;

uniform mat4 world;
uniform mat4 view;
uniform mat4 projection;

out vec3 vNormal;

void main()
{
    gl_Position = projection * view * world * vec4(pos, 1.0);
    vNormal = normal;
}
