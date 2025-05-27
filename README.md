# bin_eye_bt_receiver

A Quick and Dirty receiver app for [Binary Eye][binary_eye]'s Forward scan via Bluetooth functionality Written in Rust.

Or more simply, it lets you use your phone like you would use a dedicated barcode scanner, via bluetooth.

## Requirements

You will need to install:
```bash
sudo apt install libdbus-1-dev pkg-config libxdo-dev
```

Compatibility on Linux with regards to keyboard emulation is as follows:

| Desktop Type | Supported  | Requirements                                             |
|--------------|------------|----------------------------------------------------------|
| Xorg/X11     | Yes        | `xdotool`                                                |
| Wayland      | Yes\*      | compositor supporting the [virtual_keyboard_v1] protocol |

\*as of 2025-05-25, neither KDE (KWin 6.3) nor GNOME (Mutter 48.0) support the aforementioned Wayland protocol.

## Usage

```
Usage: bin_eye_bt_receiver [OPTIONS]
Options:
  -u, --uuid <UUID>  UUID of scaner [default: 8a8478c9-2ca8-404b-a0de-101f34ab71ae]
  -k, --keyboard     Enable keyboard output
  -h, --help         Print help
  -V, --version      Print version
```

### Allowed Characters Argument
#### Simple explaination
The `--allowed-characters` argument allows you to filter out characters from the scanned barcodes (for example to prevent them from inputting control characters like ESC).

The syntax is fairly simple, it contains ordered, comma-separated (`,`) rules, which consist of:
- Either the `+` or `-` sign, which mean "allow" or "remove" respectively.
- A character class or range, which is one of:
  - one of the character classes from the `char-class` definition in the technical explanation section below, like `all`, `unicode` or `numeric`
  - an ASCII character specified as `x20`, where `20` is the hexadecimal character code, or an inclusive range, for example `x30..39` would be the same as the `numeric` class and include every digit `0123456789`
  - a Unicode character, specified as `u:1F98A:`, where `1F98A` is a Unicode code point, a 4-6 digits long hexadecimal number, this one would match "🦊", or a range specified as `u:0030:..:000039:`

As an example, the default is `--allowed-characters=+all,-control,+IS,+x09`, which allows all characters except ASCII control characters, and also allows ASCII Information Separators and the TAB character.

#### Technical explaination
[ABNF] syntax for the `--allowed-characters` argument value:
```
allowed-chars  =  rule *( "," rule )
rule           =  ("+" / "-") (char-class / ascii-range / unicode-range)

char-class     =  "all"          ; all ASCII and Unicode characters
char-class     =/ "ascii"        ; from U+0000 to U+007F
char-class     =/ "unicode"      ; from U+0080 to U+10FFFF
char-class     =/ "control"      ; %x00-1F and %x7F

char-class     =/ "CC"           ; ASCII Communication Control: %x01-06 and %x10 and %x15-17
char-class     =/ "FE"           ; ASCII Format Effector: %x08-0D
char-class     =/ "IS"           ; ASCII Information Separator: %x1C-1F

char-class     =/ "whitespace"   ; %x09-0A and %x0C-0D and %x20 (TAB, LF, FF, CR, SP)
char-class     =/ "newline"      ; %x0A and %x0D (LF and CR)

char-class     =/ "graphic"      ; %x20-1E (not "control")
char-class     =/ "punctuation"  ; %x20-2F and %x3A-40 and %x5B-60 and %7B-7E
char-class     =/ "alnum"        ; "alpha" and "numeric"
char-class     =/ "numeric"      ; %x30-39 (0-9)
char-class     =/ "alpha"        ; "upper" and "lower"
char-class     =/ "upper"        ; %x41-5A (A-Z)
char-class     =/ "lower"        ; %x61-7A (a-z)

ascii-range    =  "x" 2*HEXDIG [ ".." 2*HEXDIG ]  ; ranges are right-inclusive
unicode-range  =  "u" uchar [ ".." uchar ]        ; unicode range can also represent ASCII characters
uchar          =  ":" 4*6HEXDIG ":"
```

#### Character codes reference
ASCII character codes based on [RFC20]:

| Hex | Char  | Hex | Char  | Hex | Char  | Hex | Char  |
|-----|-------|-----|-------|-----|-------|-----|-------|
| 00  | NUL   | 20  | Space | 41  | A     | 61  | a     |
| 01  | SOH   | 21  | !     | 42  | B     | 62  | b     |
| 02  | STX   | 22  | "     | 43  | C     | 63  | c     |
| 03  | ETX   | 24  | $     | 44  | D     | 64  | d     |
| 04  | EOT   | 25  | %     | 45  | E     | 65  | e     |
| 05  | ENQ   | 26  | &     | 46  | F     | 66  | f     |
| 06  | ACK   | 27  | '     | 47  | G     | 67  | g     |
| 07  | BEL   | 28  | (     | 48  | H     | 68  | h     |
| 08  | BS    | 29  | )     | 49  | I     | 69  | i     |
| 09  | TAB   | 2a  | *     | 4a  | J     | 6a  | j     |
| 0a  | LF    | 2b  | +     | 4b  | K     | 6b  | k     |
| 0b  | VT    | 2c  | ,     | 4c  | L     | 6c  | l     |
| 0c  | FF    | 2d  | -     | 4d  | M     | 6d  | m     |
| 0d  | CR    | 2e  | .     | 4e  | N     | 6e  | n     |
| 0e  | SO    | 2f  | /     | 4f  | O     | 6f  | o     |
| 0f  | SI    | 30  | 0     | 50  | P     | 70  | p     |
| 10  | DLE   | 31  | 1     | 51  | Q     | 71  | q     |
| 11  | DC1   | 32  | 2     | 52  | R     | 72  | r     |
| 12  | DC2   | 33  | 3     | 53  | S     | 73  | s     |
| 13  | DC3   | 34  | 4     | 54  | T     | 74  | t     |
| 14  | DC4   | 35  | 5     | 55  | U     | 75  | u     |
| 15  | NAK   | 36  | 6     | 56  | V     | 76  | v     |
| 16  | SYN   | 37  | 7     | 57  | W     | 77  | w     |
| 17  | ETB   | 38  | 8     | 58  | X     | 78  | x     |
| 18  | CAN   | 39  | 9     | 59  | Y     | 79  | y     |
| 19  | EM    | 3a  | :     | 5a  | Z     | 7a  | z     |
| 1a  | SUB   | 3b  | ;     | 5b  | \[    | 7b  | {     |
| 1b  | ESC   | 3c  | <     | 5c  | \\    | 7c  | \|    |
| 1c  | FS    | 3d  | =     | 5d  | \]    | 7d  | }     |
| 1d  | GS    | 3e  | >     | 5e  | ^     | 7e  | ~     |
| 1e  | RS    | 3f  | ?     | 5f  | _     | 7f  | DEL   |
| 1f  | US    | 40  | @     | 60  | `     |     |       |


[RFC20]: https://www.rfc-editor.org/rfc/rfc20
[ABNF]: https://www.rfc-editor.org/rfc/rfc5234
[binary_eye]: https://github.com/markusfisch/BinaryEye
[virtual_keyboard_v1]: https://wayland.app/protocols/virtual-keyboard-unstable-v1#compositor-support
