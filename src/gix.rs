// Based on GitoxideLabs/gitoxide/src/gix.rs
// Source: https://github.com/GitoxideLabs/gitoxide/blob/main/src/gix.rs
// Modifications: Minimal - removed feature flags, updated crate name reference

fn main() -> anyhow::Result<()> {
    portable_git::plumbing::main()
}
