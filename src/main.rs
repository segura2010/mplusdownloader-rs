
mod mangaplus_api;
mod mangaplus_pb;

fn main() {
    println!("Hello, world!");
    let mut mplus = mangaplus_api::MangaPlusClient::new();
    println!("{:?}", mplus.get_device_secret());
    println!("{:?}", mplus.get_title_details(100003));
    println!("{:?}", mplus.get_pages(1016705));
}
