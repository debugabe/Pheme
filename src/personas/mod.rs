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
    pub mood: String,
}

impl Persona {
    pub fn get_axes(&self) -> PersonalityAxes {
        presets::get_axes_for_mood(&self.mood)
    }

    pub fn build_prompt_instructions(&self) -> String {
        let axes = self.get_axes();

        match self.role {
            Role::Interviewer => format!(
                "Você é {}, o(a) apresentador(a) e entrevistador(a) do podcast. Seu papel é guiar a conversa, fazer perguntas instigantes e aprofundar a discussão com base na notícia do dia.\n\
                Tom de voz e postura: {}\n\
                Diretrizes comportamentais:\n\
                - Concisão vs Expansividade: {}\n\
                - Didática vs Diretividade: {}\n\
                - Ceticismo vs Entusiasmo: {}\n\
                - Formalidade vs Descontração: {}\n\
                - Estilo: {}\n\
                - Abordagem: {}",
                self.name,
                axes.summary_description(),
                axes.concise_vs_expansive,
                axes.didactic_vs_direct,
                axes.skeptical_vs_enthusiastic,
                axes.formal_vs_casual,
                axes.analytical_vs_storytelling,
                axes.provocative_vs_conciliatory
            ),
            Role::Specialist => format!(
                "Você é {}, o(a) especialista convidado(a) no tema da notícia em pauta. Seu papel é trazer explicações profundas, análises técnicas e visão prática sobre o assunto discutido.\n\
                Tom de voz e postura: {}\n\
                Diretrizes comportamentais:\n\
                - Concisão vs Expansividade: {}\n\
                - Didática vs Diretividade: {}\n\
                - Ceticismo vs Entusiasmo: {}\n\
                - Formalidade vs Descontração: {}\n\
                - Estilo: {}\n\
                - Abordagem: {}",
                self.name,
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
