use serde::{de, Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Copy, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PetStatus {
    #[default]
    Available,
    Pending,
    Sold,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Pet {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub photo_urls: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub status: PetStatus,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PetRequest {
    pub name: String,
    #[serde(default)]
    pub photo_urls: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub status: PetStatus,
}

#[derive(Debug, Deserialize)]
pub struct StatusQuery {
    pub status: Option<PetStatus>,
}

#[derive(Debug, Deserialize)]
pub struct TagsQuery {
    #[serde(default, deserialize_with = "deserialize_tags")]
    pub tags: Vec<String>,
}

fn deserialize_tags<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct TagsVisitor;

    impl<'de> de::Visitor<'de> for TagsVisitor {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a tag or a sequence of tags")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(vec![value.to_owned()])
        }

        fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let mut tags = Vec::new();
            while let Some(tag) = sequence.next_element()? {
                tags.push(tag);
            }
            Ok(tags)
        }
    }

    deserializer.deserialize_any(TagsVisitor)
}
