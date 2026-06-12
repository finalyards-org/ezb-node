# Details (for AI; coding style)

## Use `Self` within method implementations

```
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyInUse => {
                write!(f, "Zigbee node is already in use")
            }
            Self::InitializationFailed(e) => {
                write!(f, "Zigbee node initialization failed: {e}")
            }
            Self::SpawnFailed(e) => {
                write!(f, "Failed to spawn Zigbee task: {e}")
            }
        }
    }
}
```

Note how instead of `Error::AlreadyInUse`, we're using `Self`. Please prefer this style: avoid repeating the name of a struct when `Self` is an alias for it.

- easier to read
