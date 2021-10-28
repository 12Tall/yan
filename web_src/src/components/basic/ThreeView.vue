<template>
    <div :id="rand_id"></div>
</template>

<script>
import * as Three from 'three'
import { ref, onMounted } from 'vue'
import { uniqueId } from 'lodash'
export default {
    setup() {
        let rand_id = ref(uniqueId('_yan_three_'));
        onMounted(() => {
            const scene = new Three.Scene();
            const camera = new Three.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
            const renderer = new Three.WebGLRenderer();
            renderer.setSize(400, 300);
            document.querySelector(`#${rand_id.value}`).appendChild(renderer.domElement);

            const geometry = new Three.BoxGeometry();
            const material = new Three.MeshBasicMaterial({ color: 0x00ff00 });
            const cube = new Three.Mesh(geometry, material);
            scene.add(cube);

            camera.position.z = 5;
            function animate() {
                requestAnimationFrame(animate);
                cube.rotation.x += 0.01;
                cube.rotation.y += 0.01;
                renderer.render(scene, camera);
            }
            animate();

        });

        return {
            rand_id
        }
    }

}
</script>

<style>
</style>