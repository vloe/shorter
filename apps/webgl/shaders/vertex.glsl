attribute vec2 position;
attribute vec3 color;
varying vec3 vColor;
uniform float u_time;
uniform vec2 u_cursor;

void main() {
    vec2 pos = position;
    float t = pos.y;
    float y_offset = -0.5 + t * 0.5;
    float amplitude = 0.19 - t * 0.25;
    float phase = t * 3.14159 * 1.7;
    float frequency = 2.0 + t * 0.8;

    pos.y = amplitude * sin((pos.x * frequency * 6.28318) + phase) + y_offset;

    float baseFrequency = 6.0;
    float baseAmplitude = 0.05;
    float baseWave = sin(pos.x * baseFrequency + u_time * 0.4) * baseAmplitude;
    
    // transform cursor position to match the wave's coordinate space
    float angle = radians(40.0);
    vec2 transformedCursor = u_cursor;
    transformedCursor = (transformedCursor - vec2(0.8, 0.5)) / vec2(0.6, -0.6) / 1.5;
    transformedCursor = vec2(
        transformedCursor.x * cos(-angle) - transformedCursor.y * sin(-angle),
        transformedCursor.x * sin(-angle) + transformedCursor.y * cos(-angle)
    );
    
    float distanceToCursor = distance(pos, transformedCursor);
    float cursorInfluence = smoothstep(0.4, 0.0, distanceToCursor);
    
    float modifiedFrequency = mix(baseFrequency, baseFrequency * 1.1, cursorInfluence);
    float modifiedAmplitude = mix(baseAmplitude, baseAmplitude * 1.1, cursorInfluence);
    float modifiedPhase = cursorInfluence * sin(u_time * 2.0) * 1.2;
    
    float modifiedWave = sin(pos.x * modifiedFrequency + u_time * 0.5 + modifiedPhase) * modifiedAmplitude;
    
    float finalWave = mix(baseWave, modifiedWave, cursorInfluence);
    
    pos.y += finalWave;

    // apply transformation to rotate and position the wave
    vec2 rotatedPos;
    rotatedPos.x = pos.x * cos(angle) - pos.y * sin(angle);
    rotatedPos.y = pos.x * sin(angle) + pos.y * cos(angle);

    // scale and position
    rotatedPos = rotatedPos * 1.5;
    rotatedPos.x = rotatedPos.x * 0.6 + 0.75;
    rotatedPos.y = -rotatedPos.y * 0.6 + 0.5;

    gl_Position = vec4(rotatedPos * 2.0 - 1.0, 0.0, 1.0);
    
    vColor = mix(color, vec3(0.2, 0.4, 1.0), cursorInfluence * 0.3);
}
