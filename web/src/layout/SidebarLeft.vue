<template>
  <Splitter class="sidebar-right" layout="vertical" v-on:resize="resize">
    <SplitterPanel
      class="flex align-items-center justify-content-center"
      :size="50"
      :min-size="20"
    >
      <div class="right-sidebar-header">123</div>
      <ItemList />
    </SplitterPanel>
    <SplitterPanel class="folder" :size="50">
      <FolderTree />
    </SplitterPanel>
  </Splitter>
</template>

<script setup lang="ts">
import Splitter from "primevue/splitter";
import SplitterPanel from "primevue/splitterpanel";
import { SplitterResizeEvent } from "primevue/splitter";

import FolderTree from "@/components/FloderTree.vue";
import ItemList from "@/components/ItemList.vue";
import { useLayoutStore } from "@/stores";
const layoutStore = useLayoutStore();

function resize(e: SplitterResizeEvent) {
  console.log(e);
  layoutStore.setLeftSidebarSize(e.sizes);
}
</script>

<style lang="less" scoped>
// .dashboard-layout {
//   min-height: 100vh;
//   .right-sidebar-header {
//     display: flex;
//   }
// }
.sidebar-right {
  height: 100%;
  max-height: calc(100% - 8px);
}
</style>
