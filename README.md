# GS - General Store
General Store is a Salsify like object server written in Rust using Redis. Featuring a management interface to manage files and directories, access control, custom metadata, tagging, and search for your personal files managed by the server. Many applications can benefit from General Store as a standalone app or a combined service to serve static files for your web application or other services.

This service is written to serve custom themes from the themes directory. A default theme already exists if you choose not to provide your own.

## Theming
General Store is a decoupled application that serves static front-ends that use the core service as a backend. You can create your own theme to be served by the server or use the default theme bundled with the project. 

## Building and Running Instructions
Use Cargo for orchestration of the application

To build an executible from cli
`cargo build`

To run the application from cli
`cargo run`

TODO: clean up this section add default theme building instructions.
