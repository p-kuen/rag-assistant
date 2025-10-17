import { createRouter, createWebHistory } from "vue-router";
import ChatView from "../views/ChatView.vue";
import AdminView from "../views/AdminView.vue";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      redirect: "/chat",
    },
    {
      path: "/chat",
      name: "chat",
      component: ChatView,
      meta: {
        title: "Chat-Assistent",
      },
    },
    {
      path: "/admin/knowledge",
      name: "admin-knowledge",
      component: AdminView,
      meta: {
        title: "Wissensverwaltung",
      },
    },
    // Legacy-Route für Kompatibilität
    {
      path: "/admin",
      redirect: "/admin/knowledge",
    },
  ],
});

// Optional: Titel setzen
router.beforeEach((to, _from, next) => {
  document.title = (to.meta.title as string) || "RAG Assistant";
  next();
});

export default router;
