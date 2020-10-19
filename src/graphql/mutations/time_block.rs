#[cynic::query_module(
    schema_path = "schema.graphql",
    query_module = "query_dsl",
)]
pub mod add {
    use crate::graphql::{query_dsl, types::*};
    
    ///```graphql
    /// mutation {
    ///     addTimeBlock(input: {
    ///       id: "[time_block id]",
    ///       name: "",
    ///       status: UNPAID,
    ///       duration: 72000,
    ///       client: { id: "[client id]" },
    ///     }) {
    ///       numUids
    ///     }
    ///   }
    ///```
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(
        graphql_type = "Mutation",
        argument_struct = "AddTimeBlockArguments",
    )]
    pub struct Mutation {
        #[arguments(input = vec![
            AddTimeBlockInput {
                id: args.id.clone(),
                name: String::new(),
                duration: args.duration.num_seconds() as i32,
                status: TimeBlockStatus::Unpaid,
                client: ClientRef {
                    id: Some(args.client.clone())
                },
            }
        ])]
        pub add_time_block: Option<AddTimeBlockPayload>,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct AddTimeBlockArguments {
        pub id: String,
        pub client: String,
        pub duration: chrono::Duration,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "AddTimeBlockInput")]
    pub struct AddTimeBlockInput {
        id: String,
        name: String,
        duration: i32,
        status: TimeBlockStatus,
        client: ClientRef,
    }

    #[derive(cynic::Enum, Debug, Copy, Clone)]
    #[cynic(graphql_type = "TimeBlockStatus", rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum TimeBlockStatus {
        NonBillable,
        Unpaid,
        Paid,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "ClientRef")]
    pub struct ClientRef {
        id: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "AddTimeBlockPayload")]
    pub struct AddTimeBlockPayload {
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
    ///     updateTimeBlock(input: {
    ///       filter: {id: {eq: "[time_block id]"}}
    ///       set: {name: "New TimeBlock Name"}
    ///     }) {
    ///       numUids
    ///     }
    ///   }
    ///```
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(
        graphql_type = "Mutation",
        argument_struct = "RenameTimeBlockArguments",
    )]
    pub struct Mutation {
        #[arguments(input = UpdateTimeBlockInput {
            filter: TimeBlockFilter {
                id: Some(StringHashFilter {
                    eq: Some(args.id.clone()),
                }),
            },
            set: Some(TimeBlockPatch {
                name: Some(args.name.clone()),
            }),
        })]
        pub update_time_block: Option<UpdateTimeBlockPayload>,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct RenameTimeBlockArguments {
        pub id: String,
        pub name: String,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "UpdateTimeBlockInput")]
    pub struct UpdateTimeBlockInput {
        pub filter: TimeBlockFilter,
        pub set: Option<TimeBlockPatch>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "TimeBlockFilter")]
    pub struct TimeBlockFilter {
        pub id: Option<StringHashFilter>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "StringHashFilter")]
    pub struct StringHashFilter {
        pub eq: Option<String>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "TimeBlockPatch")]
    pub struct TimeBlockPatch {
        pub name: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "UpdateTimeBlockPayload")]
    pub struct UpdateTimeBlockPayload {
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
    ///     deleteTimeBlock(input: {
    ///       filter: {id: {eq: "[time_block id]"}}
    ///     }) {
    ///       numUids
    ///     }
    ///   }
    ///```
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(
        graphql_type = "Mutation",
        argument_struct = "DeleteTimeBlockArguments",
    )]
    pub struct Mutation {
        #[arguments(filter = TimeBlockFilter {
            id: Some(StringHashFilter {
                eq: Some(args.id.clone()),
            })
        })]
        pub delete_time_block: Option<DeleteTimeBlockPayload>,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct DeleteTimeBlockArguments {
        pub id: String,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "TimeBlockFilter")]
    pub struct TimeBlockFilter {
        pub id: Option<StringHashFilter>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "StringHashFilter")]
    pub struct StringHashFilter {
        pub eq: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "DeleteTimeBlockPayload")]
    pub struct DeleteTimeBlockPayload {
        pub num_uids: Option<i32>,
    }
}

