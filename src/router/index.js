import Vue from 'vue'
import VueRouter from 'vue-router'
import Home from '../views/Home.vue'
import Vote from '../views/Vote.vue'
import List from '../views/List.vue'

Vue.use(VueRouter)

const routes = [
  {
    path: '/',
    redirect: '/list',
    name: 'home',
    component: Home,
    children: [
      {
        path: 'list',
        component: List
      },
      {
        path: 'vote',
        redirect: '/list'
      },
      {
        path: 'vote/:vid',
        name: 'vote',
        component: Vote
      }
    ]
  }
]

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes
})

export default router
