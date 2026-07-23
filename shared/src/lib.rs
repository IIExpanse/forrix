pub mod messages {
    use tonic::include_proto;

    include_proto!("grpc.tradeapi.v1.marketdata");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
