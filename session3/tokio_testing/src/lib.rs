async fn double(n: u32) -> u32 {
    n * 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn will_not_compile() {
        // assert_eq!(double(2).await, 4); // dont compile

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        assert_eq!(rt.block_on(double(2)), 4)
    }

    // #[tokio::test] // this way it does the whole line 18 from test above
    #[tokio::test(flavor = "multi_thread")]
    async fn the_easy_way() {
        assert_eq!(double(2).await, 4);
    }
}
