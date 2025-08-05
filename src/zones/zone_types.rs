use crate::ZoneType;
use serde_json::Value;

impl ZoneType {
    /// Creates a new `ZoneType` instance.
    ///
    /// # Arguments
    ///
    /// * `description` - A description of the zone type.
    /// * `id` - The unique identifier for the zone type.
    /// * `name` - The name of the zone type.
    /// * `prices` - Optional prices associated with the zone type.
    ///
    /// # Returns
    ///
    /// A new `ZoneType` instance.
    pub fn new(description: String, id: String, name: String, prices: Option<Value>) -> Self {
        ZoneType {
            description,
            id,
            name,
            prices,
        }
    }
}
