#version 450

out vec4 outColor;

precision mediump float;

uniform float iTime;

void main() {
	outColor = vec4(abs(sin(iTime)),0.0,0.0,1.0);
}
