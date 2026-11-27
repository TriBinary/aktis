use std::path::PathBuf;

/// One place assets can come from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssetSource {
    /// The user's installed Minecraft client jar (vanilla assets). Never redistributed.
    ClientJar(PathBuf),
    /// A resource pack, either a `.zip` file or an unpacked directory.
    ResourcePack(PathBuf),
}

/// An ordered stack of asset sources.
///
/// Lookups go from the top (highest priority, last pushed) down to the client jar,
/// exactly like Minecraft's resource pack screen.
#[derive(Debug, Clone, Default)]
pub struct AssetStack {
    sources: Vec<AssetSource>,
}

impl AssetStack {
    /// Creates an empty stack.
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes a source on top of the stack (it wins over everything below it).
    pub fn push(&mut self, source: AssetSource) {
        self.sources.push(source);
    }

    /// Sources from highest to lowest priority.
    pub fn by_priority(&self) -> impl Iterator<Item = &AssetSource> {
        self.sources.iter().rev()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_of_stack_wins() {
        let mut stack = AssetStack::new();
        stack.push(AssetSource::ClientJar("1.21.jar".into()));
        stack.push(AssetSource::ResourcePack("pbr.zip".into()));
        let first = stack.by_priority().next().unwrap();
        assert_eq!(first, &AssetSource::ResourcePack("pbr.zip".into()));
    }
}
