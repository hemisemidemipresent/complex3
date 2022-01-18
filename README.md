# complex3

[A 3D web-based complex function grapher](https://hemisemidemipresent.github.io/complex3/)

![](https://media.discordapp.net/attachments/699781597515481159/932949769142288474/unknown.png?width=600&height=300)

> example of a Re-Im plot

![](https://media.discordapp.net/attachments/699781597515481159/932951742486818866/unknown.png?width=600&height=300)

> example of a Mod-Arg plot

Uses [Math.js](https://mathjs.org/) for expression parsing and evaluation and [three.js](https://threejs.org/) for visualising functions

## Features

-   ### Expandable function input box

    ![](https://media.discordapp.net/attachments/699781597515481159/932952744669618196/unknown.png?width=532&height=57)

    ![](https://media.discordapp.net/attachments/699781597515481159/932953635560775680/unknown.png)

    > for this to update you have to press the Load button

-   ### Different types of Plots

    ![](https://media.discordapp.net/attachments/699781597515481159/932953830725922816/unknown.png)

    > for this to update you have to press the Load button

-   ### (Rough) control over how many vertices loaded

    ![](https://media.discordapp.net/attachments/699781597515481159/932954030588690482/unknown.png)

    > for this to update you have to press the Load button

-   ### Autorotate on/off

    ![](https://media.discordapp.net/attachments/699781597515481159/932961738108850216/unknown.png)

-   ### Glassy/Shiny surface on/off

    ![](https://media.discordapp.net/attachments/699781597515481159/932962790711365642/unknown.png)

    When shine is on:

    ![](https://media.discordapp.net/attachments/699781597515481159/932962092514967592/unknown.png?width=600&height=274)

    > Note: This might not accurately represent how real life translucent shiny surfaces work, and there may be issues on devices with less great GPUs

-   ### If you dont like B/W with Re-Im/Im-Re plots

    ![](https://media.discordapp.net/attachments/699781597515481159/932971032665554944/unknown.png)

-   ### Lower Left Buttons

    ![](https://media.discordapp.net/attachments/699781597515481159/932963119192481822/unknown.png)

    the book icon hides/shows the topleft tab

    the camera icon **exports the graph as a .png**

## Purpose

I saw cool stuff like this graph from [this youtube video](https://www.youtube.com/watch?v=3qEJeP6qQGA):

![](https://media.discordapp.net/attachments/699781597515481159/932950259175424020/unknown.png?width=600&height=317)

but there is no easy way to render such Mod-Arg plots. So the objective of this is to create an optimized way to graph 3D complex plots **on the web**.

## TODO

-   [ ] Logarithmic height option
-   [ ] cut off after certain height option
-   [ ] Export to a `.stl` file 3D printing (probably defer [this](https://github.com/eligrey/FileSaver.js/) or smth)
-   [ ] Opacity, Saturation, Value option

### Done

-   [x] Hide title option
-   [x] Add an option for how many points u want in a mesh (aka how good is your device)
-   [x] Mod-Arg, Im-Re plot
-   [x] Export to an image - ~~compromised performance by adding `preserveDrawingBuffer: true` to renderer~~ re-renders with `preserveDrawingBuffer: true`, exports image, and re-render with `preserveDrawingBuffer: false`
-   [x] Optimize reloading
-   [x] shiny/glassy surface option

## What are Re-Im/Im-Re/Mod-Arg plots?

for a complex-valued function, the complex input consists of 2 values (Real and Imaginary part i.e. `x+iy`) while the complex output similarly also consists of 2 values. This would require a 4-dimensional graph that we cannot visualize, however, so instead, we plot 3 values and leave the 4th one to color

for example for the given function f(x+iy) = w+iv

-   a Re-Im plot takes the real component of the output `w` and uses that as the height of the surface at a given point. the imaginary part `v` is sigmoid-ed to B/W value
-   a Im-Re plot, on the other hand takes the imaginary component of the output `v` and uses that as the height of the surface at a given point. the real part `w` is sigmoid-ed to B/W value
-   a Mod-Arg plot takes the **modulus** as the height and the **argument**:

![](https://media.discordapp.net/attachments/699781597515481159/932965860870586478/unknown.png)

## examples

note: for functions with asymptotes/large changes in gradient it is necessary to increase the "How good is your device" to a value higher, perhaps 10 but 25 is better

![](https://media.discordapp.net/attachments/699781597515481159/932969004719562772/unknown.png?width=600&height=300)

> tan(z), Mod-Arg

![](https://media.discordapp.net/attachments/699781597515481159/932970020332527616/unknown.png?width=600&height=300)

> gamma(z), Mod-Arg

![](https://media.discordapp.net/attachments/699781597515481159/932970810426163231/unknown.png?width=600&height=300)

> sec(z), Re-Im, colored

![](https://media.discordapp.net/attachments/699781597515481159/932972496720904252/unknown.png?width=600&height=300)

> sinh(z), Mod-Arg
