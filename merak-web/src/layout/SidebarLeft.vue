<template>
  <Splitter class="left-splitter" layout="vertical" v-on:resize="resize">
    <SplitterPanel
      class="left-splitter-item flex align-items-center justify-content-center"
      :size="50"
      :min-size="20"
    >
      <div class="right-sidebar-header">123</div>
      <ItemList />
      <FolderTree />
    </SplitterPanel>
  </Splitter>
</template>

<script setup lang="ts">
import { ref } from "vue";
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
.left-splitter {
  height: calc(100% - 4px);
  max-height: calc(100% - 4px);
  // &-item {
  //   height: calc(v-bind(sizeItem) - 4px);
  // }
  // &-folder {
  //   height: calc(v-bind(sizeFolder) - 4px);
  // }
}
</style>
