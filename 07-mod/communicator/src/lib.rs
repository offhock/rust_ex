pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    // use super::*;
    use crate::client;
    #[test]
    fn it_works() {
        crate::client::connect();
    }
}
