use seed::{prelude::*};

use cynic;

pub type Result<T> = std::result::Result<T, GraphQLError>;

pub async fn send_query<'a, ResponseData: 'a, Root: cynic::QueryRoot>(
    selection_set: cynic::SelectionSet<'a, ResponseData, Root>
) -> Result<ResponseData> {
    let query = cynic::Operation::query(selection_set);

    let graphql_response = 
        // @TODO: Move url to a config file.
        Request::new("https://time-tracker.eu-central-1.aws.cloud.dgraph.io/graphql")
            .method(Method::Post)
            .json(&query)?
            .fetch()
            .await?
            .check_status()?
            .json()
            .await?;

    let response_data = query.decode_response(graphql_response)?;
    if let Some(errors) = response_data.errors {
        Err(errors)?
    }
    Ok(response_data.data.expect("response data"))
}

pub async fn send_mutation<'a, ResponseData: 'a, Root: cynic::MutationRoot>(
    selection_set: cynic::SelectionSet<'a, ResponseData, Root>
) -> Result<ResponseData> {
    let mutation = cynic::Operation::mutation(selection_set);

    let graphql_response = 
        // @TODO: Move url to a config file.
        Request::new("https://time-tracker.eu-central-1.aws.cloud.dgraph.io/graphql")
            .method(Method::Post)
            .json(&mutation)?
            .fetch()
            .await?
            .check_status()?
            .json()
            .await?;

    let response_data = mutation.decode_response(graphql_response)?;
    if let Some(errors) = response_data.errors {
        Err(errors)?
    }
    Ok(response_data.data.expect("response data"))
}

// ------ Error ------

#[derive(Debug)]
pub enum GraphQLError {
    FetchError(FetchError),
    ResponseErrors(Vec<cynic::GraphQLError>),
    DecodeError(cynic::DecodeError)
}

impl From<FetchError> for GraphQLError {
    fn from(fetch_error: FetchError) -> Self {
        Self::FetchError(fetch_error)
    }
}

impl From<Vec<cynic::GraphQLError>> for GraphQLError {
    fn from(response_errors: Vec<cynic::GraphQLError>) -> Self {
        Self::ResponseErrors(response_errors)
    }
}

impl From<cynic::DecodeError> for GraphQLError {
    fn from(decode_error: cynic::DecodeError) -> Self {
        Self::DecodeError(decode_error)
    }
}

// ------ ------
// GraphQL items
// ------ ------

pub mod mutations {
    #[cynic::query_module(
        schema_path = "schema.graphql",
        query_module = "query_dsl",
    )]
    pub mod rename_client {
        use crate::graphql::query_dsl;

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
                    id: StringHashFilter {
                        eq: Some(args.id),
                    }
                },
                set: Some(ClientPatch {
                    name: Some(args.name),
                    projects: None,
                    time_blocks: None,
                    user: None,
                }),
                remove: None, 
            })]
            pub update_client: Option<UpdateClientPayload>,
        }

        #[derive(cynic::FragmentArguments, Clone, Debug)]
        pub struct RenameClientArguments {
            pub id: String,
            pub name: String,
        }

        #[derive(cynic::InputObject, Clone, Debug)]
        #[cynic(graphql_type = "UpdateClientInput")]
        pub struct UpdateClientInput {
            pub filter: ClientFilter,
            pub set: Option<ClientPatch>,
            pub remove: Option<ClientPatch>,
        }

        // @TODO Cannot make recursive structs 
        // because `cynic` doesn't support wrappers like `Rc` and `Box`.
        #[derive(cynic::InputObject, Debug)]
        #[cynic(graphql_type = "ClientFilter")]
        pub struct ClientFilter {
            pub id: Option<StringHashFilter>,
            pub and: Option<Box<ClientFilter>>,
            pub or: Option<Box<ClientFilter>>,
            pub not: Option<Box<ClientFilter>>,
        }

        #[derive(cynic::InputObject, Debug)]
        #[cynic(graphql_type = "StringHashFilter")]
        pub struct StringHashFilter {
            pub eq: Option<String>,
        }

        // @TODO: Can't derive `Debug`. `*Ref` types should implement `Debug`.
        // @TODO Is `Clone` for `InputObject` necessary? If so, `**Ref` types should implement `Clone`.
        // @TODO Should `*Ref` types implement `cynic::Scalar` to compile the code below?
        // @TODO How to correctly create and use `*Ref` types`?
        #[derive(cynic::InputObject, Debug)]
        #[cynic(graphql_type = "ClientPatch")]
        pub struct ClientPatch {
            pub name: Option<String>,
            pub projects: Option<Vec<query_dsl::ProjectRef>>,
            pub time_blocks: Option<Vec<query_dsl::TimeBlockRef>>,
            pub user: Option<String>,
        }

        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "UpdateClientPayload")]
        pub struct UpdateClientPayload {
            pub num_uids: Option<i32>,
        }
    }
}

