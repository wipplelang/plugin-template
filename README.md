# Wipple interpreter plugin template

You can use this repository as a template to create your own Wipple interpreter plugins, allowing you to define low-level functionality and access Rust and C code from Wipple.

## Usage

- Create your own repository from this template
- Develop your plugin in `src/lib.rs`
- Run `build.sh` to build a `.wplplugin` file

To use your plugin in a Wipple project, upload the generated `plugin.zip` somewhere and add it as a dependency to your `project.wpl`:

```wipple
dependencies : {
    my-plugin : url "https://example.com/plugin.zip"
}
```

Alternatively, you can use the generated `plugin.wplplugin` directly using the `plugin!` function:

```wipple
plugin! "path/to/plugin.wplplugin"
```

The plugin will be loaded directly into the current scope, so make sure to wrap it in a module if needed:

```wipple
-- In src/lib.rs:
--
-- env.borrow_mut()
--     .set_variable("hello", Value::of(Text::new("Hello, world")));

plugin : {
    plugin! "path/to/plugin.wplplugin"
}

plugin hello -- Hello, world
```
