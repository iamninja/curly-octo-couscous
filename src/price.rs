#![allow(dead_code)]
use std::fmt;

/// Currency enumerator
enum Currency {
    Euro,
    USD,
}

/// Representing a price
pub struct Price {
    /// Price value
    value: f64,
    currency: Currency,
}

impl Price {
    /// Returns a Price
    ///
    /// # Arguments
    ///
    /// * `value` - the price...
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use doc::Person;
    /// let person = Person::new("Monitor A", "123");
    /// ```
    pub fn new(value: f64) -> Price {
        let value: f64 = value;
        Price {
            value,
            currency: Currency::Euro,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn currency(&self) -> &str {
        match self.currency {
            Currency::Euro => "€",
            Currency::USD => "$",
        }
    }

    pub fn printable(&self) -> String {
        format!("{:.2} {}", self.value(), self.currency())
    }
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} {}", self.value(), self.currency())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_works() {
        let price = Price::new(123_f64);
        assert_eq!(price.value(), 123_f64);
        let price = Price::new(283.99);
        assert_eq!(price.value(), 283.99);
    }

    #[test]
    fn printable_works() {
        let price = Price::new(123_f64);
        assert_eq!(price.printable(), "123.00 €");
        let price = Price::new(283.99);
        assert_eq!(price.printable(), "283.99 €");
    }
}
