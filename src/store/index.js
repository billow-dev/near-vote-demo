import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    isSigned: false,
    accountId: ''
  },
  mutations: {
    signIn (state) {
      state.isSigned = true
    },
    signOut (state) {
      state.isSigned = false
    },
    setAccountId (state, accountId) {
      state.accountId = accountId
    }
  }
})
