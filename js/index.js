const container = document.getElementById('container');
const inputBox = document.getElementById('fn');
const errPara = document.getElementById('err');
const plotChooser = document.getElementById('plotChooser');
const rotBtn = document.getElementById('rotBtn');
const shineBtn = document.getElementById('shineBtn');
const bwBtn = document.getElementById('bwBtn');

const saturation = 0.75;

const scene = new THREE.Scene();
let camera = new THREE.PerspectiveCamera(80, window.innerWidth / window.innerHeight, 0.1, 1000);
camera.position.set(25, 2.5, 25);
let isPerspective = true;
let renderer, controls;
initRenderer();
function initRenderer() {
    renderer = new THREE.WebGLRenderer({
        antialias: true,
        alpha: true,
        preserveDrawingBuffer: false
    });
    renderer.setSize(window.innerWidth, window.innerHeight);
    renderer.setPixelRatio(window.devicePixelRatio);
    container.appendChild(renderer.domElement);
    controls = new THREE.OrbitControls(camera, renderer.domElement);
}

const clock = new THREE.Clock();
const light = new THREE.DirectionalLight(0xffffff, 0.75);
light.position.setScalar(10);
scene.add(light);
scene.add(new THREE.AmbientLight(0xffffff, 0.5));

const color = 0xd69dff;
const length = 10;
let multiplier = document.getElementById('gpuLevel').value;
let isRotate = true;
let isShiny = false;
let isBW = true;
let isLogHeight = false;

let resolution = new THREE.Vector2(window.innerWidth, window.innerHeight);
let graph = new THREE.Object3D();
let grid = new THREE.Object3D();
grid.name = grid;
scene.add(graph);

createGrid();
createText();
window.init().then(() => {
    load();
});
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
    mesh.name = 'gridline';
    grid.add(mesh);
}

function createGrid() {
    for (let i = -length; i <= length; i++) {
        let line = new THREE.Geometry();

        let width = 5;
        if (i == 0) width = 20;

        let axesColor = 0xd69dff;

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
    return new Promise(function (resolve, reject) {
        let width = 2 * length; // 20
        let height = width;
        let segments = multiplier * width;
        let plane = new THREE.PlaneBufferGeometry(width, height, segments, segments);
        let legend = document.getElementById('legend');
        if (plot == 'Re-Im') {
            if (isBW)
                legend.innerHTML =
                    'height of surface = Re(f(z))<br>color of surface - white = bigger Im(f(z)), black = smaller Im(f(z))';
            else
                legend.innerHTML =
                    'height of surface = Re(f(z))<br>color of surface - magenta = bigger Im(f(z)), light cyan = 0, red = smaller Im(f(z))';
        } else if (plot == 'Im-Re') {
            if (isBW)
                legend.innerHTML =
                    'height of surface = Im(f(z))<br>color of surface - magenta = bigger Re(f(z)), red = smaller Re(f(z))';
            else
                legend.innerHTML =
                    'height of surface = Im(f(z))<br>color of surface - white = bigger Re(f(z)), black = smaller Re(f(z))';
        } else {
            if (isBW) toggleBW();
            legend.innerHTML = 'height of surface = modulus of output<br>color of surface - argument of output (R→G→B)';
        }
        let graph_type = 4;
        if (plot == 'Re-Im') {
            if (isBW) graph_type = 1;
            else graph_type = 0;
        } else if (plot == 'Im-Re') {
            if (isBW) graph_type = 3;
            else graph_type = 2;
        }
        let res = window.evaluate(inputBox.value, multiplier, graph_type, isLogHeight);
        let posLen = 3 * Math.pow(20 * multiplier + 1, 2);
        let pos = res.slice(0, posLen);
        let colors = res.slice(posLen);

        // color logging
        // for (let i = 0; i < colors.length; i += 3) {
        //     let r = parseInt(colors[i] * 255);
        //     let g = parseInt(colors[i + 1] * 255);
        //     let b = parseInt(colors[i + 2] * 255);
        //     let R = r.toString(16);
        //     let G = g.toString(16);
        //     let B = b.toString(16);
        //     console.log('%c#' + R + G + B, `color:#${R + G + B}`);
        // }
        plane.attributes.position.array = pos;
        plane.setAttribute('color', new THREE.BufferAttribute(new Float32Array(colors), 3));

        plane.computeVertexNormals();
        let mesh;
        if (isShiny)
            mesh = new THREE.Mesh(
                plane,
                new THREE.MeshStandardMaterial({
                    vertexColors: THREE.VertexColors,
                    side: THREE.DoubleSide,
                    specular: '#333333',
                    transparent: true,
                    opacity: 0.7
                })
            );
        else
            mesh = new THREE.Mesh(
                plane,
                new THREE.MeshLambertMaterial({
                    vertexColors: THREE.VertexColors,
                    side: THREE.DoubleSide
                })
            );
        mesh.rotation.x = -Math.PI / 2;
        mesh.name = 'mesh';
        resolve(mesh);
    });
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
        ImMesh.position.z = -length;
        ImMesh.rotation.x = -Math.PI / 2;
        ImMesh.name = 'ImMesh';
        graph.add(ImMesh);

        let Relabel = new THREE.TextGeometry('Re', options);
        materials = [
            new THREE.MeshPhongMaterial({ color: null, flatShading: true }), // front
            new THREE.MeshPhongMaterial({ color: null }) // side
        ];
        let ReMesh = new THREE.Mesh(Relabel, materials);
        ReMesh.position.x = length;
        ReMesh.rotation.x = -Math.PI / 2;
        ReMesh.name = 'ReMesh';
        graph.add(ReMesh);
    });
}

