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

-   ### Lower Left Buttons

    ![](https://media.discordapp.net/attachments/699781597515481159/932963119192481822/unknown.png)

    the book icon hides/shows the topleft tab

    the camera icon **exports the graph as a .png**

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

## History

I saw cool stuff like this graph from [this youtube video](https://www.youtube.com/watch?v=3qEJeP6qQGA):

![](https://media.discordapp.net/attachments/699781597515481159/932950259175424020/unknown.png?width=600&height=317)

but there is no easy way to render such Mod-Arg plots. So the objective of this is to create an optimized way to graph 3D complex plots **on the web**.
