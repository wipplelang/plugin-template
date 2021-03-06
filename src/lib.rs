use wipple::*;
use wipple_plugins::wipple_plugin;

wipple_plugin!(|env: &EnvironmentRef, stack: &Stack| {
    Ok(Value::of(Text {
        text: String::from("Hello, world!"),
        location: None,
    }))
});
