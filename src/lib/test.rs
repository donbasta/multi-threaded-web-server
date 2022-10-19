#[cfg(test)]
mod threadpool_test {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn purposedly_failing_test() {
        let result = 2 + 2;
        assert_ne!(result, 5);
    }
}

#[cfg(test)]
mod worker_test {
    #[test]
    fn purposedly_failing_test() {
        let result = 2 + 2;
        assert_ne!(result, 5);
    }
}
