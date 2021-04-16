use wipple::*;
use wipple_plugins::wipple_plugin;

#[wipple_plugin]
pub fn plugin(env: &EnvironmentRef, stack: &Stack) -> Result {
    env.borrow_mut()
        .set_variable("hello", Value::of(Text::new("Hello, world!")));

    Ok(Value::empty())
}
