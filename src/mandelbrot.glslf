#version 150 core

// https://www.khronos.org/opengl/wiki/GLSL_Uniform#Uniform_management
// Parameter for all invocations of the shader
uniform int maxIterations;
uniform float windowHight;
uniform vec4 baseColor = vec4(1.0, 1.0, 1.0, 1.0);
out vec4 color;

/* The value in gl_FragCoord is the actual pixel position
 * (e.g. between 0 and 767 since our window has the height of 768).
 * The point of origin (0, 0) is the bottom-left corner (y is going up,
 * x is going right).
 */
void main() {
    vec2 c = gl_FragCoord.xy / windowHight * 4.0 - 2.0;
    vec2 z = c;
    int iterationen = 0;
    while (iterationen < maxIterations && length(z) < 2.0) {
        z = vec2(pow(z.x, 2) - pow(z.y, 2), 2 * z.x * z.y) + c;
        iterationen++;
    }

    // Color calculation
    // Copied from my Java implementation of the mandelbrot set.
    // Back then I made comments in German and I'm to lazy to translate them.


    /**Die Farbe wird berechnet mit dem HSV-Farbkreis.<br></br>
     * Der HSV-Farbkreis wird sozusagen
     * einmal rundumgegangen mit den ersten 15 Iterationen. Für die
     * zweite Runde werden 31 Iterationen benötigt.<br></br>
     * Die benötigten Iterationen fürs rundumgehen wachsen
     * exponentiell, da die Iterationen exponentiell gegen unendlich gehen,
     * wenn man sich der Grenzlinie nähert.<br></br>
     * Die HSV-Werte werden noch in RGB umgewandelt.

     * @param iterationen Die Anzahl der Iterationen, bis der Punkt abgehaut ist.
     * @param maxIterationen Wenn die Iteration größergleich maxIterationen
     * ist, wird die Grundfarbe verwendet.
     * @param grundfarbe Die Farbe, die der Mittelteil bekommt.
     * @return Die Farbe als Int im argb-Format
     */

    if (iterationen == maxIterations)
        color = baseColor;
    else {
        /* Exponentielles Verhalten. Der HSV-Farbkreis wird sozusagen
         * einmal rundumgegangen mit den ersten 15 Iterationen. Für die
         * zweite Runde werden 31 Iterationen benötigt, für die Dritte
         * noch mal doppelt so viele. */
        iterationen += 8;
        int runde = 15;
        while (iterationen >= runde) {
            // Bitmuster ist immer nur einsen, somit kein Problem mit MAX_INT >= runde.
            runde = runde * 2 + 1;
            //runde = runde * 4 + 3
        }
        /* iterationen ist nun zwischen runde/2 und runde.
         * Deshalb zuerst minus runde/2; dadurch ist es zwischen 0 und runde/2.
         * Und nur noch in den Bereich von 0 bis 1 bingen. */
        //float fraction = (iterationen - (runde / 2)) / (runde / 2);
        //fraction = (iterationen - runde / 4) / ((3*runde) / 4);

        float fraction = float(iterationen) / float(runde);

        /** Die Farbe wird berechnet mit dem HSV-Farbkreis.
         *
         * Die RGB-Werte folgen einem relativ einfachem Muster:
         * Es ist immer eine Farbe auf 255, eine auf 0 und die Dritte variabel.
         * Nach 60° ändert sich, welche. Siehe Farbauswahl bei GIMP.
         * Schritte:
         * 1. rot max,         grün 0,         blau wird mehr
         * 2. rot wird weniger,grün 0,         blau max
         * 3. rot 0,           grün wird mehr, blau max
         * 4. rot 0,           grün max,       blau wird weniger
         * 5. rot wird mehr,   grün max,       blau 0
         * 6. rot max,         grün weniger,   blau 0
         *
         * @param hue Die Rotation im Farbkreis. Im Bereich [0 bis 1]
         *      0 == 0°, 1 == 360°
         */
        float hue = fraction;
        if (hue < 0) color = baseColor;
        else if (hue < 1.0 / 6.0) color = vec4(1.0, 0.0, hue * 6, 1.0);
        else if (hue < 2.0 / 6.0) color = vec4(1 - (hue - 1.0 / 6) * 6, 0.0, 1.0, 1.0);
        else if (hue < 3.0 / 6.0) color = vec4(0.0, (hue - 2.0 / 6) * 6, 1.0, 1.0);
        else if (hue < 4.0 / 6.0) color = vec4(0.0, 1.0, 1 - (hue - 3.0 / 6) * 6, 1.0);
        else if (hue < 5.0 / 6.0) color = vec4((hue - 4.0 / 6) * 6, 1.0, 0.0, 1.0);
        else if (hue <=6.0 / 6.0) color = vec4(1.0, 1 - (hue - 5.0 / 6) * 6, 0.0, 1.0);
        else color = baseColor;
    }
}
