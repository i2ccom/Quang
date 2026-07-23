//! GraphQL schema definitions for QuangHub.
//!
//! These define the query/mutation operations exposed by the server
//! and consumed by the Dioxus client. MVP uses raw query strings;
//! future versions will use cynic codegen.

/// Core GraphQL queries as typed string constants.
pub mod queries {
    // ── Auth ──

    pub const ME: &str = "
    query Me {
        me {
            id
            email
            displayName
            avatarUrl
            provider
            isAgent
        }
    }";

    // ── WorkSpace ──

    pub const WORKSPACES: &str = "
    query WorkSpaces {
        workspaces {
            id
            name
            description
            slug
            owner
            createdAt
            memberCount
        }
    }";

    pub const WORKSPACE: &str = "
    query WorkSpace($id: ID!) {
        workspace(id: $id) {
            id
            name
            description
            slug
            owner
            createdAt
            settings
            teams { id name memberCount }
            projects { id name status }
            channels { id name topic }
        }
    }";

    pub const CREATE_WORKSPACE: &str = "
    mutation CreateWorkSpace($name: String!, $description: String!, $slug: String!) {
        createWorkspace(name: $name, description: $description, slug: $slug) {
            id
            name
            slug
        }
    }";

    // ── Tasks ──

    pub const TASKS: &str = "
    query Tasks($projectId: ID!) {
        tasks(projectId: $projectId) {
            id
            title
            description
            status
            priority
            assignee { id displayName avatarUrl }
            createdAt
            dueDate
        }
    }";

    pub const CREATE_TASK: &str = "
    mutation CreateTask($title: String!, $description: String!, $projectId: ID!) {
        createTask(title: $title, description: $description, projectId: $projectId) {
            id
            title
            status
        }
    }";

    pub const UPDATE_TASK_STATUS: &str = "
    mutation UpdateTaskStatus($id: ID!, $status: String!) {
        updateTaskStatus(id: $id, status: $status) {
            id
            status
        }
    }";

    // ── Teams & Members ──

    pub const TEAM: &str = "
    query Team($id: ID!) {
        team(id: $id) {
            id
            name
            description
            members {
                id displayName avatarUrl role isAgent
            }
        }
    }";

    // ── Channels & Chat ──

    pub const CHANNEL_MESSAGES: &str = "
    query ChannelMessages($channelId: ID!, $limit: Int) {
        channelMessages(channelId: $channelId, limit: $limit) {
            id
            content
            author { id displayName avatarUrl }
            createdAt
            reactions { emoji count }
        }
    }";

    pub const SEND_MESSAGE: &str = "
    mutation SendMessage($channelId: ID!, $content: String!) {
        sendMessage(channelId: $channelId, content: $content) {
            id
            content
            createdAt
        }
    }";

    // ── Goals ──

    pub const GOALS: &str = "
    query Goals($projectId: ID!) {
        goals(projectId: $projectId) {
            id
            title
            description
            progress
            status
            period
        }
    }";

    // ── Reviews ──

    pub const REVIEWS: &str = "
    query Reviews($projectId: ID!) {
        reviews(projectId: $projectId) {
            id
            title
            status
            reviewer { id displayName }
            createdAt
        }
    }";

    // ── Meet ──

    pub const ROOMS: &str = "
    query Rooms {
        rooms {
            id
            name
            participantCount
            isActive
            createdAt
        }
    }";

    pub const CREATE_ROOM: &str = "
    mutation CreateRoom($name: String!) {
        createRoom(name: $name) {
            id
            name
            inviteUrl
        }
    }";

    // ── Repo ──

    pub const REPOS: &str = "
    query Repos {
        repos {
            id
            name
            description
            url
            defaultBranch
            lastCommitAt
            isLinked
        }
    }";

    pub const LINK_REPO: &str = "
    mutation LinkRepo($githubUrl: String!, $name: String!) {
        linkRepo(githubUrl: $githubUrl, name: $name) {
            id
            name
            url
            isLinked
        }
    }";

    pub const REPO_BROWSE: &str = "
    query RepoBrowse($repoId: ID!, $path: String!) {
        repoBrowse(repoId: $repoId, path: $path) {
            name
            path
            type
            size
            lastCommit
        }
    }";

    // ── AI Agents ──

    pub const AGENT_CHAT: &str = "
    mutation AgentChat($message: String!, $agentType: String!) {
        agentChat(message: $message, agentType: $agentType) {
            response
            actions { type description }
            artifacts { name url }
        }
    }";
}

/// Mutation helpers — a minimal set of typed wrappers.
pub mod mutations {
    use std::collections::HashMap;

    pub fn create_task_vars(
        title: &str,
        description: &str,
        project_id: &str,
    ) -> HashMap<String, serde_json::Value> {
        let mut vars = HashMap::new();
        vars.insert("title".into(), serde_json::json!(title));
        vars.insert("description".into(), serde_json::json!(description));
        vars.insert("projectId".into(), serde_json::json!(project_id));
        vars
    }

    pub fn create_workspace_vars(
        name: &str,
        description: &str,
        slug: &str,
    ) -> HashMap<String, serde_json::Value> {
        let mut vars = HashMap::new();
        vars.insert("name".into(), serde_json::json!(name));
        vars.insert("description".into(), serde_json::json!(description));
        vars.insert("slug".into(), serde_json::json!(slug));
        vars
    }

    pub fn send_message_vars(
        channel_id: &str,
        content: &str,
    ) -> HashMap<String, serde_json::Value> {
        let mut vars = HashMap::new();
        vars.insert("channelId".into(), serde_json::json!(channel_id));
        vars.insert("content".into(), serde_json::json!(content));
        vars
    }
}
