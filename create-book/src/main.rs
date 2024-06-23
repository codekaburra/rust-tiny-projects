//main.rs
use regex::Regex;
use reqwest::Error as reqwestError;
use std::io::{Error, ErrorKind};
// use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};

async fn get_page(path: &str) -> Result<String, reqwestError> {
    let response = reqwest::get(["https://www.xbanxia.com/", path].concat()).await?;
    let body = response.text().await?;
    let (temp1, temp2) = body.split_once("<script>\n     (adsbygoogle = window.adsbygoogle || []).push({});\n</script>").unwrap();
    // let (header1, header2) = temp1.split_once("<li class=\"next\">下一章︰<a href=\"").unwrap();
    // let (nextUrl, header3) = header2.split_once("\" rel=\"next\">").unwrap();
    let (content, footer) = temp2.split_once("<script async src=\"https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client=ca-pub-4380028352467606\"\n     crossorigin=\"anonymous\"></script>").unwrap();
    
    // println!("{:?}", content);
    println!("{:?}", content);
    // println!("Body:\n{}", body);

    // let re = Regex::new(r"#([a-z0-9_]+)").unwrap();

    // for cap in re.captures_iter(&body) {
    //     println!("Hashtag: {}", &cap[1]);
    // }
    Ok(content.to_string())
}
// #[derive(Debug, Serialize, Deserialize)]
#[derive(Debug)]
struct ChapterDetails {
    name: String,
    href: String,
    name2: String,
}
async fn get_page_list(url: &str) -> Result<Vec<ChapterDetails>, Error> {
    let response = reqwest::get(url).await?;
    println!("Status: {}", response.status());

    let body = response.text().await?;
    let (temp1, temp2) = body.split_once("<!--正文-->\n        \n        \n        \n        ").unwrap();
    // let (header1, header2) = temp1.split_once("<li class=\"next\">下一章︰<a href=\"").unwrap();
    // let (nextUrl, header3) = header2.split_once("\" rel=\"next\">").unwrap();
    let (content, footer) = temp2.split_once("</ul>").unwrap();
    let list: Vec<&str> = content.split('\n').collect();
    // println!("{:?}", list);
    
    // let re = Regex::new(r"I am (\w+)\. I am (\w+) years old\.").unwrap();
    let re = Regex::new(r#"<li><a\s+target="_blank"\s+title="([^"]+)"\s+href="([^"]+)">([^<]+)</a></li>"#).unwrap();

    let result: Vec<ChapterDetails> = list
        .iter()
        .filter_map(|input| {
    println!("{:?}", input);
    re.captures(input).map(|cap| ChapterDetails {
                name: cap[1].to_string(),
                href: cap[2].to_string(),
                name2: cap[3].to_string(),
                // age: cap[2].parse().unwrap(),
            })
        })
        .collect();

    println!("{:?}", result);

    Ok(result)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let file_path = "output.txt";
    let mut file = File::create(file_path)?;
    let mut contents: Vec<String> = vec![];

    let chapters = get_page_list("https://www.xbanxia.com/books/304044.html").await?;
    for item in chapters.iter() {
        contents.push(get_page(&item.href).await?)
    }
    file.write_all(contents.join("\n").as_bytes());
    
    Ok(())
}
