<template>
  <Splitter class="dashboard-layout mb-5">
    <template v-if="layoutStore.showLeftSidbar">
      <SplitterPanel
        layout="vertical"
        class="left-panel flex align-items-center justify-content-center"
        :size="20"
        :minSize="10"
      >
        <SidebarLeft />
      </SplitterPanel>
    </template>
    <SplitterPanel :size="60">
      <Content />
    </SplitterPanel>
    <SplitterPanel
      class="flex align-items-center justify-content-center"
      :size="20"
    >
      <SidebarRight />
    </SplitterPanel>
  </Splitter>
</template>

<script setup lang="ts">
import Splitter from "primevue/splitter";
import SplitterPanel from "primevue/splitterpanel";
import { onBeforeMount } from "vue";
import { useLayoutStore } from "@/stores";

import SidebarRight from "./SidebarRight.vue";
import SidebarLeft from "./SidebarLeft.vue";
import Content from "./Content.vue";
const layoutStore = useLayoutStore();
onBeforeMount(() => {
  console.log("onBeforeMount");
  console.log(layoutStore.$state.showLeftSidbar);
});
</script>

<style lang="less" scoped>
.dashboard-layout {
  min-height: 100vh;
  .left-panel {
    height: calc(100vh - 8px);
  }
  .right-sidebar-header {
    display: flex;
  }
}
</style>
