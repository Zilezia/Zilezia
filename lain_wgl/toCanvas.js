let gl, gRLoop,	gShader, gModal, gCamera, gCameraCtrl;
let gGridShader, gGridModal;
// var gModal2, gModal3;
let modals = [];

async function loadObjFile(objPath) {
    const response = await fetch(`./objects/${objPath}.obj`);
    const objText = await response.text();
    return objText;
}

async function loadShaderFile(shaderPath) {
    const response = await fetch(`./wgsl/${shaderPath}.wgsl`);
    const shaderText = await response.text();
    return shaderText;
}

async function loadTexFile(texPath) {
    return new Promise((resolve, reject) => {
        const img = new Image();
        img.src = `./objects/${texPath}`;
        img.crossOrigin = "anonymous";
        img.onload = () => resolve(img);
        img.onerror = (err) => reject(err);
    });
}

window.addEventListener("load", async function(){
    //Main Setup
    gl = GLInstance("glcanvas").fFitScreen(1,1).fClear();
    
    gCamera = new Camera(gl,70,0.1,500);
    gCamera.transform.position.set(0,1,3);
    gCameraCtrl = new CameraController(gl,gCamera);

    //....................................
    //Load up resources
    let imgTex;
    // const objText = await loadObjFile("cube.obj");
    // const objText = await loadObjFile("pirate_girl.obj");
    // const imgTex = await loadTexFile("pirate_girl.png");
    const objText = await loadObjFile("lain_ring_lvl2");
    
    imgTex = await loadTexFile("lain_ring_lvl-texture.png"); // default ring texture ?
    // imgTex = await loadTexFile("scribble.png");
    // if no texture dont give any (black).
    imgTex ? gl.fLoadTexture("tex001", imgTex) : null;

    gl.fLoadCubeDirections("skybox01",[
        { target: gl.TEXTURE_CUBE_MAP_POSITIVE_X, faceColor: '#0f002e' },
        { target: gl.TEXTURE_CUBE_MAP_NEGATIVE_X, faceColor: '#0f002e' },
        { target: gl.TEXTURE_CUBE_MAP_POSITIVE_Y, faceColor: '#0f002e' },
        { target: gl.TEXTURE_CUBE_MAP_NEGATIVE_Y, faceColor: '#0f002e' },
        { target: gl.TEXTURE_CUBE_MAP_POSITIVE_Z, faceColor: '#0f002e' },
        { target: gl.TEXTURE_CUBE_MAP_NEGATIVE_Z, faceColor: '#0f002e' },
    ]);

    gl.fLoadCubeDirections("skybox02",[
        { target: gl.TEXTURE_CUBE_MAP_POSITIVE_X, faceColor: '#F00', textColor: '#0FF', text: '+X' },
        { target: gl.TEXTURE_CUBE_MAP_NEGATIVE_X, faceColor: '#FF0', textColor: '#00F', text: '-X' },
        { target: gl.TEXTURE_CUBE_MAP_POSITIVE_Y, faceColor: '#0F0', textColor: '#F0F', text: '+Y' },
        { target: gl.TEXTURE_CUBE_MAP_NEGATIVE_Y, faceColor: '#0FF', textColor: '#F00', text: '-Y' },
        { target: gl.TEXTURE_CUBE_MAP_POSITIVE_Z, faceColor: '#00F', textColor: '#FF0', text: '+Z' },
        { target: gl.TEXTURE_CUBE_MAP_NEGATIVE_Z, faceColor: '#F0F', textColor: '#0F0', text: '-Z' },
    ]);

    //....................................
    //Setup Grid
    /*
    gGridShader = new GridAxisShader(gl,gCamera.projectionMatrix);
    gGridModal = Primatives.GridAxis.createModal(gl,false);
    */
    //....................................
    //Setup Test Shader, Modal, Meshes
    gShader = new TestShader(gl,gCamera.projectionMatrix)
        .setTexture(gl.mTextureCache["tex001"]);
    
    const ringCount = 15;
    let positions = [];
    let scales = [];
    let rotations = [];

    let rotationSpeed = 0.7;

    for (let i=0; i<ringCount; i++) {
        if (!positions[i]) positions[i] = [0, i/2, 0];
        if (!scales[i]) scales[i] = [1,1,1];
        if (!rotations[i]) {
            // const ranRot = ((i+1)*15)*plusOrMinus;
            const randomRot = (Math.random()*360)*(Math.random() < 0.5 ? -1 : 1);
            rotations[i] = [0,randomRot,0];
        }
        
        let modal = new Modal(ObjLoader.textToMesh(`obj${i}`,objText))
        modal.setPosition(...positions[i]).setScale(...scales[i]).setRotation(...rotations[i]);
        
        function SpinRing(mod) { // basically continous .addRotation
            // const randomSpeeds = (rotationSpeed/(Math.random()*5)) // glitchy behaviour/effect?
            const randomSpeeds = (rotationSpeed*Math.random())/.5

            // clockwise
            if (mod===0) modal.addRotation(0,randomSpeeds,0);
            // anti-clockwise
            else if (mod===1) modal.addRotation(0,-randomSpeeds,0);
            // all directions?
            else modal.addRotation(randomSpeeds,randomSpeeds,randomSpeeds);
            requestAnimationFrame(() =>SpinRing(mod));
        }
        
        if (i%2==0) SpinRing(0);
        else if (i%3==0) SpinRing(2);
        else SpinRing(1);
        
        modals.push(modal);
    }

    // gModal = Primatives.Cube.createModal(gl);
    // gModal.setPosition(0,0,0);

    // gModal2 = new Modal( ObjLoader.textToMesh("obj1", objText,true) )
    // gModal2.setPosition(0,0,0).setScale(1,1,1);

    // gModal3 = new Modal( ObjLoader.textToMesh("obj2", objText,true) )
    // gModal3.setPosition(0,1.5,0).setScale(1,1,1);
    
    // Skympa setup
    const skyS = 25;
    gSkymap = new Modal(Primatives.Cube.createMesh(gl,"Skymap", skyS,skyS,skyS, 0,0,0));
    async function initShadersAndStartRendering() {
        const skyBoxType = "skybox01";
        gSkyMapShader = await SkymapShader.init(gl, gCamera.projectionMatrix, gl.mTextureCache[skyBoxType]);
        RLoop = new RenderLoop(onRender,60).start();
    }
    //....................................
    //Start Rendering
    initShadersAndStartRendering()
    //onRender(0);
});

