use reqwest::Error;
use structopt::StructOpt;
use std::env;
use select::document;
use select::predicate::{*};

#[derive(StructOpt)]
struct MusidDBCli {
    song_name: String, 
    band: String,

}


#[tokio::main]
async fn main() -> Result<(), Error> {

    // CLI buildup
    let args: MusidDBCli = MusidDBCli::from_args();
    let cur_db: String = match env::var("DB_AZ_LYRICS_URL"){
        Ok(value) => value,
        Err(_err) => "".to_string(),

    };
    let song_without_spaces = args.song_name
        .replace(" ", "")
        .replace('"', "");
    println!("You chose the song: {}", args.song_name);
    println!("Your choice for db is: {}", cur_db);

    // TODO Minor Case Sensitive
    //  Build Main REqwest at website
    let res = reqwest::get("https://www.azlyrics.com/lyrics/".to_string()
        + &*args.band + "/"
        + &*song_without_spaces + ".html").await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    println!("{}", "https://www.azlyrics.com/lyrics/".to_string()
        + &*args.band + "/"
        + &*song_without_spaces + ".html");

    // Setting the body to Scrap
    let body = res.text().await?;
    // println!("Body:\n{}", body);

    let doc = document::Document::from(&*body); // Great hack for String => &str
    
    for node in doc.find(Name("b")) {
        if args.song_name.eq(&node.text()) { 
            println!("{}", node.text());
        }
    }

    Ok(())
}
