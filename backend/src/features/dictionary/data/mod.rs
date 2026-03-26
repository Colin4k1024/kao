pub mod model;
pub mod repo;
pub mod service;
pub mod routes;

pub use model::{CreateDataRequest, UpdateDataRequest, DataResponse, DataRecord};
pub use repo::DataRepository;
pub use service::DataService;
