import { createRouter, createWebHashHistory, RouteRecordRaw } from "vue-router";
import Dashboard from "@/layout/Dashboard.vue";
import FolderTree from "@/components/FloderTree.vue";

const HomeRoute: RouteRecordRaw = {
  path: "/",
  name: "home",
  component: Dashboard,
  meta: {
    title: "login",
  },
};

const TestRoute: RouteRecordRaw = {
  path: "/test",
  name: "test",
  component: FolderTree,
  meta: {
    title: "login",
  },
};

const router = createRouter({
  history: createWebHashHistory(),
  routes: [HomeRoute, TestRoute],
  // strict: true,
});

export { router };
