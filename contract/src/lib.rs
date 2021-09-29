use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::Serialize;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use std::collections::{HashMap, HashSet};

near_sdk::setup_alloc!();

#[derive(BorshDeserialize, BorshSerialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Topic {
    pub id: String,         // id格式： AccountId_高度_ts
    pub topic_desc: String, // 描述议题描述
    pub items: Vec<String>, // 议题选项,
    pub item_account_by_index: HashMap<usize, HashSet<AccountId>>, // 记录每个选项的选择人
    pub total_limit: u128,  // 总参与人数限制
    pub current: Vec<AccountId>, // 当前参与的投票人
    pub over: bool,         // 提案是否结束
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    topics: HashMap<String, Topic>, // 所有的提案, 提案id->提案
    // topics: LookupMap<String, Topic>, // 所有的提案, 提案id->提案
    search: LookupMap<AccountId, HashSet<String>>, // 每个人的提案集合, accountid-> set<topicid>
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            topics: HashMap::new(),
            // topics: LookupMap::new(b"t".to_vec()),
            search: LookupMap::new(b"s".to_vec()),
        }
    }

    // 0. 拉取议题, 每个人只能拉起他自己的议题
    pub fn list_topic(&self, account_id: AccountId) -> Option<Vec<Topic>> {
        if let Some(ids) = self.search.get(&account_id) {
            let mut topics: Vec<Topic> = Vec::new();
            for id in ids.iter() {
                if let Some(t) = self.topics.get(id) {
                    topics.push(t.clone());
                }
            }
            topics.sort_by(|a, b| b.id.cmp(&a.id));
            Some(topics)
        } else {
            None
        }
    }

    // 1. 创建议题
    pub fn create_topic(
        &mut self,
        topic_desc: String,
        items: Vec<String>,
        total_limit: u128,
    ) -> Result<bool, String> {
        let account_id = env::signer_account_id();
        if topic_desc.len() == 0 {
            return Err("议题描述不能为空".to_string());
        }
        if items.len() == 0 {
            return Err("必须有议题投票选项".to_string());
        }
        if total_limit <= 0 {
            return Err("参与人数需要大于0".to_string());
        }
        let mut t = Topic {
            id: format!(
                "{}_{}_{}",
                account_id,
                env::block_index(),
                env::block_timestamp()
            ),
            topic_desc,
            items: items.clone(),
            item_account_by_index: HashMap::new(),
            total_limit,
            current: Vec::new(),
            over: false,
        };
        for (k, _v) in items.iter().enumerate() {
            let s: HashSet<AccountId> = HashSet::new();
            t.item_account_by_index.insert(k, s);
        }
        self.topics.insert(t.id.clone(), t.clone());
        if let Some(mut topics) = self.search.get(&account_id) {
            // 有了就加入
            topics.insert(t.id);
            self.search.insert(&account_id, &topics);
        } else {
            // 没有就创建再加入
            let mut topics: HashSet<String> = HashSet::new();
            topics.insert(t.id.clone());
            self.search.insert(&account_id, &topics);
        }
        Ok(true)
    }

    // 2. 投票
    pub fn vote(&mut self, topic_id: String, item: usize) -> Result<bool, String> {
        // 一个人只能投一次票
        let account_id = env::signer_account_id();
        if let Some(topic) = self.topics.get_mut(&topic_id) {
            if topic.over {
                return Err("该议题已经结束".to_string());
            }
            if topic.current.contains(&account_id) {
                return Err("您已经投过票了".to_string());
            }
            if let Some(account_ids) = topic.item_account_by_index.get_mut(&item) {
                account_ids.insert(account_id.clone());
                topic.current.push(account_id.clone());
                if topic.total_limit == topic.current.len() as u128 {
                    topic.over = true;
                }
                return Ok(true);
            } else {
                return Err("该议题没有这个选项".to_string());
            }
        } else {
            return Err("这个议题不存在".to_string());
        }
    }

    // 3. 结束议题
    pub fn over(&mut self, topic_id: String) -> Result<bool, String> {
        // 议题不能重复结束
        if let Some(topic) = self.topics.get_mut(&topic_id) {
            if topic.over {
                return Err("该议题已经结束".to_string());
            }
            topic.over = true;
        } else {
            return Err("这个议题不存在".to_string());
        }
        Ok(true)
    }

    // 4. 查看议题
    pub fn show(&self, topic_id: String) -> Result<Topic, String> {
        if let Some(topic) = self.topics.get(&topic_id) {
            return Ok(topic.clone());
        } else {
            return Err("这个议题不存在".to_string());
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, MockedBlockchain};

    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }
    // 测试default
    #[test]
    #[should_panic]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }
    // 测试新建
    #[test]
    fn test_create_topic() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new();
        assert_eq!(
            contract.create_topic("topic1".to_string(), vec!(), 2),
            Err("必须有议题投票选项".to_string())
        );
        assert_eq!(
            contract.create_topic("".to_string(), vec!(), 2),
            Err("议题描述不能为空".to_string())
        );
        assert_eq!(
            contract.create_topic("x".to_string(), vec!("1".to_string(), "2".to_string()), 0),
            Err("参与人数需要大于0".to_string())
        );
    }

    // 测试投票
    #[test]
    fn test_vote() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new();
        assert_eq!(
            contract.create_topic("x".to_string(), vec!["1".to_string(), "2".to_string()], 5),
            Ok(true)
        );
        let topic_id = format!(
            "{}_{}_{}",
            env::signer_account_id(),
            env::block_index(),
            env::block_timestamp()
        );
        assert_eq!(
            contract.vote(topic_id.clone(), 4),
            Err("该议题没有这个选项".to_string())
        );
        assert_eq!(
            contract.vote("x".to_string(), 4),
            Err("这个议题不存在".to_string())
        );
        assert_eq!(contract.vote(topic_id.clone(), 1), Ok(true));
        assert_eq!(
            contract.vote(topic_id.clone(), 1),
            Err("您已经投过票了".to_string())
        );
    }

    // 测试结束
    #[test]
    fn test_over() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new();
        assert_eq!(
            contract.create_topic("x".to_string(), vec!["1".to_string(), "2".to_string()], 5),
            Ok(true)
        );
        assert_eq!(
            contract.over("x".to_string()),
            Err("这个议题不存在".to_string())
        );
        let topic_id = format!(
            "{}_{}_{}",
            env::signer_account_id(),
            env::block_index(),
            env::block_timestamp()
        );
        assert_eq!(contract.over(topic_id.to_string()), Ok(true));
        assert_eq!(
            contract.over(topic_id.to_string()),
            Err("该议题已经结束".to_string())
        );
    }
}
