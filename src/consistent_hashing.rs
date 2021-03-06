use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::{Hash, Hasher};

/// Each server occupies 5 positions in the ring.
const NUMBER_OF_POSITIONS_IN_RING: u8 = 5;

/// Server is just a string, representing the name of the server.
type Server = String;

pub struct Ring {
    servers: HashMap<u8, Server>,
}

impl Ring {
    /// Returns what server holds they key passed as a parameter
    pub fn get_server_for_key(&self, key: &str) -> Result<Server, &str> {
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

    pub fn new(servers: Vec<Server>) -> Result<Self, String> {
        let mut ring = Self {
            servers: HashMap::new(),
        };

        for server in servers {
            ring.add_server(server)?;
        }

        Ok(ring)
    }

    /// This adds a server to the ring, at "random" positions.
    /// In practice, it simply adds some salt to the server name
    /// and then hashes the value to get a position in the ring.
    /// It repeats that until we fit the server in as many positions
    /// as we wanted.
    /// If there isn't enough space to fit the server, then it
    /// returns an error instead.
    fn add_server(&mut self, server: Server) -> Result<(), &str> {
        if self.servers.len() + NUMBER_OF_POSITIONS_IN_RING as usize > 255 {
            return Err("The ring is already full");
        }

        let mut inserted_count = 0;
        let mut salt = 0;
        while inserted_count < NUMBER_OF_POSITIONS_IN_RING {
            let server_hash_with_salt = hash(format!("{}_{}", &server, salt).as_str());
            salt += 217; // Arbitrary number

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

    /// We should be able to remove a server from the ring.
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
        (25, "Alice".to_string()),
        (28, "Alice".to_string()),
        (90, "Alice".to_string()),
        (99, "Alice".to_string()),
        (191, "Alice".to_string()),
        (35, "Bob".to_string()),
        (51, "Bob".to_string()),
        (57, "Bob".to_string()),
        (81, "Bob".to_string()),
        (206, "Bob".to_string()),
        (16, "Charlie".to_string()),
        (39, "Charlie".to_string()),
        (108, "Charlie".to_string()),
        (132, "Charlie".to_string()),
        (210, "Charlie".to_string()),
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

impl Display for Ring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut server_positions: Vec<_> = self.servers.iter().collect();
        server_positions.sort_by_key(|(&pos, _server_name)| pos);

        for (position, server_name) in server_positions {
            std::fmt::Formatter::write_fmt(f, format_args!("{},{}\n", position, server_name))?;
        }

        Ok(())
    }
}

#[test]
fn test_display() {
    use std::io::Write;

    let ring = Ring {
        servers: HashMap::from([
            (203, "A".to_string()),
            (88, "A".to_string()),
            (10, "B".to_string()),
            (0, "B".to_string()),
            (137, "C".to_string()),
            (50, "C".to_string()),
        ]),
    };

    let mut output = Vec::new();
    write!(output, "{}", ring).unwrap();

    let want = r#"0,B
10,B
50,C
88,A
137,C
203,A
"#;

    let got = String::from_utf8(output).unwrap();
    assert_eq!(want, got)
}
