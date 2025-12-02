# Tracker

A platform-independent desktop application built with Tauri, React, and TypeScript.

## Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://www.rust-lang.org/tools/install)
- Platform-specific dependencies:
  - **macOS**: Xcode Command Line Tools
  - **Linux**: WebKit2GTK, build-essential, and other dependencies
  - **Windows**: Visual Studio Build Tools with C++ workload

## Development

Install dependencies:
```bash
npm install
```

Run the development server:
```bash
npm run tauri:dev
```

## Building

Create a production build:
```bash
npm run tauri:build
```

The compiled application will be in `src-tauri/target/release/bundle/`.

## Project Structure

- `src/` - React frontend (TypeScript)
- `src-tauri/` - Rust backend (Tauri)
- `src-tauri/src/` - Rust source code
- `dist/` - Built frontend assets
- `src-tauri/target/` - Rust build artifacts

## Learn More

- [Tauri Documentation](https://tauri.app/)
- [React Documentation](https://react.dev/)
- [TypeScript Documentation](https://www.typescriptlang.org/)
