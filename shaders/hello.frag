precision mediump float;

uniform float iTime;

void main(){
	gl_FragColor=vec4(abs(sin(iTime)),0.,0.,1.);
}
