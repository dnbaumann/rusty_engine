pub mod timer;

pub mod prelude {
    pub use crate::timer::Timer;
}

#[cfg(test)]
mod tests {
    #[test]
    fn rusty_core_works() {
        assert_eq!(2 + 2, 4);
    }
}
