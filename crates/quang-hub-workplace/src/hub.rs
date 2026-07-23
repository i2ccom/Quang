//! WorkplaceHub — the top-level orchestrator for the collaboration ecosystem.
//!
//! The WorkplaceHub owns the HyperGraph, the EventBus, the ViewRegistry,
//! and exposes high-level operations that emit events and mutate the graph.
//! It is the primary API surface for both human UI and agent tools.

use crate::event::{CollabEvent, EventBus};
use crate::graph::{ActorId, Edge, EdgeKind, HyperGraph, NodeId, NodeKind};
use crate::view::{ViewConfig, ViewProjection, ViewRegistry};
use crate::views;

use crate::channel::{Channel, ChannelKind};
use crate::chat::{ChatMessage, MessageContent};
use crate::goal::Goal;
use crate::project::Project;
use crate::review::{Review, ReviewTargetKind};
use crate::summary::{Summary, SummaryKind};
use crate::task::{Task, TaskStatus};
use crate::team::{Team, TeamMember};
use crate::workspace::WorkSpace;

/// The central orchestrator for workplace collaboration.
/// Wraps the HyperGraph, EventBus, and ViewRegistry.
pub struct WorkplaceHub {
    pub graph: HyperGraph,
    pub events: EventBus,
    pub views: ViewRegistry,
}

impl WorkplaceHub {
    pub fn new() -> Self {
        let mut hub = Self {
            graph: HyperGraph::new(),
            events: EventBus::new(),
            views: ViewRegistry::new(),
        };
        views::register_builtin_views(&mut hub.views);
        hub
    }

    // ── WorkSpace operations ──

    pub fn create_workspace(
        &mut self,
        name: &str,
        description: &str,
        slug: &str,
        owner: ActorId,
    ) -> NodeId {
        let ws = WorkSpace::new(name, description, slug, owner.clone());
        let id = ws.id.clone();
        self.graph.add_node(id.clone(), NodeKind::WorkSpace, &ws);
        self.events.emit(CollabEvent::WorkSpaceCreated {
            workspace_id: id.clone(),
            actor: owner,
        });
        id
    }

