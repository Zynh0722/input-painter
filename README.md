<div align="center">
  <h1 align="center">input-painter</h1>
  <div display="flex" align="center">
    <img src="https://img.shields.io/crates/l/bat.svg" alt="license">
    <a href="https://crates.io/crates/bat"><img src="https://img.shields.io/crates/v/input-painter.svg?colorB=319e8c" alt="Version info"></a>
  </div>
  <p>a simple way to color the output of other programs</p>
</div>

### Installation

```
cargo install input-painter
```

### Usage

The intended use of this program is for other programs to pipe into them.

```
cat README.md | input-painter rgb 1 152 157
```

You can also select any of `termcolor`'s which are listed by
```
input-painter --help
```

#### TODO:
 - More thorough documentation
 - Support hex values
 - Support changing background as well as style
  - this will cause breaking changes, I will likely move
    everything to options rather than subcommands, as the current
    API is restrictive
