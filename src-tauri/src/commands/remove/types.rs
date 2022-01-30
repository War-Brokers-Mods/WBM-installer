/// must be synced with `src/pages/Remove/types.ts`

#[derive(Clone, Copy)]
pub enum RemoveErr {}

impl serde::Serialize for RemoveErr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(*self as i64)
    }
}
