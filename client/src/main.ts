import { createApp } from "vue";
import "./style.css";

import App from "./App.vue";

import { invoke } from "@tauri-apps/api";
import router from "./router";

// now we can call our Command!
// Right-click the application background and open the developer tools.
// You will see "Hello, World!" printed in the console!
invoke("greet", { name: "World" })
  // `invoke` returns a Promise
  .then((response) => console.log(response));

createApp(App).use(router).mount("#app");
