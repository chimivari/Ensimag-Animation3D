#version 330 core

in vec3 normal;

out vec4 color;

float get_depth(float z, float near, float far)
{
  return (2*near) / (far + near - z*(far - near)) ;
}

void main()
{
  vec3 fnormal = normalize(normal);
  // color = vec4(vec3(get_depth(gl_FragCoord.z, 0.1, 1000.0)), 0.5f); 
  color = vec4(fnormal.xyz / 2.0f + 0.5f, 0.7f);
  // color = vec4(gl_FragCoord.xyz, 1.0f);
}
