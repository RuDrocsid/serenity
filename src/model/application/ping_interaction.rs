use serde::{Deserialize, Serialize};

use crate::model::id::{ApplicationId, InteractionId};

/// A ping interaction, which can only be received through an endpoint url.
///
/// [Discord docs](https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-structure).
#[cfg_attr(feature = "typesize", derive(typesize::derive::TypeSize))]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(any(not(feature = "unstable_exhaustive_types"), doc), non_exhaustive)]
pub struct PingInteraction {
    /// Id of the interaction.
    pub id: InteractionId,
    /// Id of the application this interaction is for.
    pub application_id: ApplicationId,
    /// A continuation token for responding to the interaction.
    pub token: String,
    /// Always `1`.
    pub version: u8,
}
