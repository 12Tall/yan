<template>
    <div :id="rand_id"></div>
</template>

<script>
import { ref, watch, onMounted, toRaw } from 'vue'
import katex from 'katex'
import { uniqueId } from 'lodash'
export default {
    props: ['latex'],

    setup(props) {
        const rand_id = uniqueId('_yan_katex_');
        let div;
        onMounted(() => {
            div = document.querySelector(`#${rand_id}`);
            katex.render(props.latex, div, {
                throwOnError: false
            });
        });

        watch(props, (np, op) => {
            katex.render(props.latex, div, {
                throwOnError: false
            });
        });

        return {
            rand_id
        }
    }

}
</script>

<style>
</style>