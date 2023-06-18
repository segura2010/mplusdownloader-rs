// to generate Rust structs from .proto file use:
// pb-rs.exe src/mangaplus_pb.proto -D --owned
// the -owned and -D flags are very important to make it work, since not using them will make it
// unable to return the proto structs to use them out of the API functions

mod mangaplus_api;
mod mangaplus_pb;

fn main() {
    let mut mplus = mangaplus_api::MangaPlusClient::new();
    println!("{:?}", mplus.get_device_secret());
    println!("{:?}", mplus.get_title_details(100003));
    println!("{:?}", mplus.get_pages(1016705));
}
