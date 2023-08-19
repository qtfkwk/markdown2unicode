# About

Rust port of [USBashka]'s [markdown2unicode].

[USBashka]: https://github.com/USBashka
[markdown2unicode]: https://github.com/USBashka/markdown2unicode

Converts simple Markdown text to Unicode

* [x] `*Emphasis*` => 𝘌𝘮𝘱𝘩𝘢𝘴𝘪𝘴
* [x] `**Strong**` => 𝐒𝐭𝐫𝐨𝐧𝐠
* [x] `***Emphasis***` => 𝑬𝒎𝒑𝒉𝒂𝒔𝒊𝒔 𝒔𝒕𝒓𝒐𝒏𝒈
* [x] `` `Code span` `` => 𝙲𝚘𝚍𝚎 𝚜𝚙𝚊𝚗

# Usage

```text
$ markdown2unicode -h
Converter from markdown notation to unicode characters

Usage: markdown2unicode [FILES]...

Arguments:
  [FILES]...  

Options:
  -h, --help     Print help
  -V, --version  Print version

```

```text
$ markdown2unicode -V
markdown2unicode 0.1.0

```

# Example

```text
$ cat input
Here is some *emphasis* **bold** ***bold emphasis*** and `mono` text.

$ markdown2unicode input.txt
Here is some 𝘦𝘮𝘱𝘩𝘢𝘴𝘪𝘴 𝐛𝐨𝐥𝐝 𝒃𝒐𝒍𝒅 𝒆𝒎𝒑𝒉𝒂𝒔𝒊𝒔 and 𝚖𝚘𝚗𝚘 text.

```

