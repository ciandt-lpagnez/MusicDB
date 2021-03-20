use structopt::StructOpt;

#[derive(StructOpt)]
struct MusidDBCli {
    song_name: String, 

}


fn main(){

    let args: MusidDBCli = MusidDBCli::from_args();
    println!("You chose the song: {}", args.song_name);
}