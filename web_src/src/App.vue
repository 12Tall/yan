<template>
  <el-container>
    <!-- <el-header></el-header> -->
    <el-main>
      <!-- <FunctionView></FunctionView> -->
      <MGFView></MGFView>
    </el-main>
  </el-container>
</template>

<script>
import FunctionView from "./components/FunctionView.vue";
import MGFView from "./components/MGFView.vue";
export default {
  components: { FunctionView, MGFView },
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
