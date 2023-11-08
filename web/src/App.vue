<script setup lang="ts">
import { RouterView, useRoute } from "vue-router";
import { onMounted } from "vue";
import { testStore } from "@/stores/index";
import HelloWorld from "@/components/HelloWorld.vue";

const route = useRoute();
const test = testStore();

onMounted(() => {
  console.log(route.path);
});
</script>

<template>
  <HelloWorld msg="hello" />
  <router-view class="router-view" />
</template>

<style scoped lang="less"></style>
