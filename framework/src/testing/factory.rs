//! Database factories for generating fake test data
//!
//! Provides Laravel-like model factories using the `fake` crate.
//!
//! # Example
//!
//! ```rust,ignore
//! use cancer_rs::testing::{Factory, Fake};
//!
//! // Define a factory for your model
//! impl Factory for User {
//!     fn definition() -> Self {
//!         User {
//!             id: 0, // Will be set by database
//!             name: Fake::name(),
//!             email: Fake::email(),
//!             created_at: Fake::datetime(),
//!         }
//!     }
//! }
//!
//! // Use in tests
//! let user = User::factory()
//!     .state(|u| u.name = "Custom Name".to_string())
//!     .create()
//!     .await?;
//! ```

use rand::Rng;
use std::marker::PhantomData;

/// Trait for models that can be created via factories
pub trait Factory: Sized {
    /// Define the default state of the model
    fn definition() -> Self;

    /// Create a factory builder for this model
    fn factory() -> FactoryBuilder<Self> {
        FactoryBuilder::new()
    }
}

/// Builder for creating model instances with customizations
pub struct FactoryBuilder<T: Factory> {
    count: usize,
    states: Vec<Box<dyn FnOnce(&mut T)>>,
    _marker: PhantomData<T>,
}

impl<T: Factory> FactoryBuilder<T> {
    /// Create a new factory builder
    pub fn new() -> Self {
        Self {
            count: 1,
            states: Vec::new(),
            _marker: PhantomData,
        }
    }

    /// Set the number of models to create
    pub fn count(mut self, count: usize) -> Self {
        self.count = count;
        self
    }

    /// Apply a state transformation to the model
    pub fn state<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut T) + 'static,
    {
        self.states.push(Box::new(f));
        self
    }

    /// Build a single model instance without persisting
    pub fn make(self) -> T {
        let mut instance = T::definition();
        for state in self.states {
            state(&mut instance);
        }
        instance
    }

    /// Build multiple model instances without persisting
    pub fn make_many(self) -> Vec<T> {
        let count = self.count;
        (0..count)
            .map(|_| {
                let builder = FactoryBuilder::<T>::new();
                builder.make()
            })
            .collect()
    }
}

impl<T: Factory> Default for FactoryBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper for generating fake data
///
/// Provides convenient methods for generating common types of fake data.
///
/// # Example
///
/// ```rust,ignore
/// use cancer_rs::testing::Fake;
///
/// let name = Fake::name();
/// let email = Fake::email();
/// let sentence = Fake::sentence();
/// ```
pub struct Fake;

impl Fake {
    /// Generate a random first name
    pub fn first_name() -> String {
        let names = [
            "James", "Mary", "John", "Patricia", "Robert", "Jennifer",
            "Michael", "Linda", "William", "Elizabeth", "David", "Barbara",
            "Richard", "Susan", "Joseph", "Jessica", "Thomas", "Sarah",
            "Charles", "Karen", "Emma", "Olivia", "Ava", "Isabella",
            "Sophia", "Mia", "Charlotte", "Amelia", "Harper", "Evelyn",
        ];
        let mut rng = rand::thread_rng();
        names[rng.gen_range(0..names.len())].to_string()
    }

    /// Generate a random last name
    pub fn last_name() -> String {
        let names = [
            "Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia",
            "Miller", "Davis", "Rodriguez", "Martinez", "Hernandez", "Lopez",
            "Gonzalez", "Wilson", "Anderson", "Thomas", "Taylor", "Moore",
            "Jackson", "Martin", "Lee", "Perez", "Thompson", "White",
            "Harris", "Sanchez", "Clark", "Ramirez", "Lewis", "Robinson",
        ];
        let mut rng = rand::thread_rng();
        names[rng.gen_range(0..names.len())].to_string()
    }

    /// Generate a random full name
    pub fn name() -> String {
        format!("{} {}", Self::first_name(), Self::last_name())
    }

    /// Generate a random email address
    pub fn email() -> String {
        let mut rng = rand::thread_rng();
        let id: u32 = rng.gen_range(1000..9999);
        let domains = ["example.com", "test.com", "mail.test", "fake.org"];
        let domain = domains[rng.gen_range(0..domains.len())];
        format!(
            "{}.{}{}@{}",
            Self::first_name().to_lowercase(),
            Self::last_name().to_lowercase(),
            id,
            domain
        )
    }

