use std::collections::hash_map::{self, DefaultHasher};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

fn main() {
    println!("Hello, world!");

    let servers = vec!["A".into(), "B".into(), "C".into()];
    let ring = Ring::new(servers);

    for key in vec!["hello", "world"] {
        println!(
            "key {} goes into server {}",
            key,
            ring.get_server_for_key(key).unwrap()
        );
    }
}

/// Server is just a string, representing the name of the server.
type Server = String;

struct Ring {
    servers: HashMap<u8, Server>,
}

impl Ring {
    /// Returns what server holds they key passed as a parameter
    fn get_server_for_key(&self, key: &str) -> Result<Server, &str> {
        if self.servers.is_empty() {
            return Err("No servers available");
        }

        let mut h = hash(key);

        loop {
            match self.servers.get(&h) {
                None => h = h.checked_add(1).or(Some(0)).unwrap(),
                Some(value) => return Ok(value.to_owned()),
            }
        }
    }

    fn new(servers: Vec<Server>) -> Self {
        let mut map = HashMap::new();
        for server in servers {
            for i in 0..5 {
                let server_hash_with_salt = hash(format!("{}_{}", &server, i).as_str());
                map.insert(server_hash_with_salt, server.clone());
            }
        }

        Self { servers: map }
    }
}

#[test]
fn test_get_server_for_key() {
    let ring = Ring {
        servers: HashMap::from([(0, "A".to_string()), (128, "B".to_string())]),
    };

    for (key, want) in vec![
        ("world", "A"),
        ("some other key", "A"),
        ("ABCDEFGH", "A"),
        ("hello", "B"),
        ("consistent hashing", "B"),
    ] {
        let got = ring.get_server_for_key(key);
        assert_eq!(Ok(want.to_string()), got, "key: {}", key);
    }
}

/// In: key
/// Out: 0..255
fn hash(key: &str) -> u8 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    u8::try_from(hasher.finish() % 255).unwrap() // since we %255, it can't fail to convert into a u8.
}

#[test]
fn test_hash() {
    for (key, want) in vec![
        ("", 154),
        ("0", 234),
        ("1", 85),
        ("00", 80),
        ("01", 112),
        ("0123-4567-89ab-cdef", 128),
        ("0123-4567-89ab-cdee", 163),
        ("1234-5678-90ab-cdef", 54),
        ("abcd-ef12-3456-7890", 236),
    ] {
        let got = hash(key);
        assert_eq!(want, got, "key: {}", key);
    }
}
