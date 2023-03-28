use ambient_api::{
    message::server::{MessageExt, Source, Target},
    prelude::*,
};

#[main]
pub fn main() {
    messages::Local::subscribe(move |source, data| {
        println!("{source:?}: {data:?}");
        if let Source::Local(id) = source {
            messages::Local::new("Hi, back!").send(Target::Local(id));
        }
    });
}