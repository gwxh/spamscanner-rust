
// #[cfg(test)]
// mod tests {
//     // use bayespam::classifier::Classifier;
//     use tokio::test;
//     use crate::SpamScanner;

//     // #[test]
//     // fn spam_test(){
//     //     let mut classifier = Classifier::new();

//     //     let spam = "Don't forget our special promotion: -30% on men shoes, only today!";
//     //     classifier.train_spam(spam);

//     //     let ham = "Hi Bob, don't forget our meeting today at 4pm.";
//     //     classifier.train_ham(ham);

//     //     let spam = "Lose up to 19% weight. Special promotion on our new weightloss.";
//     //     let is_spam = classifier.identify(spam);
//     //     assert!(is_spam);
//     // }

//     #[test]
//     #[tokio::test]
//     async fn spam_scanner_test() -> Result<(), Box<dyn std::error::Error>> {
//         let email_path = "fixtures/spam.eml".to_string();
//         let mut scanner = SpamScanner::new();
//         let result = scanner.scan(email_path).await;
//         assert!(result.is_spam);
//         Ok(())
//     }
// }
