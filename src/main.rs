use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// Each server occupies 5 positions in the ring.
const NUMBER_OF_POSITIONS_IN_RING: u8 = 5;

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
        let mut ring = Self {
            servers: HashMap::new(),
        };

        for server in servers {
            ring.add_server(server);
        }

        ring
    }

    fn add_server(&mut self, server: Server) {
        let mut inserted_count = 0;
        let mut salt = 0;
        while inserted_count < NUMBER_OF_POSITIONS_IN_RING {
            let server_hash_with_salt = hash(format!("{}_{}", &server, salt).as_str());
            salt += 1;

            // TODO: handle occupied position
            self.servers.insert(server_hash_with_salt, server.clone());
            inserted_count += 1;
        }
    }

    fn _remove_server(&mut self, _server: Server) {
        todo!()
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

#[test]
fn test_new_ring() {
    let got = Ring::new(vec!["Alice".into(), "Bob".into(), "Charlie".into()]);
    let want = HashMap::from([
        (28, "Alice".into()),
        (39, "Alice".into()),
        (131, "Alice".into()),
        (148, "Alice".into()),
        (219, "Alice".into()),
        (51, "Bob".into()),
        (161, "Bob".into()),
        (186, "Bob".into()),
        (203, "Bob".into()),
        (236, "Bob".into()),
        (94, "Charlie".into()),
        (106, "Charlie".into()),
        (135, "Charlie".into()),
        (196, "Charlie".into()),
        (210, "Charlie".into()),
    ]);

    assert_eq!(want, got.servers);
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
