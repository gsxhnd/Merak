import { createPinia, defineStore } from "pinia";
import { useLayoutStore } from "./layout";

export function useStore() {
  useLayoutStore();
}

export { useLayoutStore };
