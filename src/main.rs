use local_ip_address::local_ip;
use qr2term::print_qr;
use std::env;
use std::path::Path;
use warp::Filter;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args[1].clone();

    let file_name = Path::new(&path).file_name().unwrap().to_str().unwrap();

    let my_local_ip = local_ip().unwrap();
    let url = format!("http://{}:3030/{}", my_local_ip, file_name);

    println!("{}", url);
    print_qr(url).unwrap();

    let file = warp::get()
        .and(warp::fs::file(path))
        .with(warp::reply::with::header(
            "Content-Disposition",
            "attachment",
        ));
    warp::serve(file).run((my_local_ip, 3030)).await;
}
