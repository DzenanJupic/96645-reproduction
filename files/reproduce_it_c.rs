/// https://github.com/rust-lang/rust/issues/96645#issuecomment-1153902305
mod c {
    pub struct C;
    pub struct RecKey;
    pub struct RecSeries;
    pub struct Error;

    impl C {
        pub async fn rs(&mut self, key: &RecKey) -> Result<&RecSeries, Error> { todo!() }
    }
}