    /// Generate a safe email (always @example.com)
    pub fn safe_email() -> String {
        let mut rng = rand::thread_rng();
        let id: u32 = rng.gen_range(1000..9999);
        format!(
            "{}.{}{}@example.com",
            Self::first_name().to_lowercase(),
            Self::last_name().to_lowercase(),
            id
        )
    }

    /// Generate a random username
    pub fn username() -> String {
        let mut rng = rand::thread_rng();
        let id: u32 = rng.gen_range(100..999);
        format!(
            "{}{}{}",
            Self::first_name().to_lowercase(),
            Self::last_name().chars().next().unwrap_or('x'),
            id
        )
    }

    /// Generate a random password (for testing only)
    pub fn password() -> String {
        let mut rng = rand::thread_rng();
        let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%"
            .chars()
            .collect();
        (0..16)
            .map(|_| chars[rng.gen_range(0..chars.len())])
            .collect()
    }

    /// Generate a random sentence
    pub fn sentence() -> String {
        let words = [
            "the", "quick", "brown", "fox", "jumps", "over", "lazy", "dog",
            "lorem", "ipsum", "dolor", "sit", "amet", "consectetur", "adipiscing",
            "elit", "sed", "do", "eiusmod", "tempor", "incididunt", "ut", "labore",
            "et", "dolore", "magna", "aliqua", "enim", "ad", "minim", "veniam",
        ];
        let mut rng = rand::thread_rng();
        let count = rng.gen_range(5..12);
        let sentence: Vec<&str> = (0..count)
            .map(|_| words[rng.gen_range(0..words.len())])
            .collect();
        let mut s = sentence.join(" ");
        if let Some(c) = s.get_mut(0..1) {
            c.make_ascii_uppercase();
        }
        s.push('.');
        s
    }

    /// Generate a random paragraph
    pub fn paragraph() -> String {
        let mut rng = rand::thread_rng();
        let count = rng.gen_range(3..6);
        (0..count)
            .map(|_| Self::sentence())
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Generate random text of approximately the given length
    pub fn text(approx_length: usize) -> String {
        let mut result = String::new();
        while result.len() < approx_length {
            if !result.is_empty() {
                result.push(' ');
            }
            result.push_str(&Self::paragraph());
        }
        result.truncate(approx_length);
        result
    }

    /// Generate a random integer in a range
    pub fn number(min: i64, max: i64) -> i64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(min..=max)
    }

