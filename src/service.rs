
extern crate scraper;
extern crate reqwest;
use reqwest::Client;
use select::document::Document;
use select::predicate::Class;
use select::predicate::Name;
use select::predicate::Predicate;
use serde_json::json;
use serde_json::Value as JsonValue;

    pub fn get_product() -> Vec<JsonValue> {
        let url = "https://www.amazon.com/gp/most-gifted/electronics/ref=zg-mg_electronics_dw_sml";
        let img: serde_json::Value = serde_json::from_str(&&get_img(url)).unwrap();
        let name: serde_json::Value = serde_json::from_str(&get_name(url)).unwrap();
        let link: serde_json::Value = serde_json::from_str(&get_link(url)).unwrap();
        let price: serde_json::Value = serde_json::from_str(&get_price(url)).unwrap();
        let mut products = vec![];
        for i in 0..img.as_array().unwrap().len() {
            let product = json!({
                "images": img.get(i).unwrap(),
                "names": name[i],
                "links": link[i],
                "prices": price[i],
            });
        products.push(product);
        }
        products
    }
    pub fn get_document(url: &str) -> Document{
        let client = Client::new();
        let body = client.get(url).send().unwrap().text().unwrap();
        let document = Document::from(body.as_str());
        document
    }
    pub fn get_img(url: &str) -> String {
        let document = get_document(url);
        let images = document.find(Name("img").and(Class("p13n-product-image"))).filter_map(|n| n.attr("src")).collect::<Vec<_>>();
        serde_json::to_string(&images).unwrap()
    }
    pub fn get_name(url: &str) -> String {
        let document = get_document(url);
        let items  = document.find(Class("_cDEzb_p13n-sc-css-line-clamp-3_g3dy1"))
        .map(|n| n.text())
        .collect::<Vec<_>>();
        serde_json::to_string(&items).unwrap()
    }
    pub fn get_link(url: &str) -> String {
        let document = get_document(url);
        let all_links = document.find(Name("a").and(Class("a-link-normal"))).filter_map(|n| n.attr("href")).collect::<Vec<_>>();
        let links = get_corect_links(&all_links);
        serde_json::to_string(&links).unwrap()
    }
    pub fn get_corect_links<'a>(links: &'a Vec<&'a str>) -> Vec<&'a str> {
        links.iter().step_by(4).map(|x| *x).collect()
    }
    pub fn get_price(url: &str) -> String {
        let document = get_document(url);
        let prices  = document.find(Class("_cDEzb_p13n-sc-price_3mJ9Z"))
        .map(|n| n.text())
        .collect::<Vec<_>>();
        serde_json::to_string(&prices).unwrap()
    }

