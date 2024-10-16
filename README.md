# Voltage Monitor for Raspberry Pi (Rust)

This project monitors the voltage of a Raspberry Pi and displays the detected voltage alongside the recommended voltage in the console. It continuously updates the console with the current voltage and uses ASCII formatting to show a table with colored output:

- **Green**: Voltage is equal to the recommended 5.0V.
- **Red**: Voltage is below the recommended 5.0V.
- **Yellow**: Voltage is above the recommended 5.0V.

## Features

- Constant monitoring of the voltage using `vcgencmd`.
- ASCII table output with real-time voltage information.
- Colored display to indicate voltage status:
  - **Green**: Normal (5.0V).
  - **Red**: Under voltage.
  - **Yellow**: Over voltage.
- Auto-refreshes every 2 seconds.

## Requirements

- Raspberry Pi (tested on Raspberry Pi 3 and 4)
- Rust (installed on the Raspberry Pi)
- `vcgencmd` utility (comes pre-installed with Raspberry Pi OS)
- `colored` crate for terminal color support

### Crate dependencies

In the `Cargo.toml`, you need to add the following dependency:

```toml
[dependencies]
col1ored = "2.0"
```

## Installation

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   ```

2. **Navigate to the project directory**:
   ```bash
   cd voltage-monitor
   ```

3. **Build the project**:
   ```bash
   cargo build --release
   ```

4. **Run the program**:
   ```bash
   cargo run --release
   ```

## How it works

The program uses the Raspberry Pi's `vcgencmd measure_volts` command to read the voltage of the core and parses the result. This value is then displayed in an ASCII-formatted table alongside the recommended 5.0V.

Every 2 seconds, the table refreshes, showing the updated voltage, with colors indicating the status of the voltage:
- **Green**: Voltage is normal (5.0V).
- **Red**: Voltage is lower than 5.0V.
- **Yellow**: Voltage is higher than 5.0V.

## Example output

```
+-----------------------------------------+
|         Voltage Monitor                 |
+--------------------+--------------------+
| Detected Voltage    | Recommended (5.0V)|
+--------------------+--------------------+
|       5.01 V       |       5.00 V       |
+--------------------+--------------------+
```

## License
This project is open-source and available under the [MIT License](LICENSE).
