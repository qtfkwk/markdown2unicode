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
!run:../target/release/markdown2unicode -h
```

```text
$ markdown2unicode -V
!run:../target/release/markdown2unicode -V
```

# Examples

```text
$ markdown2unicode 'Here is some *emphasis*, **strong**, ***strong emphasis***, ~~strike~~, and `code` text.'
!run:../target/release/markdown2unicode 'Here is some *emphasis*, **strong**, ***strong emphasis***, ~~strike~~, and `code` text.' |perl -ne 'print unless /^$/'
```

