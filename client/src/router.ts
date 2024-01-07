import { inject } from "vue";
import { createRouter, createWebHistory } from "vue-router";
import { Authentication } from "./composables/useAuthentication";

const routes = [
  {
    path: "/",
    name: "Home",
    component: () => import("./views/Home.vue"),
  },
  {
    path: "/characters",
    name: "Characters",
    component: () => import("./views/Characters.vue"),
  },
  {
    path: "/login",
    name: "Login",
    component: () => import("./views/Database.vue"),
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

router.beforeEach((to, _from, next) => {
  const authentication = inject<Authentication>("authentication");

  if (to.name !== "Login" && !authentication?.isAuthenticated.value)
    next({ name: "Login" });
  else next();
});

export default router;