function load() {
    multiplier = document.getElementById('gpuLevel').value;
    plot = plotChooser.value;
    clearMesh();
    createMesh().then((mesh) => {
        graph.add(mesh);
    });
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
    if (isRotate && isPerspective) graph.rotation.y += 0.001; // graph.rotation.y += 0.05 * clock.getDelta();
    renderer.render(scene, camera);
}

function clearMesh() {
    for (let i = 0; i < graph.children.length; i++) {
        if (graph.children[i].name == 'mesh') {
            graph.remove(graph.children[i]);
        }
    }
    for (let i = 0; i < graph.children.length; i++) {
        if (graph.children[i].name == 'mesh') {
            graph.remove(graph.children[i]);
        }
    }
}

function exportImg() {
    // replace renderer
    renderer = new THREE.WebGLRenderer({
        antialias: true,
        alpha: true,
        preserveDrawingBuffer: true
    });
    renderer.setSize(window.innerWidth, window.innerHeight);
    renderer.setPixelRatio(window.devicePixelRatio);
    // remove previous canvas
    container.innerHTML = '';
    // add preserveDrawingBuffer canvas
    container.appendChild(renderer.domElement);
    controls = new THREE.OrbitControls(camera, renderer.domElement);

    load();
    renderer.render(scene, camera);
    // image stuff
    let imgData = document.getElementsByTagName('canvas')[0].toDataURL('image/png');
    var element = document.createElement('a');
    element.setAttribute('href', imgData.replace(/^data:image\/[^;]/, 'data:application/octet-stream'));
    element.setAttribute('download', 'graph.png');
    element.style.display = 'none';

    element.click();
    document.removeChild(element);
    // remove preserveDrawingBuffer canvas
    container.innerHTML = '';
    initRenderer();
}

function toggleRot() {
    if (isRotate) rotBtn.innerText = 'Rotate Off';
    else rotBtn.innerText = 'Rotate On';
    isRotate = !isRotate;
}

function toggleShiny() {
    if (isShiny) shineBtn.innerText = 'Shine Off';
    else shineBtn.innerText = 'Shine On';
    isShiny = !isShiny;
    load();
}
function toggleBW() {
    if (isBW) bwBtn.innerText = 'Switch to B/W';
    else bwBtn.innerText = 'Switch to color';
    isBW = !isBW;
    load();
}

function toggleLog() {
    isLogHeight = !isLogHeight;
    load();
}
function toggleOrth() {
    isPerspective = !isPerspective;
    if (isPerspective) {
        camera = new THREE.PerspectiveCamera(80, window.innerWidth / window.innerHeight, 0.1, 100);
        camera.position.set(25, 2.5, 25);
        controls = new THREE.OrbitControls(camera, renderer.domElement);
    } else {
        camera = new THREE.OrthographicCamera(
            window.innerWidth / -2,
            window.innerWidth / 2,
            window.innerHeight / 2,
            window.innerHeight / -2,
            1,
            1000
        );
        camera.position.set(0, 1000, 0);
        controls = new THREE.OrbitControls(camera, renderer.domElement);
        graph.rotation.y = 0;
    }
}
