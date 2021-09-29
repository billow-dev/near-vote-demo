<template>
  <div class="home">
    <a-layout>
      <a-layout-header id="header">
        <div id="sign">
          <router-link to="/" id="click_list">我的话题</router-link>
          <a-button type="primary" v-if="updateSign" @click="signOut">{{
            updateAccount
          }}</a-button>
          <a-button type="danger" v-else @click="signIn">登陆</a-button>
        </div>
        欢迎使用基于near的区块链投票Demo
        <div id="new">
          <a-button type="dashed" @click="showDrawer"> 新的投票 </a-button>
        </div>
      </a-layout-header>
      <a-drawer
        title="创建一个新的投票提案"
        :width="560"
        :visible="visible"
        :body-style="{ paddingBottom: '80px' }"
        @close="onClose"
      >
        <a-form
          :form="form"
          :label-col="{ span: 3 }"
          :wrapper-col="{ span: 18 }"
          @submit="handleSubmit"
        >
          <a-form-item label="议题" :required="true">
            <a-input
              v-decorator="[
                'topic_desc',
                {
                  rules: [
                    {
                      required: true,
                      message: '请输入你的议题',
                    },
                  ],
                },
              ]"
            />
          </a-form-item>
          <a-form-item label="人数" :required="true">
            <a-input-number
              v-decorator="[
                'total_limit',
                {
                  rules: [
                    {
                      required: true,
                      message: '限制投票人数',
                    },
                  ],
                },
              ]"
            />
          </a-form-item>
          <a-form-item
            v-for="(k, index) in form.getFieldValue('items')"
            :key="k"
            v-bind="index === 0 ? formItemLayout : formItemLayoutWithOutLabel"
            label="选项"
            :required="false"
          >
            <a-input
              v-decorator="[
                `names[${k}]`,
                {
                  validateTrigger: ['change', 'blur'],
                  rules: [
                    {
                      required: true,
                      whitespace: true,
                      message: '输入选项',
                    },
                  ],
                },
              ]"
              placeholder="添加选项"
              style="width: 300px"
            />
            <a-icon
              v-if="form.getFieldValue('items').length > 1"
              class="dynamic-delete-button"
              type="minus-circle-o"
              :disabled="form.getFieldValue('items').length === 1"
              @click="() => remove(k)"
            />
          </a-form-item>
          <a-form-item v-bind="formItemLayoutWithOutLabel">
            <a-button type="dashed" style="width: 60%" @click="add">
              <a-icon type="plus" /> 添加选项
            </a-button>
          </a-form-item>
          <a-form-item v-bind="formItemLayoutWithOutLabel">
            <a-button
              type="primary"
              html-type="submit"
              :loading="submitLoading"
            > Submit </a-button>
          </a-form-item>
        </a-form>
      </a-drawer>

      <a-layout-content id="content">
        <router-view />
      </a-layout-content>
    </a-layout>
    <!-- <HelloWorld msg="Welcome to Your Vue.js App"/> -->
  </div>
</template>

<script>
import getConfig from '@/config'

const nearConfig = getConfig('testnet')
let id = 0

export default {
  name: 'Home',
  data () {
    return {
      submitLoading: false,
      visible: false,
      formItemLayout: {
        labelCol: {
          xs: { span: 24 },
          sm: { span: 3 }
        },
        wrapperCol: {
          xs: { span: 24 },
          sm: { span: 20 }
        }
      },
      formItemLayoutWithOutLabel: {
        wrapperCol: {
          xs: { span: 24, offset: 0 },
          sm: { span: 20, offset: 0 }
        }
      }
    }
  },
  beforeCreate () {
    this.form = this.$form.createForm(this, { name: 'dynamic_form_item' })
    this.form.getFieldDecorator('items', { initialValue: [], preserve: true })
    this.form.getFieldDecorator('topic_desc', {
      initialValue: '',
      preserve: true
    })
    this.form.getFieldDecorator('total_limit', {
      initialValue: 5,
      preserve: true
    })

    this.$store.commit('setAccountId', window.walletAccount.getAccountId())
    if (window.walletAccount.getAccountId()) {
      this.$store.commit('signIn')
    }
  },
  components: {},
  methods: {
    signIn: function () {
      window.walletAccount.requestSignIn(nearConfig.contractName, 'near vote')
      this.$store.commit('signIn')
    },
    signOut: function () {
      window.walletAccount.signOut()
      this.$store.commit('signOut')
      this.$store.commit('setAccountId', '')
      this.$router.go(0)
    },
    showDrawer: function () {
      this.visible = true
    },
    onClose: function () {
      this.visible = false
    },
    remove (i) {
      const { form } = this
      const items = form.getFieldValue('items')
      if (items.length === 1) {
        // return
      }
      form.setFieldsValue({
        items: items.filter((item) => item !== i)
      })
    },
    add () {
      const { form } = this
      const items = form.getFieldValue('items')
      const nextKeys = items.concat(id++)
      form.setFieldsValue({
        items: nextKeys
      })
    },
    handleSubmit (e) {
      this.submitLoading = true
      e.preventDefault()
      this.form.validateFields((err, values) => {
        if (!err) {
          console.log('Received values of form: ', values)
        }
      })
      const { form } = this
      window.contract.create_topic(
        {
          topic_desc: form.getFieldValue('topic_desc'),
          items: form.getFieldValue('names'),
          total_limit: form.getFieldValue('total_limit')
        }
        // gas (optional)
        // deposit (optional)
      ).then((res) => {
        this.visible = false
        if (res.Ok) {
          this.submitLoading = false
          this.visible = false
          this.$router.go(0)
        } else {
          this.$notification.error({ message: res.Err })
        }
      })
    }
  },
  computed: {
    updateSign () {
      return this.$store.state.isSigned
    },
    updateAccount () {
      return this.$store.state.accountId
    }
  }
}
</script>

<style>
#click_list {
  margin-right: 20px;
  font-size:large;
}
/* .home {
  height: 100vh;
} */
#header {
  background-color: rgb(217, 232, 240);
  font-size: 200%;
  box-shadow: 0 0 10px rgb(148, 139, 139);
}
#sign {
  margin-left: 30px;
  float: left;
}
#new {
  float: right;
}
#content {
}
.dynamic-delete-button {
  cursor: pointer;
  position: relative;
  top: 4px;
  font-size: 24px;
  color: #999;
  transition: all 0.3s;
}
.dynamic-delete-button:hover {
  color: #777;
}
.dynamic-delete-button[disabled] {
  cursor: not-allowed;
  opacity: 0.5;
}
</style>
