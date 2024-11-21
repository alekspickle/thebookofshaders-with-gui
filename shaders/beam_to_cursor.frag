#ifdef GL_ES
precision lowp float;
#endif

uniform vec2 u_resolution;
uniform vec2 u_mouse;
uniform float u_time;

// Plot a line on Y using a value between 0.0-1.0
float plot(vec2 st){
    vec2 m=u_mouse/u_resolution;
    
    return smoothstep(.02,0.,abs(st.y*m.x-st.x*m.y));
}

void main(){
    vec2 st=gl_FragCoord.xy/u_resolution;
    
    float y=st.x;
    
    vec3 color=vec3(y);
    
    // Plot a line
    float pct=plot(st);
    color=(1.-pct)*color+pct*vec3(0.,1.,0.);
    vec2 m=u_mouse/u_resolution;
    color=vec3(color.x+m.x,color.y+m.y,color.z);
    gl_FragColor=vec4(color,1.);
}