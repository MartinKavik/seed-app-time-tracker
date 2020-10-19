#[cynic::query_module(
    schema_path = "schema.graphql",
    query_module = "query_dsl",
)]
pub mod add {
    use crate::graphql::{query_dsl, types::*};
    
    ///```graphql
    /// mutation {
    ///     addInvoice(input: {
    ///       id: "[invoice id]",
    ///       time_block: { id: "[time_block id]" },
    ///     }) {
    ///       numUids
    ///     }
    ///   }
    ///```
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(
        graphql_type = "Mutation",
        argument_struct = "AddInvoiceArguments",
    )]
    pub struct Mutation {
        #[arguments(input = vec![
            AddInvoiceInput {
                id: args.id.clone(),
                time_block: TimeBlockRef {
                    id: Some(args.time_block.clone())
                },
            }
        ])]
        pub add_invoice: Option<AddInvoicePayload>,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct AddInvoiceArguments {
        pub id: String,
        pub time_block: String,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "AddInvoiceInput")]
    pub struct AddInvoiceInput {
        id: String,
        time_block: TimeBlockRef,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "TimeBlockRef")]
    pub struct TimeBlockRef {
        id: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "AddInvoicePayload")]
    pub struct AddInvoicePayload {
        pub num_uids: Option<i32>,
    }
}

#[cynic::query_module(
    schema_path = "schema.graphql",
    query_module = "query_dsl",
)]
pub mod set_custom_id {
    use crate::graphql::{query_dsl, types::*};

    ///```graphql
    /// mutation {
    ///     updateInvoice(input: {
    ///       filter: {id: {eq: "[invoice id]"}}
    ///       set: { custom_id: "2020-05" }
    ///     }) {
    ///       numUids
    ///     }
    ///   }
    ///```
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(
        graphql_type = "Mutation",
        argument_struct = "SetInvoiceCustomIdArguments",
    )]
    pub struct Mutation {
        #[arguments(input = UpdateInvoiceInput {
            filter: InvoiceFilter {
                id: Some(StringHashFilter {
                    eq: Some(args.id.clone()),
                }),
            },
            set: Some(InvoicePatch {
                custom_id: Some(args.custom_id.clone()),
            }),
        })]
        pub update_invoice: Option<UpdateInvoicePayload>,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct SetInvoiceCustomIdArguments {
        pub id: String,
        pub custom_id: String,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "UpdateInvoiceInput")]
    pub struct UpdateInvoiceInput {
        pub filter: InvoiceFilter,
        pub set: Option<InvoicePatch>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "InvoiceFilter")]
    pub struct InvoiceFilter {
        pub id: Option<StringHashFilter>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "StringHashFilter")]
    pub struct StringHashFilter {
        pub eq: Option<String>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "InvoicePatch")]
    pub struct InvoicePatch {
        pub custom_id: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "UpdateInvoicePayload")]
    pub struct UpdateInvoicePayload {
        pub num_uids: Option<i32>,
    }
}

#[cynic::query_module(
    schema_path = "schema.graphql",
    query_module = "query_dsl",
)]
pub mod set_url {
    use crate::graphql::{query_dsl, types::*};

    ///```graphql
    /// mutation {
    ///     updateInvoice(input: {
    ///       filter: {id: {eq: "[invoice id]"}}
    ///       set: { url: "https://example.com/my-invoice.pdf" }
    ///     }) {
    ///       numUids
    ///     }
    ///   }
    ///```
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(
        graphql_type = "Mutation",
        argument_struct = "SetInvoiceUrlArguments",
    )]
    pub struct Mutation {
        #[arguments(input = UpdateInvoiceInput {
            filter: InvoiceFilter {
                id: Some(StringHashFilter {
                    eq: Some(args.id.clone()),
                }),
            },
            set: Some(InvoicePatch {
                url: Some(args.url.clone()),
            }),
        })]
        pub update_invoice: Option<UpdateInvoicePayload>,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct SetInvoiceUrlArguments {
        pub id: String,
        pub url: String,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "UpdateInvoiceInput")]
    pub struct UpdateInvoiceInput {
        pub filter: InvoiceFilter,
        pub set: Option<InvoicePatch>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "InvoiceFilter")]
    pub struct InvoiceFilter {
        pub id: Option<StringHashFilter>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "StringHashFilter")]
    pub struct StringHashFilter {
        pub eq: Option<String>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "InvoicePatch")]
    pub struct InvoicePatch {
        pub url: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "UpdateInvoicePayload")]
    pub struct UpdateInvoicePayload {
        pub num_uids: Option<i32>,
    }
}

#[cynic::query_module(
    schema_path = "schema.graphql",
    query_module = "query_dsl",
)]
pub mod delete {
    use crate::graphql::{query_dsl, types::*};

    ///```graphql
    /// mutation {
    ///     deleteInvoice(input: {
    ///       filter: {id: {eq: "[invoice id]"}}
    ///     }) {
    ///       numUids
    ///     }
    ///   }
    ///```
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(
        graphql_type = "Mutation",
        argument_struct = "DeleteInvoiceArguments",
    )]
    pub struct Mutation {
        #[arguments(filter = InvoiceFilter {
            id: Some(StringHashFilter {
                eq: Some(args.id.clone()),
            })
        })]
        pub delete_invoice: Option<DeleteInvoicePayload>,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct DeleteInvoiceArguments {
        pub id: String,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "InvoiceFilter")]
    pub struct InvoiceFilter {
        pub id: Option<StringHashFilter>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "StringHashFilter")]
    pub struct StringHashFilter {
        pub eq: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "DeleteInvoicePayload")]
    pub struct DeleteInvoicePayload {
        pub num_uids: Option<i32>,
    }
}


