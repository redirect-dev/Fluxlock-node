pub fn build_transfer_message(
    from: &Vec<u8>,
    to: &Vec<u8>,
    amount: u128,
    nonce: u64,
) -> Vec<u8> {
    let mut msg = vec![];
    msg.extend(from);
    msg.extend(to);
    msg.extend(&amount.to_le_bytes());
    msg.extend(&nonce.to_le_bytes());
    msg
}