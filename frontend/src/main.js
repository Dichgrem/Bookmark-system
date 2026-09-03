import { createApp } from "vue";
import "element-plus/dist/index.css";
import App from "./App.vue";
import router from "./router/index.js";
import i18n from "./i18n/index.js";
import "./style.css";

const app = createApp(App);
app.use(router);
app.use(i18n);
app.mount("#app");
