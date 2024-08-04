attribute vec2 position;
attribute vec3 color;
varying vec3 vColor;
uniform float u_time;
uniform vec2 u_cursor;
uniform float u_scroll;

void main() {
    vec2 pos = position;
    float t = pos.y;
    float y_offset = -0.5 + t * 0.5; // Reduced offset range
    float amplitude = 0.2 - t * 0.25; // Reduced amplitude
    float phase = t * 3.14159 * 1.7; // Increased phase multiplier
    float frequency = 2.0 + t * 0.5; // Increased base frequency

    pos.y = amplitude * sin((pos.x * frequency * 6.28318) + phase) + y_offset;

    float baseFrequency = 6.0;
    float baseAmplitude = 0.08;
    float baseWave = sin(pos.x * baseFrequency + u_time * 0.5) * baseAmplitude;
    
    float scrollEffect = u_scroll * 0.1;
    float scrollWave = sin(pos.x * 3.0 - u_time * 0.3) * scrollEffect;
    
    float distanceToCursor = distance(pos, u_cursor);
    float cursorInfluence = smoothstep(0.4, 0.0, distanceToCursor);
    
    float modifiedFrequency = mix(baseFrequency, baseFrequency * 1.3, cursorInfluence);
    float modifiedAmplitude = mix(baseAmplitude, baseAmplitude * 1.3, cursorInfluence);
    float modifiedPhase = cursorInfluence * sin(u_time * 2.0) * 1.5;
    
    float modifiedWave = sin(pos.x * modifiedFrequency + u_time * 0.5 + modifiedPhase) * modifiedAmplitude;
    
    float finalWave = mix(baseWave, modifiedWave, cursorInfluence) + scrollWave;
    
    pos.y += finalWave;

    // Apply transformation to rotate and position the wave
    float angle = radians(40.0); // Slightly reduced angle
    vec2 rotatedPos;
    rotatedPos.x = pos.x * cos(angle) - pos.y * sin(angle);
    rotatedPos.y = pos.x * sin(angle) + pos.y * cos(angle);

    // Scale and position
    rotatedPos = rotatedPos * 1.5; // Increase scale to make it longer
    rotatedPos.x = rotatedPos.x * 0.6 + 0.8; // Adjust horizontal position
    rotatedPos.y = -rotatedPos.y * 0.6 + 0.5; // Adjust vertical position

    gl_Position = vec4(rotatedPos * 2.0 - 1.0, 0.0, 1.0);
    
    vColor = mix(color, vec3(0.2, 0.4, 1.0), cursorInfluence * 0.5); // Adjusted color to be more blue
}
