use error_chain::error_chain;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

error_chain! {
      foreign_links {
          ReqError(reqwest::Error);
          IoError(std::io::Error);
      }
}

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("https://amedia.site/1723-nesravnennyj-boevoj-duh.html")
        .await?
        .text()
        .await?;

    Document::from(res.as_str())
        .find(Attr("id", "elementb").descendant(Name("a")))
        .filter_map(|e| e.attr("data-vlnk"))
        .for_each(|x| println!("{:?}", x));

    Ok(())
}
