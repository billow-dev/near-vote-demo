import Vue from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'

import Antd from 'ant-design-vue'
import 'ant-design-vue/dist/antd.css'
import utils from '@/utils/near-utils'
const { initContract } = utils
Vue.use(Antd)

Vue.config.productionTip = false

window.nearInitPromise = initContract()
  .then(() => {
    new Vue({
      router,
      store,
      render: h => h(App)
    }).$mount('#app')
  })
