# taskstats-rs

This crate provides a high-level encapsulation of Linux's
[per-task statistics interface](https://www.kernel.org/doc/html/latest/accounting/taskstats.html).

Currently only limited information is exposed in `Taskstats` struct,
but new field can be easily added for future version.
If there are any other fields in `taskstats` struct useful to you,
feel free to submit PR to add them.

## Examples

Query aggregated taskstats of a task:
```rust
let mut conn = TaskstatsConnection::new()?;
let stats = conn.get_pid_stats(1)?;
// ...
```

Listen to taskstats of exited tasks:
```rust
let conn = TaskstatsConnection::new()?;
let cpu_mask = CString::new("0-7\0".to_owned()).unwrap().into_boxed_c_str();
let mut listener = TaskstatsListener::register(conn, cpu_mask)?;
loop {
    let (pid, stats) = listener.get_next()?;
    // ...
}
```

## Development

`raw.rs` files in `src`, `src/netlink`, and `src/genetlink` are generated from corresponding headers,
then manually adjusted to fit the needs.

## Note

This crate only exposes stats for pid / task, but not tgid / process,
because the kernel doesn't really pass much useful information for tgid.
It is recommended that you rely on pid stats and aggregate for process manually.
