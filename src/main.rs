use consistent_hashing::Ring;

mod consistent_hashing;

fn main() {
    let servers = vec!["A".into(), "B".into(), "C".into()];
    let ring = Ring::new(servers).expect("Should be able to create a ring");

    for key in vec!["hello", "world", "something", "something else"] {
        println!(
            "key `{}` goes into server `{}`",
            key,
            ring.get_server_for_key(key).unwrap()
        );
    }
}
