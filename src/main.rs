#[allow(dead_code)]
#[allow(unused_variables)]
mod product;
use crate::product::Product;

fn main() {
    let mut product = Product::new("https://skroutz.gr/s/21404653/Dell-P2720D.html");
    println!("{:#?}", product.name());
    product.update_product();
    println!("{:#?}", product.name());
}
