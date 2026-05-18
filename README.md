# yei component registry

This is the official component registry for the **yei** Yew component library.

Each component is a self-contained `.rs` file that can be copied directly into a Yew project.
Dependencies required by each component are declared in `registry.json`.

## Contributing

Community contributions are welcome. To add a new component, open a pull request that includes:

- A self-contained `<component-name>.rs` file in the registry root
- An entry in `registry.json` with the component name, description, files list, and any Cargo dependencies

Please ensure submitted components compile against Yew 0.21 and follow the existing code style.
