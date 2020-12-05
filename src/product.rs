extern crate reqwest;
extern crate select;

use select::predicate::Class;

/// Representing a product
pub struct Product {
    /// Product name
    name: String,
    /// Product's lowest price
    price: String,
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
            price: String::from(details[1].as_str()),
            url: String::from(url),
        }
    }

    /// Gives a friendly hello!
    ///
    /// Says "Hello, [name]" to the `Product` it is called on.
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn price(&self) -> &String {
        &self.price
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
        self.price = String::from(details[1].as_str());
    }
}
