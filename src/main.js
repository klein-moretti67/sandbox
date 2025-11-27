import { createEngine } from './engine/engine.js';
import { createWorld } from './world/world.js';
import { setupPlayer } from './player/playerControls.js';

const canvas = document.getElementById('renderCanvas');
const { engine, scene } = createEngine(canvas);

createWorld(scene);
setupPlayer(scene, canvas);

engine.runRenderLoop(() => {
    scene.render();
});

window.addEventListener('resize', () => {
    engine.resize();
});