#[cynic::query_module(
    schema_path = "schema.graphql",
    query_module = "query_dsl",
)]
pub mod set_duration {
    use crate::graphql::{query_dsl, types::*};

    ///```graphql
    /// mutation {
    ///     updateTimeBlock(input: {
    ///       filter: {id: {eq: "[time_block id]"}}
    ///       set: { duration: 36000 }
    ///     }) {
    ///       numUids
    ///     }
    ///   }
    ///```
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(
        graphql_type = "Mutation",
        argument_struct = "SetTimeBlockDurationArguments",
    )]
    pub struct Mutation {
        #[arguments(input = UpdateTimeBlockInput {
            filter: TimeBlockFilter {
                id: Some(StringHashFilter {
                    eq: Some(args.id.clone()),
                }),
            },
            set: Some(TimeBlockPatch {
                duration: Some(args.duration),
            }),
        })]
        pub update_time_block: Option<UpdateTimeBlockPayload>,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct SetTimeBlockDurationArguments {
        pub id: String,
        pub duration: i32,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "UpdateTimeBlockInput")]
    pub struct UpdateTimeBlockInput {
        pub filter: TimeBlockFilter,
        pub set: Option<TimeBlockPatch>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "TimeBlockFilter")]
    pub struct TimeBlockFilter {
        pub id: Option<StringHashFilter>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "StringHashFilter")]
    pub struct StringHashFilter {
        pub eq: Option<String>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "TimeBlockPatch")]
    pub struct TimeBlockPatch {
        pub duration: Option<i32>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "UpdateTimeBlockPayload")]
    pub struct UpdateTimeBlockPayload {
        pub num_uids: Option<i32>,
    }
}

#[cynic::query_module(
    schema_path = "schema.graphql",
    query_module = "query_dsl",
)]
pub mod set_status {
    use crate::graphql::{query_dsl, types::*};

    ///```graphql
    /// mutation {
    ///     updateTimeBlock(input: {
    ///       filter: {id: {eq: "[time_block id]"}}
    ///       set: { status: PAID }
    ///     }) {
    ///       numUids
    ///     }
    ///   }
    ///```
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(
        graphql_type = "Mutation",
        argument_struct = "SetTimeBlockStatusArguments",
    )]
    pub struct Mutation {
        #[arguments(input = UpdateTimeBlockInput {
            filter: TimeBlockFilter {
                id: Some(StringHashFilter {
                    eq: Some(args.id.clone()),
                }),
            },
            set: Some(TimeBlockPatch {
                status: Some(args.status),
            }),
        })]
        pub update_time_block: Option<UpdateTimeBlockPayload>,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct SetTimeBlockStatusArguments {
        pub id: String,
        pub status: TimeBlockStatus,
    }

    #[derive(cynic::Enum, Debug, Copy, Clone)]
    #[cynic(graphql_type = "TimeBlockStatus", rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum TimeBlockStatus {
        NonBillable,
        Unpaid,
        Paid,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "UpdateTimeBlockInput")]
    pub struct UpdateTimeBlockInput {
        pub filter: TimeBlockFilter,
        pub set: Option<TimeBlockPatch>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "TimeBlockFilter")]
    pub struct TimeBlockFilter {
        pub id: Option<StringHashFilter>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "StringHashFilter")]
    pub struct StringHashFilter {
        pub eq: Option<String>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "TimeBlockPatch")]
    pub struct TimeBlockPatch {
        pub status: Option<TimeBlockStatus>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "UpdateTimeBlockPayload")]
    pub struct UpdateTimeBlockPayload {
        pub num_uids: Option<i32>,
    }
}
