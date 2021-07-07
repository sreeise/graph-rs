use crate::{builder::Builder, parser::client_resource::ClientResource};

pub trait Parse<ParseFrom> {
    type Error: std::error::Error;

    fn parse(
        parse_from: ParseFrom,
        client_resource: ClientResource,
    ) -> Result<Builder, Self::Error>;
}
