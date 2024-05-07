# Stated Scope Guard

Scope guard is a practical usage of RAII (Resource Acquisition Is Initialization) to avoid resource leak, and stated scope guard is a more flexible RAII pattern for stated resouce management.

To use this crate, just add the following content in `Cargo.toml`:

```toml
[dependencies]
stated-scope-guard = "0.1"
```

## Background

If you are familiar with RAII, feel free to skip this section.

For programming languages supporting contructor and destructor (for Rust, is `new()` and `drop()`), resource can be managed inside the ctor and dtor to avoid resource leak. For example, in POSIX environment, files can be opened by libc's `open` function, and shall be closed by libc's `close` function. If an opened is never closed, it may cause resource leak for a long-running application. To resolve this problem, we can encapsulate the file descriptor inside a Rust struct `File`. To create a `File` instance, we shall call its `new` method, in which the libc's `open` function is called, and the file descriptor is stored in the `File` structure. When the `File` instance leaves its scope, its `drop` method will be automatically called, inside which, the libc's `close` function is called with the store file descriptor. This pattern is called RAII, or Scope guard.

The best thing of scope guard is to make compiler to take the responsibility to make sure resource is managed properly. Resource management may drive developers crazy in old days in C, for example:

```c
void every_fault_shall_be_handled(char *path) {
    int fd = open(path, O_RDONLY);
    if (fd == OPEN_FAILED) { return; }

    if (file_op_may_fail(fd) == FAILED) { close(fd); return; }

    int sock = socket(/* ... */);
    if (sock == OPEN_FAILED) { close(fd); return; }

    if (send_file_to_sock(fd, sock) == FAILED) {
        close(fd);
        close(sock);
        return;
    }

    close(sock);
    close(fd);
}
```

When handling errors, the resource must be dealt carefully before each `return`, and in large projects such as Linux kernel, they tend to use `goto err` for resource cleanning.

With RAII in C++/Rust/Python/Java/..., each `return` means a scope ending, which will automatically call the destructor of remained instances, and the resource is just released with nothing to do for developers, great!

## Stated resource management

Things become more complicated when we need stated resource management. Let's consider the following situation: the resources shall be reverted if any step failed, while if the whole steps succeed, the resources shall be preserved (not reverted) even after the program exits. For example:

```rust, ignore
fn setup() -> anyhow::Result<()> {
    let log_dir = LogDir::create()?;
    let user_account = UserAccount::create().inspect_err(|_| {
        delete_log_dir(log_dir);
    })?;
    let network = UserNetwork::create().inspect_err(|_|) {
        delete_user_account(user_account);
        delete_log_dir(log_dir);
    }?;
    Ok(())
}
```

In this situation, the traditional scope guard cannot work as expected, since resources like `logdir` will always be deleted **except** all things go right. This problem can be further expanded as stated resource management, i.e., the resource is managed differently according to the state. For `logdir`, the state is either `AllThingsGoRight` or `SomethingWrong`. If `SomethingWrong`, we shall delete the `logdir`, and when `AllThingsGoRight`, we don't need to delete the `logdir`. Now, what can we do if there are many states and many resources to deal with?

## Usage

To solve the stated resource management, we can use `state-scope-guard` crate like this:

```rust
use stated_scope_guard::ScopeGuard;

struct Resource;
impl Resource { fn new() -> Self { Self }}
fn do_something() {}

enum State {
    State1,
    State2,
    // ...
}

fn setup() {
    let mut resource_guard = ScopeGuard::new(Resource::new(), State::State1, |res, state| {
        match state {
            State::State1 => { /* do something with res */ },
            State::State2 => { /* do something else with res */ },
            // ...
        }
    });
    do_something();
    resource_guard.set_state(State::State2);
    // ...
}
```

The third argument passed to `ScopeGuard` is a callback which will be called when the scope guard is dropped. It takes current resource and state as parameter, and is expected to deal with the resource according to the state. When the state needs to be changed, we can use `set_state` to do so.

## Dismissble scope guard

For a more common and simple situation, where there are only two states, and the default state action is to revert, the other is do nothing, which is just the case for `logdir` mentioned above, we provide `DismissbleScopeGuard`, which we can use it as:


```rust, ignore
let mut log_dir_guard = new_dismissable(|| { delete_log_dir(); });
do_something()?;
log_dir_guard.dismiss();
Ok(())
```

After calling `dismiss`, the callback will never be executed.