function onRender(dt){
    gCamera.updateViewMatrix();
    gl.fClear();
    
    
    gSkyMapShader.activate().preRender()
        .setCameraMatrix(gCamera.getTranslatelessMatrix())
        .renderModal(gSkymap)
        // if want "day-night" cycle
        // .setTime(performance.now())
    ;
    /*
    gGridShader.activate()
        .setCameraMatrix(gCamera.viewMatrix)
        .renderModal( gGridModal.preRender() );
    */

    gShader.activate().preRender()
        .setCameraMatrix(gCamera.viewMatrix)
        // previous way of loading vvv
        // .renderModal(gModal.preRender())
    ;
    modals.forEach((modal) => {
        gShader.renderModal(modal.preRender());
    });
}

class TestShader extends Shader{
    constructor(gl,pMatrix){
        const vertSrc = ShaderUtil.domShaderSrc("vertex_shader");
        const fragSrc = ShaderUtil.domShaderSrc("fragment_shader");
        super(gl,vertSrc,fragSrc);

        //Standrd Uniforms
        this.setPerspective(pMatrix);
        this.mainTexture = -1; //Store Our Texture ID
        gl.useProgram(null); //Done setting up shader
    }

    setTexture(texID){ this.mainTexture = texID; return this; }

    //Override
    preRender(){
        //Setup Texture
        this.gl.activeTexture(this.gl.TEXTURE0);
        this.gl.bindTexture(this.gl.TEXTURE_2D, this.mainTexture);
        this.gl.uniform1i(this.uniformLoc.mainTexture,0); //Our predefined uniformLoc.mainTexture is uMainTex, Prev Lessons we made ShaderUtil.getStandardUniformLocations() function in Shaders.js to get its location.

        return this;
    }
}

class SkymapShader extends Shader{
    constructor(gl, vertSrc,fragSrc, pMatrix, skyTex,){
        // const vertSrc = ShaderUtil.domShaderSrc("sky_vshader");
        // const fragSrc = ShaderUtil.domShaderSrc("sky_fshader");
        super(gl,vertSrc,fragSrc);

        //Custom Uniforms
        // this.uniformLoc.time = gl.getUniformLocation(this.program,"uTime");
        this.uniformLoc.skyTex = gl.getUniformLocation(this.program,"uSkyTex");
        // this.uniformLoc.nightTex = gl.getUniformLocation(this.program,"uNightTex");

        //Standrd Uniforms
        this.setPerspective(pMatrix);
        this.texNor = skyTex;
        // this.texNight = nightTex;
        gl.useProgram(null); //Done setting up shader
    }

    // setTime(t){ this.gl.uniform1f(this.uniformLoc.time,t); return this; }

    static async init(gl, pMatrix, skyTex) {
        const vertSrc = await ShaderUtil.shaderFile("sky_vshader");
        const fragSrc = await ShaderUtil.shaderFile("sky_fshader");
        return new SkymapShader(gl, vertSrc, fragSrc, pMatrix, skyTex);
    }

    //Override
    preRender(){
        //Setup Texture
        this.gl.activeTexture(this.gl.TEXTURE0);
        this.gl.bindTexture(this.gl.TEXTURE_CUBE_MAP, this.texNor);
        this.gl.uniform1i(this.uniformLoc.skyTex,0);

        // this.gl.activeTexture(this.gl.TEXTURE1);
        // this.gl.bindTexture(this.gl.TEXTURE_CUBE_MAP, this.texNight);
        // this.gl.uniform1i(this.uniformLoc.nightTex,1);
        return this;
    }
}

async function gInitSky(gl, pMatrix, skyTex) {
    const gSkyMapShader = await SkymapShader.init(gl, pMatrix, skyTex);
    gSkyMapShader.activate().preRender()
        .setCameraMatrix(gCamera.getTranslatelessMatrix())
        .renderModal(gSkymap);
}