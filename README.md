# complex3

[A 3D web-based complex function grapher](https://hemisemidemipresent.github.io/complex3/)

Uses [math.js](https://mathjs.org/) and [three.js](https://threejs.org/)

## TODO

-   [ ] Logarithmic height option
-   [ ] Export to a `.stl` file 3D printing (probably defer [this](https://github.com/eligrey/FileSaver.js/) or smth)

### Done

-   [x] Hide title option
-   [x] Add an option for how many points u want in a mesh (aka how good is your device)
-   [x] Mod-Arg, Im-Re plot
-   [x] Export to an image - ~~compromised performance by adding `preserveDrawingBuffer: true` to renderer~~ re-renders with `preserveDrawingBuffer: true`, exports image, and re-render with `preserveDrawingBuffer: false`
-   [x] Optimize reloading
-   [x] shiny surface option

## History

you see cool stuff like [this](https://www.youtube.com/watch?v=3qEJeP6qQGA) but those need to install stuff to run and take forever to render. So the objective of this is to create an optimized way to graph 3D complex plots on the web.

Many tradeoffs were made, such as abandoning the Complex.js library for the Math.js one as that had expression parsing (ease of use)
