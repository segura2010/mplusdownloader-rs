
mod mangaplus_api;
mod mangaplus_pb;

fn main() {
    println!("Hello, world!");
    let mut mplus = mangaplus_api::MangaPlusClient::new();
    println!("{:?}", mplus.get_device_secret());
}
