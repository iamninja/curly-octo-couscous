extern crate reqwest;
extern crate select;

use select::predicate::Class;

use crate::price::Price;

/// Representing a product
pub struct Product {
    /// Product name
    name: String,
    /// Product's lowest price
    pub price: Price,
    /// Product's url
    url: String,
}

impl Product {
    /// Returns a product with the given name and price
    ///
    /// # Arguments
    ///
    /// * `name` - A string representing the name
    /// * `price` - Product's price
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use doc::Person;
    /// let person = Person::new("Monitor A", "123");
    /// ```
    pub fn new(url: &str) -> Product {
        let details = Product::skroutz_prices(String::from(url));

        Product {
            name: String::from(details[0].as_str()),
            price: Product::price_from_string(String::from(details[1].as_str())),
            url: String::from(url),
        }
    }

    /// Gives a friendly hello!
    ///
    /// Says "Hello, [name]" to the `Product` it is called on.
    pub fn name(&self) -> String {
        String::from(&self.name)
    }

    pub fn price(&self) -> String {
        self.price.printable()
    }

    /// Returns the lowest price found
    /// (the first .price found in soup)
    fn skroutz_prices(url: String) -> [String; 2] {
        let resp = reqwest::blocking::get(url.as_str()).unwrap();
        // assert!(resp.status().is_success());

        let document = select::document::Document::from_read(resp).unwrap();

        // let mut price = String::new();
        let mut name = String::from("Product");
        let mut price = String::from("0");

        for node in document.find(Class("page-title")).take(1) {
            name = node.text();
            // println!("{}", name)
        }
        for node in document.find(Class("price")).take(1) {
            price = node.text();
            // println!("{}", price)
        }

        // return Product::new("Monitor A".to_string(), price.to_string());
        [name, price]
    }

    pub fn update_product(&mut self) {
        let url = &self.url;
        let details = Product::skroutz_prices(String::from(url.as_str()));
        self.name = String::from(details[0].as_str());
        self.price = Product::price_from_string(String::from(details[1].as_str()));
    }

    pub fn get_vector(&mut self) -> Vec<String> {
        return vec![self.name(), self.price()];
    }

    fn price_from_string(string: String) -> Price {
        Price::new(
            string
                .replace("€", "")
                .replace(",", ".")
                .trim()
                .parse()
                .unwrap_or(0_f64),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getting_works() {
        let mut prod = Product {
            name: String::from("Testing 1"),
            price: Product::price_from_string(String::from("123")),
            url: String::from("http://testing.test"),
        };
        assert_eq!(prod.price.printable(), "123.00 €");
        assert_eq!(prod.name, "Testing 1");
    }

    #[test]
    fn price_from_string_works() {
        let string1 = String::from("123 €");
        let string2 = String::from("123,45 €");
        let string3 = String::from(" 124,78    €   ");
        let string4 = String::from("asdf");
        assert_eq!(Product::price_from_string(string1).printable(), "123.00 €");
        assert_eq!(Product::price_from_string(string2).printable(), "123.45 €");
        assert_eq!(Product::price_from_string(string3).printable(), "124.78 €");
        assert_eq!(Product::price_from_string(string4).printable(), "0.00 €");
    }

    #[test]
    fn get_vector_works() {
        let mut prod = Product {
            name: String::from("Testing 1"),
            price: Product::price_from_string(String::from("123")),
            url: String::from("http://testing.test"),
        };
        assert_eq!(prod.get_vector(), vec!["Testing 1", "123.00 €"]);
    }
}
