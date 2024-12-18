use std::fmt::format;
use futures::Future;

use egg_mode::{KeyPair, Token};
use egg_mode::tweet::DraftTweet;
use egg_mode;
use tokio;
use log::LevelFilter;
use fern::Dispatch;
use egg_mode::stream::{filter, StreamMessage};
use tokio::sync::futures;

// const CLIENT_ID: &str = "TU12MTVIcjVDdHdLNEh6VFFZY1U6MTpjaQ";
// const CLIENT_SECRET: &str = "ivmEp12m06sfM7c-Uu_5GsDwNmJUJ2AyojHywOZsXAB58hB6Hb";

#[tokio::main]
async fn main() {
    setup_logging().expect("Unable to initialize personal tweet logging");
    setup_tweets_logging().expect("Unable to initialize tweets logging");
    // log::info!("This is a info message");
    // log::error!("This is a error message");
    let tweet_message = String::from("Hello Twitter");
    /*
    tokio::runtime::Builder::enable_io();
    tokio::runtime::Builder::new_current_thread().build().unwrap().block_on(
    send_tweet(tweet_message)
    );
    */
    send_tweet(tweet_message).await;
}


async fn send_tweet(tweet: String) {
    println!("Send tweet function called");
    let consumer_key: KeyPair = KeyPair::new("3Stoqah5IAnUNvugCp9Xkvp3g", "HgQ1UGq1u1jRQFBHBQUPmHJkGwQdV6iYuq0eyTTvwkvCvHUfZI");
    let access_token: KeyPair = KeyPair::new("1829625659942957057-ofqddGw0K8arwCajMSLKWrcZB4nlUy", "iFFPetePoceiVKLraBdCSu8oRPtlgfaBOOmqkvsoTy28a");
    let token: Token = Token::Access { consumer: consumer_key, access: access_token };
    let draft = DraftTweet::new(tweet);
    try {
        let tweet_result = draft.send(&token).await;
    } 
    if tweet_result.is_err() {
        log::error!("Tweet was not sent");
    } else {
        log::info!("Tweet was sent");
    }
}

fn setup_logging() -> Result<(), fern::InitError> {
    let logger = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!("[{}][{}] {}", record.target(), record.level(), message))
        }).level(LevelFilter::Info)
        .chain(fern::log_file("output.log")?)
        .apply();
    logger?;
    Ok(())
}

fn setup_tweets_logging() -> Result<(), fern::InitError> {
    Ok(())
}

fn receive_tweet_stream() {
    const USER_ID: &str = "";
    let consumer_key: KeyPair = KeyPair::new("3Stoqah5IAnUNvugCp9Xkvp3g", "HgQ1UGq1u1jRQFBHBQUPmHJkGwQdV6iYuq0eyTTvwkvCvHUfZI");
    let access_token: KeyPair = KeyPair::new("1829625659942957057-ofqddGw0K8arwCajMSLKWrcZB4nlUy", "iFFPetePoceiVKLraBdCSu8oRPtlgfaBOOmqkvsoTy28a");
    let token: Token = Token::Access { consumer: consumer_key, access: access_token };
    let tracked_topics: [&str; 4] = ["rustlang", "javascript", "python", "java"];
    let stream = filter()
        .track(&tracked_topics)
        .start(&token);
}

async fn search_user(user_query: String) {
    let consumer_key: KeyPair = KeyPair::new("3Stoqah5IAnUNvugCp9Xkvp3g", "HgQ1UGq1u1jRQFBHBQUPmHJkGwQdV6iYuq0eyTTvwkvCvHUfZI");
    let access_token: KeyPair = KeyPair::new("1829625659942957057-ofqddGw0K8arwCajMSLKWrcZB4nlUy", "iFFPetePoceiVKLraBdCSu8oRPtlgfaBOOmqkvsoTy28a");
    let token: Token = Token::Access { consumer: consumer_key, access: access_token };
    // egg_mode::user::search("girls", &token).take(10).try_for_each(|resp| {
    //   println!("{}", resp.screen_name);
    // }).await.unwrap();
    // futures::future::ready(Ok(()))
}