    /// Generate a random float in a range
    pub fn float(min: f64, max: f64) -> f64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(min..=max)
    }

    /// Generate a random boolean
    pub fn boolean() -> bool {
        let mut rng = rand::thread_rng();
        rng.gen_bool(0.5)
    }

    /// Generate a random boolean with given probability of true
    pub fn boolean_with_probability(probability: f64) -> bool {
        let mut rng = rand::thread_rng();
        rng.gen_bool(probability.clamp(0.0, 1.0))
    }

    /// Generate a random UUID v4
    pub fn uuid() -> String {
        let mut rng = rand::thread_rng();
        let mut bytes: [u8; 16] = rng.gen();

        // Set version to 4 (bits 4-7 of byte 6)
        bytes[6] = (bytes[6] & 0x0f) | 0x40;
        // Set variant to 10xx (bits 6-7 of byte 8)
        bytes[8] = (bytes[8] & 0x3f) | 0x80;

        format!(
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5],
            bytes[6], bytes[7],
            bytes[8], bytes[9],
            bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
        )
    }

    /// Generate a random phone number
    pub fn phone() -> String {
        let mut rng = rand::thread_rng();
        format!(
            "+1-{:03}-{:03}-{:04}",
            rng.gen_range(200..999),
            rng.gen_range(200..999),
            rng.gen_range(1000..9999)
        )
    }

    /// Generate a random street address
    pub fn address() -> String {
        let mut rng = rand::thread_rng();
        let streets = [
            "Main St", "Oak Ave", "Maple Dr", "Cedar Ln", "Pine Rd",
            "Elm St", "Washington Blvd", "Park Ave", "Lake Dr", "Hill Rd",
        ];
        format!(
            "{} {}",
            rng.gen_range(100..9999),
            streets[rng.gen_range(0..streets.len())]
        )
    }

    /// Generate a random city name
    pub fn city() -> String {
        let cities = [
            "New York", "Los Angeles", "Chicago", "Houston", "Phoenix",
            "Philadelphia", "San Antonio", "San Diego", "Dallas", "Austin",
            "San Jose", "Seattle", "Denver", "Boston", "Portland",
        ];
        let mut rng = rand::thread_rng();
        cities[rng.gen_range(0..cities.len())].to_string()
    }

    /// Generate a random US state
    pub fn state() -> String {
        let states = [
            "Alabama", "Alaska", "Arizona", "Arkansas", "California",
            "Colorado", "Connecticut", "Delaware", "Florida", "Georgia",
            "Hawaii", "Idaho", "Illinois", "Indiana", "Iowa",
        ];
        let mut rng = rand::thread_rng();
        states[rng.gen_range(0..states.len())].to_string()
    }

    /// Generate a random US zip code
    pub fn zip_code() -> String {
        let mut rng = rand::thread_rng();
        format!("{:05}", rng.gen_range(10000..99999))
    }

    /// Generate a random country
    pub fn country() -> String {
        let countries = [
            "United States", "United Kingdom", "Canada", "Australia",
            "Germany", "France", "Japan", "Brazil", "India", "Mexico",
        ];
        let mut rng = rand::thread_rng();
        countries[rng.gen_range(0..countries.len())].to_string()
    }

    /// Generate a random company name
    pub fn company() -> String {
        let prefixes = ["Tech", "Global", "United", "Advanced", "Premier", "Elite"];
        let suffixes = ["Solutions", "Systems", "Industries", "Corp", "Inc", "LLC"];
        let mut rng = rand::thread_rng();
        format!(
            "{} {} {}",
            prefixes[rng.gen_range(0..prefixes.len())],
            Self::last_name(),
            suffixes[rng.gen_range(0..suffixes.len())]
        )
    }

    /// Generate a random URL
    pub fn url() -> String {
        let mut rng = rand::thread_rng();
        let tlds = ["com", "org", "net", "io", "co"];
        format!(
            "https://www.{}.{}",
            Self::last_name().to_lowercase(),
            tlds[rng.gen_range(0..tlds.len())]
        )
    }

    /// Generate a random image URL (placeholder)
    pub fn image_url(width: u32, height: u32) -> String {
        format!("https://via.placeholder.com/{}x{}", width, height)
    }

    /// Generate a random hex color
    pub fn hex_color() -> String {
        let mut rng = rand::thread_rng();
        format!(
            "#{:02x}{:02x}{:02x}",
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255)
        )
    }

    /// Generate a random IPv4 address
    pub fn ipv4() -> String {
        let mut rng = rand::thread_rng();
        format!(
            "{}.{}.{}.{}",
            rng.gen_range(1..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(1..255)
        )
    }

    /// Generate a random MAC address
    pub fn mac_address() -> String {
        let mut rng = rand::thread_rng();
        format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255)
        )
    }

    /// Generate a random date in YYYY-MM-DD format
    pub fn date() -> String {
        let mut rng = rand::thread_rng();
        format!(
            "{:04}-{:02}-{:02}",
            rng.gen_range(2000..2025),
            rng.gen_range(1..=12),
            rng.gen_range(1..=28)
        )
    }

    /// Generate a random datetime in ISO 8601 format
    pub fn datetime() -> String {
        let mut rng = rand::thread_rng();
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
            rng.gen_range(2000..2025),
            rng.gen_range(1..=12),
            rng.gen_range(1..=28),
            rng.gen_range(0..24),
            rng.gen_range(0..60),
            rng.gen_range(0..60)
        )
    }

    /// Generate a random future date
    pub fn future_date() -> String {
        let mut rng = rand::thread_rng();
        format!(
            "{:04}-{:02}-{:02}",
            rng.gen_range(2025..2030),
            rng.gen_range(1..=12),
            rng.gen_range(1..=28)
        )
    }

    /// Generate a random past date
    pub fn past_date() -> String {
        let mut rng = rand::thread_rng();
        format!(
            "{:04}-{:02}-{:02}",
            rng.gen_range(2010..2024),
            rng.gen_range(1..=12),
            rng.gen_range(1..=28)
        )
    }

    /// Generate a random slug from words
    pub fn slug() -> String {
        let words = ["hello", "world", "test", "example", "demo", "sample"];
        let mut rng = rand::thread_rng();
        let count = rng.gen_range(2..4);
        (0..count)
            .map(|_| words[rng.gen_range(0..words.len())])
            .collect::<Vec<_>>()
            .join("-")
    }

    /// Pick a random element from a slice
    pub fn one_of<T: Clone>(items: &[T]) -> T {
        let mut rng = rand::thread_rng();
        items[rng.gen_range(0..items.len())].clone()
    }

    /// Pick n random elements from a slice
    pub fn many_of<T: Clone>(items: &[T], count: usize) -> Vec<T> {
        let mut rng = rand::thread_rng();
        let count = count.min(items.len());
        let mut indices: Vec<usize> = (0..items.len()).collect();

        // Fisher-Yates shuffle for first n elements
        for i in 0..count {
            let j = rng.gen_range(i..items.len());
            indices.swap(i, j);
        }

        indices[..count]
            .iter()
            .map(|&i| items[i].clone())
            .collect()
    }

    /// Generate a random credit card number (for testing only, not valid)
    pub fn credit_card() -> String {
        let mut rng = rand::thread_rng();
        format!(
            "4{:03}-{:04}-{:04}-{:04}",
            rng.gen_range(0..1000),
            rng.gen_range(0..10000),
            rng.gen_range(0..10000),
            rng.gen_range(0..10000)
        )
    }

    /// Generate random bytes
    pub fn bytes(length: usize) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        (0..length).map(|_| rng.gen()).collect()
    }

    /// Generate a random hex string
    pub fn hex(length: usize) -> String {
        Self::bytes(length / 2 + 1)
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
            .chars()
            .take(length)
            .collect()
    }
}

