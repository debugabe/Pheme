pub mod presets;

use presets::PersonalityAxes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    Interviewer,
    Specialist,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Persona {
    pub name: String,
    pub role: Role,
    pub domain: Option<String>,
    pub mood: String,
}

impl Persona {
    pub fn get_axes(&self) -> PersonalityAxes {
        presets::get_axes_for_mood(&self.mood)
    }

    pub fn build_prompt_instructions(&self) -> String {
        let axes = self.get_axes();
        let domain_str = self.domain.as_deref().unwrap_or("Tecnologia Geral");

        match self.role {
            Role::Interviewer => format!(
                "Você é {}, entrevistador(a) fixo(a) do podcast. Seu domínio de foco é {}.\n\
                Tom de voz e postura: {}\n\
                Diretrizes comportamentais:\n\
                - Concisão vs Expansividade: {}\n\
                - Didática vs Diretivera: {}\n\
                - Ceticismo vs Entusiasmo: {}\n\
                - Formalidade vs Descontração: {}\n\
                - Estilo: {}\n\
                - Abordagem: {}",
                self.name,
                domain_str,
                axes.summary_description(),
                axes.concise_vs_expansive,
                axes.didactic_vs_direct,
                axes.skeptical_vs_enthusiastic,
                axes.formal_vs_casual,
                axes.analytical_vs_storytelling,
                axes.provocative_vs_conciliatory
            ),
            Role::Specialist => format!(
                "Você é {}, especialista convidado(a) no tema '{}'.\n\
                Tom de voz e postura: {}\n\
                Diretrizes comportamentais:\n\
                - Concisão vs Expansividade: {}\n\
                - Didática vs Diretivera: {}\n\
                - Ceticismo vs Entusiasmo: {}\n\
                - Formalidade vs Descontração: {}\n\
                - Estilo: {}\n\
                - Abordagem: {}",
                self.name,
                domain_str,
                axes.summary_description(),
                axes.concise_vs_expansive,
                axes.didactic_vs_direct,
                axes.skeptical_vs_enthusiastic,
                axes.formal_vs_casual,
                axes.analytical_vs_storytelling,
                axes.provocative_vs_conciliatory
            ),
        }
    }
}
