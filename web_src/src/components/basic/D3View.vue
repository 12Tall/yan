<template>
    <div :id="rand_id">
        D3 View
        <p>🛴</p>
    </div>
</template>

<script>
import { uniqueId } from 'lodash'
import { ref, onMounted } from 'vue'
import * as d3 from 'd3'
export default {
    setup() {
        let rand_id = ref(uniqueId('_yan_d3_'));
        let ctx = null;

        onMounted(() => {
            ctx = d3.select(`#${rand_id.value}`);
            ctx.text('').selectAll('*').remove();
            ctx = ctx.append('svg').attr("height", 300).attr("width", 300);

            // ===== ===== ===== ===== //
            ctx.on('zoom',()=>{
                console.log(d3.event.scale);
            })

        });

        return {
            rand_id
        }
    }

}
</script>

<style>
</style>