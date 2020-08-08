use std::{thread, time};

use mio;
use crate::network::tls_client::TlsClient;

pub fn wait_and_read_branched(tls_client: &mut TlsClient, poll: &mut mio::Poll, 
                              retries: Option<i8>, sleep_time_ms: Option<u64>) -> Result<(), String> {
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
    Err("WAIT_AND_READ_BRANCHED_TIMEDOUT".to_string())
}
