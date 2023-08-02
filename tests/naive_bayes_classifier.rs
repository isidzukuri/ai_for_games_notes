use games_ai_book::learning::{self};

#[test]
fn naive_bayes_classifier_test() {
    let mut classifier = learning::NaiveBayesClassifier::new();

    let cat_1_label = "cat_1".to_string();
    let cat_2_label = "cat_2".to_string();
    let cat_3_label = "cat_3".to_string();
    let cat_4_label = "cat_4".to_string();
    let cat_5_label = "cat_5".to_string();

    classifier.train(cat_1_label.clone(), "word_1".to_string());
    classifier.train(cat_1_label.clone(), "word_1".to_string());
    classifier.train(cat_1_label.clone(), "word_1".to_string());
    classifier.train(cat_1_label.clone(), "word_2".to_string());
    classifier.train(cat_2_label.clone(), "word_1".to_string());
    classifier.train(cat_3_label.clone(), "word_3".to_string());
    classifier.train(cat_4_label.clone(), "word_4".to_string());
    classifier.train(cat_5_label.clone(), "word_4".to_string());

    assert_eq!(
        *classifier
            .categories
            .get(&cat_1_label)
            .unwrap()
            .examples
            .get(&"word_2".to_string())
            .unwrap(),
        1 as i32
    );

    assert_eq!(
        *classifier
            .categories
            .get(&cat_1_label)
            .unwrap()
            .examples
            .get(&"word_1".to_string())
            .unwrap(),
        3 as i32
    );

    assert_eq!(
        *classifier
            .categories
            .get(&cat_2_label)
            .unwrap()
            .examples
            .get(&"word_1".to_string())
            .unwrap(),
        1 as i32
    );

    assert_eq!(
        *classifier
            .categories
            .get(&cat_3_label)
            .unwrap()
            .examples
            .get(&"word_3".to_string())
            .unwrap(),
        1 as i32
    );

    assert_eq!(classifier.uniq_examples_count(), 4);
    assert_eq!(classifier.total_examples_count(), 8);
    assert_eq!(
        classifier.example_in_all_categories_total_count(&"word_1".to_string()),
        4
    );
    assert_eq!(
        classifier.example_in_all_categories_total_count(&"word_2".to_string()),
        1
    );
    assert_eq!(
        classifier.example_in_all_categories_total_count(&"word_3".to_string()),
        1
    );

    let result = classifier.classify("unknown".to_string());

    assert_eq!(result.probabilities.len(), 0);
    assert_eq!(result.assumption(), None);

    let result = classifier.classify("word_1".to_string());

    assert_eq!(result.assumption().unwrap(), cat_1_label);
    assert_eq!(result.probabilities.len(), 2);

    let result = classifier.classify("word_2".to_string());

    assert_eq!(result.assumption().unwrap(), cat_1_label);
    assert_eq!(result.probabilities.len(), 1);

    let result = classifier.classify("word_3".to_string());

    assert_eq!(result.assumption().unwrap(), cat_3_label);
    assert_eq!(result.probabilities.len(), 1);

    let result = classifier.classify("word_4".to_string());

    assert_eq!(
        result.assumption().unwrap() == cat_4_label || result.assumption().unwrap() == cat_5_label,
        true
    );
    assert_eq!(result.probabilities.len(), 2);
}
