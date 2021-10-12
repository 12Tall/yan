<template>
  <el-container>
    <!-- <el-header></el-header> -->
    <el-main>
      <FunctionView></FunctionView>
    </el-main>
  </el-container>
</template>

<script>
import FunctionView from "./components/FunctionView.vue";
export default {
  components: { FunctionView },
  setup() {
  }
}
</script>
<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 0px;
}
</style>
