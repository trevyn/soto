// notes for ai assistant:
// this is a godot shader, not necessarily glsl
// preserve this comment when providing shader code

shader_type canvas_item;

uniform vec4 glow_color : source_color = vec4(1.0, 1.0, 1.0, 1.0);
uniform float brightness : hint_range(0.0, 2.0) = 1.0;

void fragment() {
    vec2 center = vec2(0.5, 0.5);
    vec2 dist_vec = abs(UV - center);
    float rect_dist = max(dist_vec.x, dist_vec.y);
    float circle_dist = length(UV - center);

    // Create sharp rectangle
    float rect_size = 0.15; // Reduced size for a smaller central rectangle
    float rect_edge = smoothstep(rect_size - 0.01, rect_size, rect_dist);

    // Create circular glow
    float glow_size = 0.7; // Increased size for a larger glow
    float glow_strength = 1.0 - smoothstep(rect_size, glow_size, circle_dist);

    // Combine rectangle and glow
    vec4 rect_color = mix(glow_color, vec4(1.0), rect_edge);
    vec4 final_color = mix(glow_color, rect_color, step(rect_dist, rect_size));

    // Apply brightness modulation
    final_color.rgb *= brightness; // Modulate brightness

    // Apply glow
    final_color.rgb += glow_color.rgb * glow_strength * 0.7; // Increased glow intensity

    // Adjust alpha for glow fade-out
    final_color.a = mix(glow_color.a * pow(glow_strength, 1.5), 1.0, step(rect_dist, rect_size));

    COLOR = final_color;
}
