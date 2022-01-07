const container = document.getElementById('container');

var scene = new THREE.Scene();
var camera = new THREE.PerspectiveCamera(60, window.innerWidth / window.innerHeight, 0.1, 1000);
// var camera = new THREE.OrthographicCamera(-1, 1, 1, -1, 1, 1000);
camera.position.set(25, 2.5, 25);
var frustumSize = 69;

var renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
renderer.setSize(window.innerWidth, window.innerHeight);
renderer.setPixelRatio(window.devicePixelRatio);
renderer.shadowMap.enabled = true;
renderer.shadowMap.type = THREE.PCFSoftShadowMap; // default THREE.PCFShadowMap

container.appendChild(renderer.domElement);

var controls = new THREE.OrbitControls(camera, renderer.domElement);
var clock = new THREE.Clock();
var light = new THREE.DirectionalLight(0xffffff, 0.5);
light.position.setScalar(10);
scene.add(light);
scene.add(new THREE.AmbientLight(0xffffff, 0.5));

const axesColor = 0x5ca4a9;
var color = 0xd69dff;
const length = 10;
var func = new Function('return new Complex()');

var resolution = new THREE.Vector2(window.innerWidth, window.innerHeight);
var graph = new THREE.Object3D();
scene.add(graph);

init();
render();

function makeLine(geo, color, lineWidth = 10, opacity = 1) {
    var g = new MeshLine();
    g.setGeometry(geo);

    var material = new MeshLineMaterial({
        useMap: false,
        color: color,
        opacity: opacity,
        resolution: resolution,
        sizeAttenuation: false,
        lineWidth: lineWidth
    });
    var mesh = new THREE.Mesh(g.geometry, material);
    graph.add(mesh);
}

function init() {
    load();
}

function createGrid() {
    for (var i = -length; i <= length; i++) {
        var line = new THREE.Geometry();

        let width = 5;
        if (i == 0) width = 20;

        line.vertices.push(new THREE.Vector3(i, 0, -length));
        line.vertices.push(new THREE.Vector3(i, 0, length));
        makeLine(line, axesColor, width);

        var line = new THREE.Geometry();
        line.vertices.push(new THREE.Vector3(-length, 0, i));
        line.vertices.push(new THREE.Vector3(length, 0, i));
        makeLine(line, axesColor, width);
    }
}
function createMesh() {
    var width = 2 * length; // 20
    var height = width;
    var multiplier = 10;
    var segments = multiplier * width;
    var plane = new THREE.PlaneBufferGeometry(width, height, segments, segments);

    var colors = [];
    for (var i = 0; i < plane.attributes.position.count; i++) {
        let im = (i % (segments + 1)) - segments / 2;
        im /= multiplier;
        let re = (i - (i % (segments + 1))) / (segments + 1) - segments / 2;
        re /= multiplier;
        let input = new Complex(re, im);
        let output = func(input);
        plane.attributes.position.setZ(i, output.re);
        let arg = output.arg() / Math.PI / 2;
        if (arg < 0) arg += 1;
        let color = HSVtoRGB(arg, 0.6, 1);
        // console.log('%ccolor', `background:rgb(${color.r},${color.g},${color.b})`);
        // colors.push(color.r / 255);
        // colors.push(color.g / 255);
        // colors.push(color.b / 255);
        colors.push(sig(im));
        colors.push(sig(im));
        colors.push(sig(im));
    }

    plane.setAttribute('color', new THREE.BufferAttribute(new Float32Array(colors), 3));

    plane.computeVertexNormals();
    var mesh = new THREE.Mesh(
        plane,
        new THREE.MeshStandardMaterial({
            vertexColors: THREE.VertexColors,
            side: THREE.DoubleSide
        })
    );
    mesh.rotation.x = -Math.PI / 2;
    graph.add(mesh);
}
function createText() {
    const loader = new THREE.FontLoader();

    loader.load('fonts/CMU.json', function (font) {
        let options = {
            font: font,
            size: 3,
            height: 0,
            curveSegments: 12,
            bevelEnabled: false
        };

        let Imlabel = new THREE.TextGeometry('Im', options);
        materials = [
            new THREE.MeshPhongMaterial({ color: null, flatShading: true }), // front
            new THREE.MeshPhongMaterial({ color: null }) // side
        ];

        let ImMesh = new THREE.Mesh(Imlabel, materials);
        ImMesh.position.x = length;
        ImMesh.rotation.x = 3.14159 / 2;
        ImMesh.rotation.y = 3.14159;
        ImMesh.rotation.z = 3.14159 / 2;

        graph.add(ImMesh);

        let Relabel = new THREE.TextGeometry('Re', options);
        materials = [
            new THREE.MeshPhongMaterial({ color: null, flatShading: true }), // front
            new THREE.MeshPhongMaterial({ color: null }) // side
        ];
        let ReMesh = new THREE.Mesh(Relabel, materials);
        ReMesh.position.z = length;
        ReMesh.rotation.x = 3.14159 / 2;
        ReMesh.rotation.y = 3.14159;
        ReMesh.rotation.z = 3.14159 / 2;

        graph.add(ReMesh);
    });
}
function fn(input) {
    return func(input);
}
function load() {
    var inputBox = document.getElementById('fn');
    var errPara = document.getElementById('err');

    try {
        func = Function('input', inputBox.value);
        errPara.innerText = '';
        clear();
        createGrid();
        createMesh();
        createText();
    } catch (e) {
        func = Function('return new Complex()');
    }
}
function throwInvalidJS() {
    var errPara = document.getElementById('err');
    errPara.innerText = 'Invalid JS';
}
onWindowResize();

function onWindowResize() {
    camera.aspect = window.innerWidth / window.innerHeight;
    camera.updateProjectionMatrix();

    renderer.setSize(window.innerWidth, window.innerHeight);
}

window.addEventListener('resize', onWindowResize);

function render() {
    requestAnimationFrame(render);
    controls.update();
    graph.rotation.y += 0.1 * clock.getDelta();
    renderer.render(scene, camera);
}
function HSVtoRGB(h, s, v) {
    var r, g, b, i, f, p, q, t;
    if (arguments.length === 1) {
        (s = h.s), (v = h.v), (h = h.h);
    }
    i = Math.floor(h * 6);
    f = h * 6 - i;
    p = v * (1 - s);
    q = v * (1 - f * s);
    t = v * (1 - (1 - f) * s);
    switch (i % 6) {
        case 0:
            (r = v), (g = t), (b = p);
            break;
        case 1:
            (r = q), (g = v), (b = p);
            break;
        case 2:
            (r = p), (g = v), (b = t);
            break;
        case 3:
            (r = p), (g = q), (b = v);
            break;
        case 4:
            (r = t), (g = p), (b = v);
            break;
        case 5:
            (r = v), (g = p), (b = q);
            break;
    }

    r = Math.round(r * 255);
    g = Math.round(g * 255);
    b = Math.round(b * 255);
    return { r, g, b };
}
// sigmoid
function sig(x) {
    return 1 / (1 + Math.pow(Math.E, -x));
}
function clear() {
    for (var i = graph.children.length - 1; i >= 0; i--) {
        graph.remove(graph.children[i]);
    }
}