    pub fn get_workspace(&self, id: &NodeId) -> Option<WorkSpace> {
        self.graph
            .nodes
            .get(id)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    // ── Team operations ──

    pub fn create_team(
        &mut self,
        name: &str,
        description: &str,
        workspace_id: &NodeId,
        actor: ActorId,
    ) -> NodeId {
        let team = Team::new(name, description);
        let id = team.id.clone();

        self.graph.add_node(id.clone(), NodeKind::Team, &team);
        self.graph.add_edge(Edge::new(
            EdgeKind::ChildOf,
            workspace_id.clone(),
            id.clone(),
        ));
        self.events.emit(CollabEvent::TeamCreated {
            team_id: id.clone(),
            workspace_id: workspace_id.clone(),
            actor,
        });
        id
    }

    pub fn add_team_member(&mut self, team_id: &NodeId, member: TeamMember, actor: ActorId) {
        if let Some(mut team) = self.get_team(team_id) {
            let member_actor = member.actor.clone();
            team.add_member(member);
            self.graph.add_node(team_id.clone(), NodeKind::Team, &team);
            self.graph.add_edge(Edge::new(
                EdgeKind::MemberOf,
                NodeId(member_actor.to_string()),
                team_id.clone(),
            ));
            self.events.emit(CollabEvent::TeamMemberAdded {
                team_id: team_id.clone(),
                member: member_actor,
                actor,
            });
        }
    }

    pub fn get_team(&self, id: &NodeId) -> Option<Team> {
        self.graph
            .nodes
            .get(id)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    // ── Project operations ──

    pub fn create_project(
        &mut self,
        name: &str,
        description: &str,
        owner: ActorId,
        parent_id: &NodeId,
    ) -> NodeId {
        let project = Project::new(name, description, owner.clone());
        let id = project.id.clone();

        self.graph.add_node(id.clone(), NodeKind::Project, &project);
        self.graph
            .add_edge(Edge::new(EdgeKind::ChildOf, parent_id.clone(), id.clone()));
        self.events.emit(CollabEvent::ProjectCreated {
            project_id: id.clone(),
            workspace_id: parent_id.clone(),
            actor: owner,
        });
        id
    }

    pub fn get_project(&self, id: &NodeId) -> Option<Project> {
        self.graph
            .nodes
            .get(id)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    // ── Channel operations ──

    pub fn create_channel(
        &mut self,
        name: &str,
        topic: &str,
        kind: ChannelKind,
        parent_id: &NodeId,
        created_by: ActorId,
    ) -> NodeId {
        let channel = Channel::new(name, topic, kind, created_by.clone());
        let id = channel.id.clone();

        self.graph.add_node(id.clone(), NodeKind::Channel, &channel);
        self.graph
            .add_edge(Edge::new(EdgeKind::ChildOf, parent_id.clone(), id.clone()));
        self.events.emit(CollabEvent::ChannelCreated {
            channel_id: id.clone(),
            parent_id: parent_id.clone(),
            actor: created_by,
        });
        id
    }

    pub fn get_channel(&self, id: &NodeId) -> Option<Channel> {
        self.graph
            .nodes
            .get(id)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    // ── Chat operations ──

    pub fn send_message(
        &mut self,
        channel_id: &NodeId,
        author: ActorId,
        content: MessageContent,
    ) -> NodeId {
        let msg = ChatMessage::new(channel_id.clone(), author.clone(), content);
        let id = msg.id.clone();

        self.graph.add_node(id.clone(), NodeKind::ChatMessage, &msg);
        self.graph.add_edge(Edge::new(
            EdgeKind::BelongsTo,
            channel_id.clone(),
            id.clone(),
        ));
        self.graph.add_edge(Edge::new(
            EdgeKind::CreatedBy,
            id.clone(),
            NodeId(author.to_string()),
        ));

        // Update channel activity
        if let Some(mut channel) = self.get_channel(channel_id) {
            channel.touch();
            self.graph
                .add_node(channel_id.clone(), NodeKind::Channel, &channel);
        }

        self.events.emit(CollabEvent::MessageSent {
            message_id: id.clone(),
            channel_id: channel_id.clone(),
            author,
        });
        id
    }

    // ── Task operations ──

    pub fn create_task(
        &mut self,
        title: &str,
        description: &str,
        created_by: ActorId,
        parent_id: &NodeId,
    ) -> NodeId {
        let task = Task::new(title, description, created_by.clone());
        let id = task.id.clone();

        self.graph.add_node(id.clone(), NodeKind::Task, &task);
        self.graph
            .add_edge(Edge::new(EdgeKind::ChildOf, parent_id.clone(), id.clone()));
        self.events.emit(CollabEvent::TaskCreated {
            task_id: id.clone(),
            parent_id: parent_id.clone(),
            actor: created_by,
        });
        id
    }

    pub fn assign_task(&mut self, task_id: &NodeId, assignee: ActorId, actor: ActorId) {
        if let Some(mut task) = self.get_task(task_id) {
            task.assign(assignee.clone());
            self.graph.add_node(task_id.clone(), NodeKind::Task, &task);
            self.graph.add_edge(Edge::new(
                EdgeKind::AssignedTo,
                task_id.clone(),
                NodeId(assignee.to_string()),
            ));
            self.events.emit(CollabEvent::TaskAssigned {
                task_id: task_id.clone(),
                assignee,
                actor,
            });
        }
    }

    pub fn transition_task(
        &mut self,
        task_id: &NodeId,
        new_status: TaskStatus,
        actor: ActorId,
    ) -> Result<(), crate::task::TaskError> {
        if let Some(mut task) = self.get_task(task_id) {
            let old_status = task.status.to_string();
            task.transition_to(new_status)?;
            let new_status_str = task.status.to_string();
            self.graph.add_node(task_id.clone(), NodeKind::Task, &task);

            if task.status == TaskStatus::Done {
                self.events.emit(CollabEvent::TaskCompleted {
                    task_id: task_id.clone(),
                    actor,
                    evidence_count: task.evidence.len(),
                });
            } else {
                self.events.emit(CollabEvent::TaskStatusChanged {
                    task_id: task_id.clone(),
                    old_status,
                    new_status: new_status_str,
                    actor,
                });
            }
            Ok(())
        } else {
            Err(crate::task::TaskError::NotFound(task_id.to_string()))
        }
    }

    pub fn get_task(&self, id: &NodeId) -> Option<Task> {
        self.graph
            .nodes
            .get(id)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    // ── Goal operations ──

    pub fn create_goal(
        &mut self,
        title: &str,
        description: &str,
        period: &str,
        owner: ActorId,
        parent_id: &NodeId,
    ) -> NodeId {
        let goal = Goal::new(title, description, period, owner.clone());
        let id = goal.id.clone();

        self.graph.add_node(id.clone(), NodeKind::Goal, &goal);
        self.graph
            .add_edge(Edge::new(EdgeKind::ChildOf, parent_id.clone(), id.clone()));
        self.events.emit(CollabEvent::GoalCreated {
            goal_id: id.clone(),
            parent_id: parent_id.clone(),
            actor: owner,
        });
        id
    }

    pub fn update_goal_progress(&mut self, goal_id: &NodeId, actor: ActorId) {
        if let Some(mut goal) = self.get_goal(goal_id) {
            goal.recalculate_status();
            let progress = goal.progress();
            self.graph.add_node(goal_id.clone(), NodeKind::Goal, &goal);
            self.events.emit(CollabEvent::GoalProgressUpdated {
                goal_id: goal_id.clone(),
                progress,
                actor,
            });
        }
    }

    pub fn get_goal(&self, id: &NodeId) -> Option<Goal> {
        self.graph
            .nodes
            .get(id)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    // ── Review operations ──

    pub fn create_review(
        &mut self,
        title: &str,
        target_kind: ReviewTargetKind,
        target_id: NodeId,
        created_by: ActorId,
    ) -> NodeId {
        let review = Review::new(title, target_kind, target_id.clone(), created_by.clone());
        let id = review.id.clone();

        self.graph.add_node(id.clone(), NodeKind::Review, &review);
        self.graph.add_edge(Edge::new(
            EdgeKind::BelongsTo,
            target_id.clone(),
            id.clone(),
        ));
        self.events.emit(CollabEvent::ReviewCreated {
            review_id: id.clone(),
            target_id,
            actor: created_by,
        });
        id
    }

    pub fn approve_review(&mut self, review_id: &NodeId, reviewer: ActorId) {
        if let Some(mut review) = self.get_review(review_id) {
            let _ = review.approve(&reviewer);
            self.graph
                .add_node(review_id.clone(), NodeKind::Review, &review);
            self.events.emit(CollabEvent::ReviewApproved {
                review_id: review_id.clone(),
                reviewer,
            });
        }
    }

    pub fn get_review(&self, id: &NodeId) -> Option<Review> {
        self.graph
            .nodes
            .get(id)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    // ── Summary operations ──

    pub fn create_summary(
        &mut self,
        title: &str,
        kind: SummaryKind,
        source_id: NodeId,
        generated_by: ActorId,
        is_ai_generated: bool,
    ) -> NodeId {
        let summary = Summary::new(
            title,
            kind,
            source_id.clone(),
            generated_by.clone(),
            is_ai_generated,
        );
        let id = summary.id.clone();

        self.graph.add_node(id.clone(), NodeKind::Summary, &summary);
        self.graph.add_edge(Edge::new(
            EdgeKind::GeneratedFrom,
            id.clone(),
            source_id.clone(),
        ));
        self.events.emit(CollabEvent::SummaryGenerated {
            summary_id: id.clone(),
            source_id,
            actor: generated_by,
        });
        id
    }

    pub fn get_summary(&self, id: &NodeId) -> Option<Summary> {
        self.graph
            .nodes
            .get(id)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    // ── View operations ──

    pub fn project_view(&self, view_type: &str, config: &ViewConfig) -> Option<ViewProjection> {
        self.views.project(view_type, &self.graph, config)
    }

    pub fn available_views(&self) -> Vec<String> {
        self.views.available_views()
    }

    // ── Query helpers ──

    /// Get all children of a node of a specific kind.
    pub fn get_children_of_kind(&self, parent_id: &NodeId, kind: &NodeKind) -> Vec<NodeId> {
        self.graph
            .children(parent_id)
            .into_iter()
            .filter_map(|(_, val)| {
                let node_id = val.get("id").and_then(|v| v.as_str())?;
                let nid = NodeId(node_id.to_string());
                let node_kind = self.graph.node_kinds.get(&nid)?;
                if node_kind == kind {
                    Some(nid)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get all tasks belonging to a project.
    pub fn project_tasks(&self, project_id: &NodeId) -> Vec<Task> {
        self.get_children_of_kind(project_id, &NodeKind::Task)
            .into_iter()
            .filter_map(|id| self.get_task(&id))
            .collect()
    }

    /// Get all channels belonging to a workspace.
    pub fn workspace_channels(&self, workspace_id: &NodeId) -> Vec<Channel> {
        self.get_children_of_kind(workspace_id, &NodeKind::Channel)
            .into_iter()
            .filter_map(|id| self.get_channel(&id))
            .collect()
    }
}

impl Default for WorkplaceHub {
    fn default() -> Self {
        Self::new()
    }
}
