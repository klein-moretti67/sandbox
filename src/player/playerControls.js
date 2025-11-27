import * as BABYLON from 'babylonjs';

export function setupPlayer(scene, canvas) {
    // FreeCamera is simple for first person
    const camera = new BABYLON.FreeCamera("camera1", new BABYLON.Vector3(0, 2, -10), scene);
    camera.setTarget(BABYLON.Vector3.Zero());
    camera.attachControl(canvas, true);

    // WASD keys
    camera.keysUp.push(87);    // W
    camera.keysDown.push(83);  // S
    camera.keysLeft.push(65);  // A
    camera.keysRight.push(68); // D

    // Speed
    camera.speed = 0.5;
    camera.inertia = 0.9;

    return camera;
}
