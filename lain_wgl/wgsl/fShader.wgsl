#version 300 es
precision mediump float;

in highp vec2 texCoord;		//What pixel to pull from the texture
uniform sampler2D uMainTex;	//Holds the texture we loaded to the GPU

out vec4 finalColor;
void main(void){
    finalColor = texture(uMainTex,texCoord);
}