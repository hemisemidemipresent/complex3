const container = document.getElementById('container');

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(60, window.innerWidth / window.innerHeight, 0.1, 1000);
camera.position.set(25, 2.5, 25);
const frustumSize = 69;

const renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
renderer.setSize(window.innerWidth, window.innerHeight);
renderer.setPixelRatio(window.devicePixelRatio);
renderer.shadowMap.enabled = true;
renderer.shadowMap.type = THREE.PCFSoftShadowMap; // default THREE.PCFShadowMap

container.appendChild(renderer.domElement);

const controls = new THREE.OrbitControls(camera, renderer.domElement);
const clock = new THREE.Clock();
const light = new THREE.DirectionalLight(0xffffff, 0.5);
light.position.setScalar(10);
scene.add(light);
scene.add(new THREE.AmbientLight(0xffffff, 0.5));

const axesColor = 0x5ca4a9;
const color = 0xd69dff;
const length = 10;
let multiplier = 10;
let isModArg = false;
let isRotate = true;

let resolution = new THREE.Vector2(window.innerWidth, window.innerHeight);
let graph = new THREE.Object3D();
let grid = new THREE.Object3D();
grid.name = grid;
scene.add(graph);
createText();
createGrid();
load();
render();
onWindowResize();
window.addEventListener('resize', onWindowResize);

function makeLine(geo, color, lineWidth = 10, opacity = 1) {
    const g = new MeshLine();
    g.setGeometry(geo);

    const material = new MeshLineMaterial({
        useMap: false,
        color: color,
        opacity: opacity,
        resolution: resolution,
        sizeAttenuation: false,
        lineWidth: lineWidth
    });
    const mesh = new THREE.Mesh(g.geometry, material);
    grid.add(mesh);
}

function createGrid() {
    for (let i = -length; i <= length; i++) {
        let line = new THREE.Geometry();

        let width = 5;
        if (i == 0) width = 20;

        line.vertices.push(new THREE.Vector3(i, 0, -length));
        line.vertices.push(new THREE.Vector3(i, 0, length));
        makeLine(line, axesColor, width);

        line = new THREE.Geometry();
        line.vertices.push(new THREE.Vector3(-length, 0, i));
        line.vertices.push(new THREE.Vector3(length, 0, i));
        makeLine(line, axesColor, width);
    }
    graph.add(grid);
}

function createMesh() {
    let width = 2 * length; // 20
    let height = width;
    let segments = multiplier * width;
    let plane = new THREE.PlaneBufferGeometry(width, height, segments, segments);

    let colors = [];
    for (let i = 0; i < plane.attributes.position.count; i++) {
        let im = (i % (segments + 1)) - segments / 2;
        im /= multiplier;
        let re = (i - (i % (segments + 1))) / (segments + 1) - segments / 2;
        re /= multiplier;
        let input = new Complex(re, im);
        let output = func(input);

        let legend = document.getElementById('legend');
        if (!isModArg) {
            plane.attributes.position.setZ(i, output.re);
            let sigmoid_im = sig(im);
            colors.push(sigmoid_im, sigmoid_im, sigmoid_im);
            legend.innerHTML =
                'height of surface = real part of output<br>color of surface - white = bigger Im, black = smaller Im';
        } else {
            plane.attributes.position.setZ(i, output.abs());
            let arg = output.arg() / Math.PI / 2;
            if (arg < 0) arg += 1;
            let color = HSVtoRGB(arg, 0.6, 1);
            colors.push(color.r / 255);
            colors.push(color.g / 255);
            colors.push(color.b / 255);
            legend.innerHTML =
                'height of surface = modulus of output<br>color of surface - argument of output (R→G→B)';
        }
    }

    plane.setAttribute('color', new THREE.BufferAttribute(new Float32Array(colors), 3));

    plane.computeVertexNormals();
    let mesh = new THREE.Mesh(
        plane,
        new THREE.MeshLambertMaterial({
            vertexColors: THREE.VertexColors,
            side: THREE.DoubleSide
        })
    );
    mesh.rotation.x = -Math.PI / 2;
    mesh.name = 'mesh';
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

function load() {
    let inputBox = document.getElementById('fn');
    let errPara = document.getElementById('err');
    let checkBox = document.getElementById('checkbox');
    isModArg = checkBox.checked;
    try {
        func = Function('input', inputBox.value);
        errPara.innerText = '';
        clearMesh();
        createMesh();
    } catch (e) {
        func = Function('return new Complex()');
        console.log(e);
    }
}

function throwInvalidJS() {
    var errPara = document.getElementById('err');
    errPara.innerText = 'Invalid JS';
}

function onWindowResize() {
    camera.aspect = window.innerWidth / window.innerHeight;
    camera.updateProjectionMatrix();

    renderer.setSize(window.innerWidth, window.innerHeight);
}

function render() {
    requestAnimationFrame(render);
    controls.update();
    if (isRotate) {
        graph.rotation.y += 0.05 * clock.getDelta();
    }
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

function clearMesh() {
    for (let i = 0; i < graph.children.length; i++) {
        if (graph.children[i].name == 'mesh') graph.remove(graph.children[i]);
    }
}
