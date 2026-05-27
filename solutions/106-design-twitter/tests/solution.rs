include!("../src/lib.rs");

#[test]
fn canonical_example() {
    // LC: user 1 posts tweet 5 -> news feed = [5];
    //     user 1 follows user 2; user 2 posts tweet 6 -> news feed = [6,5];
    //     user 1 unfollows user 2 -> news feed = [5].
    let tw = Twitter::new();
    tw.post_tweet(1, 5);
    assert_eq!(tw.get_news_feed(1), vec![5]);
    tw.follow(1, 2);
    tw.post_tweet(2, 6);
    assert_eq!(tw.get_news_feed(1), vec![6, 5]);
    tw.unfollow(1, 2);
    assert_eq!(tw.get_news_feed(1), vec![5]);
}

#[test]
fn empty_feed_for_new_user() {
    let tw = Twitter::new();
    assert_eq!(tw.get_news_feed(42), Vec::<i32>::new());
}

#[test]
fn feed_includes_only_own_tweets_without_follows() {
    let tw = Twitter::new();
    tw.post_tweet(1, 100);
    tw.post_tweet(2, 200);
    assert_eq!(tw.get_news_feed(1), vec![100]);
    assert_eq!(tw.get_news_feed(2), vec![200]);
}

#[test]
fn feed_is_capped_at_ten_most_recent() {
    let tw = Twitter::new();
    for tweet_id in 1..=15 {
        tw.post_tweet(1, tweet_id);
    }
    let feed = tw.get_news_feed(1);
    assert_eq!(feed.len(), 10);
    assert_eq!(feed, vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6]);
}

#[test]
fn unfollow_unknown_relationship_is_a_noop() {
    let tw = Twitter::new();
    tw.unfollow(1, 2); // shouldn't panic
    tw.post_tweet(1, 7);
    assert_eq!(tw.get_news_feed(1), vec![7]);
}

#[test]
fn cannot_follow_self() {
    // Per LC, a user shouldn't end up double-counted from following themselves.
    let tw = Twitter::new();
    tw.follow(1, 1);
    tw.post_tweet(1, 11);
    assert_eq!(tw.get_news_feed(1), vec![11]);
}

#[test]
fn feed_merges_multiple_followees_in_chronological_order() {
    let tw = Twitter::new();
    tw.post_tweet(1, 1); // ts=1
    tw.post_tweet(2, 2); // ts=2
    tw.post_tweet(3, 3); // ts=3
    tw.follow(1, 2);
    tw.follow(1, 3);
    tw.post_tweet(2, 4); // ts=4
    tw.post_tweet(3, 5); // ts=5
    assert_eq!(tw.get_news_feed(1), vec![5, 4, 3, 2, 1]);
}

#[test]
fn unfollow_removes_their_tweets_from_feed() {
    let tw = Twitter::new();
    tw.post_tweet(1, 10);
    tw.follow(1, 2);
    tw.post_tweet(2, 20);
    tw.post_tweet(2, 30);
    assert_eq!(tw.get_news_feed(1), vec![30, 20, 10]);
    tw.unfollow(1, 2);
    assert_eq!(tw.get_news_feed(1), vec![10]);
}
