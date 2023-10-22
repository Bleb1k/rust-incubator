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

trait CommandHandler<C: Command> {
    type Context: ?Sized;
    type Result;

    fn handle_command(&self, cmd: &C, ctx: &Self::Context) -> Self::Result;
}

impl CommandHandler<CreateUser> for User {
    type Context = dyn UserRepository;
    type Result = Result<(), UserError>;

    fn handle_command(&self, cmd: &C, ctx: &Self::Context) -> Self::Result {
        // Here we operate with UserRepository
        // via its trait object: &dyn UserRepository
    }
}

fn main() {
    // use std::time::{Duration, Instant};
    // {
    //     let mut a = UserRepository::HashMap(HashMap::new());
    //     let start = Instant::now();
    //     for i in 0..1000000 {
    //         a.add(User {
    //             id: i,
    //             email: Cow::from("email"),
    //             activated: true,
    //         });
    //         a.update(User {
    //             id: i,
    //             email: Cow::from("email2"),
    //             activated: true,
    //         });
    //         a.get(&i);
    //         a.remove(&i);
    //     }
    //     println!(
    //         "HashMap: {:?}",
    //         Duration::from_millis(start.elapsed().as_millis().try_into().unwrap_or(0))
    //     )
    // }
    // {
    //     let mut a: Box<dyn Storage<u64, User>> = Box::new(HashMap::new());
    //     let start = Instant::now();
    //     for i in 0..1000000 {
    //         a.set(
    //             i,
    //             User {
    //                 id: i,
    //                 email: Cow::from("email"),
    //                 activated: true,
    //             },
    //         );
    //         a.set(
    //             i,
    //             User {
    //                 id: i,
    //                 email: Cow::from("email2"),
    //                 activated: true,
    //             },
    //         );
    //         a.get(&i);
    //         a.remove(&i);
    //     }
    //     println!(
    //         "Dynamic(HashMap): {:?}",
    //         Duration::from_millis(start.elapsed().as_millis().try_into().unwrap_or(0))
    //     )
    // }
    // {
    //     let mut a = UserRepository::Dynamic(Box::new(BTreeMap::new()));
    //     let start = Instant::now();
    //     for i in 0..1000000 {
    //         a.add(User {
    //             id: i,
    //             email: Cow::from("email"),
    //             activated: true,
    //         });
    //         a.update(User {
    //             id: i,
    //             email: Cow::from("email2"),
    //             activated: true,
    //         });
    //         a.get(&i);
    //         a.remove(&i);
    //     }
    //     println!(
    //         "Dynamic(BTreeMap): {:?}",
    //         Duration::from_millis(start.elapsed().as_millis().try_into().unwrap_or(0))
    //     )
    // }
    // {
    //     let mut a = UserRepository::Vec(vec![]);
    //     let start = Instant::now();
    //     for i in 0..1000000 {
    //         a.add(User {
    //             id: i,
    //             email: Cow::from("email"),
    //             activated: true,
    //         });
    //         a.update(User {
    //             id: i,
    //             email: Cow::from("email2"),
    //             activated: true,
    //         });
    //         a.get(&i);
    //         a.remove(&i);
    //     }
    //     println!(
    //         "Vec: {:?}",
    //         Duration::from_millis(start.elapsed().as_millis().try_into().unwrap_or(0))
    //     )
    // }
    // {
    //     let mut a: Box<dyn Storage<u64, User>> = Box::new(Vec::new());
    //     let start = Instant::now();
    //     for i in 0..1000000 {
    //         a.set(
    //             i,
    //             User {
    //                 id: i,
    //                 email: Cow::from("email"),
    //                 activated: true,
    //             },
    //         );
    //         a.set(
    //             i,
    //             User {
    //                 id: i,
    //                 email: Cow::from("email2"),
    //                 activated: true,
    //             },
    //         );
    //         a.get(&i);
    //         a.remove(&i);
    //     }
    //     println!(
    //         "Dynamic(Vec): {:?}",
    //         Duration::from_millis(start.elapsed().as_millis().try_into().unwrap_or(0))
    //     )
    // }

    // i don't get why dynamics are same speed as statics
    // or maybe i don't understood dispatches??? i think i do...
}
