#version 300 es
precision mediump float;

in highp vec3 texCoord;
uniform samplerCube uDayTex;
uniform samplerCube uNightTex;
uniform float uTime;

out vec4 finalColor;
void main(void){
    finalColor = mix( texture(uDayTex, texCoord), texture(uNightTex, texCoord), abs(sin(uTime * 0.0005)) );
}