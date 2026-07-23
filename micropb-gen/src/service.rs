//! Extension points for generating code from Protobuf services.
//!
//! [`Generator::add_service_generator`] registers a [`ServiceGenerator`]. Its
//! output is placed alongside the generated message types.
//!
//! [`Generator::add_service_generator`]: crate::Generator::add_service_generator

use proc_macro2::TokenStream;

/// A Protobuf service method.
#[derive(Debug, Clone)]
pub struct MethodView<'a> {
    /// Name as declared in the `.proto` file.
    pub name: &'a str,
    /// gRPC-style path, such as `"/pkg.Service/Read"`.
    pub full_path: String,
    /// Fully qualified Protobuf name of the request type.
    pub input_type: &'a str,
    /// Fully qualified Protobuf name of the response type.
    pub output_type: &'a str,
    /// Whether the request is streamed.
    pub client_streaming: bool,
    /// Whether the response is streamed.
    pub server_streaming: bool,
}

/// A Protobuf service.
#[derive(Debug, Clone)]
pub struct ServiceView<'a> {
    /// Name as declared in the `.proto` file.
    pub name: &'a str,
    /// Package of the declaring file, or an empty string if it has none.
    pub package: &'a str,
    /// Zero-based index of this service within its file's `service` list.
    pub index: u16,
    /// Methods in declaration order.
    pub methods: Vec<MethodView<'a>>,
}

/// Resolves Protobuf type names to their generated Rust paths.
///
/// The returned paths are relative to the package module containing the
/// generated service code and account for extern types and configured naming.
pub trait TypeResolver {
    /// Resolve a fully qualified Protobuf type name such as
    /// `".pkg.ReadRequest"`.
    fn rust_path(&self, proto_type: &str) -> TokenStream;
}

/// Generates Rust code from Protobuf `service` definitions.
///
/// One instance is reused for the entire descriptor set. Returned tokens are
/// emitted into the service's package module.
///
/// [`Generator::add_service_generator`]: crate::Generator::add_service_generator
pub trait ServiceGenerator {
    /// Generate code for one service.
    fn generate(&mut self, service: &ServiceView, resolver: &dyn TypeResolver) -> TokenStream;
}
