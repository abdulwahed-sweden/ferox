//! Attack Wizard Module
//!
//! Interactive step-by-step guided attack builder that:
//! - Guides users through target definition
//! - Recommends modules based on scope
//! - Generates executable attack plans
//! - Executes with real-time progress feedback
//!
//! # Usage
//!
//! ```text
//! ferox wizard                    # Interactive wizard
//! ferox wizard --quick            # Skip to module selection
//! ferox wizard --template recon   # Use pre-built template
//! ferox wizard --load plan.yaml   # Load saved plan
//! ferox wizard --resume           # Resume interrupted plan
//! ```

mod types;
mod steps;
mod plan;
mod executor;
mod templates;
mod wizard;

pub use types::*;
pub use plan::{AttackPlan, Phase, ModuleConfig};
pub use executor::PlanExecutor;
pub use templates::{AttackTemplate, TemplateType};
pub use wizard::{AttackWizard, WizardArgs, list_templates};
