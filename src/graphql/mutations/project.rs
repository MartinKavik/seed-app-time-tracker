#[cynic::query_module(
    schema_path = "schema.graphql",
    query_module = "query_dsl",
)]
pub mod add {
    use crate::graphql::{query_dsl, types::*};
    
    ///```graphql
    /// mutation {
    ///     addProject(input: {
    ///       id: "[project id]",
    ///       name: "",
    ///       time_entries: [],
    ///       client: { id: "[client id]" },
    ///     }) {
    ///       numUids
    ///     }
    ///   }
    ///```
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(
        graphql_type = "Mutation",
        argument_struct = "AddProjectArguments",
    )]
    pub struct Mutation {
        #[arguments(input = vec![
            AddProjectInput {
                id: args.id.clone(),
                name: String::new(),
                time_entries: Vec::new(),
                client: ClientRef {
                    id: Some(args.client.clone())
                },
            }
        ])]
        pub add_project: Option<AddProjectPayload>,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct AddProjectArguments {
        pub id: String,
        pub client: String,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "AddProjectInput")]
    pub struct AddProjectInput {
        id: String,
        name: String,
        time_entries: Vec<TimeEntryRef>,
        client: ClientRef,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "TimeEntryRef")]
    pub struct TimeEntryRef {}

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "ClientRef")]
    pub struct ClientRef {
        id: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "AddProjectPayload")]
    pub struct AddProjectPayload {
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
    ///     updateProject(input: {
    ///       filter: {id: {eq: "[project id]"}}
    ///       set: {name: "New Project Name"}
    ///     }) {
    ///       numUids
    ///     }
    ///   }
    ///```
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(
        graphql_type = "Mutation",
        argument_struct = "RenameProjectArguments",
    )]
    pub struct Mutation {
        #[arguments(input = UpdateProjectInput {
            filter: ProjectFilter {
                id: Some(StringHashFilter {
                    eq: Some(args.id.clone()),
                }),
            },
            set: Some(ProjectPatch {
                name: Some(args.name.clone()),
            }),
        })]
        pub update_project: Option<UpdateProjectPayload>,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct RenameProjectArguments {
        pub id: String,
        pub name: String,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "UpdateProjectInput")]
    pub struct UpdateProjectInput {
        pub filter: ProjectFilter,
        pub set: Option<ProjectPatch>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "ProjectFilter")]
    pub struct ProjectFilter {
        pub id: Option<StringHashFilter>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "StringHashFilter")]
    pub struct StringHashFilter {
        pub eq: Option<String>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "ProjectPatch")]
    pub struct ProjectPatch {
        pub name: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "UpdateProjectPayload")]
    pub struct UpdateProjectPayload {
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
    ///     deleteProject(input: {
    ///       filter: {id: {eq: "[project id]"}}
    ///     }) {
    ///       numUids
    ///     }
    ///   }
    ///```
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(
        graphql_type = "Mutation",
        argument_struct = "DeleteProjectArguments",
    )]
    pub struct Mutation {
        #[arguments(filter = ProjectFilter {
            id: Some(StringHashFilter {
                eq: Some(args.id.clone()),
            })
        })]
        pub delete_project: Option<DeleteProjectPayload>,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct DeleteProjectArguments {
        pub id: String,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "ProjectFilter")]
    pub struct ProjectFilter {
        pub id: Option<StringHashFilter>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "StringHashFilter")]
    pub struct StringHashFilter {
        pub eq: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "DeleteProjectPayload")]
    pub struct DeleteProjectPayload {
        pub num_uids: Option<i32>,
    }
}


