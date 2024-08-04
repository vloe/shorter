attribute vec2 position;
attribute vec3 color;
varying vec3 vColor;
uniform float u_time;
uniform vec2 u_cursor;
uniform float u_scroll;

void main() {
    vec2 pos = position;
    float t = pos.y;
    float y_offset = -0.4 + t;
    float amplitude = 0.4 - t * 0.3;
    float phase = t * 3.14159 * 1.5;
    float frequency = 1.0 + t * 0.2;

    pos.y = amplitude * sin((pos.x * frequency * 6.28318) + phase) + y_offset;

    float baseFrequency = 6.0;
    float baseAmplitude = 0.08;
    float baseWave = sin(pos.x * baseFrequency + u_time * 0.5) * baseAmplitude;
    
    float scrollEffect = u_scroll * 0.2;
    float scrollWave = sin(pos.x * 3.0 - u_time * 0.3) * scrollEffect;
    
    float distanceToCursor = distance(pos, u_cursor);
    float cursorInfluence = smoothstep(0.4, 0.0, distanceToCursor);
    
    float modifiedFrequency = mix(baseFrequency, baseFrequency * 1.3, cursorInfluence);
    float modifiedAmplitude = mix(baseAmplitude, baseAmplitude * 1.3, cursorInfluence);
    float modifiedPhase = cursorInfluence * sin(u_time * 2.0) * 1.5;
    
    float modifiedWave = sin(pos.x * modifiedFrequency + u_time * 0.5 + modifiedPhase) * modifiedAmplitude;
    
    float finalWave = mix(baseWave, modifiedWave, cursorInfluence) + scrollWave;
    
    pos.y += finalWave;
    gl_Position = vec4(pos.x, pos.y, 0.0, 1.0);
    
    vColor = mix(color, vec3(1.0, 0.5, 0.2), cursorInfluence * 0.5);
}
