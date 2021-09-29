// import { Account, keyStores, Near, WalletConnection } from 'near-api-js'
import getConfig from '../config'
import * as nearAPI from 'near-api-js'
import {
  formatNearAmount,
  parseNearAmount
} from 'near-api-js/lib/utils/format'

const nearConfig = getConfig('development')
window.nearConfig = nearConfig

async function initContract () {
  // Initializing connection to the NEAR node.
  const near = await nearAPI.connect({
    deps: {
      keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore()
    },
    ...nearConfig
  })

  // Needed to access wallet login
  const walletAccount = new nearAPI.WalletConnection(near)

  // Initializing our contract APIs by contract name and configuration.
  const contract = await new nearAPI.Contract(
    walletAccount.account(),
    nearConfig.contractName,
    {
      viewMethods: ['list_topic', 'show'],
      changeMethods: ['create_topic', 'over', 'vote'],
      sender: walletAccount.getAccountId()
    }
  )
  window.walletAccount = walletAccount
  window.contract = contract
}

const getBalance = async ({ wallet }) => {
  return formatNearAmount(
    (await wallet.account().getAccountBalance()).available,
    4
  )
}

export default {
  initContract,
  getBalance,
  parseNearAmount,
  nearConfig
}
