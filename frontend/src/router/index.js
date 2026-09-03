import { createRouter, createWebHistory } from "vue-router";
import LoginView from "../views/LoginView.vue";
import HomeView from "../views/HomeView.vue";
import { LS_TOKEN } from "../utils/constants.js";

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

const isLoggedIn = () => !!localStorage.getItem(LS_TOKEN);

router.beforeEach((to) => {
  if (to.meta.requiresAuth && !isLoggedIn()) {
    return { name: "Login" };
  }
  if (to.name === "Login" && isLoggedIn()) {
    return { name: "Home" };
  }
});

export default router;
