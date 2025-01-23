use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use tor_cell::chancell::{
    ChanCell, CircId,
    msg::{AnyChanMsg, Create2, Destroy, DestroyReason},
    codec::ChannelCodec,
};
use bytes::BytesMut;

/// Decodes a Tor channel cell from bytes
#[pyfunction]
fn decode_channel_cell(data: &[u8]) -> PyResult<(u32, u32, Vec<u8>)> {
    let mut codec = ChannelCodec::new(4); // Use link protocol version 4
    let mut buffer = BytesMut::from(data);

    // Decode the cell
    let cell = codec.decode_cell::<AnyChanMsg>(&mut buffer)
        .map_err(|e| PyValueError::new_err(format!("Decode error: {}", e)))?
        .ok_or_else(|| PyValueError::new_err("Incomplete cell data"))?;

    // Extract circuit ID and message type
    let circ_id = CircId::get_or_zero(cell.circid());
    let (command, body) = match cell.into_circid_and_msg().1 {
        AnyChanMsg::Create2(msg) => {
            let handshake_data = msg.body(); // Extract the handshake body
            (
                7, // Integer representation of the "Create2" command
                handshake_data.to_vec(), // Convert &[u8] to Vec<u8>
            )
        },
        AnyChanMsg::Destroy(_) => (14, vec![]), // Integer representation of the "Destroy" command
        _ => (0, vec![]), // 0 for unknown commands
    };

    Ok((circ_id, command, body))
}

/// Encodes a Tor channel cell into bytes
#[pyfunction]
fn encode_channel_cell(circ_id: u32, command: u32, body: Vec<u8>) -> PyResult<Vec<u8>> {
    let mut codec = ChannelCodec::new(4); // Use link protocol version 4
    let mut buffer = BytesMut::new();

    // Create a circuit ID
    let circ_id = CircId::new(circ_id)
        .ok_or_else(|| PyValueError::new_err("Circuit ID must be non-zero"))?;

    // Create the appropriate message based on the command
    let msg = match command {
        7 => AnyChanMsg::Create2(Create2::new(0x01.into(), body)), // "Create2" command
        14 => AnyChanMsg::Destroy(Destroy::new(DestroyReason::NONE)), // "Destroy" command
        _ => return Err(PyValueError::new_err("Unsupported command type")),
    };

    // Wrap the message in a channel cell
    let cell = ChanCell::new(Some(circ_id), msg);

    // Encode the cell
    codec.write_cell(cell, &mut buffer)
        .map_err(|e| PyValueError::new_err(format!("Encode error: {}", e)))?;

    Ok(buffer.to_vec())
}

/// A Python module implemented in Rust
#[pymodule]
fn tor_cell_codec(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(decode_channel_cell, m)?)?;
    m.add_function(wrap_pyfunction!(encode_channel_cell, m)?)?;
    Ok(())
}
