use std::ops::Deref;

use rand::seq::SliceRandom;

struct EmailString(String);

impl EmailString {
    fn new(email: &str) -> Self {
        let _dot_ch = 0;
        let _at_ch = 0;
        let at_index = email.find('@').expect("invalid email");
        if email[at_index..]
            .chars()
            .fold(0, |_a, c| if c == '.' { c as i32 + 1 } else { c as i32 })
            != 1
        {
            panic!("invalid email");
        }
        Self(email.to_owned())
    }
}

impl Deref for EmailString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Into<String>> From<T> for EmailString {
    fn from(data: T) -> Self {
        EmailString(data.into())
    }
}

struct Random<T>([T; 3]);

impl<T> Random<T> {
    fn new(a: T, b: T, c: T) -> Self {
        Self([a, b, c])
    }

    fn get(&self) -> &T {
        self.choose(&mut rand::thread_rng()).unwrap()
    }
}

impl<T> Deref for Random<T> {
    type Target = [T; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, U> From<T> for Random<U>
where
    T: Into<Vec<U>>,
{
    fn from(data: T) -> Self {
        let data = data.into();
        if data.len() < 3 {
            panic!("data must have 3 elements");
        }
        let a = unsafe {
            [
                data.as_ptr().read(),
                data.as_ptr().add(1).read(),
                data.as_ptr().add(2).read(),
            ]
        }; // beware, unsafe code! ^v^
        Self(a)
    }
    // data drops right where scope ends
    // I've checked with vscode's debug feature ^v^
}

#[allow(unused_must_use)]
fn main() {
    Random::new(1, 2, 3);
    Random::from(['2', '4', '8']);
    Random::from("abc");
    Random::from("abc".to_string());
    Random::from(vec![3, 31, 314]);
    Random::from("many characters edition");
    Random::from(EmailString::new("email@email.email").as_str());

    // idk what to add there :)

    // EmailString::from(EmailString::from("email@a.b") + "lock".to_owned());
    // doesnt deref??? (can't add EmailString(String) to String)

    // Random::from((1, 2, 3));
    // errors because Vec might add conversion from
    // (i32, i32, i32) to Vec<i32> sometime later :(

    let a: Random<i32> = Random::new(1, 2, 3);
    let b: Random<char> = Random::from(['a', 'b', 'c']); // chars check
    let c: Random<u8> = Random::from("#$%"); // u8 check
    for _ in 0..10 {
        eprintln!("a: {:4}, b: {:4}, c: {:4}", a.get(), b.get(), c.get());
    }
}
