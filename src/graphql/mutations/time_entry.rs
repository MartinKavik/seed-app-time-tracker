#[cynic::query_module(
    schema_path = "schema.graphql",
    query_module = "query_dsl",
)]
pub mod add {
    use crate::graphql::{query_dsl, types::*};
    
    ///```graphql
    /// mutation {
    ///     addTimeEntry(input: {
    ///       id: "[time_entry id]",
    ///       name: "[time_entry name]",
    ///       started: "2020-01-15T15:53:39Z",
    ///       project: { id: "[project id]" },
    ///     }) {
    ///       numUids
    ///     }
    ///   }
    ///```
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(
        graphql_type = "Mutation",
        argument_struct = "AddTimeEntryArguments",
    )]
    pub struct Mutation {
        #[arguments(input = vec![
            AddTimeEntryInput {
                id: args.id.clone(),
                name: args.name.clone(),
                started: DateTime(args.started.to_rfc3339()),
                project: ProjectRef {
                    id: Some(args.project.clone())
                },
            }
        ])]
        pub add_time_entry: Option<AddTimeEntryPayload>,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct AddTimeEntryArguments {
        pub id: String,
        pub name: String,
        pub project: String,
        pub started: chrono::DateTime<chrono::Local>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "AddTimeEntryInput")]
    pub struct AddTimeEntryInput {
        id: String,
        name: String,
        started: DateTime,
        project: ProjectRef,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "ProjectRef")]
    pub struct ProjectRef {
        id: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "AddTimeEntryPayload")]
    pub struct AddTimeEntryPayload {
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
    ///     updateTimeEntry(input: {
    ///       filter: {id: {eq: "[time_entry id]"}}
    ///       set: {name: "New Time Entry Name"}
    ///     }) {
    ///       numUids
    ///     }
    ///   }
    ///```
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(
        graphql_type = "Mutation",
        argument_struct = "RenameTimeEntryArguments",
    )]
    pub struct Mutation {
        #[arguments(input = UpdateTimeEntryInput {
            filter: TimeEntryFilter {
                id: Some(StringHashFilter {
                    eq: Some(args.id.clone()),
                }),
            },
            set: Some(TimeEntryPatch {
                name: Some(args.name.clone()),
            }),
        })]
        pub update_time_entry: Option<UpdateTimeEntryPayload>,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct RenameTimeEntryArguments {
        pub id: String,
        pub name: String,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "UpdateTimeEntryInput")]
    pub struct UpdateTimeEntryInput {
        pub filter: TimeEntryFilter,
        pub set: Option<TimeEntryPatch>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "TimeEntryFilter")]
    pub struct TimeEntryFilter {
        pub id: Option<StringHashFilter>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "StringHashFilter")]
    pub struct StringHashFilter {
        pub eq: Option<String>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "TimeEntryPatch")]
    pub struct TimeEntryPatch {
        pub name: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "UpdateTimeEntryPayload")]
    pub struct UpdateTimeEntryPayload {
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
    ///     deleteTimeEntry(input: {
    ///       filter: {id: {eq: "[time_entry id]"}}
    ///     }) {
    ///       numUids
    ///     }
    ///   }
    ///```
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(
        graphql_type = "Mutation",
        argument_struct = "DeleteTimeEntryArguments",
    )]
    pub struct Mutation {
        #[arguments(filter = TimeEntryFilter {
            id: Some(StringHashFilter {
                eq: Some(args.id.clone()),
            })
        })]
        pub delete_time_entry: Option<DeleteTimeEntryPayload>,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct DeleteTimeEntryArguments {
        pub id: String,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "TimeEntryFilter")]
    pub struct TimeEntryFilter {
        pub id: Option<StringHashFilter>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "StringHashFilter")]
    pub struct StringHashFilter {
        pub eq: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "DeleteTimeEntryPayload")]
    pub struct DeleteTimeEntryPayload {
        pub num_uids: Option<i32>,
    }
}

#[cynic::query_module(
    schema_path = "schema.graphql",
    query_module = "query_dsl",
)]
pub mod set_times {
    use crate::graphql::{query_dsl, types::*};

    ///```graphql
    /// mutation {
    ///     updateTimeEntry(input: {
    ///       filter: {id: {eq: "[time_entry id]"}}
    ///       set: {
    ///         started: "2020-01-15T15:53:39Z",
    ///         stopped: "2020-01-15T15:53:39Z",
    ///       }
    ///     }) {
    ///       numUids
    ///     }
    ///   }
    ///```
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(
        graphql_type = "Mutation",
        argument_struct = "SetTimeEntryTimesArguments",
    )]
    pub struct Mutation {
        #[arguments(input = UpdateTimeEntryInput {
            filter: TimeEntryFilter {
                id: Some(StringHashFilter {
                    eq: Some(args.id.clone()),
                }),
            },
            set: Some(TimeEntryPatch {
                started: Some(DateTime(args.started.to_rfc3339())),
                stopped: args.stopped.map(|stopped| DateTime(stopped.to_rfc3339())),
            }),
        })]
        pub update_time_entry: Option<UpdateTimeEntryPayload>,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct SetTimeEntryTimesArguments {
        pub id: String,
        pub started: chrono::DateTime<chrono::Local>,
        pub stopped: Option<chrono::DateTime<chrono::Local>>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "UpdateTimeEntryInput")]
    pub struct UpdateTimeEntryInput {
        pub filter: TimeEntryFilter,
        pub set: Option<TimeEntryPatch>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "TimeEntryFilter")]
    pub struct TimeEntryFilter {
        pub id: Option<StringHashFilter>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "StringHashFilter")]
    pub struct StringHashFilter {
        pub eq: Option<String>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "TimeEntryPatch")]
    pub struct TimeEntryPatch {
        pub started: Option<DateTime>,
        pub stopped: Option<DateTime>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "UpdateTimeEntryPayload")]
    pub struct UpdateTimeEntryPayload {
        pub num_uids: Option<i32>,
    }
}


