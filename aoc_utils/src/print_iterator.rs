pub trait PrintIterator<T> {
    fn print(&mut self, message: &str) -> Box<dyn Iterator<Item = T>>;
}

// TODO: remove 'static, and pass self instead
impl<T: std::fmt::Debug + 'static, U: Iterator<Item = T>> PrintIterator<T> for U {
    fn print(&mut self, message: &str) -> Box<dyn Iterator<Item = T>> {
        let v = self.collect::<Vec<_>>();
        println!("{}: {:?}", message, v);
        Box::new(v.into_iter())
    }
}