pub mod queries {
    #[cynic::query_module(
        schema_path = "schema.graphql",
        query_module = "query_dsl",
    )]
    pub mod clients_with_projects {
        use crate::graphql::query_dsl;

        ///```graphql
        ///{
        ///    queryClient {
        ///        id
        ///        name
        ///        projects {
        ///            id
        ///            name
        ///        }
        ///    }
        ///}
        ///```
        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "Query")]
        pub struct Query {
            pub query_client: Option<Vec<Option<Client>>>,
        }

        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "Client")]
        pub struct Client {
            pub id: String,
            pub name: String,
            pub projects: Vec<Project>,
        }

        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "Project")]
        pub struct Project {
            pub id: String,
            pub name: String,
        }
    }

    #[cynic::query_module(
        schema_path = "schema.graphql",
        query_module = "query_dsl",
    )]
    pub mod clients_with_projects_with_time_entries {
        use crate::graphql::{query_dsl, types::*};

        ///```graphql
        ///{
        ///    queryClient {
        ///        id
        ///        name
        ///        projects {
        ///            id
        ///            name
        ///            time_entries {
        ///                id
        ///                name
        ///                started
        ///                stopped
        ///            }
        ///        }
        ///    }
        ///}
        ///```
        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "Query")]
        pub struct Query {
            pub query_client: Option<Vec<Option<Client>>>,
        }

        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "Client")]
        pub struct Client {
            pub id: String,
            pub name: String,
            pub projects: Vec<Project>,
        }

        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "Project")]
        pub struct Project {
            pub id: String,
            pub name: String,
            pub time_entries: Vec<TimeEntry>,
        }

        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "TimeEntry")]
        pub struct TimeEntry {
            pub id: String,
            pub name: String,
            pub started: DateTime,
            pub stopped: Option<DateTime>,
        }
    }

    #[cynic::query_module(
        schema_path = "schema.graphql",
        query_module = "query_dsl",
    )]
    pub mod clients_with_time_blocks_and_time_entries {
        use crate::graphql::{query_dsl, types::*};

        ///```graphql
        ///{
        ///    queryClient {
        ///        id
        ///        name
        ///        time_blocks {
        ///            id
        ///            name
        ///            status
        ///            duration
        ///            invoice {
        ///                id
        ///                custom_id
        ///                url
        ///            }
        ///        }
        ///        projects {
        ///            time_entries {
        ///                started
        ///                stopped
        ///            }
        ///        }
        ///    }
        ///}
        ///```
        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "Query")]
        pub struct Query {
            pub query_client: Option<Vec<Option<Client>>>,
        }

        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "Client")]
        pub struct Client {
            pub id: String,
            pub name: String,
            pub time_blocks: Vec<TimeBlock>,
            pub projects: Vec<Project>,
        }

        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "TimeBlock")]
        pub struct TimeBlock {
            pub id: String,
            pub name: String,
            pub status: TimeBlockStatus,
            pub duration: i32,
            pub invoice: Option<Invoice>,
        }

        #[allow(non_camel_case_types)]
        #[derive(cynic::Enum, Debug, Copy, Clone)]
        #[cynic(graphql_type = "TimeBlockStatus")]
        pub enum TimeBlockStatus {
            NON_BILLABLE,
            UNPAID,
            PAID,
        }

        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "Invoice")]
        pub struct Invoice {
            pub id: String,
            pub custom_id: Option<String>,
            pub url: Option<String>,
        }

        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "Project")]
        pub struct Project {
            pub time_entries: Vec<TimeEntry>,
        }

        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "TimeEntry")]
        pub struct TimeEntry {
            pub started: DateTime,
            pub stopped: Option<DateTime>,
        }
    }
}

mod types {
    #[derive(cynic::Scalar, Debug)]
    pub struct DateTime(pub String);
}

mod query_dsl {
    use super::types::*;
    cynic::query_dsl!("schema.graphql");
}

