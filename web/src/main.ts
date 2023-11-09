import { createApp } from "vue";
import { router } from "@/router";
import { createPinia } from "pinia";
import PrimeVue from "primevue/config";
import { useStore } from "@/stores/index";

import "primevue/resources/themes/lara-light-teal/theme.css";
import "./style.less";
import App from "./App.vue";

const pinia = createPinia();
const app = createApp(App);
app.use(PrimeVue);
app.use(router);
app.use(pinia);
useStore();
app.mount("#app");
