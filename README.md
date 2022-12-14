# utfdump(1)
Display information about UTF-8 characters read from stdin.

## Examples
```
$ printf '¿Cómo estás?' | utfdump
┌───┬────────┬───────────┬────────────────────────┬──────────┬───────────┐
│   │ Code   │ UTF-8     │ Name                   │ Category │ Combining │
├───┼────────┼───────────┼────────────────────────┼──────────┼───────────┤
│ ¿ │ U+00bf │ 0xc2 0xbf │ INVERTED QUESTION MARK │ Po       │ 0         │
├───┼────────┼───────────┼────────────────────────┼──────────┼───────────┤
│ C │ U+0043 │ 0x43      │ LATIN CAPITAL LETTER C │ Lu       │ 0         │
├───┼────────┼───────────┼────────────────────────┼──────────┼───────────┤
│ o │ U+006f │ 0x6f      │ LATIN SMALL LETTER O   │ Ll       │ 0         │
├───┼────────┼───────────┼────────────────────────┼──────────┼───────────┤
│ ◌́ │ U+0301 │ 0xcc 0x81 │ COMBINING ACUTE ACCENT │ Mn       │ 230       │
├───┼────────┼───────────┼────────────────────────┼──────────┼───────────┤
│ m │ U+006d │ 0x6d      │ LATIN SMALL LETTER M   │ Ll       │ 0         │
├───┼────────┼───────────┼────────────────────────┼──────────┼───────────┤
│ o │ U+006f │ 0x6f      │ LATIN SMALL LETTER O   │ Ll       │ 0         │
├───┼────────┼───────────┼────────────────────────┼──────────┼───────────┤
│   │ U+0020 │ 0x20      │ SPACE                  │ Zs       │ 0         │
├───┼────────┼───────────┼────────────────────────┼──────────┼───────────┤
│ e │ U+0065 │ 0x65      │ LATIN SMALL LETTER E   │ Ll       │ 0         │
├───┼────────┼───────────┼────────────────────────┼──────────┼───────────┤
│ s │ U+0073 │ 0x73      │ LATIN SMALL LETTER S   │ Ll       │ 0         │
├───┼────────┼───────────┼────────────────────────┼──────────┼───────────┤
│ t │ U+0074 │ 0x74      │ LATIN SMALL LETTER T   │ Ll       │ 0         │
├───┼────────┼───────────┼────────────────────────┼──────────┼───────────┤
│ a │ U+0061 │ 0x61      │ LATIN SMALL LETTER A   │ Ll       │ 0         │
├───┼────────┼───────────┼────────────────────────┼──────────┼───────────┤
│ ◌́ │ U+0301 │ 0xcc 0x81 │ COMBINING ACUTE ACCENT │ Mn       │ 230       │
├───┼────────┼───────────┼────────────────────────┼──────────┼───────────┤
│ s │ U+0073 │ 0x73      │ LATIN SMALL LETTER S   │ Ll       │ 0         │
├───┼────────┼───────────┼────────────────────────┼──────────┼───────────┤
│ ? │ U+003f │ 0x3f      │ QUESTION MARK          │ Po       │ 0         │
└───┴────────┴───────────┴────────────────────────┴──────────┴───────────┘
```

## Usage
`utfdump` receives its input string from stdin and writes its outputs to stdout. The input string is assumed to be UTF-8 encoded.

Arguments:

| Short | Long                    | Effect                                                                             |
|-------|-------------------------|------------------------------------------------------------------------------------|
| `-f`  | `--full-category-names` | Display category names in plain English, rather than using their abbreviated names |

## Download
Pre-built binaries are available in [the GitHub releases](https://github.com/pantonshire/utfdump/releases/latest).
