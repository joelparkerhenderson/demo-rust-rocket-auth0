/// When we trigger Auth0, we can optionally send along a
/// state string, such as a random one-time use string,
/// a.k.a. a nonce. This is to mitigate CSRF attacks.
///
/// Our implementation uses random 128 bits as a hex string,
/// because that's the same general shape as a UUID or GUID.
/// Any other approach or implementation is fine too.
///
pub fn nonce() -> String {
    use rand::Rng;
    let mut bytes = [0u8; 16];
    rand::thread_rng().fill(&mut bytes);
    hex::encode(&bytes)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_nonce() {
        let x = crate::util::nonce::nonce();
        assert!(!x.is_empty());
    }

}
