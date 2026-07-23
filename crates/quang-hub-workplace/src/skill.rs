//! Skill — a shared capability model for both humans and agents.
//!
//! Skills describe what an actor can do. For humans: technical skills,
//! certifications, years of experience. For agents: tool capabilities,
//! domain expertise, model strengths. Skills are used for:
//! - Task assignment (matching required_skills to actor skills)
//! - Team composition
//! - Career / capability development tracking

use serde::{Deserialize, Serialize};

use crate::graph::{ActorId, Timestamp, now};

/// A skill or capability that an actor possesses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    /// Proficiency level from 0.0 (novice) to 1.0 (expert)
    pub proficiency: f64,
    pub certifications: Vec<String>,
    pub years_experience: Option<f64>,
    /// Agent-specific: the tool/API this skill maps to (e.g., "github_api", "python_repl")
    pub tool_id: Option<String>,
    /// Agent-specific: domain expertise tags (e.g., "networking", "frontend", "data_science")
    pub domains: Vec<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl Skill {
    pub fn new(name: &str, category: &str, proficiency: f64) -> Self {
        let now = now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            category: category.to_string(),
            description: String::new(),
            proficiency: proficiency.clamp(0.0, 1.0),
            certifications: Vec::new(),
            years_experience: None,
            tool_id: None,
            domains: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_certification(mut self, cert: &str) -> Self {
        self.certifications.push(cert.to_string());
        self
    }

    pub fn with_domain(mut self, domain: &str) -> Self {
        self.domains.push(domain.to_string());
        self
    }

    pub fn with_tool(mut self, tool_id: &str) -> Self {
        self.tool_id = Some(tool_id.to_string());
        self
    }

    /// Human-readable proficiency label.
    pub fn proficiency_label(&self) -> &str {
        if self.proficiency >= 0.9 {
            "Expert"
        } else if self.proficiency >= 0.7 {
            "Advanced"
        } else if self.proficiency >= 0.4 {
            "Intermediate"
        } else {
            "Beginner"
        }
    }
}

/// A collection of skills belonging to an actor, with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorSkillSet {
    pub actor_id: ActorId,
    pub skills: Vec<Skill>,
    pub updated_at: Timestamp,
}

impl ActorSkillSet {
    pub fn new(actor_id: ActorId) -> Self {
        Self {
            actor_id,
            skills: Vec::new(),
            updated_at: now(),
        }
    }

    pub fn add_skill(&mut self, skill: Skill) {
        // Replace skill with same name+category
        if let Some(existing) = self
            .skills
            .iter_mut()
            .find(|s| s.name == skill.name && s.category == skill.category)
        {
            *existing = skill;
        } else {
            self.skills.push(skill);
        }
        self.updated_at = now();
    }

    pub fn remove_skill(&mut self, name: &str, category: &str) {
        self.skills.retain(|s| !(s.name == name && s.category == category));
        self.updated_at = now();
    }

    pub fn has_skill(&self, name: &str, min_proficiency: f64) -> bool {
        self.skills
            .iter()
            .any(|s| s.name == name && s.proficiency >= min_proficiency)
    }

    /// Find the best match for a required skill.
    pub fn best_match(&self, required_skill: &str) -> Option<&Skill> {
        self.skills
            .iter()
            .filter(|s| {
                s.name.to_lowercase() == required_skill.to_lowercase()
                    || s.domains.iter().any(|d| d.to_lowercase() == required_skill.to_lowercase())
            })
            .max_by(|a, b| a.proficiency.partial_cmp(&b.proficiency).unwrap_or(std::cmp::Ordering::Equal))
    }

    pub fn skill_count(&self) -> usize {
        self.skills.len()
    }
}
