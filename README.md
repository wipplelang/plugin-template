# Wipple interpreter plugin template

You can use this repository as a template to create your own Wipple interpreter plugins, allowing you to define low-level functionality and access Rust and C code from Wipple.

## Usage

- Create your own repository from this template
- Build your plugin in `src/lib.rs`
- Run `build.sh` to build a `.wplplugin` file
- Upload the plugin file somewhere accessible

To use your plugin in a Wipple project, add it to your `project.wpl`:

```wipple
dependencies : {
    my-plugin : plugin (url (format "https://example.com/my-plugin/_.wplplugin" host))
}
```
