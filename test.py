from tor_cell_codec import encode_channel_cell, decode_channel_cell

def test_encode_decode():
    # Test data
    test_circuit_id = 123
    test_command = 7  # Integer representation of the "Create2" command
    test_body = b"Hello Tor!"

    try:
        # Encode
        encoded_data = encode_channel_cell(test_circuit_id, test_command, list(test_body))
        print(f"Encoded data: {encoded_data}")

        # Decode
        circuit_id, command, body = decode_channel_cell(bytes(encoded_data))
        print(f"Decoded:\n  Circuit ID: {circuit_id}\n  Command: {command}\n  Body: {bytes(body)}")

        # Verify
        assert circuit_id == test_circuit_id, f"Circuit ID mismatch: {circuit_id} != {test_circuit_id}"
        assert command == test_command, f"Command mismatch: {command} != {test_command}"
        assert bytes(body) == test_body, f"Body mismatch: {bytes(body)} != {test_body}"
        print("All tests passed!")
    except Exception as e:
        print(f"Test failed: {e}")

if __name__ == "__main__":
    test_encode_decode()
