<template>
    <el-card>
        <template #header>
            <el-input v-model="expr" placeholder="x">
                <template #prepend>y=</template>
            </el-input>
        </template>
        <FuncView :data="data"></FuncView>
    </el-card>
</template>

<script>
import FuncView from "./basic/FuncView.vue";
import { reactive, ref, watch } from 'vue';
export default {
    components: { FuncView },
    setup() {
        const expr = ref("x+2");
        const data = reactive([]);

        watch(() => {
            data.length = 0;
            data.push({ fn: expr.value });
        });


        return {
            data,
            expr
        }
    }
}
</script>

<style>
</style>