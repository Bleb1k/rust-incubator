use clap::Parser;
use std::{borrow::Cow, env, path::PathBuf};

#[derive(Parser)]
struct Args {
    #[arg(long, value_hint = clap::ValueHint::DirPath)]
    conf: Option<String>,
}

struct LazyPath<'a> {
    conf: Cow<'a, PathBuf>,
}

impl<'a> LazyPath<'a> {
    fn detect() -> Self {
        Self {
            conf: if let Some(path) = Args::parse().conf {
                Cow::Owned(path.into())
            } else if let Ok(path) = env::var("APP_CONF") {
                Cow::Owned(path.into())
            } else {
                Cow::Owned("/etc/app/app.conf".into())
            },
        }
    }
}

fn main() {
    let a: LazyPath = LazyPath::detect();
    println!("{}", a.conf.display());
}
