import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../pages/HomeView.vue'
import DashboardView from '../pages/DashboardView.vue'
import BiographyEditor from '../pages/BiographyEditor.vue'
import AiAssistant from '../pages/AiAssistant.vue'

const routes = [
  {
    path: '/',
    name: 'home',
    component: HomeView
  },
  {
    path: '/dashboard',
    name: 'dashboard',
    component: DashboardView
  },
  {
    path: '/editor/:id?',
    name: 'editor',
    component: BiographyEditor,
    props: true
  },
  {
    path: '/ai-assistant',
    name: 'ai-assistant',
    component: AiAssistant
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router