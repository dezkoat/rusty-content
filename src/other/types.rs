use serde::Serialize;
use diesel::serialize::Output;
use std::io::Write;
use diesel::sql_types::{BigInt, Timestamp, Timestamptz};
use diesel::pg::Pg;
use serde::Serializer;
use diesel::serialize::{self, ToSql};
use diesel::deserialize::{self, FromSql};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromSqlRow, AsExpression)]
#[sql_type = "Timestamp"]
#[sql_type = "Timestamptz"]
pub struct PgTimestamp (
    pub i64,
);

impl Serialize for PgTimestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(self.0)
    }
}

impl ToSql<Timestamp, Pg> for PgTimestamp {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        ToSql::<BigInt, Pg>::to_sql(&self.0, out)
    }
}

impl FromSql<Timestamp, Pg> for PgTimestamp {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        FromSql::<BigInt, Pg>::from_sql(bytes).map(PgTimestamp)
    }
}

impl ToSql<Timestamptz, Pg> for PgTimestamp {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        ToSql::<Timestamp, Pg>::to_sql(self, out)
    }
}

impl FromSql<Timestamptz, Pg> for PgTimestamp {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        FromSql::<Timestamp, Pg>::from_sql(bytes)
    }
}
