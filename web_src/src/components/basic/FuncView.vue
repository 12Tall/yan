<template>
    <div :id="rand_id"></div>
</template>

<script>
// https://stackoverflow.com/questions/66382293/how-to-use-props-in-script-setup-in-vue3
import { uniqueId } from 'lodash'
import { onMounted, watch, toRaw } from 'vue'

export default {
    props: ['data', 'title'],
    setup(props) {
        let rand_id = uniqueId('_func_view_');
        let option = {
            target: `#${rand_id}`,
            data: []
        };
        let plot = null;

        onMounted(() => {
            option.data = toRaw(props.data)
            plot = functionPlot(option)
        });

        watch(props, (np, op) => {
            // props 里的data 指针不能变，只能push/pop
            // 否则就需要每次重新初始化并绘制
            try {  // 如果不捕获异常，则会导致图像永远不能更新
                plot.draw();
            } catch (error) {

            }
        });
        return {
            rand_id
        }
    }
}



</script>

<style>
</style>