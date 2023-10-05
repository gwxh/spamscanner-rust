
use bayespam::classifier::{self, Classifier};
use mail_parser::{core::message, *};
use std::{fs, sync::Arc, time::Duration};
use tokio::sync::Mutex;

pub struct SpamScanner {
    token_bucket: Arc<Mutex<TokenBucket>>,
    classifier: Arc<Mutex<Classifier>>,
}

impl SpamScanner {
    pub fn new() -> SpamScanner {
        SpamScanner {
            token_bucket: Arc::new(Mutex::new(TokenBucket::new(5, Duration::from_secs(1)))),
            classifier: Arc::new(Mutex::new(Classifier::new())),
        }
    }

    pub async fn scan(&mut self, source: String) -> ScanResult {
        let token_bucket: Arc<Mutex<TokenBucket>> = Arc::clone(&self.token_bucket);
        let classifier = Arc::clone(&self.classifier);

        let handle = tokio::spawn(async move {
            let mut bucket = token_bucket.lock().await;
            let mut classifier = classifier.lock().await;
            if bucket.get_token().await {
                let email_content = fs::read_to_string(source).expect("Unable to read file");
                let message = MessageParser::default()
                    .parse(email_content.as_str())
                    .unwrap();
                let body = message.body_text(0).unwrap();
                classifier.train_spam("Be duly informed that because of our Western Union transfer policy, your funds will be paid to you via our Western Union Daily Transfer limit of $4,600.00 USD. This means that you will Continuously receive a daily amount of $4,600.00 USD, and this amount Can be collected from any of our numerous Western Union outlets in your current location.");
                classifier.identify(body.trim())
            } else {
                panic!("令牌不足，请求被限制");
            }
        });

        return ScanResult {
            is_spam: handle.await.unwrap(),
        };
    }
}

pub struct ScanResult {
    pub is_spam: bool,
}

pub struct TokenBucket {
    tokens: usize,
    max_tokens: usize,
    refill_interval: Duration,
}

impl TokenBucket {
    fn new(max_tokens: usize, refill_interval: Duration) -> Self {
        TokenBucket {
            tokens: max_tokens,
            max_tokens,
            refill_interval,
        }
    }

    async fn get_token(&mut self) -> bool {
        if self.tokens > 0 {
            self.tokens -= 1;
            return true;
        } else {
            return false;
        }
    }

    async fn release_token(&mut self) {
        if self.tokens < self.max_tokens {
            self.tokens += 1;
        }
    }
}

async fn processed(token_bucket: Arc<Mutex<TokenBucket>>) {
    let mut bucket = token_bucket.lock().await;
    if bucket.get_token().await {
        // 有足够的令牌，处理请求
        println!("Request processed.");
    } else {
        // 令牌不足，请求被限制
        println!("Request blocked due to rate limiting.");
    }
}


#[test]
#[tokio::test]
async fn spam_scanner_test() {
    let email_path = "fixtures/spam.eml".to_string();
    let mut scanner = ::new();
    let result = scanner.scan(email_path).await;
    assert!(result.is_spam);
}