import { defineStore } from "pinia";

export const useLayoutStore = defineStore("layout", {
  state: () => {
    return {
      showRightSidbar: true,
      showLeftSidbar: true,
      leftSidbarSize: [50,50] as Array<number>,
    };
  },
  actions: {
    setLeftSidebarSize(size: Array<number>) {
      this.leftSidbarSize = size;
    },
  },
});
