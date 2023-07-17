Generic Robot Framework CLI
===

This tool is made to work with the Generic Robot Framework library

Folder architecture :

```yaml
project_workspace: # GRF package typed "Workspace"
  src:
    launch:
      - my_launch_file.rs
    packages:
      example_adapter:   # GRF package typed "Adapter"
        ...
      example_resource:  # GRF package typed "Resource"
        ...
      example_package:   # GRF package typed "Module"
        src:
          msg:           # Folder containing messages structs
            - example_message.rs
          bin:           # Folder containing nodes scripts
            - example_node.rs
        - Cargo.toml
        - Cargo.lock
  - Cargo.toml
  - Cargo.lock
```