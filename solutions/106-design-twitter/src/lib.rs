use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

pub struct Solution;

struct Twitter {
    inner: RefCell<Inner>,
}

struct Inner {
    clock: u64,
    tweets: HashMap<i32, Vec<(u64, i32)>>,
    follows: HashMap<i32, HashSet<i32>>,
}

impl Twitter {
    fn new() -> Self {
        Self {
            inner: RefCell::new(Inner {
                clock: 0,
                tweets: HashMap::new(),
                follows: HashMap::new(),
            }),
        }
    }

    fn post_tweet(&self, user_id: i32, tweet_id: i32) {
        let mut inner = self.inner.borrow_mut();
        inner.clock += 1;
        let ts = inner.clock;
        inner.tweets.entry(user_id).or_default().push((ts, tweet_id));
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let inner = self.inner.borrow();
        let mut sources: Vec<i32> = inner
            .follows
            .get(&user_id)
            .map(|s| s.iter().copied().collect())
            .unwrap_or_default();
        sources.push(user_id);

        let mut all: Vec<(u64, i32)> = sources
            .iter()
            .flat_map(|u| inner.tweets.get(u).into_iter().flatten().copied())
            .collect();

        all.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        all.into_iter().take(10).map(|(_, id)| id).collect()
    }

    fn follow(&self, follower_id: i32, followee_id: i32) {
        if follower_id == followee_id {
            return;
        }
        self.inner
            .borrow_mut()
            .follows
            .entry(follower_id)
            .or_default()
            .insert(followee_id);
    }

    fn unfollow(&self, follower_id: i32, followee_id: i32) {
        if let Some(set) = self.inner.borrow_mut().follows.get_mut(&follower_id) {
            set.remove(&followee_id);
        }
    }
}
