# Wipple interpreter plugin template

You can use this repository as a template to create your own Wipple interpreter plugins, allowing you to define low-level functionality and access Rust and C code from Wipple.

## Usage

- Create your own repository from this template
- Build your plugin in `src/lib.rs`
- Run `build.sh` to build a `<platform>.wplplugin` file
- Upload the plugin file somewhere accessible

To use your plugin in a Wipple project, add it to your `project.wpl`:

```wipple
dependencies : {
    -- Wipple will append the `.wplplugin` file corresponding to the current
    -- platform to this URL. For example, on macOS the URL will become
    -- "https://mysite.com/my-plugin/x86_64-apple-darwin.wplplugin"
    my-plugin : plugin "https://mysite.com/my-plugin"
}
```
