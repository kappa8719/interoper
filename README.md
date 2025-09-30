# Interoper

A build-time tool for seamlessly integrating Node.js packages into your Rust projects. Interoper automatically resolves and manages JavaScript dependencies using your preferred package manager during the build process.

## Features

- **Zero Runtime Overhead**: All package resolution happens at build time
- **Package Manager Agnostic**: Supports npm, pnpm, yarn, and bun
- **Simple Configuration**: Configure dependencies with a single TOML file
- **Automatic Detection**: Auto-detects your preferred package manager when not specified

## Quick Start

1. **Add Interoper to your build dependencies**:
   ```toml
   [build-dependencies]
   interoper = "0.1"
   ```

2. **Create an `Interoper.toml` file** in your project root:
   ```toml
   package-manager = "bun"

   [dependencies]
   daisyui = "latest"
   lodash = "^4.17.21"
   ```

3. **Add a build script** (`build.rs`):
   ```rust
   fn main() {
       interoper::build().expect("Failed to build interoper dependencies");
   }
   ```

4. **Build your project** - Interoper will automatically resolve and install the specified Node.js packages.

## Configuration

All configuration is done through the `Interoper.toml` file placed in your project root alongside `Cargo.toml`.

```toml
# Interoper.toml
package-manager = "pnpm"

[dependencies]
# UI Framework
daisyui = "latest"
tailwindcss = "^3.3.0"

# Utilities  
lodash = "^4.17.21"
date-fns = "^2.30.0"

# Development tools
typescript = "^5.1.0"
```

### package-manager

**Optional | Default**: `"auto"`

Specifies which package manager to use. When set to `"auto"`, Interoper will automatically detect and use the best available package manager on your system.

**Supported values**:
- `"auto"` - Automatic detection
- `"npm"`, `"pnpm"`, `"yarn"`, `"bun"` - Specific package managers
- `"/path/to/executable"` - Custom executable path

### dependencies

**Required**

Map of Node.js package dependencies in `name = "version"` format. Supports all standard npm version specifications:

```toml
[dependencies]
# Latest version
react = "latest"

# Specific version
lodash = "4.17.21"

# Semver ranges
typescript = "^5.0.0"
next = "~14.0.0"

# Beta/prerelease
vue = "3.4.0-beta.1"
```

## Complete Example

```toml
# Interoper.toml
package-manager = "pnpm"

[dependencies]
# UI Framework
daisyui = "latest"
tailwindcss = "^3.3.0"

# Utilities
lodash = "^4.17.21"
date-fns = "^2.30.0"

# Development tools
typescript = "^5.1.0"
```

```rust
// build.rs
fn main() {
    // Run interoper to resolve Node.js dependencies
    if let Err(e) = interoper::build() {
        panic!("Interoper build failed: {}", e);
    }
    
    // Continue with other build steps...
    println!("cargo:rerun-if-changed=Interoper.toml");
}
```

## Requirements

- Rust 1.70.0 or later
- At least one supported package manager (npm, pnpm, yarn, or bun) installed on your system
- Node.js (required by most package managers)

## License

This project is licensed under the MIT License.
