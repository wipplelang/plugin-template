use wipple::*;
use wipple_plugin_loader::wipple_plugin;

wipple_plugin!(|env: &EnvironmentRef, stack: &Stack| {
    Ok(Value::of(Text {
        text: String::from("Hello, world!"),
        location: None,
    }))
});
