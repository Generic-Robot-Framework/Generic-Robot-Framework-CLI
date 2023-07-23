Generic Robot Framework CLI
===

This tool is made to work with the [Generic Robot Framework library](https://crates.io/crates/generic_robot_framework).

## Commands

**Build**

Builds the workspace

```shell
grf build
```

Options:

- `--path <PATH>`  Optional, serve a workspace from outside

---

**Serve**

Start the server

```shell
grf serve
```

Arguments:

- `-p, --port <PORT>` Optional, serve with a specific port
- `--path <PATH>` Optional, serve a workspace from outside

---

**Topic pub**

Topic subscription command

```shell
grf topic sub <topic> [message]
```

Arguments:
- `<topic>` Name of the topic to pub to
- `[message]` Message to send

---

**Topic sub**

Topic publication command

```shell
grf topic sub <topic> [message_type]
```

Arguments:
- `<topic>` Name of the topic to sub to
- `[message_type]` Topic message type

---

**Topic list**

Topic list command

```shell
grf topic list
```

---

**Message get**

Get message type for the given topic

```bash
grf msg get <topic>
```

Arguments:

- `<topic>` Name of the topic to retrieve message type

---

**Message show**

Show default data for the given message type

```shell
grf msg show <message_type>
```

Arguments:

- `<message_type>` Name of the message type to show default data
 
---

**Message list**

List registered messages

```shell
grf msg list
```

---

**Completions**

Creates the completion files to source in order to use topics and default messages.

```shell
grf completions
```

Arguments:

- `-n, --no-sourcing` Avoid sourcing the file after it's generated

---

**Help**

Print this message or the help of the given subcommand(s).

```shell
grf help
```

## Workspace architecture:

```yaml
project_workspace: # GRF package typed "Workspace"
  src:
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