impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut sorted_products = products;
        sorted_products.sort(); // Sort products lexicographically

        let mut result = Vec::new();
        let mut left = 0;
        let mut right = sorted_products.len() - 1;

        for i in 1..=search_word.len() {
            let prefix = &search_word[0..i];
            
            // Find the range of products that match the current prefix
            while left <= right && !sorted_products[left].starts_with(prefix) {
                left += 1;
            }
            while left <= right && !sorted_products[right].starts_with(prefix) {
                right -= 1;
            }

            // Add up to 3 matching products to the result
            let mut suggestions = Vec::new();
            for j in left..=right.min(left + 2) {
                suggestions.push(sorted_products[j].clone());
            }
            result.push(suggestions);

            // If no products match the current prefix, break early
            if left > right {
                break;
            }
        }

        // Fill remaining results with empty vectors if necessary
        while result.len() < search_word.len() {
            result.push(Vec::new());
        }

        result
    }
}
