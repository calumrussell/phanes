use alloy_sol_types::{sol, SolCall};

sol!(
    TornadoCash,
    r#"[{
         "constant": false,
         "inputs": [
             {
                 "name": "_tornado",
                 "type": "address"
             },
             {
                 "name": "_proof",
                 "type": "bytes"
             },
             {
                 "name": "_root",
                 "type": "bytes32"
             },
             {
                 "name": "_nullifierHash",
                 "type": "bytes32"
             },
             {
                 "name": "_recipient",
                 "type": "address"
             },
             {
                 "name": "_relayer",
                 "type": "address"
             },
             {
                 "name": "_fee",
                 "type": "uint256"
             },
             {
                 "name": "_refund",
                 "type": "uint256"
             }
         ],
         "name": "withdraw",
         "outputs": [
            {
                "name": "",
                "type": "string"
            }
         ],
         "type": "function",
         "stateMutability": "view"
     }]"#
);


pub fn decode_tc(hex: &[u8]) -> Result<TornadoCash::withdrawCall, alloy_sol_types::Error> {
    TornadoCash::withdrawCall::abi_decode(hex, false)
}

#[cfg(test)]
mod tests {
    use alloy_primitives::hex;

    #[test]
    fn test_decode() {
        //Hex calls withdraw function
        let hex = hex!("b438689f00000000000000000000000047ce0c6ed5b0ce3d3a51fdb1c52dc66a7c3c293600000000000000000000000000000000000000000000000000000000000001001efbc18073424c95fb9c5dbecdcb4826231128c234e6603971eefd8020ce2c9c29146141a3bb3fa33feee7881da76e11e999e8b5fb919aacf1024f0a5afa9edd000000000000000000000000380e141e9b7efd1cd6e8bb4fc40235bae2821405000000000000000000000000d8f1eb586ecb93745392ee254a028f1f67e1437e0000000000000000000000000000000000000000000000000054fb7f1df8ad3000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100010ad65872ef89ef024dee4ac7b5e73b1b073ea1d817ecee80dba5e0cb1f72ac1c0ca8f886ee6eeae72f7b8c630d012500471a1d538b8f260370ca647b6806640073b8ceba9787f92e71b46f92052dfd0a2ebb2ec973820bbd9a3ea939f46009211021c4eb42bb746c21160cbc2ac49c776a95e636bf063ceaa933e19d4b32f102dbdeed4fe29d3331e8349e59e4dbae4194360a4c7f74261d735ffd4b335f8e0fff48d5b602530ac8910af164b97de4351d4e55b8376ef52fb1251ec9fe83050b197e6443ad8a067bafedd65c81508c22d45e4a6d2636bfd4b8a445c32f4fca1eac601f3cf7b504c3beb982b6cb986e90fdb2c39f5651f2bc5901c213e7a158");
        //Will throw error on unwrap if sol code doesn't fit hex
        let test = super::decode_tc(&hex).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_decode_fail() {
        //Random input data hex from transaction
        let hex = hex!("a9059cbb000000000000000000000000c5b0fa1cc90f15c8407b9e6bd9b0d0eac61e7b09000000000000000000000000000000000000000000000000000000004f454620");
        //Will throw error here because hex doesn't fit
        let test = super::decode_tc(&hex).unwrap();
    }
}