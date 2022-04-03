use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// Each server occupies 5 positions in the ring.
const NUMBER_OF_POSITIONS_IN_RING: u8 = 5;

fn main() {
    println!("Hello, world!");

    let servers = vec!["A".into(), "B".into(), "C".into()];
    let ring = Ring::new(servers).expect("Should be able to create a ring");

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

    fn new(servers: Vec<Server>) -> Result<Self, String> {
        let mut ring = Self {
            servers: HashMap::new(),
        };

        for server in servers {
            ring.add_server(server)?;
        }

        Ok(ring)
    }

    fn add_server(&mut self, server: Server) -> Result<(), &str> {
        if self.servers.len() + NUMBER_OF_POSITIONS_IN_RING as usize > 255 {
            return Err("The ring is already full");
        }

        let mut inserted_count = 0;
        let mut salt = 0;
        while inserted_count < NUMBER_OF_POSITIONS_IN_RING {
            let server_hash_with_salt = hash(format!("{}_{}", &server, salt).as_str());
            salt += 1;

            match self.servers.contains_key(&server_hash_with_salt) {
                true => continue, // If we already have a server in this position of the ring, just try again with a different salt.
                false => {
                    self.servers.insert(server_hash_with_salt, server.clone());
                    inserted_count += 1;
                }
            }
        }

        Ok(())
    }

    fn _remove_server(&mut self, _server: Server) -> Option<Server> {
        // To make things simple for removing servers from the ring, we could have another HashMap
        // that has the server for keys, and a vec of positions for values.
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
    let got = Ring::new(vec!["Alice".into(), "Bob".into(), "Charlie".into()]).unwrap();
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

#[test]
fn test_add_server_conflict() {
    let mut ring = Ring::new(vec!["Alice".into()]).unwrap();
    assert_eq!(
        (1 * NUMBER_OF_POSITIONS_IN_RING) as usize,
        ring.servers.len()
    );

    // When inserting another server with conflicting keys (here, we're just
    // reusing the same Server name, so all first 5 keys conflict), it should
    // still be able to insert the new server at 5 locations.

    ring.add_server("Alice".into()).unwrap();
    assert_eq!(
        (2 * NUMBER_OF_POSITIONS_IN_RING) as usize,
        ring.servers.len()
    );
}

#[test]
fn test_cannot_add_server_to_full_ring() {
    let mut ring = Ring::new(vec![]).unwrap();

    let number_of_server_we_can_fit_in_the_ring = 255 / NUMBER_OF_POSITIONS_IN_RING as usize;
    for i in 0..number_of_server_we_can_fit_in_the_ring {
        ring.add_server(format!("Server number {}", i)).unwrap();
    }

    assert_eq!(
        Err("The ring is already full"),
        ring.add_server("Another server".to_string())
    );
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
