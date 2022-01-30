mod internal_request;
mod remote_request;

pub use internal_request::{
    InternalRequest, 
    InternalResponse,
    Request,
};
pub use remote_request::RemoteRequest;
