"use client";

import { useEffect, useRef } from "react";
import { BACKGROUND_COLOR } from "@/app/theme";

export function AnimatedBackground() {
    const canvasRef = useRef<HTMLCanvasElement>(null);

    useEffect(() => {
        const canvas = canvasRef.current;
        if (!canvas) return;

        const gl = canvas.getContext("webgl");
        if (!gl) {
            console.warn("WebGL not supported, falling back to static background");
            return;
        }

        // Set canvas size
        const resizeCanvas = () => {
            canvas.width = window.innerWidth;
            canvas.height = window.innerHeight;
            gl.viewport(0, 0, canvas.width, canvas.height);
        };
        resizeCanvas();
        window.addEventListener("resize", resizeCanvas);

        // Vertex shader
        const vertexShaderSource = /*glsl*/`
            attribute vec2 position;
            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        `;

        // Fragment shader for metaballs
        const fragmentShaderSource = /*glsl*/`
            precision highp float;
            uniform vec2 resolution;
            uniform float time;
            
            // Theme colors (from theme.ts: accent #ff5c8a, background #0d0410)
            const vec3 accent = vec3(1.0, 0.361, 0.541); // accent pink
            const vec3 purple = vec3(0.6, 0.3, 0.9); 
            const vec3 blue = vec3(0.3, 0.5, 1.0); 
            const vec3 orange = vec3(1.0, 0.5, 0.2); 
            const vec3 dark = vec3(0.051, 0.016, 0.063); // background color
            
            // Metaball function
            float metaball(vec2 p, vec2 center, float radius) {
                float d = length(p - center);
                return radius / (d * d + 0.1);
            }
            
            void main() {
                vec2 uv = gl_FragCoord.xy / resolution.xy;
                vec2 p = uv * 2.0 - 1.0;
                p.x *= resolution.x / resolution.y;
                
                float t = time * 0.05; // very slow movement
                
                // Create 6 metaballs with smooth movement
                float m1, m2, m3, m4, m5, m6;
                
                // Ball 1 - circular motion (pink)
                vec2 pos1 = vec2(sin(t * 0.7) * 0.6, cos(t * 0.5) * 0.5);
                m1 = metaball(p, pos1, 0.25);
                
                // Ball 2 - figure-8 motion (purple)
                vec2 pos2 = vec2(sin(t * 0.5) * 0.7, sin(t * 1.0) * 0.6);
                m2 = metaball(p, pos2, 0.23);
                
                // Ball 3 - elliptical (blue)
                vec2 pos3 = vec2(cos(t * 0.6 + 1.0) * 0.8, sin(t * 0.4 + 1.0) * 0.4);
                m3 = metaball(p, pos3, 0.22);
                
                // Ball 4 - slow drift (orange)
                vec2 pos4 = vec2(sin(t * 0.3 + 2.0) * 0.5, cos(t * 0.35 + 2.0) * 0.7);
                m4 = metaball(p, pos4, 0.2);
                
                // Ball 5 - counter-clockwise (pink-purple mix)
                vec2 pos5 = vec2(cos(t * 0.45 + 4.0) * 0.6, sin(t * 0.55 + 4.0) * 0.55);
                m5 = metaball(p, pos5, 0.21);
                
                // Ball 6 - diagonal (blue-orange mix)
                vec2 pos6 = vec2(sin(t * 0.4 + 3.0) * 0.65, cos(t * 0.48 + 3.0) * 0.45);
                m6 = metaball(p, pos6, 0.2);
                
                // Combine metaballs with different colors
                float m = (m1 + m2 + m3 + m4 + m5 + m6) * 0.1; // lower intensity
                
                // Color mixing - very subtle gradients
                vec3 color = dark;
                
                // Add colored glows very subtly
                color += accent * m1 * 0.04;
                color += purple * m2 * 0.04;
                color += blue * m3 * 0.04;
                color += orange * m4 * 0.04;
                color += mix(accent, purple, 0.5) * m5 * 0.04;
                color += mix(blue, orange, 0.5) * m6 * 0.04;
                
                // Minimal ambient glow
                float glow = smoothstep(0.3, 1.5, m);
                color += accent * glow * 0.015;
                
                // Very subtle vignette
                float vignette = 1.0 - length(uv - 0.5) * 0.2;
                color *= vignette;
                
                // Keep it very dark and subtle
                color = mix(dark, color, 0.6);
                
                gl_FragColor = vec4(color, 1.0);
            }
        `;

        // Compile shader
        function compileShader(source: string, type: number) {
            if (!gl) return null;
            const shader = gl.createShader(type);
            if (!shader) return null;
            gl.shaderSource(shader, source);
            gl.compileShader(shader);
            if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
                console.error("Shader compilation error:", gl.getShaderInfoLog(shader));
                gl.deleteShader(shader);
                return null;
            }
            return shader;
        }

        const vertexShader = compileShader(vertexShaderSource, gl.VERTEX_SHADER);
        const fragmentShader = compileShader(fragmentShaderSource, gl.FRAGMENT_SHADER);

        if (!vertexShader || !fragmentShader) return;

        // Create program
        const program = gl.createProgram();
        if (!program) return;

        gl.attachShader(program, vertexShader);
        gl.attachShader(program, fragmentShader);
        gl.linkProgram(program);

        if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
            console.error("Program linking error:", gl.getProgramInfoLog(program));
            return;
        }

        gl.useProgram(program);

        // Create fullscreen quad
        const positions = new Float32Array([
            -1, -1,
            1, -1,
            -1, 1,
            1, 1,
        ]);

        const buffer = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
        gl.bufferData(gl.ARRAY_BUFFER, positions, gl.STATIC_DRAW);

        const positionLocation = gl.getAttribLocation(program, "position");
        gl.enableVertexAttribArray(positionLocation);
        gl.vertexAttribPointer(positionLocation, 2, gl.FLOAT, false, 0, 0);

        // Get uniform locations
        const resolutionLocation = gl.getUniformLocation(program, "resolution");
        const timeLocation = gl.getUniformLocation(program, "time");

        // Animation loop
        let animationFrameId: number;
        const startTime = Date.now();

        function render() {
            if (!gl || !canvas) return;

            const time = (Date.now() - startTime) / 1000;

            gl.uniform2f(resolutionLocation, canvas.width, canvas.height);
            gl.uniform1f(timeLocation, time);

            gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);

            animationFrameId = requestAnimationFrame(render);
        }

        render();

        return () => {
            window.removeEventListener("resize", resizeCanvas);
            cancelAnimationFrame(animationFrameId);
            if (gl) {
                gl.deleteShader(vertexShader);
                gl.deleteShader(fragmentShader);
                gl.deleteProgram(program);
                gl.deleteBuffer(buffer);
            }
        };
    }, []);

    return (
        <canvas
            ref={canvasRef}
            className="fixed inset-0 w-full h-full"
            style={{
                background: BACKGROUND_COLOR,
                zIndex: -10,
                pointerEvents: "none"
            }}
        />
    );
}
