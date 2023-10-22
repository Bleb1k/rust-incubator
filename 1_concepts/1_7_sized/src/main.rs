use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
    fmt::Debug,
};

trait Storage<K, V>: Debug {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

#[allow(dead_code)]
#[derive(Debug)]
struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

impl Storage<u64, User> for HashMap<u64, User> {
    fn set(&mut self, key: u64, val: User) {
        self.insert(key, val);
    }
    fn get(&self, key: &u64) -> Option<&User> {
        self.get(key)
    }
    fn remove(&mut self, key: &u64) -> Option<User> {
        self.remove(key)
    }
}
impl Storage<u64, User> for Vec<User> {
    fn set(&mut self, key: u64, val: User) {
        if let Ok(id) = self.binary_search_by(|x| x.id.cmp(&key)) {
            self.remove(id);
            self.insert(id, val)
        } else {
            self.push(val);
        }
    }
    fn get(&self, key: &u64) -> Option<&User> {
        if let Ok(i) = self.binary_search_by(|x| x.id.cmp(key)) {
            Some(&self[i])
        } else {
            None
        }
    }
    fn remove(&mut self, key: &u64) -> Option<User> {
        if let Ok(i) = self.binary_search_by(|x| x.id.cmp(key)) {
            Some(self.remove(i))
        } else {
            None
        }
    }
}
impl Storage<u64, User> for BTreeMap<u64, User> {
    fn set(&mut self, key: u64, val: User) {
        self.insert(key, val);
    }
    fn get(&self, key: &u64) -> Option<&User> {
        self.get(key)
    }
    fn remove(&mut self, key: &u64) -> Option<User> {
        self.remove(key)
    }
}

#[derive(Debug)]
enum UserRepository {
    HashMap(HashMap<u64, User>),
    Vec(Vec<User>),
    Dynamic(Box<dyn Storage<u64, User>>), // might be any type
}
impl UserRepository {
    fn add(&mut self, val: User) {
        match self {
            UserRepository::HashMap(map) => {
                map.insert(val.id, val);
            }
            UserRepository::Vec(vec) => {
                if let Err(_) = vec.binary_search_by(|x| x.id.cmp(&val.id)) {
                    vec.push(val);
                }
            }
            UserRepository::Dynamic(stuff) => stuff.set(val.id, val),
        };
    }
    fn update(&mut self, val: User) {
        match self {
            UserRepository::HashMap(map) => {
                map.insert(val.id, val);
            }
            UserRepository::Vec(vec) => {
                if let Ok(id) = vec.binary_search_by(|x| x.id.cmp(&val.id)) {
                    vec.remove(id);
                    vec.insert(id, val)
                }
            }
            UserRepository::Dynamic(stuff) => stuff.set(val.id, val),
        };
    }
}
impl Storage<u64, User> for UserRepository {
    fn set(&mut self, key: u64, val: User) {
        match self {
            UserRepository::HashMap(map) => {
                map.insert(key, val);
            }
            UserRepository::Vec(vec) => {
                if let Ok(id) = vec.binary_search_by(|x| x.id.cmp(&key)) {
                    vec.remove(id);
                    vec.insert(id, val)
                } else {
                    vec.push(val);
                }
            }
            UserRepository::Dynamic(stuff) => stuff.set(key, val),
        };
    }
    fn get(&self, key: &u64) -> Option<&User> {
        match self {
            UserRepository::HashMap(map) => map.get(key),
            UserRepository::Vec(vec) => {
                if let Ok(i) = vec.binary_search_by(|x| x.id.cmp(key)) {
                    Some(&vec[i])
                } else {
                    None
                }
            }
            UserRepository::Dynamic(stuff) => stuff.get(key),
        }
    }
    fn remove(&mut self, key: &u64) -> Option<User> {
        match self {
            UserRepository::HashMap(map) => map.remove(key),
            UserRepository::Vec(vec) => {
                if let Ok(i) = vec.binary_search_by(|x| x.id.cmp(key)) {
                    Some(vec.remove(i))
                } else {
                    None
                }
            }
            UserRepository::Dynamic(stuff) => stuff.remove(key),
        }
    }
}

fn main() {
    use std::time::{Duration, Instant};
    {
        let mut a = UserRepository::HashMap(HashMap::new());
        let start = Instant::now();
        for i in 0..1000000 {
            a.add(User {
                id: i,
                email: Cow::from("email"),
                activated: true,
            });
            a.update(User {
                id: i,
                email: Cow::from("email2"),
                activated: true,
            });
            a.get(&i);
            a.remove(&i);
        }
        println!(
            "HashMap: {:?}",
            Duration::from_millis(start.elapsed().as_millis().try_into().unwrap_or(0))
        )
    }
    {
        let mut a: Box<dyn Storage<u64, User>> = Box::new(HashMap::new());
        let start = Instant::now();
        for i in 0..1000000 {
            a.set(
                i,
                User {
                    id: i,
                    email: Cow::from("email"),
                    activated: true,
                },
            );
            a.set(
                i,
                User {
                    id: i,
                    email: Cow::from("email2"),
                    activated: true,
                },
            );
            a.get(&i);
            a.remove(&i);
        }
        println!(
            "Dynamic(HashMap): {:?}",
            Duration::from_millis(start.elapsed().as_millis().try_into().unwrap_or(0))
        )
    }
    {
        let mut a = UserRepository::Dynamic(Box::new(BTreeMap::new()));
        let start = Instant::now();
        for i in 0..1000000 {
            a.add(User {
                id: i,
                email: Cow::from("email"),
                activated: true,
            });
            a.update(User {
                id: i,
                email: Cow::from("email2"),
                activated: true,
            });
            a.get(&i);
            a.remove(&i);
        }
        println!(
            "Dynamic(BTreeMap): {:?}",
            Duration::from_millis(start.elapsed().as_millis().try_into().unwrap_or(0))
        )
    }
    {
        let mut a = UserRepository::Vec(vec![]);
        let start = Instant::now();
        for i in 0..1000000 {
            a.add(User {
                id: i,
                email: Cow::from("email"),
                activated: true,
            });
            a.update(User {
                id: i,
                email: Cow::from("email2"),
                activated: true,
            });
            a.get(&i);
            a.remove(&i);
        }
        println!(
            "Vec: {:?}",
            Duration::from_millis(start.elapsed().as_millis().try_into().unwrap_or(0))
        )
    }
    {
        let mut a: Box<dyn Storage<u64, User>> = Box::new(Vec::new());
        let start = Instant::now();
        for i in 0..1000000 {
            a.set(
                i,
                User {
                    id: i,
                    email: Cow::from("email"),
                    activated: true,
                },
            );
            a.set(
                i,
                User {
                    id: i,
                    email: Cow::from("email2"),
                    activated: true,
                },
            );
            a.get(&i);
            a.remove(&i);
        }
        println!(
            "Dynamic(Vec): {:?}",
            Duration::from_millis(start.elapsed().as_millis().try_into().unwrap_or(0))
        )
    }

    // i don't get why dynamics are same speed as statics
    // or maybe i don't understood dispatches??? i think i do...
}
