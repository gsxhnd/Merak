import { createRouter, createWebHashHistory, RouteRecordRaw } from "vue-router";

const Login = { template: "<div>Login</div>" };

// const RootRoute: RouteRecordRaw = {
//   path: "/",
//   name: "Root",
//   component: DashboardLayout,
//   meta: {
//     title: "Root",
//   },
// };

const LoginRoute: RouteRecordRaw = {
  path: "/",
  name: "Login",
  component: Login,
  meta: {
    title: "login",
  },
};

const router = createRouter({
  history: createWebHashHistory(),
  routes: [LoginRoute],
  // strict: true,
});

export { router };
