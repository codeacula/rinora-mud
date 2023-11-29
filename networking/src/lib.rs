use models::*;
use std::sync::mpsc::{Receiver, Sender};

mod models;

pub fn start_listening() -> (Receiver<NetworkRequest>, Sender<NetworkCommand>) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
