import { createRouter, createWebHistory } from "vue-router";
import LoginView from "../views/LoginView.vue";
import HomeView from "../views/HomeView.vue";
import { isTokenValid } from "../utils/auth.js";
import { LS_TOKEN, LS_USER } from "../utils/constants.js";

const routes = [
  {
    path: "/login",
    name: "Login",
    component: LoginView,
  },
  {
    path: "/",
    name: "Home",
    component: HomeView,
    meta: { requiresAuth: true },
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

router.beforeEach((to) => {
  const tokenExists = !!localStorage.getItem(LS_TOKEN);
  const valid = isTokenValid();
  if (tokenExists && !valid) {
    localStorage.removeItem(LS_TOKEN);
    localStorage.removeItem(LS_USER);
  }
  if (to.meta.requiresAuth && !valid) {
    return { name: "Login" };
  }
  if (to.name === "Login" && valid) {
    return { name: "Home" };
  }
});

export default router;