/// Convenience function to create a sequence of unique items
pub struct Sequence {
    current: usize,
}

impl Sequence {
    /// Create a new sequence starting at 1
    pub fn new() -> Self {
        Self { current: 0 }
    }

    /// Create a sequence starting at a specific value
    pub fn starting_at(value: usize) -> Self {
        Self { current: value.saturating_sub(1) }
    }

    /// Get the next value in the sequence
    pub fn next(&mut self) -> usize {
        self.current += 1;
        self.current
    }

    /// Get the next value as a string with prefix
    pub fn next_with_prefix(&mut self, prefix: &str) -> String {
        format!("{}{}", prefix, self.next())
    }
}

impl Default for Sequence {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fake_name() {
        let name = Fake::name();
        assert!(!name.is_empty());
        assert!(name.contains(' '));
    }

    #[test]
    fn test_fake_email() {
        let email = Fake::email();
        assert!(email.contains('@'));
        assert!(email.contains('.'));
    }

    #[test]
    fn test_fake_uuid() {
        let uuid = Fake::uuid();
        assert_eq!(uuid.len(), 36);
        assert!(uuid.contains('-'));
    }

    #[test]
    fn test_fake_sentence() {
        let sentence = Fake::sentence();
        assert!(!sentence.is_empty());
        assert!(sentence.ends_with('.'));
        // First character should be uppercase
        assert!(sentence.chars().next().unwrap().is_uppercase());
    }

    #[test]
    fn test_fake_number() {
        let num = Fake::number(1, 10);
        assert!(num >= 1 && num <= 10);
    }

    #[test]
    fn test_fake_one_of() {
        let options = vec!["a", "b", "c"];
        let choice = Fake::one_of(&options);
        assert!(options.contains(&choice));
    }

    #[test]
    fn test_sequence() {
        let mut seq = Sequence::new();
        assert_eq!(seq.next(), 1);
        assert_eq!(seq.next(), 2);
        assert_eq!(seq.next(), 3);
    }

    #[test]
    fn test_sequence_with_prefix() {
        let mut seq = Sequence::new();
        assert_eq!(seq.next_with_prefix("user_"), "user_1");
        assert_eq!(seq.next_with_prefix("user_"), "user_2");
    }

    // Test factory pattern with a simple struct
    struct TestModel {
        id: i32,
        name: String,
        email: String,
    }

    impl Factory for TestModel {
        fn definition() -> Self {
            Self {
                id: Fake::number(1, 1000) as i32,
                name: Fake::name(),
                email: Fake::email(),
            }
        }
    }

    #[test]
    fn test_factory_make() {
        let model = TestModel::factory().make();
        assert!(!model.name.is_empty());
        assert!(model.email.contains('@'));
    }

    #[test]
    fn test_factory_with_state() {
        let model = TestModel::factory()
            .state(|m| m.name = "Custom Name".to_string())
            .make();
        assert_eq!(model.name, "Custom Name");
    }

    #[test]
    fn test_factory_make_many() {
        let models = TestModel::factory().count(5).make_many();
        assert_eq!(models.len(), 5);
    }
}
