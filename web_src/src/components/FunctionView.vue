<template>
    <el-card>
        <template #header>
            <el-input v-model="expr" placeholder="x">
                <template #prepend>y=</template>
            </el-input>
            <KatexView :latex="latex"></KatexView>
        </template>
        <FuncView :data="data"></FuncView>
    </el-card>
</template>

<script>
import FuncView from "./basic/FuncView.vue";
import { reactive, ref, watch, computed } from 'vue';
import KatexView from "./basic/KatexView.vue";
import * as math from 'mathjs'
export default {
    components: { FuncView, KatexView },
    setup() {
        const expr = ref("x+2");
        const data = reactive([]);
        let latex_value = math.simplify(expr.value).toTex();
        const latex = computed(() => {
            try {
                latex_value = math.simplify(expr.value).toTex();
            } catch (error) {

            }
            // console.log(res);
            return `y=${latex_value}`
        });

        watch(() => {
            data.length = 0;
            data.push({ fn: expr.value });
        });


        return {
            data,
            expr,
            latex
        }
    }
}
</script>

<style>
</style>