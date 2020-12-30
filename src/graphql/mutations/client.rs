#[cynic::query_module(
    schema_path = "schema.graphql",
    query_module = "query_dsl",
)]
pub mod add {
    use crate::graphql::{query_dsl, types::*};

    ///```graphql
    /// mutation {
    ///     addClient(input: {
    ///       id: "[client id]",
    ///       name: "",
    ///       projects: [],
    ///       time_blocks: [],
    ///       user: "[user id]",
    ///     }) {
    ///       numUids
    ///     }
    ///   }
    ///```
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(
        graphql_type = "Mutation",
        argument_struct = "AddClientArguments",
    )]
    pub struct Mutation {
        #[arguments(input = vec![
            AddClientInput {
                id: args.id.clone(),
                name: String::new(),
                projects: Vec::new(),
                time_blocks: Vec::new(),
                user: args.user.clone(),
            }
        ])]
        pub add_client: Option<AddClientPayload>,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct AddClientArguments {
        pub id: String,
        pub user: String,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "AddClientInput", rename_all = "None")]
    pub struct AddClientInput {
        id: String,
        name: String,
        projects: Vec<ProjectRef>,
        time_blocks: Vec<TimeBlockRef>,
        user: String,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "ProjectRef")]
    pub struct ProjectRef {}

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "TimeBlockRef")]
    pub struct TimeBlockRef {}

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "AddClientPayload")]
    pub struct AddClientPayload {
        pub num_uids: Option<i32>,
    }
}

#[cynic::query_module(
    schema_path = "schema.graphql",
    query_module = "query_dsl",
)]
pub mod rename {
    use crate::graphql::{query_dsl, types::*};

    ///```graphql
    /// mutation {
    ///     updateClient(input: {
    ///       filter: {id: {eq: "[client id]"}}
    ///       set: {name: "New Client Name"}
    ///     }) {
    ///       numUids
    ///     }
    ///   }
    ///```
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(
        graphql_type = "Mutation",
        argument_struct = "RenameClientArguments",
    )]
    pub struct Mutation {
        #[arguments(input = UpdateClientInput {
            filter: ClientFilter {
                id: Some(StringHashFilter {
                    eq: Some(args.id.clone()),
                }),
            },
            set: Some(ClientPatch {
                name: Some(args.name.clone()),
            }),
        })]
        pub update_client: Option<UpdateClientPayload>,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct RenameClientArguments {
        pub id: String,
        pub name: String,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "UpdateClientInput")]
    pub struct UpdateClientInput {
        pub filter: ClientFilter,
        pub set: Option<ClientPatch>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "ClientFilter")]
    pub struct ClientFilter {
        pub id: Option<StringHashFilter>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "StringHashFilter")]
    pub struct StringHashFilter {
        pub eq: Option<String>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "ClientPatch")]
    pub struct ClientPatch {
        pub name: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "UpdateClientPayload")]
    pub struct UpdateClientPayload {
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
    ///     deleteClient(input: {
    ///       filter: {id: {eq: "[client id]"}}
    ///     }) {
    ///       numUids
    ///     }
    ///   }
    ///```
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(
        graphql_type = "Mutation",
        argument_struct = "DeleteClientArguments",
    )]
    pub struct Mutation {
        #[arguments(filter = ClientFilter {
            id: Some(StringHashFilter {
                eq: Some(args.id.clone()),
            })
        })]
        pub delete_client: Option<DeleteClientPayload>,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct DeleteClientArguments {
        pub id: String,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "ClientFilter")]
    pub struct ClientFilter {
        pub id: Option<StringHashFilter>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "StringHashFilter")]
    pub struct StringHashFilter {
        pub eq: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "DeleteClientPayload")]
    pub struct DeleteClientPayload {
        pub num_uids: Option<i32>,
    }
}


