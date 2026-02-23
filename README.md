# CS2 Demo Parser

A Rust-based tool for parsing Counter-Strike 2 (CS2) demo files and extracting game events, player information, and chat messages.

## Features

- **Parse CS2 demo files** - Read and analyze CS2 replay files (.dem)
- **Extract player deaths** - Get detailed information about kills including:
  - Killer and victim names
  - Weapon used
  - Kill attributes (headshot, noscope, through smoke, etc.)
  - Distance of the kill
- **Extract chat messages** - Filter and display in-game chat messages
- **Filter capabilities** - Filter events by:
  - Player names (killer or victim)
  - Weapon used
  - Kill attributes (headshot, noscope, through smoke, etc.)
- **Programmatic output** - Get structured CSV-like output for further processing

## Installation

### Prerequisites

- Rust (1.70+ recommended)
- Cargo

### Building

```bash
git clone https://github.com/yourusername/csgo-parser.git
cd csgo-parser
cargo build --release
```

## Usage

### Options

| Option              | Short | Long                    | Description                                                                            | Example                   |
|---------------------|-------|-------------------------|----------------------------------------------------------------------------------------|---------------------------|
| Demo file           |       | `file_path`             | Path to the demo file                                                                  | `./game/demos/match1.dem` |
| Find chat           | `-c`  | `--find-chat`           | Extract chat messages from the demo                                                    | `-c`                      |
| Chat filter         | `-f`  | `--filter`              | Filter chat messages containing specific text (by default: "#CS2InterestingReplayTag") | `-f "hello"`              |
| Programmatic output | `-p`  | `--programmatic-output` | Output in CSV format (tick_start,tick_end,player)                                      | `-p`                      |
| Find kills          | `-k`  | `--find-kills`          | Extract kill events from the demo                                                      | `-k`                      |
| Killer name         |       | `--killer-name`         | Filter kills by killer name                                                            | `--killer-name "Player1"` |
| Killed name         |       | `--killed-name`         | Filter kills by victim name                                                            | `--killed-name "Player2"` |
| Weapon              | `-w`  | `--weapon`              | Filter kills by weapon                                                                 | `-w "AK-47"`              |
| Filter attributes   | `-a`  | `--filter-attributes`   | Filter kills by attributes (can be used multiple times)                                | `-a headshot -a noscope`  |

### Kill Attributes

The following kill attributes can be filtered:
- `headshot` - Kill was a headshot
- `noscope` - Kill was a noscope
- `thrusmoke` - Kill was through smoke
- `attackerblind` - Attacker was blinded
- `assistedflash` - Victim was blinded by flash

### Examples

**Extract all chat messages:**
```bash
./target/release/csgo-parser -c -f '' match.dem
```

**Extract all kills:**
```bash
./target/release/csgo-parser -k match.dem
```

**Extract headshot kills by AK-47:**
```bash
./target/release/csgo-parser -k -w "ak47" -a headshot match.dem
```

**Extract kills where Player1 killed Player2:**
```bash
./target/release/csgo-parser -k --killer-name "Player1" --killed-name "Player2" match.dem
```

**Get programmatic output for all kills:**
```bash
./target/release/csgo-parser -k -p match.dem
```

**Extract chat messages containing "hello":**
```bash
./target/release/csgo-parser -c -f "hello" match.dem
```

## Output Formats

### Human-readable format (default)

```
Map: de_dust2
[Round 1, 0:12]: Player1 killed Player2, with ak47, headshot, at a distance of 1234.5 units
[Round 1, 0:15]: Player3: Hello everyone!
```

### Programmatic format (`-p` flag)

```
1234,1300,Player1
1234,1300,Player3
```

This format is useful for importing into spreadsheets or other data processing tools.

## Development

### Running

```bash
cargo run -- <demo_file.dem> [options]
```

### Credits

The project uses the [source2-demo](https://github.com/Rupas1k/source2-demo/) crate which provides the core parsing functionality for Valve's Source 2 engine demo files.
Some code was stolen off of [LaihoE's demo parser](https://github.com/LaihoE/demoparser) to parse Source1 Legacy Game Events.

## License

MIT