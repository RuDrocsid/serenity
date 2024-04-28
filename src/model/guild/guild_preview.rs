use crate::model::guild::Emoji;
use crate::model::id::GuildId;
use crate::model::misc::ImageHash;
use crate::model::sticker::Sticker;

/// Preview [`Guild`] information.
///
/// [Discord docs](https://discord.com/developers/docs/resources/guild#guild-preview-object).
///
/// [`Guild`]: super::Guild
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(any(not(feature = "unstable_exhaustive_types"), doc), non_exhaustive)]
pub struct GuildPreview {
    /// The guild Id.
    pub id: GuildId,
    /// The guild name.
    pub name: String,
    /// The guild icon hash if it has one.
    pub icon: Option<ImageHash>,
    /// The guild splash hash if it has one.
    pub splash: Option<ImageHash>,
    /// The guild discovery splash hash it it has one.
    pub discovery_splash: Option<ImageHash>,
    /// The custom guild emojis.
    pub emojis: Vec<Emoji>,
    /// The guild features. See [`Guild::features`]
    ///
    /// [`Guild::features`]: super::Guild::features
    pub features: Vec<String>,
    /// Approximate number of members in this guild.
    pub approximate_member_count: u64,
    /// Approximate number of online members in this guild.
    pub approximate_presence_count: u64,
    /// The description for the guild, if the guild has the `DISCOVERABLE` feature.
    pub description: Option<String>,
    /// Custom guild stickers.
    pub stickers: Vec<Sticker>,
}
