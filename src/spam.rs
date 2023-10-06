use anyhow::Result;
use bayespam::classifier::Classifier;
use mail_parser::*;
use std::{
    fmt::{self, Display, Formatter},
    fs,
    sync::Arc,
    thread,
    time::Duration,
};
use thiserror::Error;
use tokio::sync::Mutex;

#[derive(Error, Debug)]
pub enum SpamScanError {
    IoError(std::io::Error),
    TokenBucketError(String),
}
impl Display for SpamScanError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SpamScanError::IoError(e) => write!(f, "IO error:{}", e),
            SpamScanError::TokenBucketError(e) => write!(f, "Token bucket Error:{}", e),
        }
    }
}

pub struct SpamScanner {
    token_bucket: Arc<Mutex<TokenBucket>>,
    classifier: Arc<Mutex<Classifier>>,
}

impl SpamScanner {
    pub fn new() -> SpamScanner {
        SpamScanner {
            token_bucket: Arc::new(Mutex::new(TokenBucket::new(5))),
            classifier: Arc::new(Mutex::new(Classifier::new())),
        }
    }

    pub async fn scan(&mut self, source: &str) -> Result<ScanResult> {
        let token_bucket = Arc::clone(&self.token_bucket);
        let classifier = Arc::clone(&self.classifier);

        let mut bucket = token_bucket.lock().await;
        let mut classifier = classifier.lock().await;
        bucket.get_token().await?;

        println!("{}线程正在执行任务", thread::current().name().unwrap());

        thread::sleep(Duration::from_secs(2));

        let email_content = fs::read_to_string(source).expect("Unable to read file");
        let message = MessageParser::default()
            .parse(email_content.as_str())
            .unwrap();
        let body = message.body_text(0).unwrap();
        classifier.train_spam("Be duly informed that because of our Western Union transfer policy, your funds will be paid to you via our Western Union Daily Transfer limit of $4,600.00 USD. This means that you will Continuously receive a daily amount of $4,600.00 USD, and this amount Can be collected from any of our numerous Western Union outlets in your current location.");

        bucket.release_token().await;

        Ok(ScanResult {
            is_spam: classifier.identify(body.trim()),
        })
    }
}

pub struct ScanResult {
    pub is_spam: bool,
}

pub struct TokenBucket {
    tokens: usize,
    max_tokens: usize,
}

impl TokenBucket {
    fn new(max_tokens: usize) -> Self {
        TokenBucket {
            tokens: max_tokens,
            max_tokens,
        }
    }

    async fn get_token(&mut self) -> Result<bool> {
        if self.tokens > 0 {
            self.tokens -= 1;
            Ok(true)
        } else {
            Err(SpamScanError::TokenBucketError(
                "Request blocked due to rate limiting.".to_string(),
            )
            .into())
        }
    }

    async fn release_token(&mut self) {
        if self.tokens < self.max_tokens {
            self.tokens += 1;
        }
    }
}

#[tokio::test]
async fn spam_scanner_test() {
    let email_path = "fixtures/spam.eml";
    let mut scanner = SpamScanner::new();
    let result = scanner.scan(email_path).await;
    let scan_result = result.expect("Request blocked due to rate limiting.");
    assert!(scan_result.is_spam);
}

#[tokio::test]
async fn token_bucket_error_test() {
    let email_path = "fixtures/spam.eml";
    let mut scanner = SpamScanner::new();
    for _ in 0..5 {
        let result = scanner.scan(email_path).await;
        assert!(result.is_ok());
    }

    let result = scanner.scan(email_path).await;
    assert!(result.is_err())
}
