use super::Decode;
use crate::io::Buf;
use std::io;

#[derive(Debug)]
pub struct CommandComplete {
    affected_rows: u64,
}

impl CommandComplete {
    #[inline]
    pub fn affected_rows(&self) -> u64 {
        self.affected_rows
    }
}

impl Decode for CommandComplete {
    fn decode(mut buf: &[u8]) -> crate::Result<Self> {
        // TODO: Mysql/MySQL return 0 for affected rows in a SELECT .. statement.
        //       PostgreSQL returns a row count. Should we force return 0 for compatibilities sake?

        // Attempt to parse the last word in the command tag as an integer
        // If it can't be parased, the tag is probably "CREATE TABLE" or something
        // and we should return 0 rows

        let rows = buf
            .get_str_nul()?
            .rsplit(' ')
            .next()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        Ok(Self {
            affected_rows: rows,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{CommandComplete, Decode};

    const COMMAND_COMPLETE_INSERT: &[u8] = b"INSERT 0 1\0";
    const COMMAND_COMPLETE_UPDATE: &[u8] = b"UPDATE 512\0";
    const COMMAND_COMPLETE_CREATE_TABLE: &[u8] = b"CREATE TABLE\0";
    const COMMAND_COMPLETE_BEGIN: &[u8] = b"BEGIN\0";

    #[test]
    fn it_decodes_command_complete_for_insert() {
        let message = CommandComplete::decode(COMMAND_COMPLETE_INSERT).unwrap();

        assert_eq!(message.affected_rows(), 1);
    }

    #[test]
    fn it_decodes_command_complete_for_update() {
        let message = CommandComplete::decode(COMMAND_COMPLETE_UPDATE).unwrap();

        assert_eq!(message.affected_rows(), 512);
    }

    #[test]
    fn it_decodes_command_complete_for_begin() {
        let message = CommandComplete::decode(COMMAND_COMPLETE_BEGIN).unwrap();

        assert_eq!(message.affected_rows(), 0);
    }

    #[test]
    fn it_decodes_command_complete_for_create_table() {
        let message = CommandComplete::decode(COMMAND_COMPLETE_CREATE_TABLE).unwrap();

        assert_eq!(message.affected_rows(), 0);
    }
}
