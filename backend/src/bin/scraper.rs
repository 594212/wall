use error_chain::error_chain;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate, Text};

error_chain! {
      foreign_links {
          ReqError(reqwest::Error);
          IoError(std::io::Error);
      }
}

#[tokio::main]
async fn main() -> Result<()> {
    let res =
        reqwest::get("https://amedia.site/1413-boevoj-kontinent-2-neprevzojdennyj-klan-tan.html")
            .await?
            .text()
            .await?;

    let doc = Document::from(res.as_str());
    let description = doc
        .find(Class("pmovie__descr").descendant(Text))
        .filter_map(|n| n.as_text().map(|s| s.trim()))
        .collect::<String>();

    let title = doc
        .find(Name("h1").descendant(Text))
        .filter_map(|t| t.as_text().map(|s| s.trim()))
        .collect::<String>();

    let genre = doc
        .find(Class("animli").descendant(Name("a").descendant(Text)))
        .map(|n| n.text())
        .collect::<Vec<String>>();

    let poster = doc
        .find(Class("pmovie__poster").descendant(Name("img")))
        .flat_map(|n| n.attr("src"))
        .collect::<String>();

    let mut videos = doc
        .clone()
        .find(Attr("id", "elementb").descendant(Name("a")))
        .filter_map(|e| e.attr("data-vlnk").map(|s| s.to_string()))
        .collect::<Vec<String>>();
    videos.pop();

    println!("poster {:?}", poster);
    println!("description {:?}", description);
    println!("titles {:?}", title);
    println!("genres {:?}", genre);
    println!("videos {:?}", videos);

    Ok(())
}
