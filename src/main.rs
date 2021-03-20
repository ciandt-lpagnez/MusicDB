use reqwest::Error;
use structopt::StructOpt;
use std::env;

#[derive(StructOpt)]
struct MusidDBCli {
    song_name: String, 

}


#[tokio::main]
async fn main() -> Result<(), Error> {

    // CLI buildup
    let args: MusidDBCli = MusidDBCli::from_args();
    let cur_db: String = match env::var("DB_AZ_LYRICS_URL"){
        Ok(value) => value,
        Err(_err) => "".to_string(),

    };
    println!("You chose the song: {}", args.song_name);
    println!("Your choice for db is: {}", cur_db);

    //  Build Main REqwest at website
    let res = reqwest::get("http://httpbin.org/get").await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    // Setting the body to Scrap
    let body = res.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}