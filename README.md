# complex3

[A 3D web-based complex function grapher](https://hemisemidemipresent.github.io/complex3/)

<img src="https://github.com/hemisemidemipresent/complex3/blob/main/imgs/Re-Im.png" height=600/>


> example of a Re-Im plot

<img src="https://github.com/hemisemidemipresent/complex3/blob/main/imgs/Mod-Arg.png" height=600/>

> example of a Mod-Arg plot

Uses a custom rust expression parser and evaluator (in `./rust`) compiled to wasm

## Features

-   ### Expandable function input box

    > for the function to update you have to press the Load button

-   ### Different types of Plots

    <img src="https://github.com/hemisemidemipresent/complex3/blob/main/imgs/select_plot.png"/>

    > for this to update you have to press the Load button

-   ### (Rough) control over how many vertices loaded

    > for this to update you have to press the Load button

-   ### Autorotate on/off

-   ### Glassy/Shiny surface on/off

    When shine is on:

    <img src="https://github.com/hemisemidemipresent/complex3/blob/main/imgs/shine.png"/>


    > Note: This might not accurately represent how real life translucent shiny surfaces work, and there may be issues on devices with less great GPUs

-   ### Colored Re-Im/Im-Re plots

-   ### Lower Left Buttons

    the book icon hides/shows the topleft tab

    ~~the camera icon exports the graph as a .png~~

## Purpose

I saw cool stuff like this graph from [this youtube video](https://www.youtube.com/watch?v=3qEJeP6qQGA):

<img src="https://github.com/hemisemidemipresent/complex3/blob/main/imgs/youtube.png"/>

but there is no easy way to render such Mod-Arg plots. So the objective of this is to create an optimized way to graph 3D complex plots **on the web** (snappily).

## TODO

-   [ ] cut off after certain height option
-   [ ] Export to a `.stl` file 3D printing (probably defer [this](https://github.com/eligrey/FileSaver.js/) or smth)
-   [ ] Opacity, Saturation, Value option
-   [ ] Export to an image - ~~compromised performance by adding `preserveDrawingBuffer: true` to renderer~~ re-renders with `preserveDrawingBuffer: true`, exports image, and re-render with `preserveDrawingBuffer: false`


### Done

-   [x] Logarithmic height option
-   [x] Hide title option
-   [x] Add an option for how many points u want in a mesh (aka how good is your device)
-   [x] Mod-Arg, Im-Re plot
-   [x] Optimize reloading
-   [x] shiny/glassy surface option

## What are Re-Im/Im-Re/Mod-Arg plots?

For a complex-valued function, the complex input consists of 2 values (Real and Imaginary part i.e. $x+iy$) while the complex output similarly also consists of 2 values. This would require a 4-dimensional graph that we cannot visualize, however, so instead, we plot 3 values and leave the 4th one to color

For example for the given function $f(x+iy) = w+iv$

-   a Re-Im plot takes the real component of the output $w$ and uses that as the height of the surface at a given point. the imaginary part $v$ is sigmoid-ed to B/W value
-   a Im-Re plot, on the other hand takes the imaginary component of the output $v$ and uses that as the height of the surface at a given point. the real part $w$ is sigmoid-ed to B/W value
-   a Mod-Arg plot takes the **modulus** as the height and the **argument**:

<img src="https://github.com/hemisemidemipresent/complex3/blob/main/imgs/mod_arg_explanation.png"/>

## examples

note: for functions with asymptotes/large changes in gradient it is necessary to increase the "How good is your device" to a value higher, perhaps 10 but 25 is better

<img src="https://github.com/hemisemidemipresent/complex3/blob/main/imgs/tan_z.png"/>

> $\tan(z)$ (`tan(z)`), Mod-Arg

<img src="https://github.com/hemisemidemipresent/complex3/blob/main/imgs/gamma_z.png"/>

> $\Gamma(z)$ (`gamma(z)`), Mod-Arg

<img src="https://github.com/hemisemidemipresent/complex3/blob/main/imgs/sec_z.png"/>

> $\sec(z)$ (`sec(z)`), Re-Im, colored

<img src="https://github.com/hemisemidemipresent/complex3/blob/main/imgs/sinh_z.png"/>

> $\sinh(z)$ (`sinh(z)`), Mod-Arg

<img src="https://github.com/hemisemidemipresent/complex3/blob/main/imgs/asinh_z.png"/>

> $4sin^{-1}\frac{z}{4}$ (`4asin(z/4)`), Re-Im

<img src="https://github.com/hemisemidemipresent/complex3/blob/main/imgs/fn_1.png"/>

> `1/(1+(z/5)^2)`, Re-Im, colored, you can see the 2 asymptotes at z=±i that causes the radius of convergence to be 1
