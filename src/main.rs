pub fn main() {
    #[allow(unused_must_use)] {
        api::services::server::start("127.0.0.1:8080", true);
    }
}