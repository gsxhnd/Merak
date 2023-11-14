<template>
  <div class="content-header">content-header</div>
  <Button label="Submit" @click="toggleLeftSidebar" />
</template>

<script setup lang="ts">
import Button from "primevue/button";
import { useLayoutStore } from "@/stores";
const layoutStore = useLayoutStore();

function toggleLeftSidebar() {
  layoutStore.showLeftSidbar = !layoutStore.showLeftSidbar;
}
</script>

<style lang="less" scoped>
.sidebar-right {
  height: 100%;
}
</style>
