<script setup lang="ts">
import { RouterView, useRoute } from "vue-router";
import { onMounted } from "vue";

const route = useRoute();

onMounted(() => {
  console.log(route.path);
});
</script>

<template>
  <router-view class="router-view" />
</template>

<style scoped lang="less"></style>
