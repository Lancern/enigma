# Enigma Machine Configuration

The initial configuration of the Enigma machine is given in a JSON formatted
text file. An example of configuration file is given in
[config.example.json](../config.example.json).

## Specification

The JSON text should represent an object prototyped as the following:

```text
{
  "plug_board": PlugBoardSettings,
  "rotators": RotatorGroupSettings,
  "reflector": ReflectorSettings
}
```

### `PlugBoardSettings`

Prototype:

```text
[PermutationSwap]
```

Example:

```JSON
[["a", "b"], ["e", "q"], ["j", "h"]]
```

The rune permutation of the plug board is initialized by applying swap operation
to all character pairs listed in the array.

### `RotatorGroupSettings`

Prototype:

```text
[RotatorSettings, RotatorSettings, RotatorSettings]
```

The 3 rotators within the Enigma machine is configured by the corresponding
entry in the array.

### `ReflectorSettings`

Prototype:

```text
[string, integer]
```

Example:

```JSON
["rcpdnugiozlmhetwsjxykvfqab", 5]
```

The rune permutation of the reflector is initialized as the string element. The
integer element indicates the initial offset of the rotator.

### `PermutationSwap`

Prototype:

```text
[char, char]
```

Example:

```JSON
["a", "b"]
```
