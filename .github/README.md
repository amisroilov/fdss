# FDSS
![License](https://img.shields.io/github/license/amisroilov/fdss)
![Rust](https://img.shields.io/badge/language-rust-orange)
![GitHub stars](https://img.shields.io/github/stars/amisroilov/fdss)
![GitHub issues](https://img.shields.io/github/issues/amisroilov/fdss)

**Fast Dynamic Site Server** is a tool I made because I was annoyed by slow site hosting - I just wanted a quick way to test my site.

Good for testing and quick demos.

--- 

## Contents

- [Quick Start](#quick-start)
- [Installation](#installation)
- [Using FDSS](#using-fdss)
- [Contributing](#how-to-contribute)
- [License](#license)

## Quick Start

```bash
git clone https://github.com/amisroilov/fdsh
cd fdsh
cargo install --path .
fdsh
```
Then open:

`http://localhost:8080`

## Installation

**Prerequisites**: 
  - Since FDSS is written in Rust, you will *need* Rust and Cargo. They can be installed from https://rust-lang.org/learn/get-started.
  - Git. You can install Git from https://git-scm.com.

### Building from source :
> [!NOTE]
> Currently, there is no other way to install FDSH than building from source.
> We are working on a binary release, keep your eyes open!

  Open the terminal (after making sure you have the prerequisites installed) and run this command :
  ```bash
  git clone https://github.com/amisroilov/fdss
  cd fdss
  cargo install --path .
  ```
  See that `--path` flag? This installs the executable to Cargo's binary directory (usually ~/.cargo/bin), which is typically already included in your PATH. This lets you run `fdss` from anywhere,
  and BAM! You're done installing.  The next step...


## Using FDSS

Navigate to the directory containing your site (e.g. `cd C:\Users\Me\MyCoolProject`), then run `fdss`. 
FDSS will serve the files in that directory at the given port (defaults to 8080).
You can also run `fdsh ./mydirectory` to serve the given directory.

**Flags**:
- `--port <number>` : Changes the port number. For example, if you did `fdsh --port 3000`, you could type `http://localhost:3000` and you'd see your site.

## Contributors

**Thank you to our contributors!**

<a href="https://github.com/amisroilov/fdss/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=amisroilov/fdss" width="5%"/>
</a>

## How to contribute

> [!IMPORTANT]
> Before you participate in contributing, please read our [code of conduct.](CODE_OF_CONDUCT.md)


> [!TIP]
> I'm pretty far from being an expert in Rust, or programming in general. There's probably tons of ways you can contribute, including fixing bugs in the code, fixing typos, etc.

See [CONTRIBUTING.md](CONTRIBUTING.md) for more details.



## License
FDSH is licensed under the [MIT License.](LICENSE)

