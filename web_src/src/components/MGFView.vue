<template>
    <el-card>
        <el-row>
            <el-col :span="24" :gutter="20">
                <el-input
                    v-model="input_str"
                    :autosize="{ minRows: 4 }"
                    type="textarea"
                    placeholder="Please input with csv format: node_out, node_in, gain"
                ></el-input>
            </el-col>
        </el-row>
        <el-row :gutter="20">
            <el-col :span="8">
                <el-input type="number" v-model="node_input">
                    <template #prepend>输入节点</template>
                </el-input>
            </el-col>
            <el-col :span="8">
                <el-input type="number" v-model="node_output">
                    <template #prepend>输出节点</template>
                </el-input>
            </el-col>
            <el-col :span="8">
                <el-button type="primary" @click="MGF">计算</el-button>
            </el-col>
        </el-row>
        <el-divider content-position="left">Latex 结果：</el-divider>
        <el-row :gutter="20">
            <el-col :span="24">
                <KatexView :latex="latex"></KatexView>
            </el-col>
        </el-row>
        <el-divider content-position="left">原始结果：</el-divider>
        <el-row :gutter="20">
            <el-col :span="24">
                {{ text }}
            </el-col>
        </el-row>
    </el-card>
</template>

<script>
import { ref, } from 'vue';
import { imatrix, matgetcol, matsetrow, invert, convertToLaTeX, matget } from 'nerdamer/all.min'
import * as nerdamer from 'nerdamer/all.min'
import KatexView from './basic/KatexView.vue';

export default {
    setup() {
        let str =
            `0	1	1
1	2	G1
2	3	G2
3	4	G3
4	5	G4
5	6	1
4	2	-H2
5	3	-H3
5	1	-H1`;
        let input_str = ref(str),
            node_input = ref("0"),
            node_output = ref(6),
            latex = ref(""),
            text = ref("");

        function MGF() {
            let jt = parseInput();
            let vec = GenVector(jt);
            let res = matget(vec, node_output.value, 0).toString();
            res = nerdamer(res);
            latex.value = res.toTeX();
            text.value = res.toString();
        }

        return {
            input_str,
            node_input,
            node_output,
            latex,
            text,
            MGF
        };

        function parseInput() {
            return input_str.value.split(/\r\n|\n/).map(line => {
                let node = line.trim().split(/\s+/);
                if (node.length < 3) {
                    alert("accept 3 params only")
                    return { i: 0, o: 0, g: 1 }
                }
                return { i: parseInt(node[0]), o: parseInt(node[1]), g: node[2] }
            }).sort((elem1, elem2) => {
                // 按输出节点排序，逆序
                let temp = elem2.i - elem1.i;
                return (elem2.o - elem1.o) * (Math.abs(temp) + 1) + temp;
            });
        }

        function GenVector(jt) {
            let dim = jt[0].o + 1;

            let dat = new Array();
            for (var r = 0; r < dim; r++) {
                dat[r] = new Array();
                for (var c = 0; c < dim; c++) {
                    dat[r][c] = 0;
                }
            }
            jt.forEach(el => {
                dat[el.o][el.i] = el.g
            });

            let mat = imatrix(dim);
            for (var i = 0; i < dim; i++) {
                let dat_i = JSON.stringify(dat[i]).replace(/\"/g, "")
                mat = matsetrow(mat, i, dat_i);
            }

            let imat = imatrix(dim);
            mat = imat.subtract(mat)
            return invert(mat).multiply(matgetcol(imat, node_input.value))
        }
    },
    components: { KatexView }
}


// 1. 解析文本，返回邻接表。其实是[{i:int, o: int, g:string}, ...]



</script>

<style>
.el-row {
    margin-bottom: 20px;
}
</style>