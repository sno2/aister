<template>
  <Header />
  <RouterView />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import Header from "@/comps/Header.vue";

export default defineComponent({
  name: "App",
  components: { Header },
});
</script>

<style lang="scss">
* {
  box-sizing: border-box;
}

html {
  font-family: "Source Sans Pro", Helvetica, sans-serif;
}

body {
  margin: 0;
  padding: 0;
}

#app {
  width: 100%;
  min-height: 100vh;
}
</style>
