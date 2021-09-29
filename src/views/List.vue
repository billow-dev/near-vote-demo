<template>
    <div id="list">
        <a-timeline mode="alternate">
            <a-timeline-item
                id="topic"
                v-for="(t, i) in topics"
                :color="t.over === true ? 'green' : 'blue'"
                :key="i"
                @click="toVote(t.id)"
            >
            {{t.topic_desc}}
            </a-timeline-item>
        </a-timeline>
    </div>
</template>

<script>

export default {
  name: 'list',
  data () {
    return {
      topics: []
    }
  },
  created () {
    // 拉取投票列表
    this.getList().then((topics) => {
      this.topics = topics
    })
  },

  methods: {
    toVote (id) {
      this.$router.push({ name: 'vote', params: { vid: id } })
    },
    // 拉取该登陆用户的投票列表
    async getList () {
      return await window.contract.list_topic({ account_id: window.walletAccount.getAccountId() })
    }
  }
}
</script>

<style scoped>
#list {
    padding-top: 5%;
    padding-left: 30%;
    padding-right: 30%;
    height: 93vh;
    /* background-image: linear-gradient(to bottom, rgba(233, 212, 204, 0.685) 0%, rgb(216, 233, 241) 100%); */
}
#topic {
  font-size: large;
  font-weight: bold;
}
</style>
