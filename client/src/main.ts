import { createApp } from "vue";
import "./style.css";

import App from "./App.vue";
import router from "./router";
import useAuthentication from "./composables/useAuthentication";

const app = createApp(App);

app.use(router);

useAuthentication(app);

app.mount("#app");
