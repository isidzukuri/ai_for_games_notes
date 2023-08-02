use std::collections::HashMap;

#[derive(Debug)]
pub struct Category {
    pub label: String,
    pub examples: HashMap<String, i32>,
}

#[derive(Debug)]
pub struct ClassificationResult {
    pub probabilities: HashMap<String, f64>,
}

impl ClassificationResult {
    pub fn new() -> Self {
        Self {
            probabilities: HashMap::new(),
        }
    }

    pub fn assumption(&self) -> Option<String> {
        self.probabilities
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(k, _v)| k)
            .cloned()
    }
}

static DEFAULT_SMOOTHING: f64 = 1.0f64;
static DEFAULT_LIKELIHOOD: f64 = 1.0f64;

#[derive(Debug)]
pub struct NaiveBayesClassifier {
    pub categories: HashMap<String, Category>,
    pub smoothing: f64,
    pub likelihood: f64,
}

impl NaiveBayesClassifier {
    pub fn new() -> Self {
        Self {
            categories: HashMap::new(),
            smoothing: DEFAULT_SMOOTHING,
            likelihood: DEFAULT_LIKELIHOOD,
        }
    }

    pub fn train(&mut self, label: String, example: String) {
        let category = self.categories.entry(label.clone()).or_insert(Category {
            label: label.clone(),
            examples: HashMap::new(),
        });

        *category.examples.entry(example).or_insert(0) += 1;
    }

    pub fn classify(&self, example: String) -> ClassificationResult {
        let mut result = ClassificationResult::new();

        let example_in_all_categories_total_count =
            self.example_in_all_categories_total_count(&example);

        if example_in_all_categories_total_count == 0 {
            return result;
        }

        for category in self.categories.values() {
            let count_in_category = *category.examples.get(&example).unwrap_or(&0i32) as f64;

            if count_in_category == 0.0 {
                continue;
            };

            let evidence = self.total_examples_count() as f64;

            let prior = count_in_category / (example_in_all_categories_total_count as f64);

            let posterior = prior * self.likelihood / evidence;

            let normalized_posterior = (posterior + self.smoothing).log10();

            result
                .probabilities
                .insert(category.label.clone(), normalized_posterior);
        }

        result
    }

    pub fn total_examples_count(&self) -> i32 {
        self.categories
            .values()
            .map(|ctgr| ctgr.examples.values().sum::<i32>())
            .sum()
    }

    pub fn uniq_examples_count(&self) -> i32 {
        let mut uniq_examples = Vec::new();

        for category in self.categories.values() {
            for example in category.examples.keys() {
                if uniq_examples.contains(example) {
                    continue;
                };

                uniq_examples.push(example.clone());
            }
        }

        uniq_examples.len() as i32
    }

    pub fn example_in_all_categories_total_count(&self, example: &String) -> i32 {
        self.categories
            .values()
            .map(|ctgr| ctgr.examples.get(example).unwrap_or(&0i32))
            .sum()
    }
}
