import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    name: "Home",
    component: () => import("./views/Home.vue"),
  },
  {
    path: "/chat",
    name: "Chat",
    component: () => import("./views/Chat.vue"),
  },
  {
    path: "/translate",
    name: "Translate",
    component: () => import("./views/Translate.vue"),
  },
];

export const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});
