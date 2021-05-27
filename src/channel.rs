use std::sync::mpsc;

pub struct Channel<T> {
    pub send: mpsc::Sender<T>,
    pub receive: mpsc::Receiver<T>,
}

impl<T> Channel<T> {
    pub fn pair() -> (Channel<T>, Channel<T>) {
        let (s1, r1) = mpsc::channel();
        let (s2, r2) = mpsc::channel();
        (
            Channel { send: s1, receive: r2 },
            Channel { send: s2, receive: r1 },
        )
    }
}
