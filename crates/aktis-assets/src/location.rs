use std::fmt;

/// A namespaced Minecraft identifier such as `minecraft:block/stone`.
///
/// A missing namespace defaults to `minecraft`, matching vanilla behaviour.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceLocation {
    namespace: String,
    path: String,
}

/// Error returned when parsing an invalid [`ResourceLocation`].
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("invalid resource location `{input}`: {reason}")]
pub struct ResourceLocationError {
    input: String,
    reason: &'static str,
}

impl ResourceLocation {
    /// Parses `namespace:path` or `path` (namespace defaults to `minecraft`).
    pub fn parse(input: &str) -> Result<Self, ResourceLocationError> {
        let err = |reason| ResourceLocationError {
            input: input.to_owned(),
            reason,
        };
        let (namespace, path) = match input.split_once(':') {
            Some((ns, p)) => (ns, p),
            None => ("minecraft", input),
        };
        if path.is_empty() {
            return Err(err("empty path"));
        }
        let ns_ok = namespace
            .chars()
            .all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '-' | '.'));
        let path_ok = path
            .chars()
            .all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '-' | '.' | '/'));
        if namespace.is_empty() || !ns_ok {
            return Err(err("invalid namespace"));
        }
        if !path_ok {
            return Err(err("invalid path"));
        }
        Ok(Self {
            namespace: namespace.to_owned(),
            path: path.to_owned(),
        })
    }

    /// The namespace, e.g. `minecraft`.
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// The path, e.g. `block/stone`.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Path of this asset inside a jar or resource pack, e.g.
    /// `assets/minecraft/textures/block/stone.png` for kind `textures` and extension `png`.
    pub fn asset_path(&self, kind: &str, extension: &str) -> String {
        format!(
            "assets/{}/{}/{}.{}",
            self.namespace, kind, self.path, extension
        )
    }
}

impl fmt::Display for ResourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_namespace() {
        let loc = ResourceLocation::parse("block/stone").unwrap();
        assert_eq!(loc.namespace(), "minecraft");
        assert_eq!(loc.to_string(), "minecraft:block/stone");
    }

    #[test]
    fn asset_path() {
        let loc = ResourceLocation::parse("minecraft:block/stone").unwrap();
        assert_eq!(
            loc.asset_path("textures", "png"),
            "assets/minecraft/textures/block/stone.png"
        );
    }

    #[test]
    fn rejects_invalid() {
        assert!(ResourceLocation::parse("Minecraft:Stone").is_err());
        assert!(ResourceLocation::parse("minecraft:").is_err());
    }
}
