import * as BABYLON from 'babylonjs';

export function createTerrain(scene) {
    // Create a large ground
    const ground = BABYLON.MeshBuilder.CreateGround("ground", { width: 1000, height: 1000, subdivisions: 50 }, scene);

    const groundMat = new BABYLON.StandardMaterial("groundMat", scene);
    groundMat.diffuseTexture = new BABYLON.Texture("https://playground.babylonjs.com/textures/grass.png", scene);
    groundMat.diffuseTexture.uScale = 50; // Repeat texture
    groundMat.diffuseTexture.vScale = 50;
    groundMat.specularColor = new BABYLON.Color3(0, 0, 0); // No shine

    ground.material = groundMat;

    return ground;
}
