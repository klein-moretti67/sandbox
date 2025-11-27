import * as BABYLON from 'babylonjs';
import { createSkybox } from './skybox.js';
import { createTerrain } from './terrain.js';

export function createWorld(scene) {
    createSkybox(scene);
    createTerrain(scene);

    // Wrap around logic
    scene.registerBeforeRender(() => {
        const camera = scene.activeCamera;
        if (camera) {
            const limit = 500; // Half of ground size (1000)
            // If player goes past limit, teleport to opposite side
            if (camera.position.x > limit) camera.position.x -= limit * 2;
            if (camera.position.x < -limit) camera.position.x += limit * 2;
            if (camera.position.z > limit) camera.position.z -= limit * 2;
            if (camera.position.z < -limit) camera.position.z += limit * 2;
        }
    });
}
