<template>
    <div id="vote">
        <div id='vh1'>{{topic.topic_desc}}</div>
        <h5 id='vh5'>当前投票人数: {{topic.current.length}}, 还差{{topic.total_limit - topic.current.length}}人</h5>
        <div id="show_options">
          <a-radio-group v-model="select" @change="onChange">
            <a-radio
              v-for="(item, i) in topic.items"
              :key="i"
              :style="radioStyle"
              :value="i"
            >选项: {{item}} <span id="option_account">{{getOptionAccount(i)}}</span>
            </a-radio>
          </a-radio-group>
        </div>
        <a-button @click="handlerVote" v-if="!topic.over" :loading="voteLoading">投票</a-button>
        <a-button v-else disabled>投票结束了</a-button>
        <a-button v-show="isOwner && !topic.over" type="danger" ghost @click="overVote" :loading="overLoading">结束投票</a-button>
        <div v-show="topic.over" id="winner">{{winner()}} 获胜</div>
    </div>
</template>

<script>
export default {
  name: 'vote',
  created () {
    this.getRouterData().then((topic) => {
      if (topic.Err !== undefined) {
        this.$notification.warning({
          message: topic.Err
        })
      } else {
        this.topic = topic.Ok
      }
    })
  },
  data () {
    return {
      overLoading: false,
      voteLoading: false,
      radioStyle: {
        display: 'block',
        height: '50px',
        lineHeight: '50px',
        fontSize: '20px'
      },
      select: 1,
      topic: {
        id: '',
        topic_desc: '',
        items: [],
        item_account_by_index: {},
        total_limit: 0,
        current: [],
        over: false
      }
    }
  },
  methods: {
    async getRouterData () {
      const id = this.$route.params.vid
      return await window.contract.show({ topic_id: id })
    },
    onChange (e) {
      this.select = e.target.value
      console.log('radio checked', e.target.value)
    },
    getOptionAccount (index) {
      if (this.topic.item_account_by_index[index].length === 0) {
        return ''
      }
      return this.topic.item_account_by_index[index]
    },
    winner () {
      let max = 0
      const wins = []
      for (const key in this.topic.item_account_by_index) {
        if (this.topic.item_account_by_index[key].length > max) {
          max = this.topic.item_account_by_index[key].length
        }
      }
      for (const key in this.topic.item_account_by_index) {
        if (this.topic.item_account_by_index[key].length === max) {
          wins.push(this.topic.items[key])
        }
      }
      return wins
    },
    overVote () {
      this.overLoading = true
      window.contract.over(
        {
          topic_id: this.topic.id
        }
        // GAS
        // deposit
      ).then((res) => {
        this.overLoading = false
        if (res.Ok) {
          this.$router.go(0) // 刷新本页面
        } else {
          this.$notification.error({ message: res.Err })
        }
      })
    },
    handlerVote () {
      this.voteLoading = true
      window.contract.vote(
        {
          topic_id: this.topic.id,
          item: this.select
        }
        // GAS
        // deposit
      ).then((res) => {
        this.voteLoading = false
        if (res.Ok) {
          this.$notification.success({ message: '投票成功' })
          this.$router.go(0) // 刷新本页面
        } else {
          this.$notification.error({ message: res.Err })
        }
      }
      )
    }
  },
  computed: {
    isOwner () {
      if (window.walletAccount.getAccountId() === this.topic.id.split('_')[0]) {
        return true
      }
      return false
    }
  }

}
</script>

<style scoped>
#vote {
    height: 87vh;
}
#show_options{
  margin-left: 45%;
  text-align: left;
}
#vh1{
  font-size: xx-large;
  font-weight: bolder;
}
#vh5{
  color: rgb(160, 156, 156);
}
#option_account {
  color: rgb(160, 156, 156);
  font-size: xx-small;
}
#winner {
  font-size: large;
  font-weight: bold;
  color: rgb(105, 165, 120);
}

</style>
