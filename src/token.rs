// use std::{
//     sync::Arc,
//     time::Duration,
// };

// use tokio::sync::Mutex;

// pub struct TokenBucket {
//     tokens: usize,
//     max_tokens: usize,
//     refill_interval: Duration,
// }

// impl TokenBucket {
//     fn new(max_tokens: usize, refill_interval: Duration) -> Self {
//         TokenBucket {
//             tokens: max_tokens,
//             max_tokens,
//             refill_interval,
//         }
//     }

//     async fn get_token(&mut self) -> bool {
//         if self.tokens > 0 {
//             self.tokens -= 1;
//             return true;
//         } else {
//             return false;
//         }
//     }

//     async fn release_token(&mut self) {
//         if self.tokens < self.max_tokens {
//             self.tokens += 1;
//         } 
//     }
// }

// async fn processed(token_bucket: Arc<Mutex<TokenBucket>>) {
//     let mut bucket = token_bucket.lock().await;
//     if bucket.get_token().await {
//         // 有足够的令牌，处理请求
//         println!("Request processed.");
//     } else {
//         // 令牌不足，请求被限制
//         println!("Request blocked due to rate limiting.");
//     }
// }

// #[tokio::main]
// async fn main() {
//     let token_bucket = Arc::new(Mutex::new(TokenBucket::new(5, Duration::from_secs(1))));

//     for _ in 0..10 {
//         //克隆令牌桶的引用
//         let token_bucket: Arc<Mutex<TokenBucket>> = Arc::clone(&token_bucket);

//         tokio::spawn(async move {
//             processed(token_bucket).await;
//         });
//     }

//        // 等待所有任务完成
//        tokio::time::sleep(Duration::from_secs(2)).await;
// }