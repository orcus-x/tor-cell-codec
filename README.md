# Tor Cell Codec

Python bindings for encoding and decoding Tor channel cells, implemented in Rust using the `tor-cell` crate. The library provides two functions:

- `decode_channel_cell(data: bytes) -> (circ_id: int, command: int, body: bytes)`
- `encode_channel_cell(circ_id: int, command: int, body: bytes) -> bytes`

## Installation

1. Install Rust and Python (version >= 3.7).
2. Install `maturin`:
   ```bash
   pip install maturin
   ```
3. Build and install the Python module:
   ```bash
   maturin develop
   ```

## Usage

### Example

```python
from tor_cell_codec import encode_channel_cell, decode_channel_cell

# Encode a cell
encoded_data = encode_channel_cell(123, 7, b"Hello Tor!")  # 7 represents the "Create2" command
print(f"Encoded: {encoded_data}")

# Decode a cell
circ_id, command, body = decode_channel_cell(encoded_data)
print(f"Decoded: Circuit ID={circ_id}, Command={command}, Body={body}")
```

## Development

Run tests using the provided `test.py` script:
```bash
python test.py
```

## Command Mapping

- `7`: "Create2"
- `14`: "Destroy"
- `0`: Unknown command
