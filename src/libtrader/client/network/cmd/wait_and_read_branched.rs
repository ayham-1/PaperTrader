use std::{thread, time};

use crate::client::network::tls_client::TlsClient;
use crate::common::misc::return_flags::ReturnFlags;
use mio;

/// Waits and issues a branched read when TLS client recieves data to be processed.
///
/// Handles all events on TLS client as branched control. Checks if read data is ready and returns.
///
/// Arguments:
/// tls_client - The TLS Client to use.
/// poll - The mio::Poll to get the Events from.
/// retries - Optional, number of times to retry reading from TLS session before giving up.
/// sleep_time_ms - Optional, time in ms between each try.
///
/// Returns: nothing on success, ReturnFlag on failure containing the reason of failure.
///
/// Example:
/// ```rust
///     /* wait for response */
///     wait_and_read_branched(tls_client, poll, Some(15), Some(500))?;
/// ```
pub fn wait_and_read_branched(
    tls_client: &mut TlsClient,
    poll: &mut mio::Poll,
    retries: Option<i8>,
    sleep_time_ms: Option<u64>,
) -> Result<(), ReturnFlags> {
    tls_client.branch_ctrl = true;

    let mut events = mio::Events::with_capacity(32);
    let mut counter = retries.unwrap_or(10);
    while counter != 0 {
        poll.poll(&mut events, None).unwrap();

        for ev in &events {
            tls_client.ready(&ev);
            tls_client.reregister(poll.registry());

            if tls_client.read_plaintext.len() != 0 {
                tls_client.branch_ctrl = false;
                return Ok(());
            }
        }

        thread::sleep(time::Duration::from_millis(sleep_time_ms.unwrap_or(50)));
        counter -= 1;
    }
    Err(ReturnFlags::ClientWaitAndReadBranched)
}
