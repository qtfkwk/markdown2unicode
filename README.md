# About

Rust port of [USBashka]'s [markdown2unicode].

[USBashka]: https://github.com/USBashka
[markdown2unicode]: https://github.com/USBashka/markdown2unicode

Converts simple Markdown text to Unicode

* [x] `*Emphasis*` => 𝘌𝘮𝘱𝘩𝘢𝘴𝘪𝘴
* [x] `**Strong**` => 𝐒𝐭𝐫𝐨𝐧𝐠
* [x] `***Emphasis***` => 𝑬𝒎𝒑𝒉𝒂𝒔𝒊𝒔 𝒔𝒕𝒓𝒐𝒏𝒈
* [x] `` `Code span` `` => 𝙲𝚘𝚍𝚎 𝚜𝚙𝚊𝚗

**NOTE: This crate is superceded by [`unidown`].
Please use it instead of this one.**

[`unidown`]: https://crates.io/crates/unidown

# Usage

```text
$ markdown2unicode -h
Converter from markdown notation to unicode characters

Usage: markdown2unicode [OPTIONS] [STRING]...

Arguments:
  [STRING]...  Markdown string(s)

Options:
  -i <PATH>      Input file(s)
  -h, --help     Print help
  -V, --version  Print version
```

```text
$ markdown2unicode -V
markdown2unicode 0.2.0
```

# Examples

```text
$ markdown2unicode 'Here is some *emphasis*, **strong**, ***strong emphasis***, ~~strike~~, and `code` text.'
Here is some 𝘦𝘮𝘱𝘩𝘢𝘴𝘪𝘴, 𝐬𝐭𝐫𝐨𝐧𝐠, 𝒔𝒕𝒓𝒐𝒏𝒈 𝒆𝒎𝒑𝒉𝒂𝒔𝒊𝒔, ~~strike~~, and 𝚌𝚘𝚍𝚎 text.
```

