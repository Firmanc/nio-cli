# Design Spec: `nio kill` Command

A command to identify and kill processes by port or PID with interactive confirmation.

## 1. User Interface

- **Command:** `nio kill <identifiers>...`
- **Arguments:** One or more numeric identifiers (ports or PIDs).
- **Interactive Flow:**
    1. User runs `nio kill 8000 1234`.
    2. CLI searches for processes listening on port 8000.
    3. CLI searches for process with PID 1234.
    4. CLI displays a table of matches:
       ```
       Found 2 processes:
       PID    NAME         TYPE    IDENTIFIER
       8540   node         Port    8000
       1234   python3      PID     1234
       
       Kill these processes? [y/N]
       ```
    5. If 'y', kills the processes.

## 2. Logic & Discovery

### Port Discovery
- For each identifier, check if any process is listening on that port.
- On macOS (Darwin), this can be done via `lsof -i :<port> -n -P`.
- On Linux, this can be done via `/proc/net/tcp` or `ss`.
- Cross-platform alternative: Use the `sysinfo` crate to list processes and potentially a helper for port mapping if `sysinfo` doesn't provide it directly.

### PID Discovery
- If an identifier didn't match a port, check if it matches an existing PID.
- Verify the process exists.

### Termination
- Use `sysinfo` to send termination signals.
- Attempt a graceful termination (SIGTERM) first, or just a direct kill (SIGKILL) depending on user preference (default to SIGKILL for a "kill" command is common, but SIGTERM is safer). Let's stick to a standard kill.

## 3. Architecture

- New file `src/kill.rs` to house the logic.
- Update `src/cli.rs` to include the `Kill` subcommand.
- Update `src/main.rs` to route to `kill::handle_command`.

## 4. Dependencies

- `sysinfo`: For process management and killing.
- `dialoguer`: For the confirmation prompt (already in project).
- `tabwriter` or similar (optional): For formatted table output, or just manual padding.

## 5. Error Handling

- Invalid (non-numeric) identifiers should be reported as errors.
- If an identifier matches nothing, inform the user but continue with others.
- Permission errors when killing should be reported gracefully.

## 6. Testing Strategy

- Unit tests for the identifier parsing logic.
- Integration tests (mocked or using dummy processes) to verify discovery and the confirmation flow.
